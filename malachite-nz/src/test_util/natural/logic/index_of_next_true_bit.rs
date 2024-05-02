// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::BitIterable;

pub fn natural_index_of_next_true_bit_alt(n: &Natural, u: u64) -> Option<u64> {
    for (i, bit) in n.bits().enumerate().skip(usize::exact_from(u)) {
        if bit {
            return Some(u64::wrapping_from(i));
        }
    }
    None
}
