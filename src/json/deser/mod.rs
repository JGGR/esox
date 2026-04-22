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

pub enum JsonCheckError {
    Io(std::io::Error),
    Json(Vec<serde_json::Error>),
}

impl From<std::io::Error> for JsonCheckError {
    fn from(err: std::io::Error) -> Self {
        JsonCheckError::Io(err)
    }
}

use serde::de::DeserializeOwned;
use serde_json::Deserializer;
use std::io::{BufRead, BufReader, Read};

fn dispatch_json_input<R, T, FArray, FStream, Out>(
    reader: R,
    array_fn: FArray,
    stream_fn: FStream,
) -> Out
where
    R: Read,
    T: DeserializeOwned,
    FArray: FnOnce(Result<Vec<T>, serde_json::Error>) -> Out,
    FStream: FnOnce(Deserializer<serde_json::de::IoRead<BufReader<R>>>) -> Out,
{
    let mut reader = BufReader::new(reader);
    let peek = match reader.fill_buf() {
        Ok(buf) => buf,
        Err(e) => {
            // turn IO failure into serde's error domain
            let err = serde_json::Error::io(e);
            return array_fn(Err(err));
        }
    };

    let first = peek.iter().copied().find(|b| !b.is_ascii_whitespace());

    match first {
        Some(b'[') => {
            let res = serde_json::from_reader::<_, Vec<T>>(reader);
            array_fn(res)
        }
        _ => {
            let iter = Deserializer::from_reader(reader);
            stream_fn(iter)
        }
    }
}
