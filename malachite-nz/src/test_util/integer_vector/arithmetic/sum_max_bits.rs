// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::logic::traits::SignificantBits;

// Adds the absolute values as `Natural`s and takes the largest number of significant bits of any
// element. Nothing is shared with the implementation.
pub fn vec_sum_max_bits_naive(xs: &[Integer]) -> (u64, u64) {
    (
        xs.iter()
            .map(Integer::unsigned_abs_ref)
            .sum::<Natural>()
            .significant_bits(),
        xs.iter()
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(0),
    )
}
