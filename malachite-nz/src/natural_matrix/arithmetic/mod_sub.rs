// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use core::mem::take;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModSub, ModSubAssign};

fn assert_reduced(a: &NaturalMatrix, b: &NaturalMatrix, m: &Natural) {
    assert!(
        a.height == b.height && a.width == b.width,
        "cannot subtract matrices of different shapes"
    );
    assert!(
        a.mod_is_reduced(m),
        "self must be reduced mod m, but {a} has an element >= {m}"
    );
    assert!(
        b.mod_is_reduced(m),
        "other must be reduced mod m, but {b} has an element >= {m}"
    );
}

fn mod_sub_assign_val(a: &mut NaturalMatrix, b: NaturalMatrix, m: &Natural) {
    assert_reduced(a, &b, m);
    for (x, y) in a.elements.iter_mut().zip(b.elements) {
        x.mod_sub_assign(y, m);
    }
}

fn mod_sub_assign_ref(a: &mut NaturalMatrix, b: &NaturalMatrix, m: &Natural) {
    assert_reduced(a, b, m);
    for (x, y) in a.elements.iter_mut().zip(&b.elements) {
        x.mod_sub_assign(y, m);
    }
}

fn mod_sub_ref_val(a: &NaturalMatrix, b: &mut NaturalMatrix, m: &Natural) {
    assert_reduced(a, b, m);
    for (x, y) in b.elements.iter_mut().zip(&a.elements) {
        *x = y.mod_sub(take(x), m);
    }
}

fn mod_sub_ref_ref(a: &NaturalMatrix, b: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    assert_reduced(a, b, m);
    NaturalMatrix::from_owned_elements(
        a.height(),
        a.width(),
        a.elements
            .iter()
            .zip(&b.elements)
            .map(|(x, y)| x.mod_sub(y, m))
            .collect(),
    )
}

impl ModSub<Self, Natural> for NaturalMatrix {
    type Output = Self;

    /// Subtracts two [`NaturalMatrix`]s modulo a [`Natural`] $m$, taking the first by value, the
    /// second by value, and $m$ by value. The elements of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_sub(b, Natural::from(8u32)).to_string(),
    ///     "[1, 2; 3, 4]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(mut self, other: Self, m: Natural) -> Self {
        mod_sub_assign_val(&mut self, other, &m);
        self
    }
}

impl ModSub<Self, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Subtracts two [`NaturalMatrix`]s modulo a [`Natural`] $m$, taking the first by value, the
    /// second by value, and $m$ by reference. The elements of both must already be reduced modulo
    /// $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_sub(b, &Natural::from(8u32)).to_string(),
    ///     "[1, 2; 3, 4]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(mut self, other: Self, m: &Natural) -> Self {
        mod_sub_assign_val(&mut self, other, m);
        self
    }
}

impl ModSub<&Self, Natural> for NaturalMatrix {
    type Output = Self;

    /// Subtracts two [`NaturalMatrix`]s modulo a [`Natural`] $m$, taking the first by value, the
    /// second by reference, and $m$ by value. The elements of both must already be reduced modulo
    /// $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_sub(&b, Natural::from(8u32)).to_string(),
    ///     "[1, 2; 3, 4]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(mut self, other: &Self, m: Natural) -> Self {
        mod_sub_assign_ref(&mut self, other, &m);
        self
    }
}

impl ModSub<&Self, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Subtracts two [`NaturalMatrix`]s modulo a [`Natural`] $m$, taking the first by value, the
    /// second by reference, and $m$ by reference. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(
    ///     a.mod_sub(&b, &Natural::from(8u32)).to_string(),
    ///     "[1, 2; 3, 4]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(mut self, other: &Self, m: &Natural) -> Self {
        mod_sub_assign_ref(&mut self, other, m);
        self
    }
}

impl ModSub<NaturalMatrix, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Subtracts two [`NaturalMatrix`]s modulo a [`Natural`] $m$, taking the first by reference,
    /// the second by value, and $m$ by value. The elements of both must already be reduced modulo
    /// $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_sub(b, Natural::from(8u32)).to_string(),
    ///     "[1, 2; 3, 4]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, mut other: NaturalMatrix, m: Natural) -> NaturalMatrix {
        mod_sub_ref_val(self, &mut other, &m);
        other
    }
}

impl ModSub<NaturalMatrix, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Subtracts two [`NaturalMatrix`]s modulo a [`Natural`] $m$, taking the first by reference,
    /// the second by value, and $m$ by reference. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_sub(b, &Natural::from(8u32)).to_string(),
    ///     "[1, 2; 3, 4]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, mut other: NaturalMatrix, m: &Natural) -> NaturalMatrix {
        mod_sub_ref_val(self, &mut other, m);
        other
    }
}

impl ModSub<&NaturalMatrix, Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Subtracts two [`NaturalMatrix`]s modulo a [`Natural`] $m$, taking the first by reference,
    /// the second by reference, and $m$ by value. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_sub(&b, Natural::from(8u32)).to_string(),
    ///     "[1, 2; 3, 4]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, other: &NaturalMatrix, m: Natural) -> NaturalMatrix {
        mod_sub_ref_ref(self, other, &m)
    }
}

impl ModSub<&NaturalMatrix, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Subtracts two [`NaturalMatrix`]s modulo a [`Natural`] $m$, taking the first by reference,
    /// the second by reference, and $m$ by reference. The elements of both must already be reduced
    /// modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[4, 7; 0, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_sub(&b, &Natural::from(8u32)).to_string(),
    ///     "[1, 2; 3, 4]"
    /// );
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, other: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
        mod_sub_ref_ref(self, other, m)
    }
}

impl ModSubAssign<Self, Natural> for NaturalMatrix {
    /// Subtracts a [`NaturalMatrix`] from a [`NaturalMatrix`] modulo a [`Natural`] $m$, in place,
    /// taking the [`NaturalMatrix`] on the right-hand side by value and $m$ by value. The elements
    /// of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[4, 7; 0, 6]").unwrap();
    /// a.mod_sub_assign(b, Natural::from(8u32));
    /// assert_eq!(a.to_string(), "[1, 2; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub_assign(&mut self, other: Self, m: Natural) {
        mod_sub_assign_val(self, other, &m);
    }
}

impl ModSubAssign<Self, &Natural> for NaturalMatrix {
    /// Subtracts a [`NaturalMatrix`] from a [`NaturalMatrix`] modulo a [`Natural`] $m$, in place,
    /// taking the [`NaturalMatrix`] on the right-hand side by value and $m$ by reference. The
    /// elements of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[4, 7; 0, 6]").unwrap();
    /// a.mod_sub_assign(b, &Natural::from(8u32));
    /// assert_eq!(a.to_string(), "[1, 2; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub_assign(&mut self, other: Self, m: &Natural) {
        mod_sub_assign_val(self, other, m);
    }
}

impl ModSubAssign<&Self, Natural> for NaturalMatrix {
    /// Subtracts a [`NaturalMatrix`] from a [`NaturalMatrix`] modulo a [`Natural`] $m$, in place,
    /// taking the [`NaturalMatrix`] on the right-hand side by reference and $m$ by value. The
    /// elements of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[4, 7; 0, 6]").unwrap();
    /// a.mod_sub_assign(&b, Natural::from(8u32));
    /// assert_eq!(a.to_string(), "[1, 2; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub_assign(&mut self, other: &Self, m: Natural) {
        mod_sub_assign_ref(self, other, &m);
    }
}

impl ModSubAssign<&Self, &Natural> for NaturalMatrix {
    /// Subtracts a [`NaturalMatrix`] from a [`NaturalMatrix`] modulo a [`Natural`] $m$, in place,
    /// taking the [`NaturalMatrix`] on the right-hand side by reference and $m$ by reference. The
    /// elements of both must already be reduced modulo $m$.
    ///
    /// The difference is taken element by element, each reduced modulo $m$, so the result has the
    /// same shape as the operands.
    ///
    /// $$
    /// f(A, B, m) = A - B \bmod m.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.height() * self.width()` times
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if `self` and `other` have different shapes, or if any element of
    /// either is greater than or equal to `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[5, 1; 3, 2]").unwrap();
    /// let b = NaturalMatrix::from_str("[4, 7; 0, 6]").unwrap();
    /// a.mod_sub_assign(&b, &Natural::from(8u32));
    /// assert_eq!(a.to_string(), "[1, 2; 3, 4]");
    /// ```
    ///
    /// This is equivalent to `_fmpz_mod_vec_sub` from `fmpz_mod_vec/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub_assign(&mut self, other: &Self, m: &Natural) {
        mod_sub_assign_ref(self, other, m);
    }
}
