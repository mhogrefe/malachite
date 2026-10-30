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
use crate::natural_polynomial::arithmetic::mod_div_rem::{from_words, to_words, trimmed};
use crate::natural_polynomial::arithmetic::mod_power_of_2_div_rem::{
    assert_reduced, checked_helper as dr_helper, mod_power_of_2_quotient_with,
};
use crate::natural_polynomial::arithmetic::mod_power_of_2_div_truncated::*;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Inverse, Parity};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{
    CheckedModPowerOf2DivExact, ModPowerOf2DivExact, ModPowerOf2DivExactAssign,
};
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_div_exact::*;

// The quotient modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, reduced, assuming that the division is exact, where `ys` is nonempty and no longer than
// `xs`, `inverse` is the inverse of the last coefficient of `ys` modulo $2^k$, and `series` divides
// power series. The low zeros of `ys` are first discarded, along with as many coefficients of `xs`;
// then, if the lowest coefficient of `ys` is odd, the low half of the quotient is found by dividing
// the polynomials as power series, and the high half from the top, as in
// `mod_power_of_2_quotient_with`, by the high coefficients of `xs`. Otherwise the whole quotient is
// found from the top.
//
// This is equivalent to `_gr_poly_divexact_bidirectional` from `gr_poly/divexact_bidirectional.c`,
// FLINT 3.6.0, with coefficients modulo $2^k$ and `series` in place of `_gr_poly_div_series`,
// except that an even lowest coefficient is handled.
crate_test_fn! {mod_power_of_2_div_exact_bidirectional(
    xs: &[Natural],
    ys: &[Natural],
    pow: u64,
    inverse: &Natural,
    series: fn(&[Natural], &[Natural], usize, u64, &Natural) -> Vec<Natural>,
) -> Vec<Natural> {
    let zeros = ys.iter().take_while(|&y| *y == 0u32).count();
    let (xs, ys) = (&xs[zeros..], &ys[zeros..]);
    let q_len = xs.len() - ys.len() + 1;
    if ys.len() == 1 || q_len == 1 {
        return mod_power_of_2_quotient_with(xs, ys, pow, inverse, series);
    }
    let Some(low_inverse) = (&ys[0]).mod_power_of_2_inverse(pow) else {
        return mod_power_of_2_quotient_with(xs, ys, pow, inverse, series);
    };
    let low_len = q_len - (q_len >> 1);
    let mut qs = series(xs, ys, low_len, pow, &low_inverse);
    qs.extend(mod_power_of_2_quotient_with(&xs[low_len..], ys, pow, inverse, series));
    qs
}}

// The quotient modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, reduced, assuming that the division is exact, where `ys` is nonempty and no longer than
// `xs`, and `inverse` is the inverse of the last coefficient of `ys` modulo $2^k$.
//
// This is equivalent to `_gr_poly_divexact_generic` from `gr_poly/divexact.c`, FLINT 3.6.0, with
// coefficients modulo $2^k$.
fn mod_power_of_2_div_exact_dispatch(
    xs: &[Natural],
    ys: &[Natural],
    pow: u64,
    inverse: &Natural,
) -> Vec<Natural> {
    if ys.len() <= 2 {
        mod_power_of_2_quotient_with(xs, ys, pow, inverse, mod_power_of_2_div_truncated_dispatch)
    } else {
        mod_power_of_2_div_exact_bidirectional(
            xs,
            ys,
            pow,
            inverse,
            mod_power_of_2_div_truncated_dispatch,
        )
    }
}

// The quotient modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, reduced, assuming that the division is exact, with `kernel` doing the work in the general
// case, or `None` if `ys` is zero or its leading coefficient is even. Modulo 1, every polynomial is
// 0, and so is the result.
crate_test_fn! {mod_power_of_2_div_exact_with(
    xs: &[Natural],
    ys: &[Natural],
    pow: u64,
    kernel: fn(&[Natural], &[Natural], u64, &Natural) -> Vec<Natural>,
) -> Option<NaturalPolynomial> {
    if pow == 0 {
        return Some(NaturalPolynomial::ZERO);
    }
    let inverse = ys.last()?.mod_power_of_2_inverse(pow)?;
    if xs.len() < ys.len() {
        return Some(NaturalPolynomial::ZERO);
    }
    Some(trimmed(kernel(xs, ys, pow, &inverse)))
}}

// The quotient modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, reduced, assuming that the division is exact, computed in `T`, which $2^k$ must fit in.
fn mod_power_of_2_div_exact_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    pow: u64,
) -> NaturalPolynomial
where
    Natural: From<T>,
{
    from_words(mod_power_of_2_div_exact_vec(&to_words::<T>(xs), &to_words::<T>(ys), pow).unwrap())
}

// Panics if `q` is zero, unless `pow` is 0, or if its leading coefficient is even.
pub(crate) fn assert_invertible(q: &NaturalPolynomial, pow: u64) {
    if pow != 0 {
        assert_ne!(*q, NaturalPolynomial::ZERO, "division by zero");
        assert!(
            q.coefficients.last().unwrap().odd(),
            "the leading coefficient of the divisor must be odd"
        );
    }
}

// The quotient modulo $2^k$, where $k$ is `pow`, of `p` by `q`, assuming that the division is
// exact. When $2^k$ fits in one or two limbs, this is done in wrapping word arithmetic.
//
// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0, with
// the modulus $2^k$, without the check that the division is exact.
fn helper(p: &NaturalPolynomial, q: &NaturalPolynomial, pow: u64) -> NaturalPolynomial {
    assert_reduced(p, q, pow);
    assert_invertible(q, pow);
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    if pow <= Limb::WIDTH {
        mod_power_of_2_div_exact_word::<Limb>(xs, ys, pow)
    } else if pow <= DoubleLimb::WIDTH {
        mod_power_of_2_div_exact_word::<DoubleLimb>(xs, ys, pow)
    } else {
        mod_power_of_2_div_exact_with(xs, ys, pow, mod_power_of_2_div_exact_dispatch).unwrap()
    }
}

// The quotient modulo $2^k$, where $k$ is `pow`, of `p` by `q`, or `None` if `q` does not divide
// `p`.
//
// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0, with
// the modulus $2^k$, except that Malachite panics where FLINT assumes the leading coefficient
// invertible.
fn checked_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<NaturalPolynomial> {
    assert_reduced(p, q, pow);
    assert_invertible(q, pow);
    let (q, r) = dr_helper(p, q, pow).unwrap();
    if r == NaturalPolynomial::ZERO {
        Some(q)
    } else {
        None
    }
}

impl ModPowerOf2DivExact<Self> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, assuming that the second divides the first,
    /// taking the dividend by value and the divisor by value. The coefficients of both must already
    /// be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $k$ is 0, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// odd, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivExact;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+4").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// assert_eq!(p.mod_power_of_2_div_exact(q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, with the modulus $2^k$, except that Malachite
    /// panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_exact(self, other: Self, pow: u64) -> Self {
        helper(&self, &other, pow)
    }
}

impl ModPowerOf2DivExact<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, assuming that the second divides the first,
    /// taking the dividend by value and the divisor by reference. The coefficients of both must
    /// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $k$ is 0, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// odd, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivExact;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+4").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// assert_eq!(p.mod_power_of_2_div_exact(&q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, with the modulus $2^k$, except that Malachite
    /// panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_exact(self, other: &Self, pow: u64) -> Self {
        helper(&self, other, pow)
    }
}

impl ModPowerOf2DivExact<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, assuming that the second divides the first,
    /// taking the dividend by reference and the divisor by value. The coefficients of both must
    /// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $k$ is 0, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// odd, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivExact;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+4").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// assert_eq!((&p).mod_power_of_2_div_exact(q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, with the modulus $2^k$, except that Malachite
    /// panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_exact(self, other: NaturalPolynomial, pow: u64) -> NaturalPolynomial {
        helper(self, &other, pow)
    }
}

impl ModPowerOf2DivExact<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, assuming that the second divides the first,
    /// taking the dividend by reference and the divisor by reference. The coefficients of both must
    /// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $k$ is 0, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// odd, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivExact;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+4").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// assert_eq!((&p).mod_power_of_2_div_exact(&q, 3).to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, with the modulus $2^k$, except that Malachite
    /// panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_exact(self, other: &NaturalPolynomial, pow: u64) -> NaturalPolynomial {
        helper(self, other, pow)
    }
}

impl ModPowerOf2DivExactAssign<Self> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $2^k$ in place, assuming that the second
    /// divides the first, taking the divisor by value. The coefficients of both must already be
    /// reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $k$ is 0, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// odd, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivExactAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+4").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// p.mod_power_of_2_div_exact_assign(q, 3);
    /// assert_eq!(p.to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, with the modulus $2^k$, except that Malachite
    /// panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_exact_assign(&mut self, other: Self, pow: u64) {
        *self = helper(self, &other, pow);
    }
}

impl ModPowerOf2DivExactAssign<&Self> for NaturalPolynomial {
    /// Divides a [`NaturalPolynomial`] by another modulo $2^k$ in place, assuming that the second
    /// divides the first, taking the divisor by reference. The coefficients of both must already be
    /// reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq \pmod{2^k}.
    /// $$
    ///
    /// If the divisor does not divide the dividend, the result is meaningless. When $k$ is 0, every
    /// polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The low
    /// zeros of the divisor are discarded first. Long quotients are then found half from the
    /// bottom, by dividing the polynomials as power series, and half from the top, by dividing the
    /// reversed polynomials as power series; when the divisor's lowest nonzero coefficient is not
    /// odd, the whole quotient is found from the top.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2DivExactAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+4").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// p.mod_power_of_2_div_exact_assign(&q, 3);
    /// assert_eq!(p.to_string(), "5*x^2+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, without the
    /// check that the division is exact, FLINT 3.6.0, with the modulus $2^k$, except that Malachite
    /// panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_power_of_2_div_exact_assign(&mut self, other: &Self, pow: u64) {
        *self = helper(self, other, pow);
    }
}

impl CheckedModPowerOf2DivExact<Self> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by value and the divisor by value. The coefficients of
    /// both must already be reduced modulo $2^k$, and the leading coefficient of the divisor must
    /// be odd.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{2^k}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// quotient and remainder are found as by
    /// [`ModPowerOf2DivRem`](malachite_base::polynomial::ModPowerOf2DivRem), and the quotient is
    /// returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivExact;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+4").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let s = p.checked_mod_power_of_2_div_exact(q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+3").unwrap();
    /// assert!(p.checked_mod_power_of_2_div_exact(q, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the
    /// leading coefficient invertible.
    #[inline]
    fn checked_mod_power_of_2_div_exact(self, other: Self, pow: u64) -> Option<Self> {
        checked_helper(&self, &other, pow)
    }
}

impl CheckedModPowerOf2DivExact<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by value and the divisor by reference. The
    /// coefficients of both must already be reduced modulo $2^k$, and the leading coefficient of
    /// the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{2^k}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// quotient and remainder are found as by
    /// [`ModPowerOf2DivRem`](malachite_base::polynomial::ModPowerOf2DivRem), and the quotient is
    /// returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivExact;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+4").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let s = p.checked_mod_power_of_2_div_exact(&q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+3").unwrap();
    /// assert!(p.checked_mod_power_of_2_div_exact(&q, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the
    /// leading coefficient invertible.
    #[inline]
    fn checked_mod_power_of_2_div_exact(self, other: &Self, pow: u64) -> Option<Self> {
        checked_helper(&self, other, pow)
    }
}

impl CheckedModPowerOf2DivExact<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by reference and the divisor by value. The
    /// coefficients of both must already be reduced modulo $2^k$, and the leading coefficient of
    /// the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{2^k}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// quotient and remainder are found as by
    /// [`ModPowerOf2DivRem`](malachite_base::polynomial::ModPowerOf2DivRem), and the quotient is
    /// returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivExact;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+4").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let s = (&p).checked_mod_power_of_2_div_exact(q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+3").unwrap();
    /// assert!((&p).checked_mod_power_of_2_div_exact(q, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the
    /// leading coefficient invertible.
    #[inline]
    fn checked_mod_power_of_2_div_exact(
        self,
        other: NaturalPolynomial,
        pow: u64,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, &other, pow)
    }
}

impl CheckedModPowerOf2DivExact<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$ if the second divides the first, returning
    /// `None` otherwise, taking the dividend by reference and the divisor by reference. The
    /// coefficients of both must already be reduced modulo $2^k$, and the leading coefficient of
    /// the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = \\begin{cases}
    ///     \operatorname{Some}(q) & \text{if } a \equiv bq \pmod{2^k}, \\\\
    ///     \operatorname{None} & \text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and the result is `Some` of the zero polynomial.
    ///
    /// When $2^k$ fits in one or two limbs, this is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). The
    /// quotient and remainder are found as by
    /// [`ModPowerOf2DivRem`](malachite_base::polynomial::ModPowerOf2DivRem), and the quotient is
    /// returned if the remainder is 0.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` is 0 (unless $k$ is 0) or its leading coefficient is not odd.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivExact;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+4").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let s = (&p).checked_mod_power_of_2_div_exact(&q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+3").unwrap();
    /// assert!((&p).checked_mod_power_of_2_div_exact(&q, 3).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divides` from `fmpz_mod_poly/divides.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts or assumes the
    /// leading coefficient invertible.
    #[inline]
    fn checked_mod_power_of_2_div_exact(
        self,
        other: &NaturalPolynomial,
        pow: u64,
    ) -> Option<NaturalPolynomial> {
        checked_helper(self, other, pow)
    }
}
