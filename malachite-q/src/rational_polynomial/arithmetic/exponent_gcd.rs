// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::polynomial::{ExponentGcd, slice_exponent_gcd};

impl ExponentGcd for RationalPolynomial {
    /// Computes the greatest common divisor of the exponents at which a [`RationalPolynomial`] has
    /// nonzero coefficients.
    ///
    /// $$
    /// f(p) = \gcd \\{i : p_i \neq 0\\}.
    /// $$
    ///
    /// When the polynomial is not constant, this is the largest $k$ such that $p(x) = q(x^k)$ for
    /// some polynomial $q$, and
    /// [`compose_power_of_x`](malachite_base::polynomial::ComposePowerOfX::compose_power_of_x) with
    /// $k$ recovers $p$ from $q$. A constant polynomial, including zero, gives 0.
    ///
    /// The denominator makes no difference, so this is the same as for the numerator.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::polynomial::ExponentGcd;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^6-2/3*x^3+1")
    ///         .unwrap()
    ///         .exponent_gcd(),
    ///     3
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^4+x")
    ///         .unwrap()
    ///         .exponent_gcd(),
    ///     1
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("1/2*x^4")
    ///         .unwrap()
    ///         .exponent_gcd(),
    ///     4
    /// );
    /// assert_eq!(
    ///     RationalPolynomial::from_str("-5/3").unwrap().exponent_gcd(),
    ///     0
    /// );
    /// assert_eq!(RationalPolynomial::ZERO.exponent_gcd(), 0);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_deflation` from `fmpz_poly/deflation.c`, applied to the
    /// numerator, FLINT 3.6.0, except that a constant gives 0 rather than 1.
    #[inline]
    fn exponent_gcd(&self) -> u64 {
        slice_exponent_gcd(self.numerator.coefficients_asc(), |c| *c == 0u32)
    }
}
