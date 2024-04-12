// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{ModPowerOf2MulAssign, ModPowerOf2SquareAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::BitIterable;

pub fn simple_binary_mod_power_of_2_pow(x: &Natural, exp: &Natural, pow: u64) -> Natural {
    if pow == 0 {
        return Natural::ZERO;
    }
    let mut out = Natural::ONE;
    for bit in exp.bits().rev() {
        out.mod_power_of_2_square_assign(pow);
        if bit {
            out.mod_power_of_2_mul_assign(x, pow);
        }
    }
    out
}
