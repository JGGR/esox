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
//! Parsing module for intermediate structs.
//!
//! This module provides support for:
//!
//! - Validation of the intermediate structs
//! - Enforcement of `crate::domain` invariants
//!
//! Next version will:
//!
//! - Drop the `crate::csv::parser` module
//! - Expose more methods here to provide the same functionality
//!
//! In the meantime, consider using:
//!
//! - `parse_records`
//! - `check_records`
//!
//! from `crate::domain` structs.

use chrono::NaiveDate;

pub fn parse_date(date_str: &str) -> Result<NaiveDate, chrono::format::ParseError> {
    let normalized = date_str.replace("/", "-"); // Replace all / with -
    NaiveDate::parse_from_str(&normalized, "%d-%m-%Y")
}

pub mod hfbi;
pub mod niseci;
