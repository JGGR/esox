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

pub mod utils;

use crate::csv::stanis::field_name;
use crate::csv::stanis::giorgio::translate_error_message as priv_translate;
use crate::deser::TipoRecord;
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

fn parse_csv_pos(pos: &Option<csv::Position>) -> String {
    let res;
    match pos {
        Some(p) => {
            // These should be equal. We may show the value only once if they are
            let line_offset = p.line();
            let record_offset = p.record();

            if line_offset == record_offset {
                res = format!("Riga: {}", line_offset);
            } else {
                // TODO: How can we hit this branch?
                res = format!("Riga: {} Record: {}", line_offset, record_offset);
            }

            // We ignore this since I don't think users may care?
            // let byte_offset = p.byte();
            // res = format!("Riga: {} Record: {} Char: {} ", line_offset, record_offset, byte_offset);
        }
        None => {
            res = "none".to_string();
        }
    }
    res
}

#[deprecated(
    note = "v0.2 will change visibility.\nConsider using crate::csv::stanis::giorgio::format_csv_errors instead"
)]
pub fn process_csv_errors(errors: &Vec<csv::Error>, tipo_csv: TipoRecord) -> Vec<String> {
    let mut res = Vec::new();
    for error in errors {
        match error.kind() {
            csv::ErrorKind::Deserialize { pos, err } => {
                let field_str = match err.field().map(|f| f as usize) {
                    Some(idx) => field_name(tipo_csv, idx),
                    None => "none".to_string(),
                };
                let mut curr_err = format!(
                    "  Errore di deserializzazione alla posizione: {}: campo {}",
                    parse_csv_pos(pos),
                    field_str,
                );
                match err.kind() {
                    csv::DeserializeErrorKind::Message(msg) => {
                        curr_err = format!("{curr_err}: {}", priv_translate(msg));
                    }
                    csv::DeserializeErrorKind::Unsupported(msg) => {
                        curr_err = format!("{curr_err}: {}", priv_translate(msg));
                    }
                    csv::DeserializeErrorKind::UnexpectedEndOfRow => {
                        curr_err = format!("{curr_err}: Fine riga inatteso");
                    }
                    csv::DeserializeErrorKind::InvalidUtf8(utf8err) => {
                        curr_err = format!("{curr_err}: {}", priv_translate(&utf8err.to_string()));
                    }
                    csv::DeserializeErrorKind::ParseBool(boolerr) => {
                        curr_err = format!("{curr_err}: {}", priv_translate(&boolerr.to_string()));
                    }
                    csv::DeserializeErrorKind::ParseInt(interr) => {
                        curr_err = format!("{curr_err}: {}", priv_translate(&interr.to_string()));
                    }
                    csv::DeserializeErrorKind::ParseFloat(floaterr) => {
                        curr_err = format!("{curr_err}: {}", priv_translate(&floaterr.to_string()));
                    }
                }
                res.push(curr_err);
            }
            csv::ErrorKind::Io(io_error) => {
                res.push(format!(
                    "  Errore di I/O: {}",
                    priv_translate(&io_error.to_string())
                ));
            }
            csv::ErrorKind::Utf8 { pos, err } => {
                res.push(format!(
                    "  Errore UTF-8 alla posizione: {}: {}",
                    parse_csv_pos(pos),
                    priv_translate(&err.to_string())
                ));
            }
            csv::ErrorKind::UnequalLengths {
                pos,
                expected_len,
                len,
            } => {
                res.push(format!(
                    "  Errore numero campi alla posizione: {}: lunghezza attesa {}, trovata {}",
                    parse_csv_pos(pos),
                    expected_len,
                    len // no priv_translate() anche se teoricamente lo supporta
                ));
            }
            _ => {
                res.push(format!(
                    "  Errore sconosciuto: {}",
                    priv_translate(&error.to_string())
                ));
            }
        }
    }
    res
}

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

#[deprecated(
    note = "v0.2 will change visibility.\nConsider using crate::csv::stanis::giorgio::format_csv_error instead"
)]
pub fn translate_error_message(msg: &str) -> String {
    priv_translate(msg)
}
pub mod hfbi;
pub mod niseci;
