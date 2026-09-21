// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use malachite_base::u64_polynomial::U64Polynomial;

impl From<U64Polynomial> for IntegerPolynomial {
    /// Converts a [`U64Polynomial`] to an [`IntegerPolynomial`].
    ///
    /// Every [`u64`] is an [`Integer`], so nothing is lost and nothing can fail. The coefficients
    /// are converted one by one, and the leading one stays nonzero, so the degree is unchanged.
    ///
    /// $f(p) = p$, read on the left over the [`u64`]s and on the right over $\Z$.
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
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = U64Polynomial::from_str("x^2+3*x+2").unwrap();
    /// assert_eq!(IntegerPolynomial::from(p).to_string(), "x^2+3*x+2");
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from(U64Polynomial::default()).to_string(),
    ///     "0"
    /// );
    /// ```
    #[inline]
    fn from(p: U64Polynomial) -> Self {
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
