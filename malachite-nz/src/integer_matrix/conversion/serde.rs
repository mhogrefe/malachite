// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::{IntegerMatrix, SerdeIntegerMatrix};
use alloc::string::{String, ToString};

impl From<IntegerMatrix> for SerdeIntegerMatrix {
    #[inline]
    fn from(m: IntegerMatrix) -> Self {
        Self {
            height: m.height,
            width: m.width,
            elements: m.elements,
        }
    }
}

impl TryFrom<SerdeIntegerMatrix> for IntegerMatrix {
    type Error = String;

    // The condition is the one `IntegerMatrix::is_valid` checks. It is spelled out here rather than
    // deferring to it because that function is only built for testing.
    #[inline]
    fn try_from(m: SerdeIntegerMatrix) -> Result<Self, String> {
        if m.height.checked_mul(m.width) != Some(m.elements.len()) {
            return Err("The number of elements is not the height times the width".to_string());
        }
        Ok(Self {
            elements: m.elements,
            height: m.height,
            width: m.width,
        })
    }
}
