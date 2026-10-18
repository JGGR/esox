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

//! This module provides support for single step load of CSV data for NISECI:
//! - [`crate::csv::deser::niseci`] + [`crate::parser::niseci`]
//! ## Example usage
//!
//!```rust
//! use std::io::Cursor;
//! use esox::csv::load::InputFormat;
//! use esox::csv::load::niseci::*;
//! const CAMPIONAMENTO_DATA: &[u8] =
//! include_bytes!("../../../../templates/campionamento_niseci.csv");
//!
//! const ANAGRAFICA_DATA: &[u8] =
//! include_bytes!("../../../../templates/anagrafica_niseci.csv");
//!
//! let has_headers = true;
//! let format = InputFormat::Alternative;
//! let riferimento = load_riferimento_niseci_from_path("./templates/riferimento_niseci.csv", has_headers, format).unwrap();
//! let campionamento_reader = Cursor::new(CAMPIONAMENTO_DATA);
//! let campionamento = load_campionamento_niseci_from_reader(campionamento_reader, has_headers, &riferimento, format).unwrap();
//! let anagrafica_reader = Cursor::new(ANAGRAFICA_DATA);
//! let anagrafica = load_anagrafica_niseci_from_reader(anagrafica_reader, has_headers, format).unwrap();
//!```

use crate::csv::deser::niseci::{
    check_anagrafica_niseci_reader_conf, check_campionamento_niseci_reader_conf,
    check_riferimento_niseci_reader_conf, PlainRecordCsvAnagraficaNISECI,
    PlainRecordCsvCampionamentoNISECI, PlainRecordCsvRiferimentoNISECI,
};
use crate::csv::deser::{CsvConfig, NormalizerReader, RecordCsv};
use crate::csv::load::{CsvInput, CsvLoadConfig, InputFormat};
use crate::csv::stanis::niseci::{
    VeryItalianRecordAnagraficaNISECI, VeryItalianRecordCampionamentoNISECI,
    VeryItalianRecordRiferimentoNISECI,
};
use crate::deser::{RecordAnagraficaNISECI, RecordCampionamentoNISECI, RecordRiferimentoNISECI};
use crate::domain::niseci::CampionamentoNISECI;
use crate::domain::niseci::{AnagraficaNISECI, RiferimentoNISECI};
use crate::parser::niseci::{
    check_records_anagrafica_niseci, check_records_campionamento_niseci,
    check_records_riferimento_niseci, RecordAnagraficaNISECIError, RecordCampionamentoNISECIError,
    RecordRiferimentoNISECIError,
};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Encodes the possible errors returned by NISECI reference CSV deser and parse steps.
#[derive(Debug)]
pub enum RiferimentoNISECIError {
    Csv(Vec<csv::Error>),
    Value(Vec<RecordRiferimentoNISECIError>),
}

impl fmt::Display for RiferimentoNISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiferimentoNISECIError::Csv(_) => {
                write!(f, "CSV parsing error for Riferimento NISECI")
            }
            RiferimentoNISECIError::Value(_) => {
                write!(f, "value validation error for Riferimento NISECI")
            }
        }
    }
}

impl std::error::Error for RiferimentoNISECIError {}

/// Load reference CSV from a [`Read`].
pub fn load_csv_riferimento_niseci_from_reader<R, T>(
    reader: R,
    has_headers: bool,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    R: Read,
    T: RecordRiferimentoNISECI + RecordCsv,
{
    load_csv_riferimento_niseci_from_reader_conf::<R, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(T::DELIMITER)
            .with_headers(has_headers),
    )
}

/// Load reference CSV from a [`Read`].
/// Allows passing a custom CSV delimiter.
pub fn load_csv_riferimento_niseci_from_reader_conf<R, T>(
    reader: R,
    config: CsvConfig,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    R: Read,
    T: RecordRiferimentoNISECI,
{
    let normalizing_reader = NormalizerReader::new(reader);
    let csv_records =
        check_riferimento_niseci_reader_conf::<NormalizerReader<R>, T>(normalizing_reader, config)
            .map_err(RiferimentoNISECIError::Csv)?;
    check_records_riferimento_niseci(csv_records).map_err(RiferimentoNISECIError::Value)
}

/// Load reference CSV from a [`AsRef`]<[`Path`]>.
pub fn load_csv_riferimento_niseci_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    T: RecordRiferimentoNISECI + RecordCsv,
{
    load_csv_riferimento_niseci_from_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(T::DELIMITER)
            .with_headers(has_headers),
    )
}

/// Load reference CSV from a [`AsRef`]<[`Path`]>.
/// Allows passing a custom CSV delimiter.
pub fn load_csv_riferimento_niseci_from_path_conf<T>(
    path: impl AsRef<Path>,
    config: CsvConfig,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    T: RecordRiferimentoNISECI,
{
    let file =
        File::open(path).map_err(|e| RiferimentoNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_csv_riferimento_niseci_from_reader_conf::<_, T>(file, config)
}

/// Load reference CSV from a [`Read`].
/// Uses [`InputFormat`] to select a CSV delimiter.
pub fn load_riferimento_niseci_from_reader<R>(
    reader: R,
    has_headers: bool,
    format: InputFormat,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    R: Read,
{
    let normalizing_reader = NormalizerReader::new(reader);
    match format {
        InputFormat::Standard => load_csv_riferimento_niseci_from_reader_conf::<
            NormalizerReader<R>,
            PlainRecordCsvRiferimentoNISECI,
        >(
            normalizing_reader,
            CsvConfig::default().with_headers(has_headers),
        ),
        InputFormat::Alternative => load_csv_riferimento_niseci_from_reader_conf::<
            NormalizerReader<R>,
            VeryItalianRecordRiferimentoNISECI,
        >(
            normalizing_reader,
            CsvConfig::default()
                .with_delimiter(b';')
                .with_headers(has_headers),
        ),
    }
}

/// Load reference CSV from a [`AsRef`]<[`Path`]>.
/// Uses [`InputFormat`] to select a CSV delimiter.
pub fn load_riferimento_niseci_from_path(
    path: impl AsRef<Path>,
    has_headers: bool,
    format: InputFormat,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError> {
    let file =
        File::open(path).map_err(|e| RiferimentoNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_riferimento_niseci_from_reader::<_>(file, has_headers, format)
}

pub fn load_riferimento<'a, R, T>(
    source: CsvInput<'a, R>,
    config: CsvLoadConfig,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    R: Read,
    T: RecordRiferimentoNISECI,
{
    match config {
        CsvLoadConfig::Format {
            has_headers,
            format,
        } => match source {
            CsvInput::Reader(reader) => {
                load_riferimento_niseci_from_reader(reader, has_headers, format)
            }
            CsvInput::Path(path) => load_riferimento_niseci_from_path(path, has_headers, format),
        },
        CsvLoadConfig::Csv(config) => match source {
            CsvInput::Reader(reader) => {
                load_csv_riferimento_niseci_from_reader_conf::<_, T>(reader, config)
            }
            CsvInput::Path(path) => load_csv_riferimento_niseci_from_path_conf::<T>(path, config),
        },
    }
}

/// Encodes the possible errors returned by NISECI capture set CSV deser and parse steps.
#[derive(Debug)]
pub enum CampionamentoNISECIError {
    Csv(Vec<csv::Error>),
    Value(Vec<RecordCampionamentoNISECIError>),
}

impl fmt::Display for CampionamentoNISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CampionamentoNISECIError::Csv(_) => {
                write!(f, "CSV parsing error for Campionamento NISECI")
            }
            CampionamentoNISECIError::Value(_) => {
                write!(f, "value validation error for Campionamento NISECI")
            }
        }
    }
}

impl std::error::Error for CampionamentoNISECIError {}

/// Load capture set CSV from a [`Read`].
pub fn load_csv_campionamento_niseci_from_reader<R, T>(
    reader: R,
    has_headers: bool,
    riferimento: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    R: Read,
    T: RecordCampionamentoNISECI + RecordCsv,
{
    load_csv_campionamento_niseci_from_reader_conf::<R, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(T::DELIMITER)
            .with_headers(has_headers),
        riferimento,
    )
}

/// Load capture set CSV from a [`Read`].
/// Allows passing a custom CSV delimiter.
pub fn load_csv_campionamento_niseci_from_reader_conf<R, T>(
    reader: R,
    config: CsvConfig,
    riferimento: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    R: Read,
    T: RecordCampionamentoNISECI,
{
    let normalizing_reader = NormalizerReader::new(reader);
    let csv_records = check_campionamento_niseci_reader_conf::<NormalizerReader<R>, T>(
        normalizing_reader,
        config,
    )
    .map_err(CampionamentoNISECIError::Csv)?;

    check_records_campionamento_niseci(csv_records, riferimento)
        .map_err(CampionamentoNISECIError::Value)
}

/// Load capture set CSV from a [`AsRef`]<[`Path`]>.
pub fn load_csv_campionamento_niseci_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
    riferimento: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    T: RecordCampionamentoNISECI + RecordCsv,
{
    load_csv_campionamento_niseci_from_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(T::DELIMITER)
            .with_headers(has_headers),
        riferimento,
    )
}

/// Load capture set CSV from a [`AsRef`]<[`Path`]>.
/// Allows passing a custom CSV delimiter.
pub fn load_csv_campionamento_niseci_from_path_conf<T>(
    path: impl AsRef<Path>,
    config: CsvConfig,
    riferimento: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    T: RecordCampionamentoNISECI,
{
    let file =
        File::open(path).map_err(|e| CampionamentoNISECIError::Csv(vec![csv::Error::from(e)]))?;
    load_csv_campionamento_niseci_from_reader_conf::<_, T>(file, config, riferimento)
}

/// Load capture set CSV from a [`Read`].
/// Uses [`InputFormat`] to select a CSV delimiter.
pub fn load_campionamento_niseci_from_reader<R>(
    reader: R,
    has_headers: bool,
    riferimento: &RiferimentoNISECI,
    format: InputFormat,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    R: Read,
{
    let normalizing_reader = NormalizerReader::new(reader);
    match format {
        InputFormat::Standard => load_csv_campionamento_niseci_from_reader_conf::<
            NormalizerReader<R>,
            PlainRecordCsvCampionamentoNISECI,
        >(
            normalizing_reader,
            CsvConfig::default().with_headers(has_headers),
            riferimento,
        ),
        InputFormat::Alternative => load_csv_campionamento_niseci_from_reader_conf::<
            NormalizerReader<R>,
            VeryItalianRecordCampionamentoNISECI,
        >(
            normalizing_reader,
            CsvConfig::default()
                .with_delimiter(b';')
                .with_headers(has_headers),
            riferimento,
        ),
    }
}

/// Load capture set CSV from a [`AsRef`]<[`Path`]>.
/// Uses [`InputFormat`] to select a CSV delimiter.
pub fn load_campionamento_niseci_from_path(
    path: impl AsRef<Path>,
    has_headers: bool,
    riferimento: &RiferimentoNISECI,
    format: InputFormat,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError> {
    let file =
        File::open(path).map_err(|e| CampionamentoNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_campionamento_niseci_from_reader::<_>(file, has_headers, riferimento, format)
}

pub fn load_campionamento<'a, R, T>(
    riferimento: &RiferimentoNISECI,
    source: CsvInput<'a, R>,
    config: CsvLoadConfig,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    R: Read,
    T: RecordCampionamentoNISECI,
{
    match config {
        CsvLoadConfig::Format {
            has_headers,
            format,
        } => match source {
            CsvInput::Reader(reader) => {
                load_campionamento_niseci_from_reader(reader, has_headers, riferimento, format)
            }
            CsvInput::Path(path) => {
                load_campionamento_niseci_from_path(path, has_headers, riferimento, format)
            }
        },
        CsvLoadConfig::Csv(config) => match source {
            CsvInput::Reader(reader) => {
                load_csv_campionamento_niseci_from_reader_conf::<_, T>(reader, config, riferimento)
            }
            CsvInput::Path(path) => {
                load_csv_campionamento_niseci_from_path_conf::<T>(path, config, riferimento)
            }
        },
    }
}

/// Encodes the possible errors returned by NISECI additional info CSV deser and parse steps.
#[derive(Debug)]
pub enum AnagraficaNISECIError {
    Csv(Vec<csv::Error>),
    Value(Vec<RecordAnagraficaNISECIError>),
}

impl fmt::Display for AnagraficaNISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnagraficaNISECIError::Csv(_) => {
                write!(f, "CSV parsing error for Anagrafica NISECI")
            }
            AnagraficaNISECIError::Value(_) => {
                write!(f, "value validation error for Anagrafica NISECI")
            }
        }
    }
}

impl std::error::Error for AnagraficaNISECIError {}

/// Load additional info CSV from a [`Read`].
pub fn load_csv_anagrafica_niseci_from_reader<R, T>(
    reader: R,
    has_headers: bool,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    R: Read,
    T: RecordAnagraficaNISECI + RecordCsv,
{
    load_csv_anagrafica_niseci_from_reader_conf::<R, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(T::DELIMITER)
            .with_headers(has_headers),
    )
}

/// Load additional info CSV from a [`Read`].
/// Allows passing a custom CSV delimiter.
pub fn load_csv_anagrafica_niseci_from_reader_conf<R, T>(
    reader: R,
    config: CsvConfig,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    R: Read,
    T: RecordAnagraficaNISECI,
{
    let normalizing_reader = NormalizerReader::new(reader);

    let csv_records =
        check_anagrafica_niseci_reader_conf::<NormalizerReader<R>, T>(normalizing_reader, config)
            .map_err(AnagraficaNISECIError::Csv)?;
    check_records_anagrafica_niseci(csv_records).map_err(AnagraficaNISECIError::Value)
}

/// Load additional info CSV from a [`AsRef`]<[`Path`]>.
pub fn load_csv_anagrafica_niseci_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    T: RecordAnagraficaNISECI + RecordCsv,
{
    load_csv_anagrafica_niseci_from_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(T::DELIMITER)
            .with_headers(has_headers),
    )
}

/// Load additional info CSV from a [`AsRef`]<[`Path`]>.
/// Allows passing a custom CSV delimiter.
pub fn load_csv_anagrafica_niseci_from_path_conf<T>(
    path: impl AsRef<Path>,
    config: CsvConfig,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    T: RecordAnagraficaNISECI,
{
    let file =
        File::open(path).map_err(|e| AnagraficaNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_csv_anagrafica_niseci_from_reader_conf::<_, T>(file, config)
}

/// Load additional info CSV from a [`Read`].
/// Uses [`InputFormat`] to select a CSV delimiter.
pub fn load_anagrafica_niseci_from_reader<R>(
    reader: R,
    has_headers: bool,
    format: InputFormat,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    R: Read,
{
    let normalizing_reader = NormalizerReader::new(reader);
    match format {
        InputFormat::Standard => load_csv_anagrafica_niseci_from_reader_conf::<
            NormalizerReader<R>,
            PlainRecordCsvAnagraficaNISECI,
        >(
            normalizing_reader,
            CsvConfig::default().with_headers(has_headers),
        ),
        InputFormat::Alternative => load_csv_anagrafica_niseci_from_reader_conf::<
            NormalizerReader<R>,
            VeryItalianRecordAnagraficaNISECI,
        >(
            normalizing_reader,
            CsvConfig::default()
                .with_delimiter(b';')
                .with_headers(has_headers),
        ),
    }
}

/// Load additional info CSV from a [`AsRef`]<[`Path`]>.
/// Uses [`InputFormat`] to select a CSV delimiter.
pub fn load_anagrafica_niseci_from_path(
    path: impl AsRef<Path>,
    has_headers: bool,
    format: InputFormat,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError> {
    let file =
        File::open(path).map_err(|e| AnagraficaNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_anagrafica_niseci_from_reader::<_>(file, has_headers, format)
}

pub fn load_anagrafica<'a, R, T>(
    source: CsvInput<'a, R>,
    config: CsvLoadConfig,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    R: Read,
    T: RecordAnagraficaNISECI,
{
    match config {
        CsvLoadConfig::Format {
            has_headers,
            format,
        } => match source {
            CsvInput::Reader(reader) => {
                load_anagrafica_niseci_from_reader(reader, has_headers, format)
            }
            CsvInput::Path(path) => load_anagrafica_niseci_from_path(path, has_headers, format),
        },
        CsvLoadConfig::Csv(config) => match source {
            CsvInput::Reader(reader) => {
                load_csv_anagrafica_niseci_from_reader_conf::<_, T>(reader, config)
            }
            CsvInput::Path(path) => load_csv_anagrafica_niseci_from_path_conf::<T>(path, config),
        },
    }
}
