// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2Add, ModPowerOf2AddAssign, ModPowerOf2IsReduced};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::swap;

fn assert_reduced<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    pow: u64,
) {
    assert!(pow <= T::WIDTH);
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        q.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {q} has a coefficient >= 2^{pow}"
    );
}

// Adds `ys` into `xs` modulo 2^pow, copying the coefficients of `ys` past the end of `xs`. The
// caller trims, since leading coefficients can cancel.
fn add_assign_ref<T: PrimitiveUnsigned>(xs: &mut Vec<T>, ys: &[T], pow: u64) {
    let common = min(xs.len(), ys.len());
    for (x, &y) in xs.iter_mut().zip(&ys[..common]) {
        *x = x.wrapping_add(y).mod_power_of_2(pow);
    }
    if ys.len() > common {
        xs.extend_from_slice(&ys[common..]);
    }
}

// Adds `ys` into `xs` modulo 2^pow, reusing whichever of the two is longer. The caller trims.
fn add_assign_val<T: PrimitiveUnsigned>(xs: &mut Vec<T>, mut ys: Vec<T>, pow: u64) {
    if ys.len() > xs.len() {
        swap(xs, &mut ys);
    }
    for (x, y) in xs.iter_mut().zip(ys) {
        *x = x.wrapping_add(y).mod_power_of_2(pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Add<Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Adds two [`UnsignedPolynomial`]s modulo $2^k$, taking both by value. The coefficients of
    /// both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p + q \bmod 2^k.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo $2^k$, and
    /// then the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `pow`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Add;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // Every coefficient wraps around, and the leading ones cancel.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("5*x^2+x+3")
    ///         .unwrap()
    ///         .mod_power_of_2_add(
    ///             UnsignedPolynomial::<u8>::from_str("3*x^2+7*x+1").unwrap(),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 1.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x+3")
    ///         .unwrap()
    ///         .mod_power_of_2_add(UnsignedPolynomial::<u8>::from_str("x^2+6").unwrap(), 3)
    ///         .to_string(),
    ///     "x^2+x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_add` from `nmod_poly/add.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_add(mut self, other: Self, pow: u64) -> Self {
        assert_reduced(&self, &other, pow);
        add_assign_val(&mut self.coefficients, other.coefficients, pow);
        self.trim();
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Add<&Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Adds two [`UnsignedPolynomial`]s modulo $2^k$, taking the first by value and the second by
    /// reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p + q \bmod 2^k.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo $2^k$, and
    /// then the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `pow`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Add;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // Every coefficient wraps around, and the leading ones cancel.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("5*x^2+x+3")
    ///         .unwrap()
    ///         .mod_power_of_2_add(
    ///             &UnsignedPolynomial::<u8>::from_str("3*x^2+7*x+1").unwrap(),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 1.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x+3")
    ///         .unwrap()
    ///         .mod_power_of_2_add(&UnsignedPolynomial::<u8>::from_str("x^2+6").unwrap(), 3)
    ///         .to_string(),
    ///     "x^2+x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_add` from `nmod_poly/add.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_add(mut self, other: &Self, pow: u64) -> Self {
        assert_reduced(&self, other, pow);
        add_assign_ref(&mut self.coefficients, &other.coefficients, pow);
        self.trim();
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Add<UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Adds two [`UnsignedPolynomial`]s modulo $2^k$, taking the first by reference and the second
    /// by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p + q \bmod 2^k.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo $2^k$, and
    /// then the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `pow`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Add;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // Every coefficient wraps around, and the leading ones cancel.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap())
    ///         .mod_power_of_2_add(
    ///             UnsignedPolynomial::<u8>::from_str("3*x^2+7*x+1").unwrap(),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 1.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x+3").unwrap())
    ///         .mod_power_of_2_add(UnsignedPolynomial::<u8>::from_str("x^2+6").unwrap(), 3)
    ///         .to_string(),
    ///     "x^2+x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_add` from `nmod_poly/add.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_add(
        self,
        mut other: UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        assert_reduced(self, &other, pow);
        add_assign_ref(&mut other.coefficients, &self.coefficients, pow);
        other.trim();
        other
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Add<&UnsignedPolynomial<T>> for &UnsignedPolynomial<T> {
    type Output = UnsignedPolynomial<T>;

    /// Adds two [`UnsignedPolynomial`]s modulo $2^k$, taking both by reference. The coefficients of
    /// both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p + q \bmod 2^k.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo $2^k$, and
    /// then the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `pow`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Add;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // Every coefficient wraps around, and the leading ones cancel.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap())
    ///         .mod_power_of_2_add(
    ///             &UnsignedPolynomial::<u8>::from_str("3*x^2+7*x+1").unwrap(),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "4"
    /// );
    /// // Wrapping around makes the constant term 1.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x+3").unwrap())
    ///         .mod_power_of_2_add(&UnsignedPolynomial::<u8>::from_str("x^2+6").unwrap(), 3)
    ///         .to_string(),
    ///     "x^2+x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_add` from `nmod_poly/add.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_add(self, other: &UnsignedPolynomial<T>, pow: u64) -> UnsignedPolynomial<T> {
        assert_reduced(self, other, pow);
        let mut coefficients = self.coefficients.clone();
        add_assign_ref(&mut coefficients, &other.coefficients, pow);
        let mut result = UnsignedPolynomial { coefficients };
        result.trim();
        result
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddAssign<Self> for UnsignedPolynomial<T> {
    /// Adds a [`UnsignedPolynomial`] to a [`UnsignedPolynomial`] modulo $2^k$, in place, taking the
    /// second by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets p + q \bmod 2^k.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo $2^k$, and
    /// then the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `pow`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // Every coefficient wraps around, and the leading ones cancel.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap();
    /// p.mod_power_of_2_add_assign(
    ///     UnsignedPolynomial::<u8>::from_str("3*x^2+7*x+1").unwrap(),
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "4");
    ///
    /// // Wrapping around makes the constant term 1.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// p.mod_power_of_2_add_assign(UnsignedPolynomial::<u8>::from_str("x^2+6").unwrap(), 3);
    /// assert_eq!(p.to_string(), "x^2+x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_add` from `nmod_poly/add.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_add_assign(&mut self, other: Self, pow: u64) {
        assert_reduced(self, &other, pow);
        add_assign_val(&mut self.coefficients, other.coefficients, pow);
        self.trim();
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2AddAssign<&Self> for UnsignedPolynomial<T> {
    /// Adds a [`UnsignedPolynomial`] to a [`UnsignedPolynomial`] modulo $2^k$, in place, taking the
    /// second by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets p + q \bmod 2^k.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. When the two
    /// polynomials have the same degree, their leading coefficients can cancel modulo $2^k$, and
    /// then the degree of the sum is lower.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients of the
    /// longer polynomial times `pow`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2AddAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// // Every coefficient wraps around, and the leading ones cancel.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("5*x^2+x+3").unwrap();
    /// p.mod_power_of_2_add_assign(
    ///     &UnsignedPolynomial::<u8>::from_str("3*x^2+7*x+1").unwrap(),
    ///     3,
    /// );
    /// assert_eq!(p.to_string(), "4");
    ///
    /// // Wrapping around makes the constant term 1.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x+3").unwrap();
    /// p.mod_power_of_2_add_assign(&UnsignedPolynomial::<u8>::from_str("x^2+6").unwrap(), 3);
    /// assert_eq!(p.to_string(), "x^2+x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_add` from `nmod_poly/add.c`, FLINT 3.6.0, with the modulus
    /// $2^k$.
    fn mod_power_of_2_add_assign(&mut self, other: &Self, pow: u64) {
        assert_reduced(self, other, pow);
        add_assign_ref(&mut self.coefficients, &other.coefficients, pow);
        self.trim();
    }
}
