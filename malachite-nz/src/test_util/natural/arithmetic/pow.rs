// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::SquareAssign;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::BitIterable;

pub fn natural_pow_naive(n: &Natural, exp: u64) -> Natural {
    let mut result = Natural::ONE;
    for _ in 0..exp {
        result *= n;
    }
    result
}

pub fn natural_pow_simple_binary(n: &Natural, exp: u64) -> Natural {
    let mut result = Natural::ONE;
    for bit in exp.bits().rev() {
        result.square_assign();
        if bit {
            result *= n;
        }
    }
    result
}
