// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
//      Copyright © 2012 Lina Kulakova
//
//      Copyright © 2013, 2014 Martin Lee
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_div_rem_precomputed::precompute_helper;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul_mod_polynomial::*;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Mul, ModPowerOf2Square};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::polynomial::{
    ModPolynomialData, ModPowerOf2PowModPolynomial, ModPowerOf2PowModPolynomialAssign,
    ModPowerOf2Rem, ModPowerOf2RemPrecomputed, MulPowerOfX, Polynomial,
};
use malachite_base::unsigned_polynomial::arithmetic::mod_pow_mod_polynomial::*;

// Whether `a` is the polynomial $x$.
pub(crate) fn is_x(a: &NaturalPolynomial) -> bool {
    a.coefficients == [Natural::ZERO, Natural::ONE]
}

// The power $x^e$ modulo `f`, with coefficients modulo $2^k$, where $e$ is `exp`, which is at least
// 3, and `f` has at least 3 coefficients, with `data` precomputed from `f` and $k$.
pub(crate) fn power_of_x_precomputed(
    exp: u64,
    f: &NaturalPolynomial,
    pow: u64,
    data: &ModPolynomialData<Natural>,
) -> NaturalPolynomial {
    let reduce = |p: NaturalPolynomial| {
        if p.len() < f.len() {
            p
        } else {
            (&p).mod_power_of_2_rem_precomputed(f, pow, data)
        }
    };
    pow_of_x_sliding_window(
        exp,
        f.len(),
        NaturalPolynomial::one(),
        |r| reduce((&r).mod_power_of_2_square(pow)),
        |r, w| reduce(r.mul_power_of_x(w)),
    )
}

// The power of `a` with exponent `exp` modulo `f`, with coefficients modulo $2^k$.
//
// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus $2^k$,
// except that the base must be reduced modulo $f$.
pub(crate) fn mod_power_of_2_pow_mod_polynomial_helper(
    a: &NaturalPolynomial,
    exp: u64,
    f: &NaturalPolynomial,
    pow: u64,
) -> NaturalPolynomial {
    if assert_mul_mod_polynomial_arguments(a, a, f, pow, true) || f.len() == 1 {
        return NaturalPolynomial::ZERO;
    }
    match exp {
        0 => return NaturalPolynomial::one(),
        1 => return a.clone(),
        _ => {}
    }
    if exp > 2 && is_x(a) {
        return power_of_x_precomputed(exp, f, pow, &precompute_helper(f, pow));
    }
    let reduce = |p: NaturalPolynomial| {
        if p.len() < f.len() {
            p
        } else {
            (&p).mod_power_of_2_rem(f, pow)
        }
    };
    let mut r = a.clone();
    for i in (0..exp.significant_bits() - 1).rev() {
        r = reduce((&r).mod_power_of_2_square(pow));
        if exp.get_bit(i) {
            r = reduce((&r).mod_power_of_2_mul(a, pow));
        }
    }
    r
}

impl ModPowerOf2PowModPolynomial<Self> for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the base by value and `f` by value. The base must already be reduced modulo
    /// $f$, with a degree less than that of $f$, and its coefficients and those of $f$ must be
    /// reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModPowerOf2DivRemPrecomputed`](malachite_base::polynomial::ModPowerOf2DivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, k, e) = O(n(k + \log n) \log (nk) \log\log (nk) \log e)$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $k$ is `pow`, and $e$ is
    /// `exp`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `f` is greater than or equal to $2^k$, or if `f` is 0
    /// (unless $k$ is 0) or its leading coefficient is even, or if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2PowModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_pow_mod_polynomial(10, f, 3).to_string(),
    ///     "5*x^2+x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that the base must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_pow_mod_polynomial(self, exp: u64, f: Self, pow: u64) -> Self {
        mod_power_of_2_pow_mod_polynomial_helper(&self, exp, &f, pow)
    }
}

impl ModPowerOf2PowModPolynomial<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the base by value and `f` by reference. The base must already be reduced
    /// modulo $f$, with a degree less than that of $f$, and its coefficients and those of $f$ must
    /// be reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModPowerOf2DivRemPrecomputed`](malachite_base::polynomial::ModPowerOf2DivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, k, e) = O(n(k + \log n) \log (nk) \log\log (nk) \log e)$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $k$ is `pow`, and $e$ is
    /// `exp`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `f` is greater than or equal to $2^k$, or if `f` is 0
    /// (unless $k$ is 0) or its leading coefficient is even, or if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2PowModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_pow_mod_polynomial(10, &f, 3).to_string(),
    ///     "5*x^2+x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that the base must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_pow_mod_polynomial(self, exp: u64, f: &Self, pow: u64) -> Self {
        mod_power_of_2_pow_mod_polynomial_helper(&self, exp, f, pow)
    }
}

impl ModPowerOf2PowModPolynomial<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the base by reference and `f` by value. The base must already be reduced
    /// modulo $f$, with a degree less than that of $f$, and its coefficients and those of $f$ must
    /// be reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModPowerOf2DivRemPrecomputed`](malachite_base::polynomial::ModPowerOf2DivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, k, e) = O(n(k + \log n) \log (nk) \log\log (nk) \log e)$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $k$ is `pow`, and $e$ is
    /// `exp`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `f` is greater than or equal to $2^k$, or if `f` is 0
    /// (unless $k$ is 0) or its leading coefficient is even, or if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2PowModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_pow_mod_polynomial(10, f, 3).to_string(),
    ///     "5*x^2+x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that the base must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_pow_mod_polynomial(
        self,
        exp: u64,
        f: NaturalPolynomial,
        pow: u64,
    ) -> NaturalPolynomial {
        mod_power_of_2_pow_mod_polynomial_helper(self, exp, &f, pow)
    }
}

impl ModPowerOf2PowModPolynomial<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, taking the base by reference and `f` by reference. The base must already be reduced
    /// modulo $f$, with a degree less than that of $f$, and its coefficients and those of $f$ must
    /// be reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModPowerOf2DivRemPrecomputed`](malachite_base::polynomial::ModPowerOf2DivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, k, e) = O(n(k + \log n) \log (nk) \log\log (nk) \log e)$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $k$ is `pow`, and $e$ is
    /// `exp`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `f` is greater than or equal to $2^k$, or if `f` is 0
    /// (unless $k$ is 0) or its leading coefficient is even, or if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2PowModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_pow_mod_polynomial(10, &f, 3)
    ///         .to_string(),
    ///     "5*x^2+x+7"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that the base must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_pow_mod_polynomial(
        self,
        exp: u64,
        f: &NaturalPolynomial,
        pow: u64,
    ) -> NaturalPolynomial {
        mod_power_of_2_pow_mod_polynomial_helper(self, exp, f, pow)
    }
}

impl ModPowerOf2PowModPolynomialAssign<Self> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, in place, taking `f` by value. The base must already be reduced modulo $f$, with a
    /// degree less than that of $f$, and its coefficients and those of $f$ must be reduced modulo
    /// $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModPowerOf2DivRemPrecomputed`](malachite_base::polynomial::ModPowerOf2DivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, k, e) = O(n(k + \log n) \log (nk) \log\log (nk) \log e)$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $k$ is `pow`, and $e$ is
    /// `exp`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `f` is greater than or equal to $2^k$, or if `f` is 0
    /// (unless $k$ is 0) or its leading coefficient is even, or if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2PowModPolynomialAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// a.mod_power_of_2_pow_mod_polynomial_assign(10, f, 3);
    /// assert_eq!(a.to_string(), "5*x^2+x+7");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that the base must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_pow_mod_polynomial_assign(&mut self, exp: u64, f: Self, pow: u64) {
        *self = mod_power_of_2_pow_mod_polynomial_helper(self, exp, &f, pow);
    }
}

impl ModPowerOf2PowModPolynomialAssign<&Self> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, in place, taking `f` by reference. The base must already be reduced modulo $f$, with
    /// a degree less than that of $f$, and its coefficients and those of $f$ must be reduced modulo
    /// $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModPowerOf2DivRemPrecomputed`](malachite_base::polynomial::ModPowerOf2DivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, k, e) = O(n(k + \log n) \log (nk) \log\log (nk) \log e)$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $k$ is `pow`, and $e$ is
    /// `exp`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `f` is greater than or equal to $2^k$, or if `f` is 0
    /// (unless $k$ is 0) or its leading coefficient is even, or if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2PowModPolynomialAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// a.mod_power_of_2_pow_mod_polynomial_assign(10, &f, 3);
    /// assert_eq!(a.to_string(), "5*x^2+x+7");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that the base must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_pow_mod_polynomial_assign(&mut self, exp: u64, f: &Self, pow: u64) {
        *self = mod_power_of_2_pow_mod_polynomial_helper(self, exp, f, pow);
    }
}
