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
//! NISECI computation module.
//!
//! Routines and types for computing NISECI index from [`crate::domain::niseci`] structs.
//!
//! The main methods are:
//!
//! - [`full::calculate_niseci`]
//! - [`full::calculate_rqe_niseci`]
//! - [`full::calculate_stato_ecologico_niseci`]

pub mod full;
pub mod linear_regression;
pub mod x1;
pub mod x2;
pub mod x3;
