// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::{SerdeUnsignedPolynomial, UnsignedPolynomial};
use alloc::string::{String, ToString};

impl<T: PrimitiveUnsigned> From<UnsignedPolynomial<T>> for SerdeUnsignedPolynomial<T> {
    #[inline]
    fn from(p: UnsignedPolynomial<T>) -> Self {
        Self(p.coefficients)
    }
}

impl<T: PrimitiveUnsigned> TryFrom<SerdeUnsignedPolynomial<T>> for UnsignedPolynomial<T> {
    type Error = String;

    // The condition is the one `UnsignedPolynomial::is_valid` checks. It is spelled out here rather
    // than deferring to it because that function is only built for testing.
    #[inline]
    fn try_from(p: SerdeUnsignedPolynomial<T>) -> Result<Self, String> {
        if p.0.last() == Some(&T::ZERO) {
            return Err("Last coefficient is zero".to_string());
        }
        Ok(Self { coefficients: p.0 })
    }
}
