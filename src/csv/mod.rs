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

pub mod deser;
pub mod load;
pub mod parser;

// This must be kept aligned with RecordCsvRiferimentoNISECI definition.
// TODO: get this stuff with some macro?
pub const RIFERIMENTO_NISECI_HEADER_FIELDS: [&str; 17] = [
    "nomeComune",
    "nomeLatino",
    "codiceSpecie",
    "origine",
    "tipoAutoctono",
    "alloNocivita",
    "specieAttesa",
    "clSoglia1",
    "clSoglia2",
    "clSoglia3",
    "clSoglia4",
    "adJuvSoglia1",
    "adJuvSoglia2",
    "adJuvSoglia3",
    "adJuvSoglia4",
    "densSoglia1",
    "densSoglia2",
];
pub const RIFERIMENTO_NISECI_HEADER_FIELD_TYPES: [&str; 17] = [
    "String", "String", "String", "String", "u32", "u32", "u32", "u32", "u32", "u32", "u32", "f32",
    "f32", "f32", "f32", "f32", "f32",
];
pub const RIFERIMENTO_NISECI_HEADER: &str = "\
nomeComune;nomeLatino;codiceSpecie;origine;tipoAutoctono;alloNocivita;specieAttesa;clSoglia1;clSoglia2;clSoglia3;clSoglia4;adJuvSoglia1;adJuvSoglia2;adJuvSoglia3;adJuvSoglia4;densSoglia1;densSoglia2";

// This must be kept aligned with RecordCsvCampionamentoNISECI definition.
// TODO: get this stuff with some macro?
pub const CAMPIONAMENTO_NISECI_HEADER_FIELDS: [&str; 6] = [
    "data",
    "stazione",
    "numPassaggio",
    "codiceSpecie",
    "lunghezza",
    "peso",
];
pub const CAMPIONAMENTO_NISECI_HEADER_FIELD_TYPES: [&str; 6] =
    ["String", "String", "u32", "String", "u32", "f32"];
pub const CAMPIONAMENTO_NISECI_HEADER: &str = "\
data;stazione;numPassaggio;codiceSpecie;lunghezza;peso";

// This must be kept aligned with RecordCsvAnagraficaNISECI definition.
// TODO: get this stuff with some macro?
pub const ANAGRAFICA_NISECI_HEADER_FIELDS: [&str; 13] = [
    "codiceStazione",
    "corpoIdrico",
    "regione",
    "provincia",
    "data",
    "lunghezzaStazione",
    "larghezzaStazione",
    "tipoComunita",
    "fonte",
    "numeroProtocollo",
    "idroEcoRegione",
    "areaAlpina",
    "nomeBacino",
];
pub const ANAGRAFICA_NISECI_HEADER_FIELD_TYPES: [&str; 13] = [
    "String", "String", "String", "String", "String", "f32", "f32", "u32", "String", "String",
    "u32", "u32", "String",
];
pub const ANAGRAFICA_NISECI_HEADER: &str = "\
codiceStazione;corpoIdrico;regione;provincia;data;lunghezzaStazione;larghezzaStazione;tipoComunita;fonte;numeroProtocollo;idroEcoRegione;areaAlpina;nomeBacino";

// This must be kept aligned with RecordCsvCampionamentoHFBI definition.
// TODO: get this stuff with some macro?
pub const CAMPIONAMENTO_HFBI_HEADER_FIELDS: [&str; 3] = ["codiceSpecie", "numeroIndividui", "peso"];
pub const CAMPIONAMENTO_HFBI_HEADER_FIELD_TYPES: [&str; 3] = ["String", "u32", "f32"];
pub const CAMPIONAMENTO_HFBI_HEADER: &str = "\
codiceSpecie;numeroIndividui;peso";

// This must be kept aligned with RecordCsvAnagraficaHFBI definition.
// TODO: get this stuff with some macro?
pub const ANAGRAFICA_HFBI_HEADER_FIELDS: [&str; 10] = [
    "codiceStazione",
    "corpoIdrico",
    "regione",
    "provincia",
    "data",
    "lunghezzaStazione",
    "larghezzaStazione",
    "stagione",
    "habitat",
    "tipoLaguna",
];
pub const ANAGRAFICA_HFBI_HEADER_FIELD_TYPES: [&str; 10] = [
    "String", "String", "String", "String", "String", "f32", "f32", "u32", "u32", "u32",
];
pub const ANAGRAFICA_HFBI_HEADER: &str = "\
codiceStazione;corpoIdrico;regione;provincia;data;lunghezzaStazione;larghezzaStazione;stagione;habitat;tipoLaguna";

#[deprecated(note = "v0.2 will drop this reexport.\nConsider using esox::deser::TipoRecord")]
pub use crate::deser::TipoRecord as TipoRecordCsv;

#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using esox::deser::RecordAnagraficaHFBI"
)]
pub use crate::deser::RecordAnagraficaHFBI as RecordCsvAnagraficaHFBI;
#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using esox::deser::RecordAnagraficaNISECI"
)]
pub use crate::deser::RecordAnagraficaNISECI as RecordCsvAnagraficaNISECI;
#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using esox::deser::RecordCampionamentoHFBI"
)]
pub use crate::deser::RecordCampionamentoHFBI as RecordCsvCampionamentoHFBI;
#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using esox::deser::RecordCampionamentoNISECI"
)]
pub use crate::deser::RecordCampionamentoNISECI as RecordCsvCampionamentoNISECI;
#[deprecated(
    note = "v0.2 will drop this reexport.\nConsider using esox::deser::RecordRiferimentoNISECI"
)]
pub use crate::deser::RecordRiferimentoNISECI as RecordCsvRiferimentoNISECI;
