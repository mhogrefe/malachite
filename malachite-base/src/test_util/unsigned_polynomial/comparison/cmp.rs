// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::unsigned_polynomial::UnsignedPolynomial;
use core::cmp::Ordering;

/// `p` evaluated at `x` by Horner's rule, or `None` if the value does not fit in a [`u128`].
///
/// Polynomial arithmetic is not implemented yet, so this does the evaluation itself.
pub fn unsigned_polynomial_checked_evaluate(p: &UnsignedPolynomial<u64>, x: u128) -> Option<u128> {
    let mut sum = 0u128;
    for &c in p.coefficients_asc().iter().rev() {
        sum = sum.checked_mul(x)?.checked_add(u128::from(c))?;
    }
    Some(sum)
}

/// The comparison worked out from its definition rather than from any rule about degrees and
/// coefficients, or `None` where the values are too large to evaluate.
///
/// The two polynomials are evaluated past the largest root of their difference, where the
/// comparison of the values no longer changes. Two more than the largest coefficient of either is
/// such a point, since there the leading term of the difference outgrows everything below it.
///
/// Since a [`u128`] holds only so much, this gives up rather than answer wrongly; a wider type
/// would push the giving-up further out rather than remove it, which is why malachite-nz cross-
/// checks this ordering against `NaturalPolynomial`'s, where the evaluation is exact at any size.
pub fn unsigned_polynomial_cmp_evaluated(
    p: &UnsignedPolynomial<u64>,
    q: &UnsignedPolynomial<u64>,
) -> Option<Ordering> {
    let bound = u128::from(
        p.coefficients_asc()
            .iter()
            .chain(q.coefficients_asc().iter())
            .copied()
            .max()
            .unwrap_or(0),
    ) + 2;
    Some(
        unsigned_polynomial_checked_evaluate(p, bound)?
            .cmp(&unsigned_polynomial_checked_evaluate(q, bound)?),
    )
}
