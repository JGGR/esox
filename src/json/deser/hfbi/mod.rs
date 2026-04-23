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
    parse_serialized_records, validate_serialized_records, RecordAnagraficaHFBI,
    RecordCampionamentoHFBI,
};
use crate::json::deser::{dispatch_json_input, JsonDeserError, JsonPathCheckError};
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub fn parse_json_campionamento_hfbi<R, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    R: std::io::Read,
    T: RecordCampionamentoHFBI,
{
    dispatch_json_input(
        reader,
        |res| res.map_err(Into::into),
        |deser| {
            let (records, errs) = parse_serialized_records(deser.into_iter::<T>());
            errs.is_empty()
                .then_some(records)
                .ok_or(JsonDeserError::Json(errs))
        },
    )
}

pub fn check_campionamento_hfbi_reader<R: Read, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    T: RecordCampionamentoHFBI,
{
    dispatch_json_input(
        reader,
        |res| res.map_err(Into::into),
        |deser| {
            validate_serialized_records(deser.into_iter::<T>(), |errs| {
                errs.iter().for_each(|e| eprintln!("  {}", e));
            })
            .map_err(JsonDeserError::Json)
        },
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
    dispatch_json_input(
        reader,
        |res| res.map_err(Into::into),
        |deser| {
            let (records, errs) = parse_serialized_records(deser.into_iter::<T>());
            errs.is_empty()
                .then_some(records)
                .ok_or(JsonDeserError::Json(errs))
        },
    )
}

pub fn check_anagrafica_hfbi_reader<R: Read, T>(reader: R) -> Result<Vec<T>, JsonDeserError>
where
    T: RecordAnagraficaHFBI,
{
    dispatch_json_input(
        reader,
        |res| res.map_err(Into::into),
        |deser| {
            validate_serialized_records(deser.into_iter::<T>(), |errs| {
                errs.iter().for_each(|e| eprintln!("  {}", e));
            })
            .map_err(JsonDeserError::Json)
        },
    )
}

pub fn check_anagrafica_hfbi_path<T>(path: impl AsRef<Path>) -> Result<Vec<T>, JsonPathCheckError>
where
    T: RecordAnagraficaHFBI,
{
    let file = File::open(path)?;
    check_anagrafica_hfbi_reader(file).map_err(Into::into)
}
