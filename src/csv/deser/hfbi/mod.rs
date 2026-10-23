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

//! CSV deserialization routines for HFBI.
//! Output is meant to be consumed by [`crate::parser::hfbi`] routines.

use crate::csv::deser::{
    check_path_is_file_ends_with_csv, CsvConfig, DefaultRecordCsv, NormalizerReader, RecordCsv,
};

use crate::deser::limits::{with_limited_reader, ByteLimit, DefaultByteLimit};
use crate::deser::{
    check_serialized_records, parse_serialized_records, RecordAnagraficaHFBI,
    RecordCampionamentoHFBI,
};
use std::fmt;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

/// Default implementation of CSV records for [`crate::deser::RecordCampionamentoHFBI`].
/// Currently allows unknown fields; will switch to
/// `#[serde(deny_unknown_fields)]` in a future release.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlainRecordCsvCampionamentoHFBI {
    /// Species id.
    pub codice_specie: String,
    /// Number of individuals.
    pub numero_individui: u32,
    /// Weight.
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

impl DefaultRecordCsv for PlainRecordCsvCampionamentoHFBI {}

impl fmt::Display for PlainRecordCsvCampionamentoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCampionamentoHFBI: {{ codice_specie: [{}], numero_individui: [{}], peso: [{}] }}",
              self.codice_specie, self.numero_individui, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

/// Generic parsing of [`crate::deser::RecordCampionamentoHFBI`], returning a
/// tuple of vecs for values and errors.
/// If you don't care about partial successes, use [`check_campionamento_hfbi_reader`].
pub fn parse_csv_campionamento_hfbi<R, T>(mut rdr: csv::Reader<R>) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordCampionamentoHFBI,
{
    let iter = rdr.deserialize();
    parse_serialized_records(iter)
}

/// Generic parsing of [`crate::deser::RecordCampionamentoHFBI`].
/// If you care about partial successes, use [`parse_csv_campionamento_hfbi`].
pub fn check_campionamento_hfbi_reader<R: Read, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoHFBI + RecordCsv,
{
    private_check_campionamento_hfbi_reader_conf::<R, DefaultByteLimit, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(T::DELIMITER)
            .with_headers(has_headers),
    )
}

/// Generic parsing of [`crate::deser::RecordCampionamentoHFBI`].
/// Uses [`crate::csv::deser::CsvConfig`].
/// If you care about partial successes, use [`parse_csv_campionamento_hfbi`].
pub fn check_campionamento_hfbi_reader_conf<R: Read, T>(
    reader: R,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoHFBI,
{
    private_check_campionamento_hfbi_reader_conf::<R, DefaultByteLimit, T>(reader, config)
}

fn private_check_campionamento_hfbi_reader_conf<R: Read, BL: ByteLimit, T>(
    reader: R,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoHFBI,
{
    let normalizing_reader = NormalizerReader::new(reader).take(BL::MAX_BYTES);
    with_limited_reader(
        normalizing_reader,
        BL::MAX_BYTES,
        |limited_reader| {
            let mut rdr = csv::ReaderBuilder::new()
                .delimiter(config.delimiter())
                .has_headers(config.has_headers())
                .from_reader(limited_reader);
            let iter = rdr.deserialize();
            check_serialized_records(iter)
        },
        |limit_error| vec![csv::Error::from(limit_error)],
    )
}

/// Generic parsing of [`crate::deser::RecordCampionamentoHFBI`] from [`PathBuf`].
pub fn check_campionamento_hfbi_path<T>(
    path: PathBuf,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoHFBI + RecordCsv,
{
    check_campionamento_hfbi_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(T::DELIMITER)
            .with_headers(has_headers),
    )
}

/// Generic parsing of [`crate::deser::RecordCampionamentoHFBI`] from [`PathBuf`].
/// Uses [`crate::csv::deser::CsvConfig`].
pub fn check_campionamento_hfbi_path_conf<T>(
    path: PathBuf,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoHFBI,
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
    private_check_campionamento_hfbi_reader_conf::<File, DefaultByteLimit, T>(file, config)
}

/// Default implementation of CSV records for [`crate::deser::RecordAnagraficaHFBI`].
/// Currently allows unknown fields; will switch to
/// `#[serde(deny_unknown_fields)]` in a future release.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlainRecordCsvAnagraficaHFBI {
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
    pub lunghezza_stazione: f32,
    /// Width of transet.
    pub larghezza_stazione: f32,
    /// Season, relates to [`crate::domain::hfbi::StagioneHFBI`].
    pub stagione: u32,
    /// Habitat, relates to [`crate::domain::hfbi::HabitatHFBI`].
    pub habitat: u32,
    /// Lagoon type.
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

impl DefaultRecordCsv for PlainRecordCsvAnagraficaHFBI {}

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

/// Generic parsing of [`crate::deser::RecordAnagraficaHFBI`], returning a
/// tuple of vecs for values and errors.
/// If you don't care about partial successes, use [`check_anagrafica_hfbi_reader`].
pub fn parse_csv_anagrafica_hfbi<R, T>(mut rdr: csv::Reader<R>) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordAnagraficaHFBI,
{
    let iter = rdr.deserialize();
    parse_serialized_records(iter)
}

/// Generic parsing of [`crate::deser::RecordAnagraficaHFBI`].
/// If you care about partial successes, use [`parse_csv_anagrafica_hfbi`].
pub fn check_anagrafica_hfbi_reader<R: Read, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaHFBI + RecordCsv,
{
    private_check_anagrafica_hfbi_reader_conf::<R, DefaultByteLimit, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(T::DELIMITER)
            .with_headers(has_headers),
    )
}

/// Generic parsing of [`crate::deser::RecordAnagraficaHFBI`].
/// Uses [`crate::csv::deser::CsvConfig`].
/// If you care about partial successes, use [`parse_csv_anagrafica_hfbi`].
pub fn check_anagrafica_hfbi_reader_conf<R: Read, T>(
    reader: R,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaHFBI,
{
    private_check_anagrafica_hfbi_reader_conf::<R, DefaultByteLimit, T>(reader, config)
}

fn private_check_anagrafica_hfbi_reader_conf<R: Read, BL: ByteLimit, T>(
    reader: R,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaHFBI,
{
    let normalizing_reader = NormalizerReader::new(reader).take(BL::MAX_BYTES);
    with_limited_reader(
        normalizing_reader,
        BL::MAX_BYTES,
        |limited_reader| {
            let mut rdr = csv::ReaderBuilder::new()
                .delimiter(config.delimiter())
                .has_headers(config.has_headers())
                .from_reader(limited_reader);
            let iter = rdr.deserialize();
            check_serialized_records(iter)
        },
        |limit_error| vec![csv::Error::from(limit_error)],
    )
}

/// Generic parsing of [`crate::deser::RecordAnagraficaHFBI`] from [`PathBuf`].
pub fn check_anagrafica_hfbi_path<T>(
    path: PathBuf,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaHFBI + RecordCsv,
{
    check_anagrafica_hfbi_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(T::DELIMITER)
            .with_headers(has_headers),
    )
}

/// Generic parsing of [`crate::deser::RecordAnagraficaHFBI`] from [`PathBuf`].
/// Uses [`crate::csv::deser::CsvConfig`].
pub fn check_anagrafica_hfbi_path_conf<T>(
    path: PathBuf,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaHFBI,
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
    private_check_anagrafica_hfbi_reader_conf::<File, DefaultByteLimit, T>(file, config)
}
