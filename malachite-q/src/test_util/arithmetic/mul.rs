// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::basic::traits::{One, Zero};

pub fn mul_naive(x: Rational, y: Rational) -> Rational {
    if x == 0u32 || y == 0u32 {
        Rational::ZERO
    } else {
        let sign = (x >= 0) == (y >= 0);
        let (xn, xd) = x.into_numerator_and_denominator();
        let (yn, yd) = y.into_numerator_and_denominator();
        Rational::from_sign_and_naturals(sign, xn * yn, xd * yd)
    }
}

pub fn rational_product_naive<I: Iterator<Item = Rational>>(xs: I) -> Rational {
    let mut p = Rational::ONE;
    for x in xs {
        if x == 0 {
            return Rational::ZERO;
        }
        p *= x;
    }
    p
}
