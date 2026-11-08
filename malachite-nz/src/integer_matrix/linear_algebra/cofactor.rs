// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::linear_algebra::traits::{Cofactor, Minor};

impl Cofactor for IntegerMatrix {
    type Output = Integer;

    /// Computes the cofactor of a square [`IntegerMatrix`] at row `i` and column `j`, taking the
    /// matrix by value.
    ///
    /// $$
    /// f(A, i, j) = C_{i, j} = (-1)^{i + j} M_{i, j},
    /// $$
    ///
    /// where $M_{i, j}$ is the [`minor`](Minor::minor), so that $\det A = \sum_j A_{i, j} C_{i, j}$
    /// for every row $i$.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of [`minor`](Minor::minor).
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if `i` or `j` is not less than its height.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::Cofactor;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// assert_eq!(
    ///     IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; 7, -8, 9]")
    ///         .unwrap()
    ///         .cofactor(1, 2)
    ///         .to_string(),
    ///     "-6"
    /// );
    /// assert_eq!(
    ///     IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; 7, -8, 9]")
    ///         .unwrap()
    ///         .cofactor(0, 1)
    ///         .to_string(),
    ///     "-78"
    /// );
    /// assert_eq!(IntegerMatrix::from_str("[5]").unwrap().cofactor(0, 0), 1);
    /// ```
    fn cofactor(self, i: u64, j: u64) -> Integer {
        let m = self.minor(i, j);
        if (i + j).even() { m } else { -m }
    }
}

impl Cofactor for &IntegerMatrix {
    type Output = Integer;

    /// Computes the cofactor of a square [`IntegerMatrix`] at row `i` and column `j`, taking the
    /// matrix by reference.
    ///
    /// $$
    /// f(A, i, j) = C_{i, j} = (-1)^{i + j} M_{i, j},
    /// $$
    ///
    /// where $M_{i, j}$ is the [`minor`](Minor::minor), so that $\det A = \sum_j A_{i, j} C_{i, j}$
    /// for every row $i$.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of [`minor`](Minor::minor).
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if `i` or `j` is not less than its height.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::Cofactor;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let m = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; 7, -8, 9]").unwrap();
    /// assert_eq!((&m).cofactor(1, 2).to_string(), "-6");
    /// assert_eq!((&m).cofactor(0, 1).to_string(), "-78");
    /// assert_eq!(IntegerMatrix::from_str("[5]").unwrap().cofactor(0, 0), 1);
    /// ```
    fn cofactor(self, i: u64, j: u64) -> Integer {
        let m = self.minor(i, j);
        if (i + j).even() { m } else { -m }
    }
}
