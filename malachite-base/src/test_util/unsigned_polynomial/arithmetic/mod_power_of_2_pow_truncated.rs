// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModPowerOf2;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::test_util::unsigned_polynomial::arithmetic::mod_power_of_2_mul::*;
use crate::unsigned_polynomial::UnsignedPolynomial;

// Raises the polynomial to the power `e` modulo $2^k$, where $k$ is `pow`, by `e` schoolbook
// multiplications modulo $2^k$, starting from 1 and truncating after each one.
pub fn mod_power_of_2_pow_truncated_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    e: u64,
    len: u64,
    pow: u64,
) -> UnsignedPolynomial<T> {
    let p = p.truncate(len);
    let mut power = UnsignedPolynomial::one().truncate(len).mod_power_of_2(pow);
    for _ in 0..e {
        power = mod_power_of_2_mul_polynomial_naive(&power, &p, pow).truncate(len);
    }
    power
}
