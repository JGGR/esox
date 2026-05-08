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

use serde::{Deserialize, Serialize};
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fmt;
use std::vec::Vec;

use crate::domain::localize::CommaFormat;
use crate::domain::niseci::{
    AnagraficaNISECI, ClassiEta, IdSpecieNISECI, InfoIntermediePopolazioniNISECI, MetricheX2A,
    MetricheX2aB, RecordNISECI, RiferimentoNISECI, StatoEcologicoNISECI,
};

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

    #[inline(always)]
    pub fn densita_stimata(&self) -> f32 {
        self.densita_stimata
    }
    #[inline(always)]
    pub fn quantita_stimata(&self) -> u32 {
        self.quantita_stimata
    }
    #[inline(always)]
    pub fn classi_eta(&self) -> &ClassiEtaSpecieNISECI {
        &self.classi_eta
    }
    #[inline(always)]
    pub fn rapporto_ad_juv(&self) -> Option<f32> {
        self.rapporto_ad_juv
    }
    #[inline(always)]
    pub fn x2_a_a(&self) -> u8 {
        self.x2_a_a
    }
    #[inline(always)]
    pub fn x2_a_b(&self) -> u8 {
        self.x2_a_b
    }
    #[inline(always)]
    pub fn x2_b(&self) -> f32 {
        self.x2_b
    }
}

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
    #[inline(always)]
    pub fn x1(&self) -> f32 {
        self.x1
    }
    #[inline(always)]
    pub fn x2(&self) -> Option<f32> {
        self.x2
    }
    #[inline(always)]
    pub fn x3(&self) -> f32 {
        self.x3
    }
    #[inline(always)]
    pub fn x2_a(&self) -> f32 {
        self.x2_a
    }
    #[inline(always)]
    pub fn x2_b(&self) -> f32 {
        self.x2_b
    }
    #[inline(always)]
    pub fn x3_a(&self) -> Option<f32> {
        self.x3_a
    }
    #[inline(always)]
    pub fn x3_b(&self) -> Option<f32> {
        self.x3_b
    }
    #[inline(always)]
    pub fn specie_specifici(&self) -> &HashMap<IdSpecieNISECI, ValoriIntermediSpecieNISECI> {
        &self.specie_specifici
    }
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
    pub fn get_valore(&self) -> Option<f32> {
        self.valore
    }
    pub fn get_rqe(&self) -> Option<f32> {
        self.rqe
    }
    pub fn get_x1(&self) -> f32 {
        self.valori_intermedi.x1()
    }
    pub fn get_x2(&self) -> Option<f32> {
        self.valori_intermedi.x2()
    }
    pub fn get_x3(&self) -> f32 {
        self.valori_intermedi.x3()
    }
    pub fn get_x3_a(&self) -> Option<f32> {
        self.valori_intermedi.x3_a()
    }
    pub fn get_x3_b(&self) -> Option<f32> {
        self.valori_intermedi.x3_b()
    }

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

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ClassiEtaSpecieNISECI {
    id: IdSpecieNISECI,
    tipo_autoctono: u8,
    tipo_alloctono: u8,
    specie_attesa: bool,
    ad_juv_soglia_1: f32,
    ad_juv_soglia_2: f32,
    ad_juv_soglia_3: f32,
    ad_juv_soglia_4: f32,
    cl1: u32,
    cl2: u32,
    cl3: u32,
    cl4: u32,
    cl5: u32,
}

impl fmt::Display for ClassiEtaSpecieNISECI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let specie_attesa_str = match self.specie_attesa() {
            true => "SI".to_string(),
            false => "NO".to_string(),
        };
        let string_representation = format!(
            "{}; {}; {}; {}; {}; {}; {}; {}; {}",
            self.id(),
            self.tipo_autoctono(),
            self.tipo_alloctono(),
            specie_attesa_str,
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
            specie_attesa: false,
            tipo_autoctono: 0,
            tipo_alloctono: 0,
            ad_juv_soglia_1: 0.0,
            ad_juv_soglia_2: 0.0,
            ad_juv_soglia_3: 0.0,
            ad_juv_soglia_4: 0.0,
            cl1: 0,
            cl2: 0,
            cl3: 0,
            cl4: 0,
            cl5: 0,
        }
    }
    pub fn new_cl_prevalorizzata(
        record: &RecordNISECI,
        riferimento: &RiferimentoNISECI,
    ) -> Option<ClassiEtaSpecieNISECI> {
        let id = riferimento.get_inner_id(record.id())?;
        let mut classe = ClassiEtaSpecieNISECI {
            id,
            specie_attesa: record.specie_attesa(),
            tipo_autoctono: record.tipo_autoctono(),
            tipo_alloctono: record.tipo_alloctono(),
            ad_juv_soglia_1: record.ad_juv_soglia_1(),
            ad_juv_soglia_2: record.ad_juv_soglia_2(),
            ad_juv_soglia_3: record.ad_juv_soglia_3(),
            ad_juv_soglia_4: record.ad_juv_soglia_4(),
            cl1: 0,
            cl2: 0,
            cl3: 0,
            cl4: 0,
            cl5: 0,
        };
        classe.update_classi_eta(record);
        Some(classe)
    }

    pub fn update_classi_eta(&mut self, record: &RecordNISECI) {
        match ClassiEta::find_classe_eta(record) {
            ClassiEta::CL1 => self.cl1 += 1,
            ClassiEta::CL2 => self.cl2 += 1,
            ClassiEta::CL3 => self.cl3 += 1,
            ClassiEta::CL4 => self.cl4 += 1,
            ClassiEta::CL5 => self.cl5 += 1,
        }
    }

    #[inline(always)]
    pub fn cl_1(&self) -> u32 {
        self.cl1
    }
    #[inline(always)]
    pub fn cl_2(&self) -> u32 {
        self.cl2
    }
    #[inline(always)]
    pub fn cl_3(&self) -> u32 {
        self.cl3
    }
    #[inline(always)]
    pub fn cl_4(&self) -> u32 {
        self.cl4
    }
    #[inline(always)]
    pub fn cl_5(&self) -> u32 {
        self.cl5
    }

    #[inline(always)]
    pub fn id(&self) -> IdSpecieNISECI {
        self.id
    }

    #[inline(always)]
    pub fn specie_attesa(&self) -> bool {
        self.specie_attesa
    }

    #[inline(always)]
    pub(crate) fn tipo_autoctono(&self) -> u8 {
        self.tipo_autoctono
    }

    #[inline(always)]
    pub(crate) fn tipo_alloctono(&self) -> u8 {
        self.tipo_alloctono
    }

    #[inline(always)]
    pub fn ad_juv_soglia_1(&self) -> f32 {
        self.ad_juv_soglia_1
    }

    #[inline(always)]
    pub fn ad_juv_soglia_2(&self) -> f32 {
        self.ad_juv_soglia_2
    }

    #[inline(always)]
    pub fn ad_juv_soglia_3(&self) -> f32 {
        self.ad_juv_soglia_3
    }

    #[inline(always)]
    pub fn ad_juv_soglia_4(&self) -> f32 {
        self.ad_juv_soglia_4
    }

    fn get_how_many_classes(&self) -> usize {
        [self.cl1, self.cl2, self.cl3, self.cl4, self.cl5]
            .into_iter()
            .filter(|&value| value > 0)
            .count()
    }

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

    pub fn get_x2_a_criterio_b(&self) -> (u8, Option<f32>) {
        if (self.cl2 + self.cl3) == 0 {
            return (3, None);
        }

        let ad_juv = (self.cl4 + self.cl5) as f32 / (self.cl2 + self.cl3) as f32;
        if ad_juv < self.ad_juv_soglia_1() {
            return (3, Some(ad_juv));
        }
        if ad_juv <= self.ad_juv_soglia_2() {
            return (2, Some(ad_juv));
        }
        if ad_juv <= self.ad_juv_soglia_3() {
            return (1, Some(ad_juv));
        }
        if ad_juv <= self.ad_juv_soglia_4() {
            return (2, Some(ad_juv));
        }
        (3, Some(ad_juv))
    }

    /// questa fn viene usata sia per x2_a che per x3
    /// i suoi test sono esattamente quelli per calculate_x2_a
    pub fn calculate_struttura_popolazione(&self) -> Result<(f32, MetricheX2A), String> {
        let criterio_a: u8 = self.get_x2_a_criterio_a();
        let (criterio_b, ad_juv): (u8, Option<f32>) = self.get_x2_a_criterio_b();

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

    #[inline(always)]
    pub fn popolazione_piu_strutt(&self) -> f32 {
        self.popolazione_piu_strutt
    }

    #[inline(always)]
    pub fn species_strutt(&self) -> u32 {
        self.species_strutt
    }

    #[inline(always)]
    pub fn species_mediamente_strutt(&self) -> u32 {
        self.species_mediamente_strutt
    }

    #[inline(always)]
    pub fn species_destrutt(&self) -> u32 {
        self.species_destrutt
    }

    #[inline(always)]
    pub fn tot_species(&self) -> usize {
        self.tot_species
    }

    #[inline(always)]
    pub fn intermediates_map(&self) -> &HashMap<IdSpecieNISECI, InfoIntermediePopolazioniNISECI> {
        &self.intermediates_map
    }

    pub fn get_info_pop(
        map: &HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI>,
    ) -> Result<InfoPopolazioniNISECI, Vec<String>> {
        let mut errors: Vec<String> = Vec::with_capacity(map.len()); // prenoto ora e poi restringo dopo

        let mut info_pop = InfoPopolazioniNISECI::new();
        info_pop.tot_species = map.len();
        let epsilon: f32 = 1e-6;
        for classe in map.values() {
            match classe.calculate_struttura_popolazione() {
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

/// struct che aiuta nel calcolo di x3
/// una volta valorizzata avremmo tutte le informazioni utili
/// a calcolare x3
/// per ogni tipo di alloctono abbiamo una InfoPopolazioniNISECI
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

    #[inline(always)]
    pub fn tipo_1(&self) -> &InfoPopolazioniNISECI {
        &self.tipo_1
    }
    #[inline(always)]
    pub fn tipo_2(&self) -> &InfoPopolazioniNISECI {
        &self.tipo_2
    }
    #[inline(always)]
    pub fn tipo_3(&self) -> &InfoPopolazioniNISECI {
        &self.tipo_3
    }

    #[inline(always)]
    pub fn tot_specie_aliene(&self) -> usize {
        self.tot_specie_aliene
    }

    #[inline(always)]
    pub fn tot_specie_autoctone(&self) -> usize {
        self.tot_specie_autoctone
    }

    pub fn get_info_pop_aliene(
        classi_eta: &ClassiEtaAlieniNISECI,
    ) -> Result<InfoPopolazioniAlieneNISECI, Vec<String>> {
        let tipo_1 = InfoPopolazioniNISECI::get_info_pop(&classi_eta.map_tipo_1)?;
        let tipo_2 = InfoPopolazioniNISECI::get_info_pop(&classi_eta.map_tipo_2)?;
        let tipo_3 = InfoPopolazioniNISECI::get_info_pop(&classi_eta.map_tipo_3)?;

        let info_pop_aliene = InfoPopolazioniAlieneNISECI {
            tipo_1,
            tipo_2,
            tipo_3,
            tot_specie_aliene: classi_eta.tot_specie_aliene,
            tot_specie_autoctone: classi_eta.tot_specie_autoctone,
        };

        Ok(info_pop_aliene)
    }

    pub fn get_species_mediamente_strutt(&self) -> u32 {
        self.tipo_1.species_mediamente_strutt()
            + self.tipo_2.species_mediamente_strutt()
            + self.tipo_3.species_mediamente_strutt()
    }
    pub fn get_species_destrutt(&self) -> u32 {
        self.tipo_1.species_destrutt()
            + self.tipo_2.species_destrutt()
            + self.tipo_3.species_destrutt()
    }
}

/// struct che aiuta nel calcolo x3
/// ci aiuta a suddividere le specie aliene in base alla tipologia
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
    pub fn new() -> ClassiEtaAlieniNISECI {
        ClassiEtaAlieniNISECI {
            map_tipo_1: HashMap::with_capacity(100),
            map_tipo_2: HashMap::with_capacity(100),
            map_tipo_3: HashMap::with_capacity(100),
            tot_specie_aliene: 0,
            tot_specie_autoctone: 0,
        }
    }
    #[inline(always)]
    pub fn map_tipo_1(&self) -> &HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI> {
        &self.map_tipo_1
    }
    #[inline(always)]
    pub fn map_tipo_2(&self) -> &HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI> {
        &self.map_tipo_1
    }
    #[inline(always)]
    pub fn map_tipo_3(&self) -> &HashMap<IdSpecieNISECI, ClassiEtaSpecieNISECI> {
        &self.map_tipo_1
    }
    #[inline(always)]
    pub fn tot_specie_aliene(&self) -> usize {
        self.tot_specie_aliene
    }
    #[inline(always)]
    pub fn set_tot_specie_aliene(&mut self, val: usize) {
        self.tot_specie_aliene = val;
    }
    #[inline(always)]
    pub fn tot_specie_autoctone(&self) -> usize {
        self.tot_specie_autoctone
    }
    #[inline(always)]
    pub fn set_tot_specie_autoctone(&mut self, val: usize) {
        self.tot_specie_autoctone = val;
    }
}

/// struct che quando valorizzata
/// esprime, data una SpecieNISECI,
/// il numero di esemplari trovati
/// suddivisi in base al numero di passaggio
pub struct EsemplariPerCattura {
    id: IdSpecieNISECI,
    mappa: HashMap<u8, u32>, // la key è il numero del passaggio
    dens_soglia_1: f32,
    dens_soglia_2: f32,
}

impl EsemplariPerCattura {
    pub fn id(&self) -> IdSpecieNISECI {
        self.id
    }
    pub fn mappa(&self) -> &HashMap<u8, u32> {
        &self.mappa
    }
    pub fn dens_soglia_1(&self) -> f32 {
        self.dens_soglia_1
    }
    #[inline(always)]
    pub fn dens_soglia_2(&self) -> f32 {
        self.dens_soglia_2
    }

    pub fn new(id: IdSpecieNISECI, dens_soglia_1: f32, dens_soglia_2: f32) -> Self {
        EsemplariPerCattura {
            id,
            mappa: HashMap::new(),
            dens_soglia_1,
            dens_soglia_2,
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

#[cfg(test)]
mod domain_niseci_private_tests {
    use super::*;
    use crate::domain::niseci::SpecieNISECI;

    #[cfg(test)]
    impl ClassiEtaSpecieNISECI {
        #[cfg(test)]
        pub(crate) fn new_custom(
            specie: &SpecieNISECI,
            id: IdSpecieNISECI,
            cl1: u32,
            cl2: u32,
            cl3: u32,
            cl4: u32,
            cl5: u32,
        ) -> Self {
            Self {
                id,
                tipo_autoctono: specie.tipo_autoctono(),
                tipo_alloctono: specie.tipo_alloctono(),
                specie_attesa: specie.specie_attesa(),
                ad_juv_soglia_1: specie.ad_juv_soglia_1(),
                ad_juv_soglia_2: specie.ad_juv_soglia_2(),
                ad_juv_soglia_3: specie.ad_juv_soglia_3(),
                ad_juv_soglia_4: specie.ad_juv_soglia_4(),
                cl1,
                cl2,
                cl3,
                cl4,
                cl5,
            }
        }
    }
}
