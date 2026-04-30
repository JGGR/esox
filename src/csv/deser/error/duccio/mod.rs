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
    format_csv_error as generic_format_csv_error, CsvDiagnosticFormatter, CsvDiagnosticLayout,
    CsvDiagnosticLocalization, CsvFieldResolver, CsvPositionFormatter,
};
use crate::csv::deser::{
    ANAGRAFICA_HFBI_HEADER_FIELDS, ANAGRAFICA_NISECI_HEADER_FIELDS,
    CAMPIONAMENTO_HFBI_HEADER_FIELDS, CAMPIONAMENTO_NISECI_HEADER_FIELDS,
    RIFERIMENTO_NISECI_HEADER_FIELDS,
};
use crate::deser::TipoRecord;

pub struct CsvDiagnosticItalian;

impl CsvDiagnosticLocalization for CsvDiagnosticItalian {
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

pub struct ItalianCsvPosition;

impl CsvPositionFormatter for ItalianCsvPosition {
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

pub struct ItalianCsvFieldResolver;

impl CsvFieldResolver for ItalianCsvFieldResolver {
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

pub(crate) fn translate_error_message(msg: &str) -> String {
    if msg.starts_with("missing field") {
        msg.replace("missing field", "campo mancante")
    } else if msg.starts_with("invalid type") {
        msg.replace("invalid type", "tipo non valido")
    } else if msg.starts_with("unexpected end of input") {
        msg.replace("unexpected end of input", "fine inaspettata dell'input")
    } else if msg.contains("invalid UTF-8 sequence") {
        msg.replace("invalid UTF-8 sequence", "sequenza UTF-8 non valida")
    } else if msg.contains("file not found") {
        msg.replace("file not found", "file non trovato")
    } else if msg.contains("invalid digit found in string") {
        msg.replace(
            "invalid digit found in string",
            "tipo non valido: numero, attesa stringa",
        )
        .replace("field", "campo")
    } else if msg.contains("invalid float literal") {
        msg.replace("invalid float literal", "tipo non valido: atteso decimale")
            .replace("field", "campo")
    } else if msg.contains("cannot parse") && msg.contains("from empty string") {
        // NOTE: there's a leading space in " from empty string", it enables us to attach the ","
        // to the previous part
        msg.replace("cannot parse", "campo vuoto: atteso")
            .replace("field", "campo")
            .replace("float", "decimale")
            .replace("integer", "intero")
            .replace(" from empty string", ", trovato: stringa vuota")
    } else if msg.contains("fields, but the previous record has") {
        msg.replace("found record with", "numero campi: trovato record con")
            .replace("but the previous record has", "ma il record precedente ha")
            .replace("fields", "campi")
    } else {
        eprintln!("Unmatched translation for {msg}");
        msg.to_string() // Default to original message if no match
    }
}

pub struct ItalianCsvDiagnosticLayout;

impl CsvDiagnosticLayout for ItalianCsvDiagnosticLayout {
    fn deserialize(&self, base: &str, pos: &str, field: &str, detail: &str) -> String {
        format!(
            "{base} ({pos}, campo {field}) {}",
            translate_error_message(detail)
        )
    }

    fn unequal_lengths(&self, pos: &str, expected: u64, found: u64) -> String {
        format!(
            "Errore numero campi alla posizione ({pos}) attesi {} trovati {}",
            expected, found
        )
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

pub(crate) type ItalianDiagnosticFormatter = CsvDiagnosticFormatter<
    CsvDiagnosticItalian,
    ItalianCsvPosition,
    ItalianCsvFieldResolver,
    ItalianCsvDiagnosticLayout,
>;

impl ItalianDiagnosticFormatter {
    pub(crate) fn new() -> Self {
        Self {
            loc: CsvDiagnosticItalian,
            pos: ItalianCsvPosition,
            field: ItalianCsvFieldResolver,
            layout: ItalianCsvDiagnosticLayout,
        }
    }
}

/// Transitional private helper, used in csv::deser::{niseci,hfbi} as the closure argument for
/// validate_serialized_records(), to print italian error messages.
/// v0.2 will drop this implicit logging, hence this method will not be needed anymore.
pub(crate) fn csv_error_handler(record: TipoRecord) -> impl Fn(&Vec<csv::Error>) {
    super::csv_error_handler(
        ItalianDiagnosticFormatter::new(),
        "Errori incontrati durante l'elaborazione csv".to_string(),
        record,
    )
}

pub fn format_csv_error(error: &csv::Error, record: TipoRecord) -> String {
    generic_format_csv_error(ItalianDiagnosticFormatter::new(), error, record)
}

pub fn format_csv_errors(errors: &[csv::Error], record: TipoRecord) -> Vec<String> {
    errors.iter().map(|e| format_csv_error(e, record)).collect()
}
