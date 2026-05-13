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

#[cfg_attr(not(feature = "lessclone"), expect(dead_code))]
pub(crate) trait Capacity {
    const VALUE: usize;
}

#[cfg_attr(feature = "lessclone", expect(dead_code))]
struct Small;
#[cfg_attr(feature = "lessclone", expect(dead_code))]
struct Medium;
#[derive(Default)]
pub(crate) struct DefaultCapacity;
#[expect(dead_code)]
struct Custom<const N: usize>;

const SMALL_CAP: usize = 100;
#[cfg_attr(feature = "lessclone", expect(dead_code))]
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
