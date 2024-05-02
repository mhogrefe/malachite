// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use num::{BigUint, One, Zero};

pub fn num_get_bit(x: &BigUint, index: u64) -> bool {
    x & (BigUint::one() << usize::exact_from(index)) != BigUint::zero()
}
