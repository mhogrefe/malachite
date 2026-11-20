// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use alloc::vec;
use malachite_base::num::arithmetic::traits::Pow;
use malachite_base::num::basic::traits::{One, OneHalf};
use malachite_base::num::number_theory::traits::BinomialCoefficient;
use malachite_base::polynomial::Polynomial;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::integer::number_theory::euler_number::euler_numbers_prefix_naive;

// E_n(x) = sum_k binomial(n, k) E_k / 2^k (x - 1/2)^(n - k), with the Euler numbers from the
// Seidel–Entringer triangle.
pub fn euler_polynomial_naive(n: u64) -> RationalPolynomial {
    let es = euler_numbers_prefix_naive(n + 1);
    let shift = RationalPolynomial::from_coefficients_asc(vec![-Rational::ONE_HALF, Rational::ONE]);
    let mut sum = RationalPolynomial::from_coefficients_asc(vec![]);
    for (k, e) in (0u64..).zip(es) {
        let mut c = Rational::from(e) >> k;
        c *= Rational::from(Natural::binomial_coefficient(
            Natural::from(n),
            Natural::from(k),
        ));
        sum += (&shift).pow(n - k) * RationalPolynomial::from(c);
    }
    sum
}
