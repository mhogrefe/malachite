// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use alloc::vec;

impl<T: Into<Integer>> From<T> for IntegerPolynomial {
    /// Converts a value to a constant [`IntegerPolynomial`].
    ///
    /// This works for anything a [`Integer`] can be converted from, and for a [`Integer`] itself.
    /// The polynomial is the constant one, whose only coefficient is the value; zero becomes the
    /// zero polynomial, which has no coefficients at all.
    ///
    /// $f(x) = x$, read on the left as a number and on the right as a polynomial.
    ///
    /// # Worst-case complexity
    /// Same as the time and additional memory complexity of converting the value to a [`Integer`].
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(IntegerPolynomial::from(123u32).to_string(), "123");
    /// assert_eq!(IntegerPolynomial::from(true).to_string(), "1");
    /// assert_eq!(
    ///     IntegerPolynomial::from(Integer::from(10u32).pow(20)).to_string(),
    ///     "100000000000000000000"
    /// );
    ///
    /// // Zero is the zero polynomial, which has no coefficients.
    /// assert_eq!(IntegerPolynomial::from(0u32).to_string(), "0");
    /// ```
    #[inline]
    fn from(x: T) -> Self {
        Self::from_coefficients_asc(vec![x.into()])
    }
}
