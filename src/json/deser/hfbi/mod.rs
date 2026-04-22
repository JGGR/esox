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

use crate::deser::{
    parse_serialized_records, validate_serialized_records, RecordAnagraficaHFBI,
    RecordCampionamentoHFBI,
};
use crate::json::deser::JsonPathCheckError;
use serde_json::Deserializer;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlainRecordJsonCampionamentoHFBI {
    pub codice_specie: String,
    pub numero_individui: u32,
    pub peso: f32,
}

impl RecordCampionamentoHFBI for PlainRecordJsonCampionamentoHFBI {
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

impl fmt::Display for PlainRecordJsonCampionamentoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCampionamentoHFBI: {{ codice_specie: [{}], numero_individui: [{}], peso: [{}] }}",
              self.codice_specie, self.numero_individui, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

pub fn parse_json_campionamento_hfbi<R, T>(reader: R) -> (Vec<T>, Vec<serde_json::Error>)
where
    R: std::io::Read,
    T: RecordCampionamentoHFBI,
{
    let iter = Deserializer::from_reader(reader).into_iter::<T>();
    parse_serialized_records(iter)
}

pub fn check_campionamento_hfbi_reader<R: Read, T>(
    reader: R,
) -> Result<Vec<T>, Vec<serde_json::Error>>
where
    T: RecordCampionamentoHFBI,
{
    let iter = Deserializer::from_reader(reader).into_iter::<T>();
    validate_serialized_records(iter, |errors| {
        for error in errors {
            eprintln!("  {}", error);
        }
    })
}

pub fn check_campionamento_hfbi_path<T>(
    path: impl AsRef<Path>,
) -> Result<Vec<T>, JsonPathCheckError>
where
    T: RecordCampionamentoHFBI,
{
    let file = File::open(path)?;
    check_campionamento_hfbi_reader(file).map_err(JsonPathCheckError::Json)
}

#[derive(Debug, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PlainRecordJsonAnagraficaHFBI {
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

impl RecordAnagraficaHFBI for PlainRecordJsonAnagraficaHFBI {
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

impl fmt::Display for PlainRecordJsonAnagraficaHFBI {
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

pub fn parse_json_anagrafica_hfbi<R, T>(reader: R) -> (Vec<T>, Vec<serde_json::Error>)
where
    R: std::io::Read,
    T: RecordAnagraficaHFBI,
{
    let iter = Deserializer::from_reader(reader).into_iter::<T>();
    parse_serialized_records(iter)
}

pub fn check_anagrafica_hfbi_reader<R: Read, T>(reader: R) -> Result<Vec<T>, Vec<serde_json::Error>>
where
    T: RecordAnagraficaHFBI,
{
    let iter = Deserializer::from_reader(reader).into_iter::<T>();
    validate_serialized_records(iter, |errors| {
        for error in errors {
            eprintln!("  {}", error);
        }
    })
}

pub fn check_anagrafica_hfbi_path<T>(path: impl AsRef<Path>) -> Result<Vec<T>, JsonPathCheckError>
where
    T: RecordAnagraficaHFBI,
{
    let file = File::open(path)?;
    check_anagrafica_hfbi_reader(file).map_err(JsonPathCheckError::Json)
}
