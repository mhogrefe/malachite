// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::mod_polynomial::mod_compose_mod_polynomial::{
    from_limbs, padded, to_limbs,
};
use crate::natural_polynomial::modular::mod_power_of_2_div_rem_precomputed::precompute_helper;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::take;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::modular::traits::{ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Mul};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::mod_polynomial::traits::{
    ModPowerOf2ComposeModPolynomial, ModPowerOf2ComposeModPolynomialAssign,
};
use malachite_base::polynomial::modular::ModPolynomialData;
use malachite_base::polynomial::modular::traits::{
    ModPowerOf2DivRemPrecomputed, ModPowerOf2RemPrecomputed,
};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;
use malachite_base::unsigned_polynomial::mod_polynomial::mod_power_of_2_compose_mod_polynomial::*;

// The ring of polynomials with coefficients modulo $2^k$, modulo a polynomial `h`, using `data`, if
// it is given, to reduce.
pub(crate) struct NaturalModPowerOf2Ring<'a> {
    pub(crate) pow: u64,
    pub(crate) h: &'a NaturalPolynomial,
    pub(crate) data: &'a ModPolynomialData<Natural>,
}

impl NaturalModPowerOf2Ring<'_> {
    fn reduce(&self, p: NaturalPolynomial) -> NaturalPolynomial {
        if p.len() < self.h.len() {
            p
        } else {
            (&p).mod_power_of_2_rem_precomputed(self.h, self.pow, self.data)
        }
    }

    fn product(&self, xs: &[Natural], ys: &[Natural]) -> NaturalPolynomial {
        (&NaturalPolynomial::from_coefficients_asc(xs.to_vec())).mod_power_of_2_mul(
            &NaturalPolynomial::from_coefficients_asc(ys.to_vec()),
            self.pow,
        )
    }
}

impl ComposeModRing for NaturalModPowerOf2Ring<'_> {
    type C = Natural;

    fn zero(&self) -> Natural {
        Natural::ZERO
    }

    fn one(&self) -> Natural {
        Natural::ONE
    }

    #[inline]
    fn add(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_power_of_2_add(y, self.pow)
    }

    #[inline]
    fn mul(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_power_of_2_mul(y, self.pow)
    }

    #[inline]
    fn mul_mod(&self, xs: &[Natural], ys: &[Natural]) -> Vec<Natural> {
        padded(self.reduce(self.product(xs, ys)), xs.len())
    }

    #[inline]
    fn poly_mul(&self, xs: &[Natural], ys: &[Natural]) -> Vec<Natural> {
        padded(self.product(xs, ys), xs.len() + ys.len() - 1)
    }

    #[inline]
    fn rem(&self, xs: &[Natural]) -> Vec<Natural> {
        padded(
            self.reduce(NaturalPolynomial::from_coefficients_asc(xs.to_vec())),
            self.h.coefficients.len() - 1,
        )
    }

    fn matrix_mul(&self, b: &[Vec<Natural>], a: &[Vec<Natural>]) -> Vec<Vec<Natural>> {
        let width = a[0].len();
        let b = NaturalMatrix::from_owned_elements(
            u64::exact_from(b.len()),
            u64::exact_from(a.len()),
            b.concat(),
        );
        let a = NaturalMatrix::from_owned_elements(
            u64::exact_from(a.len()),
            u64::exact_from(width),
            a.concat(),
        );
        b.mod_power_of_2_mul(&a, self.pow)
            .into_elements()
            .chunks(width)
            .map(<[Natural]>::to_vec)
            .collect()
    }

    // Measured with a 101-bit modulus, 2026-11-01.
    fn brent_kung_cutoff(&self, n: usize) -> usize {
        match n {
            0..=5 => 7,
            6..=32 => 6,
            _ => 5,
        }
    }
}

// Checks that `f`, `g`, and `h` are reduced modulo $2^k$, that `h` is nonzero with an odd leading
// coefficient, unless $k$ is 0, and that `g` is shorter than `h`, and returns whether $k$ is 0.
pub(crate) fn assert_power_of_2_compose_mod_polynomial_arguments(
    f: &NaturalPolynomial,
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    pow: u64,
) -> bool {
    assert!(
        f.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {f} has a coefficient >= 2^{pow}"
    );
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
    assert_ne!(*h, NaturalPolynomial::ZERO, "division by zero");
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

// The composition $f(g)$ modulo $h$, with coefficients modulo $2^k$, reducing with `data` if it is
// given.
//
// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
// 3.6.0, with the modulus $2^k$, or, with `data`, to `gr_poly_compose_mod_preinv` from
// `gr_poly/compose_mod_preinv.c`, except that `g` must be reduced modulo $h$.
pub(crate) fn mod_power_of_2_compose_mod_polynomial_helper(
    f: &NaturalPolynomial,
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    pow: u64,
    data: Option<&ModPolynomialData<Natural>>,
) -> NaturalPolynomial {
    if assert_power_of_2_compose_mod_polynomial_arguments(f, g, h, pow) {
        return NaturalPolynomial::ZERO;
    }
    if pow <= Limb::WIDTH {
        let m_word = pow;
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
        return from_limbs(&mod_power_of_2_compose_mod_polynomial_vec(
            &to_limbs(&f.coefficients),
            &to_limbs(&g.coefficients),
            &hs,
            m_word,
            &inverse,
        ));
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
    NaturalPolynomial::from_coefficients_asc(compose_mod_dispatch(
        &NaturalModPowerOf2Ring { pow, h, data },
        &f.coefficients,
        &gs,
        n,
    ))
}

impl ModPowerOf2ComposeModPolynomial<Self, Self> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// taking the outer polynomial by value, `g` by value, and `h` by value. The inner polynomial
    /// $g$ must already be reduced modulo $h$, with degree less than that of $h$, and the
    /// coefficients of all three must be reduced modulo $2^k$; the leading coefficient of $h$ must
    /// be odd. The outer polynomial may have any degree.
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
    /// series, computed once. When $k$ is no greater than the width of a limb, the work is done
    /// with word-sized coefficients, as for [`UnsignedPolynomial`].
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
    /// `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not shorter than
    /// `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(g, h.clone(), 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(g, h, 8).to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(self, g: Self, h: Self, pow: u64) -> Self {
        mod_power_of_2_compose_mod_polynomial_helper(&self, &g, &h, pow, None)
    }
}

impl ModPowerOf2ComposeModPolynomial<Self, &Self> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// taking the outer polynomial by value, `g` by value, and `h` by reference. The inner
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
    /// series, computed once. When $k$ is no greater than the width of a limb, the work is done
    /// with word-sized coefficients, as for [`UnsignedPolynomial`].
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
    /// `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not shorter than
    /// `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(g, &h, 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(g, &h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(self, g: Self, h: &Self, pow: u64) -> Self {
        mod_power_of_2_compose_mod_polynomial_helper(&self, &g, h, pow, None)
    }
}

impl ModPowerOf2ComposeModPolynomial<&Self, Self> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// taking the outer polynomial by value, `g` by reference, and `h` by value. The inner
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
    /// series, computed once. When $k$ is no greater than the width of a limb, the work is done
    /// with word-sized coefficients, as for [`UnsignedPolynomial`].
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
    /// `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not shorter than
    /// `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(&g, h.clone(), 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(&g, h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(self, g: &Self, h: Self, pow: u64) -> Self {
        mod_power_of_2_compose_mod_polynomial_helper(&self, g, &h, pow, None)
    }
}

impl ModPowerOf2ComposeModPolynomial<&Self, &Self> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// taking the outer polynomial by value, `g` by reference, and `h` by reference. The inner
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
    /// series, computed once. When $k$ is no greater than the width of a limb, the work is done
    /// with word-sized coefficients, as for [`UnsignedPolynomial`].
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
    /// `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not shorter than
    /// `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(&g, &h, 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_power_of_2_compose_mod_polynomial(&g, &h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(self, g: &Self, h: &Self, pow: u64) -> Self {
        mod_power_of_2_compose_mod_polynomial_helper(&self, g, h, pow, None)
    }
}

impl ModPowerOf2ComposeModPolynomial<NaturalPolynomial, NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// taking the outer polynomial by reference, `g` by value, and `h` by value. The inner
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
    /// series, computed once. When $k$ is no greater than the width of a limb, the work is done
    /// with word-sized coefficients, as for [`UnsignedPolynomial`].
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
    /// `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not shorter than
    /// `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(g, h.clone(), 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(g, h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(
        self,
        g: NaturalPolynomial,
        h: NaturalPolynomial,
        pow: u64,
    ) -> NaturalPolynomial {
        mod_power_of_2_compose_mod_polynomial_helper(self, &g, &h, pow, None)
    }
}

impl ModPowerOf2ComposeModPolynomial<NaturalPolynomial, &NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// taking the outer polynomial by reference, `g` by value, and `h` by reference. The inner
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
    /// series, computed once. When $k$ is no greater than the width of a limb, the work is done
    /// with word-sized coefficients, as for [`UnsignedPolynomial`].
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
    /// `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not shorter than
    /// `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(g, &h, 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(g, &h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(
        self,
        g: NaturalPolynomial,
        h: &NaturalPolynomial,
        pow: u64,
    ) -> NaturalPolynomial {
        mod_power_of_2_compose_mod_polynomial_helper(self, &g, h, pow, None)
    }
}

impl ModPowerOf2ComposeModPolynomial<&NaturalPolynomial, NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// taking the outer polynomial by reference, `g` by reference, and `h` by value. The inner
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
    /// series, computed once. When $k$ is no greater than the width of a limb, the work is done
    /// with word-sized coefficients, as for [`UnsignedPolynomial`].
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
    /// `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not shorter than
    /// `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(&g, h.clone(), 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(&g, h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(
        self,
        g: &NaturalPolynomial,
        h: NaturalPolynomial,
        pow: u64,
    ) -> NaturalPolynomial {
        mod_power_of_2_compose_mod_polynomial_helper(self, g, &h, pow, None)
    }
}

impl ModPowerOf2ComposeModPolynomial<&NaturalPolynomial, &NaturalPolynomial>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $2^k$,
    /// taking the outer polynomial by reference, `g` by reference, and `h` by reference. The inner
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
    /// series, computed once. When $k$ is no greater than the width of a limb, the work is done
    /// with word-sized coefficients, as for [`UnsignedPolynomial`].
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
    /// `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not shorter than
    /// `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModPowerOf2ComposeModPolynomial;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(&g, &h, 8)
    ///         .to_string(),
    ///     "3*x^2+8*x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_power_of_2_compose_mod_polynomial(&g, &h, 8)
    ///         .to_string(),
    ///     "127*x^2+89*x+15"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial(
        self,
        g: &NaturalPolynomial,
        h: &NaturalPolynomial,
        pow: u64,
    ) -> NaturalPolynomial {
        mod_power_of_2_compose_mod_polynomial_helper(self, g, h, pow, None)
    }
}

impl ModPowerOf2ComposeModPolynomialAssign<Self, Self> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
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
    /// series, computed once. When $k$ is no greater than the width of a limb, the work is done
    /// with word-sized coefficients, as for [`UnsignedPolynomial`].
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
    /// `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not shorter than
    /// `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(g, h.clone(), 8);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(g, h, 8);
    /// assert_eq!(f.to_string(), "127*x^2+89*x+15");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_assign(&mut self, g: Self, h: Self, pow: u64) {
        *self = mod_power_of_2_compose_mod_polynomial_helper(&take(self), &g, &h, pow, None);
    }
}

impl ModPowerOf2ComposeModPolynomialAssign<Self, &Self> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
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
    /// series, computed once. When $k$ is no greater than the width of a limb, the work is done
    /// with word-sized coefficients, as for [`UnsignedPolynomial`].
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
    /// `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not shorter than
    /// `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(g, &h, 8);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(g, &h, 8);
    /// assert_eq!(f.to_string(), "127*x^2+89*x+15");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_assign(&mut self, g: Self, h: &Self, pow: u64) {
        *self = mod_power_of_2_compose_mod_polynomial_helper(&take(self), &g, h, pow, None);
    }
}

impl ModPowerOf2ComposeModPolynomialAssign<&Self, Self> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
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
    /// series, computed once. When $k$ is no greater than the width of a limb, the work is done
    /// with word-sized coefficients, as for [`UnsignedPolynomial`].
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
    /// `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not shorter than
    /// `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(&g, h.clone(), 8);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(&g, h, 8);
    /// assert_eq!(f.to_string(), "127*x^2+89*x+15");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_assign(&mut self, g: &Self, h: Self, pow: u64) {
        *self = mod_power_of_2_compose_mod_polynomial_helper(&take(self), g, &h, pow, None);
    }
}

impl ModPowerOf2ComposeModPolynomialAssign<&Self, &Self> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
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
    /// series, computed once. When $k$ is no greater than the width of a limb, the work is done
    /// with word-sized coefficients, as for [`UnsignedPolynomial`].
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
    /// `h` is 0 (unless $k$ is 0) or its leading coefficient is even, or if `g` is not shorter than
    /// `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::*;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(&g, &h, 8);
    /// assert_eq!(f.to_string(), "3*x^2+8*x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_power_of_2_compose_mod_polynomial_assign(&g, &h, 8);
    /// assert_eq!(f.to_string(), "127*x^2+89*x+15");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, with the modulus $2^k$, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_power_of_2_compose_mod_polynomial_assign(&mut self, g: &Self, h: &Self, pow: u64) {
        *self = mod_power_of_2_compose_mod_polynomial_helper(&take(self), g, h, pow, None);
    }
}
