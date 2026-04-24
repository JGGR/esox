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
use crate::deser::limits::{with_limited_reader, ByteLimit, DefaultByteLimit};
use crate::deser::{
    check_serialized_records, parse_serialized_records, RecordAnagraficaNISECI,
    RecordCampionamentoNISECI, RecordRiferimentoNISECI,
};
use crate::json::deser::{dispatch_json_input, JsonDeserError, JsonPathCheckError};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn parse_json_riferimento_niseci<R, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    R: std::io::Read,
    T: RecordRiferimentoNISECI,
{
    with_limited_reader(
        reader,
        DefaultByteLimit::MAX_BYTES,
        |limited_reader| {
            dispatch_json_input(
                limited_reader,
                |res| res.map_err(Into::into),
                |deser| {
                    let (records, errs) = parse_serialized_records(deser.into_iter::<T>());
                    errs.is_empty()
                        .then_some(records)
                        .ok_or(JsonDeserError::Json(errs))
                },
            )
        },
        |limit_error| JsonDeserError::Io(limit_error),
    )
}

pub fn check_riferimento_niseci_reader<R: Read, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    T: RecordRiferimentoNISECI,
{
    with_limited_reader(
        reader,
        DefaultByteLimit::MAX_BYTES,
        |limited_reader| {
            dispatch_json_input(
                limited_reader,
                |res| res.map_err(Into::into),
                |deser| {
                    check_serialized_records(deser.into_iter::<T>()).map_err(JsonDeserError::Json)
                },
            )
        },
        |limit_error| JsonDeserError::Io(limit_error),
    )
}

pub fn parse_json_campionamento_niseci<R, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    R: std::io::Read,
    T: RecordCampionamentoNISECI,
{
    with_limited_reader(
        reader,
        DefaultByteLimit::MAX_BYTES,
        |limited_reader| {
            dispatch_json_input(
                limited_reader,
                |res| res.map_err(Into::into),
                |deser| {
                    let (records, errs) = parse_serialized_records(deser.into_iter::<T>());
                    errs.is_empty()
                        .then_some(records)
                        .ok_or(JsonDeserError::Json(errs))
                },
            )
        },
        |limit_error| JsonDeserError::Io(limit_error),
    )
}

pub fn check_campionamento_niseci_reader<R: Read, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    T: RecordCampionamentoNISECI,
{
    with_limited_reader(
        reader,
        DefaultByteLimit::MAX_BYTES,
        |limited_reader| {
            dispatch_json_input(
                limited_reader,
                |res| res.map_err(Into::into),
                |deser| {
                    check_serialized_records(deser.into_iter::<T>()).map_err(JsonDeserError::Json)
                },
            )
        },
        |limit_error| JsonDeserError::Io(limit_error),
    )
}

pub fn parse_json_anagrafica_niseci<R, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    R: std::io::Read,
    T: RecordAnagraficaNISECI,
{
    with_limited_reader(
        reader,
        DefaultByteLimit::MAX_BYTES,
        |limited_reader| {
            dispatch_json_input(
                limited_reader,
                |res| res.map_err(Into::into),
                |deser| {
                    let (records, errs) = parse_serialized_records(deser.into_iter::<T>());
                    errs.is_empty()
                        .then_some(records)
                        .ok_or(JsonDeserError::Json(errs))
                },
            )
        },
        |limit_error| JsonDeserError::Io(limit_error),
    )
}

pub fn check_anagrafica_niseci_reader<R: Read, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    T: RecordAnagraficaNISECI,
{
    with_limited_reader(
        reader,
        DefaultByteLimit::MAX_BYTES,
        |limited_reader| {
            dispatch_json_input(
                limited_reader,
                |res| res.map_err(Into::into),
                |deser| {
                    check_serialized_records(deser.into_iter::<T>()).map_err(JsonDeserError::Json)
                },
            )
        },
        |limit_error| JsonDeserError::Io(limit_error),
    )
}

pub fn check_riferimento_niseci_path<T>(
    path: impl AsRef<Path>,
) -> Result<Vec<T>, JsonPathCheckError>
where
    T: RecordRiferimentoNISECI,
{
    let file = File::open(path)?;
    check_riferimento_niseci_reader(file).map_err(Into::into)
}

pub fn check_campionamento_niseci_path<T>(
    path: impl AsRef<Path>,
) -> Result<Vec<T>, JsonPathCheckError>
where
    T: RecordCampionamentoNISECI,
{
    let file = File::open(path)?;
    check_campionamento_niseci_reader(file).map_err(Into::into)
}

pub fn check_anagrafica_niseci_path<T>(path: impl AsRef<Path>) -> Result<Vec<T>, JsonPathCheckError>
where
    T: RecordAnagraficaNISECI,
{
    let file = File::open(path)?;
    check_anagrafica_niseci_reader(file).map_err(Into::into)
}
