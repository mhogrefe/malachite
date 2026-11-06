// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
//      Copyright © 2011, 2012 Sebastian Pancratz
//
//      Copyright © 2013, 2014 Martin Lee
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::modular::traits::{ModPowerOf2IsReduced, ModPowerOf2Mul};
use crate::polynomial::Polynomial;
use crate::polynomial::mod_polynomial::traits::{
    ModPowerOf2ComposeModPolynomial, ModPowerOf2ComposeModPolynomialAssign,
};
use crate::polynomial::modular::ModPolynomialData;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;
use crate::unsigned_polynomial::modular::mod_power_of_2_div_rem_precomputed::{
    mod_power_of_2_div_rem_blocks, precompute_helper,
};
use crate::unsigned_polynomial::modular::mod_power_of_2_mul::{
    mask_coefficients, mod_power_of_2_mul_to_out,
};
use alloc::vec::Vec;
use core::mem::take;

// The ring of polynomials with coefficients modulo $2^k$, modulo a polynomial with coefficients
// `h`, with at least 3 of them, whose reversal has the inverse `inverse` as a power series.
pub(crate) struct ModPowerOf2Ring<'a, T: PrimitiveUnsigned> {
    pub(crate) pow: u64,
    pub(crate) h: &'a [T],
    pub(crate) inverse: &'a [T],
}

impl<T: PrimitiveUnsigned> ComposeModRing for ModPowerOf2Ring<'_, T> {
    type C = T;

    fn zero(&self) -> T {
        T::ZERO
    }

    fn one(&self) -> T {
        T::ONE
    }

    #[inline]
    fn add(&self, x: &T, y: &T) -> T {
        x.mod_power_of_2_add(*y, self.pow)
    }

    #[inline]
    fn mul(&self, x: &T, y: &T) -> T {
        x.mod_power_of_2_mul(*y, self.pow)
    }

    #[inline]
    fn mul_mod(&self, xs: &[T], ys: &[T]) -> Vec<T> {
        self.rem(&self.poly_mul(xs, ys))
    }

    fn poly_mul(&self, xs: &[T], ys: &[T]) -> Vec<T> {
        let mut out = vec![T::ZERO; xs.len() + ys.len() - 1];
        mod_power_of_2_mul_to_out(&mut out, xs, ys, self.pow);
        out
    }

    fn rem(&self, xs: &[T]) -> Vec<T> {
        let mut r = mod_power_of_2_div_rem_blocks(xs, self.h, self.inverse, self.pow).1;
        mask_coefficients(&mut r, self.pow);
        r.resize(self.h.len() - 1, T::ZERO);
        r
    }

    fn matrix_mul(&self, b: &[Vec<T>], a: &[Vec<T>]) -> Vec<Vec<T>> {
        let width = a[0].len();
        let b = UnsignedMatrix::from_owned_elements(
            u64::exact_from(b.len()),
            u64::exact_from(a.len()),
            b.concat(),
        );
        let a = UnsignedMatrix::from_owned_elements(
            u64::exact_from(a.len()),
            u64::exact_from(width),
            a.concat(),
        );
        b.mod_power_of_2_mul(a, self.pow)
            .into_elements()
            .chunks(width)
            .map(<[T]>::to_vec)
            .collect()
    }

    // Measured with 64-bit coefficients modulo $2^{64}$, 2026-11-01. Multiplication modulo $2^k$ is
    // cheaper than modulo $m$, which favors Horner's rule a little longer.
    fn brent_kung_cutoff(&self, n: usize) -> usize {
        match n {
            0..=5 => 13,
            6..=12 => 10,
            13..=16 => 9,
            17..=64 => 6,
            _ => 5,
        }
    }
}

// Checks that `g` and `h` are reduced modulo $2^k$, that $k$ is valid, that `h` is nonzero with an
// odd leading coefficient, unless $k$ is 0, and that `g` is shorter than `h`, and returns whether
// $k$ is 0.
pub(crate) fn assert_power_of_2_compose_mod_polynomial_inner_and_modulus_arguments<
    T: PrimitiveUnsigned,
>(
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    pow: u64,
) -> bool {
    assert!(pow <= T::WIDTH);
    assert!(
        g.mod_power_of_2_is_reduced(pow),
        "g must be reduced mod 2^pow, but {g} has a coefficient >= 2^{pow}"
    );
    assert!(
        h.mod_power_of_2_is_reduced(pow),
        "h must be reduced mod 2^pow, but {h} has a coefficient >= 2^{pow}"
    );
    if pow == 0 {
        return true;
    }
    assert_ne!(*h, UnsignedPolynomial::ZERO, "division by zero");
    assert!(
        h.coefficients.last().unwrap().odd(),
        "the leading coefficient of h must be odd"
    );
    assert!(
        g.len() < h.len(),
        "g must be reduced mod h, but {g} has a degree >= that of {h}"
    );
    false
}

// Checks that `f` is reduced, and the other arguments as
// `assert_power_of_2_compose_mod_polynomial_inner_and_modulus_arguments` does, and returns its
// result.
pub(crate) fn assert_power_of_2_compose_mod_polynomial_arguments<T: PrimitiveUnsigned>(
    f: &UnsignedPolynomial<T>,
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    pow: u64,
) -> bool {
    assert!(
        f.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {f} has a coefficient >= 2^{pow}"
    );
    assert_power_of_2_compose_mod_polynomial_inner_and_modulus_arguments(g, h, pow)
}

// The composition $f(g)$ modulo $h$, with coefficients modulo $2^k$, using `data` if it is given
// and computing it otherwise.
//
// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0, with
// the modulus $2^k$, or, with `data`, to `gr_poly_compose_mod_preinv` from
// `gr_poly/compose_mod_preinv.c`, except that `g` must be reduced modulo $h$ and that the inverse
// of the reversal of $h$ is used for every reduction.
pub(crate) fn mod_power_of_2_compose_mod_polynomial_helper<T: PrimitiveUnsigned>(
    f: &UnsignedPolynomial<T>,
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    pow: u64,
    data: Option<&ModPolynomialData<T>>,
) -> UnsignedPolynomial<T> {
    if assert_power_of_2_compose_mod_polynomial_arguments(f, g, h, pow) {
        return UnsignedPolynomial::ZERO;
    }
    let computed;
    let data = if let Some(data) = data {
        data
    } else {
        computed = precompute_helper(h, pow);
        &computed
    };
    UnsignedPolynomial::from_coefficients_asc(mod_power_of_2_compose_mod_polynomial_vec(
        &f.coefficients,
        &g.coefficients,
        &h.coefficients,
        pow,
        &data.inverse,
    ))
}

// The coefficients of $f(g)$ modulo $h$, with coefficients modulo $2^k$, as for
// `mod_compose_mod_polynomial_vec`.
//
// This is not part of the public API; it is public so that `malachite-nz` can compose
// `NaturalPolynomial`s modulo a power of 2 no larger than a word.
#[doc(hidden)]
pub fn mod_power_of_2_compose_mod_polynomial_vec<T: PrimitiveUnsigned>(
    f: &[T],
    g: &[T],
    h: &[T],
    pow: u64,
    inverse: &[T],
) -> Vec<T> {
    let n = h.len() - 1;
    let mut gs = g.to_vec();
    gs.resize(n, T::ZERO);
    compose_mod_dispatch(&ModPowerOf2Ring { pow, h, inverse }, f, &gs, n)
}

// The powers of `g` modulo `h` that Brent–Kung composition with `g` needs, where everything is
// reduced, `g` is shorter than `h`, `h` has at least 2 coefficients and an invertible leading
// coefficient, and `inverse` is the inverse of the reversal of `h` truncated to `h.len()` terms.
//
// This is not part of the public API; it is public so that `malachite-nz` can compose
// `NaturalPolynomial`s with word-sized coefficients.
#[doc(hidden)]
pub fn mod_power_of_2_compose_mod_polynomial_powers_vec<T: PrimitiveUnsigned>(
    g: &[T],
    h: &[T],
    pow: u64,
    inverse: &[T],
) -> Vec<Vec<T>> {
    let n = h.len() - 1;
    let mut gs = g.to_vec();
    gs.resize(n, T::ZERO);
    precompute_matrix(&ModPowerOf2Ring { pow, h, inverse }, &gs, n)
}

// The coefficients of $f(g)$ modulo `h`, where `powers` holds the powers of `g` from
// `mod_power_of_2_compose_mod_polynomial_powers_vec`, and the rest is as for that function.
//
// This is not part of the public API; it is public so that `malachite-nz` can compose
// `NaturalPolynomial`s with word-sized coefficients.
#[doc(hidden)]
#[inline]
pub fn mod_power_of_2_compose_mod_polynomial_with_powers_vec<T: PrimitiveUnsigned>(
    f: &[T],
    h: &[T],
    pow: u64,
    powers: &[Vec<T>],
    inverse: &[T],
) -> Vec<T> {
    compose_mod_brent_kung_with_powers(&ModPowerOf2Ring { pow, h, inverse }, f, powers, h.len() - 1)
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomial<Self, Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo
    /// $2^k$, taking the outer polynomial by value, `g` by value, and `h` by value. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $h$
    /// must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `g`, or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(g, h.clone(), 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(g, h, 8).to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(self, g: Self, h: Self, pow: u64) -> Self {
        mod_power_of_2_compose_mod_polynomial_helper(&self, &g, &h, pow, None)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomial<Self, &Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo
    /// $2^k$, taking the outer polynomial by value, `g` by value, and `h` by reference. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $h$
    /// must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `g`, or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(g, &h, 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(g, &h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(self, g: Self, h: &Self, pow: u64) -> Self {
        mod_power_of_2_compose_mod_polynomial_helper(&self, &g, h, pow, None)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomial<&Self, Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo
    /// $2^k$, taking the outer polynomial by value, `g` by reference, and `h` by value. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $h$
    /// must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `g`, or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(&g, h.clone(), 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(&g, h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(self, g: &Self, h: Self, pow: u64) -> Self {
        mod_power_of_2_compose_mod_polynomial_helper(&self, g, &h, pow, None)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomial<&Self, &Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo
    /// $2^k$, taking the outer polynomial by value, `g` by reference, and `h` by reference. The
    /// inner polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$,
    /// and the coefficients of all three must be reduced modulo $2^k$; the leading coefficient of
    /// $h$ must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `g`, or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(&g, &h, 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(&g, &h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(self, g: &Self, h: &Self, pow: u64) -> Self {
        mod_power_of_2_compose_mod_polynomial_helper(&self, g, h, pow, None)
    }
}

impl<T: PrimitiveUnsigned>
    ModPowerOf2ComposeModPolynomial<UnsignedPolynomial<T>, UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo
    /// $2^k$, taking the outer polynomial by reference, `g` by value, and `h` by value. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $h$
    /// must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `g`, or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(g, h.clone(), 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(g, h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(
        self,
        g: UnsignedPolynomial<T>,
        h: UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        mod_power_of_2_compose_mod_polynomial_helper(self, &g, &h, pow, None)
    }
}

impl<T: PrimitiveUnsigned>
    ModPowerOf2ComposeModPolynomial<UnsignedPolynomial<T>, &UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo
    /// $2^k$, taking the outer polynomial by reference, `g` by value, and `h` by reference. The
    /// inner polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$,
    /// and the coefficients of all three must be reduced modulo $2^k$; the leading coefficient of
    /// $h$ must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `g`, or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(g, &h, 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(g, &h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(
        self,
        g: UnsignedPolynomial<T>,
        h: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        mod_power_of_2_compose_mod_polynomial_helper(self, &g, h, pow, None)
    }
}

impl<T: PrimitiveUnsigned>
    ModPowerOf2ComposeModPolynomial<&UnsignedPolynomial<T>, UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo
    /// $2^k$, taking the outer polynomial by reference, `g` by reference, and `h` by value. The
    /// inner polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$,
    /// and the coefficients of all three must be reduced modulo $2^k$; the leading coefficient of
    /// $h$ must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `g`, or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(&g, h.clone(), 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(&g, h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(
        self,
        g: &UnsignedPolynomial<T>,
        h: UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        mod_power_of_2_compose_mod_polynomial_helper(self, g, &h, pow, None)
    }
}

impl<T: PrimitiveUnsigned>
    ModPowerOf2ComposeModPolynomial<&UnsignedPolynomial<T>, &UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo
    /// $2^k$, taking the outer polynomial by reference, `g` by reference, and `h` by reference. The
    /// inner polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$,
    /// and the coefficients of all three must be reduced modulo $2^k$; the leading coefficient of
    /// $h$ must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `g`, or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(&g, &h, 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(&g, &h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(
        self,
        g: &UnsignedPolynomial<T>,
        h: &UnsignedPolynomial<T>,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        mod_power_of_2_compose_mod_polynomial_helper(self, g, h, pow, None)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomialAssign<Self, Self>
    for UnsignedPolynomial<T>
{
    /// Replaces a [`UnsignedPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $2^k$, taking `g` by value, and `h` by value. The inner polynomial
    /// $g$ must already be reduced modulo $h$, with degree less than that of $h$, and the
    /// coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $h$ must
    /// be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `g`, or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(g, h.clone(), 8);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(g, h, 8);
    /// assert_eq!(f.to_string(), "127*x^2+89*x+15");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_assign(&mut self, g: Self, h: Self, pow: u64) {
        *self = mod_power_of_2_compose_mod_polynomial_helper(&take(self), &g, &h, pow, None);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomialAssign<Self, &Self>
    for UnsignedPolynomial<T>
{
    /// Replaces a [`UnsignedPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $2^k$, taking `g` by value, and `h` by reference. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $h$
    /// must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `g`, or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(g, &h, 8);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(g, &h, 8);
    /// assert_eq!(f.to_string(), "127*x^2+89*x+15");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_assign(&mut self, g: Self, h: &Self, pow: u64) {
        *self = mod_power_of_2_compose_mod_polynomial_helper(&take(self), &g, h, pow, None);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomialAssign<&Self, Self>
    for UnsignedPolynomial<T>
{
    /// Replaces a [`UnsignedPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $2^k$, taking `g` by reference, and `h` by value. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $h$
    /// must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `g`, or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(&g, h.clone(), 8);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(&g, h, 8);
    /// assert_eq!(f.to_string(), "127*x^2+89*x+15");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_assign(&mut self, g: &Self, h: Self, pow: u64) {
        *self = mod_power_of_2_compose_mod_polynomial_helper(&take(self), g, &h, pow, None);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomialAssign<&Self, &Self>
    for UnsignedPolynomial<T>
{
    /// Replaces a [`UnsignedPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $2^k$, taking `g` by reference, and `h` by reference. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $h$
    /// must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self`, `g`, or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(&g, &h, 8);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(&g, &h, 8);
    /// assert_eq!(f.to_string(), "127*x^2+89*x+15");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod` from `nmod_poly/compose_mod.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_assign(&mut self, g: &Self, h: &Self, pow: u64) {
        *self = mod_power_of_2_compose_mod_polynomial_helper(&take(self), g, h, pow, None);
    }
}
