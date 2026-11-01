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

use crate::num::arithmetic::traits::{ModMul, ModSquare, PowerOf2};
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::num::logic::traits::{BitAccess, SignificantBits};
use crate::polynomial::{
    ModPolynomialData, ModPowModPolynomial, ModPowModPolynomialAssign, ModRem, ModRemPrecomputed,
    MulPowerOfX, Polynomial,
};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_div_rem_precomputed::precompute_helper;
use crate::unsigned_polynomial::arithmetic::mod_mul_mod_polynomial::*;

// The power $x^e$ modulo a polynomial with `f_len` coefficients, where $e$ is `exp`, by a sliding
// window over the bits of $e$: `square` squares and `shift` multiplies by $x^w$, each reducing
// modulo the polynomial, and `one` is the polynomial 1. `exp` must be at least 3 and `f_len` at
// least 3, so that the window, at most half of `f_len - 1`, keeps every shifted polynomial short
// enough for one reduction.
//
// This is equivalent to `_nmod_poly_powmod_x_fmpz_preinv` from `nmod_poly/powmod_x_preinv.c`, FLINT
// 3.6.0, with `square` and `shift` in place of the products and divisions.
#[doc(hidden)]
pub fn pow_of_x_sliding_window<P>(
    exp: u64,
    f_len: u64,
    one: P,
    mut square: impl FnMut(P) -> P,
    mut shift: impl FnMut(P, u64) -> P,
) -> P {
    let mut res = one;
    let mut l = i64::wrapping_from((f_len - 1).significant_bits()) - 2;
    let mut window = u64::power_of_2(u64::wrapping_from(l));
    let mut c = l;
    let mut i = i64::wrapping_from(exp.significant_bits()) - 2;
    if i <= l {
        window = u64::power_of_2(u64::wrapping_from(i));
        c = i;
        l = i;
    }
    if c == 0 {
        res = shift(res, window);
        c = l + 1;
        window = 0;
    }
    while i >= 0 {
        res = square(res);
        c -= 1;
        if exp.get_bit(u64::wrapping_from(i)) {
            if window == 0 && i < l {
                c = i;
            }
            if c >= 0 {
                window |= u64::power_of_2(u64::wrapping_from(c));
            }
        } else if window == 0 {
            c = l + 1;
        }
        if c == 0 {
            res = shift(res, window);
            c = l + 1;
            window = 0;
        }
        i -= 1;
    }
    res
}

// Whether `a` is the polynomial $x$.
pub(crate) fn is_x<T: PrimitiveUnsigned>(a: &UnsignedPolynomial<T>) -> bool {
    a.coefficients == [T::ZERO, T::ONE]
}

// The power $x^e$ modulo `f`, with coefficients modulo $m$, where $e$ is `exp`, which is at least
// 3, and `f` has at least 3 coefficients, with `data` precomputed from `f` and $m$.
pub(crate) fn power_of_x_precomputed<T: PrimitiveUnsigned>(
    exp: u64,
    f: &UnsignedPolynomial<T>,
    m: T,
    data: &ModPolynomialData<T>,
) -> UnsignedPolynomial<T> {
    let reduce = |p: UnsignedPolynomial<T>| {
        if p.len() < f.len() {
            p
        } else {
            (&p).mod_rem_precomputed(f, m, data)
        }
    };
    pow_of_x_sliding_window(
        exp,
        f.len(),
        UnsignedPolynomial::one(),
        |r| reduce((&r).mod_square(m)),
        |r, w| reduce(r.mul_power_of_x(w)),
    )
}

// The power of `a` with exponent `exp` modulo `f`, with coefficients modulo $m$.
//
// This is equivalent to `nmod_poly_powmod_ui_binexp`, or, for a base of $x$,
// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
// reduced modulo $f$.
pub(crate) fn mod_pow_mod_polynomial_helper<T: PrimitiveUnsigned>(
    a: &UnsignedPolynomial<T>,
    exp: u64,
    f: &UnsignedPolynomial<T>,
    m: T,
) -> UnsignedPolynomial<T> {
    if assert_mul_mod_polynomial_arguments(a, a, f, m, true) || f.len() == 1 {
        return UnsignedPolynomial::ZERO;
    }
    match exp {
        0 => return UnsignedPolynomial::one(),
        1 => return a.clone(),
        _ => {}
    }
    if exp > 2 && is_x(a) {
        return power_of_x_precomputed(exp, f, m, &precompute_helper(f, m));
    }
    let reduce = |p: UnsignedPolynomial<T>| {
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

impl<T: PrimitiveUnsigned> ModPowModPolynomial<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Raises a [`UnsignedPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the base by value and `f` by value. The base must already be reduced modulo $f$,
    /// with a degree less than that of $f$, and its coefficients and those of $f$ must be reduced
    /// modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
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
    /// [`ModDivRemPrecomputed`](crate::polynomial::ModDivRemPrecomputed).
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
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(a.mod_pow_mod_polynomial(10, f, 7).to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
    /// reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial(self, exp: u64, f: Self, m: T) -> Self {
        mod_pow_mod_polynomial_helper(&self, exp, &f, m)
    }
}

impl<T: PrimitiveUnsigned> ModPowModPolynomial<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Raises a [`UnsignedPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the base by value and `f` by reference. The base must already be reduced modulo
    /// $f$, with a degree less than that of $f$, and its coefficients and those of $f$ must be
    /// reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
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
    /// [`ModDivRemPrecomputed`](crate::polynomial::ModDivRemPrecomputed).
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
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!(a.mod_pow_mod_polynomial(10, &f, 7).to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
    /// reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial(self, exp: u64, f: &Self, m: T) -> Self {
        mod_pow_mod_polynomial_helper(&self, exp, f, m)
    }
}

impl<T: PrimitiveUnsigned> ModPowModPolynomial<UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Raises a [`UnsignedPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the base by reference and `f` by value. The base must already be reduced modulo
    /// $f$, with a degree less than that of $f$, and its coefficients and those of $f$ must be
    /// reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
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
    /// [`ModDivRemPrecomputed`](crate::polynomial::ModDivRemPrecomputed).
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
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!((&a).mod_pow_mod_polynomial(10, f, 7).to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
    /// reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial(
        self,
        exp: u64,
        f: UnsignedPolynomial<T>,
        m: T,
    ) -> UnsignedPolynomial<T> {
        mod_pow_mod_polynomial_helper(self, exp, &f, m)
    }
}

impl<T: PrimitiveUnsigned> ModPowModPolynomial<&UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Raises a [`UnsignedPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, taking the base by reference and `f` by reference. The base must already be reduced
    /// modulo $f$, with a degree less than that of $f$, and its coefficients and those of $f$ must
    /// be reduced modulo $m$; the leading coefficient of $f$ must be invertible modulo $m$.
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
    /// [`ModDivRemPrecomputed`](crate::polynomial::ModDivRemPrecomputed).
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
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// assert_eq!((&a).mod_pow_mod_polynomial(10, &f, 7).to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
    /// reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial(
        self,
        exp: u64,
        f: &UnsignedPolynomial<T>,
        m: T,
    ) -> UnsignedPolynomial<T> {
        mod_pow_mod_polynomial_helper(self, exp, f, m)
    }
}

impl<T: PrimitiveUnsigned> ModPowModPolynomialAssign<Self, T> for UnsignedPolynomial<T> {
    /// Raises a [`UnsignedPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, in place, taking `f` by value. The base must already be reduced modulo $f$, with a
    /// degree less than that of $f$, and its coefficients and those of $f$ must be reduced modulo
    /// $m$; the leading coefficient of $f$ must be invertible modulo $m$.
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
    /// [`ModDivRemPrecomputed`](crate::polynomial::ModDivRemPrecomputed).
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
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// a.mod_pow_mod_polynomial_assign(10, f, 7);
    /// assert_eq!(a.to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
    /// reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial_assign(&mut self, exp: u64, f: Self, m: T) {
        *self = mod_pow_mod_polynomial_helper(self, exp, &f, m);
    }
}

impl<T: PrimitiveUnsigned> ModPowModPolynomialAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Raises a [`UnsignedPolynomial`] to a power modulo a polynomial $f$, with coefficients modulo
    /// $m$, in place, taking `f` by reference. The base must already be reduced modulo $f$, with a
    /// degree less than that of $f$, and its coefficients and those of $f$ must be reduced modulo
    /// $m$; the leading coefficient of $f$ must be invertible modulo $m$.
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
    /// [`ModDivRemPrecomputed`](crate::polynomial::ModDivRemPrecomputed).
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
    /// or if `f` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or
    /// if `self` is not shorter than `f`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowModPolynomialAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("x^3+2*x+1").unwrap();
    ///
    /// let mut a = UnsignedPolynomial::<u8>::from_str("3*x^2+x+5").unwrap();
    /// a.mod_pow_mod_polynomial_assign(10, &f, 7);
    /// assert_eq!(a.to_string(), "2*x+2");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_powmod_ui_binexp`, or, for a base of $x$,
    /// `nmod_poly_powmod_x_ui_preinv`, from `nmod_poly`, FLINT 3.6.0, except that the base must be
    /// reduced modulo $f$.
    #[inline]
    fn mod_pow_mod_polynomial_assign(&mut self, exp: u64, f: &Self, m: T) {
        *self = mod_pow_mod_polynomial_helper(self, exp, f, m);
    }
}
