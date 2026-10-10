// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural::exhaustive::exhaustive_natural_range;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Square, PowerOf2};
use malachite_base::num::basic::traits::Zero;

// Tries every residue modulo `2 ^ pow` in increasing order and returns the first whose square is
// `x`, so the result is the least root. This takes time exponential in `pow`.
pub fn mod_power_of_2_sqrt_naive(x: &Natural, pow: u64) -> Option<Natural> {
    exhaustive_natural_range(Natural::ZERO, Natural::power_of_2(pow))
        .find(|r| r.mod_power_of_2_square(pow) == *x)
}
