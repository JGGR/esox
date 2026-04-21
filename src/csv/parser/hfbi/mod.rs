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
use crate::parser::hfbi::{parse_records_campionamento_hfbi, CampionamentoHFBIParseResult, parse_records_anagrafica_hfbi,RecordCampionamentoHFBIError,RecordAnagraficaHFBIError};
use crate::domain::hfbi::{
    AnagraficaHFBI, CampionamentoHFBI, RecordHFBI,
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
    note = "v0.2 will change signature to return CampionamentoHFBI on success\nConsider using CampionamentoHFBI::check_recordcsv(records)"
)]
pub fn check_records_campionamento_hfbi<T: RecordCampionamentoHFBI>(
    records: Vec<T>,
) -> Result<Vec<RecordHFBI>, Vec<RecordCampionamentoHFBIError>> {
    check_records_campionamento_hfbi_impl::<T>(records).map(|v| v.into())
}

impl CampionamentoHFBI {
    pub fn parse_records<T>(vec: Vec<T>) -> CampionamentoHFBIParseResult
    where
        T: RecordCampionamentoHFBI,
    {
        CampionamentoHFBIParseResult::parse::<T>(vec)
    }
    pub fn check_recordcsv<T>(vec: Vec<T>) -> Result<Self, Vec<RecordCampionamentoHFBIError>>
    where
        T: RecordCampionamentoHFBI,
    {
        check_records_campionamento_hfbi_impl::<T>(vec)
    }
}

/// v0.2 will have this method public without the _impl suffix
/// Internal transitional API for migrating:
///   - returning CampionamentoHFBI for success over Vec<RecordHFBI>
pub(crate) fn check_records_campionamento_hfbi_impl<T: RecordCampionamentoHFBI>(
    records: Vec<T>,
) -> Result<CampionamentoHFBI, Vec<RecordCampionamentoHFBIError>> {
    let (camp, errors) = parse_records_campionamento_hfbi(records).into_parts();

    println!(
        "Campionamento HFBI: Numero record validi: {}",
        camp.as_vec().len()
    );
    println!(
        "Campionamento HFBI: Numero record non validi: {}",
        errors.len()
    );

    if !errors.is_empty() {
        eprintln!("Errori incontrati durante l'elaborazione dei record per campionamento HFBI: {{");
        //TODO: add process_record_campionamentoNISECI_errors()
        for error in &errors {
            eprintln!("  {}", error);
        }
        eprintln!("}}");
        Err(errors)
    } else {
        //TODO: handle verbosity
        //println!("Tutti i record del campionamento HFBI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        Ok(camp)
    }
}

impl AnagraficaHFBI {
    pub fn parse_records<T>(vec: Vec<T>) -> Result<Self, Vec<RecordAnagraficaHFBIError>>
    where
        T: RecordAnagraficaHFBI,
    {
        parse_recordcsv_anagrafica_hfbi::<T>(vec)
    }
    pub fn check_recordcsv<T>(vec: Vec<T>) -> Result<Self, Vec<RecordAnagraficaHFBIError>>
    where
        T: RecordAnagraficaHFBI,
    {
        check_records_anagrafica_hfbi::<T>(vec)
    }
}

pub fn check_records_anagrafica_hfbi<T: RecordAnagraficaHFBI>(
    records: Vec<T>,
) -> Result<AnagraficaHFBI, Vec<RecordAnagraficaHFBIError>> {
    let res = parse_recordcsv_anagrafica_hfbi(records);

    match res {
        Ok(anagrafica) => {
            println!("Anagrafica HFBI: {}", anagrafica);
            //TODO: handle verbosity
            //println!("Tutti i record dell'anagrafica HFBI sono stati processati con successo!");
            /*
            for record in &records {
                println!("  Record: {{{record}}}");
            }
            */
            Ok(anagrafica)
        }
        Err(errors) => {
            println!(
                "Anagrafica HFBI: Numero record non validi: {}",
                errors.len()
            );
            eprintln!(
                "Errori incontrati durante l'elaborazione dei record per anagrafica HFBI: {{"
            );
            //TODO: add process_record_anagraficaHFBI_errors()
            for error in &errors {
                eprintln!("  {}", error);
            }
            eprintln!("}}");
            Err(errors)
        }
    }
}
