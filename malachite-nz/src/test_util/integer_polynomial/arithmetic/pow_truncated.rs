// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::test_util::integer_polynomial::arithmetic::mul::mul_naive;
use malachite_base::polynomial::Polynomial;

// Raises the polynomial to the power `e` by `e` schoolbook multiplications, starting from 1 and
// truncating after each one.
pub fn pow_truncated_naive(p: &IntegerPolynomial, e: u64, len: u64) -> IntegerPolynomial {
    let p = p.clone().truncate(len);
    let mut power = IntegerPolynomial::one().truncate(len);
    for _ in 0..e {
        power = mul_naive(&power, &p).truncate(len);
    }
    power
}
