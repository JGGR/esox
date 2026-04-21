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
pub enum RecordRiferimentoNISECIError {
    ValoreInvalido { msg: String }, //TODO: add position?
    SoglieCLNonCrescenti { msg: String },
    SoglieADJUVNonCrescenti { msg: String },
}

impl fmt::Display for RecordRiferimentoNISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            RecordRiferimentoNISECIError::ValoreInvalido { msg } => {
                format!("Errore record riferimento NISECI: {}", msg)
            }
            RecordRiferimentoNISECIError::SoglieCLNonCrescenti { msg } => {
                format!("Errore record riferimento NISECI: {}", msg)
            }
            RecordRiferimentoNISECIError::SoglieADJUVNonCrescenti { msg } => {
                format!("Errore record riferimento NISECI: {}", msg)
            }
        };
        write!(f, "{}", string_representation)
    }
}

impl std::error::Error for RecordRiferimentoNISECIError {}

#[derive(Debug)]
pub enum RecordCampionamentoNISECIError {
    ValoreInvalido { msg: String }, //TODO: add position?
}

impl fmt::Display for RecordCampionamentoNISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            RecordCampionamentoNISECIError::ValoreInvalido { msg } => {
                format!("Errore record campionamento NISECI: {}", msg)
            }
        };
        write!(f, "{}", string_representation)
    }
}

impl std::error::Error for RecordCampionamentoNISECIError {}

#[derive(Debug)]
pub enum RecordAnagraficaNISECIError {
    ValoreInvalido { msg: String }, //TODO: add position?
}

impl fmt::Display for RecordAnagraficaNISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self {
            RecordAnagraficaNISECIError::ValoreInvalido { msg } => {
                format!("Errore record anagrafica NISECI: {}", msg)
            }
        };
        write!(f, "{}", string_representation)
    }
}

impl std::error::Error for RecordAnagraficaNISECIError {}
