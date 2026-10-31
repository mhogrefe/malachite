// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::SubMulAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::Polynomial;

// Divides one polynomial by another, which must be nonzero, by schoolbook long division over the
// rationals, cancelling the leading coefficient of the remainder one term at a time.
pub fn div_rem_naive(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
) -> (RationalPolynomial, RationalPolynomial) {
    let ys = q.to_coefficients_asc();
    assert!(!ys.is_empty(), "division by zero");
    let mut rs = p.to_coefficients_asc();
    if rs.len() < ys.len() {
        return (RationalPolynomial::ZERO, p.clone());
    }
    let lead = ys.last().unwrap();
    let mut qs = vec![Rational::ZERO; rs.len() - ys.len() + 1];
    for i in (0..qs.len()).rev() {
        let c = &rs[i + ys.len() - 1] / lead;
        for (r, y) in rs[i..].iter_mut().zip(&ys) {
            r.sub_mul_assign(&c, y);
        }
        qs[i] = c;
    }
    rs.truncate(ys.len() - 1);
    (
        RationalPolynomial::from_coefficients_asc(qs),
        RationalPolynomial::from_coefficients_asc(rs),
    )
}

// The largest power of `q`, which must not be zero or constant, that divides `p`, divided out, and
// its exponent, by repeated naive division.
pub fn remove_power_naive(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
) -> (RationalPolynomial, u64) {
    let mut p = p.clone();
    let mut e = 0;
    if p == RationalPolynomial::ZERO {
        return (p, 0);
    }
    loop {
        let (s, r) = div_rem_naive(&p, q);
        if r != RationalPolynomial::ZERO {
            return (p, e);
        }
        p = s;
        e += 1;
    }
}
