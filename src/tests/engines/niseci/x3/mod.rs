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

#[cfg(not(feature = "lessclone"))]
use crate::engines::niseci::x3::calculate_x3;
use crate::tests::test_utils::{
    create_massive_campionamento_ciacci_2, create_massive_campionamento_ciacci_con_bronzi_strutt,
    create_massive_campionamento_ciacci_con_tappi_destrutt,
    create_massive_campionamento_ciacci_con_tappi_mediam_strutt,
    create_massive_campionamento_ciacci_con_tappi_strutt,
    create_massive_campionamento_ciacci_con_trocchi_strutt,
    create_massive_campionamento_ciacci_solo_autoctoni_1,
};
#[cfg(feature = "lessclone")]
use crate::{
    engines::niseci::x3::lessclone::calculate_x3,
    tests::test_utils::{
        create_massive_riferimento_ciacci_2, create_massive_riferimento_ciacci_con_bronzi_strutt,
        create_massive_riferimento_ciacci_con_tappi_destrutt,
        create_massive_riferimento_ciacci_con_tappi_mediam_strutt,
        create_massive_riferimento_ciacci_con_tappi_strutt,
        create_massive_riferimento_ciacci_con_trocchi_strutt,
        create_massive_riferimento_ciacci_solo_autoctoni_1,
    },
};

#[test]
fn calculate_x3_assenza_specie_aliene() {
    let c = create_massive_campionamento_ciacci_solo_autoctoni_1();
    #[cfg(not(feature = "lessclone"))]
    let x3 = calculate_x3(&c);
    #[cfg(feature = "lessclone")]
    let x3 = calculate_x3(&c, &create_massive_riferimento_ciacci_solo_autoctoni_1());

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 1.0);
}

#[test]
fn calculate_x3_un_trocchio() {
    let c = create_massive_campionamento_ciacci_2();
    #[cfg(not(feature = "lessclone"))]
    let x3 = calculate_x3(&c);
    #[cfg(feature = "lessclone")]
    let x3 = calculate_x3(&c, &create_massive_riferimento_ciacci_2());

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.75);
}

#[test]
fn calculate_x3_alieni_magg_uguale_autoctoni() {
    let c = create_massive_campionamento_ciacci_con_trocchi_strutt();
    #[cfg(not(feature = "lessclone"))]
    let x3 = calculate_x3(&c);
    #[cfg(feature = "lessclone")]
    let x3 = calculate_x3(&c, &create_massive_riferimento_ciacci_con_trocchi_strutt());

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.0);
}

#[test]
fn calculate_x3_alieni_tipo_1_strutt() {
    let c = create_massive_campionamento_ciacci_con_trocchi_strutt();

    #[cfg(not(feature = "lessclone"))]
    let x3 = calculate_x3(&c);
    #[cfg(feature = "lessclone")]
    let x3 = calculate_x3(&c, &create_massive_riferimento_ciacci_con_trocchi_strutt());

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.0);
}

#[test]
fn calculate_x3_alieni_tipo_2_strutt() {
    let c = create_massive_campionamento_ciacci_con_bronzi_strutt();

    #[cfg(feature = "lessclone")]
    let r = create_massive_riferimento_ciacci_con_bronzi_strutt();

    #[cfg(not(feature = "lessclone"))]
    let x3 = calculate_x3(&c);
    #[cfg(feature = "lessclone")]
    let x3 = calculate_x3(&c, &r);

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.25);
}

#[test]
fn calculate_x3_alieni_tipo_3_strutt() {
    let c = create_massive_campionamento_ciacci_con_tappi_strutt();

    #[cfg(feature = "lessclone")]
    let r = create_massive_riferimento_ciacci_con_tappi_strutt();

    #[cfg(not(feature = "lessclone"))]
    let x3 = calculate_x3(&c);
    #[cfg(feature = "lessclone")]
    let x3 = calculate_x3(&c, &r);

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.375);
}

#[test]
fn calculate_x3_alieni_tipo_3_destrutt() {
    let c = create_massive_campionamento_ciacci_con_tappi_destrutt();

    #[cfg(feature = "lessclone")]
    let r = create_massive_riferimento_ciacci_con_tappi_destrutt();

    #[cfg(not(feature = "lessclone"))]
    let x3 = calculate_x3(&c);
    #[cfg(feature = "lessclone")]
    let x3 = calculate_x3(&c, &r);

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.875);
}

#[test]
fn calculate_x3_alieni_tipo_3_mediam_strutt() {
    let c = create_massive_campionamento_ciacci_con_tappi_mediam_strutt();

    #[cfg(feature = "lessclone")]
    let r = create_massive_riferimento_ciacci_con_tappi_mediam_strutt();

    #[cfg(not(feature = "lessclone"))]
    let x3 = calculate_x3(&c);
    #[cfg(feature = "lessclone")]
    let x3 = calculate_x3(&c, &r);

    assert!(x3.is_ok());
    let (x3, _criteri_x3) = x3.unwrap();
    assert_eq!(x3, 0.625);
}
