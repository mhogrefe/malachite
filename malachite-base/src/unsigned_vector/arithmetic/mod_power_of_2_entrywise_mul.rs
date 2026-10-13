// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    ModPowerOf2EntrywiseMul, ModPowerOf2EntrywiseMulAssign, ModPowerOf2IsReduced,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec::Vec;

fn assert_valid<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, w: &UnsignedVector<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot multiply vectors of different dimensions entrywise"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
    assert!(
        w.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {w} has an element >= 2^{pow}"
    );
}

// Multiplies each element of `xs` by the element of `ys` at the same index, modulo `2^pow`.
fn mod_power_of_2_entrywise_mul_assign_slices<T: PrimitiveUnsigned>(
    xs: &mut [T],
    ys: &[T],
    pow: u64,
) {
    for (x, &y) in xs.iter_mut().zip(ys) {
        x.mod_power_of_2_mul_assign(y, pow);
    }
}

// The products of the elements of `xs` and `ys` at the same indices, modulo `2^pow`.
fn mod_power_of_2_entrywise_mul_slices<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    pow: u64,
) -> Vec<T> {
    xs.iter()
        .zip(ys)
        .map(|(&x, &y)| x.mod_power_of_2_mul(y, pow))
        .collect()
}

fn mod_power_of_2_entrywise_mul_assign_helper<T: PrimitiveUnsigned>(
    v: &mut UnsignedVector<T>,
    w: &UnsignedVector<T>,
    pow: u64,
) {
    assert_valid(v, w, pow);
    mod_power_of_2_entrywise_mul_assign_slices(&mut v.elements, &w.elements, pow);
}

impl<T: PrimitiveUnsigned> ModPowerOf2EntrywiseMul<Self> for UnsignedVector<T> {
    type Output = Self;

    /// Takes the entrywise product of two [`UnsignedVector`]s modulo $2^k$, taking the first by
    /// value and the second by value. The elements of both must already be reduced modulo $2^k$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $2^k$, so the result has the same dimension as both vectors.
    ///
    /// $$
    /// f(v, w, k) = (v_0w_0 \bmod 2^k, v_1w_1 \bmod 2^k, \ldots, v_{n-1}w_{n-1} \bmod 2^k).
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
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `other` have different dimensions,
    /// or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 7, 6)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_entrywise_mul(w, 3).to_string(),
    ///     "(4, 7, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_mul(mut self, other: Self, pow: u64) -> Self {
        mod_power_of_2_entrywise_mul_assign_helper(&mut self, &other, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2EntrywiseMul<&Self> for UnsignedVector<T> {
    type Output = Self;

    /// Takes the entrywise product of two [`UnsignedVector`]s modulo $2^k$, taking the first by
    /// value and the second by reference. The elements of both must already be reduced modulo
    /// $2^k$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $2^k$, so the result has the same dimension as both vectors.
    ///
    /// $$
    /// f(v, w, k) = (v_0w_0 \bmod 2^k, v_1w_1 \bmod 2^k, \ldots, v_{n-1}w_{n-1} \bmod 2^k).
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
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `other` have different dimensions,
    /// or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 7, 6)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_entrywise_mul(&w, 3).to_string(),
    ///     "(4, 7, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_mul(mut self, other: &Self, pow: u64) -> Self {
        mod_power_of_2_entrywise_mul_assign_helper(&mut self, other, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2EntrywiseMul<UnsignedVector<T>> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Takes the entrywise product of two [`UnsignedVector`]s modulo $2^k$, taking the first by
    /// reference and the second by value. The elements of both must already be reduced modulo
    /// $2^k$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $2^k$, so the result has the same dimension as both vectors.
    ///
    /// $$
    /// f(v, w, k) = (v_0w_0 \bmod 2^k, v_1w_1 \bmod 2^k, \ldots, v_{n-1}w_{n-1} \bmod 2^k).
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
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `other` have different dimensions,
    /// or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 7, 6)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_entrywise_mul(w, 3).to_string(),
    ///     "(4, 7, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_mul(
        self,
        mut other: UnsignedVector<T>,
        pow: u64,
    ) -> UnsignedVector<T> {
        // The product is commutative, so the owned vector can hold the result.
        mod_power_of_2_entrywise_mul_assign_helper(&mut other, self, pow);
        other
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2EntrywiseMul<&UnsignedVector<T>> for &UnsignedVector<T> {
    type Output = UnsignedVector<T>;

    /// Takes the entrywise product of two [`UnsignedVector`]s modulo $2^k$, taking the first by
    /// reference and the second by reference. The elements of both must already be reduced modulo
    /// $2^k$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $2^k$, so the result has the same dimension as both vectors.
    ///
    /// $$
    /// f(v, w, k) = (v_0w_0 \bmod 2^k, v_1w_1 \bmod 2^k, \ldots, v_{n-1}w_{n-1} \bmod 2^k).
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
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `other` have different dimensions,
    /// or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseMul;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 7, 6)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_entrywise_mul(&w, 3).to_string(),
    ///     "(4, 7, 2)"
    /// );
    /// ```
    fn mod_power_of_2_entrywise_mul(
        self,
        other: &UnsignedVector<T>,
        pow: u64,
    ) -> UnsignedVector<T> {
        assert_valid(self, other, pow);
        UnsignedVector {
            elements: mod_power_of_2_entrywise_mul_slices(&self.elements, &other.elements, pow),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2EntrywiseMulAssign<Self> for UnsignedVector<T> {
    /// Replaces an [`UnsignedVector`] by its entrywise product with another [`UnsignedVector`]
    /// modulo $2^k$, taking the other by value. The elements of both must already be reduced modulo
    /// $2^k$.
    ///
    /// Every element is multiplied by the corresponding element of `other` and reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(v, w, k) = (v_0w_0 \bmod 2^k, v_1w_1 \bmod 2^k, \ldots, v_{n-1}w_{n-1} \bmod 2^k).
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
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `other` have different dimensions,
    /// or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseMulAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 7, 6)").unwrap();
    /// v.mod_power_of_2_entrywise_mul_assign(w, 3);
    /// assert_eq!(v.to_string(), "(4, 7, 2)");
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_mul_assign(&mut self, other: Self, pow: u64) {
        mod_power_of_2_entrywise_mul_assign_helper(self, &other, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2EntrywiseMulAssign<&Self> for UnsignedVector<T> {
    /// Replaces an [`UnsignedVector`] by its entrywise product with another [`UnsignedVector`]
    /// modulo $2^k$, taking the other by reference. The elements of both must already be reduced
    /// modulo $2^k$.
    ///
    /// Every element is multiplied by the corresponding element of `other` and reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(v, w, k) = (v_0w_0 \bmod 2^k, v_1w_1 \bmod 2^k, \ldots, v_{n-1}w_{n-1} \bmod 2^k).
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
    /// Panics if `pow` is greater than `T::WIDTH`, if `self` and `other` have different dimensions,
    /// or if any element of either is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseMulAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let mut v = UnsignedVector::<u8>::from_str("(5, 1, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 7, 6)").unwrap();
    /// v.mod_power_of_2_entrywise_mul_assign(&w, 3);
    /// assert_eq!(v.to_string(), "(4, 7, 2)");
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_mul_assign(&mut self, other: &Self, pow: u64) {
        mod_power_of_2_entrywise_mul_assign_helper(self, other, pow);
    }
}
