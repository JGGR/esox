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
use crate::csv::load::hfbi::{
    load_anagrafica_hfbi_from_reader, load_campionamento_hfbi_from_reader,
};
use crate::csv::load::niseci::{
    load_anagrafica_niseci_from_reader, load_campionamento_niseci_from_reader,
    load_riferimento_niseci_from_reader,
};
use crate::csv::load::InputFormat;
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
use crate::domain::niseci::{RiferimentoNISECI, SpecieNISECI};
use crate::parser::hfbi::{check_records_anagrafica_hfbi, check_records_campionamento_hfbi};
use crate::parser::niseci::{
    check_records_anagrafica_niseci, check_records_campionamento_niseci,
    check_records_riferimento_niseci,
};
use crate::tests::test_utils::{
    ANAGRAFICA_HFBI_TEMPLATE_DATA, ANAGRAFICA_NISECI_TEMPLATE_DATA,
    CAMPIONAMENTO_HFBI_TEMPLATE_DATA, CAMPIONAMENTO_NISECI_TEMPLATE_DATA,
    RIFERIMENTO_NISECI_TEMPLATE_DATA,
};
use std::io::Cursor;

#[test]
fn test_valid_csv_riferimento_niseci() {
    let csv_data = format!(
        "{}\nCervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02
        Cervo;Cervus elaphus;abc;Italia;1;0;1;10;20;30;40;0.1;0.2;0.3;0.4;0.01;0.02
        Cervo;Cervus elaphus;1234;Italia;1;0;1;10;20;30;40;1;0.2;0.3;0.4;0.01;0.02",
        RIFERIMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader_conf::<_, VeryItalianRecordRiferimentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(!result.is_err());
}

#[test]
fn test_empty_csv_riferimento_niseci() {
    let csv_data = RIFERIMENTO_NISECI_HEADER.to_string(); // Only header, no data
    let reader = Cursor::new(csv_data);
    let result = check_riferimento_niseci_reader_conf::<_, VeryItalianRecordRiferimentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_ok());
    let records = result.unwrap();
    assert_eq!(records.len(), 0); // No records
}

#[test]
fn test_valid_recordcsv_riferimento_niseci() {
    let record_1 = VeryItalianRecordRiferimentoNISECI {
        nome_comune: "Cervo".to_string(),
        nome_latino: "Cervus elaphus".to_string(),
        codice_specie: "1234".to_string(),
        origine: "AUT".to_string(),
        tipo_autoctono: 1,
        allo_nocivita: 0,
        specie_attesa: 1,
        cl_soglia1: 10,
        cl_soglia2: 20,
        cl_soglia3: 30,
        cl_soglia4: 40,
        ad_juv_soglia1: 0.1,
        ad_juv_soglia2: 0.2,
        ad_juv_soglia3: 0.3,
        ad_juv_soglia4: 0.4,
        dens_soglia1: 0.1,
        dens_soglia2: 0.2,
    };
    let recordcsv_data = vec![record_1];
    let result = check_records_riferimento_niseci(recordcsv_data);

    assert!(!result.is_err());
}

#[test]
fn test_recordcsv_riferimento_niseci_soglie_cl_error() {
    let record_1 = VeryItalianRecordRiferimentoNISECI {
        nome_comune: "Cervo".to_string(),
        nome_latino: "Cervus elaphus".to_string(),
        codice_specie: "1234".to_string(),
        origine: "AUT".to_string(),
        tipo_autoctono: 1,
        allo_nocivita: 0,
        specie_attesa: 1,
        cl_soglia1: 10,
        cl_soglia2: 50,
        cl_soglia3: 30,
        cl_soglia4: 40,
        ad_juv_soglia1: 0.1,
        ad_juv_soglia2: 0.2,
        ad_juv_soglia3: 0.3,
        ad_juv_soglia4: 0.4,
        dens_soglia1: 0.1,
        dens_soglia2: 0.2,
    };
    let recordcsv_data = vec![record_1];
    let result = check_records_riferimento_niseci(recordcsv_data);

    assert!(result.is_err());

    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_recordcsv_riferimento_niseci_soglie_ad_juv_error() {
    let record_1 = VeryItalianRecordRiferimentoNISECI {
        nome_comune: "Cervo".to_string(),
        nome_latino: "Cervus elaphus".to_string(),
        codice_specie: "1234".to_string(),
        origine: "AUT".to_string(),
        tipo_autoctono: 1,
        allo_nocivita: 0,
        specie_attesa: 1,
        cl_soglia1: 10,
        cl_soglia2: 20,
        cl_soglia3: 30,
        cl_soglia4: 40,
        ad_juv_soglia1: 0.1,
        ad_juv_soglia2: 0.7,
        ad_juv_soglia3: 0.3,
        ad_juv_soglia4: 0.4,
        dens_soglia1: 0.1,
        dens_soglia2: 0.2,
    };
    let recordcsv_data = vec![record_1];
    let result = check_records_riferimento_niseci(recordcsv_data);

    assert!(result.is_err());

    let errors = result.err().unwrap();
    assert_eq!(errors.len(), 1);
}

#[test]
fn test_load_riferimento_niseci() {
    let riferimento_reader = Cursor::new(RIFERIMENTO_NISECI_TEMPLATE_DATA);
    let riferimento = load_riferimento_niseci_from_reader::<_>(
        riferimento_reader,
        true,
        InputFormat::Alternative,
    );
    assert!(riferimento.is_ok());
}

#[test]
fn test_valid_csv_campionamento_niseci() {
    let csv_data = format!(
        "{}\n07/07/2019;2190627 Reno 390;1;BA;275;152
        07/07/2019;2190627 Reno 390;1;BA;275;152
        abc;2190627 Reno 390;1;BA;275;152",
        CAMPIONAMENTO_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_niseci_reader_conf::<_, VeryItalianRecordCampionamentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(!result.is_err());
}

#[test]
fn test_empty_csv_campionamento_niseci() {
    let csv_data = CAMPIONAMENTO_NISECI_HEADER.to_string(); // Only header, no data
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_niseci_reader_conf::<_, VeryItalianRecordCampionamentoNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_ok());
    let records = result.unwrap();
    assert_eq!(records.len(), 0); // No records
}

#[test]
fn test_valid_recordcsv_campionamento_niseci() {
    let specie_1 = SpecieNISECI::new(
        "1234",
        "Cervus elaphus",
        1,
        0,
        true,
        0, // in cm
        0, // in cm
        0, // in cm
        0, // in cm
        0.0,
        0.0,
        0.0,
        0.0,
        1.0,
        2.0,
    );

    let riferimento_specie = vec![specie_1];

    let record_1 = VeryItalianRecordCampionamentoNISECI {
        data: "07/07/2007".to_string(),
        stazione: "Foo".to_string(),
        num_passaggio: 1,
        codice_specie: "1234".to_string(),
        lunghezza: 100,
        peso: 100.0,
    };
    let recordcsv_data = vec![record_1];
    let result = check_records_campionamento_niseci(
        recordcsv_data,
        &RiferimentoNISECI::new_from_map(riferimento_specie.into()),
    );

    assert!(!result.is_err());
}

#[test]
fn test_load_campionamento_niseci() {
    let riferimento_reader = Cursor::new(RIFERIMENTO_NISECI_TEMPLATE_DATA);
    let riferimento = load_riferimento_niseci_from_reader::<_>(
        riferimento_reader,
        true,
        InputFormat::Alternative,
    );
    assert!(riferimento.is_ok());
    let riferimento = riferimento.unwrap();
    let campionamento_reader = Cursor::new(CAMPIONAMENTO_NISECI_TEMPLATE_DATA);
    let campionamento = load_campionamento_niseci_from_reader::<_>(
        campionamento_reader,
        true,
        &riferimento,
        InputFormat::Alternative,
    );
    assert!(campionamento.is_ok());
}

#[test]
fn test_valid_csv_anagrafica_niseci() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;100;8;0;;;20;1;BACINO",
        ANAGRAFICA_NISECI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_niseci_reader_conf::<_, VeryItalianRecordAnagraficaNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(!result.is_err());
}

#[test]
fn test_empty_csv_anagrafica_niseci() {
    let csv_data = ANAGRAFICA_NISECI_HEADER.to_string(); // Only header, no data
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_niseci_reader_conf::<_, VeryItalianRecordAnagraficaNISECI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_ok());
    let records = result.unwrap();
    assert_eq!(records.len(), 0); // No records
}

#[test]
fn test_valid_recordcsv_anagrafica_niseci() {
    let record_1 = VeryItalianRecordAnagraficaNISECI {
        codice_stazione: "Foo".to_string(),
        corpo_idrico: "Bar".to_string(),
        regione: "Foo".to_string(),
        provincia: "Bar".to_string(),
        data: "07/07/2007".to_string(),
        lunghezza_stazione: 100.0,
        larghezza_stazione: 6.0,
        tipo_comunita: 0,
        fonte: "foo".to_string(),
        numero_protocollo: "foo".to_string(),
        idro_eco_regione: 0,
        area_alpina: 0,
        nome_bacino: "Foo".to_string(),
    };
    let recordcsv_data = vec![record_1];
    let result = check_records_anagrafica_niseci(recordcsv_data);

    assert!(!result.is_err());
}

#[test]
fn test_load_anagrafica_niseci() {
    let anagrafica_reader = Cursor::new(ANAGRAFICA_NISECI_TEMPLATE_DATA);
    let anagrafica =
        load_anagrafica_niseci_from_reader::<_>(anagrafica_reader, true, InputFormat::Alternative);
    assert!(anagrafica.is_ok());
}

#[test]
fn test_valid_csv_campionamento_hfbi() {
    let csv_data = format!("{}\nAN;25;240", CAMPIONAMENTO_HFBI_HEADER);
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_hfbi_reader_conf::<_, VeryItalianRecordCampionamentoHFBI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(!result.is_err());
}

#[test]
fn test_empty_csv_campionamento_hfbi() {
    let csv_data = CAMPIONAMENTO_HFBI_HEADER.to_string(); // Only header, no data
    let reader = Cursor::new(csv_data);
    let result = check_campionamento_hfbi_reader_conf::<_, VeryItalianRecordCampionamentoHFBI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_ok());
    let records = result.unwrap();
    assert_eq!(records.len(), 0); // No records
}

#[test]
fn test_valid_recordcsv_campionamento_hfbi() {
    let record_1 = VeryItalianRecordCampionamentoHFBI {
        codice_specie: "AN".to_string(),
        peso: 100.0,
        numero_individui: 10,
    };
    let recordcsv_data = vec![record_1];
    let result = check_records_campionamento_hfbi(recordcsv_data);

    assert!(!result.is_err());
}

#[test]
fn test_load_campionamento_hfbi() {
    let campionamento_reader = Cursor::new(CAMPIONAMENTO_HFBI_TEMPLATE_DATA);
    let campionamento = load_campionamento_hfbi_from_reader::<_>(
        campionamento_reader,
        true,
        InputFormat::Alternative,
    );
    assert!(campionamento.is_ok());
}

#[test]
fn test_valid_csv_anagrafica_hfbi() {
    let csv_data = format!(
        "{}\nCODICE;CORPO;REGIONE;PROVINCIA;1/1/1111;100;8;0;1;1",
        ANAGRAFICA_HFBI_HEADER
    );
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_hfbi_reader_conf::<_, VeryItalianRecordAnagraficaHFBI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(!result.is_err());
}

#[test]
fn test_empty_csv_anagrafica_hfbi() {
    let csv_data = ANAGRAFICA_HFBI_HEADER.to_string(); // Only header, no data
    let reader = Cursor::new(csv_data);
    let result = check_anagrafica_hfbi_reader_conf::<_, VeryItalianRecordAnagraficaHFBI>(
        reader,
        CsvConfig::default().with_delimiter(b';'),
    );

    assert!(result.is_ok());
    let records = result.unwrap();
    assert_eq!(records.len(), 0); // No records
}

#[test]
fn test_valid_recordcsv_anagrafica_hfbi() {
    let record_1 = VeryItalianRecordAnagraficaHFBI {
        codice_stazione: "Foo".to_string(),
        corpo_idrico: "Bar".to_string(),
        regione: "Foo".to_string(),
        provincia: "Bar".to_string(),
        data: "07/07/2007".to_string(),
        lunghezza_stazione: 100.0,
        larghezza_stazione: 6.0,
        stagione: 0,
        habitat: 1,
        tipo_laguna: 1,
    };
    let recordcsv_data = vec![record_1];
    let result = check_records_anagrafica_hfbi(recordcsv_data);

    assert!(!result.is_err());
}

#[test]
fn test_load_anagrafica_hfbi() {
    let anagrafica_reader = Cursor::new(ANAGRAFICA_HFBI_TEMPLATE_DATA);
    let anagrafica =
        load_anagrafica_hfbi_from_reader::<_>(anagrafica_reader, true, InputFormat::Alternative);
    assert!(anagrafica.is_ok());
}
