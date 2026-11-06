// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2013, 2014 Martin Lee
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::FloorSqrt;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::modular::traits::ModPowerOf2IsReduced;
use crate::polynomial::Polynomial;
use crate::polynomial::mod_polynomial::ModComposeModPolynomialData;
use crate::polynomial::mod_polynomial::traits::{
    ModPowerOf2ComposeModPolynomialPrecomputedPowers,
    ModPowerOf2ComposeModPolynomialPrecomputedPowersAssign,
};
use crate::polynomial::modular::ModPolynomialData;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::{
    compose_mod_brent_kung_with_powers, precompute_matrix,
};
use crate::unsigned_polynomial::mod_polynomial::mod_power_of_2_compose_mod_polynomial::*;
use crate::unsigned_polynomial::modular::mod_power_of_2_div_rem_precomputed::precompute_helper;
use alloc::vec::Vec;
use core::mem::take;

// The powers of `g` modulo `h` that Brent–Kung composition with `g` needs, and the data for
// dividing by `h`.
//
// This is equivalent to `nmod_poly_precompute_matrix` from
// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, together with the inverse of
// the reversal of `h`, except that `g` must be reduced modulo `h`.
pub(crate) fn precompute_power_of_2_powers_helper<T: PrimitiveUnsigned>(
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    pow: u64,
) -> ModComposeModPolynomialData<T> {
    if assert_power_of_2_compose_mod_polynomial_inner_and_modulus_arguments(g, h, pow)
        || h.len() == 1
    {
        return ModComposeModPolynomialData {
            powers: Vec::new(),
            division: ModPolynomialData {
                inverse: Vec::new(),
            },
        };
    }
    let division = precompute_helper(h, pow);
    let n = h.coefficients.len() - 1;
    let mut gs = g.coefficients.clone();
    gs.resize(n, T::ZERO);
    let powers = precompute_matrix(
        &ModPowerOf2Ring {
            pow,
            h: &h.coefficients,
            inverse: &division.inverse,
        },
        &gs,
        n,
    );
    ModComposeModPolynomialData { powers, division }
}

// The composition $f(g)$ modulo $h$, with coefficients modulo $2^k$, where `data` holds the powers
// of `g`.
//
// This is equivalent to `nmod_poly_compose_mod_brent_kung_precomp_preinv` from
// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `f` may have any
// degree.
pub(crate) fn mod_power_of_2_compose_mod_polynomial_precomputed_powers_helper<
    T: PrimitiveUnsigned,
>(
    f: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    pow: u64,
    data: &ModComposeModPolynomialData<T>,
) -> UnsignedPolynomial<T> {
    assert!(pow <= T::WIDTH);
    assert!(
        f.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {f} has a coefficient >= 2^{pow}"
    );
    assert!(
        h.mod_power_of_2_is_reduced(pow),
        "h must be reduced mod 2^pow, but {h} has a coefficient >= 2^{pow}"
    );
    if pow == 0 {
        return UnsignedPolynomial::ZERO;
    }
    assert_ne!(*h, UnsignedPolynomial::ZERO, "division by zero");
    let n = h.coefficients.len() - 1;
    if n == 0 {
        return UnsignedPolynomial::ZERO;
    }
    assert!(
        data.powers.len() == n.floor_sqrt() + 1 && data.powers.iter().all(|p| p.len() == n),
        "data was not precomputed for h"
    );
    let ring = ModPowerOf2Ring {
        pow,
        h: &h.coefficients,
        inverse: &data.division.inverse,
    };
    UnsignedPolynomial::from_coefficients_asc(compose_mod_brent_kung_with_powers(
        &ring,
        &f.coefficients,
        &data.powers,
        n,
    ))
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomialPrecomputedPowers<Self>
    for UnsignedPolynomial<T>
{
    type Output = Self;
    type Data = ModComposeModPolynomialData<T>;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $2^k$, for composing
    /// [`UnsignedPolynomial`]s with `g` modulo `h` by
    /// [`ModPowerOf2ComposeModPolynomialPrecomputedPowers`] or
    /// [`ModPowerOf2ComposeModPolynomialPrecomputedPowersAssign`]. `g` must already be reduced
    /// modulo `h`, the coefficients of both must be reduced modulo $2^k$, and the leading
    /// coefficient of `h` must be odd.
    ///
    /// The data is the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg
    /// h}\rfloor + 1$, and the inverse of the reversal of $h$ as a power series.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n)$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `h.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `g` or `h` is greater
    /// than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient is even,
    /// or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
    ///     &g, &h, &8,
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(h.clone(), 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(h, 8, &data)
    ///         .to_string(),
    ///     "17*x^2+35*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_precompute_matrix` from
    /// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// together with the inverse of the reversal of `h`, except that `g` must be reduced modulo
    /// $h$.
    #[inline]
    fn precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
        g: &Self,
        h: &Self,
        pow: &u64,
    ) -> ModComposeModPolynomialData<T> {
        precompute_power_of_2_powers_helper(g, h, *pow)
    }

    /// Composes a [`UnsignedPolynomial`] with the inner polynomial $g$ that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $2^k$, taking the outer polynomial
    /// by value, and `h` by value. The coefficients of the outer polynomial and of $h$ must be
    /// reduced modulo $2^k$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// `precompute_mod_power_of_2_compose_mod_polynomial_powers_data`. It holds the powers $g^0,
    /// \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor + 1$, so the blocks of
    /// $s$ coefficients of the outer polynomial are composed with $g$ all at once, with one matrix
    /// product, and are then combined by evaluation at $g^s$.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(ln + l\sqrt{n} \log n)$
    ///
    /// $M(n, l) = O(l\sqrt{n} + n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even. If `data` was not precomputed for `h`, the result is meaningless, and the function
    /// may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
    ///     &g, &h, &8,
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(h.clone(), 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(h, 8, &data)
    ///         .to_string(),
    ///     "17*x^2+35*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// except that `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed_powers(
        self,
        h: Self,
        pow: u64,
        data: &ModComposeModPolynomialData<T>,
    ) -> Self {
        mod_power_of_2_compose_mod_polynomial_precomputed_powers_helper(&self, &h, pow, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomialPrecomputedPowers<&Self>
    for UnsignedPolynomial<T>
{
    type Output = Self;
    type Data = ModComposeModPolynomialData<T>;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $2^k$, for composing
    /// [`UnsignedPolynomial`]s with `g` modulo `h` by
    /// [`ModPowerOf2ComposeModPolynomialPrecomputedPowers`] or
    /// [`ModPowerOf2ComposeModPolynomialPrecomputedPowersAssign`]. `g` must already be reduced
    /// modulo `h`, the coefficients of both must be reduced modulo $2^k$, and the leading
    /// coefficient of `h` must be odd.
    ///
    /// The data is the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg
    /// h}\rfloor + 1$, and the inverse of the reversal of $h$ as a power series.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n)$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `h.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `g` or `h` is greater
    /// than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient is even,
    /// or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
    ///     &g, &h, &8,
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(h.clone(), 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(h, 8, &data)
    ///         .to_string(),
    ///     "17*x^2+35*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_precompute_matrix` from
    /// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// together with the inverse of the reversal of `h`, except that `g` must be reduced modulo
    /// $h$.
    #[inline]
    fn precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
        g: &&Self,
        h: &&Self,
        pow: &u64,
    ) -> ModComposeModPolynomialData<T> {
        precompute_power_of_2_powers_helper(g, h, *pow)
    }

    /// Composes a [`UnsignedPolynomial`] with the inner polynomial $g$ that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $2^k$, taking the outer polynomial
    /// by value, and `h` by reference. The coefficients of the outer polynomial and of $h$ must be
    /// reduced modulo $2^k$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// `precompute_mod_power_of_2_compose_mod_polynomial_powers_data`. It holds the powers $g^0,
    /// \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor + 1$, so the blocks of
    /// $s$ coefficients of the outer polynomial are composed with $g$ all at once, with one matrix
    /// product, and are then combined by evaluation at $g^s$.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(ln + l\sqrt{n} \log n)$
    ///
    /// $M(n, l) = O(l\sqrt{n} + n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even. If `data` was not precomputed for `h`, the result is meaningless, and the function
    /// may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
    ///     &g, &h, &8,
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(&h, 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(&h, 8, &data)
    ///         .to_string(),
    ///     "17*x^2+35*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// except that `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed_powers(
        self,
        h: &Self,
        pow: u64,
        data: &ModComposeModPolynomialData<T>,
    ) -> Self {
        mod_power_of_2_compose_mod_polynomial_precomputed_powers_helper(&self, h, pow, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomialPrecomputedPowers<UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModComposeModPolynomialData<T>;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $2^k$, for composing
    /// [`UnsignedPolynomial`]s with `g` modulo `h` by
    /// [`ModPowerOf2ComposeModPolynomialPrecomputedPowers`] or
    /// [`ModPowerOf2ComposeModPolynomialPrecomputedPowersAssign`]. `g` must already be reduced
    /// modulo `h`, the coefficients of both must be reduced modulo $2^k$, and the leading
    /// coefficient of `h` must be odd.
    ///
    /// The data is the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg
    /// h}\rfloor + 1$, and the inverse of the reversal of $h$ as a power series.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n)$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `h.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `g` or `h` is greater
    /// than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient is even,
    /// or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
    ///     &g, &h, &8,
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(h.clone(), 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(h, 8, &data)
    ///         .to_string(),
    ///     "17*x^2+35*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_precompute_matrix` from
    /// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// together with the inverse of the reversal of `h`, except that `g` must be reduced modulo
    /// $h$.
    #[inline]
    fn precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
        g: &UnsignedPolynomial<T>,
        h: &UnsignedPolynomial<T>,
        pow: &u64,
    ) -> ModComposeModPolynomialData<T> {
        precompute_power_of_2_powers_helper(g, h, *pow)
    }

    /// Composes a [`UnsignedPolynomial`] with the inner polynomial $g$ that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $2^k$, taking the outer polynomial
    /// by reference, and `h` by value. The coefficients of the outer polynomial and of $h$ must be
    /// reduced modulo $2^k$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// `precompute_mod_power_of_2_compose_mod_polynomial_powers_data`. It holds the powers $g^0,
    /// \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor + 1$, so the blocks of
    /// $s$ coefficients of the outer polynomial are composed with $g$ all at once, with one matrix
    /// product, and are then combined by evaluation at $g^s$.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(ln + l\sqrt{n} \log n)$
    ///
    /// $M(n, l) = O(l\sqrt{n} + n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even. If `data` was not precomputed for `h`, the result is meaningless, and the function
    /// may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
    ///     &g, &h, &8,
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial_precomputed_powers(h.clone(), 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial_precomputed_powers(h, 8, &data)
    ///         .to_string(),
    ///     "17*x^2+35*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// except that `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed_powers(
        self,
        h: UnsignedPolynomial<T>,
        pow: u64,
        data: &ModComposeModPolynomialData<T>,
    ) -> UnsignedPolynomial<T> {
        mod_power_of_2_compose_mod_polynomial_precomputed_powers_helper(self, &h, pow, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomialPrecomputedPowers<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModComposeModPolynomialData<T>;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $2^k$, for composing
    /// [`UnsignedPolynomial`]s with `g` modulo `h` by
    /// [`ModPowerOf2ComposeModPolynomialPrecomputedPowers`] or
    /// [`ModPowerOf2ComposeModPolynomialPrecomputedPowersAssign`]. `g` must already be reduced
    /// modulo `h`, the coefficients of both must be reduced modulo $2^k$, and the leading
    /// coefficient of `h` must be odd.
    ///
    /// The data is the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg
    /// h}\rfloor + 1$, and the inverse of the reversal of $h$ as a power series.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{3/2} \log n)$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `h.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `g` or `h` is greater
    /// than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient is even,
    /// or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
    ///     &g, &h, &8,
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(h.clone(), 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed_powers(h, 8, &data)
    ///         .to_string(),
    ///     "17*x^2+35*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_precompute_matrix` from
    /// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// together with the inverse of the reversal of `h`, except that `g` must be reduced modulo
    /// $h$.
    #[inline]
    fn precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
        g: &&UnsignedPolynomial<T>,
        h: &&UnsignedPolynomial<T>,
        pow: &u64,
    ) -> ModComposeModPolynomialData<T> {
        precompute_power_of_2_powers_helper(g, h, *pow)
    }

    /// Composes a [`UnsignedPolynomial`] with the inner polynomial $g$ that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $2^k$, taking the outer polynomial
    /// by reference, and `h` by reference. The coefficients of the outer polynomial and of $h$ must
    /// be reduced modulo $2^k$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// `precompute_mod_power_of_2_compose_mod_polynomial_powers_data`. It holds the powers $g^0,
    /// \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor + 1$, so the blocks of
    /// $s$ coefficients of the outer polynomial are composed with $g$ all at once, with one matrix
    /// product, and are then combined by evaluation at $g^s$.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(ln + l\sqrt{n} \log n)$
    ///
    /// $M(n, l) = O(l\sqrt{n} + n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even. If `data` was not precomputed for `h`, the result is meaningless, and the function
    /// may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
    ///     &g, &h, &8,
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial_precomputed_powers(&h, 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial_precomputed_powers(&h, 8, &data)
    ///         .to_string(),
    ///     "17*x^2+35*x+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// except that `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed_powers(
        self,
        h: &UnsignedPolynomial<T>,
        pow: u64,
        data: &ModComposeModPolynomialData<T>,
    ) -> UnsignedPolynomial<T> {
        mod_power_of_2_compose_mod_polynomial_precomputed_powers_helper(self, h, pow, data)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomialPrecomputedPowersAssign<Self>
    for UnsignedPolynomial<T>
{
    /// Replaces a [`UnsignedPolynomial`] with its composition with the inner polynomial $g$ that
    /// `data` was precomputed from, modulo a polynomial $h$, with coefficients modulo $2^k$, taking
    /// `h` by value. The coefficients of the outer polynomial and of $h$ must be reduced modulo
    /// $2^k$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// `precompute_mod_power_of_2_compose_mod_polynomial_powers_data`. It holds the powers $g^0,
    /// \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor + 1$, so the blocks of
    /// $s$ coefficients of the outer polynomial are composed with $g$ all at once, with one matrix
    /// product, and are then combined by evaluation at $g^s$.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(ln + l\sqrt{n} \log n)$
    ///
    /// $M(n, l) = O(l\sqrt{n} + n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even. If `data` was not precomputed for `h`, the result is meaningless, and the function
    /// may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::{
    ///     ModPowerOf2ComposeModPolynomialPrecomputedPowers,
    ///     ModPowerOf2ComposeModPolynomialPrecomputedPowersAssign,
    /// };
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
    ///     &g, &h, &8,
    /// );
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_precomputed_powers_assign(h.clone(), 8, &data);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_precomputed_powers_assign(h, 8, &data);
    /// assert_eq!(f.to_string(), "17*x^2+35*x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// except that `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed_powers_assign(
        &mut self,
        h: Self,
        pow: u64,
        data: &ModComposeModPolynomialData<T>,
    ) {
        *self = mod_power_of_2_compose_mod_polynomial_precomputed_powers_helper(
            &take(self),
            &h,
            pow,
            data,
        );
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeModPolynomialPrecomputedPowersAssign<&Self>
    for UnsignedPolynomial<T>
{
    /// Replaces a [`UnsignedPolynomial`] with its composition with the inner polynomial $g$ that
    /// `data` was precomputed from, modulo a polynomial $h$, with coefficients modulo $2^k$, taking
    /// `h` by reference. The coefficients of the outer polynomial and of $h$ must be reduced modulo
    /// $2^k$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with
    /// `precompute_mod_power_of_2_compose_mod_polynomial_powers_data`. It holds the powers $g^0,
    /// \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor + 1$, so the blocks of
    /// $s$ coefficients of the outer polynomial are composed with $g$ all at once, with one matrix
    /// product, and are then combined by evaluation at $g^s$.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(ln + l\sqrt{n} \log n)$
    ///
    /// $M(n, l) = O(l\sqrt{n} + n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of `self` or `h` is
    /// greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient
    /// is even. If `data` was not precomputed for `h`, the result is meaningless, and the function
    /// may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::{
    ///     ModPowerOf2ComposeModPolynomialPrecomputedPowers,
    ///     ModPowerOf2ComposeModPolynomialPrecomputedPowersAssign,
    /// };
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_compose_mod_polynomial_powers_data(
    ///     &g, &h, &8,
    /// );
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_precomputed_powers_assign(&h, 8, &data);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_precomputed_powers_assign(&h, 8, &data);
    /// assert_eq!(f.to_string(), "17*x^2+35*x+1");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// except that `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed_powers_assign(
        &mut self,
        h: &Self,
        pow: u64,
        data: &ModComposeModPolynomialData<T>,
    ) {
        *self = mod_power_of_2_compose_mod_polynomial_precomputed_powers_helper(
            &take(self),
            h,
            pow,
            data,
        );
    }
}
