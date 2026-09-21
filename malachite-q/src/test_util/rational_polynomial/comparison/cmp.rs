// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use std::vec::Vec;

/// The asymptotic comparison, worked out by materializing every coefficient as a
/// [`Rational`](crate::Rational).
///
/// This is what the real implementation is written to avoid: building a [`Rational`] out of a
/// numerator coefficient and the shared denominator reduces the two against each other, and it
/// happens once per coefficient. The answer is the same.
pub fn rational_polynomial_cmp_naive(p: &RationalPolynomial, q: &RationalPolynomial) -> Ordering {
    let ps = p.to_coefficients_asc();
    let qs = q.to_coefficients_asc();
    match ps.len().cmp(&qs.len()) {
        Equal => ps.iter().rev().cmp(qs.iter().rev()),
        Greater => ps.last().unwrap().sign(),
        Less => qs.last().unwrap().sign().reverse(),
    }
}

/// The shortlex comparison, worked out by materializing every coefficient as a
/// [`Rational`](crate::Rational).
pub fn rational_polynomial_shortlex_cmp_naive(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
) -> Ordering {
    let ps = p.to_coefficients_asc();
    let qs = q.to_coefficients_asc();
    match ps.len().cmp(&qs.len()) {
        Equal => ps.iter().rev().cmp(qs.iter().rev()),
        c => c,
    }
}

/// The coefficients of $bP - aQ$, where `p` is $P/a$ and `q` is $Q/b$.
///
/// The difference $p - q$ is this polynomial over $ab$, which is positive, so the sign of this
/// polynomial's leading coefficient is the asymptotic comparison. Polynomial arithmetic is not
/// implemented yet, so the coefficients are built one at a time.
pub fn rational_polynomial_difference_numerator(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
) -> Vec<Integer> {
    let a = Integer::from(p.denominator_ref());
    let b = Integer::from(q.denominator_ref());
    let ps = p.numerator_ref().coefficients_asc();
    let qs = q.numerator_ref().coefficients_asc();
    let mut ds = Vec::new();
    for i in 0..max(ps.len(), qs.len()) {
        let x = ps.get(i).map_or_else(|| Integer::ZERO, Clone::clone);
        let y = qs.get(i).map_or_else(|| Integer::ZERO, Clone::clone);
        ds.push(&b * x - &a * y);
    }
    while ds.last() == Some(&Integer::ZERO) {
        ds.pop();
    }
    ds
}

/// The asymptotic comparison, worked out from its definition rather than from any rule about
/// degrees and coefficients.
///
/// The difference is evaluated past its largest root, where its sign is its leading coefficient's
/// and no longer changes; one more than the largest coefficient of $bP - aQ$ is such a point, since
/// there the leading term outgrows the sum of everything below it.
pub fn rational_polynomial_cmp_evaluated(
    p: &RationalPolynomial,
    q: &RationalPolynomial,
) -> Ordering {
    let ds = rational_polynomial_difference_numerator(p, q);
    if ds.is_empty() {
        return Equal;
    }
    let bound = Integer::ONE
        + ds.iter()
            .map(|d| Integer::from(d.unsigned_abs_ref()))
            .max()
            .unwrap();
    let mut sum = Integer::ZERO;
    for d in ds.iter().rev() {
        sum *= &bound;
        sum += d;
    }
    sum.sign()
}
