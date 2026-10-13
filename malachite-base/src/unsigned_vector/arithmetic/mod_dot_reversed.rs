// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::ModDotReversed;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use crate::unsigned_vector::arithmetic::mod_dot::mod_dot_reversed_vectors;

impl<T: PrimitiveUnsigned> ModDotReversed<Self, T> for UnsignedVector<T> {
    type Output = T;

    /// Computes the dot product of an [`UnsignedVector`] and the reverse of another modulo $m$,
    /// taking the first by value and the second by value. The elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// Panics if `m` is zero, if the vectors have different dimensions, or if any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDotReversed;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_dot_reversed(w.clone(), 11), 7);
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_dot_rev` from `nmod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot_reversed(self, other: Self, m: T) -> T {
        mod_dot_reversed_vectors(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDotReversed<&Self, T> for UnsignedVector<T> {
    type Output = T;

    /// Computes the dot product of an [`UnsignedVector`] and the reverse of another modulo $m$,
    /// taking the first by value and the second by reference. The elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// Panics if `m` is zero, if the vectors have different dimensions, or if any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDotReversed;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_dot_reversed(&w, 11), 7);
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_dot_rev` from `nmod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot_reversed(self, other: &Self, m: T) -> T {
        mod_dot_reversed_vectors(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDotReversed<UnsignedVector<T>, T> for &UnsignedVector<T> {
    type Output = T;

    /// Computes the dot product of an [`UnsignedVector`] and the reverse of another modulo $m$,
    /// taking the first by reference and the second by value. The elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// Panics if `m` is zero, if the vectors have different dimensions, or if any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDotReversed;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_dot_reversed(w.clone(), 11), 7);
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_dot_rev` from `nmod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot_reversed(self, other: UnsignedVector<T>, m: T) -> T {
        mod_dot_reversed_vectors(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDotReversed<&UnsignedVector<T>, T> for &UnsignedVector<T> {
    type Output = T;

    /// Computes the dot product of an [`UnsignedVector`] and the reverse of another modulo $m$,
    /// taking the first by reference and the second by reference. The elements must already be
    /// reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, m) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod m,
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
    /// Panics if `m` is zero, if the vectors have different dimensions, or if any element is
    /// greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModDotReversed;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_dot_reversed(&w, 11), 7);
    /// ```
    ///
    /// This is equivalent to `_nmod_vec_dot_rev` from `nmod_vec/dot.c`, FLINT 3.6.0.
    #[inline]
    fn mod_dot_reversed(self, other: &UnsignedVector<T>, m: T) -> T {
        mod_dot_reversed_vectors(self, other, m)
    }
}
