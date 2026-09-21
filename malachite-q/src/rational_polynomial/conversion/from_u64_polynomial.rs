// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_polynomial::RationalPolynomial;
use malachite_base::u64_polynomial::U64Polynomial;
use malachite_nz::integer_polynomial::IntegerPolynomial;

impl From<U64Polynomial> for RationalPolynomial {
    /// Converts a [`U64Polynomial`] to a [`RationalPolynomial`].
    ///
    /// Every [`u64`] is a [`Rational`](crate::Rational), so nothing is lost and nothing can fail.
    /// The coefficients become the numerator and the denominator is 1.
    ///
    /// $f(p) = p$, read on the left over the [`u64`]s and on the right over $\Q$.
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
    /// use malachite_base::u64_polynomial::U64Polynomial;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// let p = U64Polynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(RationalPolynomial::from(p).to_string(), "x^2+3*x+2");
    ///
    /// assert_eq!(
    ///     RationalPolynomial::from(U64Polynomial::default()).to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn from(p: U64Polynomial) -> Self {
        Self::from(IntegerPolynomial::from(p))
    }
}
