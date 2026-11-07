// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use alloc::vec;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::series::traits::MulTruncated;

// The $n$th falling factorial polynomial truncated to its first `len` coefficients, by multiplying
// by the factors $x - i$ one at a time, truncating each product.
pub fn falling_factorial_polynomial_truncated_sequential(n: u64, len: u64) -> IntegerPolynomial {
    if len == 0 {
        return IntegerPolynomial::ZERO;
    }
    let mut p = IntegerPolynomial::from_coefficients_asc(vec![Integer::ONE]);
    for i in 0..n {
        p = p.mul_truncated(
            IntegerPolynomial::from_coefficients_asc(vec![-Integer::from(i), Integer::ONE]),
            len,
        );
    }
    p
}
