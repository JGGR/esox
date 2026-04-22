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

pub fn validate_serialized_records<T, E, F>(
    iter: impl IntoIterator<Item = Result<T, E>>,
    on_error: F,
) -> Result<Vec<T>, Vec<E>>
where
    F: Fn(&Vec<E>),
{
    let (records, errors) = parse_serialized_records(iter);

    if !errors.is_empty() {
        on_error(&errors);
        Err(errors)
    } else {
        Ok(records)
    }
}
