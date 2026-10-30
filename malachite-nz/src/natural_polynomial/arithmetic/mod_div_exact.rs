// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
//      Copyright © 2011 William Hart
//
//      Copyright © 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_div_rem::{
    assert_reduced, checked_helper as div_rem_checked_helper, from_words, mod_quotient_with,
    to_words, trimmed,
};
use crate::natural_polynomial::arithmetic::mod_div_truncated::mod_div_truncated_dispatch;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{CoprimeWith, ModInverse};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{CheckedModDivExact, ModDivExact, ModDivExactAssign};
use malachite_base::unsigned_polynomial::arithmetic::mod_div_exact::mod_div_exact_vec;

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, assuming
// that the division is exact, where `ys` is nonempty and no longer than `xs`, `inverse` is the
// inverse of the last coefficient of `ys` modulo `m`, and `series` divides power series. The low
// zeros of `ys` are first discarded, along with as many coefficients of `xs`; then, if the lowest
// coefficient of `ys` is invertible, the low half of the quotient is found by dividing the
// polynomials as power series, and the high half from the top, as in `mod_quotient_with`, by the
// high coefficients of `xs`. Otherwise the whole quotient is found from the top.
//
// This is equivalent to `_gr_poly_divexact_bidirectional` from `gr_poly/divexact_bidirectional.c`,
// FLINT 3.6.0, with coefficients modulo `m` and `series` in place of `_gr_poly_div_series`, except
// that a lowest coefficient that is not invertible is handled.
crate_test_fn! {mod_div_exact_bidirectional(
    xs: &[Natural],
    ys: &[Natural],
    m: &Natural,
    inverse: &Natural,
    series: fn(&[Natural], &[Natural], usize, &Natural, &Natural) -> Vec<Natural>,
) -> Vec<Natural> {
    let zeros = ys.iter().take_while(|&y| *y == 0u32).count();
    let (xs, ys) = (&xs[zeros..], &ys[zeros..]);
    let q_len = xs.len() - ys.len() + 1;
    if ys.len() == 1 || q_len == 1 {
        return mod_quotient_with(xs, ys, m, inverse, series);
    }
    let Some(low_inverse) = (&ys[0]).mod_inverse(m) else {
        return mod_quotient_with(xs, ys, m, inverse, series);
    };
    let low_len = q_len - (q_len >> 1);
    let mut qs = series(xs, ys, low_len, m, &low_inverse);
    qs.extend(mod_quotient_with(&xs[low_len..], ys, m, inverse, series));
    qs
}}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, assuming
// that the division is exact, where `ys` is nonempty and no longer than `xs`, and `inverse` is the
// inverse of the last coefficient of `ys` modulo `m`.
//
// This is equivalent to `_gr_poly_divexact_generic` from `gr_poly/divexact.c`, FLINT 3.6.0, with
// coefficients modulo `m`.
fn mod_div_exact_dispatch(
    xs: &[Natural],
    ys: &[Natural],
    m: &Natural,
    inverse: &Natural,
) -> Vec<Natural> {
    if ys.len() <= 2 {
        mod_quotient_with(xs, ys, m, inverse, mod_div_truncated_dispatch)
    } else {
        mod_div_exact_bidirectional(xs, ys, m, inverse, mod_div_truncated_dispatch)
    }
}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, assuming
// that the division is exact, with `kernel` doing the work in the general case, or `None` if `ys`
// is zero or its leading coefficient is not invertible. Modulo 1, every polynomial is 0, and so is
// the result.
crate_test_fn! {mod_div_exact_with(
    xs: &[Natural],
    ys: &[Natural],
    m: &Natural,
    kernel: fn(&[Natural], &[Natural], &Natural, &Natural) -> Vec<Natural>,
) -> Option<NaturalPolynomial> {
    if *m == 1u32 {
        return Some(NaturalPolynomial::ZERO);
    }
    let inverse = ys.last()?.mod_inverse(m)?;
    if xs.len() < ys.len() {
        return Some(NaturalPolynomial::ZERO);
    }
    Some(trimmed(kernel(xs, ys, m, &inverse)))
}}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, assuming
// that the division is exact, computed in `T`, which `m` must fit in.
fn mod_div_exact_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    m: &Natural,
) -> NaturalPolynomial
where
    Natural: From<T>,
{
    from_words(mod_div_exact_vec(&to_words::<T>(xs), &to_words::<T>(ys), T::exact_from(m)).unwrap())
}

// Panics if `q` is zero, unless `m` is 1, or if its leading coefficient is not invertible modulo
// `m`.
pub(crate) fn assert_invertible(q: &NaturalPolynomial, m: &Natural) {
    if *m != 1u32 {
        assert_ne!(*q, NaturalPolynomial::ZERO, "division by zero");
        assert!(
            q.coefficients.last().unwrap().coprime_with(m),
            "the leading coefficient of the divisor must be invertible modulo m"
        );
    }
}

// The quotient modulo `m` of `p` by `q`, assuming that the division is exact. When `m` fits in one
// or two limbs, this is done in word arithmetic.
//
// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0,
// without the check that the division is exact.
fn helper(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
    assert_reduced(p, q, m);
    assert_invertible(q, m);
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    let bits = m.significant_bits();
    if bits <= Limb::WIDTH {
        mod_div_exact_word::<Limb>(xs, ys, m)
    } else if bits <= DoubleLimb::WIDTH {
        mod_div_exact_word::<DoubleLimb>(xs, ys, m)
    } else {
        mod_div_exact_with(xs, ys, m, mod_div_exact_dispatch).unwrap()
    }
}

// The quotient modulo `m` of `p` by `q`, or `None` if `q` does not divide `p`.
//
// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0, except
// that Malachite panics where FLINT assumes the leading coefficient invertible.
fn checked_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<NaturalPolynomial> {
    assert_reduced(p, q, m);
    assert_invertible(q, m);
    let (q, r) = div_rem_checked_helper(p, q, m).unwrap();
    if r == NaturalPolynomial::ZERO {
        Some(q)
    } else {
        None
    }
}

impl ModDivExact<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, assuming that the second divides the first,
    /// taking the dividend by value, the divisor by value, and the modulus by value. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_div_exact(q, Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, except that Malachite panics where FLINT
    /// aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact(self, other: Self, m: Natural) -> Self {
        helper(&self, &other, &m)
    }
}

impl ModDivExact<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, assuming that the second divides the first,
    /// taking the dividend by value, the divisor by value, and the modulus by reference. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_div_exact(q, &Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, except that Malachite panics where FLINT
    /// aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact(self, other: Self, m: &Natural) -> Self {
        helper(&self, &other, m)
    }
}

impl ModDivExact<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, assuming that the second divides the first,
    /// taking the dividend by value, the divisor by reference, and the modulus by value. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_div_exact(&q, Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, except that Malachite panics where FLINT
    /// aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact(self, other: &Self, m: Natural) -> Self {
        helper(&self, other, &m)
    }
}

impl ModDivExact<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, assuming that the second divides the first,
    /// taking the dividend by value, the divisor by reference, and the modulus by reference. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     p.mod_div_exact(&q, &Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, except that Malachite panics where FLINT
    /// aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact(self, other: &Self, m: &Natural) -> Self {
        helper(&self, other, m)
    }
}

impl ModDivExact<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, assuming that the second divides the first,
    /// taking the dividend by reference, the divisor by value, and the modulus by value. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_div_exact(q, Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, except that Malachite panics where FLINT
    /// aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact(self, other: NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        helper(self, &other, &m)
    }
}

impl ModDivExact<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, assuming that the second divides the first,
    /// taking the dividend by reference, the divisor by value, and the modulus by reference. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_div_exact(q, &Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, except that Malachite panics where FLINT
    /// aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact(self, other: NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        helper(self, &other, m)
    }
}

impl ModDivExact<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, assuming that the second divides the first,
    /// taking the dividend by reference, the divisor by reference, and the modulus by value. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_div_exact(&q, Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, except that Malachite panics where FLINT
    /// aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact(self, other: &NaturalPolynomial, m: Natural) -> NaturalPolynomial {
        helper(self, other, &m)
    }
}

impl ModDivExact<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, assuming that the second divides the first,
    /// taking the dividend by reference, the divisor by reference, and the modulus by reference.
    /// The coefficients of both must already be reduced modulo $m$, and the leading coefficient of
    /// the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// assert_eq!(
    ///     (&p).mod_div_exact(&q, &Natural::from(7u32)).to_string(),
    ///     "5*x^3+4*x^2+4*x+2"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, except that Malachite panics where FLINT
    /// aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact(self, other: &NaturalPolynomial, m: &Natural) -> NaturalPolynomial {
        helper(self, other, m)
    }
}

impl ModDivExactAssign<Self, Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, assuming that the second
    /// divides the first, taking the divisor by value, and the modulus by value. The coefficients
    /// of both must already be reduced modulo $m$, and the leading coefficient of the divisor must
    /// be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExactAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// p.mod_div_exact_assign(q, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, except that Malachite panics where FLINT
    /// aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact_assign(&mut self, other: Self, m: Natural) {
        *self = helper(self, &other, &m);
    }
}

impl ModDivExactAssign<Self, &Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, assuming that the second
    /// divides the first, taking the divisor by value, and the modulus by reference. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExactAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// p.mod_div_exact_assign(q, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, except that Malachite panics where FLINT
    /// aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact_assign(&mut self, other: Self, m: &Natural) {
        *self = helper(self, &other, m);
    }
}

impl ModDivExactAssign<&Self, Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, assuming that the second
    /// divides the first, taking the divisor by reference, and the modulus by value. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExactAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// p.mod_div_exact_assign(&q, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, except that Malachite panics where FLINT
    /// aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact_assign(&mut self, other: &Self, m: Natural) {
        *self = helper(self, other, &m);
    }
}

impl ModDivExactAssign<&Self, &Natural> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, assuming that the second
    /// divides the first, taking the divisor by reference, and the modulus by reference. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq \pmod{m}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $m$ is 1, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// invertible, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModDivExactAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// p.mod_div_exact_assign(&q, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, except that Malachite panics where FLINT
    /// aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_exact_assign(&mut self, other: &Self, m: &Natural) {
        *self = helper(self, other, m);
    }
}

impl CheckedModDivExact<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by value, the divisor by value, and the modulus by
    /// value. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{m}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// quotient and remainder are found as by [`ModDivRem`](malachite_base::polynomial::ModDivRem),
    /// and the quotient is returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let s = p.checked_mod_div_exact(q, Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(p.checked_mod_div_exact(q, Natural::from(7u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn checked_mod_div_exact(self, other: Self, m: Natural) -> Option<Self> {
        checked_helper(&self, &other, &m)
    }
}

impl CheckedModDivExact<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by value, the divisor by value, and the modulus by
    /// reference. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{m}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// quotient and remainder are found as by [`ModDivRem`](malachite_base::polynomial::ModDivRem),
    /// and the quotient is returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let s = p.checked_mod_div_exact(q, &Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(p.checked_mod_div_exact(q, &Natural::from(7u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn checked_mod_div_exact(self, other: Self, m: &Natural) -> Option<Self> {
        checked_helper(&self, &other, m)
    }
}

impl CheckedModDivExact<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by value, the divisor by reference, and the modulus by
    /// value. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{m}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// quotient and remainder are found as by [`ModDivRem`](malachite_base::polynomial::ModDivRem),
    /// and the quotient is returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let s = p.checked_mod_div_exact(&q, Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(p.checked_mod_div_exact(&q, Natural::from(7u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn checked_mod_div_exact(self, other: &Self, m: Natural) -> Option<Self> {
        checked_helper(&self, other, &m)
    }
}

impl CheckedModDivExact<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by value, the divisor by reference, and the modulus by
    /// reference. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{m}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// quotient and remainder are found as by [`ModDivRem`](malachite_base::polynomial::ModDivRem),
    /// and the quotient is returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let s = p.checked_mod_div_exact(&q, &Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(p.checked_mod_div_exact(&q, &Natural::from(7u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn checked_mod_div_exact(self, other: &Self, m: &Natural) -> Option<Self> {
        checked_helper(&self, other, m)
    }
}

impl CheckedModDivExact<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by reference, the divisor by value, and the modulus by
    /// value. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{m}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// quotient and remainder are found as by [`ModDivRem`](malachite_base::polynomial::ModDivRem),
    /// and the quotient is returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let s = (&p).checked_mod_div_exact(q, Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!((&p).checked_mod_div_exact(q, Natural::from(7u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn checked_mod_div_exact(
        self,
        other: NaturalPolynomial,
        m: Natural,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, &other, &m)
    }
}

impl CheckedModDivExact<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by reference, the divisor by value, and the modulus by
    /// reference. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{m}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// quotient and remainder are found as by [`ModDivRem`](malachite_base::polynomial::ModDivRem),
    /// and the quotient is returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let s = (&p).checked_mod_div_exact(q, &Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(
    ///     (&p).checked_mod_div_exact(q, &Natural::from(7u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn checked_mod_div_exact(
        self,
        other: NaturalPolynomial,
        m: &Natural,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, &other, m)
    }
}

impl CheckedModDivExact<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by reference, the divisor by reference, and the
    /// modulus by value. The coefficients of both must already be reduced modulo $m$, and the
    /// leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{m}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// quotient and remainder are found as by [`ModDivRem`](malachite_base::polynomial::ModDivRem),
    /// and the quotient is returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let s = (&p).checked_mod_div_exact(&q, Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(
    ///     (&p).checked_mod_div_exact(&q, Natural::from(7u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn checked_mod_div_exact(
        self,
        other: &NaturalPolynomial,
        m: Natural,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, other, &m)
    }
}

impl CheckedModDivExact<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by reference, the divisor by reference, and the
    /// modulus by reference. The coefficients of both must already be reduced modulo $m$, and the
    /// leading coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{m}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// When $m$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// quotient and remainder are found as by [`ModDivRem`](malachite_base::polynomial::ModDivRem),
    /// and the quotient is returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n(m + \log n) \log (nm) \log\log (nm))$
    ///
    /// $M(n, m) = O(n(m + \log n) \log (nm))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $m$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` is 0 (unless $m$ is 1) or its leading coefficient is not invertible
    /// modulo `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivExact;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+6*x+2").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let s = (&p)
    ///     .checked_mod_div_exact(&q, &Natural::from(7u32))
    ///     .unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    ///
    /// let p = NaturalPolynomial::from_str("x^3+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("x^2+3").unwrap();
    /// assert!(
    ///     (&p).checked_mod_div_exact(&q, &Natural::from(7u32))
    ///         .is_none()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts or assumes the leading coefficient
    /// invertible.
    #[inline]
    fn checked_mod_div_exact(
        self,
        other: &NaturalPolynomial,
        m: &Natural,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, other, m)
    }
}
