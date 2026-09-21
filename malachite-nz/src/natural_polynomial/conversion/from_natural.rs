// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec;

impl<T: Into<Natural>> From<T> for NaturalPolynomial {
    /// Converts a value to a constant [`NaturalPolynomial`].
    ///
    /// This works for anything a [`Natural`] can be converted from, and for a [`Natural`] itself.
    /// The polynomial is the constant one, whose only coefficient is the value; zero becomes the
    /// zero polynomial, which has no coefficients at all.
    ///
    /// $f(x) = x$, read on the left as a number and on the right as a polynomial.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of converting the value to a [`Natural`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(NaturalPolynomial::from(123u32).degree(), Some(0));
    /// assert_eq!(*NaturalPolynomial::from(123u32).coefficient(0), 123);
    /// assert_eq!(
    ///     NaturalPolynomial::from(Natural::from(10u32).pow(20)).degree(),
    ///     Some(0)
    /// );
    /// assert_eq!(NaturalPolynomial::from(true).degree(), Some(0));
    ///
    /// // Zero is the zero polynomial, which has no coefficients.
    /// assert_eq!(NaturalPolynomial::from(0u32), NaturalPolynomial::default());
    /// ```
    #[inline]
    fn from(x: T) -> Self {
        Self::from_coefficients_asc(vec![x.into()])
    }
}
