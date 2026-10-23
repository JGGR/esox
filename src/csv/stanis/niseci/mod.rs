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
use crate::deser::{RecordAnagraficaNISECI, RecordCampionamentoNISECI, RecordRiferimentoNISECI};
use std::fmt;

/// Alternative italian-format implementation of CSV records for [`crate::deser::RecordRiferimentoNISECI`].
/// Currently allows unknown fields; will switch to
/// `#[serde(deny_unknown_fields)]` in a future release.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VeryItalianRecordRiferimentoNISECI {
    /// Common name.
    pub nome_comune: String,
    /// Latin name.
    pub nome_latino: String,
    /// Species id.
    pub codice_specie: String,
    /// Origin (one of `"AUT"` or `"ALL"`).
    pub origine: String,
    /// Autochthonous type.
    pub tipo_autoctono: u32,
    /// Allochthonous type.
    pub allo_nocivita: u32,
    /// Expected species flag.
    pub specie_attesa: u32,
    /// Threshold for class 1 of [`crate::domain::niseci::ClassiEtaSpecieNISECI`].
    pub cl_soglia1: u32, // in mm
    /// Threshold for class 2 of [`crate::domain::niseci::ClassiEtaSpecieNISECI`].
    pub cl_soglia2: u32, // in mm
    /// Threshold for class 3 of [`crate::domain::niseci::ClassiEtaSpecieNISECI`].
    pub cl_soglia3: u32, // in mm
    /// Threshold for class 4 of [`crate::domain::niseci::ClassiEtaSpecieNISECI`].
    pub cl_soglia4: u32, // in mm
    /// Threshold for `ad_juv_soglia_1`.
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub ad_juv_soglia1: f32,
    /// Threshold for `ad_juv_soglia_2`.
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub ad_juv_soglia2: f32,
    /// Threshold for `ad_juv_soglia_3`.
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub ad_juv_soglia3: f32,
    /// Threshold for `ad_juv_soglia_4`.
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub ad_juv_soglia4: f32,
    /// Threshold for `dens_soglia_1`.
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub dens_soglia1: f32,
    /// Threshold for `dens_soglia_2`.
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub dens_soglia2: f32,
}

impl RecordRiferimentoNISECI for VeryItalianRecordRiferimentoNISECI {
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

impl RecordCsv for VeryItalianRecordRiferimentoNISECI {
    type D = SemicolonDelimiter;
}

impl fmt::Display for VeryItalianRecordRiferimentoNISECI {
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

/// Alternative italian-format implementation of CSV records for [`crate::deser::RecordCampionamentoNISECI`].
/// Currently allows unknown fields; will switch to
/// `#[serde(deny_unknown_fields)]` in a future release.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VeryItalianRecordCampionamentoNISECI {
    /// Formatted date string.
    pub data: String,
    /// Station code.
    pub stazione: String,
    /// Passage number.
    pub num_passaggio: u32,
    /// Species id.
    pub codice_specie: String,
    /// Length of record.
    pub lunghezza: u32,
    /// Weight of record.
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub peso: f32,
}

impl RecordCampionamentoNISECI for VeryItalianRecordCampionamentoNISECI {
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

impl RecordCsv for VeryItalianRecordCampionamentoNISECI {
    type D = SemicolonDelimiter;
}

impl fmt::Display for VeryItalianRecordCampionamentoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCampionamentoNISECI: {{ data: [{}], stazione: [{}], num_passaggio: [{}], codice_specie: [{}], lunghezza: [{}], peso: [{}] }}",
              self.data, self.stazione, self.num_passaggio,
              self.codice_specie, self.lunghezza, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

/// Alternative italian-format implementation of CSV records for
/// [`crate::deser::RecordAnagraficaNISECI`].
/// Currently allows unknown fields; will switch to
/// `#[serde(deny_unknown_fields)]` in a future release.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VeryItalianRecordAnagraficaNISECI {
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
    /// Station length.
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub lunghezza_stazione: f32,
    /// Station width.
    #[serde(deserialize_with = "deserialize_comma_f32")]
    pub larghezza_stazione: f32,
    /// Community kind, relates to [`crate::domain::niseci::TipoComunitaNISECI`].
    pub tipo_comunita: u32,
    /// Source.
    pub fonte: String,
    /// Protocol number.
    pub numero_protocollo: String,
    /// Hydro-eco region.
    pub idro_eco_regione: u32,
    /// Alpine area flag, relates to [`crate::domain::niseci::AreaNISECI`].
    pub area_alpina: u32,
    /// Water basin.
    pub nome_bacino: String,
}

impl RecordAnagraficaNISECI for VeryItalianRecordAnagraficaNISECI {
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

impl RecordCsv for VeryItalianRecordAnagraficaNISECI {
    type D = SemicolonDelimiter;
}

impl fmt::Display for VeryItalianRecordAnagraficaNISECI {
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
