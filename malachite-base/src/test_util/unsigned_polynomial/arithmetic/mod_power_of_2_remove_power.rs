// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_div_rem::*;
use crate::unsigned_polynomial::UnsignedPolynomial;

// Removes the largest power of `q` that divides `p` modulo 2^k by naive long division, dividing
// while the remainder is 0.
pub fn mod_power_of_2_remove_power_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) -> (UnsignedPolynomial<T>, u64) {
    let mut p = p.clone();
    let mut e = 0;
    while p != UnsignedPolynomial::ZERO {
        let (s, r) = mod_power_of_2_div_rem_naive(&p, q, pow).unwrap();
        if r != UnsignedPolynomial::ZERO {
            break;
        }
        p = s;
        e += 1;
    }
    (p, e)
}
