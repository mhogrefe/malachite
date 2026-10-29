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
use crate::natural_polynomial::arithmetic::mod_div_truncated::mod_div_truncated_dispatch;
use crate::natural_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_ref_ref;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{ModInverse, ModIsReduced, ModSub};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{CheckedModDivRem, ModDivAssignRem, ModDivRem, Polynomial};
use malachite_base::unsigned_polynomial::arithmetic::mod_div_rem::mod_div_rem_vec;

pub(crate) fn assert_reduced(p: &NaturalPolynomial, q: &NaturalPolynomial, m: &Natural) {
    assert!(
        p.mod_is_reduced(m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(
        q.mod_is_reduced(m),
        "other must be reduced mod m, but {q} has a coefficient >= {m}"
    );
}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, where `ys`
// is nonempty and no longer than `xs`, and `inverse` is the inverse of the last coefficient of `ys`
// modulo `m`. The quotient has `xs.len() - ys.len() + 1` coefficients; reversed, it is the quotient
// of the reversals of the polynomials as power series, which `series` computes.
//
// This is equivalent to `_gr_poly_div_newton` from `gr_poly/div_newton.c`, FLINT 3.6.0, with
// coefficients modulo `m` and `series` in place of `_gr_poly_div_series`.
crate_test_fn! {mod_quotient_with(
    xs: &[Natural],
    ys: &[Natural],
    m: &Natural,
    inverse: &Natural,
    series: fn(&[Natural], &[Natural], usize, &Natural, &Natural) -> Vec<Natural>,
) -> Vec<Natural> {
    let q_len = xs.len() - ys.len() + 1;
    let xs_rev: Vec<Natural> = xs[xs.len() - q_len..].iter().rev().cloned().collect();
    let ys_rev: Vec<Natural> = ys[ys.len().saturating_sub(q_len)..].iter().rev().cloned().collect();
    let mut q = series(&xs_rev, &ys_rev, q_len, m, inverse);
    q.reverse();
    q
}}

// The remainder modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, given their
// quotient `qs`: the `ys.len() - 1` coefficients of `xs - qs * ys` that are not cancelled.
//
// This is equivalent to the remainder step of `_gr_poly_divrem_newton` from
// `gr_poly/divrem_newton.c`, FLINT 3.6.0, with coefficients modulo `m`.
fn remainder(xs: &[Natural], ys: &[Natural], qs: &[Natural], m: &Natural) -> Vec<Natural> {
    let r_len = ys.len() - 1;
    if r_len == 0 {
        return Vec::new();
    }
    let product = mod_mul_truncated_ref_ref(qs, &ys[..r_len], u64::exact_from(r_len), m)
        .into_coefficients_asc();
    xs[..r_len]
        .iter()
        .enumerate()
        .map(|(i, x)| {
            product
                .get(i)
                .map_or_else(|| x.clone(), |y| x.mod_sub(y, m))
        })
        .collect()
}

// The quotient and remainder modulo `m` of the polynomials with coefficients `xs` and `ys`,
// reduced, with `series` dividing the reversals as power series in the general case, or `None` if
// `ys` is zero or its leading coefficient is not invertible. Modulo 1, every polynomial is 0, and
// so is the result.
//
// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0, except
// that `None` is returned where FLINT aborts.
crate_test_fn! {mod_div_rem_with(
    xs: &[Natural],
    ys: &[Natural],
    m: &Natural,
    series: fn(&[Natural], &[Natural], usize, &Natural, &Natural) -> Vec<Natural>,
) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
    if *m == 1u32 {
        return Some((NaturalPolynomial::ZERO, NaturalPolynomial::ZERO));
    }
    let inverse = ys.last()?.mod_inverse(m)?;
    if xs.len() < ys.len() {
        return Some((NaturalPolynomial::ZERO, trimmed(xs.to_vec())));
    }
    let qs = mod_quotient_with(xs, ys, m, &inverse, series);
    let rs = remainder(xs, ys, &qs, m);
    Some((trimmed(qs), trimmed(rs)))
}}

// The polynomial with coefficients `xs`, trimmed.
pub(crate) fn trimmed(xs: Vec<Natural>) -> NaturalPolynomial {
    let mut p = NaturalPolynomial { coefficients: xs };
    p.trim();
    p
}

// The coefficients of the polynomial `xs` converted to `T`, which they must fit in.
pub(crate) fn to_words<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
) -> Vec<T> {
    xs.iter().map(T::exact_from).collect()
}

// The polynomial whose coefficients are the words `xs`.
pub(crate) fn from_words<T: PrimitiveUnsigned>(xs: Vec<T>) -> NaturalPolynomial
where
    Natural: From<T>,
{
    NaturalPolynomial {
        coefficients: xs.into_iter().map(Natural::from).collect(),
    }
}

// The quotient and remainder modulo `m` of the polynomials with coefficients `xs` and `ys`,
// reduced, computed in `T`, which `m` must fit in.
fn mod_div_rem_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    m: &Natural,
) -> Option<(NaturalPolynomial, NaturalPolynomial)>
where
    Natural: From<T>,
{
    let (qs, rs) = mod_div_rem_vec(&to_words::<T>(xs), &to_words::<T>(ys), T::exact_from(m))?;
    Some((from_words(qs), from_words(rs)))
}

// The quotient and remainder modulo `m` of the polynomials with coefficients `xs` and `ys`,
// reduced, or `None` if the divisor is zero or its leading coefficient is not invertible. When `m`
// fits in one or two limbs, this is done in word arithmetic.
pub(crate) fn checked_helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
    assert_reduced(p, q, m);
    let (xs, ys) = (&p.coefficients, &q.coefficients);
    let bits = m.significant_bits();
    if bits <= Limb::WIDTH {
        mod_div_rem_word::<Limb>(xs, ys, m)
    } else if bits <= DoubleLimb::WIDTH {
        mod_div_rem_word::<DoubleLimb>(xs, ys, m)
    } else {
        mod_div_rem_with(xs, ys, m, mod_div_truncated_dispatch)
    }
}

fn helper(
    p: &NaturalPolynomial,
    q: &NaturalPolynomial,
    m: &Natural,
) -> (NaturalPolynomial, NaturalPolynomial) {
    assert!(
        *m == 1u32 || *q != NaturalPolynomial::ZERO,
        "division by zero"
    );
    checked_helper(p, q, m)
        .expect("the leading coefficient of the divisor must be invertible modulo m")
}

impl ModDivRem<Self, Natural> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, taking
    /// the dividend by value, the divisor by value, and the modulus by value. The coefficients of
    /// both must already be reduced modulo $m$, and the leading coefficient of the divisor must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = p.mod_div_rem(q, Natural::from(7u32));
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_div_rem(self, other: Self, m: Natural) -> (Self, Self) {
        helper(&self, &other, &m)
    }
}

impl ModDivRem<Self, &Natural> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, taking
    /// the dividend by value, the divisor by value, and the modulus by reference. The coefficients
    /// of both must already be reduced modulo $m$, and the leading coefficient of the divisor must
    /// be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = p.mod_div_rem(q, &Natural::from(7u32));
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_div_rem(self, other: Self, m: &Natural) -> (Self, Self) {
        helper(&self, &other, m)
    }
}

impl ModDivRem<&Self, Natural> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, taking
    /// the dividend by value, the divisor by reference, and the modulus by value. The coefficients
    /// of both must already be reduced modulo $m$, and the leading coefficient of the divisor must
    /// be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = p.mod_div_rem(&q, Natural::from(7u32));
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_div_rem(self, other: &Self, m: Natural) -> (Self, Self) {
        helper(&self, other, &m)
    }
}

impl ModDivRem<&Self, &Natural> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, taking
    /// the dividend by value, the divisor by reference, and the modulus by reference. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = p.mod_div_rem(&q, &Natural::from(7u32));
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_div_rem(self, other: &Self, m: &Natural) -> (Self, Self) {
        helper(&self, other, m)
    }
}

impl ModDivRem<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, taking
    /// the dividend by reference, the divisor by value, and the modulus by value. The coefficients
    /// of both must already be reduced modulo $m$, and the leading coefficient of the divisor must
    /// be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem(q, Natural::from(7u32));
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_div_rem(
        self,
        other: NaturalPolynomial,
        m: Natural,
    ) -> (NaturalPolynomial, NaturalPolynomial) {
        helper(self, &other, &m)
    }
}

impl ModDivRem<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, taking
    /// the dividend by reference, the divisor by value, and the modulus by reference. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem(q, &Natural::from(7u32));
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_div_rem(
        self,
        other: NaturalPolynomial,
        m: &Natural,
    ) -> (NaturalPolynomial, NaturalPolynomial) {
        helper(self, &other, m)
    }
}

impl ModDivRem<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, taking
    /// the dividend by reference, the divisor by reference, and the modulus by value. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem(&q, Natural::from(7u32));
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_div_rem(
        self,
        other: &NaturalPolynomial,
        m: Natural,
    ) -> (NaturalPolynomial, NaturalPolynomial) {
        helper(self, other, &m)
    }
}

impl ModDivRem<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, taking
    /// the dividend by reference, the divisor by reference, and the modulus by reference. The
    /// coefficients of both must already be reduced modulo $m$, and the leading coefficient of the
    /// divisor must be invertible modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem(&q, &Natural::from(7u32));
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_div_rem(
        self,
        other: &NaturalPolynomial,
        m: &Natural,
    ) -> (NaturalPolynomial, NaturalPolynomial) {
        helper(self, other, m)
    }
}

impl ModDivAssignRem<Self, Natural> for NaturalPolynomial {
    type RemOutput = Self;

    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, replacing it with the
    /// quotient and returning the remainder, taking the divisor by value, and the modulus by value.
    /// The coefficients of both must already be reduced modulo $m$, and the leading coefficient of
    /// the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModDivAssignRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let r = p.mod_div_assign_rem(q, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_div_assign_rem(&mut self, other: Self, m: Natural) -> Self {
        let (q, r) = helper(self, &other, &m);
        *self = q;
        r
    }
}

impl ModDivAssignRem<Self, &Natural> for NaturalPolynomial {
    type RemOutput = Self;

    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, replacing it with the
    /// quotient and returning the remainder, taking the divisor by value, and the modulus by
    /// reference. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModDivAssignRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let r = p.mod_div_assign_rem(q, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_div_assign_rem(&mut self, other: Self, m: &Natural) -> Self {
        let (q, r) = helper(self, &other, m);
        *self = q;
        r
    }
}

impl ModDivAssignRem<&Self, Natural> for NaturalPolynomial {
    type RemOutput = Self;

    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, replacing it with the
    /// quotient and returning the remainder, taking the divisor by reference, and the modulus by
    /// value. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModDivAssignRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let r = p.mod_div_assign_rem(&q, Natural::from(7u32));
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_div_assign_rem(&mut self, other: &Self, m: Natural) -> Self {
        let (q, r) = helper(self, other, &m);
        *self = q;
        r
    }
}

impl ModDivAssignRem<&Self, &Natural> for NaturalPolynomial {
    type RemOutput = Self;

    /// Divides a [`NaturalPolynomial`] by another modulo $m$ in place, replacing it with the
    /// quotient and returning the remainder, taking the divisor by reference, and the modulus by
    /// reference. The coefficients of both must already be reduced modulo $m$, and the leading
    /// coefficient of the divisor must be invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// use malachite_base::polynomial::ModDivAssignRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let r = p.mod_div_assign_rem(&q, &Natural::from(7u32));
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that Malachite panics where FLINT aborts.
    #[inline]
    fn mod_div_assign_rem(&mut self, other: &Self, m: &Natural) -> Self {
        let (q, r) = helper(self, other, m);
        *self = q;
        r
    }
}

impl CheckedModDivRem<Self, Natural> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not invertible modulo $m$, taking the
    /// dividend by value, the divisor by value, and the modulus by value. The coefficients of both
    /// must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient and remainder are unique. When $m$ is 1, every polynomial is 0, and
    /// so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = p.checked_mod_div_rem(q, Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_div_rem(q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_div_rem(self, other: Self, m: Natural) -> Option<(Self, Self)> {
        checked_helper(&self, &other, &m)
    }
}

impl CheckedModDivRem<Self, &Natural> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not invertible modulo $m$, taking the
    /// dividend by value, the divisor by value, and the modulus by reference. The coefficients of
    /// both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient and remainder are unique. When $m$ is 1, every polynomial is 0, and
    /// so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = p.checked_mod_div_rem(q, &Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_div_rem(q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_div_rem(self, other: Self, m: &Natural) -> Option<(Self, Self)> {
        checked_helper(&self, &other, m)
    }
}

impl CheckedModDivRem<&Self, Natural> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not invertible modulo $m$, taking the
    /// dividend by value, the divisor by reference, and the modulus by value. The coefficients of
    /// both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient and remainder are unique. When $m$ is 1, every polynomial is 0, and
    /// so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = p.checked_mod_div_rem(&q, Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_div_rem(&q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_div_rem(self, other: &Self, m: Natural) -> Option<(Self, Self)> {
        checked_helper(&self, other, &m)
    }
}

impl CheckedModDivRem<&Self, &Natural> for NaturalPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not invertible modulo $m$, taking the
    /// dividend by value, the divisor by reference, and the modulus by reference. The coefficients
    /// of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient and remainder are unique. When $m$ is 1, every polynomial is 0, and
    /// so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = p.checked_mod_div_rem(&q, &Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_div_rem(&q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_div_rem(self, other: &Self, m: &Natural) -> Option<(Self, Self)> {
        checked_helper(&self, other, m)
    }
}

impl CheckedModDivRem<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not invertible modulo $m$, taking the
    /// dividend by reference, the divisor by value, and the modulus by value. The coefficients of
    /// both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient and remainder are unique. When $m$ is 1, every polynomial is 0, and
    /// so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = (&p).checked_mod_div_rem(q, Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_div_rem(q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_div_rem(
        self,
        other: NaturalPolynomial,
        m: Natural,
    ) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
        checked_helper(self, &other, &m)
    }
}

impl CheckedModDivRem<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not invertible modulo $m$, taking the
    /// dividend by reference, the divisor by value, and the modulus by reference. The coefficients
    /// of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient and remainder are unique. When $m$ is 1, every polynomial is 0, and
    /// so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = (&p).checked_mod_div_rem(q, &Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_div_rem(q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_div_rem(
        self,
        other: NaturalPolynomial,
        m: &Natural,
    ) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
        checked_helper(self, &other, m)
    }
}

impl CheckedModDivRem<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not invertible modulo $m$, taking the
    /// dividend by reference, the divisor by reference, and the modulus by value. The coefficients
    /// of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient and remainder are unique. When $m$ is 1, every polynomial is 0, and
    /// so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = (&p).checked_mod_div_rem(&q, Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_div_rem(&q, Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_div_rem(
        self,
        other: &NaturalPolynomial,
        m: Natural,
    ) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
        checked_helper(self, other, &m)
    }
}

impl CheckedModDivRem<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type DivOutput = NaturalPolynomial;
    type RemOutput = NaturalPolynomial;

    /// Divides two [`NaturalPolynomial`]s modulo $m$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not invertible modulo $m$, taking the
    /// dividend by reference, the divisor by reference, and the modulus by reference. The
    /// coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient and remainder are unique. When $m$ is 1, every polynomial is 0, and
    /// so is the result.
    ///
    /// When $m$ fits in one or two limbs, the division is done in word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial). Otherwise
    /// the quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
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
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivRem;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let p = NaturalPolynomial::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = NaturalPolynomial::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = (&p).checked_mod_div_rem(&q, &Natural::from(7u32)).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = NaturalPolynomial::from_str("x^2+1").unwrap();
    /// let q = NaturalPolynomial::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_div_rem(&q, &Natural::from(6u32)).is_none());
    /// ```
    ///
    /// This is equivalent to `fmpz_mod_poly_divrem` from `fmpz_mod_poly/divrem.c`, FLINT 3.6.0,
    /// except that `None` is returned where FLINT aborts.
    #[inline]
    fn checked_mod_div_rem(
        self,
        other: &NaturalPolynomial,
        m: &Natural,
    ) -> Option<(NaturalPolynomial, NaturalPolynomial)> {
        checked_helper(self, other, m)
    }
}
