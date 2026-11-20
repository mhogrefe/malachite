// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use crate::test_util::rational::number_theory::bernoulli_number::bernoulli_numbers_prefix_tangent;
use alloc::vec::Vec;
use malachite_base::num::number_theory::traits::BinomialCoefficient;
use malachite_base::polynomial::Polynomial;
use malachite_nz::natural::Natural;

// B_n(x) = sum_k binomial(n, k) B_k x^(n - k), with the Bernoulli numbers from the tangent numbers.
pub fn bernoulli_polynomial_naive(n: u64) -> RationalPolynomial {
    let bs = bernoulli_numbers_prefix_tangent(n + 1);
    let mut coefficients: Vec<Rational> = bs
        .into_iter()
        .zip(0u64..)
        .map(|(b, k)| {
            b * Rational::from(Natural::binomial_coefficient(
                Natural::from(n),
                Natural::from(k),
            ))
        })
        .collect();
    coefficients.reverse();
    RationalPolynomial::from_coefficients_asc(coefficients)
}
