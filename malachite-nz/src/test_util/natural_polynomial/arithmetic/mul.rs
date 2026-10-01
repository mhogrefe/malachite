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
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{AddMulAssign, PowerOf2};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// Multiplies the polynomials with coefficients `xs` and `ys` by schoolbook multiplication.
pub fn naturals_mul_naive(xs: &[Natural], ys: &[Natural]) -> Vec<Natural> {
    if xs.is_empty() || ys.is_empty() {
        return Vec::new();
    }
    let mut out = vec![Natural::ZERO; xs.len() + ys.len() - 1];
    for (i, x) in xs.iter().enumerate() {
        for (o, y) in out[i..].iter_mut().zip(ys) {
            o.add_mul_assign(x, y);
        }
    }
    out
}

pub fn mul_naive(p: &NaturalPolynomial, q: &NaturalPolynomial) -> NaturalPolynomial {
    NaturalPolynomial::from_coefficients_asc(naturals_mul_naive(
        p.coefficients_asc(),
        q.coefficients_asc(),
    ))
}

// `len` coefficients of exactly `bits` bits each, all different, for exercising the multiplication
// algorithms at chosen sizes.
pub fn natural_generated_coefficients(len: usize, bits: u64) -> Vec<Natural> {
    (0..len)
        .map(|i| Natural::power_of_2(bits) - Natural::from(i + 1))
        .collect()
}
