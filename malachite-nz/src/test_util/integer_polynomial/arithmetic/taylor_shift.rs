// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{BinomialCoefficient, Pow};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;

// Shifts a polynomial by expanding every term with the binomial theorem: the coefficient of $x^k$
// in $p(x + c)$ is $\sum_{i \geq k} p_i \binom{i}{k} c^{i-k}$. This shares nothing with Horner's
// rule or the divide-and-conquer scheme, so it can check both.
pub fn taylor_shift_naive(p: &IntegerPolynomial, c: &Integer) -> IntegerPolynomial {
    let xs = p.coefficients_asc();
    let n = xs.len();
    IntegerPolynomial::from_coefficients_asc(
        (0..n)
            .map(|k| {
                let mut sum = Integer::ZERO;
                for (i, x) in xs.iter().enumerate().skip(k) {
                    sum +=
                        x * Integer::from(Natural::binomial_coefficient(
                            Natural::from(i),
                            Natural::from(k),
                        )) * c.pow(u64::exact_from(i - k));
                }
                sum
            })
            .collect::<Vec<Integer>>(),
    )
}
