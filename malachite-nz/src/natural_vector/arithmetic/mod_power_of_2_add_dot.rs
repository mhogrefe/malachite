// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use crate::natural_vector::arithmetic::mod_power_of_2_dot::mod_power_of_2_dot_general;
use malachite_base::num::arithmetic::traits::{ModPowerOf2AddDot, ModPowerOf2AddDotAssign};

impl ModPowerOf2AddDot<NaturalVector, NaturalVector> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $2^k$, taking the
    /// number by value, the first vector by value, and the second by value. The number and the
    /// elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, k) = x + \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if `x` or any element is greater than or
    /// equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(5u32).mod_power_of_2_add_dot(v.clone(), w.clone(), 4),
    ///     8u32
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot(self, v: NaturalVector, w: NaturalVector, pow: u64) -> Self {
        mod_power_of_2_dot_general(&self, false, &v, &w, false, pow)
    }
}

impl ModPowerOf2AddDot<NaturalVector, &NaturalVector> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $2^k$, taking the
    /// number by value, the first vector by value, and the second by reference. The number and the
    /// elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, k) = x + \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if `x` or any element is greater than or
    /// equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(5u32).mod_power_of_2_add_dot(v.clone(), &w, 4),
    ///     8u32
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot(self, v: NaturalVector, w: &NaturalVector, pow: u64) -> Self {
        mod_power_of_2_dot_general(&self, false, &v, w, false, pow)
    }
}

impl ModPowerOf2AddDot<&NaturalVector, NaturalVector> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $2^k$, taking the
    /// number by value, the first vector by reference, and the second by value. The number and the
    /// elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, k) = x + \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if `x` or any element is greater than or
    /// equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(5u32).mod_power_of_2_add_dot(&v, w.clone(), 4),
    ///     8u32
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot(self, v: &NaturalVector, w: NaturalVector, pow: u64) -> Self {
        mod_power_of_2_dot_general(&self, false, v, &w, false, pow)
    }
}

impl ModPowerOf2AddDot<&NaturalVector, &NaturalVector> for Natural {
    type Output = Self;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $2^k$, taking the
    /// number by value, the first vector by reference, and the second by reference. The number and
    /// the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, k) = x + \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if `x` or any element is greater than or
    /// equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(Natural::from(5u32).mod_power_of_2_add_dot(&v, &w, 4), 8u32);
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot(self, v: &NaturalVector, w: &NaturalVector, pow: u64) -> Self {
        mod_power_of_2_dot_general(&self, false, v, w, false, pow)
    }
}

impl ModPowerOf2AddDot<&NaturalVector, &NaturalVector> for &Natural {
    type Output = Natural;

    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] modulo $2^k$, taking the
    /// number by reference, the first vector by reference, and the second by reference. The number
    /// and the elements must already be reduced.
    ///
    /// $$
    /// f(x, v, w, k) = x + \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if `x` or any element is greater than or
    /// equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDot;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     (&Natural::from(5u32)).mod_power_of_2_add_dot(&v, &w, 4),
    ///     8u32
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot(self, v: &NaturalVector, w: &NaturalVector, pow: u64) -> Natural {
        mod_power_of_2_dot_general(self, false, v, w, false, pow)
    }
}

impl ModPowerOf2AddDotAssign<NaturalVector, NaturalVector> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place modulo $2^k$,
    /// taking the first vector by value and the second by value. The number and the elements must
    /// already be reduced.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if `x` or any element is greater than or
    /// equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(5u32);
    /// x.mod_power_of_2_add_dot_assign(v.clone(), w.clone(), 4);
    /// assert_eq!(x, 8u32);
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot_assign(&mut self, v: NaturalVector, w: NaturalVector, pow: u64) {
        *self = mod_power_of_2_dot_general(self, false, &v, &w, false, pow);
    }
}

impl ModPowerOf2AddDotAssign<NaturalVector, &NaturalVector> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place modulo $2^k$,
    /// taking the first vector by value and the second by reference. The number and the elements
    /// must already be reduced.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if `x` or any element is greater than or
    /// equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(5u32);
    /// x.mod_power_of_2_add_dot_assign(v.clone(), &w, 4);
    /// assert_eq!(x, 8u32);
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot_assign(&mut self, v: NaturalVector, w: &NaturalVector, pow: u64) {
        *self = mod_power_of_2_dot_general(self, false, &v, w, false, pow);
    }
}

impl ModPowerOf2AddDotAssign<&NaturalVector, NaturalVector> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place modulo $2^k$,
    /// taking the first vector by reference and the second by value. The number and the elements
    /// must already be reduced.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if `x` or any element is greater than or
    /// equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(5u32);
    /// x.mod_power_of_2_add_dot_assign(&v, w.clone(), 4);
    /// assert_eq!(x, 8u32);
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot_assign(&mut self, v: &NaturalVector, w: NaturalVector, pow: u64) {
        *self = mod_power_of_2_dot_general(self, false, v, &w, false, pow);
    }
}

impl ModPowerOf2AddDotAssign<&NaturalVector, &NaturalVector> for Natural {
    /// Adds the dot product of two [`NaturalVector`]s to a [`Natural`] in place modulo $2^k$,
    /// taking the first vector by reference and the second by reference. The number and the
    /// elements must already be reduced.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_i \bmod 2^k,
    /// $$
    ///
    /// where $n$ is the dimension of $v$ and $w$.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(nk \log k \log\log k)$
    ///
    /// $M(k) = O(k \log k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the vectors, and $k$ is
    /// `pow`.
    ///
    /// # Panics
    /// Panics if the vectors have different dimensions, or if `x` or any element is greater than or
    /// equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDotAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(5u32);
    /// x.mod_power_of_2_add_dot_assign(&v, &w, 4);
    /// assert_eq!(x, 8u32);
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot_assign(&mut self, v: &NaturalVector, w: &NaturalVector, pow: u64) {
        *self = mod_power_of_2_dot_general(self, false, v, w, false, pow);
    }
}
