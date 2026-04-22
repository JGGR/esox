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
use crate::deser::{RecordAnagraficaHFBI, RecordCampionamentoHFBI};
use crate::domain::hfbi::{AnagraficaHFBI, CampionamentoHFBI};
use crate::json::deser::hfbi::{check_anagrafica_hfbi_reader, check_campionamento_hfbi_reader};
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
    Io(std::io::Error),
    Json(Vec<serde_json::Error>),
    Value(Vec<RecordCampionamentoHFBIError>),
}

impl From<Vec<serde_json::Error>> for CampionamentoHFBIError {
    fn from(errs: Vec<serde_json::Error>) -> Self {
        CampionamentoHFBIError::Json(errs)
    }
}

impl fmt::Display for CampionamentoHFBIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CampionamentoHFBIError::Io(_) => {
                write!(f, "I/O parsing error for CampionamentoHFBI")
            }
            CampionamentoHFBIError::Json(_) => {
                write!(f, "JSON parsing error for CampionamentoHFBI")
            }
            CampionamentoHFBIError::Value(_) => {
                write!(f, "value validation error for CampionamentoHFBI")
            }
        }
    }
}

impl std::error::Error for CampionamentoHFBIError {}

pub fn load_json_campionamento_hfbi_from_reader<R, T>(
    reader: R,
) -> Result<CampionamentoHFBI, CampionamentoHFBIError>
where
    R: Read,
    T: RecordCampionamentoHFBI,
{
    let json_records = check_campionamento_hfbi_reader::<R, T>(reader)?;
    check_records_campionamento_hfbi(json_records).map_err(CampionamentoHFBIError::Value)
}

pub fn load_json_campionamento_hfbi_from_path<T>(
    path: impl AsRef<Path>,
) -> Result<CampionamentoHFBI, CampionamentoHFBIError>
where
    T: RecordCampionamentoHFBI,
{
    let file = File::open(path).map_err(|e| CampionamentoHFBIError::Io(e))?;

    load_json_campionamento_hfbi_from_reader::<_, T>(file)
}

#[derive(Debug)]
pub enum AnagraficaHFBIError {
    Io(std::io::Error),
    Json(Vec<serde_json::Error>),
    Value(Vec<RecordAnagraficaHFBIError>),
}

impl From<Vec<serde_json::Error>> for AnagraficaHFBIError {
    fn from(errs: Vec<serde_json::Error>) -> Self {
        AnagraficaHFBIError::Json(errs)
    }
}

impl fmt::Display for AnagraficaHFBIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnagraficaHFBIError::Io(_) => {
                write!(f, "I/O parsing error for AnagraficaHFBI")
            }
            AnagraficaHFBIError::Json(_) => {
                write!(f, "JSON parsing error for AnagraficaHFBI")
            }
            AnagraficaHFBIError::Value(_) => {
                write!(f, "value validation error for AnagraficaHFBI")
            }
        }
    }
}

impl std::error::Error for AnagraficaHFBIError {}

pub fn load_json_anagrafica_hfbi_from_reader<R, T>(
    reader: R,
) -> Result<AnagraficaHFBI, AnagraficaHFBIError>
where
    R: Read,
    T: RecordAnagraficaHFBI,
{
    let json_records = check_anagrafica_hfbi_reader::<R, T>(reader)?;
    check_records_anagrafica_hfbi(json_records).map_err(AnagraficaHFBIError::Value)
}

pub fn load_json_anagrafica_hfbi_from_path<T>(
    path: impl AsRef<Path>,
) -> Result<AnagraficaHFBI, AnagraficaHFBIError>
where
    T: RecordAnagraficaHFBI,
{
    let file = File::open(path).map_err(|e| AnagraficaHFBIError::Io(e))?;

    load_json_anagrafica_hfbi_from_reader::<_, T>(file)
}
