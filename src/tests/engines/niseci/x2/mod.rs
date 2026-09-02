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

use crate::engines::niseci::x2::calculate_x2;

use crate::tests::test_utils::{
    create_massive_riferimento_ciacci, create_massive_riferimento_ciacci_2, DummyIds,
};

use crate::{
    domain::{
        location::Location,
        niseci::{
            AnagraficaNISECI, AreaNISECI, ClassiEtaSpecieNISECI, ComunitaNISECI,
            IdroEcoRegioneNISECI, RiferimentoNISECI, TipoComunitaNISECI,
        },
    },
    tests::test_utils::{
        create_massive_campionamento_ciacci, create_massive_campionamento_ciacci_2, get_ciaccio,
    },
};

#[test]
fn calculate_x2_a_criterio_a_5_classi_valorizzate() {
    let classe = ClassiEtaSpecieNISECI::new_custom(DummyIds::Ciaccio as u32, 1, 1, 1, 1, 1);

    let x2_a_criterio_a = classe.get_x2_a_criterio_a();
    assert_eq!(1, x2_a_criterio_a)
}

#[test]
fn calculate_x2_a_criterio_a_3_classi_valorizzate() {
    let classe = ClassiEtaSpecieNISECI::new_custom(DummyIds::Ciaccio as u32, 0, 0, 1, 1, 1);

    let x2_a_criterio_a = classe.get_x2_a_criterio_a();
    assert_eq!(2, x2_a_criterio_a)
}

#[test]
fn calculate_x2_a_criterio_a_2_classi_valorizzate() {
    let classe = ClassiEtaSpecieNISECI::new_custom(DummyIds::Ciaccio as u32, 0, 0, 0, 1, 1);

    let x2_a_criterio_a = classe.get_x2_a_criterio_a();
    assert_eq!(3, x2_a_criterio_a)
}

#[test]
fn calculate_x2_a_criterio_b_zero_giovani() {
    let classe = ClassiEtaSpecieNISECI::new_custom(DummyIds::Ciaccio as u32, 0, 0, 0, 1, 1);

    let Some((x2_a_criterio_b, _ad_juv)) =
        classe.get_x2_a_criterio_b(&RiferimentoNISECI::new(vec![get_ciaccio()]))
    else {
        panic!("Unexpected None")
    };
    assert_eq!(3, x2_a_criterio_b)
}

#[test]
fn calculate_x2_a_criterio_b_1_bilanciato() {
    let classe = ClassiEtaSpecieNISECI::new_custom(DummyIds::Ciaccio as u32, 0, 1, 1, 1, 1);

    let Some((x2_a_criterio_b, _ad_juv)) =
        classe.get_x2_a_criterio_b(&RiferimentoNISECI::new(vec![get_ciaccio()]))
    else {
        panic!("Unexpected None")
    };
    assert_eq!(1, x2_a_criterio_b)
}

#[test]
fn calculate_x2_a_criterio_b_2_medio_sbilanciato_adulti() {
    let classe = ClassiEtaSpecieNISECI::new_custom(DummyIds::Ciaccio as u32, 0, 1, 1, 2, 2);

    let Some((x2_a_criterio_b, _ad_juv)) =
        classe.get_x2_a_criterio_b(&RiferimentoNISECI::new(vec![get_ciaccio()]))
    else {
        panic!("Unexpected None")
    };
    assert_eq!(2, x2_a_criterio_b)
}

#[test]
fn calculate_x2_a_criterio_b_2_medio_sbilanciato_giovani() {
    let classe = ClassiEtaSpecieNISECI::new_custom(DummyIds::Ciaccio as u32, 0, 2, 2, 1, 1);

    let Some((x2_a_criterio_b, _ad_juv)) =
        classe.get_x2_a_criterio_b(&RiferimentoNISECI::new(vec![get_ciaccio()]))
    else {
        panic!("Unexpected None")
    };
    assert_eq!(2, x2_a_criterio_b)
}

#[test]
fn calculate_x2_a_criterio_b_3_molto_sbilanciato_adulti() {
    let classe = ClassiEtaSpecieNISECI::new_custom(DummyIds::Ciaccio as u32, 0, 1, 1, 3, 3);

    let Some((x2_a_criterio_b, _ad_juv)) =
        classe.get_x2_a_criterio_b(&RiferimentoNISECI::new(vec![get_ciaccio()]))
    else {
        panic!("Unexpected None")
    };
    assert_eq!(3, x2_a_criterio_b)
}

#[test]
fn calculate_x2_a_criterio_b_3_molto_sbilanciato_giovani() {
    let classe = ClassiEtaSpecieNISECI::new_custom(DummyIds::Ciaccio as u32, 0, 3, 3, 1, 1);

    let Some((x2_a_criterio_b, _ad_juv)) =
        classe.get_x2_a_criterio_b(&RiferimentoNISECI::new(vec![get_ciaccio()]))
    else {
        panic!("Unexpected None")
    };
    assert_eq!(3, x2_a_criterio_b)
}

#[test]
fn calculate_x2_test_1() {
    let campionamento = create_massive_campionamento_ciacci();

    let comunita = ComunitaNISECI::new(
        TipoComunitaNISECI::Dm260_2010,
        Some("hey".to_string()),
        None,
    );

    let anagrafica = AnagraficaNISECI::new_raw_unchecked(
        comunita,
        "dummy".to_string(),
        "Never".to_string(),
        AreaNISECI::Alpina,
        "Foo".to_string(),
        "canaletta".to_string(),
        IdroEcoRegioneNISECI::AlpiCentroOrientali,
        Location {
            regione: "sardninaia".to_string(),
            provincia: "oristano".to_string(),
        },
        1.0,
        10.0,
    );

    let x2 = calculate_x2(
        &campionamento,
        &anagrafica,
        &create_massive_riferimento_ciacci(),
        true,
    );

    assert!(x2.is_ok());

    let (x2, _criteri_x2) = x2.unwrap();
    assert_eq!(Some(1.0), x2);

    // secondo test con valori alternativi

    let campionamento = create_massive_campionamento_ciacci_2();

    let comunita = ComunitaNISECI::new(
        TipoComunitaNISECI::Dm260_2010,
        Some("hey".to_string()),
        None,
    );

    let anagrafica = AnagraficaNISECI::new_raw_unchecked(
        comunita,
        "dummy".to_string(),
        "Never".to_string(),
        AreaNISECI::Alpina,
        "Foo".to_string(),
        "canaletta".to_string(),
        IdroEcoRegioneNISECI::AlpiCentroOrientali,
        Location {
            regione: "sardninaia".to_string(),
            provincia: "oristano".to_string(),
        },
        1.0,
        10.0,
    );

    let x2 = calculate_x2(
        &campionamento,
        &anagrafica,
        &create_massive_riferimento_ciacci_2(),
        true,
    );

    assert!(x2.is_ok());
    let epsilon: f32 = 1e-6;
    let (x2, _criteri_x2) = x2.unwrap();
    assert!(x2.is_some());
    let x2_val = x2.unwrap();
    assert!((0.7 - x2_val).abs() < epsilon);
}
