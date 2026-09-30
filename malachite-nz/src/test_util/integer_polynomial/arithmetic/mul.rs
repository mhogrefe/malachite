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
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{AddMulAssign, Parity, PowerOf2};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// The coefficients of the product of the polynomials with coefficients `xs` and `ys`, by schoolbook
// multiplication: each product of a coefficient of one by a coefficient of the other is added to
// the coefficient it contributes to. The result is empty if either input is, and is not trimmed.
pub fn integers_mul_naive(xs: &[Integer], ys: &[Integer]) -> Vec<Integer> {
    if xs.is_empty() || ys.is_empty() {
        return Vec::new();
    }
    let mut out = vec![Integer::ZERO; xs.len() + ys.len() - 1];
    for (i, x) in xs.iter().enumerate() {
        for (o, y) in out[i..].iter_mut().zip(ys) {
            o.add_mul_assign(x, y);
        }
    }
    out
}

// Multiplies two polynomials by schoolbook multiplication of their coefficients.
pub fn mul_naive(p: &IntegerPolynomial, q: &IntegerPolynomial) -> IntegerPolynomial {
    IntegerPolynomial::from_coefficients_asc(integers_mul_naive(
        p.coefficients_asc(),
        q.coefficients_asc(),
    ))
}

// `len` coefficients, each with exactly `bits` significant bits, alternating in sign: $2^b - 1$,
// $-(2^b - 2)$, $2^b - 3$, and so on. `bits` must be large enough that none is zero. Unit tests use
// these to reach the branches of the multiplication dispatchers that depend on coefficient size.
pub fn generated_coefficients(len: usize, bits: u64) -> Vec<Integer> {
    (0..len)
        .map(|i| {
            let x = Integer::power_of_2(bits) - Integer::from(i + 1);
            if i.odd() { -x } else { x }
        })
        .collect()
}
