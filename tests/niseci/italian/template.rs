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

#[expect(deprecated)]
use esox::csv::deser::niseci::{
    check_anagrafica_niseci_reader, check_campionamento_niseci_reader,
    check_riferimento_niseci_reader,
};
#[expect(deprecated)]
use esox::csv::parser::niseci::{
    check_records_anagrafica_niseci, check_records_campionamento_niseci,
    check_records_riferimento_niseci,
};
use esox::csv::stanis::niseci::{
    VeryItalianRecordAnagraficaNISECI, VeryItalianRecordCampionamentoNISECI,
    VeryItalianRecordRiferimentoNISECI,
};

use esox::{
    domain::niseci::{CampionamentoNISECI, RiferimentoNISECI},
    engines::niseci::full::calculate_niseci,
};
use std::io::Cursor;

const RIFERIMENTO_NISECI_TEMPLATE_DATA: &[u8] =
    include_bytes!("../../../templates/riferimento_niseci.csv");
const CAMPIONAMENTO_NISECI_TEMPLATE_DATA: &[u8] =
    include_bytes!("../../../templates/campionamento_niseci.csv");
const ANAGRAFICA_NISECI_TEMPLATE_DATA: &[u8] =
    include_bytes!("../../../templates/anagrafica_niseci.csv");

#[test]
fn calculate_niseci_template() {
    let has_headers = true;
    let riferimento_reader = Cursor::new(RIFERIMENTO_NISECI_TEMPLATE_DATA);
    #[expect(deprecated)]
    let riferimento_csv_check = check_riferimento_niseci_reader::<
        _,
        VeryItalianRecordRiferimentoNISECI,
    >(riferimento_reader, has_headers);

    assert!(riferimento_csv_check.is_ok());

    let riferimento_csv_records = riferimento_csv_check.expect("is_ok() was checked before");

    #[expect(deprecated)]
    let riferimento_value_check = check_records_riferimento_niseci(riferimento_csv_records);

    assert!(riferimento_value_check.is_ok());

    let riferimento = riferimento_value_check.expect("is_ok() was checked before");

    let campionamento_reader = Cursor::new(CAMPIONAMENTO_NISECI_TEMPLATE_DATA);

    #[expect(deprecated)]
    let campionamento_csv_check = check_campionamento_niseci_reader::<
        _,
        VeryItalianRecordCampionamentoNISECI,
    >(campionamento_reader, has_headers);

    assert!(campionamento_csv_check.is_ok());

    let campionamento_csv_records = campionamento_csv_check.expect("is_ok() was checked before");

    #[expect(deprecated)]
    let campionamento_value_check =
        check_records_campionamento_niseci(campionamento_csv_records, riferimento.clone());

    assert!(campionamento_value_check.is_ok());

    let campionamento = campionamento_value_check.expect("is_ok() was checked before");

    let anagrafica_reader = Cursor::new(ANAGRAFICA_NISECI_TEMPLATE_DATA);

    #[expect(deprecated)]
    let anagrafica_csv_check = check_anagrafica_niseci_reader::<_, VeryItalianRecordAnagraficaNISECI>(
        anagrafica_reader,
        has_headers,
    );

    assert!(anagrafica_csv_check.is_ok());

    let anagrafica_csv_records = anagrafica_csv_check.expect("is_ok() was checked before");

    #[expect(deprecated)]
    let anagrafica_value_check = check_records_anagrafica_niseci(anagrafica_csv_records);

    assert!(anagrafica_value_check.is_ok());

    let anagrafica = anagrafica_value_check.expect("is_ok() was checked before");

    let calc_niseci_res = calculate_niseci(
        &CampionamentoNISECI::new(campionamento),
        &RiferimentoNISECI::new(riferimento),
        &anagrafica,
    );

    assert!(calc_niseci_res.is_ok());

    let (niseci, intermediates) = calc_niseci_res.expect("is_ok() was checked before");

    assert_eq!(niseci, Some(0.209));
    assert_eq!(intermediates.x1, 0.429);
    assert_eq!(intermediates.x2, Some(0.267));
    assert_eq!(intermediates.x3, 1.0);
}
