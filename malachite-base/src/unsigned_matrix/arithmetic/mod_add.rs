// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModAdd, ModAddAssign, ModIsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;

// Adds each element of `ys` to the element of `xs` at the same index, modulo `m`.
pub(crate) fn mod_add_assign_slice<T: PrimitiveUnsigned>(xs: &mut [T], ys: &[T], m: T) {
    for (x, &y) in xs.iter_mut().zip(ys) {
        *x = x.mod_add(y, m);
    }
}

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedMatrix<T>, w: &UnsignedMatrix<T>, m: T) {
    assert!(
        v.height == w.height && v.width == w.width,
        "cannot add matrices of different shapes"
    );
    assert!(
        v.mod_is_reduced(&m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        w.mod_is_reduced(&m),
        "other must be reduced mod m, but {w} has an element >= {m}"
    );
}

fn mod_add_assign<T: PrimitiveUnsigned>(v: &mut UnsignedMatrix<T>, w: &UnsignedMatrix<T>, m: T) {
    assert_reduced(v, w, m);
    mod_add_assign_slice(&mut v.elements, &w.elements, m);
}

impl<T: PrimitiveUnsigned> ModAdd<Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Adds two [`UnsignedMatrix`]s modulo $m$, taking the first by value and the second by value.
    /// The elements of both must already be reduced modulo $m$.
    ///
    /// The sum is taken element by element, each reduced modulo $m$, so the result has the same
    /// shape as the summands.
    ///
    /// $$
    /// f(A, B, m) = A + B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(a.mod_add(b, 8).to_string(), "[1, 0; 3, 0]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(mut self, other: Self, m: T) -> Self {
        mod_add_assign(&mut self, &other, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModAdd<&Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Adds two [`UnsignedMatrix`]s modulo $m$, taking the first by value and the second by
    /// reference. The elements of both must already be reduced modulo $m$.
    ///
    /// The sum is taken element by element, each reduced modulo $m$, so the result has the same
    /// shape as the summands.
    ///
    /// $$
    /// f(A, B, m) = A + B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(a.mod_add(&b, 8).to_string(), "[1, 0; 3, 0]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(mut self, other: &Self, m: T) -> Self {
        mod_add_assign(&mut self, other, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModAdd<UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Adds two [`UnsignedMatrix`]s modulo $m$, taking the first by reference and the second by
    /// value. The elements of both must already be reduced modulo $m$.
    ///
    /// The sum is taken element by element, each reduced modulo $m$, so the result has the same
    /// shape as the summands.
    ///
    /// $$
    /// f(A, B, m) = A + B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!((&a).mod_add(b, 8).to_string(), "[1, 0; 3, 0]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, mut other: UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
        mod_add_assign(&mut other, self, m);
        other
    }
}

impl<T: PrimitiveUnsigned> ModAdd<&UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Adds two [`UnsignedMatrix`]s modulo $m$, taking the first by reference and the second by
    /// reference. The elements of both must already be reduced modulo $m$.
    ///
    /// The sum is taken element by element, each reduced modulo $m$, so the result has the same
    /// shape as the summands.
    ///
    /// $$
    /// f(A, B, m) = A + B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!((&a).mod_add(&b, 8).to_string(), "[1, 0; 3, 0]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: &UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
        let mut v = self.clone();
        mod_add_assign(&mut v, other, m);
        v
    }
}

impl<T: PrimitiveUnsigned> ModAddAssign<Self, T> for UnsignedMatrix<T> {
    /// Adds an [`UnsignedMatrix`] to an [`UnsignedMatrix`] modulo $m$, in place, taking the
    /// [`UnsignedMatrix`] on the right-hand side by value. The elements of both must already be
    /// reduced modulo $m$.
    ///
    /// The sum is taken element by element, each reduced modulo $m$, so the result has the same
    /// shape as the summands.
    ///
    /// $$
    /// f(A, B, m) = A + B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// a.mod_add_assign(b, 8);
    /// assert_eq!(a.to_string(), "[1, 0; 3, 0]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add_assign(&mut self, other: Self, m: T) {
        mod_add_assign(self, &other, m);
    }
}

impl<T: PrimitiveUnsigned> ModAddAssign<&Self, T> for UnsignedMatrix<T> {
    /// Adds an [`UnsignedMatrix`] to an [`UnsignedMatrix`] modulo $m$, in place, taking the
    /// [`UnsignedMatrix`] on the right-hand side by reference. The elements of both must already be
    /// reduced modulo $m$.
    ///
    /// The sum is taken element by element, each reduced modulo $m$, so the result has the same
    /// shape as the summands.
    ///
    /// $$
    /// f(A, B, m) = A + B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// a.mod_add_assign(&b, 8);
    /// assert_eq!(a.to_string(), "[1, 0; 3, 0]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_add` from `nmod_vec/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add_assign(&mut self, other: &Self, m: T) {
        mod_add_assign(self, other, m);
    }
}
