// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};

fn assert_same_shape(a: &RationalMatrix, b: &RationalMatrix) {
    assert!(
        a.height == b.height && a.width == b.width,
        "cannot add-multiply matrices of different shapes"
    );
}

impl AddMul<Self, Rational> for RationalMatrix {
    type Output = Self;

    /// Adds the product of a [`RationalMatrix`] and a [`Rational`] to a [`RationalMatrix`], taking
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
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    #[inline]
    fn add_mul(mut self, b: Self, c: Rational) -> Self {
        self.add_mul_assign(b, &c);
        self
    }
}

impl<'b> AddMul<Self, &'b Rational> for RationalMatrix {
    type Output = Self;

    /// Adds the product of a [`RationalMatrix`] and a [`Rational`] to a [`RationalMatrix`], taking
    /// the first matrix by value, the second matrix by value, and the scalar by reference.
    ///
    /// See the documentation for the [`AddMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    #[inline]
    fn add_mul(mut self, b: Self, c: &'b Rational) -> Self {
        self.add_mul_assign(b, c);
        self
    }
}

impl<'a> AddMul<&'a Self, Rational> for RationalMatrix {
    type Output = Self;

    /// Adds the product of a [`RationalMatrix`] and a [`Rational`] to a [`RationalMatrix`], taking
    /// the first matrix by value, the second matrix by reference, and the scalar by value.
    ///
    /// See the documentation for the [`AddMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    #[inline]
    fn add_mul(mut self, b: &'a Self, c: Rational) -> Self {
        self.add_mul_assign(b, &c);
        self
    }
}

impl<'a, 'b> AddMul<&'a Self, &'b Rational> for RationalMatrix {
    type Output = Self;

    /// Adds the product of a [`RationalMatrix`] and a [`Rational`] to a [`RationalMatrix`], taking
    /// the first matrix by value, the second matrix by reference, and the scalar by reference.
    ///
    /// See the documentation for the [`AddMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    #[inline]
    fn add_mul(mut self, b: &'a Self, c: &'b Rational) -> Self {
        self.add_mul_assign(b, c);
        self
    }
}

impl AddMul<&RationalMatrix, &Rational> for &RationalMatrix {
    type Output = RationalMatrix;

    /// Adds the product of a [`RationalMatrix`] and a [`Rational`] to a [`RationalMatrix`], taking
    /// all three by reference.
    ///
    /// See the documentation for the [`AddMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    fn add_mul(self, b: &RationalMatrix, c: &Rational) -> RationalMatrix {
        assert_same_shape(self, b);
        match *c {
            Rational::ZERO => self.clone(),
            Rational::ONE => self + b,
            Rational::NEGATIVE_ONE => self - b,
            _ => RationalMatrix {
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

impl AddMulAssign<Self, Rational> for RationalMatrix {
    /// Adds the product of a [`RationalMatrix`] and a [`Rational`] to a [`RationalMatrix`] in
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
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul_assign).
    #[inline]
    fn add_mul_assign(&mut self, b: Self, c: Rational) {
        self.add_mul_assign(b, &c);
    }
}

impl<'b> AddMulAssign<Self, &'b Rational> for RationalMatrix {
    /// Adds the product of a [`RationalMatrix`] and a [`Rational`] to a [`RationalMatrix`] in
    /// place, taking the matrix on the right-hand side by value and the scalar by reference.
    ///
    /// See the documentation for the [`AddMulAssign`] implementation that takes both by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul_assign).
    fn add_mul_assign(&mut self, b: Self, c: &'b Rational) {
        assert_same_shape(self, &b);
        match *c {
            Rational::ZERO => {}
            Rational::ONE => *self += b,
            Rational::NEGATIVE_ONE => *self -= b,
            _ => {
                for (x, y) in self.elements.iter_mut().zip(b.elements) {
                    x.add_mul_assign(y, c);
                }
            }
        }
    }
}

impl<'a> AddMulAssign<&'a Self, Rational> for RationalMatrix {
    /// Adds the product of a [`RationalMatrix`] and a [`Rational`] to a [`RationalMatrix`] in
    /// place, taking the matrix on the right-hand side by reference and the scalar by value.
    ///
    /// See the documentation for the [`AddMulAssign`] implementation that takes both by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul_assign).
    #[inline]
    fn add_mul_assign(&mut self, b: &'a Self, c: Rational) {
        self.add_mul_assign(b, &c);
    }
}

impl<'a, 'b> AddMulAssign<&'a Self, &'b Rational> for RationalMatrix {
    /// Adds the product of a [`RationalMatrix`] and a [`Rational`] to a [`RationalMatrix`] in
    /// place, taking the matrix and the scalar on the right-hand side by reference.
    ///
    /// See the documentation for the [`AddMulAssign`] implementation that takes both by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both matrices and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `b` have different shapes.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul_assign).
    fn add_mul_assign(&mut self, b: &'a Self, c: &'b Rational) {
        assert_same_shape(self, b);
        match *c {
            Rational::ZERO => {}
            Rational::ONE => *self += b,
            Rational::NEGATIVE_ONE => *self -= b,
            _ => {
                for (x, y) in self.elements.iter_mut().zip(&b.elements) {
                    x.add_mul_assign(y, c);
                }
            }
        }
    }
}
