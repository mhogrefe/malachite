// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_polynomial::IntegerPolynomial;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::L2NormSquared;

impl L2NormSquared for &IntegerPolynomial {
    type Output = Natural;

    /// Computes the sum of the squares of an [`IntegerPolynomial`]'s coefficients, which is the
    /// square of its $L^2$ norm.
    ///
    /// $$
    /// f(p) = \sum_i p_i^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::L2NormSquared;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2-3*x+2")
    ///         .unwrap()
    ///         .l2_norm_squared(),
    ///     14u32
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("0").unwrap().l2_norm_squared(),
    ///     0u32
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("-5").unwrap().l2_norm_squared(),
    ///     25u32
    /// );
    /// ```
    ///
    /// This is the dot product of the coefficients with themselves that `_fmpz_poly_2norm` in
    /// `fmpz_poly/2norm.c`, FLINT 3.6.0, computes before taking the square root.
    fn l2_norm_squared(self) -> Natural {
        let mut sum = Natural::ZERO;
        for c in &self.coefficients {
            let a = c.unsigned_abs_ref();
            sum.add_mul_assign(a, a);
        }
        sum
    }
}
