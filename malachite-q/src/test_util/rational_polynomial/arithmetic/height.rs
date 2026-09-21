// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::arithmetic::traits::Height;
use malachite_base::num::basic::traits::One;
use malachite_nz::natural::Natural;

/// The height, worked out by materializing every coefficient as a [`Rational`](crate::Rational) and
/// taking the largest of their heights.
///
/// This is the definition written out. The real implementation reduces each numerator against the
/// shared denominator directly instead, which is the same work without building the
/// [`Rational`](crate::Rational)s.
pub fn rational_polynomial_height_naive(p: &RationalPolynomial) -> Natural {
    p.to_coefficients_asc()
        .iter()
        .map(Height::to_height)
        .max()
        .unwrap_or(Natural::ONE)
}
