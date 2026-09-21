// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use core::cmp::Ordering;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::SignificantBits;

/// `p` evaluated at `x` by Horner's rule.
///
/// Polynomial arithmetic is not implemented yet, so this does the evaluation itself.
pub fn natural_polynomial_evaluate(p: &NaturalPolynomial, x: &Natural) -> Natural {
    let mut sum = Natural::ZERO;
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
/// comparison of the values no longer changes. Twice the largest coefficient of either is such a
/// point, since there the leading term of the difference outgrows everything below it.
pub fn natural_polynomial_cmp_evaluated(p: &NaturalPolynomial, q: &NaturalPolynomial) -> Ordering {
    let bound = Natural::from(2u32)
        << p.coefficients_asc()
            .iter()
            .chain(q.coefficients_asc().iter())
            .map(SignificantBits::significant_bits)
            .max()
            .unwrap_or(0);
    natural_polynomial_evaluate(p, &bound).cmp(&natural_polynomial_evaluate(q, &bound))
}
