// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::basic::traits::Zero;

pub fn div_naive(x: Rational, y: Rational) -> Rational {
    if x == 0u32 {
        Rational::ZERO
    } else if y == 0u32 {
        panic!("division by zero");
    } else {
        let sign = (x >= 0) == (y >= 0);
        let (xn, xd) = x.into_numerator_and_denominator();
        let (yn, yd) = y.into_numerator_and_denominator();
        Rational::from_sign_and_naturals(sign, xn * yd, xd * yn)
    }
}
