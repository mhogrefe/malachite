// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::compose_truncated::*;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{MulTruncated, Polynomial};

// Composes two polynomials as power series with Horner's rule, using only the public arithmetic:
// $p(q) = (\cdots(p_{n-1} q + p_{n-2}) q + \cdots) q + p_0$, with each product truncated.
pub fn integer_polynomial_compose_truncated_naive(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
    len: u64,
) -> IntegerPolynomial {
    let mut result = IntegerPolynomial::ZERO;
    for c in p.coefficients_asc().iter().rev() {
        result = result.mul_truncated(q, len) + IntegerPolynomial::from(c.clone());
    }
    result.truncate(len)
}

// The composition of two polynomials as power series, by Horner's rule.
pub fn integer_polynomial_compose_truncated_horner(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
    len: u64,
) -> IntegerPolynomial {
    compose_truncated_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        len,
        compose_truncated_horner,
    )
}

// The composition of two polynomials as power series, by the Brent–Kung algorithm.
pub fn integer_polynomial_compose_truncated_brent_kung(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
    len: u64,
) -> IntegerPolynomial {
    compose_truncated_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        len,
        compose_truncated_brent_kung,
    )
}

// The composition of two polynomials as power series, by the Kinoshita–Li algorithm.
pub fn integer_polynomial_compose_truncated_kinoshita_li(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
    len: u64,
) -> IntegerPolynomial {
    compose_truncated_with(
        p.coefficients_asc(),
        q.coefficients_asc(),
        len,
        compose_truncated_kinoshita_li,
    )
}
