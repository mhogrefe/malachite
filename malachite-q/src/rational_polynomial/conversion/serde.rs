// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::{RationalPolynomial, SerdeRationalPolynomial, content};
use alloc::string::{String, ToString};
use malachite_base::num::arithmetic::traits::CoprimeWith;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer_polynomial::IntegerPolynomial;

impl From<RationalPolynomial> for SerdeRationalPolynomial {
    #[inline]
    fn from(p: RationalPolynomial) -> Self {
        Self {
            numerator: p.numerator,
            denominator: p.denominator,
        }
    }
}

impl TryFrom<SerdeRationalPolynomial> for RationalPolynomial {
    type Error = String;

    // The three conditions are the ones `RationalPolynomial::is_valid` checks. They are spelled out
    // here rather than deferring to it because that function is only built for testing. The
    // numerator needs no checking of its own: it arrives as an `IntegerPolynomial`, which has
    // already been through its own validation.
    fn try_from(p: SerdeRationalPolynomial) -> Result<Self, String> {
        if p.denominator == 0u32 {
            return Err("Denominator is zero".to_string());
        }
        if p.numerator == IntegerPolynomial::ZERO {
            return if p.denominator == 1u32 {
                Ok(Self::ZERO)
            } else {
                Err("Zero polynomial's denominator is not 1".to_string())
            };
        }
        if !content(&p.numerator).coprime_with(&p.denominator) {
            return Err("Numerator and denominator are not relatively prime".to_string());
        }
        Ok(Self {
            numerator: p.numerator,
            denominator: p.denominator,
        })
    }
}
