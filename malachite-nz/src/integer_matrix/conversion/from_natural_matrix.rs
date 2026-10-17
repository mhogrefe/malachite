// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::matrix::Matrix;

impl From<NaturalMatrix> for IntegerMatrix {
    /// Converts a [`NaturalMatrix`] to an [`IntegerMatrix`].
    ///
    /// Every [`Natural`](crate::natural::Natural) is an [`Integer`], so nothing is lost and nothing
    /// can fail. The elements are converted one by one, so the height and width are unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.elements.len()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(IntegerMatrix::from(m).to_string(), "[1, 2, 3; 4, 5, 6]");
    ///
    /// assert_eq!(
    ///     IntegerMatrix::from(NaturalMatrix::from_str("[]_{0,2}").unwrap()).to_string(),
    ///     "[]_{0,2}"
    /// );
    /// ```
    #[inline]
    fn from(m: NaturalMatrix) -> Self {
        Self::from_owned_elements(
            m.height(),
            m.width(),
            m.elements.into_iter().map(Integer::from).collect(),
        )
    }
}
