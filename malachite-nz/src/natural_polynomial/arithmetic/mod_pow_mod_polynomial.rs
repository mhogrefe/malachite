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
use crate::natural_polynomial::arithmetic::mod_div_rem_precomputed::precompute_helper;
use crate::natural_polynomial::arithmetic::mod_mul_mod_polynomial::*;
use malachite_base::num::arithmetic::traits::{ModMul, ModSquare};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::polynomial::{
    ModPolynomialData, ModPowModPolynomial, ModPowModPolynomialAssign, ModRem, ModRemPrecomputed,
    MulPowerOfX, Polynomial,
};
use malachite_base::unsigned_polynomial::arithmetic::mod_pow_mod_polynomial::*;

// Whether `a` is the polynomial $x$.
pub(crate) fn is_x(a: &NaturalPolynomial) -> bool {
    a.coefficients == [Natural::ZERO, Natural::ONE]
}

// The power $x^e$ modulo `f`, with coefficients modulo $m$, where $e$ is `exp`, which is at least
// 3, and `f` has at least 3 coefficients, with `data` precomputed from `f` and $m$.
pub(crate) fn power_of_x_precomputed(
    exp: u64,
    f: &NaturalPolynomial,
    m: &Natural,
    data: &ModPolynomialData<Natural>,
) -> NaturalPolynomial {
    let reduce = |p: NaturalPolynomial| {
        if p.len() < f.len() {
            p
        } else {
            (&p).mod_rem_precomputed(f, m, data)
        }
    };
    pow_of_x_sliding_window(
        exp,
        f.len(),
        NaturalPolynomial::one(),
        |r| reduce((&r).mod_square(m)),
        |r, w| reduce(r.mul_power_of_x(w)),
    )
}

// The power of `a` with exponent `exp` modulo `f`, with coefficients modulo $m$.
//
// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, except that the base
// must be reduced modulo $f$.
pub(crate) fn mod_pow_mod_polynomial_helper(
    a: &NaturalPolynomial,
    exp: u64,
    f: &NaturalPolynomial,
    m: &Natural,
) -> NaturalPolynomial {
    if assert_mul_mod_polynomial_arguments(a, a, f, m, true) || f.len() == 1 {
        return NaturalPolynomial::ZERO;
    }
    match exp {
        0 => return NaturalPolynomial::one(),
        1 => return a.clone(),
        _ => {}
    }
    if exp > 2 && is_x(a) {
        return power_of_x_precomputed(exp, f, m, &precompute_helper(f, m));
    }
    let reduce = |p: NaturalPolynomial| {
        if p.len() < f.len() {
            p
        } else {
            (&p).mod_rem(f, m)
        }
    };
    let mut r = a.clone();
    for i in (0..exp.significant_bits() - 1).rev() {
        r = reduce((&r).mod_square(m));
        if exp.get_bit(i) {
            r = reduce((&r).mod_mul(a, m));
        }
    }
    r
}

impl ModPowModPolynomial<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the base by value, `f` by value, and the modulus by value. The base must already
    /// be reduced modulo $f$, with a degree less than that of $f$, and its coefficients and those
    /// of $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModDivRemPrecomputed`](malachite_base::polynomial::ModDivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(a.mod_pow_mod_polynomial(10, f, m).to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, except that the
    /// base must be reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial(self, exp: u64, f: Self, m: Natural) -> Self {
        mod_pow_mod_polynomial_helper(&self, exp, &f, &m)
    }
}

impl ModPowModPolynomial<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the base by value, `f` by value, and the modulus by reference. The base must
    /// already be reduced modulo $f$, with a degree less than that of $f$, and its coefficients and
    /// those of $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModDivRemPrecomputed`](malachite_base::polynomial::ModDivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(a.mod_pow_mod_polynomial(10, f, &m).to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, except that the
    /// base must be reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial(self, exp: u64, f: Self, m: &Natural) -> Self {
        mod_pow_mod_polynomial_helper(&self, exp, &f, m)
    }
}

impl ModPowModPolynomial<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the base by value, `f` by reference, and the modulus by value. The base must
    /// already be reduced modulo $f$, with a degree less than that of $f$, and its coefficients and
    /// those of $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModDivRemPrecomputed`](malachite_base::polynomial::ModDivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(a.mod_pow_mod_polynomial(10, &f, m).to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, except that the
    /// base must be reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial(self, exp: u64, f: &Self, m: Natural) -> Self {
        mod_pow_mod_polynomial_helper(&self, exp, f, &m)
    }
}

impl ModPowModPolynomial<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the base by value, `f` by reference, and the modulus by reference. The base must
    /// already be reduced modulo $f$, with a degree less than that of $f$, and its coefficients and
    /// those of $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModDivRemPrecomputed`](malachite_base::polynomial::ModDivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(a.mod_pow_mod_polynomial(10, &f, &m).to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, except that the
    /// base must be reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial(self, exp: u64, f: &Self, m: &Natural) -> Self {
        mod_pow_mod_polynomial_helper(&self, exp, f, m)
    }
}

impl ModPowModPolynomial<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the base by reference, `f` by value, and the modulus by value. The base must
    /// already be reduced modulo $f$, with a degree less than that of $f$, and its coefficients and
    /// those of $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModDivRemPrecomputed`](malachite_base::polynomial::ModDivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!((&a).mod_pow_mod_polynomial(10, f, m).to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, except that the
    /// base must be reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial(
        self,
        exp: u64,
        f: NaturalPolynomial,
        m: Natural,
    ) -> NaturalPolynomial {
        mod_pow_mod_polynomial_helper(self, exp, &f, &m)
    }
}

impl ModPowModPolynomial<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the base by reference, `f` by value, and the modulus by reference. The base must
    /// already be reduced modulo $f$, with a degree less than that of $f$, and its coefficients and
    /// those of $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModDivRemPrecomputed`](malachite_base::polynomial::ModDivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!((&a).mod_pow_mod_polynomial(10, f, &m).to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, except that the
    /// base must be reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial(
        self,
        exp: u64,
        f: NaturalPolynomial,
        m: &Natural,
    ) -> NaturalPolynomial {
        mod_pow_mod_polynomial_helper(self, exp, &f, m)
    }
}

impl ModPowModPolynomial<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the base by reference, `f` by reference, and the modulus by value. The base must
    /// already be reduced modulo $f$, with a degree less than that of $f$, and its coefficients and
    /// those of $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModDivRemPrecomputed`](malachite_base::polynomial::ModDivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!((&a).mod_pow_mod_polynomial(10, &f, m).to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, except that the
    /// base must be reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial(
        self,
        exp: u64,
        f: &NaturalPolynomial,
        m: Natural,
    ) -> NaturalPolynomial {
        mod_pow_mod_polynomial_helper(self, exp, f, &m)
    }
}

impl ModPowModPolynomial<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the base by reference, `f` by reference, and the modulus by reference. The base
    /// must already be reduced modulo $f$, with a degree less than that of $f$, and its
    /// coefficients and those of $f$ must be reduced modulo $m$; the leading coefficient of $f$
    /// must be invertible modulo $m$.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModDivRemPrecomputed`](malachite_base::polynomial::ModDivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!((&a).mod_pow_mod_polynomial(10, &f, &m).to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, except that the
    /// base must be reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial(
        self,
        exp: u64,
        f: &NaturalPolynomial,
        m: &Natural,
    ) -> NaturalPolynomial {
        mod_pow_mod_polynomial_helper(self, exp, f, m)
    }
}

impl ModPowModPolynomialAssign<Self, Natural> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, in place, taking `f` by value and the modulus by value. The base must already be
    /// reduced modulo $f$, with a degree less than that of $f$, and its coefficients and those of
    /// $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModDivRemPrecomputed`](malachite_base::polynomial::ModDivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomialAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// a.mod_pow_mod_polynomial_assign(10, f, m);
    /// assert_eq!(a.to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, except that the
    /// base must be reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial_assign(&mut self, exp: u64, f: Self, m: Natural) {
        *self = mod_pow_mod_polynomial_helper(self, exp, &f, &m);
    }
}

impl ModPowModPolynomialAssign<Self, &Natural> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, in place, taking `f` by value and the modulus by reference. The base must already be
    /// reduced modulo $f$, with a degree less than that of $f$, and its coefficients and those of
    /// $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModDivRemPrecomputed`](malachite_base::polynomial::ModDivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomialAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// a.mod_pow_mod_polynomial_assign(10, f, &m);
    /// assert_eq!(a.to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, except that the
    /// base must be reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial_assign(&mut self, exp: u64, f: Self, m: &Natural) {
        *self = mod_pow_mod_polynomial_helper(self, exp, &f, m);
    }
}

impl ModPowModPolynomialAssign<&Self, Natural> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, in place, taking `f` by reference and the modulus by value. The base must already be
    /// reduced modulo $f$, with a degree less than that of $f$, and its coefficients and those of
    /// $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModDivRemPrecomputed`](malachite_base::polynomial::ModDivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomialAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// a.mod_pow_mod_polynomial_assign(10, &f, m);
    /// assert_eq!(a.to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, except that the
    /// base must be reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial_assign(&mut self, exp: u64, f: &Self, m: Natural) {
        *self = mod_pow_mod_polynomial_helper(self, exp, f, &m);
    }
}

impl ModPowModPolynomialAssign<&Self, &Natural> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, in place, taking `f` by reference and the modulus by reference. The base must already
    /// be reduced modulo $f$, with a degree less than that of $f$, and its coefficients and those
    /// of $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The power is found by binary exponentiation, each product reduced modulo $f$ by Euclidean
    /// division; a base of $x$ is instead raised by a sliding window over the exponent, multiplying
    /// by powers of $x$ by shifting, with data precomputed from $f$ as by
    /// [`ModDivRemPrecomputed`](malachite_base::polynomial::ModDivRemPrecomputed).
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n(m + \log n) \log (nm) \log\log (nm) \log e)$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, $m$ is
    /// `m.significant_bits()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomialAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// a.mod_pow_mod_polynomial_assign(10, &f, &m);
    /// assert_eq!(a.to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, except that the
    /// base must be reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial_assign(&mut self, exp: u64, f: &Self, m: &Natural) {
        *self = mod_pow_mod_polynomial_helper(self, exp, f, m);
    }
}
