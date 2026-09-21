// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::{NaturalPolynomial, SerdeNaturalPolynomial};
use alloc::string::{String, ToString};
use malachite_base::num::basic::traits::Zero;

impl From<NaturalPolynomial> for SerdeNaturalPolynomial {
    #[inline]
    fn from(p: NaturalPolynomial) -> Self {
        Self(p.coefficients)
    }
}

impl TryFrom<SerdeNaturalPolynomial> for NaturalPolynomial {
    type Error = String;

    // The condition is the one `NaturalPolynomial::is_valid` checks. It is spelled out here rather
    // than deferring to it because that function is only built for testing.
    #[inline]
    fn try_from(p: SerdeNaturalPolynomial) -> Result<Self, String> {
        if p.0.last() == Some(&Natural::ZERO) {
            return Err("Last coefficient is zero".to_string());
        }
        Ok(Self { coefficients: p.0 })
    }
}
