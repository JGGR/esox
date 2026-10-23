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

//! Types for modeling the domain of NISECI index computation.
//! See:
//! - [`RiferimentoNISECI`]
//! - [`CampionamentoNISECI`]
//! - [`AnagraficaNISECI`]
//! - [`RisultatoNISECI`]

/// We use [`hashbrown::HashMap`] only for [`InternerSpecieNISECI`].
/// This is because [`std::collections::hash_map::RawEntryMut`] is not stable yet.
/// See [`esox::domain::niseci::InternerSpecieNISECI::intern`].
/// Other maps in this file use [`std::collections::HashMap`].
use hashbrown::{hash_map::RawEntryMut as HBRawEntryMut, HashMap as HBHashMap};
use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::vec::Vec;

use super::localize::CommaFormat;
use super::location::Location;
#[cfg(feature = "experimental")]
use super::posf32::deserialize_positive_f32;
use super::posf32::{PositiveF32, PositiveF32Error};
use crate::capacity::{Capacity, DefaultCapacity};

#[cfg(test)]
use crate::engines::niseci::linear_regression::Point; // Needed by fishes_for_every_passage() only
                                                      // in test builds

/// Denotes the origin of a [`SpecieNISECI`].
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum OrigineSpecieNISECI {
    /// Autochthonous origin.
    Autoctono(u8),
    /// Allochthonous origin.
    Alloctono(u8),
}

/// Defines one valid species.
/// Used by [`RiferimentoNISECI`] and [`CampionamentoNISECI`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecieNISECI {
    id: String,
    nome: String,
    origine: OrigineSpecieNISECI,
    specie_attesa: bool,
    cl_soglia1: u32, // in mm
    cl_soglia2: u32, // in mm
    cl_soglia3: u32, // in mm
    cl_soglia4: u32, // in mm
    ad_juv_soglia1: f32,
    ad_juv_soglia2: f32,
    ad_juv_soglia3: f32,
    ad_juv_soglia4: f32,
    dens_soglia1: f32,
    dens_soglia2: f32,
}

impl fmt::Display for SpecieNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let specie_attesa_str = match self.specie_attesa() {
            true => "SI",
            false => "NO",
        };
        let string_representation = format!(
            "{}; {}; {}; {}; {}",
            self.id(),
            self.nome(),
            self.tipo_autoctono(),
            self.tipo_alloctono(),
            specie_attesa_str
        );
        write!(f, "{}", string_representation)
    }
}

impl SpecieNISECI {
    pub(crate) fn new(
        id: &str,
        nome: &str,
        origine: OrigineSpecieNISECI,
        specie_attesa: bool,
        cl_soglia_1: u32,
        cl_soglia_2: u32,
        cl_soglia_3: u32,
        cl_soglia_4: u32,
        ad_juv_soglia_1: f32,
        ad_juv_soglia_2: f32,
        ad_juv_soglia_3: f32,
        ad_juv_soglia_4: f32,
        dens_soglia_1: f32,
        dens_soglia_2: f32,
    ) -> Self {
        SpecieNISECI {
            id: id.to_string(),
            nome: nome.to_string(),
            origine,
            specie_attesa,
            cl_soglia1: cl_soglia_1,
            cl_soglia2: cl_soglia_2,
            cl_soglia3: cl_soglia_3,
            cl_soglia4: cl_soglia_4,
            ad_juv_soglia1: ad_juv_soglia_1,
            ad_juv_soglia2: ad_juv_soglia_2,
            ad_juv_soglia3: ad_juv_soglia_3,
            ad_juv_soglia4: ad_juv_soglia_4,
            dens_soglia1: dens_soglia_1,
            dens_soglia2: dens_soglia_2,
        }
    }

    #[inline(always)]
    pub(crate) fn id(&self) -> &str {
        &self.id
    }
    pub(crate) fn nome(&self) -> &str {
        &self.nome
    }
    #[inline(always)]
    pub(crate) fn specie_attesa(&self) -> bool {
        self.specie_attesa
    }
    #[inline(always)]
    pub(crate) fn origine(&self) -> OrigineSpecieNISECI {
        self.origine
    }
    #[inline(always)]
    pub(crate) fn tipo_autoctono(&self) -> u8 {
        match self.origine() {
            OrigineSpecieNISECI::Autoctono(val) => val,
            OrigineSpecieNISECI::Alloctono(_) => 0,
        }
    }
    #[inline(always)]
    pub(crate) fn tipo_alloctono(&self) -> u8 {
        match self.origine() {
            OrigineSpecieNISECI::Autoctono(_) => 0,
            OrigineSpecieNISECI::Alloctono(val) => val,
        }
    }
    #[inline(always)]
    pub(crate) fn cl_soglia_1(&self) -> u32 {
        self.cl_soglia1
    }
    #[inline(always)]
    pub(crate) fn cl_soglia_2(&self) -> u32 {
        self.cl_soglia2
    }
    #[inline(always)]
    pub(crate) fn cl_soglia_3(&self) -> u32 {
        self.cl_soglia3
    }
    #[inline(always)]
    pub(crate) fn cl_soglia_4(&self) -> u32 {
        self.cl_soglia4
    }
    #[inline(always)]
    pub(crate) fn ad_juv_soglia_1(&self) -> f32 {
        self.ad_juv_soglia1
    }
    #[inline(always)]
    pub(crate) fn ad_juv_soglia_2(&self) -> f32 {
        self.ad_juv_soglia2
    }
    #[inline(always)]
    pub(crate) fn ad_juv_soglia_3(&self) -> f32 {
        self.ad_juv_soglia3
    }
    #[inline(always)]
    pub(crate) fn ad_juv_soglia_4(&self) -> f32 {
        self.ad_juv_soglia4
    }
    #[inline(always)]
    pub(crate) fn dens_soglia_1(&self) -> f32 {
        self.dens_soglia1
    }
    #[cfg(test)]
    pub(crate) fn set_dens_soglia_1(&mut self, val: f32) {
        self.dens_soglia1 = val;
    }
    #[inline(always)]
    pub(crate) fn dens_soglia_2(&self) -> f32 {
        self.dens_soglia2
    }
    #[cfg(test)]
    pub(crate) fn set_dens_soglia_2(&mut self, val: f32) {
        self.dens_soglia2 = val;
    }
}

pub(crate) type IdSpecieNISECI = u32;

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "experimental", derive(Deserialize))]
pub(crate) struct StoreSpecieNISECI {
    values: Vec<SpecieNISECI>,
}

impl StoreSpecieNISECI {
    fn new() -> Self {
        Self { values: Vec::new() }
    }

    #[inline(always)]
    fn insert(&mut self, id: IdSpecieNISECI, value: SpecieNISECI) {
        debug_assert_eq!(
            id as usize,
            self.values.len(),
            "Ids are supposed to be dense"
        );
        self.values.push(value);
    }

    #[inline(always)]
    fn get(&self, id: IdSpecieNISECI) -> Option<&SpecieNISECI> {
        self.values.get(id as usize)
    }
}

#[derive(Debug, Clone, Serialize)]
#[cfg_attr(feature = "experimental", derive(Deserialize))]
pub(crate) struct InternerSpecieNISECI {
    map: HBHashMap<String, IdSpecieNISECI>,
    store: StoreSpecieNISECI,
    next_id: IdSpecieNISECI,
}

impl InternerSpecieNISECI {
    pub(crate) fn new() -> Self {
        Self {
            map: HBHashMap::new(),
            next_id: 0,
            store: StoreSpecieNISECI::new(),
        }
    }
    #[inline(always)]
    pub(crate) fn contains_id(&self, s: &str) -> bool {
        self.map.contains_key(s)
    }
    #[inline(always)]
    pub(crate) fn contains_plain_id(&self, id: IdSpecieNISECI) -> bool {
        self.store.get(id).is_some()
    }
    #[inline(always)]
    pub(crate) fn get(&self, s: &str) -> Option<&SpecieNISECI> {
        self.map.get(s).and_then(|k| self.store.get(*k))
    }
    #[inline(always)]
    pub(crate) fn get_by_id(&self, id: IdSpecieNISECI) -> Option<&SpecieNISECI> {
        self.store.get(id)
    }
    #[inline(always)]
    pub(crate) fn get_by_str_id(&self, s: &str) -> Option<IdSpecieNISECI> {
        self.map.get(s).copied()
    }
    /// This API allows:
    /// - Avoiding always allocating a String from s
    /// - Avoiding double lookup to avoid reinsert
    ///
    /// It needs [`hashbrown::hash_map`] since [`std::collections::hash_map::RawEntryMut`] is
    /// still unstable.
    ///
    /// Holds invariant: the underlying map is holding the set of present entries.
    /// Do not modify if you don't want to break stuff.
    #[inline(always)]
    pub(crate) fn intern(&mut self, val: SpecieNISECI) -> IdSpecieNISECI {
        match self.map.raw_entry_mut().from_key(val.id()) {
            HBRawEntryMut::Occupied(entry) => *entry.get(),

            HBRawEntryMut::Vacant(entry) => {
                let id = self.next_id;
                self.next_id += 1;

                entry.insert(val.id().to_owned(), id);
                self.store.insert(id, val);

                id
            }
        }
    }
}

impl From<Vec<SpecieNISECI>> for InternerSpecieNISECI {
    fn from(val: Vec<SpecieNISECI>) -> Self {
        let mut res = Self::new();
        for v in val {
            res.intern(v);
        }
        res
    }
}

/// Defines one valid expected set of [`SpecieNISECI`].
#[derive(Clone, Serialize)]
#[cfg_attr(feature = "experimental", derive(Deserialize))]
#[serde(deny_unknown_fields)]
pub struct RiferimentoNISECI {
    map_ids: InternerSpecieNISECI,
}

impl fmt::Display for RiferimentoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_representation = "RiferimentoNISECI: {".to_string();
        for s in self {
            string_representation = format!("{string_representation}\n  {{{s}}},");
        }
        string_representation = format!("{string_representation}\n}}");
        write!(f, "{}", string_representation)
    }
}

impl RiferimentoNISECI {
    /// Construct a new instance from some set.
    /// Does not validate the input (e.g. disallowing duplicate entries).
    pub fn new(elenco_specie: Vec<SpecieNISECI>) -> Self {
        let mut interner = InternerSpecieNISECI::new();
        for rec in elenco_specie {
            interner.intern(rec);
        }
        Self { map_ids: interner }
    }
    pub(crate) fn new_from_map(map_ids: InternerSpecieNISECI) -> Self {
        Self { map_ids }
    }
    /// Check if a species id is present.
    #[inline(always)]
    pub fn contains_id(&self, id: &str) -> bool {
        self.map_ids.contains_id(id)
    }
    /// Check if a species integer id is present.
    #[inline(always)]
    pub fn contains_plain_id(&self, id: IdSpecieNISECI) -> bool {
        self.map_ids.contains_plain_id(id)
    }
    /// Get a reference [`SpecieNISECI`] by id.
    #[inline(always)]
    pub fn get_ref_by_id(&self, id: &str) -> Option<&SpecieNISECI> {
        self.map_ids.get(id)
    }
    /// Get a reference [`SpecieNISECI`] by integer id.
    #[inline(always)]
    pub fn get_ref_by_plain_id(&self, id: IdSpecieNISECI) -> Option<&SpecieNISECI> {
        self.map_ids.get_by_id(id)
    }
    /// Convert id to corresponding integer id.
    #[inline(always)]
    pub fn get_inner_id(&self, id: &str) -> Option<IdSpecieNISECI> {
        self.map_ids.get_by_str_id(id)
    }

    /// Produces formatted csv for intermediate values by joining the species
    /// by id with [`ValoriIntermediNISECI`].
    pub fn to_csv_intermediates_joined(
        &self,
        intermediates: &ValoriIntermediNISECI,
        comma_csv_delimiter: bool,
    ) -> String {
        intermediates.to_csv_joined(self, comma_csv_delimiter)
    }
}

impl From<RiferimentoNISECI> for Vec<SpecieNISECI> {
    fn from(val: RiferimentoNISECI) -> Self {
        val.map_ids.store.values
    }
}

impl<'a> IntoIterator for &'a RiferimentoNISECI {
    type Item = &'a SpecieNISECI;
    type IntoIter = std::slice::Iter<'a, SpecieNISECI>;

    fn into_iter(self) -> Self::IntoIter {
        self.map_ids.store.values.iter()
    }
}

/// Defines one valid capture record species.
/// Used by [`CampionamentoNISECI`].
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordNISECI {
    id: IdSpecieNISECI,
    passaggio_cattura: u8,
    lunghezza: u32,
    /// in millimetri
    peso: f32, // in grammi
}

impl fmt::Display for RecordNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!("RecordNISECI: {{ specie: {{{}}}, passaggio_cattura {{{}}}, lunghezza: {{{}}}, peso: {{{}}}",
                self.id(), self.passaggio_cattura(), self.lunghezza(), self.peso());
        write!(f, "{}", string_representation)
    }
}

impl RecordNISECI {
    /// Create a new instance. Since the ids must be in sync, direct creation
    /// of this object is discouraged.
    /// Usage of [`crate::deser::RecordRiferimentoNISECI`] parsing is recommended.
    pub fn new(id: IdSpecieNISECI, passaggio_cattura: u8, lunghezza: u32, peso: f32) -> Self {
        Self {
            id,
            passaggio_cattura,
            lunghezza,
            peso,
        }
    }
    /// Getter for `passaggio_cattura`.
    #[inline(always)]
    pub fn passaggio_cattura(&self) -> u8 {
        self.passaggio_cattura
    }
    /// Getter for `lunghezza`.
    #[inline(always)]
    pub fn lunghezza(&self) -> u32 {
        self.lunghezza
    }
    /// Getter for `peso`.
    #[inline(always)]
    pub fn peso(&self) -> f32 {
        self.peso
    }
    /// Getter for integer id.
    #[inline(always)]
    pub(crate) fn id(&self) -> IdSpecieNISECI {
        self.id
    }
    /// Getter for `dens_soglia_1`.
    #[inline(always)]
    pub fn dens_soglia_1(&self, r: &RiferimentoNISECI) -> Option<f32> {
        r.get_ref_by_plain_id(self.id()).map(|s| s.dens_soglia_1())
    }
    /// Getter for `dens_soglia_2`.
    #[inline(always)]
    pub fn dens_soglia_2(&self, r: &RiferimentoNISECI) -> Option<f32> {
        r.get_ref_by_plain_id(self.id()).map(|s| s.dens_soglia_2())
    }
}

/// Defines one valid capture set of [`RecordNISECI`].
#[derive(Clone, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CampionamentoNISECI {
    campionamento: Vec<RecordNISECI>,
}

impl fmt::Display for CampionamentoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_representation = "CampionamentoNISECI: {".to_string();
        for r in self {
            string_representation = format!("{string_representation}\n  {{{r}}},");
        }
        string_representation = format!("{string_representation}\n}}");
        write!(f, "{}", string_representation)
    }
}

impl CampionamentoNISECI {
    #[cfg(test)]
    pub fn fishes_for_every_passage(&self) -> Vec<Point<i32>> {
        let mut max_pass = 0;
        for record in self {
            if record.passaggio_cattura() > max_pass {
                max_pass = record.passaggio_cattura();
            }
        }

        let mut passaggi: Vec<i32> = vec![0; max_pass as usize];
        for record in self {
            passaggi[(record.passaggio_cattura() - 1) as usize] += 1;
        }

        let mut tot = 0;

        // x = pesci totali fino a quel passaggio y = pesci del passaggio
        let mut pass_sum: Vec<Point<i32>> = Vec::with_capacity(max_pass as usize);
        for pass in passaggi.iter() {
            tot += pass;
            pass_sum.push(Point::new(tot, *pass));
        }

        pass_sum
    }

    /// Construct a new instance from some set.
    /// Does not validate the input (e.g. species missing from the related
    /// [`RiferimentoNISECI`].
    pub fn new(campionamento: Vec<RecordNISECI>) -> Self {
        Self { campionamento }
    }

    /// Get [`AlieniIndigeni`] by joining with [`RiferimentoNISECI`].
    pub fn get_numero_pesci_alieni_e_indigeni(&self, r: &RiferimentoNISECI) -> AlieniIndigeni {
        let mut alieni_indigeni = AlieniIndigeni {
            alieni: 0,
            indigeni: 0,
        };

        for pesce in self {
            debug_assert!(
                r.contains_plain_id(pesce.id()),
                "Riferimento did not contain id {}",
                pesce.id()
            );
            let specie = r
                .get_ref_by_plain_id(pesce.id())
                .expect("Riferimento should contain this id");
            let tipo_autoctono = specie.tipo_autoctono();
            let tipo_alloctono = specie.tipo_alloctono();
            if tipo_alloctono > 0 && tipo_alloctono <= 3 {
                alieni_indigeni.alieni += 1;
            } else if tipo_autoctono == 1 || tipo_autoctono == 2 {
                alieni_indigeni.indigeni += 1;
            }
        }

        alieni_indigeni
    }

    /// Get number of expected autochthonous species by joining with [`RiferimentoNISECI`].
    pub fn get_tot_specie_autoctone_attese(&self, r: &RiferimentoNISECI) -> usize {
        let mut set = HashSet::new();

        for cattura in self {
            debug_assert!(
                r.contains_plain_id(cattura.id()),
                "Riferimento did not contain id {}",
                cattura.id()
            );
            let specie = r
                .get_ref_by_plain_id(cattura.id())
                .expect("Riferimento should contain this id");
            if specie.specie_attesa()
                && (specie.tipo_autoctono() == 1 || specie.tipo_autoctono() == 2)
            {
                set.insert(cattura.id());
            }
        }

        set.len()
    }
}

impl From<CampionamentoNISECI> for Vec<RecordNISECI> {
    fn from(val: CampionamentoNISECI) -> Self {
        val.campionamento
    }
}

impl<'a> IntoIterator for &'a CampionamentoNISECI {
    type Item = &'a RecordNISECI;
    type IntoIter = std::slice::Iter<'a, RecordNISECI>;

    fn into_iter(self) -> Self::IntoIter {
        self.campionamento.iter()
    }
}

/// Utility struct for keeping track of population composition in
/// [`CampionamentoNISECI::get_numero_pesci_alieni_e_indigeni`].
pub struct AlieniIndigeni {
    alieni: u32,
    indigeni: u32,
}

impl AlieniIndigeni {
    /// Get number of alien species.
    #[inline(always)]
    pub fn alieni(&self) -> u32 {
        self.alieni
    }
    /// Get number of indigenous species.
    #[inline(always)]
    pub fn indigeni(&self) -> u32 {
        self.indigeni
    }
}

/// Denotes the kind of [`ComunitaNISECI`].
#[derive(Copy, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TipoComunitaNISECI {
    /// Made by the operator.
    Redatta,
    /// Found in reference sources.
    Recuperata,
    /// DM 260/2010.
    Dm260_2010,
    /// Finetuned by Mase.
    AffinataDalMase,
}

impl fmt::Display for TipoComunitaNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            TipoComunitaNISECI::Redatta => "Redatta dall'operatore",
            TipoComunitaNISECI::Recuperata => "Recuperata da fonti bibliografiche",
            TipoComunitaNISECI::Dm260_2010 => "DM 260/2010",
            TipoComunitaNISECI::AffinataDalMase => "Affinata dal Mase",
        };
        write!(f, "{}", string_representation)
    }
}

impl TryFrom<i32> for TipoComunitaNISECI {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == TipoComunitaNISECI::Redatta as i32 => Ok(TipoComunitaNISECI::Redatta),
            x if x == TipoComunitaNISECI::Recuperata as i32 => Ok(TipoComunitaNISECI::Recuperata),
            x if x == TipoComunitaNISECI::Dm260_2010 as i32 => Ok(TipoComunitaNISECI::Dm260_2010),
            x if x == TipoComunitaNISECI::AffinataDalMase as i32 => {
                Ok(TipoComunitaNISECI::AffinataDalMase)
            }
            _ => Err(()),
        }
    }
}

/// Denotes community info for [`AnagraficaNISECI`].
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ComunitaNISECI {
    tipo: TipoComunitaNISECI,
    fonte: Option<String>,
    numero_protocollo: Option<String>,
}

impl fmt::Display for ComunitaNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match self.tipo() {
            TipoComunitaNISECI::Redatta | TipoComunitaNISECI::Dm260_2010 => {
                format!("Comunita di Riferimento: tipo: {} ", self.tipo())
            }
            TipoComunitaNISECI::Recuperata => {
                if let Some(fonte) = self.fonte() {
                    //TODO: is this good?
                    format!(
                        "Comunita di Riferimento: tipo: {}, fonte: {}",
                        self.tipo(),
                        fonte
                    )
                } else {
                    format!(
                        "Comunita di Riferimento: tipo: {}, fonte: MANCANTE",
                        self.tipo()
                    )
                }
            }
            TipoComunitaNISECI::AffinataDalMase => {
                if let Some(num_proto) = self.num_protocollo() {
                    //TODO: is this good?
                    format!(
                        "Comunita di Riferimento: tipo: {}, numero_protocollo: {}",
                        self.tipo(),
                        num_proto
                    )
                } else {
                    format!(
                        "Comunita di Riferimento: tipo: {}, numero_protocollo: MANCANTE",
                        self.tipo()
                    )
                }
            }
        };
        write!(f, "{}", string_representation)
    }
}

impl ComunitaNISECI {
    /// Create a new instance.
    pub fn new(
        tipo: TipoComunitaNISECI,
        fonte: Option<String>,
        num_protocollo: Option<String>,
    ) -> Self {
        Self {
            tipo,
            fonte,
            numero_protocollo: num_protocollo,
        }
    }
    /// Getter for [`TipoComunitaNISECI`].
    #[inline(always)]
    pub fn tipo(&self) -> TipoComunitaNISECI {
        self.tipo
    }
    /// Getter for `fonte`.
    #[inline(always)]
    pub fn fonte(&self) -> Option<&str> {
        self.fonte.as_deref()
    }
    /// Getter for `num_protocollo`.
    #[inline(always)]
    pub fn num_protocollo(&self) -> Option<&str> {
        self.numero_protocollo.as_deref()
    }
}

/// Denotes area for [`AnagraficaNISECI`].
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum AreaNISECI {
    /// Alpine area.
    Alpina,
    /// Mediterranean area.
    Mediterranea,
}

impl fmt::Display for AreaNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            AreaNISECI::Alpina => "Area NISECI: Alpina".to_string(),
            AreaNISECI::Mediterranea => "Area NISECI: Mediterranea".to_string(),
        };
        write!(f, "{}", string_representation)
    }
}

impl TryFrom<i32> for AreaNISECI {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == AreaNISECI::Alpina as i32 => Ok(AreaNISECI::Alpina),
            x if x == AreaNISECI::Mediterranea as i32 => Ok(AreaNISECI::Mediterranea),
            _ => Err(()),
        }
    }
}

/// Utility struct for form-like tentative definition of [`AnagraficaNISECI`].
#[derive(Clone)]
pub struct AnagraficaNISECIDraft {
    /// [`ComunitaNISECI`] for this object.
    pub comunita: ComunitaNISECI,
    /// Station code for this object.
    pub codice_stazione: String,
    /// Formatted date string for this object.
    pub date_string: String, // Formato gg/mm/aaaa
    /// [`AreaNISECI`] for this object.
    pub area: AreaNISECI,
    /// Water body for this object.
    pub corpo_idrico: String,
    /// Water basin for this object.
    pub bacino_appartenenza: String,
    /// [`IdroEcoRegioneNISECI`] for this object.
    pub idro_eco_regione: IdroEcoRegioneNISECI,
    /// [`crate::domain::location::Location`] for this object.
    pub posizione: Location,
    /// Station length for this object.
    pub lunghezza_media_stazione: String,
    /// Station width for this object.
    pub larghezza_media_stazione: String,
}

/// Defines one valid additional info set.
/// Used by [`crate::engines::niseci::full`].
#[derive(Clone, Serialize)]
#[cfg_attr(feature = "experimental", derive(Deserialize))]
#[serde(deny_unknown_fields)]
pub struct AnagraficaNISECI {
    /// [`ComunitaNISECI`] for this object.
    pub comunita: ComunitaNISECI,
    /// Station code for this object.
    pub codice_stazione: String,
    /// Formatted date string for this object.
    pub date_string: String, // Formato gg/mm/aaaa
    /// [`AreaNISECI`] for this object.
    pub area: AreaNISECI,
    /// Water body for this object.
    pub corpo_idrico: String,
    /// Water basin for this object.
    pub bacino_appartenenza: String,
    /// [`IdroEcoRegioneNISECI`] for this object.
    pub idro_eco_regione: IdroEcoRegioneNISECI,
    /// [`crate::domain::location::Location`] for this object.
    pub posizione: Location,
    #[serde(deserialize_with = "deserialize_positive_f32")]
    lunghezza_media_stazione: f32,
    #[serde(deserialize_with = "deserialize_positive_f32")]
    larghezza_media_stazione: f32,
}

impl AnagraficaNISECI {
    /// Create a new instance.
    /// Does not validate the input (e.g. checking date string).
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        comunita: ComunitaNISECI,
        codice_stazione: String,
        date_string: String,
        area: AreaNISECI,
        corpo_idrico: String,
        bacino_appartenenza: String,
        idro_eco_regione: IdroEcoRegioneNISECI,
        posizione: Location,
        lunghezza_media_stazione: PositiveF32,
        larghezza_media_stazione: PositiveF32,
    ) -> Self {
        Self {
            comunita,
            codice_stazione,
            date_string,
            area,
            corpo_idrico,
            bacino_appartenenza,
            idro_eco_regione,
            posizione,
            lunghezza_media_stazione: *lunghezza_media_stazione,
            larghezza_media_stazione: *larghezza_media_stazione,
        }
    }
    /// Getter for station length.
    pub fn get_lunghezza_media(&self) -> f32 {
        self.lunghezza_media_stazione
    }
    /// Setter for station length. Checks for >0 and finite input.
    pub fn set_lunghezza_media(&mut self, val: f32) -> Result<(), PositiveF32Error> {
        if !val.is_finite() {
            return Err(PositiveF32Error::NotFinite);
        }
        if val <= 0.0 {
            return Err(PositiveF32Error::NotPositive);
        }
        self.lunghezza_media_stazione = val;
        Ok(())
    }
    /// Getter for station width.
    pub fn get_larghezza_media(&self) -> f32 {
        self.larghezza_media_stazione
    }
    /// Setter for station width. Checks for >0 and finite input.
    pub fn set_larghezza_media(&mut self, val: f32) -> Result<(), PositiveF32Error> {
        if !val.is_finite() {
            return Err(PositiveF32Error::NotFinite);
        }
        if val <= 0.0 {
            return Err(PositiveF32Error::NotPositive);
        }
        self.larghezza_media_stazione = val;
        Ok(())
    }
}

impl fmt::Display for AnagraficaNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!("AnagraficaNISECI: {{ comunita: {{{}}}, codice_stazione {{{}}}, area: {{{}}}, corpo_idrico: {{{}}}, bacino_appartenenza: {{{}}}, idro_eco_regione: {{{}}}, posizione: {{{}}}, lunghezza_stazione: {{{}}}, larghezza_stazione: {{{}}} }}",
        self.comunita, self.codice_stazione, self.area, self.corpo_idrico, self.bacino_appartenenza, self.idro_eco_regione, self.posizione, self.get_lunghezza_media(), self.get_larghezza_media());
        write!(f, "{}", string_representation)
    }
}
/// Denotes classified region for [`AnagraficaNISECI`].
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum IdroEcoRegioneNISECI {
    AlpiOccidentali,
    PrealpiDolomiti,
    AlpiCentroOrientali,
    AlpiMeridionali,
    Monferrato,
    PianuraPadana,
    Carso,
    AppenninoPiemontese,
    AlpiMediterranee,
    AppenninoSettentrionale,
    Toscana,
    CostaAdriatica,
    AppenninoCentrale,
    RomaViterbese,
    BassoLazio,
    Vesuvio,
    BasilicataTavoliere,
    PugliaCarsica,
    AppenninoMeridionale,
    Sicilia,
    Sardegna,
}

impl fmt::Display for IdroEcoRegioneNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            IdroEcoRegioneNISECI::AlpiOccidentali => "Alpi Occidentali",
            IdroEcoRegioneNISECI::PrealpiDolomiti => "Prealpi Dolomiti",
            IdroEcoRegioneNISECI::AlpiCentroOrientali => "Alpi Centro-orientali",
            IdroEcoRegioneNISECI::AlpiMeridionali => "Alpi Meridionali",
            IdroEcoRegioneNISECI::Monferrato => "Monferrato",
            IdroEcoRegioneNISECI::PianuraPadana => "Pianura Padana",
            IdroEcoRegioneNISECI::Carso => "Carso",
            IdroEcoRegioneNISECI::AppenninoPiemontese => "Appennino Piemontese",
            IdroEcoRegioneNISECI::AlpiMediterranee => "Alpi Mediterranee",
            IdroEcoRegioneNISECI::AppenninoSettentrionale => "Appennino Settentrionale",
            IdroEcoRegioneNISECI::Toscana => "Toscana",
            IdroEcoRegioneNISECI::CostaAdriatica => "Costa Adriatica",
            IdroEcoRegioneNISECI::AppenninoCentrale => "Appennino Centrale",
            IdroEcoRegioneNISECI::RomaViterbese => "Roma-Viterbese",
            IdroEcoRegioneNISECI::BassoLazio => "Basso Lazio",
            IdroEcoRegioneNISECI::Vesuvio => "Vesuvio",
            IdroEcoRegioneNISECI::BasilicataTavoliere => "Basilicata Tavoliere",
            IdroEcoRegioneNISECI::PugliaCarsica => "Puglia Carsica",
            IdroEcoRegioneNISECI::AppenninoMeridionale => "Appennino Meridionale",
            IdroEcoRegioneNISECI::Sicilia => "Sicilia",
            IdroEcoRegioneNISECI::Sardegna => "Sardegna",
        };
        write!(f, "{}", string_representation)
    }
}

impl TryFrom<i32> for IdroEcoRegioneNISECI {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == IdroEcoRegioneNISECI::AlpiOccidentali as i32 => {
                Ok(IdroEcoRegioneNISECI::AlpiOccidentali)
            }
            x if x == IdroEcoRegioneNISECI::PrealpiDolomiti as i32 => {
                Ok(IdroEcoRegioneNISECI::PrealpiDolomiti)
            }
            x if x == IdroEcoRegioneNISECI::AlpiCentroOrientali as i32 => {
                Ok(IdroEcoRegioneNISECI::AlpiCentroOrientali)
            }
            x if x == IdroEcoRegioneNISECI::AlpiMeridionali as i32 => {
                Ok(IdroEcoRegioneNISECI::AlpiMeridionali)
            }
            x if x == IdroEcoRegioneNISECI::Monferrato as i32 => {
                Ok(IdroEcoRegioneNISECI::Monferrato)
            }
            x if x == IdroEcoRegioneNISECI::PianuraPadana as i32 => {
                Ok(IdroEcoRegioneNISECI::PianuraPadana)
            }
            x if x == IdroEcoRegioneNISECI::Carso as i32 => Ok(IdroEcoRegioneNISECI::Carso),
            x if x == IdroEcoRegioneNISECI::AppenninoPiemontese as i32 => {
                Ok(IdroEcoRegioneNISECI::AppenninoPiemontese)
            }
            x if x == IdroEcoRegioneNISECI::AlpiMediterranee as i32 => {
                Ok(IdroEcoRegioneNISECI::AlpiMediterranee)
            }
            x if x == IdroEcoRegioneNISECI::AppenninoSettentrionale as i32 => {
                Ok(IdroEcoRegioneNISECI::AppenninoSettentrionale)
            }
            x if x == IdroEcoRegioneNISECI::Toscana as i32 => Ok(IdroEcoRegioneNISECI::Toscana),
            x if x == IdroEcoRegioneNISECI::CostaAdriatica as i32 => {
                Ok(IdroEcoRegioneNISECI::CostaAdriatica)
            }
            x if x == IdroEcoRegioneNISECI::AppenninoCentrale as i32 => {
                Ok(IdroEcoRegioneNISECI::AppenninoCentrale)
            }
            x if x == IdroEcoRegioneNISECI::RomaViterbese as i32 => {
                Ok(IdroEcoRegioneNISECI::RomaViterbese)
            }
            x if x == IdroEcoRegioneNISECI::BassoLazio as i32 => {
                Ok(IdroEcoRegioneNISECI::BassoLazio)
            }
            x if x == IdroEcoRegioneNISECI::Vesuvio as i32 => Ok(IdroEcoRegioneNISECI::Vesuvio),
            x if x == IdroEcoRegioneNISECI::BasilicataTavoliere as i32 => {
                Ok(IdroEcoRegioneNISECI::BasilicataTavoliere)
            }
            x if x == IdroEcoRegioneNISECI::PugliaCarsica as i32 => {
                Ok(IdroEcoRegioneNISECI::PugliaCarsica)
            }
            x if x == IdroEcoRegioneNISECI::AppenninoMeridionale as i32 => {
                Ok(IdroEcoRegioneNISECI::AppenninoMeridionale)
            }
            x if x == IdroEcoRegioneNISECI::Sicilia as i32 => Ok(IdroEcoRegioneNISECI::Sicilia),
            x if x == IdroEcoRegioneNISECI::Sardegna as i32 => Ok(IdroEcoRegioneNISECI::Sardegna),
            _ => Err(()),
        }
    }
}

/// Intermediate species-specific values for NISECI calc.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValoriIntermediSpecieNISECI {
    densita_stimata: f32,
    quantita_stimata: u32,
    classi_eta: ClassiEtaSpecieNISECI,
    rapporto_ad_juv: Option<f32>,
    x2_a_a: u8,
    x2_a_b: u8,
    x2_b: f32,
}

impl fmt::Display for ValoriIntermediSpecieNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rapporto_ad_juv_str = match self.rapporto_ad_juv() {
            Some(v) => {
                format!("{v}")
            }
            None => "NC".to_string(),
        };
        let string_representation = format!(
            "{}; {}; {}; {}; {}; {}; {}",
            self.classi_eta(),
            self.densita_stimata(),
            self.quantita_stimata(),
            self.x2_b(),
            rapporto_ad_juv_str,
            self.x2_a_a(),
            self.x2_a_b()
        );
        write!(f, "{}", string_representation)
    }
}

impl ValoriIntermediSpecieNISECI {
    pub(crate) fn new(
        classi_eta: ClassiEtaSpecieNISECI,
        densita_stimata: f32,
        quantita_stimata: u32,
        rapporto_ad_juv: Option<f32>,
        x2_a_a: u8,
        x2_a_b: u8,
        x2_b: f32,
    ) -> Self {
        Self {
            densita_stimata,
            quantita_stimata,
            classi_eta,
            rapporto_ad_juv,
            x2_a_a,
            x2_a_b,
            x2_b,
        }
    }

    /// Getter for `densita_stimata`.
    #[inline(always)]
    pub fn densita_stimata(&self) -> f32 {
        self.densita_stimata
    }
    /// Getter for `quantita_stimata`.
    #[inline(always)]
    pub fn quantita_stimata(&self) -> u32 {
        self.quantita_stimata
    }
    /// Getter for [`ClassiEtaSpecieNISECI`].
    #[inline(always)]
    pub fn classi_eta(&self) -> &ClassiEtaSpecieNISECI {
        &self.classi_eta
    }
    /// Getter for `rapporto_ad_juv`.
    #[inline(always)]
    pub fn rapporto_ad_juv(&self) -> Option<f32> {
        self.rapporto_ad_juv
    }
    /// Getter for `x2_a_a`.
    #[inline(always)]
    pub fn x2_a_a(&self) -> u8 {
        self.x2_a_a
    }
    /// Getter for `x2_a_b`.
    #[inline(always)]
    pub fn x2_a_b(&self) -> u8 {
        self.x2_a_b
    }
    /// Getter for `x2_b`.
    #[inline(always)]
    pub fn x2_b(&self) -> f32 {
        self.x2_b
    }
}

/// Intermediate values for NISECI calc.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValoriIntermediNISECI {
    x1: f32,
    x2: Option<f32>,
    x3: f32,
    specie_specifici: HashMap<IdSpecieNISECI, ValoriIntermediSpecieNISECI>,
    x2_a: f32,
    x2_b: f32,
    x3_a: Option<f32>,
    x3_b: Option<f32>,
}

impl fmt::Display for ValoriIntermediNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x2_str = match self.x2() {
            Some(v) => format!("{v}"),
            None => "NC".to_string(),
        };
        let x3_a_str = match self.x3_a() {
            Some(v) => format!("{v}"),
            None => "NC".to_string(),
        };
        let x3_b_str = match self.x3_b() {
            Some(v) => format!("{v}"),
            None => "NC".to_string(),
        };
        let mut string_representation = format!("x1: {}, x2: {}, x3: {},\nx2_a: {}, x2_b: {}, x3_a: {}, x3_b: {},\n\nspecie, nome latino, tipo autoctono, tipo alloctono, specie attesa, cl1, cl2, cl3, cl4, cl5, densita stimata, quantita stimata, x2_b, rapporto ad/juv, x2a_a, x2a_b\n",
        self.x1(), x2_str, self.x3(),
        self.x2_a(), self.x2_b(), x3_a_str, x3_b_str);

        for (_k, v) in self.specie_specifici().iter() {
            string_representation = format!("{}\n{}", string_representation, v);
        }
        write!(f, "{}", string_representation)
    }
}

impl ValoriIntermediNISECI {
    pub(crate) fn new(
        x1: f32,
        x2: Option<f32>,
        x3: f32,
        specie_specifici: HashMap<IdSpecieNISECI, ValoriIntermediSpecieNISECI>,
        x2_a: f32,
        x2_b: f32,
        x3_a: Option<f32>,
        x3_b: Option<f32>,
    ) -> Self {
        Self {
            x1,
            x2,
            x3,
            specie_specifici,
            x2_a,
            x2_b,
            x3_a,
            x3_b,
        }
    }
    /// Getter for `x1`.
    #[inline(always)]
    pub fn x1(&self) -> f32 {
        self.x1
    }
    /// Getter for `x2`.
    #[inline(always)]
    pub fn x2(&self) -> Option<f32> {
        self.x2
    }
    /// Getter for `x3`.
    #[inline(always)]
    pub fn x3(&self) -> f32 {
        self.x3
    }
    /// Getter for `x2_a`.
    #[inline(always)]
    pub fn x2_a(&self) -> f32 {
        self.x2_a
    }
    /// Getter for `x2_b`.
    #[inline(always)]
    pub fn x2_b(&self) -> f32 {
        self.x2_b
    }
    /// Getter for `x3_a`.
    #[inline(always)]
    pub fn x3_a(&self) -> Option<f32> {
        self.x3_a
    }
    /// Getter for `x3_b`.
    #[inline(always)]
    pub fn x3_b(&self) -> Option<f32> {
        self.x3_b
    }
    /// Getter for species-specific [`ValoriIntermediSpecieNISECI`].
    #[inline(always)]
    pub fn specie_specifici(&self) -> &HashMap<IdSpecieNISECI, ValoriIntermediSpecieNISECI> {
        &self.specie_specifici
    }
    /// Produces formatted csv for species-specific intermediate values by joining the species
    /// by id with [`RiferimentoNISECI`].
    pub fn to_csv_joined(
        &self,
        riferimento: &RiferimentoNISECI,
        comma_csv_delimiter: bool,
    ) -> String {
        let mut string_representation = if comma_csv_delimiter {
            "specie, nome latino, tipo autoctono, tipo alloctono, specie attesa, cl1, cl2, cl3, cl4, cl5, densita stimata, quantita stimata, x2b, rapporto ad/juv, x2a_a, x2a_b".to_string()
        } else {
            "specie; nome latino; tipo autoctono; tipo alloctono; specie attesa; cl1; cl2; cl3; cl4; cl5; densita stimata; quantita stimata; x2b; rapporto ad/juv; x2a_a; x2a_b".to_string()
        };
        for (id, v) in self.specie_specifici().iter() {
            let specie_ref = match riferimento.get_ref_by_plain_id(*id) {
                Some(s) => s,
                None => {
                    string_representation = format!(
                        "{}\n?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?",
                        string_representation
                    );
                    return string_representation;
                }
            };
            let key_id = specie_ref.id();
            let rapporto_ad_juv_str = match v.rapporto_ad_juv() {
                Some(v) => {
                    if comma_csv_delimiter {
                        format!("{v}")
                    } else {
                        v.comma().to_string()
                    }
                }
                None => "NC".to_string(),
            };
            let specie_attesa_str = if specie_ref.specie_attesa() {
                "SI"
            } else {
                "NO"
            };
            string_representation = if comma_csv_delimiter {
                format!(
                    "{}\n{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
                    string_representation,
                    key_id,
                    specie_ref.nome(),
                    specie_ref.tipo_autoctono(),
                    specie_ref.tipo_alloctono(),
                    specie_attesa_str,
                    v.classi_eta().cl_1(),
                    v.classi_eta().cl_2(),
                    v.classi_eta().cl_3(),
                    v.classi_eta().cl_4(),
                    v.classi_eta().cl_5(),
                    v.densita_stimata(),
                    v.quantita_stimata(),
                    v.x2_b(),
                    rapporto_ad_juv_str,
                    v.x2_a_a(),
                    v.x2_a_b()
                )
            } else {
                format!(
                    "{}\n{}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}",
                    string_representation,
                    key_id,
                    specie_ref.nome(),
                    specie_ref.tipo_autoctono(),
                    specie_ref.tipo_alloctono(),
                    specie_attesa_str,
                    v.classi_eta().cl_1(),
                    v.classi_eta().cl_2(),
                    v.classi_eta().cl_3(),
                    v.classi_eta().cl_4(),
                    v.classi_eta().cl_5(),
                    v.densita_stimata().comma(),
                    v.quantita_stimata(),
                    v.x2_b(),
                    rapporto_ad_juv_str,
                    v.x2_a_a(),
                    v.x2_a_b()
                )
            }
        }
        string_representation
    }
}

/// Defines result of NISECI calc.
/// Returned by [`crate::engines::niseci::full`].
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RisultatoNISECI {
    valore: Option<f32>,
    rqe: Option<f32>,
    valori_intermedi: ValoriIntermediNISECI,
}

impl fmt::Display for RisultatoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let valore_str = match self.valore {
            Some(v) => format!("{v}"),
            None => "NC".to_string(),
        };
        let rqe_str = match self.rqe {
            Some(v) => format!("{v}"),
            None => "NC".to_string(),
        };
        let string_representation = format!("RisultatoNISECI: {{ valore NISECI: {{{}}}, valore RQE NISECI: {{{}}}, valori intermedi: {{{}}} }}", valore_str, rqe_str, self.valori_intermedi);
        write!(f, "{}", string_representation)
    }
}

impl RisultatoNISECI {
    /// Construct a new instance from the result of
    /// [`crate::engines::niseci::full`].
    /// Next version will provide direct construction from the engines.
    pub fn new(
        valore: Option<f32>,
        rqe: Option<f32>,
        valori_intermedi: ValoriIntermediNISECI,
    ) -> Self {
        Self {
            valore,
            rqe,
            valori_intermedi,
        }
    }
    /// Getter for NISECI value.
    pub fn get_valore(&self) -> Option<f32> {
        self.valore
    }
    /// Getter for NISECI RQE value.
    pub fn get_rqe(&self) -> Option<f32> {
        self.rqe
    }
    /// Getter for `x1` value.
    pub fn get_x1(&self) -> f32 {
        self.valori_intermedi.x1()
    }
    /// Getter for `x2` value.
    pub fn get_x2(&self) -> Option<f32> {
        self.valori_intermedi.x2()
    }
    /// Getter for `x3` value.
    pub fn get_x3(&self) -> f32 {
        self.valori_intermedi.x3()
    }
    /// Getter for `x3_a` value.
    pub fn get_x3_a(&self) -> Option<f32> {
        self.valori_intermedi.x3_a()
    }
    /// Getter for `x3_b` value.
    pub fn get_x3_b(&self) -> Option<f32> {
        self.valori_intermedi.x3_b()
    }

    /// Produces formatted csv for result values by joining with
    /// [`AnagraficaNISECI`].
    pub fn to_csv(&self, anagrafica: &AnagraficaNISECI, comma_csv_delimiter: bool) -> String {
        let niseci = self.get_valore();
        let niseci_str = match niseci {
            Some(val) => {
                if comma_csv_delimiter {
                    format!("{val}")
                } else {
                    val.comma().to_string()
                }
            }
            None => "NC".to_string(),
        };

        let rqe_niseci = self.get_rqe();
        let rqe_niseci_str = match rqe_niseci {
            Some(val) => {
                if comma_csv_delimiter {
                    format!("{val}")
                } else {
                    val.comma().to_string()
                }
            }
            None => "NC".to_string(),
        };
        let stato_ecologico =
            rqe_niseci.map(|val| StatoEcologicoNISECI::from((val, &anagrafica.area)));
        let stato_ecologico_str = match stato_ecologico {
            Some(val) => {
                format!("{val}")
            }
            None => "NC".to_string(),
        };
        let string_representation = if comma_csv_delimiter {
            format!("Codice stazione, Data, Regione, Idroecoregione, Area pertinenza, Bacino, NISECI, RQE NISECI, Stato ecologico, x1, x2, x3, x3_a, x3_b\n{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}",
                anagrafica.codice_stazione,
                anagrafica.date_string,
                anagrafica.posizione.regione,
                anagrafica.idro_eco_regione,
                anagrafica.area,
                anagrafica.corpo_idrico,
                niseci_str,
                rqe_niseci_str,
                stato_ecologico_str,
                self.get_x1(),
                match self.get_x2() {
                    Some(v) => format!("{v}"),
                    None => "NC".to_string(),
                },
                self.get_x3(),
                match self.get_x3_a() {
                    Some(v) => format!("{v}"),
                    None => "NC".to_string(),
                },
                match self.get_x3_b() {
                    Some(v) => format!("{v}"),
                    None => "NC".to_string(),
                }
            )
        } else {
            format!("Codice stazione; Data; Regione; Idroecoregione; Area pertinenza; Bacino; NISECI; RQE NISECI; Stato ecologico; x1; x2; x3; x3_a; x3_b\n{}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}; {}",
                anagrafica.codice_stazione,
                anagrafica.date_string,
                anagrafica.posizione.regione,
                anagrafica.idro_eco_regione,
                anagrafica.area,
                anagrafica.corpo_idrico,
                niseci_str,
                rqe_niseci_str,
                stato_ecologico_str,
                self.get_x1().comma(),
                match self.get_x2() {
                    Some(v) => v.comma().to_string(),
                    None => "NC".to_string(),
                },
                self.get_x3().comma(),
                match self.get_x3_a() {
                    Some(v) => v.comma().to_string(),
                    None => "NC".to_string(),
                },
                match self.get_x3_b() {
                    Some(v) => v.comma().to_string(),
                    None => "NC".to_string(),
                }
            )
        };
        string_representation
    }
}

/// Intermediate values of `x2A_b` for NISECI calc.
#[derive(Clone, Copy)]
pub struct MetricheX2aB {
    criterio_b: u8,
    rapporto_ad_juv: Option<f32>,
}

impl MetricheX2aB {
    /// Construct a new instance.
    pub fn new(criterio_b: u8, rapporto_ad_juv: Option<f32>) -> Self {
        Self {
            criterio_b,
            rapporto_ad_juv,
        }
    }
    /// Getter for `criterio_b`.
    pub fn get_criterio_b(&self) -> u8 {
        self.criterio_b
    }
    /// Getter for `rapporto_ad_juv`.
    pub fn get_rapporto_ad_juv(&self) -> Option<f32> {
        self.rapporto_ad_juv
    }
}

/// Intermediate values of `x2A` for NISECI calc.
#[derive(Clone, Copy)]
pub struct MetricheX2A {
    criterio_a: u8,
    criteri_x2a_b: MetricheX2aB,
}

impl MetricheX2A {
    /// Construct a new instance.
    pub fn new(criterio_a: u8, criteri_x2a_b: MetricheX2aB) -> Self {
        Self {
            criterio_a,
            criteri_x2a_b,
        }
    }
    /// Getter for `criterio_a`.
    pub fn get_criterio_a(&self) -> u8 {
        self.criterio_a
    }
    /// Getter for `criterio_b`.
    pub fn get_criterio_b(&self) -> u8 {
        self.criteri_x2a_b.get_criterio_b()
    }
    /// Getter for `rapporto_ad_juv`.
    pub fn get_rapporto_ad_juv(&self) -> Option<f32> {
        self.criteri_x2a_b.get_rapporto_ad_juv()
    }
}

/// Utility enum that helps initialise [`ClassiEtaSpecieNISECI`].
/// See:
/// - [`ClassiEtaSpecieNISECI::update_classi_eta`]
pub enum ClassiEta {
    CL1,
    CL2,
    CL3,
    CL4,
    CL5,
}

impl ClassiEta {
    /// Get a [`ClassiEta`] for a given [`RecordNISECI`] by joining with [`RiferimentoNISECI`].
    /// Currently panics if the species is not present.
    pub fn find_classe_eta(record: &RecordNISECI, r: &RiferimentoNISECI) -> ClassiEta {
        let specie = r
            .get_ref_by_plain_id(record.id())
            .expect("Riferimento should contain this id");
        if record.lunghezza() < specie.cl_soglia_1() {
            ClassiEta::CL1
        } else if record.lunghezza() < specie.cl_soglia_2() {
            ClassiEta::CL2
        } else if record.lunghezza() < specie.cl_soglia_3() {
            ClassiEta::CL3
        } else if record.lunghezza() < specie.cl_soglia_4() {
            ClassiEta::CL4
        } else {
            ClassiEta::CL5
        }
    }
}

/// Contains number of specimens found in a capture set:
/// - for each species captured
/// - split in their age class (according to length)
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClassiEtaSpecieNISECI {
    id: IdSpecieNISECI,
    cl1: u32,
    cl2: u32,
    cl3: u32,
    cl4: u32,
    cl5: u32,
}

impl fmt::Display for ClassiEtaSpecieNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "{}; {}; {}; {}; {}; {}",
            self.id(),
            self.cl_1(),
            self.cl_2(),
            self.cl_3(),
            self.cl_4(),
            self.cl_5()
        );
        write!(f, "{}", string_representation)
    }
}

impl ClassiEtaSpecieNISECI {
    pub(crate) fn new_empty() -> Self {
        Self {
            id: 420,
            cl1: 0,
            cl2: 0,
            cl3: 0,
            cl4: 0,
            cl5: 0,
        }
    }
    /// Construct a new instance from a [`RecordNISECI`] and the corresponding
    /// [`RiferimentoNISECI`].
    pub fn new_cl_prevalorizzata(
        record: &RecordNISECI,
        r: &RiferimentoNISECI,
    ) -> Option<ClassiEtaSpecieNISECI> {
        let mut classe = ClassiEtaSpecieNISECI {
            id: record.id(),
            cl1: 0,
            cl2: 0,
            cl3: 0,
            cl4: 0,
            cl5: 0,
        };
        classe.update_classi_eta(record, r);
        Some(classe)
    }

    /// Update count of classes.
    pub fn update_classi_eta(&mut self, record: &RecordNISECI, r: &RiferimentoNISECI) {
        match ClassiEta::find_classe_eta(record, r) {
            ClassiEta::CL1 => self.cl1 += 1,
            ClassiEta::CL2 => self.cl2 += 1,
            ClassiEta::CL3 => self.cl3 += 1,
            ClassiEta::CL4 => self.cl4 += 1,
            ClassiEta::CL5 => self.cl5 += 1,
        }
    }

    /// Getter for `cl1`.
    #[inline(always)]
    pub fn cl_1(&self) -> u32 {
        self.cl1
    }
    /// Getter for `cl2`.
    #[inline(always)]
    pub fn cl_2(&self) -> u32 {
        self.cl2
    }
    /// Getter for `cl3`.
    #[inline(always)]
    pub fn cl_3(&self) -> u32 {
        self.cl3
    }
    /// Getter for `cl4`.
    #[inline(always)]
    pub fn cl_4(&self) -> u32 {
        self.cl4
    }
    /// Getter for `cl5`.
    #[inline(always)]
    pub fn cl_5(&self) -> u32 {
        self.cl5
    }

    /// Getter for integer id.
    #[inline(always)]
    pub fn id(&self) -> IdSpecieNISECI {
        self.id
    }

    /// Getter for expected species flag.
    #[inline(always)]
    pub fn specie_attesa(&self, r: &RiferimentoNISECI) -> Option<bool> {
        r.get_ref_by_plain_id(self.id()).map(|s| s.specie_attesa())
    }

    /// Getter for autochthonous type.
    #[inline(always)]
    pub fn tipo_autoctono(&self, r: &RiferimentoNISECI) -> Option<u8> {
        r.get_ref_by_plain_id(self.id()).map(|s| s.tipo_autoctono())
    }

    /// Getter for allochthonous type.
    #[inline(always)]
    pub fn tipo_alloctono(&self, r: &RiferimentoNISECI) -> Option<u8> {
        r.get_ref_by_plain_id(self.id()).map(|s| s.tipo_alloctono())
    }

    /// Getter for `ad_juv_soglia_1`.
    #[inline(always)]
    pub fn ad_juv_soglia_1(&self, r: &RiferimentoNISECI) -> Option<f32> {
        r.get_ref_by_plain_id(self.id())
            .map(|s| s.ad_juv_soglia_1())
    }

    /// Getter for `ad_juv_soglia_2`.
    #[inline(always)]
    pub fn ad_juv_soglia_2(&self, r: &RiferimentoNISECI) -> Option<f32> {
        r.get_ref_by_plain_id(self.id())
            .map(|s| s.ad_juv_soglia_2())
    }

    /// Getter for `ad_juv_soglia_3`.
    #[inline(always)]
    pub fn ad_juv_soglia_3(&self, r: &RiferimentoNISECI) -> Option<f32> {
        r.get_ref_by_plain_id(self.id())
            .map(|s| s.ad_juv_soglia_3())
    }

    /// Getter for `ad_juv_soglia_4`.
    #[inline(always)]
    pub fn ad_juv_soglia_4(&self, r: &RiferimentoNISECI) -> Option<f32> {
        r.get_ref_by_plain_id(self.id())
            .map(|s| s.ad_juv_soglia_4())
    }

    fn get_how_many_classes(&self) -> usize {
        [self.cl1, self.cl2, self.cl3, self.cl4, self.cl5]
            .into_iter()
            .filter(|&value| value > 0)
            .count()
    }

    /// Getter for `x2_a` `criterio_a`.
    pub fn get_x2_a_criterio_a(&self) -> u8 {
        let count = self.get_how_many_classes();
        if count >= 4 {
            return 1;
        }
        if count == 3 {
            return 2;
        }
        3
    }

    /// Getter for `x2_a` `criterio_b`.
    pub fn get_x2_a_criterio_b(&self, r: &RiferimentoNISECI) -> Option<(u8, Option<f32>)> {
        if (self.cl2 + self.cl3) == 0 {
            return Some((3, None));
        }

        let ad_juv = (self.cl4 + self.cl5) as f32 / (self.cl2 + self.cl3) as f32;
        if ad_juv < self.ad_juv_soglia_1(r)? {
            return Some((3, Some(ad_juv)));
        }
        if ad_juv <= self.ad_juv_soglia_2(r)? {
            return Some((2, Some(ad_juv)));
        }
        if ad_juv <= self.ad_juv_soglia_3(r)? {
            return Some((1, Some(ad_juv)));
        }
        if ad_juv <= self.ad_juv_soglia_4(r)? {
            return Some((2, Some(ad_juv)));
        }
        Some((3, Some(ad_juv)))
    }

    /// Used for both `x2_a` and `x3`.
    // Its tests are the same as `crate::engines::niseci::x2::calculate_x2_a`.
    pub fn calculate_struttura_popolazione(
        &self,
        r: &RiferimentoNISECI,
    ) -> Result<(f32, MetricheX2A), String> {
        let criterio_a: u8 = self.get_x2_a_criterio_a();
        let (criterio_b, ad_juv): (u8, Option<f32>) = self.get_x2_a_criterio_b(r).ok_or(
            format!("id: {} not present in RiferimentoNISECI", self.id()),
        )?;

        if criterio_a == 1 && criterio_b == 3 {
            return Ok((
                0.5,
                MetricheX2A::new(criterio_a, MetricheX2aB::new(criterio_b, ad_juv)),
            ));
        }
        if criterio_a == 1 {
            return Ok((
                1.0,
                MetricheX2A::new(criterio_a, MetricheX2aB::new(criterio_b, ad_juv)),
            ));
        }
        if criterio_a == 2 && criterio_b == 3 {
            return Ok((
                0.0,
                MetricheX2A::new(criterio_a, MetricheX2aB::new(criterio_b, ad_juv)),
            ));
        }
        if criterio_a == 2 {
            return Ok((
                0.5,
                MetricheX2A::new(criterio_a, MetricheX2aB::new(criterio_b, ad_juv)),
            ));
        }
        if criterio_a == 3 {
            return Ok((
                0.0,
                MetricheX2A::new(criterio_a, MetricheX2aB::new(criterio_b, ad_juv)),
            ));
        }
        Err(format!(
            "Il Criterio A o B di x2a è diverso da 1 o 2 o 3. criterio A = {}, criterio B = {}",
            criterio_a, criterio_b
        ))
    }
}

/// Intermediate species-specific `x3` values for NISECI calc.
pub struct InfoIntermediePopolazioniNISECI {
    criterio_a: u8,
    criterio_b: u8,
    rapporto_ad_juv: Option<f32>,
}

impl InfoIntermediePopolazioniNISECI {
    /// Construct a new instance.
    pub fn new(
        criterio_a: u8,
        criterio_b: u8,
        rapporto_ad_juv: Option<f32>,
    ) -> InfoIntermediePopolazioniNISECI {
        InfoIntermediePopolazioniNISECI {
            criterio_a,
            criterio_b,
            rapporto_ad_juv,
        }
    }
    /// Getter for `criterio_a`.
    pub fn get_criterio_a(&self) -> u8 {
        self.criterio_a
    }
    /// Getter for `criterio_b`.
    pub fn get_criterio_b(&self) -> u8 {
        self.criterio_b
    }
    /// Getter for `rapporto_ad_juv`.
    pub fn get_rapporto_ad_juv(&self) -> Option<f32> {
        self.rapporto_ad_juv
    }
}

/// Utility struct that helps in [`crate::engines::niseci::x3`].
pub struct InfoPopolazioniNISECI {
    pub(crate) popolazione_piu_strutt: f32,
    pub(crate) species_strutt: u32,
    pub(crate) species_mediamente_strutt: u32,
    pub(crate) species_destrutt: u32,
    pub(crate) tot_species: usize,
    intermediates_map: HashMap<IdSpecieNISECI, InfoIntermediePopolazioniNISECI>,
}

impl Default for InfoPopolazioniNISECI {
    fn default() -> Self {
        Self::new()
    }
}

impl InfoPopolazioniNISECI {
    /// Construct a new instance.
    pub fn new() -> InfoPopolazioniNISECI {
        InfoPopolazioniNISECI {
            popolazione_piu_strutt: 0.0,
            species_strutt: 0,
            species_mediamente_strutt: 0,
            species_destrutt: 0,
            tot_species: 0,
            intermediates_map: HashMap::new(),
        }
    }

    /// Getter for `popolazione_piu_strutt`.
    #[inline(always)]
    pub fn popolazione_piu_strutt(&self) -> f32 {
        self.popolazione_piu_strutt
    }

    /// Getter for `species_strutt`.
    #[inline(always)]
    pub fn species_strutt(&self) -> u32 {
        self.species_strutt
    }

    /// Getter for `species_mediamente_strutt`.
    #[inline(always)]
    pub fn species_mediamente_strutt(&self) -> u32 {
        self.species_mediamente_strutt
    }

    /// Getter for `species_destrutt`.
    #[inline(always)]
    pub fn species_destrutt(&self) -> u32 {
        self.species_destrutt
    }

    /// Getter for `tot_species`.
    #[inline(always)]
    pub fn tot_species(&self) -> usize {
        self.tot_species
    }

    /// Getter for [`InfoIntermediPopolazioniNISECI`] map.
    #[inline(always)]
    pub fn intermediates_map(&self) -> &HashMap<IdSpecieNISECI, InfoIntermediePopolazioniNISECI> {
        &self.intermediates_map
    }

    /// Getter for [`InfoPopolazioniNISECI`] from [`ClassiEtaSpecieNISECI`] map.
    pub fn get_info_pop(
        map: &HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI>,
        r: &RiferimentoNISECI,
    ) -> Result<InfoPopolazioniNISECI, Vec<String>> {
        let mut errors: Vec<String> = Vec::with_capacity(map.len()); // prenoto ora e poi restringo dopo

        let mut info_pop = InfoPopolazioniNISECI::new();
        info_pop.tot_species = map.len();
        let epsilon: f32 = 1e-6;
        for classe in map.values() {
            match classe.calculate_struttura_popolazione(r) {
                Ok((popolazione, criteri_x2_a)) => {
                    if info_pop.popolazione_piu_strutt() < popolazione {
                        info_pop.popolazione_piu_strutt = popolazione;
                    }
                    if (popolazione - 1.0).abs() < epsilon {
                        info_pop.species_strutt += 1;
                    }
                    if (popolazione - 0.5).abs() < epsilon {
                        info_pop.species_mediamente_strutt += 1;
                    }
                    if popolazione.abs() < epsilon {
                        info_pop.species_destrutt += 1;
                    }
                    let criterio_a = criteri_x2_a.get_criterio_a();
                    let criterio_b = criteri_x2_a.get_criterio_b();
                    let rapporto_ad_juv = criteri_x2_a.get_rapporto_ad_juv();

                    info_pop.intermediates_map.insert(
                        classe.id(),
                        InfoIntermediePopolazioniNISECI::new(
                            criterio_a,
                            criterio_b,
                            rapporto_ad_juv,
                        ),
                    );
                }
                Err(error) => errors.push(error),
            }
        }

        if !errors.is_empty() {
            errors.shrink_to_fit();
            return Err(errors);
        }

        Ok(info_pop)
    }
}

/// Utility struct that helps in [`crate::engines::niseci::x3`].
/// Once initialised we have all data needed to calc `x3`.
/// For each allochthonous specimen we have a [`InfoPopolazioniNISECI`].
pub struct InfoPopolazioniAlieneNISECI {
    pub(crate) tipo_1: InfoPopolazioniNISECI,
    pub(crate) tipo_2: InfoPopolazioniNISECI,
    pub(crate) tipo_3: InfoPopolazioniNISECI,
    pub(crate) tot_specie_aliene: usize,
    pub(crate) tot_specie_autoctone: usize,
}

impl InfoPopolazioniAlieneNISECI {
    #[cfg(test)]
    pub fn new() -> InfoPopolazioniAlieneNISECI {
        InfoPopolazioniAlieneNISECI {
            tipo_1: InfoPopolazioniNISECI::new(),
            tipo_2: InfoPopolazioniNISECI::new(),
            tipo_3: InfoPopolazioniNISECI::new(),
            tot_specie_aliene: 0,
            tot_specie_autoctone: 0,
        }
    }

    /// Getter for `tipo_1` [`InfoPopolazioniNISECI`].
    #[inline(always)]
    pub fn tipo_1(&self) -> &InfoPopolazioniNISECI {
        &self.tipo_1
    }
    /// Getter for `tipo_2` [`InfoPopolazioniNISECI`].
    #[inline(always)]
    pub fn tipo_2(&self) -> &InfoPopolazioniNISECI {
        &self.tipo_2
    }
    /// Getter for `tipo_3` [`InfoPopolazioniNISECI`].
    #[inline(always)]
    pub fn tipo_3(&self) -> &InfoPopolazioniNISECI {
        &self.tipo_3
    }

    /// Getter for `tot_specie_aliene`.
    #[inline(always)]
    pub fn tot_specie_aliene(&self) -> usize {
        self.tot_specie_aliene
    }

    /// Getter for `tot_specie_autoctone`.
    #[inline(always)]
    pub fn tot_specie_autoctone(&self) -> usize {
        self.tot_specie_autoctone
    }

    /// Getter for [`InfoPopolazioniAlieneNISECI`] from [`ClassiEtaAlieniNISECI`].
    pub fn get_info_pop_aliene(
        classi_eta: &ClassiEtaAlieniNISECI,
        r: &RiferimentoNISECI,
    ) -> Result<InfoPopolazioniAlieneNISECI, Vec<String>> {
        let tipo_1 = InfoPopolazioniNISECI::get_info_pop(&classi_eta.map_tipo_1, r)?;
        let tipo_2 = InfoPopolazioniNISECI::get_info_pop(&classi_eta.map_tipo_2, r)?;
        let tipo_3 = InfoPopolazioniNISECI::get_info_pop(&classi_eta.map_tipo_3, r)?;

        let info_pop_aliene = InfoPopolazioniAlieneNISECI {
            tipo_1,
            tipo_2,
            tipo_3,
            tot_specie_aliene: classi_eta.tot_specie_aliene,
            tot_specie_autoctone: classi_eta.tot_specie_autoctone,
        };

        Ok(info_pop_aliene)
    }

    /// Getter for `species_mediamente_strutt`.
    pub fn get_species_mediamente_strutt(&self) -> u32 {
        self.tipo_1.species_mediamente_strutt()
            + self.tipo_2.species_mediamente_strutt()
            + self.tipo_3.species_mediamente_strutt()
    }
    /// Getter for `species_destrutt`.
    pub fn get_species_destrutt(&self) -> u32 {
        self.tipo_1.species_destrutt()
            + self.tipo_2.species_destrutt()
            + self.tipo_3.species_destrutt()
    }
}

/// Utility struct that helps in [`crate::engines::niseci::x3`].
/// Splits alien species according to their type.
pub struct ClassiEtaAlieniNISECI {
    pub(crate) map_tipo_1: HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI>,
    pub(crate) map_tipo_2: HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI>,
    pub(crate) map_tipo_3: HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI>,
    tot_specie_aliene: usize,
    tot_specie_autoctone: usize,
}

impl Default for ClassiEtaAlieniNISECI {
    fn default() -> Self {
        Self::new()
    }
}

impl ClassiEtaAlieniNISECI {
    /// Construct a new instance.
    pub fn new() -> ClassiEtaAlieniNISECI {
        Self::new_with_capacity::<DefaultCapacity>()
    }
    pub(crate) fn new_with_capacity<C: Capacity>() -> ClassiEtaAlieniNISECI {
        ClassiEtaAlieniNISECI {
            map_tipo_1: HashMap::with_capacity(C::VALUE),
            map_tipo_2: HashMap::with_capacity(C::VALUE),
            map_tipo_3: HashMap::with_capacity(C::VALUE),
            tot_specie_aliene: 0,
            tot_specie_autoctone: 0,
        }
    }
    /// Getter for `tipo_1` [`ClassiEtaSpecieNISECI`] map.
    #[inline(always)]
    pub fn map_tipo_1(&self) -> &HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI> {
        &self.map_tipo_1
    }
    /// Getter for `tipo_2` [`ClassiEtaSpecieNISECI`] map.
    #[inline(always)]
    pub fn map_tipo_2(&self) -> &HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI> {
        &self.map_tipo_2
    }
    /// Getter for `tipo_3` [`ClassiEtaSpecieNISECI`] map.
    #[inline(always)]
    pub fn map_tipo_3(&self) -> &HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI> {
        &self.map_tipo_3
    }
    /// Getter for `tot_specie_aliene`].
    #[inline(always)]
    pub fn tot_specie_aliene(&self) -> usize {
        self.tot_specie_aliene
    }
    /// Setter for `tot_specie_aliene`].
    #[inline(always)]
    pub fn set_tot_specie_aliene(&mut self, val: usize) {
        self.tot_specie_aliene = val;
    }
    /// Getter for `tot_specie_autoctone`].
    #[inline(always)]
    pub fn tot_specie_autoctone(&self) -> usize {
        self.tot_specie_autoctone
    }
    /// Setter for `tot_specie_autoctone`].
    #[inline(always)]
    pub fn set_tot_specie_autoctone(&mut self, val: usize) {
        self.tot_specie_autoctone = val;
    }
}

/// Utility struct that stores, for a given [`SpecieNISECI`], the number of
/// specimens found split by their capture step.
pub struct EsemplariPerCattura {
    id: IdSpecieNISECI,
    mappa: HashMap<u8, u32>, // la key è il numero del passaggio
}

impl EsemplariPerCattura {
    /// Getter for integer id.
    #[inline(always)]
    pub fn id(&self) -> IdSpecieNISECI {
        self.id
    }
    /// Getter for passage number / number of specimens map.
    #[inline(always)]
    pub fn mappa(&self) -> &HashMap<u8, u32> {
        &self.mappa
    }
    /// Getter for `dens_soglia_1`.
    #[inline(always)]
    pub fn dens_soglia_1(&self, r: &RiferimentoNISECI) -> Option<f32> {
        r.get_ref_by_plain_id(self.id()).map(|s| s.dens_soglia_1())
    }
    /// Getter for `dens_soglia_2`.
    #[inline(always)]
    pub fn dens_soglia_2(&self, r: &RiferimentoNISECI) -> Option<f32> {
        r.get_ref_by_plain_id(self.id()).map(|s| s.dens_soglia_2())
    }

    /// Construct a new instance.
    pub fn new(id: IdSpecieNISECI) -> Self {
        EsemplariPerCattura {
            id,
            mappa: HashMap::new(),
        }
    }

    pub fn fill_passaggio(&mut self, numero_passaggio: u8) {
        match self.mappa.entry(numero_passaggio) {
            Entry::Occupied(occupied) => {
                let numero_esemplari = occupied.get() + 1;
                self.mappa.insert(numero_passaggio, numero_esemplari);
            }
            Entry::Vacant(_) => {
                self.mappa.insert(numero_passaggio, 1);
            }
        }
    }
}

/// Enum for the final result of a NISECI calc
/// See:
/// - [`crate::engines::niseci::full::calculate_stato_ecologico_niseci`]
pub enum StatoEcologicoNISECI {
    Elevato,
    Buono,
    Moderato,
    Scadente,
    Cattivo,
}

impl fmt::Display for StatoEcologicoNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            StatoEcologicoNISECI::Elevato => "Elevato",
            StatoEcologicoNISECI::Buono => "Buono",
            StatoEcologicoNISECI::Moderato => "Moderato",
            StatoEcologicoNISECI::Scadente => "Scadente",
            StatoEcologicoNISECI::Cattivo => "Cattivo",
        };
        write!(f, "{}", string_representation)
    }
}

/// Threshold for excellent NISECI ecological status.
const STATO_ECOLOGICO_NISECI_SOGLIA_ELEVATO: f32 = 0.8;
/// Threshold for good NISECI ecological status for alpine areas.
const STATO_ECOLOGICO_NISECI_SOGLIA_BUONO_AREA_ALPINA: f32 = 0.52;
/// Threshold for good NISECI ecological status for mediterranean areas.
const STATO_ECOLOGICO_NISECI_SOGLIA_BUONO_AREA_MEDITERRANEA: f32 = 0.6;
/// Threshold for moderate NISECI ecological status for alpine areas.
const STATO_ECOLOGICO_NISECI_SOGLIA_MODERATO: f32 = 0.4;
/// Threshold for poor NISECI ecological status for alpine areas.
const STATO_ECOLOGICO_NISECI_SOGLIA_SCADENTE: f32 = 0.2;

impl From<(f32, &AreaNISECI)> for StatoEcologicoNISECI {
    fn from((val, area): (f32, &AreaNISECI)) -> Self {
        if val >= STATO_ECOLOGICO_NISECI_SOGLIA_ELEVATO {
            return StatoEcologicoNISECI::Elevato;
        }
        match area {
            AreaNISECI::Alpina => {
                if val >= STATO_ECOLOGICO_NISECI_SOGLIA_BUONO_AREA_ALPINA {
                    return StatoEcologicoNISECI::Buono;
                }
            }
            AreaNISECI::Mediterranea => {
                if val >= STATO_ECOLOGICO_NISECI_SOGLIA_BUONO_AREA_MEDITERRANEA {
                    return StatoEcologicoNISECI::Buono;
                }
            }
        }
        if val >= STATO_ECOLOGICO_NISECI_SOGLIA_MODERATO {
            return StatoEcologicoNISECI::Moderato;
        }
        if val >= STATO_ECOLOGICO_NISECI_SOGLIA_SCADENTE {
            return StatoEcologicoNISECI::Scadente;
        }
        StatoEcologicoNISECI::Cattivo
    }
}

#[cfg(test)]
mod domain_niseci_private_tests {
    use super::*;

    #[cfg(test)]
    impl RiferimentoNISECI {
        #[cfg(test)]
        pub(crate) fn push(&mut self, value: SpecieNISECI) -> IdSpecieNISECI {
            eprintln!("Pushing specie {} in riferimento", value.id());
            if !self.contains_id(value.id()) {
                eprintln!(
                    "Specie id {} nome {} was NOT in riferimento already",
                    value.id(),
                    value.nome()
                );
                self.map_ids.intern(value)
            } else {
                let inner_id = self
                    .get_inner_id(value.id())
                    .expect("contains_id was checked");
                eprintln!(
                    "Specie id {} nome {} was in riferimento already with id",
                    inner_id,
                    value.nome()
                );
                inner_id
            }
        }
    }

    #[cfg(test)]
    impl ClassiEtaSpecieNISECI {
        #[cfg(test)]
        pub(crate) fn new_custom(
            id: IdSpecieNISECI,
            cl1: u32,
            cl2: u32,
            cl3: u32,
            cl4: u32,
            cl5: u32,
        ) -> Self {
            Self {
                id,
                cl1,
                cl2,
                cl3,
                cl4,
                cl5,
            }
        }
    }
    #[cfg(test)]
    impl CampionamentoNISECI {
        #[cfg(test)]
        pub(crate) fn push(&mut self, value: RecordNISECI) {
            self.campionamento.push(value);
        }
        #[cfg(test)]
        pub(crate) fn as_mut_vec(&mut self) -> &mut Vec<RecordNISECI> {
            &mut self.campionamento
        }
    }

    #[cfg(test)]
    impl AnagraficaNISECI {
        /// Test helper to build unchecked instances
        #[cfg(test)]
        pub(crate) fn new_raw_unchecked(
            comunita: ComunitaNISECI,
            codice_stazione: String,
            date_string: String,
            area: AreaNISECI,
            corpo_idrico: String,
            bacino_appartenenza: String,
            idro_eco_regione: IdroEcoRegioneNISECI,
            posizione: Location,
            lunghezza_media_stazione: f32,
            larghezza_media_stazione: f32,
        ) -> Self {
            Self {
                comunita,
                codice_stazione,
                date_string,
                area,
                corpo_idrico,
                bacino_appartenenza,
                idro_eco_regione,
                posizione,
                lunghezza_media_stazione,
                larghezza_media_stazione,
            }
        }
        /// Test helper to mutate lunghezza with no checks
        #[cfg(test)]
        pub(crate) fn set_lunghezza_unchecked(&mut self, val: f32) {
            self.lunghezza_media_stazione = val;
        }
        /// Test helper to mutate larghezza with no checks
        #[cfg(test)]
        pub(crate) fn set_larghezza_unchecked(&mut self, val: f32) {
            self.larghezza_media_stazione = val;
        }
    }
}
