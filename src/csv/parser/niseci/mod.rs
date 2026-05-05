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

use crate::deser::{RecordAnagraficaNISECI, RecordCampionamentoNISECI, RecordRiferimentoNISECI};
use crate::domain::niseci::{AnagraficaNISECI, RecordNISECI, RiferimentoNISECI, SpecieNISECI};
use crate::parser::niseci::{
    parse_records_anagrafica_niseci, parse_records_campionamento_niseci,
    parse_records_riferimento_niseci, RecordAnagraficaNISECIError, RecordCampionamentoNISECIError,
    RecordRiferimentoNISECIError,
};

#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using crate::parser::niseci::RecordRiferimentoNISECIError instead"
)]
pub use crate::parser::niseci::RecordRiferimentoNISECIError as RecordCsvRiferimentoNISECIError;

#[deprecated(
    note = "v0.2 will drop visibility.\nConsider using RiferimentoNISECI::parse_records(records).into_parts()"
)]
pub fn parse_recordcsv_riferimento_niseci<T: RecordRiferimentoNISECI>(
    records: Vec<T>,
) -> (Vec<SpecieNISECI>, Vec<RecordRiferimentoNISECIError>) {
    let (rif, errs) = parse_records_riferimento_niseci::<T>(records).into_parts();
    (rif.into(), errs)
}

#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using crate::parser::niseci::RecordCampionamentoNISECIError instead"
)]
pub use crate::parser::niseci::RecordCampionamentoNISECIError as RecordCsvCampionamentoNISECIError;

#[deprecated(
    note = "v0.2 will drop visibility.\n  Consider using CampionamentoNISECI::parse_records(records, riferimento).into_parts()"
)]
pub fn parse_recordcsv_campionamento_niseci<T: RecordCampionamentoNISECI>(
    records: Vec<T>,
    riferimento_specie: Vec<SpecieNISECI>,
) -> (Vec<RecordNISECI>, Vec<RecordCampionamentoNISECIError>) {
    let (camp, errs) = parse_records_campionamento_niseci::<T>(
        records,
        &RiferimentoNISECI::new_from_map(riferimento_specie.into()),
    )
    .into_parts();
    (camp.into(), errs)
}

#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using crate::parser::niseci::RecordAnagraficaNISECIError instead"
)]
pub use crate::parser::niseci::RecordAnagraficaNISECIError as RecordCsvAnagraficaNISECIError;

#[deprecated(
    note = "v0.2 will drop visibility.\nConsider using AnagraficaNISECI::parse_records(records)"
)]
pub fn parse_recordcsv_anagrafica_niseci<T: RecordAnagraficaNISECI>(
    records: Vec<T>,
) -> Result<AnagraficaNISECI, Vec<RecordAnagraficaNISECIError>> {
    parse_records_anagrafica_niseci::<T>(records)
}

#[deprecated(
    note = "v0.2 will drop visibility.\nConsider using RiferimentoNISECI::check_records(records)"
)]
pub fn check_records_riferimento_niseci<T: RecordRiferimentoNISECI>(
    records: Vec<T>,
) -> Result<Vec<SpecieNISECI>, Vec<RecordRiferimentoNISECIError>> {
    crate::parser::niseci::check_records_riferimento_niseci::<T>(records).map(|v| v.into())
}

#[deprecated(
    note = "v0.2 will drop visibility.\nConsider using CampionamentoNISECI::check_records(records, &riferimento)"
)]
pub fn check_records_campionamento_niseci<T: RecordCampionamentoNISECI>(
    records: Vec<T>,
    riferimento_specie: Vec<SpecieNISECI>,
) -> Result<Vec<RecordNISECI>, Vec<RecordCampionamentoNISECIError>> {
    crate::parser::niseci::check_records_campionamento_niseci::<T>(
        records,
        &RiferimentoNISECI::new_from_map(riferimento_specie.into()),
    )
    .map(|v| v.into())
}

#[deprecated(
    note = "v0.2 will drop visibility.\nConsider using AnagraficaNISECI::check_records(records)"
)]
pub fn check_records_anagrafica_niseci<T: RecordAnagraficaNISECI>(
    records: Vec<T>,
) -> Result<AnagraficaNISECI, Vec<RecordAnagraficaNISECIError>> {
    crate::parser::niseci::check_records_anagrafica_niseci::<T>(records)
}
