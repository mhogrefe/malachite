// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::{UnsignedMatrix, write_rows};
use core::fmt::{Debug, Display, Formatter, Result};

impl<T: PrimitiveUnsigned> Display for UnsignedMatrix<T> {
    /// Converts an [`UnsignedMatrix`] to a [`String`].
    ///
    /// The elements of each row are separated by `, `, the rows are separated by `; `, and the
    /// whole is enclosed in square brackets, so the 2-by-2 matrix with rows $(1, 2)$ and $(3, 4)$
    /// is `[1, 2; 3, 4]`. A matrix with no rows or no columns has no elements to show its shape, so
    /// it is written as `[]` followed by its height and width: a 4-by-0 matrix is `[]_{4,0}`, and
    /// the 0-by-0 matrix is `[]_{0,0}`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let m = UnsignedMatrix::from_elements(2, 3, &[1u64, 2, 3, 4, 5, 6]);
    /// assert_eq!(m.to_string(), "[1, 2, 3; 4, 5, 6]");
    ///
    /// let m = UnsignedMatrix::from_elements(3, 1, &[1u64, 2, 3]);
    /// assert_eq!(m.to_string(), "[1; 2; 3]");
    ///
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_elements(4, 0, &[]).to_string(),
    ///     "[]_{4,0}"
    /// );
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_elements(0, 4, &[]).to_string(),
    ///     "[]_{0,4}"
    /// );
    /// assert_eq!(
    ///     UnsignedMatrix::<u64>::from_elements(0, 0, &[]).to_string(),
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

impl<T: PrimitiveUnsigned> Debug for UnsignedMatrix<T> {
    /// Converts an [`UnsignedMatrix`] to a [`String`].
    ///
    /// This is the same as the [`Display::fmt`] implementation, so that a collection of
    /// [`UnsignedMatrix`]s is written the same way its elements are displayed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let ms = vec![
    ///     UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]").unwrap(),
    ///     UnsignedMatrix::<u64>::from_str("[]_{0,3}").unwrap(),
    /// ];
    /// assert_eq!(ms[0].to_debug_string(), "[1, 2; 3, 4]");
    /// assert_eq!(ms.to_debug_string(), "[[1, 2; 3, 4], []_{0,3}]");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        Display::fmt(self, f)
    }
}
