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
    check_anagrafica_niseci_reader, check_campionamento_niseci_reader,
    check_riferimento_niseci_reader, PlainRecordCsvAnagraficaNISECI,
    PlainRecordCsvCampionamentoNISECI, PlainRecordCsvRiferimentoNISECI,
    VeryItalianRecordCsvAnagraficaNISECI, VeryItalianRecordCsvCampionamentoNISECI,
    VeryItalianRecordCsvRiferimentoNISECI,
};
use crate::csv::deser::NormalizerReader;
use crate::csv::load::InputFormat;
use crate::csv::parser::niseci::{
    check_records_anagrafica_niseci, check_records_campionamento_niseci_impl,
    check_records_riferimento_niseci_impl, RecordCsvAnagraficaNISECIError,
    RecordCsvCampionamentoNISECIError, RecordCsvRiferimentoNISECIError,
};
use crate::csv::{
    RecordCsvAnagraficaNISECI, RecordCsvCampionamentoNISECI, RecordCsvRiferimentoNISECI,
};
use crate::domain::niseci::{AnagraficaNISECI, CampionamentoNISECI, RiferimentoNISECI};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub enum RiferimentoNISECIError {
    Csv(Vec<csv::Error>),
    Value(Vec<RecordCsvRiferimentoNISECIError>),
}

pub fn load_csv_riferimento_niseci_from_reader<R, T>(
    reader: R,
    has_headers: bool,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    R: Read,
    T: RecordCsvRiferimentoNISECI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);
    let csv_records =
        check_riferimento_niseci_reader::<NormalizerReader<R>, T>(normalizing_reader, has_headers)
            .map_err(RiferimentoNISECIError::Csv)?;
    check_records_riferimento_niseci_impl(csv_records).map_err(RiferimentoNISECIError::Value)
}

pub fn load_csv_riferimento_niseci_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    T: RecordCsvRiferimentoNISECI + 'static,
{
    let file =
        File::open(path).map_err(|e| RiferimentoNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_csv_riferimento_niseci_from_reader::<_, T>(file, has_headers)
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
        InputFormat::Standard => load_csv_riferimento_niseci_from_reader::<
            NormalizerReader<R>,
            PlainRecordCsvRiferimentoNISECI,
        >(normalizing_reader, has_headers),
        InputFormat::Alternative => load_csv_riferimento_niseci_from_reader::<
            NormalizerReader<R>,
            VeryItalianRecordCsvRiferimentoNISECI,
        >(normalizing_reader, has_headers),
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
    Value(Vec<RecordCsvCampionamentoNISECIError>),
}

pub fn load_csv_campionamento_niseci_from_reader<R, T>(
    reader: R,
    has_headers: bool,
    riferimento: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    R: Read,
    T: RecordCsvCampionamentoNISECI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);
    let csv_records = check_campionamento_niseci_reader::<NormalizerReader<R>, T>(
        normalizing_reader,
        has_headers,
    )
    .map_err(CampionamentoNISECIError::Csv)?;

    check_records_campionamento_niseci_impl(csv_records, riferimento)
        .map_err(CampionamentoNISECIError::Value)
}

pub fn load_csv_campionamento_niseci_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
    riferimento: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    T: RecordCsvCampionamentoNISECI + 'static,
{
    let file =
        File::open(path).map_err(|e| CampionamentoNISECIError::Csv(vec![csv::Error::from(e)]))?;
    load_csv_campionamento_niseci_from_reader::<_, T>(file, has_headers, riferimento)
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
        InputFormat::Standard => load_csv_campionamento_niseci_from_reader::<
            NormalizerReader<R>,
            PlainRecordCsvCampionamentoNISECI,
        >(normalizing_reader, has_headers, riferimento),
        InputFormat::Alternative => load_csv_campionamento_niseci_from_reader::<
            NormalizerReader<R>,
            VeryItalianRecordCsvCampionamentoNISECI,
        >(normalizing_reader, has_headers, riferimento),
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
    Value(Vec<RecordCsvAnagraficaNISECIError>),
}

pub fn load_csv_anagrafica_niseci_from_reader<R, T>(
    reader: R,
    has_headers: bool,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    R: Read,
    T: RecordCsvAnagraficaNISECI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);

    let csv_records =
        check_anagrafica_niseci_reader::<NormalizerReader<R>, T>(normalizing_reader, has_headers)
            .map_err(AnagraficaNISECIError::Csv)?;
    check_records_anagrafica_niseci(csv_records).map_err(AnagraficaNISECIError::Value)
}

pub fn load_csv_anagrafica_niseci_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    T: RecordCsvAnagraficaNISECI + 'static,
{
    let file =
        File::open(path).map_err(|e| AnagraficaNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_csv_anagrafica_niseci_from_reader::<_, T>(file, has_headers)
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
        InputFormat::Standard => load_csv_anagrafica_niseci_from_reader::<
            NormalizerReader<R>,
            PlainRecordCsvAnagraficaNISECI,
        >(normalizing_reader, has_headers),
        InputFormat::Alternative => load_csv_anagrafica_niseci_from_reader::<
            NormalizerReader<R>,
            VeryItalianRecordCsvAnagraficaNISECI,
        >(normalizing_reader, has_headers),
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
