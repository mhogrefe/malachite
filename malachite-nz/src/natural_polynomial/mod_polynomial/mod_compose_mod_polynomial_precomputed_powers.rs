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

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;
use crate::natural_polynomial::modular::mod_div_rem_precomputed::precompute_helper;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::take;
use malachite_base::num::arithmetic::traits::FloorSqrt;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::ModIsReduced;
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::mod_polynomial::ModComposeModPolynomialData;
use malachite_base::polynomial::mod_polynomial::traits::{
    ModComposeModPolynomialPrecomputedPowers, ModComposeModPolynomialPrecomputedPowersAssign,
};
use malachite_base::polynomial::modular::ModPolynomialData;
use malachite_base::polynomial::modular::traits::ModDivRemPrecomputed;
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;

// The powers of `g` modulo `h` that Brent–Kung composition with `g` needs, and the data for
// dividing by `h`.
//
// This is equivalent to `nmod_poly_precompute_matrix` from
// `nmod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, together with the inverse of
// the reversal of `h`, except that `g` must be reduced modulo `h`.
pub(crate) fn precompute_powers_helper(
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    m: &Natural,
) -> ModComposeModPolynomialData<Natural> {
    if assert_compose_mod_polynomial_arguments(&NaturalPolynomial::ZERO, g, h, m) || h.len() == 1 {
        return ModComposeModPolynomialData {
            powers: Vec::new(),
            division: ModPolynomialData {
                inverse: Vec::new(),
            },
        };
    }
    if m.significant_bits() <= Limb::WIDTH {
        let m_word = Limb::wrapping_from(m);
        let hs = to_limbs(&h.coefficients);
        let inverse = UnsignedPolynomial::<Limb>::precompute_mod_div_rem_data(
            &UnsignedPolynomial::from_coefficients_asc(hs.clone()),
            &m_word,
        )
        .inverse;
        let powers = mod_compose_mod_polynomial_powers_vec(
            &to_limbs(&g.coefficients),
            &hs,
            m_word,
            &inverse,
        );
        return ModComposeModPolynomialData {
            powers: powers
                .into_iter()
                .map(|p| p.into_iter().map(Natural::from).collect())
                .collect(),
            division: ModPolynomialData {
                inverse: inverse.into_iter().map(Natural::from).collect(),
            },
        };
    }
    let division = precompute_helper(h, m);
    let n = h.coefficients.len() - 1;
    let mut gs = g.coefficients.clone();
    gs.resize(n, Natural::ZERO);
    let powers = precompute_matrix(
        &NaturalModRing {
            m,
            h,
            data: &division,
        },
        &gs,
        n,
    );
    ModComposeModPolynomialData { powers, division }
}

// The composition $f(g)$ modulo $h$, where `data` holds the powers of `g`.
//
// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv` from
// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `f` may have
// any degree.
pub(crate) fn mod_compose_mod_polynomial_precomputed_powers_helper(
    f: &NaturalPolynomial,
    h: &NaturalPolynomial,
    m: &Natural,
    data: &ModComposeModPolynomialData<Natural>,
) -> NaturalPolynomial {
    assert!(
        f.mod_is_reduced(m),
        "self must be reduced mod m, but {f} has a coefficient >= {m}"
    );
    assert!(
        h.mod_is_reduced(m),
        "h must be reduced mod m, but {h} has a coefficient >= {m}"
    );
    if *m == 1u32 {
        return NaturalPolynomial::ZERO;
    }
    assert_ne!(*h, NaturalPolynomial::ZERO, "division by zero");
    let n = h.coefficients.len() - 1;
    if n == 0 {
        return NaturalPolynomial::ZERO;
    }
    assert!(
        data.powers.len() == n.floor_sqrt() + 1 && data.powers.iter().all(|p| p.len() == n),
        "data was not precomputed for h"
    );
    if m.significant_bits() <= Limb::WIDTH {
        let powers: Vec<Vec<Limb>> = data.powers.iter().map(|p| to_limbs(p)).collect();
        return from_limbs(&mod_compose_mod_polynomial_with_powers_vec(
            &to_limbs(&f.coefficients),
            &to_limbs(&h.coefficients),
            Limb::wrapping_from(m),
            &powers,
            &to_limbs(&data.division.inverse),
        ));
    }
    NaturalPolynomial::from_coefficients_asc(compose_mod_brent_kung_with_powers(
        &NaturalModRing {
            m,
            h,
            data: &data.division,
        },
        &f.coefficients,
        &data.powers,
        n,
    ))
}

impl ModComposeModPolynomialPrecomputedPowers<Self, Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModComposeModPolynomialData<Natural>;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $m$, for composing
    /// [`NaturalPolynomial`]s with `g` modulo `h` by [`ModComposeModPolynomialPrecomputedPowers`]
    /// or [`ModComposeModPolynomialPrecomputedPowersAssign`]. `g` must already be reduced modulo
    /// `h`, the coefficients of both must be reduced modulo $m$, and the leading coefficient of `h`
    /// must be invertible modulo $m$.
    ///
    /// The data is the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg
    /// h}\rfloor + 1$, and the inverse of the reversal of $h$ as a power series.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \cdot m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^{3/2}m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `g` or `h` is greater than or equal to `m`, or
    /// if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or if
    /// `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h, m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_precompute_matrix` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, together with the
    /// inverse of the reversal of `h`, except that `g` must be reduced modulo $h$.
    #[inline]
    fn precompute_mod_compose_mod_polynomial_powers_data(
        g: &Self,
        h: &Self,
        m: &Natural,
    ) -> ModComposeModPolynomialData<Natural> {
        precompute_powers_helper(g, h, m)
    }

    /// Composes a [`NaturalPolynomial`] with the inner polynomial $g$ that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $m$, taking the outer polynomial by
    /// value, `h` by value, and the modulus by value. The coefficients of the outer polynomial and
    /// of $h$ must be reduced modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_compose_mod_polynomial_powers_data`. It
    /// holds the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor +
    /// 1$, so the blocks of $s$ coefficients of the outer polynomial are composed with $g$ all at
    /// once, with one matrix product, and are then combined by evaluation at $g^s$. When $m$ fits
    /// in a limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((ln + l\sqrt{n} \log n)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((l\sqrt{n} + n)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `h` is greater than or equal to `m`,
    /// or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`. If
    /// `data` was not precomputed for `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h, m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `self` may
    /// have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_powers(
        self,
        h: Self,
        m: Natural,
        data: &ModComposeModPolynomialData<Natural>,
    ) -> Self {
        mod_compose_mod_polynomial_precomputed_powers_helper(&self, &h, &m, data)
    }
}

impl ModComposeModPolynomialPrecomputedPowers<Self, &Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModComposeModPolynomialData<Natural>;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $m$, for composing
    /// [`NaturalPolynomial`]s with `g` modulo `h` by [`ModComposeModPolynomialPrecomputedPowers`]
    /// or [`ModComposeModPolynomialPrecomputedPowersAssign`]. `g` must already be reduced modulo
    /// `h`, the coefficients of both must be reduced modulo $m$, and the leading coefficient of `h`
    /// must be invertible modulo $m$.
    ///
    /// The data is the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg
    /// h}\rfloor + 1$, and the inverse of the reversal of $h$ as a power series.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \cdot m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^{3/2}m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `g` or `h` is greater than or equal to `m`, or
    /// if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or if
    /// `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h, m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_precompute_matrix` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, together with the
    /// inverse of the reversal of `h`, except that `g` must be reduced modulo $h$.
    #[inline]
    fn precompute_mod_compose_mod_polynomial_powers_data(
        g: &Self,
        h: &Self,
        m: &&Natural,
    ) -> ModComposeModPolynomialData<Natural> {
        precompute_powers_helper(g, h, m)
    }

    /// Composes a [`NaturalPolynomial`] with the inner polynomial $g$ that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $m$, taking the outer polynomial by
    /// value, `h` by value, and the modulus by reference. The coefficients of the outer polynomial
    /// and of $h$ must be reduced modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_compose_mod_polynomial_powers_data`. It
    /// holds the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor +
    /// 1$, so the blocks of $s$ coefficients of the outer polynomial are composed with $g$ all at
    /// once, with one matrix product, and are then combined by evaluation at $g^s$. When $m$ fits
    /// in a limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((ln + l\sqrt{n} \log n)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((l\sqrt{n} + n)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `h` is greater than or equal to `m`,
    /// or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`. If
    /// `data` was not precomputed for `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h.clone(), &m, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h, &m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `self` may
    /// have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_powers(
        self,
        h: Self,
        m: &Natural,
        data: &ModComposeModPolynomialData<Natural>,
    ) -> Self {
        mod_compose_mod_polynomial_precomputed_powers_helper(&self, &h, m, data)
    }
}

impl ModComposeModPolynomialPrecomputedPowers<&Self, Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModComposeModPolynomialData<Natural>;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $m$, for composing
    /// [`NaturalPolynomial`]s with `g` modulo `h` by [`ModComposeModPolynomialPrecomputedPowers`]
    /// or [`ModComposeModPolynomialPrecomputedPowersAssign`]. `g` must already be reduced modulo
    /// `h`, the coefficients of both must be reduced modulo $m$, and the leading coefficient of `h`
    /// must be invertible modulo $m$.
    ///
    /// The data is the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg
    /// h}\rfloor + 1$, and the inverse of the reversal of $h$ as a power series.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \cdot m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^{3/2}m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `g` or `h` is greater than or equal to `m`, or
    /// if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or if
    /// `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h, m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_precompute_matrix` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, together with the
    /// inverse of the reversal of `h`, except that `g` must be reduced modulo $h$.
    #[inline]
    fn precompute_mod_compose_mod_polynomial_powers_data(
        g: &&Self,
        h: &&Self,
        m: &Natural,
    ) -> ModComposeModPolynomialData<Natural> {
        precompute_powers_helper(g, h, m)
    }

    /// Composes a [`NaturalPolynomial`] with the inner polynomial $g$ that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $m$, taking the outer polynomial by
    /// value, `h` by reference, and the modulus by value. The coefficients of the outer polynomial
    /// and of $h$ must be reduced modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_compose_mod_polynomial_powers_data`. It
    /// holds the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor +
    /// 1$, so the blocks of $s$ coefficients of the outer polynomial are composed with $g$ all at
    /// once, with one matrix product, and are then combined by evaluation at $g^s$. When $m$ fits
    /// in a limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((ln + l\sqrt{n} \log n)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((l\sqrt{n} + n)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `h` is greater than or equal to `m`,
    /// or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`. If
    /// `data` was not precomputed for `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(&h, m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(&h, m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `self` may
    /// have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_powers(
        self,
        h: &Self,
        m: Natural,
        data: &ModComposeModPolynomialData<Natural>,
    ) -> Self {
        mod_compose_mod_polynomial_precomputed_powers_helper(&self, h, &m, data)
    }
}

impl ModComposeModPolynomialPrecomputedPowers<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;
    type Data = ModComposeModPolynomialData<Natural>;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $m$, for composing
    /// [`NaturalPolynomial`]s with `g` modulo `h` by [`ModComposeModPolynomialPrecomputedPowers`]
    /// or [`ModComposeModPolynomialPrecomputedPowersAssign`]. `g` must already be reduced modulo
    /// `h`, the coefficients of both must be reduced modulo $m$, and the leading coefficient of `h`
    /// must be invertible modulo $m$.
    ///
    /// The data is the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg
    /// h}\rfloor + 1$, and the inverse of the reversal of $h$ as a power series.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \cdot m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^{3/2}m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `g` or `h` is greater than or equal to `m`, or
    /// if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or if
    /// `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h, m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_precompute_matrix` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, together with the
    /// inverse of the reversal of `h`, except that `g` must be reduced modulo $h$.
    #[inline]
    fn precompute_mod_compose_mod_polynomial_powers_data(
        g: &&Self,
        h: &&Self,
        m: &&Natural,
    ) -> ModComposeModPolynomialData<Natural> {
        precompute_powers_helper(g, h, m)
    }

    /// Composes a [`NaturalPolynomial`] with the inner polynomial $g$ that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $m$, taking the outer polynomial by
    /// value, `h` by reference, and the modulus by reference. The coefficients of the outer
    /// polynomial and of $h$ must be reduced modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_compose_mod_polynomial_powers_data`. It
    /// holds the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor +
    /// 1$, so the blocks of $s$ coefficients of the outer polynomial are composed with $g$ all at
    /// once, with one matrix product, and are then combined by evaluation at $g^s$. When $m$ fits
    /// in a limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((ln + l\sqrt{n} \log n)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((l\sqrt{n} + n)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `h` is greater than or equal to `m`,
    /// or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`. If
    /// `data` was not precomputed for `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(&h, &m, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(&h, &m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `self` may
    /// have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_powers(
        self,
        h: &Self,
        m: &Natural,
        data: &ModComposeModPolynomialData<Natural>,
    ) -> Self {
        mod_compose_mod_polynomial_precomputed_powers_helper(&self, h, m, data)
    }
}

impl ModComposeModPolynomialPrecomputedPowers<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Data = ModComposeModPolynomialData<Natural>;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $m$, for composing
    /// [`NaturalPolynomial`]s with `g` modulo `h` by [`ModComposeModPolynomialPrecomputedPowers`]
    /// or [`ModComposeModPolynomialPrecomputedPowersAssign`]. `g` must already be reduced modulo
    /// `h`, the coefficients of both must be reduced modulo $m$, and the leading coefficient of `h`
    /// must be invertible modulo $m$.
    ///
    /// The data is the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg
    /// h}\rfloor + 1$, and the inverse of the reversal of $h$ as a power series.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \cdot m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^{3/2}m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `g` or `h` is greater than or equal to `m`, or
    /// if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or if
    /// `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h, m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_precompute_matrix` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, together with the
    /// inverse of the reversal of `h`, except that `g` must be reduced modulo $h$.
    #[inline]
    fn precompute_mod_compose_mod_polynomial_powers_data(
        g: &NaturalPolynomial,
        h: &NaturalPolynomial,
        m: &Natural,
    ) -> ModComposeModPolynomialData<Natural> {
        precompute_powers_helper(g, h, m)
    }

    /// Composes a [`NaturalPolynomial`] with the inner polynomial $g$ that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $m$, taking the outer polynomial by
    /// reference, `h` by value, and the modulus by value. The coefficients of the outer polynomial
    /// and of $h$ must be reduced modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_compose_mod_polynomial_powers_data`. It
    /// holds the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor +
    /// 1$, so the blocks of $s$ coefficients of the outer polynomial are composed with $g$ all at
    /// once, with one matrix product, and are then combined by evaluation at $g^s$. When $m$ fits
    /// in a limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((ln + l\sqrt{n} \log n)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((l\sqrt{n} + n)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `h` is greater than or equal to `m`,
    /// or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`. If
    /// `data` was not precomputed for `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed_powers(h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed_powers(h, m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `self` may
    /// have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_powers(
        self,
        h: NaturalPolynomial,
        m: Natural,
        data: &ModComposeModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_precomputed_powers_helper(self, &h, &m, data)
    }
}

impl ModComposeModPolynomialPrecomputedPowers<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Data = ModComposeModPolynomialData<Natural>;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $m$, for composing
    /// [`NaturalPolynomial`]s with `g` modulo `h` by [`ModComposeModPolynomialPrecomputedPowers`]
    /// or [`ModComposeModPolynomialPrecomputedPowersAssign`]. `g` must already be reduced modulo
    /// `h`, the coefficients of both must be reduced modulo $m$, and the leading coefficient of `h`
    /// must be invertible modulo $m$.
    ///
    /// The data is the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg
    /// h}\rfloor + 1$, and the inverse of the reversal of $h$ as a power series.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \cdot m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^{3/2}m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `g` or `h` is greater than or equal to `m`, or
    /// if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or if
    /// `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h, m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_precompute_matrix` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, together with the
    /// inverse of the reversal of `h`, except that `g` must be reduced modulo $h$.
    #[inline]
    fn precompute_mod_compose_mod_polynomial_powers_data(
        g: &NaturalPolynomial,
        h: &NaturalPolynomial,
        m: &&Natural,
    ) -> ModComposeModPolynomialData<Natural> {
        precompute_powers_helper(g, h, m)
    }

    /// Composes a [`NaturalPolynomial`] with the inner polynomial $g$ that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $m$, taking the outer polynomial by
    /// reference, `h` by value, and the modulus by reference. The coefficients of the outer
    /// polynomial and of $h$ must be reduced modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_compose_mod_polynomial_powers_data`. It
    /// holds the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor +
    /// 1$, so the blocks of $s$ coefficients of the outer polynomial are composed with $g$ all at
    /// once, with one matrix product, and are then combined by evaluation at $g^s$. When $m$ fits
    /// in a limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((ln + l\sqrt{n} \log n)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((l\sqrt{n} + n)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `h` is greater than or equal to `m`,
    /// or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`. If
    /// `data` was not precomputed for `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed_powers(h.clone(), &m, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed_powers(h, &m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `self` may
    /// have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_powers(
        self,
        h: NaturalPolynomial,
        m: &Natural,
        data: &ModComposeModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_precomputed_powers_helper(self, &h, m, data)
    }
}

impl ModComposeModPolynomialPrecomputedPowers<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Data = ModComposeModPolynomialData<Natural>;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $m$, for composing
    /// [`NaturalPolynomial`]s with `g` modulo `h` by [`ModComposeModPolynomialPrecomputedPowers`]
    /// or [`ModComposeModPolynomialPrecomputedPowersAssign`]. `g` must already be reduced modulo
    /// `h`, the coefficients of both must be reduced modulo $m$, and the leading coefficient of `h`
    /// must be invertible modulo $m$.
    ///
    /// The data is the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg
    /// h}\rfloor + 1$, and the inverse of the reversal of $h$ as a power series.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \cdot m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^{3/2}m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `g` or `h` is greater than or equal to `m`, or
    /// if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or if
    /// `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h, m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_precompute_matrix` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, together with the
    /// inverse of the reversal of `h`, except that `g` must be reduced modulo $h$.
    #[inline]
    fn precompute_mod_compose_mod_polynomial_powers_data(
        g: &&NaturalPolynomial,
        h: &&NaturalPolynomial,
        m: &Natural,
    ) -> ModComposeModPolynomialData<Natural> {
        precompute_powers_helper(g, h, m)
    }

    /// Composes a [`NaturalPolynomial`] with the inner polynomial $g$ that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $m$, taking the outer polynomial by
    /// reference, `h` by reference, and the modulus by value. The coefficients of the outer
    /// polynomial and of $h$ must be reduced modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_compose_mod_polynomial_powers_data`. It
    /// holds the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor +
    /// 1$, so the blocks of $s$ coefficients of the outer polynomial are composed with $g$ all at
    /// once, with one matrix product, and are then combined by evaluation at $g^s$. When $m$ fits
    /// in a limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((ln + l\sqrt{n} \log n)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((l\sqrt{n} + n)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `h` is greater than or equal to `m`,
    /// or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`. If
    /// `data` was not precomputed for `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed_powers(&h, m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed_powers(&h, m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `self` may
    /// have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_powers(
        self,
        h: &NaturalPolynomial,
        m: Natural,
        data: &ModComposeModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_precomputed_powers_helper(self, h, &m, data)
    }
}

impl ModComposeModPolynomialPrecomputedPowers<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;
    type Data = ModComposeModPolynomialData<Natural>;

    /// Precomputes the powers of `g` modulo `h`, with coefficients modulo $m$, for composing
    /// [`NaturalPolynomial`]s with `g` modulo `h` by [`ModComposeModPolynomialPrecomputedPowers`]
    /// or [`ModComposeModPolynomialPrecomputedPowersAssign`]. `g` must already be reduced modulo
    /// `h`, the coefficients of both must be reduced modulo $m$, and the leading coefficient of `h`
    /// must be invertible modulo $m$.
    ///
    /// The data is the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg
    /// h}\rfloor + 1$, and the inverse of the reversal of $h$ as a power series.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \cdot m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^{3/2}m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `g` or `h` is greater than or equal to `m`, or
    /// if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`, or if
    /// `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h.clone(), m.clone(), &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial_precomputed_powers(h, m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_precompute_matrix` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, together with the
    /// inverse of the reversal of `h`, except that `g` must be reduced modulo $h$.
    #[inline]
    fn precompute_mod_compose_mod_polynomial_powers_data(
        g: &&NaturalPolynomial,
        h: &&NaturalPolynomial,
        m: &&Natural,
    ) -> ModComposeModPolynomialData<Natural> {
        precompute_powers_helper(g, h, m)
    }

    /// Composes a [`NaturalPolynomial`] with the inner polynomial $g$ that `data` was precomputed
    /// from, modulo a polynomial $h$, with coefficients modulo $m$, taking the outer polynomial by
    /// reference, `h` by reference, and the modulus by reference. The coefficients of the outer
    /// polynomial and of $h$ must be reduced modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_compose_mod_polynomial_powers_data`. It
    /// holds the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor +
    /// 1$, so the blocks of $s$ coefficients of the outer polynomial are composed with $g$ all at
    /// once, with one matrix product, and are then combined by evaluation at $g^s$. When $m$ fits
    /// in a limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((ln + l\sqrt{n} \log n)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((l\sqrt{n} + n)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `h` is greater than or equal to `m`,
    /// or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`. If
    /// `data` was not precomputed for `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed_powers(&h, &m, &data)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial_precomputed_powers(&h, &m, &data)
    ///         .to_string(),
    ///     "3*x^2+1"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `self` may
    /// have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_powers(
        self,
        h: &NaturalPolynomial,
        m: &Natural,
        data: &ModComposeModPolynomialData<Natural>,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_precomputed_powers_helper(self, h, m, data)
    }
}

impl ModComposeModPolynomialPrecomputedPowersAssign<Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with the inner polynomial $g$ that
    /// `data` was precomputed from, modulo a polynomial $h$, with coefficients modulo $m$, taking
    /// `h` by value, and the modulus by value. The coefficients of the outer polynomial and of $h$
    /// must be reduced modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_compose_mod_polynomial_powers_data`. It
    /// holds the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor +
    /// 1$, so the blocks of $s$ coefficients of the outer polynomial are composed with $g$ all at
    /// once, with one matrix product, and are then combined by evaluation at $g^s$. When $m$ fits
    /// in a limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((ln + l\sqrt{n} \log n)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((l\sqrt{n} + n)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `h` is greater than or equal to `m`,
    /// or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`. If
    /// `data` was not precomputed for `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::{
    ///     ModComposeModPolynomialPrecomputedPowers,
    ///     ModComposeModPolynomialPrecomputedPowersAssign,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_powers_assign(h.clone(), m.clone(), &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_powers_assign(h, m, &data);
    /// assert_eq!(f.to_string(), "3*x^2+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `self` may
    /// have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_powers_assign(
        &mut self,
        h: Self,
        m: Natural,
        data: &ModComposeModPolynomialData<Natural>,
    ) {
        *self = mod_compose_mod_polynomial_precomputed_powers_helper(&take(self), &h, &m, data);
    }
}

impl ModComposeModPolynomialPrecomputedPowersAssign<Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with the inner polynomial $g$ that
    /// `data` was precomputed from, modulo a polynomial $h$, with coefficients modulo $m$, taking
    /// `h` by value, and the modulus by reference. The coefficients of the outer polynomial and of
    /// $h$ must be reduced modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_compose_mod_polynomial_powers_data`. It
    /// holds the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor +
    /// 1$, so the blocks of $s$ coefficients of the outer polynomial are composed with $g$ all at
    /// once, with one matrix product, and are then combined by evaluation at $g^s$. When $m$ fits
    /// in a limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((ln + l\sqrt{n} \log n)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((l\sqrt{n} + n)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `h` is greater than or equal to `m`,
    /// or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`. If
    /// `data` was not precomputed for `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::{
    ///     ModComposeModPolynomialPrecomputedPowers,
    ///     ModComposeModPolynomialPrecomputedPowersAssign,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_powers_assign(h.clone(), &m, &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_powers_assign(h, &m, &data);
    /// assert_eq!(f.to_string(), "3*x^2+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `self` may
    /// have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_powers_assign(
        &mut self,
        h: Self,
        m: &Natural,
        data: &ModComposeModPolynomialData<Natural>,
    ) {
        *self = mod_compose_mod_polynomial_precomputed_powers_helper(&take(self), &h, m, data);
    }
}

impl ModComposeModPolynomialPrecomputedPowersAssign<&Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with the inner polynomial $g$ that
    /// `data` was precomputed from, modulo a polynomial $h$, with coefficients modulo $m$, taking
    /// `h` by reference, and the modulus by value. The coefficients of the outer polynomial and of
    /// $h$ must be reduced modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_compose_mod_polynomial_powers_data`. It
    /// holds the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor +
    /// 1$, so the blocks of $s$ coefficients of the outer polynomial are composed with $g$ all at
    /// once, with one matrix product, and are then combined by evaluation at $g^s$. When $m$ fits
    /// in a limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((ln + l\sqrt{n} \log n)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((l\sqrt{n} + n)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `h` is greater than or equal to `m`,
    /// or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`. If
    /// `data` was not precomputed for `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::{
    ///     ModComposeModPolynomialPrecomputedPowers,
    ///     ModComposeModPolynomialPrecomputedPowersAssign,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_powers_assign(&h, m.clone(), &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_powers_assign(&h, m, &data);
    /// assert_eq!(f.to_string(), "3*x^2+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `self` may
    /// have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_powers_assign(
        &mut self,
        h: &Self,
        m: Natural,
        data: &ModComposeModPolynomialData<Natural>,
    ) {
        *self = mod_compose_mod_polynomial_precomputed_powers_helper(&take(self), h, &m, data);
    }
}

impl ModComposeModPolynomialPrecomputedPowersAssign<&Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with the inner polynomial $g$ that
    /// `data` was precomputed from, modulo a polynomial $h$, with coefficients modulo $m$, taking
    /// `h` by reference, and the modulus by reference. The coefficients of the outer polynomial and
    /// of $h$ must be reduced modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The data should be obtained with `precompute_mod_compose_mod_polynomial_powers_data`. It
    /// holds the powers $g^0, \ldots, g^{s-1}$ modulo $h$, where $s = \lfloor\sqrt{\deg h}\rfloor +
    /// 1$, so the blocks of $s$ coefficients of the outer polynomial are composed with $g$ all at
    /// once, with one matrix product, and are then combined by evaluation at $g^s$. When $m$ fits
    /// in a limb, the work is done with word-sized coefficients, as for [`UnsignedPolynomial`].
    ///
    /// # Worst-case complexity
    /// $T(n, l, m) = O((ln + l\sqrt{n} \log n)m \log m \log\log m)$
    ///
    /// $M(n, l, m) = O((l\sqrt{n} + n)m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `h.len()`, $l$ is `self.len()`, and $m$
    /// is `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `h` is greater than or equal to `m`,
    /// or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo `m`. If
    /// `data` was not precomputed for `h`, the result is meaningless, and the function may panic.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::{
    ///     ModComposeModPolynomialPrecomputedPowers,
    ///     ModComposeModPolynomialPrecomputedPowersAssign,
    /// };
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// let data = NaturalPolynomial::precompute_mod_compose_mod_polynomial_powers_data(&g, &h, &m);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_powers_assign(&h, &m, &data);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// f.mod_compose_mod_polynomial_precomputed_powers_assign(&h, &m, &data);
    /// assert_eq!(f.to_string(), "3*x^2+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod_brent_kung_precomp_preinv` from
    /// `fmpz_mod_poly/compose_mod_brent_kung_precomp_preinv.c`, FLINT 3.6.0, except that `self` may
    /// have any degree.
    #[inline]
    fn mod_compose_mod_polynomial_precomputed_powers_assign(
        &mut self,
        h: &Self,
        m: &Natural,
        data: &ModComposeModPolynomialData<Natural>,
    ) {
        *self = mod_compose_mod_polynomial_precomputed_powers_helper(&take(self), h, m, data);
    }
}
