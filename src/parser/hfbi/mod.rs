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
use crate::domain::hfbi::{
    AnagraficaHFBI, CampionamentoHFBI, HabitatHFBI, RecordHFBI, StagioneHFBI,
    TipoLagunaCostieraHFBI, RIFERIMENTO_HFBI_MAP,
};
use crate::domain::location::Location;
use crate::domain::posf32::PositiveF32;
use crate::parser::parse_date;
use chrono::format::ParseErrorKind;
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

pub struct CampionamentoHFBIParseResult(CampionamentoHFBI, Vec<RecordCampionamentoHFBIError>);

impl CampionamentoHFBIParseResult {
    pub fn parse<T: RecordCampionamentoHFBI>(records: Vec<T>) -> Self {
        parse_records_campionamento_hfbi(records)
    }
    pub fn into_parts(self) -> (CampionamentoHFBI, Vec<RecordCampionamentoHFBIError>) {
        (self.0, self.1)
    }
    pub fn value(&self) -> &CampionamentoHFBI {
        &self.0
    }
    pub fn errors(&self) -> &Vec<RecordCampionamentoHFBIError> {
        &self.1
    }
}

/// v0.2 will have this method public
/// Internal transitional API for migrating:
///   - returning CampionamentoHFBIParseResult instead of tuple
///     - success field (.0) used to be Vec<RecordHFBI>
pub(crate) fn parse_records_campionamento_hfbi<T: RecordCampionamentoHFBI>(
    records: Vec<T>,
) -> CampionamentoHFBIParseResult {
    let mut campioni = Vec::new();
    let mut errors = Vec::new();
    let mut idx = 0;
    for r in records {
        idx += 1;
        if r.codice_specie().is_empty() {
            let err = RecordCampionamentoHFBIError::ValoreInvalido {
                msg: format!("Record {idx}: codice_specie non valido (lunghezza < 1)"),
            };
            errors.push(err);
            continue;
        }
        let codice_specie = r.codice_specie();
        let opt_matched_specie = RIFERIMENTO_HFBI_MAP.get(&codice_specie);
        let matched_specie;
        if let Some(specie) = opt_matched_specie {
            matched_specie = specie;
        } else {
            let err = RecordCampionamentoHFBIError::ValoreInvalido {
                msg: format!(
                    "Record {idx}: codice_specie non valido (non presente nel riferimento): {}",
                    codice_specie
                ),
            };
            errors.push(err);
            continue;
        }

        //TODO: update this abomination when records change to have an integer directly
        if r.numero_individui() < 1 {
            let err = RecordCampionamentoHFBIError::ValoreInvalido {
                msg: format!(
                    "Record {idx}: numero_individui non valido (<1): {}",
                    r.numero_individui()
                ),
            };
            errors.push(err);
            continue;
        }

        if !r.peso().is_finite() {
            let err = RecordCampionamentoHFBIError::ValoreInvalido {
                msg: format!("Record {idx}: peso non valido (not finite): {}", r.peso()),
            };
            errors.push(err);
            continue;
        }
        let peso = r.peso();

        let hfbi_rec = RecordHFBI {
            specie: matched_specie.clone(),
            numero_individui: r.numero_individui(),
            peso,
        };
        campioni.push(hfbi_rec);
    }
    CampionamentoHFBIParseResult(CampionamentoHFBI::new(campioni), errors)
}

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

pub fn parse_records_anagrafica_hfbi<T: RecordAnagraficaHFBI>(
    records: Vec<T>,
) -> Result<AnagraficaHFBI, Vec<RecordAnagraficaHFBIError>> {
    let mut errors = Vec::new();
    if records.len() > 1 {
        let err = RecordAnagraficaHFBIError::ValoreInvalido {
            msg: format!("Troppi record: {}, atteso 1", records.len()),
        };
        errors.push(err);
    }
    let r = if let Some(r) = records.first() {
        r
    } else {
        let err = RecordAnagraficaHFBIError::ValoreInvalido {
            msg: "Nessun record trovato: atteso 1".to_string(),
        };
        errors.push(err);
        return Err(errors);
    };
    if r.codice_stazione().is_empty() {
        let err = RecordAnagraficaHFBIError::ValoreInvalido {
            msg: format!("Codice stazione troppo corto: {}", r.codice_stazione()),
        };
        errors.push(err);
    }

    if r.corpo_idrico().is_empty() {
        let err = RecordAnagraficaHFBIError::ValoreInvalido {
            msg: format!("Corpo idrico troppo corto: {}", r.corpo_idrico()),
        };
        errors.push(err);
    }

    if r.regione().is_empty() {
        let err = RecordAnagraficaHFBIError::ValoreInvalido {
            msg: format!("Regione troppo corta: {}", r.regione()),
        };
        errors.push(err);
    }

    if r.provincia().is_empty() {
        let err = RecordAnagraficaHFBIError::ValoreInvalido {
            msg: format!("Provincia troppo corta: {}", r.provincia()),
        };
        errors.push(err);
    }

    match parse_date(&r.data()) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ParseErrorKind::OutOfRange => {
                let err = RecordAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: fuori range".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::Impossible => {
                let err = RecordAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: valori non possibili".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::NotEnough => {
                let err = RecordAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: specifica insufficiente".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::Invalid => {
                let err = RecordAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: presenza di caratteri non attesi".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::TooShort => {
                let err = RecordAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: terminazione prematura dell'input".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::TooLong => {
                let err = RecordAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: input in eccesso".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::BadFormat => {
                let err = RecordAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: errore nella specifica di formattazione"
                        .to_string(),
                };
                errors.push(err);
            }
            _ => {
                let err = RecordAnagraficaHFBIError::ValoreInvalido {
                    msg: "Data fornita non valida: errore sconosciuto".to_string(),
                };
                errors.push(err);
            }
        },
    }

    let lunghezza = PositiveF32::new(r.lunghezza_stazione()).unwrap_or_else(|_| {
        let err = RecordAnagraficaHFBIError::ValoreInvalido {
            msg: format!(
                "Lunghezza stazione non finito e positivo: {}",
                r.lunghezza_stazione()
            ),
        };
        errors.push(err);
        PositiveF32::new(1.0).expect("1.0 should be a valid positive finite f32")
        // It looks like we still take this value but we will return with Err since errors is not
        // empty
    });

    let larghezza = PositiveF32::new(r.larghezza_stazione()).unwrap_or_else(|_| {
        let err = RecordAnagraficaHFBIError::ValoreInvalido {
            msg: format!(
                "Larghezza stazione non finito e positivo: {}",
                r.larghezza_stazione()
            ),
        };
        errors.push(err);
        PositiveF32::new(1.0).expect("1.0 should be a valid positive finite f32")
        // It looks like we still take this value but we will return with Err since errors is not
        // empty
    });

    let mut stagione = StagioneHFBI::Primavera;
    match r.stagione() {
        0 => {
            stagione = StagioneHFBI::Primavera;
        }
        1 => {
            stagione = StagioneHFBI::Autunno;
        }
        _ => {
            let err = RecordAnagraficaHFBIError::ValoreInvalido {
                msg: format!("Stagione HFBI non valido: {}, atteso [0, 1]", r.stagione()),
            };
            errors.push(err);
        }
    }

    let habitat = match r.habitat() {
        0 => HabitatHFBI::Vegetato,
        1 => HabitatHFBI::NonVegetato,
        _ => {
            let err = RecordAnagraficaHFBIError::ValoreInvalido {
                msg: format!("HabitatHFBI non valido: {}, atteso [0, 1]", r.habitat()),
            };
            errors.push(err);
            HabitatHFBI::Vegetato // To still assign something by default
        }
    };

    let mut tipo_laguna = TipoLagunaCostieraHFBI::MAt1;
    match r.tipo_laguna() {
        1 => {
            tipo_laguna = TipoLagunaCostieraHFBI::MAt1;
        }
        2 => {
            tipo_laguna = TipoLagunaCostieraHFBI::MAt2;
        }
        3 => {
            tipo_laguna = TipoLagunaCostieraHFBI::MAt3;
        }
        _ => {
            let err = RecordAnagraficaHFBIError::ValoreInvalido {
                msg: format!(
                    "TipoLagunaCostieraHFBI non valido: {}, atteso [1, 3]",
                    r.tipo_laguna()
                ),
            };
            errors.push(err);
        }
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    let res = AnagraficaHFBI::new(
        r.codice_stazione(),
        r.corpo_idrico(),
        Location {
            regione: r.regione(),
            provincia: r.provincia(),
        },
        r.data(), // Formato gg/mm/aaaa
        tipo_laguna,
        stagione,
        habitat,
        lunghezza,
        larghezza,
    );
    Ok(res)
}

pub fn check_records_campionamento_hfbi<T: RecordCampionamentoHFBI>(
    records: Vec<T>,
) -> Result<CampionamentoHFBI, Vec<RecordCampionamentoHFBIError>> {
    let (camp, errors) = parse_records_campionamento_hfbi(records).into_parts();

    println!(
        "Campionamento HFBI: Numero record validi: {}",
        camp.into_iter().collect::<Vec<_>>().len()
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

pub fn check_records_anagrafica_hfbi<T: RecordAnagraficaHFBI>(
    records: Vec<T>,
) -> Result<AnagraficaHFBI, Vec<RecordAnagraficaHFBIError>> {
    let res = parse_records_anagrafica_hfbi(records);

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

impl CampionamentoHFBI {
    pub fn parse_records<T>(vec: Vec<T>) -> CampionamentoHFBIParseResult
    where
        T: RecordCampionamentoHFBI,
    {
        CampionamentoHFBIParseResult::parse::<T>(vec)
    }
    pub fn check_records<T>(vec: Vec<T>) -> Result<Self, Vec<RecordCampionamentoHFBIError>>
    where
        T: RecordCampionamentoHFBI,
    {
        check_records_campionamento_hfbi::<T>(vec)
    }
    #[deprecated(note = "v0.2 will drop visibility.\nConsider using check_records instead")]
    pub fn check_record<T>(vec: Vec<T>) -> Result<Self, Vec<RecordCampionamentoHFBIError>>
    where
        T: RecordCampionamentoHFBI,
    {
        Self::check_records::<T>(vec)
    }
}

impl AnagraficaHFBI {
    pub fn parse_records<T>(vec: Vec<T>) -> Result<Self, Vec<RecordAnagraficaHFBIError>>
    where
        T: RecordAnagraficaHFBI,
    {
        parse_records_anagrafica_hfbi::<T>(vec)
    }
    pub fn check_records<T>(vec: Vec<T>) -> Result<Self, Vec<RecordAnagraficaHFBIError>>
    where
        T: RecordAnagraficaHFBI,
    {
        check_records_anagrafica_hfbi::<T>(vec)
    }
}
