// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::{RationalMatrix, write_rows};
use core::fmt::{Debug, Display, Formatter, Result};

impl Display for RationalMatrix {
    /// Converts a [`RationalMatrix`] to a [`String`].
    ///
    /// The elements of each row are separated by `, `, the rows are separated by `; `, and the
    /// whole is enclosed in square brackets, so the 2-by-2 matrix with rows $(1, 2)$ and $(3, 4)$
    /// is `[1, 2; 3, 4]`. A matrix with no rows or no columns has no elements to show its shape, so
    /// it is written as `[]` followed by its height and width: a 4-by-0 matrix is `[]_{4,0}`, and
    /// the 0-by-0 matrix is `[]_{0,0}`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the bits of the elements.
    ///
    /// # Examples
    /// ```
    /// use malachite_q::Rational;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let m = RationalMatrix::from_elements(2, 3, &[1, 2, 3, 4, 5, 6].map(Rational::from));
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 6]");
    ///
    /// let m = RationalMatrix::from_elements(3, 1, &[1, -2, 3].map(Rational::from));
    /// assert_eq!(m.to_string(), "[1; -2; 3]");
    ///
    /// let m = RationalMatrix::from_elements(
    ///     1,
    ///     2,
    ///     &[Rational::from_signeds(1, 2), Rational::from_signeds(-3, 4)],
    /// );
    /// assert_eq!(m.to_string(), "[1/2, -3/4]");
    ///
    /// assert_eq!(
    ///     RationalMatrix::from_elements(4, 0, &[]).to_string(),
    ///     "[]_{4,0}"
    /// );
    /// assert_eq!(
    ///     RationalMatrix::from_elements(0, 4, &[]).to_string(),
    ///     "[]_{0,4}"
    /// );
    /// assert_eq!(
    ///     RationalMatrix::from_elements(0, 0, &[]).to_string(),
    ///     "[]_{0,0}"
    /// );
    /// ```
    fn fmt(&self, f: &mut Formatter) -> Result {
        if self.elements.is_empty() {
            return write!(f, "[]_{{{},{}}}", self.height, self.width);
        }
        f.write_str("[")?;
        write_rows(self, f, ", ", "; ", Display::fmt)?;
        f.write_str("]")
    }
}

impl Debug for RationalMatrix {
    /// Converts a [`RationalMatrix`] to a [`String`].
    ///
    /// This is the same as the [`Display::fmt`] implementation, so that a collection of
    /// [`RationalMatrix`]s is written the same way its elements are displayed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the sum of the bits of the elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let ms = vec![
    ///     RationalMatrix::from_str("[1, 2; 3, 4]").unwrap(),
    ///     RationalMatrix::from_str("[]_{0,3}").unwrap(),
    /// ];
    /// assert_eq!(ms[0].to_debug_string(), "[1, 2; 3, 4]");
    /// assert_eq!(ms.to_debug_string(), "[[1, 2; 3, 4], []_{0,3}]");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}
