// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomialPrecomputed;
use crate::polynomial::modular::ModPolynomialData;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::mod_polynomial::mod_power_of_2_compose_mod_polynomial_many::*;
use alloc::vec::Vec;

impl<T: PrimitiveUnsigned>
    ModPowerOf2ComposeModPolynomialPrecomputed<UnsignedPolynomial<T>, UnsignedPolynomial<T>>
    for &[UnsignedPolynomial<T>]
{
    type Output = Vec<UnsignedPolynomial<T>>;
    type Data = ModPolynomialData<T>;

    /// Composes each [`UnsignedPolynomial`] in a slice with the same polynomial $g$ modulo a
    /// polynomial $h$, with coefficients modulo $2^k$, using data precomputed from $h$ and $k$,
    /// taking both by value. The inner polynomial $g$ must already be reduced modulo $h$, with
    /// degree less than that of $h$, and the coefficients of all the polynomials must be reduced
    /// modulo $2^k$; the leading coefficient of $h$ must be odd. The outer polynomials may have any
    /// degrees.
    ///
    /// $$
    /// (f_0, \ldots, f_{c-1}) \mapsto (r_0, \ldots, r_{c-1}),
    /// \quad f_i(g) \equiv hq_i + r_i \pmod{2^k}, \quad \deg r_i < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so are the results.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`](
    /// crate::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed),
    /// as for division. A single polynomial, or polynomials with few coefficients in all, are
    /// composed one at a time. Otherwise all of them are composed together by the Brent–Kung
    /// algorithm: about $\sqrt{l}$ powers of $g$ modulo $h$ are computed once, where $l$ is the sum
    /// of the lengths of the outer polynomials, the blocks of that many coefficients of all the
    /// outer polynomials are composed with $g$ at once, with one matrix product, and each
    /// polynomial's blocks are combined by evaluation at the next power of $g$. Every reduction
    /// modulo $h$ uses the inverse of the reversal of $h$ as a power series, given by the data.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of a polynomial in `self`,
    /// `g`, or `h` is greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its
    /// leading coefficient is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let fs = [
    ///     UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap(),
    /// ];
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 3);
    /// let rs = fs
    ///     .as_slice()
    ///     .mod_power_of_2_compose_mod_polynomial_precomputed(g, h, 3, &data);
    /// assert_eq!(
    ///     rs.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["3*x^2+6", "5", "x^2+3*x+1"]
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_vec_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that the polynomials may have any degrees, that the inverse of the reversal of $h$ is given
    /// by the data rather than as a polynomial, that the results are returned, and that a few
    /// polynomials with few coefficients are composed one at a time.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed(
        self,
        g: UnsignedPolynomial<T>,
        h: UnsignedPolynomial<T>,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> Vec<UnsignedPolynomial<T>> {
        mod_power_of_2_compose_mod_polynomial_many_helper(self, &g, &h, pow, Some(data))
    }
}

impl<T: PrimitiveUnsigned>
    ModPowerOf2ComposeModPolynomialPrecomputed<UnsignedPolynomial<T>, &UnsignedPolynomial<T>>
    for &[UnsignedPolynomial<T>]
{
    type Output = Vec<UnsignedPolynomial<T>>;
    type Data = ModPolynomialData<T>;

    /// Composes each [`UnsignedPolynomial`] in a slice with the same polynomial $g$ modulo a
    /// polynomial $h$, with coefficients modulo $2^k$, using data precomputed from $h$ and $k$,
    /// taking `g` by value and `h` by reference. The inner polynomial $g$ must already be reduced
    /// modulo $h$, with degree less than that of $h$, and the coefficients of all the polynomials
    /// must be reduced modulo $2^k$; the leading coefficient of $h$ must be odd. The outer
    /// polynomials may have any degrees.
    ///
    /// $$
    /// (f_0, \ldots, f_{c-1}) \mapsto (r_0, \ldots, r_{c-1}),
    /// \quad f_i(g) \equiv hq_i + r_i \pmod{2^k}, \quad \deg r_i < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so are the results.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`](
    /// crate::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed),
    /// as for division. A single polynomial, or polynomials with few coefficients in all, are
    /// composed one at a time. Otherwise all of them are composed together by the Brent–Kung
    /// algorithm: about $\sqrt{l}$ powers of $g$ modulo $h$ are computed once, where $l$ is the sum
    /// of the lengths of the outer polynomials, the blocks of that many coefficients of all the
    /// outer polynomials are composed with $g$ at once, with one matrix product, and each
    /// polynomial's blocks are combined by evaluation at the next power of $g$. Every reduction
    /// modulo $h$ uses the inverse of the reversal of $h$ as a power series, given by the data.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of a polynomial in `self`,
    /// `g`, or `h` is greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its
    /// leading coefficient is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let fs = [
    ///     UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap(),
    /// ];
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 3);
    /// let rs = fs
    ///     .as_slice()
    ///     .mod_power_of_2_compose_mod_polynomial_precomputed(g, &h, 3, &data);
    /// assert_eq!(
    ///     rs.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["3*x^2+6", "5", "x^2+3*x+1"]
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_vec_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that the polynomials may have any degrees, that the inverse of the reversal of $h$ is given
    /// by the data rather than as a polynomial, that the results are returned, and that a few
    /// polynomials with few coefficients are composed one at a time.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed(
        self,
        g: UnsignedPolynomial<T>,
        h: &UnsignedPolynomial<T>,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> Vec<UnsignedPolynomial<T>> {
        mod_power_of_2_compose_mod_polynomial_many_helper(self, &g, h, pow, Some(data))
    }
}

impl<T: PrimitiveUnsigned>
    ModPowerOf2ComposeModPolynomialPrecomputed<&UnsignedPolynomial<T>, UnsignedPolynomial<T>>
    for &[UnsignedPolynomial<T>]
{
    type Output = Vec<UnsignedPolynomial<T>>;
    type Data = ModPolynomialData<T>;

    /// Composes each [`UnsignedPolynomial`] in a slice with the same polynomial $g$ modulo a
    /// polynomial $h$, with coefficients modulo $2^k$, using data precomputed from $h$ and $k$,
    /// taking `g` by reference and `h` by value. The inner polynomial $g$ must already be reduced
    /// modulo $h$, with degree less than that of $h$, and the coefficients of all the polynomials
    /// must be reduced modulo $2^k$; the leading coefficient of $h$ must be odd. The outer
    /// polynomials may have any degrees.
    ///
    /// $$
    /// (f_0, \ldots, f_{c-1}) \mapsto (r_0, \ldots, r_{c-1}),
    /// \quad f_i(g) \equiv hq_i + r_i \pmod{2^k}, \quad \deg r_i < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so are the results.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`](
    /// crate::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed),
    /// as for division. A single polynomial, or polynomials with few coefficients in all, are
    /// composed one at a time. Otherwise all of them are composed together by the Brent–Kung
    /// algorithm: about $\sqrt{l}$ powers of $g$ modulo $h$ are computed once, where $l$ is the sum
    /// of the lengths of the outer polynomials, the blocks of that many coefficients of all the
    /// outer polynomials are composed with $g$ at once, with one matrix product, and each
    /// polynomial's blocks are combined by evaluation at the next power of $g$. Every reduction
    /// modulo $h$ uses the inverse of the reversal of $h$ as a power series, given by the data.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of a polynomial in `self`,
    /// `g`, or `h` is greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its
    /// leading coefficient is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let fs = [
    ///     UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap(),
    /// ];
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 3);
    /// let rs = fs
    ///     .as_slice()
    ///     .mod_power_of_2_compose_mod_polynomial_precomputed(&g, h, 3, &data);
    /// assert_eq!(
    ///     rs.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["3*x^2+6", "5", "x^2+3*x+1"]
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_vec_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that the polynomials may have any degrees, that the inverse of the reversal of $h$ is given
    /// by the data rather than as a polynomial, that the results are returned, and that a few
    /// polynomials with few coefficients are composed one at a time.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed(
        self,
        g: &UnsignedPolynomial<T>,
        h: UnsignedPolynomial<T>,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> Vec<UnsignedPolynomial<T>> {
        mod_power_of_2_compose_mod_polynomial_many_helper(self, g, &h, pow, Some(data))
    }
}

impl<T: PrimitiveUnsigned>
    ModPowerOf2ComposeModPolynomialPrecomputed<&UnsignedPolynomial<T>, &UnsignedPolynomial<T>>
    for &[UnsignedPolynomial<T>]
{
    type Output = Vec<UnsignedPolynomial<T>>;
    type Data = ModPolynomialData<T>;

    /// Composes each [`UnsignedPolynomial`] in a slice with the same polynomial $g$ modulo a
    /// polynomial $h$, with coefficients modulo $2^k$, using data precomputed from $h$ and $k$,
    /// taking both by reference. The inner polynomial $g$ must already be reduced modulo $h$, with
    /// degree less than that of $h$, and the coefficients of all the polynomials must be reduced
    /// modulo $2^k$; the leading coefficient of $h$ must be odd. The outer polynomials may have any
    /// degrees.
    ///
    /// $$
    /// (f_0, \ldots, f_{c-1}) \mapsto (r_0, \ldots, r_{c-1}),
    /// \quad f_i(g) \equiv hq_i + r_i \pmod{2^k}, \quad \deg r_i < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so are the results.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`](
    /// crate::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed),
    /// as for division. A single polynomial, or polynomials with few coefficients in all, are
    /// composed one at a time. Otherwise all of them are composed together by the Brent–Kung
    /// algorithm: about $\sqrt{l}$ powers of $g$ modulo $h$ are computed once, where $l$ is the sum
    /// of the lengths of the outer polynomials, the blocks of that many coefficients of all the
    /// outer polynomials are composed with $g$ at once, with one matrix product, and each
    /// polynomial's blocks are combined by evaluation at the next power of $g$. Every reduction
    /// modulo $h$ uses the inverse of the reversal of $h$ as a power series, given by the data.
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
    /// Panics if `pow` is greater than `T::WIDTH`, or if any coefficient of a polynomial in `self`,
    /// `g`, or `h` is greater than or equal to $2^k$, or if `h` is 0 (unless $k$ is 0) or its
    /// leading coefficient is even, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let fs = [
    ///     UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5").unwrap(),
    ///     UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap(),
    /// ];
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 3);
    /// let rs = fs
    ///     .as_slice()
    ///     .mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, 3, &data);
    /// assert_eq!(
    ///     rs.iter().map(ToString::to_string).collect::<Vec<_>>(),
    ///     ["3*x^2+6", "5", "x^2+3*x+1"]
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_vec_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_vec_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that the polynomials may have any degrees, that the inverse of the reversal of $h$ is given
    /// by the data rather than as a polynomial, that the results are returned, and that a few
    /// polynomials with few coefficients are composed one at a time.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed(
        self,
        g: &UnsignedPolynomial<T>,
        h: &UnsignedPolynomial<T>,
        pow: u64,
        data: &ModPolynomialData<T>,
    ) -> Vec<UnsignedPolynomial<T>> {
        mod_power_of_2_compose_mod_polynomial_many_helper(self, g, h, pow, Some(data))
    }
}
