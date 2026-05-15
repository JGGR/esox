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
//! HFBI computation module.
//!
//! Routines and types for computing HFBI index from [`crate::domain::hfbi`] structs.
//!
//! The main methods are:
//!
//! - [`full::calculate_hfbi`]
//! - [`full::calculate_mmi`]
//! - [`full::calculate_stato_ecologico_hfbi`]

pub mod bbent;
pub mod bn;
pub mod dbent;
pub mod ddom;
pub mod dhzp;
pub mod dmig;
pub mod full;
