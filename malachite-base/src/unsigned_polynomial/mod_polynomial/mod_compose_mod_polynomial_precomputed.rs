// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2013, 2014 Martin Lee
//
//      Copyright © 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::mod_polynomial::traits::{
    ModComposeModPolynomialPrecomputed, ModComposeModPolynomialPrecomputedAssign,
};
use crate::polynomial::modular::ModPolynomialData;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;
use core::mem::take;

impl<T: PrimitiveUnsigned> ModComposeModPolynomialPrecomputed<Self, Self, T>
    for UnsignedPolynomial<T>
{
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by value, `g` by value,
    /// and `h` by value. The inner polynomial $g$ must already be reduced modulo $h$, with degree
    /// less than that of $h$, and the coefficients of all three must be reduced modulo $m$; the
    /// leading coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any
    /// degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::modular::traits::ModDivRemPrecomputed::precompute_mod_div_rem_data), as
    /// for division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &7);
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(g, h.clone(), 7, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(g, h, 7, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be reduced
    /// modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: Self,
        h: Self,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> Self {
        mod_compose_mod_polynomial_helper(&self, &g, &h, m, Some(data))
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomialPrecomputed<Self, &Self, T>
    for UnsignedPolynomial<T>
{
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by value, `g` by value,
    /// and `h` by reference. The inner polynomial $g$ must already be reduced modulo $h$, with
    /// degree less than that of $h$, and the coefficients of all three must be reduced modulo $m$;
    /// the leading coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have
    /// any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::modular::traits::ModDivRemPrecomputed::precompute_mod_div_rem_data), as
    /// for division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &7);
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(g, &h, 7, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(g, &h, 7, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be reduced
    /// modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: Self,
        h: &Self,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> Self {
        mod_compose_mod_polynomial_helper(&self, &g, h, m, Some(data))
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomialPrecomputed<&Self, Self, T>
    for UnsignedPolynomial<T>
{
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by value, `g` by
    /// reference, and `h` by value. The inner polynomial $g$ must already be reduced modulo $h$,
    /// with degree less than that of $h$, and the coefficients of all three must be reduced modulo
    /// $m$; the leading coefficient of $h$ must be invertible modulo $m$. The outer polynomial may
    /// have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::modular::traits::ModDivRemPrecomputed::precompute_mod_div_rem_data), as
    /// for division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &7);
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(&g, h.clone(), 7, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(&g, h, 7, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be reduced
    /// modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: &Self,
        h: Self,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> Self {
        mod_compose_mod_polynomial_helper(&self, g, &h, m, Some(data))
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomialPrecomputed<&Self, &Self, T>
    for UnsignedPolynomial<T>
{
    type Output = Self;
    type Data = ModPolynomialData<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by value, `g` by
    /// reference, and `h` by reference. The inner polynomial $g$ must already be reduced modulo
    /// $h$, with degree less than that of $h$, and the coefficients of all three must be reduced
    /// modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$. The outer
    /// polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::modular::traits::ModDivRemPrecomputed::precompute_mod_div_rem_data), as
    /// for division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &7);
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(&g, &h, 7, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(&g, &h, 7, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be reduced
    /// modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: &Self,
        h: &Self,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> Self {
        mod_compose_mod_polynomial_helper(&self, g, h, m, Some(data))
    }
}

impl<T: PrimitiveUnsigned>
    ModComposeModPolynomialPrecomputed<UnsignedPolynomial<T>, UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by reference, `g` by
    /// value, and `h` by value. The inner polynomial $g$ must already be reduced modulo $h$, with
    /// degree less than that of $h$, and the coefficients of all three must be reduced modulo $m$;
    /// the leading coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have
    /// any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::modular::traits::ModDivRemPrecomputed::precompute_mod_div_rem_data), as
    /// for division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &7);
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(g, h.clone(), 7, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(g, h, 7, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be reduced
    /// modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: UnsignedPolynomial<T>,
        h: UnsignedPolynomial<T>,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> UnsignedPolynomial<T> {
        mod_compose_mod_polynomial_helper(self, &g, &h, m, Some(data))
    }
}

impl<T: PrimitiveUnsigned>
    ModComposeModPolynomialPrecomputed<UnsignedPolynomial<T>, &UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by reference, `g` by
    /// value, and `h` by reference. The inner polynomial $g$ must already be reduced modulo $h$,
    /// with degree less than that of $h$, and the coefficients of all three must be reduced modulo
    /// $m$; the leading coefficient of $h$ must be invertible modulo $m$. The outer polynomial may
    /// have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::modular::traits::ModDivRemPrecomputed::precompute_mod_div_rem_data), as
    /// for division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &7);
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(g, &h, 7, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(g, &h, 7, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be reduced
    /// modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: UnsignedPolynomial<T>,
        h: &UnsignedPolynomial<T>,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> UnsignedPolynomial<T> {
        mod_compose_mod_polynomial_helper(self, &g, h, m, Some(data))
    }
}

impl<T: PrimitiveUnsigned>
    ModComposeModPolynomialPrecomputed<&UnsignedPolynomial<T>, UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by reference, `g` by
    /// reference, and `h` by value. The inner polynomial $g$ must already be reduced modulo $h$,
    /// with degree less than that of $h$, and the coefficients of all three must be reduced modulo
    /// $m$; the leading coefficient of $h$ must be invertible modulo $m$. The outer polynomial may
    /// have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::modular::traits::ModDivRemPrecomputed::precompute_mod_div_rem_data), as
    /// for division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &7);
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(&g, h.clone(), 7, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(&g, h, 7, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be reduced
    /// modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: &UnsignedPolynomial<T>,
        h: UnsignedPolynomial<T>,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> UnsignedPolynomial<T> {
        mod_compose_mod_polynomial_helper(self, g, &h, m, Some(data))
    }
}

impl<T: PrimitiveUnsigned>
    ModComposeModPolynomialPrecomputed<&UnsignedPolynomial<T>, &UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;
    type Data = ModPolynomialData<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by reference, `g` by
    /// reference, and `h` by reference. The inner polynomial $g$ must already be reduced modulo
    /// $h$, with degree less than that of $h$, and the coefficients of all three must be reduced
    /// modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$. The outer
    /// polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::modular::traits::ModDivRemPrecomputed::precompute_mod_div_rem_data), as
    /// for division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialPrecomputed;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &7);
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(&g, &h, 7, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(&g, &h, 7, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be reduced
    /// modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: &UnsignedPolynomial<T>,
        h: &UnsignedPolynomial<T>,
        m: T,
        data: &ModPolynomialData<T>,
    ) -> UnsignedPolynomial<T> {
        mod_compose_mod_polynomial_helper(self, g, h, m, Some(data))
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomialPrecomputedAssign<Self, Self, T>
    for UnsignedPolynomial<T>
{
    /// Replaces a [`UnsignedPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, using data precomputed from $h$ and $m$, taking `g` by value,
    /// and `h` by value. The inner polynomial $g$ must already be reduced modulo $h$, with degree
    /// less than that of $h$, and the coefficients of all three must be reduced modulo $m$; the
    /// leading coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any
    /// degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::modular::traits::ModDivRemPrecomputed::precompute_mod_div_rem_data), as
    /// for division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &7);
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(g, h.clone(), 7, &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(g, h, 7, &data);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be reduced
    /// modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: Self,
        h: Self,
        m: T,
        data: &ModPolynomialData<T>,
    ) {
        *self = mod_compose_mod_polynomial_helper(&take(self), &g, &h, m, Some(data));
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomialPrecomputedAssign<Self, &Self, T>
    for UnsignedPolynomial<T>
{
    /// Replaces a [`UnsignedPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, using data precomputed from $h$ and $m$, taking `g` by value,
    /// and `h` by reference. The inner polynomial $g$ must already be reduced modulo $h$, with
    /// degree less than that of $h$, and the coefficients of all three must be reduced modulo $m$;
    /// the leading coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have
    /// any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::modular::traits::ModDivRemPrecomputed::precompute_mod_div_rem_data), as
    /// for division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &7);
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(g, &h, 7, &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(g, &h, 7, &data);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be reduced
    /// modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: Self,
        h: &Self,
        m: T,
        data: &ModPolynomialData<T>,
    ) {
        *self = mod_compose_mod_polynomial_helper(&take(self), &g, h, m, Some(data));
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomialPrecomputedAssign<&Self, Self, T>
    for UnsignedPolynomial<T>
{
    /// Replaces a [`UnsignedPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, using data precomputed from $h$ and $m$, taking `g` by
    /// reference, and `h` by value. The inner polynomial $g$ must already be reduced modulo $h$,
    /// with degree less than that of $h$, and the coefficients of all three must be reduced modulo
    /// $m$; the leading coefficient of $h$ must be invertible modulo $m$. The outer polynomial may
    /// have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::modular::traits::ModDivRemPrecomputed::precompute_mod_div_rem_data), as
    /// for division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &7);
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(&g, h.clone(), 7, &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(&g, h, 7, &data);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be reduced
    /// modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: &Self,
        h: Self,
        m: T,
        data: &ModPolynomialData<T>,
    ) {
        *self = mod_compose_mod_polynomial_helper(&take(self), g, &h, m, Some(data));
    }
}

impl<T: PrimitiveUnsigned> ModComposeModPolynomialPrecomputedAssign<&Self, &Self, T>
    for UnsignedPolynomial<T>
{
    /// Replaces a [`UnsignedPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, using data precomputed from $h$ and $m$, taking `g` by
    /// reference, and `h` by reference. The inner polynomial $g$ must already be reduced modulo
    /// $h$, with degree less than that of $h$, and the coefficients of all three must be reduced
    /// modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$. The outer
    /// polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// crate::polynomial::modular::traits::ModDivRemPrecomputed::precompute_mod_div_rem_data), as
    /// for division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse.
    ///
    /// # Worst-case complexity
    /// $T(n, l) = O(n\sqrt{l} \log n + ln)$
    ///
    /// $M(n, l) = O(n\sqrt{l} + l)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $l$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let h = UnsignedPolynomial::<u8>::from_str("x^3+2*x^2+1").unwrap();
    /// let data = UnsignedPolynomial::precompute_mod_div_rem_data(&h, &7);
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(&g, &h, 7, &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = UnsignedPolynomial::<u8>::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = UnsignedPolynomial::<u8>::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(&g, &h, 7, &data);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_compose_mod_brent_kung_preinv` from
    /// `nmod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be reduced
    /// modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: &Self,
        h: &Self,
        m: T,
        data: &ModPolynomialData<T>,
    ) {
        *self = mod_compose_mod_polynomial_helper(&take(self), g, h, m, Some(data));
    }
}
