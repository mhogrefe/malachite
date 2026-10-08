// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{DivExact, LcmAssign};
use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;

/// Clears the denominators of a [`RationalVector`], returning a vector of [`Integer`] numerators
/// and a single positive [`Natural`] denominator whose quotients are the elements.
///
/// The denominator is the least common multiple of the elements' denominators, the smallest that
/// works, and each numerator is its element times that denominator. Because the elements are in
/// lowest terms, so is the result: no prime divides the denominator and every numerator. The
/// 0-dimensional vector gives the denominator 1. This is FLINT's `_fmpq_vec_get_fmpz_vec_fmpz`.
///
/// This is the form in which a sequence of vector operations over one denominator can be done in
/// integer arithmetic;
/// [`from_numerators_and_denominator`](
/// super::from_numerators_and_denominator::from_numerators_and_denominator)
/// converts back.
///
/// # Worst-case complexity
/// $T(n, m) = O(mn (\log n)^2 \log\log n)$
///
/// $M(n, m) = O(mn)$
///
/// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements'
/// numerators and denominators, and $m$ is the dimension. The denominator can have nearly $n$ bits,
/// and each numerator is scaled up to it.
///
/// # Examples
/// ```
/// use core::str::FromStr;
/// use malachite_q::rational_vector::RationalVector;
/// use malachite_q::rational_vector::conversion::to_numerators_and_denominator::*;
///
/// let v = RationalVector::from_str("(1/2, -2/3, 5)").unwrap();
/// let (ns, d) = to_numerators_and_denominator(&v);
/// assert_eq!(ns.to_string(), "(3, -4, 30)");
/// assert_eq!(d, 6);
///
/// // The denominator is the least common multiple, not the product.
/// let v = RationalVector::from_str("(1/4, 1/6)").unwrap();
/// let (ns, d) = to_numerators_and_denominator(&v);
/// assert_eq!(ns.to_string(), "(3, 2)");
/// assert_eq!(d, 12);
///
/// let v = RationalVector::from_str("()").unwrap();
/// let (ns, d) = to_numerators_and_denominator(&v);
/// assert_eq!(ns.to_string(), "()");
/// assert_eq!(d, 1);
/// ```
pub fn to_numerators_and_denominator(v: &RationalVector) -> (IntegerVector, Natural) {
    let mut d = Natural::ONE;
    for x in &v.elements {
        d.lcm_assign(x.denominator_ref());
    }
    let numerators = v
        .elements
        .iter()
        .map(|x| {
            Integer::from_sign_and_abs(
                *x >= 0u32,
                (&d).div_exact(x.denominator_ref()) * x.numerator_ref(),
            )
        })
        .collect();
    (
        IntegerVector {
            elements: numerators,
        },
        d,
    )
}
