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

use crate::domain::niseci::{CampionamentoNISECI, IdSpecieNISECI, RiferimentoNISECI};
use std::collections::HashSet;

pub fn calculate_x1(campionamento: &CampionamentoNISECI, riferimento: &RiferimentoNISECI) -> f32 {
    // n_i è il numero di specie autoctone di maggiore importanza ecologico-funzionale campionate
    // n_a è il numero di altre specie autoctone campionate
    // m_i è il numero di specie autoctone di maggiore importanza ecologico-funzionale attese
    // m_a è il numero di altre specie autoctone attese

    // creo un set delle specie campionate
    let mut set_specie_campionate: HashSet<IdSpecieNISECI> = HashSet::new();
    for camp in campionamento {
        set_specie_campionate.insert(camp.id());
    }

    let mut n_i: f32 = 0.0;
    let mut n_a: f32 = 0.0;
    for s_id in set_specie_campionate {
        debug_assert!(riferimento.contains_plain_id(s_id));
        let specie = riferimento
            .get_ref_by_plain_id(s_id)
            .expect("Riferimento should contain this id");
        if specie.specie_attesa() {
            if specie.tipo_autoctono() == 1 {
                // tipo_autoctono == 1 allora specie importante
                n_i += 1.0;
            } else if specie.tipo_autoctono() == 2 {
                n_a += 1.0;
            }
        }
    }

    // ora trovo le specie attese dal riferimento
    // per evitare doppioni, anche se non dovrebbero esserci,
    // ricavo il set delle specie attese
    let mut m_i: f32 = 0.0;
    let mut m_a: f32 = 0.0;
    for specie in riferimento {
        if specie.specie_attesa() {
            if specie.tipo_autoctono() == 1 {
                // tipo_autoctono == 1 allora specie importante
                m_i += 1.0;
            } else if specie.tipo_autoctono() == 2 {
                m_a += 1.0;
            }
        }
    }

    // this is the formula use in the NISECI docs
    let x1 = (1.2 * n_i + 0.8 * n_a) / (1.2 * m_i + 0.8 * m_a);

    (1000.0 * x1).round() / 1000.0
}
