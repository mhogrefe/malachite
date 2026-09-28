// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModAdd, ModAddAssign, ModIsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::swap;

pub(crate) fn assert_reduced<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(
        q.mod_is_reduced(&m),
        "other must be reduced mod m, but {q} has a coefficient >= {m}"
    );
}

// Adds `ys` into `xs` modulo m, copying the coefficients of `ys` past the end of `xs`. The caller
// trims, since leading coefficients can cancel.
pub(crate) fn add_assign_ref<T: PrimitiveUnsigned>(xs: &mut Vec<T>, ys: &[T], m: T) {
    let common = min(xs.len(), ys.len());
    for (x, &y) in xs.iter_mut().zip(&ys[..common]) {
        *x = x.mod_add(y, m);
    }
    if ys.len() > common {
        xs.extend_from_slice(&ys[common..]);
    }
}

// Adds `ys` into `xs` modulo m, reusing whichever of the two is longer. The caller trims.
pub(crate) fn add_assign_val<T: PrimitiveUnsigned>(xs: &mut Vec<T>, mut ys: Vec<T>, m: T) {
    if ys.len() > xs.len() {
        swap(xs, &mut ys);
    }
    for (x, y) in xs.iter_mut().zip(ys) {
        *x = x.mod_add(y, m);
    }
}

fn mod_add_owned_owned<T: PrimitiveUnsigned>(
    mut p: UnsignedPolynomial<T>,
    q: UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_reduced(&p, &q, m);
    add_assign_val(&mut p.coefficients, q.coefficients, m);
    p.trim();
    p
}

fn mod_add_owned_ref<T: PrimitiveUnsigned>(
    mut p: UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_reduced(&p, q, m);
    add_assign_ref(&mut p.coefficients, &q.coefficients, m);
    p.trim();
    p
}

fn mod_add_ref_owned<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    mut q: UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_reduced(p, &q, m);
    add_assign_ref(&mut q.coefficients, &p.coefficients, m);
    q.trim();
    q
}

fn mod_add_ref_ref<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_reduced(p, q, m);
    let mut coefficients = p.coefficients.clone();
    add_assign_ref(&mut coefficients, &q.coefficients, m);
    let mut result = UnsignedPolynomial { coefficients };
    result.trim();
    result
}

fn mod_add_assign_owned<T: PrimitiveUnsigned>(
    p: &mut UnsignedPolynomial<T>,
    q: UnsignedPolynomial<T>,
    m: T,
) {
    assert_reduced(p, &q, m);
    add_assign_val(&mut p.coefficients, q.coefficients, m);
    p.trim();
}

fn mod_add_assign_ref<T: PrimitiveUnsigned>(
    p: &mut UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) {
    assert_reduced(p, q, m);
    add_assign_ref(&mut p.coefficients, &q.coefficients, m);
    p.trim();
}

impl<T: PrimitiveUnsigned> ModAdd<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Adds two [`UnsignedPolynomial`]s modulo `m`, taking both by value. The coefficients of both
    /// must already be reduced modulo `m`.
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
    /// longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("5*x^2+x+3")
    ///         .unwrap()
    ///         .mod_add(UnsignedPolynomial::from_str("2*x^2+6*x+1").unwrap(), 7)
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 2.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x+3")
    ///         .unwrap()
    ///         .mod_add(UnsignedPolynomial::from_str("x^2+6").unwrap(), 7)
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_add` from `nmod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: Self, m: T) -> Self {
        mod_add_owned_owned(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModAdd<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Adds two [`UnsignedPolynomial`]s modulo `m`, taking the first by value and the second by
    /// reference. The coefficients of both must already be reduced modulo `m`.
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
    /// longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("5*x^2+x+3")
    ///         .unwrap()
    ///         .mod_add(&UnsignedPolynomial::from_str("2*x^2+6*x+1").unwrap(), 7)
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 2.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x+3")
    ///         .unwrap()
    ///         .mod_add(&UnsignedPolynomial::from_str("x^2+6").unwrap(), 7)
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_add` from `nmod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: &Self, m: T) -> Self {
        mod_add_owned_ref(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModAdd<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Adds two [`UnsignedPolynomial`]s modulo `m`, taking the first by reference and the second by
    /// value. The coefficients of both must already be reduced modulo `m`.
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
    /// longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap())
    ///         .mod_add(UnsignedPolynomial::from_str("2*x^2+6*x+1").unwrap(), 7)
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 2.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x+3").unwrap())
    ///         .mod_add(UnsignedPolynomial::from_str("x^2+6").unwrap(), 7)
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_add` from `nmod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        mod_add_ref_owned(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModAdd<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Adds two [`UnsignedPolynomial`]s modulo `m`, taking both by reference. The coefficients of
    /// both must already be reduced modulo `m`.
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
    /// longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAdd;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap())
    ///         .mod_add(&UnsignedPolynomial::from_str("2*x^2+6*x+1").unwrap(), 7)
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 2.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x+3").unwrap())
    ///         .mod_add(&UnsignedPolynomial::from_str("x^2+6").unwrap(), 7)
    ///         .to_string(),
    ///     "x^2+x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_add` from `nmod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add(self, other: &UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        mod_add_ref_ref(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModAddAssign<Self, T> for UnsignedPolynomial<T> {
    /// Adds an [`UnsignedPolynomial`] to an [`UnsignedPolynomial`] modulo `m`, in place, taking the
    /// second by value. The coefficients of both must already be reduced modulo `m`.
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
    /// longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap();
    /// p.mod_add_assign(UnsignedPolynomial::from_str("2*x^2+6*x+1").unwrap(), 7);
    /// assert_eq!(p.to_string(), "4");
    ///
    /// // Wrapping around makes the constant term 2.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// p.mod_add_assign(UnsignedPolynomial::from_str("x^2+6").unwrap(), 7);
    /// assert_eq!(p.to_string(), "x^2+x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_add` from `nmod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add_assign(&mut self, other: Self, m: T) {
        mod_add_assign_owned(self, other, m);
    }
}

impl<T: PrimitiveUnsigned> ModAddAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Adds an [`UnsignedPolynomial`] to an [`UnsignedPolynomial`] modulo `m`, in place, taking the
    /// second by reference. The coefficients of both must already be reduced modulo `m`.
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
    /// longer polynomial.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModAddAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // The leading coefficients cancel, and so do the linear ones.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap();
    /// p.mod_add_assign(&UnsignedPolynomial::from_str("2*x^2+6*x+1").unwrap(), 7);
    /// assert_eq!(p.to_string(), "4");
    ///
    /// // Wrapping around makes the constant term 2.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// p.mod_add_assign(&UnsignedPolynomial::from_str("x^2+6").unwrap(), 7);
    /// assert_eq!(p.to_string(), "x^2+x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_add` from `nmod_poly/add.c`, FLINT 3.6.0.
    #[inline]
    fn mod_add_assign(&mut self, other: &Self, m: T) {
        mod_add_assign_ref(self, other, m);
    }
}
