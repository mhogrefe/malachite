// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::traits::{One, Zero};

fn assert_same_shape(a: &IntegerMatrix, b: &IntegerMatrix) {
    assert!(
        a.height == b.height && a.width == b.width,
        "cannot add-multiply matrices of different shapes"
    );
}

impl AddMul<Self, Integer> for IntegerMatrix {
    type Output = Self;

    /// Adds the product of an [`IntegerMatrix`] and an [`Integer`] to an [`IntegerMatrix`], taking
    /// all three by value.
    ///
    /// The operation is taken element by element, so the result has the same shape as the matrices.
    /// This special-cases multiplication by 0, 1, and $-1$.
    ///
    /// $$
    /// f(A, B, c) = A + cB, \quad (A + cB)_{ij} = a_{ij} + cb_{ij}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `b` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    ///
    /// This is equivalent to `fmpz_mat_scalar_addmul_fmpz` from `fmpz_mat/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn add_mul(mut self, b: Self, c: Integer) -> Self {
        self.add_mul_assign(b, &c);
        self
    }
}

impl<'b> AddMul<Self, &'b Integer> for IntegerMatrix {
    type Output = Self;

    /// Adds the product of an [`IntegerMatrix`] and an [`Integer`] to an [`IntegerMatrix`], taking
    /// the first matrix by value, the second matrix by value, and the scalar by reference.
    ///
    /// See the documentation for the [`AddMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `b` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    ///
    /// This is equivalent to `fmpz_mat_scalar_addmul_fmpz` from `fmpz_mat/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn add_mul(mut self, b: Self, c: &'b Integer) -> Self {
        self.add_mul_assign(b, c);
        self
    }
}

impl<'a> AddMul<&'a Self, Integer> for IntegerMatrix {
    type Output = Self;

    /// Adds the product of an [`IntegerMatrix`] and an [`Integer`] to an [`IntegerMatrix`], taking
    /// the first matrix by value, the second matrix by reference, and the scalar by value.
    ///
    /// See the documentation for the [`AddMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `b` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    ///
    /// This is equivalent to `fmpz_mat_scalar_addmul_fmpz` from `fmpz_mat/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn add_mul(mut self, b: &'a Self, c: Integer) -> Self {
        self.add_mul_assign(b, &c);
        self
    }
}

impl<'a, 'b> AddMul<&'a Self, &'b Integer> for IntegerMatrix {
    type Output = Self;

    /// Adds the product of an [`IntegerMatrix`] and an [`Integer`] to an [`IntegerMatrix`], taking
    /// the first matrix by value, the second matrix by reference, and the scalar by reference.
    ///
    /// See the documentation for the [`AddMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `b` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    ///
    /// This is equivalent to `fmpz_mat_scalar_addmul_fmpz` from `fmpz_mat/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn add_mul(mut self, b: &'a Self, c: &'b Integer) -> Self {
        self.add_mul_assign(b, c);
        self
    }
}

impl AddMul<&IntegerMatrix, &Integer> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Adds the product of an [`IntegerMatrix`] and an [`Integer`] to an [`IntegerMatrix`], taking
    /// all three by reference.
    ///
    /// See the documentation for the [`AddMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `b` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    ///
    /// This is equivalent to `fmpz_mat_scalar_addmul_fmpz` from `fmpz_mat/scalar.c`, FLINT 3.6.0.
    fn add_mul(self, b: &IntegerMatrix, c: &Integer) -> IntegerMatrix {
        assert_same_shape(self, b);
        match *c {
            integer_zero!() => self.clone(),
            integer_one!() => self + b,
            integer_negative_one!() => self - b,
            _ => IntegerMatrix {
                height: self.height,
                width: self.width,
                elements: self
                    .elements
                    .iter()
                    .zip(&b.elements)
                    .map(|(x, y)| x.add_mul(y, c))
                    .collect(),
            },
        }
    }
}

impl AddMulAssign<Self, Integer> for IntegerMatrix {
    /// Adds the product of an [`IntegerMatrix`] and an [`Integer`] to an [`IntegerMatrix`] in
    /// place, taking the matrix and the scalar on the right-hand side by value.
    ///
    /// The operation is taken element by element. This special-cases multiplication by 0, 1, and
    /// $-1$.
    ///
    /// $$
    /// A \gets A + cB.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `b` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul_assign).
    ///
    /// This is equivalent to `fmpz_mat_scalar_addmul_fmpz` from `fmpz_mat/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn add_mul_assign(&mut self, b: Self, c: Integer) {
        self.add_mul_assign(b, &c);
    }
}

impl<'b> AddMulAssign<Self, &'b Integer> for IntegerMatrix {
    /// Adds the product of an [`IntegerMatrix`] and an [`Integer`] to an [`IntegerMatrix`] in
    /// place, taking the matrix on the right-hand side by value and the scalar by reference.
    ///
    /// See the documentation for the [`AddMulAssign`] implementation that takes both by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `b` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul_assign).
    ///
    /// This is equivalent to `fmpz_mat_scalar_addmul_fmpz` from `fmpz_mat/scalar.c`, FLINT 3.6.0.
    fn add_mul_assign(&mut self, b: Self, c: &'b Integer) {
        assert_same_shape(self, &b);
        match *c {
            integer_zero!() => {}
            integer_one!() => *self += b,
            integer_negative_one!() => *self -= b,
            _ => {
                for (x, y) in self.elements.iter_mut().zip(b.elements) {
                    x.add_mul_assign(y, c);
                }
            }
        }
    }
}

impl<'a> AddMulAssign<&'a Self, Integer> for IntegerMatrix {
    /// Adds the product of an [`IntegerMatrix`] and an [`Integer`] to an [`IntegerMatrix`] in
    /// place, taking the matrix on the right-hand side by reference and the scalar by value.
    ///
    /// See the documentation for the [`AddMulAssign`] implementation that takes both by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `b` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul_assign).
    ///
    /// This is equivalent to `fmpz_mat_scalar_addmul_fmpz` from `fmpz_mat/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn add_mul_assign(&mut self, b: &'a Self, c: Integer) {
        self.add_mul_assign(b, &c);
    }
}

impl<'a, 'b> AddMulAssign<&'a Self, &'b Integer> for IntegerMatrix {
    /// Adds the product of an [`IntegerMatrix`] and an [`Integer`] to an [`IntegerMatrix`] in
    /// place, taking the matrix and the scalar on the right-hand side by reference.
    ///
    /// See the documentation for the [`AddMulAssign`] implementation that takes both by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `b` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul_assign).
    ///
    /// This is equivalent to `fmpz_mat_scalar_addmul_fmpz` from `fmpz_mat/scalar.c`, FLINT 3.6.0.
    fn add_mul_assign(&mut self, b: &'a Self, c: &'b Integer) {
        assert_same_shape(self, b);
        match *c {
            integer_zero!() => {}
            integer_one!() => *self += b,
            integer_negative_one!() => *self -= b,
            _ => {
                for (x, y) in self.elements.iter_mut().zip(&b.elements) {
                    x.add_mul_assign(y, c);
                }
            }
        }
    }
}
