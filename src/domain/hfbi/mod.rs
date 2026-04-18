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
use std::sync::LazyLock;

use super::localize::CommaFormat;
use super::posf32::{deserialize_positive_f32, PositiveF32};
use crate::domain::location::Location;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum GruppoEcoHFBI {
    MigratoriMarini,
    Diadromi,
    ResidentiDiEstuario,
    #[allow(dead_code)]
    OccasionaliMarini,
    #[allow(dead_code)]
    OccasionaliDiAcqueDolci,
}

impl fmt::Display for GruppoEcoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            GruppoEcoHFBI::MigratoriMarini => "MigratoriMarini",
            GruppoEcoHFBI::Diadromi => "Diadromi",
            GruppoEcoHFBI::ResidentiDiEstuario => "ResidentiDiEstuario",
            GruppoEcoHFBI::OccasionaliMarini => "OccasionaliMarini",
            GruppoEcoHFBI::OccasionaliDiAcqueDolci => "OccasionaliDiAcqueDolci",
        };
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GruppoTrofHFBI {
    pub microbentivori: f32,
    pub macrobentivori: f32,
    pub iperbentivori: f32,
    pub erbivori: f32,
    pub detritivori: f32,
    pub planctivori: f32,
    pub onnivori: f32,
}

impl fmt::Display for GruppoTrofHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "{}, {}, {}, {}, {}, {}, {}",
            self.microbentivori,
            self.macrobentivori,
            self.iperbentivori,
            self.erbivori,
            self.detritivori,
            self.planctivori,
            self.onnivori
        );
        write!(f, "{}", string_representation)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SpecieHFBI {
    pub nome_comune: String,
    pub codice_specie: String,
    pub autoctono: bool,
    pub gruppo_eco: GruppoEcoHFBI,
    pub gruppo_trofico: GruppoTrofHFBI,
}

impl fmt::Display for SpecieHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let autoctono_str = match self.autoctono {
            true => "SI".to_string(),
            false => "NO".to_string(),
        };
        let string_representation = format!(
            "{}, {}, {}, {}, {}",
            self.nome_comune,
            self.codice_specie,
            autoctono_str,
            self.gruppo_eco,
            self.gruppo_trofico
        );
        write!(f, "{}", string_representation)
    }
}

pub static RIFERIMENTO_HFBI: LazyLock<Vec<SpecieHFBI>> = LazyLock::new(|| {
    vec![
        SpecieHFBI {
            nome_comune: "Cheppia".to_string(),
            codice_specie: "CH".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::Diadromi,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.0,
                macrobentivori: 0.0,
                iperbentivori: 1.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Anguilla".to_string(),
            codice_specie: "AN".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::Diadromi,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.2,
                macrobentivori: 0.4,
                iperbentivori: 0.4,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Nono".to_string(),
            codice_specie: "NO".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.5,
                macrobentivori: 0.0,
                iperbentivori: 0.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.5,
            },
        },
        SpecieHFBI {
            nome_comune: "Latterino di lago".to_string(),
            codice_specie: "LAT".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.0,
                macrobentivori: 0.0,
                iperbentivori: 1.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Aguglia".to_string(),
            codice_specie: "BBE".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.0,
                macrobentivori: 0.0,
                iperbentivori: 1.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Gallinella".to_string(),
            codice_specie: "CLU".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.4,
                macrobentivori: 0.4,
                iperbentivori: 0.2,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Muggine labbrone".to_string(),
            codice_specie: "CEL".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.0,
                macrobentivori: 0.0,
                iperbentivori: 0.5,
                erbivori: 0.0,
                detritivori: 0.5,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Spigola branzino".to_string(),
            codice_specie: "DIC".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.0,
                macrobentivori: 0.0,
                iperbentivori: 1.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Alice (Acciuga Europea)".to_string(),
            codice_specie: "DIC".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.0,
                macrobentivori: 0.0,
                iperbentivori: 0.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 1.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Ghiozzo nero".to_string(),
            codice_specie: "GHN".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.4,
                macrobentivori: 0.4,
                iperbentivori: 0.2,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Cavalluccio marino".to_string(),
            codice_specie: "HGU".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.5,
                macrobentivori: 0.0,
                iperbentivori: 0.5,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Cavalluccio camuso".to_string(),
            codice_specie: "HHI".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.5,
                macrobentivori: 0.0,
                iperbentivori: 0.5,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Ghiozzetto di laguna".to_string(),
            codice_specie: "GHL".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 2.0 / 3.0,
                macrobentivori: 0.0,
                iperbentivori: 1.0 / 3.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Muggine dorato".to_string(),
            codice_specie: "CED".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.0,
                macrobentivori: 0.0,
                iperbentivori: 0.5,
                erbivori: 0.0,
                detritivori: 0.5,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Muggine calamita".to_string(),
            codice_specie: "CEC".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::Diadromi,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.0,
                macrobentivori: 0.0,
                iperbentivori: 0.5,
                erbivori: 0.0,
                detritivori: 0.5,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Muggine musino".to_string(),
            codice_specie: "MUS".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.0,
                macrobentivori: 0.0,
                iperbentivori: 0.5,
                erbivori: 0.0,
                detritivori: 0.5,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Cefalo".to_string(),
            codice_specie: "MUG".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::Diadromi,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.0,
                macrobentivori: 0.0,
                iperbentivori: 0.5,
                erbivori: 0.0,
                detritivori: 0.5,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Triglia di scoglio".to_string(),
            codice_specie: "MSU".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 2.0 / 3.0,
                macrobentivori: 1.0 / 3.0,
                iperbentivori: 0.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Pesce ago sottile".to_string(),
            codice_specie: "NOP".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 1.0,
                macrobentivori: 0.0,
                iperbentivori: 0.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Passera pianuzza".to_string(),
            codice_specie: "PFL".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.4,
                macrobentivori: 0.4,
                iperbentivori: 0.2,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Ghiozzetto cenerino".to_string(),
            codice_specie: "GHC".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 2.0 / 3.0,
                macrobentivori: 0.0,
                iperbentivori: 1.0 / 3.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Ghiozzetto marmorizzato".to_string(),
            codice_specie: "GHM".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 2.0 / 3.0,
                macrobentivori: 0.0,
                iperbentivori: 1.0 / 3.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Ghiozzetto minuto".to_string(),
            codice_specie: "GHE".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 2.0 / 3.0,
                macrobentivori: 0.0,
                iperbentivori: 1.0 / 3.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Bavosa pavone".to_string(),
            codice_specie: "BAP".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.5,
                macrobentivori: 0.0,
                iperbentivori: 0.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.5,
            },
        },
        SpecieHFBI {
            nome_comune: "Sardina".to_string(),
            codice_specie: "SPI".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.0,
                macrobentivori: 0.0,
                iperbentivori: 0.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 1.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Sogliola comune".to_string(),
            codice_specie: "SSO".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 2.0 / 3.0,
                macrobentivori: 1.0 / 3.0,
                iperbentivori: 0.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Orata".to_string(),
            codice_specie: "SAU".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::MigratoriMarini,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.4,
                macrobentivori: 0.2,
                iperbentivori: 0.4,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Pesce ago di rio".to_string(),
            codice_specie: "PAR".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 2.0 / 3.0,
                macrobentivori: 0.0,
                iperbentivori: 1.0 / 3.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Pesce ago adriatico".to_string(),
            codice_specie: "STA".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.0,
                macrobentivori: 0.0,
                iperbentivori: 1.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Pesce ago cavallino".to_string(),
            codice_specie: "STY".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 0.2,
                macrobentivori: 0.0,
                iperbentivori: 0.8,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
        SpecieHFBI {
            nome_comune: "Ghiozzo gò".to_string(),
            codice_specie: "GHG".to_string(),
            autoctono: true,
            gruppo_eco: GruppoEcoHFBI::ResidentiDiEstuario,
            gruppo_trofico: GruppoTrofHFBI {
                microbentivori: 1.0 / 3.0,
                macrobentivori: 1.0 / 3.0,
                iperbentivori: 1.0 / 3.0,
                erbivori: 0.0,
                detritivori: 0.0,
                planctivori: 0.0,
                onnivori: 0.0,
            },
        },
    ]
});

use std::{collections::HashMap, fmt};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecordHFBI {
    pub specie: SpecieHFBI,
    pub numero_individui: u32, // in millimetri
    pub peso: f32,             // in grammi
}

impl fmt::Display for RecordHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!(
            "RecordHFBI: {{ specie: {{{}}}, numero_individui: {{{}}}, peso: {{{}}}",
            self.specie, self.numero_individui, self.peso
        );
        write!(f, "{}", string_representation)
    }
}

/// Since calculations of HFBI rely on iteration order, this type must enforce
/// the internal invariant that records are sorted by descending `peso`.
/// v0.2 will change visibility of internals to avoid improper construction / mutation.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CampionamentoHFBI {
    #[deprecated(
        note = "v0.2 will change visibility.\nConsider using self.into() for owned conversion, &self for borrowed iteration, CampionamentoHFBI::new() to construct"
    )]
    pub campionamento: Vec<RecordHFBI>,
}

impl fmt::Display for CampionamentoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_representation = "CampionaHFBI: {".to_string();
        for r in self {
            string_representation = format!("{string_representation}\n  {{{r}}},");
        }
        string_representation = format!("{string_representation}\n}}");
        write!(f, "{}", string_representation)
    }
}

impl CampionamentoHFBI {
    pub fn new(campionamento: Vec<RecordHFBI>) -> Self {
        #[allow(deprecated)]
        let mut sorted = Self { campionamento };
        #[allow(deprecated)]
        sorted
            .campionamento
            .sort_by(|a, b| b.peso.total_cmp(&a.peso));
        sorted
    }
    pub fn as_vec(&self) -> Vec<&RecordHFBI> {
        #[allow(deprecated)]
        let mut v: Vec<&RecordHFBI> = self.campionamento.iter().collect();

        v.sort_by(|a, b| b.peso.total_cmp(&a.peso));

        v
    }
    pub fn sort_by_peso_desc(&mut self) {
        #[allow(deprecated)]
        self.campionamento.sort_by(|a, b| b.peso.total_cmp(&a.peso));
    }
    pub fn sorted_by_peso_desc(&self) -> impl Iterator<Item = &RecordHFBI> {
        #[allow(deprecated)]
        let mut v: Vec<&RecordHFBI> = self.campionamento.iter().collect();

        v.sort_by(|a, b| b.peso.total_cmp(&a.peso));

        v.into_iter()
    }
}

impl From<CampionamentoHFBI> for Vec<RecordHFBI> {
    fn from(val: CampionamentoHFBI) -> Self {
        #[allow(deprecated)]
        let mut v = val.campionamento;

        v.sort_by(|a, b| b.peso.total_cmp(&a.peso));

        v
    }
}

impl<'a> IntoIterator for &'a CampionamentoHFBI {
    type Item = &'a RecordHFBI;
    type IntoIter = std::vec::IntoIter<&'a RecordHFBI>;

    fn into_iter(self) -> Self::IntoIter {
        #[allow(deprecated)]
        let mut v: Vec<&RecordHFBI> = self.campionamento.iter().collect();

        v.sort_by(|a, b| b.peso.total_cmp(&a.peso));

        v.into_iter()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum TipoLagunaCostieraHFBI {
    MAt1,
    MAt2,
    MAt3,
}

impl fmt::Display for TipoLagunaCostieraHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            TipoLagunaCostieraHFBI::MAt1 => "M-AT-1",
            TipoLagunaCostieraHFBI::MAt2 => "M-AT-2",
            TipoLagunaCostieraHFBI::MAt3 => "M-AT-3",
        };
        write!(f, "{}", string_representation)
    }
}

impl TryFrom<i32> for TipoLagunaCostieraHFBI {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == TipoLagunaCostieraHFBI::MAt1 as i32 => Ok(TipoLagunaCostieraHFBI::MAt1),
            x if x == TipoLagunaCostieraHFBI::MAt2 as i32 => Ok(TipoLagunaCostieraHFBI::MAt2),
            x if x == TipoLagunaCostieraHFBI::MAt3 as i32 => Ok(TipoLagunaCostieraHFBI::MAt3),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum StagioneHFBI {
    Primavera,
    Autunno,
}

impl fmt::Display for StagioneHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            StagioneHFBI::Primavera => "Primavera",
            StagioneHFBI::Autunno => "Autunno",
        };
        write!(f, "{}", string_representation)
    }
}

impl TryFrom<i32> for StagioneHFBI {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == StagioneHFBI::Primavera as i32 => Ok(StagioneHFBI::Primavera),
            x if x == StagioneHFBI::Autunno as i32 => Ok(StagioneHFBI::Autunno),
            _ => Err(()),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum HabitatHFBI {
    Vegetato,
    NonVegetato,
}

impl fmt::Display for HabitatHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            HabitatHFBI::Vegetato => "Vegetato",
            HabitatHFBI::NonVegetato => "Non Vegetato",
        };
        write!(f, "{}", string_representation)
    }
}

impl TryFrom<i32> for HabitatHFBI {
    type Error = ();

    fn try_from(v: i32) -> Result<Self, Self::Error> {
        match v {
            x if x == HabitatHFBI::Vegetato as i32 => Ok(HabitatHFBI::Vegetato),
            x if x == HabitatHFBI::NonVegetato as i32 => Ok(HabitatHFBI::NonVegetato),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
pub struct AnagraficaHFBIDraft {
    pub codice_stazione: String,
    pub corpo_idrico: String,
    pub posizione: Location,
    pub date_string: String, // Formato gg/mm/aaaa
    pub tipo_laguna: TipoLagunaCostieraHFBI,
    pub stagione: StagioneHFBI,
    pub habitat_vegetato: HabitatHFBI,
    pub lunghezza_media_transetto: String,
    pub larghezza_media_transetto: String,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AnagraficaHFBI {
    pub codice_stazione: String,
    pub corpo_idrico: String,
    pub posizione: Location,
    pub date_string: String, // Formato gg/mm/aaaa
    pub tipo_laguna: TipoLagunaCostieraHFBI,
    pub stagione: StagioneHFBI,
    pub habitat_vegetato: HabitatHFBI,
    #[deprecated(
        note = "v0.2 will change visibility.\nConsider using self.get_lunghezza_media(), self.set_lunghezza_media(), AnagraficaHFBI::new() to construct"
    )]
    #[serde(deserialize_with = "deserialize_positive_f32")]
    pub lunghezza_media_transetto: f32,
    #[deprecated(
        note = "v0.2 will change visibility.\nConsider using self.get_larghezza_media(), self.set_larghezza_media(), AnagraficaHFBI::new() to construct"
    )]
    #[serde(deserialize_with = "deserialize_positive_f32")]
    pub larghezza_media_transetto: f32,
}

impl AnagraficaHFBI {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        codice_stazione: String,
        corpo_idrico: String,
        posizione: Location,
        date_string: String,
        tipo_laguna: TipoLagunaCostieraHFBI,
        stagione: StagioneHFBI,
        habitat_vegetato: HabitatHFBI,
        lunghezza_media_transetto: PositiveF32,
        larghezza_media_transetto: PositiveF32,
    ) -> Self {
        Self {
            codice_stazione,
            corpo_idrico,
            posizione,
            date_string,
            tipo_laguna,
            stagione,
            habitat_vegetato,
            #[allow(deprecated)]
            lunghezza_media_transetto: *lunghezza_media_transetto,
            #[allow(deprecated)]
            larghezza_media_transetto: *larghezza_media_transetto,
        }
    }
    pub fn get_lunghezza_media(&self) -> f32 {
        #[allow(deprecated)]
        self.lunghezza_media_transetto
    }
    pub fn set_lunghezza_media(&mut self, val: f32) -> Result<(), ()> {
        if !val.is_finite() {
            return Err(());
        }
        if val <= 0.0 {
            return Err(());
        }
        #[allow(deprecated)]
        {
            self.lunghezza_media_transetto = val;
        }
        Ok(())
    }
    pub fn get_larghezza_media(&self) -> f32 {
        #[allow(deprecated)]
        self.larghezza_media_transetto
    }

    pub fn set_larghezza_media(&mut self, val: f32) -> Result<(), ()> {
        if !val.is_finite() {
            return Err(());
        }
        if val <= 0.0 {
            return Err(());
        }
        #[allow(deprecated)]
        {
            self.larghezza_media_transetto = val;
        }
        Ok(())
    }

    pub fn get_cond_riferimento_key(&self) -> CondizioniRiferimentoKeyHFBI {
        CondizioniRiferimentoKeyHFBI {
            tipo_laguna: self.tipo_laguna.clone(),
            stagione: self.stagione.clone(),
            habitat_vegetato: self.habitat_vegetato.clone(),
        }
    }
}

impl fmt::Display for AnagraficaHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = format!("AnagraficaHFBI: {{ codice_stazione {{{}}}, corpo_idrico: {{{}}}, posizione: {{{}}}, data: {{{}}}, tipo_laguna: {{{}}}, stagione: {{{}}}, habitat: {{{}}}, lunghezza_transetto: {{{}}}, larghezza_transetto: {{{}}} }}",
        self.codice_stazione, self.corpo_idrico, self.posizione, self.date_string, self.tipo_laguna, self.stagione, self.habitat_vegetato, self.get_lunghezza_media(), self.get_larghezza_media());
        write!(f, "{}", string_representation)
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValoriIntermediHFBI {
    pub bbent: f32,
    pub bn: f32,
    pub dbent: f32,
    pub ddom: f32,
    pub dhzp: f32,
    pub dmig: f32,
    pub mmi: f32,
}

impl fmt::Display for ValoriIntermediHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut string_representation = format!(
            "bbent: {}, bn: {}, dbent: {}, ddom: {}, dhzp: {}, dmig: {}, mmi: {}",
            self.bbent, self.bn, self.dbent, self.ddom, self.dhzp, self.dmig, self.mmi
        );

        string_representation = string_representation.to_string(); //FIXME: Why is this here?
        write!(f, "{}", string_representation)
    }
}

impl ValoriIntermediHFBI {
    pub fn log(&self) {
        //TODO: a proper format? we count on the embedded newlines to leverage the
        //chopping on newlines from add_console_message()
        println!("Valori intermedi: {{{self}}}");
    }

    pub fn to_csv(&self, comma_csv_delimiter: bool) -> String {
        let string_representation = if comma_csv_delimiter {
            format!(
                "bbent, bn, dbent, ddom, dhzp, dmig\n{}, {}, {}, {}, {}, {}",
                self.bbent, self.bn, self.dbent, self.ddom, self.dhzp, self.dmig
            )
        } else {
            format!(
                "bbent; bn; dbent; ddom; dhzp; dmig\n{}; {}; {}; {}; {}; {}",
                self.bbent.comma(),
                self.bn.comma(),
                self.dbent.comma(),
                self.ddom.comma(),
                self.dhzp.comma(),
                self.dmig.comma()
            )
        };
        string_representation
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RisultatoHFBI {
    valore: Option<f32>,
    intermediates: ValoriIntermediHFBI,
}

impl fmt::Display for RisultatoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let valore_str = match self.valore {
            Some(v) => format!("{v}"),
            None => "NC".to_string(),
        };
        let string_representation = format!(
            "RisultatoHFBI: {{ valore HFBI: {{{}}}, intermediates: {{{}}} }}",
            valore_str, self.intermediates
        );
        write!(f, "{}", string_representation)
    }
}

impl RisultatoHFBI {
    pub fn new(valore: Option<f32>, intermediates: ValoriIntermediHFBI) -> Self {
        Self {
            valore,
            intermediates,
        }
    }
    pub fn get_valore(&self) -> Option<f32> {
        self.valore
    }
    pub fn get_intermediates(&self) -> ValoriIntermediHFBI {
        self.intermediates.clone()
    }
    pub fn to_csv(&self, anagrafica: &AnagraficaHFBI, comma_csv_delimiter: bool) -> String {
        let hfbi_opt = self.get_valore();
        if hfbi_opt.is_none() {
            return "NC".to_string();
        }
        let hfbi = hfbi_opt.unwrap();
        let intermediates = self.get_intermediates();
        let stato_ecologico = Some(StatoEcologicoHFBI::from(hfbi));
        let stato_ecologico_str = match stato_ecologico {
            Some(val) => {
                format!("{val}")
            }
            None => "NC".to_string(),
        };
        let string_representation = if comma_csv_delimiter {
            format!("Codice stazione, stagione, habitat vegetato, tipo laguna, MMI, HFBI, Stato ecologico\n{}, {}, {}, {}, {}, {}, {}",
                    anagrafica.codice_stazione,
                    anagrafica.stagione,
                    anagrafica.habitat_vegetato,
                    anagrafica.tipo_laguna,
                    intermediates.mmi,
                    hfbi,
                    stato_ecologico_str
            )
        } else {
            format!("Codice stazione; stagione; habitat vegetato; tipo laguna; MMI; HFBI; Stato ecologico\n{}; {}; {}; {}; {}; {}; {}",
                    anagrafica.codice_stazione,
                    anagrafica.stagione,
                    anagrafica.habitat_vegetato,
                    anagrafica.tipo_laguna,
                    intermediates.mmi.comma(),
                    hfbi.comma(),
                    stato_ecologico_str
            )
        };
        string_representation
    }
}

#[derive(Hash, PartialEq, Eq)]
pub struct CondizioniRiferimentoKeyHFBI {
    pub tipo_laguna: TipoLagunaCostieraHFBI,
    pub stagione: StagioneHFBI,
    pub habitat_vegetato: HabitatHFBI,
}

#[derive(Clone)]
pub struct CondizioniRiferimentoHFBI {
    pub bn: f32,
    pub ddom: f32,
    pub dmig: f32,
    pub bbent: f32,
    pub dbent: f32,
    pub dhzp: f32,
}

impl CondizioniRiferimentoHFBI {
    pub fn get_cond_riferimento(
        anagrafica: &AnagraficaHFBI,
    ) -> Option<&'static CondizioniRiferimentoHFBI> {
        let key = anagrafica.get_cond_riferimento_key();
        CONDIZIONI_RIFERIMENTO_HFBI_HASHMAP.get(&key)
    }
}

static CONDIZIONI_RIFERIMENTO_HFBI_HASHMAP: LazyLock<
    HashMap<CondizioniRiferimentoKeyHFBI, CondizioniRiferimentoHFBI>,
> = LazyLock::new(|| {
    HashMap::from([
        // M-AT-1 data
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt1,
                stagione: StagioneHFBI::Primavera,
                habitat_vegetato: HabitatHFBI::NonVegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.232,
                ddom: 2.052,
                dmig: 3.212,
                bbent: 6.537,
                dbent: 3.768,
                dhzp: 2.856,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt1,
                stagione: StagioneHFBI::Autunno,
                habitat_vegetato: HabitatHFBI::NonVegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 1.932,
                ddom: 2.268,
                dmig: 2.014,
                bbent: 6.867,
                dbent: 2.944,
                dhzp: 2.570,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt1,
                stagione: StagioneHFBI::Primavera,
                habitat_vegetato: HabitatHFBI::Vegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.232,
                ddom: 1.784,
                dmig: 3.212,
                bbent: 7.242,
                dbent: 3.153,
                dhzp: 2.369,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt1,
                stagione: StagioneHFBI::Autunno,
                habitat_vegetato: HabitatHFBI::Vegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 1.932,
                ddom: 2.001,
                dmig: 2.014,
                bbent: 7.572,
                dbent: 2.329,
                dhzp: 2.083,
            },
        ),
        // M-AT-2 data
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt2,
                stagione: StagioneHFBI::Primavera,
                habitat_vegetato: HabitatHFBI::NonVegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.539,
                ddom: 2.052,
                dmig: 3.212,
                bbent: 5.221,
                dbent: 3.768,
                dhzp: 2.856,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt2,
                stagione: StagioneHFBI::Autunno,
                habitat_vegetato: HabitatHFBI::NonVegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.238,
                ddom: 2.268,
                dmig: 2.014,
                bbent: 5.551,
                dbent: 2.944,
                dhzp: 2.570,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt2,
                stagione: StagioneHFBI::Primavera,
                habitat_vegetato: HabitatHFBI::Vegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.539,
                ddom: 1.784,
                dmig: 3.212,
                bbent: 5.925,
                dbent: 3.153,
                dhzp: 2.369,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt2,
                stagione: StagioneHFBI::Autunno,
                habitat_vegetato: HabitatHFBI::Vegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.238,
                ddom: 2.001,
                dmig: 2.014,
                bbent: 6.255,
                dbent: 2.329,
                dhzp: 2.083,
            },
        ),
        // M-AT-3 data
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt3,
                stagione: StagioneHFBI::Primavera,
                habitat_vegetato: HabitatHFBI::NonVegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.217,
                ddom: 2.052,
                dmig: 3.212,
                bbent: 4.561,
                dbent: 3.768,
                dhzp: 2.856,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt3,
                stagione: StagioneHFBI::Autunno,
                habitat_vegetato: HabitatHFBI::NonVegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 1.917,
                ddom: 2.268,
                dmig: 2.014,
                bbent: 4.891,
                dbent: 2.944,
                dhzp: 2.570,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt3,
                stagione: StagioneHFBI::Primavera,
                habitat_vegetato: HabitatHFBI::Vegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 2.217,
                ddom: 1.784,
                dmig: 3.212,
                bbent: 5.265,
                dbent: 3.153,
                dhzp: 2.369,
            },
        ),
        (
            CondizioniRiferimentoKeyHFBI {
                tipo_laguna: TipoLagunaCostieraHFBI::MAt3,
                stagione: StagioneHFBI::Autunno,
                habitat_vegetato: HabitatHFBI::Vegetato,
            },
            CondizioniRiferimentoHFBI {
                bn: 1.917,
                ddom: 2.001,
                dmig: 2.014,
                bbent: 5.595,
                dbent: 2.329,
                dhzp: 2.083,
            },
        ),
    ])
});

/// enum per il risultato finale di un calcolo hfbi
/// (vedi calculate_stato_ecologico)
pub enum StatoEcologicoHFBI {
    Eccellente,
    Buono,
    Sufficiente,
    Scarso,
    Cattivo,
}

impl fmt::Display for StatoEcologicoHFBI {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string_representation = match *self {
            StatoEcologicoHFBI::Eccellente => "Eccellente",
            StatoEcologicoHFBI::Buono => "Buono",
            StatoEcologicoHFBI::Sufficiente => "Sufficiente",
            StatoEcologicoHFBI::Scarso => "Scarso",
            StatoEcologicoHFBI::Cattivo => "Cattivo",
        };
        write!(f, "{}", string_representation)
    }
}

const STATO_ECOLOGICO_HFBI_SOGLIA_ECCELLENTE: f32 = 0.94;
const STATO_ECOLOGICO_HFBI_SOGLIA_BUONO: f32 = 0.55;
const STATO_ECOLOGICO_HFBI_SOGLIA_SUFFICIENTE: f32 = 0.33;
const STATO_ECOLOGICO_HFBI_SOGLIA_SCARSO: f32 = 0.11;

impl From<f32> for StatoEcologicoHFBI {
    fn from(val: f32) -> Self {
        if val >= STATO_ECOLOGICO_HFBI_SOGLIA_ECCELLENTE {
            return StatoEcologicoHFBI::Eccellente;
        }
        if val >= STATO_ECOLOGICO_HFBI_SOGLIA_BUONO {
            return StatoEcologicoHFBI::Buono;
        }
        if val >= STATO_ECOLOGICO_HFBI_SOGLIA_SUFFICIENTE {
            return StatoEcologicoHFBI::Sufficiente;
        }
        if val >= STATO_ECOLOGICO_HFBI_SOGLIA_SCARSO {
            return StatoEcologicoHFBI::Scarso;
        }
        StatoEcologicoHFBI::Cattivo
    }
}

#[cfg(test)]
mod domain_hfbi_private_tests {
    use super::*;
    fn create_specie_record(
        codice_specie: &'static str,
        gruppo_eco: GruppoEcoHFBI,
        peso: f32,
    ) -> RecordHFBI {
        RecordHFBI {
            specie: SpecieHFBI {
                nome_comune: "Test Specie".to_string(),
                codice_specie: codice_specie.to_string(),
                autoctono: true,
                gruppo_eco,
                gruppo_trofico: GruppoTrofHFBI {
                    microbentivori: 0.0,
                    macrobentivori: 0.0,
                    iperbentivori: 0.0,
                    erbivori: 0.0,
                    detritivori: 0.0,
                    planctivori: 0.0,
                    onnivori: 0.0,
                },
            },
            numero_individui: 1,
            peso,
        }
    }
    #[cfg(test)]
    impl CampionamentoHFBI {
        /// Test helper to build unsorted instances
        #[cfg(test)]
        pub(crate) fn new_raw_unsorted(campionamento: Vec<RecordHFBI>) -> Self {
            #[allow(deprecated)]
            Self { campionamento }
        }
    }
    #[cfg(test)]
    impl AnagraficaHFBI {
        /// Test helper to build unchecked instances
        #[cfg(test)]
        pub(crate) fn new_raw_unchecked(
            codice_stazione: String,
            corpo_idrico: String,
            posizione: Location,
            date_string: String,
            tipo_laguna: TipoLagunaCostieraHFBI,
            stagione: StagioneHFBI,
            habitat_vegetato: HabitatHFBI,
            lunghezza_media_transetto: f32,
            larghezza_media_transetto: f32,
        ) -> Self {
            Self {
                codice_stazione,
                corpo_idrico,
                posizione,
                date_string,
                tipo_laguna,
                stagione,
                habitat_vegetato,
                #[allow(deprecated)]
                lunghezza_media_transetto,
                #[allow(deprecated)]
                larghezza_media_transetto,
            }
        }

        /// Test helper to mutate lunghezza with no checks
        #[cfg(test)]
        pub(crate) fn set_lunghezza_unchecked(&mut self, val: f32) {
            #[allow(deprecated)]
            {
                self.lunghezza_media_transetto = val;
            }
        }
        /// Test helper to mutate larghezza with no checks
        #[cfg(test)]
        pub(crate) fn set_larghezza_unchecked(&mut self, val: f32) {
            #[allow(deprecated)]
            {
                self.larghezza_media_transetto = val;
            }
        }
    }
    #[test]
    fn test_campionamentohfbi_new_order_invariant() {
        let campione = CampionamentoHFBI::new(vec![
            // Species 2: Migratory, contributes to most metrics
            create_specie_record("SP2", GruppoEcoHFBI::MigratoriMarini, 200.0),
            // Species 3: Resident, not dominant
            create_specie_record("SP3", GruppoEcoHFBI::ResidentiDiEstuario, 100.0),
            // Species 1: Migratory, dominant, contributes to all metrics
            create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 500.0),
        ]);

        #[allow(deprecated)]
        let actual: Vec<(&str, f32)> = campione
            .campionamento
            .iter()
            .map(|r| (r.specie.codice_specie.as_str(), r.peso))
            .collect();

        let expected = vec![("SP1", 500.0), ("SP2", 200.0), ("SP3", 100.0)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_campionamentohfbi_as_vec_order_invariant() {
        let campione = CampionamentoHFBI::new_raw_unsorted(vec![
            // Species 2: Migratory, contributes to most metrics
            create_specie_record("SP2", GruppoEcoHFBI::MigratoriMarini, 200.0),
            // Species 3: Resident, not dominant
            create_specie_record("SP3", GruppoEcoHFBI::ResidentiDiEstuario, 100.0),
            // Species 1: Migratory, dominant, contributes to all metrics
            create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 500.0),
        ]);

        let actual: Vec<(&str, f32)> = campione
            .as_vec()
            .iter()
            .map(|r| (r.specie.codice_specie.as_str(), r.peso))
            .collect();

        let expected = vec![("SP1", 500.0), ("SP2", 200.0), ("SP3", 100.0)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_campionamentohfbi_into_iter_order_invariant() {
        let campione = CampionamentoHFBI::new_raw_unsorted(vec![
            // Species 2: Migratory, contributes to most metrics
            create_specie_record("SP2", GruppoEcoHFBI::MigratoriMarini, 200.0),
            // Species 3: Resident, not dominant
            create_specie_record("SP3", GruppoEcoHFBI::ResidentiDiEstuario, 100.0),
            // Species 1: Migratory, dominant, contributes to all metrics
            create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 500.0),
        ]);

        let actual: Vec<(&str, f32)> = campione
            .into_iter()
            .map(|r| (r.specie.codice_specie.as_str(), r.peso))
            .collect();

        let expected = vec![("SP1", 500.0), ("SP2", 200.0), ("SP3", 100.0)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_from_campionamentohfbi_to_vec_recordhfbi_order_invariant() {
        let campione = CampionamentoHFBI::new_raw_unsorted(vec![
            // Species 2: Migratory, contributes to most metrics
            create_specie_record("SP2", GruppoEcoHFBI::MigratoriMarini, 200.0),
            // Species 3: Resident, not dominant
            create_specie_record("SP3", GruppoEcoHFBI::ResidentiDiEstuario, 100.0),
            // Species 1: Migratory, dominant, contributes to all metrics
            create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 500.0),
        ]);

        let into: Vec<RecordHFBI> = campione.into();

        let actual: Vec<(&str, f32)> = into
            .iter()
            .map(|r| (r.specie.codice_specie.as_str(), r.peso))
            .collect();

        let expected = vec![("SP1", 500.0), ("SP2", 200.0), ("SP3", 100.0)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_campionamentohfbi_sort_by_peso_desc() {
        let mut campione = CampionamentoHFBI::new_raw_unsorted(vec![
            // Species 2: Migratory, contributes to most metrics
            create_specie_record("SP2", GruppoEcoHFBI::MigratoriMarini, 200.0),
            // Species 3: Resident, not dominant
            create_specie_record("SP3", GruppoEcoHFBI::ResidentiDiEstuario, 100.0),
            // Species 1: Migratory, dominant, contributes to all metrics
            create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 500.0),
        ]);

        campione.sort_by_peso_desc();

        #[allow(deprecated)]
        let actual: Vec<(&str, f32)> = campione
            .campionamento
            .iter()
            .map(|r| (r.specie.codice_specie.as_str(), r.peso))
            .collect();

        let expected = vec![("SP1", 500.0), ("SP2", 200.0), ("SP3", 100.0)];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_campionamentohfbi_sorted_by_peso_desc() {
        let campione = CampionamentoHFBI::new_raw_unsorted(vec![
            // Species 2: Migratory, contributes to most metrics
            create_specie_record("SP2", GruppoEcoHFBI::MigratoriMarini, 200.0),
            // Species 3: Resident, not dominant
            create_specie_record("SP3", GruppoEcoHFBI::ResidentiDiEstuario, 100.0),
            // Species 1: Migratory, dominant, contributes to all metrics
            create_specie_record("SP1", GruppoEcoHFBI::Diadromi, 500.0),
        ]);

        let sorted = campione.sorted_by_peso_desc();

        let actual: Vec<(&str, f32)> = sorted
            .map(|r| (r.specie.codice_specie.as_str(), r.peso))
            .collect();

        let expected = vec![("SP1", 500.0), ("SP2", 200.0), ("SP3", 100.0)];

        assert_eq!(actual, expected);
    }
}
