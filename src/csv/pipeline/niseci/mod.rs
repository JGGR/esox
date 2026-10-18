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
//!
//! Builds and calculates a NISECI index from CSV inputs.

//! ## Example usage
//!
//! ```rust
//! use std::path::Path;
//! use std::io::{Cursor, Read};
//! use std::fs::File;
//! use esox::csv::pipeline::niseci::{NISECIBuilder, NISECIError};
//! use esox::csv::load::CsvLoadConfig;
//! use esox::csv::load::InputFormat;
//! use esox::domain::niseci::RisultatoNISECI;
//! const CAMPIONAMENTO_DATA: &[u8] =
//! include_bytes!("../../../../templates/campionamento_niseci.csv");
//!
//! const CONFIG: CsvLoadConfig = CsvLoadConfig::Format {
//!     has_headers: true,
//!     format: InputFormat::Alternative,
//! };
//!
//! let risultato: Result<RisultatoNISECI, NISECIError> = NISECIBuilder::<File, Cursor<&[u8]>, File>::new(CONFIG)
//!     .riferimento_path(Path::new("./templates/riferimento_niseci.csv"))
//!     .campionamento_reader(Cursor::new(CAMPIONAMENTO_DATA))
//!     .anagrafica_path(Path::new("./templates/anagrafica_niseci.csv"))
//!     .process();
//! assert!(risultato.is_ok());
//! ```

use crate::csv::load::niseci::{
    load_anagrafica, load_campionamento, load_riferimento, AnagraficaNISECIError,
    CampionamentoNISECIError, RiferimentoNISECIError,
};
use crate::csv::load::{CsvInput, CsvLoadConfig};
use crate::deser::{
    PlainRecordAnagraficaNISECI, PlainRecordCampionamentoNISECI, PlainRecordRiferimentoNISECI,
    RecordAnagraficaNISECI, RecordCampionamentoNISECI, RecordRiferimentoNISECI,
};
use crate::domain::niseci::RisultatoNISECI;
use crate::engines::niseci::full::{calculate_niseci, calculate_rqe_niseci, NISECICalcError};
use std::fmt;
use std::io::Read;
use std::path::Path;

/// Encodes the possible errors returned by NISECI pipeline.
#[derive(Debug)]
pub enum NISECIError {
    MissingInput(String),
    Riferimento(RiferimentoNISECIError),
    Campionamento(CampionamentoNISECIError),
    Anagrafica(AnagraficaNISECIError),
    Calc(Vec<NISECICalcError>),
}

impl fmt::Display for NISECIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NISECIError::MissingInput(e) => {
                write!(f, "{e}")
            }
            NISECIError::Riferimento(e) => {
                write!(f, "{e}")
            }
            NISECIError::Campionamento(e) => {
                write!(f, "{e}")
            }
            NISECIError::Anagrafica(e) => {
                write!(f, "{e}")
            }
            NISECIError::Calc(_) => {
                write!(f, "Calc error")
            }
        }
    }
}

impl std::error::Error for NISECIError {}

pub fn process<'a, RR, RC, RA, TRif, TCamp, TAnag>(
    riferimento_source: CsvInput<'a, RR>,
    campionamento_source: CsvInput<'a, RC>,
    anagrafica_source: CsvInput<'a, RA>,
    config: CsvLoadConfig,
) -> Result<RisultatoNISECI, NISECIError>
where
    RR: Read,
    RC: Read,
    RA: Read,
    TRif: RecordRiferimentoNISECI,
    TCamp: RecordCampionamentoNISECI,
    TAnag: RecordAnagraficaNISECI,
{
    let riferimento;
    match load_riferimento::<RR, TRif>(riferimento_source, &config) {
        Ok(res) => {
            riferimento = res;
        }
        Err(err) => {
            return Err(NISECIError::Riferimento(err));
        }
    }
    let campionamento;
    match load_campionamento::<RC, TCamp>(&riferimento, campionamento_source, &config) {
        Ok(res) => {
            campionamento = res;
        }
        Err(err) => {
            return Err(NISECIError::Campionamento(err));
        }
    }
    let anagrafica;
    match load_anagrafica::<RA, TAnag>(anagrafica_source, &config) {
        Ok(res) => {
            anagrafica = res;
        }
        Err(err) => {
            return Err(NISECIError::Anagrafica(err));
        }
    }
    let niseci_res;
    match calculate_niseci(&campionamento, &riferimento, &anagrafica) {
        Ok(res) => {
            niseci_res = res;
        }
        Err(err) => {
            return Err(NISECIError::Calc(err));
        }
    }
    let (niseci, intermediates) = niseci_res;
    let rqe_niseci = calculate_rqe_niseci(niseci);
    return Ok(RisultatoNISECI::new(niseci, rqe_niseci, intermediates));
}

pub struct NISECIBuilder<
    'a,
    RR,
    RC,
    RA,
    TRif = PlainRecordRiferimentoNISECI,
    TCamp = PlainRecordCampionamentoNISECI,
    TAnag = PlainRecordAnagraficaNISECI,
> {
    riferimento: Option<CsvInput<'a, RR>>,
    campionamento: Option<CsvInput<'a, RC>>,
    anagrafica: Option<CsvInput<'a, RA>>,
    config: CsvLoadConfig,
    _records: std::marker::PhantomData<(TRif, TCamp, TAnag)>,
}

impl<
        'a,
        RR: Read,
        RC: Read,
        RA: Read,
        TRif: RecordRiferimentoNISECI,
        TCamp: RecordCampionamentoNISECI,
        TAnag: RecordAnagraficaNISECI,
    > NISECIBuilder<'a, RR, RC, RA, TRif, TCamp, TAnag>
{
    pub fn new(config: CsvLoadConfig) -> Self {
        Self {
            riferimento: None,
            campionamento: None,
            anagrafica: None,
            config,
            _records: std::marker::PhantomData::<(TRif, TCamp, TAnag)>,
        }
    }
    pub fn with_records<TR, TC, TA>(
        config: CsvLoadConfig,
    ) -> NISECIBuilder<'a, RR, RC, RA, TR, TC, TA>
    where
        TR: RecordRiferimentoNISECI,
        TC: RecordCampionamentoNISECI,
        TA: RecordAnagraficaNISECI,
    {
        NISECIBuilder::<RR, RC, RA, TR, TC, TA> {
            riferimento: None,
            campionamento: None,
            anagrafica: None,
            config,
            _records: std::marker::PhantomData,
        }
    }
    pub fn riferimento_path(mut self, path: &'a Path) -> Self {
        self.riferimento = Some(CsvInput::Path(path));
        self
    }
    pub fn riferimento_reader(mut self, reader: RR) -> Self {
        self.riferimento = Some(CsvInput::Reader(reader));
        self
    }

    pub fn campionamento_path(mut self, path: &'a Path) -> Self {
        self.campionamento = Some(CsvInput::Path(path));
        self
    }

    pub fn campionamento_reader(mut self, reader: RC) -> Self {
        self.campionamento = Some(CsvInput::Reader(reader));
        self
    }

    pub fn anagrafica_path(mut self, path: &'a Path) -> Self {
        self.anagrafica = Some(CsvInput::Path(path));
        self
    }

    pub fn anagrafica_reader(mut self, reader: RA) -> Self {
        self.anagrafica = Some(CsvInput::Reader(reader));
        self
    }
    pub fn process(self) -> Result<RisultatoNISECI, NISECIError> {
        let riferimento = self
            .riferimento
            .ok_or(NISECIError::MissingInput("riferimento".to_string()))?;

        let campionamento = self
            .campionamento
            .ok_or(NISECIError::MissingInput("campionamento".to_string()))?;

        let anagrafica = self
            .anagrafica
            .ok_or(NISECIError::MissingInput("anagrafica".to_string()))?;

        process::<RR, RC, RA, TRif, TCamp, TAnag>(
            riferimento,
            campionamento,
            anagrafica,
            self.config,
        )
    }
}
