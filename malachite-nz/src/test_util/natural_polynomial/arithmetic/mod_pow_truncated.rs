// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::arithmetic::mod_mul_truncated::*;
use malachite_base::num::arithmetic::traits::Mod;
use malachite_base::polynomial::Polynomial;

// Raises the polynomial to the power `e` modulo `m` by `e` schoolbook multiplications modulo `m`,
// starting from 1 and truncating after each one.
pub fn mod_pow_truncated_naive(
    p: &NaturalPolynomial,
    e: u64,
    len: u64,
    m: &Natural,
) -> NaturalPolynomial {
    let p = p.truncate(len);
    let mut power = NaturalPolynomial::one().truncate(len).mod_op(m);
    for _ in 0..e {
        power = mod_mul_truncated_naive(&power, &p, len, m);
    }
    power
}
