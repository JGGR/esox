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

use crate::deser::{RecordAnagraficaHFBI, RecordCampionamentoHFBI};
use crate::domain::hfbi::{AnagraficaHFBI, RecordHFBI};
use crate::parser::hfbi::{
    parse_records_anagrafica_hfbi, parse_records_campionamento_hfbi, RecordAnagraficaHFBIError,
    RecordCampionamentoHFBIError,
};

#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using crate::parser::hfbi::RecordCampionamentoHFBIError instead"
)]
pub use crate::parser::hfbi::RecordCampionamentoHFBIError as RecordCsvCampionamentoHFBIError;

#[deprecated(
    note = "v0.2 will change signature to return CampionamentoHFBIParseResult\nConsider using CampionamentoHFBI::parse_records(records).into_parts()"
)]
pub fn parse_recordcsv_campionamento_hfbi<T: RecordCampionamentoHFBI>(
    records: Vec<T>,
) -> (Vec<RecordHFBI>, Vec<RecordCampionamentoHFBIError>) {
    let (camp, errs) = parse_records_campionamento_hfbi::<T>(records).into_parts();
    (camp.into(), errs)
}

#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using crate::parser::hfbi::RecordAnagraficaHFBIError instead"
)]
pub use crate::parser::hfbi::RecordAnagraficaHFBIError as RecordCsvAnagraficaHFBIError;

pub fn parse_recordcsv_anagrafica_hfbi<T: RecordAnagraficaHFBI>(
    records: Vec<T>,
) -> Result<AnagraficaHFBI, Vec<RecordAnagraficaHFBIError>> {
    parse_records_anagrafica_hfbi::<T>(records)
}

#[deprecated(
    note = "v0.2 will change signature to return CampionamentoHFBI on success\nConsider using CampionamentoHFBI::check_records(records)"
)]
pub fn check_records_campionamento_hfbi<T: RecordCampionamentoHFBI>(
    records: Vec<T>,
) -> Result<Vec<RecordHFBI>, Vec<RecordCampionamentoHFBIError>> {
    crate::parser::hfbi::check_records_campionamento_hfbi::<T>(records).map(|v| v.into())
}

pub fn check_records_anagrafica_hfbi<T: RecordAnagraficaHFBI>(
    records: Vec<T>,
) -> Result<AnagraficaHFBI, Vec<RecordAnagraficaHFBIError>> {
    crate::parser::hfbi::check_records_anagrafica_hfbi::<T>(records)
}
