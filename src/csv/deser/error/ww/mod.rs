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

pub struct EngCsvDiagnostic;

impl CsvDiagnosticLocalization for EngCsvDiagnostic {
    fn io_error(&self) -> &'static str {
        "I/O error"
    }
    fn utf8_error(&self) -> &'static str {
        "UTF-8 error"
    }
    fn unexpected_eof(&self) -> &'static str {
        "Unexpected EOF"
    }
    fn parse_generic_error(&self) -> &'static str {
        "Generic deserialization error"
    }
    fn parse_utf8_error(&self) -> &'static str {
        "UTF-8 deserialzsation error"
    }
    fn parse_bool_error(&self) -> &'static str {
        "Bool deserialization error"
    }
    fn parse_float_error(&self) -> &'static str {
        "Float deserialization error"
    }
    fn parse_int_error(&self) -> &'static str {
        "Integer deserialization error"
    }
    fn unsupported(&self) -> &'static str {
        "Unsupported format"
    }
    fn unknown(&self) -> &'static str {
        "Unknown error"
    }
}

pub struct EngCsvPosition;

impl CsvPositionFormatter for EngCsvPosition {
    fn format(&self, pos: &Option<csv::Position>) -> String {
        match pos {
            Some(p) => format!(
                "line {}, record {}, byte {}",
                p.line(),
                p.record(),
                p.byte()
            ),
            None => "unknown".to_string(),
        }
    }
}

pub struct EngCsvFieldResolver;

impl CsvFieldResolver for EngCsvFieldResolver {
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

pub struct EngCsvDiagnosticLayout;

impl CsvDiagnosticLayout for EngCsvDiagnosticLayout {
    fn deserialize(&self, base: &str, pos: &str, field: &str, detail: &str) -> String {
        format!("{base} ({pos}, field {field}) {}", detail)
    }

    fn unequal_lengths(&self, pos: &str, expected: u64, found: u64) -> String {
        format!(
            "Number of fields error at ({pos}) expected {} found {}",
            expected, found
        )
    }

    fn io(&self, base: &str, pos: &str, detail: &str) -> String {
        format!("{} ({pos}): {}", base, detail)
    }

    fn utf8(&self, base: &str, pos: &str, detail: &str) -> String {
        format!("{} ({pos}): {}", base, detail)
    }

    fn unknown(&self, base: &str, pos: &str, detail: &str) -> String {
        format!("{} ({pos}): {}", base, detail)
    }
}

pub(crate) type EngDiagnosticFormatter = CsvDiagnosticFormatter<
    EngCsvDiagnostic,
    EngCsvPosition,
    EngCsvFieldResolver,
    EngCsvDiagnosticLayout,
>;

impl EngDiagnosticFormatter {
    pub(crate) fn new() -> Self {
        Self {
            loc: EngCsvDiagnostic,
            pos: EngCsvPosition,
            field: EngCsvFieldResolver,
            layout: EngCsvDiagnosticLayout,
        }
    }
}

pub fn format_csv_error(error: &csv::Error, record: TipoRecord) -> String {
    generic_format_csv_error(EngDiagnosticFormatter::new(), error, record)
}

pub fn format_csv_errors(errors: &[csv::Error], record: TipoRecord) -> Vec<String> {
    errors.iter().map(|e| format_csv_error(e, record)).collect()
}
