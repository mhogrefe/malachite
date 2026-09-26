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
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2AddAssign, ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2NegAssign,
    ModPowerOf2Sub, ModPowerOf2SubAssign,
};

fn assert_reduced(p: &NaturalPolynomial, q: &NaturalPolynomial, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        q.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {q} has a coefficient >= 2^{pow}"
    );
}

// Subtracts `ys` from `xs` modulo 2^pow, negating the coefficients of `ys` past the end of `xs`. A
// nonzero reduced coefficient stays nonzero when negated. The caller trims, since leading
// coefficients can cancel.
fn sub_assign_ref(xs: &mut Vec<Natural>, ys: &[Natural], pow: u64) {
    let common = min(xs.len(), ys.len());
    for (x, y) in xs.iter_mut().zip(&ys[..common]) {
        x.mod_power_of_2_sub_assign(y, pow);
    }
    if ys.len() > common {
        xs.extend(ys[common..].iter().map(|y| y.mod_power_of_2_neg(pow)));
    }
}

// Replaces `ys` with `xs - ys` modulo 2^pow, reusing the storage of `ys`. The caller trims.
fn rsub_assign_ref(ys: &mut Vec<Natural>, xs: &[Natural], pow: u64) {
    for y in ys.iter_mut() {
        y.mod_power_of_2_neg_assign(pow);
    }
    let common = min(xs.len(), ys.len());
    for (y, x) in ys.iter_mut().zip(&xs[..common]) {
        y.mod_power_of_2_add_assign(x, pow);
    }
    if xs.len() > common {
        ys.extend_from_slice(&xs[common..]);
    }
}

// Subtracts `ys` from `xs` modulo 2^pow, reusing whichever of the two is longer. The caller trims.
fn sub_assign_val(xs: &mut Vec<Natural>, mut ys: Vec<Natural>, pow: u64) {
    if ys.len() > xs.len() {
        rsub_assign_ref(&mut ys, xs, pow);
        *xs = ys;
    } else {
        sub_assign_ref(xs, &ys, pow);
    }
}

impl ModPowerOf2Sub<Self> for NaturalPolynomial {
    type Output = Self;

    /// Subtracts one [`NaturalPolynomial`] from another modulo $2^k$, taking both by value. The
    /// coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p - q \bmod 2^k.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. Where the second
    /// polynomial is longer, its coefficients are negated modulo $2^k$. When the two polynomials
    /// have the same degree, their leading coefficients can cancel modulo $2^k$, and then the
    /// degree of the difference is lower.
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
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("5*x^2+x+3")
    ///         .unwrap()
    ///         .mod_power_of_2_sub(NaturalPolynomial::from_str("3*x^2+7*x+1").unwrap(), 3)
    ///         .to_string(),
    ///     "2*x^2+2*x+2"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("5*x^2+x")
    ///         .unwrap()
    ///         .mod_power_of_2_sub(NaturalPolynomial::from_str("5*x^2+3").unwrap(), 3)
    ///         .to_string(),
    ///     "x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sub` from `fmpz_mod_poly/sub.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_sub(mut self, other: Self, pow: u64) -> Self {
        assert_reduced(&self, &other, pow);
        sub_assign_val(&mut self.coefficients, other.coefficients, pow);
        self.trim();
        self
    }
}

impl ModPowerOf2Sub<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Subtracts one [`NaturalPolynomial`] from another modulo $2^k$, taking the first by value and
    /// the second by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p - q \bmod 2^k.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. Where the second
    /// polynomial is longer, its coefficients are negated modulo $2^k$. When the two polynomials
    /// have the same degree, their leading coefficients can cancel modulo $2^k$, and then the
    /// degree of the difference is lower.
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
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("5*x^2+x+3")
    ///         .unwrap()
    ///         .mod_power_of_2_sub(&NaturalPolynomial::from_str("3*x^2+7*x+1").unwrap(), 3)
    ///         .to_string(),
    ///     "2*x^2+2*x+2"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("5*x^2+x")
    ///         .unwrap()
    ///         .mod_power_of_2_sub(&NaturalPolynomial::from_str("5*x^2+3").unwrap(), 3)
    ///         .to_string(),
    ///     "x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sub` from `fmpz_mod_poly/sub.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_sub(mut self, other: &Self, pow: u64) -> Self {
        assert_reduced(&self, other, pow);
        sub_assign_ref(&mut self.coefficients, &other.coefficients, pow);
        self.trim();
        self
    }
}

impl ModPowerOf2Sub<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Subtracts one [`NaturalPolynomial`] from another modulo $2^k$, taking the first by reference
    /// and the second by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p - q \bmod 2^k.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. Where the second
    /// polynomial is longer, its coefficients are negated modulo $2^k$. When the two polynomials
    /// have the same degree, their leading coefficients can cancel modulo $2^k$, and then the
    /// degree of the difference is lower.
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
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("5*x^2+x+3").unwrap())
    ///         .mod_power_of_2_sub(NaturalPolynomial::from_str("3*x^2+7*x+1").unwrap(), 3)
    ///         .to_string(),
    ///     "2*x^2+2*x+2"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("5*x^2+x").unwrap())
    ///         .mod_power_of_2_sub(NaturalPolynomial::from_str("5*x^2+3").unwrap(), 3)
    ///         .to_string(),
    ///     "x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sub` from `fmpz_mod_poly/sub.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_sub(self, mut other: NaturalPolynomial, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, &other, pow);
        rsub_assign_ref(&mut other.coefficients, &self.coefficients, pow);
        other.trim();
        other
    }
}

impl ModPowerOf2Sub<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Subtracts one [`NaturalPolynomial`] from another modulo $2^k$, taking both by reference. The
    /// coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, k) = p - q \bmod 2^k.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. Where the second
    /// polynomial is longer, its coefficients are negated modulo $2^k$. When the two polynomials
    /// have the same degree, their leading coefficients can cancel modulo $2^k$, and then the
    /// degree of the difference is lower.
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
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Sub;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("5*x^2+x+3").unwrap())
    ///         .mod_power_of_2_sub(&NaturalPolynomial::from_str("3*x^2+7*x+1").unwrap(), 3)
    ///         .to_string(),
    ///     "2*x^2+2*x+2"
    /// );
    /// // The leading coefficients cancel.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("5*x^2+x").unwrap())
    ///         .mod_power_of_2_sub(&NaturalPolynomial::from_str("5*x^2+3").unwrap(), 3)
    ///         .to_string(),
    ///     "x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sub` from `fmpz_mod_poly/sub.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_sub(self, other: &NaturalPolynomial, pow: u64) -> NaturalPolynomial {
        assert_reduced(self, other, pow);
        let mut coefficients = self.coefficients.clone();
        sub_assign_ref(&mut coefficients, &other.coefficients, pow);
        let mut result = NaturalPolynomial { coefficients };
        result.trim();
        result
    }
}

impl ModPowerOf2SubAssign<Self> for NaturalPolynomial {
    /// Subtracts a [`NaturalPolynomial`] from a [`NaturalPolynomial`] modulo $2^k$, in place,
    /// taking the second by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \gets p - q \bmod 2^k.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. Where the second
    /// polynomial is longer, its coefficients are negated modulo $2^k$. When the two polynomials
    /// have the same degree, their leading coefficients can cancel modulo $2^k$, and then the
    /// degree of the difference is lower.
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
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// p.mod_power_of_2_sub_assign(NaturalPolynomial::from_str("3*x^2+7*x+1").unwrap(), 3);
    /// assert_eq!(p.to_string(), "2*x^2+2*x+2");
    ///
    /// // The leading coefficients cancel.
    /// let mut p = NaturalPolynomial::from_str("5*x^2+x").unwrap();
    /// p.mod_power_of_2_sub_assign(NaturalPolynomial::from_str("5*x^2+3").unwrap(), 3);
    /// assert_eq!(p.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sub` from `fmpz_mod_poly/sub.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_sub_assign(&mut self, other: Self, pow: u64) {
        assert_reduced(self, &other, pow);
        sub_assign_val(&mut self.coefficients, other.coefficients, pow);
        self.trim();
    }
}

impl ModPowerOf2SubAssign<&Self> for NaturalPolynomial {
    /// Subtracts a [`NaturalPolynomial`] from a [`NaturalPolynomial`] modulo $2^k$, in place,
    /// taking the second by reference. The coefficients of both must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// p \gets p - q \bmod 2^k.
    /// $$
    ///
    /// Coefficients past the end of the shorter polynomial are taken to be zero. Where the second
    /// polynomial is longer, its coefficients are negated modulo $2^k$. When the two polynomials
    /// have the same degree, their leading coefficients can cancel modulo $2^k$, and then the
    /// degree of the difference is lower.
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
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SubAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("5*x^2+x+3").unwrap();
    /// p.mod_power_of_2_sub_assign(&NaturalPolynomial::from_str("3*x^2+7*x+1").unwrap(), 3);
    /// assert_eq!(p.to_string(), "2*x^2+2*x+2");
    ///
    /// // The leading coefficients cancel.
    /// let mut p = NaturalPolynomial::from_str("5*x^2+x").unwrap();
    /// p.mod_power_of_2_sub_assign(&NaturalPolynomial::from_str("5*x^2+3").unwrap(), 3);
    /// assert_eq!(p.to_string(), "x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_sub` from `fmpz_mod_poly/sub.c`, FLINT 3.6.0, with the
    /// modulus $2^k$.
    fn mod_power_of_2_sub_assign(&mut self, other: &Self, pow: u64) {
        assert_reduced(self, other, pow);
        sub_assign_ref(&mut self.coefficients, &other.coefficients, pow);
        self.trim();
    }
}
