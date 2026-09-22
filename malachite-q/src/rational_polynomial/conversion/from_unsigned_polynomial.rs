// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::IntegerPolynomial;

impl<T: PrimitiveUnsigned> From<UnsignedPolynomial<T>> for RationalPolynomial
where
    Integer: From<T>,
{
    /// Converts a [`UnsignedPolynomial`] to a [`RationalPolynomial`].
    ///
    /// Every unsigned primitive is a [`Rational`](crate::Rational), so nothing is lost and nothing
    /// can fail. The coefficients become the numerator and the denominator is 1.
    ///
    /// $f(p) = p$, read on the left over the coefficients and on the right over $\Q$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u64>::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(RationalPolynomial::from(p).to_string(), "x^2+3*x+2");
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from(UnsignedPolynomial::<u64>::default()).to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn from(p: UnsignedPolynomial<T>) -> Self {
        Self::from(IntegerPolynomial::from(p))
    }
}
