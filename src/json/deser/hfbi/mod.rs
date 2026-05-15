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
/// v0.2 will drop implicit logging, hence this method will not be needed anymore.
/// Callsites will switch to crate::deser::check_serialized_records.
/// Usercode will need to handle the format/printing of errors separately.
#[allow(deprecated)]
use crate::deser::validate_serialized_records;

use crate::deser::limits::{with_limited_reader, ByteLimit, DefaultByteLimit};
use crate::deser::{parse_serialized_records, RecordAnagraficaHFBI, RecordCampionamentoHFBI};
use crate::json::deser::{dispatch_json_input, JsonDeserError, JsonPathCheckError};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn parse_json_campionamento_hfbi<R, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    R: std::io::Read,
    T: RecordCampionamentoHFBI,
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

pub fn check_campionamento_hfbi_reader<R: Read, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    T: RecordCampionamentoHFBI,
{
    with_limited_reader(
        reader,
        DefaultByteLimit::MAX_BYTES,
        |limited_reader| {
            dispatch_json_input(
                limited_reader,
                |res| res.map_err(Into::into),
                |deser| {
                    #[allow(deprecated)]
                    validate_serialized_records(deser.into_iter::<T>(), |errs| {
                        errs.iter().for_each(|e| eprintln!("  {}", e));
                    })
                    .map_err(JsonDeserError::Json)
                },
            )
        },
        |limit_error| JsonDeserError::Io(limit_error),
    )
}

pub fn check_campionamento_hfbi_path<T>(
    path: impl AsRef<Path>,
) -> Result<Vec<T>, JsonPathCheckError>
where
    T: RecordCampionamentoHFBI,
{
    let file = File::open(path)?;
    check_campionamento_hfbi_reader(file).map_err(Into::into)
}

pub fn parse_json_anagrafica_hfbi<R, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    R: std::io::Read,
    T: RecordAnagraficaHFBI,
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

pub fn check_anagrafica_hfbi_reader<R: Read, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    T: RecordAnagraficaHFBI,
{
    with_limited_reader(
        reader,
        DefaultByteLimit::MAX_BYTES,
        |limited_reader| {
            dispatch_json_input(
                limited_reader,
                |res| res.map_err(Into::into),
                |deser| {
                    #[allow(deprecated)]
                    validate_serialized_records(deser.into_iter::<T>(), |errs| {
                        errs.iter().for_each(|e| eprintln!("  {}", e));
                    })
                    .map_err(JsonDeserError::Json)
                },
            )
        },
        |limit_error| JsonDeserError::Io(limit_error),
    )
}

pub fn check_anagrafica_hfbi_path<T>(path: impl AsRef<Path>) -> Result<Vec<T>, JsonPathCheckError>
where
    T: RecordAnagraficaHFBI,
{
    let file = File::open(path)?;
    check_anagrafica_hfbi_reader(file).map_err(Into::into)
}
