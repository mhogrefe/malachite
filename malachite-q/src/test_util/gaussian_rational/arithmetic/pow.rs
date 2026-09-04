// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::arithmetic::traits::{Reciprocal, SquareAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};

// Square-and-multiply over `GaussianRational`s, reducing after every step: the algorithm the
// content-and-primitive-part version replaced.
pub fn gaussian_rational_pow_binary(x: &GaussianRational, exp: u64) -> GaussianRational {
    let mut power = GaussianRational::ONE;
    for i in (0..exp.significant_bits()).rev() {
        power.square_assign();
        if exp.get_bit(i) {
            power *= x;
        }
    }
    power
}

// Repeated multiplication, of the reciprocal for a negative exponent.
pub fn gaussian_rational_pow_naive(x: &GaussianRational, exp: i64) -> GaussianRational {
    let base = if exp < 0 { x.reciprocal() } else { x.clone() };
    let mut power = GaussianRational::ONE;
    for _ in 0..exp.unsigned_abs() {
        power *= &base;
    }
    power
}
