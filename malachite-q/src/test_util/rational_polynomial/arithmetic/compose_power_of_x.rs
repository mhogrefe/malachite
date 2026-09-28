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
use alloc::vec::Vec;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// Builds p(x^k) coefficient by coefficient, reading the coefficient i/k through `coefficient` where
// k divides i and giving zero elsewhere; for k = 0, sums the coefficients. Nothing is shared with
// the implementation.
pub fn compose_power_of_x_naive(p: &RationalPolynomial, k: u64) -> RationalPolynomial {
    if k == 0 {
        let sum = (0..p.len()).map(|i| p.coefficient(i)).sum::<Rational>();
        return RationalPolynomial::from_coefficients_asc(vec![sum]);
    }
    if p.len() == 0 {
        return RationalPolynomial::ZERO;
    }
    RationalPolynomial::from_coefficients_asc(
        (0..=(p.len() - 1) * k)
            .map(|i| {
                if i % k == 0 {
                    p.coefficient(i / k)
                } else {
                    Rational::ZERO
                }
            })
            .collect::<Vec<Rational>>(),
    )
}
