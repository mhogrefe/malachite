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
use malachite_base::num::arithmetic::traits::Determinant;

impl Determinant for NaturalMatrix {
    type Output = Integer;

    /// Computes the determinant of a square [`NaturalMatrix`], taking it by value.
    ///
    /// The determinant may be negative, so it is an [`Integer`].
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `determinant` for an [`IntegerMatrix`] with the same elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Determinant;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.determinant().to_string(), "-2");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(a.determinant().to_string(), "-3");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.determinant().to_string(), "0");
    /// ```
    #[inline]
    fn determinant(self) -> Integer {
        IntegerMatrix::from(self).determinant()
    }
}

impl Determinant for &NaturalMatrix {
    type Output = Integer;

    /// Computes the determinant of a square [`NaturalMatrix`], taking it by reference.
    ///
    /// The determinant may be negative, so it is an [`Integer`].
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `determinant` for an [`IntegerMatrix`] with the same elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Determinant;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).determinant().to_string(), "-2");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!((&a).determinant().to_string(), "-3");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!((&a).determinant().to_string(), "0");
    /// ```
    #[inline]
    fn determinant(self) -> Integer {
        IntegerMatrix::from(self).determinant()
    }
}
