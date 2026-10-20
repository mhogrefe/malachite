// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2016 Marco Bodrato
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::mul::{ProductShape, matrix_mul};
use crate::integer_matrix::arithmetic::square::bodrato::matrix_square_bodrato;
use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};

/// Bodrato's algorithm.
pub mod bodrato;

// Returns the elements of the square of the `n` by `n` matrix with elements `xs`, in row-major
// order, choosing between Bodrato's algorithm and multiplying the matrix by itself.
//
// This is equivalent to `fmpz_mat_sqr` from `fmpz_mat/sqr.c`, FLINT 3.6.0.
crate_test_fn! {matrix_square<C: PolynomialCoefficient>(xs: &[C], n: usize) -> Vec<C> {
    if n <= 3 || n == 4 && vec_max_bits(xs).0 >= 1024 {
        matrix_square_bodrato(xs, n)
    } else {
        matrix_mul(
            xs,
            xs,
            ProductShape {
                ar: n,
                br: n,
                bc: n,
            },
        )
    }
}}

pub(crate) fn assert_square<M: Matrix>(a: &M) {
    assert!(
        a.is_square(),
        "the matrix must be square, but it is {} by {}",
        a.height(),
        a.width()
    );
}

impl Square for IntegerMatrix {
    type Output = Self;

    /// Squares a square [`IntegerMatrix`], taking it by value.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// assert_eq!(a.square().to_string(), "[-5, -10; 15, 10]");
    ///
    /// let a = IntegerMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
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

impl Square for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Squares a square [`IntegerMatrix`], taking it by reference.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// assert_eq!((&a).square().to_string(), "[-5, -10; 15, 10]");
    ///
    /// let a = IntegerMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!((&a).square().to_string(), "[4, 1, 3; 5, 9, 1; 1, 4, 1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_sqr` from `fmpz_mat/sqr.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square, which FLINT does not check.
    fn square(self) -> IntegerMatrix {
        assert_square(self);
        IntegerMatrix {
            elements: matrix_square(&self.elements, self.height),
            height: self.height,
            width: self.width,
        }
    }
}

impl SquareAssign for IntegerMatrix {
    /// Squares a square [`IntegerMatrix`] in place.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// a.square_assign();
    /// assert_eq!(a.to_string(), "[-5, -10; 15, 10]");
    ///
    /// let mut a = IntegerMatrix::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
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
