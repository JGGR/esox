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

pub mod hfbi;
pub mod niseci;

use crate::deser::limits::ByteLimit;

pub enum JsonDispatchError {
    Io(std::io::Error),
    JsonArray(serde_json::Error),
}

#[derive(Debug)]
pub enum JsonDeserError {
    Io(std::io::Error),
    Json(Vec<serde_json::Error>),
    JsonArray(serde_json::Error),
}

impl From<JsonDispatchError> for JsonDeserError {
    fn from(err: JsonDispatchError) -> Self {
        match err {
            JsonDispatchError::Io(e) => JsonDeserError::Io(e),
            JsonDispatchError::JsonArray(errs) => JsonDeserError::JsonArray(errs),
        }
    }
}

pub enum JsonPathCheckError {
    Io(std::io::Error),
    DeserIo(std::io::Error),
    Json(Vec<serde_json::Error>),
    JsonArray(serde_json::Error),
}

impl From<std::io::Error> for JsonPathCheckError {
    fn from(err: std::io::Error) -> Self {
        JsonPathCheckError::Io(err)
    }
}

impl From<JsonDeserError> for JsonPathCheckError {
    fn from(err: JsonDeserError) -> Self {
        match err {
            JsonDeserError::Json(errs) => JsonPathCheckError::Json(errs),
            JsonDeserError::JsonArray(err) => JsonPathCheckError::JsonArray(err),
            JsonDeserError::Io(e) => JsonPathCheckError::DeserIo(e),
        }
    }
}

use serde::de::DeserializeOwned;
use serde_json::Deserializer;
use std::io::{BufRead, BufReader, Read};

/// Dispatches JSON input to either a bulk array handler or a streaming handler.
///
/// This helper inspects the first non-whitespace byte of the input to determine
/// whether it represents a top-level JSON array (`[...]`) or a sequence of
/// whitespace-separated JSON values.
///
/// If the input begins with `[`, the entire stream is deserialized into
/// `Vec<T>` and passed to `array_fn`. Otherwise, a `serde_json::Deserializer`
/// is constructed and passed to `stream_fn` for incremental processing.
///
/// I/O errors encountered while peeking are converted into `serde_json::Error`
/// and routed through `array_fn`.
///
/// This function is intended for internal use in implementing APIs that accept
/// both batched and streaming JSON inputs.
fn dispatch_json_input<R, BL, T, FArray, FStream, Out>(
    reader: R,
    array_fn: FArray,
    stream_fn: FStream,
) -> Out
where
    R: Read,
    BL: ByteLimit,
    T: DeserializeOwned,
    FArray: FnOnce(Result<Vec<T>, JsonDispatchError>) -> Out,
    FStream: FnOnce(Deserializer<serde_json::de::IoRead<std::io::Take<BufReader<R>>>>) -> Out,
{
    let mut reader = BufReader::new(reader).take(BL::MAX_BYTES);
    let peek = match reader.fill_buf() {
        Ok(buf) => buf,
        Err(e) => {
            return array_fn(Err(JsonDispatchError::Io(e)));
        }
    };

    let first = peek.iter().copied().find(|b| !b.is_ascii_whitespace());

    match first {
        Some(b'[') => {
            let res = serde_json::from_reader::<_, Vec<T>>(reader)
                .map_err(|e| JsonDispatchError::JsonArray(e));
            array_fn(res)
        }
        _ => {
            let iter = Deserializer::from_reader(reader);
            stream_fn(iter)
        }
    }
}
