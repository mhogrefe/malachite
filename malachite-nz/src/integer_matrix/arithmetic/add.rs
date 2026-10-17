// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use core::ops::{Add, AddAssign};
use malachite_base::matrix::Matrix;

fn assert_same_shape(a: &IntegerMatrix, b: &IntegerMatrix) {
    assert!(
        a.height == b.height && a.width == b.width,
        "cannot add matrices of different shapes"
    );
}

impl Add<Self> for IntegerMatrix {
    type Output = Self;

    /// Adds two [`IntegerMatrix`]s, taking both by value.
    ///
    /// The sum is taken element by element, so the result has the same shape as the summands.
    ///
    /// $$
    /// f(A, B) = A + B.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different shapes.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// let b = IntegerMatrix::from_str("[-1, 5; 0, 4]").unwrap();
    /// assert_eq!((a + b).to_string(), "[0, 3; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn add(mut self, other: Self) -> Self {
        self += other;
        self
    }
}

impl Add<&Self> for IntegerMatrix {
    type Output = Self;

    /// Adds two [`IntegerMatrix`]s, taking the first by value and the second by reference.
    ///
    /// The sum is taken element by element, so the result has the same shape as the summands.
    ///
    /// $$
    /// f(A, B) = A + B.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different shapes.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// let b = IntegerMatrix::from_str("[-1, 5; 0, 4]").unwrap();
    /// assert_eq!((a + &b).to_string(), "[0, 3; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn add(mut self, other: &Self) -> Self {
        self += other;
        self
    }
}

impl Add<IntegerMatrix> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Adds two [`IntegerMatrix`]s, taking the first by reference and the second by value.
    ///
    /// The sum is taken element by element, so the result has the same shape as the summands.
    ///
    /// $$
    /// f(A, B) = A + B.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different shapes.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// let b = IntegerMatrix::from_str("[-1, 5; 0, 4]").unwrap();
    /// assert_eq!((&a + b).to_string(), "[0, 3; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn add(self, mut other: IntegerMatrix) -> IntegerMatrix {
        other += self;
        other
    }
}

impl Add<&IntegerMatrix> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Adds two [`IntegerMatrix`]s, taking both by reference.
    ///
    /// The sum is taken element by element, so the result has the same shape as the summands.
    ///
    /// $$
    /// f(A, B) = A + B.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different shapes.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// let b = IntegerMatrix::from_str("[-1, 5; 0, 4]").unwrap();
    /// assert_eq!((&a + &b).to_string(), "[0, 3; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    fn add(self, other: &IntegerMatrix) -> IntegerMatrix {
        assert_same_shape(self, other);
        IntegerMatrix::from_owned_elements(
            self.height(),
            self.width(),
            self.elements
                .iter()
                .zip(&other.elements)
                .map(|(x, y)| x + y)
                .collect(),
        )
    }
}

impl AddAssign<Self> for IntegerMatrix {
    /// Adds an [`IntegerMatrix`] to an [`IntegerMatrix`] in place, taking the [`IntegerMatrix`] on
    /// the right-hand side by value.
    ///
    /// The sum is taken element by element, so the result has the same shape as the summands.
    ///
    /// $$
    /// f(A, B) = A + B.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different shapes.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// a += IntegerMatrix::from_str("[-1, 5; 0, 4]").unwrap();
    /// assert_eq!(a.to_string(), "[0, 3; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    fn add_assign(&mut self, other: Self) {
        assert_same_shape(self, &other);
        for (x, y) in self.elements.iter_mut().zip(other.elements) {
            *x += y;
        }
    }
}

impl AddAssign<&Self> for IntegerMatrix {
    /// Adds an [`IntegerMatrix`] to an [`IntegerMatrix`] in place, taking the [`IntegerMatrix`] on
    /// the right-hand side by reference.
    ///
    /// The sum is taken element by element, so the result has the same shape as the summands.
    ///
    /// $$
    /// f(A, B) = A + B.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different shapes.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2; 3, 0]").unwrap();
    /// a += &IntegerMatrix::from_str("[-1, 5; 0, 4]").unwrap();
    /// assert_eq!(a.to_string(), "[0, 3; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_add` from `fmpz_vec/add.c`, FLINT 3.6.0.
    fn add_assign(&mut self, other: &Self) {
        assert_same_shape(self, other);
        for (x, y) in self.elements.iter_mut().zip(&other.elements) {
            *x += y;
        }
    }
}
