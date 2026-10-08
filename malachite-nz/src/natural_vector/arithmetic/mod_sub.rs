// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModSub, ModSubAssign};

fn assert_reduced(v: &NaturalVector, w: &NaturalVector, m: &Natural) {
    assert_eq!(
        v.elements.len(),
        w.elements.len(),
        "cannot subtract vectors of different dimensions"
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

fn mod_sub_assign_val(v: &mut NaturalVector, w: NaturalVector, m: &Natural) {
    assert_reduced(v, &w, m);
    for (x, y) in v.elements.iter_mut().zip(w.elements) {
        x.mod_sub_assign(y, m);
    }
}

fn mod_sub_assign_ref(v: &mut NaturalVector, w: &NaturalVector, m: &Natural) {
    assert_reduced(v, w, m);
    for (x, y) in v.elements.iter_mut().zip(&w.elements) {
        x.mod_sub_assign(y, m);
    }
}

fn mod_sub_ref_val(v: &NaturalVector, w: &mut NaturalVector, m: &Natural) {
    assert_reduced(v, w, m);
    for (x, y) in w.elements.iter_mut().zip(&v.elements) {
        *x = y.mod_sub(take(x), m);
    }
}

fn mod_sub_ref_ref(v: &NaturalVector, w: &NaturalVector, m: &Natural) -> NaturalVector {
    assert_reduced(v, w, m);
    NaturalVector {
        elements: v
            .elements
            .iter()
            .zip(&w.elements)
            .map(|(x, y)| x.mod_sub(y, m))
            .collect(),
    }
}

impl ModSub<Self, Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the first by value, the
    /// second by value, and $m$ by value. The elements of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, m) = v - w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!(v.mod_sub(w, Natural::from(8u32)).to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(mut self, other: Self, m: Natural) -> Self {
        mod_sub_assign_val(&mut self, other, &m);
        self
    }
}

impl ModSub<Self, &Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the first by value, the
    /// second by value, and $m$ by reference. The elements of both must already be reduced modulo
    /// $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, m) = v - w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!(v.mod_sub(w, &Natural::from(8u32)).to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(mut self, other: Self, m: &Natural) -> Self {
        mod_sub_assign_val(&mut self, other, m);
        self
    }
}

impl ModSub<&Self, Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the first by value, the
    /// second by reference, and $m$ by value. The elements of both must already be reduced modulo
    /// $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, m) = v - w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!(v.mod_sub(&w, Natural::from(8u32)).to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(mut self, other: &Self, m: Natural) -> Self {
        mod_sub_assign_ref(&mut self, other, &m);
        self
    }
}

impl ModSub<&Self, &Natural> for NaturalVector {
    type Output = Self;

    /// Subtracts two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the first by value, the
    /// second by reference, and $m$ by reference. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, m) = v - w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!(v.mod_sub(&w, &Natural::from(8u32)).to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(mut self, other: &Self, m: &Natural) -> Self {
        mod_sub_assign_ref(&mut self, other, m);
        self
    }
}

impl ModSub<NaturalVector, Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Subtracts two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the first by reference,
    /// the second by value, and $m$ by value. The elements of both must already be reduced modulo
    /// $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, m) = v - w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_sub(w, Natural::from(8u32)).to_string(),
    ///     "(1, 2, 3)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, mut other: NaturalVector, m: Natural) -> NaturalVector {
        mod_sub_ref_val(self, &mut other, &m);
        other
    }
}

impl ModSub<NaturalVector, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Subtracts two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the first by reference,
    /// the second by value, and $m$ by reference. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, m) = v - w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_sub(w, &Natural::from(8u32)).to_string(),
    ///     "(1, 2, 3)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, mut other: NaturalVector, m: &Natural) -> NaturalVector {
        mod_sub_ref_val(self, &mut other, m);
        other
    }
}

impl ModSub<&NaturalVector, Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Subtracts two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the first by reference,
    /// the second by reference, and $m$ by value. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, m) = v - w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_sub(&w, Natural::from(8u32)).to_string(),
    ///     "(1, 2, 3)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, other: &NaturalVector, m: Natural) -> NaturalVector {
        mod_sub_ref_ref(self, other, &m)
    }
}

impl ModSub<&NaturalVector, &Natural> for &NaturalVector {
    type Output = NaturalVector;

    /// Subtracts two [`NaturalVector`]s modulo a [`Natural`] $m$, taking the first by reference,
    /// the second by reference, and $m$ by reference. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, m) = v - w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// assert_eq!(
    ///     (&v).mod_sub(&w, &Natural::from(8u32)).to_string(),
    ///     "(1, 2, 3)"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, other: &NaturalVector, m: &Natural) -> NaturalVector {
        mod_sub_ref_ref(self, other, m)
    }
}

impl ModSubAssign<Self, Natural> for NaturalVector {
    /// Subtracts a [`NaturalVector`] from a [`NaturalVector`] modulo a [`Natural`] $m$, in place,
    /// taking the [`NaturalVector`] on the right-hand side by value and $m$ by value. The elements
    /// of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, m) = v - w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// v.mod_sub_assign(w, Natural::from(8u32));
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub_assign(&mut self, other: Self, m: Natural) {
        mod_sub_assign_val(self, other, &m);
    }
}

impl ModSubAssign<Self, &Natural> for NaturalVector {
    /// Subtracts a [`NaturalVector`] from a [`NaturalVector`] modulo a [`Natural`] $m$, in place,
    /// taking the [`NaturalVector`] on the right-hand side by value and $m$ by reference. The
    /// elements of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, m) = v - w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// v.mod_sub_assign(w, &Natural::from(8u32));
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub_assign(&mut self, other: Self, m: &Natural) {
        mod_sub_assign_val(self, other, m);
    }
}

impl ModSubAssign<&Self, Natural> for NaturalVector {
    /// Subtracts a [`NaturalVector`] from a [`NaturalVector`] modulo a [`Natural`] $m$, in place,
    /// taking the [`NaturalVector`] on the right-hand side by reference and $m$ by value. The
    /// elements of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, m) = v - w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// v.mod_sub_assign(&w, Natural::from(8u32));
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub_assign(&mut self, other: &Self, m: Natural) {
        mod_sub_assign_ref(self, other, &m);
    }
}

impl ModSubAssign<&Self, &Natural> for NaturalVector {
    /// Subtracts a [`NaturalVector`] from a [`NaturalVector`] modulo a [`Natural`] $m$, in place,
    /// taking the [`NaturalVector`] on the right-hand side by reference and $m$ by reference. The
    /// elements of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same dimension as the operands.
    ///
    /// $$
    /// f(v, w, m) = v - w \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different dimensions, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let mut v = NaturalVector::from_str("(5, 1, 3)").unwrap();
    /// let w = NaturalVector::from_str("(4, 7, 0)").unwrap();
    /// v.mod_sub_assign(&w, &Natural::from(8u32));
    /// assert_eq!(v.to_string(), "(1, 2, 3)");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub_assign(&mut self, other: &Self, m: &Natural) {
        mod_sub_assign_ref(self, other, m);
    }
}
