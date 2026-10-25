// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::Rank;

impl Rank for NaturalMatrix {
    /// Computes the rank of a [`NaturalMatrix`]: the number of linearly independent rows, or
    /// equivalently columns, over the rationals, taking the matrix by value.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `rank` for an [`IntegerMatrix`] with the same elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Rank;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.rank(), 2);
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.rank(), 2);
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// assert_eq!(a.rank(), 1);
    /// ```
    #[inline]
    fn rank(self) -> u64 {
        IntegerMatrix::from(self).rank()
    }
}

impl Rank for &NaturalMatrix {
    /// Computes the rank of a [`NaturalMatrix`]: the number of linearly independent rows, or
    /// equivalently columns, over the rationals, taking the matrix by reference.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `rank` for an [`IntegerMatrix`] with the same elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Rank;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).rank(), 2);
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!((&a).rank(), 2);
    ///
    /// let a = NaturalMatrix::from_str("[2, 4; 1, 2]").unwrap();
    /// assert_eq!((&a).rank(), 1);
    /// ```
    #[inline]
    fn rank(self) -> u64 {
        IntegerMatrix::from(self).rank()
    }
}
