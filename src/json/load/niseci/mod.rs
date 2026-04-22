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

use crate::deser::{RecordAnagraficaNISECI, RecordCampionamentoNISECI, RecordRiferimentoNISECI};
use crate::domain::niseci::{AnagraficaNISECI, CampionamentoNISECI, RiferimentoNISECI};
use crate::json::deser::niseci::{
    check_anagrafica_niseci_reader, check_campionamento_niseci_reader,
    check_riferimento_niseci_reader,
};
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
    Io(std::io::Error),
    Json(Vec<serde_json::Error>),
    Value(Vec<RecordRiferimentoNISECIError>),
}

impl From<Vec<serde_json::Error>> for RiferimentoNISECIError {
    fn from(errs: Vec<serde_json::Error>) -> Self {
        RiferimentoNISECIError::Json(errs)
    }
}

impl fmt::Display for RiferimentoNISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RiferimentoNISECIError::Io(_) => {
                write!(f, "I/O error for Riferimento NISECI")
            }
            RiferimentoNISECIError::Json(_) => {
                write!(f, "JSON parsing error for Riferimento NISECI")
            }
            RiferimentoNISECIError::Value(_) => {
                write!(f, "value validation error for Riferimento NISECI")
            }
        }
    }
}

impl std::error::Error for RiferimentoNISECIError {}

pub fn load_json_riferimento_niseci_from_reader<R, T>(
    reader: R,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    R: Read,
    T: RecordRiferimentoNISECI,
{
    let json_records = check_riferimento_niseci_reader::<R, T>(reader)?;
    check_records_riferimento_niseci(json_records).map_err(RiferimentoNISECIError::Value)
}

pub fn load_json_riferimento_niseci_from_path<T>(
    path: impl AsRef<Path>,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    T: RecordRiferimentoNISECI,
{
    let file = File::open(path).map_err(|e| RiferimentoNISECIError::Io(e))?;

    load_json_riferimento_niseci_from_reader::<_, T>(file)
}

#[derive(Debug)]
pub enum CampionamentoNISECIError {
    Io(std::io::Error),
    Json(Vec<serde_json::Error>),
    Value(Vec<RecordCampionamentoNISECIError>),
}

impl From<Vec<serde_json::Error>> for CampionamentoNISECIError {
    fn from(errs: Vec<serde_json::Error>) -> Self {
        CampionamentoNISECIError::Json(errs)
    }
}

impl fmt::Display for CampionamentoNISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CampionamentoNISECIError::Io(_) => {
                write!(f, "I/O parsing error for Campionamento NISECI")
            }
            CampionamentoNISECIError::Json(_) => {
                write!(f, "JSON parsing error for Campionamento NISECI")
            }
            CampionamentoNISECIError::Value(_) => {
                write!(f, "value validation error for Campionamento NISECI")
            }
        }
    }
}

impl std::error::Error for CampionamentoNISECIError {}

pub fn load_json_campionamento_niseci_from_reader<R, T>(
    reader: R,
    riferimento: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    R: Read,
    T: RecordCampionamentoNISECI,
{
    let json_records = check_campionamento_niseci_reader::<R, T>(reader)?;

    check_records_campionamento_niseci(json_records, riferimento)
        .map_err(CampionamentoNISECIError::Value)
}

pub fn load_json_campionamento_niseci_from_path<T>(
    path: impl AsRef<Path>,
    riferimento: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, CampionamentoNISECIError>
where
    T: RecordCampionamentoNISECI,
{
    let file = File::open(path).map_err(|e| CampionamentoNISECIError::Io(e))?;
    load_json_campionamento_niseci_from_reader::<_, T>(file, riferimento)
}

#[derive(Debug)]
pub enum AnagraficaNISECIError {
    Io(std::io::Error),
    Json(Vec<serde_json::Error>),
    Value(Vec<RecordAnagraficaNISECIError>),
}

impl From<Vec<serde_json::Error>> for AnagraficaNISECIError {
    fn from(errs: Vec<serde_json::Error>) -> Self {
        AnagraficaNISECIError::Json(errs)
    }
}

impl fmt::Display for AnagraficaNISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AnagraficaNISECIError::Io(_) => {
                write!(f, "I/O parsing error for Anagrafica NISECI")
            }
            AnagraficaNISECIError::Json(_) => {
                write!(f, "JSON parsing error for Anagrafica NISECI")
            }
            AnagraficaNISECIError::Value(_) => {
                write!(f, "value validation error for Anagrafica NISECI")
            }
        }
    }
}

impl std::error::Error for AnagraficaNISECIError {}

pub fn load_json_anagrafica_niseci_from_reader<R, T>(
    reader: R,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    R: Read,
    T: RecordAnagraficaNISECI,
{
    let json_records = check_anagrafica_niseci_reader::<R, T>(reader)?;
    check_records_anagrafica_niseci(json_records).map_err(AnagraficaNISECIError::Value)
}

pub fn load_json_anagrafica_niseci_from_path<T>(
    path: impl AsRef<Path>,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    T: RecordAnagraficaNISECI,
{
    let file = File::open(path).map_err(|e| AnagraficaNISECIError::Io(e))?;

    load_json_anagrafica_niseci_from_reader::<_, T>(file)
}
