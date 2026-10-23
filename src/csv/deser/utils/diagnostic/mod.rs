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

//! Types for custom formatting of CSV errors.
//! The [`ww`] module provides a default english implementation of the provided interface.

pub mod ww;

use crate::deser::TipoRecord;
use csv::Position;
use std::num::IntErrorKind;

/// Defines the different possible CSV errors.
pub enum CsvDiagnostic {
    /// Deserialize error.
    Deserialize {
        /// Error position in input source.
        pos: Option<Position>,
        /// Field index in input source.
        field: Option<usize>,
        /// Kind of diagnostic.
        diagnostic: DeserDiagnostic,
    },
    /// I/O error.
    Io {
        /// Error position in input source.
        pos: Option<Position>,
        /// Error message.
        message: String,
    },
    /// UTF-8 error.
    Utf8 {
        /// Error position in input source.
        pos: Option<Position>,
        /// Error message.
        message: String,
    },
    /// Unequal record length error.
    UnequalLengths {
        /// Error position in input source.
        pos: Option<Position>,
        /// Expected number of fields.
        expected: u64,
        /// Found number of fields.
        found: u64,
    },
    Unknown {
        /// Error position in input source.
        pos: Option<Position>,
        /// Error message.
        message: String,
    },
}

/// Defines a CSV deserialization diagnostics.
pub struct DeserDiagnostic {
    kind: DeserDiagnosticKind,
}

impl DeserDiagnostic {
    /// Getter for [`DeserDiagnosticKind`].
    pub fn kind(&self) -> &DeserDiagnosticKind {
        &self.kind
    }
}

impl From<DeserDiagnosticKind> for DeserDiagnostic {
    fn from(kind: DeserDiagnosticKind) -> Self {
        Self { kind }
    }
}

/// Defines a CSV deserialization diagnostics kind.
pub enum DeserDiagnosticKind {
    /// Unexpected end of row, missing fields.
    UnexpectedEndOfRow,
    /// Custom message.
    Message(String),
    /// Unsupported error.
    Unsupported(String),
    /// Parse error.
    Parse { diagnostic: ParseDiagnostic },
}

/// Defines a CSV parsing diagnostics.
pub struct ParseDiagnostic {
    kind: ParseDiagnosticKind,
}

impl ParseDiagnostic {
    /// Getter for [`ParseDiagnosticKind`].
    pub fn kind(&self) -> &ParseDiagnosticKind {
        &self.kind
    }
}

impl From<ParseDiagnosticKind> for ParseDiagnostic {
    fn from(kind: ParseDiagnosticKind) -> Self {
        Self { kind }
    }
}

/// Defines a CSV parsing diagnostics kind.
pub enum ParseDiagnosticKind {
    /// UTF-8 parse error.
    Utf8(String),
    /// Bool parse error.
    Bool(String),
    /// Int parse error.
    Int { diagnostic: ParseIntDiagnostic },
    /// Float parse error.
    Float(String),
}

/// Defines a CSV int parsing diagnostics.
pub struct ParseIntDiagnostic {
    kind: ParseIntDiagnosticKind,
}

impl ParseIntDiagnostic {
    /// Getter for [`ParseIntDiagnosticKind`].
    pub fn kind(&self) -> &ParseIntDiagnosticKind {
        &self.kind
    }
}

impl From<ParseIntDiagnosticKind> for ParseIntDiagnostic {
    fn from(kind: ParseIntDiagnosticKind) -> Self {
        Self { kind }
    }
}

/// Defines a CSV int parsing diagnostics kind.
pub enum ParseIntDiagnosticKind {
    /// Empty input error.
    Empty(String),
    /// Invalid digit error.
    InvalidDigit(String),
    /// Positive overflow error.
    PosOverflow(String),
    /// Negative overflow error.
    NegOverflow(String),
    /// Zero error.
    Zero(String),
    /// Unknown error.
    Unknown(String),
}

/// Convert a slice of [`csv::Error`] into a [`Vec<CsvDiagnostic`].
pub fn normalize(errors: &[csv::Error]) -> Vec<CsvDiagnostic> {
    errors.iter().map(normalize_one).collect()
}

/// Convert a [`csv::Error`] into a [`CsvDiagnostic`].
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

/// CSV diagnostics localization interface.
pub trait CsvDiagnosticLocalization {
    /// Localized string for I/O error.
    fn io_error(&self) -> &'static str;
    /// Localized string for UTF-8 error.
    fn utf8_error(&self) -> &'static str;
    /// Localized string for unexpected EOF error.
    fn unexpected_eof(&self) -> &'static str;
    /// Localized string for generic parse error.
    fn parse_generic_error(&self) -> &'static str;
    /// Localized string for UTF-8 parse error.
    fn parse_utf8_error(&self) -> &'static str;
    /// Localized string for bool parse error.
    fn parse_bool_error(&self) -> &'static str;
    /// Localized string for float parse error.
    fn parse_float_error(&self) -> &'static str;
    /// Localized string for int parse error.
    fn parse_int_error(&self) -> &'static str;
    /// Localized string for unsupported error.
    fn unsupported(&self) -> &'static str;
    /// Localized string for unknown error.
    fn unknown(&self) -> &'static str;
}

/// Formatter interface for position of CSV diagnostic.
pub trait CsvPositionFormatter {
    /// Produce a formatted string for a given [`csv::Position`].
    fn format(&self, pos: Option<&csv::Position>) -> String;
}

/// Field name resolver interface for CSV diagnostic.
pub trait CsvFieldResolver {
    /// Produce the field name for a given [`crate::deser::TipoRecord`]
    /// and a field index.
    fn resolve(&self, record: TipoRecord, idx: usize) -> String;
}

/// Formatter interface for layout of CSV diagnostic.
pub trait CsvDiagnosticLayout {
    /// Produce a formatted string for a deserialize CSV diagnostic.
    fn deserialize(&self, base: &str, pos: &str, field: &str, detail: &str) -> String;

    /// Produce a formatted string for an unequal lengths CSV diagnostic.
    fn unequal_lengths(&self, pos: &str, expected: u64, found: u64) -> String;

    /// Produce a formatted string for an I/O CSV diagnostic.
    fn io(&self, base: &str, pos: &str, detail: &str) -> String;

    /// Produce a formatted string for an UTF-8 CSV diagnostic.
    fn utf8(&self, base: &str, pos: &str, detail: &str) -> String;

    /// Produce a formatted string for an unknown CSV diagnostic.
    fn unknown(&self, base: &str, pos: &str, detail: &str) -> String;
}

/// Formatter interface for CSV diagnostic.
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
    /// Produce a formatted string for a given [`CsvDiagnostic`] and its
    /// relative [`crate::deser::TipoRecord`].
    pub fn format(&self, err: &CsvDiagnostic, record: TipoRecord) -> String {
        let pos = self.pos.format(match err {
            CsvDiagnostic::Deserialize { pos, .. } => pos.as_ref(),
            CsvDiagnostic::Io { pos, .. } => pos.as_ref(),
            CsvDiagnostic::Utf8 { pos, .. } => pos.as_ref(),
            CsvDiagnostic::UnequalLengths { pos, .. } => pos.as_ref(),
            CsvDiagnostic::Unknown { pos, .. } => pos.as_ref(),
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

/// Produce a formatted string for a given [`csv::Error`] and its relative
/// [`crate::deser::TipoRecord`], using a [`CsvDiagnosticFormatter`].
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

/// Produce a formatted string for a given slice of [`csv::Error`] and its relative
/// [`crate::deser::TipoRecord`], using a [`CsvDiagnosticFormatter`].
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
