// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::basic::traits::{One, Zero};

pub fn ceiling_log_base_power_of_2_naive_nz(x: &Natural, pow: u64) -> u64 {
    assert_ne!(*x, Natural::ZERO);
    assert_ne!(pow, 0);
    let mut result = 0;
    let mut p = Natural::ONE;
    while p < *x {
        result += 1;
        p <<= pow;
    }
    result
}
