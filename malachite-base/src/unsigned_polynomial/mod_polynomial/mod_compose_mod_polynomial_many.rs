// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
//      Copyright © 2014 Martin Lee
//
//      Copyright © 2020 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::modular::traits::ModIsReduced;
use crate::polynomial::Polynomial;
use crate::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
use crate::polynomial::modular::ModPolynomialData;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;
use crate::unsigned_polynomial::modular::mod_div_rem_precomputed::precompute_helper;
use alloc::vec::Vec;

// The compositions $f_i(g)$ modulo $h$ of the polynomials $f_i$ in `fs`, with coefficients modulo
// $m$, using `data` if it is given and computing it otherwise.
//
// This is equivalent to `nmod_poly_compose_mod_brent_kung_vec_preinv` from
// `nmod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, except that the $f_i$ may have any
// lengths and that `data` is computed if it is not given.
pub(crate) fn mod_compose_mod_polynomial_many_helper<T: PrimitiveUnsigned>(
    fs: &[UnsignedPolynomial<T>],
    g: &UnsignedPolynomial<T>,
    h: &UnsignedPolynomial<T>,
    m: T,
    data: Option<&ModPolynomialData<T>>,
) -> Vec<UnsignedPolynomial<T>> {
    let trivial = assert_compose_mod_polynomial_inner_and_modulus_arguments(g, h, m);
    for f in fs {
        assert!(
            f.mod_is_reduced(&m),
            "every polynomial must be reduced mod m, but {f} has a coefficient >= {m}"
        );
    }
    if trivial {
        return vec![UnsignedPolynomial::ZERO; fs.len()];
    } else if fs.is_empty() {
        return Vec::new();
    }
    let computed;
    let data = if let Some(data) = data {
        data
    } else {
        computed = precompute_helper(h, m);
        &computed
    };
    let fs: Vec<&[T]> = fs.iter().map(|f| f.coefficients.as_slice()).collect();
    mod_compose_mod_polynomial_many_vec(&fs, &g.coefficients, &h.coefficients, m, &data.inverse)
        .into_iter()
        .map(UnsignedPolynomial::from_coefficients_asc)
        .collect()
}

// The coefficients of the $f_i(g)$ modulo $h$, with coefficients modulo $m$, where everything is
// reduced, `g` is shorter than `h`, `h` is nonzero with an invertible leading coefficient, and
// `inverse` is the inverse of the reversal of `h`, as a power series, truncated to `h.len()` terms.
// Each result has `h.len() - 1` coefficients, or none if `h` is constant.
//
// This is not part of the public API; it is public so that `malachite-nz` can compose
// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
pub fn mod_compose_mod_polynomial_many_vec<T: PrimitiveUnsigned>(
    fs: &[&[T]],
    g: &[T],
    h: &[T],
    m: T,
    inverse: &[T],
) -> Vec<Vec<T>> {
    let n = h.len() - 1;
    let mut gs = g.to_vec();
    gs.resize(n, T::ZERO);
    compose_mod_dispatch_vec(&ModRing { m, h, inverse }, fs, &gs, n)
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomial<UnsignedPolynomial<T>, UnsignedPolynomial<T>, T>
    for &[UnsignedPolynomial<T>]
{
    type Output = Vec<UnsignedPolynomial<T>>;

    /// Composes each [`UnsignedPolynomial`] in a slice with the same polynomial $g$ modulo a
    /// polynomial $h$, with coefficients modulo $m$, taking both by value. The inner polynomial $g$
    /// must already be reduced modulo $h$, with degree less than that of $h$, and the coefficients
    /// of all the polynomials must be reduced modulo $m$; the leading coefficient of $h$ must be
    /// invertible modulo $m$. The outer polynomials may have any degrees.
    ///
    /// $$
    /// (f_0, \ldots, f_{c-1}) \mapsto (r_0, \ldots, r_{c-1}),
    /// \quad f_i(g) \equiv hq_i + r_i \pmod{m}, \quad \deg r_i < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so are the results.
    ///
    /// A single polynomial, or polynomials with few coefficients in all, are composed one at a
    /// time. Otherwise all of them are composed together by the Brent–Kung algorithm: about
    /// $\sqrt{l}$ powers of $g$ modulo $h$ are computed once, where $l$ is the sum of the lengths
    /// of the outer polynomials, the blocks of that many coefficients of all the outer polynomials
    /// are composed with $g$ at once, with one matrix product, and each polynomial's blocks are
    /// combined by evaluation at the next power of $g$. Every reduction modulo $h$ uses the inverse
    /// of the reversal of $h$ as a power series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l, c) = O(n\sqrt{l} \log n + (l + c\sqrt{l})n)$
    ///
    /// $M(n, l, c) = O((\sqrt{l} + c)n + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is the sum of the lengths
    /// of the polynomials in `self`, and $c$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of a polynomial in `self`, `g`, or `h` is greater
    /// than or equal to `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not
    /// invertible modulo `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let fs = [
    ///     UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap(),
    /// ];
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let rs = fs.as_slice().mod_compose_mod_polynomial(g, h, 7);
    /// assert_eq!(
    ///     rs.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["3*x^2+x+6", "5", "3*x^2+1"]
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_vec_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, except that the polynomials
    /// may have any degrees, that the inverse of the reversal of $h$ is computed rather than
    /// passed, that the results are returned, and that a few polynomials with few coefficients are
    /// composed one at a time.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: UnsignedPolynomial<T>,
        h: UnsignedPolynomial<T>,
        m: T,
    ) -> Vec<UnsignedPolynomial<T>> {
        mod_compose_mod_polynomial_many_helper(self, &g, &h, m, None)
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomial<UnsignedPolynomial<T>, &UnsignedPolynomial<T>, T>
    for &[UnsignedPolynomial<T>]
{
    type Output = Vec<UnsignedPolynomial<T>>;

    /// Composes each [`UnsignedPolynomial`] in a slice with the same polynomial $g$ modulo a
    /// polynomial $h$, with coefficients modulo $m$, taking `g` by value and `h` by reference. The
    /// inner polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$,
    /// and the coefficients of all the polynomials must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomials may have any
    /// degrees.
    ///
    /// $$
    /// (f_0, \ldots, f_{c-1}) \mapsto (r_0, \ldots, r_{c-1}),
    /// \quad f_i(g) \equiv hq_i + r_i \pmod{m}, \quad \deg r_i < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so are the results.
    ///
    /// A single polynomial, or polynomials with few coefficients in all, are composed one at a
    /// time. Otherwise all of them are composed together by the Brent–Kung algorithm: about
    /// $\sqrt{l}$ powers of $g$ modulo $h$ are computed once, where $l$ is the sum of the lengths
    /// of the outer polynomials, the blocks of that many coefficients of all the outer polynomials
    /// are composed with $g$ at once, with one matrix product, and each polynomial's blocks are
    /// combined by evaluation at the next power of $g$. Every reduction modulo $h$ uses the inverse
    /// of the reversal of $h$ as a power series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l, c) = O(n\sqrt{l} \log n + (l + c\sqrt{l})n)$
    ///
    /// $M(n, l, c) = O((\sqrt{l} + c)n + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is the sum of the lengths
    /// of the polynomials in `self`, and $c$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of a polynomial in `self`, `g`, or `h` is greater
    /// than or equal to `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not
    /// invertible modulo `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let fs = [
    ///     UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap(),
    /// ];
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let rs = fs.as_slice().mod_compose_mod_polynomial(g, &h, 7);
    /// assert_eq!(
    ///     rs.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["3*x^2+x+6", "5", "3*x^2+1"]
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_vec_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, except that the polynomials
    /// may have any degrees, that the inverse of the reversal of $h$ is computed rather than
    /// passed, that the results are returned, and that a few polynomials with few coefficients are
    /// composed one at a time.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: UnsignedPolynomial<T>,
        h: &UnsignedPolynomial<T>,
        m: T,
    ) -> Vec<UnsignedPolynomial<T>> {
        mod_compose_mod_polynomial_many_helper(self, &g, h, m, None)
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomial<&UnsignedPolynomial<T>, UnsignedPolynomial<T>, T>
    for &[UnsignedPolynomial<T>]
{
    type Output = Vec<UnsignedPolynomial<T>>;

    /// Composes each [`UnsignedPolynomial`] in a slice with the same polynomial $g$ modulo a
    /// polynomial $h$, with coefficients modulo $m$, taking `g` by reference and `h` by value. The
    /// inner polynomial $g$ must already be reduced modulo $h$, with degree less than that of $h$,
    /// and the coefficients of all the polynomials must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomials may have any
    /// degrees.
    ///
    /// $$
    /// (f_0, \ldots, f_{c-1}) \mapsto (r_0, \ldots, r_{c-1}),
    /// \quad f_i(g) \equiv hq_i + r_i \pmod{m}, \quad \deg r_i < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so are the results.
    ///
    /// A single polynomial, or polynomials with few coefficients in all, are composed one at a
    /// time. Otherwise all of them are composed together by the Brent–Kung algorithm: about
    /// $\sqrt{l}$ powers of $g$ modulo $h$ are computed once, where $l$ is the sum of the lengths
    /// of the outer polynomials, the blocks of that many coefficients of all the outer polynomials
    /// are composed with $g$ at once, with one matrix product, and each polynomial's blocks are
    /// combined by evaluation at the next power of $g$. Every reduction modulo $h$ uses the inverse
    /// of the reversal of $h$ as a power series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l, c) = O(n\sqrt{l} \log n + (l + c\sqrt{l})n)$
    ///
    /// $M(n, l, c) = O((\sqrt{l} + c)n + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is the sum of the lengths
    /// of the polynomials in `self`, and $c$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of a polynomial in `self`, `g`, or `h` is greater
    /// than or equal to `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not
    /// invertible modulo `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let fs = [
    ///     UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap(),
    /// ];
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let rs = fs.as_slice().mod_compose_mod_polynomial(&g, h, 7);
    /// assert_eq!(
    ///     rs.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["3*x^2+x+6", "5", "3*x^2+1"]
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_vec_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, except that the polynomials
    /// may have any degrees, that the inverse of the reversal of $h$ is computed rather than
    /// passed, that the results are returned, and that a few polynomials with few coefficients are
    /// composed one at a time.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: &UnsignedPolynomial<T>,
        h: UnsignedPolynomial<T>,
        m: T,
    ) -> Vec<UnsignedPolynomial<T>> {
        mod_compose_mod_polynomial_many_helper(self, g, &h, m, None)
    }
}

impl<T: PrimitiveUnsigned>
    ModComposeModPolynomial<&UnsignedPolynomial<T>, &UnsignedPolynomial<T>, T>
    for &[UnsignedPolynomial<T>]
{
    type Output = Vec<UnsignedPolynomial<T>>;

    /// Composes each [`UnsignedPolynomial`] in a slice with the same polynomial $g$ modulo a
    /// polynomial $h$, with coefficients modulo $m$, taking both by reference. The inner polynomial
    /// $g$ must already be reduced modulo $h$, with degree less than that of $h$, and the
    /// coefficients of all the polynomials must be reduced modulo $m$; the leading coefficient of
    /// $h$ must be invertible modulo $m$. The outer polynomials may have any degrees.
    ///
    /// $$
    /// (f_0, \ldots, f_{c-1}) \mapsto (r_0, \ldots, r_{c-1}),
    /// \quad f_i(g) \equiv hq_i + r_i \pmod{m}, \quad \deg r_i < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so are the results.
    ///
    /// A single polynomial, or polynomials with few coefficients in all, are composed one at a
    /// time. Otherwise all of them are composed together by the Brent–Kung algorithm: about
    /// $\sqrt{l}$ powers of $g$ modulo $h$ are computed once, where $l$ is the sum of the lengths
    /// of the outer polynomials, the blocks of that many coefficients of all the outer polynomials
    /// are composed with $g$ at once, with one matrix product, and each polynomial's blocks are
    /// combined by evaluation at the next power of $g$. Every reduction modulo $h$ uses the inverse
    /// of the reversal of $h$ as a power series, computed once.
    ///
    /// # Worst-case complexity
    /// $T(n, l, c) = O(n\sqrt{l} \log n + (l + c\sqrt{l})n)$
    ///
    /// $M(n, l, c) = O((\sqrt{l} + c)n + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is the sum of the lengths
    /// of the polynomials in `self`, and $c$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of a polynomial in `self`, `g`, or `h` is greater
    /// than or equal to `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not
    /// invertible modulo `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let fs = [
    ///     UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap(),
    /// ];
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let rs = fs.as_slice().mod_compose_mod_polynomial(&g, &h, 7);
    /// assert_eq!(
    ///     rs.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["3*x^2+x+6", "5", "3*x^2+1"]
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_vec_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, except that the polynomials
    /// may have any degrees, that the inverse of the reversal of $h$ is computed rather than
    /// passed, that the results are returned, and that a few polynomials with few coefficients are
    /// composed one at a time.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: &UnsignedPolynomial<T>,
        h: &UnsignedPolynomial<T>,
        m: T,
    ) -> Vec<UnsignedPolynomial<T>> {
        mod_compose_mod_polynomial_many_helper(self, g, h, m, None)
    }
}
