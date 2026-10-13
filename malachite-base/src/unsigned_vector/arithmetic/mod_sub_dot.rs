// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModSubDot, ModSubDotAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use crate::unsigned_vector::arithmetic::mod_dot::mod_dot_general;

impl<T: PrimitiveUnsigned> ModSubDot<UnsignedVector<T>, UnsignedVector<T>, T> for T {
    type Output = Self;

    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` modulo $m$, the first vector
    /// by value, and the second by value. The number and the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, m) = x - \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubDot;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(3u8.mod_sub_dot(v.clone(), w.clone(), 11), 1);
    /// ```
    #[inline]
    fn mod_sub_dot(self, v: UnsignedVector<T>, w: UnsignedVector<T>, m: T) -> Self {
        mod_dot_general(self, true, &v, &w, false, m)
    }
}

impl<T: PrimitiveUnsigned> ModSubDot<UnsignedVector<T>, &UnsignedVector<T>, T> for T {
    type Output = Self;

    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` modulo $m$, the first vector
    /// by value, and the second by reference. The number and the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, m) = x - \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubDot;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(3u8.mod_sub_dot(v.clone(), &w, 11), 1);
    /// ```
    #[inline]
    fn mod_sub_dot(self, v: UnsignedVector<T>, w: &UnsignedVector<T>, m: T) -> Self {
        mod_dot_general(self, true, &v, w, false, m)
    }
}

impl<T: PrimitiveUnsigned> ModSubDot<&UnsignedVector<T>, UnsignedVector<T>, T> for T {
    type Output = Self;

    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` modulo $m$, the first vector
    /// by reference, and the second by value. The number and the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, m) = x - \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubDot;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(3u8.mod_sub_dot(&v, w.clone(), 11), 1);
    /// ```
    #[inline]
    fn mod_sub_dot(self, v: &UnsignedVector<T>, w: UnsignedVector<T>, m: T) -> Self {
        mod_dot_general(self, true, v, &w, false, m)
    }
}

impl<T: PrimitiveUnsigned> ModSubDot<&UnsignedVector<T>, &UnsignedVector<T>, T> for T {
    type Output = Self;

    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` modulo $m$, the first vector
    /// by reference, and the second by reference. The number and the elements must already be
    /// reduced.
    ///
    /// $$
    /// f(x, v, w, m) = x - \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubDot;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(3u8.mod_sub_dot(&v, &w, 11), 1);
    /// ```
    #[inline]
    fn mod_sub_dot(self, v: &UnsignedVector<T>, w: &UnsignedVector<T>, m: T) -> Self {
        mod_dot_general(self, true, v, w, false, m)
    }
}

impl<T: PrimitiveUnsigned> ModSubDotAssign<UnsignedVector<T>, UnsignedVector<T>, T> for T {
    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` in place modulo $m$, taking
    /// the first vector by value and the second by value. The number and the elements must already
    /// be reduced.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubDotAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// let mut x = 3u8;
    /// x.mod_sub_dot_assign(v.clone(), w.clone(), 11);
    /// assert_eq!(x, 1);
    /// ```
    #[inline]
    fn mod_sub_dot_assign(&mut self, v: UnsignedVector<T>, w: UnsignedVector<T>, m: T) {
        *self = mod_dot_general(*self, true, &v, &w, false, m);
    }
}

impl<T: PrimitiveUnsigned> ModSubDotAssign<UnsignedVector<T>, &UnsignedVector<T>, T> for T {
    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` in place modulo $m$, taking
    /// the first vector by value and the second by reference. The number and the elements must
    /// already be reduced.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubDotAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// let mut x = 3u8;
    /// x.mod_sub_dot_assign(v.clone(), &w, 11);
    /// assert_eq!(x, 1);
    /// ```
    #[inline]
    fn mod_sub_dot_assign(&mut self, v: UnsignedVector<T>, w: &UnsignedVector<T>, m: T) {
        *self = mod_dot_general(*self, true, &v, w, false, m);
    }
}

impl<T: PrimitiveUnsigned> ModSubDotAssign<&UnsignedVector<T>, UnsignedVector<T>, T> for T {
    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` in place modulo $m$, taking
    /// the first vector by reference and the second by value. The number and the elements must
    /// already be reduced.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubDotAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// let mut x = 3u8;
    /// x.mod_sub_dot_assign(&v, w.clone(), 11);
    /// assert_eq!(x, 1);
    /// ```
    #[inline]
    fn mod_sub_dot_assign(&mut self, v: &UnsignedVector<T>, w: UnsignedVector<T>, m: T) {
        *self = mod_dot_general(*self, true, v, &w, false, m);
    }
}

impl<T: PrimitiveUnsigned> ModSubDotAssign<&UnsignedVector<T>, &UnsignedVector<T>, T> for T {
    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` in place modulo $m$, taking
    /// the first vector by reference and the second by reference. The number and the elements must
    /// already be reduced.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_i \bmod m,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the vectors.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the vectors have different dimensions, or if `x` or any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubDotAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// let mut x = 3u8;
    /// x.mod_sub_dot_assign(&v, &w, 11);
    /// assert_eq!(x, 1);
    /// ```
    #[inline]
    fn mod_sub_dot_assign(&mut self, v: &UnsignedVector<T>, w: &UnsignedVector<T>, m: T) {
        *self = mod_dot_general(*self, true, v, w, false, m);
    }
}
