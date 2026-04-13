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
//! # esox: NISECI / HFBI computation crate
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
//! 1. CSV deserialization (`csv::deser`)
//!    - converts raw CSV into typed intermediate structs
//!    - validates CSV structure and field types (e.g. integer fields must be valid numbers)
//!
//! 2. CSV parsing and validation (`csv::parser`)
//!    - converts the intermediate CSV structs into domain models
//!    - enforces domain rules (e.g. value ranges, invariants)
//!
//! 3. Index computation (`engines`)
//!    - computes NISECI and HFBI from validated domain data
//!
//! ### Convenience API
//!
//! `csv::load` provides a single-step interface that combines
//! deserialization and parsing (`csv::deser` + `csv::parser`),
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
//! ```
pub mod csv;
pub mod domain;
pub mod engines;
pub mod meta;
#[cfg(test)]
mod tests;
