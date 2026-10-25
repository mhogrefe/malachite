// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModSub, ModSubAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;

// Subtracts each element of `ys` from the element of `xs` at the same index, modulo `m`.
pub(crate) fn mod_sub_assign_slice<T: PrimitiveUnsigned>(xs: &mut [T], ys: &[T], m: T) {
    for (x, &y) in xs.iter_mut().zip(ys) {
        *x = x.mod_sub(y, m);
    }
}

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedMatrix<T>, w: &UnsignedMatrix<T>, m: T) {
    assert!(
        v.height == w.height && v.width == w.width,
        "cannot subtract matrices of different shapes"
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

fn mod_sub_assign<T: PrimitiveUnsigned>(v: &mut UnsignedMatrix<T>, w: &UnsignedMatrix<T>, m: T) {
    assert_reduced(v, w, m);
    mod_sub_assign_slice(&mut v.elements, &w.elements, m);
}

fn mod_sub_ref_val<T: PrimitiveUnsigned>(v: &UnsignedMatrix<T>, w: &mut UnsignedMatrix<T>, m: T) {
    assert_reduced(v, w, m);
    for (x, &y) in w.elements.iter_mut().zip(&v.elements) {
        *x = y.mod_sub(*x, m);
    }
}

impl<T: PrimitiveUnsigned> ModSub<Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Subtracts two [`UnsignedMatrix`]s modulo $m$, taking the first by value and the second by
    /// value. The elements of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(a.mod_sub(b, 8).to_string(), "[1, 2; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_sub` from `nmod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(mut self, other: Self, m: T) -> Self {
        mod_sub_assign(&mut self, &other, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModSub<&Self, T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Subtracts two [`UnsignedMatrix`]s modulo $m$, taking the first by value and the second by
    /// reference. The elements of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(a.mod_sub(&b, 8).to_string(), "[1, 2; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_sub` from `nmod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(mut self, other: &Self, m: T) -> Self {
        mod_sub_assign(&mut self, other, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModSub<UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Subtracts two [`UnsignedMatrix`]s modulo $m$, taking the first by reference and the second
    /// by value. The elements of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!((&a).mod_sub(b, 8).to_string(), "[1, 2; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_sub` from `nmod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, mut other: UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
        mod_sub_ref_val(self, &mut other, m);
        other
    }
}

impl<T: PrimitiveUnsigned> ModSub<&UnsignedMatrix<T>, T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Subtracts two [`UnsignedMatrix`]s modulo $m$, taking the first by reference and the second
    /// by reference. The elements of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!((&a).mod_sub(&b, 8).to_string(), "[1, 2; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_sub` from `nmod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, other: &UnsignedMatrix<T>, m: T) -> UnsignedMatrix<T> {
        let mut v = self.clone();
        mod_sub_assign(&mut v, other, m);
        v
    }
}

impl<T: PrimitiveUnsigned> ModSubAssign<Self, T> for UnsignedMatrix<T> {
    /// Subtracts an [`UnsignedMatrix`] from an [`UnsignedMatrix`] modulo $m$, in place, taking the
    /// [`UnsignedMatrix`] on the right-hand side by value. The elements of both must already be
    /// reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
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
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// a.mod_sub_assign(b, 8);
    /// assert_eq!(a.to_string(), "[1, 2; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_sub` from `nmod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub_assign(&mut self, other: Self, m: T) {
        mod_sub_assign(self, &other, m);
    }
}

impl<T: PrimitiveUnsigned> ModSubAssign<&Self, T> for UnsignedMatrix<T> {
    /// Subtracts an [`UnsignedMatrix`] from an [`UnsignedMatrix`] modulo $m$, in place, taking the
    /// [`UnsignedMatrix`] on the right-hand side by reference. The elements of both must already be
    /// reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
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
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = UnsignedMatrix::<u8>::from_str("[4, 7; 0, 6]").unwrap();
    /// a.mod_sub_assign(&b, 8);
    /// assert_eq!(a.to_string(), "[1, 2; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_sub` from `nmod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub_assign(&mut self, other: &Self, m: T) {
        mod_sub_assign(self, other, m);
    }
}
