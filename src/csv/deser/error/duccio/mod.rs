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

use crate::csv::deser::error::{
    CsvDiagnosticFormatter, CsvLayout, CsvLocalization, CsvPositionFormatter, FieldResolver,
};
use crate::csv::deser::{
    translate_error_message, ANAGRAFICA_HFBI_HEADER_FIELDS, ANAGRAFICA_NISECI_HEADER_FIELDS,
    CAMPIONAMENTO_HFBI_HEADER_FIELDS, CAMPIONAMENTO_NISECI_HEADER_FIELDS,
    RIFERIMENTO_NISECI_HEADER_FIELDS,
};
use crate::deser::TipoRecord;

pub struct CsvItalian;

impl CsvLocalization for CsvItalian {
    fn io_error(&self) -> &'static str {
        "Errore I/O"
    }
    fn utf8_error(&self) -> &'static str {
        "Errore UTF-8"
    }
    fn unexpected_eof(&self) -> &'static str {
        "Fine riga inatteso"
    }
    fn parse_generic_error(&self) -> &'static str {
        "Errore di deserializzazione"
    }
    fn parse_utf8_error(&self) -> &'static str {
        "Errore di deserializzazione utf8"
    }
    fn parse_bool_error(&self) -> &'static str {
        "Errore di deserializzazione booleano"
    }
    fn parse_float_error(&self) -> &'static str {
        "Errore di deserializzazione numero decimale"
    }
    fn parse_int_error(&self) -> &'static str {
        "Errore di deserializzazione numero intero"
    }
    fn unsupported(&self) -> &'static str {
        "Formato non supportato"
    }
    fn unknown(&self) -> &'static str {
        "Errore sconosciuto"
    }
}

pub struct ItalianPosition;

impl CsvPositionFormatter for ItalianPosition {
    fn format(&self, pos: &Option<csv::Position>) -> String {
        let res;
        match pos {
            Some(p) => {
                // These should be equal. We may show the value only once if they are
                let line_offset = p.line();
                let record_offset = p.record();

                if line_offset == record_offset {
                    res = format!("Riga: {}", line_offset);
                } else {
                    // TODO: How can we hit this branch?
                    res = format!("Riga: {} Record: {}", line_offset, record_offset);
                }

                // We ignore this since I don't think users may care?
                // let byte_offset = p.byte();
                // res = format!("Riga: {} Record: {} Char: {} ", line_offset, record_offset, byte_offset);
            }
            None => {
                res = "none".to_string();
            }
        }
        res
    }
}

pub struct ItalianFieldResolver;

impl FieldResolver for ItalianFieldResolver {
    fn resolve(&self, record: TipoRecord, idx: usize) -> String {
        match record {
            TipoRecord::RiferimentoNISECI => RIFERIMENTO_NISECI_HEADER_FIELDS
                .get(idx)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "???".to_string()),

            TipoRecord::CampionamentoNISECI => CAMPIONAMENTO_NISECI_HEADER_FIELDS
                .get(idx)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "???".to_string()),

            TipoRecord::AnagraficaNISECI => ANAGRAFICA_NISECI_HEADER_FIELDS
                .get(idx)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "???".to_string()),

            TipoRecord::CampionamentoHFBI => CAMPIONAMENTO_HFBI_HEADER_FIELDS
                .get(idx)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "???".to_string()),

            TipoRecord::AnagraficaHFBI => ANAGRAFICA_HFBI_HEADER_FIELDS
                .get(idx)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "???".to_string()),
        }
    }
}

pub struct ItalianLayout;

impl CsvLayout for ItalianLayout {
    fn deserialize(&self, base: &str, pos: &str, field: &str, detail: &str) -> String {
        format!(
            "{base} ({pos}, campo {field}) {}",
            translate_error_message(detail)
        )
    }

    fn unequal_lengths(&self, pos: &str, expected: u64, found: u64) -> String {
        format!("({pos}) attesi {} trovati {}", expected, found)
    }

    fn io(&self, base: &str, pos: &str, detail: &str) -> String {
        format!("{} ({pos}): {}", base, translate_error_message(detail))
    }

    fn utf8(&self, base: &str, pos: &str, detail: &str) -> String {
        format!("{} ({pos}): {}", base, translate_error_message(detail))
    }

    fn unknown(&self, base: &str, pos: &str, detail: &str) -> String {
        format!("{} ({pos}): {}", base, translate_error_message(detail))
    }
}

pub(crate) type ItalianFormatter =
    CsvDiagnosticFormatter<CsvItalian, ItalianPosition, ItalianFieldResolver, ItalianLayout>;

impl ItalianFormatter {
    pub(crate) fn new() -> Self {
        Self {
            loc: CsvItalian,
            pos: ItalianPosition,
            field: ItalianFieldResolver,
            layout: ItalianLayout,
        }
    }
}

pub(crate) fn csv_error_handler(record: TipoRecord) -> impl Fn(&Vec<csv::Error>) {
    super::csv_error_handler(
        ItalianFormatter::new(),
        "Errori incontrati durante l'elaborazione csv".to_string(),
        record,
    )
}

pub fn format_csv_error(error: &csv::Error, record: TipoRecord) -> String {
    super::format_csv_error(ItalianFormatter::new(), error, record)
}

pub fn format_csv_errors(errors: &[csv::Error], record: TipoRecord) -> Vec<String> {
    errors.iter().map(|e| format_csv_error(e, record)).collect()
}
