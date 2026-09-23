// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_polynomial::RationalPolynomial;
use alloc::vec;
use malachite_base::polynomial::Polynomial;

impl<T: Into<Rational>> From<T> for RationalPolynomial {
    /// Converts a value to a constant [`RationalPolynomial`].
    ///
    /// This works for anything a [`Rational`] can be converted from, and for a [`Rational`] itself.
    /// The polynomial is the constant one, whose only coefficient is the value; zero becomes the
    /// zero polynomial, which has no coefficients at all.
    ///
    /// $f(x) = x$, read on the left as a number and on the right as a polynomial.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of converting the value to a [`Rational`].
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use malachite_q::rational_polynomial::RationalPolynomial;
    ///
    /// assert_eq!(RationalPolynomial::from(123u32).to_string(), "123");
    /// assert_eq!(RationalPolynomial::from(true).to_string(), "1");
    /// assert_eq!(RationalPolynomial::from(-5i32).to_string(), "-5");
    /// assert_eq!(
    ///     RationalPolynomial::from(Rational::from_signeds(1, 3)).to_string(),
    ///     "1/3"
    /// );
    ///
    /// // Zero is the zero polynomial, which has no coefficients.
    /// assert_eq!(RationalPolynomial::from(0u32).to_string(), "0");
    /// ```
    #[inline]
    fn from(x: T) -> Self {
        Self::from_coefficients_asc(vec![x.into()])
    }
}
