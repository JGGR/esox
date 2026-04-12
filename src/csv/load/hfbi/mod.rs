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
use crate::csv::deser::hfbi::{check_anagrafica_hfbi_reader, check_campionamento_hfbi_reader};
use crate::csv::deser::NormalizerReader;
use crate::csv::parser::hfbi::{
    check_records_anagrafica_hfbi, check_records_campionamento_hfbi, RecordCsvAnagraficaHFBIError,
    RecordCsvCampionamentoHFBIError,
};
use crate::csv::{RecordCsvAnagraficaHFBI, RecordCsvCampionamentoHFBI};
use crate::domain::hfbi::{AnagraficaHFBI, RecordHFBI};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
pub enum CampionamentoHFBIError {
    Csv(Vec<csv::Error>),
    Value(Vec<RecordCsvCampionamentoHFBIError>),
}

pub fn load_campionamento_hfbi_from_reader<R, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<RecordHFBI>, CampionamentoHFBIError>
where
    R: Read,
    T: RecordCsvCampionamentoHFBI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);
    let csv_records =
        check_campionamento_hfbi_reader::<NormalizerReader<R>, T>(normalizing_reader, has_headers)
            .map_err(CampionamentoHFBIError::Csv)?;

    let records =
        check_records_campionamento_hfbi(csv_records).map_err(CampionamentoHFBIError::Value)?;

    Ok(records)
}

pub fn load_campionamento_hfbi_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
) -> Result<Vec<RecordHFBI>, CampionamentoHFBIError>
where
    T: RecordCsvCampionamentoHFBI + 'static,
{
    let file =
        File::open(path).map_err(|e| CampionamentoHFBIError::Csv(vec![csv::Error::from(e)]))?;

    load_campionamento_hfbi_from_reader::<_, T>(file, has_headers)
}

#[derive(Debug)]
pub enum AnagraficaHFBIError {
    Csv(Vec<csv::Error>),
    Value(Vec<RecordCsvAnagraficaHFBIError>),
}

pub fn load_anagrafica_hfbi_from_reader<R, T>(
    reader: R,
    has_headers: bool,
) -> Result<AnagraficaHFBI, AnagraficaHFBIError>
where
    R: Read,
    T: RecordCsvAnagraficaHFBI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);
    let csv_records =
        check_anagrafica_hfbi_reader::<NormalizerReader<R>, T>(normalizing_reader, has_headers)
            .map_err(AnagraficaHFBIError::Csv)?;

    let anagrafica =
        check_records_anagrafica_hfbi(csv_records).map_err(AnagraficaHFBIError::Value)?;

    Ok(anagrafica)
}

pub fn load_anagrafica_hfbi_from_path<T>(
    path: impl AsRef<Path>,
    has_headers: bool,
) -> Result<AnagraficaHFBI, AnagraficaHFBIError>
where
    T: RecordCsvAnagraficaHFBI + 'static,
{
    let file = File::open(path).map_err(|e| AnagraficaHFBIError::Csv(vec![csv::Error::from(e)]))?;

    load_anagrafica_hfbi_from_reader::<_, T>(file, has_headers)
}
