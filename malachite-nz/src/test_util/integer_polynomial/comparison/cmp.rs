// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use core::cmp::Ordering::{self, *};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;

/// `p` evaluated at `x` by Horner's rule.
///
/// Polynomial arithmetic is not implemented yet, so this does the evaluation itself.
pub fn integer_polynomial_evaluate(p: &IntegerPolynomial, x: &Integer) -> Integer {
    let mut sum = Integer::ZERO;
    for c in p.coefficients_asc().iter().rev() {
        sum *= x;
        sum += c;
    }
    sum
}

/// The comparison worked out from its definition rather than from any rule about degrees and
/// coefficients.
///
/// The two polynomials are evaluated past the largest root of their difference, where the
/// comparison of the values no longer changes. Four times the largest coefficient of either is such
/// a point, since there the leading term of the difference outgrows everything below it.
pub fn integer_polynomial_cmp_evaluated(p: &IntegerPolynomial, q: &IntegerPolynomial) -> Ordering {
    let bound = Integer::from(4u32)
        << p.coefficients_asc()
            .iter()
            .chain(q.coefficients_asc().iter())
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(0);
    integer_polynomial_evaluate(p, &bound).cmp(&integer_polynomial_evaluate(q, &bound))
}

/// The shortlex comparison, written as an explicit walk from the highest-degree coefficient down
/// rather than as a comparison of two reversed iterators.
pub fn integer_polynomial_shortlex_cmp_naive(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> Ordering {
    let xs = p.coefficients_asc();
    let ys = q.coefficients_asc();
    let c = xs.len().cmp(&ys.len());
    if c != Equal {
        return c;
    }
    for i in (0..xs.len()).rev() {
        let c = xs[i].cmp(&ys[i]);
        if c != Equal {
            return c;
        }
    }
    Equal
}
