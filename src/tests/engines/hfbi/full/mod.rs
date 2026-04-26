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
use crate::csv::deser::hfbi::{
    check_anagrafica_hfbi_reader_conf, check_campionamento_hfbi_reader_conf,
};
use crate::csv::deser::CsvConfig;
use crate::csv::load::{
    hfbi::{load_anagrafica_hfbi_from_reader, load_campionamento_hfbi_from_reader},
    InputFormat,
};
use crate::csv::stanis::hfbi::{
    VeryItalianRecordAnagraficaHFBI, VeryItalianRecordCampionamentoHFBI,
};
use crate::domain::hfbi::ValoriIntermediHFBI;
use crate::engines::hfbi::full::calculate_hfbi;
use crate::parser::hfbi::{check_records_anagrafica_hfbi, check_records_campionamento_hfbi};
use crate::tests::test_utils::{ANAGRAFICA_HFBI_TEMPLATE_DATA, CAMPIONAMENTO_HFBI_TEMPLATE_DATA};
use std::io::Cursor;

#[test]
fn calculate_hfbi_template() {
    let config = CsvConfig::default().with_delimiter(b';');
    let campionamento_reader = Cursor::new(CAMPIONAMENTO_HFBI_TEMPLATE_DATA);

    let campionamento_csv_check = check_campionamento_hfbi_reader_conf::<
        _,
        VeryItalianRecordCampionamentoHFBI,
    >(campionamento_reader, config);

    assert!(campionamento_csv_check.is_ok());

    let campionamento_csv_records = campionamento_csv_check.expect("is_ok() was checked before");

    let campionamento_value_check = check_records_campionamento_hfbi(campionamento_csv_records);

    assert!(campionamento_value_check.is_ok());

    let campionamento = campionamento_value_check.expect("is_ok() was checked before");

    let anagrafica_reader = Cursor::new(ANAGRAFICA_HFBI_TEMPLATE_DATA);

    let anagrafica_csv_check = check_anagrafica_hfbi_reader_conf::<
        _,
        VeryItalianRecordAnagraficaHFBI,
    >(anagrafica_reader, config);

    assert!(anagrafica_csv_check.is_ok());

    let anagrafica_csv_records = anagrafica_csv_check.expect("is_ok() was checked before");

    let anagrafica_value_check = check_records_anagrafica_hfbi(anagrafica_csv_records);

    assert!(anagrafica_value_check.is_ok());

    let anagrafica = anagrafica_value_check.expect("is_ok() was checked before");

    let calc_hfbi_res = calculate_hfbi(&campionamento, &anagrafica);

    assert!(calc_hfbi_res.is_ok());

    let (hfbi, intermediates) = calc_hfbi_res.expect("is_ok() was checked before");

    assert_eq!(hfbi, 1.167);
    assert_eq!(intermediates.mmi, 0.342);
    assert_eq!(intermediates.bbent, 3.638);
    assert_eq!(intermediates.bn, 1.587);
    assert_eq!(intermediates.dbent, 0.154);
    assert_eq!(intermediates.ddom, 0.399);
    assert_eq!(intermediates.dhzp, 0.417);
    assert_eq!(intermediates.dmig, 0.421);
}

fn calc_templates_with_area(
    has_headers: bool,
    format: InputFormat,
    x: f32,
    y: f32,
) -> Result<(f32, ValoriIntermediHFBI), String> {
    let campionamento_reader = Cursor::new(CAMPIONAMENTO_HFBI_TEMPLATE_DATA);
    let campionamento =
        load_campionamento_hfbi_from_reader(campionamento_reader, has_headers, format)
            .expect("Failed loading template CampionamentoHFBI");
    let anagrafica_reader = Cursor::new(ANAGRAFICA_HFBI_TEMPLATE_DATA);
    let mut anagrafica = load_anagrafica_hfbi_from_reader(anagrafica_reader, has_headers, format)
        .expect("Failed loading template AnagraficaHFBI");
    anagrafica.set_lunghezza_unchecked(x);
    anagrafica.set_larghezza_unchecked(y);

    calculate_hfbi(&campionamento, &anagrafica)
}

#[test]
fn calculate_hfbi_template_zero_area() {
    let has_headers = true;
    let format = InputFormat::Alternative;

    let calc_hfbi_res = calc_templates_with_area(has_headers, format, 0.0, 0.0);

    assert!(calc_hfbi_res.is_err());
}

#[test]
fn calculate_hfbi_template_subzero_area() {
    let has_headers = true;
    let format = InputFormat::Alternative;

    let calc_hfbi_res = calc_templates_with_area(has_headers, format, -1.0, 100.0);

    assert!(calc_hfbi_res.is_err());
}

#[test]
fn calculate_hfbi_template_infinite_area() {
    let has_headers = true;
    let format = InputFormat::Alternative;

    let calc_hfbi_res = calc_templates_with_area(has_headers, format, f32::INFINITY, 1.0);

    assert!(calc_hfbi_res.is_err());
}

#[test]
fn calculate_hfbi_template_subzero_infinite_area() {
    let has_headers = true;
    let format = InputFormat::Alternative;

    let calc_hfbi_res = calc_templates_with_area(has_headers, format, f32::NEG_INFINITY, 1.0);

    assert!(calc_hfbi_res.is_err());
}

#[test]
fn calculate_hfbi_template_quietnan_area() {
    let has_headers = true;
    let format = InputFormat::Alternative;

    // From https://doc.rust-lang.org/std/primitive.f32.html#associatedconstant.NAN
    // This constant is guaranteed to be a quiet NaN (on targets that follow
    // the Rust assumptions that the quiet/signaling bit being set to 1
    // indicates a quiet NaN)
    let quiet_nan = f32::NAN;
    let calc_hfbi_res = calc_templates_with_area(has_headers, format, quiet_nan, 1.0);

    assert!(calc_hfbi_res.is_err());
}
