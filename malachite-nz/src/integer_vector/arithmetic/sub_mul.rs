// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{SubMul, SubMulAssign};
use malachite_base::num::basic::traits::{One, Zero};

fn assert_same_dimension(v: &IntegerVector, w: &IntegerVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot subtract-multiply vectors of different dimensions"
    );
}

impl SubMul<Self, Integer> for IntegerVector {
    type Output = Self;

    /// Subtracts the product of an [`IntegerVector`] and an [`Integer`] from an [`IntegerVector`],
    /// taking all three by value.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors. Like FLINT's, this special-cases multiplication by 0, 1, and $-1$.
    ///
    /// $$
    /// f(v, w, c) = v - cw = (v_0 - cw_0, v_1 - cw_1, \ldots, v_{n-1} - cw_{n-1}).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn sub_mul(mut self, w: Self, c: Integer) -> Self {
        self.sub_mul_assign(w, &c);
        self
    }
}

impl<'b> SubMul<Self, &'b Integer> for IntegerVector {
    type Output = Self;

    /// Subtracts the product of an [`IntegerVector`] and an [`Integer`] from an [`IntegerVector`],
    /// taking the first vector by value, the second vector by value, and the scalar by reference.
    ///
    /// See the documentation for the [`SubMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn sub_mul(mut self, w: Self, c: &'b Integer) -> Self {
        self.sub_mul_assign(w, c);
        self
    }
}

impl<'a> SubMul<&'a Self, Integer> for IntegerVector {
    type Output = Self;

    /// Subtracts the product of an [`IntegerVector`] and an [`Integer`] from an [`IntegerVector`],
    /// taking the first vector by value, the second vector by reference, and the scalar by value.
    ///
    /// See the documentation for the [`SubMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn sub_mul(mut self, w: &'a Self, c: Integer) -> Self {
        self.sub_mul_assign(w, &c);
        self
    }
}

impl<'a, 'b> SubMul<&'a Self, &'b Integer> for IntegerVector {
    type Output = Self;

    /// Subtracts the product of an [`IntegerVector`] and an [`Integer`] from an [`IntegerVector`],
    /// taking the first vector by value, the second vector by reference, and the scalar by
    /// reference.
    ///
    /// See the documentation for the [`SubMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn sub_mul(mut self, w: &'a Self, c: &'b Integer) -> Self {
        self.sub_mul_assign(w, c);
        self
    }
}

impl SubMul<&IntegerVector, &Integer> for &IntegerVector {
    type Output = IntegerVector;

    /// Subtracts the product of an [`IntegerVector`] and an [`Integer`] from an [`IntegerVector`],
    /// taking all three by reference.
    ///
    /// See the documentation for the [`SubMul`] implementation that takes all three by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    fn sub_mul(self, w: &IntegerVector, c: &Integer) -> IntegerVector {
        assert_same_dimension(self, w);
        match *c {
            integer_zero!() => self.clone(),
            integer_one!() => self - w,
            integer_negative_one!() => self + w,
            _ => IntegerVector {
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

impl SubMulAssign<Self, Integer> for IntegerVector {
    /// Subtracts the product of an [`IntegerVector`] and an [`Integer`] from an [`IntegerVector`]
    /// in place, taking the vector and the scalar on the right-hand side by value.
    ///
    /// The operation is taken element by element. Like FLINT's, this special-cases multiplication
    /// by 0, 1, and $-1$.
    ///
    /// $$
    /// v \gets v - cw.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul_assign).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn sub_mul_assign(&mut self, w: Self, c: Integer) {
        self.sub_mul_assign(w, &c);
    }
}

impl<'b> SubMulAssign<Self, &'b Integer> for IntegerVector {
    /// Subtracts the product of an [`IntegerVector`] and an [`Integer`] from an [`IntegerVector`]
    /// in place, taking the vector on the right-hand side by value and the scalar by reference.
    ///
    /// See the documentation for the [`SubMulAssign`] implementation that takes both by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul_assign).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    fn sub_mul_assign(&mut self, w: Self, c: &'b Integer) {
        assert_same_dimension(self, &w);
        match *c {
            integer_zero!() => {}
            integer_one!() => *self -= w,
            integer_negative_one!() => *self += w,
            _ => {
                for (x, y) in self.elements.iter_mut().zip(w.elements) {
                    x.sub_mul_assign(y, c);
                }
            }
        }
    }
}

impl<'a> SubMulAssign<&'a Self, Integer> for IntegerVector {
    /// Subtracts the product of an [`IntegerVector`] and an [`Integer`] from an [`IntegerVector`]
    /// in place, taking the vector on the right-hand side by reference and the scalar by value.
    ///
    /// See the documentation for the [`SubMulAssign`] implementation that takes both by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul_assign).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn sub_mul_assign(&mut self, w: &'a Self, c: Integer) {
        self.sub_mul_assign(w, &c);
    }
}

impl<'a, 'b> SubMulAssign<&'a Self, &'b Integer> for IntegerVector {
    /// Subtracts the product of an [`IntegerVector`] and an [`Integer`] from an [`IntegerVector`]
    /// in place, taking the vector and the scalar on the right-hand side by reference.
    ///
    /// See the documentation for the [`SubMulAssign`] implementation that takes both by value for
    /// details.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the elements
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::sub_mul#sub_mul_assign).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_submul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    fn sub_mul_assign(&mut self, w: &'a Self, c: &'b Integer) {
        assert_same_dimension(self, w);
        match *c {
            integer_zero!() => {}
            integer_one!() => *self -= w,
            integer_negative_one!() => *self += w,
            _ => {
                for (x, y) in self.elements.iter_mut().zip(&w.elements) {
                    x.sub_mul_assign(y, c);
                }
            }
        }
    }
}
