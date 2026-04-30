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

pub mod duccio;

use crate::deser::TipoRecord;
use csv::Position;
use std::num::IntErrorKind;

pub enum CsvDiagnostic {
    Deserialize {
        pos: Option<Position>,
        field: Option<usize>,
        diagnostic: DeserDiagnostic,
    },
    Io {
        pos: Option<Position>,
        message: String,
    },
    Utf8 {
        pos: Option<Position>,
        message: String,
    },
    UnequalLengths {
        pos: Option<Position>,
        expected: u64,
        found: u64,
    },
    Unknown {
        pos: Option<Position>,
        message: String,
    },
}

pub struct DeserDiagnostic {
    kind: DeserDiagnosticKind,
}

impl DeserDiagnostic {
    pub fn kind(&self) -> &DeserDiagnosticKind {
        &self.kind
    }
}

impl From<DeserDiagnosticKind> for DeserDiagnostic {
    fn from(kind: DeserDiagnosticKind) -> Self {
        Self { kind }
    }
}

pub enum DeserDiagnosticKind {
    UnexpectedEndOfRow,
    Message(String),
    Unsupported(String),
    Parse { diagnostic: ParseDiagnostic },
}

pub struct ParseDiagnostic {
    kind: ParseDiagnosticKind,
}

impl ParseDiagnostic {
    pub fn kind(&self) -> &ParseDiagnosticKind {
        &self.kind
    }
}

impl From<ParseDiagnosticKind> for ParseDiagnostic {
    fn from(kind: ParseDiagnosticKind) -> Self {
        Self { kind }
    }
}

pub enum ParseDiagnosticKind {
    Utf8(String),
    Bool(String),
    Int { diagnostic: ParseIntDiagnostic },
    Float(String),
}

pub struct ParseIntDiagnostic {
    kind: ParseIntDiagnosticKind,
}

impl ParseIntDiagnostic {
    pub fn kind(&self) -> &ParseIntDiagnosticKind {
        &self.kind
    }
}

impl From<ParseIntDiagnosticKind> for ParseIntDiagnostic {
    fn from(kind: ParseIntDiagnosticKind) -> Self {
        Self { kind }
    }
}

pub enum ParseIntDiagnosticKind {
    Empty(String),
    InvalidDigit(String),
    PosOverflow(String),
    NegOverflow(String),
    Zero(String),
    Unknown(String),
}

pub fn normalize(errors: &[csv::Error]) -> Vec<CsvDiagnostic> {
    errors.iter().map(normalize_one).collect()
}

pub fn normalize_one(error: &csv::Error) -> CsvDiagnostic {
    match error.kind() {
        csv::ErrorKind::Deserialize { pos, err } => {
            let diagnostic = match err.kind() {
                csv::DeserializeErrorKind::UnexpectedEndOfRow => {
                    DeserDiagnosticKind::UnexpectedEndOfRow
                }

                csv::DeserializeErrorKind::Message(m) => {
                    DeserDiagnosticKind::Message(m.to_string())
                }

                csv::DeserializeErrorKind::Unsupported(m) => {
                    DeserDiagnosticKind::Unsupported(m.to_string())
                }

                csv::DeserializeErrorKind::InvalidUtf8(e) => DeserDiagnosticKind::Parse {
                    diagnostic: ParseDiagnosticKind::Utf8(e.to_string()).into(),
                },

                csv::DeserializeErrorKind::ParseBool(e) => DeserDiagnosticKind::Parse {
                    diagnostic: ParseDiagnosticKind::Bool(e.to_string()).into(),
                },

                csv::DeserializeErrorKind::ParseInt(e) => {
                    let kind = match e.kind() {
                        IntErrorKind::Empty => ParseIntDiagnosticKind::Empty(e.to_string()),
                        IntErrorKind::InvalidDigit => {
                            ParseIntDiagnosticKind::InvalidDigit(e.to_string())
                        }
                        IntErrorKind::PosOverflow => {
                            ParseIntDiagnosticKind::PosOverflow(e.to_string())
                        }
                        IntErrorKind::NegOverflow => {
                            ParseIntDiagnosticKind::NegOverflow(e.to_string())
                        }
                        IntErrorKind::Zero => ParseIntDiagnosticKind::Zero(e.to_string()),
                        _ => ParseIntDiagnosticKind::Unknown(e.to_string()),
                    };
                    DeserDiagnosticKind::Parse {
                        diagnostic: ParseDiagnostic::from(ParseDiagnosticKind::Int {
                            diagnostic: kind.into(),
                        }),
                    }
                }

                csv::DeserializeErrorKind::ParseFloat(e) => DeserDiagnosticKind::Parse {
                    diagnostic: ParseDiagnosticKind::Float(e.to_string()).into(),
                },
            }
            .into();

            CsvDiagnostic::Deserialize {
                pos: pos.clone(),
                field: err.field().map(|f| f as usize),
                diagnostic,
            }
        }

        csv::ErrorKind::Io(e) => CsvDiagnostic::Io {
            pos: None,
            message: e.to_string(),
        },

        csv::ErrorKind::Utf8 { pos, err } => CsvDiagnostic::Utf8 {
            pos: pos.clone(),
            message: err.to_string(),
        },

        csv::ErrorKind::UnequalLengths {
            pos,
            expected_len,
            len,
        } => CsvDiagnostic::UnequalLengths {
            pos: pos.clone(),
            expected: *expected_len,
            found: *len,
        },

        _ => CsvDiagnostic::Unknown {
            pos: None,
            message: error.to_string(),
        },
    }
}

pub trait CsvDiagnosticLocalization {
    fn io_error(&self) -> &'static str;
    fn utf8_error(&self) -> &'static str;
    fn unexpected_eof(&self) -> &'static str;
    fn parse_generic_error(&self) -> &'static str;
    fn parse_utf8_error(&self) -> &'static str;
    fn parse_bool_error(&self) -> &'static str;
    fn parse_float_error(&self) -> &'static str;
    fn parse_int_error(&self) -> &'static str;
    fn unsupported(&self) -> &'static str;
    fn unknown(&self) -> &'static str;
}

pub trait CsvPositionFormatter {
    fn format(&self, pos: &Option<csv::Position>) -> String;
}

/*
pub struct SimplePosition;

impl CsvPositionFormatter for SimplePosition {
    fn format(&self, pos: &Option<csv::Position>) -> String {
        match pos {
            Some(p) => format!(
                "line={}, record={}, byte={}",
                p.line(),
                p.record(),
                p.byte()
            ),
            None => "unknown".to_string(),
        }
    }
}
*/

pub trait CsvFieldResolver {
    fn resolve(&self, record: TipoRecord, idx: usize) -> String;
}

pub trait CsvDiagnosticLayout {
    fn deserialize(&self, base: &str, pos: &str, field: &str, detail: &str) -> String;

    fn unequal_lengths(&self, pos: &str, expected: u64, found: u64) -> String;

    fn io(&self, base: &str, pos: &str, detail: &str) -> String;

    fn utf8(&self, base: &str, pos: &str, detail: &str) -> String;

    fn unknown(&self, base: &str, pos: &str, detail: &str) -> String;
}

pub struct CsvDiagnosticFormatter<L, P, F, T> {
    pub loc: L,
    pub pos: P,
    pub field: F,
    pub layout: T,
}

impl<L, P, F, T> CsvDiagnosticFormatter<L, P, F, T>
where
    L: CsvDiagnosticLocalization,
    P: CsvPositionFormatter,
    F: CsvFieldResolver,
    T: CsvDiagnosticLayout,
{
    pub fn format(&self, err: &CsvDiagnostic, record: TipoRecord) -> String {
        let pos = self.pos.format(match err {
            CsvDiagnostic::Deserialize { pos, .. } => pos,
            CsvDiagnostic::Io { pos, .. } => pos,
            CsvDiagnostic::Utf8 { pos, .. } => pos,
            CsvDiagnostic::UnequalLengths { pos, .. } => pos,
            CsvDiagnostic::Unknown { pos, .. } => pos,
        });

        match err {
            CsvDiagnostic::Deserialize {
                field, diagnostic, ..
            } => {
                let field = field
                    .map(|idx| self.field.resolve(record, idx))
                    .unwrap_or_else(|| "none".to_string());

                let base = match diagnostic.kind() {
                    DeserDiagnosticKind::UnexpectedEndOfRow => self.loc.unexpected_eof(),

                    DeserDiagnosticKind::Message(_) => self.loc.parse_generic_error(),

                    DeserDiagnosticKind::Unsupported(_) => self.loc.unsupported(),

                    DeserDiagnosticKind::Parse { diagnostic } => match diagnostic.kind() {
                        ParseDiagnosticKind::Utf8(_) => self.loc.parse_utf8_error(),
                        ParseDiagnosticKind::Bool(_) => self.loc.parse_bool_error(),
                        ParseDiagnosticKind::Float(_) => self.loc.parse_float_error(),
                        ParseDiagnosticKind::Int { diagnostic: _ } => self.loc.parse_int_error(),
                    },
                };

                let detail = match diagnostic.kind() {
                    DeserDiagnosticKind::Message(m) | DeserDiagnosticKind::Unsupported(m) => m,

                    DeserDiagnosticKind::UnexpectedEndOfRow => "",

                    DeserDiagnosticKind::Parse { diagnostic } => match diagnostic.kind() {
                        ParseDiagnosticKind::Utf8(m)
                        | ParseDiagnosticKind::Bool(m)
                        | ParseDiagnosticKind::Float(m) => m,
                        ParseDiagnosticKind::Int { diagnostic } => match diagnostic.kind() {
                            ParseIntDiagnosticKind::Empty(m)
                            | ParseIntDiagnosticKind::InvalidDigit(m)
                            | ParseIntDiagnosticKind::PosOverflow(m)
                            | ParseIntDiagnosticKind::NegOverflow(m)
                            | ParseIntDiagnosticKind::Zero(m)
                            | ParseIntDiagnosticKind::Unknown(m) => m,
                        },
                    },
                };

                self.layout.deserialize(base, &pos, &field, detail)
            }

            CsvDiagnostic::Io { message, .. } => self.layout.io(self.loc.io_error(), &pos, message),

            CsvDiagnostic::Utf8 { message, .. } => {
                self.layout.utf8(self.loc.utf8_error(), &pos, message)
            }

            CsvDiagnostic::UnequalLengths {
                expected, found, ..
            } => self.layout.unequal_lengths(&pos, *expected, *found),

            CsvDiagnostic::Unknown { message, .. } => {
                self.layout.unknown(self.loc.unknown(), &pos, message)
            }
        }
    }
}

/// Transitional private helper, used in csv::deser::{niseci,hfbi} as the closure argument for
/// validate_serialized_records(), to print italian error messages.
/// v0.2 will drop this implicit logging, hence this method will not be needed anymore.
fn csv_error_handler<L, P, Fld, Lay>(
    formatter: CsvDiagnosticFormatter<L, P, Fld, Lay>,
    header: String,
    record: TipoRecord,
) -> impl Fn(&Vec<csv::Error>)
where
    L: CsvDiagnosticLocalization,
    P: CsvPositionFormatter,
    Fld: CsvFieldResolver,
    Lay: CsvDiagnosticLayout,
{
    move |errors: &Vec<csv::Error>| {
        let diagnostics = normalize(errors);

        eprintln!("{header} {{");

        for d in diagnostics {
            eprintln!("  {}", formatter.format(&d, record));
        }

        eprintln!("}}");
    }
}

pub fn format_csv_error<L, P, Fld, Lay>(
    formatter: CsvDiagnosticFormatter<L, P, Fld, Lay>,
    error: &csv::Error,
    record: TipoRecord,
) -> String
where
    L: CsvDiagnosticLocalization,
    P: CsvPositionFormatter,
    Fld: CsvFieldResolver,
    Lay: CsvDiagnosticLayout,
{
    let d = normalize_one(error);
    formatter.format(&d, record)
}

pub fn format_csv_errors<L, P, Fld, Lay>(
    formatter: CsvDiagnosticFormatter<L, P, Fld, Lay>,
    errors: &[csv::Error],
    record: TipoRecord,
) -> Vec<String>
where
    L: CsvDiagnosticLocalization,
    P: CsvPositionFormatter,
    Fld: CsvFieldResolver,
    Lay: CsvDiagnosticLayout,
{
    errors
        .iter()
        .map(|e| formatter.format(&normalize_one(e), record))
        .collect()
}
