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

use crate::csv::deser::niseci::{
    check_anagrafica_niseci_reader_conf, check_campionamento_niseci_reader_conf,
    check_riferimento_niseci_reader_conf,
};
use crate::csv::deser::CsvConfig;
use crate::csv::load::{
    niseci::{
        load_anagrafica_niseci_from_reader, load_campionamento_niseci_from_reader,
        load_riferimento_niseci_from_reader,
    },
    InputFormat,
};
use crate::csv::stanis::niseci::{
    VeryItalianRecordAnagraficaNISECI, VeryItalianRecordCampionamentoNISECI,
    VeryItalianRecordRiferimentoNISECI,
};
#[cfg(not(feature = "lessclone"))]
use crate::engines::niseci::full::calculate_niseci;
#[cfg(feature = "lessclone")]
use crate::engines::niseci::full::lessclone::calculate_niseci;
use crate::parser::niseci::{
    check_records_anagrafica_niseci, check_records_campionamento_niseci,
    check_records_riferimento_niseci,
};
use crate::tests::test_utils::{
    create_dummy_anagrafica, create_dummy_campionamento_chopped, create_dummy_campionamento_full,
    create_dummy_riferimento, ANAGRAFICA_NISECI_TEMPLATE_DATA, CAMPIONAMENTO_NISECI_TEMPLATE_DATA,
    RIFERIMENTO_NISECI_TEMPLATE_DATA,
};

#[cfg(feature = "lessclone")]
use crate::domain::niseci::lessclone::ValoriIntermediNISECI;
#[cfg(not(feature = "lessclone"))]
use crate::domain::niseci::ValoriIntermediNISECI;

use std::io::Cursor;

#[test]
fn calculate_dummy_niseci_campionamento_full() {
    let riferimento = create_dummy_riferimento();
    let campionamento = create_dummy_campionamento_full();
    let anagrafica = create_dummy_anagrafica();
    let res = calculate_niseci(&campionamento, &riferimento, &anagrafica);

    assert!(res.is_ok());

    //TODO: assert the expected result
    //assert!(res.expect("calculate_dummy_niseci_campionamento_full(): is_ok was checked") == 3.14);
}

#[test]
fn calculate_dummy_niseci_campionamento_chopped() {
    let riferimento = create_dummy_riferimento();
    let campionamento = create_dummy_campionamento_chopped();
    let anagrafica = create_dummy_anagrafica();
    let res = calculate_niseci(&campionamento, &riferimento, &anagrafica);

    assert!(res.is_ok());

    //TODO: assert the expected result
    //assert!(res.expect("calculate_dummy_niseci_campionamento_chopped(): is_ok was checked") == 3.14);
}

#[test]
fn calculate_niseci_template() {
    let config = CsvConfig::default().with_delimiter(b';');
    let riferimento_reader = Cursor::new(RIFERIMENTO_NISECI_TEMPLATE_DATA);
    let riferimento_csv_check = check_riferimento_niseci_reader_conf::<
        _,
        VeryItalianRecordRiferimentoNISECI,
    >(riferimento_reader, config);

    assert!(riferimento_csv_check.is_ok());

    let riferimento_csv_records = riferimento_csv_check.expect("is_ok() was checked before");

    let riferimento_value_check = check_records_riferimento_niseci(riferimento_csv_records);

    assert!(riferimento_value_check.is_ok());

    let riferimento = riferimento_value_check.expect("is_ok() was checked before");

    let campionamento_reader = Cursor::new(CAMPIONAMENTO_NISECI_TEMPLATE_DATA);

    let campionamento_csv_check = check_campionamento_niseci_reader_conf::<
        _,
        VeryItalianRecordCampionamentoNISECI,
    >(campionamento_reader, config);

    assert!(campionamento_csv_check.is_ok());

    let campionamento_csv_records = campionamento_csv_check.expect("is_ok() was checked before");

    let campionamento_value_check =
        check_records_campionamento_niseci(campionamento_csv_records, &riferimento);

    assert!(campionamento_value_check.is_ok());

    let campionamento = campionamento_value_check.expect("is_ok() was checked before");

    let anagrafica_reader = Cursor::new(ANAGRAFICA_NISECI_TEMPLATE_DATA);

    let anagrafica_csv_check = check_anagrafica_niseci_reader_conf::<
        _,
        VeryItalianRecordAnagraficaNISECI,
    >(anagrafica_reader, config);

    assert!(anagrafica_csv_check.is_ok());

    let anagrafica_csv_records = anagrafica_csv_check.expect("is_ok() was checked before");

    let anagrafica_value_check = check_records_anagrafica_niseci(anagrafica_csv_records);

    assert!(anagrafica_value_check.is_ok());

    let anagrafica = anagrafica_value_check.expect("is_ok() was checked before");

    let calc_niseci_res = calculate_niseci(&campionamento, &riferimento, &anagrafica);

    assert!(calc_niseci_res.is_ok());

    let (niseci, intermediates) = calc_niseci_res.expect("is_ok() was checked before");

    assert_eq!(niseci, Some(0.209));
    assert_eq!(intermediates.x1(), 0.429);
    assert_eq!(intermediates.x2(), Some(0.267));
    assert_eq!(intermediates.x3(), 1.0);
}

fn calc_templates_with_area(
    has_headers: bool,
    format: InputFormat,
    x: f32,
    y: f32,
) -> Result<(Option<f32>, ValoriIntermediNISECI), Vec<String>> {
    let riferimento_reader = Cursor::new(RIFERIMENTO_NISECI_TEMPLATE_DATA);
    let riferimento = load_riferimento_niseci_from_reader(riferimento_reader, has_headers, format)
        .expect("Failed loading template RiferimentoNISECI");
    let campionamento_reader = Cursor::new(CAMPIONAMENTO_NISECI_TEMPLATE_DATA);
    let campionamento = load_campionamento_niseci_from_reader(
        campionamento_reader,
        has_headers,
        &riferimento,
        format,
    )
    .expect("Failed loading template CampionamentoNISECI");
    let anagrafica_reader = Cursor::new(ANAGRAFICA_NISECI_TEMPLATE_DATA);
    let mut anagrafica = load_anagrafica_niseci_from_reader(anagrafica_reader, has_headers, format)
        .expect("Failed loading template AnagraficaNISECI");
    anagrafica.set_lunghezza_unchecked(x);
    anagrafica.set_larghezza_unchecked(y);

    calculate_niseci(&campionamento, &riferimento, &anagrafica)
}

#[test]
fn calculate_niseci_template_zero_area() {
    let has_headers = true;
    let format = InputFormat::Alternative;
    let calc_niseci_res = calc_templates_with_area(has_headers, format, 0.0, 0.0);

    assert!(calc_niseci_res.is_err());
}

#[test]
fn calculate_niseci_template_subzero_area() {
    let has_headers = true;
    let format = InputFormat::Alternative;
    let calc_niseci_res = calc_templates_with_area(has_headers, format, -1.0, 100.0);

    assert!(calc_niseci_res.is_err());
}

#[test]
fn calculate_niseci_template_infinite_area() {
    let has_headers = true;
    let format = InputFormat::Alternative;
    let calc_niseci_res = calc_templates_with_area(has_headers, format, f32::INFINITY, 1.0);

    assert!(calc_niseci_res.is_err());
}

#[test]
fn calculate_niseci_template_subzero_infinite_area() {
    let has_headers = true;
    let format = InputFormat::Alternative;
    let calc_niseci_res = calc_templates_with_area(has_headers, format, f32::NEG_INFINITY, 1.0);

    assert!(calc_niseci_res.is_err());
}

#[test]
fn calculate_niseci_template_quietnan_area() {
    let has_headers = true;
    let format = InputFormat::Alternative;
    // From https://doc.rust-lang.org/std/primitive.f32.html#associatedconstant.NAN
    // This constant is guaranteed to be a quiet NaN (on targets that follow
    // the Rust assumptions that the quiet/signaling bit being set to 1
    // indicates a quiet NaN)
    let quiet_nan = f32::NAN;
    let calc_niseci_res = calc_templates_with_area(has_headers, format, quiet_nan, 1.0);

    assert!(calc_niseci_res.is_err());
}
