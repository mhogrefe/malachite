// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2Dot, ModPowerOf2IsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;

// Checks the arguments of a dot product modulo $2^k$: `pow` is at most the width of `T`, the
// vectors have the same dimension, and their elements are reduced. `v_name` and `w_name` name the
// vectors in the panic messages.
fn assert_mod_power_of_2_dot_arguments<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    w: &UnsignedVector<T>,
    pow: u64,
    v_name: &str,
    w_name: &str,
) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most {}, but is {pow}",
        T::WIDTH
    );
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "the vectors must have the same dimension"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "{v_name} must be reduced mod 2^{pow}, but {v} has an element >= 2^{pow}"
    );
    assert!(
        w.mod_power_of_2_is_reduced(pow),
        "{w_name} must be reduced mod 2^{pow}, but {w} has an element >= 2^{pow}"
    );
}

// The dot product of `xs` and `ys`, with `ys` read backwards if `reverse`, modulo $2^k$: the
// wrapping sum of the wrapping products with all but the lowest $k$ bits cleared, since $2^k$
// divides $2^W$.
fn wrapping_dot<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], reverse: bool, pow: u64) -> T {
    let mut sum = T::ZERO;
    if reverse {
        for (&x, &y) in xs.iter().zip(ys.iter().rev()) {
            sum.wrapping_add_assign(x.wrapping_mul(y));
        }
    } else {
        for (&x, &y) in xs.iter().zip(ys) {
            sum.wrapping_add_assign(x.wrapping_mul(y));
        }
    }
    sum.mod_power_of_2(pow)
}

// The dot product of two vectors modulo $2^k$, after checking the arguments.
fn mod_power_of_2_dot<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    w: &UnsignedVector<T>,
    pow: u64,
) -> T {
    assert_mod_power_of_2_dot_arguments(v, w, pow, "self", "other");
    wrapping_dot(&v.elements, &w.elements, false, pow)
}

// The dot product of `v` and the reverse of `w` modulo $2^k$, after checking the arguments.
pub(crate) fn mod_power_of_2_dot_reversed_vectors<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    w: &UnsignedVector<T>,
    pow: u64,
) -> T {
    assert_mod_power_of_2_dot_arguments(v, w, pow, "self", "other");
    wrapping_dot(&v.elements, &w.elements, true, pow)
}

// `x` plus, or if `subtract` minus, the dot product of `v` and `w` modulo $2^k$, with `w` read
// backwards if `reverse`, after checking the arguments.
pub(crate) fn mod_power_of_2_dot_general<T: PrimitiveUnsigned>(
    x: T,
    subtract: bool,
    v: &UnsignedVector<T>,
    w: &UnsignedVector<T>,
    reverse: bool,
    pow: u64,
) -> T {
    assert_mod_power_of_2_dot_arguments(v, w, pow, "v", "w");
    assert!(
        x.mod_power_of_2_is_reduced(pow),
        "x must be reduced mod 2^{pow}, but {x} >= 2^{pow}"
    );
    let d = wrapping_dot(&v.elements, &w.elements, reverse, pow);
    if subtract {
        x.mod_power_of_2_sub(d, pow)
    } else {
        x.mod_power_of_2_add(d, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Dot<Self> for UnsignedVector<T> {
    type Output = T;

    /// Computes the dot product of two [`UnsignedVector`]s modulo $2^k$, taking the first by value
    /// and the second by value. The elements of both must already be reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, k) = \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
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
    /// Panics if `pow` is greater than `T::WIDTH`, if the vectors have different dimensions, or if
    /// any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Dot;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_power_of_2_dot(w.clone(), 4), 3);
    /// ```
    #[inline]
    fn mod_power_of_2_dot(self, other: Self, pow: u64) -> T {
        mod_power_of_2_dot(&self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Dot<&Self> for UnsignedVector<T> {
    type Output = T;

    /// Computes the dot product of two [`UnsignedVector`]s modulo $2^k$, taking the first by value
    /// and the second by reference. The elements of both must already be reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, k) = \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
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
    /// Panics if `pow` is greater than `T::WIDTH`, if the vectors have different dimensions, or if
    /// any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Dot;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_power_of_2_dot(&w, 4), 3);
    /// ```
    #[inline]
    fn mod_power_of_2_dot(self, other: &Self, pow: u64) -> T {
        mod_power_of_2_dot(&self, other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Dot<UnsignedVector<T>> for &UnsignedVector<T> {
    type Output = T;

    /// Computes the dot product of two [`UnsignedVector`]s modulo $2^k$, taking the first by
    /// reference and the second by value. The elements of both must already be reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, k) = \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
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
    /// Panics if `pow` is greater than `T::WIDTH`, if the vectors have different dimensions, or if
    /// any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Dot;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_dot(w.clone(), 4), 3);
    /// ```
    #[inline]
    fn mod_power_of_2_dot(self, other: UnsignedVector<T>, pow: u64) -> T {
        mod_power_of_2_dot(self, &other, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Dot<&UnsignedVector<T>> for &UnsignedVector<T> {
    type Output = T;

    /// Computes the dot product of two [`UnsignedVector`]s modulo $2^k$, taking the first by
    /// reference and the second by reference. The elements of both must already be reduced.
    ///
    /// The dot product of two 0-dimensional vectors is 0.
    ///
    /// $$
    /// f(v, w, k) = \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
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
    /// Panics if `pow` is greater than `T::WIDTH`, if the vectors have different dimensions, or if
    /// any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Dot;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_dot(&w, 4), 3);
    /// ```
    #[inline]
    fn mod_power_of_2_dot(self, other: &UnsignedVector<T>, pow: u64) -> T {
        mod_power_of_2_dot(self, other, pow)
    }
}
