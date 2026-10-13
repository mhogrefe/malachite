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
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2AddDotReversed, ModPowerOf2AddDotReversedAssign,
};

impl ModPowerOf2AddDotReversed<NaturalVector, NaturalVector> for Natural {
    type Output = Self;

    /// Adds the dot product of a [`NaturalVector`] and the reverse of another to a [`Natural`]
    /// modulo $2^k$, taking the number by value, the first vector by value, and the second by
    /// value. The number and the elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w, k) = x + \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod 2^k,
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(5u32).mod_power_of_2_add_dot_reversed(v.clone(), w.clone(), 4),
    ///     2u32
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot_reversed(self, v: NaturalVector, w: NaturalVector, pow: u64) -> Self {
        mod_power_of_2_dot_general(&self, false, &v, &w, true, pow)
    }
}

impl ModPowerOf2AddDotReversed<NaturalVector, &NaturalVector> for Natural {
    type Output = Self;

    /// Adds the dot product of a [`NaturalVector`] and the reverse of another to a [`Natural`]
    /// modulo $2^k$, taking the number by value, the first vector by value, and the second by
    /// reference. The number and the elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w, k) = x + \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod 2^k,
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(5u32).mod_power_of_2_add_dot_reversed(v.clone(), &w, 4),
    ///     2u32
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot_reversed(
        self,
        v: NaturalVector,
        w: &NaturalVector,
        pow: u64,
    ) -> Self {
        mod_power_of_2_dot_general(&self, false, &v, w, true, pow)
    }
}

impl ModPowerOf2AddDotReversed<&NaturalVector, NaturalVector> for Natural {
    type Output = Self;

    /// Adds the dot product of a [`NaturalVector`] and the reverse of another to a [`Natural`]
    /// modulo $2^k$, taking the number by value, the first vector by reference, and the second by
    /// value. The number and the elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w, k) = x + \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod 2^k,
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(5u32).mod_power_of_2_add_dot_reversed(&v, w.clone(), 4),
    ///     2u32
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot_reversed(
        self,
        v: &NaturalVector,
        w: NaturalVector,
        pow: u64,
    ) -> Self {
        mod_power_of_2_dot_general(&self, false, v, &w, true, pow)
    }
}

impl ModPowerOf2AddDotReversed<&NaturalVector, &NaturalVector> for Natural {
    type Output = Self;

    /// Adds the dot product of a [`NaturalVector`] and the reverse of another to a [`Natural`]
    /// modulo $2^k$, taking the number by value, the first vector by reference, and the second by
    /// reference. The number and the elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w, k) = x + \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod 2^k,
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     Natural::from(5u32).mod_power_of_2_add_dot_reversed(&v, &w, 4),
    ///     2u32
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot_reversed(
        self,
        v: &NaturalVector,
        w: &NaturalVector,
        pow: u64,
    ) -> Self {
        mod_power_of_2_dot_general(&self, false, v, w, true, pow)
    }
}

impl ModPowerOf2AddDotReversed<&NaturalVector, &NaturalVector> for &Natural {
    type Output = Natural;

    /// Adds the dot product of a [`NaturalVector`] and the reverse of another to a [`Natural`]
    /// modulo $2^k$, taking the number by reference, the first vector by reference, and the second
    /// by reference. The number and the elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(x, v, w, k) = x + \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod 2^k,
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(
    ///     (&Natural::from(5u32)).mod_power_of_2_add_dot_reversed(&v, &w, 4),
    ///     2u32
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot_reversed(
        self,
        v: &NaturalVector,
        w: &NaturalVector,
        pow: u64,
    ) -> Natural {
        mod_power_of_2_dot_general(self, false, v, w, true, pow)
    }
}

impl ModPowerOf2AddDotReversedAssign<NaturalVector, NaturalVector> for Natural {
    /// Adds the dot product of a [`NaturalVector`] and the reverse of another to a [`Natural`] in
    /// place modulo $2^k$, taking the first vector by value and the second by value. The number and
    /// the elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod 2^k,
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDotReversedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(5u32);
    /// x.mod_power_of_2_add_dot_reversed_assign(v.clone(), w.clone(), 4);
    /// assert_eq!(x, 2u32);
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot_reversed_assign(
        &mut self,
        v: NaturalVector,
        w: NaturalVector,
        pow: u64,
    ) {
        *self = mod_power_of_2_dot_general(self, false, &v, &w, true, pow);
    }
}

impl ModPowerOf2AddDotReversedAssign<NaturalVector, &NaturalVector> for Natural {
    /// Adds the dot product of a [`NaturalVector`] and the reverse of another to a [`Natural`] in
    /// place modulo $2^k$, taking the first vector by value and the second by reference. The number
    /// and the elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod 2^k,
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDotReversedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(5u32);
    /// x.mod_power_of_2_add_dot_reversed_assign(v.clone(), &w, 4);
    /// assert_eq!(x, 2u32);
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot_reversed_assign(
        &mut self,
        v: NaturalVector,
        w: &NaturalVector,
        pow: u64,
    ) {
        *self = mod_power_of_2_dot_general(self, false, &v, w, true, pow);
    }
}

impl ModPowerOf2AddDotReversedAssign<&NaturalVector, NaturalVector> for Natural {
    /// Adds the dot product of a [`NaturalVector`] and the reverse of another to a [`Natural`] in
    /// place modulo $2^k$, taking the first vector by reference and the second by value. The number
    /// and the elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod 2^k,
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDotReversedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(5u32);
    /// x.mod_power_of_2_add_dot_reversed_assign(&v, w.clone(), 4);
    /// assert_eq!(x, 2u32);
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot_reversed_assign(
        &mut self,
        v: &NaturalVector,
        w: NaturalVector,
        pow: u64,
    ) {
        *self = mod_power_of_2_dot_general(self, false, v, &w, true, pow);
    }
}

impl ModPowerOf2AddDotReversedAssign<&NaturalVector, &NaturalVector> for Natural {
    /// Adds the dot product of a [`NaturalVector`] and the reverse of another to a [`Natural`] in
    /// place modulo $2^k$, taking the first vector by reference and the second by reference. The
    /// number and the elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// x \gets x + \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod 2^k,
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
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddDotReversedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// let mut x = Natural::from(5u32);
    /// x.mod_power_of_2_add_dot_reversed_assign(&v, &w, 4);
    /// assert_eq!(x, 2u32);
    /// ```
    #[inline]
    fn mod_power_of_2_add_dot_reversed_assign(
        &mut self,
        v: &NaturalVector,
        w: &NaturalVector,
        pow: u64,
    ) {
        *self = mod_power_of_2_dot_general(self, false, v, w, true, pow);
    }
}
