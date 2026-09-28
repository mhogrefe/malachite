// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// Builds the result coefficient by coefficient: zero below `n`, and the coefficient `n` places down
// above it, read through `coefficient`. Nothing is shared with the implementation.
pub fn mul_power_of_x_naive(p: &NaturalPolynomial, n: u64) -> NaturalPolynomial {
    if *p == NaturalPolynomial::ZERO {
        return NaturalPolynomial::ZERO;
    }
    NaturalPolynomial::from_coefficients_asc(
        (0..p.len() + n)
            .map(|i| {
                if i < n {
                    Natural::ZERO
                } else {
                    p.coefficient(i - n).clone()
                }
            })
            .collect::<Vec<Natural>>(),
    )
}
