// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::PowerOf2;

// Multiplies by $2^k$ instead of shifting.
pub fn add_mul_shl_naive(x: &Natural, y: &Natural, z: &Natural, bits: u64) -> Natural {
    x + y * z * Natural::power_of_2(bits)
}
