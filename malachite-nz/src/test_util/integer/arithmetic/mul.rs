// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::basic::traits::{One, Zero};

pub fn integer_product_naive<I: Iterator<Item = Integer>>(xs: I) -> Integer {
    let mut p = Integer::ONE;
    for x in xs {
        if x == 0 {
            return Integer::ZERO;
        }
        p *= x;
    }
    p
}
