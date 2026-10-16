// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;

impl RationalMatrix {
    /// Determines whether a [`RationalMatrix`] is square, that is, whether its height equals its
    /// width.
    ///
    /// The 0-by-0 matrix is square; the other matrices with no elements, such as a 3-by-0 matrix,
    /// are not.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// assert!(
    ///     RationalMatrix::from_str("[1, 2; 3, 4]")
    ///         .unwrap()
    ///         .is_square()
    /// );
    /// assert!(
    ///     !RationalMatrix::from_str("[1, 2, 3; 4, 5, 6]")
    ///         .unwrap()
    ///         .is_square()
    /// );
    /// assert!(RationalMatrix::from_str("[]_{0,0}").unwrap().is_square());
    /// assert!(!RationalMatrix::from_str("[]_{3,0}").unwrap().is_square());
    /// ```
    #[inline]
    pub const fn is_square(&self) -> bool {
        self.height == self.width
    }
}
