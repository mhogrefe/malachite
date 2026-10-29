// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::Polynomial;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_div_rem::*;
use crate::unsigned_polynomial::arithmetic::mod_div_truncated::*;

// Divides one polynomial by another modulo m by schoolbook long division, using only the scalar
// modular arithmetic: while the remainder is at least as long as the divisor, its leading term is
// cancelled by a multiple of the divisor. Returns `None` if the divisor is zero or its leading
// coefficient is not invertible.
pub fn mod_div_rem_naive<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
    if m == T::ONE {
        return Some((UnsignedPolynomial::ZERO, UnsignedPolynomial::ZERO));
    }
    let ys = q.coefficients_asc();
    let inverse = ys.last()?.mod_inverse(m)?;
    let mut rs = p.coefficients_asc().to_vec();
    if rs.len() < ys.len() {
        return Some((UnsignedPolynomial::ZERO, p.clone()));
    }
    let mut qs = vec![T::ZERO; rs.len() - ys.len() + 1];
    for i in (0..qs.len()).rev() {
        let c = rs[i + ys.len() - 1].mod_mul(inverse, m);
        qs[i] = c;
        for (j, &y) in ys.iter().enumerate() {
            rs[i + j].mod_sub_assign(c.mod_mul(y, m), m);
        }
    }
    rs.truncate(ys.len() - 1);
    Some((
        UnsignedPolynomial::from_coefficients_asc(qs),
        UnsignedPolynomial::from_coefficients_asc(rs),
    ))
}

// The quotient and remainder of two polynomials modulo m, with the reversals divided as power
// series coefficient by coefficient.
pub fn mod_div_rem_basecase_wrapper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
    mod_div_rem_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        m,
        mod_div_truncated_basecase,
    )
}

// The quotient and remainder of two polynomials modulo m, with the reversals divided as power
// series by the method of Karp and Markstein.
pub fn mod_div_rem_newton_wrapper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
    mod_div_rem_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        m,
        mod_div_truncated_newton,
    )
}
