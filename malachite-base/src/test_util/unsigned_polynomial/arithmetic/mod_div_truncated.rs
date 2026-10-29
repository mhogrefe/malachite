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
use crate::unsigned_polynomial::arithmetic::mod_div_truncated::*;

// Divides one polynomial by another modulo m as power series, keeping the first `len` coefficients,
// one coefficient at a time, using only the scalar modular arithmetic: $q_i = y_0^{-1} (x_i -
// \sum_{j=1}^i y_j q_{i-j})$. Returns `None` if the divisor's constant coefficient is not
// invertible.
pub fn mod_div_truncated_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    if len == 0 || m == T::ONE {
        return Some(UnsignedPolynomial::ZERO);
    }
    let y0 = q.coefficient(0);
    if y0 == T::ZERO {
        return None;
    }
    let y0_inverse = y0.mod_inverse(m)?;
    let n = usize::exact_from(len);
    let mut qs: Vec<T> = Vec::with_capacity(n);
    for i in 0..n {
        let mut s = p.coefficient(u64::exact_from(i));
        for j in 1..=i {
            s.mod_sub_assign(q.coefficient(u64::exact_from(j)).mod_mul(qs[i - j], m), m);
        }
        qs.push(s.mod_mul(y0_inverse, m));
    }
    Some(UnsignedPolynomial::from_coefficients_asc(qs))
}

// The quotient of two polynomials modulo m as power series, coefficient by coefficient.
pub fn mod_div_truncated_basecase_wrapper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    mod_div_truncated_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        len,
        m,
        mod_div_truncated_basecase,
    )
}

// The quotient of two polynomials modulo m as power series, by the method of Karp and Markstein.
pub fn mod_div_truncated_newton_wrapper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    len: u64,
    m: T,
) -> Option<UnsignedPolynomial<T>> {
    mod_div_truncated_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        len,
        m,
        mod_div_truncated_newton,
    )
}
