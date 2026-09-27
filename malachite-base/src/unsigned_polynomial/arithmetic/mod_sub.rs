// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModSub, ModSubAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;
use core::cmp::min;

fn assert_reduced<T: PrimitiveUnsigned>(
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

// Subtracts `ys` from `xs` modulo m, negating the coefficients of `ys` past the end of `xs`. A
// nonzero reduced coefficient stays nonzero when negated. The caller trims, since leading
// coefficients can cancel.
fn sub_assign_ref<T: PrimitiveUnsigned>(xs: &mut Vec<T>, ys: &[T], m: T) {
    let common = min(xs.len(), ys.len());
    for (x, &y) in xs.iter_mut().zip(&ys[..common]) {
        *x = x.mod_sub(y, m);
    }
    if ys.len() > common {
        xs.extend(ys[common..].iter().map(|&y| y.mod_neg(m)));
    }
}

// Replaces `ys` with `xs - ys` modulo m, reusing the storage of `ys`. The caller trims.
fn rsub_assign_ref<T: PrimitiveUnsigned>(ys: &mut Vec<T>, xs: &[T], m: T) {
    let common = min(xs.len(), ys.len());
    for (y, &x) in ys.iter_mut().zip(&xs[..common]) {
        *y = x.mod_sub(*y, m);
    }
    for y in &mut ys[common..] {
        *y = y.mod_neg(m);
    }
    if xs.len() > common {
        ys.extend_from_slice(&xs[common..]);
    }
}

// Subtracts `ys` from `xs` modulo m, reusing whichever of the two is longer. The caller trims.
fn sub_assign_val<T: PrimitiveUnsigned>(xs: &mut Vec<T>, mut ys: Vec<T>, m: T) {
    if ys.len() > xs.len() {
        rsub_assign_ref(&mut ys, xs, m);
        *xs = ys;
    } else {
        sub_assign_ref(xs, &ys, m);
    }
}

fn mod_sub_owned_owned<T: PrimitiveUnsigned>(
    mut p: UnsignedPolynomial<T>,
    q: UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_reduced(&p, &q, m);
    sub_assign_val(&mut p.coefficients, q.coefficients, m);
    p.trim();
    p
}

fn mod_sub_owned_ref<T: PrimitiveUnsigned>(
    mut p: UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_reduced(&p, q, m);
    sub_assign_ref(&mut p.coefficients, &q.coefficients, m);
    p.trim();
    p
}

fn mod_sub_ref_owned<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    mut q: UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_reduced(p, &q, m);
    rsub_assign_ref(&mut q.coefficients, &p.coefficients, m);
    q.trim();
    q
}

fn mod_sub_ref_ref<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    assert_reduced(p, q, m);
    let mut coefficients = p.coefficients.clone();
    sub_assign_ref(&mut coefficients, &q.coefficients, m);
    let mut result = UnsignedPolynomial { coefficients };
    result.trim();
    result
}

fn mod_sub_assign_owned<T: PrimitiveUnsigned>(
    p: &mut UnsignedPolynomial<T>,
    q: UnsignedPolynomial<T>,
    m: T,
) {
    assert_reduced(p, &q, m);
    sub_assign_val(&mut p.coefficients, q.coefficients, m);
    p.trim();
}

fn mod_sub_assign_ref<T: PrimitiveUnsigned>(
    p: &mut UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) {
    assert_reduced(p, q, m);
    sub_assign_ref(&mut p.coefficients, &q.coefficients, m);
    p.trim();
}

impl<T: PrimitiveUnsigned> ModSub<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Subtracts one [`UnsignedPolynomial`] from another modulo `m`, taking both by value. The
    /// coefficients of both must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = p - q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. Where the second
    /// polynomial is longer, its coefficients are negated modulo `m`. When the two polynomials have
    /// the same degree, their leading coefficients can cancel modulo `m`, and then the degree of
    /// the difference is lower.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("5*x^2+x+3")
    ///         .unwrap()
    ///         .mod_sub(UnsignedPolynomial::from_str("2*x^2+6*x+1").unwrap(), 7)
    ///         .to_string(),
    ///     "3*x^2+2*x+2"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("5*x^2+x")
    ///         .unwrap()
    ///         .mod_sub(UnsignedPolynomial::from_str("5*x^2+3").unwrap(), 7)
    ///         .to_string(),
    ///     "x+4"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_sub` from `nmod_poly/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, other: Self, m: T) -> Self {
        mod_sub_owned_owned(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModSub<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Subtracts one [`UnsignedPolynomial`] from another modulo `m`, taking the first by value and
    /// the second by reference. The coefficients of both must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = p - q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. Where the second
    /// polynomial is longer, its coefficients are negated modulo `m`. When the two polynomials have
    /// the same degree, their leading coefficients can cancel modulo `m`, and then the degree of
    /// the difference is lower.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("5*x^2+x+3")
    ///         .unwrap()
    ///         .mod_sub(&UnsignedPolynomial::from_str("2*x^2+6*x+1").unwrap(), 7)
    ///         .to_string(),
    ///     "3*x^2+2*x+2"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("5*x^2+x")
    ///         .unwrap()
    ///         .mod_sub(&UnsignedPolynomial::from_str("5*x^2+3").unwrap(), 7)
    ///         .to_string(),
    ///     "x+4"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_sub` from `nmod_poly/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, other: &Self, m: T) -> Self {
        mod_sub_owned_ref(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModSub<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Subtracts one [`UnsignedPolynomial`] from another modulo `m`, taking the first by reference
    /// and the second by value. The coefficients of both must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = p - q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. Where the second
    /// polynomial is longer, its coefficients are negated modulo `m`. When the two polynomials have
    /// the same degree, their leading coefficients can cancel modulo `m`, and then the degree of
    /// the difference is lower.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap())
    ///         .mod_sub(UnsignedPolynomial::from_str("2*x^2+6*x+1").unwrap(), 7)
    ///         .to_string(),
    ///     "3*x^2+2*x+2"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("5*x^2+x").unwrap())
    ///         .mod_sub(UnsignedPolynomial::from_str("5*x^2+3").unwrap(), 7)
    ///         .to_string(),
    ///     "x+4"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_sub` from `nmod_poly/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, other: UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        mod_sub_ref_owned(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModSub<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Subtracts one [`UnsignedPolynomial`] from another modulo `m`, taking both by reference. The
    /// coefficients of both must already be reduced modulo `m`.
    ///
    /// $$
    /// f(p, q, m) = p - q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. Where the second
    /// polynomial is longer, its coefficients are negated modulo `m`. When the two polynomials have
    /// the same degree, their leading coefficients can cancel modulo `m`, and then the degree of
    /// the difference is lower.
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
    /// use malachite_base::num::arithmetic::traits::ModSub;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap())
    ///         .mod_sub(&UnsignedPolynomial::from_str("2*x^2+6*x+1").unwrap(), 7)
    ///         .to_string(),
    ///     "3*x^2+2*x+2"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("5*x^2+x").unwrap())
    ///         .mod_sub(&UnsignedPolynomial::from_str("5*x^2+3").unwrap(), 7)
    ///         .to_string(),
    ///     "x+4"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_sub` from `nmod_poly/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub(self, other: &UnsignedPolynomial<T>, m: T) -> UnsignedPolynomial<T> {
        mod_sub_ref_ref(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModSubAssign<Self, T> for UnsignedPolynomial<T> {
    /// Subtracts an [`UnsignedPolynomial`] from an [`UnsignedPolynomial`] modulo `m`, in place,
    /// taking the second by value. The coefficients of both must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p - q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. Where the second
    /// polynomial is longer, its coefficients are negated modulo `m`. When the two polynomials have
    /// the same degree, their leading coefficients can cancel modulo `m`, and then the degree of
    /// the difference is lower.
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
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap();
    /// p.mod_sub_assign(UnsignedPolynomial::from_str("2*x^2+6*x+1").unwrap(), 7);
    /// assert_eq!(p.to_string(), "3*x^2+2*x+2");
    ///
    /// // The leading coefficients cancel.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("5*x^2+x").unwrap();
    /// p.mod_sub_assign(UnsignedPolynomial::from_str("5*x^2+3").unwrap(), 7);
    /// assert_eq!(p.to_string(), "x+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_sub` from `nmod_poly/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub_assign(&mut self, other: Self, m: T) {
        mod_sub_assign_owned(self, other, m);
    }
}

impl<T: PrimitiveUnsigned> ModSubAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Subtracts an [`UnsignedPolynomial`] from an [`UnsignedPolynomial`] modulo `m`, in place,
    /// taking the second by reference. The coefficients of both must already be reduced modulo `m`.
    ///
    /// $$
    /// p \gets p - q \bmod m.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. Where the second
    /// polynomial is longer, its coefficients are negated modulo `m`. When the two polynomials have
    /// the same degree, their leading coefficients can cancel modulo `m`, and then the degree of
    /// the difference is lower.
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
    /// use malachite_base::num::arithmetic::traits::ModSubAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap();
    /// p.mod_sub_assign(&UnsignedPolynomial::from_str("2*x^2+6*x+1").unwrap(), 7);
    /// assert_eq!(p.to_string(), "3*x^2+2*x+2");
    ///
    /// // The leading coefficients cancel.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("5*x^2+x").unwrap();
    /// p.mod_sub_assign(&UnsignedPolynomial::from_str("5*x^2+3").unwrap(), 7);
    /// assert_eq!(p.to_string(), "x+4");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_sub` from `nmod_poly/sub.c`, FLINT 3.6.0.
    #[inline]
    fn mod_sub_assign(&mut self, other: &Self, m: T) {
        mod_sub_assign_ref(self, other, m);
    }
}
