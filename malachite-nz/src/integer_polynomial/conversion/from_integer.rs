// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::natural_polynomial::NaturalPolynomial;
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

impl From<NaturalPolynomial> for IntegerPolynomial {
    /// Converts a [`NaturalPolynomial`] to an [`IntegerPolynomial`].
    ///
    /// Every polynomial with [`Natural`](crate::natural::Natural) coefficients is one with
    /// [`Integer`](crate::integer::Integer) coefficients, so nothing is lost and nothing can fail.
    /// The coefficients are converted one by one, and the leading one stays nonzero, so the degree
    /// is unchanged.
    ///
    /// $f(p) = p$, read on the left over $\N$ and on the right over $\Z$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(IntegerPolynomial::from(p).to_string(), "x^2+3*x+2");
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from(NaturalPolynomial::default()).to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn from(p: NaturalPolynomial) -> Self {
        // The coefficients keep their order and their nonzero leading one, so no trimming is
        // needed; but going through the constructor costs one comparison and cannot be wrong.
        Self::from_coefficients_asc(
            p.into_coefficients_asc()
                .into_iter()
                .map(Integer::from)
                .collect(),
        )
    }
}
