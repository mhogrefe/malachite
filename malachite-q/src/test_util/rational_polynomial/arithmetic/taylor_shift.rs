// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{BinomialCoefficient, Pow};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;
use malachite_nz::natural::Natural;

// Shifts a polynomial by expanding every term with the binomial theorem, in `Rational` arithmetic:
// the coefficient of $x^k$ in $p(x + c)$ is $\sum_{i \geq k} p_i \binom{i}{k} c^{i-k}$. Nothing is
// shared with the implementation, which works on a scaled integer numerator.
pub fn taylor_shift_naive(p: &RationalPolynomial, c: &Rational) -> RationalPolynomial {
    let n = p.len();
    RationalPolynomial::from_coefficients_asc(
        (0..n)
            .map(|k| {
                let mut sum = Rational::ZERO;
                for i in k..n {
                    sum += p.coefficient(i)
                        * Rational::from(Natural::binomial_coefficient(
                            Natural::from(i),
                            Natural::from(k),
                        ))
                        * c.pow(i - k);
                }
                sum
            })
            .collect::<Vec<Rational>>(),
    )
}
