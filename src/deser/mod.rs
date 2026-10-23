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
//! Deserialization module for intermediate structs.
//!
//! This module provides generic support for:
//! - Converting raw data into typed intermediate structs
//! - Validating raw data structure and field types
//!   - Eg. Integer fields must not be `"foo"`)
//!
//! To validate intermediate structs into [`crate::domain`] structs, see:
//! - [`crate::parser`]

pub(crate) mod limits;

use std::fmt;

/// Denotes record kinds.
#[derive(Copy, Clone)]
pub enum TipoRecord {
    RiferimentoNISECI,
    CampionamentoNISECI,
    AnagraficaNISECI,
    CampionamentoHFBI,
    AnagraficaHFBI,
}

/// Defines expected methods for a deserialized reference set record of NISECI.
pub trait RecordRiferimentoNISECI: serde::de::DeserializeOwned {
    /// Getter for common name.
    fn nome_comune(&self) -> String;
    /// Getter for latin name.
    fn nome_latino(&self) -> String;
    /// Getter for species id.
    fn codice_specie(&self) -> String;
    /// Getter for origin (one of `"AUT"` and `"ALL"`).
    fn origine(&self) -> String;
    /// Getter for autochthonous type.
    fn tipo_autoctono(&self) -> u32;
    /// Getter for allochthonous type.
    fn allo_nocivita(&self) -> u32;
    /// Getter for expected species flag.
    fn specie_attesa(&self) -> u32;
    /// Getter for threshold for class 1 of [`crate::domain::niseci::ClassiEtaSpecieNISECI`].
    fn cl_soglia1(&self) -> u32;
    /// Getter for threshold for class 2 of [`crate::domain::niseci::ClassiEtaSpecieNISECI`].
    fn cl_soglia2(&self) -> u32;
    /// Getter for threshold for class 3 of [`crate::domain::niseci::ClassiEtaSpecieNISECI`].
    fn cl_soglia3(&self) -> u32;
    /// Getter for threshold for class 4 of [`crate::domain::niseci::ClassiEtaSpecieNISECI`].
    fn cl_soglia4(&self) -> u32;
    /// Getter for threshold for `ad_juv_soglia_1`.
    fn ad_juv_soglia1(&self) -> f32;
    /// Getter for threshold for `ad_juv_soglia_2`.
    fn ad_juv_soglia2(&self) -> f32;
    /// Getter for threshold for `ad_juv_soglia_3`.
    fn ad_juv_soglia3(&self) -> f32;
    /// Getter for threshold for `ad_juv_soglia_4`.
    fn ad_juv_soglia4(&self) -> f32;
    /// Getter for threshold for `dens_soglia_1`.
    fn dens_soglia1(&self) -> f32;
    /// Getter for threshold for `dens_soglia_2`.
    fn dens_soglia2(&self) -> f32;
}

/// Defines expected methods for a deserialized capture set record of NISECI.
pub trait RecordCampionamentoNISECI: serde::de::DeserializeOwned {
    /// Getter for formatted date string.
    fn data(&self) -> String;
    /// Getter for station name.
    fn stazione(&self) -> String;
    /// Getter for passage number.
    fn num_passaggio(&self) -> u32;
    /// Getter for species id.
    fn codice_specie(&self) -> String;
    /// Getter for length of record.
    fn lunghezza(&self) -> u32;
    /// Getter for weight of record.
    fn peso(&self) -> f32;
}

/// Defines expected methods for a deserialized additional info record of NISECI.
pub trait RecordAnagraficaNISECI: serde::de::DeserializeOwned {
    /// Getter for station code.
    fn codice_stazione(&self) -> String;
    /// Getter for water body name.
    fn corpo_idrico(&self) -> String;
    /// Getter for region name.
    fn regione(&self) -> String;
    /// Getter for province name.
    fn provincia(&self) -> String;
    /// Getter for formatted date string.
    fn data(&self) -> String;
    /// Getter for station length.
    fn lunghezza_stazione(&self) -> f32;
    /// Getter for station width.
    fn larghezza_stazione(&self) -> f32;
    /// Getter for community kind.
    fn tipo_comunita(&self) -> u32;
    /// Getter for source.
    fn fonte(&self) -> String;
    /// Getter for protocol number.
    fn numero_protocollo(&self) -> String;
    /// Getter for hydro-eco region.
    fn idro_eco_regione(&self) -> u32;
    /// Getter for alpine region flag.
    fn area_alpina(&self) -> u32;
    /// Getter for water basin name.
    fn nome_bacino(&self) -> String;
}

/// Defines expected methods for a deserialized capture set record of HFBI.
pub trait RecordCampionamentoHFBI: serde::de::DeserializeOwned {
    /// Getter for species id.
    fn codice_specie(&self) -> String;
    /// Getter for number of individuals.
    fn numero_individui(&self) -> u32;
    /// Getter for weight.
    fn peso(&self) -> f32;
}

/// Defines expected methods for a deserialized additional info record of HFBI.
pub trait RecordAnagraficaHFBI: serde::de::DeserializeOwned {
    /// Getter for station code.
    fn codice_stazione(&self) -> String;
    /// Getter for water body name.
    fn corpo_idrico(&self) -> String;
    /// Getter for region name.
    fn regione(&self) -> String;
    /// Getter for province name.
    fn provincia(&self) -> String;
    /// Getter for formatted date string.
    fn data(&self) -> String;
    /// Getter for station length.
    fn lunghezza_stazione(&self) -> f32;
    /// Getter for station width.
    fn larghezza_stazione(&self) -> f32;
    /// Getter for season. Relates to [`crate::domain::hfbi::StagioneHFBI`].
    fn stagione(&self) -> u32;
    /// Getter for habitat. Relates to [`crate::domain::hfbi::HabitatHFBI`].
    fn habitat(&self) -> u32;
    /// Getter for lagoon type.
    fn tipo_laguna(&self) -> u32;
}

/// Generic collecting of iterable [`Result`] into a tuple of vectors for
/// values and errors.
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

/// Generic collecting of iterable [`Result`] into a vector of either
/// values or errors.
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

/// Generic collecting of iterable [`Result`] into a vector of either
/// values or errors. When the error case is reached, `on_error` is called.
#[deprecated(
    note = "Use `check_serialized_records` and handle errors explicitly instead.\nItalian formatting can be used from esox::csv::stanis::giorgio::format_csv_errors"
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

/// Default implementation of [`RecordRiferimentoNISECI`].
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlainRecordRiferimentoNISECI {
    /// Common name.
    pub nome_comune: String,
    /// Latin name.
    pub nome_latino: String,
    /// Species id.
    pub codice_specie: String,
    /// Origin (should be one of `"AUT"` or `"ALL"`).
    pub origine: String,
    /// Autochthonous type.
    pub tipo_autoctono: u32,
    /// Autochthonous type.
    pub allo_nocivita: u32,
    /// Allochthonous type.
    pub specie_attesa: u32,
    /// Threshold for class `1` of [`crate::domain::niseci::SpecieNISECI`].
    pub cl_soglia1: u32, // in mm
    /// Threshold for class `2` of [`crate::domain::niseci::SpecieNISECI`].
    pub cl_soglia2: u32, // in mm
    /// Threshold for class `3` of [`crate::domain::niseci::SpecieNISECI`].
    pub cl_soglia3: u32, // in mm
    /// Threshold for class `4` of [`crate::domain::niseci::SpecieNISECI`].
    pub cl_soglia4: u32, // in mm
    /// Threshold for `ad_juv_soglia_1` of [`crate::domain::niseci::SpecieNISECI`].
    pub ad_juv_soglia1: f32,
    /// Threshold for `ad_juv_soglia_2` of [`crate::domain::niseci::SpecieNISECI`].
    pub ad_juv_soglia2: f32,
    /// Threshold for `ad_juv_soglia_3` of [`crate::domain::niseci::SpecieNISECI`].
    pub ad_juv_soglia3: f32,
    /// Threshold for `ad_juv_soglia_4` of [`crate::domain::niseci::SpecieNISECI`].
    pub ad_juv_soglia4: f32,
    /// Threshold for `dens_soglia_1` of [`crate::domain::niseci::SpecieNISECI`].
    pub dens_soglia1: f32,
    /// Threshold for `dens_soglia_2` of [`crate::domain::niseci::SpecieNISECI`].
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

/// Default implementation of [`RecordCampionamentoNISECI`].
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlainRecordCampionamentoNISECI {
    /// Date of capture.
    pub data: String,
    /// Station of capture.
    pub stazione: String,
    /// Passage number of capture.
    pub num_passaggio: u32,
    /// Species id of capture.
    pub codice_specie: String,
    /// Length of capture.
    pub lunghezza: u32,
    /// Weight of capture.
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

/// Default implementation of [`RecordAnagraficaNISECI`].
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlainRecordAnagraficaNISECI {
    /// Station code.
    pub codice_stazione: String,
    /// Water body.
    pub corpo_idrico: String,
    /// Region of capture set.
    pub regione: String,
    /// Province of capture set.
    pub provincia: String,
    /// Date of capture set.
    pub data: String,
    /// Length of station.
    pub lunghezza_stazione: f32,
    /// Width of station.
    pub larghezza_stazione: f32,
    /// Community kind, relates to [`crate::domain::niseci::TipoComunitaNISECI`].
    pub tipo_comunita: u32,
    /// Source for the community.
    pub fonte: String,
    /// Protocol number for the community.
    pub numero_protocollo: String,
    /// Hydro-eco region.
    pub idro_eco_regione: u32,
    /// Alpine area flag, relates to [`crate::domain::niseci::AreaNISECI`].
    pub area_alpina: u32,
    /// Water basin.
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

/// Default implementation of [`RecordCampionamentoHFBI`].
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlainRecordCampionamentoHFBI {
    /// Species id.
    pub codice_specie: String,
    /// Number of individuals.
    pub numero_individui: u32,
    /// Weight.
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

/// Default implementation of [`RecordAnagraficaHFBI`].
#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlainRecordAnagraficaHFBI {
    /// Station code.
    pub codice_stazione: String,
    /// Water body.
    pub corpo_idrico: String,
    /// Region of capture set.
    pub regione: String,
    /// Province of capture set.
    pub provincia: String,
    /// Date of capture set.
    pub data: String,
    /// Length of transept.
    pub lunghezza_stazione: f32,
    /// Width of transept.
    pub larghezza_stazione: f32,
    /// Season, relates to [`crate::domain::hfbi::StagioneHFBI`].
    pub stagione: u32,
    /// Habitat, relates to [`crate::domain::hfbi::HabitatHFBI`].
    pub habitat: u32,
    /// Lagoon type.
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
