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
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul_mod_polynomial::*;
use crate::natural_polynomial::arithmetic::mod_power_of_2_pow_mod_polynomial::{
    is_x, power_of_x_precomputed,
};
use malachite_base::num::arithmetic::traits::{ModPowerOf2Mul, ModPowerOf2Square};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::{BitAccess, SignificantBits};
use malachite_base::polynomial::{
    ModPolynomialData, ModPowerOf2PowModPolynomialPrecomputed,
    ModPowerOf2PowModPolynomialPrecomputedAssign, ModPowerOf2RemPrecomputed, Polynomial,
};

// The power of `a` with exponent `exp` modulo `f`, with coefficients modulo $2^k$, with `data`
// precomputed from `f` and $k$.
//
// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus $2^k$,
// except that the base must be reduced modulo $f$.
pub(crate) fn mod_power_of_2_pow_mod_polynomial_precomputed_helper(
    a: &NaturalPolynomial,
    exp: u64,
    f: &NaturalPolynomial,
    pow: u64,
    data: &ModPolynomialData<Natural>,
) -> NaturalPolynomial {
    if assert_mul_mod_polynomial_arguments(a, a, f, pow, false) || f.len() == 1 {
        return NaturalPolynomial::ZERO;
    }
    match exp {
        0 => return NaturalPolynomial::one(),
        1 => return a.clone(),
        _ => {}
    }
    if exp > 2 && is_x(a) {
        return power_of_x_precomputed(exp, f, pow, data);
    }
    let reduce = |p: NaturalPolynomial| {
        if p.len() < f.len() {
            p
        } else {
            (&p).mod_power_of_2_rem_precomputed(f, pow, data)
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

impl ModPowerOf2PowModPolynomialPrecomputed<Self> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, using data precomputed from $f$ and $k$, taking the base by value and `f` by value.
    /// The base must already be reduced modulo $f$, with a degree less than that of $f$, and its
    /// coefficients and those of $f$ must be reduced modulo $2^k$; the leading coefficient of $f$
    /// must be odd.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::ModPowerOf2DivRemPrecomputed), as for division. The power is
    /// found by binary exponentiation, each product reduced modulo $f$ with the data; a base of $x$
    /// is instead raised by a sliding window over the exponent, multiplying by powers of $x$ by
    /// shifting.
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
    /// (unless $k$ is 0), or if `self` is not shorter than `f`. If `data` was not precomputed from
    /// `f` and `pow`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_pow_mod_polynomial_precomputed(10, f.clone(), 3, &data)
    ///         .to_string(),
    ///     "5*x^2+x+7"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("x").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_pow_mod_polynomial_precomputed(100, f, 3, &data)
    ///         .to_string(),
    ///     "6*x^2+7*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that the base must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_pow_mod_polynomial_precomputed(
        self,
        exp: u64,
        f: Self,
        pow: u64,
        data: &Self::Data,
    ) -> Self {
        mod_power_of_2_pow_mod_polynomial_precomputed_helper(&self, exp, &f, pow, data)
    }
}

impl ModPowerOf2PowModPolynomialPrecomputed<&Self> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, using data precomputed from $f$ and $k$, taking the base by value and `f` by
    /// reference. The base must already be reduced modulo $f$, with a degree less than that of $f$,
    /// and its coefficients and those of $f$ must be reduced modulo $2^k$; the leading coefficient
    /// of $f$ must be odd.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::ModPowerOf2DivRemPrecomputed), as for division. The power is
    /// found by binary exponentiation, each product reduced modulo $f$ with the data; a base of $x$
    /// is instead raised by a sliding window over the exponent, multiplying by powers of $x$ by
    /// shifting.
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
    /// (unless $k$ is 0), or if `self` is not shorter than `f`. If `data` was not precomputed from
    /// `f` and `pow`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_pow_mod_polynomial_precomputed(10, &f, 3, &data)
    ///         .to_string(),
    ///     "5*x^2+x+7"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("x").unwrap();
    /// assert_eq!(
    ///     a.mod_power_of_2_pow_mod_polynomial_precomputed(100, &f, 3, &data)
    ///         .to_string(),
    ///     "6*x^2+7*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that the base must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_pow_mod_polynomial_precomputed(
        self,
        exp: u64,
        f: &Self,
        pow: u64,
        data: &Self::Data,
    ) -> Self {
        mod_power_of_2_pow_mod_polynomial_precomputed_helper(&self, exp, f, pow, data)
    }
}

impl ModPowerOf2PowModPolynomialPrecomputed<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, using data precomputed from $f$ and $k$, taking the base by reference and `f` by
    /// value. The base must already be reduced modulo $f$, with a degree less than that of $f$, and
    /// its coefficients and those of $f$ must be reduced modulo $2^k$; the leading coefficient of
    /// $f$ must be odd.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::ModPowerOf2DivRemPrecomputed), as for division. The power is
    /// found by binary exponentiation, each product reduced modulo $f$ with the data; a base of $x$
    /// is instead raised by a sliding window over the exponent, multiplying by powers of $x$ by
    /// shifting.
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
    /// (unless $k$ is 0), or if `self` is not shorter than `f`. If `data` was not precomputed from
    /// `f` and `pow`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_pow_mod_polynomial_precomputed(10, f.clone(), 3, &data)
    ///         .to_string(),
    ///     "5*x^2+x+7"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("x").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_pow_mod_polynomial_precomputed(100, f, 3, &data)
    ///         .to_string(),
    ///     "6*x^2+7*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that the base must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_pow_mod_polynomial_precomputed(
        self,
        exp: u64,
        f: NaturalPolynomial,
        pow: u64,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_power_of_2_pow_mod_polynomial_precomputed_helper(self, exp, &f, pow, data)
    }
}

impl ModPowerOf2PowModPolynomialPrecomputed<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, using data precomputed from $f$ and $k$, taking the base by reference and `f` by
    /// reference. The base must already be reduced modulo $f$, with a degree less than that of $f$,
    /// and its coefficients and those of $f$ must be reduced modulo $2^k$; the leading coefficient
    /// of $f$ must be odd.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::ModPowerOf2DivRemPrecomputed), as for division. The power is
    /// found by binary exponentiation, each product reduced modulo $f$ with the data; a base of $x$
    /// is instead raised by a sliding window over the exponent, multiplying by powers of $x$ by
    /// shifting.
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
    /// (unless $k$ is 0), or if `self` is not shorter than `f`. If `data` was not precomputed from
    /// `f` and `pow`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_pow_mod_polynomial_precomputed(10, &f, 3, &data)
    ///         .to_string(),
    ///     "5*x^2+x+7"
    /// );
    ///
    /// let a = NaturalPolynomial::from_str("x").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_pow_mod_polynomial_precomputed(100, &f, 3, &data)
    ///         .to_string(),
    ///     "6*x^2+7*x"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that the base must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_pow_mod_polynomial_precomputed(
        self,
        exp: u64,
        f: &NaturalPolynomial,
        pow: u64,
        data: &Self::Data,
    ) -> NaturalPolynomial {
        mod_power_of_2_pow_mod_polynomial_precomputed_helper(self, exp, f, pow, data)
    }
}

impl ModPowerOf2PowModPolynomialPrecomputedAssign<Self> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, in place, using data precomputed from $f$ and $k$, taking `f` by value. The base must
    /// already be reduced modulo $f$, with a degree less than that of $f$, and its coefficients and
    /// those of $f$ must be reduced modulo $2^k$; the leading coefficient of $f$ must be odd.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::ModPowerOf2DivRemPrecomputed), as for division. The power is
    /// found by binary exponentiation, each product reduced modulo $f$ with the data; a base of $x$
    /// is instead raised by a sliding window over the exponent, multiplying by powers of $x$ by
    /// shifting.
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
    /// (unless $k$ is 0), or if `self` is not shorter than `f`. If `data` was not precomputed from
    /// `f` and `pow`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// a.mod_power_of_2_pow_mod_polynomial_precomputed_assign(10, f.clone(), 3, &data);
    /// assert_eq!(a.to_string(), "5*x^2+x+7");
    ///
    /// let mut a = NaturalPolynomial::from_str("x").unwrap();
    /// a.mod_power_of_2_pow_mod_polynomial_precomputed_assign(100, f, 3, &data);
    /// assert_eq!(a.to_string(), "6*x^2+7*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that the base must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_pow_mod_polynomial_precomputed_assign(
        &mut self,
        exp: u64,
        f: Self,
        pow: u64,
        data: &Self::Data,
    ) {
        *self = mod_power_of_2_pow_mod_polynomial_precomputed_helper(self, exp, &f, pow, data);
    }
}

impl ModPowerOf2PowModPolynomialPrecomputedAssign<&Self> for NaturalPolynomial {
    /// Raises a [`NaturalPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $2^k$, in place, using data precomputed from $f$ and $k$, taking `f` by reference. The base
    /// must already be reduced modulo $f$, with a degree less than that of $f$, and its
    /// coefficients and those of $f$ must be reduced modulo $2^k$; the leading coefficient of $f$
    /// must be odd.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \pmod{2^k}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The data should be obtained with [`precompute_mod_power_of_2_div_rem_data`](
    /// malachite_base::polynomial::ModPowerOf2DivRemPrecomputed), as for division. The power is
    /// found by binary exponentiation, each product reduced modulo $f$ with the data; a base of $x$
    /// is instead raised by a sliding window over the exponent, multiplying by powers of $x$ by
    /// shifting.
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
    /// (unless $k$ is 0), or if `self` is not shorter than `f`. If `data` was not precomputed from
    /// `f` and `pow`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let f = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&f, 3);
    ///
    /// let mut a = NaturalPolynomial::from_str("3*x^2+x+5").unwrap();
    /// a.mod_power_of_2_pow_mod_polynomial_precomputed_assign(10, &f, 3, &data);
    /// assert_eq!(a.to_string(), "5*x^2+x+7");
    ///
    /// let mut a = NaturalPolynomial::from_str("x").unwrap();
    /// a.mod_power_of_2_pow_mod_polynomial_precomputed_assign(100, &f, 3, &data);
    /// assert_eq!(a.to_string(), "6*x^2+7*x");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
    /// `fmpz_mod_poly_powmod_x_fmpz_preinv`, from `fmpz_mod_poly`, FLINT 3.6.0, with the modulus
    /// $2^k$, except that the base must be reduced modulo $f$.
    #[inline]
    fn mod_power_of_2_pow_mod_polynomial_precomputed_assign(
        &mut self,
        exp: u64,
        f: &Self,
        pow: u64,
        data: &Self::Data,
    ) {
        *self = mod_power_of_2_pow_mod_polynomial_precomputed_helper(self, exp, f, pow, data);
    }
}
