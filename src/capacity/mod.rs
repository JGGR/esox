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
//! Capacity trait, work-in-progress.
//! Used only by:
//! - [`crate::domain::niseci::ClassiEtaAlieniNISECI::new_with_capacity`]
//! - Some methods in [`crate::engines::niseci::x2`]

/// Denotes the starting capacity of some collections. Work-in-progress.
pub trait Capacity {
    /// Value for starting capacity.
    const VALUE: usize;
}

/// Preset [`Capacity`] value.
pub struct Small;
/// Preset [`Capacity`] value.
pub struct Medium;
/// Default [`Capacity`] value.
pub struct DefaultCapacity;
/// Custom [`Capacity`] value.
pub struct Custom<const N: usize>;

const SMALL_CAP: usize = 100;
const MEDIUM_CAP: usize = 1000;

impl Capacity for Small {
    const VALUE: usize = SMALL_CAP;
}

impl Capacity for Medium {
    const VALUE: usize = MEDIUM_CAP;
}

impl Capacity for DefaultCapacity {
    const VALUE: usize = SMALL_CAP;
}
