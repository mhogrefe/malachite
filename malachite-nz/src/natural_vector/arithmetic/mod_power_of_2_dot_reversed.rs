// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use crate::natural_vector::arithmetic::mod_power_of_2_dot::mod_power_of_2_dot_reversed_vectors;
use malachite_base::num::arithmetic::traits::ModPowerOf2DotReversed;

impl ModPowerOf2DotReversed<Self> for NaturalVector {
    type Output = Natural;

    /// Computes the dot product of a [`NaturalVector`] and the reverse of another modulo $2^k$,
    /// taking the first by value and the second by value. The elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, k) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod 2^k,
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
    /// Panics if the vectors have different dimensions, or if any element is greater than or equal
    /// to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2DotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_power_of_2_dot_reversed(w.clone(), 4), 13u32);
    /// ```
    #[inline]
    fn mod_power_of_2_dot_reversed(self, other: Self, pow: u64) -> Natural {
        mod_power_of_2_dot_reversed_vectors(&self, &other, pow)
    }
}

impl ModPowerOf2DotReversed<&Self> for NaturalVector {
    type Output = Natural;

    /// Computes the dot product of a [`NaturalVector`] and the reverse of another modulo $2^k$,
    /// taking the first by value and the second by reference. The elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, k) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod 2^k,
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
    /// Panics if the vectors have different dimensions, or if any element is greater than or equal
    /// to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2DotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!(v.clone().mod_power_of_2_dot_reversed(&w, 4), 13u32);
    /// ```
    #[inline]
    fn mod_power_of_2_dot_reversed(self, other: &Self, pow: u64) -> Natural {
        mod_power_of_2_dot_reversed_vectors(&self, other, pow)
    }
}

impl ModPowerOf2DotReversed<NaturalVector> for &NaturalVector {
    type Output = Natural;

    /// Computes the dot product of a [`NaturalVector`] and the reverse of another modulo $2^k$,
    /// taking the first by reference and the second by value. The elements must already be reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, k) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod 2^k,
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
    /// Panics if the vectors have different dimensions, or if any element is greater than or equal
    /// to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2DotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_dot_reversed(w.clone(), 4), 13u32);
    /// ```
    #[inline]
    fn mod_power_of_2_dot_reversed(self, other: NaturalVector, pow: u64) -> Natural {
        mod_power_of_2_dot_reversed_vectors(self, &other, pow)
    }
}

impl ModPowerOf2DotReversed<&NaturalVector> for &NaturalVector {
    type Output = Natural;

    /// Computes the dot product of a [`NaturalVector`] and the reverse of another modulo $2^k$,
    /// taking the first by reference and the second by reference. The elements must already be
    /// reduced.
    ///
    /// The second vector is read backwards, so that the first element of one vector is paired with
    /// the last of the other, as in the coefficient of a polynomial product.
    ///
    /// $$
    /// f(v, w, k) = \sum_{i=0}^{n-1} v_iw_{n-1-i} \bmod 2^k,
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
    /// Panics if the vectors have different dimensions, or if any element is greater than or equal
    /// to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2DotReversed;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 5, 7)").unwrap();
    /// assert_eq!((&v).mod_power_of_2_dot_reversed(&w, 4), 13u32);
    /// ```
    #[inline]
    fn mod_power_of_2_dot_reversed(self, other: &NaturalVector, pow: u64) -> Natural {
        mod_power_of_2_dot_reversed_vectors(self, other, pow)
    }
}
