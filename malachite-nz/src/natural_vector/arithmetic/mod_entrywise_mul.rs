// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::{
    ModEntrywiseMul, ModEntrywiseMulAssign, ModIsReduced, ModMulPrecomputed,
    ModMulPrecomputedAssign,
};

fn assert_valid(v: &NaturalVector, w: &NaturalVector, m: &Natural) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot multiply vectors of different dimensions entrywise"
    );
    assert!(
        v.mod_is_reduced(m),
        "self must be reduced mod m, but {v} has an element >= {m}"
    );
    assert!(
        w.mod_is_reduced(m),
        "other must be reduced mod m, but {w} has an element >= {m}"
    );
}

// Multiplies each element of `v` by the element of `w` at the same index, modulo `m`. The data for
// multiplying modulo `m` is computed once and shared by all the elements.
fn mod_entrywise_mul_assign_ref(v: &mut NaturalVector, w: &NaturalVector, m: &Natural) {
    assert_valid(v, w, m);
    let data = <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
    for (x, y) in v.elements.iter_mut().zip(&w.elements) {
        x.mod_mul_precomputed_assign(y, m, &data);
    }
}

fn mod_entrywise_mul_ref_ref(v: &NaturalVector, w: &NaturalVector, m: &Natural) -> NaturalVector {
    assert_valid(v, w, m);
    let data = <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
    NaturalVector {
        elements: v
            .elements
            .iter()
            .zip(&w.elements)
            .map(|(x, y)| x.mod_mul_precomputed(y, m, &data))
            .collect(),
    }
}

impl ModEntrywiseMul<Self, Natural> for NaturalVector {
    type Output = Self;

    /// Takes the entrywise product of two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the
    /// first by value, the second by value, and $m$ by value. The elements of both must already be
    /// reduced modulo $m$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $m$, so the result has the same dimension as both vectors. The data for multiplying modulo
    /// $m$ is precomputed once and shared by all the elements.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 6, 5)").unwrap();
    /// assert_eq!(
    ///     v.mod_entrywise_mul(w, Natural::from(7u32)).to_string(),
    ///     "(6, 6, 1)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_mul` from `fmpz_mod_vec/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mod_entrywise_mul(mut self, other: Self, m: Natural) -> Self {
        mod_entrywise_mul_assign_ref(&mut self, &other, &m);
        self
    }
}

impl ModEntrywiseMul<&Self, Natural> for NaturalVector {
    type Output = Self;

    /// Takes the entrywise product of two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the
    /// first by value, the second by reference, and $m$ by value. The elements of both must already
    /// be reduced modulo $m$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $m$, so the result has the same dimension as both vectors. The data for multiplying modulo
    /// $m$ is precomputed once and shared by all the elements.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 6, 5)").unwrap();
    /// assert_eq!(
    ///     v.mod_entrywise_mul(&w, Natural::from(7u32)).to_string(),
    ///     "(6, 6, 1)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_mul` from `fmpz_mod_vec/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mod_entrywise_mul(mut self, other: &Self, m: Natural) -> Self {
        mod_entrywise_mul_assign_ref(&mut self, other, &m);
        self
    }
}

impl ModEntrywiseMul<NaturalVector, Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Takes the entrywise product of two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the
    /// first by reference, the second by value, and $m$ by value. The elements of both must already
    /// be reduced modulo $m$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $m$, so the result has the same dimension as both vectors. The data for multiplying modulo
    /// $m$ is precomputed once and shared by all the elements.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 6, 5)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_entrywise_mul(w, Natural::from(7u32)).to_string(),
    ///     "(6, 6, 1)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_mul` from `fmpz_mod_vec/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mod_entrywise_mul(self, mut other: NaturalVector, m: Natural) -> NaturalVector {
        // The product is commutative, so the owned vector can hold the result.
        mod_entrywise_mul_assign_ref(&mut other, self, &m);
        other
    }
}

impl ModEntrywiseMul<&NaturalVector, Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Takes the entrywise product of two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the
    /// first by reference, the second by reference, and $m$ by value. The elements of both must
    /// already be reduced modulo $m$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $m$, so the result has the same dimension as both vectors. The data for multiplying modulo
    /// $m$ is precomputed once and shared by all the elements.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 6, 5)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_entrywise_mul(&w, Natural::from(7u32)).to_string(),
    ///     "(6, 6, 1)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_mul` from `fmpz_mod_vec/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mod_entrywise_mul(self, other: &NaturalVector, m: Natural) -> NaturalVector {
        mod_entrywise_mul_ref_ref(self, other, &m)
    }
}

impl ModEntrywiseMul<Self, &Natural> for NaturalVector {
    type Output = Self;

    /// Takes the entrywise product of two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the
    /// first by value, the second by value, and $m$ by reference. The elements of both must already
    /// be reduced modulo $m$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $m$, so the result has the same dimension as both vectors. The data for multiplying modulo
    /// $m$ is precomputed once and shared by all the elements.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 6, 5)").unwrap();
    /// assert_eq!(
    ///     v.mod_entrywise_mul(w, &Natural::from(7u32)).to_string(),
    ///     "(6, 6, 1)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_mul` from `fmpz_mod_vec/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mod_entrywise_mul(mut self, other: Self, m: &Natural) -> Self {
        mod_entrywise_mul_assign_ref(&mut self, &other, m);
        self
    }
}

impl ModEntrywiseMul<&Self, &Natural> for NaturalVector {
    type Output = Self;

    /// Takes the entrywise product of two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the
    /// first by value, the second by reference, and $m$ by reference. The elements of both must
    /// already be reduced modulo $m$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $m$, so the result has the same dimension as both vectors. The data for multiplying modulo
    /// $m$ is precomputed once and shared by all the elements.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 6, 5)").unwrap();
    /// assert_eq!(
    ///     v.mod_entrywise_mul(&w, &Natural::from(7u32)).to_string(),
    ///     "(6, 6, 1)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_mul` from `fmpz_mod_vec/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mod_entrywise_mul(mut self, other: &Self, m: &Natural) -> Self {
        mod_entrywise_mul_assign_ref(&mut self, other, m);
        self
    }
}

impl ModEntrywiseMul<NaturalVector, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Takes the entrywise product of two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the
    /// first by reference, the second by value, and $m$ by reference. The elements of both must
    /// already be reduced modulo $m$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $m$, so the result has the same dimension as both vectors. The data for multiplying modulo
    /// $m$ is precomputed once and shared by all the elements.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 6, 5)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_entrywise_mul(w, &Natural::from(7u32)).to_string(),
    ///     "(6, 6, 1)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_mul` from `fmpz_mod_vec/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mod_entrywise_mul(self, mut other: NaturalVector, m: &Natural) -> NaturalVector {
        // The product is commutative, so the owned vector can hold the result.
        mod_entrywise_mul_assign_ref(&mut other, self, m);
        other
    }
}

impl ModEntrywiseMul<&NaturalVector, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Takes the entrywise product of two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the
    /// first by reference, the second by reference, and $m$ by reference. The elements of both must
    /// already be reduced modulo $m$.
    ///
    /// Every element of the result is the product of the corresponding elements, reduced modulo
    /// $m$, so the result has the same dimension as both vectors. The data for multiplying modulo
    /// $m$ is precomputed once and shared by all the elements.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMul;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 6, 5)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_entrywise_mul(&w, &Natural::from(7u32)).to_string(),
    ///     "(6, 6, 1)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_mul` from `fmpz_mod_vec/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mod_entrywise_mul(self, other: &NaturalVector, m: &Natural) -> NaturalVector {
        mod_entrywise_mul_ref_ref(self, other, m)
    }
}

impl ModEntrywiseMulAssign<Self, Natural> for NaturalVector {
    /// Replaces a [`NaturalVector`] by its entrywise product with another [`NaturalVector`] modulo
    /// a [`Natural`] $m$, taking the other by value, and $m$ by value. The elements of both must
    /// already be reduced modulo $m$.
    ///
    /// Every element is multiplied by the corresponding element of `other` and reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 6, 5)").unwrap();
    /// v.mod_entrywise_mul_assign(w, Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(6, 6, 1)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_mul` from `fmpz_mod_vec/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mod_entrywise_mul_assign(&mut self, other: Self, m: Natural) {
        mod_entrywise_mul_assign_ref(self, &other, &m);
    }
}

impl ModEntrywiseMulAssign<&Self, Natural> for NaturalVector {
    /// Replaces a [`NaturalVector`] by its entrywise product with another [`NaturalVector`] modulo
    /// a [`Natural`] $m$, taking the other by reference, and $m$ by value. The elements of both
    /// must already be reduced modulo $m$.
    ///
    /// Every element is multiplied by the corresponding element of `other` and reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 6, 5)").unwrap();
    /// v.mod_entrywise_mul_assign(&w, Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(6, 6, 1)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_mul` from `fmpz_mod_vec/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mod_entrywise_mul_assign(&mut self, other: &Self, m: Natural) {
        mod_entrywise_mul_assign_ref(self, other, &m);
    }
}

impl ModEntrywiseMulAssign<Self, &Natural> for NaturalVector {
    /// Replaces a [`NaturalVector`] by its entrywise product with another [`NaturalVector`] modulo
    /// a [`Natural`] $m$, taking the other by value, and $m$ by reference. The elements of both
    /// must already be reduced modulo $m$.
    ///
    /// Every element is multiplied by the corresponding element of `other` and reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 6, 5)").unwrap();
    /// v.mod_entrywise_mul_assign(w, &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(6, 6, 1)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_mul` from `fmpz_mod_vec/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mod_entrywise_mul_assign(&mut self, other: Self, m: &Natural) {
        mod_entrywise_mul_assign_ref(self, &other, m);
    }
}

impl ModEntrywiseMulAssign<&Self, &Natural> for NaturalVector {
    /// Replaces a [`NaturalVector`] by its entrywise product with another [`NaturalVector`] modulo
    /// a [`Natural`] $m$, taking the other by reference, and $m$ by reference. The elements of both
    /// must already be reduced modulo $m$.
    ///
    /// Every element is multiplied by the corresponding element of `other` and reduced modulo $m$.
    ///
    /// $$
    /// f(v, w, m) = (v_0w_0 \bmod m, v_1w_1 \bmod m, \ldots, v_{n-1}w_{n-1} \bmod m).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(nm \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModEntrywiseMulAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 6, 5)").unwrap();
    /// v.mod_entrywise_mul_assign(&w, &Natural::from(7u32));
    /// assert_eq!(v.to_string(), "(6, 6, 1)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_mul` from `fmpz_mod_vec/mul.c`, FLINT 3.6.0.
    #[inline]
    fn mod_entrywise_mul_assign(&mut self, other: &Self, m: &Natural) {
        mod_entrywise_mul_assign_ref(self, other, m);
    }
}
