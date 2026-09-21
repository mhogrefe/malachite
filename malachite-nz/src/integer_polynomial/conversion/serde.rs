// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::{IntegerPolynomial, SerdeIntegerPolynomial};
use alloc::string::{String, ToString};
use malachite_base::num::basic::traits::Zero;

impl From<IntegerPolynomial> for SerdeIntegerPolynomial {
    #[inline]
    fn from(p: IntegerPolynomial) -> Self {
        Self(p.coefficients)
    }
}

impl TryFrom<SerdeIntegerPolynomial> for IntegerPolynomial {
    type Error = String;

    // The condition is the one `IntegerPolynomial::is_valid` checks. It is spelled out here rather
    // than deferring to it because that function is only built for testing.
    #[inline]
    fn try_from(p: SerdeIntegerPolynomial) -> Result<Self, String> {
        if p.0.last() == Some(&Integer::ZERO) {
            return Err("Last coefficient is zero".to_string());
        }
        Ok(Self { coefficients: p.0 })
    }
}
