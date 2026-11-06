// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::mod_polynomial::mod_compose_mod_polynomial::{from_limbs, to_limbs};
use crate::natural_polynomial::mod_polynomial::mod_power_of_2_compose_mod_polynomial::{
    NaturalModPowerOf2Ring, assert_power_of_2_compose_mod_polynomial_inner_and_modulus_arguments,
};
use crate::natural_polynomial::modular::mod_power_of_2_div_rem_precomputed::precompute_helper;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::modular::traits::ModPowerOf2IsReduced;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
use malachite_base::polynomial::modular::ModPolynomialData;
use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;
use malachite_base::unsigned_polynomial::mod_polynomial::mod_power_of_2_compose_mod_polynomial_many::*;

// The compositions $f_i(g)$ modulo $h$ of the polynomials $f_i$ in `fs`, with coefficients modulo
// $2^k$, reducing with `data` if it is given.
//
// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_vec_preinv` from
// `fmpz_mod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
// that the $f_i$ may have any lengths and that `data` is computed if it is not given.
pub(crate) fn mod_power_of_2_compose_mod_polynomial_many_helper(
    fs: &[NaturalPolynomial],
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    pow: u64,
    data: Option<&ModPolynomialData<Natural>>,
) -> Vec<NaturalPolynomial> {
    let trivial = assert_power_of_2_compose_mod_polynomial_inner_and_modulus_arguments(g, h, pow);
    for f in fs {
        assert!(
            f.mod_power_of_2_is_reduced(pow),
            "every polynomial must be reduced mod 2^pow, but {f} has a coefficient >= 2^{pow}"
        );
    }
    if trivial {
        return vec![NaturalPolynomial::ZERO; fs.len()];
    } else if fs.is_empty() {
        return Vec::new();
    }
    if pow <= Limb::WIDTH {
        let hs = to_limbs(&h.coefficients);
        let inverse = if let Some(data) = data {
            to_limbs(&data.inverse)
        } else {
            UnsignedPolynomial::<Limb>::precompute_mod_power_of_2_div_rem_data(
                &UnsignedPolynomial::from_coefficients_asc(hs.clone()),
                pow,
            )
            .inverse
        };
        let fs: Vec<Vec<Limb>> = fs.iter().map(|f| to_limbs(&f.coefficients)).collect();
        let fs: Vec<&[Limb]> = fs.iter().map(Vec::as_slice).collect();
        return mod_power_of_2_compose_mod_polynomial_many_vec(
            &fs,
            &to_limbs(&g.coefficients),
            &hs,
            pow,
            &inverse,
        )
        .iter()
        .map(|xs| from_limbs(xs))
        .collect();
    }
    let computed;
    let data = if let Some(data) = data {
        data
    } else {
        computed = precompute_helper(h, pow);
        &computed
    };
    let n = h.coefficients.len() - 1;
    let mut gs = g.coefficients.clone();
    gs.resize(n, Natural::ZERO);
    let fs: Vec<&[Natural]> = fs.iter().map(|f| f.coefficients.as_slice()).collect();
    compose_mod_dispatch_vec(&NaturalModPowerOf2Ring { pow, h, data }, &fs, &gs, n)
        .into_iter()
        .map(NaturalPolynomial::from_coefficients_asc)
        .collect()
}

impl ModPowerOf2ComposeModPolynomial<NaturalPolynomial, NaturalPolynomial>
    for &[NaturalPolynomial]
{
    type Output = Vec<NaturalPolynomial>;

    /// Composes each [`NaturalPolynomial`] in a slice with the same polynomial $g$ modulo a
    /// polynomial $h$, with coefficients modulo $2^k$, taking both by value. The inner polynomial
    /// $g$ must already be reduced modulo $h$, with degree less than that of $h$, and the
    /// coefficients of all the polynomials must be reduced modulo $2^k$; the leading coefficient of
    /// $h$ must be odd. The outer polynomials may have any degrees.
    ///
    /// $$
    /// (f_0, \ldots, f_{c-1}) \mapsto (r_0, \ldots, r_{c-1}),
    /// \quad f_i(g) \equiv hq_i + r_i \pmod{2^k}, \quad \deg r_i < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so are the results.
    ///
    /// A single polynomial, or polynomials with few coefficients in all, are composed one at a
    /// time. Otherwise all of them are composed together by the Brent–Kung algorithm: about
    /// $\sqrt{l}$ powers of $g$ modulo $h$ are computed once, where $l$ is the sum of the lengths
    /// of the outer polynomials, the blocks of that many coefficients of all the outer polynomials
    /// are composed with $g$ at once, with one matrix product, and each polynomial's blocks are
    /// combined by evaluation at the next power of $g$. Every reduction modulo $h$ uses the inverse
    /// of the reversal of $h$ as a power series, computed once. When $k$ is at most the width of a
    /// limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, c, k) = O((n\sqrt{l} \log n + (l + c\sqrt{l})n)k \log k \log\log k)$
    ///
    /// $M(n, l, c, k) = O(((\sqrt{l} + c)n + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is the sum of the lengths
    /// of the polynomials in `self`, $c$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of a polynomial in `self`, `g`, or `h` is greater than or equal to
    /// $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not
    /// shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let fs = [
    ///     NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap(),
    ///     NaturalPolynomial::from_str("5").unwrap(),
    ///     NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap(),
    /// ];
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let rs = fs.as_slice().mod_power_of_2_compose_mod_polynomial(g, h, 3);
    /// assert_eq!(
    ///     rs.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["3*x^2+6", "5", "x^2+3*x+1"]
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_vec_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// except that the polynomials may have any degrees, that the inverse of the reversal of $h$ is
    /// computed rather than passed, that the results are returned, and that a few polynomials with
    /// few coefficients are composed one at a time.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(
        self,
        g: NaturalPolynomial,
        h: NaturalPolynomial,
        pow: u64,
    ) -> Vec<NaturalPolynomial> {
        mod_power_of_2_compose_mod_polynomial_many_helper(self, &g, &h, pow, None)
    }
}

impl ModPowerOf2ComposeModPolynomial<NaturalPolynomial, &NaturalPolynomial>
    for &[NaturalPolynomial]
{
    type Output = Vec<NaturalPolynomial>;

    /// Composes each [`NaturalPolynomial`] in a slice with the same polynomial $g$ modulo a
    /// polynomial $h$, with coefficients modulo $2^k$, taking `g` by value and `h` by reference.
    /// The inner polynomial $g$ must already be reduced modulo $h$, with degree less than that of
    /// $h$, and the coefficients of all the polynomials must be reduced modulo $2^k$; the leading
    /// coefficient of $h$ must be odd. The outer polynomials may have any degrees.
    ///
    /// $$
    /// (f_0, \ldots, f_{c-1}) \mapsto (r_0, \ldots, r_{c-1}),
    /// \quad f_i(g) \equiv hq_i + r_i \pmod{2^k}, \quad \deg r_i < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so are the results.
    ///
    /// A single polynomial, or polynomials with few coefficients in all, are composed one at a
    /// time. Otherwise all of them are composed together by the Brent–Kung algorithm: about
    /// $\sqrt{l}$ powers of $g$ modulo $h$ are computed once, where $l$ is the sum of the lengths
    /// of the outer polynomials, the blocks of that many coefficients of all the outer polynomials
    /// are composed with $g$ at once, with one matrix product, and each polynomial's blocks are
    /// combined by evaluation at the next power of $g$. Every reduction modulo $h$ uses the inverse
    /// of the reversal of $h$ as a power series, computed once. When $k$ is at most the width of a
    /// limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, c, k) = O((n\sqrt{l} \log n + (l + c\sqrt{l})n)k \log k \log\log k)$
    ///
    /// $M(n, l, c, k) = O(((\sqrt{l} + c)n + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is the sum of the lengths
    /// of the polynomials in `self`, $c$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of a polynomial in `self`, `g`, or `h` is greater than or equal to
    /// $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not
    /// shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let fs = [
    ///     NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap(),
    ///     NaturalPolynomial::from_str("5").unwrap(),
    ///     NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap(),
    /// ];
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let rs = fs
    ///     .as_slice()
    ///     .mod_power_of_2_compose_mod_polynomial(g, &h, 3);
    /// assert_eq!(
    ///     rs.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["3*x^2+6", "5", "x^2+3*x+1"]
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_vec_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// except that the polynomials may have any degrees, that the inverse of the reversal of $h$ is
    /// computed rather than passed, that the results are returned, and that a few polynomials with
    /// few coefficients are composed one at a time.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(
        self,
        g: NaturalPolynomial,
        h: &NaturalPolynomial,
        pow: u64,
    ) -> Vec<NaturalPolynomial> {
        mod_power_of_2_compose_mod_polynomial_many_helper(self, &g, h, pow, None)
    }
}

impl ModPowerOf2ComposeModPolynomial<&NaturalPolynomial, NaturalPolynomial>
    for &[NaturalPolynomial]
{
    type Output = Vec<NaturalPolynomial>;

    /// Composes each [`NaturalPolynomial`] in a slice with the same polynomial $g$ modulo a
    /// polynomial $h$, with coefficients modulo $2^k$, taking `g` by reference and `h` by value.
    /// The inner polynomial $g$ must already be reduced modulo $h$, with degree less than that of
    /// $h$, and the coefficients of all the polynomials must be reduced modulo $2^k$; the leading
    /// coefficient of $h$ must be odd. The outer polynomials may have any degrees.
    ///
    /// $$
    /// (f_0, \ldots, f_{c-1}) \mapsto (r_0, \ldots, r_{c-1}),
    /// \quad f_i(g) \equiv hq_i + r_i \pmod{2^k}, \quad \deg r_i < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so are the results.
    ///
    /// A single polynomial, or polynomials with few coefficients in all, are composed one at a
    /// time. Otherwise all of them are composed together by the Brent–Kung algorithm: about
    /// $\sqrt{l}$ powers of $g$ modulo $h$ are computed once, where $l$ is the sum of the lengths
    /// of the outer polynomials, the blocks of that many coefficients of all the outer polynomials
    /// are composed with $g$ at once, with one matrix product, and each polynomial's blocks are
    /// combined by evaluation at the next power of $g$. Every reduction modulo $h$ uses the inverse
    /// of the reversal of $h$ as a power series, computed once. When $k$ is at most the width of a
    /// limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, c, k) = O((n\sqrt{l} \log n + (l + c\sqrt{l})n)k \log k \log\log k)$
    ///
    /// $M(n, l, c, k) = O(((\sqrt{l} + c)n + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is the sum of the lengths
    /// of the polynomials in `self`, $c$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of a polynomial in `self`, `g`, or `h` is greater than or equal to
    /// $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not
    /// shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let fs = [
    ///     NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap(),
    ///     NaturalPolynomial::from_str("5").unwrap(),
    ///     NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap(),
    /// ];
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let rs = fs
    ///     .as_slice()
    ///     .mod_power_of_2_compose_mod_polynomial(&g, h, 3);
    /// assert_eq!(
    ///     rs.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["3*x^2+6", "5", "x^2+3*x+1"]
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_vec_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// except that the polynomials may have any degrees, that the inverse of the reversal of $h$ is
    /// computed rather than passed, that the results are returned, and that a few polynomials with
    /// few coefficients are composed one at a time.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(
        self,
        g: &NaturalPolynomial,
        h: NaturalPolynomial,
        pow: u64,
    ) -> Vec<NaturalPolynomial> {
        mod_power_of_2_compose_mod_polynomial_many_helper(self, g, &h, pow, None)
    }
}

impl ModPowerOf2ComposeModPolynomial<&NaturalPolynomial, &NaturalPolynomial>
    for &[NaturalPolynomial]
{
    type Output = Vec<NaturalPolynomial>;

    /// Composes each [`NaturalPolynomial`] in a slice with the same polynomial $g$ modulo a
    /// polynomial $h$, with coefficients modulo $2^k$, taking both by reference. The inner
    /// polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$, and
    /// the coefficients of all the polynomials must be reduced modulo $2^k$; the leading
    /// coefficient of $h$ must be odd. The outer polynomials may have any degrees.
    ///
    /// $$
    /// (f_0, \ldots, f_{c-1}) \mapsto (r_0, \ldots, r_{c-1}),
    /// \quad f_i(g) \equiv hq_i + r_i \pmod{2^k}, \quad \deg r_i < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so are the results.
    ///
    /// A single polynomial, or polynomials with few coefficients in all, are composed one at a
    /// time. Otherwise all of them are composed together by the Brent–Kung algorithm: about
    /// $\sqrt{l}$ powers of $g$ modulo $h$ are computed once, where $l$ is the sum of the lengths
    /// of the outer polynomials, the blocks of that many coefficients of all the outer polynomials
    /// are composed with $g$ at once, with one matrix product, and each polynomial's blocks are
    /// combined by evaluation at the next power of $g$. Every reduction modulo $h$ uses the inverse
    /// of the reversal of $h$ as a power series, computed once. When $k$ is at most the width of a
    /// limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, c, k) = O((n\sqrt{l} \log n + (l + c\sqrt{l})n)k \log k \log\log k)$
    ///
    /// $M(n, l, c, k) = O(((\sqrt{l} + c)n + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is the sum of the lengths
    /// of the polynomials in `self`, $c$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of a polynomial in `self`, `g`, or `h` is greater than or equal to
    /// $2^k$, or if `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not
    /// shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let fs = [
    ///     NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap(),
    ///     NaturalPolynomial::from_str("5").unwrap(),
    ///     NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap(),
    /// ];
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let rs = fs
    ///     .as_slice()
    ///     .mod_power_of_2_compose_mod_polynomial(&g, &h, 3);
    /// assert_eq!(
    ///     rs.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["3*x^2+6", "5", "x^2+3*x+1"]
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_vec_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, with the modulus $2^k$,
    /// except that the polynomials may have any degrees, that the inverse of the reversal of $h$ is
    /// computed rather than passed, that the results are returned, and that a few polynomials with
    /// few coefficients are composed one at a time.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(
        self,
        g: &NaturalPolynomial,
        h: &NaturalPolynomial,
        pow: u64,
    ) -> Vec<NaturalPolynomial> {
        mod_power_of_2_compose_mod_polynomial_many_helper(self, g, h, pow, None)
    }
}
