// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{DivExact, Gcd};
use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;

/// Clears denominators without computing a least common multiple: every element is scaled by the
/// product of all the denominators, and the result is then divided by the gcd of that product and
/// all the scaled numerators, which leaves the least common denominator.
pub fn rational_vector_to_numerators_and_denominator_naive(
    v: &RationalVector,
) -> (IntegerVector, Natural) {
    let mut product = Natural::ONE;
    for x in &v.elements {
        product *= x.denominator_ref();
    }
    let scaled: Vec<Integer> = v
        .elements
        .iter()
        .map(|x| {
            let m = (&product).div_exact(x.denominator_ref()) * x.numerator_ref();
            Integer::from_sign_and_abs(*x >= 0u32, m)
        })
        .collect();
    let mut g = product.clone();
    for n in &scaled {
        g = g.gcd(n.unsigned_abs_ref());
    }
    let numerators = scaled
        .into_iter()
        .map(|n| n.div_exact(Integer::from(&g)))
        .collect();
    (
        IntegerVector {
            elements: numerators,
        },
        product.div_exact(g),
    )
}
