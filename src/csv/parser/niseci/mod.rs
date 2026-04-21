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

use crate::csv::parser::parse_date;
use crate::deser::{RecordAnagraficaNISECI, RecordCampionamentoNISECI, RecordRiferimentoNISECI};
use crate::domain::location::Location;
use crate::domain::niseci::{
    AnagraficaNISECI, AreaNISECI, CampionamentoNISECI, ComunitaNISECI, IdroEcoRegioneNISECI,
    RecordNISECI, RiferimentoNISECI, SpecieNISECI, TipoComunitaNISECI,
};
use crate::domain::posf32::PositiveF32;
use chrono::format::ParseErrorKind;

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

use crate::parser::niseci::RecordRiferimentoNISECIError;

#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using crate::parser::niseci::RecordRiferimentoNISECIError instead"
)]
pub use crate::parser::niseci::RecordRiferimentoNISECIError as RecordCsvRiferimentoNISECIError;

#[deprecated(
    note = "v0.2 will change signature to return RiferimentoNISECIParseResult\nConsider using RiferimentoNISECI::parse_recordcsv(records).into_parts()"
)]
pub fn parse_recordcsv_riferimento_niseci<T: RecordRiferimentoNISECI>(
    records: Vec<T>,
) -> (Vec<SpecieNISECI>, Vec<RecordRiferimentoNISECIError>) {
    let (rif, errs) = parse_recordcsv_riferimento_niseci_impl::<T>(records).into_parts();
    (rif.into(), errs)
}

pub struct RiferimentoNISECIParseResult(RiferimentoNISECI, Vec<RecordRiferimentoNISECIError>);

impl RiferimentoNISECIParseResult {
    pub fn parse<T: RecordRiferimentoNISECI>(records: Vec<T>) -> Self {
        parse_recordcsv_riferimento_niseci_impl(records)
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

/// v0.2 will have this method public without the _impl suffix
/// Internal transitional API for migrating:
///   - returning RiferimentoNISECIParseResult instead of tuple
///     - success field (.0) used to be Vec<SpecieNISECI>
pub(crate) fn parse_recordcsv_riferimento_niseci_impl<T: RecordRiferimentoNISECI>(
    records: Vec<T>,
) -> RiferimentoNISECIParseResult {
    let mut specie = Vec::new();
    let mut errors = Vec::new();
    let mut idx = 0;
    let mut used_id_specie = Vec::new(); // Stores already-parsed ids to detect doubles
    for r in records {
        idx += 1;
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
        if origine_autoctono {
            match r.tipo_autoctono() {
                1 | 2 => {
                    tipo_autoctono = r.tipo_autoctono() as u8;
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
            tipo_alloctono = 0;
        } else {
            tipo_autoctono = 0;
            match r.allo_nocivita() {
                0..=3 => {
                    tipo_alloctono = r.allo_nocivita() as u8;
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

        if r.codice_specie().is_empty() {
            let err = RecordRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: codice_specie non valido (lunghezza < 1)"),
            };
            errors.push(err);
            continue;
        }

        let id = r.codice_specie();

        if used_id_specie.contains(&id) {
            let err = RecordRiferimentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: codice_specie non valido (ridefinizione)"),
            };
            errors.push(err);
            continue;
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

        let specie_rec = SpecieNISECI {
            id: id.clone(),
            nome,
            tipo_autoctono,
            tipo_alloctono,
            specie_attesa,
            cl_soglia1: r.cl_soglia1(), // in cm
            cl_soglia2: r.cl_soglia2(), // in cm
            cl_soglia3: r.cl_soglia3(), // in cm
            cl_soglia4: r.cl_soglia4(), // in cm
            ad_juv_soglia1: r.ad_juv_soglia1(),
            ad_juv_soglia2: r.ad_juv_soglia2(),
            ad_juv_soglia3: r.ad_juv_soglia3(),
            ad_juv_soglia4: r.ad_juv_soglia4(),
            dens_soglia1: r.dens_soglia1(),
            dens_soglia2: r.dens_soglia2(),
        };
        specie.push(specie_rec);
        used_id_specie.push(id);
    }

    RiferimentoNISECIParseResult(RiferimentoNISECI::new(specie), errors)
}

use crate::parser::niseci::RecordCampionamentoNISECIError;

#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using crate::parser::niseci::RecordCampionamentoNISECIError instead"
)]
pub use crate::parser::niseci::RecordCampionamentoNISECIError as RecordCsvCampionamentoNISECIError;

#[deprecated(
    note = "v0.2 will change signature to:\n  - expect riferimento_specie as &RiferimentoNISECI\n  - return CampionamentoNISECIParseResult\n  Consider using CampionamentoNISECI::parse_recordcsv(records, riferimento).into_parts()"
)]
pub fn parse_recordcsv_campionamento_niseci<T: RecordCampionamentoNISECI>(
    records: Vec<T>,
    riferimento_specie: Vec<SpecieNISECI>,
) -> (Vec<RecordNISECI>, Vec<RecordCampionamentoNISECIError>) {
    let (camp, errs) = parse_recordcsv_campionamento_niseci_impl::<T>(
        records,
        &RiferimentoNISECI::new(riferimento_specie),
    )
    .into_parts();
    (camp.into(), errs)
}

pub struct CampionamentoNISECIParseResult(CampionamentoNISECI, Vec<RecordCampionamentoNISECIError>);

impl CampionamentoNISECIParseResult {
    pub fn parse<T: RecordCampionamentoNISECI>(
        records: Vec<T>,
        riferimento: &RiferimentoNISECI,
    ) -> Self {
        parse_recordcsv_campionamento_niseci_impl(records, riferimento)
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

/// v0.2 will have this method public without the _impl suffix
/// Internal transitional API for migrating:
///   - borrow over riferimento_specie
///   - taking &RiferimentoNISECI over &Vec<SpecieNISECI>
///   - returning CampionamentoNISECIParseResult instead of tuple
///     - success field (.0) used to be Vec<RecordNISECI>
pub(crate) fn parse_recordcsv_campionamento_niseci_impl<T: RecordCampionamentoNISECI>(
    records: Vec<T>,
    riferimento_specie: &RiferimentoNISECI,
) -> CampionamentoNISECIParseResult {
    let mut campioni = Vec::new();
    let mut errors = Vec::new();
    let mut idx = 0;
    for r in records {
        idx += 1;
        if r.codice_specie().is_empty() {
            let err = RecordCampionamentoNISECIError::ValoreInvalido {
                msg: format!("Record {idx}: codice_specie non valido (lunghezza < 1)"),
            };
            errors.push(err);
            continue;
        }
        let codice_specie = r.codice_specie();
        let mut opt_matched_specie = None;
        for s in riferimento_specie {
            // FIXME: this is O(n^2).
            if s.id == codice_specie {
                opt_matched_specie = Some(s);
                break; // TODO: mmmh
            }
        }

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

        let niseci_rec = RecordNISECI {
            specie: matched_specie.clone(),
            passaggio_cattura: passaggio_cattura as u8,
            lunghezza: r.lunghezza(),
            peso: r.peso(),
        };
        campioni.push(niseci_rec);
    }
    CampionamentoNISECIParseResult(CampionamentoNISECI::new(campioni), errors)
}

use crate::parser::niseci::RecordAnagraficaNISECIError;

#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using crate::parser::niseci::RecordAnagraficaNISECIError instead"
)]
pub use crate::parser::niseci::RecordAnagraficaNISECIError as RecordCsvAnagraficaNISECIError;

pub fn parse_recordcsv_anagrafica_niseci<T: RecordAnagraficaNISECI>(
    records: Vec<T>,
) -> Result<AnagraficaNISECI, Vec<RecordAnagraficaNISECIError>> {
    let mut errors = Vec::new();
    if records.len() > 1 {
        let err = RecordAnagraficaNISECIError::ValoreInvalido {
            msg: format!("Troppi record: {}, atteso 1", records.len()),
        };
        errors.push(err);
    }
    if records.is_empty() {
        let err = RecordAnagraficaNISECIError::ValoreInvalido {
            msg: "Nessun record trovato: atteso 1".to_string(),
        };
        errors.push(err);
        return Err(errors);
    }

    let r = records.first().unwrap();

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
        TipoComunitaNISECI::AffinataDalMase => {
            if r.numero_protocollo().is_empty() {
                let err = RecordAnagraficaNISECIError::ValoreInvalido {
                    msg: format!("Numero protocollo troppo corto: {}", r.numero_protocollo()),
                };
                errors.push(err);
            }
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
        ComunitaNISECI {
            tipo: tipo_comunita,
            fonte: Some(r.fonte()),
            numero_protocollo: Some(r.numero_protocollo()),
        },
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

#[deprecated(
    note = "v0.2 will change signature to return RiferimentoNISECI on success\nConsider using RiferimentoNISECI::check_recordcsv(records)"
)]
pub fn check_records_riferimento_niseci<T: RecordRiferimentoNISECI>(
    records: Vec<T>,
) -> Result<Vec<SpecieNISECI>, Vec<RecordRiferimentoNISECIError>> {
    check_records_riferimento_niseci_impl::<T>(records).map(|v| v.into())
}

impl RiferimentoNISECI {
    pub fn parse_recordcsv<T>(vec: Vec<T>) -> RiferimentoNISECIParseResult
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
    let (rif, errors) = parse_recordcsv_riferimento_niseci_impl(records).into_parts();

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
    pub fn parse_recordcsv<T>(
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
        parse_recordcsv_campionamento_niseci_impl(records, riferimento_specie).into_parts();

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
    pub fn parse_recordcsv<T>(vec: Vec<T>) -> Result<Self, Vec<RecordAnagraficaNISECIError>>
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
