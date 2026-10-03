// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use crate::test_util::natural_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;
use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::polynomial::Polynomial;

// Raises the polynomial to the power `e` modulo $2^k$, where $k$ is `pow`, by `e` schoolbook
// multiplications modulo $2^k$, starting from 1 and truncating after each one.
pub fn mod_power_of_2_pow_truncated_naive(
    p: &NaturalPolynomial,
    e: u64,
    len: u64,
    pow: u64,
) -> NaturalPolynomial {
    let p = p.truncate(len);
    let mut power = NaturalPolynomial::one().truncate(len).mod_power_of_2(pow);
    for _ in 0..e {
        power = mod_power_of_2_mul_truncated_naive(&power, &p, len, pow);
    }
    power
}
