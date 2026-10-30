// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use malachite_base::num::arithmetic::traits::{DivExact, DivisibleBy, SubMulAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::Polynomial;

// Divides one power series by another one coefficient at a time, solving $\sum_{j=0}^i q_j r_{i-j}
// = p_i$ for each $r_i$ in turn, or returns `None` if the divisor's constant coefficient is 0 or
// some $r_i$ is not an integer.
pub fn div_truncated_naive(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
    len: u64,
) -> Option<IntegerPolynomial> {
    if len == 0 {
        return Some(IntegerPolynomial::ZERO);
    }
    let q0 = q.coefficient(0);
    if *q0 == 0u32 {
        return None;
    }
    let n = usize::exact_from(len);
    let mut rs: Vec<Integer> = Vec::with_capacity(n);
    for i in 0..n {
        let mut t = p.coefficient(u64::exact_from(i)).clone();
        for j in 1..=i {
            t.sub_mul_assign(q.coefficient(u64::exact_from(j)), &rs[i - j]);
        }
        if !(&t).divisible_by(q0) {
            return None;
        }
        rs.push(t.div_exact(q0));
    }
    Some(IntegerPolynomial::from_coefficients_asc(rs))
}
