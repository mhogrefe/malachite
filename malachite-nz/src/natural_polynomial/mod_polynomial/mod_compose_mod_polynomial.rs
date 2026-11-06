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
use crate::natural_polynomial::modular::mod_div_rem_precomputed::precompute_helper;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::mem::take;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::num::modular::traits::{ModAdd, ModInverse, ModIsReduced, ModMul};
use malachite_base::polynomial::Polynomial;
use malachite_base::polynomial::mod_polynomial::traits::{
    ModComposeModPolynomial, ModComposeModPolynomialAssign,
};
use malachite_base::polynomial::modular::ModPolynomialData;
use malachite_base::polynomial::modular::traits::{ModDivRemPrecomputed, ModRemPrecomputed};
use malachite_base::unsigned_polynomial::UnsignedPolynomial;
use malachite_base::unsigned_polynomial::mod_polynomial::mod_compose_mod_polynomial::*;

// The ring of polynomials with coefficients modulo $m$, modulo a polynomial `h`, using `data`, if
// it is given, to reduce.
pub(crate) struct NaturalModRing<'a> {
    pub(crate) m: &'a Natural,
    pub(crate) h: &'a NaturalPolynomial,
    pub(crate) data: &'a ModPolynomialData<Natural>,
}

pub(crate) fn to_limbs(xs: &[Natural]) -> Vec<Limb> {
    xs.iter().map(Limb::wrapping_from).collect()
}

pub(crate) fn from_limbs(xs: &[Limb]) -> NaturalPolynomial {
    NaturalPolynomial::from_coefficients_asc(xs.iter().copied().map(Natural::from).collect())
}

pub(crate) fn padded(p: NaturalPolynomial, len: usize) -> Vec<Natural> {
    let mut xs = p.into_coefficients_asc();
    xs.resize(len, Natural::ZERO);
    xs
}

impl NaturalModRing<'_> {
    fn reduce(&self, p: NaturalPolynomial) -> NaturalPolynomial {
        if p.len() < self.h.len() {
            p
        } else {
            (&p).mod_rem_precomputed(self.h, self.m, self.data)
        }
    }

    fn product(&self, xs: &[Natural], ys: &[Natural]) -> NaturalPolynomial {
        (&NaturalPolynomial::from_coefficients_asc(xs.to_vec())).mod_mul(
            &NaturalPolynomial::from_coefficients_asc(ys.to_vec()),
            self.m,
        )
    }
}

impl ComposeModRing for NaturalModRing<'_> {
    type C = Natural;

    fn zero(&self) -> Natural {
        Natural::ZERO
    }

    fn one(&self) -> Natural {
        Natural::ONE
    }

    #[inline]
    fn add(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_add(y, self.m)
    }

    #[inline]
    fn mul(&self, x: &Natural, y: &Natural) -> Natural {
        x.mod_mul(y, self.m)
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
        b.mod_mul(&a, self.m)
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

// Checks that `g` and `h` are reduced modulo $m$, that `h` is nonzero with an invertible leading
// coefficient, unless $m$ is 1, and that `g` is shorter than `h`, and returns whether $m$ is 1.
pub(crate) fn assert_compose_mod_polynomial_inner_and_modulus_arguments(
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    m: &Natural,
) -> bool {
    assert!(
        g.mod_is_reduced(m),
        "g must be reduced mod m, but {g} has a coefficient >= {m}"
    );
    assert!(
        h.mod_is_reduced(m),
        "h must be reduced mod m, but {h} has a coefficient >= {m}"
    );
    if *m == 1u32 {
        return true;
    }
    assert_ne!(*h, NaturalPolynomial::ZERO, "division by zero");
    assert!(
        h.coefficients.last().unwrap().mod_inverse(m).is_some(),
        "the leading coefficient of h must be invertible modulo m"
    );
    assert!(
        g.len() < h.len(),
        "g must be reduced mod h, but {g} has a degree >= that of {h}"
    );
    false
}

// Checks that `f` is reduced, and the other arguments as
// `assert_compose_mod_polynomial_inner_and_modulus_arguments` does, and returns its result.
pub(crate) fn assert_compose_mod_polynomial_arguments(
    f: &NaturalPolynomial,
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    m: &Natural,
) -> bool {
    assert!(
        f.mod_is_reduced(m),
        "self must be reduced mod m, but {f} has a coefficient >= {m}"
    );
    assert_compose_mod_polynomial_inner_and_modulus_arguments(g, h, m)
}

// The composition $f(g)$ modulo $h$, with coefficients modulo $m$, reducing with `data` if it is
// given.
//
// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
// 3.6.0, or, with `data`, to `gr_poly_compose_mod_preinv` from `gr_poly/compose_mod_preinv.c`,
// except that `g` must be reduced modulo $h$.
pub(crate) fn mod_compose_mod_polynomial_helper(
    f: &NaturalPolynomial,
    g: &NaturalPolynomial,
    h: &NaturalPolynomial,
    m: &Natural,
    data: Option<&ModPolynomialData<Natural>>,
) -> NaturalPolynomial {
    if assert_compose_mod_polynomial_arguments(f, g, h, m) {
        return NaturalPolynomial::ZERO;
    }
    if m.significant_bits() <= Limb::WIDTH {
        let m_word = Limb::wrapping_from(m);
        let hs = to_limbs(&h.coefficients);
        let inverse = if let Some(data) = data {
            to_limbs(&data.inverse)
        } else {
            UnsignedPolynomial::<Limb>::precompute_mod_div_rem_data(
                &UnsignedPolynomial::from_coefficients_asc(hs.clone()),
                &m_word,
            )
            .inverse
        };
        return from_limbs(&mod_compose_mod_polynomial_vec(
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
        computed = precompute_helper(h, m);
        &computed
    };
    let n = h.coefficients.len() - 1;
    let mut gs = g.coefficients.clone();
    gs.resize(n, Natural::ZERO);
    NaturalPolynomial::from_coefficients_asc(compose_mod_dispatch(
        &NaturalModRing { m, h, data },
        &f.coefficients,
        &gs,
        n,
    ))
}

impl ModComposeModPolynomial<Self, Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by value, `g` by value, `h` by value, and the modulus by value.
    /// The inner polynomial $g$ must already be reduced modulo $h$, with degree less than that of
    /// $h$, and the coefficients of all three must be reduced modulo $m$; the leading coefficient
    /// of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(g, h.clone(), m.clone())
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(g, h, m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(self, g: Self, h: Self, m: Natural) -> Self {
        mod_compose_mod_polynomial_helper(&self, &g, &h, &m, None)
    }
}

impl ModComposeModPolynomial<Self, Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by value, `g` by value, `h` by value, and the modulus by
    /// reference. The inner polynomial $g$ must already be reduced modulo $h$, with degree less
    /// than that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(g, h.clone(), &m).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(g, h, &m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(self, g: Self, h: Self, m: &Natural) -> Self {
        mod_compose_mod_polynomial_helper(&self, &g, &h, m, None)
    }
}

impl ModComposeModPolynomial<Self, &Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by value, `g` by value, `h` by reference, and the modulus by
    /// value. The inner polynomial $g$ must already be reduced modulo $h$, with degree less than
    /// that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(g, &h, m.clone()).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(g, &h, m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(self, g: Self, h: &Self, m: Natural) -> Self {
        mod_compose_mod_polynomial_helper(&self, &g, h, &m, None)
    }
}

impl ModComposeModPolynomial<Self, &Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by value, `g` by value, `h` by reference, and the modulus by
    /// reference. The inner polynomial $g$ must already be reduced modulo $h$, with degree less
    /// than that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(g, &h, &m).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(g, &h, &m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(self, g: Self, h: &Self, m: &Natural) -> Self {
        mod_compose_mod_polynomial_helper(&self, &g, h, m, None)
    }
}

impl ModComposeModPolynomial<&Self, Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by value, `g` by reference, `h` by value, and the modulus by
    /// value. The inner polynomial $g$ must already be reduced modulo $h$, with degree less than
    /// that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(&g, h.clone(), m.clone())
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(&g, h, m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(self, g: &Self, h: Self, m: Natural) -> Self {
        mod_compose_mod_polynomial_helper(&self, g, &h, &m, None)
    }
}

impl ModComposeModPolynomial<&Self, Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by value, `g` by reference, `h` by value, and the modulus by
    /// reference. The inner polynomial $g$ must already be reduced modulo $h$, with degree less
    /// than that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(&g, h.clone(), &m).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(&g, h, &m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(self, g: &Self, h: Self, m: &Natural) -> Self {
        mod_compose_mod_polynomial_helper(&self, g, &h, m, None)
    }
}

impl ModComposeModPolynomial<&Self, &Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by value, `g` by reference, `h` by reference, and the modulus by
    /// value. The inner polynomial $g$ must already be reduced modulo $h$, with degree less than
    /// that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(&g, &h, m.clone()).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(&g, &h, m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(self, g: &Self, h: &Self, m: Natural) -> Self {
        mod_compose_mod_polynomial_helper(&self, g, h, &m, None)
    }
}

impl ModComposeModPolynomial<&Self, &Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by value, `g` by reference, `h` by reference, and the modulus by
    /// reference. The inner polynomial $g$ must already be reduced modulo $h$, with degree less
    /// than that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(&g, &h, &m).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     f.mod_compose_mod_polynomial(&g, &h, &m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(self, g: &Self, h: &Self, m: &Natural) -> Self {
        mod_compose_mod_polynomial_helper(&self, g, h, m, None)
    }
}

impl ModComposeModPolynomial<NaturalPolynomial, NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by reference, `g` by value, `h` by value, and the modulus by
    /// value. The inner polynomial $g$ must already be reduced modulo $h$, with degree less than
    /// that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(g, h.clone(), m.clone())
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(g, h, m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: NaturalPolynomial,
        h: NaturalPolynomial,
        m: Natural,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, &g, &h, &m, None)
    }
}

impl ModComposeModPolynomial<NaturalPolynomial, NaturalPolynomial, &Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by reference, `g` by value, `h` by value, and the modulus by
    /// reference. The inner polynomial $g$ must already be reduced modulo $h$, with degree less
    /// than that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(g, h.clone(), &m)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(g, h, &m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: NaturalPolynomial,
        h: NaturalPolynomial,
        m: &Natural,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, &g, &h, m, None)
    }
}

impl ModComposeModPolynomial<NaturalPolynomial, &NaturalPolynomial, Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by reference, `g` by value, `h` by reference, and the modulus by
    /// value. The inner polynomial $g$ must already be reduced modulo $h$, with degree less than
    /// that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(g, &h, m.clone())
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(g, &h, m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: NaturalPolynomial,
        h: &NaturalPolynomial,
        m: Natural,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, &g, h, &m, None)
    }
}

impl ModComposeModPolynomial<NaturalPolynomial, &NaturalPolynomial, &Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by reference, `g` by value, `h` by reference, and the modulus by
    /// reference. The inner polynomial $g$ must already be reduced modulo $h$, with degree less
    /// than that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(g, &h, &m).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(g, &h, &m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: NaturalPolynomial,
        h: &NaturalPolynomial,
        m: &Natural,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, &g, h, m, None)
    }
}

impl ModComposeModPolynomial<&NaturalPolynomial, NaturalPolynomial, Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by reference, `g` by reference, `h` by value, and the modulus by
    /// value. The inner polynomial $g$ must already be reduced modulo $h$, with degree less than
    /// that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(&g, h.clone(), m.clone())
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(&g, h, m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: &NaturalPolynomial,
        h: NaturalPolynomial,
        m: Natural,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, g, &h, &m, None)
    }
}

impl ModComposeModPolynomial<&NaturalPolynomial, NaturalPolynomial, &Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by reference, `g` by reference, `h` by value, and the modulus by
    /// reference. The inner polynomial $g$ must already be reduced modulo $h$, with degree less
    /// than that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// (f, g) \mapsto r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(&g, h.clone(), &m)
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(&g, h, &m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: &NaturalPolynomial,
        h: NaturalPolynomial,
        m: &Natural,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, g, &h, m, None)
    }
}

impl ModComposeModPolynomial<&NaturalPolynomial, &NaturalPolynomial, Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by reference, `g` by reference, `h` by reference, and the
    /// modulus by value. The inner polynomial $g$ must already be reduced modulo $h$, with degree
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
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(&g, &h, m.clone())
    ///         .to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(&g, &h, m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: &NaturalPolynomial,
        h: &NaturalPolynomial,
        m: Natural,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, g, h, &m, None)
    }
}

impl ModComposeModPolynomial<&NaturalPolynomial, &NaturalPolynomial, &Natural>
    for &NaturalPolynomial
{
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo a polynomial $h$, with coefficients modulo $m$,
    /// taking the outer polynomial by reference, `g` by reference, `h` by reference, and the
    /// modulus by reference. The inner polynomial $g$ must already be reduced modulo $h$, with
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
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomial;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(&g, &h, &m).to_string(),
    ///     "3*x^2+x+6"
    /// );
    ///
    /// let f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// assert_eq!(
    ///     (&f).mod_compose_mod_polynomial(&g, &h, &m).to_string(),
    ///     "5*x^2+4*x+3"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial(
        self,
        g: &NaturalPolynomial,
        h: &NaturalPolynomial,
        m: &Natural,
    ) -> NaturalPolynomial {
        mod_compose_mod_polynomial_helper(self, g, h, m, None)
    }
}

impl ModComposeModPolynomialAssign<Self, Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, taking `g` by value, `h` by value, and the modulus by value.
    /// The inner polynomial $g$ must already be reduced modulo $h$, with degree less than that of
    /// $h$, and the coefficients of all three must be reduced modulo $m$; the leading coefficient
    /// of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_assign(g, h.clone(), m.clone());
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_assign(g, h, m);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial_assign(&mut self, g: Self, h: Self, m: Natural) {
        *self = mod_compose_mod_polynomial_helper(&take(self), &g, &h, &m, None);
    }
}

impl ModComposeModPolynomialAssign<Self, Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, taking `g` by value, `h` by value, and the modulus by
    /// reference. The inner polynomial $g$ must already be reduced modulo $h$, with degree less
    /// than that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_assign(g, h.clone(), &m);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_assign(g, h, &m);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial_assign(&mut self, g: Self, h: Self, m: &Natural) {
        *self = mod_compose_mod_polynomial_helper(&take(self), &g, &h, m, None);
    }
}

impl ModComposeModPolynomialAssign<Self, &Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, taking `g` by value, `h` by reference, and the modulus by
    /// value. The inner polynomial $g$ must already be reduced modulo $h$, with degree less than
    /// that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_assign(g, &h, m.clone());
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_assign(g, &h, m);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial_assign(&mut self, g: Self, h: &Self, m: Natural) {
        *self = mod_compose_mod_polynomial_helper(&take(self), &g, h, &m, None);
    }
}

impl ModComposeModPolynomialAssign<Self, &Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, taking `g` by value, `h` by reference, and the modulus by
    /// reference. The inner polynomial $g$ must already be reduced modulo $h$, with degree less
    /// than that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_assign(g, &h, &m);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_assign(g, &h, &m);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial_assign(&mut self, g: Self, h: &Self, m: &Natural) {
        *self = mod_compose_mod_polynomial_helper(&take(self), &g, h, m, None);
    }
}

impl ModComposeModPolynomialAssign<&Self, Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, taking `g` by reference, `h` by value, and the modulus by
    /// value. The inner polynomial $g$ must already be reduced modulo $h$, with degree less than
    /// that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_assign(&g, h.clone(), m.clone());
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_assign(&g, h, m);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial_assign(&mut self, g: &Self, h: Self, m: Natural) {
        *self = mod_compose_mod_polynomial_helper(&take(self), g, &h, &m, None);
    }
}

impl ModComposeModPolynomialAssign<&Self, Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, taking `g` by reference, `h` by value, and the modulus by
    /// reference. The inner polynomial $g$ must already be reduced modulo $h$, with degree less
    /// than that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_assign(&g, h.clone(), &m);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_assign(&g, h, &m);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial_assign(&mut self, g: &Self, h: Self, m: &Natural) {
        *self = mod_compose_mod_polynomial_helper(&take(self), g, &h, m, None);
    }
}

impl ModComposeModPolynomialAssign<&Self, &Self, Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, taking `g` by reference, `h` by reference, and the modulus by
    /// value. The inner polynomial $g$ must already be reduced modulo $h$, with degree less than
    /// that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_assign(&g, &h, m.clone());
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_assign(&g, &h, m);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial_assign(&mut self, g: &Self, h: &Self, m: Natural) {
        *self = mod_compose_mod_polynomial_helper(&take(self), g, h, &m, None);
    }
}

impl ModComposeModPolynomialAssign<&Self, &Self, &Natural> for NaturalPolynomial {
    /// Replaces a [`NaturalPolynomial`] with its composition with another modulo a polynomial $h$,
    /// with coefficients modulo $m$, taking `g` by reference, `h` by reference, and the modulus by
    /// reference. The inner polynomial $g$ must already be reduced modulo $h$, with degree less
    /// than that of $h$, and the coefficients of all three must be reduced modulo $m$; the leading
    /// coefficient of $h$ must be invertible modulo $m$. The outer polynomial may have any degree.
    ///
    /// $$
    /// f \gets r, \quad f(g) \equiv hq + r \pmod{m}, \quad \deg r < \deg h.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// An outer polynomial with few coefficients is composed by Horner's rule, and a longer one by
    /// the Brent–Kung algorithm, which computes about $\sqrt{l}$ powers of $g$ modulo $h$, where
    /// $l$ is the length of the outer polynomial, composes all the blocks of that many coefficients
    /// with $g$ at once, with one matrix product, and combines the blocks by evaluation at the next
    /// power of $g$. Every reduction modulo $h$ uses the inverse of the reversal of $h$ as a power
    /// series, computed once. When $m$ fits in a limb, the work is done with word-sized
    /// coefficients, as for [`UnsignedPolynomial`].
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
    /// `m`, or if `h` is 0 (unless $m$ is 1) or its leading coefficient is not invertible modulo
    /// `m`, or if `g` is not shorter than `h`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::mod_polynomial::traits::ModComposeModPolynomialAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let h = NaturalPolynomial::from_str("x^3+2*x^2+1").unwrap();
    /// let m = Natural::from(7u32);
    ///
    /// let mut f = NaturalPolynomial::from_str("3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x+1").unwrap();
    /// f.mod_compose_mod_polynomial_assign(&g, &h, &m);
    /// assert_eq!(f.to_string(), "3*x^2+x+6");
    ///
    /// let mut f = NaturalPolynomial::from_str("5*x^4+4*x^3+3*x^2+2*x+1").unwrap();
    /// let g = NaturalPolynomial::from_str("x^2+2").unwrap();
    /// f.mod_compose_mod_polynomial_assign(&g, &h, &m);
    /// assert_eq!(f.to_string(), "5*x^2+4*x+3");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_compose_mod` from `fmpz_mod_poly/compose_mod.c`, FLINT
    /// 3.6.0, except that `g` must be reduced modulo $h$.
    #[inline]
    fn mod_compose_mod_polynomial_assign(&mut self, g: &Self, h: &Self, m: &Natural) {
        *self = mod_compose_mod_polynomial_helper(&take(self), g, h, m, None);
    }
}
