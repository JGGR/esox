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
//! # Esox: NISECI / HFBI computation crate
//!
//! Esox parses a custom CSV format and computes two derived indices:
//! NISECI and HFBI.
//!
//! ## Stability
//!
//! *Not* stable (yet). Current API surface may change.
//!
//! ## Overview
//!
//! The processing pipeline is:
//!
//! 1. CSV deserialization: ([`csv::deser`])
//!
//! - Converts raw CSV into typed intermediate structs
//! - Validates CSV structure and field types (e.g. integer fields must be valid numbers)
//!
//! 2. CSV parsing and validation: ([`csv::parser`]) (from next version: [`parser`])
//!
//! - Converts the intermediate CSV structs into [`domain`] models
//! - Enforces domain rules (e.g. value ranges, invariants)
//!
//! 3. Index computation: ([`engines`])
//!
//! - Computes NISECI and HFBI from validated domain data
//!
//! ### Convenience API
//!
//! `csv::load` provides a single-step interface that combines
//! deserialization and parsing ([`csv::deser`] + [`csv::parser`]),
//! returning validated domain models directly from raw CSV input.
//!
//! ### Input format
//!
//! Two input formats are supported:
//!
//! - Standard format: `,` as field separator, `.` as decimal separator
//! - Alternative format: `;` as field separator, `,` as decimal separator
//!
//! Templates for input files are located in the `./templates/` directory.
//! All templates currently use the alternative (`;`) format.
//!
//! ## Example usage
//!
//! ### NISECI
//!
//!```rust
//! use std::io::Cursor;
//! use esox::csv::load::InputFormat;
//! use esox::csv::load::niseci::*;
//! #[cfg(not(feature = "lessclone"))]
//! use esox::engines::niseci::full::calculate_niseci;
//! #[cfg(feature = "lessclone")]
//! use esox::engines::niseci::full::lessclone::calculate_niseci;
//! const RIFERIMENTO_DATA: &[u8] =
//! include_bytes!("../templates/riferimento_niseci.csv");
//!
//! let has_headers = true;
//! let format = InputFormat::Alternative;
//! let riferimento_reader = Cursor::new(RIFERIMENTO_DATA);
//! let riferimento = load_riferimento_niseci_from_reader(riferimento_reader, has_headers, format).unwrap();
//! let campionamento = load_campionamento_niseci_from_path("./templates/campionamento_niseci.csv", has_headers, &riferimento, format).unwrap();
//! let anagrafica = load_anagrafica_niseci_from_path("./templates/anagrafica_niseci.csv", has_headers, format).unwrap();
//! let (niseci, intermediates) = calculate_niseci(&campionamento, &riferimento, &anagrafica).unwrap();
//!```
//!
//! ### HFBI
//!
//!```rust
//! use std::io::Cursor;
//! use esox::csv::load::InputFormat;
//! use esox::csv::load::hfbi::*;
//! use esox::engines::hfbi::full::calculate_hfbi;
//! const CAMPIONAMENTO_DATA: &[u8] =
//! include_bytes!("../templates/campionamento_hfbi.csv");
//!
//! let has_headers = true;
//! let format = InputFormat::Alternative;
//! let campionamento_reader = Cursor::new(CAMPIONAMENTO_DATA);
//! let campionamento = load_campionamento_hfbi_from_reader::<_>(campionamento_reader, has_headers, format).unwrap();
//! let anagrafica = load_anagrafica_hfbi_from_path("./templates/anagrafica_hfbi.csv", has_headers, format).unwrap();
//! let (hfbi, intermediates) = calculate_hfbi(&campionamento, &anagrafica).unwrap();
//!```

pub(crate) mod capacity;
pub mod csv;
pub mod deser;
pub mod domain;
pub mod engines;
#[cfg(feature = "json")]
pub mod json;
pub mod meta;
pub mod parser;
#[cfg(test)]
mod tests;
