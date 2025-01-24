// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::Zero;
use malachite_nz::integer::Integer;

pub fn add_naive(x: Rational, y: Rational) -> Rational {
    let x_sign = x >= 0u32;
    let y_sign = y >= 0u32;
    let (xn, xd) = x.into_numerator_and_denominator();
    let (yn, yd) = y.into_numerator_and_denominator();
    let n =
        Integer::from_sign_and_abs(x_sign, xn * &yd) + Integer::from_sign_and_abs(y_sign, yn * &xd);
    Rational::from_sign_and_naturals(n >= 0u32, n.unsigned_abs(), xd * yd)
}

pub fn rational_sum_naive<I: Iterator<Item = Rational>>(xs: I) -> Rational {
    let mut s = Rational::ZERO;
    for x in xs {
        s += x;
    }
    s
}
