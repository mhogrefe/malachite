// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitIterable, SignificantBits};

pub fn integer_index_of_next_false_bit_alt(n: &Integer, u: u64) -> Option<u64> {
    if u >= n.significant_bits() {
        if *n >= 0 {
            Some(u)
        } else {
            None
        }
    } else {
        for (i, bit) in n.bits().enumerate().skip(usize::exact_from(u)) {
            if !bit {
                return Some(u64::exact_from(i));
            }
        }
        None
    }
}
