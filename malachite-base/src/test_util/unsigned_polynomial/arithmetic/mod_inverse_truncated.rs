// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::polynomial::Polynomial;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_inverse_truncated::*;

// Inverts a polynomial modulo m as a power series, keeping the first `len` coefficients, one
// coefficient at a time, using only the scalar modular arithmetic: $y_0 = x_0^{-1}$ and $y_i =
// -x_0^{-1} \sum_{j=1}^i x_j y_{i-j}$.
pub fn mod_inverse_truncated_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    if len == 0 || m == T::ONE {
        return Some(UnsignedPolynomial::ZERO);
    }
    let x0 = p.coefficient(0);
    if x0 == T::ZERO {
        return None;
    }
    let x0_inverse = x0.mod_inverse(m)?;
    let n = usize::exact_from(len);
    let mut ys: Vec<T> = Vec::with_capacity(n);
    ys.push(x0_inverse);
    for i in 1..n {
        let mut sum = T::ZERO;
        for j in 1..=i {
            sum.mod_add_assign(p.coefficient(u64::exact_from(j)).mod_mul(ys[i - j], m), m);
        }
        ys.push(sum.mod_mul(x0_inverse, m).mod_neg(m));
    }
    Some(UnsignedPolynomial::from_coefficients_asc(ys))
}

// The inverse of a polynomial modulo m as a power series, coefficient by coefficient.
pub fn mod_inverse_truncated_basecase_wrapper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    mod_inverse_truncated_with(p.coefficients_asc(), len, m, mod_inverse_truncated_basecase)
}

fn newton_cutoff_2<T: PrimitiveUnsigned>(xs: &[T], n: usize, m: T, inverse: T) -> Vec<T> {
    mod_inverse_truncated_newton(xs, n, m, inverse, 2)
}

// The inverse of a polynomial modulo m as a power series, by Newton's iteration all the way down to
// a single coefficient.
pub fn mod_inverse_truncated_newton_wrapper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    mod_inverse_truncated_with(p.coefficients_asc(), len, m, newton_cutoff_2)
}
