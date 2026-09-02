// SPDX-License-Identifier: GPL-3.0-only
/*
    Copyright (C) 2024-2026 jgabaut, gioninjo

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, version 3 of the License.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
//! Deserialization module for CSV to intermediate structs.
//!
//! Provides support for:
//!
//! - Converting raw CSV data into typed intermediate structs
//! - Validating raw CSV data structure and field types
//!   - Eg. Integer fields must not be `"foo"`)
//!
//! To validate intermediate structs into [`crate::domain`] structs, see:
//!
//! - [`crate::parser`]
//!
//! To do deser and parse steps at once (go from raw CSV to domain structs) see:
//!
//! - [`crate::csv::load`]

pub mod utils;

use std::io::{self, Read};
use std::path::Path;

pub(crate) struct NormalizerReader<R: Read> {
    inner: R,
}

impl<R: Read> NormalizerReader<R> {
    pub(crate) fn new(inner: R) -> Self {
        Self { inner }
    }
}

impl<R: Read> Read for NormalizerReader<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let size = self.inner.read(buf)?;

        // Change very italian accented characters in place
        for byte in buf.iter_mut().take(size) {
            match *byte {
                b'\xF2' => *byte = b'o',
                b'\xE0' => *byte = b'a',
                b'\xE8' => *byte = b'e',
                b'\xF9' => *byte = b'u',
                b'\xEC' => *byte = b'i',
                _ => {}
            }
        }

        Ok(size)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CsvConfig {
    delimiter: u8,
    has_headers: bool,
}

impl Default for CsvConfig {
    fn default() -> Self {
        Self {
            delimiter: b';',
            has_headers: true,
        }
    }
}

impl CsvConfig {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn delimiter(&self) -> u8 {
        self.delimiter
    }
    pub fn has_headers(&self) -> bool {
        self.has_headers
    }
    pub fn with_delimiter(mut self, delimiter: u8) -> Self {
        self.delimiter = delimiter;
        self
    }
    pub fn with_headers(mut self, has_headers: bool) -> Self {
        self.has_headers = has_headers;
        self
    }
}

/// Preset [`Delimiter`] for comma (`b','`).
///
/// # Example
/// ```
/// use esox::csv::deser::{Delimiter, CommaDelimiter};
///
/// assert_eq!(CommaDelimiter::DELIMITER, b',');
/// ```
pub struct CommaDelimiter;

/// Preset [`Delimiter`] for semicolon (`b';'`).
///
/// # Example
/// ```
/// use esox::csv::deser::{Delimiter, SemicolonDelimiter};
///
/// assert_eq!(SemicolonDelimiter::DELIMITER, b';');
/// ```
pub struct SemicolonDelimiter;

/// A type-level delimiter.
///
/// Implemented by marker types like [`CommaDelimiter`]
/// and [`SemicolonDelimiter`].
pub trait Delimiter {
    const DELIMITER: u8;
}

impl Delimiter for CommaDelimiter {
    const DELIMITER: u8 = b',';
}
impl Delimiter for SemicolonDelimiter {
    const DELIMITER: u8 = b';';
}

/// A CSV record configuration.
///
/// This trait associates a [`Delimiter`] with a record type.
/// Available presets:
/// - [`CommaDelimiter`]
/// - [`SemicolonDelimiter`].
///
/// # Example
/// ```
/// use esox::csv::deser::{RecordCsv, CommaDelimiter};
///
/// struct MyRecord;
///
/// impl RecordCsv for MyRecord {
///     type D = CommaDelimiter;
/// }
/// ```
pub trait RecordCsv {
    type D: Delimiter;

    const DELIMITER: u8 = <Self::D as Delimiter>::DELIMITER;
}

/// Internal helper used by PlainRecord structs to autoimpl RecordCsv
trait DefaultRecordCsv: RecordCsv {}

impl<T: DefaultRecordCsv> RecordCsv for T {
    type D = CommaDelimiter;
}

impl DefaultRecordCsv for crate::deser::PlainRecordRiferimentoNISECI {}
impl DefaultRecordCsv for crate::deser::PlainRecordCampionamentoNISECI {}
impl DefaultRecordCsv for crate::deser::PlainRecordAnagraficaNISECI {}
impl DefaultRecordCsv for crate::deser::PlainRecordCampionamentoHFBI {}
impl DefaultRecordCsv for crate::deser::PlainRecordAnagraficaHFBI {}

pub fn check_path_is_file_ends_with_csv(path: &Path) -> bool {
    if !path.exists() {
        eprintln!("Error: Passed path does not exist");
        false
    } else if !path.is_file() {
        eprintln!("Error: Passed path is not a regular file");
        false
    } else {
        let ext = path.extension();
        match ext {
            Some(ex) => {
                if !(ex == "csv" || ex == "CSV") {
                    eprintln!("Error: Passed path does not end with .csv");
                    return false;
                }
                true
            }
            None => {
                eprintln!("Error: Passed path does not end with .csv");
                false
            }
        }
    }
}

pub mod hfbi;
pub mod niseci;
