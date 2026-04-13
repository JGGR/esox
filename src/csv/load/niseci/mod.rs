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
    check_riferimento_niseci_reader,
};
use crate::csv::deser::NormalizerReader;
use crate::csv::parser::niseci::{
    check_records_anagrafica_niseci, check_records_campionamento_niseci,
    check_records_riferimento_niseci, RecordCsvAnagraficaNISECIError,
    RecordCsvCampionamentoNISECIError, RecordCsvRiferimentoNISECIError,
};
use crate::csv::{
    RecordCsvAnagraficaNISECI, RecordCsvCampionamentoNISECI, RecordCsvRiferimentoNISECI,
};
use crate::domain::niseci::{AnagraficaNISECI, RecordNISECI, SpecieNISECI};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub enum RiferimentoNISECIError {
    Csv(Vec<csv::Error>),
    Value(Vec<RecordCsvRiferimentoNISECIError>),
}

pub fn load_riferimento_niseci_from_reader<R, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<SpecieNISECI>, RiferimentoNISECIError>
where
    R: Read,
    T: RecordCsvRiferimentoNISECI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);
    let csv_records =
        check_riferimento_niseci_reader::<NormalizerReader<R>, T>(normalizing_reader, has_headers)
            .map_err(RiferimentoNISECIError::Csv)?;

    let specie =
        check_records_riferimento_niseci(csv_records).map_err(RiferimentoNISECIError::Value)?;

    Ok(specie)
}

pub fn load_riferimento_niseci_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
) -> Result<Vec<SpecieNISECI>, RiferimentoNISECIError>
where
    T: RecordCsvRiferimentoNISECI + 'static,
{
    let file =
        File::open(path).map_err(|e| RiferimentoNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_riferimento_niseci_from_reader::<_, T>(file, has_headers)
}

#[derive(Debug)]
pub enum CampionamentoNISECIError {
    Csv(Vec<csv::Error>),
    Value(Vec<RecordCsvCampionamentoNISECIError>),
}

pub fn load_campionamento_niseci_from_reader<R, T>(
    reader: R,
    has_headers: bool,
    riferimento_specie: &Vec<SpecieNISECI>,
) -> Result<Vec<RecordNISECI>, CampionamentoNISECIError>
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

    let records = check_records_campionamento_niseci(csv_records, riferimento_specie)
        .map_err(CampionamentoNISECIError::Value)?;

    Ok(records)
}

pub fn load_campionamento_niseci_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
    riferimento_specie: &Vec<SpecieNISECI>,
) -> Result<Vec<RecordNISECI>, CampionamentoNISECIError>
where
    T: RecordCsvCampionamentoNISECI + 'static,
{
    let file =
        File::open(path).map_err(|e| CampionamentoNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_campionamento_niseci_from_reader::<_, T>(file, has_headers, riferimento_specie)
}

#[derive(Debug)]
pub enum AnagraficaNISECIError {
    Csv(Vec<csv::Error>),
    Value(Vec<RecordCsvAnagraficaNISECIError>),
}

pub fn load_anagrafica_niseci_from_reader<R, T>(
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

    let anagrafica =
        check_records_anagrafica_niseci(csv_records).map_err(AnagraficaNISECIError::Value)?;

    Ok(anagrafica)
}

pub fn load_anagrafica_niseci_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    T: RecordCsvAnagraficaNISECI + 'static,
{
    let file =
        File::open(path).map_err(|e| AnagraficaNISECIError::Csv(vec![csv::Error::from(e)]))?;

    load_anagrafica_niseci_from_reader::<_, T>(file, has_headers)
}
