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
use crate::json::deser::{
    dispatch_json_input, JsonDeserError, JsonDispatchError, JsonPathCheckError,
};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn parse_json_riferimento_niseci<R, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    R: std::io::Read,
    T: RecordRiferimentoNISECI,
{
    dispatch_json_input(
        reader,
        |res| match res {
            Ok(v) => Ok(v),
            Err(JsonDispatchError::Io(e)) => Err(JsonDeserError::Io(e)),
            Err(JsonDispatchError::Json(errs)) => Err(JsonDeserError::Json(errs)),
        },
        |deser| {
            let iter = deser.into_iter::<T>();
            let (records, errs) = parse_serialized_records(iter);
            if !errs.is_empty() {
                return Err(JsonDeserError::Json(errs));
            }
            Ok(records)
        },
    )
}

pub fn check_riferimento_niseci_reader<R: Read, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    T: RecordRiferimentoNISECI,
{
    dispatch_json_input(
        reader,
        |res| match res {
            Ok(v) => Ok(v),
            Err(JsonDispatchError::Io(e)) => Err(JsonDeserError::Io(e)),
            Err(JsonDispatchError::Json(errs)) => Err(JsonDeserError::Json(errs)),
        },
        |deser| {
            let iter = deser.into_iter::<T>();
            validate_serialized_records(iter, |errors| {
                for error in errors {
                    eprintln!("  {}", error);
                }
            })
            .map_err(|errs| JsonDeserError::Json(errs))
        },
    )
}

pub fn parse_json_campionamento_niseci<R, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    R: std::io::Read,
    T: RecordCampionamentoNISECI,
{
    dispatch_json_input(
        reader,
        |res| match res {
            Ok(v) => Ok(v),
            Err(JsonDispatchError::Io(e)) => Err(JsonDeserError::Io(e)),
            Err(JsonDispatchError::Json(errs)) => Err(JsonDeserError::Json(errs)),
        },
        |deser| {
            let iter = deser.into_iter::<T>();
            let (records, errs) = parse_serialized_records(iter);
            if !errs.is_empty() {
                return Err(JsonDeserError::Json(errs));
            }
            Ok(records)
        },
    )
}

pub fn check_campionamento_niseci_reader<R: Read, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    T: RecordCampionamentoNISECI,
{
    dispatch_json_input(
        reader,
        |res| match res {
            Ok(v) => Ok(v),
            Err(JsonDispatchError::Io(e)) => Err(JsonDeserError::Io(e)),
            Err(JsonDispatchError::Json(errs)) => Err(JsonDeserError::Json(errs)),
        },
        |deser| {
            let iter = deser.into_iter::<T>();
            validate_serialized_records(iter, |errors| {
                for error in errors {
                    eprintln!("  {}", error);
                }
            })
            .map_err(|errs| JsonDeserError::Json(errs))
        },
    )
}

pub fn parse_json_anagrafica_niseci<R, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    R: std::io::Read,
    T: RecordAnagraficaNISECI,
{
    dispatch_json_input(
        reader,
        |res: Result<Vec<T>, _>| match res {
            Ok(v) => Ok(v),
            Err(JsonDispatchError::Io(e)) => Err(JsonDeserError::Io(e)),
            Err(JsonDispatchError::Json(errs)) => Err(JsonDeserError::Json(errs)),
        },
        |deser| {
            let iter = deser.into_iter::<T>();
            let (records, errs) = parse_serialized_records(iter);
            if !errs.is_empty() {
                return Err(JsonDeserError::Json(errs));
            }
            Ok(records)
        },
    )
}

pub fn check_anagrafica_niseci_reader<R: Read, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    T: RecordAnagraficaNISECI,
{
    dispatch_json_input(
        reader,
        |res: Result<Vec<T>, _>| match res {
            Ok(v) => Ok(v),
            Err(JsonDispatchError::Io(e)) => Err(JsonDeserError::Io(e)),
            Err(JsonDispatchError::Json(errs)) => Err(JsonDeserError::Json(errs)),
        },
        |deser| {
            let iter = deser.into_iter::<T>();
            validate_serialized_records(iter, |errors| {
                for error in errors {
                    eprintln!("  {}", error);
                }
            })
            .map_err(|errs| JsonDeserError::Json(errs))
        },
    )
}

pub fn check_riferimento_niseci_path<T>(
    path: impl AsRef<Path>,
) -> Result<Vec<T>, JsonPathCheckError>
where
    T: RecordRiferimentoNISECI,
{
    let file = File::open(path)?;
    check_riferimento_niseci_reader(file).map_err(|e| match e {
        JsonDeserError::Json(errs) => JsonPathCheckError::Json(errs),
        JsonDeserError::Io(e) => JsonPathCheckError::Io(e),
    })
}

pub fn check_campionamento_niseci_path<T>(
    path: impl AsRef<Path>,
) -> Result<Vec<T>, JsonPathCheckError>
where
    T: RecordCampionamentoNISECI,
{
    let file = File::open(path)?;
    check_campionamento_niseci_reader(file).map_err(|e| match e {
        JsonDeserError::Json(errs) => JsonPathCheckError::Json(errs),
        JsonDeserError::Io(e) => JsonPathCheckError::Io(e),
    })
}

pub fn check_anagrafica_niseci_path<T>(path: impl AsRef<Path>) -> Result<Vec<T>, JsonPathCheckError>
where
    T: RecordAnagraficaNISECI,
{
    let file = File::open(path)?;
    check_anagrafica_niseci_reader(file).map_err(|e| match e {
        JsonDeserError::Json(errs) => JsonPathCheckError::Json(errs),
        JsonDeserError::Io(e) => JsonPathCheckError::Io(e),
    })
}
