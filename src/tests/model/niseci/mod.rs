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

use crate::{
    domain::niseci::{CampionamentoNISECI, RecordNISECI, SpecieNISECI},
    engines::niseci::linear_regression::Point,
};

#[test]
fn test_calcolo_pesci_per_passaggio() {
    let specie_1 = SpecieNISECI::new(
        "1234",
        "Cervus elaphus",
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
    );

    let record_1 = RecordNISECI::new(
        &specie_1, 1, 100,   // in millimetri
        100.0, // in grammi
    );

    let record_2 = RecordNISECI::new(
        &specie_1, 2, 100,   // in millimetri
        100.0, // in grammi
    );

    let record_3 = RecordNISECI::new(
        &specie_1, 3, 100,   // in millimetri
        100.0, // in grammi
    );

    let mut c1 = vec![record_1; 20];
    let mut c2 = vec![record_2; 15];
    let mut c3 = vec![record_3; 10];

    c1.append(&mut c2);
    c1.append(&mut c3);
    let campionamento = CampionamentoNISECI::new(c1);

    let pesci_per_passaggio = campionamento.fishes_for_every_passage();

    assert_eq!(pesci_per_passaggio[0], Point::new(20, 20));
    assert_eq!(pesci_per_passaggio[1], Point::new(35, 15));
    assert_eq!(pesci_per_passaggio[2], Point::new(45, 10));
}
