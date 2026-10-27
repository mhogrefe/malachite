// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{MulTruncated, Polynomial};

// Composes two polynomials as power series with Horner's rule, using only the public arithmetic:
// $p(q) = (\cdots(p_{n-1} q + p_{n-2}) q + \cdots) q + p_0$, with each product truncated.
pub fn rational_polynomial_compose_truncated_naive(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
    len: u64,
) -> RationalPolynomial {
    let mut result = RationalPolynomial::ZERO;
    for c in p.to_coefficients_asc().into_iter().rev() {
        result = result.mul_truncated(q, len) + RationalPolynomial::from(c);
    }
    result.truncate(len)
}
