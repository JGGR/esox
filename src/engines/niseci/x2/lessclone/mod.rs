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
//! This module contains refactored methods used with lessclone feature.
//!
//! Next version will have these in the base [`esox::engines::niseci::x2`] module.

use std::collections::{hash_map::Entry, HashMap, HashSet};

use crate::capacity::{Capacity, DefaultCapacity};
use crate::domain::niseci::lessclone::{
    CampionamentoNISECI, ClassiEtaSpecieNISECI, EsemplariPerCattura,
};
use crate::domain::niseci::{
    AnagraficaNISECI, IdSpecieNISECI, MetricheX2A, MetricheX2aB, RiferimentoNISECI,
};
use crate::domain::posf32::PositiveF32;
use crate::engines::niseci::linear_regression::{calculate_quantita_with_regression, Point};

#[derive(Clone)]
pub struct SubmetricheX2 {
    metriche_x2_a: MetricheX2A,
    classi_eta: ClassiEtaSpecieNISECI,
    metriche_x2_b: MetricheX2B,
}

impl SubmetricheX2 {
    pub fn new(
        metriche_x2_a: MetricheX2A,
        classi_eta: ClassiEtaSpecieNISECI,
        metriche_x2_b: MetricheX2B,
    ) -> Self {
        Self {
            metriche_x2_a,
            classi_eta,
            metriche_x2_b,
        }
    }
    #[inline(always)]
    pub fn get_metriche_x2_a(&self) -> MetricheX2A {
        self.metriche_x2_a
    }
    #[inline(always)]
    pub fn get_ref_classi_eta(&self) -> &ClassiEtaSpecieNISECI {
        &self.classi_eta
    }
    #[inline(always)]
    pub fn get_ref_metriche_x2_b(&self) -> &MetricheX2B {
        &self.metriche_x2_b
    }
}

pub struct MetricheX2 {
    criterio_a: f32,
    criterio_b: f32,
    submetriche_map: HashMap<IdSpecieNISECI, SubmetricheX2>,
}

impl MetricheX2 {
    pub fn new(
        criterio_a: f32,
        criterio_b: f32,
        submetriche_map: HashMap<IdSpecieNISECI, SubmetricheX2>,
    ) -> Self {
        Self {
            criterio_a,
            criterio_b,
            submetriche_map,
        }
    }
    #[inline(always)]
    pub fn get_criterio_a(&self) -> f32 {
        self.criterio_a
    }
    #[inline(always)]
    pub fn get_criterio_b(&self) -> f32 {
        self.criterio_b
    }
    #[inline(always)]
    pub fn get_ref_submetriche_map(&self) -> &HashMap<IdSpecieNISECI, SubmetricheX2> {
        &self.submetriche_map
    }
}

#[derive(Clone)]
pub struct MetricheX2B {
    id_specie: IdSpecieNISECI,
    densita_stimata: f32,
    quantita_stimata: u32,
    x2_b: f32,
}

impl MetricheX2B {
    pub fn new(
        id_specie: IdSpecieNISECI,
        densita_stimata: f32,
        quantita_stimata: u32,
        x2_b: f32,
    ) -> Self {
        Self {
            id_specie,
            densita_stimata,
            quantita_stimata,
            x2_b,
        }
    }
    #[inline(always)]
    pub fn get_id(&self) -> IdSpecieNISECI {
        self.id_specie
    }
    #[inline(always)]
    pub fn get_x2_b(&self) -> f32 {
        self.x2_b
    }
    #[inline(always)]
    pub fn get_densita_stimata(&self) -> f32 {
        self.densita_stimata
    }
    #[inline(always)]
    pub fn get_quantita_stimata(&self) -> u32 {
        self.quantita_stimata
    }
}

pub fn calculate_x2(
    campionamento: &CampionamentoNISECI,
    anagrafica: &AnagraficaNISECI,
    riferimento: &RiferimentoNISECI,
    require_specie_attesa: bool,
) -> Result<(Option<f32>, MetricheX2), Vec<String>> {
    let (x2_a, criteri_vec) = calculate_sommatoria_x2_a::<DefaultCapacity>(
        campionamento,
        riferimento,
        require_specie_attesa,
    )?;
    let (x2_b, densita_vec) = calculate_sommatoria_x2_b::<DefaultCapacity>(
        campionamento,
        anagrafica,
        riferimento,
        require_specie_attesa,
    )?;

    let mut submetriche = HashMap::<IdSpecieNISECI, SubmetricheX2>::new();

    for crit in &criteri_vec {
        match submetriche.entry(crit.get_id()) {
            Entry::Occupied(_) => {}
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(
                    // We fill densita_stimata later
                    SubmetricheX2::new(
                        crit.get_metriche_x2a(),
                        crit.get_ref_classi_eta().clone(),
                        MetricheX2B::new(crit.get_id(), -1.0, 0, 0.0),
                    ),
                );
            }
        }
    }

    let mut errors = Vec::<String>::new();

    fill_submetriche(densita_vec, &mut submetriche, &mut errors);

    if !errors.is_empty() {
        // In case the densita_vec had some problems
        return Err(errors);
    }

    let metriche_x2 = MetricheX2::new(x2_a, x2_b, submetriche);

    let mut set_specie_campionate = HashSet::new();
    for cattura in campionamento {
        let id = cattura.id();
        debug_assert!(riferimento.contains_plain_id(id));
        let specie = riferimento
            .get_ref_by_plain_id(id)
            .expect("Riferimento should contain this id");
        if (specie.specie_attesa() == require_specie_attesa)
            && (specie.tipo_autoctono() == 1 || specie.tipo_autoctono() == 2)
        {
            set_specie_campionate.insert(id);
        }
    }

    calculate_x2_absolute(metriche_x2, x2_a, x2_b, &set_specie_campionate)
}

pub fn calculate_x2_per_alloctone(
    campionamento: &CampionamentoNISECI,
    riferimento: &RiferimentoNISECI,
    anagrafica: &AnagraficaNISECI,
) -> Result<(Option<f32>, MetricheX2), Vec<String>> {
    let (x2_a, criteri_vec) =
        calculate_sommatoria_x2_a_per_alloctone::<DefaultCapacity>(campionamento, riferimento)?;
    let (x2_b, densita_vec) = calculate_sommatoria_x2_b_per_alloctone::<DefaultCapacity>(
        campionamento,
        riferimento,
        anagrafica,
    )?;

    let mut submetriche = HashMap::<IdSpecieNISECI, SubmetricheX2>::new();

    for crit in &criteri_vec {
        match submetriche.entry(crit.get_id()) {
            Entry::Occupied(_) => {}
            Entry::Vacant(vacant_entry) => {
                vacant_entry.insert(
                    // We fill densita_stimata later
                    SubmetricheX2::new(
                        crit.get_metriche_x2a(),
                        crit.get_ref_classi_eta().clone(),
                        MetricheX2B::new(crit.get_id(), -1.0, 0, 0.0),
                    ),
                );
            }
        }
    }

    let mut errors = Vec::<String>::new();

    fill_submetriche(densita_vec, &mut submetriche, &mut errors);

    if !errors.is_empty() {
        // In case the densita_vec had some problems
        return Err(errors);
    }

    let metriche_x2 = MetricheX2::new(x2_a, x2_b, submetriche);

    let mut set_specie_campionate = HashSet::new();
    for cattura in campionamento {
        let id = cattura.id();
        let specie = riferimento
            .get_ref_by_plain_id(id)
            .expect("Riferimento should contain this id");
        if specie.tipo_alloctono() > 0 {
            set_specie_campionate.insert(id);
        }
    }

    calculate_x2_absolute(metriche_x2, x2_a, x2_b, &set_specie_campionate)
}

struct RecordSubmetricheX2A {
    id: IdSpecieNISECI,
    metriche_x2a: MetricheX2A,
    classi_eta: ClassiEtaSpecieNISECI,
}

impl RecordSubmetricheX2A {
    pub fn new(
        id: IdSpecieNISECI,
        metriche_x2a: MetricheX2A,
        classi_eta: ClassiEtaSpecieNISECI,
    ) -> Self {
        Self {
            id,
            metriche_x2a,
            classi_eta,
        }
    }
    #[inline(always)]
    pub(crate) fn get_id(&self) -> IdSpecieNISECI {
        self.id
    }
    #[inline(always)]
    pub fn get_metriche_x2a(&self) -> MetricheX2A {
        self.metriche_x2a
    }
    #[inline(always)]
    pub(crate) fn get_ref_classi_eta(&self) -> &ClassiEtaSpecieNISECI {
        &self.classi_eta
    }
}

fn calculate_x2_absolute(
    metriche_x2: MetricheX2,
    x2_a: f32,
    x2_b: f32,
    specie_campionate_set: &HashSet<IdSpecieNISECI>,
) -> Result<(Option<f32>, MetricheX2), Vec<String>> {
    let tot_specie_attese_trovate = specie_campionate_set.len();

    if tot_specie_attese_trovate == 0 {
        // Nel caso in cui nessuna specie attesa sia presente nel campionamento
        return Ok((None, metriche_x2));
    }

    let result = (0.6 * x2_a + 0.4 * x2_b) / tot_specie_attese_trovate as f32;

    let rounded_result = (1000.0 * result).round() / 1000.0;

    Ok((Some(rounded_result), metriche_x2))
}

fn calculate_sommatoria_x2_a<C: Capacity>(
    c: &CampionamentoNISECI,
    r: &RiferimentoNISECI,
    require_specie_attesa: bool,
) -> Result<(f32, Vec<RecordSubmetricheX2A>), Vec<String>> {
    // ad ogni specie associo le loro classi che andrò poi a riempire
    // ho controllato i campionamenti di andrea e trovto massimo 9 specie diverse
    // per sicurezza prealloco memoria per 10 classi di eta
    let mut classi_eta_map: HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI> =
        HashMap::with_capacity(C::VALUE);

    // riempo l'hashmap con solo le specie autoctone campionate
    for cattura in c {
        let id = cattura.id();
        debug_assert!(r.contains_plain_id(id));
        let specie = r
            .get_ref_by_plain_id(id)
            .expect("Riferimento should contain this id");
        if (specie.specie_attesa() == require_specie_attesa)
            && (specie.tipo_autoctono() == 1 || specie.tipo_autoctono() == 2)
        {
            match classi_eta_map.entry(id) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().update_classi_eta(cattura, r);
                }
                Entry::Vacant(entry) => {
                    ClassiEtaSpecieNISECI::new_cl_prevalorizzata(cattura, r)
                        .and_then(|c| Some(entry.insert(c)));
                }
            };
        }
    }

    calculate_sommatoria_x2_a_absolute(classi_eta_map, r)
}

fn calculate_sommatoria_x2_b<C: Capacity>(
    c: &CampionamentoNISECI,
    anagrafica: &AnagraficaNISECI,
    r: &RiferimentoNISECI,
    require_specie_attesa: bool,
) -> Result<(f32, Vec<MetricheX2B>), Vec<String>> {
    let width = anagrafica.get_larghezza_media();
    let length = anagrafica.get_lunghezza_media();
    let width_checked = PositiveF32::new(width).map_err(|e| vec![e.to_string()])?;
    let length_checked = PositiveF32::new(length).map_err(|e| vec![e.to_string()])?;
    let superficie: f32 = *width_checked * *length_checked;
    let mut esemplari_per_cattura_map: HashMap<IdSpecieNISECI, EsemplariPerCattura> =
        HashMap::with_capacity(C::VALUE);

    for cattura in c {
        let id = cattura.id();
        debug_assert!(r.contains_plain_id(id));
        let specie = r
            .get_ref_by_plain_id(id)
            .expect("Riferimento should contain this id");
        if (specie.specie_attesa() == require_specie_attesa)
            && (specie.tipo_autoctono() == 1 || specie.tipo_autoctono() == 2)
        {
            match esemplari_per_cattura_map.entry(id) {
                Entry::Occupied(mut occupied_entry) => {
                    occupied_entry
                        .get_mut()
                        .fill_passaggio(cattura.passaggio_cattura());
                }
                Entry::Vacant(vacant_entry) => {
                    let mut epc = EsemplariPerCattura::new(id);
                    epc.fill_passaggio(cattura.passaggio_cattura());
                    vacant_entry.insert(epc);
                }
            }
        }
    }

    calculate_sommatoria_x2_b_absolute(esemplari_per_cattura_map, r, superficie)
}

fn calculate_sommatoria_x2_a_per_alloctone<C: Capacity>(
    c: &CampionamentoNISECI,
    r: &RiferimentoNISECI,
) -> Result<(f32, Vec<RecordSubmetricheX2A>), Vec<String>> {
    // ad ogni specie associo le loro classi che andrò poi a riempire
    // ho controllato i campionamenti di andrea e trovto massimo 9 specie diverse
    // per sicurezza prealloco memoria per 10 classi di eta
    let mut classi_eta_map: HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI> =
        HashMap::with_capacity(C::VALUE);

    // riempo l'hashmap con solo le specie autoctone campionate
    for cattura in c {
        let id = cattura.id();
        debug_assert!(r.contains_plain_id(id));
        let specie = r
            .get_ref_by_plain_id(id)
            .expect("Riferimento should contain this id");
        if specie.tipo_alloctono() > 0 {
            match classi_eta_map.entry(id) {
                Entry::Occupied(mut entry) => {
                    entry.get_mut().update_classi_eta(cattura, r);
                }
                Entry::Vacant(entry) => {
                    ClassiEtaSpecieNISECI::new_cl_prevalorizzata(cattura, r)
                        .and_then(|c| Some(entry.insert(c)));
                }
            };
        }
    }

    calculate_sommatoria_x2_a_absolute(classi_eta_map, r)
}

fn calculate_sommatoria_x2_b_per_alloctone<C: Capacity>(
    c: &CampionamentoNISECI,
    r: &RiferimentoNISECI,
    anagrafica: &AnagraficaNISECI,
) -> Result<(f32, Vec<MetricheX2B>), Vec<String>> {
    let width = anagrafica.get_larghezza_media();
    let length = anagrafica.get_lunghezza_media();
    if width <= 0.0 {
        return Err(vec!["Width too small".to_string()]);
    }
    if !width.is_finite() {
        return Err(vec!["Width not finite".to_string()]);
    }
    if length <= 0.0 {
        return Err(vec!["Length too small".to_string()]);
    }
    if !length.is_finite() {
        return Err(vec!["Length not finite".to_string()]);
    }
    let superficie = width * length;
    let mut esemplari_per_cattura_map: HashMap<IdSpecieNISECI, EsemplariPerCattura> =
        HashMap::with_capacity(C::VALUE);

    for cattura in c {
        let id = cattura.id();
        debug_assert!(r.contains_plain_id(id));
        let specie = r
            .get_ref_by_plain_id(id)
            .expect("Riferimento should contain this id");
        if specie.tipo_alloctono() > 0 {
            match esemplari_per_cattura_map.entry(id) {
                Entry::Occupied(mut occupied_entry) => {
                    occupied_entry
                        .get_mut()
                        .fill_passaggio(cattura.passaggio_cattura());
                }
                Entry::Vacant(vacant_entry) => {
                    let mut epc = EsemplariPerCattura::new(id);
                    epc.fill_passaggio(cattura.passaggio_cattura());
                    vacant_entry.insert(epc);
                }
            }
        }
    }

    calculate_sommatoria_x2_b_absolute(esemplari_per_cattura_map, r, superficie)
}

fn calculate_sommatoria_x2_a_absolute(
    classi_eta_map: HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI>,
    r: &RiferimentoNISECI,
) -> Result<(f32, Vec<RecordSubmetricheX2A>), Vec<String>> {
    // ora la mappa è riempita e tutte le classi sono state riempite
    // si procede quindi al calcolo di x2 a per ogni specie campionata autoctona
    // e si va a fare la sommatoria dei parametri trovati

    let mut sommatoria_x2_a = 0.0;
    let mut errors: Vec<String> = Vec::with_capacity(classi_eta_map.len()); // prenoto ora e poi restringo dopo
    let mut criteri_vec: Vec<RecordSubmetricheX2A> = Vec::with_capacity(classi_eta_map.len());
    for classe in classi_eta_map.values() {
        match calculate_x2_a(classe, r) {
            Ok((x2_a, criteri_x2_a)) => {
                let criterio_a = criteri_x2_a.get_criterio_a();
                let criterio_b = criteri_x2_a.get_criterio_b();
                let ad_juv = criteri_x2_a.get_rapporto_ad_juv();
                sommatoria_x2_a += x2_a;
                criteri_vec.push(RecordSubmetricheX2A::new(
                    classe.id(),
                    MetricheX2A::new(criterio_a, MetricheX2aB::new(criterio_b, ad_juv)),
                    classe.clone(),
                ));
            }
            Err(error) => errors.push(error),
        }
    }

    if !errors.is_empty() {
        errors.shrink_to_fit();
        return Err(errors);
    }

    criteri_vec.shrink_to_fit();

    Ok((sommatoria_x2_a, criteri_vec))
}

fn calculate_sommatoria_x2_b_absolute(
    esemplari_per_cattura_map: HashMap<IdSpecieNISECI, EsemplariPerCattura>,
    r: &RiferimentoNISECI,
    superficie: f32,
) -> Result<(f32, Vec<MetricheX2B>), Vec<String>> {
    // ora che abbiamo riempito la mappa con tutte le catture, possiamo andare
    // a calcolar x2b per ogni specie
    let mut sommatoria_x2_b = 0.0;
    let mut errors: Vec<String> = Vec::with_capacity(esemplari_per_cattura_map.len()); // prenoto ora e poi restringo dopo
    let mut densita_vec: Vec<MetricheX2B> = Vec::with_capacity(esemplari_per_cattura_map.len());
    for catture in esemplari_per_cattura_map.values() {
        match calculate_x2_b(catture, r, &superficie) {
            Ok((x2_b, densita_stimata, quantita_stimata)) => {
                sommatoria_x2_b += x2_b;
                densita_vec.push(MetricheX2B::new(
                    catture.id(),
                    densita_stimata,
                    quantita_stimata,
                    x2_b,
                ));
            }
            Err(err_mess) => errors.push(err_mess),
        }
    }

    // controllo se ci sono errori, se sì allora ritorno gli errori
    if !errors.is_empty() {
        errors.shrink_to_fit(); // restringo
        return Err(errors);
    }

    densita_vec.shrink_to_fit();

    Ok((sommatoria_x2_b, densita_vec)) // finally
}

/// fn wrapper del calcolo della struttura di una popolazione
fn calculate_x2_a(
    classe: &ClassiEtaSpecieNISECI,
    r: &RiferimentoNISECI,
) -> Result<(f32, MetricheX2A), String> {
    classe.calculate_struttura_popolazione(r)
}

fn calculate_x2_b(
    e: &EsemplariPerCattura,
    r: &RiferimentoNISECI,
    superficie: &f32,
) -> Result<(f32, f32, u32), String> {
    match get_quantita_stimata(e.mappa()) {
        Ok(q_stimata) => {
            // calcolo densita stimata
            let densita_stimata = q_stimata as f32 / superficie;

            // trovo ora x2_b
            if densita_stimata
                > e.dens_soglia_2(r)
                    .ok_or(format!("id: {} not present in RiferimentoNISECI", e.id()))?
            {
                return Ok((1.0, densita_stimata, q_stimata));
            }
            if densita_stimata
                > e.dens_soglia_1(r)
                    .ok_or(format!("id: {} not present in RiferimentoNISECI", e.id()))?
            {
                return Ok((0.5, densita_stimata, q_stimata));
            }
            Ok((0.0, densita_stimata, q_stimata))
        }
        Err(err_message) => Err(err_message),
    }
}

fn get_quantita_stimata(passaggi: &HashMap<u8, u32>) -> Result<u32, String> {
    if passaggi.len() == 1 {
        return passaggi
            .values()
            .next()
            .copied()
            .ok_or_else(|| "unexpected empty map".to_string());
    }

    // passaggi viene creata in calculate_sommatoria_x2_b()
    if passaggi.len() == 2 {
        let c1 = match passaggi.get(&1) {
            Some(v) => *v,
            None => return Err("missing key 1".to_string()),
        };

        let c2 = match passaggi.get(&2) {
            Some(v) => *v,
            None => return Err("missing key 2".to_string()),
        };

        return calculate_passaggi_ripetuti(c1, c2);
    }
    calculate_q_stimata_regression(passaggi)
}

fn calculate_passaggi_ripetuti(c1: u32, c2: u32) -> Result<u32, String> {
    if c1 == c2 || c1 == 0 || c2 == 0 {
        return Ok(c1 + c2);
    }

    let c = c1 + c2;
    let divisore = c2 as f32 / c1 as f32;

    let result = (c as f32 / (1.0 - divisore.powf(2.0))).round() as i32;

    match result > 0 {
        true => Ok(result as u32),
        false => Ok(c1 + c2), // ritorno somma come da accordi
    }
}

fn calculate_q_stimata_regression(passaggi: &HashMap<u8, u32>) -> Result<u32, String> {
    let ultimo_passaggio = match passaggi.keys().max() {
        Some(&k) => k,
        None => return Err("passaggi vuoto".to_string()),
    };

    // dalla mappa non riesco a capire se ci siano o meno dei passaggi in cui non è stato trovato pesce
    // quindi mi creo un vettore che rappresenta i pesci trovati per ogni passaggio in ordine di passaggio
    let mut esemplari_per_passaggio = vec![0_u32; ultimo_passaggio as usize];

    for (key, value) in passaggi {
        esemplari_per_passaggio[(*key - 1) as usize] = *value;
    }

    // ora creo i punti con x == esemplari catturati fino a quel passaggio
    // e y == esmplari catturati in quel passaggio
    let mut current_tot = 0;
    let points: Vec<Point<i32>> = esemplari_per_passaggio
        .iter()
        .map(|esemplari: &u32| {
            current_tot += esemplari;
            Point::new(current_tot as i32, *esemplari as i32)
        })
        .collect();

    calculate_quantita_with_regression(points.as_slice())
}

fn fill_submetriche(
    densita_vec: Vec<MetricheX2B>,
    submetriche: &mut HashMap<IdSpecieNISECI, SubmetricheX2>,
    errors: &mut Vec<String>,
) {
    for dens in &densita_vec {
        let id = dens.get_id();
        match submetriche.entry(id) {
            Entry::Occupied(mut entry) => {
                let submetr = entry.get_mut();
                *submetr = SubmetricheX2::new(
                    submetr.get_metriche_x2_a(),
                    submetr.get_ref_classi_eta().clone(),
                    MetricheX2B::new(
                        id,
                        dens.get_densita_stimata(),
                        dens.get_quantita_stimata(),
                        dens.get_x2_b(),
                    ),
                );
            }
            Entry::Vacant(_) => {
                errors.push(format!("Errore: specie {} ha una densita stimata ma manca degli altri valori intermedi", id));
            }
        }
    }
}

#[cfg(test)]
mod x2_private_tests {
    use super::*;
    use crate::tests::test_utils::get_ciaccio;

    #[test]
    fn calcola_q_stimata_regression() {
        let mut passaggi: HashMap<u8, u32> = HashMap::new();
        passaggi.insert(1, 70);
        passaggi.insert(2, 60);
        passaggi.insert(3, 20);
        passaggi.insert(4, 10);

        let q_stimata = calculate_q_stimata_regression(&passaggi);

        assert_eq!(Ok(190), q_stimata);
    }

    #[test]
    fn calcola_q_stimata_regression_m_positive() {
        let mut passaggi: HashMap<u8, u32> = HashMap::new();
        passaggi.insert(1, 50);
        passaggi.insert(2, 75);
        passaggi.insert(3, 100);

        let q_stimata = calculate_q_stimata_regression(&passaggi);

        assert_eq!(Ok(225), q_stimata);
    }

    #[test]
    fn calcola_q_stimata_regression_same_values() {
        let mut passaggi: HashMap<u8, u32> = HashMap::new();
        passaggi.insert(1, 50);
        passaggi.insert(2, 50);
        passaggi.insert(3, 50);

        let q_stimata = calculate_q_stimata_regression(&passaggi);

        assert_eq!(Ok(150), q_stimata);
    }

    #[test]
    fn calcola_passaggi_ripetuti() {
        let q_stimata_1 = calculate_passaggi_ripetuti(30, 12);

        assert_eq!(Ok(50), q_stimata_1);

        let q_stimata_2 = calculate_passaggi_ripetuti(30, 15);

        assert_eq!(Ok(60), q_stimata_2);
    }

    #[test]
    fn calcola_passaggi_ripetuti_negative() {
        let q_stimata = calculate_passaggi_ripetuti(15, 30);

        assert_eq!(Ok(45), q_stimata);
    }

    #[test]
    fn calcola_passaggi_ripetuti_same_values() {
        let q_stimata = calculate_passaggi_ripetuti(30, 30);

        assert_eq!(Ok(60), q_stimata);
    }

    #[test]
    fn get_quantita_stimata_regression() {
        let mut passaggi: HashMap<u8, u32> = HashMap::new();
        passaggi.insert(1, 70);
        passaggi.insert(2, 60);
        passaggi.insert(3, 20);
        passaggi.insert(4, 10);

        let q_stimata = get_quantita_stimata(&passaggi);

        assert_eq!(Ok(190), q_stimata);
    }

    #[test]
    fn get_quantita_stimata_passaggi_ripetuti() {
        let mut passaggi: HashMap<u8, u32> = HashMap::new();
        passaggi.insert(1, 30);
        passaggi.insert(2, 12);

        let q_stimata = get_quantita_stimata(&passaggi);

        assert_eq!(Ok(50), q_stimata);

        let mut passaggi2: HashMap<u8, u32> = HashMap::new();
        passaggi2.insert(1, 30);
        passaggi2.insert(2, 15);

        let q_stimata2 = get_quantita_stimata(&passaggi2);

        assert_eq!(Ok(60), q_stimata2);
    }

    #[test]
    fn get_quantita_stimata_passaggi_ripetuti_negative() {
        let mut passaggi: HashMap<u8, u32> = HashMap::new();
        passaggi.insert(1, 15);
        passaggi.insert(2, 30);

        let q_stimata = get_quantita_stimata(&passaggi);

        assert_eq!(Ok(45), q_stimata);
    }

    #[test]
    fn get_quantita_stimata_progression_m_positive() {
        let mut passaggi: HashMap<u8, u32> = HashMap::new();
        passaggi.insert(1, 50);
        passaggi.insert(2, 75);
        passaggi.insert(3, 100);

        let q_stimata = get_quantita_stimata(&passaggi);

        assert_eq!(Ok(225), q_stimata);
    }

    #[test]
    fn calculate_x2_b_buona() {
        let mut epc = EsemplariPerCattura::new(0);
        (0..30).for_each(|_| epc.fill_passaggio(1));
        (0..15).for_each(|_| epc.fill_passaggio(2));

        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_b = calculate_x2_b(&epc, &rif, &2.0);

        assert_eq!(Ok(1.0), x2_b.map(|(x2_b, _, _)| x2_b));
    }

    #[test]
    fn calculate_x2_b_test_intermedia() {
        let mut specie = get_ciaccio();
        specie.set_dens_soglia_1(20.0);
        specie.set_dens_soglia_2(30.0);

        let mut epc = EsemplariPerCattura::new(0);
        (0..30).for_each(|_| epc.fill_passaggio(1));
        (0..15).for_each(|_| epc.fill_passaggio(2));
        let rif = RiferimentoNISECI::new(vec![specie]);

        let x2_b = calculate_x2_b(&epc, &rif, &2.0);

        assert_eq!(Ok(0.5), x2_b.map(|(x2_b, _, _)| x2_b));
    }

    #[test]
    fn calculate_x2_b_test_scarsa() {
        let mut specie = get_ciaccio();
        specie.set_dens_soglia_1(30.0);
        specie.set_dens_soglia_2(40.0);

        let mut epc = EsemplariPerCattura::new(0);
        (0..30).for_each(|_| epc.fill_passaggio(1));
        (0..15).for_each(|_| epc.fill_passaggio(2));
        let rif = RiferimentoNISECI::new(vec![specie]);

        let x2_b = calculate_x2_b(&epc, &rif, &2.0);

        assert_eq!(Ok(0.0), x2_b.map(|(x2_b, _, _)| x2_b));
    }

    #[test]
    fn calculate_x2_a_test_ca_1_cb_3_giovani() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 3, 3, 1, 1);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(0.5), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_1_cb_3_adulti() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 1, 1, 3, 3);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(0.5), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_1_cb_2_adulti() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 1, 1, 2, 2);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(1.0), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_1_cb_2_giovani() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 2, 2, 1, 1);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(1.0), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_1_cb_1() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 1, 1, 1, 1);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(1.0), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_2_cb_1() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 0, 2, 1, 1);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(0.5), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_2_cb_2_adulti() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 0, 2, 2, 2);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(0.5), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_2_cb_2_giovani() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 2, 2, 2, 0);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(0.5), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_2_cb_3_adulti() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 0, 2, 3, 3);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(0.0), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_2_cb_3_giovani() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 0, 6, 1, 1);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(0.0), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_3_cb_3_adulti() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 0, 1, 0, 6);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(0.0), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_3_cb_3_giovani() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 0, 6, 0, 1);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(0.0), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_3_cb_1() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 0, 1, 0, 1);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(0.0), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_3_cb_2_giovani() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 0, 2, 0, 1);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(0.0), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_3_cb_2_adulti() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 0, 0, 1, 0, 2);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(0.0), x2_a.map(|(x2_a, _)| x2_a));
    }

    #[test]
    fn calculate_x2_a_test_ca_1_cb_3_adulti_cl1_valorizzato() {
        let classe = ClassiEtaSpecieNISECI::new_custom(0, 5, 0, 10, 20, 10);
        let rif = RiferimentoNISECI::new(vec![get_ciaccio()]);
        let x2_a = calculate_x2_a(&classe, &rif);

        assert_eq!(Ok(0.5), x2_a.map(|(x2_a, _)| x2_a));
    }
}
