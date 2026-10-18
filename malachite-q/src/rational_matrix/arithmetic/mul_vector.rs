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
    (a, d): (IntegerMatrix, Natural),
    (v, e): (IntegerVector, Natural),
) -> RationalVector {
    let denominator = Integer::from(d * e);
    RationalVector::from_owned_elements(
        (a * v)
            .elements
            .into_iter()
            .map(|x| Rational::from_integers_ref(&x, &denominator))
            .collect(),
    )
}

fn assert_matrix_vector_compatible(a: &RationalMatrix, v: &RationalVector) {
    assert_eq!(
        v.elements.len(),
        a.width,
        "the vector's dimension must equal the width of the matrix"
    );
}

impl Mul<RationalVector> for RationalMatrix {
    type Output = RationalVector;

    /// Multiplies a [`RationalMatrix`] by a [`RationalVector`] on the right, taking the matrix by
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
    /// Panics if the dimension of `v` is not the width of the matrix.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// let v = RationalVector::from_str("(2, 3, 1/2)").unwrap();
    /// assert_eq!((a * v).to_string(), "(4, 2)");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul_fmpq_vec` from `fmpq_mat/mul_fmpq_vec.c`, FLINT 3.6.0,
    /// except that FLINT ignores the elements of a longer vector, or treats a shorter one as padded
    /// with zeros, instead of panicking.
    #[inline]
    fn mul(self, other: RationalVector) -> RationalVector {
        &self * &other
    }
}

impl Mul<&RationalVector> for RationalMatrix {
    type Output = RationalVector;

    /// Multiplies a [`RationalMatrix`] by a [`RationalVector`] on the right, taking the matrix by
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// let v = RationalVector::from_str("(2, 3, 1/2)").unwrap();
    /// assert_eq!((a * &v).to_string(), "(4, 2)");
    /// ```
    #[inline]
    fn mul(self, other: &RationalVector) -> RationalVector {
        &self * other
    }
}

impl Mul<RationalVector> for &RationalMatrix {
    type Output = RationalVector;

    /// Multiplies a [`RationalMatrix`] by a [`RationalVector`] on the right, taking the matrix by
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// let v = RationalVector::from_str("(2, 3, 1/2)").unwrap();
    /// assert_eq!((&a * v).to_string(), "(4, 2)");
    /// ```
    #[inline]
    fn mul(self, other: RationalVector) -> RationalVector {
        self * &other
    }
}

impl Mul<&RationalVector> for &RationalMatrix {
    type Output = RationalVector;

    /// Multiplies a [`RationalMatrix`] by a [`RationalVector`] on the right, taking the matrix by
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
    /// use malachite_q::rational_matrix::RationalMatrix;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// let v = RationalVector::from_str("(2, 3, 1/2)").unwrap();
    /// assert_eq!((&a * &v).to_string(), "(4, 2)");
    /// ```
    fn mul(self, other: &RationalVector) -> RationalVector {
        assert_matrix_vector_compatible(self, other);
        mul_over_denominators(
            self.to_numerators_and_denominator(),
            other.to_numerators_and_denominator(),
        )
    }
}
