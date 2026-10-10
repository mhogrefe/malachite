// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::Mod;
use malachite_base::num::conversion::traits::ExactFrom;

// Computes the difference exactly, as an `Integer`, and reduces it once.
pub fn mod_sub_mul_naive(x: &Natural, y: &Natural, z: &Natural, m: &Natural) -> Natural {
    Natural::exact_from((Integer::from(x) - Integer::from(y * z)).mod_op(Integer::from(m)))
}
