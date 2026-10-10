// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{SubMul, SubMulAssign};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};

fn assert_same_dimension(v: &RationalVector, w: &RationalVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot subtract-multiply vectors of different dimensions"
    );
}

impl SubMul<Self, Rational> for RationalVector {
    type Output = Self;

    /// Subtracts the product of a [`RationalVector`] and a [`Rational`] from a [`RationalVector`],
    /// taking all three by value.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors. This special-cases multiplication by 0, 1, and $-1$.
    ///
    /// $$
    /// f(v, w, c) = v - cw = (v_0 - cw_0, v_1 - cw_1, \ldots, v_{n-1} - cw_{n-1}).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul).
    #[inline]
    fn sub_mul(mut self, w: Self, c: Rational) -> Self {
        self.sub_mul_assign(w, &c);
        self
    }
}

impl<'b> SubMul<Self, &'b Rational> for RationalVector {
    type Output = Self;

    /// Subtracts the product of a [`RationalVector`] and a [`Rational`] from a [`RationalVector`],
    /// taking the first vector by value, the second vector by value, and the scalar by reference.
    ///
    /// See the documentation for the [`SubMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul).
    #[inline]
    fn sub_mul(mut self, w: Self, c: &'b Rational) -> Self {
        self.sub_mul_assign(w, c);
        self
    }
}

impl<'a> SubMul<&'a Self, Rational> for RationalVector {
    type Output = Self;

    /// Subtracts the product of a [`RationalVector`] and a [`Rational`] from a [`RationalVector`],
    /// taking the first vector by value, the second vector by reference, and the scalar by value.
    ///
    /// See the documentation for the [`SubMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul).
    #[inline]
    fn sub_mul(mut self, w: &'a Self, c: Rational) -> Self {
        self.sub_mul_assign(w, &c);
        self
    }
}

impl<'a, 'b> SubMul<&'a Self, &'b Rational> for RationalVector {
    type Output = Self;

    /// Subtracts the product of a [`RationalVector`] and a [`Rational`] from a [`RationalVector`],
    /// taking the first vector by value, the second vector by reference, and the scalar by
    /// reference.
    ///
    /// See the documentation for the [`SubMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul).
    #[inline]
    fn sub_mul(mut self, w: &'a Self, c: &'b Rational) -> Self {
        self.sub_mul_assign(w, c);
        self
    }
}

impl SubMul<&RationalVector, &Rational> for &RationalVector {
    type Output = RationalVector;

    /// Subtracts the product of a [`RationalVector`] and a [`Rational`] from a [`RationalVector`],
    /// taking all three by reference.
    ///
    /// See the documentation for the [`SubMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul).
    fn sub_mul(self, w: &RationalVector, c: &Rational) -> RationalVector {
        assert_same_dimension(self, w);
        match *c {
            Rational::ZERO => self.clone(),
            Rational::ONE => self - w,
            Rational::NEGATIVE_ONE => self + w,
            _ => RationalVector {
                elements: self
                    .elements
                    .iter()
                    .zip(&w.elements)
                    .map(|(x, y)| x.sub_mul(y, c))
                    .collect(),
            },
        }
    }
}

impl SubMulAssign<Self, Rational> for RationalVector {
    /// Subtracts the product of a [`RationalVector`] and a [`Rational`] from a [`RationalVector`]
    /// in place, taking the vector and the scalar on the right-hand side by value.
    ///
    /// The operation is taken element by element. This special-cases multiplication by 0, 1, and
    /// $-1$.
    ///
    /// $$
    /// v \gets v - cw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul_assign).
    #[inline]
    fn sub_mul_assign(&mut self, w: Self, c: Rational) {
        self.sub_mul_assign(w, &c);
    }
}

impl<'b> SubMulAssign<Self, &'b Rational> for RationalVector {
    /// Subtracts the product of a [`RationalVector`] and a [`Rational`] from a [`RationalVector`]
    /// in place, taking the vector on the right-hand side by value and the scalar by reference.
    ///
    /// See the documentation for the [`SubMulAssign`] implementation that takes both by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul_assign).
    fn sub_mul_assign(&mut self, w: Self, c: &'b Rational) {
        assert_same_dimension(self, &w);
        match *c {
            Rational::ZERO => {}
            Rational::ONE => *self -= w,
            Rational::NEGATIVE_ONE => *self += w,
            _ => {
                for (x, y) in self.elements.iter_mut().zip(w.elements) {
                    x.sub_mul_assign(y, c);
                }
            }
        }
    }
}

impl<'a> SubMulAssign<&'a Self, Rational> for RationalVector {
    /// Subtracts the product of a [`RationalVector`] and a [`Rational`] from a [`RationalVector`]
    /// in place, taking the vector on the right-hand side by reference and the scalar by value.
    ///
    /// See the documentation for the [`SubMulAssign`] implementation that takes both by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul_assign).
    #[inline]
    fn sub_mul_assign(&mut self, w: &'a Self, c: Rational) {
        self.sub_mul_assign(w, &c);
    }
}

impl<'a, 'b> SubMulAssign<&'a Self, &'b Rational> for RationalVector {
    /// Subtracts the product of a [`RationalVector`] and a [`Rational`] from a [`RationalVector`]
    /// in place, taking the vector and the scalar on the right-hand side by reference.
    ///
    /// See the documentation for the [`SubMulAssign`] implementation that takes both by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements of both vectors and of `c`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul_assign).
    fn sub_mul_assign(&mut self, w: &'a Self, c: &'b Rational) {
        assert_same_dimension(self, w);
        match *c {
            Rational::ZERO => {}
            Rational::ONE => *self -= w,
            Rational::NEGATIVE_ONE => *self += w,
            _ => {
                for (x, y) in self.elements.iter_mut().zip(&w.elements) {
                    x.sub_mul_assign(y, c);
                }
            }
        }
    }
}
