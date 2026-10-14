// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Sebastian Pancratz
//
//      Copyright © 2016 Shivin Srivastava
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_polynomial::IntegerPolynomial;
use crate::integer_polynomial::arithmetic::add::add_assign_val;
use crate::integer_polynomial::arithmetic::evaluate::evaluate_integer;
use crate::integer_polynomial::arithmetic::mul::{mul_greater_to_out, mul_ref_ref};
use crate::integer_polynomial::arithmetic::square::square_ref;
use crate::integer_polynomial::arithmetic::taylor_shift::taylor_shift_in_place;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use alloc::vec::Vec;
use core::cmp::max;
use core::mem::{swap, take};
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::Zero;
use malachite_base::polynomial::{Compose, ComposeAssign};

// Whether a polynomial with `xs_len` coefficients is composed with Horner's rule rather than divide
// and conquer, when the largest coefficient of either polynomial has `bits` bits. Divide and
// conquer squares powers of the inner polynomial instead of multiplying by it again and again,
// which pays off sooner the larger the coefficients: measured on this machine, from about 8
// coefficients when they have a few bits, 7 at a word, and 3 from about a thousand bits, with the
// inner length mattering much less. FLINT switches at 5 coefficients whatever their size.
pub(crate) const fn compose_horner_preferred(xs_len: usize, bits: u64) -> bool {
    let max_len = if bits <= 32 {
        7
    } else if bits <= 128 {
        6
    } else if bits <= 512 {
        4
    } else {
        2
    };
    xs_len <= max_len
}

// The number of coefficients of the composition of polynomials with `xs_len` and `ys_len`
// coefficients, both positive, when the outer polynomial's leading coefficient is nonzero.
const fn composition_len(xs_len: usize, ys_len: usize) -> usize {
    (xs_len - 1) * (ys_len - 1) + 1
}

// The coefficients of the composition of the polynomials with coefficients `xs` and `ys`, both
// nonempty, by Horner's rule over whole polynomials: $((x_{n-1} y + x_{n-2}) y + \cdots) y + x_0$.
// The result has `composition_len(xs.len(), ys.len())` coefficients and is not trimmed.
//
// This is equivalent to `_fmpz_poly_compose_horner` from `fmpz_poly/compose_horner.c`, FLINT 3.6.0,
// except that the running polynomial and the scratch product trade places at every step instead of
// the product being copied back.
crate_test_fn! {compose_horner(xs: &[Integer], ys: &[Integer]) -> Vec<Integer> {
    let (leading, rest) = xs.split_last().unwrap();
    let Some((second, rest)) = rest.split_last() else {
        return vec![leading.clone()];
    };
    let step = ys.len() - 1;
    let mut acc: Vec<Integer> = ys.iter().map(|y| y * leading).collect();
    acc[0] += second;
    let mut scratch = Vec::with_capacity(composition_len(xs.len(), ys.len()));
    for x in rest.iter().rev() {
        // The product overwrites every element, so stale ones may stay.
        scratch.resize(acc.len() + step, Integer::ZERO);
        mul_greater_to_out(&mut scratch, &acc, ys);
        scratch[0] += x;
        swap(&mut acc, &mut scratch);
    }
    acc
}}

// The coefficients of the composition of the polynomials with coefficients `xs` and `ys`, both
// nonempty, by divide and conquer: each coefficient of `xs` starts as a constant group; at every
// level, neighboring groups $g_{2i}$ and $g_{2i+1}$ merge into $g_{2i} + y^{2^k} g_{2i+1}$, and the
// power of `ys` is squared, until one group is left. Zero groups are kept empty, so they cost
// nothing. The result has `composition_len(xs.len(), ys.len())` coefficients and is not trimmed.
//
// This is equivalent to `_fmpz_poly_compose_divconquer` from `fmpz_poly/compose_divconquer.c`,
// FLINT 3.6.0, except that each group has its own `Vec` of its actual length, so no lengths are
// precomputed, and that the first level, which FLINT does separately, is the same merge with the
// power $y$.
crate_test_fn! {compose_divide_and_conquer(xs: &[Integer], ys: &[Integer]) -> Vec<Integer> {
    let mut groups: Vec<Vec<Integer>> = xs
        .iter()
        .map(|x| if *x == 0u32 { Vec::new() } else { vec![x.clone()] })
        .collect();
    let mut power: Vec<Integer> = ys.to_vec();
    loop {
        let mut merged = Vec::with_capacity(groups.len().div_ceil(2));
        let mut groups_iter = groups.into_iter();
        while let Some(mut lo) = groups_iter.next() {
            if let Some(hi) = groups_iter.next()
                && !hi.is_empty()
            {
                add_assign_val(&mut lo, mul_ref_ref(&power, &hi));
            }
            merged.push(lo);
        }
        groups = merged;
        if groups.len() == 1 {
            break;
        }
        power = square_ref(&power);
    }
    let mut out = groups.pop().unwrap();
    // Only an outer polynomial whose coefficients are all 0 leaves the group short.
    out.resize(composition_len(xs.len(), ys.len()), Integer::ZERO);
    out
}}

// Composes the polynomial with coefficients `xs`, of length at least 2, with $bx + a$, where `ys`
// is `[a, b]` and $b \neq 0$, in place: a Taylor shift by $a$, after which the coefficient of $x^i$
// is multiplied by $b^i$. The leading coefficient becomes a nonzero multiple of the old one, so the
// result needs no trimming.
fn compose_linear_in_place(xs: &mut [Integer], ys: &[Integer]) {
    let (a, b) = (&ys[0], &ys[1]);
    if *a != 0u32 {
        taylor_shift_in_place(xs, a);
    }
    if *b == -1i32 {
        for x in xs.iter_mut().skip(1).step_by(2) {
            x.neg_assign();
        }
    } else if *b != 1u32 {
        let mut power = b.clone();
        for x in &mut xs[1..] {
            *x *= &power;
            power *= b;
        }
    }
}

// Whether the composition of the polynomials with coefficients `xs` and `ys` is done in place by
// `compose_linear_in_place`: when `ys` is linear and `xs` is not constant.
const fn is_linear_case(xs: &[Integer], ys: &[Integer]) -> bool {
    xs.len() > 1 && ys.len() == 2
}

// Composes the polynomial with coefficients `xs` with the one with coefficients `ys`, unless that
// is the linear case.
//
// This is equivalent to `fmpz_poly_compose` and `_fmpz_poly_compose` from `fmpz_poly/compose.c`,
// FLINT 3.6.0, except that a linear `ys` is handled by the callers and that the choice between
// Horner's rule and divide and conquer depends on the size of the coefficients.
fn compose_nonlinear(xs: &[Integer], ys: &[Integer]) -> IntegerPolynomial {
    match (xs, ys) {
        ([], _) => IntegerPolynomial::ZERO,
        ([x], _) | ([x, ..], []) => IntegerPolynomial::from(x.clone()),
        (_, [y]) => IntegerPolynomial::from(evaluate_integer(xs, y)),
        _ => {
            // The leading coefficient of the result is that of `xs` times a power of that of `ys`,
            // so the result needs no trimming.
            let bits = max(vec_max_bits(xs).0, vec_max_bits(ys).0);
            let coefficients = if compose_horner_preferred(xs.len(), bits) {
                compose_horner(xs, ys)
            } else {
                compose_divide_and_conquer(xs, ys)
            };
            IntegerPolynomial { coefficients }
        }
    }
}

fn compose_ref(xs: &[Integer], ys: &[Integer]) -> IntegerPolynomial {
    if is_linear_case(xs, ys) {
        compose_val(xs.to_vec(), ys)
    } else {
        compose_nonlinear(xs, ys)
    }
}

fn compose_val(mut xs: Vec<Integer>, ys: &[Integer]) -> IntegerPolynomial {
    if is_linear_case(&xs, ys) {
        compose_linear_in_place(&mut xs, ys);
        IntegerPolynomial { coefficients: xs }
    } else {
        compose_nonlinear(&xs, ys)
    }
}

impl Compose<Self> for IntegerPolynomial {
    type Output = Self;

    /// Composes two [`IntegerPolynomial`]s, giving $p(q(x))$, taking both by value.
    ///
    /// $$
    /// f(p, q) = p(q(x)).
    /// $$
    ///
    /// When $q$ is a constant $c$, the result is the constant $p(c)$. When $q$ is linear, $bx + a$,
    /// the result is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is
    /// multiplied by $b^i$; otherwise short polynomials are composed with Horner's rule and longer
    /// ones by divide and conquer, which merges neighboring groups of coefficients with ever higher
    /// powers $q^{2^j}$. The integers have no zero divisors, so if $q$ is not constant and $p$ is
    /// nonzero, the degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// coefficient of either polynomial: $s$ bounds the size of the coefficients of the result,
    /// which has about $nk$ of them, and the cost is that of the last multiplication of divide and
    /// conquer, or of the Taylor shift, each of that size.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Compose;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2-3*x+2")
    ///         .unwrap()
    ///         .compose(IntegerPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "4*x^2+14*x+12"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^3")
    ///         .unwrap()
    ///         .compose(IntegerPolynomial::from_str("x^2+x").unwrap())
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Composing with a constant evaluates.
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2+1")
    ///         .unwrap()
    ///         .compose(IntegerPolynomial::from_str("3").unwrap())
    ///         .to_string(),
    ///     "10"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_compose` from `fmpz_poly/compose.c`, FLINT 3.6.0.
    #[inline]
    fn compose(self, other: Self) -> Self {
        compose_val(self.coefficients, &other.coefficients)
    }
}

impl Compose<&Self> for IntegerPolynomial {
    type Output = Self;

    /// Composes two [`IntegerPolynomial`]s, giving $p(q(x))$, taking the first by value and the
    /// second by reference.
    ///
    /// $$
    /// f(p, q) = p(q(x)).
    /// $$
    ///
    /// When $q$ is a constant $c$, the result is the constant $p(c)$. When $q$ is linear, $bx + a$,
    /// the result is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is
    /// multiplied by $b^i$; otherwise short polynomials are composed with Horner's rule and longer
    /// ones by divide and conquer, which merges neighboring groups of coefficients with ever higher
    /// powers $q^{2^j}$. The integers have no zero divisors, so if $q$ is not constant and $p$ is
    /// nonzero, the degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// coefficient of either polynomial: $s$ bounds the size of the coefficients of the result,
    /// which has about $nk$ of them, and the cost is that of the last multiplication of divide and
    /// conquer, or of the Taylor shift, each of that size.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Compose;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2-3*x+2")
    ///         .unwrap()
    ///         .compose(&IntegerPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "4*x^2+14*x+12"
    /// );
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^3")
    ///         .unwrap()
    ///         .compose(&IntegerPolynomial::from_str("x^2+x").unwrap())
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Composing with a constant evaluates.
    /// assert_eq!(
    ///     IntegerPolynomial::from_str("x^2+1")
    ///         .unwrap()
    ///         .compose(&IntegerPolynomial::from_str("3").unwrap())
    ///         .to_string(),
    ///     "10"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_compose` from `fmpz_poly/compose.c`, FLINT 3.6.0.
    #[inline]
    fn compose(self, other: &Self) -> Self {
        compose_val(self.coefficients, &other.coefficients)
    }
}

impl Compose<IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Composes two [`IntegerPolynomial`]s, giving $p(q(x))$, taking the first by reference and the
    /// second by value.
    ///
    /// $$
    /// f(p, q) = p(q(x)).
    /// $$
    ///
    /// When $q$ is a constant $c$, the result is the constant $p(c)$. When $q$ is linear, $bx + a$,
    /// the result is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is
    /// multiplied by $b^i$; otherwise short polynomials are composed with Horner's rule and longer
    /// ones by divide and conquer, which merges neighboring groups of coefficients with ever higher
    /// powers $q^{2^j}$. The integers have no zero divisors, so if $q$ is not constant and $p$ is
    /// nonzero, the degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// coefficient of either polynomial: $s$ bounds the size of the coefficients of the result,
    /// which has about $nk$ of them, and the cost is that of the last multiplication of divide and
    /// conquer, or of the Taylor shift, each of that size.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Compose;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .compose(IntegerPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "4*x^2+14*x+12"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^3").unwrap())
    ///         .compose(IntegerPolynomial::from_str("x^2+x").unwrap())
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Composing with a constant evaluates.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2+1").unwrap())
    ///         .compose(IntegerPolynomial::from_str("3").unwrap())
    ///         .to_string(),
    ///     "10"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_compose` from `fmpz_poly/compose.c`, FLINT 3.6.0.
    #[inline]
    fn compose(self, other: IntegerPolynomial) -> IntegerPolynomial {
        compose_ref(&self.coefficients, &other.coefficients)
    }
}

impl Compose<&IntegerPolynomial> for &IntegerPolynomial {
    type Output = IntegerPolynomial;

    /// Composes two [`IntegerPolynomial`]s, giving $p(q(x))$, taking both by reference.
    ///
    /// $$
    /// f(p, q) = p(q(x)).
    /// $$
    ///
    /// When $q$ is a constant $c$, the result is the constant $p(c)$. When $q$ is linear, $bx + a$,
    /// the result is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is
    /// multiplied by $b^i$; otherwise short polynomials are composed with Horner's rule and longer
    /// ones by divide and conquer, which merges neighboring groups of coefficients with ever higher
    /// powers $q^{2^j}$. The integers have no zero divisors, so if $q$ is not constant and $p$ is
    /// nonzero, the degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// coefficient of either polynomial: $s$ bounds the size of the coefficients of the result,
    /// which has about $nk$ of them, and the cost is that of the last multiplication of divide and
    /// conquer, or of the Taylor shift, each of that size.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::Compose;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2-3*x+2").unwrap())
    ///         .compose(&IntegerPolynomial::from_str("2*x+5").unwrap())
    ///         .to_string(),
    ///     "4*x^2+14*x+12"
    /// );
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^3").unwrap())
    ///         .compose(&IntegerPolynomial::from_str("x^2+x").unwrap())
    ///         .to_string(),
    ///     "x^6+3*x^5+3*x^4+x^3"
    /// );
    /// // Composing with a constant evaluates.
    /// assert_eq!(
    ///     (&IntegerPolynomial::from_str("x^2+1").unwrap())
    ///         .compose(&IntegerPolynomial::from_str("3").unwrap())
    ///         .to_string(),
    ///     "10"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_compose` from `fmpz_poly/compose.c`, FLINT 3.6.0.
    #[inline]
    fn compose(self, other: &IntegerPolynomial) -> IntegerPolynomial {
        compose_ref(&self.coefficients, &other.coefficients)
    }
}

impl ComposeAssign<Self> for IntegerPolynomial {
    /// Composes an [`IntegerPolynomial`] with another in place, replacing $p$ with $p(q(x))$,
    /// taking the other by value.
    ///
    /// $$
    /// p \\gets p(q(x)).
    /// $$
    ///
    /// When $q$ is a constant $c$, the result is the constant $p(c)$. When $q$ is linear, $bx + a$,
    /// the result is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is
    /// multiplied by $b^i$; otherwise short polynomials are composed with Horner's rule and longer
    /// ones by divide and conquer, which merges neighboring groups of coefficients with ever higher
    /// powers $q^{2^j}$. The integers have no zero divisors, so if $q$ is not constant and $p$ is
    /// nonzero, the degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// coefficient of either polynomial: $s$ bounds the size of the coefficients of the result,
    /// which has about $nk$ of them, and the cost is that of the last multiplication of divide and
    /// conquer, or of the Taylor shift, each of that size.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.compose_assign(IntegerPolynomial::from_str("2*x+5").unwrap());
    /// assert_eq!(p.to_string(), "4*x^2+14*x+12");
    ///
    /// let mut p = IntegerPolynomial::from_str("x^3").unwrap();
    /// p.compose_assign(IntegerPolynomial::from_str("x^2+x").unwrap());
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Composing with a constant evaluates.
    /// let mut p = IntegerPolynomial::from_str("x^2+1").unwrap();
    /// p.compose_assign(IntegerPolynomial::from_str("3").unwrap());
    /// assert_eq!(p.to_string(), "10");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_compose` from `fmpz_poly/compose.c`, FLINT 3.6.0.
    #[inline]
    fn compose_assign(&mut self, other: Self) {
        *self = compose_val(take(&mut self.coefficients), &other.coefficients);
    }
}

impl ComposeAssign<&Self> for IntegerPolynomial {
    /// Composes an [`IntegerPolynomial`] with another in place, replacing $p$ with $p(q(x))$,
    /// taking the other by reference.
    ///
    /// $$
    /// p \\gets p(q(x)).
    /// $$
    ///
    /// When $q$ is a constant $c$, the result is the constant $p(c)$. When $q$ is linear, $bx + a$,
    /// the result is a Taylor shift of $p$ by $a$, after which the coefficient of $x^i$ is
    /// multiplied by $b^i$; otherwise short polynomials are composed with Horner's rule and longer
    /// ones by divide and conquer, which merges neighboring groups of coefficients with ever higher
    /// powers $q^{2^j}$. The integers have no zero divisors, so if $q$ is not constant and $p$ is
    /// nonzero, the degree of the result is the product of the degrees.
    ///
    /// # Worst-case complexity
    /// $T(n, k, s) = O((nk)^{\log_2 3} s \log s \log\log s)$
    ///
    /// $M(n, k, s) = O(nk(s + \log (nk)) \log (nks))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.len()`, $k$ is `other.len()`, and
    /// $s$ is $n(m + \log (nk))$, where $m$ is the largest number of significant bits of any
    /// coefficient of either polynomial: $s$ bounds the size of the coefficients of the result,
    /// which has about $nk$ of them, and the cost is that of the last multiplication of divide and
    /// conquer, or of the Taylor shift, each of that size.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ComposeAssign;
    /// use malachite_nz::integer_polynomial::IntegerPolynomial;
    ///
    /// let mut p = IntegerPolynomial::from_str("x^2-3*x+2").unwrap();
    /// p.compose_assign(&IntegerPolynomial::from_str("2*x+5").unwrap());
    /// assert_eq!(p.to_string(), "4*x^2+14*x+12");
    ///
    /// let mut p = IntegerPolynomial::from_str("x^3").unwrap();
    /// p.compose_assign(&IntegerPolynomial::from_str("x^2+x").unwrap());
    /// assert_eq!(p.to_string(), "x^6+3*x^5+3*x^4+x^3");
    ///
    /// // Composing with a constant evaluates.
    /// let mut p = IntegerPolynomial::from_str("x^2+1").unwrap();
    /// p.compose_assign(&IntegerPolynomial::from_str("3").unwrap());
    /// assert_eq!(p.to_string(), "10");
    /// ```
    ///
    /// This is equivalent to `fmpz_poly_compose` from `fmpz_poly/compose.c`, FLINT 3.6.0.
    #[inline]
    fn compose_assign(&mut self, other: &Self) {
        *self = compose_val(take(&mut self.coefficients), &other.coefficients);
    }
}
