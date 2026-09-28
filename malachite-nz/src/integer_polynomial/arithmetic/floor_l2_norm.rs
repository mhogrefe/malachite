// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::FloorSqrt;
use malachite_base::polynomial::{FloorL2Norm, L2NormSquared};

impl FloorL2Norm for &IntegerPolynomial {
    type Output = Natural;

    /// Computes the floor of an [`IntegerPolynomial`]'s $L^2$ norm: the floor of the square root of
    /// the sum of the squares of its coefficients.
    ///
    /// $$
    /// f(p) = \left \lfloor \sqrt{\sum_i p_i^2} \right \rfloor.
    /// $$
    ///
    /// The exact square of the norm is [`l2_norm_squared`](L2NormSquared::l2_norm_squared).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::FloorL2Norm;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2-3*x+2")
    ///         .unwrap()
    ///         .floor_l2_norm(),
    ///     3u32
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("0").unwrap().floor_l2_norm(),
    ///     0u32
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("-5").unwrap().floor_l2_norm(),
    ///     5u32
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_2norm` from `fmpz_poly/2norm.c`, FLINT 3.6.0.
    #[inline]
    fn floor_l2_norm(self) -> Natural {
        self.l2_norm_squared().floor_sqrt()
    }
}
