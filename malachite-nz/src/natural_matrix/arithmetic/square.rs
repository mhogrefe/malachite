// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::square::{assert_square, matrix_square};
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};

impl Square for NaturalMatrix {
    type Output = Self;

    /// Squares a square [`NaturalMatrix`], taking it by value.
    ///
    /// The square $A^2 = AA$ of an $n \times n$ matrix $A$ is the $n \times n$ matrix whose element
    /// in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of $A$:
    ///
    /// $$
    /// f(A)_{ij} = \sum_{k=0}^{n-1} A_{ik}A_{kj}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the largest number of significant bits of any of its elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.square().to_string(), "[7, 10; 15, 22]");
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(a.square().to_string(), "[4, 1, 3; 5, 9, 1; 1, 4, 1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_sqr` from `fmpz_mat/sqr.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square, which FLINT does not check.
    #[inline]
    fn square(self) -> Self {
        (&self).square()
    }
}

impl Square for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Squares a square [`NaturalMatrix`], taking it by reference.
    ///
    /// The square $A^2 = AA$ of an $n \times n$ matrix $A$ is the $n \times n$ matrix whose element
    /// in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of $A$:
    ///
    /// $$
    /// f(A)_{ij} = \sum_{k=0}^{n-1} A_{ik}A_{kj}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the largest number of significant bits of any of its elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).square().to_string(), "[7, 10; 15, 22]");
    ///
    /// let a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!((&a).square().to_string(), "[4, 1, 3; 5, 9, 1; 1, 4, 1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_sqr` from `fmpz_mat/sqr.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square, which FLINT does not check.
    fn square(self) -> NaturalMatrix {
        assert_square(self);
        NaturalMatrix {
            elements: matrix_square(&self.elements, self.height),
            height: self.height,
            width: self.width,
        }
    }
}

impl SquareAssign for NaturalMatrix {
    /// Squares a square [`NaturalMatrix`] in place.
    ///
    /// The square $A^2 = AA$ of an $n \times n$ matrix $A$ is the $n \times n$ matrix whose element
    /// in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of $A$:
    ///
    /// $$
    /// f(A)_{ij} = \sum_{k=0}^{n-1} A_{ik}A_{kj}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the largest number of significant bits of any of its elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.square_assign();
    /// assert_eq!(a.to_string(), "[7, 10; 15, 22]");
    ///
    /// let mut a = NaturalMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// a.square_assign();
    /// assert_eq!(a.to_string(), "[4, 1, 3; 5, 9, 1; 1, 4, 1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_sqr` from `fmpz_mat/sqr.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square, which FLINT does not check.
    fn square_assign(&mut self) {
        assert_square(self);
        self.elements = matrix_square(&self.elements, self.height);
    }
}
