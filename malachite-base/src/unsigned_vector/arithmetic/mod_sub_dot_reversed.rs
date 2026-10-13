// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModSubDotReversed, ModSubDotReversedAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use crate::unsigned_vector::arithmetic::mod_dot::mod_dot_general;

impl<T: PrimitiveUnsigned> ModSubDotReversed<UnsignedVector<T>, UnsignedVector<T>, T> for T {
    type Output = Self;

    /// Subtracts the dot product of an [`UnsignedVector`] and the reverse of another from a `T`
    /// modulo $m$, the first vector by value, and the second by value. The number and the elements
    /// must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w, m) = x - \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModSubDotReversed;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(3u8.mod_sub_dot_reversed(v.clone(), w.clone(), 11), 7);
    /// ```
    #[inline]
    fn mod_sub_dot_reversed(self, v: UnsignedVector<T>, w: UnsignedVector<T>, m: T) -> Self {
        mod_dot_general(self, true, &v, &w, true, m)
    }
}

impl<T: PrimitiveUnsigned> ModSubDotReversed<UnsignedVector<T>, &UnsignedVector<T>, T> for T {
    type Output = Self;

    /// Subtracts the dot product of an [`UnsignedVector`] and the reverse of another from a `T`
    /// modulo $m$, the first vector by value, and the second by reference. The number and the
    /// elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w, m) = x - \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModSubDotReversed;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(3u8.mod_sub_dot_reversed(v.clone(), &w, 11), 7);
    /// ```
    #[inline]
    fn mod_sub_dot_reversed(self, v: UnsignedVector<T>, w: &UnsignedVector<T>, m: T) -> Self {
        mod_dot_general(self, true, &v, w, true, m)
    }
}

impl<T: PrimitiveUnsigned> ModSubDotReversed<&UnsignedVector<T>, UnsignedVector<T>, T> for T {
    type Output = Self;

    /// Subtracts the dot product of an [`UnsignedVector`] and the reverse of another from a `T`
    /// modulo $m$, the first vector by reference, and the second by value. The number and the
    /// elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w, m) = x - \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModSubDotReversed;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(3u8.mod_sub_dot_reversed(&v, w.clone(), 11), 7);
    /// ```
    #[inline]
    fn mod_sub_dot_reversed(self, v: &UnsignedVector<T>, w: UnsignedVector<T>, m: T) -> Self {
        mod_dot_general(self, true, v, &w, true, m)
    }
}

impl<T: PrimitiveUnsigned> ModSubDotReversed<&UnsignedVector<T>, &UnsignedVector<T>, T> for T {
    type Output = Self;

    /// Subtracts the dot product of an [`UnsignedVector`] and the reverse of another from a `T`
    /// modulo $m$, the first vector by reference, and the second by reference. The number and the
    /// elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w, m) = x - \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModSubDotReversed;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(3u8.mod_sub_dot_reversed(&v, &w, 11), 7);
    /// ```
    #[inline]
    fn mod_sub_dot_reversed(self, v: &UnsignedVector<T>, w: &UnsignedVector<T>, m: T) -> Self {
        mod_dot_general(self, true, v, w, true, m)
    }
}

impl<T: PrimitiveUnsigned> ModSubDotReversedAssign<UnsignedVector<T>, UnsignedVector<T>, T> for T {
    /// Subtracts the dot product of an [`UnsignedVector`] and the reverse of another from a `T` in
    /// place modulo $m$, taking the first vector by value and the second by value. The number and
    /// the elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModSubDotReversedAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// let mut x = 3u8;
    /// x.mod_sub_dot_reversed_assign(v.clone(), w.clone(), 11);
    /// assert_eq!(x, 7);
    /// ```
    #[inline]
    fn mod_sub_dot_reversed_assign(&mut self, v: UnsignedVector<T>, w: UnsignedVector<T>, m: T) {
        *self = mod_dot_general(*self, true, &v, &w, true, m);
    }
}

impl<T: PrimitiveUnsigned> ModSubDotReversedAssign<UnsignedVector<T>, &UnsignedVector<T>, T> for T {
    /// Subtracts the dot product of an [`UnsignedVector`] and the reverse of another from a `T` in
    /// place modulo $m$, taking the first vector by value and the second by reference. The number
    /// and the elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModSubDotReversedAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// let mut x = 3u8;
    /// x.mod_sub_dot_reversed_assign(v.clone(), &w, 11);
    /// assert_eq!(x, 7);
    /// ```
    #[inline]
    fn mod_sub_dot_reversed_assign(&mut self, v: UnsignedVector<T>, w: &UnsignedVector<T>, m: T) {
        *self = mod_dot_general(*self, true, &v, w, true, m);
    }
}

impl<T: PrimitiveUnsigned> ModSubDotReversedAssign<&UnsignedVector<T>, UnsignedVector<T>, T> for T {
    /// Subtracts the dot product of an [`UnsignedVector`] and the reverse of another from a `T` in
    /// place modulo $m$, taking the first vector by reference and the second by value. The number
    /// and the elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModSubDotReversedAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// let mut x = 3u8;
    /// x.mod_sub_dot_reversed_assign(&v, w.clone(), 11);
    /// assert_eq!(x, 7);
    /// ```
    #[inline]
    fn mod_sub_dot_reversed_assign(&mut self, v: &UnsignedVector<T>, w: UnsignedVector<T>, m: T) {
        *self = mod_dot_general(*self, true, v, &w, true, m);
    }
}

impl<T: PrimitiveUnsigned> ModSubDotReversedAssign<&UnsignedVector<T>, &UnsignedVector<T>, T>
    for T
{
    /// Subtracts the dot product of an [`UnsignedVector`] and the reverse of another from a `T` in
    /// place modulo $m$, taking the first vector by reference and the second by reference. The
    /// number and the elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// use malachite_base::num::arithmetic::traits::ModSubDotReversedAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// let mut x = 3u8;
    /// x.mod_sub_dot_reversed_assign(&v, &w, 11);
    /// assert_eq!(x, 7);
    /// ```
    #[inline]
    fn mod_sub_dot_reversed_assign(&mut self, v: &UnsignedVector<T>, w: &UnsignedVector<T>, m: T) {
        *self = mod_dot_general(*self, true, v, w, true, m);
    }
}
