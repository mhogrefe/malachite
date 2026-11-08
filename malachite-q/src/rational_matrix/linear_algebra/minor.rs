// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::linear_algebra::traits::{Determinant, Minor};

// Panics unless the matrix is square.
pub(crate) fn assert_square(m: &RationalMatrix) {
    assert!(m.is_square(), "minors are only defined for square matrices");
}

impl Minor for RationalMatrix {
    type Output = Rational;

    /// Computes the minor of a square [`RationalMatrix`] at row `i` and column `j`, taking the
    /// matrix by value.
    ///
    /// The minor is the determinant of the matrix obtained by deleting row `i` and column `j`:
    ///
    /// $$
    /// f(A, i, j) = M_{i, j} = \det A_{(i, j)}.
    /// $$
    ///
    /// The minor of a $1 \times 1$ matrix is 1, the determinant of the $0 \times 0$ matrix.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of [`determinant`](Determinant::determinant) for a square
    /// [`RationalMatrix`] of height $n - 1$ with the same elements, where $n$ is the height of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if `i` or `j` is not less than its height.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::Minor;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// assert_eq!(
    ///     RationalMatrix::from_str("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]")
    ///         .unwrap()
    ///         .minor(0, 0)
    ///         .to_string(),
    ///     "-33"
    /// );
    /// assert_eq!(
    ///     RationalMatrix::from_str("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]")
    ///         .unwrap()
    ///         .minor(1, 2)
    ///         .to_string(),
    ///     "10"
    /// );
    /// assert_eq!(
    ///     RationalMatrix::from_str("[5]")
    ///         .unwrap()
    ///         .minor(0, 0)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    fn minor(self, i: u64, j: u64) -> Rational {
        assert_square(&self);
        self.into_without_row_and_column(i, j).determinant()
    }
}

impl Minor for &RationalMatrix {
    type Output = Rational;

    /// Computes the minor of a square [`RationalMatrix`] at row `i` and column `j`, taking the
    /// matrix by reference.
    ///
    /// The minor is the determinant of the matrix obtained by deleting row `i` and column `j`:
    ///
    /// $$
    /// f(A, i, j) = M_{i, j} = \det A_{(i, j)}.
    /// $$
    ///
    /// The minor of a $1 \times 1$ matrix is 1, the determinant of the $0 \times 0$ matrix.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of [`determinant`](Determinant::determinant) for a square
    /// [`RationalMatrix`] of height $n - 1$ with the same elements, where $n$ is the height of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if `i` or `j` is not less than its height.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::Minor;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let m = RationalMatrix::from_str("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]").unwrap();
    /// assert_eq!((&m).minor(0, 0).to_string(), "-33");
    /// assert_eq!((&m).minor(1, 2).to_string(), "10");
    /// assert_eq!(
    ///     RationalMatrix::from_str("[5]")
    ///         .unwrap()
    ///         .minor(0, 0)
    ///         .to_string(),
    ///     "1"
    /// );
    /// ```
    fn minor(self, i: u64, j: u64) -> Rational {
        assert_square(self);
        self.without_row_and_column(i, j).determinant()
    }
}
