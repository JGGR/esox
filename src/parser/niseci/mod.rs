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
use crate::domain::location::Location;
use crate::domain::niseci::{
    AnagraficaNISECI, AreaNISECI, CampionamentoNISECI, ComunitaNISECI, IdroEcoRegioneNISECI,
    InternerSpecieNISECI, OrigineSpecieNISECI, RecordNISECI, RiferimentoNISECI, SpecieNISECI,
    TipoComunitaNISECI,
};
use crate::domain::posf32::PositiveF32;
use crate::parser::parse_date;
use chrono::format::ParseErrorKind;
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

pub struct RiferimentoNISECIParseResult(RiferimentoNISECI, Vec<RecordRiferimentoNISECIError>);

impl RiferimentoNISECIParseResult {
    pub fn parse<T: RecordRiferimentoNISECI>(records: Vec<T>) -> Self {
        parse_records_riferimento_niseci(records)
    }
    pub fn into_parts(self) -> (RiferimentoNISECI, Vec<RecordRiferimentoNISECIError>) {
        (self.0, self.1)
    }
    pub fn value(&self) -> &RiferimentoNISECI {
        &self.0
    }
    pub fn errors(&self) -> &Vec<RecordRiferimentoNISECIError> {
        &self.1
    }
}

fn check_soglie_cl<T: RecordRiferimentoNISECI>(r: &T) -> bool {
    if r.cl_soglia1() < r.cl_soglia2()
        && r.cl_soglia2() < r.cl_soglia3()
        && r.cl_soglia3() < r.cl_soglia4()
    {
        return true;
    }
    false
}

fn check_soglie_ad_juv<T: RecordRiferimentoNISECI>(r: &T) -> bool {
    if r.ad_juv_soglia1() < r.ad_juv_soglia2()
        && r.ad_juv_soglia2() < r.ad_juv_soglia3()
        && r.ad_juv_soglia3() < r.ad_juv_soglia4()
    {
        return true;
    }
    false
}

/// v0.2 will have this method public
/// Internal transitional API for migrating:
///   - returning RiferimentoNISECIParseResult instead of tuple
///     - success field (.0) used to be Vec<SpecieNISECI>
pub(crate) fn parse_records_riferimento_niseci<T: RecordRiferimentoNISECI>(
    records: Vec<T>,
) -> RiferimentoNISECIParseResult {
    let mut errors = Vec::new();
    let mut idx = 0;
    let mut interner = InternerSpecieNISECI::new();
    for r in records {
        idx += 1;
        let id = r.codice_specie();
        if id.is_empty() {
            let err = RecordRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: codice_specie non valido (lunghezza < 1)"),
            };
            errors.push(err);
            continue;
        }

        if interner.contains_id(&id) {
            let err = RecordRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: codice_specie non valido (ridefinizione)"),
            };
            errors.push(err);
            continue;
        }
        let mut origine_autoctono = true;
        match r.origine().as_str() {
            "ALL" => {
                origine_autoctono = false;
            }
            "AUT" => {}
            _ => {
                let err = RecordRiferimentoNISECIError::ValoreInvalido {
                    msg: format!(
                        "Record {idx}: origine invalida (non \"AUT\" o \"ALL\"): {}",
                        r.origine()
                    ),
                };
                errors.push(err);
                continue;
            }
        }
        let specie_attesa = r.specie_attesa() > 0; // TODO: possiamo prendere qualsiasi non-zero come
                                                   // "atteso"?
        let tipo_autoctono: u8;
        let tipo_alloctono: u8;
        let origine;
        if origine_autoctono {
            match r.tipo_autoctono() {
                1 | 2 => {
                    tipo_autoctono = r.tipo_autoctono() as u8;
                    origine = OrigineSpecieNISECI::Autoctono(tipo_autoctono);
                }
                _ => {
                    let err = RecordRiferimentoNISECIError::ValoreInvalido {
                        msg: format!(
                            "Record {idx}: tipo_autoctono non valido (non 1 o 2): {}",
                            r.tipo_autoctono()
                        ),
                    };
                    errors.push(err);
                    continue;
                }
            }
        } else {
            match r.allo_nocivita() {
                0..=3 => {
                    tipo_alloctono = r.allo_nocivita() as u8;
                    origine = OrigineSpecieNISECI::Alloctono(tipo_alloctono);
                }
                _ => {
                    let err = RecordRiferimentoNISECIError::ValoreInvalido {
                        msg: format!(
                            "Record {idx}: allo_nocivita non valido (non [0..3]): {}",
                            r.allo_nocivita()
                        ),
                    };
                    errors.push(err);
                    continue;
                }
            }
        }

        let nome = r.nome_latino(); //TODO: controllare se dovrebbe essere nome_comune

        let epsilon: f32 = 1e-6;

        // Check dens_soglia
        if r.dens_soglia1() < 0.0 {
            let err = RecordRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: dens_soglia1 non valido (< 0)"),
            };
            errors.push(err);
            continue;
        }

        if r.dens_soglia1().abs() < epsilon && specie_attesa {
            let err = RecordRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: dens_soglia1 non valido (== 0) per una specie attesa"),
            };
            errors.push(err);
            continue;
        }

        if r.dens_soglia2() < 0.0 {
            let err = RecordRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: dens_soglia2 non valido (< 0)"),
            };
            errors.push(err);
            continue;
        }

        if r.dens_soglia2().abs() < epsilon && specie_attesa {
            let err = RecordRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: dens_soglia2 non valido (== 0) per una specie attesa"),
            };
            errors.push(err);
            continue;
        }

        if r.dens_soglia1() >= r.dens_soglia2() && specie_attesa {
            let err = RecordRiferimentoNISECIError::ValoreInvalido {
                msg: format!(
                    "Record {idx}: dens_soglia1 maggiore di dens_soglia2 per una specie attesa"
                ),
            };
            errors.push(err);
            continue;
        }

        if !check_soglie_cl(&r) {
            let err = RecordRiferimentoNISECIError::SoglieCLNonCrescenti {
                msg: format!("Record {idx}: soglie CL non crescenti"),
            };
            errors.push(err);
            continue;
        }
        if !check_soglie_ad_juv(&r) {
            let err = RecordRiferimentoNISECIError::SoglieADJUVNonCrescenti {
                msg: format!("Record {idx}: soglie AD/JUV non crescenti"),
            };
            errors.push(err);
            continue;
        }

        let specie_id = id.clone();

        let specie_rec = SpecieNISECI::new(
            &specie_id,
            &nome,
            origine,
            specie_attesa,
            r.cl_soglia1(), // in cm
            r.cl_soglia2(), // in cm
            r.cl_soglia3(), // in cm
            r.cl_soglia4(), // in cm
            r.ad_juv_soglia1(),
            r.ad_juv_soglia2(),
            r.ad_juv_soglia3(),
            r.ad_juv_soglia4(),
            r.dens_soglia1(),
            r.dens_soglia2(),
        );
        interner.intern(specie_rec);
    }

    RiferimentoNISECIParseResult(RiferimentoNISECI::new_from_map(interner), errors)
}

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

pub struct CampionamentoNISECIParseResult(CampionamentoNISECI, Vec<RecordCampionamentoNISECIError>);

impl CampionamentoNISECIParseResult {
    pub fn parse<T: RecordCampionamentoNISECI>(
        records: Vec<T>,
        riferimento: &RiferimentoNISECI,
    ) -> Self {
        parse_records_campionamento_niseci(records, riferimento)
    }
    pub fn into_parts(self) -> (CampionamentoNISECI, Vec<RecordCampionamentoNISECIError>) {
        (self.0, self.1)
    }
    pub fn value(&self) -> &CampionamentoNISECI {
        &self.0
    }
    pub fn errors(&self) -> &Vec<RecordCampionamentoNISECIError> {
        &self.1
    }
}

/// v0.2 will have this method public
/// Internal transitional API for migrating:
///   - borrow over riferimento_specie
///   - taking &RiferimentoNISECI over &Vec<SpecieNISECI>
///   - returning CampionamentoNISECIParseResult instead of tuple
///     - success field (.0) used to be Vec<RecordNISECI>
pub(crate) fn parse_records_campionamento_niseci<T: RecordCampionamentoNISECI>(
    records: Vec<T>,
    riferimento_specie: &RiferimentoNISECI,
) -> CampionamentoNISECIParseResult {
    let mut campioni = Vec::new();
    let mut errors = Vec::new();
    let mut idx = 0;
    for r in records {
        idx += 1;
        let codice_specie = r.codice_specie();
        if codice_specie.is_empty() {
            let err = RecordCampionamentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: codice_specie non valido (lunghezza < 1)"),
            };
            errors.push(err);
            continue;
        }
        let opt_matched_specie = riferimento_specie.get_ref_by_id(&codice_specie);
        let matched_specie;
        if let Some(specie) = opt_matched_specie {
            matched_specie = specie;
        } else {
            let err = RecordCampionamentoNISECIError::ValoreInvalido {
                msg: format!(
                    "Record {idx}: codice_specie non valido (non presente nel riferimento): {}",
                    codice_specie
                ),
            };
            errors.push(err);
            continue;
        }

        if r.num_passaggio() < 1 {
            let err = RecordCampionamentoNISECIError::ValoreInvalido {
                msg: format!(
                    "Record {idx}: num_passaggio non valido (<1): {}",
                    r.num_passaggio()
                ),
            };
            errors.push(err);
            continue;
        }
        let passaggio_cattura = r.num_passaggio();

        if !r.peso().is_finite() {
            let err = RecordCampionamentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: peso non valido (not finite): {}", r.peso()),
            };
            errors.push(err);
            continue;
        }

        let matched_specie = riferimento_specie
            .get_inner_id(matched_specie.id())
            .expect("Riferimento should contain this id");

        let niseci_rec = RecordNISECI::new(
            matched_specie,
            passaggio_cattura as u8,
            r.lunghezza(),
            r.peso(),
        );
        campioni.push(niseci_rec);
    }
    CampionamentoNISECIParseResult(CampionamentoNISECI::new(campioni), errors)
}

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

pub fn parse_records_anagrafica_niseci<T: RecordAnagraficaNISECI>(
    records: Vec<T>,
) -> Result<AnagraficaNISECI, Vec<RecordAnagraficaNISECIError>> {
    let mut errors = Vec::new();
    if records.len() > 1 {
        let err = RecordAnagraficaNISECIError::ValoreInvalido {
            msg: format!("Troppi record: {}, atteso 1", records.len()),
        };
        errors.push(err);
    }
    let r = if let Some(r) = records.first() {
        r
    } else {
        let err = RecordAnagraficaNISECIError::ValoreInvalido {
            msg: "Nessun record trovato: atteso 1".to_string(),
        };
        errors.push(err);
        return Err(errors);
    };

    if r.codice_stazione().is_empty() {
        let err = RecordAnagraficaNISECIError::ValoreInvalido {
            msg: format!("Codice stazione troppo corto: {}", r.codice_stazione()),
        };
        errors.push(err);
    }

    if r.corpo_idrico().is_empty() {
        let err = RecordAnagraficaNISECIError::ValoreInvalido {
            msg: format!("Corpo idrico troppo corto: {}", r.corpo_idrico()),
        };
        errors.push(err);
    }

    if r.regione().is_empty() {
        let err = RecordAnagraficaNISECIError::ValoreInvalido {
            msg: format!("Regione troppo corta: {}", r.regione()),
        };
        errors.push(err);
    }

    if r.provincia().is_empty() {
        let err = RecordAnagraficaNISECIError::ValoreInvalido {
            msg: format!("Provincia troppo corta: {}", r.provincia()),
        };
        errors.push(err);
    }

    match parse_date(&r.data()) {
        Ok(_) => {}
        Err(e) => match e.kind() {
            ParseErrorKind::OutOfRange => {
                let err = RecordAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: fuori range".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::Impossible => {
                let err = RecordAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: valori non possibili".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::NotEnough => {
                let err = RecordAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: specifica insufficiente".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::Invalid => {
                let err = RecordAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: presenza di caratteri non attesi".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::TooShort => {
                let err = RecordAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: terminazione prematura dell'input".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::TooLong => {
                let err = RecordAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: input in eccesso".to_string(),
                };
                errors.push(err);
            }
            ParseErrorKind::BadFormat => {
                let err = RecordAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: errore nella specifica di formattazione"
                        .to_string(),
                };
                errors.push(err);
            }
            _ => {
                let err = RecordAnagraficaNISECIError::ValoreInvalido {
                    msg: "Data fornita non valida: errore sconosciuto".to_string(),
                };
                errors.push(err);
            }
        },
    }

    if r.lunghezza_stazione() < 0.0 {
        let err = RecordAnagraficaNISECIError::ValoreInvalido {
            msg: format!(
                "Lunghezza stazione troppo bassa: {}",
                r.lunghezza_stazione()
            ),
        };
        errors.push(err);
    }

    let lunghezza = PositiveF32::new(r.lunghezza_stazione()).unwrap_or_else(|_| {
        let err = RecordAnagraficaNISECIError::ValoreInvalido {
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
        let err = RecordAnagraficaNISECIError::ValoreInvalido {
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

    let mut tipo_comunita = TipoComunitaNISECI::Redatta;
    match r.tipo_comunita() {
        0 => { /* Redatta */ }
        1 => {
            tipo_comunita = TipoComunitaNISECI::Recuperata;
        }
        2 => {
            tipo_comunita = TipoComunitaNISECI::Dm260_2010;
        }
        3 => {
            tipo_comunita = TipoComunitaNISECI::AffinataDalMase;
        }
        _ => {
            let err = RecordAnagraficaNISECIError::ValoreInvalido {
                msg: format!(
                    "Tipo comunita NISECI non valido: {}, atteso [0, 3]",
                    r.tipo_comunita()
                ),
            };
            errors.push(err);
        }
    }

    match tipo_comunita {
        TipoComunitaNISECI::Recuperata => {
            if r.fonte().is_empty() {
                let err = RecordAnagraficaNISECIError::ValoreInvalido {
                    msg: format!("Fonte troppo corta: {}", r.fonte()),
                };
                errors.push(err);
            }
        }
        TipoComunitaNISECI::AffinataDalMase if r.numero_protocollo().is_empty() => {
            let err = RecordAnagraficaNISECIError::ValoreInvalido {
                msg: format!("Numero protocollo troppo corto: {}", r.numero_protocollo()),
            };
            errors.push(err);
        }
        _ => {}
    }

    let idro_eco_regione = match r.idro_eco_regione() {
        0 => IdroEcoRegioneNISECI::AlpiOccidentali,
        1 => IdroEcoRegioneNISECI::PrealpiDolomiti,
        2 => IdroEcoRegioneNISECI::AlpiCentroOrientali,
        3 => IdroEcoRegioneNISECI::AlpiMeridionali,
        4 => IdroEcoRegioneNISECI::Monferrato,
        5 => IdroEcoRegioneNISECI::PianuraPadana,
        6 => IdroEcoRegioneNISECI::Carso,
        7 => IdroEcoRegioneNISECI::AppenninoPiemontese,
        8 => IdroEcoRegioneNISECI::AlpiMediterranee,
        9 => IdroEcoRegioneNISECI::AppenninoSettentrionale,
        10 => IdroEcoRegioneNISECI::Toscana,
        11 => IdroEcoRegioneNISECI::CostaAdriatica,
        12 => IdroEcoRegioneNISECI::AppenninoCentrale,
        13 => IdroEcoRegioneNISECI::RomaViterbese,
        14 => IdroEcoRegioneNISECI::BassoLazio,
        15 => IdroEcoRegioneNISECI::Vesuvio,
        16 => IdroEcoRegioneNISECI::BasilicataTavoliere,
        17 => IdroEcoRegioneNISECI::PugliaCarsica,
        18 => IdroEcoRegioneNISECI::AppenninoMeridionale,
        19 => IdroEcoRegioneNISECI::Sicilia,
        20 => IdroEcoRegioneNISECI::Sardegna,
        _ => {
            let err = RecordAnagraficaNISECIError::ValoreInvalido {
                msg: format!(
                    "IdroEcoRegioneNISECI non valido: {}, atteso [0, 20]",
                    r.idro_eco_regione()
                ),
            };
            errors.push(err);
            IdroEcoRegioneNISECI::Toscana // To still assign something by default
        }
    };

    let mut area = AreaNISECI::Mediterranea;
    if r.area_alpina() > 0 {
        area = AreaNISECI::Alpina;
    }

    if r.nome_bacino().is_empty() {
        let err = RecordAnagraficaNISECIError::ValoreInvalido {
            msg: format!("Nome bacino troppo corto: {}", r.nome_bacino()),
        };
        errors.push(err);
    }

    if !errors.is_empty() {
        return Err(errors);
    }

    let res = AnagraficaNISECI::new(
        ComunitaNISECI::new(tipo_comunita, Some(r.fonte()), Some(r.numero_protocollo())),
        r.codice_stazione(),
        r.data(), // Formato gg/mm/aaaa
        area,
        r.corpo_idrico(),
        r.nome_bacino(),
        idro_eco_regione,
        Location {
            regione: r.regione(),
            provincia: r.provincia(),
        },
        lunghezza,
        larghezza,
    );
    Ok(res)
}

pub fn check_records_riferimento_niseci<T: RecordRiferimentoNISECI>(
    records: Vec<T>,
) -> Result<RiferimentoNISECI, Vec<RecordRiferimentoNISECIError>> {
    let (rif, errors) = parse_records_riferimento_niseci(records).into_parts();

    println!(
        "Riferimento NISECI: Numero record validi: {}",
        rif.into_iter().collect::<Vec<_>>().len()
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

pub fn check_records_campionamento_niseci<T: RecordCampionamentoNISECI>(
    records: Vec<T>,
    riferimento_specie: &RiferimentoNISECI,
) -> Result<CampionamentoNISECI, Vec<RecordCampionamentoNISECIError>> {
    let (camp, errors) =
        parse_records_campionamento_niseci(records, riferimento_specie).into_parts();

    println!(
        "Campionamento NISECI: Numero record validi: {}",
        camp.into_iter().collect::<Vec<_>>().len()
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

pub fn check_records_anagrafica_niseci<T: RecordAnagraficaNISECI>(
    records: Vec<T>,
) -> Result<AnagraficaNISECI, Vec<RecordAnagraficaNISECIError>> {
    let res = parse_records_anagrafica_niseci(records);

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

impl RiferimentoNISECI {
    pub fn parse_records<T>(vec: Vec<T>) -> RiferimentoNISECIParseResult
    where
        T: RecordRiferimentoNISECI,
    {
        RiferimentoNISECIParseResult::parse::<T>(vec)
    }
    pub fn check_records<T>(vec: Vec<T>) -> Result<Self, Vec<RecordRiferimentoNISECIError>>
    where
        T: RecordRiferimentoNISECI,
    {
        check_records_riferimento_niseci::<T>(vec)
    }
}

impl CampionamentoNISECI {
    pub fn parse_records<T>(vec: Vec<T>, rif: &RiferimentoNISECI) -> CampionamentoNISECIParseResult
    where
        T: RecordCampionamentoNISECI,
    {
        CampionamentoNISECIParseResult::parse::<T>(vec, rif)
    }
    pub fn check_records<T>(
        vec: Vec<T>,
        rif: &RiferimentoNISECI,
    ) -> Result<Self, Vec<RecordCampionamentoNISECIError>>
    where
        T: RecordCampionamentoNISECI,
    {
        check_records_campionamento_niseci::<T>(vec, rif)
    }
}

impl AnagraficaNISECI {
    pub fn parse_records<T>(vec: Vec<T>) -> Result<Self, Vec<RecordAnagraficaNISECIError>>
    where
        T: RecordAnagraficaNISECI,
    {
        parse_records_anagrafica_niseci::<T>(vec)
    }
    pub fn check_records<T>(vec: Vec<T>) -> Result<Self, Vec<RecordAnagraficaNISECIError>>
    where
        T: RecordAnagraficaNISECI,
    {
        check_records_anagrafica_niseci::<T>(vec)
    }
}
