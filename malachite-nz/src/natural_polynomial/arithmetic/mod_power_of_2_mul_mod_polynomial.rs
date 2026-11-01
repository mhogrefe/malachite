// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_polynomial::NaturalPolynomial;
use malachite_base::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Mul, Parity};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{
    ModPowerOf2MulModPolynomial, ModPowerOf2MulModPolynomialAssign, ModPowerOf2Rem, Polynomial,
};

// Checks that `a`, `b`, and `f` are reduced modulo $2^k$, that `f` is nonzero, and, if `check_lead`
// is set, has an odd leading coefficient, unless $k$ is 0, and that `a` and `b` are shorter than
// `f`, and returns whether $k$ is 0.
pub(crate) fn assert_mul_mod_polynomial_arguments(
    a: &NaturalPolynomial,
    b: &NaturalPolynomial,
    f: &NaturalPolynomial,
    pow: u64,
    check_lead: bool,
) -> bool {
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {a} has a coefficient >= 2^{pow}"
    );
    assert!(
        b.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {b} has a coefficient >= 2^{pow}"
    );
    assert!(
        f.mod_power_of_2_is_reduced(pow),
        "f must be reduced mod 2^pow, but {f} has a coefficient >= 2^{pow}"
    );
    if pow == 0 {
        return true;
    }
    assert_ne!(*f, NaturalPolynomial::ZERO, "division by zero");
    assert!(
        !check_lead || f.coefficients.last().unwrap().odd(),
        "the leading coefficient of f must be odd"
    );
    assert!(
        a.len() < f.len(),
        "self must be reduced mod f, but {a} has a degree >= that of {f}"
    );
    assert!(
        b.len() < f.len(),
        "other must be reduced mod f, but {b} has a degree >= that of {f}"
    );
    false
}

// The product of `a` and `b` modulo `f`, with coefficients modulo $2^k$.
//
// This is equivalent to `fmpz_mod_poly_mulmod` from `fmpz_mod_poly/mulmod.c`, FLINT 3.6.0, with the
// modulus $2^k$, except that both factors must be reduced modulo $f$.
pub(crate) fn mod_power_of_2_mul_mod_polynomial_helper(
    a: &NaturalPolynomial,
    b: &NaturalPolynomial,
    f: &NaturalPolynomial,
    pow: u64,
) -> NaturalPolynomial {
    if assert_mul_mod_polynomial_arguments(a, b, f, pow, true) {
        return NaturalPolynomial::ZERO;
    }
    let product = a.mod_power_of_2_mul(b, pow);
    if product.len() < f.len() {
        product
    } else {
        (&product).mod_power_of_2_rem(f, pow)
    }
}

impl ModPowerOf2MulModPolynomial<Self, Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by value, the second factor by value, and `f` by value. Both
    /// factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must
    /// be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `self` or `other` is
    /// not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial(b, f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod` from `fmpz_mod_poly/mulmod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(self, other: Self, f: Self, pow: u64) -> Self {
        mod_power_of_2_mul_mod_polynomial_helper(&self, &other, &f, pow)
    }
}

impl ModPowerOf2MulModPolynomial<Self, &Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by value, the second factor by value, and `f` by reference.
    /// Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must
    /// be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `self` or `other` is
    /// not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial(b, &f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod` from `fmpz_mod_poly/mulmod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(self, other: Self, f: &Self, pow: u64) -> Self {
        mod_power_of_2_mul_mod_polynomial_helper(&self, &other, f, pow)
    }
}

impl ModPowerOf2MulModPolynomial<&Self, Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by value, the second factor by reference, and `f` by value.
    /// Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must
    /// be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `self` or `other` is
    /// not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial(&b, f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod` from `fmpz_mod_poly/mulmod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(self, other: &Self, f: Self, pow: u64) -> Self {
        mod_power_of_2_mul_mod_polynomial_helper(&self, other, &f, pow)
    }
}

impl ModPowerOf2MulModPolynomial<&Self, &Self> for NaturalPolynomial {
    type Output = Self;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by value, the second factor by reference, and `f` by
    /// reference. Both factors must already be reduced modulo $f$, with degrees less than that of
    /// $f$, and the coefficients of all three must be reduced modulo $2^k$; the leading coefficient
    /// of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `self` or `other` is
    /// not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_mul_mod_polynomial(&b, &f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod` from `fmpz_mod_poly/mulmod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(self, other: &Self, f: &Self, pow: u64) -> Self {
        mod_power_of_2_mul_mod_polynomial_helper(&self, other, f, pow)
    }
}

impl ModPowerOf2MulModPolynomial<NaturalPolynomial, NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by reference, the second factor by value, and `f` by value.
    /// Both factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must
    /// be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `self` or `other` is
    /// not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial(b, f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod` from `fmpz_mod_poly/mulmod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(
        self,
        other: NaturalPolynomial,
        f: NaturalPolynomial,
        pow: u64,
    ) -> NaturalPolynomial {
        mod_power_of_2_mul_mod_polynomial_helper(self, &other, &f, pow)
    }
}

impl ModPowerOf2MulModPolynomial<NaturalPolynomial, &NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by reference, the second factor by value, and `f` by
    /// reference. Both factors must already be reduced modulo $f$, with degrees less than that of
    /// $f$, and the coefficients of all three must be reduced modulo $2^k$; the leading coefficient
    /// of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `self` or `other` is
    /// not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial(b, &f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod` from `fmpz_mod_poly/mulmod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(
        self,
        other: NaturalPolynomial,
        f: &NaturalPolynomial,
        pow: u64,
    ) -> NaturalPolynomial {
        mod_power_of_2_mul_mod_polynomial_helper(self, &other, f, pow)
    }
}

impl ModPowerOf2MulModPolynomial<&NaturalPolynomial, NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by reference, the second factor by reference, and `f` by
    /// value. Both factors must already be reduced modulo $f$, with degrees less than that of $f$,
    /// and the coefficients of all three must be reduced modulo $2^k$; the leading coefficient of
    /// $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `self` or `other` is
    /// not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial(&b, f, 3).to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod` from `fmpz_mod_poly/mulmod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(
        self,
        other: &NaturalPolynomial,
        f: NaturalPolynomial,
        pow: u64,
    ) -> NaturalPolynomial {
        mod_power_of_2_mul_mod_polynomial_helper(self, other, &f, pow)
    }
}

impl ModPowerOf2MulModPolynomial<&NaturalPolynomial, &NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Multiplies two [`NaturalPolynomial`]s modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the first factor by reference, the second factor by reference, and `f` by
    /// reference. Both factors must already be reduced modulo $f$, with degrees less than that of
    /// $f$, and the coefficients of all three must be reduced modulo $2^k$; the leading coefficient
    /// of $f$ must be odd.
    ///
    /// $$
    /// (a, b) \mapsto r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `self` or `other` is
    /// not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_mul_mod_polynomial(&b, &f, 3)
    ///         .to_string(),
    ///     "x^2+x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod` from `fmpz_mod_poly/mulmod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial(
        self,
        other: &NaturalPolynomial,
        f: &NaturalPolynomial,
        pow: u64,
    ) -> NaturalPolynomial {
        mod_power_of_2_mul_mod_polynomial_helper(self, other, f, pow)
    }
}

impl ModPowerOf2MulModPolynomialAssign<Self, Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $2^k$, in place, taking the other factor by value and `f` by value. Both factors must
    /// already be reduced modulo $f$, with degrees less than that of $f$, and the coefficients of
    /// all three must be reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `self` or `other` is
    /// not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulModPolynomialAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_assign(b, f, 3);
    /// assert_eq!(a.to_string(), "x^2+x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod` from `fmpz_mod_poly/mulmod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_assign(&mut self, other: Self, f: Self, pow: u64) {
        *self = mod_power_of_2_mul_mod_polynomial_helper(self, &other, &f, pow);
    }
}

impl ModPowerOf2MulModPolynomialAssign<Self, &Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $2^k$, in place, taking the other factor by value and `f` by reference. Both factors
    /// must already be reduced modulo $f$, with degrees less than that of $f$, and the coefficients
    /// of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `self` or `other` is
    /// not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulModPolynomialAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_assign(b, &f, 3);
    /// assert_eq!(a.to_string(), "x^2+x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod` from `fmpz_mod_poly/mulmod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_assign(&mut self, other: Self, f: &Self, pow: u64) {
        *self = mod_power_of_2_mul_mod_polynomial_helper(self, &other, f, pow);
    }
}

impl ModPowerOf2MulModPolynomialAssign<&Self, Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $2^k$, in place, taking the other factor by reference and `f` by value. Both factors
    /// must already be reduced modulo $f$, with degrees less than that of $f$, and the coefficients
    /// of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `self` or `other` is
    /// not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulModPolynomialAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_assign(&b, f, 3);
    /// assert_eq!(a.to_string(), "x^2+x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod` from `fmpz_mod_poly/mulmod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_assign(&mut self, other: &Self, f: Self, pow: u64) {
        *self = mod_power_of_2_mul_mod_polynomial_helper(self, other, &f, pow);
    }
}

impl ModPowerOf2MulModPolynomialAssign<&Self, &Self> for NaturalPolynomial {
    /// Multiplies a [`NaturalPolynomial`] by another modulo a polynomial $f$, with coefficients
    /// modulo $2^k$, in place, taking the other factor by reference and `f` by reference. Both
    /// factors must already be reduced modulo $f$, with degrees less than that of $f$, and the
    /// coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $f$ must
    /// be odd.
    ///
    /// $$
    /// a \gets r, \quad ab \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The product is reduced modulo $f$ by Euclidean division, as by [`ModPowerOf2Rem`].
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `other`, or `f` is greater than or equal to $2^k$, or
    /// if `f` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `self` or `other` is
    /// not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2MulModPolynomialAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// let b = NaturalPolynomial::from_str("x^2+4*x+2").unwrap();
    /// a.mod_power_of_2_mul_mod_polynomial_assign(&b, &f, 3);
    /// assert_eq!(a.to_string(), "x^2+x+5");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_mulmod` from `fmpz_mod_poly/mulmod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that both factors must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_mul_mod_polynomial_assign(&mut self, other: &Self, f: &Self, pow: u64) {
        *self = mod_power_of_2_mul_mod_polynomial_helper(self, other, f, pow);
    }
}
