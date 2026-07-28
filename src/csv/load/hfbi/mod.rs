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
use crate::csv::deser::hfbi::{
    check_anagrafica_hfbi_reader_conf, check_campionamento_hfbi_reader_conf,
    PlainRecordCsvAnagraficaHFBI, PlainRecordCsvCampionamentoHFBI,
};
use crate::csv::deser::{CsvConfig, Delimiter, NormalizerReader, RecordCsv};
use crate::csv::load::InputFormat;
use crate::csv::stanis::hfbi::{
    VeryItalianRecordAnagraficaHFBI, VeryItalianRecordCampionamentoHFBI,
};
use crate::deser::{RecordAnagraficaHFBI, RecordCampionamentoHFBI};
use crate::domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI};
use crate::parser::hfbi::{
    check_records_anagrafica_hfbi, check_records_campionamento_hfbi, RecordAnagraficaHFBIError,
    RecordCampionamentoHFBIError,
};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub enum CampionamentoHFBIError {
    Csv(Vec<csv::Error>),
    Value(Vec<RecordCampionamentoHFBIError>),
}

impl fmt::Display for CampionamentoHFBIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CampionamentoHFBIError::Csv(_) => {
                write!(f, "CSV parsing error for CampionamentoHFBI")
            }
            CampionamentoHFBIError::Value(_) => {
                write!(f, "value validation error for CampionamentoHFBI")
            }
        }
    }
}

impl std::error::Error for CampionamentoHFBIError {}

pub fn load_csv_campionamento_hfbi_from_reader<R, T>(
    reader: R,
    has_headers: bool,
) -> Result<CampionamentoHFBI, CampionamentoHFBIError>
where
    R: Read,
    T: RecordCampionamentoHFBI + RecordCsv,
{
    load_csv_campionamento_hfbi_from_reader_conf::<R, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
    )
}

pub fn load_csv_campionamento_hfbi_from_reader_conf<R, T>(
    reader: R,
    config: CsvConfig,
) -> Result<CampionamentoHFBI, CampionamentoHFBIError>
where
    R: Read,
    T: RecordCampionamentoHFBI,
{
    let normalizing_reader = NormalizerReader::new(reader);
    let csv_records =
        check_campionamento_hfbi_reader_conf::<NormalizerReader<R>, T>(normalizing_reader, config)
            .map_err(CampionamentoHFBIError::Csv)?;
    check_records_campionamento_hfbi(csv_records).map_err(CampionamentoHFBIError::Value)
}

pub fn load_csv_campionamento_hfbi_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
) -> Result<CampionamentoHFBI, CampionamentoHFBIError>
where
    T: RecordCampionamentoHFBI + RecordCsv,
{
    load_csv_campionamento_hfbi_from_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
    )
}

pub fn load_csv_campionamento_hfbi_from_path_conf<T>(
    path: impl AsRef<Path>,
    config: CsvConfig,
) -> Result<CampionamentoHFBI, CampionamentoHFBIError>
where
    T: RecordCampionamentoHFBI,
{
    let file =
        File::open(path).map_err(|e| CampionamentoHFBIError::Csv(vec![csv::Error::from(e)]))?;

    load_csv_campionamento_hfbi_from_reader_conf::<_, T>(file, config)
}

pub fn load_campionamento_hfbi_from_reader<R>(
    reader: R,
    has_headers: bool,
    format: InputFormat,
) -> Result<CampionamentoHFBI, CampionamentoHFBIError>
where
    R: Read,
{
    let normalizing_reader = NormalizerReader::new(reader);
    match format {
        InputFormat::Standard => load_csv_campionamento_hfbi_from_reader_conf::<
            NormalizerReader<R>,
            PlainRecordCsvCampionamentoHFBI,
        >(
            normalizing_reader,
            CsvConfig::default().with_headers(has_headers),
        ),
        InputFormat::Alternative => load_csv_campionamento_hfbi_from_reader_conf::<
            NormalizerReader<R>,
            VeryItalianRecordCampionamentoHFBI,
        >(
            normalizing_reader,
            CsvConfig::default()
                .with_delimiter(b';')
                .with_headers(has_headers),
        ),
    }
}

pub fn load_campionamento_hfbi_from_path(
    path: impl AsRef<Path>,
    has_headers: bool,
    format: InputFormat,
) -> Result<CampionamentoHFBI, CampionamentoHFBIError> {
    let file =
        File::open(path).map_err(|e| CampionamentoHFBIError::Csv(vec![csv::Error::from(e)]))?;

    load_campionamento_hfbi_from_reader::<_>(file, has_headers, format)
}

#[derive(Debug)]
pub enum AnagraficaHFBIError {
    Csv(Vec<csv::Error>),
    Value(Vec<RecordAnagraficaHFBIError>),
}

impl fmt::Display for AnagraficaHFBIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnagraficaHFBIError::Csv(_) => {
                write!(f, "CSV parsing error for AnagraficaHFBI")
            }
            AnagraficaHFBIError::Value(_) => {
                write!(f, "value validation error for AnagraficaHFBI")
            }
        }
    }
}

impl std::error::Error for AnagraficaHFBIError {}

pub fn load_csv_anagrafica_hfbi_from_reader<R, T>(
    reader: R,
    has_headers: bool,
) -> Result<AnagraficaHFBI, AnagraficaHFBIError>
where
    R: Read,
    T: RecordAnagraficaHFBI + RecordCsv,
{
    load_csv_anagrafica_hfbi_from_reader_conf::<R, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
    )
}

pub fn load_csv_anagrafica_hfbi_from_reader_conf<R, T>(
    reader: R,
    config: CsvConfig,
) -> Result<AnagraficaHFBI, AnagraficaHFBIError>
where
    R: Read,
    T: RecordAnagraficaHFBI,
{
    let normalizing_reader = NormalizerReader::new(reader);

    let csv_records =
        check_anagrafica_hfbi_reader_conf::<NormalizerReader<R>, T>(normalizing_reader, config)
            .map_err(AnagraficaHFBIError::Csv)?;
    check_records_anagrafica_hfbi(csv_records).map_err(AnagraficaHFBIError::Value)
}

pub fn load_csv_anagrafica_hfbi_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
) -> Result<AnagraficaHFBI, AnagraficaHFBIError>
where
    T: RecordAnagraficaHFBI + RecordCsv,
{
    load_csv_anagrafica_hfbi_from_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
    )
}

pub fn load_csv_anagrafica_hfbi_from_path_conf<T>(
    path: impl AsRef<Path>,
    config: CsvConfig,
) -> Result<AnagraficaHFBI, AnagraficaHFBIError>
where
    T: RecordAnagraficaHFBI,
{
    let file = File::open(path).map_err(|e| AnagraficaHFBIError::Csv(vec![csv::Error::from(e)]))?;

    load_csv_anagrafica_hfbi_from_reader_conf::<_, T>(file, config)
}

pub fn load_anagrafica_hfbi_from_reader<R>(
    reader: R,
    has_headers: bool,
    format: InputFormat,
) -> Result<AnagraficaHFBI, AnagraficaHFBIError>
where
    R: Read,
{
    let normalizing_reader = NormalizerReader::new(reader);
    match format {
        InputFormat::Standard => load_csv_anagrafica_hfbi_from_reader_conf::<
            NormalizerReader<R>,
            PlainRecordCsvAnagraficaHFBI,
        >(
            normalizing_reader,
            CsvConfig::default().with_headers(has_headers),
        ),
        InputFormat::Alternative => load_csv_anagrafica_hfbi_from_reader_conf::<
            NormalizerReader<R>,
            VeryItalianRecordAnagraficaHFBI,
        >(
            normalizing_reader,
            CsvConfig::default()
                .with_delimiter(b';')
                .with_headers(has_headers),
        ),
    }
}

pub fn load_anagrafica_hfbi_from_path(
    path: impl AsRef<Path>,
    has_headers: bool,
    format: InputFormat,
) -> Result<AnagraficaHFBI, AnagraficaHFBIError> {
    let file = File::open(path).map_err(|e| AnagraficaHFBIError::Csv(vec![csv::Error::from(e)]))?;

    load_anagrafica_hfbi_from_reader::<_>(file, has_headers, format)
}
