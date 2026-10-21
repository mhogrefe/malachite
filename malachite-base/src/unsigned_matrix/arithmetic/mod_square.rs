// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModSquare, ModSquareAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::{ProductShape, matrix_mod_mul};
use alloc::vec::Vec;

// Checks the arguments of a square modulo `m`: `m` is positive, the matrix is square, and its
// elements are reduced.
fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) {
    assert_ne!(m, T::ZERO, "m cannot be zero");
    assert_eq!(
        a.height, a.width,
        "the matrix must be square, but it is {} by {}",
        a.height, a.width
    );
    assert!(
        a.mod_is_reduced(&m),
        "the matrix must be reduced mod m, but {a} has an element >= {m}"
    );
}

// The elements of the square of `a` modulo `m`, after checking the arguments: the product of the
// matrix with itself, as computed by `ModMul`. FLINT has no `nmod_mat_sqr`; squaring a matrix of
// words is no cheaper than multiplying two, since a square of an element costs as much as a
// product.
fn square_elements<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) -> Vec<T> {
    assert_arguments(a, m);
    let n = a.height;
    matrix_mod_mul(
        &a.elements,
        &a.elements,
        ProductShape {
            ar: n,
            br: n,
            bc: n,
        },
        m,
    )
}

impl<T: PrimitiveUnsigned> ModSquare<T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Squares a square [`UnsignedMatrix`] modulo $m$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $m$. The square $A^2 = AA$ of an
    /// $n \times n$ matrix $A$ is the $n \times n$ matrix whose element in row $i$ and column $j$
    /// is the dot product of row $i$ of $A$ with column $j$ of $A$, reduced modulo $m$:
    ///
    /// $$
    /// f(A, m)_{ij} = \left ( \sum_{l=0}^{n-1} A_{il}A_{lj} \right ) \bmod m.
    /// $$
    ///
    /// The square is the product of the matrix with itself, computed as by
    /// [`ModMul`](crate::num::arithmetic::traits::ModMul), with the classical algorithm or
    /// Strassen's.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.mod_square(9u8).to_string(), "[7, 1; 6, 4]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(a.mod_square(5u8).to_string(), "[4, 1, 3; 0, 4, 1; 1, 4, 1]");
    ///
    /// let a = UnsignedMatrix::<u32>::from_str("[4294967290, 2147483648; 5, 4294967289]").unwrap();
    /// assert_eq!(
    ///     a.mod_square(4294967291u32).to_string(),
    ///     "[2147483659, 2147483638; 4294967276, 2147483662]"
    /// );
    /// ```
    #[inline]
    fn mod_square(self, m: T) -> Self {
        (&self).mod_square(m)
    }
}

impl<T: PrimitiveUnsigned> ModSquare<T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Squares a square [`UnsignedMatrix`] modulo $m$, taking it by reference.
    ///
    /// See the documentation for the implementation that takes the matrix by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquare;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_square(9u8).to_string(), "[7, 1; 6, 4]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_square(5u8).to_string(),
    ///     "[4, 1, 3; 0, 4, 1; 1, 4, 1]"
    /// );
    ///
    /// let a = UnsignedMatrix::<u32>::from_str("[4294967290, 2147483648; 5, 4294967289]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_square(4294967291u32).to_string(),
    ///     "[2147483659, 2147483638; 4294967276, 2147483662]"
    /// );
    /// ```
    fn mod_square(self, m: T) -> UnsignedMatrix<T> {
        UnsignedMatrix {
            elements: square_elements(self, m),
            height: self.height,
            width: self.width,
        }
    }
}

impl<T: PrimitiveUnsigned> ModSquareAssign<T> for UnsignedMatrix<T> {
    /// Squares a square [`UnsignedMatrix`] modulo $m$ in place.
    ///
    /// See the documentation for the [`ModSquare`] implementation that takes the matrix by value
    /// for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, or if any of its elements is greater
    /// than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSquareAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_square_assign(9u8);
    /// assert_eq!(a.to_string(), "[7, 1; 6, 4]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// a.mod_square_assign(5u8);
    /// assert_eq!(a.to_string(), "[4, 1, 3; 0, 4, 1; 1, 4, 1]");
    ///
    /// let mut a =
    ///     UnsignedMatrix::<u32>::from_str("[4294967290, 2147483648; 5, 4294967289]").unwrap();
    /// a.mod_square_assign(4294967291u32);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[2147483659, 2147483638; 4294967276, 2147483662]"
    /// );
    /// ```
    fn mod_square_assign(&mut self, m: T) {
        let square = square_elements(self, m);
        // Keep the storage of `self`.
        self.elements.copy_from_slice(&square);
    }
}
