// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::test_util::unsigned_polynomial::arithmetic::mod_mul::*;
use crate::unsigned_polynomial::UnsignedPolynomial;

// Raises the polynomial to the power `e` modulo `m` by `e` schoolbook multiplications modulo `m`,
// starting from 1.
pub fn mod_pow_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    e: u64,
    m: T,
) -> UnsignedPolynomial<T> {
    let mut power = UnsignedPolynomial::one() % m;
    for _ in 0..e {
        power = mod_mul_polynomial_naive(&power, p, m);
    }
    power
}
