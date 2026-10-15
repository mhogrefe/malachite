// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{AbsSquared, FloorSqrt, Square};
use malachite_nz::natural::Natural;

// The floor of the square root of a natural number, if it is exact.
fn exact_sqrt(n: &Natural) -> Option<Natural> {
    let r = n.floor_sqrt();
    if (&r).square() == *n { Some(r) } else { None }
}

// The norm, found by taking the floor of the square roots of the numerator and denominator of the
// squared norm and checking whether they square back. Nothing is shared with the implementation.
pub fn rational_vector_checked_abs_naive(v: &RationalVector) -> Option<Rational> {
    let s = v.abs_squared();
    let (n, d) = s.numerator_and_denominator_ref();
    Some(Rational::from_naturals(exact_sqrt(n)?, exact_sqrt(d)?))
}
