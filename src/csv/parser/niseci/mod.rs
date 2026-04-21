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
use crate::domain::niseci::{
    AnagraficaNISECI, CampionamentoNISECI, RecordNISECI, RiferimentoNISECI, SpecieNISECI,
};
use crate::parser::niseci::{
    parse_records_anagrafica_niseci, parse_records_campionamento_niseci,
    parse_records_riferimento_niseci, CampionamentoNISECIParseResult, RecordAnagraficaNISECIError,
    RecordCampionamentoNISECIError, RecordRiferimentoNISECIError, RiferimentoNISECIParseResult,
};

#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using crate::parser::niseci::RecordRiferimentoNISECIError instead"
)]
pub use crate::parser::niseci::RecordRiferimentoNISECIError as RecordCsvRiferimentoNISECIError;

#[deprecated(
    note = "v0.2 will change signature to return RiferimentoNISECIParseResult\nConsider using RiferimentoNISECI::parse_records(records).into_parts()"
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
    note = "v0.2 will change signature to:\n  - expect riferimento_specie as &RiferimentoNISECI\n  - return CampionamentoNISECIParseResult\n  Consider using CampionamentoNISECI::parse_records(records, riferimento).into_parts()"
)]
pub fn parse_recordcsv_campionamento_niseci<T: RecordCampionamentoNISECI>(
    records: Vec<T>,
    riferimento_specie: Vec<SpecieNISECI>,
) -> (Vec<RecordNISECI>, Vec<RecordCampionamentoNISECIError>) {
    let (camp, errs) = parse_records_campionamento_niseci::<T>(
        records,
        &RiferimentoNISECI::new(riferimento_specie),
    )
    .into_parts();
    (camp.into(), errs)
}

#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using crate::parser::niseci::RecordAnagraficaNISECIError instead"
)]
pub use crate::parser::niseci::RecordAnagraficaNISECIError as RecordCsvAnagraficaNISECIError;

pub fn parse_recordcsv_anagrafica_niseci<T: RecordAnagraficaNISECI>(
    records: Vec<T>,
) -> Result<AnagraficaNISECI, Vec<RecordAnagraficaNISECIError>> {
    parse_records_anagrafica_niseci::<T>(records)
}

#[deprecated(
    note = "v0.2 will change signature to return RiferimentoNISECI on success\nConsider using RiferimentoNISECI::check_recordcsv(records)"
)]
pub fn check_records_riferimento_niseci<T: RecordRiferimentoNISECI>(
    records: Vec<T>,
) -> Result<Vec<SpecieNISECI>, Vec<RecordRiferimentoNISECIError>> {
    check_records_riferimento_niseci_impl::<T>(records).map(|v| v.into())
}

impl RiferimentoNISECI {
    pub fn parse_records<T>(vec: Vec<T>) -> RiferimentoNISECIParseResult
    where
        T: RecordRiferimentoNISECI,
    {
        RiferimentoNISECIParseResult::parse::<T>(vec)
    }
    pub fn check_recordcsv<T>(vec: Vec<T>) -> Result<Self, Vec<RecordRiferimentoNISECIError>>
    where
        T: RecordRiferimentoNISECI,
    {
        check_records_riferimento_niseci_impl::<T>(vec)
    }
}

/// v0.2 will have this method public without the _impl suffix
/// Internal transitional API for migrating:
///   - returning RiferimentoNISECI for success over Vec<SpecieNISECI>
pub(crate) fn check_records_riferimento_niseci_impl<T: RecordRiferimentoNISECI>(
    records: Vec<T>,
) -> Result<RiferimentoNISECI, Vec<RecordRiferimentoNISECIError>> {
    let (rif, errors) = parse_records_riferimento_niseci(records).into_parts();

    println!(
        "Riferimento NISECI: Numero record validi: {}",
        rif.as_vec().len()
    );
    println!(
        "Riferimento NISECI: Numero record non validi: {}",
        errors.len()
    );

    if !errors.is_empty() {
        eprintln!("Errori incontrati durante l'elaborazione dei record per riferimento NISECI: {{");
        //TODO: add process_record_riferimentoNISECI_errors()
        for error in &errors {
            eprintln!("  {}", error);
        }
        eprintln!("}}");
        Err(errors)
    } else {
        //TODO: handle verbosity
        //println!("Tutti i record del riferimento NISECI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        Ok(rif)
    }
}

#[deprecated(
    note = "v0.2 will change signature to:\n  - expect riferimento_specie as &RiferimentoNISECI\n  - return CampionamentoNISECI on success\nConsider using CampionamentoNISECI::check_recordcsv(records, &riferimento)"
)]
pub fn check_records_campionamento_niseci<T: RecordCampionamentoNISECI>(
    records: Vec<T>,
    riferimento_specie: Vec<SpecieNISECI>,
) -> Result<Vec<RecordNISECI>, Vec<RecordCampionamentoNISECIError>> {
    check_records_campionamento_niseci_impl::<T>(
        records,
        &RiferimentoNISECI::new(riferimento_specie),
    )
    .map(|v| v.into())
}

impl CampionamentoNISECI {
    pub fn parse_records<T>(
        vec: Vec<T>,
        rif: &RiferimentoNISECI,
    ) -> CampionamentoNISECIParseResult
    where
        T: RecordCampionamentoNISECI,
    {
        CampionamentoNISECIParseResult::parse::<T>(vec, rif)
    }
    pub fn check_recordcsv<T>(
        vec: Vec<T>,
        rif: &RiferimentoNISECI,
    ) -> Result<Self, Vec<RecordCampionamentoNISECIError>>
    where
        T: RecordCampionamentoNISECI,
    {
        check_records_campionamento_niseci_impl::<T>(vec, rif)
    }
}

/// v0.2 will have this method public without the _impl suffix
/// Internal transitional API for migrating:
///   - borrow over riferimento_specie
///   - taking &RiferimentoNISECI over &Vec<SpecieNISECI>
///   - returning CampionamentoNISECI for success over Vec<RecordNISECI>
pub(crate) fn check_records_campionamento_niseci_impl<T: RecordCampionamentoNISECI>(
    records: Vec<T>,
    riferimento_specie: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, Vec<RecordCampionamentoNISECIError>> {
    let (camp, errors) =
        parse_records_campionamento_niseci(records, riferimento_specie).into_parts();

    println!(
        "Campionamento NISECI: Numero record validi: {}",
        camp.as_vec().len()
    );
    println!(
        "Campionamento NISECI: Numero record non validi: {}",
        errors.len()
    );

    if !errors.is_empty() {
        eprintln!(
            "Errori incontrati durante l'elaborazione dei record per campionamento NISECI: {{"
        );
        //TODO: add process_record_campionamentoNISECI_errors()
        for error in &errors {
            eprintln!("  {}", error);
        }
        eprintln!("}}");
        Err(errors)
    } else {
        //TODO: handle verbosity
        //println!("Tutti i record del campionamento NISECI sono stati processati con successo!");
        /*
        for record in &records {
            println!("  Record: {{{record}}}");
        }
        */
        Ok(camp)
    }
}

impl AnagraficaNISECI {
    pub fn parse_records<T>(vec: Vec<T>) -> Result<Self, Vec<RecordAnagraficaNISECIError>>
    where
        T: RecordAnagraficaNISECI,
    {
        parse_recordcsv_anagrafica_niseci::<T>(vec)
    }
    pub fn check_recordcsv<T>(vec: Vec<T>) -> Result<Self, Vec<RecordAnagraficaNISECIError>>
    where
        T: RecordAnagraficaNISECI,
    {
        check_records_anagrafica_niseci::<T>(vec)
    }
}

pub fn check_records_anagrafica_niseci<T: RecordAnagraficaNISECI>(
    records: Vec<T>,
) -> Result<AnagraficaNISECI, Vec<RecordAnagraficaNISECIError>> {
    let res = parse_recordcsv_anagrafica_niseci(records);

    match res {
        Ok(anagrafica) => {
            println!("Anagrafica NISECI: {}", anagrafica);
            //TODO: handle verbosity
            //println!("Tutti i record dell'anagrafica NISECI sono stati processati con successo!");
            /*
            for record in &records {
                println!("  Record: {{{record}}}");
            }
            */
            Ok(anagrafica)
        }
        Err(errors) => {
            println!(
                "Anagrafica NISECI: Numero record non validi: {}",
                errors.len()
            );
            eprintln!(
                "Errori incontrati durante l'elaborazione dei record per anagrafica NISECI: {{"
            );
            //TODO: add process_record_anagraficaNISECI_errors()
            for error in &errors {
                eprintln!("  {}", error);
            }
            eprintln!("}}");
            Err(errors)
        }
    }
}
