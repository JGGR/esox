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
//! Full NISECI pipeline module.

use std::fmt;
use crate::csv::load::niseci::RiferimentoNISECIError;
use crate::csv::load::niseci::CampionamentoNISECIError;
use crate::csv::load::niseci::AnagraficaNISECIError;
use crate::engines::niseci::full::NISECICalcError;

/// Encodes the possible errors returned by NISECI pipeline.
#[derive(Debug)]
pub enum NISECIError {
    Riferimento(RiferimentoNISECIError),
    Campionamento(CampionamentoNISECIError),
    Anagrafica(AnagraficaNISECIError),
    Calc(NISECICalcError),
}

impl fmt::Display for NISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NISECIError::Riferimento(e) => {
                write!(f, "{e}")
            }
            NISECIError::Campionamento(e) => {
                write!(f, "{e}")
            }
            NISECIError::Anagrafica(e) => {
                write!(f, "{e}")
            }
            NISECIError::Calc(e) => {
                write!(f, "{e}")
            }
        }
    }
}

impl std::error::Error for NISECIError {}
