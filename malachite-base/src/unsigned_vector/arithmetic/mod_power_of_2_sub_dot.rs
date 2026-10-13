// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2SubDot, ModPowerOf2SubDotAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use crate::unsigned_vector::arithmetic::mod_power_of_2_dot::mod_power_of_2_dot_general;

impl<T: PrimitiveUnsigned> ModPowerOf2SubDot<UnsignedVector<T>, UnsignedVector<T>> for T {
    type Output = Self;

    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` modulo $2^k$, the first
    /// vector by value, and the second by value. The number and the elements must already be
    /// reduced.
    ///
    /// $$
    /// f(x, v, w, k) = x - \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
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
    /// `x` or any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubDot;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(5u8.mod_power_of_2_sub_dot(v.clone(), w.clone(), 4), 2);
    /// ```
    #[inline]
    fn mod_power_of_2_sub_dot(self, v: UnsignedVector<T>, w: UnsignedVector<T>, pow: u64) -> Self {
        mod_power_of_2_dot_general(self, true, &v, &w, false, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SubDot<UnsignedVector<T>, &UnsignedVector<T>> for T {
    type Output = Self;

    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` modulo $2^k$, the first
    /// vector by value, and the second by reference. The number and the elements must already be
    /// reduced.
    ///
    /// $$
    /// f(x, v, w, k) = x - \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
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
    /// `x` or any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubDot;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(5u8.mod_power_of_2_sub_dot(v.clone(), &w, 4), 2);
    /// ```
    #[inline]
    fn mod_power_of_2_sub_dot(self, v: UnsignedVector<T>, w: &UnsignedVector<T>, pow: u64) -> Self {
        mod_power_of_2_dot_general(self, true, &v, w, false, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SubDot<&UnsignedVector<T>, UnsignedVector<T>> for T {
    type Output = Self;

    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` modulo $2^k$, the first
    /// vector by reference, and the second by value. The number and the elements must already be
    /// reduced.
    ///
    /// $$
    /// f(x, v, w, k) = x - \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
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
    /// `x` or any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubDot;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(5u8.mod_power_of_2_sub_dot(&v, w.clone(), 4), 2);
    /// ```
    #[inline]
    fn mod_power_of_2_sub_dot(self, v: &UnsignedVector<T>, w: UnsignedVector<T>, pow: u64) -> Self {
        mod_power_of_2_dot_general(self, true, v, &w, false, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SubDot<&UnsignedVector<T>, &UnsignedVector<T>> for T {
    type Output = Self;

    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` modulo $2^k$, the first
    /// vector by reference, and the second by reference. The number and the elements must already
    /// be reduced.
    ///
    /// $$
    /// f(x, v, w, k) = x - \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
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
    /// `x` or any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubDot;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(5u8.mod_power_of_2_sub_dot(&v, &w, 4), 2);
    /// ```
    #[inline]
    fn mod_power_of_2_sub_dot(
        self,
        v: &UnsignedVector<T>,
        w: &UnsignedVector<T>,
        pow: u64,
    ) -> Self {
        mod_power_of_2_dot_general(self, true, v, w, false, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SubDotAssign<UnsignedVector<T>, UnsignedVector<T>> for T {
    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` in place modulo $2^k$,
    /// taking the first vector by value and the second by value. The number and the elements must
    /// already be reduced.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
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
    /// `x` or any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubDotAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// let mut x = 5u8;
    /// x.mod_power_of_2_sub_dot_assign(v.clone(), w.clone(), 4);
    /// assert_eq!(x, 2);
    /// ```
    #[inline]
    fn mod_power_of_2_sub_dot_assign(
        &mut self,
        v: UnsignedVector<T>,
        w: UnsignedVector<T>,
        pow: u64,
    ) {
        *self = mod_power_of_2_dot_general(*self, true, &v, &w, false, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SubDotAssign<UnsignedVector<T>, &UnsignedVector<T>> for T {
    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` in place modulo $2^k$,
    /// taking the first vector by value and the second by reference. The number and the elements
    /// must already be reduced.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
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
    /// `x` or any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubDotAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// let mut x = 5u8;
    /// x.mod_power_of_2_sub_dot_assign(v.clone(), &w, 4);
    /// assert_eq!(x, 2);
    /// ```
    #[inline]
    fn mod_power_of_2_sub_dot_assign(
        &mut self,
        v: UnsignedVector<T>,
        w: &UnsignedVector<T>,
        pow: u64,
    ) {
        *self = mod_power_of_2_dot_general(*self, true, &v, w, false, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SubDotAssign<&UnsignedVector<T>, UnsignedVector<T>> for T {
    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` in place modulo $2^k$,
    /// taking the first vector by reference and the second by value. The number and the elements
    /// must already be reduced.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
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
    /// `x` or any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubDotAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// let mut x = 5u8;
    /// x.mod_power_of_2_sub_dot_assign(&v, w.clone(), 4);
    /// assert_eq!(x, 2);
    /// ```
    #[inline]
    fn mod_power_of_2_sub_dot_assign(
        &mut self,
        v: &UnsignedVector<T>,
        w: UnsignedVector<T>,
        pow: u64,
    ) {
        *self = mod_power_of_2_dot_general(*self, true, v, &w, false, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SubDotAssign<&UnsignedVector<T>, &UnsignedVector<T>> for T {
    /// Subtracts the dot product of two [`UnsignedVector`]s from a `T` in place modulo $2^k$,
    /// taking the first vector by reference and the second by reference. The number and the
    /// elements must already be reduced.
    ///
    /// $$
    /// x \gets x - \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
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
    /// `x` or any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubDotAssign;
    /// use malachite_base::unsigned_vector::UnsignedVector;
    ///
    /// let v = UnsignedVector::<u8>::from_str("(1, 2, 3)").unwrap();
    /// let w = UnsignedVector::<u8>::from_str("(4, 5, 7)").unwrap();
    /// let mut x = 5u8;
    /// x.mod_power_of_2_sub_dot_assign(&v, &w, 4);
    /// assert_eq!(x, 2);
    /// ```
    #[inline]
    fn mod_power_of_2_sub_dot_assign(
        &mut self,
        v: &UnsignedVector<T>,
        w: &UnsignedVector<T>,
        pow: u64,
    ) {
        *self = mod_power_of_2_dot_general(*self, true, v, w, false, pow);
    }
}
