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

use crate::csv::deser::{
    check_path_is_file_ends_with_csv, CsvConfig, DefaultRecordCsv, Delimiter, NormalizerReader,
    RecordCsv,
};

use crate::deser::limits::{with_limited_reader, ByteLimit, DefaultByteLimit};
use crate::deser::{
    check_serialized_records, parse_serialized_records, RecordAnagraficaNISECI,
    RecordCampionamentoNISECI, RecordRiferimentoNISECI,
};
use std::fmt;
use std::fs::File;
use std::io::{Error, Read};
use std::path::PathBuf;

/// Currently allows unknown fields; will switch to
/// `#[serde(deny_unknown_fields)]` in a future release.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlainRecordCsvRiferimentoNISECI {
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

impl RecordRiferimentoNISECI for PlainRecordCsvRiferimentoNISECI {
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

impl DefaultRecordCsv for PlainRecordCsvRiferimentoNISECI {}

impl fmt::Display for PlainRecordCsvRiferimentoNISECI {
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

#[deprecated(
    note = "v0.2 will change signature to add a RecordCsv bound on T.\nConsider adding impl RecordCsv to your custom types.\nExisting provided types will receive it automatically. Consider using crate::csv::deser::niseci::check_riferimento_niseci_reader_conf() if you need runtime delimiter selection instead"
)]
pub fn parse_csv_riferimento_niseci<R, T>(mut rdr: csv::Reader<R>) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordRiferimentoNISECI,
{
    let iter = rdr.deserialize();
    parse_serialized_records(iter)
}

pub fn check_riferimento_niseci_reader<R: Read, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordRiferimentoNISECI + RecordCsv,
{
    let config = CsvConfig::default()
        .with_delimiter(T::D::DELIMITER)
        .with_headers(has_headers);
    private_check_riferimento_niseci_reader::<R, DefaultByteLimit, T>(reader, config)
}

pub fn check_riferimento_niseci_reader_conf<R: Read, T>(
    reader: R,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordRiferimentoNISECI,
{
    private_check_riferimento_niseci_reader::<R, DefaultByteLimit, T>(reader, config)
}

fn private_check_riferimento_niseci_reader<R: Read, BL: ByteLimit, T>(
    reader: R,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordRiferimentoNISECI,
{
    let normalizing_reader = NormalizerReader::new(reader);
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

pub fn check_riferimento_niseci_path<T>(
    path: PathBuf,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordRiferimentoNISECI + RecordCsv,
{
    check_riferimento_niseci_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
    )
}

pub fn check_riferimento_niseci_path_conf<T>(
    path: PathBuf,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordRiferimentoNISECI,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::other(
            "Errore riferimento NISECI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    private_check_riferimento_niseci_reader::<File, DefaultByteLimit, T>(file, config)
}

/// Currently allows unknown fields; will switch to
/// `#[serde(deny_unknown_fields)]` in a future release.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlainRecordCsvCampionamentoNISECI {
    pub data: String,
    pub stazione: String,
    pub num_passaggio: u32,
    pub codice_specie: String,
    pub lunghezza: u32,
    pub peso: f32,
}

impl RecordCampionamentoNISECI for PlainRecordCsvCampionamentoNISECI {
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

impl DefaultRecordCsv for PlainRecordCsvCampionamentoNISECI {}

impl fmt::Display for PlainRecordCsvCampionamentoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordCampionamentoNISECI: {{ data: [{}], stazione: [{}], num_passaggio: [{}], codice_specie: [{}], lunghezza: [{}], peso: [{}] }}",
              self.data, self.stazione, self.num_passaggio,
              self.codice_specie, self.lunghezza, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

#[deprecated(
    note = "v0.2 will change signature to add a RecordCsv bound on T.\nConsider adding impl RecordCsv to your custom types.\nExisting provided types will receive it automatically. Consider using crate::csv::deser::niseci::check_campionamento_niseci_reader_conf() if you need runtime delimiter selection instead"
)]
pub fn parse_csv_campionamento_niseci<R, T>(mut rdr: csv::Reader<R>) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordCampionamentoNISECI,
{
    let iter = rdr.deserialize();
    parse_serialized_records(iter)
}

pub fn check_campionamento_niseci_reader<R: Read, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoNISECI + RecordCsv,
{
    private_check_campionamento_niseci_reader_conf::<R, DefaultByteLimit, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
    )
}

pub fn check_campionamento_niseci_reader_conf<R: Read, T>(
    reader: R,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoNISECI,
{
    private_check_campionamento_niseci_reader_conf::<R, DefaultByteLimit, T>(reader, config)
}

fn private_check_campionamento_niseci_reader_conf<R: Read, BL: ByteLimit, T>(
    reader: R,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoNISECI,
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

pub fn check_campionamento_niseci_path<T>(
    path: PathBuf,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoNISECI + RecordCsv,
{
    check_campionamento_niseci_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
    )
}

pub fn check_campionamento_niseci_path_conf<T>(
    path: PathBuf,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordCampionamentoNISECI,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::other(
            "Errore campionamento NISECI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    private_check_campionamento_niseci_reader_conf::<File, DefaultByteLimit, T>(file, config)
}

/// Currently allows unknown fields; will switch to
/// `#[serde(deny_unknown_fields)]` in a future release.
#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlainRecordCsvAnagraficaNISECI {
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

impl RecordAnagraficaNISECI for PlainRecordCsvAnagraficaNISECI {
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

impl DefaultRecordCsv for PlainRecordCsvAnagraficaNISECI {}

impl fmt::Display for PlainRecordCsvAnagraficaNISECI {
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

#[deprecated(
    note = "v0.2 will change signature to add a RecordCsv bound on T.\nConsider adding impl RecordCsv to your custom types.\nExisting provided types will receive it automatically. Consider using crate::csv::deser::niseci::check_anagrafica_niseci_reader_conf() if you need runtime delimiter selection instead"
)]
pub fn parse_csv_anagrafica_niseci<R, T>(mut rdr: csv::Reader<R>) -> (Vec<T>, Vec<csv::Error>)
where
    R: std::io::Read,
    T: RecordAnagraficaNISECI,
{
    let iter = rdr.deserialize();
    parse_serialized_records(iter)
}

pub fn check_anagrafica_niseci_reader<R: Read, T>(
    reader: R,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaNISECI + RecordCsv,
{
    private_check_anagrafica_niseci_reader_conf::<R, DefaultByteLimit, T>(
        reader,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
    )
}

pub fn check_anagrafica_niseci_reader_conf<R: Read, T>(
    reader: R,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaNISECI,
{
    private_check_anagrafica_niseci_reader_conf::<R, DefaultByteLimit, T>(reader, config)
}

fn private_check_anagrafica_niseci_reader_conf<R: Read, BL: ByteLimit, T>(
    reader: R,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaNISECI,
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

pub fn check_anagrafica_niseci_path<T>(
    path: PathBuf,
    has_headers: bool,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaNISECI + RecordCsv,
{
    check_anagrafica_niseci_path_conf::<T>(
        path,
        CsvConfig::default()
            .with_delimiter(T::D::DELIMITER)
            .with_headers(has_headers),
    )
}

pub fn check_anagrafica_niseci_path_conf<T>(
    path: PathBuf,
    config: CsvConfig,
) -> Result<Vec<T>, Vec<csv::Error>>
where
    T: RecordAnagraficaNISECI,
{
    if !check_path_is_file_ends_with_csv(&path) {
        eprintln!("Il file {} non è un .csv", path.display());
        let err = csv::Error::from(Error::other(
            "Errore anagrafica NISECI: il file non è un .csv",
        ));
        let err_vec: Vec<csv::Error> = vec![err];
        return Err(err_vec);
    }
    let file = File::open(path).expect("Unable to open file");
    private_check_anagrafica_niseci_reader_conf::<File, DefaultByteLimit, T>(file, config)
}
