// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use malachite_nz::integer_matrix::IntegerMatrix;

impl From<IntegerMatrix> for RationalMatrix {
    /// Converts an [`IntegerMatrix`] to a [`RationalMatrix`].
    ///
    /// Every [`Integer`](malachite_nz::integer::Integer) is a [`Rational`], so nothing is lost and
    /// nothing can fail. The elements are converted one by one, so the height and width are
    /// unchanged.
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let m = IntegerMatrix::from_str("[1, -2, 3; -4, 5, -6]").unwrap();
    /// assert_eq!(RationalMatrix::from(m).to_string(), "[1, -2, 3; -4, 5, -6]");
    ///
    /// assert_eq!(
    ///     RationalMatrix::from(IntegerMatrix::from_str("[]_{0,2}").unwrap()).to_string(),
    ///     "[]_{0,2}"
    /// );
    /// ```
    #[inline]
    fn from(m: IntegerMatrix) -> Self {
        let (height, width) = (m.height(), m.width());
        Self {
            elements: m.elements.into_iter().map(Rational::from).collect(),
            height,
            width,
        }
    }
}
