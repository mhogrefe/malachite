// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Sebastian Pancratz
//
//      Copyright © 2011, 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::mul::{ProductShape, matrix_mul};
use crate::natural::Natural;
use crate::natural::arithmetic::mod_mul::ModMulData;
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_add::assert_reduced;
use crate::natural_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_ref_ref;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::{swap, take};
use malachite_base::num::arithmetic::traits::{
    FloorSqrt, ModAddAssign, ModAssign, ModMulPrecomputed, ModMulPrecomputedAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::polynomial::{ModComposeTruncated, ModComposeTruncatedAssign, Polynomial};
use malachite_base::unsigned_polynomial::arithmetic::mod_compose_truncated::*;

// The first `n` coefficients of the product of the polynomials with coefficients `xs` and `ys`
// modulo `m`, without zeros at the end; empty if either is.
fn mul_truncated(xs: &[Natural], ys: &[Natural], n: usize, m: &Natural) -> Vec<Natural> {
    if xs.is_empty() || ys.is_empty() {
        Vec::new()
    } else {
        mod_mul_truncated_ref_ref(xs, ys, u64::exact_from(n), m).into_coefficients_asc()
    }
}

// Adds `ys` into `xs` modulo `m`, reusing whichever of the two is longer.
fn add_assign_val(xs: &mut Vec<Natural>, mut ys: Vec<Natural>, m: &Natural) {
    if ys.len() > xs.len() {
        swap(xs, &mut ys);
    }
    for (x, y) in xs.iter_mut().zip(ys) {
        x.mod_add_assign(y, m);
    }
}

// Adds `x` to the constant coefficient of the polynomial with coefficients `xs` modulo `m`.
fn add_constant_assign(xs: &mut Vec<Natural>, x: &Natural, m: &Natural) {
    if let Some(first) = xs.first_mut() {
        first.mod_add_assign(x, m);
    } else {
        xs.push(x.clone());
    }
}

// The first `n` coefficients of the composition, modulo `m`, of the polynomials with coefficients
// `xs` and `ys`, reduced, where `xs.len() >= 2` and `ys[0]` is 0, by Horner's rule. The result may
// be shorter than `n`.
//
// This is equivalent to `_gr_poly_compose_series_horner` from `gr_poly/compose_series_horner.c`,
// FLINT 3.6.0, with coefficients modulo `m`.
crate_test_fn! {mod_compose_truncated_horner(
    xs: &[Natural],
    ys: &[Natural],
    n: usize,
    m: &Natural,
) -> Vec<Natural> {
    let mut acc = Vec::new();
    for x in xs.iter().rev() {
        acc = mul_truncated(&acc, ys, n, m);
        add_constant_assign(&mut acc, x, m);
    }
    acc
}}

// The first `n` coefficients of the composition, modulo `m`, of the polynomials with coefficients
// `xs` and `ys`, reduced, where `xs.len() >= 3` and `ys[0]` is 0, by divide and conquer:
// neighboring pairs of coefficients $x_{2i} + x_{2i+1}y$ start as groups; at every level,
// neighboring groups $g_{2i}$ and $g_{2i+1}$ merge into $g_{2i} + y^{2^j} g_{2i+1}$, and the power
// of `ys` is squared, until one group is left, everything truncated to `n` coefficients. The result
// may be shorter than `n`.
//
// This is equivalent to `_gr_poly_compose_series_divconquer` from
// `gr_poly/compose_series_divconquer.c`, FLINT 3.6.0, with coefficients modulo `m`, except that
// each group is its own `Vec`.
crate_test_fn! {mod_compose_truncated_divide_and_conquer(
    xs: &[Natural],
    ys: &[Natural],
    n: usize,
    m: &Natural,
) -> Vec<Natural> {
    let mut groups: Vec<Vec<Natural>> = xs
        .chunks(2)
        .map(|pair| {
            let mut group = if let [_, x1] = pair {
                mul_truncated(ys, core::slice::from_ref(x1), n, m)
            } else {
                Vec::new()
            };
            add_constant_assign(&mut group, &pair[0], m);
            group
        })
        .collect();
    let mut power = mul_truncated(ys, ys, n, m);
    loop {
        let mut merged = Vec::with_capacity(groups.len().div_ceil(2));
        let mut groups_iter = groups.into_iter();
        while let Some(mut lo) = groups_iter.next() {
            if let Some(hi) = groups_iter.next() {
                add_assign_val(&mut lo, mul_truncated(&power, &hi, n, m), m);
            }
            merged.push(lo);
        }
        groups = merged;
        if groups.len() == 1 {
            break;
        }
        power = mul_truncated(&power, &power, n, m);
    }
    groups.pop().unwrap()
}}

// The first `n` coefficients of the composition, modulo `m`, of the polynomials with coefficients
// `xs` and `ys`, reduced, where `ys[0]` is 0 and `2 <= xs.len() <= n`, by the Brent–Kung
// algorithm: with $k = \lfloor \sqrt n \rfloor + 1$, the products of the $k$ blocks of $k$
// coefficients of `xs` with the first $k$ powers of `ys` are the rows of a matrix product modulo
// `m`, and they are combined by Horner's rule in $y^k$. The result may be shorter than `n`.
//
// This is equivalent to `_gr_poly_compose_series_brent_kung` from
// `gr_poly/compose_series_brent_kung.c`, FLINT 3.6.0, with coefficients modulo `m`, except that the
// powers of `ys` are always computed by multiplying by `ys`.
crate_test_fn! {mod_compose_truncated_brent_kung(
    xs: &[Natural],
    ys: &[Natural],
    n: usize,
    m: &Natural,
) -> Vec<Natural> {
    let k = n.floor_sqrt() + 1;
    // The rows of B are the blocks of xs.
    let mut b = vec![Natural::ZERO; k * k];
    for (i, chunk) in xs.chunks(k).enumerate() {
        b[i * k..i * k + chunk.len()].clone_from_slice(chunk);
    }
    // The rows of A are the powers of ys.
    let mut a = vec![Natural::ZERO; k * n];
    a[0] = Natural::ONE;
    a[n..n + ys.len()].clone_from_slice(ys);
    for i in 2..k {
        let (previous, current) = a.split_at_mut(i * n);
        for (c, p) in current.iter_mut().zip(mul_truncated(&previous[(i - 1) * n..], ys, n, m)) {
            *c = p;
        }
    }
    let mut c = matrix_mul(&b, &a, ProductShape { ar: k, br: k, bc: n });
    for x in &mut c {
        x.mod_assign(m);
    }
    // Combine the rows by Horner's rule in h = ys^k.
    let h = mul_truncated(&a[(k - 1) * n..], ys, n, m);
    let mut acc: Vec<Natural> = c.drain((k - 1) * n..).collect();
    for i in (0..k - 1).rev() {
        acc = mul_truncated(&acc, &h, n, m);
        add_assign_val(&mut acc, c.drain(i * n..).collect(), m);
    }
    acc
}}

// The first `n` coefficients of $p(cx^v)$ modulo `m`, where `xs` holds the coefficients of $p$ and
// $v \geq 1$: $p_i c^i$ at $x^{iv}$.
fn mod_compose_truncated_monomial(
    xs: &[Natural],
    c: &Natural,
    v: usize,
    n: usize,
    m: &Natural,
) -> Vec<Natural> {
    let data: ModMulData =
        <Natural as ModMulPrecomputed<&Natural, &Natural>>::precompute_mod_mul_data(&m);
    let mut out = vec![Natural::ZERO; n];
    let mut power = Natural::ONE;
    for (o, x) in out.iter_mut().step_by(v).zip(xs) {
        *o = x.mod_mul_precomputed(&power, m, &data);
        power.mod_mul_precomputed_assign(c, m, &data);
    }
    out
}

// Whether a composition with an outer polynomial of `len1` coefficients is done by Horner's rule:
// as measured on this machine, when the outer polynomial has at most 4 coefficients, rather than
// FLINT's `len1 <= 7 || n <= 7`.
const fn horner_preferred(len1: usize) -> bool {
    len1 <= 4
}

// Whether a composition with an outer polynomial of `len1` coefficients and an inner polynomial of
// `len2` coefficients, truncated to `n` coefficients, is done by divide and conquer rather than by
// the Brent–Kung algorithm. Divide and conquer costs about $\min(\ell_1, \ell/n)$ products of
// length $n$, where $\ell$ is the length of the untruncated composition, and the Brent–Kung
// algorithm about $2\sqrt n$ of them plus a matrix product; as measured on this machine, divide and
// conquer is faster when $\ell_1^2 \leq 4n$ or $\ell/n \leq 0.8\sqrt n$. FLINT chooses it when
// $\ell_1^2 < n$ or $\ell < 4n$.
const fn divide_and_conquer_preferred(len1: usize, len2: usize, n: usize) -> bool {
    let q = (len1 - 1)
        .saturating_mul(len2 - 1)
        .saturating_add(1)
        .saturating_mul(5)
        / n;
    len1.saturating_mul(len1) <= n << 2 || q.saturating_mul(q) <= n << 4
}

// The first `n` coefficients of the composition, modulo `m`, of the polynomials with coefficients
// `xs` and `ys`, reduced, where `xs.len() >= 2` and `ys[0]` is 0: directly if `ys` has a single
// nonzero coefficient, and otherwise by Horner's rule, divide and conquer, or the Brent–Kung
// algorithm.
//
// This is equivalent to `_gr_poly_compose_series` from `gr_poly/compose_series.c`, FLINT 3.6.0,
// with coefficients modulo `m`, except that the monomial case allows a monomial of any degree, that
// the choice of algorithm was measured (see `horner_preferred` and `divide_and_conquer_preferred`),
// and that the Kinoshita–Li algorithm is not used.
fn mod_compose_truncated_dispatch(
    xs: &[Natural],
    ys: &[Natural],
    n: usize,
    m: &Natural,
) -> Vec<Natural> {
    let v = ys.iter().position(|y| *y != 0u32).unwrap();
    if ys[v + 1..].iter().all(|y| *y == 0u32) {
        mod_compose_truncated_monomial(xs, &ys[v], v, n, m)
    } else if horner_preferred(xs.len()) {
        mod_compose_truncated_horner(xs, ys, n, m)
    } else if divide_and_conquer_preferred(xs.len(), ys.len(), n) {
        mod_compose_truncated_divide_and_conquer(xs, ys, n, m)
    } else {
        mod_compose_truncated_brent_kung(xs, ys, n, m)
    }
}

// The composition, modulo `m`, of the polynomials with coefficients `xs` and `ys`, reduced, keeping
// only the coefficients of $x^i$ for $i$ less than `len`, with `kernel` doing the work in the
// general case. If $v$ is the index of the first nonzero coefficient of `ys`, the term $p_i q^i$ is
// divisible by $x^{iv}$, so only the first $\lceil n/v \rceil$ coefficients of `xs` are read.
//
// This is equivalent to `nmod_poly_compose_series` from `nmod_poly/compose_series.c`, FLINT 3.6.0,
// with any modulus, except that `xs` is truncated using $v$.
crate_test_fn! {mod_compose_truncated_with(
    xs: &[Natural],
    ys: &[Natural],
    len: u64,
    m: &Natural,
    kernel: fn(&[Natural], &[Natural], usize, &Natural) -> Vec<Natural>,
) -> NaturalPolynomial {
    if let Some(y) = ys.first() {
        assert_eq!(*y, 0u32, "the inner polynomial must have constant coefficient 0");
    }
    let n = usize::try_from(len).unwrap_or(usize::MAX);
    if xs.is_empty() || n == 0 {
        return NaturalPolynomial::ZERO;
    }
    // The inner polynomial may have zeros at the end, or be all zeros, if it was truncated.
    if let Some(v) = ys.iter().position(|y| *y != 0u32) {
        let xs = &xs[..min(xs.len(), (n - 1) / v + 1)];
        if xs.len() > 1 {
            let lenr = min((xs.len() - 1).saturating_mul(ys.len() - 1) + 1, n);
            let xs = &xs[..min(xs.len(), lenr)];
            let ys = &ys[..min(ys.len(), lenr)];
            let mut p = NaturalPolynomial {
                coefficients: kernel(xs, ys, lenr, m),
            };
            p.trim();
            return p;
        }
    }
    NaturalPolynomial::from(xs[0].clone())
}}

// The composition, modulo `m`, of the polynomials with coefficients `xs` and `ys`, both reduced,
// keeping only the coefficients of $x^i$ for $i$ less than `len`. When `m` fits in a limb, this is
// done with limbs.
fn mod_compose_truncated_helper(
    xs: &[Natural],
    ys: &[Natural],
    len: u64,
    m: &Natural,
) -> NaturalPolynomial {
    if m.significant_bits() <= Limb::WIDTH {
        // Only the first `len` coefficients of either polynomial can matter, but at least one, so
        // that a nonzero constant coefficient of the inner polynomial is still rejected.
        let kept = usize::try_from(len).unwrap_or(usize::MAX).max(1);
        let xs: Vec<Limb> = xs.iter().take(kept).map(Limb::wrapping_from).collect();
        let ys: Vec<Limb> = ys.iter().take(kept).map(Limb::wrapping_from).collect();
        NaturalPolynomial {
            coefficients: mod_compose_truncated_vec(&xs, &ys, len, Limb::wrapping_from(m))
                .into_iter()
                .map(Natural::from)
                .collect(),
        }
    } else {
        mod_compose_truncated_with(xs, ys, len, m, mod_compose_truncated_dispatch)
    }
}

impl ModComposeTruncated<Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo $m$ and truncates the result to its first `len`
    /// coefficients, taking all three by value. The inner polynomial must have constant coefficient
    /// 0, and the coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. Nothing is divided modulo $m$, so any modulus works, prime or not.
    /// The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^{5/2} b \\log b \\log\\log b)$
    ///
    /// $M(n, b) = O(n^{3/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_compose_truncated(NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, m)
    ///         .to_string(),
    ///     "x^3+6*x^2+5*x+2"
    /// );
    ///
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_compose_truncated(NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, m)
    ///         .to_string(),
    ///     "155*x^3+76*x^2+5*x+2"
    /// );
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let m = Natural::from(6u32);
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+2*x+1")
    ///         .unwrap()
    ///         .mod_compose_truncated(NaturalPolynomial::from_str("x^2+2*x").unwrap(), 3, m)
    ///         .to_string(),
    ///     "4*x+1"
    /// );
    /// ```
    #[inline]
    fn mod_compose_truncated(self, other: Self, len: u64, m: Natural) -> Self {
        (&self).mod_compose_truncated(&other, len, &m)
    }
}

impl ModComposeTruncated<Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo $m$ and truncates the result to its first `len`
    /// coefficients, taking the first and the second by value and the modulus by reference. The
    /// inner polynomial must have constant coefficient 0, and the coefficients of both must already
    /// be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. Nothing is divided modulo $m$, so any modulus works, prime or not.
    /// The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^{5/2} b \\log b \\log\\log b)$
    ///
    /// $M(n, b) = O(n^{3/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_compose_truncated(NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, &m)
    ///         .to_string(),
    ///     "x^3+6*x^2+5*x+2"
    /// );
    ///
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_compose_truncated(NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, &m)
    ///         .to_string(),
    ///     "155*x^3+76*x^2+5*x+2"
    /// );
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let m = Natural::from(6u32);
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+2*x+1")
    ///         .unwrap()
    ///         .mod_compose_truncated(NaturalPolynomial::from_str("x^2+2*x").unwrap(), 3, &m)
    ///         .to_string(),
    ///     "4*x+1"
    /// );
    /// ```
    #[inline]
    fn mod_compose_truncated(self, other: Self, len: u64, m: &Natural) -> Self {
        (&self).mod_compose_truncated(&other, len, m)
    }
}

impl ModComposeTruncated<&Self, Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo $m$ and truncates the result to its first `len`
    /// coefficients, taking the first and the modulus by value and the second by reference. The
    /// inner polynomial must have constant coefficient 0, and the coefficients of both must already
    /// be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. Nothing is divided modulo $m$, so any modulus works, prime or not.
    /// The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^{5/2} b \\log b \\log\\log b)$
    ///
    /// $M(n, b) = O(n^{3/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_compose_truncated(&NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, m)
    ///         .to_string(),
    ///     "x^3+6*x^2+5*x+2"
    /// );
    ///
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_compose_truncated(&NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, m)
    ///         .to_string(),
    ///     "155*x^3+76*x^2+5*x+2"
    /// );
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let m = Natural::from(6u32);
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+2*x+1")
    ///         .unwrap()
    ///         .mod_compose_truncated(&NaturalPolynomial::from_str("x^2+2*x").unwrap(), 3, m)
    ///         .to_string(),
    ///     "4*x+1"
    /// );
    /// ```
    #[inline]
    fn mod_compose_truncated(self, other: &Self, len: u64, m: Natural) -> Self {
        (&self).mod_compose_truncated(other, len, &m)
    }
}

impl ModComposeTruncated<&Self, &Natural> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo $m$ and truncates the result to its first `len`
    /// coefficients, taking the first by value and the second and the modulus by reference. The
    /// inner polynomial must have constant coefficient 0, and the coefficients of both must already
    /// be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. Nothing is divided modulo $m$, so any modulus works, prime or not.
    /// The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^{5/2} b \\log b \\log\\log b)$
    ///
    /// $M(n, b) = O(n^{3/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_compose_truncated(&NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, &m)
    ///         .to_string(),
    ///     "x^3+6*x^2+5*x+2"
    /// );
    ///
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_compose_truncated(&NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, &m)
    ///         .to_string(),
    ///     "155*x^3+76*x^2+5*x+2"
    /// );
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let m = Natural::from(6u32);
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+2*x+1")
    ///         .unwrap()
    ///         .mod_compose_truncated(&NaturalPolynomial::from_str("x^2+2*x").unwrap(), 3, &m)
    ///         .to_string(),
    ///     "4*x+1"
    /// );
    /// ```
    #[inline]
    fn mod_compose_truncated(self, other: &Self, len: u64, m: &Natural) -> Self {
        (&self).mod_compose_truncated(other, len, m)
    }
}

impl ModComposeTruncated<NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo $m$ and truncates the result to its first `len`
    /// coefficients, taking the second and the modulus by value and the first by reference. The
    /// inner polynomial must have constant coefficient 0, and the coefficients of both must already
    /// be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. Nothing is divided modulo $m$, so any modulus works, prime or not.
    /// The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^{5/2} b \\log b \\log\\log b)$
    ///
    /// $M(n, b) = O(n^{3/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_compose_truncated(NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, m)
    ///         .to_string(),
    ///     "x^3+6*x^2+5*x+2"
    /// );
    ///
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_compose_truncated(NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, m)
    ///         .to_string(),
    ///     "155*x^3+76*x^2+5*x+2"
    /// );
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let m = Natural::from(6u32);
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+2*x+1").unwrap())
    ///         .mod_compose_truncated(NaturalPolynomial::from_str("x^2+2*x").unwrap(), 3, m)
    ///         .to_string(),
    ///     "4*x+1"
    /// );
    /// ```
    #[inline]
    fn mod_compose_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        m: Natural,
    ) -> NaturalPolynomial {
        self.mod_compose_truncated(&other, len, &m)
    }
}

impl ModComposeTruncated<NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo $m$ and truncates the result to its first `len`
    /// coefficients, taking the second by value and the first and the modulus by reference. The
    /// inner polynomial must have constant coefficient 0, and the coefficients of both must already
    /// be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. Nothing is divided modulo $m$, so any modulus works, prime or not.
    /// The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^{5/2} b \\log b \\log\\log b)$
    ///
    /// $M(n, b) = O(n^{3/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_compose_truncated(NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, &m)
    ///         .to_string(),
    ///     "x^3+6*x^2+5*x+2"
    /// );
    ///
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_compose_truncated(NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, &m)
    ///         .to_string(),
    ///     "155*x^3+76*x^2+5*x+2"
    /// );
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let m = Natural::from(6u32);
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+2*x+1").unwrap())
    ///         .mod_compose_truncated(NaturalPolynomial::from_str("x^2+2*x").unwrap(), 3, &m)
    ///         .to_string(),
    ///     "4*x+1"
    /// );
    /// ```
    #[inline]
    fn mod_compose_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        m: &Natural,
    ) -> NaturalPolynomial {
        self.mod_compose_truncated(&other, len, m)
    }
}

impl ModComposeTruncated<&NaturalPolynomial, Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo $m$ and truncates the result to its first `len`
    /// coefficients, taking the modulus by value and the first and the second by reference. The
    /// inner polynomial must have constant coefficient 0, and the coefficients of both must already
    /// be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. Nothing is divided modulo $m$, so any modulus works, prime or not.
    /// The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^{5/2} b \\log b \\log\\log b)$
    ///
    /// $M(n, b) = O(n^{3/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_compose_truncated(&NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, m)
    ///         .to_string(),
    ///     "x^3+6*x^2+5*x+2"
    /// );
    ///
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_compose_truncated(&NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, m)
    ///         .to_string(),
    ///     "155*x^3+76*x^2+5*x+2"
    /// );
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let m = Natural::from(6u32);
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+2*x+1").unwrap())
    ///         .mod_compose_truncated(&NaturalPolynomial::from_str("x^2+2*x").unwrap(), 3, m)
    ///         .to_string(),
    ///     "4*x+1"
    /// );
    /// ```
    #[inline]
    fn mod_compose_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        m: Natural,
    ) -> NaturalPolynomial {
        self.mod_compose_truncated(other, len, &m)
    }
}

impl ModComposeTruncated<&NaturalPolynomial, &Natural> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo $m$ and truncates the result to its first `len`
    /// coefficients, taking all three by reference. The inner polynomial must have constant
    /// coefficient 0, and the coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. Nothing is divided modulo $m$, so any modulus works, prime or not.
    /// The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^{5/2} b \\log b \\log\\log b)$
    ///
    /// $M(n, b) = O(n^{3/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncated;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_compose_truncated(&NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, &m)
    ///         .to_string(),
    ///     "x^3+6*x^2+5*x+2"
    /// );
    ///
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_compose_truncated(&NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, &m)
    ///         .to_string(),
    ///     "155*x^3+76*x^2+5*x+2"
    /// );
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let m = Natural::from(6u32);
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+2*x+1").unwrap())
    ///         .mod_compose_truncated(&NaturalPolynomial::from_str("x^2+2*x").unwrap(), 3, &m)
    ///         .to_string(),
    ///     "4*x+1"
    /// );
    /// ```
    fn mod_compose_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        m: &Natural,
    ) -> NaturalPolynomial {
        assert_reduced(self, other, m);
        mod_compose_truncated_helper(&self.coefficients, &other.coefficients, len, m)
    }
}

impl ModComposeTruncatedAssign<Self, Natural> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another modulo $m$ in place and truncates the result
    /// to its first `len` coefficients, taking both by value. The inner polynomial must have
    /// constant coefficient 0, and the coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. Nothing is divided modulo $m$, so any modulus works, prime or not.
    /// The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^{5/2} b \\log b \\log\\log b)$
    ///
    /// $M(n, b) = O(n^{3/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// let mut p = NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_compose_truncated_assign(NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, m);
    /// assert_eq!(p.to_string(), "x^3+6*x^2+5*x+2");
    ///
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// let mut p = NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_compose_truncated_assign(NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, m);
    /// assert_eq!(p.to_string(), "155*x^3+76*x^2+5*x+2");
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let m = Natural::from(6u32);
    /// let mut p = NaturalPolynomial::from_str("x^2+2*x+1").unwrap();
    /// p.mod_compose_truncated_assign(NaturalPolynomial::from_str("x^2+2*x").unwrap(), 3, m);
    /// assert_eq!(p.to_string(), "4*x+1");
    /// ```
    #[inline]
    fn mod_compose_truncated_assign(&mut self, other: Self, len: u64, m: Natural) {
        self.mod_compose_truncated_assign(&other, len, &m);
    }
}

impl ModComposeTruncatedAssign<Self, &Natural> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another modulo $m$ in place and truncates the result
    /// to its first `len` coefficients, taking the other polynomial by value and the modulus by
    /// reference. The inner polynomial must have constant coefficient 0, and the coefficients of
    /// both must already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. Nothing is divided modulo $m$, so any modulus works, prime or not.
    /// The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^{5/2} b \\log b \\log\\log b)$
    ///
    /// $M(n, b) = O(n^{3/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// let mut p = NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_compose_truncated_assign(NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, &m);
    /// assert_eq!(p.to_string(), "x^3+6*x^2+5*x+2");
    ///
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// let mut p = NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_compose_truncated_assign(NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, &m);
    /// assert_eq!(p.to_string(), "155*x^3+76*x^2+5*x+2");
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let m = Natural::from(6u32);
    /// let mut p = NaturalPolynomial::from_str("x^2+2*x+1").unwrap();
    /// p.mod_compose_truncated_assign(NaturalPolynomial::from_str("x^2+2*x").unwrap(), 3, &m);
    /// assert_eq!(p.to_string(), "4*x+1");
    /// ```
    #[inline]
    fn mod_compose_truncated_assign(&mut self, other: Self, len: u64, m: &Natural) {
        self.mod_compose_truncated_assign(&other, len, m);
    }
}

impl ModComposeTruncatedAssign<&Self, Natural> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another modulo $m$ in place and truncates the result
    /// to its first `len` coefficients, taking the modulus by value and the other polynomial by
    /// reference. The inner polynomial must have constant coefficient 0, and the coefficients of
    /// both must already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. Nothing is divided modulo $m$, so any modulus works, prime or not.
    /// The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^{5/2} b \\log b \\log\\log b)$
    ///
    /// $M(n, b) = O(n^{3/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// let mut p = NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_compose_truncated_assign(&NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, m);
    /// assert_eq!(p.to_string(), "x^3+6*x^2+5*x+2");
    ///
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// let mut p = NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_compose_truncated_assign(&NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, m);
    /// assert_eq!(p.to_string(), "155*x^3+76*x^2+5*x+2");
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let m = Natural::from(6u32);
    /// let mut p = NaturalPolynomial::from_str("x^2+2*x+1").unwrap();
    /// p.mod_compose_truncated_assign(&NaturalPolynomial::from_str("x^2+2*x").unwrap(), 3, m);
    /// assert_eq!(p.to_string(), "4*x+1");
    /// ```
    #[inline]
    fn mod_compose_truncated_assign(&mut self, other: &Self, len: u64, m: Natural) {
        self.mod_compose_truncated_assign(other, len, &m);
    }
}

impl ModComposeTruncatedAssign<&Self, &Natural> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another modulo $m$ in place and truncates the result
    /// to its first `len` coefficients, taking both by reference. The inner polynomial must have
    /// constant coefficient 0, and the coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $m$ fits in a limb, the composition is
    /// done with limbs, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. Nothing is divided modulo $m$, so any modulus works, prime or not.
    /// The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^{5/2} b \\log b \\log\\log b)$
    ///
    /// $M(n, b) = O(n^{3/2} b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $b$ is
    /// `m.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// `m`, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncatedAssign;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let m = Natural::from(7u32);
    /// let mut p = NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_compose_truncated_assign(&NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, &m);
    /// assert_eq!(p.to_string(), "x^3+6*x^2+5*x+2");
    ///
    /// let m = Natural::from_str("10000000000000000000000").unwrap();
    /// let mut p = NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_compose_truncated_assign(&NaturalPolynomial::from_str("x^2+5*x").unwrap(), 4, &m);
    /// assert_eq!(p.to_string(), "155*x^3+76*x^2+5*x+2");
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let m = Natural::from(6u32);
    /// let mut p = NaturalPolynomial::from_str("x^2+2*x+1").unwrap();
    /// p.mod_compose_truncated_assign(&NaturalPolynomial::from_str("x^2+2*x").unwrap(), 3, &m);
    /// assert_eq!(p.to_string(), "4*x+1");
    /// ```
    fn mod_compose_truncated_assign(&mut self, other: &Self, len: u64, m: &Natural) {
        assert_reduced(self, other, m);
        *self = mod_compose_truncated_helper(
            &take(&mut self.coefficients),
            &other.coefficients,
            len,
            m,
        );
    }
}
