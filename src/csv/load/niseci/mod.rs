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

use crate::csv::deser::niseci::{
    check_anagrafica_niseci_reader_conf, check_campionamento_niseci_reader_conf,
    check_riferimento_niseci_reader_conf, PlainRecordCsvAnagraficaNISECI,
    PlainRecordCsvCampionamentoNISECI, PlainRecordCsvRiferimentoNISECI,
};
use crate::csv::deser::{CsvConfig, Delimiter, NormalizerReader, RecordCsv};
use crate::csv::load::InputFormat;
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

pub fn load_csv_riferimento_niseci_from_reader<R, T>(
    reader: R,
    has_headers: bool,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    R: Read,
    T: RecordRiferimentoNISECI + RecordCsv + 'static,
{
    load_csv_riferimento_niseci_from_reader_conf::<R, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
    )
}

pub fn load_csv_riferimento_niseci_from_reader_conf<R, T>(
    reader: R,
    config: CsvConfig,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    R: Read,
    T: RecordRiferimentoNISECI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);
    let csv_records =
        check_riferimento_niseci_reader_conf::<NormalizerReader<R>, T>(normalizing_reader, config)
            .map_err(RiferimentoNISECIError::Csv)?;
    check_records_riferimento_niseci(csv_records).map_err(RiferimentoNISECIError::Value)
}

pub fn load_csv_riferimento_niseci_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    T: RecordRiferimentoNISECI + RecordCsv + 'static,
{
    load_csv_riferimento_niseci_from_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
    )
}

pub fn load_csv_riferimento_niseci_from_path_conf<T>(
    path: impl AsRef<Path>,
    config: CsvConfig,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    T: RecordRiferimentoNISECI + 'static,
{
    let file =
        File::open(path).map_err(|e| RiferimentoNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_csv_riferimento_niseci_from_reader_conf::<_, T>(file, config)
}

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

pub fn load_riferimento_niseci_from_path(
    path: impl AsRef<Path>,
    has_headers: bool,
    format: InputFormat,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError> {
    let file =
        File::open(path).map_err(|e| RiferimentoNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_riferimento_niseci_from_reader::<_>(file, has_headers, format)
}

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

pub fn load_csv_campionamento_niseci_from_reader<R, T>(
    reader: R,
    has_headers: bool,
    riferimento: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    R: Read,
    T: RecordCampionamentoNISECI + RecordCsv + 'static,
{
    load_csv_campionamento_niseci_from_reader_conf::<R, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
        riferimento,
    )
}

pub fn load_csv_campionamento_niseci_from_reader_conf<R, T>(
    reader: R,
    config: CsvConfig,
    riferimento: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    R: Read,
    T: RecordCampionamentoNISECI + 'static,
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

pub fn load_csv_campionamento_niseci_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
    riferimento: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    T: RecordCampionamentoNISECI + RecordCsv + 'static,
{
    load_csv_campionamento_niseci_from_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
        riferimento,
    )
}

pub fn load_csv_campionamento_niseci_from_path_conf<T>(
    path: impl AsRef<Path>,
    config: CsvConfig,
    riferimento: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    T: RecordCampionamentoNISECI + 'static,
{
    let file =
        File::open(path).map_err(|e| CampionamentoNISECIError::Csv(vec![csv::Error::from(e)]))?;
    load_csv_campionamento_niseci_from_reader_conf::<_, T>(file, config, riferimento)
}

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

pub fn load_csv_anagrafica_niseci_from_reader<R, T>(
    reader: R,
    has_headers: bool,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    R: Read,
    T: RecordAnagraficaNISECI + RecordCsv + 'static,
{
    load_csv_anagrafica_niseci_from_reader_conf::<R, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
    )
}

pub fn load_csv_anagrafica_niseci_from_reader_conf<R, T>(
    reader: R,
    config: CsvConfig,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    R: Read,
    T: RecordAnagraficaNISECI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);

    let csv_records =
        check_anagrafica_niseci_reader_conf::<NormalizerReader<R>, T>(normalizing_reader, config)
            .map_err(AnagraficaNISECIError::Csv)?;
    check_records_anagrafica_niseci(csv_records).map_err(AnagraficaNISECIError::Value)
}

pub fn load_csv_anagrafica_niseci_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    T: RecordAnagraficaNISECI + RecordCsv + 'static,
{
    load_csv_anagrafica_niseci_from_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
    )
}

pub fn load_csv_anagrafica_niseci_from_path_conf<T>(
    path: impl AsRef<Path>,
    config: CsvConfig,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    T: RecordAnagraficaNISECI + 'static,
{
    let file =
        File::open(path).map_err(|e| AnagraficaNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_csv_anagrafica_niseci_from_reader_conf::<_, T>(file, config)
}

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

pub fn load_anagrafica_niseci_from_path(
    path: impl AsRef<Path>,
    has_headers: bool,
    format: InputFormat,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError> {
    let file =
        File::open(path).map_err(|e| AnagraficaNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_anagrafica_niseci_from_reader::<_>(file, has_headers, format)
}
