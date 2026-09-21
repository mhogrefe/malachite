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
