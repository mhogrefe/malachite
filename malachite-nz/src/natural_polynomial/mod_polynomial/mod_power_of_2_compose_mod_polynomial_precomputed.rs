// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::mod_polynomial::mod_power_of_2_compose_mod_polynomial::*;
use core::mem::take;
use malachite_base::polynomial::mod_polynomial::traits::{
    ModPowerOf2ComposeModPolynomialPrecomputed, ModPowerOf2ComposeModPolynomialPrecomputedAssign,
};
use malachite_base::polynomial::modular::ModPolynomialData;
#[cfg(doc)]
use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;

impl ModPowerOf2ComposeModPolynomialPrecomputed<Self, Self> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// using data precomputed from $h$ and $k$, taking the outer polynomial by value, `g` by value,
    /// and `h` by value. The inner polynomial $g$ must already be reduced modulo $h$, with degree
    /// less than that of $h$, and the coefficients of all three must be reduced modulo $2^k$; the
    /// leading coefficient of $h$ must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`], as for division. An outer polynomial with few coefficients
    /// is composed by Horner's rule, and a longer one by the Brent–Kung algorithm, which computes
    /// about $\sqrt{l}$ powers of $g$ modulo $h$, where $l$ is the length of the outer polynomial,
    /// composes all the blocks of that many coefficients with $g$ at once, with one matrix product,
    /// and combines the blocks by evaluation at the next power of $g$. Every reduction modulo $h$
    /// uses the precomputed inverse. When $k$ is no greater than the width of a limb, the work is
    /// done with word-sized coefficients, as for [`UnsignedPolynomial`](
    /// malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, k) = O((n\sqrt{l} \log n + ln)k \log k \log\log k)$
    ///
    /// $M(n, l, k) = O((n\sqrt{l} + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $k$
    /// is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `g`, or `h` is greater than or equal to $2^k$, or if
    /// `h` is 0 (unless $k$ is 0), or if `g` is not shorter than `h`. If `data` was not precomputed
    /// from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 8);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed(g, h.clone(), 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed(g, h, 8, &data)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that `g` must be reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed(
        self,
        g: Self,
        h: Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        mod_power_of_2_compose_mod_polynomial_helper(&self, &g, &h, pow, Some(data))
    }
}

impl ModPowerOf2ComposeModPolynomialPrecomputed<Self, &Self> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// using data precomputed from $h$ and $k$, taking the outer polynomial by value, `g` by value,
    /// and `h` by reference. The inner polynomial $g$ must already be reduced modulo $h$, with
    /// degree less than that of $h$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $h$ must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`], as for division. An outer polynomial with few coefficients
    /// is composed by Horner's rule, and a longer one by the Brent–Kung algorithm, which computes
    /// about $\sqrt{l}$ powers of $g$ modulo $h$, where $l$ is the length of the outer polynomial,
    /// composes all the blocks of that many coefficients with $g$ at once, with one matrix product,
    /// and combines the blocks by evaluation at the next power of $g$. Every reduction modulo $h$
    /// uses the precomputed inverse. When $k$ is no greater than the width of a limb, the work is
    /// done with word-sized coefficients, as for [`UnsignedPolynomial`](
    /// malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, k) = O((n\sqrt{l} \log n + ln)k \log k \log\log k)$
    ///
    /// $M(n, l, k) = O((n\sqrt{l} + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $k$
    /// is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `g`, or `h` is greater than or equal to $2^k$, or if
    /// `h` is 0 (unless $k$ is 0), or if `g` is not shorter than `h`. If `data` was not precomputed
    /// from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 8);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed(g, &h, 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed(g, &h, 8, &data)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that `g` must be reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed(
        self,
        g: Self,
        h: &Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        mod_power_of_2_compose_mod_polynomial_helper(&self, &g, h, pow, Some(data))
    }
}

impl ModPowerOf2ComposeModPolynomialPrecomputed<&Self, Self> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// using data precomputed from $h$ and $k$, taking the outer polynomial by value, `g` by
    /// reference, and `h` by value. The inner polynomial $g$ must already be reduced modulo $h$,
    /// with degree less than that of $h$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $h$ must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`], as for division. An outer polynomial with few coefficients
    /// is composed by Horner's rule, and a longer one by the Brent–Kung algorithm, which computes
    /// about $\sqrt{l}$ powers of $g$ modulo $h$, where $l$ is the length of the outer polynomial,
    /// composes all the blocks of that many coefficients with $g$ at once, with one matrix product,
    /// and combines the blocks by evaluation at the next power of $g$. Every reduction modulo $h$
    /// uses the precomputed inverse. When $k$ is no greater than the width of a limb, the work is
    /// done with word-sized coefficients, as for [`UnsignedPolynomial`](
    /// malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, k) = O((n\sqrt{l} \log n + ln)k \log k \log\log k)$
    ///
    /// $M(n, l, k) = O((n\sqrt{l} + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $k$
    /// is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `g`, or `h` is greater than or equal to $2^k$, or if
    /// `h` is 0 (unless $k$ is 0), or if `g` is not shorter than `h`. If `data` was not precomputed
    /// from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 8);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed(&g, h.clone(), 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed(&g, h, 8, &data)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that `g` must be reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed(
        self,
        g: &Self,
        h: Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        mod_power_of_2_compose_mod_polynomial_helper(&self, g, &h, pow, Some(data))
    }
}

impl ModPowerOf2ComposeModPolynomialPrecomputed<&Self, &Self> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// using data precomputed from $h$ and $k$, taking the outer polynomial by value, `g` by
    /// reference, and `h` by reference. The inner polynomial $g$ must already be reduced modulo
    /// $h$, with degree less than that of $h$, and the coefficients of all three must be reduced
    /// modulo $2^k$; the leading coefficient of $h$ must be odd. The outer polynomial may have any
    /// degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`], as for division. An outer polynomial with few coefficients
    /// is composed by Horner's rule, and a longer one by the Brent–Kung algorithm, which computes
    /// about $\sqrt{l}$ powers of $g$ modulo $h$, where $l$ is the length of the outer polynomial,
    /// composes all the blocks of that many coefficients with $g$ at once, with one matrix product,
    /// and combines the blocks by evaluation at the next power of $g$. Every reduction modulo $h$
    /// uses the precomputed inverse. When $k$ is no greater than the width of a limb, the work is
    /// done with word-sized coefficients, as for [`UnsignedPolynomial`](
    /// malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, k) = O((n\sqrt{l} \log n + ln)k \log k \log\log k)$
    ///
    /// $M(n, l, k) = O((n\sqrt{l} + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $k$
    /// is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `g`, or `h` is greater than or equal to $2^k$, or if
    /// `h` is 0 (unless $k$ is 0), or if `g` is not shorter than `h`. If `data` was not precomputed
    /// from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 8);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, 8, &data)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that `g` must be reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed(
        self,
        g: &Self,
        h: &Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        mod_power_of_2_compose_mod_polynomial_helper(&self, g, h, pow, Some(data))
    }
}

impl ModPowerOf2ComposeModPolynomialPrecomputed<NaturalPolynomial, NaturalPolynomial>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// using data precomputed from $h$ and $k$, taking the outer polynomial by reference, `g` by
    /// value, and `h` by value. The inner polynomial $g$ must already be reduced modulo $h$, with
    /// degree less than that of $h$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $h$ must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`], as for division. An outer polynomial with few coefficients
    /// is composed by Horner's rule, and a longer one by the Brent–Kung algorithm, which computes
    /// about $\sqrt{l}$ powers of $g$ modulo $h$, where $l$ is the length of the outer polynomial,
    /// composes all the blocks of that many coefficients with $g$ at once, with one matrix product,
    /// and combines the blocks by evaluation at the next power of $g$. Every reduction modulo $h$
    /// uses the precomputed inverse. When $k$ is no greater than the width of a limb, the work is
    /// done with word-sized coefficients, as for [`UnsignedPolynomial`](
    /// malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, k) = O((n\sqrt{l} \log n + ln)k \log k \log\log k)$
    ///
    /// $M(n, l, k) = O((n\sqrt{l} + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $k$
    /// is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `g`, or `h` is greater than or equal to $2^k$, or if
    /// `h` is 0 (unless $k$ is 0), or if `g` is not shorter than `h`. If `data` was not precomputed
    /// from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 8);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial_precomputed(g, h.clone(), 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial_precomputed(g, h, 8, &data)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that `g` must be reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed(
        self,
        g: NaturalPolynomial,
        h: NaturalPolynomial,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_power_of_2_compose_mod_polynomial_helper(self, &g, &h, pow, Some(data))
    }
}

impl ModPowerOf2ComposeModPolynomialPrecomputed<NaturalPolynomial, &NaturalPolynomial>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// using data precomputed from $h$ and $k$, taking the outer polynomial by reference, `g` by
    /// value, and `h` by reference. The inner polynomial $g$ must already be reduced modulo $h$,
    /// with degree less than that of $h$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $h$ must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`], as for division. An outer polynomial with few coefficients
    /// is composed by Horner's rule, and a longer one by the Brent–Kung algorithm, which computes
    /// about $\sqrt{l}$ powers of $g$ modulo $h$, where $l$ is the length of the outer polynomial,
    /// composes all the blocks of that many coefficients with $g$ at once, with one matrix product,
    /// and combines the blocks by evaluation at the next power of $g$. Every reduction modulo $h$
    /// uses the precomputed inverse. When $k$ is no greater than the width of a limb, the work is
    /// done with word-sized coefficients, as for [`UnsignedPolynomial`](
    /// malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, k) = O((n\sqrt{l} \log n + ln)k \log k \log\log k)$
    ///
    /// $M(n, l, k) = O((n\sqrt{l} + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $k$
    /// is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `g`, or `h` is greater than or equal to $2^k$, or if
    /// `h` is 0 (unless $k$ is 0), or if `g` is not shorter than `h`. If `data` was not precomputed
    /// from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 8);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial_precomputed(g, &h, 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial_precomputed(g, &h, 8, &data)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that `g` must be reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed(
        self,
        g: NaturalPolynomial,
        h: &NaturalPolynomial,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_power_of_2_compose_mod_polynomial_helper(self, &g, h, pow, Some(data))
    }
}

impl ModPowerOf2ComposeModPolynomialPrecomputed<&NaturalPolynomial, NaturalPolynomial>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// using data precomputed from $h$ and $k$, taking the outer polynomial by reference, `g` by
    /// reference, and `h` by value. The inner polynomial $g$ must already be reduced modulo $h$,
    /// with degree less than that of $h$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $h$ must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`], as for division. An outer polynomial with few coefficients
    /// is composed by Horner's rule, and a longer one by the Brent–Kung algorithm, which computes
    /// about $\sqrt{l}$ powers of $g$ modulo $h$, where $l$ is the length of the outer polynomial,
    /// composes all the blocks of that many coefficients with $g$ at once, with one matrix product,
    /// and combines the blocks by evaluation at the next power of $g$. Every reduction modulo $h$
    /// uses the precomputed inverse. When $k$ is no greater than the width of a limb, the work is
    /// done with word-sized coefficients, as for [`UnsignedPolynomial`](
    /// malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, k) = O((n\sqrt{l} \log n + ln)k \log k \log\log k)$
    ///
    /// $M(n, l, k) = O((n\sqrt{l} + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $k$
    /// is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `g`, or `h` is greater than or equal to $2^k$, or if
    /// `h` is 0 (unless $k$ is 0), or if `g` is not shorter than `h`. If `data` was not precomputed
    /// from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 8);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial_precomputed(&g, h.clone(), 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial_precomputed(&g, h, 8, &data)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that `g` must be reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed(
        self,
        g: &NaturalPolynomial,
        h: NaturalPolynomial,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_power_of_2_compose_mod_polynomial_helper(self, g, &h, pow, Some(data))
    }
}

impl ModPowerOf2ComposeModPolynomialPrecomputed<&NaturalPolynomial, &NaturalPolynomial>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// using data precomputed from $h$ and $k$, taking the outer polynomial by reference, `g` by
    /// reference, and `h` by reference. The inner polynomial $g$ must already be reduced modulo
    /// $h$, with degree less than that of $h$, and the coefficients of all three must be reduced
    /// modulo $2^k$; the leading coefficient of $h$ must be odd. The outer polynomial may have any
    /// degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`], as for division. An outer polynomial with few coefficients
    /// is composed by Horner's rule, and a longer one by the Brent–Kung algorithm, which computes
    /// about $\sqrt{l}$ powers of $g$ modulo $h$, where $l$ is the length of the outer polynomial,
    /// composes all the blocks of that many coefficients with $g$ at once, with one matrix product,
    /// and combines the blocks by evaluation at the next power of $g$. Every reduction modulo $h$
    /// uses the precomputed inverse. When $k$ is no greater than the width of a limb, the work is
    /// done with word-sized coefficients, as for [`UnsignedPolynomial`](
    /// malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, k) = O((n\sqrt{l} \log n + ln)k \log k \log\log k)$
    ///
    /// $M(n, l, k) = O((n\sqrt{l} + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $k$
    /// is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `g`, or `h` is greater than or equal to $2^k$, or if
    /// `h` is 0 (unless $k$ is 0), or if `g` is not shorter than `h`. If `data` was not precomputed
    /// from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 8);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, 8, &data)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial_precomputed(&g, &h, 8, &data)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that `g` must be reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed(
        self,
        g: &NaturalPolynomial,
        h: &NaturalPolynomial,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_power_of_2_compose_mod_polynomial_helper(self, g, h, pow, Some(data))
    }
}

impl ModPowerOf2ComposeModPolynomialPrecomputedAssign<Self, Self> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $2^k$, using data precomputed from $h$ and $k$, taking `g` by
    /// value, and `h` by value. The inner polynomial $g$ must already be reduced modulo $h$, with
    /// degree less than that of $h$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $h$ must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`], as for division. An outer polynomial with few coefficients
    /// is composed by Horner's rule, and a longer one by the Brent–Kung algorithm, which computes
    /// about $\sqrt{l}$ powers of $g$ modulo $h$, where $l$ is the length of the outer polynomial,
    /// composes all the blocks of that many coefficients with $g$ at once, with one matrix product,
    /// and combines the blocks by evaluation at the next power of $g$. Every reduction modulo $h$
    /// uses the precomputed inverse. When $k$ is no greater than the width of a limb, the work is
    /// done with word-sized coefficients, as for [`UnsignedPolynomial`](
    /// malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, k) = O((n\sqrt{l} \log n + ln)k \log k \log\log k)$
    ///
    /// $M(n, l, k) = O((n\sqrt{l} + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $k$
    /// is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `g`, or `h` is greater than or equal to $2^k$, or if
    /// `h` is 0 (unless $k$ is 0), or if `g` is not shorter than `h`. If `data` was not precomputed
    /// from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 8);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_precomputed_assign(g, h.clone(), 8, &data);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_precomputed_assign(g, h, 8, &data);
    /// assert_eq!(f.to_string(), "127*x^2+89*x+15");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that `g` must be reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: Self,
        h: Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = mod_power_of_2_compose_mod_polynomial_helper(&take(self), &g, &h, pow, Some(data));
    }
}

impl ModPowerOf2ComposeModPolynomialPrecomputedAssign<Self, &Self> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $2^k$, using data precomputed from $h$ and $k$, taking `g` by
    /// value, and `h` by reference. The inner polynomial $g$ must already be reduced modulo $h$,
    /// with degree less than that of $h$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $h$ must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`], as for division. An outer polynomial with few coefficients
    /// is composed by Horner's rule, and a longer one by the Brent–Kung algorithm, which computes
    /// about $\sqrt{l}$ powers of $g$ modulo $h$, where $l$ is the length of the outer polynomial,
    /// composes all the blocks of that many coefficients with $g$ at once, with one matrix product,
    /// and combines the blocks by evaluation at the next power of $g$. Every reduction modulo $h$
    /// uses the precomputed inverse. When $k$ is no greater than the width of a limb, the work is
    /// done with word-sized coefficients, as for [`UnsignedPolynomial`](
    /// malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, k) = O((n\sqrt{l} \log n + ln)k \log k \log\log k)$
    ///
    /// $M(n, l, k) = O((n\sqrt{l} + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $k$
    /// is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `g`, or `h` is greater than or equal to $2^k$, or if
    /// `h` is 0 (unless $k$ is 0), or if `g` is not shorter than `h`. If `data` was not precomputed
    /// from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 8);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_precomputed_assign(g, &h, 8, &data);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_precomputed_assign(g, &h, 8, &data);
    /// assert_eq!(f.to_string(), "127*x^2+89*x+15");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that `g` must be reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: Self,
        h: &Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = mod_power_of_2_compose_mod_polynomial_helper(&take(self), &g, h, pow, Some(data));
    }
}

impl ModPowerOf2ComposeModPolynomialPrecomputedAssign<&Self, Self> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $2^k$, using data precomputed from $h$ and $k$, taking `g` by
    /// reference, and `h` by value. The inner polynomial $g$ must already be reduced modulo $h$,
    /// with degree less than that of $h$, and the coefficients of all three must be reduced modulo
    /// $2^k$; the leading coefficient of $h$ must be odd. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`], as for division. An outer polynomial with few coefficients
    /// is composed by Horner's rule, and a longer one by the Brent–Kung algorithm, which computes
    /// about $\sqrt{l}$ powers of $g$ modulo $h$, where $l$ is the length of the outer polynomial,
    /// composes all the blocks of that many coefficients with $g$ at once, with one matrix product,
    /// and combines the blocks by evaluation at the next power of $g$. Every reduction modulo $h$
    /// uses the precomputed inverse. When $k$ is no greater than the width of a limb, the work is
    /// done with word-sized coefficients, as for [`UnsignedPolynomial`](
    /// malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, k) = O((n\sqrt{l} \log n + ln)k \log k \log\log k)$
    ///
    /// $M(n, l, k) = O((n\sqrt{l} + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $k$
    /// is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `g`, or `h` is greater than or equal to $2^k$, or if
    /// `h` is 0 (unless $k$ is 0), or if `g` is not shorter than `h`. If `data` was not precomputed
    /// from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 8);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_precomputed_assign(&g, h.clone(), 8, &data);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_precomputed_assign(&g, h, 8, &data);
    /// assert_eq!(f.to_string(), "127*x^2+89*x+15");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that `g` must be reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: &Self,
        h: Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = mod_power_of_2_compose_mod_polynomial_helper(&take(self), g, &h, pow, Some(data));
    }
}

impl ModPowerOf2ComposeModPolynomialPrecomputedAssign<&Self, &Self> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $2^k$, using data precomputed from $h$ and $k$, taking `g` by
    /// reference, and `h` by reference. The inner polynomial $g$ must already be reduced modulo
    /// $h$, with degree less than that of $h$, and the coefficients of all three must be reduced
    /// modulo $2^k$; the leading coefficient of $h$ must be odd. The outer polynomial may have any
    /// degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{2^k}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_power_of_2_div_rem_data` of
    /// [`ModPowerOf2DivRemPrecomputed`], as for division. An outer polynomial with few coefficients
    /// is composed by Horner's rule, and a longer one by the Brent–Kung algorithm, which computes
    /// about $\sqrt{l}$ powers of $g$ modulo $h$, where $l$ is the length of the outer polynomial,
    /// composes all the blocks of that many coefficients with $g$ at once, with one matrix product,
    /// and combines the blocks by evaluation at the next power of $g$. Every reduction modulo $h$
    /// uses the precomputed inverse. When $k$ is no greater than the width of a limb, the work is
    /// done with word-sized coefficients, as for [`UnsignedPolynomial`](
    /// malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, k) = O((n\sqrt{l} \log n + ln)k \log k \log\log k)$
    ///
    /// $M(n, l, k) = O((n\sqrt{l} + l)k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $k$
    /// is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self`, `g`, or `h` is greater than or equal to $2^k$, or if
    /// `h` is 0 (unless $k$ is 0), or if `g` is not shorter than `h`. If `data` was not precomputed
    /// from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_base::polynomial::modular::traits::ModPowerOf2DivRemPrecomputed;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_power_of_2_div_rem_data(&h, 8);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_precomputed_assign(&g, &h, 8, &data);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_precomputed_assign(&g, &h, 8, &data);
    /// assert_eq!(f.to_string(), "127*x^2+89*x+15");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, with the modulus $2^k$, except
    /// that `g` must be reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: &Self,
        h: &Self,
        pow: u64,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = mod_power_of_2_compose_mod_polynomial_helper(&take(self), g, h, pow, Some(data));
    }
}
