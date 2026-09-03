// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use malachite_base::num::basic::traits::One;

// Repeated multiplication.
pub fn gaussian_integer_pow_naive(x: &GaussianInteger, exp: u64) -> GaussianInteger {
    let mut power = GaussianInteger::ONE;
    for _ in 0..exp {
        power *= x;
    }
    power
}
