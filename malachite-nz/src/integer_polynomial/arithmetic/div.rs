// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2008, 2009 William Hart
//
//      Copyright © 2010, 2011 Sebastian Pancratz
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::mul::mul_ref_ref;
use crate::integer_polynomial::arithmetic::pseudo_div_rem::{basecase, trimmed};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::ops::{Div, DivAssign};
use malachite_base::num::arithmetic::traits::{CheckedDiv, DivExact, DivisibleBy, SubMulAssign};
use malachite_base::num::basic::traits::Zero;

// The length of divisor at and below which the divide-and-conquer quotients divide one coefficient
// at a time.
//
// This is `FLINT_DIV_DIVCONQUER_CUTOFF` from `fmpz_poly/div_divconquer_recursive.c` and
// `FLINT_DIVREMLOW_DIVCONQUER_CUTOFF` from `fmpz_poly/divremlow_divconquer_recursive.c`, FLINT
// 3.6.0.
const DIV_DIVIDE_AND_CONQUER_CUTOFF: usize = 16;

// The quotient, of `xs.len() - ys.len() + 1` coefficients, of the polynomials with coefficients
// `xs` and `ys`, where `ys` is nonempty, its last coefficient $\ell$ is nonzero, and `xs` is at
// least as long, or `None` if the quotient over the rationals is not integral. The leading
// coefficient of the remainder is cancelled one term at a time, each time dividing it exactly by
// $\ell$. Coefficient $i$ of the quotient is read from coefficient $i + n - 1$ of the remainder,
// where $n$ is `ys.len()`, so the remainder's coefficients below $x^{n - 1}$ are never kept.
//
// This is equivalent to `_fmpz_poly_div_basecase` from `fmpz_poly/div_basecase.c`, FLINT 3.6.0,
// with `exact` set, except that FLINT also updates some of the remainder's coefficients below $x^{n
// - 1}$.
fn div_basecase(xs: &[Integer], ys: &[Integer]) -> Option<Vec<Integer>> {
    let lead = ys.last().unwrap();
    let ys_low = &ys[..ys.len() - 1];
    let n_1 = ys_low.len();
    let mut rs = xs[n_1..].to_vec();
    let mut qs = vec![Integer::ZERO; rs.len()];
    for i in (0..qs.len()).rev() {
        let top = &rs[i];
        if !top.divisible_by(lead) {
            return None;
        }
        if *top != 0u32 {
            let q = top.div_exact(lead);
            let j = n_1.saturating_sub(i);
            for (r, y) in rs[i + j - n_1..i].iter_mut().zip(&ys_low[j..]) {
                r.sub_mul_assign(y, &q);
            }
            qs[i] = q;
        }
    }
    Some(qs)
}

// The quotient, of $n$ coefficients, of the polynomial with coefficients `xs`, of length $2n - 1$,
// by the one with coefficients `ys`, of length $n$, whose last coefficient is nonzero, and the low
// $n - 1$ coefficients of the product of the quotient and `ys`; or `None` if the quotient over the
// rationals is not integral. The divisor is split into a low part of `n2` coefficients and a high
// part of `n1`; the top of the dividend is divided by the high part, the rest of the dividend is
// corrected by the partial products, and its top is divided again.
//
// This is equivalent to `_fmpz_poly_divremlow_divconquer_recursive` from
// `fmpz_poly/divremlow_divconquer_recursive.c`, FLINT 3.6.0, with `exact` set.
fn div_rem_low_divide_and_conquer(
    xs: &[Integer],
    ys: &[Integer],
) -> Option<(Vec<Integer>, Vec<Integer>)> {
    let n = ys.len();
    if n <= DIV_DIVIDE_AND_CONQUER_CUTOFF {
        let (qs, rs, _) = basecase(xs, ys, true)?;
        let low = xs.iter().zip(rs).map(|(x, r)| x - r).collect();
        return Some((qs, low));
    }
    let n2 = n >> 1;
    let n1 = n - n2;
    // The high quotient, and the low `n1 - 1` coefficients of its product with the high part.
    let (q1, d1q1) = div_rem_low_divide_and_conquer(&xs[n2 << 1..], &ys[n2..])?;
    let d2q1 = mul_ref_ref(&q1, &ys[..n2]);
    // The top `n2` coefficients of the dividend minus x^n2 times the product of the high quotient
    // and the divisor; the low `n2 - 1` coefficients do not affect the next quotient.
    let mut p2 = vec![Integer::ZERO; (n2 << 1) - 1];
    for k in 0..n2 {
        let mut t = &xs[n - 1 + k] - &d2q1[n1 - 1 + k];
        if let Some(j) = (k + n1).checked_sub(n2 + 1) {
            t -= &d1q1[j];
        }
        p2[n2 - 1 + k] = t;
    }
    let (q2, d3q2) = div_rem_low_divide_and_conquer(&p2, &ys[n1..])?;
    let mut low = mul_ref_ref(&ys[..n1], &q2);
    for (l, x) in low[n2..].iter_mut().zip(&d2q1[..n1 - 1]) {
        *l += x;
    }
    for (l, x) in low[n1..].iter_mut().zip(d3q2) {
        *l += x;
    }
    let mut qs = q2;
    qs.extend(q1);
    Some((qs, low))
}

// The quotient, of $n$ coefficients, of the polynomial with coefficients `xs`, of length $2n - 1$,
// by the one with coefficients `ys`, of length $n$, whose last coefficient is nonzero, or `None` if
// the quotient over the rationals is not integral. The top of the dividend is divided by the top
// `n1` coefficients of the divisor, keeping the low coefficients of the product; the top of what
// remains is then divided by the top `n2` coefficients recursively.
//
// This is equivalent to `_fmpz_poly_div_divconquer_recursive` from
// `fmpz_poly/div_divconquer_recursive.c`, FLINT 3.6.0, with `exact` set.
fn div_divide_and_conquer_recursive(xs: &[Integer], ys: &[Integer]) -> Option<Vec<Integer>> {
    let n = ys.len();
    if n <= DIV_DIVIDE_AND_CONQUER_CUTOFF {
        return div_basecase(xs, ys);
    }
    let n2 = n >> 1;
    let n1 = n - n2;
    let (q1, d1q1) = div_rem_low_divide_and_conquer(&xs[n2 << 1..], &ys[n2..])?;
    let d2q1 = mul_ref_ref(&q1, &ys[..n2]);
    // The top `n2` coefficients of the remainder after subtracting x^n2 times the product of the
    // high quotient and the divisor; the low `n2 - 1` coefficients do not affect the next quotient.
    let mut p2 = vec![Integer::ZERO; (n2 << 1) - 1];
    for k in 0..n2 {
        let j = n - 1 + k;
        let r = if let Some(i) = j.checked_sub(n2 << 1) {
            &xs[j] - &d1q1[i]
        } else {
            xs[j].clone()
        };
        p2[n2 - 1 + k] = r - &d2q1[n1 - 1 + k];
    }
    let mut qs = div_divide_and_conquer_recursive(&p2, &ys[n1..])?;
    qs.extend(q1);
    Some(qs)
}

// The quotient of the polynomial with coefficients `xs` by the one with coefficients `ys`, whose
// last coefficient is nonzero, where `ys.len() <= xs.len() <= 2 * ys.len() - 1`, or `None` if the
// quotient over the rationals is not integral. A short quotient depends only on the top
// coefficients of the dividend and divisor, so the division is made balanced.
//
// This is equivalent to `__fmpz_poly_div_divconquer` from `fmpz_poly/div_divconquer.c`, FLINT
// 3.6.0, with `exact` set.
fn div_divide_and_conquer_balanced(xs: &[Integer], ys: &[Integer]) -> Option<Vec<Integer>> {
    let q1 = xs.len() - ys.len() + 1;
    let q2 = ys.len() - q1;
    div_divide_and_conquer_recursive(&xs[q2..], &ys[q2..])
}

// The quotient, of `xs.len() - ys.len() + 1` coefficients, of the polynomials with coefficients
// `xs` and `ys`, where `ys` is nonempty, its last coefficient is nonzero, and `xs` is at least as
// long, or `None` if the quotient over the rationals is not integral. A dividend of more than $2n -
// 1$ coefficients, where $n$ is `ys.len()`, is divided $2n - 1$ coefficients at a time, from the
// top, carrying the low coefficients of each remainder into the next window.
//
// This is equivalent to `_fmpz_poly_div_divconquer` from `fmpz_poly/div_divconquer.c`, FLINT 3.6.0,
// with `exact` set.
crate_test_fn! {div_divide_and_conquer(xs: &[Integer], ys: &[Integer]) -> Option<Vec<Integer>> {
    let y_len = ys.len();
    let n = (y_len << 1) - 1;
    let mut x_len = xs.len();
    if x_len <= n {
        return div_divide_and_conquer_balanced(xs, ys);
    }
    let mut qs = vec![Integer::ZERO; x_len - y_len + 1];
    let mut window = xs[x_len - n..].to_vec();
    while x_len >= n {
        let shift = x_len - n;
        let (q, low) = div_rem_low_divide_and_conquer(&window, ys)?;
        for (out, q) in qs[shift..].iter_mut().zip(q) {
            *out = q;
        }
        let next = min(y_len, shift);
        let mut new_window = xs[shift - next..shift].to_vec();
        new_window.extend(window[..y_len - 1].iter().zip(low).map(|(w, l)| w - l));
        window = new_window;
        x_len -= y_len;
    }
    if x_len >= y_len {
        for (out, q) in qs.iter_mut().zip(div_divide_and_conquer_balanced(&window, ys)?) {
            *out = q;
        }
    }
    Some(qs)
}}

// The quotient of the polynomials with coefficients `xs` and `ys`, as for `div_divide_and_conquer`,
// by `div_basecase`.
crate_test_fn! {div_basecase_wrapper(xs: &[Integer], ys: &[Integer]) -> Option<Vec<Integer>> {
    div_basecase(xs, ys)
}}

// The quotient of `p` by `q`, or `None` if `q` is 0 or the quotient over the rationals is not
// integral.
//
// This is equivalent to `fmpz_poly_div` from `fmpz_poly/div.c`, FLINT 3.6.0, with `exact` set in
// its call to `_fmpz_poly_div`, except that `None` is returned where FLINT would not be exact.
pub(crate) fn checked_div_helper(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> Option<IntegerPolynomial> {
    if *q == IntegerPolynomial::ZERO {
        return None;
    }
    if p.coefficients.len() < q.coefficients.len() {
        return Some(IntegerPolynomial::ZERO);
    }
    div_divide_and_conquer(&p.coefficients, &q.coefficients).map(trimmed)
}

fn div_helper(p: &IntegerPolynomial, q: &IntegerPolynomial) -> IntegerPolynomial {
    assert_ne!(*q, IntegerPolynomial::ZERO, "division by zero");
    checked_div_helper(p, q).expect("the quotient must have integer coefficients")
}

impl Div<Self> for IntegerPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`IntegerPolynomial`]s, taking the dividend by value and the
    /// divisor by value.
    ///
    /// $$
    /// f(a, b) = q, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if the quotient does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!((p / q).to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div` from `fmpz_poly/div.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn div(self, other: Self) -> Self {
        div_helper(&self, &other)
    }
}

impl Div<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`IntegerPolynomial`]s, taking the dividend by value and the
    /// divisor by reference.
    ///
    /// $$
    /// f(a, b) = q, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if the quotient does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!((p / &q).to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div` from `fmpz_poly/div.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn div(self, other: &Self) -> Self {
        div_helper(&self, other)
    }
}

impl Div<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Finds the quotient of two [`IntegerPolynomial`]s, taking the dividend by reference and the
    /// divisor by value.
    ///
    /// $$
    /// f(a, b) = q, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if the quotient does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!((&p / q).to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div` from `fmpz_poly/div.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn div(self, other: IntegerPolynomial) -> IntegerPolynomial {
        div_helper(self, &other)
    }
}

impl Div<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Finds the quotient of two [`IntegerPolynomial`]s, taking the dividend by reference and the
    /// divisor by reference.
    ///
    /// $$
    /// f(a, b) = q, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if the quotient does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!((&p / &q).to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div` from `fmpz_poly/div.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn div(self, other: &IntegerPolynomial) -> IntegerPolynomial {
        div_helper(self, other)
    }
}

impl DivAssign<Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its quotient by another, taking the divisor by value.
    ///
    /// $$
    /// a \gets q, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if the quotient does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// p /= q;
    /// assert_eq!(p.to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div` from `fmpz_poly/div.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn div_assign(&mut self, other: Self) {
        *self = div_helper(self, &other);
    }
}

impl DivAssign<&Self> for IntegerPolynomial {
    /// Replaces an [`IntegerPolynomial`] with its quotient by another, taking the divisor by
    /// reference.
    ///
    /// $$
    /// a \gets q, \quad a = bq + r, \quad \deg r < \deg b.
    /// $$
    ///
    /// The division is possible exactly when the quotient over the rationals has integer
    /// coefficients: when the divisor's leading coefficient divides the leading coefficient of
    /// every remainder along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Panics
    /// Panics if `other` is 0, or if the quotient does not have integer coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// p /= &q;
    /// assert_eq!(p.to_string(), "2*x+1");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_div` from `fmpz_poly/div.c`, FLINT 3.6.0, except that
    /// Malachite panics where FLINT's quotient would not be exact.
    #[inline]
    fn div_assign(&mut self, other: &Self) {
        *self = div_helper(self, other);
    }
}

impl CheckedDiv<Self> for IntegerPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`IntegerPolynomial`]s, or `None` if the division is impossible,
    /// taking the dividend by value and the divisor by value.
    ///
    /// Returns `None` if `other` is 0 or the quotient does not have integer coefficients. The
    /// division is possible exactly when the quotient over the rationals has integer coefficients:
    /// when the divisor's leading coefficient divides the leading coefficient of every remainder
    /// along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!(p.checked_div(q).unwrap().to_string(), "2*x+1");
    ///
    /// let p = IntegerPolynomial::from_str("x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x").unwrap();
    /// assert!(p.checked_div(q).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, with
    /// `exact` set.
    #[inline]
    fn checked_div(self, other: Self) -> Option<Self> {
        checked_div_helper(&self, &other)
    }
}

impl CheckedDiv<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Finds the quotient of two [`IntegerPolynomial`]s, or `None` if the division is impossible,
    /// taking the dividend by value and the divisor by reference.
    ///
    /// Returns `None` if `other` is 0 or the quotient does not have integer coefficients. The
    /// division is possible exactly when the quotient over the rationals has integer coefficients:
    /// when the divisor's leading coefficient divides the leading coefficient of every remainder
    /// along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!(p.checked_div(&q).unwrap().to_string(), "2*x+1");
    ///
    /// let p = IntegerPolynomial::from_str("x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x").unwrap();
    /// assert!(p.checked_div(&q).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, with
    /// `exact` set.
    #[inline]
    fn checked_div(self, other: &Self) -> Option<Self> {
        checked_div_helper(&self, other)
    }
}

impl CheckedDiv<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Finds the quotient of two [`IntegerPolynomial`]s, or `None` if the division is impossible,
    /// taking the dividend by reference and the divisor by value.
    ///
    /// Returns `None` if `other` is 0 or the quotient does not have integer coefficients. The
    /// division is possible exactly when the quotient over the rationals has integer coefficients:
    /// when the divisor's leading coefficient divides the leading coefficient of every remainder
    /// along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!((&p).checked_div(q).unwrap().to_string(), "2*x+1");
    ///
    /// let p = IntegerPolynomial::from_str("x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x").unwrap();
    /// assert!((&p).checked_div(q).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, with
    /// `exact` set.
    #[inline]
    fn checked_div(self, other: IntegerPolynomial) -> Option<IntegerPolynomial> {
        checked_div_helper(self, &other)
    }
}

impl CheckedDiv<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Finds the quotient of two [`IntegerPolynomial`]s, or `None` if the division is impossible,
    /// taking the dividend by reference and the divisor by reference.
    ///
    /// Returns `None` if `other` is 0 or the quotient does not have integer coefficients. The
    /// division is possible exactly when the quotient over the rationals has integer coefficients:
    /// when the divisor's leading coefficient divides the leading coefficient of every remainder
    /// along the way.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits in the
    /// coefficients of `self` and `other`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::CheckedDiv;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("6*x^2+3*x+4").unwrap();
    /// let q = IntegerPolynomial::from_str("3*x").unwrap();
    /// assert_eq!((&p).checked_div(&q).unwrap().to_string(), "2*x+1");
    ///
    /// let p = IntegerPolynomial::from_str("x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x").unwrap();
    /// assert!((&p).checked_div(&q).is_none());
    /// ```
    ///
    /// This is equivalent to `_fmpz_poly_divrem` from `fmpz_poly/divrem.c`, FLINT 3.6.0, with
    /// `exact` set.
    #[inline]
    fn checked_div(self, other: &IntegerPolynomial) -> Option<IntegerPolynomial> {
        checked_div_helper(self, other)
    }
}
