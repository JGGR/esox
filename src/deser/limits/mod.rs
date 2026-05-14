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

use std::io::{self, Read};

pub(crate) trait ByteLimit {
    const MAX_BYTES: u64;
}

#[expect(dead_code)]
struct SmallByteLimit;
#[expect(dead_code)]
struct MediumByteLimit;
#[derive(Default)]
pub(crate) struct DefaultByteLimit;
#[expect(dead_code)]
pub(crate) struct CustomByteLimit<const N: usize>;

const ONE_KI_B: u64 = 1024; // 1 KiB
#[expect(dead_code)]
const FIVE_HUNDRED_KI_B: u64 = 500 * ONE_KI_B; // 500 KiB
const ONE_MI_B: u64 = 1024 * ONE_KI_B; // 1 MiB

impl ByteLimit for SmallByteLimit {
    const MAX_BYTES: u64 = FIVE_HUNDRED_KI_B;
}

impl ByteLimit for MediumByteLimit {
    const MAX_BYTES: u64 = ONE_MI_B;
}

impl ByteLimit for DefaultByteLimit {
    const MAX_BYTES: u64 = ONE_MI_B;
}

pub(crate) fn with_limited_reader<R, F, O, E, M>(
    mut reader: R,
    byte_limit: u64,
    f: F,
    map_io_error: M,
) -> Result<O, E>
where
    R: Read,
    F: FnOnce(&mut dyn Read) -> Result<O, E>,
    M: FnOnce(io::Error) -> E,
{
    let mut limited = reader.by_ref().take(byte_limit);

    let output = f(&mut limited)?;

    // IMPORTANT: drop limited reader
    drop(limited);

    let mut extra = [0u8; 1];

    match reader.read(&mut extra) {
        Ok(0) => Ok(output),
        Ok(_) => Err(map_io_error(io::Error::new(
            io::ErrorKind::Other,
            "input exceeded maximum byte limit",
        ))),
        Err(e) => Err(map_io_error(e)),
    }
}
