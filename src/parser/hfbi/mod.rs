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

use std::fmt;

#[derive(Debug)]
pub enum RecordCampionamentoHFBIError {
    ValoreInvalido { msg: String }, //TODO: add position?
}

impl fmt::Display for RecordCampionamentoHFBIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            RecordCampionamentoHFBIError::ValoreInvalido { msg } => {
                format!("Errore record campionamento HFBI: {}", msg)
            }
        };
        write!(f, "{}", string_representation)
    }
}

impl std::error::Error for RecordCampionamentoHFBIError {}

#[derive(Debug)]
pub enum RecordAnagraficaHFBIError {
    ValoreInvalido { msg: String }, //TODO: add position?
}

impl fmt::Display for RecordAnagraficaHFBIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            RecordAnagraficaHFBIError::ValoreInvalido { msg } => {
                format!("Errore record anagrafica HFBI: {}", msg)
            }
        };
        write!(f, "{}", string_representation)
    }
}

impl std::error::Error for RecordAnagraficaHFBIError {}
