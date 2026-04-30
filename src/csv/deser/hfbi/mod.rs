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
use crate::csv::deser::{check_path_is_file_ends_with_csv, CsvConfig, NormalizerReader};

/// Used as the closure argument for
/// validate_serialized_records(), to print italian error messages.
use crate::csv::deser::error::duccio::csv_error_handler;
/// v0.2 will drop implicit logging, hence this method will not be needed anymore.
/// Callsites will switch to crate::deser::check_serialized_records.
/// Usercode will need to handle the format/printing of errors separately.
#[allow(deprecated)]
use crate::deser::validate_serialized_records;

use crate::deser::{
    parse_serialized_records, RecordAnagraficaHFBI, RecordCampionamentoHFBI, TipoRecord,
};
use std::any::TypeId;
use std::fmt;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using crate::csv::stanis::hfbi::VeryItalianRecordCampionamentoHFBI instead"
)]
pub use crate::csv::stanis::hfbi::VeryItalianRecordCampionamentoHFBI as VeryItalianRecordCsvCampionamentoHFBI;
use crate::csv::stanis::hfbi::VeryItalianRecordCampionamentoHFBI;

/// Currently allows unknown fields; will switch to
/// `#[serde(deny_unknown_fields)]` in a future release.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlainRecordCsvCampionamentoHFBI {
    pub codice_specie: String,
    pub numero_individui: u32,
    pub peso: f32,
}

impl RecordCampionamentoHFBI for PlainRecordCsvCampionamentoHFBI {
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

impl fmt::Display for PlainRecordCsvCampionamentoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCampionamentoHFBI: {{ codice_specie: [{}], numero_individui: [{}], peso: [{}] }}",
              self.codice_specie, self.numero_individui, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

pub fn parse_csv_campionamento_hfbi<R, T>(mut rdr: csv::Reader<R>) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordCampionamentoHFBI + 'static,
{
    let iter = rdr.deserialize();
    parse_serialized_records(iter)
}

#[deprecated(
    note = "v0.2 will change signature to expect an explicit delimiter argument.\nConsider using crate::csv::deser::hfbi::check_campionamento_hfbi_reader_conf() instead"
)]
pub fn check_campionamento_hfbi_reader<R: Read, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoHFBI + 'static,
{
    let type_id = TypeId::of::<T>(); // Get the TypeId of T at runtime

    // Match on the TypeId to determine the actual type of T
    let delimiter = match type_id {
        id if id == TypeId::of::<VeryItalianRecordCampionamentoHFBI>() => b';',
        _ => b',',
    };

    check_campionamento_hfbi_reader_conf::<R, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(delimiter)
            .with_headers(has_headers),
    )
}

pub fn check_campionamento_hfbi_reader_conf<R: Read, T>(
    reader: R,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoHFBI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(config.delimiter())
        .has_headers(config.has_headers())
        .from_reader(normalizing_reader);
    let iter = rdr.deserialize();
    #[allow(deprecated)]
    validate_serialized_records(iter, |errors| {
        /*
        for error in &errors {
            eprintln!("  {}", error);
        }
        let processed_errors = process_csv_errors(errors, TipoRecord::CampionamentoHFBI);
        eprintln!("Errori incontrati durante l'elaborazione csv del campionamento HFBI: {{");
        for e in processed_errors {
            eprintln!("{e}");
        }
        eprintln!("}}");
        */
        csv_error_handler(TipoRecord::CampionamentoHFBI)(errors);
    })
}

#[deprecated(
    note = "v0.2 will change signature to expect an explicit delimiter argument.\nConsider using crate::csv::deser::hfbi::check_campionamento_hfbi_path_conf() instead"
)]
pub fn check_campionamento_hfbi_path<T>(
    path: PathBuf,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoHFBI + 'static,
{
    let type_id = TypeId::of::<T>(); // Get the TypeId of T at runtime

    // Match on the TypeId to determine the actual type of T
    let delimiter = match type_id {
        id if id == TypeId::of::<VeryItalianRecordCampionamentoHFBI>() => b';',
        _ => b',',
    };

    check_campionamento_hfbi_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(delimiter)
            .with_headers(has_headers),
    )
}

pub fn check_campionamento_hfbi_path_conf<T>(
    path: PathBuf,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoHFBI + 'static,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::other(
            "Errore campionamento HFBI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_campionamento_hfbi_reader_conf(file, config)
}

#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using crate::csv::stanis::hfbi::VeryItalianRecordAnagraficaHFBI instead"
)]
pub use crate::csv::stanis::hfbi::VeryItalianRecordAnagraficaHFBI as VeryItalianRecordCsvAnagraficaHFBI;
use crate::csv::stanis::hfbi::VeryItalianRecordAnagraficaHFBI;

/// Currently allows unknown fields; will switch to
/// `#[serde(deny_unknown_fields)]` in a future release.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlainRecordCsvAnagraficaHFBI {
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

impl RecordAnagraficaHFBI for PlainRecordCsvAnagraficaHFBI {
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

impl fmt::Display for PlainRecordCsvAnagraficaHFBI {
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

pub fn parse_csv_anagrafica_hfbi<R, T>(mut rdr: csv::Reader<R>) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordAnagraficaHFBI,
{
    let iter = rdr.deserialize();
    parse_serialized_records(iter)
}

#[deprecated(
    note = "v0.2 will change signature to expect an explicit delimiter argument.\nConsider using crate::csv::deser::hfbi::check_anagrafica_hfbi_reader_conf() instead"
)]
pub fn check_anagrafica_hfbi_reader<R: Read, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaHFBI + 'static,
{
    let type_id = TypeId::of::<T>(); // Get the TypeId of T at runtime

    // Match on the TypeId to determine the actual type of T
    let delimiter = match type_id {
        id if id == TypeId::of::<VeryItalianRecordAnagraficaHFBI>() => b';',
        _ => b',',
    };

    check_anagrafica_hfbi_reader_conf::<R, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(delimiter)
            .with_headers(has_headers),
    )
}

pub fn check_anagrafica_hfbi_reader_conf<R: Read, T>(
    reader: R,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaHFBI + 'static,
{
    let normalizing_reader = NormalizerReader::new(reader);

    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(config.delimiter())
        .has_headers(config.has_headers())
        .from_reader(normalizing_reader);
    let iter = rdr.deserialize();
    #[allow(deprecated)]
    validate_serialized_records(iter, |errors| {
        /*
        for error in &errors {
            eprintln!("  {}", error);
        }
        let processed_errors = process_csv_errors(errors, TipoRecord::AnagraficaHFBI);
        eprintln!("Errori incontrati durante l'elaborazione csv dell' anagrafica HFBI: {{");
        for e in processed_errors {
            eprintln!("{e}");
        }
        eprintln!("}}");
        */
        csv_error_handler(TipoRecord::AnagraficaHFBI)(errors);
    })
}

#[deprecated(
    note = "v0.2 will change signature to expect an explicit delimiter argument.\nConsider using crate::csv::deser::hfbi::check_anagrafica_hfbi_path_conf() instead"
)]
pub fn check_anagrafica_hfbi_path<T>(
    path: PathBuf,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaHFBI + 'static,
{
    let type_id = TypeId::of::<T>(); // Get the TypeId of T at runtime

    // Match on the TypeId to determine the actual type of T
    let delimiter = match type_id {
        id if id == TypeId::of::<VeryItalianRecordAnagraficaHFBI>() => b';',
        _ => b',',
    };

    check_anagrafica_hfbi_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(delimiter)
            .with_headers(has_headers),
    )
}

pub fn check_anagrafica_hfbi_path_conf<T>(
    path: PathBuf,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaHFBI + 'static,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::other(
            "Errore anagrafica HFBI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    check_anagrafica_hfbi_reader_conf(file, config)
}
