// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::{SerdeUnsignedMatrix, UnsignedMatrix};
use alloc::string::{String, ToString};

impl<T: PrimitiveUnsigned> From<UnsignedMatrix<T>> for SerdeUnsignedMatrix<T> {
    #[inline]
    fn from(m: UnsignedMatrix<T>) -> Self {
        Self {
            height: m.height,
            width: m.width,
            elements: m.elements,
        }
    }
}

impl<T: PrimitiveUnsigned> TryFrom<SerdeUnsignedMatrix<T>> for UnsignedMatrix<T> {
    type Error = String;

    // The condition is the one `UnsignedMatrix::is_valid` checks. It is spelled out here rather
    // than deferring to it because that function is only built for testing.
    #[inline]
    fn try_from(m: SerdeUnsignedMatrix<T>) -> Result<Self, String> {
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
