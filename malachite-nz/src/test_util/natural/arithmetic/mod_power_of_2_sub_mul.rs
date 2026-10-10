// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::ModPowerOf2;

// Computes the difference exactly, as an `Integer`, and reduces it once.
pub fn mod_power_of_2_sub_mul_naive(x: &Natural, y: &Natural, z: &Natural, pow: u64) -> Natural {
    (Integer::from(x) - Integer::from(y * z)).mod_power_of_2(pow)
}
