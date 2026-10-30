// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::arithmetic::mod_power_of_2_div_rem::*;
use malachite_base::num::basic::traits::Zero;

// Removes the largest power of `q` that divides `p` modulo 2^k by naive long division, dividing
// while the remainder is 0.
pub fn mod_power_of_2_remove_power_naive(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> (NaturalPolynomial, u64) {
    let mut p = p.clone();
    let mut e = 0;
    while p != NaturalPolynomial::ZERO {
        let (s, r) = mod_power_of_2_div_rem_naive(&p, q, pow).unwrap();
        if r != NaturalPolynomial::ZERO {
            break;
        }
        p = s;
        e += 1;
    }
    (p, e)
}
