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

use crate::num::arithmetic::traits::ModIsReduced;
use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{CheckedModDivRem, ModDivAssignRem, ModDivRem};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_div_truncated::mod_div_truncated_dispatch;
use crate::unsigned_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_to_out;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;
use alloc::vec;
use alloc::vec::Vec;

pub(crate) fn assert_reduced<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) {
    assert!(
        p.mod_is_reduced(&m),
        "self must be reduced mod m, but {p} has a coefficient >= {m}"
    );
    assert!(
        q.mod_is_reduced(&m),
        "other must be reduced mod m, but {q} has a coefficient >= {m}"
    );
}

// The quotient modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, where `ys`
// is nonempty and no longer than `xs`, and `inverse` is the inverse of the last coefficient of `ys`
// modulo `m`. The quotient has `xs.len() - ys.len() + 1` coefficients; reversed, it is the quotient
// of the reversals of the polynomials as power series, which `series` computes.
//
// This is equivalent to `_gr_poly_div_newton` from `gr_poly/div_newton.c`, FLINT 3.6.0, with
// `series` in place of `_gr_poly_div_series`.
crate_test_fn! {mod_quotient_with<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    m: T,
    inverse: T,
    series: fn(&[T], &[T], usize, T, T) -> Vec<T>,
) -> Vec<T> {
    let q_len = xs.len() - ys.len() + 1;
    let xs_rev: Vec<T> = xs[xs.len() - q_len..].iter().rev().copied().collect();
    let ys_rev: Vec<T> = ys[ys.len().saturating_sub(q_len)..].iter().rev().copied().collect();
    let mut q = series(&xs_rev, &ys_rev, q_len, m, inverse);
    q.reverse();
    q
}}

// The remainder modulo `m` of the polynomials with coefficients `xs` and `ys`, reduced, given their
// quotient `qs`: the `ys.len() - 1` coefficients of `xs - qs * ys` that are not cancelled.
//
// This is equivalent to the remainder step of `_gr_poly_divrem_newton` from
// `gr_poly/divrem_newton.c`, FLINT 3.6.0.
pub(crate) fn remainder<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], qs: &[T], m: T) -> Vec<T> {
    let r_len = ys.len() - 1;
    if r_len == 0 {
        return Vec::new();
    }
    let mut rs = vec![T::ZERO; r_len];
    mod_mul_truncated_to_out(&mut rs, qs, &ys[..r_len], m);
    for (r, &x) in rs.iter_mut().zip(xs) {
        *r = x.mod_sub(*r, m);
    }
    rs
}

// The quotient and remainder modulo `m` of the polynomials with coefficients `xs` and `ys`,
// reduced, where `ys` is nonempty and no longer than `xs`, and `inverse` is the inverse of the last
// coefficient of `ys` modulo `m`, with `series` dividing the reversals as power series.
//
// This is equivalent to `_gr_poly_divrem_newton` from `gr_poly/divrem_newton.c`, FLINT 3.6.0, with
// `series` in place of `_gr_poly_div_series`.
crate_test_fn! {mod_div_rem_with_series<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    m: T,
    inverse: T,
    series: fn(&[T], &[T], usize, T, T) -> Vec<T>,
) -> (Vec<T>, Vec<T>) {
    let qs = mod_quotient_with(xs, ys, m, inverse, series);
    let rs = remainder(xs, ys, &qs, m);
    (qs, rs)
}}

// The inverse modulo `m` of the leading coefficient of the polynomial with coefficients `ys`, or
// `None` if `ys` is empty or its leading coefficient is not invertible.
pub(crate) fn leading_inverse<T: PrimitiveUnsigned>(ys: &[T], m: T) -> Option<T> {
    ys.last()?.mod_inverse(m)
}

// The quotient and remainder modulo `m` of the polynomials with coefficients `xs` and `ys`,
// reduced, with `series` dividing the reversals as power series in the general case, or `None` if
// `ys` is zero or its leading coefficient is not invertible. Modulo 1, every polynomial is 0, and
// so is the result.
//
// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, except that
// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
crate_test_fn! {mod_div_rem_with<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    m: T,
    series: fn(&[T], &[T], usize, T, T) -> Vec<T>,
) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
    if m == T::ONE {
        return Some((UnsignedPolynomial::ZERO, UnsignedPolynomial::ZERO));
    }
    let inverse = leading_inverse(ys, m)?;
    if xs.len() < ys.len() {
        return Some((UnsignedPolynomial::ZERO, from_coefficients_trimmed(xs.to_vec())));
    }
    let (qs, rs) = mod_div_rem_with_series(xs, ys, m, inverse, series);
    Some((from_coefficients_trimmed(qs), from_coefficients_trimmed(rs)))
}}

/// The coefficients of the quotient and remainder modulo `m` of the polynomials with coefficients
/// `xs` and `ys`, without zeros at the end, or `None` if `ys` is empty or its last coefficient is
/// not invertible. The coefficients must be reduced modulo `m`, `m` must be positive, and neither
/// `xs` nor `ys` may end with a zero.
///
/// This is not part of the public API; it is public so that `malachite-nz` can divide
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
#[inline]
pub fn mod_div_rem_vec<T: PrimitiveUnsigned>(xs: &[T], ys: &[T], m: T) -> Option<(Vec<T>, Vec<T>)> {
    mod_div_rem_with(xs, ys, m, mod_div_truncated_dispatch)
        .map(|(q, r)| (q.coefficients, r.coefficients))
}

fn checked_helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
    assert_reduced(p, q, m);
    mod_div_rem_with(
        &p.coefficients,
        &q.coefficients,
        m,
        mod_div_truncated_dispatch,
    )
}

fn helper<T: PrimitiveUnsigned>(
    p: &UnsignedPolynomial<T>,
    q: &UnsignedPolynomial<T>,
    m: T,
) -> (UnsignedPolynomial<T>, UnsignedPolynomial<T>) {
    assert!(
        m == T::ONE || *q != UnsignedPolynomial::ZERO,
        "division by zero"
    );
    checked_helper(p, q, m)
        .expect("the leading coefficient of the divisor must be invertible modulo m")
}

impl<T: PrimitiveUnsigned> ModDivRem<Self, T> for UnsignedPolynomial<T> {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient and remainder, taking
    /// the dividend by value and the divisor by value. The coefficients of both must already be
    /// reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = p.mod_div_rem(q, 7);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_rem(self, other: Self, m: T) -> (Self, Self) {
        helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivRem<&Self, T> for UnsignedPolynomial<T> {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient and remainder, taking
    /// the dividend by value and the divisor by reference. The coefficients of both must already be
    /// reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = p.mod_div_rem(&q, 7);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_rem(self, other: &Self, m: T) -> (Self, Self) {
        helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivRem<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type DivOutput = UnsignedPolynomial<T>;
    type RemOutput = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient and remainder, taking
    /// the dividend by reference and the divisor by value. The coefficients of both must already be
    /// reduced modulo $m$, and the leading coefficient of the divisor must be invertible modulo
    /// $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem(q, 7);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_rem(
        self,
        other: UnsignedPolynomial<T>,
        m: T,
    ) -> (UnsignedPolynomial<T>, UnsignedPolynomial<T>) {
        helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivRem<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type DivOutput = UnsignedPolynomial<T>;
    type RemOutput = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient and remainder, taking
    /// the dividend by reference and the divisor by reference. The coefficients of both must
    /// already be reduced modulo $m$, and the leading coefficient of the divisor must be invertible
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = (&p).mod_div_rem(&q, 7);
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_rem(
        self,
        other: &UnsignedPolynomial<T>,
        m: T,
    ) -> (UnsignedPolynomial<T>, UnsignedPolynomial<T>) {
        helper(self, other, m)
    }
}

impl<T: PrimitiveUnsigned> ModDivAssignRem<Self, T> for UnsignedPolynomial<T> {
    type RemOutput = Self;

    /// Divides an [`UnsignedPolynomial`] by another modulo $m$ in place, replacing it with the
    /// quotient and returning the remainder, taking the divisor by value. The coefficients of both
    /// must already be reduced modulo $m$, and the leading coefficient of the divisor must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let r = p.mod_div_assign_rem(q, 7);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_assign_rem(&mut self, other: Self, m: T) -> Self {
        let (q, r) = helper(self, &other, m);
        *self = q;
        r
    }
}

impl<T: PrimitiveUnsigned> ModDivAssignRem<&Self, T> for UnsignedPolynomial<T> {
    type RemOutput = Self;

    /// Divides an [`UnsignedPolynomial`] by another modulo $m$ in place, replacing it with the
    /// quotient and returning the remainder, taking the divisor by reference. The coefficients of
    /// both must already be reduced modulo $m$, and the leading coefficient of the divisor must be
    /// invertible modulo $m$.
    ///
    /// $$
    /// a \gets q, \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b,
    /// $$
    /// returning $r$.
    ///
    /// When $m$ is 1, every polynomial is 0, and so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
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
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let r = p.mod_div_assign_rem(&q, 7);
    /// assert_eq!(p.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn mod_div_assign_rem(&mut self, other: &Self, m: T) -> Self {
        let (q, r) = helper(self, other, m);
        *self = q;
        r
    }
}

impl<T: PrimitiveUnsigned> CheckedModDivRem<Self, T> for UnsignedPolynomial<T> {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not invertible modulo $m$, taking the
    /// dividend by value and the divisor by value. The coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient and remainder are unique. When $m$ is 1, every polynomial is 0, and
    /// so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = p.checked_mod_div_rem(q, 7).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_div_rem(q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_div_rem(self, other: Self, m: T) -> Option<(Self, Self)> {
        checked_helper(&self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModDivRem<&Self, T> for UnsignedPolynomial<T> {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not invertible modulo $m$, taking the
    /// dividend by value and the divisor by reference. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient and remainder are unique. When $m$ is 1, every polynomial is 0, and
    /// so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = p.checked_mod_div_rem(&q, 7).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!(p.checked_mod_div_rem(&q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_div_rem(self, other: &Self, m: T) -> Option<(Self, Self)> {
        checked_helper(&self, other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModDivRem<UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type DivOutput = UnsignedPolynomial<T>;
    type RemOutput = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not invertible modulo $m$, taking the
    /// dividend by reference and the divisor by value. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient and remainder are unique. When $m$ is 1, every polynomial is 0, and
    /// so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = (&p).checked_mod_div_rem(q, 7).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_div_rem(q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_div_rem(
        self,
        other: UnsignedPolynomial<T>,
        m: T,
    ) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
        checked_helper(self, &other, m)
    }
}

impl<T: PrimitiveUnsigned> CheckedModDivRem<&UnsignedPolynomial<T>, T> for &UnsignedPolynomial<T> {
    type DivOutput = UnsignedPolynomial<T>;
    type RemOutput = UnsignedPolynomial<T>;

    /// Divides two [`UnsignedPolynomial`]s modulo $m$, returning the quotient and remainder, or
    /// `None` if the divisor's leading coefficient is not invertible modulo $m$, taking the
    /// dividend by reference and the divisor by reference. The coefficients of both must already be
    /// reduced modulo $m$.
    ///
    /// $$
    /// f(a, b, m) = (q, r), \quad a \equiv bq + r \pmod{m}, \quad \deg r < \deg b.
    /// $$
    ///
    /// Returns `None` if the divisor is 0 or its leading coefficient is not invertible modulo $m$;
    /// otherwise the quotient and remainder are unique. When $m$ is 1, every polynomial is 0, and
    /// so is the result.
    ///
    /// The quotient is found by dividing the reversed polynomials as power series, coefficient by
    /// coefficient for short divisors and otherwise by the method of Karp and Markstein, and the
    /// remainder from it with one truncated product.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.len()`.
    ///
    /// # Panics
    /// Panics if `m` is 0, or if any coefficient of `self` or `other` is greater than or equal to
    /// `m`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::CheckedModDivRem;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let p = UnsignedPolynomial::<u8>::from_str("x^5+3*x^4+2*x+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("3*x^2+x+1").unwrap();
    /// let (s, r) = (&p).checked_mod_div_rem(&q, 7).unwrap();
    /// assert_eq!(s.to_string(), "5*x^3+4*x^2+4*x+2");
    /// assert_eq!(r.to_string(), "3*x+6");
    ///
    /// // The leading coefficient of the divisor is not invertible.
    /// let p = UnsignedPolynomial::<u8>::from_str("x^2+1").unwrap();
    /// let q = UnsignedPolynomial::<u8>::from_str("2*x+1").unwrap();
    /// assert!((&p).checked_mod_div_rem(&q, 6).is_none());
    /// ```
    ///
    /// This is equivalent to `nmod_poly_divrem` from `nmod_poly/divrem.c`, FLINT 3.6.0, except that
    /// `None` is returned where FLINT aborts or assumes the leading coefficient invertible.
    #[inline]
    fn checked_mod_div_rem(
        self,
        other: &UnsignedPolynomial<T>,
        m: T,
    ) -> Option<(UnsignedPolynomial<T>, UnsignedPolynomial<T>)> {
        checked_helper(self, other, m)
    }
}
