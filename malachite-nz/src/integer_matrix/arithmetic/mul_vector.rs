// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_vector::IntegerVector;
use crate::integer_vector::arithmetic::dot_general::vec_dot_general;
use alloc::vec::Vec;
use core::ops::Mul;

// The product of a `height`-by-`width` matrix, given by its elements in row-major order, and a
// vector of dimension `width`: element `i` of the result is the dot product of row `i` with the
// vector, computed by the same kernel as the dot products of vectors.
//
// This is equivalent to `fmpz_mat_mul_fmpz_vec` from `fmpz_mat/mul_fmpz_vec.c`, FLINT 3.6.0.
pub(crate) fn matrix_vector_mul<C: PolynomialCoefficient>(
    xs: &[C],
    height: usize,
    width: usize,
    ys: &[C],
) -> Vec<C> {
    (0..height)
        .map(|i| vec_dot_general(None, false, &xs[i * width..(i + 1) * width], ys, false))
        .collect()
}

fn assert_matrix_vector_compatible(a: &IntegerMatrix, v: &IntegerVector) {
    assert_eq!(
        v.elements.len(),
        a.width,
        "the vector's dimension must equal the width of the matrix"
    );
}

impl Mul<IntegerVector> for IntegerMatrix {
    type Output = IntegerVector;

    /// Multiplies an [`IntegerMatrix`] by an [`IntegerVector`] on the right, taking the matrix by
    /// value and the vector by value.
    ///
    /// The product $Av$ is the vector whose element $i$ is the dot product of row $i$ of $A$ with
    /// $v$:
    ///
    /// $$
    /// f(A, v)_i = \sum_{j=0}^{n-1} A_{ij}v_j,
    /// $$
    ///
    /// where $n$ is the width of $A$, which must be the dimension of $v$. The result's dimension is
    /// the height of $A$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the dimension of `v` is not the width of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let v = IntegerVector::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((a * v).to_string(), "(7, -8)");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul_fmpz_vec` from `fmpz_mat/mul_fmpz_vec.c`, FLINT 3.6.0,
    /// except that FLINT ignores the elements of a longer vector, or treats a shorter one as padded
    /// with zeros, instead of panicking.
    #[inline]
    fn mul(self, other: IntegerVector) -> IntegerVector {
        &self * &other
    }
}

impl Mul<&IntegerVector> for IntegerMatrix {
    type Output = IntegerVector;

    /// Multiplies an [`IntegerMatrix`] by an [`IntegerVector`] on the right, taking the matrix by
    /// value and the vector by reference.
    ///
    /// See the documentation for the [`Mul`] implementation that takes both arguments by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the dimension of `v` is not the width of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let v = IntegerVector::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((a * &v).to_string(), "(7, -8)");
    /// ```
    #[inline]
    fn mul(self, other: &IntegerVector) -> IntegerVector {
        &self * other
    }
}

impl Mul<IntegerVector> for &IntegerMatrix {
    type Output = IntegerVector;

    /// Multiplies an [`IntegerMatrix`] by an [`IntegerVector`] on the right, taking the matrix by
    /// reference and the vector by value.
    ///
    /// See the documentation for the [`Mul`] implementation that takes both arguments by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the dimension of `v` is not the width of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let v = IntegerVector::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((&a * v).to_string(), "(7, -8)");
    /// ```
    #[inline]
    fn mul(self, other: IntegerVector) -> IntegerVector {
        self * &other
    }
}

impl Mul<&IntegerVector> for &IntegerMatrix {
    type Output = IntegerVector;

    /// Multiplies an [`IntegerMatrix`] by an [`IntegerVector`] on the right, taking the matrix by
    /// reference and the vector by reference.
    ///
    /// See the documentation for the [`Mul`] implementation that takes both arguments by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the matrix,
    /// and $m$ is the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the dimension of `v` is not the width of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let v = IntegerVector::from_str("(1, 0, 2)").unwrap();
    /// assert_eq!((&a * &v).to_string(), "(7, -8)");
    /// ```
    fn mul(self, other: &IntegerVector) -> IntegerVector {
        assert_matrix_vector_compatible(self, other);
        IntegerVector {
            elements: matrix_vector_mul(&self.elements, self.height, self.width, &other.elements),
        }
    }
}
