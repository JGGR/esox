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

//! Provides custom CSV types for italian-encoded CSV for NISECI.

use crate::csv::deser::{RecordCsv, SemicolonDelimiter};
use crate::csv::stanis::deserialize_comma_f32;
use crate::deser::{RecordAnagraficaHFBI, RecordCampionamentoHFBI};
use std::fmt;

/// Alternative italian-format implementation of CSV records for
/// [`crate::deser::RecordCampionamentoHFBI`].
/// Currently allows unknown fields; will switch to
/// `#[serde(deny_unknown_fields)]` in a future release.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VeryItalianRecordCampionamentoHFBI {
    /// Species id.
    pub codice_specie: String,
    /// Number of individuals.
    pub numero_individui: u32,
    /// Weight.
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub peso: f32,
}

impl RecordCampionamentoHFBI for VeryItalianRecordCampionamentoHFBI {
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

impl RecordCsv for VeryItalianRecordCampionamentoHFBI {
    type D = SemicolonDelimiter;
}

impl fmt::Display for VeryItalianRecordCampionamentoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCampionamentoHFBI: {{ codice_specie: [{}], numero_individui: [{}], peso: [{}] }}",
              self.codice_specie, self.numero_individui, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

/// Alternative italian-format implementation of CSV records for
/// [`crate::deser::RecordAnagraficaHFBI`].
/// Currently allows unknown fields; will switch to
/// `#[serde(deny_unknown_fields)]` in a future release.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VeryItalianRecordAnagraficaHFBI {
    /// Station code.
    pub codice_stazione: String,
    /// Water body.
    pub corpo_idrico: String,
    /// Region.
    pub regione: String,
    /// Province.
    pub provincia: String,
    /// Formatted date string.
    pub data: String,
    /// Length of transet.
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub lunghezza_stazione: f32,
    /// Width of transet.
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub larghezza_stazione: f32,
    /// Season, relates to [`crate::domain::hfbi::StagioneHFBI`].
    pub stagione: u32,
    /// Habitat, relates to [`crate::domain::hfbi::HabitatHFBI`].
    pub habitat: u32,
    /// Lagoon type.
    pub tipo_laguna: u32,
}

impl RecordAnagraficaHFBI for VeryItalianRecordAnagraficaHFBI {
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

impl RecordCsv for VeryItalianRecordAnagraficaHFBI {
    type D = SemicolonDelimiter;
}

impl fmt::Display for VeryItalianRecordAnagraficaHFBI {
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
