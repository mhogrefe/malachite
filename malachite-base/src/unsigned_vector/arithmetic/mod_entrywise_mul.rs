// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModEntrywiseMul, ModEntrywiseMulAssign, ModIsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec::Vec;

fn assert_valid<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, w: &UnsignedVector<T>, m: T) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot multiply vectors of different dimensions entrywise"
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

// Multiplies each element of `xs` by the element of `ys` at the same index, modulo `m`. The data
// for multiplying modulo `m` is computed once and shared by all the elements.
fn mod_entrywise_mul_assign_slices<T: PrimitiveUnsigned>(xs: &mut [T], ys: &[T], m: T) {
    let data = T::precompute_mod_mul_data(&m);
    for (x, &y) in xs.iter_mut().zip(ys) {
        x.mod_mul_precomputed_assign(y, m, &data);
    }
}

// The products of the elements of `xs` and `ys` at the same indices, modulo `m`.
fn mod_entrywise_mul_slices<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], m: T) -> Vec<T> {
    let data = T::precompute_mod_mul_data(&m);
    xs.iter()
        .zip(ys)
        .map(|(&x, &y)| x.mod_mul_precomputed(y, m, &data))
        .collect()
}

fn mod_entrywise_mul_assign_helper<T: PrimitiveUnsigned>(
    v: &mut UnsignedVector<T>,
    w: &UnsignedVector<T>,
    m: T,
) {
    assert_valid(v, w, m);
    mod_entrywise_mul_assign_slices(&mut v.elements, &w.elements, m);
}

impl<T: PrimitiveUnsigned> ModEntrywiseMul<Self, T> for UnsignedVector<T> {
    type Output = Self;

    /// Takes the entrywise product of two [`UnsignedVector`]s modulo $m$, taking the first by value
    /// and the second by value. The elements of both must already be reduced modulo $m$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $m$, so the result has the same dimension as both vectors.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 6, 5)").unwrap();
    /// assert_eq!(v.mod_entrywise_mul(w, 7).to_string(), "(6, 6, 1)");
    /// ```
    #[inline]
    fn mod_entrywise_mul(mut self, other: Self, m: T) -> Self {
        mod_entrywise_mul_assign_helper(&mut self, &other, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModEntrywiseMul<&Self, T> for UnsignedVector<T> {
    type Output = Self;

    /// Takes the entrywise product of two [`UnsignedVector`]s modulo $m$, taking the first by value
    /// and the second by reference. The elements of both must already be reduced modulo $m$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $m$, so the result has the same dimension as both vectors.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 6, 5)").unwrap();
    /// assert_eq!(v.mod_entrywise_mul(&w, 7).to_string(), "(6, 6, 1)");
    /// ```
    #[inline]
    fn mod_entrywise_mul(mut self, other: &Self, m: T) -> Self {
        mod_entrywise_mul_assign_helper(&mut self, other, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModEntrywiseMul<UnsignedVector<T>, T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Takes the entrywise product of two [`UnsignedVector`]s modulo $m$, taking the first by
    /// reference and the second by value. The elements of both must already be reduced modulo $m$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $m$, so the result has the same dimension as both vectors.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 6, 5)").unwrap();
    /// assert_eq!((&v).mod_entrywise_mul(w, 7).to_string(), "(6, 6, 1)");
    /// ```
    #[inline]
    fn mod_entrywise_mul(self, mut other: UnsignedVector<T>, m: T) -> UnsignedVector<T> {
        // The product is commutative, so the owned vector can hold the result.
        mod_entrywise_mul_assign_helper(&mut other, self, m);
        other
    }
}

impl<T: PrimitiveUnsigned> ModEntrywiseMul<&UnsignedVector<T>, T> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Takes the entrywise product of two [`UnsignedVector`]s modulo $m$, taking the first by
    /// reference and the second by reference. The elements of both must already be reduced modulo
    /// $m$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $m$, so the result has the same dimension as both vectors.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 6, 5)").unwrap();
    /// assert_eq!((&v).mod_entrywise_mul(&w, 7).to_string(), "(6, 6, 1)");
    /// ```
    fn mod_entrywise_mul(self, other: &UnsignedVector<T>, m: T) -> UnsignedVector<T> {
        assert_valid(self, other, m);
        UnsignedVector {
            elements: mod_entrywise_mul_slices(&self.elements, &other.elements, m),
        }
    }
}

impl<T: PrimitiveUnsigned> ModEntrywiseMulAssign<Self, T> for UnsignedVector<T> {
    /// Replaces an [`UnsignedVector`] by its entrywise product with another [`UnsignedVector`]
    /// modulo $m$, taking the other by value. The elements of both must already be reduced modulo
    /// $m$.
    ///
    /// Every element is multiplied by the corresponding element of `other` and reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMulAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 6, 5)").unwrap();
    /// v.mod_entrywise_mul_assign(w, 7);
    /// assert_eq!(v.to_string(), "(6, 6, 1)");
    /// ```
    #[inline]
    fn mod_entrywise_mul_assign(&mut self, other: Self, m: T) {
        mod_entrywise_mul_assign_helper(self, &other, m);
    }
}

impl<T: PrimitiveUnsigned> ModEntrywiseMulAssign<&Self, T> for UnsignedVector<T> {
    /// Replaces an [`UnsignedVector`] by its entrywise product with another [`UnsignedVector`]
    /// modulo $m$, taking the other by reference. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// Every element is multiplied by the corresponding element of `other` and reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMulAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 6, 5)").unwrap();
    /// v.mod_entrywise_mul_assign(&w, 7);
    /// assert_eq!(v.to_string(), "(6, 6, 1)");
    /// ```
    #[inline]
    fn mod_entrywise_mul_assign(&mut self, other: &Self, m: T) {
        mod_entrywise_mul_assign_helper(self, other, m);
    }
}
