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
use serde::{de, Deserialize, Deserializer};
use std::ops::Deref;

#[derive(Debug, Clone, Copy)]
#[repr(transparent)]
pub struct PositiveF32(f32);

impl PositiveF32 {
    pub fn new(value: f32) -> Result<Self, ()> {
        if value.is_finite() && value > 0.0 {
            Ok(Self(value))
        } else {
            Err(())
        }
    }
}

impl Deref for PositiveF32 {
    type Target = f32;
    fn deref(&self) -> &f32 {
        &self.0
    }
}

impl From<PositiveF32> for f32 {
    fn from(v: PositiveF32) -> Self {
        v.0
    }
}

pub(crate) fn deserialize_positive_f32<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let v = f32::deserialize(deserializer)?;

    if v.is_finite() && v > 0.0 {
        Ok(v)
    } else {
        Err(de::Error::custom("expected a positive, finite f32"))
    }
}
