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
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{DivExact, DivisibleBy, Pow, SubMulAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{PseudoDivAssignRem, PseudoDivRem};

// The pseudo-quotient, of `xs.len() - ys.len() + 1` coefficients, the pseudo-remainder, of
// `ys.len() - 1` coefficients, and the exponent $d$, of the polynomials with coefficients `xs` and
// `ys`, where `ys` is nonempty, its last coefficient $\ell$ is nonzero, and `xs` is at least as
// long. The leading coefficient of the remainder is cancelled one term at a time; when it is not
// divisible by $\ell$, everything so far is first multiplied by $\ell$, so $d$ is the least
// exponent for which the quotient is integral. If `exact` is set, the division stops, returning
// `None`, instead of multiplying, so that it succeeds exactly when the quotient of the polynomials
// over the rationals is integral.
//
// This is equivalent to `_fmpz_poly_pseudo_divrem_basecase` from
// `fmpz_poly/pseudo_divrem_basecase.c`, FLINT 3.6.0, and, with `exact` set, to
// `_fmpz_poly_divrem_basecase` from `fmpz_poly/divrem_basecase.c` with `exact` set.
pub(crate) fn basecase(
    xs: &[Integer],
    ys: &[Integer],
    exact: bool,
) -> Option<(Vec<Integer>, Vec<Integer>, u64)> {
    let lead = ys.last().unwrap();
    let ys_low = &ys[..ys.len() - 1];
    let mut qs = vec![Integer::ZERO; xs.len() - ys.len() + 1];
    let mut rs = xs.to_vec();
    let mut d = 0;
    for i in (0..qs.len()).rev() {
        let top = i + ys_low.len();
        if (&rs[top]).divisible_by(lead) {
            qs[i] = (&rs[top]).div_exact(lead);
        } else if exact {
            return None;
        } else {
            for q in &mut qs[i + 1..] {
                *q *= lead;
            }
            qs[i] = rs[top].clone();
            for r in &mut rs[..top] {
                *r *= lead;
            }
            d += 1;
        }
        for (r, y) in rs[i..top].iter_mut().zip(ys_low) {
            r.sub_mul_assign(y, &qs[i]);
        }
    }
    rs.truncate(ys_low.len());
    Some((qs, rs, d))
}

// `xs`, with its first `n` coefficients replaced by zeros.
fn zero_bottom(xs: &[Integer], n: usize) -> Vec<Integer> {
    let mut out = xs.to_vec();
    for x in &mut out[..n] {
        *x = Integer::ZERO;
    }
    out
}

// Multiplies every element of `xs` by `c`.
fn scale(xs: &mut [Integer], c: &Integer) {
    if *c != 1u32 {
        for x in xs {
            *x *= c;
        }
    }
}

// The pseudo-quotient, pseudo-remainder, and exponent of the polynomials with coefficients `xs` and
// `ys`, as for `basecase`, including `exact`: the divisor is split in two, and the high part of the
// dividend is divided by the divisor's high part recursively. The exponent is still the least. When
// the quotient has at most as many coefficients as the divisor's high part, they depend only on the
// coefficients that the high part reaches, so the recursive division scales whenever the basecase
// would; otherwise the remainder after the first recursive division, scaled, is the basecase's
// state at that point, and the second division continues from it.
//
// This is equivalent to `__fmpz_poly_pseudo_divrem_divconquer` from
// `fmpz_poly/pseudo_divrem_divconquer.c`, FLINT 3.6.0.
fn divide_and_conquer(
    xs: &[Integer],
    ys: &[Integer],
    exact: bool,
) -> Option<(Vec<Integer>, Vec<Integer>, u64)> {
    let x_len = xs.len();
    let y_len = ys.len();
    if y_len <= 16 || (x_len > (y_len << 1) - 1 && x_len < 128) {
        return basecase(xs, ys, exact);
    }
    let lead = ys.last().unwrap();
    let n2 = y_len >> 1;
    let n1 = y_len - n2;
    Some(if x_len < y_len + n2 {
        // The quotient has at most `n2` coefficients: divide the high part of the dividend by the
        // high `n2` coefficients of the divisor, then account for the low `n1`.
        let p1 = zero_bottom(&xs[n1..], n2 - 1);
        let (qs, r1, d) = divide_and_conquer(&p1, &ys[n1..], exact)?;
        let f = lead.pow(d);
        let mut rs = xs[..y_len - 1].to_vec();
        scale(&mut rs, &f);
        for (r, x) in rs[n1..].iter_mut().zip(r1) {
            *r += x;
        }
        for (r, x) in rs.iter_mut().zip(mul_ref_ref(&ys[..n1], &qs)) {
            *r -= x;
        }
        (qs, rs, d)
    } else if x_len > (y_len << 1) - 1 {
        // Divide the top `2 * y_len - 1` coefficients of the dividend, then the rest.
        let shift = x_len - (y_len << 1) + 1;
        let p1 = zero_bottom(&xs[shift..], y_len - 1);
        let (mut q1, r1, s1) = divide_and_conquer(&p1, ys, exact)?;
        let mut t = xs[..x_len - y_len].to_vec();
        scale(&mut t, &lead.pow(s1));
        for (t, r) in t[shift..].iter_mut().zip(r1) {
            *t += r;
        }
        let (mut qs, rs, s2) = divide_and_conquer(&t, ys, exact)?;
        scale(&mut q1, &lead.pow(s2));
        qs.extend(q1);
        (qs, rs, s1 + s2)
    } else {
        // Divide the top coefficients of the dividend by the high `n1` coefficients of the divisor,
        // then the rest by the whole divisor.
        let p1 = zero_bottom(&xs[n2 << 1..], n1 - 1);
        let (mut q1, r1, s1) = divide_and_conquer(&p1, &ys[n2..], exact)?;
        let d2q1 = mul_ref_ref(&ys[..n2], &q1);
        let mut t = xs[..y_len + n2 - 1].to_vec();
        scale(&mut t, &lead.pow(s1));
        for (t, r) in t[n2 << 1..].iter_mut().zip(r1) {
            *t += r;
        }
        for (t, x) in t[n2..].iter_mut().zip(d2q1) {
            *t -= x;
        }
        let (mut qs, rs, s2) = divide_and_conquer(&t, ys, exact)?;
        scale(&mut q1, &lead.pow(s2));
        qs.extend(q1);
        (qs, rs, s1 + s2)
    })
}

// The pseudo-quotient, pseudo-remainder, and least exponent of the polynomials with coefficients
// `xs` and `ys`, where `ys` is nonempty, its last coefficient is nonzero, and `xs` is at least as
// long, by `basecase`.
crate_test_fn! {pseudo_div_rem_basecase(
    xs: &[Integer],
    ys: &[Integer],
) -> (Vec<Integer>, Vec<Integer>, u64) {
    basecase(xs, ys, false).unwrap()
}}

// The pseudo-quotient, pseudo-remainder, and least exponent of the polynomials with coefficients
// `xs` and `ys`, as for `pseudo_div_rem_basecase`, by `divide_and_conquer`.
crate_test_fn! {pseudo_div_rem_divide_and_conquer(
    xs: &[Integer],
    ys: &[Integer],
) -> (Vec<Integer>, Vec<Integer>, u64) {
    divide_and_conquer(xs, ys, false).unwrap()
}}

// The quotient and remainder of the polynomials with coefficients `xs` and `ys`, as for
// `pseudo_div_rem_basecase`, or `None` if the quotient over the rationals is not integral, by
// `basecase`.
crate_test_fn! {div_rem_basecase(
    xs: &[Integer],
    ys: &[Integer],
) -> Option<(Vec<Integer>, Vec<Integer>)> {
    basecase(xs, ys, true).map(|(qs, rs, _)| (qs, rs))
}}

// The quotient and remainder of the polynomials with coefficients `xs` and `ys`, or `None` if the
// quotient over the rationals is not integral, by `divide_and_conquer`.
crate_test_fn! {div_rem_divide_and_conquer(
    xs: &[Integer],
    ys: &[Integer],
) -> Option<(Vec<Integer>, Vec<Integer>)> {
    divide_and_conquer(xs, ys, true).map(|(qs, rs, _)| (qs, rs))
}}

// The polynomial with coefficients `xs`, trimmed.
pub(crate) fn trimmed(xs: Vec<Integer>) -> IntegerPolynomial {
    let mut p = IntegerPolynomial { coefficients: xs };
    p.trim();
    p
}

// The pseudo-quotient, pseudo-remainder, and least exponent of the polynomials with coefficients
// `xs` and `ys`, with `kernel` doing the work in the general case.
//
// This is equivalent to `fmpz_poly_pseudo_divrem` from `fmpz_poly.h`, FLINT 3.6.0.
crate_test_fn! {pseudo_div_rem_with(
    xs: &[Integer],
    ys: &[Integer],
    kernel: fn(&[Integer], &[Integer]) -> (Vec<Integer>, Vec<Integer>, u64),
) -> (IntegerPolynomial, IntegerPolynomial, u64) {
    assert!(!ys.is_empty(), "division by zero");
    if xs.len() < ys.len() {
        return (IntegerPolynomial::ZERO, trimmed(xs.to_vec()), 0);
    }
    let (qs, rs, d) = kernel(xs, ys);
    (trimmed(qs), trimmed(rs), d)
}}

pub(crate) fn helper(
    p: &IntegerPolynomial,
    q: &IntegerPolynomial,
) -> (IntegerPolynomial, IntegerPolynomial, u64) {
    pseudo_div_rem_with(
        &p.coefficients,
        &q.coefficients,
        pseudo_div_rem_divide_and_conquer,
    )
}

impl PseudoDivRem<Self> for IntegerPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Pseudo-divides two [`IntegerPolynomial`]s, returning the pseudo-quotient, the
    /// pseudo-remainder, and the exponent, taking the dividend by value and the divisor by value.
    ///
    /// $$
    /// f(a, b) = (q, r, d), \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// where $\ell$ is the leading coefficient of $b$.
    ///
    /// The exponent $d$ is the least for which the pseudo-quotient has integral coefficients; once
    /// it is fixed, the pseudo-quotient and pseudo-remainder are unique. It is at most $\deg a -
    /// \deg b + 1$, and it is 0 when $\ell$ is $\pm 1$ or $b$ divides $a$. When $\deg a < \deg b$,
    /// the pseudo-quotient is 0, the pseudo-remainder is $a$, and $d$ is 0.
    ///
    /// Short divisors are handled one coefficient at a time, multiplying by $\ell$ only when the
    /// next coefficient of the quotient would not be integral; longer ones are split in half
    /// recursively, which finds the same exponent.
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
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PseudoDivRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let (s, r, d) = p.pseudo_div_rem(q);
    /// assert_eq!(s.to_string(), "4*x^2-2*x-1");
    /// assert_eq!(r.to_string(), "11*x+9");
    /// assert_eq!(d, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pseudo_divrem` from `fmpz_poly.h`, FLINT 3.6.0.
    #[inline]
    fn pseudo_div_rem(self, other: Self) -> (Self, Self, u64) {
        helper(&self, &other)
    }
}

impl PseudoDivRem<&Self> for IntegerPolynomial {
    type DivOutput = Self;
    type RemOutput = Self;

    /// Pseudo-divides two [`IntegerPolynomial`]s, returning the pseudo-quotient, the
    /// pseudo-remainder, and the exponent, taking the dividend by value and the divisor by
    /// reference.
    ///
    /// $$
    /// f(a, b) = (q, r, d), \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// where $\ell$ is the leading coefficient of $b$.
    ///
    /// The exponent $d$ is the least for which the pseudo-quotient has integral coefficients; once
    /// it is fixed, the pseudo-quotient and pseudo-remainder are unique. It is at most $\deg a -
    /// \deg b + 1$, and it is 0 when $\ell$ is $\pm 1$ or $b$ divides $a$. When $\deg a < \deg b$,
    /// the pseudo-quotient is 0, the pseudo-remainder is $a$, and $d$ is 0.
    ///
    /// Short divisors are handled one coefficient at a time, multiplying by $\ell$ only when the
    /// next coefficient of the quotient would not be integral; longer ones are split in half
    /// recursively, which finds the same exponent.
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
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PseudoDivRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let (s, r, d) = p.pseudo_div_rem(&q);
    /// assert_eq!(s.to_string(), "4*x^2-2*x-1");
    /// assert_eq!(r.to_string(), "11*x+9");
    /// assert_eq!(d, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pseudo_divrem` from `fmpz_poly.h`, FLINT 3.6.0.
    #[inline]
    fn pseudo_div_rem(self, other: &Self) -> (Self, Self, u64) {
        helper(&self, other)
    }
}

impl PseudoDivRem<IntegerPolynomial> for &IntegerPolynomial {
    type DivOutput = IntegerPolynomial;
    type RemOutput = IntegerPolynomial;

    /// Pseudo-divides two [`IntegerPolynomial`]s, returning the pseudo-quotient, the
    /// pseudo-remainder, and the exponent, taking the dividend by reference and the divisor by
    /// value.
    ///
    /// $$
    /// f(a, b) = (q, r, d), \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// where $\ell$ is the leading coefficient of $b$.
    ///
    /// The exponent $d$ is the least for which the pseudo-quotient has integral coefficients; once
    /// it is fixed, the pseudo-quotient and pseudo-remainder are unique. It is at most $\deg a -
    /// \deg b + 1$, and it is 0 when $\ell$ is $\pm 1$ or $b$ divides $a$. When $\deg a < \deg b$,
    /// the pseudo-quotient is 0, the pseudo-remainder is $a$, and $d$ is 0.
    ///
    /// Short divisors are handled one coefficient at a time, multiplying by $\ell$ only when the
    /// next coefficient of the quotient would not be integral; longer ones are split in half
    /// recursively, which finds the same exponent.
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
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PseudoDivRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let (s, r, d) = (&p).pseudo_div_rem(q);
    /// assert_eq!(s.to_string(), "4*x^2-2*x-1");
    /// assert_eq!(r.to_string(), "11*x+9");
    /// assert_eq!(d, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pseudo_divrem` from `fmpz_poly.h`, FLINT 3.6.0.
    #[inline]
    fn pseudo_div_rem(
        self,
        other: IntegerPolynomial,
    ) -> (IntegerPolynomial, IntegerPolynomial, u64) {
        helper(self, &other)
    }
}

impl PseudoDivRem<&IntegerPolynomial> for &IntegerPolynomial {
    type DivOutput = IntegerPolynomial;
    type RemOutput = IntegerPolynomial;

    /// Pseudo-divides two [`IntegerPolynomial`]s, returning the pseudo-quotient, the
    /// pseudo-remainder, and the exponent, taking the dividend by reference and the divisor by
    /// reference.
    ///
    /// $$
    /// f(a, b) = (q, r, d), \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// where $\ell$ is the leading coefficient of $b$.
    ///
    /// The exponent $d$ is the least for which the pseudo-quotient has integral coefficients; once
    /// it is fixed, the pseudo-quotient and pseudo-remainder are unique. It is at most $\deg a -
    /// \deg b + 1$, and it is 0 when $\ell$ is $\pm 1$ or $b$ divides $a$. When $\deg a < \deg b$,
    /// the pseudo-quotient is 0, the pseudo-remainder is $a$, and $d$ is 0.
    ///
    /// Short divisors are handled one coefficient at a time, multiplying by $\ell$ only when the
    /// next coefficient of the quotient would not be integral; longer ones are split in half
    /// recursively, which finds the same exponent.
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
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PseudoDivRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let (s, r, d) = (&p).pseudo_div_rem(&q);
    /// assert_eq!(s.to_string(), "4*x^2-2*x-1");
    /// assert_eq!(r.to_string(), "11*x+9");
    /// assert_eq!(d, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pseudo_divrem` from `fmpz_poly.h`, FLINT 3.6.0.
    #[inline]
    fn pseudo_div_rem(
        self,
        other: &IntegerPolynomial,
    ) -> (IntegerPolynomial, IntegerPolynomial, u64) {
        helper(self, other)
    }
}

impl PseudoDivAssignRem<Self> for IntegerPolynomial {
    type RemOutput = Self;

    /// Pseudo-divides an [`IntegerPolynomial`] by another in place, replacing it with the
    /// pseudo-quotient and returning the pseudo-remainder and the exponent, taking the divisor by
    /// value.
    ///
    /// $$
    /// a \gets q, \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// returning $(r, d)$, where $\ell$ is the leading coefficient of $b$.
    ///
    /// The exponent $d$ is the least for which the pseudo-quotient has integral coefficients; once
    /// it is fixed, the pseudo-quotient and pseudo-remainder are unique. It is at most $\deg a -
    /// \deg b + 1$, and it is 0 when $\ell$ is $\pm 1$ or $b$ divides $a$. When $\deg a < \deg b$,
    /// the pseudo-quotient is 0, the pseudo-remainder is $a$, and $d$ is 0.
    ///
    /// Short divisors are handled one coefficient at a time, multiplying by $\ell$ only when the
    /// next coefficient of the quotient would not be integral; longer ones are split in half
    /// recursively, which finds the same exponent.
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
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PseudoDivAssignRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let (r, d) = p.pseudo_div_assign_rem(q);
    /// assert_eq!(p.to_string(), "4*x^2-2*x-1");
    /// assert_eq!(r.to_string(), "11*x+9");
    /// assert_eq!(d, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pseudo_divrem` from `fmpz_poly.h`, FLINT 3.6.0.
    #[inline]
    fn pseudo_div_assign_rem(&mut self, other: Self) -> (Self, u64) {
        let (q, r, d) = helper(self, &other);
        *self = q;
        (r, d)
    }
}

impl PseudoDivAssignRem<&Self> for IntegerPolynomial {
    type RemOutput = Self;

    /// Pseudo-divides an [`IntegerPolynomial`] by another in place, replacing it with the
    /// pseudo-quotient and returning the pseudo-remainder and the exponent, taking the divisor by
    /// reference.
    ///
    /// $$
    /// a \gets q, \quad \ell^d a = bq + r, \quad \deg r < \deg b,
    /// $$
    /// returning $(r, d)$, where $\ell$ is the leading coefficient of $b$.
    ///
    /// The exponent $d$ is the least for which the pseudo-quotient has integral coefficients; once
    /// it is fixed, the pseudo-quotient and pseudo-remainder are unique. It is at most $\deg a -
    /// \deg b + 1$, and it is 0 when $\ell$ is $\pm 1$ or $b$ divides $a$. When $\deg a < \deg b$,
    /// the pseudo-quotient is 0, the pseudo-remainder is $a$, and $d$ is 0.
    ///
    /// Short divisors are handled one coefficient at a time, multiplying by $\ell$ only when the
    /// next coefficient of the quotient would not be integral; longer ones are split in half
    /// recursively, which finds the same exponent.
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
    /// Panics if `other` is 0.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::PseudoDivAssignRem;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^4+x+1").unwrap();
    /// let q = IntegerPolynomial::from_str("2*x^2+x+1").unwrap();
    /// let (r, d) = p.pseudo_div_assign_rem(&q);
    /// assert_eq!(p.to_string(), "4*x^2-2*x-1");
    /// assert_eq!(r.to_string(), "11*x+9");
    /// assert_eq!(d, 3);
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_pseudo_divrem` from `fmpz_poly.h`, FLINT 3.6.0.
    #[inline]
    fn pseudo_div_assign_rem(&mut self, other: &Self) -> (Self, u64) {
        let (q, r, d) = helper(self, other);
        *self = q;
        (r, d)
    }
}
