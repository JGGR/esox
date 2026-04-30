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

use crate::csv::deser::{
    hfbi::{check_anagrafica_hfbi_reader_conf, check_campionamento_hfbi_reader_conf},
    niseci::{
        check_anagrafica_niseci_reader_conf, check_campionamento_niseci_reader_conf,
        check_riferimento_niseci_reader_conf,
    },
    CsvConfig,
};
use crate::csv::stanis::giorgio::format_csv_error;
use crate::csv::stanis::hfbi::{
    VeryItalianRecordAnagraficaHFBI, VeryItalianRecordCampionamentoHFBI,
};
use crate::csv::stanis::niseci::{
    VeryItalianRecordAnagraficaNISECI, VeryItalianRecordCampionamentoNISECI,
    VeryItalianRecordRiferimentoNISECI,
};
use crate::csv::{
    ANAGRAFICA_HFBI_HEADER, ANAGRAFICA_NISECI_HEADER, CAMPIONAMENTO_HFBI_HEADER,
    CAMPIONAMENTO_NISECI_HEADER, RIFERIMENTO_NISECI_HEADER,
};
use crate::deser::TipoRecord;
use std::io::Cursor;

#[test]
fn test_csv_riferimento_niseci_found_string_expect_int() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;abc;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader_conf::<_, VeryItalianRecordRiferimentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::RiferimentoNISECI);
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_riferimento_niseci_found_string_expect_float() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;abc;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader_conf::<_, VeryItalianRecordRiferimentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::RiferimentoNISECI);
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_riferimento_niseci_found_empty_string_expect_int() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader_conf::<_, VeryItalianRecordRiferimentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::RiferimentoNISECI);
    assert!(translated_error.contains("campo vuoto"));
}

#[test]
fn test_csv_riferimento_niseci_found_empty_string_expect_float() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader_conf::<_, VeryItalianRecordRiferimentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::RiferimentoNISECI);
    assert!(translated_error.contains("campo vuoto"));
}

#[test]
fn test_csv_riferimento_niseci_found_float_expect_int() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;1.0;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader_conf::<_, VeryItalianRecordRiferimentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::RiferimentoNISECI);
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_riferimento_niseci_err_found_cl_negative() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02
        Cervo;Cervus elaphus;abc;Italia;1;0;1;-10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02
        Cervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;1;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader_conf::<_, VeryItalianRecordRiferimentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::RiferimentoNISECI);
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_riferimento_niseci_lessfields() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader_conf::<_, VeryItalianRecordRiferimentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record

    let translated_error = format_csv_error(&errors[0], TipoRecord::RiferimentoNISECI);
    assert!(translated_error.contains("numero campi"));
}

#[test]
fn test_csv_campionamento_niseci_found_string_expect_int() {
    let csv_data = format!(
        "{}\n07/07/2019;2190627 Reno 390;1;BA;abc;152",
        CAMPIONAMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_niseci_reader_conf::<_, VeryItalianRecordCampionamentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::CampionamentoNISECI);
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_campionamento_niseci_found_empty_string_expect_int() {
    let csv_data = format!(
        "{}\n07/07/2019;2190627 Reno 390;1;BA;;152",
        CAMPIONAMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_niseci_reader_conf::<_, VeryItalianRecordCampionamentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::CampionamentoNISECI);
    assert!(translated_error.contains("campo vuoto"));
}

#[test]
fn test_csv_campionamento_niseci_found_float_expect_int() {
    let csv_data = format!(
        "{}\n07/07/2019;2190627 Reno 390;1;BA;2.75;152",
        CAMPIONAMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_niseci_reader_conf::<_, VeryItalianRecordCampionamentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::CampionamentoNISECI);
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_campionamento_niseci_lessfields() {
    let csv_data = format!(
        "{}\n07/07/2019;2190627 Reno 390;1;BA;2.75",
        CAMPIONAMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_niseci_reader_conf::<_, VeryItalianRecordCampionamentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record

    let translated_error = format_csv_error(&errors[0], TipoRecord::CampionamentoNISECI);
    assert!(translated_error.contains("numero campi"));
}

#[test]
fn test_csv_anagrafica_niseci_found_string_expect_int() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;abc;8;0;;;20;1;BACINO",
        ANAGRAFICA_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_niseci_reader_conf::<_, VeryItalianRecordAnagraficaNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::AnagraficaNISECI);
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_anagrafica_niseci_found_empty_string_expect_int() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;;8;0;;;20;1;BACINO",
        ANAGRAFICA_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_niseci_reader_conf::<_, VeryItalianRecordAnagraficaNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::AnagraficaNISECI);
    assert!(translated_error.contains("campo vuoto"));
}

#[test]
fn test_csv_anagrafica_niseci_found_float_expect_int() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;100;8;0,1;;;20;1;BACINO",
        ANAGRAFICA_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_niseci_reader_conf::<_, VeryItalianRecordAnagraficaNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::AnagraficaNISECI);
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_anagrafica_niseci_lessfields() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;100;8;0;;;20;1",
        ANAGRAFICA_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_niseci_reader_conf::<_, VeryItalianRecordAnagraficaNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record

    let translated_error = format_csv_error(&errors[0], TipoRecord::AnagraficaNISECI);
    assert!(translated_error.contains("numero campi"));
}

#[test]
fn test_csv_campionamento_hfbi_found_string_expect_int() {
    let csv_data = format!("{}\nAN;foo;240", CAMPIONAMENTO_HFBI_HEADER);
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_hfbi_reader_conf::<_, VeryItalianRecordCampionamentoHFBI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::CampionamentoHFBI);
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_campionamento_hfbi_found_empty_string_expect_int() {
    let csv_data = format!("{}\nAN;;240", CAMPIONAMENTO_HFBI_HEADER);
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_hfbi_reader_conf::<_, VeryItalianRecordCampionamentoHFBI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::CampionamentoHFBI);
    assert!(translated_error.contains("campo vuoto"));
}

#[test]
fn test_csv_campionamento_hfbi_found_float_expect_int() {
    let csv_data = format!("{}\nAN;2.75;240", CAMPIONAMENTO_HFBI_HEADER);
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_hfbi_reader_conf::<_, VeryItalianRecordCampionamentoHFBI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::CampionamentoHFBI);
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_campionamento_hfbi_lessfields() {
    let csv_data = format!("{}\nAN;2.75", CAMPIONAMENTO_HFBI_HEADER);
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_hfbi_reader_conf::<_, VeryItalianRecordCampionamentoHFBI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record

    let translated_error = format_csv_error(&errors[0], TipoRecord::CampionamentoHFBI);
    assert!(translated_error.contains("numero campi"));
}

#[test]
fn test_csv_anagrafica_hfbi_found_string_expect_int() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;abc;8;0;1;1",
        ANAGRAFICA_HFBI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_hfbi_reader_conf::<_, VeryItalianRecordAnagraficaHFBI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::AnagraficaHFBI);
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_anagrafica_hfbi_found_empty_string_expect_int() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;;8;0;1;1",
        ANAGRAFICA_HFBI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_hfbi_reader_conf::<_, VeryItalianRecordAnagraficaHFBI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::AnagraficaHFBI);
    assert!(translated_error.contains("campo vuoto"));
}

#[test]
fn test_csv_anagrafica_hfbi_found_float_expect_int() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;100;8;2.75;1;1",
        ANAGRAFICA_HFBI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_hfbi_reader_conf::<_, VeryItalianRecordAnagraficaHFBI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record
    let translated_error = format_csv_error(&errors[0], TipoRecord::AnagraficaHFBI);
    assert!(translated_error.contains("tipo non valido"));
}

#[test]
fn test_csv_anagrafica_hfbi_lessfields() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;2.5;8;0;1",
        ANAGRAFICA_HFBI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_hfbi_reader_conf::<_, VeryItalianRecordAnagraficaHFBI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_err());
    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1); // One invalid record

    let translated_error = format_csv_error(&errors[0], TipoRecord::AnagraficaHFBI);
    assert!(translated_error.contains("numero campi"));
}
