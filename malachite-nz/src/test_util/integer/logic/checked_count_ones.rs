// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::{BitIterable, CountOnes};

pub fn integer_checked_count_ones_alt_1(n: &Integer) -> Option<u64> {
    if *n >= 0 {
        Some(u64::wrapping_from(n.bits().filter(|&b| b).count()))
    } else {
        None
    }
}

pub fn integer_checked_count_ones_alt_2(n: &Integer) -> Option<u64> {
    if *n >= 0 {
        Some(n.twos_complement_limbs().map(CountOnes::count_ones).sum())
    } else {
        None
    }
}
