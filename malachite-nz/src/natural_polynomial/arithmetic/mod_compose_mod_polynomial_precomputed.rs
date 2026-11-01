// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_compose_mod_polynomial::*;
use core::mem::take;
use malachite_base::polynomial::{
    ModComposeModPolynomialPrecomputed, ModComposeModPolynomialPrecomputedAssign, ModPolynomialData,
};

impl ModComposeModPolynomialPrecomputed<Self, Self, Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by value, `g` by value,
    /// `h` by value, and the modulus by value. The inner polynomial $g$ must already be reduced
    /// modulo $h$, with degree less than that of $h$, and the coefficients of all three must be
    /// reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$. The outer
    /// polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(g, h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(g, h, m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: Self,
        h: Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        mod_compose_mod_polynomial_helper(&self, &g, &h, &m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<Self, Self, &Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by value, `g` by value,
    /// `h` by value, and the modulus by reference. The inner polynomial $g$ must already be reduced
    /// modulo $h$, with degree less than that of $h$, and the coefficients of all three must be
    /// reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$. The outer
    /// polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(g, h.clone(), &m, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(g, h, &m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: Self,
        h: Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        mod_compose_mod_polynomial_helper(&self, &g, &h, m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<Self, &Self, Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by value, `g` by value,
    /// `h` by reference, and the modulus by value. The inner polynomial $g$ must already be reduced
    /// modulo $h$, with degree less than that of $h$, and the coefficients of all three must be
    /// reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$. The outer
    /// polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(g, &h, m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(g, &h, m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: Self,
        h: &Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        mod_compose_mod_polynomial_helper(&self, &g, h, &m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<Self, &Self, &Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by value, `g` by value,
    /// `h` by reference, and the modulus by reference. The inner polynomial $g$ must already be
    /// reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(g, &h, &m, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(g, &h, &m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: Self,
        h: &Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        mod_compose_mod_polynomial_helper(&self, &g, h, m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<&Self, Self, Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by value, `g` by
    /// reference, `h` by value, and the modulus by value. The inner polynomial $g$ must already be
    /// reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(&g, h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(&g, h, m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: &Self,
        h: Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        mod_compose_mod_polynomial_helper(&self, g, &h, &m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<&Self, Self, &Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by value, `g` by
    /// reference, `h` by value, and the modulus by reference. The inner polynomial $g$ must already
    /// be reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(&g, h.clone(), &m, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(&g, h, &m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: &Self,
        h: Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        mod_compose_mod_polynomial_helper(&self, g, &h, m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<&Self, &Self, Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by value, `g` by
    /// reference, `h` by reference, and the modulus by value. The inner polynomial $g$ must already
    /// be reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(&g, &h, m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(&g, &h, m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: &Self,
        h: &Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        mod_compose_mod_polynomial_helper(&self, g, h, &m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<&Self, &Self, &Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by value, `g` by
    /// reference, `h` by reference, and the modulus by reference. The inner polynomial $g$ must
    /// already be reduced modulo $h$, with degree less than that of $h$, and the coefficients of
    /// all three must be reduced modulo $m$; the leading coefficient of $h$ must be invertible
    /// modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(&g, &h, &m, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed(&g, &h, &m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: &Self,
        h: &Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> Self {
        mod_compose_mod_polynomial_helper(&self, g, h, m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<NaturalPolynomial, NaturalPolynomial, Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by reference, `g` by
    /// value, `h` by value, and the modulus by value. The inner polynomial $g$ must already be
    /// reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(g, h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(g, h, m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: NaturalPolynomial,
        h: NaturalPolynomial,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, &g, &h, &m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<NaturalPolynomial, NaturalPolynomial, &Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by reference, `g` by
    /// value, `h` by value, and the modulus by reference. The inner polynomial $g$ must already be
    /// reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(g, h.clone(), &m, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(g, h, &m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: NaturalPolynomial,
        h: NaturalPolynomial,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, &g, &h, m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<NaturalPolynomial, &NaturalPolynomial, Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by reference, `g` by
    /// value, `h` by reference, and the modulus by value. The inner polynomial $g$ must already be
    /// reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(g, &h, m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(g, &h, m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: NaturalPolynomial,
        h: &NaturalPolynomial,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, &g, h, &m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<NaturalPolynomial, &NaturalPolynomial, &Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by reference, `g` by
    /// value, `h` by reference, and the modulus by reference. The inner polynomial $g$ must already
    /// be reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(g, &h, &m, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(g, &h, &m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: NaturalPolynomial,
        h: &NaturalPolynomial,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, &g, h, m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<&NaturalPolynomial, NaturalPolynomial, Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by reference, `g` by
    /// reference, `h` by value, and the modulus by value. The inner polynomial $g$ must already be
    /// reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(&g, h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(&g, h, m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: &NaturalPolynomial,
        h: NaturalPolynomial,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, g, &h, &m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<&NaturalPolynomial, NaturalPolynomial, &Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by reference, `g` by
    /// reference, `h` by value, and the modulus by reference. The inner polynomial $g$ must already
    /// be reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(&g, h.clone(), &m, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(&g, h, &m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: &NaturalPolynomial,
        h: NaturalPolynomial,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, g, &h, m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<&NaturalPolynomial, &NaturalPolynomial, Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by reference, `g` by
    /// reference, `h` by reference, and the modulus by value. The inner polynomial $g$ must already
    /// be reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(&g, &h, m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(&g, &h, m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: &NaturalPolynomial,
        h: &NaturalPolynomial,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, g, h, &m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputed<&NaturalPolynomial, &NaturalPolynomial, &Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;
    type Data = ModPolynomialData<Natural>;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// using data precomputed from $h$ and $m$, taking the outer polynomial by reference, `g` by
    /// reference, `h` by reference, and the modulus by reference. The inner polynomial $g$ must
    /// already be reduced modulo $h$, with degree less than that of $h$, and the coefficients of
    /// all three must be reduced modulo $m$; the leading coefficient of $h$ must be invertible
    /// modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputed, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(&g, &h, &m, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed(&g, &h, &m, &data)
    ///         .to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed(
        self,
        g: &NaturalPolynomial,
        h: &NaturalPolynomial,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, g, h, m, Some(data))
    }
}

impl ModComposeModPolynomialPrecomputedAssign<Self, Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, using data precomputed from $h$ and $m$, taking `g` by value,
    /// `h` by value, and the modulus by value. The inner polynomial $g$ must already be reduced
    /// modulo $h$, with degree less than that of $h$, and the coefficients of all three must be
    /// reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$. The outer
    /// polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputedAssign, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(g, h.clone(), m.clone(), &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(g, h, m, &data);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: Self,
        h: Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = mod_compose_mod_polynomial_helper(&take(self), &g, &h, &m, Some(data));
    }
}

impl ModComposeModPolynomialPrecomputedAssign<Self, Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, using data precomputed from $h$ and $m$, taking `g` by value,
    /// `h` by value, and the modulus by reference. The inner polynomial $g$ must already be reduced
    /// modulo $h$, with degree less than that of $h$, and the coefficients of all three must be
    /// reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$. The outer
    /// polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputedAssign, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(g, h.clone(), &m, &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(g, h, &m, &data);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: Self,
        h: Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = mod_compose_mod_polynomial_helper(&take(self), &g, &h, m, Some(data));
    }
}

impl ModComposeModPolynomialPrecomputedAssign<Self, &Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, using data precomputed from $h$ and $m$, taking `g` by value,
    /// `h` by reference, and the modulus by value. The inner polynomial $g$ must already be reduced
    /// modulo $h$, with degree less than that of $h$, and the coefficients of all three must be
    /// reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$. The outer
    /// polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputedAssign, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(g, &h, m.clone(), &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(g, &h, m, &data);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: Self,
        h: &Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = mod_compose_mod_polynomial_helper(&take(self), &g, h, &m, Some(data));
    }
}

impl ModComposeModPolynomialPrecomputedAssign<Self, &Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, using data precomputed from $h$ and $m$, taking `g` by value,
    /// `h` by reference, and the modulus by reference. The inner polynomial $g$ must already be
    /// reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputedAssign, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(g, &h, &m, &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(g, &h, &m, &data);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: Self,
        h: &Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = mod_compose_mod_polynomial_helper(&take(self), &g, h, m, Some(data));
    }
}

impl ModComposeModPolynomialPrecomputedAssign<&Self, Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, using data precomputed from $h$ and $m$, taking `g` by
    /// reference, `h` by value, and the modulus by value. The inner polynomial $g$ must already be
    /// reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputedAssign, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(&g, h.clone(), m.clone(), &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(&g, h, m, &data);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: &Self,
        h: Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = mod_compose_mod_polynomial_helper(&take(self), g, &h, &m, Some(data));
    }
}

impl ModComposeModPolynomialPrecomputedAssign<&Self, Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, using data precomputed from $h$ and $m$, taking `g` by
    /// reference, `h` by value, and the modulus by reference. The inner polynomial $g$ must already
    /// be reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputedAssign, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(&g, h.clone(), &m, &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(&g, h, &m, &data);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: &Self,
        h: Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = mod_compose_mod_polynomial_helper(&take(self), g, &h, m, Some(data));
    }
}

impl ModComposeModPolynomialPrecomputedAssign<&Self, &Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, using data precomputed from $h$ and $m$, taking `g` by
    /// reference, `h` by reference, and the modulus by value. The inner polynomial $g$ must already
    /// be reduced modulo $h$, with degree less than that of $h$, and the coefficients of all three
    /// must be reduced modulo $m$; the leading coefficient of $h$ must be invertible modulo $m$.
    /// The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputedAssign, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(&g, &h, m.clone(), &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(&g, &h, m, &data);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: &Self,
        h: &Self,
        m: Natural,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = mod_compose_mod_polynomial_helper(&take(self), g, h, &m, Some(data));
    }
}

impl ModComposeModPolynomialPrecomputedAssign<&Self, &Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, using data precomputed from $h$ and $m$, taking `g` by
    /// reference, `h` by reference, and the modulus by reference. The inner polynomial $g$ must
    /// already be reduced modulo $h$, with degree less than that of $h$, and the coefficients of
    /// all three must be reduced modulo $m$; the leading coefficient of $h$ must be invertible
    /// modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with [`precompute_mod_div_rem_data`](
    /// malachite_base::polynomial::ModDivRemPrecomputed::precompute_mod_div_rem_data), as for
    /// division. An outer polynomial with few coefficients is composed by Horner's rule, and a
    /// longer one by the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$
    /// modulo $h$, where $l$ is the length of the outer polynomial, composes all the blocks of that
    /// many coefficients with $g$ at once, with one matrix product, and combines the blocks by
    /// evaluation at the next power of $g$. Every reduction modulo $h$ uses the precomputed
    /// inverse. When $m$ fits in a limb, the work is done with word-sized coefficients, as for
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial).
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((n\sqrt{l} \log n + ln)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((n\sqrt{l} + l)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self`, `g`, or `h` is greater than or equal to
    /// `m`, or if `h` is 0 (unless $m$ is 1), or if `g` is not shorter than `h`. If `data` was not
    /// precomputed from `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::{
    ///     ModComposeModPolynomialPrecomputedAssign, ModDivRemPrecomputed,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let data = NaturalPolynomial::precompute_mod_div_rem_data(&h, &m);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(&g, &h, &m, &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_assign(&g, &h, &m, &data);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_preinv.c`, FLINT 3.6.0, except that `g` must be
    /// reduced modulo $h$ and `self` may have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_assign(
        &mut self,
        g: &Self,
        h: &Self,
        m: &Natural,
        data: &ModPolynomialData<Natural>,
    ) {
        *self = mod_compose_mod_polynomial_helper(&take(self), g, h, m, Some(data));
    }
}
