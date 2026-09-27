// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{ModAdd, ModAddAssign, ModIsReduced};

fn assert_reduced(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(
        q.mod_is_reduced(m),
        "other must be reduced mod m, but {q} has a coefficient >= {m}"
    );
}

// Adds `ys` into `xs` modulo m, cloning the coefficients of `ys` past the end of `xs`. The caller
// trims, since leading coefficients can cancel.
fn add_assign_ref(xs: &mut Vec<Natural>, ys: &[Natural], m: &Natural) {
    let common = min(xs.len(), ys.len());
    for (x, y) in xs.iter_mut().zip(&ys[..common]) {
        x.mod_add_assign(y, m);
    }
    if ys.len() > common {
        xs.extend_from_slice(&ys[common..]);
    }
}

// Adds `ys` into `xs` modulo m, reusing whichever of the two is longer. The caller trims.
fn add_assign_val(xs: &mut Vec<Natural>, mut ys: Vec<Natural>, m: &Natural) {
    if ys.len() > xs.len() {
        swap(xs, &mut ys);
    }
    for (x, y) in xs.iter_mut().zip(ys) {
        x.mod_add_assign(y, m);
    }
}

fn mod_add_owned_owned(
    mut p: NaturalPolynomial,
    q: NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    assert_reduced(&p, &q, m);
    add_assign_val(&mut p.coefficients, q.coefficients, m);
    p.trim();
    p
}

fn mod_add_owned_ref(
    mut p: NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    assert_reduced(&p, q, m);
    add_assign_ref(&mut p.coefficients, &q.coefficients, m);
    p.trim();
    p
}

fn mod_add_ref_owned(
    p: &NaturalPolynomial,
    mut q: NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    assert_reduced(p, &q, m);
    add_assign_ref(&mut q.coefficients, &p.coefficients, m);
    q.trim();
    q
}

fn mod_add_ref_ref(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    assert_reduced(p, q, m);
    let mut coefficients = p.coefficients.clone();
    add_assign_ref(&mut coefficients, &q.coefficients, m);
    let mut result = NaturalPolynomial { coefficients };
    result.trim();
    result
}

fn mod_add_assign_owned(p: &mut NaturalPolynomial, q: NaturalPolynomial, m: &Natural) {
    assert_reduced(p, &q, m);
    add_assign_val(&mut p.coefficients, q.coefficients, m);
    p.trim();
}

fn mod_add_assign_ref(p: &mut NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) {
    assert_reduced(p, q, m);
    add_assign_ref(&mut p.coefficients, &q.coefficients, m);
    p.trim();
}

impl ModAdd<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by value, the second
    /// by value, and the modulus by value. The coefficients of both polynomials must already be
    /// reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = p + q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo `m`, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("5*x^2+x+3")
    ///         .unwrap()
    ///         .mod_add(
    ///             NaturalPolynomial::from_str("2*x^2+6*x+1").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 2.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x+3")
    ///         .unwrap()
    ///         .mod_add(
    ///             NaturalPolynomial::from_str("x^2+6").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add` from `fmpz_mod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: Self, m: Natural) -> Self {
        mod_add_owned_owned(self, other, &m)
    }
}

impl ModAdd<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by value, the second
    /// by value, and the modulus by reference. The coefficients of both polynomials must already be
    /// reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = p + q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo `m`, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("5*x^2+x+3")
    ///         .unwrap()
    ///         .mod_add(
    ///             NaturalPolynomial::from_str("2*x^2+6*x+1").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 2.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x+3")
    ///         .unwrap()
    ///         .mod_add(
    ///             NaturalPolynomial::from_str("x^2+6").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add` from `fmpz_mod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: Self, m: &Natural) -> Self {
        mod_add_owned_owned(self, other, m)
    }
}

impl ModAdd<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by value, the second
    /// by reference, and the modulus by value. The coefficients of both polynomials must already be
    /// reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = p + q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo `m`, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("5*x^2+x+3")
    ///         .unwrap()
    ///         .mod_add(
    ///             &NaturalPolynomial::from_str("2*x^2+6*x+1").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 2.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x+3")
    ///         .unwrap()
    ///         .mod_add(
    ///             &NaturalPolynomial::from_str("x^2+6").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add` from `fmpz_mod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: &Self, m: Natural) -> Self {
        mod_add_owned_ref(self, other, &m)
    }
}

impl ModAdd<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by value, the second
    /// by reference, and the modulus by reference. The coefficients of both polynomials must
    /// already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = p + q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo `m`, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("5*x^2+x+3")
    ///         .unwrap()
    ///         .mod_add(
    ///             &NaturalPolynomial::from_str("2*x^2+6*x+1").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 2.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x+3")
    ///         .unwrap()
    ///         .mod_add(
    ///             &NaturalPolynomial::from_str("x^2+6").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add` from `fmpz_mod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: &Self, m: &Natural) -> Self {
        mod_add_owned_ref(self, other, m)
    }
}

impl ModAdd<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by reference, the
    /// second by value, and the modulus by value. The coefficients of both polynomials must already
    /// be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = p + q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo `m`, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("5*x^2+x+3").unwrap())
    ///         .mod_add(
    ///             NaturalPolynomial::from_str("2*x^2+6*x+1").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 2.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+3").unwrap())
    ///         .mod_add(
    ///             NaturalPolynomial::from_str("x^2+6").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add` from `fmpz_mod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        mod_add_ref_owned(self, other, &m)
    }
}

impl ModAdd<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by reference, the
    /// second by value, and the modulus by reference. The coefficients of both polynomials must
    /// already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = p + q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo `m`, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("5*x^2+x+3").unwrap())
    ///         .mod_add(
    ///             NaturalPolynomial::from_str("2*x^2+6*x+1").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 2.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+3").unwrap())
    ///         .mod_add(
    ///             NaturalPolynomial::from_str("x^2+6").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add` from `fmpz_mod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        mod_add_ref_owned(self, other, m)
    }
}

impl ModAdd<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by reference, the
    /// second by reference, and the modulus by value. The coefficients of both polynomials must
    /// already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = p + q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo `m`, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("5*x^2+x+3").unwrap())
    ///         .mod_add(
    ///             &NaturalPolynomial::from_str("2*x^2+6*x+1").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 2.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+3").unwrap())
    ///         .mod_add(
    ///             &NaturalPolynomial::from_str("x^2+6").unwrap(),
    ///             Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add` from `fmpz_mod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: &NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        mod_add_ref_ref(self, other, &m)
    }
}

impl ModAdd<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Adds two [`NaturalPolynomial`]s modulo `m`, taking the first polynomial by reference, the
    /// second by reference, and the modulus by reference. The coefficients of both polynomials must
    /// already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = p + q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo `m`, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("5*x^2+x+3").unwrap())
    ///         .mod_add(
    ///             &NaturalPolynomial::from_str("2*x^2+6*x+1").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 2.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x+3").unwrap())
    ///         .mod_add(
    ///             &NaturalPolynomial::from_str("x^2+6").unwrap(),
    ///             &Natural::from(7u32)
    ///         )
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add` from `fmpz_mod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        mod_add_ref_ref(self, other, m)
    }
}

impl ModAddAssign<Self, Natural> for NaturalPolynomial {
    /// Adds a [`NaturalPolynomial`] to a [`NaturalPolynomial`] modulo `m`, in place, taking the
    /// second polynomial by value and the modulus by value. The coefficients of both polynomials
    /// must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p + q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo `m`, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// let mut p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// p.mod_add_assign(
    ///     NaturalPolynomial::from_str("2*x^2+6*x+1").unwrap(),
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "4");
    ///
    /// // Wrapping around makes the constant term 2.
    /// let mut p = NaturalPolynomial::from_str("x+3").unwrap();
    /// p.mod_add_assign(
    ///     NaturalPolynomial::from_str("x^2+6").unwrap(),
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "x^2+x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add` from `fmpz_mod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add_assign(&mut self, other: Self, m: Natural) {
        mod_add_assign_owned(self, other, &m);
    }
}

impl ModAddAssign<Self, &Natural> for NaturalPolynomial {
    /// Adds a [`NaturalPolynomial`] to a [`NaturalPolynomial`] modulo `m`, in place, taking the
    /// second polynomial by value and the modulus by reference. The coefficients of both
    /// polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p + q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo `m`, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// let mut p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// p.mod_add_assign(
    ///     NaturalPolynomial::from_str("2*x^2+6*x+1").unwrap(),
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "4");
    ///
    /// // Wrapping around makes the constant term 2.
    /// let mut p = NaturalPolynomial::from_str("x+3").unwrap();
    /// p.mod_add_assign(
    ///     NaturalPolynomial::from_str("x^2+6").unwrap(),
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "x^2+x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add` from `fmpz_mod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add_assign(&mut self, other: Self, m: &Natural) {
        mod_add_assign_owned(self, other, m);
    }
}

impl ModAddAssign<&Self, Natural> for NaturalPolynomial {
    /// Adds a [`NaturalPolynomial`] to a [`NaturalPolynomial`] modulo `m`, in place, taking the
    /// second polynomial by reference and the modulus by value. The coefficients of both
    /// polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p + q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo `m`, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// let mut p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// p.mod_add_assign(
    ///     &NaturalPolynomial::from_str("2*x^2+6*x+1").unwrap(),
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "4");
    ///
    /// // Wrapping around makes the constant term 2.
    /// let mut p = NaturalPolynomial::from_str("x+3").unwrap();
    /// p.mod_add_assign(
    ///     &NaturalPolynomial::from_str("x^2+6").unwrap(),
    ///     Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "x^2+x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add` from `fmpz_mod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add_assign(&mut self, other: &Self, m: Natural) {
        mod_add_assign_ref(self, other, &m);
    }
}

impl ModAddAssign<&Self, &Natural> for NaturalPolynomial {
    /// Adds a [`NaturalPolynomial`] to a [`NaturalPolynomial`] modulo `m`, in place, taking the
    /// second polynomial by reference and the modulus by reference. The coefficients of both
    /// polynomials must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p + q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo `m`, and then
    /// the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// let mut p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// p.mod_add_assign(
    ///     &NaturalPolynomial::from_str("2*x^2+6*x+1").unwrap(),
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "4");
    ///
    /// // Wrapping around makes the constant term 2.
    /// let mut p = NaturalPolynomial::from_str("x+3").unwrap();
    /// p.mod_add_assign(
    ///     &NaturalPolynomial::from_str("x^2+6").unwrap(),
    ///     &Natural::from(7u32),
    /// );
    /// assert_eq!(p.to_string(), "x^2+x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_add` from `fmpz_mod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add_assign(&mut self, other: &Self, m: &Natural) {
        mod_add_assign_ref(self, other, m);
    }
}
