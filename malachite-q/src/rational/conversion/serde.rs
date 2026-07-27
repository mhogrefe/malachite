// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational::{Rational, SerdeRational};
use alloc::string::{String, ToString};
use malachite_base::num::arithmetic::traits::CoprimeWith;

impl From<Rational> for SerdeRational {
    #[inline]
    fn from(x: Rational) -> Self {
        Self {
            sign: x.sign,
            numerator: x.numerator,
            denominator: x.denominator,
        }
    }
}

impl TryFrom<SerdeRational> for Rational {
    type Error = String;

    // The three conditions are the ones `Rational::is_valid` checks. They are spelled out here
    // rather than deferring to it because that function is only built for testing.
    fn try_from(x: SerdeRational) -> Result<Self, String> {
        if x.denominator == 0 {
            return Err("Denominator is zero".to_string());
        }
        if !x.sign && x.numerator == 0 {
            return Err("Zero is negative".to_string());
        }
        if !(&x.numerator).coprime_with(&x.denominator) {
            return Err("Numerator and denominator are not relatively prime".to_string());
        }
        Ok(Self {
            sign: x.sign,
            numerator: x.numerator,
            denominator: x.denominator,
        })
    }
}
