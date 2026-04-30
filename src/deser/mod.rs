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
use std::fmt;

#[derive(Copy, Clone)]
pub enum TipoRecord {
    RiferimentoNISECI,
    CampionamentoNISECI,
    AnagraficaNISECI,
    CampionamentoHFBI,
    AnagraficaHFBI,
}

pub trait RecordRiferimentoNISECI: serde::de::DeserializeOwned {
    #[allow(dead_code)]
    fn nome_comune(&self) -> String;
    fn nome_latino(&self) -> String;
    fn codice_specie(&self) -> String;
    fn origine(&self) -> String;
    fn tipo_autoctono(&self) -> u32;
    fn allo_nocivita(&self) -> u32;
    fn specie_attesa(&self) -> u32;
    fn cl_soglia1(&self) -> u32;
    fn cl_soglia2(&self) -> u32;
    fn cl_soglia3(&self) -> u32;
    fn cl_soglia4(&self) -> u32;
    fn ad_juv_soglia1(&self) -> f32;
    fn ad_juv_soglia2(&self) -> f32;
    fn ad_juv_soglia3(&self) -> f32;
    fn ad_juv_soglia4(&self) -> f32;
    fn dens_soglia1(&self) -> f32;
    fn dens_soglia2(&self) -> f32;
}

pub trait RecordCampionamentoNISECI: serde::de::DeserializeOwned {
    #[allow(dead_code)]
    fn data(&self) -> String;
    #[allow(dead_code)]
    fn stazione(&self) -> String;
    fn num_passaggio(&self) -> u32;
    fn codice_specie(&self) -> String;
    fn lunghezza(&self) -> u32;
    fn peso(&self) -> f32;
}

pub trait RecordAnagraficaNISECI: serde::de::DeserializeOwned {
    fn codice_stazione(&self) -> String;
    fn corpo_idrico(&self) -> String;
    fn regione(&self) -> String;
    fn provincia(&self) -> String;
    fn data(&self) -> String;
    fn lunghezza_stazione(&self) -> f32;
    fn larghezza_stazione(&self) -> f32;
    fn tipo_comunita(&self) -> u32;
    fn fonte(&self) -> String;
    fn numero_protocollo(&self) -> String;
    fn idro_eco_regione(&self) -> u32;
    fn area_alpina(&self) -> u32;
    fn nome_bacino(&self) -> String;
}

pub trait RecordCampionamentoHFBI: serde::de::DeserializeOwned {
    fn codice_specie(&self) -> String;
    fn numero_individui(&self) -> u32;
    fn peso(&self) -> f32;
}

pub trait RecordAnagraficaHFBI: serde::de::DeserializeOwned {
    fn codice_stazione(&self) -> String;
    fn corpo_idrico(&self) -> String;
    fn regione(&self) -> String;
    fn provincia(&self) -> String;
    fn data(&self) -> String;
    fn lunghezza_stazione(&self) -> f32;
    fn larghezza_stazione(&self) -> f32;
    fn stagione(&self) -> u32;
    fn habitat(&self) -> u32;
    fn tipo_laguna(&self) -> u32;
}

pub fn parse_serialized_records<I, T, E>(iter: I) -> (Vec<T>, Vec<E>)
where
    I: IntoIterator<Item = Result<T, E>>,
{
    let mut records = Vec::new();
    let mut errors = Vec::new();

    for result in iter {
        match result {
            Ok(record) => records.push(record),
            Err(e) => errors.push(e),
        }
    }

    (records, errors)
}

pub fn check_serialized_records<T, E>(
    iter: impl IntoIterator<Item = Result<T, E>>,
) -> Result<Vec<T>, Vec<E>> {
    let (records, errors) = parse_serialized_records(iter);

    if errors.is_empty() {
        Ok(records)
    } else {
        Err(errors)
    }
}

#[deprecated(
    note = "v0.2 will drop visibility. Use `check_serialized_records` and handle errors explicitly instead.\nItalian formatting can be used from esox::csv::deser::error::duccio::format_csv_errors"
)]
pub fn validate_serialized_records<T, E, F>(
    iter: impl IntoIterator<Item = Result<T, E>>,
    on_error: F,
) -> Result<Vec<T>, Vec<E>>
where
    F: Fn(&Vec<E>),
{
    #[allow(deprecated)]
    debug_serialized_records::<T, E, F>(iter, on_error)
}

#[deprecated(
    note = "Use `check_serialized_records` and handle errors explicitly instead.\nItalian formatting can be used from esox::csv::deser::error::duccio::format_csv_errors"
)]
pub fn debug_serialized_records<T, E, F>(
    iter: impl IntoIterator<Item = Result<T, E>>,
    on_error: F,
) -> Result<Vec<T>, Vec<E>>
where
    F: Fn(&Vec<E>),
{
    match check_serialized_records(iter) {
        Ok(records) => Ok(records),
        Err(errors) => {
            on_error(&errors);
            Err(errors)
        }
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlainRecordRiferimentoNISECI {
    pub nome_comune: String,
    pub nome_latino: String,
    pub codice_specie: String,
    pub origine: String,
    pub tipo_autoctono: u32,
    pub allo_nocivita: u32,
    pub specie_attesa: u32,
    pub cl_soglia1: u32, // in mm
    pub cl_soglia2: u32, // in mm
    pub cl_soglia3: u32, // in mm
    pub cl_soglia4: u32, // in mm
    pub ad_juv_soglia1: f32,
    pub ad_juv_soglia2: f32,
    pub ad_juv_soglia3: f32,
    pub ad_juv_soglia4: f32,
    pub dens_soglia1: f32,
    pub dens_soglia2: f32,
}

impl RecordRiferimentoNISECI for PlainRecordRiferimentoNISECI {
    fn nome_comune(&self) -> String {
        self.nome_comune.clone()
    }
    fn nome_latino(&self) -> String {
        self.nome_latino.clone()
    }
    fn codice_specie(&self) -> String {
        self.codice_specie.clone()
    }
    fn origine(&self) -> String {
        self.origine.clone()
    }
    fn tipo_autoctono(&self) -> u32 {
        self.tipo_autoctono
    }
    fn allo_nocivita(&self) -> u32 {
        self.allo_nocivita
    }
    fn specie_attesa(&self) -> u32 {
        self.specie_attesa
    }
    fn cl_soglia1(&self) -> u32 {
        self.cl_soglia1
    }
    fn cl_soglia2(&self) -> u32 {
        self.cl_soglia2
    }
    fn cl_soglia3(&self) -> u32 {
        self.cl_soglia3
    }
    fn cl_soglia4(&self) -> u32 {
        self.cl_soglia4
    }
    fn ad_juv_soglia1(&self) -> f32 {
        self.ad_juv_soglia1
    }
    fn ad_juv_soglia2(&self) -> f32 {
        self.ad_juv_soglia2
    }
    fn ad_juv_soglia3(&self) -> f32 {
        self.ad_juv_soglia3
    }
    fn ad_juv_soglia4(&self) -> f32 {
        self.ad_juv_soglia4
    }
    fn dens_soglia1(&self) -> f32 {
        self.dens_soglia1
    }
    fn dens_soglia2(&self) -> f32 {
        self.dens_soglia2
    }
}

impl fmt::Display for PlainRecordRiferimentoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordRiferimentoNISECI: {{ nome_comune: [{}], nome_latino: [{}], codice_specie: [{}], origine: [{}], tipo_autoctono: [{}], allo_nocivita: [{}], specie_attesa: [{}], cl_soglia1: [{}], cl_soglia2: [{}], cl_soglia3: [{}], cl_soglia4: [{}], ad_juv_soglia1: [{}], ad_juv_soglia2: [{}], ad_juv_soglia3: [{}], ad_juv_soglia4: [{}], dens_soglia1: [{}], dens_soglia2: [{}] }}",
              self.nome_comune, self.nome_latino, self.codice_specie, self.origine,
              self.tipo_autoctono, self.allo_nocivita, self.specie_attesa,
              self.cl_soglia1, self.cl_soglia2, self.cl_soglia3, self.cl_soglia4,
              self.ad_juv_soglia1, self.ad_juv_soglia2, self.ad_juv_soglia3, self.ad_juv_soglia4,
              self.dens_soglia1, self.dens_soglia2
        );
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlainRecordCampionamentoNISECI {
    pub data: String,
    pub stazione: String,
    pub num_passaggio: u32,
    pub codice_specie: String,
    pub lunghezza: u32,
    pub peso: f32,
}

impl RecordCampionamentoNISECI for PlainRecordCampionamentoNISECI {
    fn data(&self) -> String {
        self.data.clone()
    }
    fn stazione(&self) -> String {
        self.stazione.clone()
    }
    fn num_passaggio(&self) -> u32 {
        self.num_passaggio
    }
    fn codice_specie(&self) -> String {
        self.codice_specie.clone()
    }
    fn lunghezza(&self) -> u32 {
        self.lunghezza
    }
    fn peso(&self) -> f32 {
        self.peso
    }
}

impl fmt::Display for PlainRecordCampionamentoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCampionamentoNISECI: {{ data: [{}], stazione: [{}], num_passaggio: [{}], codice_specie: [{}], lunghezza: [{}], peso: [{}] }}",
              self.data, self.stazione, self.num_passaggio,
              self.codice_specie, self.lunghezza, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlainRecordAnagraficaNISECI {
    pub codice_stazione: String,
    pub corpo_idrico: String,
    pub regione: String,
    pub provincia: String,
    pub data: String,
    pub lunghezza_stazione: f32,
    pub larghezza_stazione: f32,
    pub tipo_comunita: u32,
    pub fonte: String,
    pub numero_protocollo: String,
    pub idro_eco_regione: u32,
    pub area_alpina: u32,
    pub nome_bacino: String,
}

impl RecordAnagraficaNISECI for PlainRecordAnagraficaNISECI {
    fn codice_stazione(&self) -> String {
        self.codice_stazione.clone()
    }
    fn corpo_idrico(&self) -> String {
        self.corpo_idrico.clone()
    }
    fn regione(&self) -> String {
        self.regione.clone()
    }
    fn provincia(&self) -> String {
        self.provincia.clone()
    }
    fn data(&self) -> String {
        self.data.clone()
    }
    fn lunghezza_stazione(&self) -> f32 {
        self.lunghezza_stazione
    }
    fn larghezza_stazione(&self) -> f32 {
        self.larghezza_stazione
    }
    fn tipo_comunita(&self) -> u32 {
        self.tipo_comunita
    }
    fn fonte(&self) -> String {
        self.fonte.clone()
    }
    fn numero_protocollo(&self) -> String {
        self.numero_protocollo.clone()
    }
    fn idro_eco_regione(&self) -> u32 {
        self.idro_eco_regione
    }
    fn area_alpina(&self) -> u32 {
        self.area_alpina
    }
    fn nome_bacino(&self) -> String {
        self.nome_bacino.clone()
    }
}

impl fmt::Display for PlainRecordAnagraficaNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordAnagraficaNISECI: {{ codice_stazione: [{}], corpo_idrico: [{}],\
            regione: [{}], provincia: [{}], data: [{}], lunghezza_stazione: [{}],\
            larghezza_stazione: [{}], tipo_comunita [{}], fonte [{}],\
            numero_protocollo: [{}], idro_eco_regione: [{}],\
            area_alpina: [{}], nome_bacino: [{}]}}",
            self.codice_stazione,
            self.corpo_idrico,
            self.regione,
            self.provincia,
            self.data,
            self.lunghezza_stazione,
            self.larghezza_stazione,
            self.tipo_comunita,
            self.fonte,
            self.numero_protocollo,
            self.idro_eco_regione,
            self.area_alpina,
            self.nome_bacino
        );
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlainRecordCampionamentoHFBI {
    pub codice_specie: String,
    pub numero_individui: u32,
    pub peso: f32,
}

impl RecordCampionamentoHFBI for PlainRecordCampionamentoHFBI {
    fn codice_specie(&self) -> String {
        self.codice_specie.clone()
    }
    fn numero_individui(&self) -> u32 {
        self.numero_individui
    }
    fn peso(&self) -> f32 {
        self.peso
    }
}

impl fmt::Display for PlainRecordCampionamentoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCampionamentoHFBI: {{ codice_specie: [{}], numero_individui: [{}], peso: [{}] }}",
              self.codice_specie, self.numero_individui, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlainRecordAnagraficaHFBI {
    pub codice_stazione: String,
    pub corpo_idrico: String,
    pub regione: String,
    pub provincia: String,
    pub data: String,
    pub lunghezza_stazione: f32,
    pub larghezza_stazione: f32,
    pub stagione: u32,
    pub habitat: u32,
    pub tipo_laguna: u32,
}

impl RecordAnagraficaHFBI for PlainRecordAnagraficaHFBI {
    fn codice_stazione(&self) -> String {
        self.codice_stazione.clone()
    }
    fn corpo_idrico(&self) -> String {
        self.corpo_idrico.clone()
    }
    fn regione(&self) -> String {
        self.regione.clone()
    }
    fn provincia(&self) -> String {
        self.provincia.clone()
    }
    fn data(&self) -> String {
        self.data.clone()
    }
    fn lunghezza_stazione(&self) -> f32 {
        self.lunghezza_stazione
    }
    fn larghezza_stazione(&self) -> f32 {
        self.larghezza_stazione
    }
    fn stagione(&self) -> u32 {
        self.stagione
    }
    fn habitat(&self) -> u32 {
        self.habitat
    }
    fn tipo_laguna(&self) -> u32 {
        self.tipo_laguna
    }
}

impl fmt::Display for PlainRecordAnagraficaHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordAnagraficaHFBI: {{ codice_stazione: [{}], corpo_idrico: [{}],\
            regione: [{}], provincia: [{}], data: [{}], lunghezza_stazione: [{}],\
            larghezza_stazione: [{}], stagione [{}], habitat [{}],\
            tipo_laguna: [{}]}}",
            self.codice_stazione,
            self.corpo_idrico,
            self.regione,
            self.provincia,
            self.data,
            self.lunghezza_stazione,
            self.larghezza_stazione,
            self.stagione,
            self.habitat,
            self.tipo_laguna
        );
        write!(f, "{}", string_representation)
    }
}
