// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Square, ModPowerOf2SquareAssign,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::ProductShape;
use crate::unsigned_matrix::arithmetic::mod_power_of_2_mul_matrix::matrix_mod_power_of_2_mul;
use alloc::vec::Vec;

// Checks the arguments of a square modulo $2^k$: `pow` is at most the width of `T`, the matrix is
// square, and its elements are reduced.
fn assert_arguments<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most {}, but is {pow}",
        T::WIDTH
    );
    assert_eq!(
        a.height, a.width,
        "the matrix must be square, but it is {} by {}",
        a.height, a.width
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^{pow}, but {a} has an element >= 2^{pow}"
    );
}

// The elements of the square of `a` modulo $2^k$, after checking the arguments: the product of the
// matrix with itself, as computed by `ModPowerOf2Mul`. Squaring a matrix is no cheaper than
// multiplying two, since a square of an element costs as much as a product in word arithmetic.
fn square_elements<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, pow: u64) -> Vec<T> {
    assert_arguments(a, pow);
    let n = a.height;
    matrix_mod_power_of_2_mul(
        &a.elements,
        &a.elements,
        ProductShape {
            ar: n,
            br: n,
            bc: n,
        },
        pow,
    )
}

impl<T: PrimitiveUnsigned> ModPowerOf2Square for UnsignedMatrix<T> {
    type Output = Self;

    /// Squares a square [`UnsignedMatrix`] modulo $2^k$, taking it by value.
    ///
    /// The elements of the matrix must already be reduced modulo $2^k$. The square $A^2 = AA$ of an
    /// $n \times n$ matrix $A$ is the $n \times n$ matrix whose element in row $i$ and column $j$
    /// is the dot product of row $i$ of $A$ with column $j$ of $A$, reduced modulo $2^k$:
    ///
    /// $$
    /// f(A, k)_{ij} = \left ( \sum_{l=0}^{n-1} A_{il}A_{lj} \right ) \bmod 2^k.
    /// $$
    ///
    /// The square is the product of the matrix with itself, computed as by
    /// [`ModPowerOf2Mul`](crate::num::arithmetic::traits::ModPowerOf2Mul).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^3)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if the matrix is not square, or if any of its
    /// elements is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.mod_power_of_2_square(4).to_string(), "[7, 10; 15, 6]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(a.mod_power_of_2_square(3).to_string(), "[4, 1, 3; 5, 1, 1; 1, 4, 1]");
    /// ```
    #[inline]
    fn mod_power_of_2_square(self, pow: u64) -> Self {
        (&self).mod_power_of_2_square(pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Square for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Squares a square [`UnsignedMatrix`] modulo $2^k$, taking it by reference.
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
    /// Panics if `pow` is greater than `T::WIDTH`, if the matrix is not square, or if any of its
    /// elements is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Square;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_square(4).to_string(), "[7, 10; 15, 6]");
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_square(3).to_string(), "[4, 1, 3; 5, 1, 1; 1, 4, 1]");
    /// ```
    fn mod_power_of_2_square(self, pow: u64) -> UnsignedMatrix<T> {
        UnsignedMatrix {
            elements: square_elements(self, pow),
            height: self.height,
            width: self.width,
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SquareAssign for UnsignedMatrix<T> {
    /// Squares a square [`UnsignedMatrix`] modulo $2^k$ in place.
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
    /// Panics if `pow` is greater than `T::WIDTH`, if the matrix is not square, or if any of its
    /// elements is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SquareAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2; 3, 4]").unwrap();
    /// a.mod_power_of_2_square_assign(4);
    /// assert_eq!(a.to_string(), "[7, 10; 15, 6]");
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[2, 0, 1; 1, 3, 0; 0, 1, 1]").unwrap();
    /// a.mod_power_of_2_square_assign(3);
    /// assert_eq!(a.to_string(), "[4, 1, 3; 5, 1, 1; 1, 4, 1]");
    /// ```
    fn mod_power_of_2_square_assign(&mut self, pow: u64) {
        let square = square_elements(self, pow);
        // Keep the storage of `self`.
        self.elements.copy_from_slice(&square);
    }
}
