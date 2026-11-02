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
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;

// Divides one power series by another one coefficient at a time over the rationals, solving
// $\sum_{j=0}^i q_j r_{i-j} = p_i$ for each $r_i$ in turn, or returns `None` if the divisor's
// constant coefficient is 0.
pub fn div_truncated_naive(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
    len: u64,
) -> Option<RationalPolynomial> {
    if len == 0 {
        return Some(RationalPolynomial::ZERO);
    }
    let q0 = q.coefficient(0);
    if q0 == 0u32 {
        return None;
    }
    let n = usize::exact_from(len);
    let mut rs: Vec<Rational> = Vec::with_capacity(n);
    for i in 0..n {
        let mut t = p.coefficient(u64::exact_from(i));
        for j in 1..=i {
            t.sub_mul_assign(q.coefficient(u64::exact_from(j)), &rs[i - j]);
        }
        rs.push(t / &q0);
    }
    Some(RationalPolynomial::from_coefficients_asc(rs))
}

// Inverts a power series by dividing 1 by it one coefficient at a time, or returns `None` if its
// constant coefficient is 0.
pub fn inverse_truncated_naive(p: &RationalPolynomial, len: u64) -> Option<RationalPolynomial> {
    div_truncated_naive(&RationalPolynomial::one(), p, len)
}
