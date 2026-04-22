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

use crate::deser::{
    parse_serialized_records, validate_serialized_records, RecordAnagraficaNISECI,
    RecordCampionamentoNISECI, RecordRiferimentoNISECI,
};
use crate::json::deser::{dispatch_json_input, JsonCheckError};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn parse_json_riferimento_niseci<R, T>(reader: R) -> (Vec<T>, Vec<serde_json::Error>)
where
    R: std::io::Read,
    T: RecordRiferimentoNISECI,
{
    dispatch_json_input(
        reader,
        |res| match res {
            Ok(v) => (v, vec![]),
            Err(JsonCheckError::Io(e)) => (vec![], vec![serde_json::Error::io(e)]),
            Err(JsonCheckError::Json(errs)) => (vec![], errs),
        },
        |deser| {
            let iter = deser.into_iter::<T>();
            parse_serialized_records(iter)
        },
    )
}

pub fn check_riferimento_niseci_reader<R: Read, T>(
    reader: R,
) -> Result<Vec<T>, Vec<serde_json::Error>>
where
    T: RecordRiferimentoNISECI,
{
    dispatch_json_input(
        reader,
        |res| match res {
            Ok(v) => Ok(v),
            Err(JsonCheckError::Io(e)) => Err(vec![serde_json::Error::io(e)]),
            Err(JsonCheckError::Json(errs)) => Err(errs),
        },
        |deser| {
            let iter = deser.into_iter::<T>();
            validate_serialized_records(iter, |errors| {
                for error in errors {
                    eprintln!("  {}", error);
                }
            })
        },
    )
}

pub fn parse_json_campionamento_niseci<R, T>(reader: R) -> (Vec<T>, Vec<serde_json::Error>)
where
    R: std::io::Read,
    T: RecordCampionamentoNISECI,
{
    dispatch_json_input(
        reader,
        |res| match res {
            Ok(v) => (v, vec![]),
            Err(JsonCheckError::Io(e)) => (vec![], vec![serde_json::Error::io(e)]),
            Err(JsonCheckError::Json(errs)) => (vec![], errs),
        },
        |deser| {
            let iter = deser.into_iter::<T>();
            parse_serialized_records(iter)
        },
    )
}

pub fn check_campionamento_niseci_reader<R: Read, T>(
    reader: R,
) -> Result<Vec<T>, Vec<serde_json::Error>>
where
    T: RecordCampionamentoNISECI,
{
    dispatch_json_input(
        reader,
        |res| match res {
            Ok(v) => Ok(v),
            Err(JsonCheckError::Io(e)) => Err(vec![serde_json::Error::io(e)]),
            Err(JsonCheckError::Json(errs)) => Err(errs),
        },
        |deser| {
            let iter = deser.into_iter::<T>();
            validate_serialized_records(iter, |errors| {
                for error in errors {
                    eprintln!("  {}", error);
                }
            })
        },
    )
}

pub fn parse_json_anagrafica_niseci<R, T>(reader: R) -> (Vec<T>, Vec<serde_json::Error>)
where
    R: std::io::Read,
    T: RecordAnagraficaNISECI,
{
    dispatch_json_input(
        reader,
        |res: Result<Vec<T>, _>| match res {
            Ok(v) => (v, vec![]),
            Err(JsonCheckError::Io(e)) => (vec![], vec![serde_json::Error::io(e)]),
            Err(JsonCheckError::Json(errs)) => (vec![], errs),
        },
        |deser| {
            let iter = deser.into_iter::<T>();
            parse_serialized_records(iter)
        },
    )
}

pub fn check_anagrafica_niseci_reader<R: Read, T>(
    reader: R,
) -> Result<Vec<T>, Vec<serde_json::Error>>
where
    T: RecordAnagraficaNISECI,
{
    dispatch_json_input(
        reader,
        |res: Result<Vec<T>, _>| match res {
            Ok(v) => Ok(v),
            Err(JsonCheckError::Io(e)) => Err(vec![serde_json::Error::io(e)]),
            Err(JsonCheckError::Json(errs)) => Err(errs),
        },
        |deser| {
            let iter = deser.into_iter::<T>();
            validate_serialized_records(iter, |errors| {
                for error in errors {
                    eprintln!("  {}", error);
                }
            })
        },
    )
}

pub fn check_riferimento_niseci_path<T>(path: impl AsRef<Path>) -> Result<Vec<T>, JsonCheckError>
where
    T: RecordRiferimentoNISECI,
{
    let file = File::open(path)?;
    check_riferimento_niseci_reader(file).map_err(JsonCheckError::Json)
}

pub fn check_campionamento_niseci_path<T>(path: impl AsRef<Path>) -> Result<Vec<T>, JsonCheckError>
where
    T: RecordCampionamentoNISECI,
{
    let file = File::open(path)?;
    check_campionamento_niseci_reader(file).map_err(JsonCheckError::Json)
}

pub fn check_anagrafica_niseci_path<T>(path: impl AsRef<Path>) -> Result<Vec<T>, JsonCheckError>
where
    T: RecordAnagraficaNISECI,
{
    let file = File::open(path)?;
    check_anagrafica_niseci_reader(file).map_err(JsonCheckError::Json)
}
