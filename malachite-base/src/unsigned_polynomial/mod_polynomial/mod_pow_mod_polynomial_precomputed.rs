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

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::{BitAccess, SignificantBits};
use crate::num::modular::traits::{ModMul, ModSquare};
use crate::polynomial::Polynomial;
use crate::polynomial::mod_polynomial::traits::{
    ModPowModPolynomialPrecomputed, ModPowModPolynomialPrecomputedAssign,
};
use crate::polynomial::modular::ModPolynomialData;
use crate::polynomial::modular::traits::ModRemPrecomputed;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::mod_polynomial::mod_mul_mod_polynomial::*;
use crate::unsigned_polynomial::mod_polynomial::mod_pow_mod_polynomial::{
    is_x, power_of_x_precomputed,
};

// The power of `a` with exponent `exp` modulo `f`, with coefficients modulo $m$, with `data`
// precomputed from `f` and $m$.
//
// This is equivalent to `nmod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
// reduced modulo $f$.
pub(crate) fn mod_pow_mod_polynomial_precomputed_helper<T: PrimitiveUnsigned>(
    a: &UnsignedPolynomial<T>,
    exp: u64,
    f: &UnsignedPolynomial<T>,
    m: T,
    data: &ModPolynomialData<T>,
) -> UnsignedPolynomial<T> {
    if assert_mul_mod_polynomial_arguments(a, a, f, m, false) || f.len() == 1 {
        return UnsignedPolynomial::ZERO;
    }
    match exp {
        0 => return UnsignedPolynomial::one(),
        1 => return a.clone(),
        _ => {}
    }
    if exp > 2 && is_x(a) {
        return power_of_x_precomputed(exp, f, m, data);
    }
    let reduce = |p: UnsignedPolynomial<T>| {
        if p.len() < f.len() {
            p
        } else {
            (&p).mod_rem_precomputed(f, m, data)
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

impl<T: PrimitiveUnsigned> ModPowModPolynomialPrecomputed<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Raises a [`UnsignedPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, using data precomputed from $f$ and $m$, taking the base by value and `f` by value. The
    /// base must already be reduced modulo $f$, with a degree less than that of $f$, and its
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
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](crate::polynomial::modular::traits::ModDivRemPrecomputed),
    /// as for division. The power is found by binary exponentiation, each product reduced modulo
    /// $f$ with the data; a base of $x$ is instead raised by a sliding window over the exponent,
    /// multiplying by powers of $x$ by shifting.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n \log n \log e)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1), or if `self` is not shorter than `f`. If `data` was not
    /// precomputed from `f` and `m`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(
    ///     a.mod_pow_mod_polynomial_precomputed(10, f.clone(), 7, &data)
    ///         .to_string(),
    ///     "2*x+2"
    /// );
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    /// assert_eq!(
    ///     a.mod_pow_mod_polynomial_precomputed(100, f, 7, &data)
    ///         .to_string(),
    ///     "x^2+3*x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
    /// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
    /// reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial_precomputed(
        self,
        exp: u64,
        f: Self,
        m: T,
        data: &Self::Data,
    ) -> Self {
        mod_pow_mod_polynomial_precomputed_helper(&self, exp, &f, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowModPolynomialPrecomputed<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Raises a [`UnsignedPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, using data precomputed from $f$ and $m$, taking the base by value and `f` by reference.
    /// The base must already be reduced modulo $f$, with a degree less than that of $f$, and its
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
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](crate::polynomial::modular::traits::ModDivRemPrecomputed),
    /// as for division. The power is found by binary exponentiation, each product reduced modulo
    /// $f$ with the data; a base of $x$ is instead raised by a sliding window over the exponent,
    /// multiplying by powers of $x$ by shifting.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n \log n \log e)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1), or if `self` is not shorter than `f`. If `data` was not
    /// precomputed from `f` and `m`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(
    ///     a.mod_pow_mod_polynomial_precomputed(10, &f, 7, &data)
    ///         .to_string(),
    ///     "2*x+2"
    /// );
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    /// assert_eq!(
    ///     a.mod_pow_mod_polynomial_precomputed(100, &f, 7, &data)
    ///         .to_string(),
    ///     "x^2+3*x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
    /// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
    /// reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial_precomputed(
        self,
        exp: u64,
        f: &Self,
        m: T,
        data: &Self::Data,
    ) -> Self {
        mod_pow_mod_polynomial_precomputed_helper(&self, exp, f, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowModPolynomialPrecomputed<UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Raises a [`UnsignedPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, using data precomputed from $f$ and $m$, taking the base by reference and `f` by value.
    /// The base must already be reduced modulo $f$, with a degree less than that of $f$, and its
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
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](crate::polynomial::modular::traits::ModDivRemPrecomputed),
    /// as for division. The power is found by binary exponentiation, each product reduced modulo
    /// $f$ with the data; a base of $x$ is instead raised by a sliding window over the exponent,
    /// multiplying by powers of $x$ by shifting.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n \log n \log e)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1), or if `self` is not shorter than `f`. If `data` was not
    /// precomputed from `f` and `m`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow_mod_polynomial_precomputed(10, f.clone(), 7, &data)
    ///         .to_string(),
    ///     "2*x+2"
    /// );
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow_mod_polynomial_precomputed(100, f, 7, &data)
    ///         .to_string(),
    ///     "x^2+3*x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
    /// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
    /// reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial_precomputed(
        self,
        exp: u64,
        f: UnsignedPolynomial<T>,
        m: T,
        data: &Self::Data,
    ) -> UnsignedPolynomial<T> {
        mod_pow_mod_polynomial_precomputed_helper(self, exp, &f, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowModPolynomialPrecomputed<&UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Raises a [`UnsignedPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, using data precomputed from $f$ and $m$, taking the base by reference and `f` by
    /// reference. The base must already be reduced modulo $f$, with a degree less than that of $f$,
    /// and its coefficients and those of $f$ must be reduced modulo $m$; the leading coefficient of
    /// $f$ must be invertible modulo $m$.
    ///
    /// $$
    /// (a, n) \mapsto r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](crate::polynomial::modular::traits::ModDivRemPrecomputed),
    /// as for division. The power is found by binary exponentiation, each product reduced modulo
    /// $f$ with the data; a base of $x$ is instead raised by a sliding window over the exponent,
    /// multiplying by powers of $x$ by shifting.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n \log n \log e)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1), or if `self` is not shorter than `f`. If `data` was not
    /// precomputed from `f` and `m`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow_mod_polynomial_precomputed(10, &f, 7, &data)
    ///         .to_string(),
    ///     "2*x+2"
    /// );
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    /// assert_eq!(
    ///     (&a).mod_pow_mod_polynomial_precomputed(100, &f, 7, &data)
    ///         .to_string(),
    ///     "x^2+3*x+5"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
    /// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
    /// reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial_precomputed(
        self,
        exp: u64,
        f: &UnsignedPolynomial<T>,
        m: T,
        data: &Self::Data,
    ) -> UnsignedPolynomial<T> {
        mod_pow_mod_polynomial_precomputed_helper(self, exp, f, m, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowModPolynomialPrecomputedAssign<Self, T> for UnsignedPolynomial<T> {
    /// Raises a [`UnsignedPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, in place, using data precomputed from $f$ and $m$, taking `f` by value. The base must
    /// already be reduced modulo $f$, with a degree less than that of $f$, and its coefficients and
    /// those of $f$ must be reduced modulo $m$; the leading coefficient of $f$ must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](crate::polynomial::modular::traits::ModDivRemPrecomputed),
    /// as for division. The power is found by binary exponentiation, each product reduced modulo
    /// $f$ with the data; a base of $x$ is instead raised by a sliding window over the exponent,
    /// multiplying by powers of $x$ by shifting.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n \log n \log e)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1), or if `self` is not shorter than `f`. If `data` was not
    /// precomputed from `f` and `m`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// a.mod_pow_mod_polynomial_precomputed_assign(10, f.clone(), 7, &data);
    /// assert_eq!(a.to_string(), "2*x+2");
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    /// a.mod_pow_mod_polynomial_precomputed_assign(100, f, 7, &data);
    /// assert_eq!(a.to_string(), "x^2+3*x+5");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
    /// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
    /// reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial_precomputed_assign(
        &mut self,
        exp: u64,
        f: Self,
        m: T,
        data: &Self::Data,
    ) {
        *self = mod_pow_mod_polynomial_precomputed_helper(self, exp, &f, m, data);
    }
}

impl<T: PrimitiveUnsigned> ModPowModPolynomialPrecomputedAssign<&Self, T>
    for UnsignedPolynomial<T>
{
    /// Raises a [`UnsignedPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, in place, using data precomputed from $f$ and $m$, taking `f` by reference. The base
    /// must already be reduced modulo $f$, with a degree less than that of $f$, and its
    /// coefficients and those of $f$ must be reduced modulo $m$; the leading coefficient of $f$
    /// must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets r, \quad a^n \equiv fq + r \pmod{m}, \quad \deg r < \deg f.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result. When $f$ is a constant, the
    /// result is 0; otherwise, $a^0 = 1$.
    ///
    /// The data should be obtained with
    /// [`precompute_mod_div_rem_data`](crate::polynomial::modular::traits::ModDivRemPrecomputed),
    /// as for division. The power is found by binary exponentiation, each product reduced modulo
    /// $f$ with the data; a base of $x$ is instead raised by a sliding window over the exponent,
    /// multiplying by powers of $x$ by shifting.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n \log n \log e)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `f.len()`, and $e$ is `exp`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `f` is greater than or equal to `m`,
    /// or if `f` is 0 (unless $m$ is 1), or if `self` is not shorter than `f`. If `data` was not
    /// precomputed from `f` and `m`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::arithmetic::traits::*;
    /// use malachite_base::polynomial::calculus::traits::*;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::*;
    /// use malachite_base::polynomial::series::traits::*;
    /// use malachite_base::polynomial::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&f, &7);
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// a.mod_pow_mod_polynomial_precomputed_assign(10, &f, 7, &data);
    /// assert_eq!(a.to_string(), "2*x+2");
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("x").unwrap();
    /// a.mod_pow_mod_polynomial_precomputed_assign(100, &f, 7, &data);
    /// assert_eq!(a.to_string(), "x^2+3*x+5");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_powmod_ui_binexp_preinv`, or, for a base of $x$,
    /// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
    /// reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial_precomputed_assign(
        &mut self,
        exp: u64,
        f: &Self,
        m: T,
        data: &Self::Data,
    ) {
        *self = mod_pow_mod_polynomial_precomputed_helper(self, exp, f, m, data);
    }
}
