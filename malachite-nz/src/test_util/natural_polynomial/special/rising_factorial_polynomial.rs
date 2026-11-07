// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::series::traits::MulTruncated;

// The $n$th rising factorial polynomial truncated to its first `len` coefficients, the unsigned
// Stirling numbers of the first kind $\left[{n \atop k}\right]$ for $k < \text{len}$, by filling
// the triangle with the recurrence $\left[{m \atop k}\right] = \left[{m - 1 \atop k - 1}\right] +
// (m - 1)\left[{m - 1 \atop k}\right]$, keeping only the first `len` columns.
pub fn rising_factorial_polynomial_truncated_naive(n: u64, len: u64) -> NaturalPolynomial {
    let len = usize::exact_from(len);
    if len == 0 {
        return NaturalPolynomial::ZERO;
    }
    // row[k] = [m, k] for the current m, starting from m = 0
    let mut row = vec![Natural::ZERO; len];
    row[0] = Natural::ONE;
    for m in 1..=n {
        let factor = Natural::from(m - 1);
        let top = usize::exact_from(m.min(u64::exact_from(len - 1)));
        for k in (1..=top).rev() {
            let previous = row[k - 1].clone();
            row[k] *= &factor;
            row[k] += previous;
        }
        row[0] *= &factor;
    }
    NaturalPolynomial::from_coefficients_asc(row)
}

// The $n$th rising factorial polynomial truncated to its first `len` coefficients, by multiplying
// by the factors $x + i$ one at a time, truncating each product.
pub fn rising_factorial_polynomial_truncated_sequential(n: u64, len: u64) -> NaturalPolynomial {
    if len == 0 {
        return NaturalPolynomial::ZERO;
    }
    let mut p = NaturalPolynomial::from_coefficients_asc(vec![Natural::ONE]);
    for i in 0..n {
        p = p.mul_truncated(
            NaturalPolynomial::from_coefficients_asc(vec![Natural::from(i), Natural::ONE]),
            len,
        );
    }
    p
}
