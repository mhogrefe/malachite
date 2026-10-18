// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use crate::rational_vector::RationalVector;
use core::ops::Mul;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;

// A product of integers over denominators `d` and `e`, as rationals: one integer product and one
// reduction per element of the result.
fn mul_over_denominators(
    (v, d): (IntegerVector, Natural),
    (a, e): (IntegerMatrix, Natural),
) -> RationalVector {
    let denominator = Integer::from(d * e);
    RationalVector::from_owned_elements(
        (v * a)
            .elements
            .into_iter()
            .map(|x| Rational::from_integers_ref(&x, &denominator))
            .collect(),
    )
}

fn assert_vector_matrix_compatible(v: &RationalVector, a: &RationalMatrix) {
    assert_eq!(
        v.elements.len(),
        a.height,
        "the vector's dimension must equal the height of the matrix"
    );
}

impl Mul<RationalMatrix> for RationalVector {
    type Output = Self;

    /// Multiplies a [`RationalVector`] by a [`RationalMatrix`] on the right, taking the vector by
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
    /// The matrix and the vector are each written as integers over a common denominator, so the
    /// whole product is computed over the integers and each element of the result is reduced once.
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(2, 3/2)").unwrap();
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// assert_eq!((v * a).to_string(), "(1, 5/2, 3)");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_fmpq_vec_mul` from `fmpq_mat/fmpq_vec_mul.c`, FLINT 3.6.0,
    /// except that FLINT ignores the elements of a longer vector, or treats a shorter one as padded
    /// with zeros, instead of panicking.
    #[inline]
    fn mul(self, other: RationalMatrix) -> Self {
        &self * &other
    }
}

impl Mul<&RationalMatrix> for RationalVector {
    type Output = Self;

    /// Multiplies a [`RationalVector`] by a [`RationalMatrix`] on the right, taking the vector by
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(2, 3/2)").unwrap();
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// assert_eq!((v * &a).to_string(), "(1, 5/2, 3)");
    /// ```
    #[inline]
    fn mul(self, other: &RationalMatrix) -> Self {
        &self * other
    }
}

impl Mul<RationalMatrix> for &RationalVector {
    type Output = RationalVector;

    /// Multiplies a [`RationalVector`] by a [`RationalMatrix`] on the right, taking the vector by
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(2, 3/2)").unwrap();
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// assert_eq!((&v * a).to_string(), "(1, 5/2, 3)");
    /// ```
    #[inline]
    fn mul(self, other: RationalMatrix) -> RationalVector {
        self * &other
    }
}

impl Mul<&RationalMatrix> for &RationalVector {
    type Output = RationalVector;

    /// Multiplies a [`RationalVector`] by a [`RationalMatrix`] on the right, taking the vector by
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(2, 3/2)").unwrap();
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// assert_eq!((&v * &a).to_string(), "(1, 5/2, 3)");
    /// ```
    fn mul(self, other: &RationalMatrix) -> RationalVector {
        assert_vector_matrix_compatible(self, other);
        mul_over_denominators(
            self.to_numerators_and_denominator(),
            other.to_numerators_and_denominator(),
        )
    }
}
