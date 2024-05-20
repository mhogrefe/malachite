// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{Reciprocal, RoundToMultiple};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::natural::exhaustive::exhaustive_natural_inclusive_range;
use malachite_nz::natural::Natural;

// Slow! Only use for small `max_denominator`s
pub fn approximate_naive(x: &Rational, max_denominator: &Natural) -> Rational {
    let mut nearest = Rational::ZERO;
    for d in exhaustive_natural_inclusive_range(Natural::ONE, max_denominator.clone()) {
        let q = x
            .round_to_multiple(Rational::from(d).reciprocal(), Nearest)
            .0;
        if (x - &q).lt_abs(&(x - &nearest)) {
            nearest = q;
        }
    }
    nearest
}
