// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use alloc::vec::Vec;
use malachite_base::polynomial::Polynomial;

// Returns `None` when some exponent with a nonzero coefficient is not a multiple of `n`, checking
// every exponent; otherwise builds the result from the coefficients at multiples of `n`, read one
// at a time through `coefficient`. Nothing is shared with the implementation.
pub fn deflate_power_of_x_naive(p: &IntegerPolynomial, n: u64) -> Option<IntegerPolynomial> {
    assert_ne!(n, 0);
    if (0..p.len()).any(|i| i % n != 0 && *p.coefficient(i) != 0u32) {
        return None;
    }
    Some(IntegerPolynomial::from_coefficients_asc(
        (0..p.len())
            .filter(|i| i % n == 0)
            .map(|i| p.coefficient(i).clone())
            .collect::<Vec<Integer>>(),
    ))
}
