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

pub(crate) trait ByteLimit {
    const MAX_BYTES: u64;
}

pub(crate) struct SmallByteLimit;
pub(crate) struct MediumByteLimit;
pub(crate) struct LargeByteLimit;
#[derive(Default)]
pub(crate) struct DefaultByteLimit;
pub(crate) struct CustomByteLimit<const N: usize>;

const ONE_KI_B: u64 = 1024; // 1 KiB
const ONE_HUNDRED_KI_B: u64 = 100 * ONE_KI_B; // 100 KiB
const FIVE_HUNDRED_KI_B: u64 = 500 * ONE_KI_B; // 500 KiB
const ONE_MI_B: u64 = 1024 * ONE_KI_B; // 1 MiB

impl ByteLimit for SmallByteLimit {
    const MAX_BYTES: u64 = ONE_HUNDRED_KI_B;
}

impl ByteLimit for MediumByteLimit {
    const MAX_BYTES: u64 = FIVE_HUNDRED_KI_B;
}

impl ByteLimit for LargeByteLimit {
    const MAX_BYTES: u64 = ONE_MI_B;
}

impl ByteLimit for DefaultByteLimit {
    const MAX_BYTES: u64 = FIVE_HUNDRED_KI_B;
}
