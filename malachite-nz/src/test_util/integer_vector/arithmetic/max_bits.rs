// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::logic::traits::SignificantBits;

// Takes the largest number of significant bits of any element, and checks each element's sign.
// Nothing is shared with the implementation.
pub fn vec_max_bits_naive(xs: &[Integer]) -> (u64, bool) {
    (
        xs.iter()
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(0),
        xs.iter().any(|x| *x < 0u32),
    )
}
