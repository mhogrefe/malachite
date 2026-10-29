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
use crate::natural_polynomial::arithmetic::mod_power_of_2_div_truncated::*;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Inverse, ModPowerOf2IsReduced, ModPowerOf2Sub,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{
    CheckedModPowerOf2DivRem, ModPowerOf2DivAssignRem, ModPowerOf2DivRem, Polynomial,
};
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_div_rem::*;

pub(crate) fn assert_reduced(p: &NaturalPolynomial, q: &NaturalPolynomial, pow: u64) {
    assert!(
        p.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {p} has a coefficient >= 2^{pow}"
    );
    assert!(
        q.mod_power_of_2_is_reduced(pow),
        "other must be reduced mod 2^pow, but {q} has a coefficient >= 2^{pow}"
    );
}

// The quotient modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, reduced, where `ys` is nonempty and no longer than `xs`, and `inverse` is the inverse of
// the last coefficient of `ys` modulo $2^k$. The quotient has `xs.len() - ys.len() + 1`
// coefficients; reversed, it is the quotient of the reversals of the polynomials as power series,
// which `series` computes.
//
// This is equivalent to `_gr_poly_div_newton` from `gr_poly/div_newton.c`, FLINT 3.6.0, with
// coefficients modulo $2^k$ and `series` in place of `_gr_poly_div_series`.
crate_test_fn! {mod_power_of_2_quotient_with(
    xs: &[Natural],
    ys: &[Natural],
    pow: u64,
    inverse: &Natural,
    series: fn(&[Natural], &[Natural], usize, u64, &Natural) -> Vec<Natural>,
) -> Vec<Natural> {
    let q_len = xs.len() - ys.len() + 1;
    let xs_rev: Vec<Natural> = xs[xs.len() - q_len..].iter().rev().cloned().collect();
    let ys_rev: Vec<Natural> = ys[ys.len().saturating_sub(q_len)..].iter().rev().cloned().collect();
    let mut q = series(&xs_rev, &ys_rev, q_len, pow, inverse);
    q.reverse();
    q
}}

// The remainder modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, reduced, given their quotient `qs`: the `ys.len() - 1` coefficients of `xs - qs * ys` that
// are not cancelled.
//
// This is equivalent to the remainder step of `_gr_poly_divrem_newton` from
// `gr_poly/divrem_newton.c`, FLINT 3.6.0, with coefficients modulo $2^k$.
fn remainder(xs: &[Natural], ys: &[Natural], qs: &[Natural], pow: u64) -> Vec<Natural> {
    let r_len = ys.len() - 1;
    if r_len == 0 {
        return Vec::new();
    }
    let product =
        mod_power_of_2_mul_truncated_ref_ref(qs, &ys[..r_len], u64::exact_from(r_len), pow)
            .into_coefficients_asc();
    xs[..r_len]
        .iter()
        .enumerate()
        .map(|(i, x)| {
            product
                .get(i)
                .map_or_else(|| x.clone(), |y| x.mod_power_of_2_sub(y, pow))
        })
        .collect()
}

// The quotient and remainder modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients
// `xs` and `ys`, reduced, with `series` dividing the reversals as power series in the general case,
// or `None` if `ys` is zero or its leading coefficient is even. Modulo 1, every polynomial is 0,
// and so is the result.
//
// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0, with the
// modulus $2^k$, except that `None` is returned where FLINT aborts.
crate_test_fn! {mod_power_of_2_div_rem_with(
    xs: &[Natural],
    ys: &[Natural],
    pow: u64,
    series: fn(&[Natural], &[Natural], usize, u64, &Natural) -> Vec<Natural>,
) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
    if pow == 0 {
        return Some((NaturalPolynomial::ZERO, NaturalPolynomial::ZERO));
    }
    let inverse = ys.last()?.mod_power_of_2_inverse(pow)?;
    if xs.len() < ys.len() {
        return Some((NaturalPolynomial::ZERO, trimmed(xs.to_vec())));
    }
    let qs = mod_power_of_2_quotient_with(xs, ys, pow, &inverse, series);
    let rs = remainder(xs, ys, &qs, pow);
    Some((trimmed(qs), trimmed(rs)))
}}

// The quotient and remainder modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients
// `xs` and `ys`, reduced, computed in `T`, which $2^k$ must fit in.
fn mod_power_of_2_div_rem_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    pow: u64,
) -> Option<(NaturalPolynomial, NaturalPolynomial)>
where
    Natural: From<T>,
{
    let (qs, rs) = mod_power_of_2_div_rem_vec(&to_words::<T>(xs), &to_words::<T>(ys), pow)?;
    Some((from_words(qs), from_words(rs)))
}

// The quotient and remainder modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients
// `xs` and `ys`, reduced, or `None` if the divisor is zero or its leading coefficient is even. When
// $2^k$ fits in one or two limbs, this is done in wrapping word arithmetic.
pub(crate) fn checked_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
    assert_reduced(p, q, pow);
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    if pow <= Limb::WIDTH {
        mod_power_of_2_div_rem_word::<Limb>(xs, ys, pow)
    } else if pow <= DoubleLimb::WIDTH {
        mod_power_of_2_div_rem_word::<DoubleLimb>(xs, ys, pow)
    } else {
        mod_power_of_2_div_rem_with(xs, ys, pow, mod_power_of_2_div_truncated_dispatch)
    }
}

fn helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    pow: u64,
) -> (NaturalPolynomial, NaturalPolynomial) {
    assert!(
        pow == 0 || *q != NaturalPolynomial::ZERO,
        "division by zero"
    );
    checked_helper(p, q, pow).expect("the leading coefficient of the divisor must be odd")
}

impl ModPowerOf2DivRem<Self> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient and remainder,
    /// taking the dividend by value and the divisor by value. The coefficients of both must already
    /// be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModPowerOf2DivRem;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem(q, 3);
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_power_of_2_div_rem(self, other: Self, pow: u64) -> (Self, Self) {
        helper(&self, &other, pow)
    }
}

impl ModPowerOf2DivRem<&Self> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient and remainder,
    /// taking the dividend by value and the divisor by reference. The coefficients of both must
    /// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModPowerOf2DivRem;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let (s, r) = p.mod_power_of_2_div_rem(&q, 3);
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_power_of_2_div_rem(self, other: &Self, pow: u64) -> (Self, Self) {
        helper(&self, other, pow)
    }
}

impl ModPowerOf2DivRem<NaturalPolynomial> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient and remainder,
    /// taking the dividend by reference and the divisor by value. The coefficients of both must
    /// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModPowerOf2DivRem;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem(q, 3);
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_power_of_2_div_rem(
        self,
        other: NaturalPolynomial,
        pow: u64,
    ) -> (NaturalPolynomial, NaturalPolynomial) {
        helper(self, &other, pow)
    }
}

impl ModPowerOf2DivRem<&NaturalPolynomial> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient and remainder,
    /// taking the dividend by reference and the divisor by reference. The coefficients of both must
    /// already be reduced modulo $2^k$, and the leading coefficient of the divisor must be odd.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModPowerOf2DivRem;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let (s, r) = (&p).mod_power_of_2_div_rem(&q, 3);
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_power_of_2_div_rem(
        self,
        other: &NaturalPolynomial,
        pow: u64,
    ) -> (NaturalPolynomial, NaturalPolynomial) {
        helper(self, other, pow)
    }
}

impl ModPowerOf2DivAssignRem<Self> for NaturalPolynomial {
    type RemOutput = Self;

    /// Divides a [`NaturalPolynomial`] by another modulo $2^k$ in place, replacing it with the
    /// quotient and returning the remainder, taking the divisor by value. The coefficients of both
    /// must already be reduced modulo $2^k$, and the leading coefficient of the divisor must be
    /// odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModPowerOf2DivAssignRem;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let r = p.mod_power_of_2_div_assign_rem(q, 3);
    /// assert_eq!(p.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_power_of_2_div_assign_rem(&mut self, other: Self, pow: u64) -> Self {
        let (q, r) = helper(self, &other, pow);
        *self = q;
        r
    }
}

impl ModPowerOf2DivAssignRem<&Self> for NaturalPolynomial {
    type RemOutput = Self;

    /// Divides a [`NaturalPolynomial`] by another modulo $2^k$ in place, replacing it with the
    /// quotient and returning the remainder, taking the divisor by reference. The coefficients of
    /// both must already be reduced modulo $2^k$, and the leading coefficient of the divisor must
    /// be odd.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    ///
    /// When $k$ is 0, every polynomial is 0, and so is the result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModPowerOf2DivAssignRem;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let r = p.mod_power_of_2_div_assign_rem(&q, 3);
    /// assert_eq!(p.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_power_of_2_div_assign_rem(&mut self, other: &Self, pow: u64) -> Self {
        let (q, r) = helper(self, other, pow);
        *self = q;
        r
    }
}

impl CheckedModPowerOf2DivRem<Self> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not odd, taking the dividend by value and the
    /// divisor by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not odd; otherwise the
    /// quotient and remainder are unique. When $k$ is 0, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivRem;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let (s, r) = p.checked_mod_power_of_2_div_rem(q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = NaturalPolynomial::from_str("x^3+2*x^2+3").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_power_of_2_div_rem(q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_power_of_2_div_rem(self, other: Self, pow: u64) -> Option<(Self, Self)> {
        checked_helper(&self, &other, pow)
    }
}

impl CheckedModPowerOf2DivRem<&Self> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not odd, taking the dividend by value and the
    /// divisor by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not odd; otherwise the
    /// quotient and remainder are unique. When $k$ is 0, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivRem;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let (s, r) = p.checked_mod_power_of_2_div_rem(&q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = NaturalPolynomial::from_str("x^3+2*x^2+3").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_power_of_2_div_rem(&q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_power_of_2_div_rem(self, other: &Self, pow: u64) -> Option<(Self, Self)> {
        checked_helper(&self, other, pow)
    }
}

impl CheckedModPowerOf2DivRem<NaturalPolynomial> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not odd, taking the dividend by reference and
    /// the divisor by value. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not odd; otherwise the
    /// quotient and remainder are unique. When $k$ is 0, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivRem;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let (s, r) = (&p).checked_mod_power_of_2_div_rem(q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = NaturalPolynomial::from_str("x^3+2*x^2+3").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_power_of_2_div_rem(q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_power_of_2_div_rem(
        self,
        other: NaturalPolynomial,
        pow: u64,
    ) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
        checked_helper(self, &other, pow)
    }
}

impl CheckedModPowerOf2DivRem<&NaturalPolynomial> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $2^k$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not odd, taking the dividend by reference and
    /// the divisor by reference. The coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(a, b, k) = (q, r), \quad a \equiv bq + r \pmod{2^k}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not odd; otherwise the
    /// quotient and remainder are unique. When $k$ is 0, every polynomial is 0, and so is the
    /// result.
    ///
    /// When $2^k$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n(k + \log n) \log (nk) \log\log (nk))$
    ///
    /// $M(n, k) = O(n(k + \log n) \log (nk))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModPowerOf2DivRem;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^4+7*x+5").unwrap();
    /// let q = NaturalPolynomial::from_str("5*x^2+2").unwrap();
    /// let (s, r) = (&p).checked_mod_power_of_2_div_rem(&q, 3).unwrap();
    /// assert_eq!(s.to_string(), "5*x^2+6");
    /// assert_eq!(r.to_string(), "7*x+1");
    ///
    /// // The leading coefficient of the divisor is not odd.
    /// let p = NaturalPolynomial::from_str("x^3+2*x^2+3").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_power_of_2_div_rem(&q, 2).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// with the modulus $2^k$, except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_power_of_2_div_rem(
        self,
        other: &NaturalPolynomial,
        pow: u64,
    ) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
        checked_helper(self, other, pow)
    }
}
