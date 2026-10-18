// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::vector_mul::vector_matrix_mul;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_vector::NaturalVector;
use core::ops::Mul;

fn assert_vector_matrix_compatible(v: &NaturalVector, a: &NaturalMatrix) {
    assert_eq!(
        v.elements.len(),
        a.height,
        "the vector's dimension must equal the height of the matrix"
    );
}

impl Mul<NaturalMatrix> for NaturalVector {
    type Output = Self;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right, taking the vector by
    /// value and the matrix by value.
    ///
    /// The product $vA$ is the vector whose element $j$ is the dot product of $v$ with column $j$
    /// of $A$:
    ///
    /// $$
    /// f(v, A)_j = \sum_{i=0}^{n-1} v_iA_{ij},
    /// $$
    ///
    /// where $n$ is the height of $A$, which must be the dimension of $v$. The result's dimension
    /// is the width of $A$.
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
    /// Panics if the dimension of `v` is not the height of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((v * a).to_string(), "(9, 12, 15)");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_fmpz_vec_mul` from `fmpz_mat/fmpz_vec_mul.c`, FLINT 3.6.0,
    /// except that FLINT ignores the elements of a longer vector, or treats a shorter one as padded
    /// with zeros, instead of panicking.
    #[inline]
    fn mul(self, other: NaturalMatrix) -> Self {
        &self * &other
    }
}

impl Mul<&NaturalMatrix> for NaturalVector {
    type Output = Self;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right, taking the vector by
    /// value and the matrix by reference.
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
    /// Panics if the dimension of `v` is not the height of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((v * &a).to_string(), "(9, 12, 15)");
    /// ```
    #[inline]
    fn mul(self, other: &NaturalMatrix) -> Self {
        &self * other
    }
}

impl Mul<NaturalMatrix> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right, taking the vector by
    /// reference and the matrix by value.
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
    /// Panics if the dimension of `v` is not the height of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((&v * a).to_string(), "(9, 12, 15)");
    /// ```
    #[inline]
    fn mul(self, other: NaturalMatrix) -> NaturalVector {
        self * &other
    }
}

impl Mul<&NaturalMatrix> for &NaturalVector {
    type Output = NaturalVector;

    /// Multiplies a [`NaturalVector`] by a [`NaturalMatrix`] on the right, taking the vector by
    /// reference and the matrix by reference.
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
    /// Panics if the dimension of `v` is not the height of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2)").unwrap();
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!((&v * &a).to_string(), "(9, 12, 15)");
    /// ```
    fn mul(self, other: &NaturalMatrix) -> NaturalVector {
        assert_vector_matrix_compatible(self, other);
        NaturalVector {
            elements: vector_matrix_mul(&self.elements, &other.elements, other.width),
        }
    }
}
