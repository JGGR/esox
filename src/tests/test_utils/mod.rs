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

use crate::domain::niseci::{CampionamentoNISECI, RecordNISECI};
use crate::domain::{
    location::Location,
    niseci::{
        AnagraficaNISECI, AreaNISECI, ComunitaNISECI, IdSpecieNISECI, IdroEcoRegioneNISECI,
        RiferimentoNISECI, SpecieNISECI, TipoComunitaNISECI,
    },
};

pub const RIFERIMENTO_NISECI_TEMPLATE_DATA: &[u8] =
    include_bytes!("../../../templates/riferimento_niseci.csv");
pub const CAMPIONAMENTO_NISECI_TEMPLATE_DATA: &[u8] =
    include_bytes!("../../../templates/campionamento_niseci.csv");
pub const ANAGRAFICA_NISECI_TEMPLATE_DATA: &[u8] =
    include_bytes!("../../../templates/anagrafica_niseci.csv");
pub const CAMPIONAMENTO_HFBI_TEMPLATE_DATA: &[u8] =
    include_bytes!("../../../templates/campionamento_hfbi.csv");
pub const ANAGRAFICA_HFBI_TEMPLATE_DATA: &[u8] =
    include_bytes!("../../../templates/anagrafica_hfbi.csv");

pub fn create_dummy_anagrafica() -> AnagraficaNISECI {
    return AnagraficaNISECI::new_raw_unchecked(
        ComunitaNISECI {
            #[expect(deprecated)]
            tipo: TipoComunitaNISECI::Redatta,
            #[expect(deprecated)]
            fonte: None,
            #[expect(deprecated)]
            numero_protocollo: None,
        },
        "foo".to_string(),
        "foo".to_string(),
        AreaNISECI::Alpina,
        "foo".to_string(),
        "foo".to_string(),
        IdroEcoRegioneNISECI::Toscana,
        Location {
            regione: "foo".to_string(),
            provincia: "foo".to_string(),
        },
        1.0,
        1.0,
    );
}

pub struct DummySetSpecieNISECI {
    importante_1: (SpecieNISECI, IdSpecieNISECI),
    importante_2: (SpecieNISECI, IdSpecieNISECI),
    importante_3: (SpecieNISECI, IdSpecieNISECI),
    normale_1: (SpecieNISECI, IdSpecieNISECI),
    normale_2: (SpecieNISECI, IdSpecieNISECI),
    alloctono_1: (SpecieNISECI, IdSpecieNISECI),
    alloctono_2: (SpecieNISECI, IdSpecieNISECI),
    inatteso_1: (SpecieNISECI, IdSpecieNISECI),
    inatteso_2: (SpecieNISECI, IdSpecieNISECI),
}

#[derive(Debug, Clone, Copy)]
enum IdDummySetSpecieNISECI {
    CiaccioCiaccensis = 0,
    CiaccioSbribbrensis = 1,
    CiaccioCozzensis = 2,
    NormusSempliciottum = 3,
    NormusQualunquis = 4,
    DisturbusInfognatus = 5,
    DisturbusSotterfugius = 6,
    SorprendoSorprendentes = 7, // TODO: Check if non-duplicate id still passes the test, i think so
    SorprendoImprovvisus = 8,
}

pub fn create_dummy_specie_niseci_set() -> DummySetSpecieNISECI {
    let importante_1 = (
        SpecieNISECI::new(
            &(IdDummySetSpecieNISECI::CiaccioCiaccensis as u32).to_string(),
            "Ciaccio ciaccensis",
            2,
            0,
            true,
            1,
            2,
            3,
            4,
            1.0,
            2.0,
            3.0,
            4.0,
            1.0,
            2.0,
        ),
        IdDummySetSpecieNISECI::CiaccioCiaccensis as u32,
    );
    let importante_2 = (
        SpecieNISECI::new(
            &(IdDummySetSpecieNISECI::CiaccioSbribbrensis as u32).to_string(),
            "Ciaccio sbribbrensis",
            2,
            0,
            true,
            1,
            2,
            3,
            4,
            1.0,
            2.0,
            3.0,
            4.0,
            1.0,
            2.0,
        ),
        IdDummySetSpecieNISECI::CiaccioSbribbrensis as u32,
    );
    let importante_3 = (
        SpecieNISECI::new(
            &(IdDummySetSpecieNISECI::CiaccioCozzensis as u32).to_string(),
            "Ciaccio cozzensis",
            2,
            0,
            true,
            1,
            2,
            3,
            4,
            1.0,
            2.0,
            3.0,
            4.0,
            1.0,
            2.0,
        ),
        IdDummySetSpecieNISECI::CiaccioCozzensis as u32,
    );
    let normale_1 = (
        SpecieNISECI::new(
            &(IdDummySetSpecieNISECI::NormusSempliciottum as u32).to_string(),
            "Normus sempliciottum",
            1,
            0,
            true,
            1,
            2,
            3,
            4,
            1.0,
            2.0,
            3.0,
            4.0,
            1.0,
            2.0,
        ),
        IdDummySetSpecieNISECI::NormusSempliciottum as u32,
    );
    let normale_2 = (
        SpecieNISECI::new(
            &(IdDummySetSpecieNISECI::NormusQualunquis as u32).to_string(),
            "Normus qualunquis",
            1,
            0,
            true,
            1,
            2,
            3,
            4,
            1.0,
            2.0,
            3.0,
            4.0,
            1.0,
            2.0,
        ),
        IdDummySetSpecieNISECI::NormusQualunquis as u32,
    );
    let alloctono_1 = (
        SpecieNISECI::new(
            &(IdDummySetSpecieNISECI::DisturbusInfognatus as u32).to_string(),
            "Disturbus infognatus",
            0,
            1,
            true,
            1,
            2,
            3,
            4,
            1.0,
            2.0,
            3.0,
            4.0,
            1.0,
            2.0,
        ),
        IdDummySetSpecieNISECI::DisturbusInfognatus as u32,
    );
    let alloctono_2 = (
        SpecieNISECI::new(
            &(IdDummySetSpecieNISECI::DisturbusSotterfugius as u32).to_string(),
            "Disturbus sotterfugius",
            0,
            1,
            true,
            1,
            2,
            3,
            4,
            1.0,
            2.0,
            3.0,
            4.0,
            1.0,
            2.0,
        ),
        IdDummySetSpecieNISECI::DisturbusSotterfugius as u32,
    );
    let inatteso_1 = (
        SpecieNISECI::new(
            &(IdDummySetSpecieNISECI::SorprendoSorprendentes as u32).to_string(),
            "Sorprendo sorprendentes",
            0,
            1,
            false,
            1,
            2,
            3,
            4,
            1.0,
            2.0,
            3.0,
            4.0,
            1.0,
            2.0,
        ),
        IdDummySetSpecieNISECI::SorprendoSorprendentes as u32,
    );
    let inatteso_2 = (
        SpecieNISECI::new(
            &(IdDummySetSpecieNISECI::SorprendoImprovvisus as u32).to_string(),
            "Sorprendo improvvisus",
            2,
            0,
            false,
            1,
            2,
            3,
            4,
            1.0,
            2.0,
            3.0,
            4.0,
            1.0,
            2.0,
        ),
        IdDummySetSpecieNISECI::SorprendoImprovvisus as u32,
    );

    DummySetSpecieNISECI {
        importante_1,
        importante_2,
        importante_3,
        normale_1,
        normale_2,
        alloctono_1,
        alloctono_2,
        inatteso_1,
        inatteso_2,
    }
}

pub fn create_dummy_riferimento() -> RiferimentoNISECI {
    let ds = create_dummy_specie_niseci_set();

    let mut elenco_specie = Vec::with_capacity(9);
    elenco_specie.push(ds.importante_1.0);
    elenco_specie.push(ds.importante_2.0);
    elenco_specie.push(ds.importante_3.0);
    elenco_specie.push(ds.normale_1.0);
    elenco_specie.push(ds.normale_2.0);
    elenco_specie.push(ds.alloctono_1.0);
    elenco_specie.push(ds.alloctono_2.0);
    elenco_specie.push(ds.inatteso_1.0);
    elenco_specie.push(ds.inatteso_2.0);

    RiferimentoNISECI::new_from_map(elenco_specie.into())
}

/// campionamento che ha dentro tutte le specie autoctone attese
/// in @create_dummy_riferimento.
/// Nel campionamento per futuri test si può modificare tutto
/// tranne le specie dei recordCsv,
/// che servono in @test_calculate_x1
pub fn create_dummy_campionamento_full() -> CampionamentoNISECI {
    let ds = create_dummy_specie_niseci_set();
    let record_1 = RecordNISECI::new(ds.importante_1.1, 5, 1, 5.0);

    let record_2 = RecordNISECI::new(ds.importante_2.1, 5, 1, 5.0);

    let record_3 = RecordNISECI::new(ds.importante_3.1, 5, 1, 5.0);

    let record_4 = RecordNISECI::new(ds.normale_2.1, 5, 1, 5.0);

    let record_5 = RecordNISECI::new(ds.normale_1.1, 5, 1, 5.0);

    let record_6 = RecordNISECI::new(ds.inatteso_1.1, 5, 1, 5.0);

    let record_7 = RecordNISECI::new(ds.inatteso_2.1, 5, 1, 5.0);

    let record_8 = RecordNISECI::new(ds.alloctono_1.1, 5, 1, 5.0);

    let record_9 = RecordNISECI::new(ds.alloctono_2.1, 5, 1, 5.0);

    let mut campionamento = Vec::with_capacity(9);
    campionamento.push(record_1);
    campionamento.push(record_2);
    campionamento.push(record_3);
    campionamento.push(record_4);
    campionamento.push(record_5);
    campionamento.push(record_6);
    campionamento.push(record_7);
    campionamento.push(record_8);
    campionamento.push(record_9);

    CampionamentoNISECI::new(campionamento)
}

/// campionamento che non contiene tutte le specie
/// create in @create_dummy_riferimento
pub fn create_dummy_campionamento_chopped() -> CampionamentoNISECI {
    // uso il full campionamento e vado poi a togliere alcuni record
    let campionamento = create_dummy_campionamento_full();

    let mut chopped: Vec<RecordNISECI> = campionamento.into();
    chopped.remove(1);

    CampionamentoNISECI::new(chopped)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
/// aggiungo in C2 anche una specie alloctono per testare la conta delle specie autoctone trovate
/// x2a qua vale 1.0 e x2b 1.0
pub fn create_massive_campionamento_ciacci() -> CampionamentoNISECI {
    let mut c = create_massive_campionamento_ciacci_solo_autoctoni_1();

    let r = create_massive_riferimento_ciacci();

    let t = r
        .get_inner_id(&(DummyIds::Trocchio as u32).to_string())
        .expect("create_massive_riferimento_niseci has a Trocchio");
    // pesce alloctono in C2
    let trocchio = RecordNISECI::new(t, 2, 2, 2.0);
    c.push(trocchio);

    c
}

pub fn create_massive_riferimento_ciacci() -> RiferimentoNISECI {
    let mut r = create_massive_riferimento_ciacci_solo_autoctoni_1();
    r.push(get_trocchio());
    r
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
/// -> C2
///   -> 10 ciacci cl4
///   -> 5 ciacci cl1
/// aggiungo in C2 anche una specie alloctono per testare la conta delle specie autoctone trovate
/// x2a qua vale 0.5 e x2b 1.0
pub fn create_massive_campionamento_ciacci_2() -> CampionamentoNISECI {
    let mut c = create_massive_campionamento_ciacci_solo_autoctoni_2();
    let r = create_massive_riferimento_ciacci_2();

    let t = r
        .get_inner_id(&(DummyIds::Trocchio as u32).to_string())
        .expect("create_massive_riferimento_niseci has a Trocchio");
    assert_eq!(t, 1);
    // pesce alloctono in C2
    let trocchio = RecordNISECI::new(t, 2, 2, 2.0);
    c.push(trocchio);
    c
}

pub fn create_massive_riferimento_ciacci_2() -> RiferimentoNISECI {
    let mut r = create_massive_riferimento_ciacci_solo_autoctoni_2();
    r.push(get_trocchio());
    r
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
/// -> C2
///   -> 10 ciacci cl4
///   -> 5 ciacci cl1
/// non sono presenti specie alloctone
pub fn create_massive_campionamento_ciacci_solo_autoctoni_2() -> CampionamentoNISECI {
    let mut ciaccio_v1 = get_ciaccio();
    ciaccio_v1.set_dens_soglia_1(3.0);
    ciaccio_v1.set_dens_soglia_2(5.0);
    let ciaccio = DummyIds::Ciaccio as u32;

    let mut campionamento: Vec<RecordNISECI> = Vec::with_capacity(45);

    // 10 ciacci cl5 in c1
    let ciaccio_cl5_c1 = RecordNISECI::new(ciaccio, 1, 13, 10.0);
    for _ in 0..10 {
        campionamento.push(ciaccio_cl5_c1.clone());
    }

    // 10 ciacci cl4 in c1
    let ciaccio_cl4_c1 = RecordNISECI::new(ciaccio, 1, 10, 10.0);
    for _ in 0..10 {
        campionamento.push(ciaccio_cl4_c1.clone());
    }

    // 10 ciacci cl3 in c1
    let ciaccio_cl3_c1 = RecordNISECI::new(ciaccio, 1, 7, 10.0);
    for _ in 0..10 {
        campionamento.push(ciaccio_cl3_c1.clone());
    }

    // 10 ciacci cl4 in c2
    let ciaccio_cl4_c2 = RecordNISECI::new(ciaccio, 2, 10, 10.0);
    for _ in 0..10 {
        campionamento.push(ciaccio_cl4_c2.clone());
    }

    // 5 ciacci cl1 in c2
    let ciaccio_cl1_c2 = RecordNISECI::new(ciaccio, 2, 2, 10.0);
    for _ in 0..5 {
        campionamento.push(ciaccio_cl1_c2.clone());
    }

    CampionamentoNISECI::new(campionamento)
}

pub fn create_massive_riferimento_ciacci_solo_autoctoni_2() -> RiferimentoNISECI {
    let c = create_massive_campionamento_ciacci_solo_autoctoni_2();
    let mut rif = Vec::new();
    for r in &c {
        let id = r.id();
        assert_eq!(id, DummyIds::Ciaccio as u32);
        rif.push(get_ciaccio());
    }
    RiferimentoNISECI::new(rif)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
/// non sono presenti specie alloctone
pub fn create_massive_campionamento_ciacci_solo_autoctoni_1() -> CampionamentoNISECI {
    let mut ciaccio_v1 = get_ciaccio();
    ciaccio_v1.set_dens_soglia_1(3.0);
    ciaccio_v1.set_dens_soglia_2(5.0);
    let ciaccio = DummyIds::Ciaccio as u32;

    let mut campionamento: Vec<RecordNISECI> = Vec::with_capacity(45);

    // 10 ciacci cl5 in c1
    let ciaccio_cl5_c1 = RecordNISECI::new(ciaccio, 1, 13, 10.0);
    for _ in 0..10 {
        campionamento.push(ciaccio_cl5_c1.clone());
    }

    // 10 ciacci cl4 in c1
    let ciaccio_cl4_c1 = RecordNISECI::new(ciaccio, 1, 10, 10.0);
    for _ in 0..10 {
        campionamento.push(ciaccio_cl4_c1.clone());
    }

    // 10 ciacci cl3 in c1
    let ciaccio_cl3_c1 = RecordNISECI::new(ciaccio, 1, 7, 10.0);
    for _ in 0..10 {
        campionamento.push(ciaccio_cl3_c1.clone());
    }

    // 10 ciacci cl2 in c2
    let ciaccio_cl2_c2 = RecordNISECI::new(ciaccio, 2, 4, 10.0);
    for _ in 0..10 {
        campionamento.push(ciaccio_cl2_c2.clone());
    }

    // 5 ciacci cl1 in c2
    let ciaccio_cl1_c2 = RecordNISECI::new(ciaccio, 2, 4, 10.0);
    for _ in 0..5 {
        campionamento.push(ciaccio_cl1_c2.clone());
    }

    CampionamentoNISECI::new(campionamento)
}

pub fn create_massive_riferimento_ciacci_solo_autoctoni_1() -> RiferimentoNISECI {
    let c = create_massive_campionamento_ciacci_solo_autoctoni_1();
    let mut rif = Vec::new();
    for r in &c {
        let id = r.id();
        assert_eq!(id, DummyIds::Ciaccio as u32);
        rif.push(get_ciaccio());
    }
    RiferimentoNISECI::new(rif)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 trocchi cl5
///   -> 10 trocchi cl4
///   -> 10 trocchi cl3
/// -> C2
///   -> 10 trocchi cl2
///   -> 5 trocchi cl1
pub fn create_massive_campionamento_solo_tipo_alloctono_1_strutt() -> CampionamentoNISECI {
    let mut trocchio = get_trocchio();
    trocchio.set_dens_soglia_1(3.0);
    trocchio.set_dens_soglia_2(5.0);
    let trocchio = 1;

    create_campionamento_strutturato_data_una_specie(trocchio)
}

pub fn create_massive_riferimento_solo_tipo_alloctono_1_strutt() -> RiferimentoNISECI {
    let mut species = Vec::new();
    species.push(get_ciaccio());
    species.push(get_trocchio());
    RiferimentoNISECI::new(species)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 bronzi cl5
///   -> 10 bronzi cl4
///   -> 10 bronzi cl3
/// -> C2
///   -> 10 bronzi cl2
///   -> 5 bronzi cl1
pub fn create_massive_campionamento_solo_tipo_alloctono_2_strutt() -> CampionamentoNISECI {
    let mut bronzo = get_bronzo();
    bronzo.set_dens_soglia_1(3.0);
    bronzo.set_dens_soglia_2(5.0);
    let bronzo = 1;

    create_campionamento_strutturato_data_una_specie(bronzo)
}

pub fn create_massive_riferimento_solo_tipo_alloctono_2_strutt() -> RiferimentoNISECI {
    let mut species = Vec::new();
    species.push(get_ciaccio());
    species.push(get_bronzo());
    RiferimentoNISECI::new(species)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 tappi cl5
///   -> 10 tappi cl4
///   -> 10 tappi cl3
/// -> C2
///   -> 10 tappi cl2
///   -> 5 tappi cl1
pub fn create_massive_campionamento_solo_tipo_alloctono_3_strutt() -> CampionamentoNISECI {
    let mut tappo = get_tappo();
    tappo.set_dens_soglia_1(3.0);
    tappo.set_dens_soglia_2(5.0);
    let tappo = 1;

    create_campionamento_strutturato_data_una_specie(tappo)
}

pub fn create_massive_riferimento_solo_tipo_alloctono_3_strutt() -> RiferimentoNISECI {
    let mut species = Vec::new();
    species.push(get_ciaccio());
    species.push(get_tappo());
    RiferimentoNISECI::new(species)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 15 tappi cl5
///   -> 20 tappi cl4
/// -> C2
///   -> 5 tappi cl2
pub fn create_massive_campionamento_solo_tipo_alloctono_3_destrutt() -> CampionamentoNISECI {
    let mut tappo = get_tappo();
    tappo.set_dens_soglia_1(3.0);
    tappo.set_dens_soglia_2(5.0);
    let tappo = 1;

    create_campionamento_destrutturato_data_una_specie(tappo)
}

pub fn create_massive_riferimento_solo_tipo_alloctono_3_destrutt() -> RiferimentoNISECI {
    let mut species = Vec::new();
    species.push(get_ciaccio());
    species.push(get_tappo());
    RiferimentoNISECI::new(species)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 15 tappi cl5
///   -> 20 tappi cl4
/// -> C2
///   -> 5 tappi cl2
///   -> 5 tappi cl1
pub fn create_massive_campionamento_solo_tipo_alloctono_3_mediam_strutt() -> CampionamentoNISECI {
    let mut tappo = get_tappo();
    tappo.set_dens_soglia_1(3.0);
    tappo.set_dens_soglia_2(5.0);
    let tappo = 1;

    create_campionamento_mediam_strutturato_data_una_specie(tappo)
}

pub fn create_massive_riferimento_solo_tipo_alloctono_3_mediam_strutt() -> RiferimentoNISECI {
    let mut species = Vec::new();
    species.push(get_ciaccio());
    species.push(get_tappo());
    RiferimentoNISECI::new(species)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
///   -> 10 trocchi cl5
///   -> 10 trocchi cl4
///   -> 10 trocchi cl3
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
///   -> 10 trocchi cl2
///   -> 5 trocchi cl1
pub fn create_massive_campionamento_ciacci_con_trocchi_strutt() -> CampionamentoNISECI {
    let mut c_ciacci = create_massive_campionamento_ciacci_solo_autoctoni_1();
    let mut c_trocchi = create_massive_campionamento_solo_tipo_alloctono_1_strutt();

    c_trocchi.as_mut_vec().append(&mut c_ciacci.as_mut_vec());

    c_trocchi
}

pub fn create_massive_riferimento_ciacci_con_trocchi_strutt() -> RiferimentoNISECI {
    let mut r_1 = create_massive_riferimento_ciacci_solo_autoctoni_1();
    let r_2 = create_massive_riferimento_solo_tipo_alloctono_1_strutt();
    for s in &r_2 {
        r_1.push(s.clone());
    }
    r_1
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
///   -> 10 bronzi cl5
///   -> 10 bronzi cl4
///   -> 10 bronzi cl3
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
///   -> 10 bronzi cl2
///   -> 5 bronzi cl1
pub fn create_massive_campionamento_ciacci_con_bronzi_strutt() -> CampionamentoNISECI {
    let mut c_ciacci = create_massive_campionamento_ciacci_solo_autoctoni_1();
    let mut c_bronzi = create_massive_campionamento_solo_tipo_alloctono_2_strutt();

    c_bronzi.as_mut_vec().append(&mut c_ciacci.as_mut_vec());

    let ciaccio = DummyIds::Ciaccio as u32;
    let ciaccio = RecordNISECI::new(ciaccio, 2, 2, 2.0);
    c_bronzi.push(ciaccio);

    c_bronzi
}

pub fn create_massive_riferimento_ciacci_con_bronzi_strutt() -> RiferimentoNISECI {
    let mut r_1 = create_massive_riferimento_ciacci_solo_autoctoni_1();
    let r_2 = create_massive_riferimento_solo_tipo_alloctono_2_strutt();
    for s in &r_2 {
        r_1.push(s.clone());
    }
    r_1
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
///   -> 10 tappi cl5
///   -> 10 tappi cl4
///   -> 10 tappi cl3
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
///   -> 10 tappi cl2
///   -> 5 tappi cl1
pub fn create_massive_campionamento_ciacci_con_tappi_strutt() -> CampionamentoNISECI {
    let mut c_ciacci = create_massive_campionamento_ciacci_solo_autoctoni_1();
    let mut c_tappi = create_massive_campionamento_solo_tipo_alloctono_3_strutt();

    c_tappi.as_mut_vec().append(&mut c_ciacci.as_mut_vec());

    let ciaccio = DummyIds::Ciaccio as u32;
    let ciaccio = RecordNISECI::new(ciaccio, 2, 2, 2.0);
    c_tappi.push(ciaccio);

    c_tappi
}

pub fn create_massive_riferimento_ciacci_con_tappi_strutt() -> RiferimentoNISECI {
    let mut r_1 = create_massive_riferimento_ciacci_solo_autoctoni_1();
    let r_2 = create_massive_riferimento_solo_tipo_alloctono_3_strutt();
    for s in &r_2 {
        r_1.push(s.clone());
    }
    r_1
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
///   -> 15 tappi cl5
///   -> 20 tappi cl4
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
///   -> 5 tappi cl2
pub fn create_massive_campionamento_ciacci_con_tappi_destrutt() -> CampionamentoNISECI {
    let mut c_ciacci = create_massive_campionamento_ciacci_solo_autoctoni_1();
    let mut c_tappi = create_massive_campionamento_solo_tipo_alloctono_3_destrutt();

    c_tappi.as_mut_vec().append(&mut c_ciacci.as_mut_vec());

    let ciaccio = DummyIds::Ciaccio as u32;
    let ciaccio = RecordNISECI::new(ciaccio, 2, 2, 2.0);
    c_tappi.push(ciaccio);

    c_tappi
}

pub fn create_massive_riferimento_ciacci_con_tappi_destrutt() -> RiferimentoNISECI {
    let mut r_1 = create_massive_riferimento_ciacci_solo_autoctoni_1();
    let r_2 = create_massive_riferimento_solo_tipo_alloctono_3_destrutt();
    for s in &r_2 {
        r_1.push(s.clone());
    }
    r_1
}

/// In questo campionamento troverai:
/// -> C1
///   -> 10 ciacci cl5
///   -> 10 ciacci cl4
///   -> 10 ciacci cl3
///   -> 15 tappi cl5
///   -> 20 tappi cl4
/// -> C2
///   -> 10 ciacci cl2
///   -> 5 ciacci cl1
///   -> 5 tappi cl2
///   -> 5 tappi cl1
pub fn create_massive_campionamento_ciacci_con_tappi_mediam_strutt() -> CampionamentoNISECI {
    let mut c_ciacci = create_massive_campionamento_ciacci_solo_autoctoni_1();
    let mut c_tappi = create_massive_campionamento_solo_tipo_alloctono_3_mediam_strutt();

    c_tappi.as_mut_vec().append(&mut c_ciacci.as_mut_vec());

    let ciaccio = DummyIds::Ciaccio as u32;
    let ciaccio = RecordNISECI::new(ciaccio, 2, 2, 2.0);
    c_tappi.push(ciaccio);

    c_tappi
}

pub fn create_massive_riferimento_ciacci_con_tappi_mediam_strutt() -> RiferimentoNISECI {
    let mut r_1 = create_massive_riferimento_ciacci_solo_autoctoni_1();
    let r_2 = create_massive_riferimento_solo_tipo_alloctono_3_mediam_strutt();
    for s in &r_2 {
        r_1.push(s.clone());
    }
    r_1
}

type ArgSpecieNISECI = IdSpecieNISECI;

/// In questo campionamento troverai:
/// -> C1
///   -> 10 esemplari cl5
///   -> 10 esemplari cl4
///   -> 10 esemplari cl3
/// -> C2
///   -> 10 esemplari cl2
///   -> 5 esemplari cl1
fn create_campionamento_strutturato_data_una_specie(
    specie: ArgSpecieNISECI,
) -> CampionamentoNISECI {
    let mut campionamento: Vec<RecordNISECI> = Vec::with_capacity(45);

    // 10 esemplari cl5 in c1
    let cl5_c1 = RecordNISECI::new(specie, 1, 13, 10.0);
    for _ in 0..10 {
        campionamento.push(cl5_c1.clone());
    }

    // 10 esemplari cl4 in c1
    let cl4_c1 = RecordNISECI::new(specie, 1, 10, 10.0);
    for _ in 0..10 {
        campionamento.push(cl4_c1.clone());
    }

    // 10 esemplari cl3 in c1
    let cl3_c1 = RecordNISECI::new(specie, 1, 7, 10.0);
    for _ in 0..10 {
        campionamento.push(cl3_c1.clone());
    }

    // 10 esemplari cl2 in c2
    let cl2_c2 = RecordNISECI::new(specie, 2, 4, 10.0);
    for _ in 0..10 {
        campionamento.push(cl2_c2.clone());
    }

    // 5 esemplari cl1 in c2
    let cl1_c2 = RecordNISECI::new(specie, 2, 4, 10.0);
    for _ in 0..5 {
        campionamento.push(cl1_c2.clone());
    }

    CampionamentoNISECI::new(campionamento)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 15 esemplari cl5
///   -> 20 esemplari cl4
/// -> C2
///   -> 5 esemplari cl2
fn create_campionamento_destrutturato_data_una_specie(
    specie: ArgSpecieNISECI,
) -> CampionamentoNISECI {
    let mut campionamento: Vec<RecordNISECI> = Vec::with_capacity(45);

    // 10 esemplari cl5 in c1
    let cl5_c1 = RecordNISECI::new(specie, 1, 13, 10.0);
    for _ in 0..10 {
        campionamento.push(cl5_c1.clone());
    }

    // 20 esemplari cl4 in c1
    let cl4_c1 = RecordNISECI::new(specie, 1, 10, 10.0);
    for _ in 0..20 {
        campionamento.push(cl4_c1.clone());
    }

    // 10 esemplari cl2 in c2
    let cl2_c2 = RecordNISECI::new(specie, 2, 4, 10.0);
    for _ in 0..10 {
        campionamento.push(cl2_c2.clone());
    }

    CampionamentoNISECI::new(campionamento)
}

/// In questo campionamento troverai:
/// -> C1
///   -> 15 esemplari cl5
///   -> 20 esemplari cl4
/// -> C2
///   -> 5 esemplari cl2
///   -> 5 esemplari cl1
fn create_campionamento_mediam_strutturato_data_una_specie(
    specie: ArgSpecieNISECI,
) -> CampionamentoNISECI {
    let mut campionamento: Vec<RecordNISECI> = Vec::with_capacity(45);

    // 10 esemplari cl5 in c1
    let cl5_c1 = RecordNISECI::new(specie, 1, 13, 10.0);
    for _ in 0..10 {
        campionamento.push(cl5_c1.clone());
    }

    // 20 esemplari cl4 in c1
    let cl4_c1 = RecordNISECI::new(specie, 1, 10, 10.0);
    for _ in 0..20 {
        campionamento.push(cl4_c1.clone());
    }

    // 10 esemplari cl2 in c2
    let cl2_c2 = RecordNISECI::new(specie, 2, 4, 10.0);
    for _ in 0..10 {
        campionamento.push(cl2_c2.clone());
    }

    // 5 esemplari cl1 in c2
    let cl1_c2 = RecordNISECI::new(specie, 2, 4, 10.0);
    for _ in 0..5 {
        campionamento.push(cl1_c2.clone());
    }

    CampionamentoNISECI::new(campionamento)
}

#[derive(Debug, Clone, Copy)]
pub enum DummyIds {
    Ciaccio = 0,
    Trocchio = 1,
    Bronzo = 2,
    Tappo = 3,
}

impl TryFrom<u32> for DummyIds {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(DummyIds::Ciaccio),
            1 => Ok(DummyIds::Trocchio),
            2 => Ok(DummyIds::Bronzo),
            3 => Ok(DummyIds::Tappo),
            _ => Err(()),
        }
    }
}

/// id == 1
pub fn get_ciaccio() -> SpecieNISECI {
    SpecieNISECI::new(
        &(DummyIds::Ciaccio as u32).to_string(),
        "Ciaccio ciaccensis",
        2,
        0,
        true,
        3,
        6,
        9,
        12,
        0.5,
        0.67,
        1.5,
        2.0,
        3.0,
        5.0,
    )
}

/// id == 2
pub fn get_trocchio() -> SpecieNISECI {
    SpecieNISECI::new(
        &(DummyIds::Trocchio as u32).to_string(),
        "Trocchio trocchiensis",
        0,
        1,
        false,
        3,
        6,
        9,
        12,
        0.5,
        0.67,
        1.5,
        2.0,
        3.0,
        5.0,
    )
}

/// id == 3
pub fn get_bronzo() -> SpecieNISECI {
    SpecieNISECI::new(
        &(DummyIds::Bronzo as u32).to_string(),
        "Bronzo bronzensis",
        0,
        2,
        false,
        3,
        6,
        9,
        12,
        0.5,
        0.67,
        1.5,
        2.0,
        3.0,
        5.0,
    )
}

/// id == 4
pub fn get_tappo() -> SpecieNISECI {
    SpecieNISECI::new(
        &(DummyIds::Tappo as u32).to_string(),
        "Tappo sugheribus",
        0,
        3,
        false,
        3,
        6,
        9,
        12,
        0.5,
        0.67,
        1.5,
        2.0,
        3.0,
        5.0,
    )
}
