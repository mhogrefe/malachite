// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::Gcd;
use malachite_base::polynomial::Polynomial;

// Takes the GCD of every exponent with a nonzero coefficient, one at a time, with no skipping and
// no early exit. Nothing is shared with the implementation.
pub fn exponent_gcd_naive(p: &NaturalPolynomial) -> u64 {
    (0..p.len())
        .filter(|&i| *p.coefficient(i) != 0u32)
        .fold(0, Gcd::gcd)
}
