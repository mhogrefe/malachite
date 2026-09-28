// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use alloc::vec::Vec;
use core::ops::Shl;
use malachite_base::polynomial::Polynomial;

// Shifts every coefficient, read one at a time through `coefficient`, as a `Rational`, and rebuilds
// the polynomial from the results. Nothing is shared with the implementation.
pub fn shl_naive<T: Copy>(p: &RationalPolynomial, bits: T) -> RationalPolynomial
where
    Rational: Shl<T, Output = Rational>,
{
    RationalPolynomial::from_coefficients_asc(
        (0..p.len())
            .map(|i| p.coefficient(i) << bits)
            .collect::<Vec<Rational>>(),
    )
}
