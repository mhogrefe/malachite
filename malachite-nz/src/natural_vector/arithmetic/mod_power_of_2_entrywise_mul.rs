// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2EntrywiseMul, ModPowerOf2EntrywiseMulAssign, ModPowerOf2IsReduced, ModPowerOf2Mul,
    ModPowerOf2MulAssign,
};

fn assert_valid(v: &NaturalVector, w: &NaturalVector, pow: u64) {
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

fn mod_power_of_2_entrywise_mul_assign_val(v: &mut NaturalVector, w: NaturalVector, pow: u64) {
    assert_valid(v, &w, pow);
    for (x, y) in v.elements.iter_mut().zip(w.elements) {
        x.mod_power_of_2_mul_assign(y, pow);
    }
}

fn mod_power_of_2_entrywise_mul_assign_ref(v: &mut NaturalVector, w: &NaturalVector, pow: u64) {
    assert_valid(v, w, pow);
    for (x, y) in v.elements.iter_mut().zip(&w.elements) {
        x.mod_power_of_2_mul_assign(y, pow);
    }
}

fn mod_power_of_2_entrywise_mul_ref_ref(
    v: &NaturalVector,
    w: &NaturalVector,
    pow: u64,
) -> NaturalVector {
    assert_valid(v, w, pow);
    NaturalVector {
        elements: v
            .elements
            .iter()
            .zip(&w.elements)
            .map(|(x, y)| x.mod_power_of_2_mul(y, pow))
            .collect(),
    }
}

impl ModPowerOf2EntrywiseMul<Self> for NaturalVector {
    type Output = Self;

    /// Takes the entrywise product of two [`NaturalVector`]s modulo $2^k$, taking the first by
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
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions, or if any element of either is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseMul;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 6)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_entrywise_mul(w, 3).to_string(),
    ///     "(4, 7, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_mul(mut self, other: Self, pow: u64) -> Self {
        mod_power_of_2_entrywise_mul_assign_val(&mut self, other, pow);
        self
    }
}

impl ModPowerOf2EntrywiseMul<&Self> for NaturalVector {
    type Output = Self;

    /// Takes the entrywise product of two [`NaturalVector`]s modulo $2^k$, taking the first by
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
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions, or if any element of either is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseMul;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 6)").unwrap();
    /// assert_eq!(
    ///     v.mod_power_of_2_entrywise_mul(&w, 3).to_string(),
    ///     "(4, 7, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_mul(mut self, other: &Self, pow: u64) -> Self {
        mod_power_of_2_entrywise_mul_assign_ref(&mut self, other, pow);
        self
    }
}

impl ModPowerOf2EntrywiseMul<NaturalVector> for &NaturalVector {
    type Output = NaturalVector;

    /// Takes the entrywise product of two [`NaturalVector`]s modulo $2^k$, taking the first by
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
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions, or if any element of either is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseMul;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 6)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_entrywise_mul(w, 3).to_string(),
    ///     "(4, 7, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_mul(self, mut other: NaturalVector, pow: u64) -> NaturalVector {
        // The product is commutative, so the owned vector can hold the result.
        mod_power_of_2_entrywise_mul_assign_ref(&mut other, self, pow);
        other
    }
}

impl ModPowerOf2EntrywiseMul<&NaturalVector> for &NaturalVector {
    type Output = NaturalVector;

    /// Takes the entrywise product of two [`NaturalVector`]s modulo $2^k$, taking the first by
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
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions, or if any element of either is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseMul;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 6)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_power_of_2_entrywise_mul(&w, 3).to_string(),
    ///     "(4, 7, 2)"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_mul(self, other: &NaturalVector, pow: u64) -> NaturalVector {
        mod_power_of_2_entrywise_mul_ref_ref(self, other, pow)
    }
}

impl ModPowerOf2EntrywiseMulAssign<Self> for NaturalVector {
    /// Replaces a [`NaturalVector`] by its entrywise product with another [`NaturalVector`] modulo
    /// $2^k$, taking the other by value. The elements of both must already be reduced modulo $2^k$.
    ///
    /// Every element is multiplied by the corresponding element of `other` and reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// f(v, w, k) = (v_0w_0 \bmod 2^k, v_1w_1 \bmod 2^k, \ldots, v_{n-1}w_{n-1} \bmod 2^k).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions, or if any element of either is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseMulAssign;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 6)").unwrap();
    /// v.mod_power_of_2_entrywise_mul_assign(w, 3);
    /// assert_eq!(v.to_string(), "(4, 7, 2)");
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_mul_assign(&mut self, other: Self, pow: u64) {
        mod_power_of_2_entrywise_mul_assign_val(self, other, pow);
    }
}

impl ModPowerOf2EntrywiseMulAssign<&Self> for NaturalVector {
    /// Replaces a [`NaturalVector`] by its entrywise product with another [`NaturalVector`] modulo
    /// $2^k$, taking the other by reference. The elements of both must already be reduced modulo
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
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` and `other` have different dimensions, or if any element of either is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2EntrywiseMulAssign;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 6)").unwrap();
    /// v.mod_power_of_2_entrywise_mul_assign(&w, 3);
    /// assert_eq!(v.to_string(), "(4, 7, 2)");
    /// ```
    #[inline]
    fn mod_power_of_2_entrywise_mul_assign(&mut self, other: &Self, pow: u64) {
        mod_power_of_2_entrywise_mul_assign_ref(self, other, pow);
    }
}
