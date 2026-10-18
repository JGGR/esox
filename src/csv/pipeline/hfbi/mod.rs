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
//! Full HFBI pipeline module.
//!
//! Builds and calculates a HFBI index from CSV inputs.
//!
//! ## Example usage
//!
//! ```rust
//! use std::path::Path;
//! use std::io::{Cursor, Read};
//! use std::fs::File;
//! use esox::csv::pipeline::hfbi::{HFBIBuilder, HFBIError};
//! use esox::csv::stanis::hfbi::{VeryItalianRecordCampionamentoHFBI,
//! VeryItalianRecordAnagraficaHFBI};
//! use esox::csv::load::CsvLoadConfig;
//! use esox::csv::load::InputFormat;
//! use esox::csv::deser::CsvConfig;
//! use esox::domain::hfbi::RisultatoHFBI;
//! const CAMPIONAMENTO_DATA: &[u8] =
//! include_bytes!("../../../../templates/campionamento_hfbi.csv");
//!
//! let config: CsvLoadConfig = CsvLoadConfig::Csv (
//!     CsvConfig::default()
//! );
//!
//! let risultato: Result<RisultatoHFBI, HFBIError> =
//! HFBIBuilder::<Cursor<&[u8]>, File>::with_records::<VeryItalianRecordCampionamentoHFBI, VeryItalianRecordAnagraficaHFBI>(config)
//!     .campionamento_reader(Cursor::new(CAMPIONAMENTO_DATA))
//!     .anagrafica_path(Path::new("./templates/anagrafica_hfbi.csv"))
//!     .process();
//! assert!(risultato.is_ok());
//! ```

use crate::csv::load::hfbi::{
    load_anagrafica, load_campionamento, AnagraficaHFBIError, CampionamentoHFBIError,
};
use crate::csv::load::{CsvInput, CsvLoadConfig};
use crate::deser::{
    PlainRecordAnagraficaHFBI, PlainRecordCampionamentoHFBI, RecordAnagraficaHFBI,
    RecordCampionamentoHFBI,
};
use crate::domain::hfbi::RisultatoHFBI;
use crate::engines::hfbi::full::{calculate_hfbi, HFBICalcError};
use std::fmt;
use std::io::Read;
use std::path::Path;

/// Encodes the possible errors returned by HFBI pipeline.
#[derive(Debug)]
pub enum HFBIError {
    MissingInput(String),
    Campionamento(CampionamentoHFBIError),
    Anagrafica(AnagraficaHFBIError),
    Calc(HFBICalcError),
}

impl fmt::Display for HFBIError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HFBIError::MissingInput(e) => {
                write!(f, "{e}")
            }
            HFBIError::Campionamento(e) => {
                write!(f, "{e}")
            }
            HFBIError::Anagrafica(e) => {
                write!(f, "{e}")
            }
            HFBIError::Calc(e) => {
                write!(f, "{e}")
            }
        }
    }
}

impl std::error::Error for HFBIError {}

pub fn process<'a, RC, RA, TCamp, TAnag>(
    campionamento_source: CsvInput<'a, RC>,
    anagrafica_source: CsvInput<'a, RA>,
    config: CsvLoadConfig,
) -> Result<RisultatoHFBI, HFBIError>
where
    RC: Read,
    RA: Read,
    TCamp: RecordCampionamentoHFBI,
    TAnag: RecordAnagraficaHFBI,
{
    let campionamento;
    match load_campionamento::<RC, TCamp>(campionamento_source, &config) {
        Ok(res) => {
            campionamento = res;
        }
        Err(err) => {
            return Err(HFBIError::Campionamento(err));
        }
    }
    let anagrafica;
    match load_anagrafica::<RA, TAnag>(anagrafica_source, &config) {
        Ok(res) => {
            anagrafica = res;
        }
        Err(err) => {
            return Err(HFBIError::Anagrafica(err));
        }
    }
    let hfbi_res;
    match calculate_hfbi(&campionamento, &anagrafica) {
        Ok(res) => {
            hfbi_res = res;
        }
        Err(err) => {
            return Err(HFBIError::Calc(err));
        }
    }
    let (hfbi, intermediates) = hfbi_res;
    return Ok(RisultatoHFBI::new(Some(hfbi), intermediates));
}

pub struct HFBIBuilder<
    'a,
    RC,
    RA,
    TCamp = PlainRecordCampionamentoHFBI,
    TAnag = PlainRecordAnagraficaHFBI,
> {
    campionamento: Option<CsvInput<'a, RC>>,
    anagrafica: Option<CsvInput<'a, RA>>,
    config: CsvLoadConfig,
    _records: std::marker::PhantomData<(TCamp, TAnag)>,
}

impl<'a, RC: Read, RA: Read, TCamp: RecordCampionamentoHFBI, TAnag: RecordAnagraficaHFBI>
    HFBIBuilder<'a, RC, RA, TCamp, TAnag>
{
    pub fn new(config: CsvLoadConfig) -> Self {
        Self {
            campionamento: None,
            anagrafica: None,
            config,
            _records: std::marker::PhantomData::<(TCamp, TAnag)>,
        }
    }
    pub fn with_records<TC, TA>(config: CsvLoadConfig) -> HFBIBuilder<'a, RC, RA, TC, TA>
    where
        TC: RecordCampionamentoHFBI,
        TA: RecordAnagraficaHFBI,
    {
        HFBIBuilder::<RC, RA, TC, TA> {
            campionamento: None,
            anagrafica: None,
            config,
            _records: std::marker::PhantomData,
        }
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
    pub fn process(self) -> Result<RisultatoHFBI, HFBIError> {
        let campionamento = self
            .campionamento
            .ok_or(HFBIError::MissingInput("campionamento".to_string()))?;

        let anagrafica = self
            .anagrafica
            .ok_or(HFBIError::MissingInput("anagrafica".to_string()))?;

        process::<RC, RA, TCamp, TAnag>(campionamento, anagrafica, self.config)
    }
}
