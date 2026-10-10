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
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::traits::{One, Zero};

fn assert_same_dimension(v: &IntegerVector, w: &IntegerVector) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot add-multiply vectors of different dimensions"
    );
}

impl AddMul<Self, Integer> for IntegerVector {
    type Output = Self;

    /// Adds the product of an [`IntegerVector`] and an [`Integer`] to an [`IntegerVector`], taking
    /// all three by value.
    ///
    /// The operation is taken element by element, so the result has the same dimension as the
    /// vectors. Like FLINT's, this special-cases multiplication by 0, 1, and $-1$.
    ///
    /// $$
    /// f(v, w, c) = v + cw = (v_0 + cw_0, v_1 + cw_1, \ldots, v_{n-1} + cw_{n-1}).
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
    /// See [here](super::add_mul#add_mul).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn add_mul(mut self, w: Self, c: Integer) -> Self {
        self.add_mul_assign(w, &c);
        self
    }
}

impl<'b> AddMul<Self, &'b Integer> for IntegerVector {
    type Output = Self;

    /// Adds the product of an [`IntegerVector`] and an [`Integer`] to an [`IntegerVector`], taking
    /// the first vector by value, the second vector by value, and the scalar by reference.
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
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn add_mul(mut self, w: Self, c: &'b Integer) -> Self {
        self.add_mul_assign(w, c);
        self
    }
}

impl<'a> AddMul<&'a Self, Integer> for IntegerVector {
    type Output = Self;

    /// Adds the product of an [`IntegerVector`] and an [`Integer`] to an [`IntegerVector`], taking
    /// the first vector by value, the second vector by reference, and the scalar by value.
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
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn add_mul(mut self, w: &'a Self, c: Integer) -> Self {
        self.add_mul_assign(w, &c);
        self
    }
}

impl<'a, 'b> AddMul<&'a Self, &'b Integer> for IntegerVector {
    type Output = Self;

    /// Adds the product of an [`IntegerVector`] and an [`Integer`] to an [`IntegerVector`], taking
    /// the first vector by value, the second vector by reference, and the scalar by reference.
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
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn add_mul(mut self, w: &'a Self, c: &'b Integer) -> Self {
        self.add_mul_assign(w, c);
        self
    }
}

impl AddMul<&IntegerVector, &Integer> for &IntegerVector {
    type Output = IntegerVector;

    /// Adds the product of an [`IntegerVector`] and an [`Integer`] to an [`IntegerVector`], taking
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
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    fn add_mul(self, w: &IntegerVector, c: &Integer) -> IntegerVector {
        assert_same_dimension(self, w);
        match *c {
            integer_zero!() => self.clone(),
            integer_one!() => self + w,
            integer_negative_one!() => self - w,
            _ => IntegerVector {
                elements: self
                    .elements
                    .iter()
                    .zip(&w.elements)
                    .map(|(x, y)| x.add_mul(y, c))
                    .collect(),
            },
        }
    }
}

impl AddMulAssign<Self, Integer> for IntegerVector {
    /// Adds the product of an [`IntegerVector`] and an [`Integer`] to an [`IntegerVector`] in
    /// place, taking the vector and the scalar on the right-hand side by value.
    ///
    /// The operation is taken element by element. Like FLINT's, this special-cases multiplication
    /// by 0, 1, and $-1$.
    ///
    /// $$
    /// v \gets v + cw.
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
    /// See [here](super::add_mul#add_mul_assign).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn add_mul_assign(&mut self, w: Self, c: Integer) {
        self.add_mul_assign(w, &c);
    }
}

impl<'b> AddMulAssign<Self, &'b Integer> for IntegerVector {
    /// Adds the product of an [`IntegerVector`] and an [`Integer`] to an [`IntegerVector`] in
    /// place, taking the vector on the right-hand side by value and the scalar by reference.
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
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul_assign).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    fn add_mul_assign(&mut self, w: Self, c: &'b Integer) {
        assert_same_dimension(self, &w);
        match *c {
            integer_zero!() => {}
            integer_one!() => *self += w,
            integer_negative_one!() => *self -= w,
            _ => {
                for (x, y) in self.elements.iter_mut().zip(w.elements) {
                    x.add_mul_assign(y, c);
                }
            }
        }
    }
}

impl<'a> AddMulAssign<&'a Self, Integer> for IntegerVector {
    /// Adds the product of an [`IntegerVector`] and an [`Integer`] to an [`IntegerVector`] in
    /// place, taking the vector on the right-hand side by reference and the scalar by value.
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
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul_assign).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    #[inline]
    fn add_mul_assign(&mut self, w: &'a Self, c: Integer) {
        self.add_mul_assign(w, &c);
    }
}

impl<'a, 'b> AddMulAssign<&'a Self, &'b Integer> for IntegerVector {
    /// Adds the product of an [`IntegerVector`] and an [`Integer`] to an [`IntegerVector`] in
    /// place, taking the vector and the scalar on the right-hand side by reference.
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
    /// of `w` and of `c`, and $m$ is the total number of bits of the elements of `self`.
    ///
    /// # Panics
    /// Panics if `self` and `w` have different dimensions.
    ///
    /// # Examples
    /// See [here](super::add_mul#add_mul_assign).
    ///
    /// This is equivalent to `_fmpz_vec_scalar_addmul_fmpz` from `fmpz_vec/scalar.c`, FLINT 3.6.0.
    fn add_mul_assign(&mut self, w: &'a Self, c: &'b Integer) {
        assert_same_dimension(self, w);
        match *c {
            integer_zero!() => {}
            integer_one!() => *self += w,
            integer_negative_one!() => *self -= w,
            _ => {
                for (x, y) in self.elements.iter_mut().zip(&w.elements) {
                    x.add_mul_assign(y, c);
                }
            }
        }
    }
}
