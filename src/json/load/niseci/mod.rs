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

//! This module provides support for single step load of JSON data for NISECI:
//! - [`crate::json::deser::niseci`] + [`crate::parser::niseci`]

use crate::deser::{RecordAnagraficaNISECI, RecordCampionamentoNISECI, RecordRiferimentoNISECI};
use crate::domain::niseci::CampionamentoNISECI;
use crate::domain::niseci::{AnagraficaNISECI, RiferimentoNISECI};
use crate::json::deser::niseci::{
    check_anagrafica_niseci_reader, check_campionamento_niseci_reader,
    check_riferimento_niseci_reader,
};
use crate::json::deser::JsonDeserError;
use crate::parser::niseci::{
    check_records_anagrafica_niseci, check_records_campionamento_niseci,
    check_records_riferimento_niseci, RecordAnagraficaNISECIError, RecordCampionamentoNISECIError,
    RecordRiferimentoNISECIError,
};
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

/// Encodes the possible errors returned by NISECI reference JSON deser and parse steps.
#[derive(Debug)]
pub enum RiferimentoNISECIError {
    /// I/O error.
    Io(std::io::Error),
    /// JSON errors.
    Json(Vec<serde_json::Error>),
    /// JSON array error.
    JsonArray(serde_json::Error),
    /// Value check errors.
    Value(Vec<RecordRiferimentoNISECIError>),
}

impl From<JsonDeserError> for RiferimentoNISECIError {
    fn from(err: JsonDeserError) -> Self {
        match err {
            JsonDeserError::Json(errs) => RiferimentoNISECIError::Json(errs),
            JsonDeserError::JsonArray(err) => RiferimentoNISECIError::JsonArray(err),
            JsonDeserError::Io(e) => RiferimentoNISECIError::Io(e),
        }
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
            RiferimentoNISECIError::JsonArray(_) => {
                write!(f, "JSON array parsing error for Riferimento NISECI")
            }
            RiferimentoNISECIError::Value(_) => {
                write!(f, "value validation error for Riferimento NISECI")
            }
        }
    }
}

impl std::error::Error for RiferimentoNISECIError {}

/// Load reference JSON from a [`Read`].
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

/// Load reference JSON from a [`AsRef`]<[`Path`]>.
pub fn load_json_riferimento_niseci_from_path<T>(
    path: impl AsRef<Path>,
) -> Result<RiferimentoNISECI, RiferimentoNISECIError>
where
    T: RecordRiferimentoNISECI,
{
    let file = File::open(path).map_err(|e| RiferimentoNISECIError::Io(e))?;

    load_json_riferimento_niseci_from_reader::<_, T>(file)
}

/// Encodes the possible errors returned by NISECI capture set JSON deser and parse steps.
#[derive(Debug)]
pub enum CampionamentoNISECIError {
    /// I/O error.
    Io(std::io::Error),
    /// JSON errors.
    Json(Vec<serde_json::Error>),
    /// JSON array errors.
    JsonArray(serde_json::Error),
    /// Value check errors.
    Value(Vec<RecordCampionamentoNISECIError>),
}

impl From<JsonDeserError> for CampionamentoNISECIError {
    fn from(err: JsonDeserError) -> Self {
        match err {
            JsonDeserError::Json(errs) => CampionamentoNISECIError::Json(errs),
            JsonDeserError::JsonArray(err) => CampionamentoNISECIError::JsonArray(err),
            JsonDeserError::Io(e) => CampionamentoNISECIError::Io(e),
        }
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
            CampionamentoNISECIError::JsonArray(_) => {
                write!(f, "JSON array parsing error for Campionamento NISECI")
            }
            CampionamentoNISECIError::Value(_) => {
                write!(f, "value validation error for Campionamento NISECI")
            }
        }
    }
}

impl std::error::Error for CampionamentoNISECIError {}

/// Load capture set JSON from a [`Read`].
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

/// Load capture set JSON from a [`AsRef`]<[`Path`]>.
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

/// Encodes the possible errors returned by NISECI additional info JSON deser and parse steps.
#[derive(Debug)]
pub enum AnagraficaNISECIError {
    /// I/O error.
    Io(std::io::Error),
    /// JSON errors.
    Json(Vec<serde_json::Error>),
    /// JSON array error.
    JsonArray(serde_json::Error),
    /// Value check errors.
    Value(Vec<RecordAnagraficaNISECIError>),
}

impl From<JsonDeserError> for AnagraficaNISECIError {
    fn from(err: JsonDeserError) -> Self {
        match err {
            JsonDeserError::Json(errs) => AnagraficaNISECIError::Json(errs),
            JsonDeserError::JsonArray(err) => AnagraficaNISECIError::JsonArray(err),
            JsonDeserError::Io(e) => AnagraficaNISECIError::Io(e),
        }
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
            AnagraficaNISECIError::JsonArray(_) => {
                write!(f, "JSON array parsing error for Anagrafica NISECI")
            }
            AnagraficaNISECIError::Value(_) => {
                write!(f, "value validation error for Anagrafica NISECI")
            }
        }
    }
}

impl std::error::Error for AnagraficaNISECIError {}

/// Load additional info JSON from a [`Read`].
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

/// Load additional info JSON from a [`AsRef`]<[`Path`]>.
pub fn load_json_anagrafica_niseci_from_path<T>(
    path: impl AsRef<Path>,
) -> Result<AnagraficaNISECI, AnagraficaNISECIError>
where
    T: RecordAnagraficaNISECI,
{
    let file = File::open(path).map_err(|e| AnagraficaNISECIError::Io(e))?;

    load_json_anagrafica_niseci_from_reader::<_, T>(file)
}
