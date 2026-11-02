// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;

// Tries every residue modulo `2 ^ pow` in increasing order and returns the first whose square is
// `x`, so the result is the least root. This takes time exponential in `pow`.
pub fn mod_power_of_2_sqrt_naive<T: PrimitiveUnsigned>(x: T, pow: u64) -> Option<T> {
    let max = T::low_mask(pow);
    let mut r = T::ZERO;
    loop {
        if r.wrapping_mul(r).mod_power_of_2(pow) == x {
            return Some(r);
        }
        if r == max {
            return None;
        }
        r += T::ONE;
    }
}
