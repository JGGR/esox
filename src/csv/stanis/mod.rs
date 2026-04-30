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
//! Dedicated deserialization module for semicolon delimited CSV.
//!
//! This module provides support for the alternative CSV input format.
//! The alterative format comes from Excel behaviour with the Italian locale.
//!
//! By default, Excel exports CSV files with:
//! - Semicolon `;` as csv field delimiter
//! - Comma `,` as float decimal delimiter

pub mod hfbi;
pub mod niseci;
use crate::csv::{
    ANAGRAFICA_HFBI_HEADER_FIELDS, ANAGRAFICA_NISECI_HEADER_FIELDS,
    CAMPIONAMENTO_HFBI_HEADER_FIELDS, CAMPIONAMENTO_NISECI_HEADER_FIELDS,
    RIFERIMENTO_NISECI_HEADER_FIELDS,
};
use crate::deser::TipoRecord;
use serde::{de, Deserialize, Deserializer};

fn deserialize_comma_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let s = s.replace(',', "."); // Replace comma with dot
    s.parse::<f32>().map_err(de::Error::custom)
}

pub(crate) fn field_name(record: TipoRecord, idx: usize) -> String {
    match record {
        TipoRecord::RiferimentoNISECI => RIFERIMENTO_NISECI_HEADER_FIELDS
            .get(idx)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "???".to_string()),

        TipoRecord::CampionamentoNISECI => CAMPIONAMENTO_NISECI_HEADER_FIELDS
            .get(idx)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "???".to_string()),

        TipoRecord::AnagraficaNISECI => ANAGRAFICA_NISECI_HEADER_FIELDS
            .get(idx)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "???".to_string()),

        TipoRecord::CampionamentoHFBI => CAMPIONAMENTO_HFBI_HEADER_FIELDS
            .get(idx)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "???".to_string()),

        TipoRecord::AnagraficaHFBI => ANAGRAFICA_HFBI_HEADER_FIELDS
            .get(idx)
            .map(|s| s.to_string())
            .unwrap_or_else(|| "???".to_string()),
    }
}
