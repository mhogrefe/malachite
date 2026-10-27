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
use crate::natural_polynomial::NaturalPolynomial;
use crate::natural_polynomial::arithmetic::mod_power_of_2_add::assert_reduced;
use crate::natural_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};
use core::mem::{swap, take};
use malachite_base::num::arithmetic::traits::{
    FloorSqrt, ModPowerOf2AddAssign, ModPowerOf2Assign, ModPowerOf2Mul, ModPowerOf2MulAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::polynomial::{
    ModPowerOf2ComposeTruncated, ModPowerOf2ComposeTruncatedAssign, Polynomial,
};
use malachite_base::unsigned_polynomial::arithmetic::mod_power_of_2_compose_truncated::*;

// The first `n` coefficients of the product of the polynomials with coefficients `xs` and `ys`
// modulo $2^k$, where $k$ is `pow`, without zeros at the end; empty if either is.
fn mul_truncated(xs: &[Natural], ys: &[Natural], n: usize, pow: u64) -> Vec<Natural> {
    if xs.is_empty() || ys.is_empty() {
        Vec::new()
    } else {
        mod_power_of_2_mul_truncated_ref_ref(xs, ys, u64::exact_from(n), pow)
            .into_coefficients_asc()
    }
}

// Adds `ys` into `xs` modulo $2^k$, where $k$ is `pow`, reusing whichever of the two is longer.
fn add_assign_val(xs: &mut Vec<Natural>, mut ys: Vec<Natural>, pow: u64) {
    if ys.len() > xs.len() {
        swap(xs, &mut ys);
    }
    for (x, y) in xs.iter_mut().zip(ys) {
        x.mod_power_of_2_add_assign(y, pow);
    }
}

// Adds `x` to the constant coefficient of the polynomial with coefficients `xs` modulo $2^k$, where
// $k$ is `pow`.
fn add_constant_assign(xs: &mut Vec<Natural>, x: &Natural, pow: u64) {
    if let Some(first) = xs.first_mut() {
        first.mod_power_of_2_add_assign(x, pow);
    } else {
        xs.push(x.clone());
    }
}

// The first `n` coefficients of the composition, modulo $2^k$, where $k$ is `pow`, of the
// polynomials with coefficients `xs` and `ys`, reduced, where `xs.len() >= 2` and `ys[0]` is 0, by
// Horner's rule. The result may be shorter than `n`.
//
// This is equivalent to `_gr_poly_compose_series_horner` from `gr_poly/compose_series_horner.c`,
// FLINT 3.6.0, with coefficients modulo $2^k$.
crate_test_fn! {mod_power_of_2_compose_truncated_horner(
    xs: &[Natural],
    ys: &[Natural],
    n: usize,
    pow: u64,
) -> Vec<Natural> {
    let mut acc = Vec::new();
    for x in xs.iter().rev() {
        acc = mul_truncated(&acc, ys, n, pow);
        add_constant_assign(&mut acc, x, pow);
    }
    acc
}}

// The first `n` coefficients of the composition, modulo $2^k$, where $k$ is `pow`, of the
// polynomials with coefficients `xs` and `ys`, reduced, where `xs.len() >= 3` and `ys[0]` is 0, by
// divide and conquer: neighboring pairs of coefficients $x_{2i} + x_{2i+1}y$ start as groups; at
// every level, neighboring groups $g_{2i}$ and $g_{2i+1}$ merge into $g_{2i} + y^{2^j} g_{2i+1}$,
// and the power of `ys` is squared, until one group is left, everything truncated to `n`
// coefficients. The result may be shorter than `n`.
//
// This is equivalent to `_gr_poly_compose_series_divconquer` from
// `gr_poly/compose_series_divconquer.c`, FLINT 3.6.0, with coefficients modulo $2^k$, except that
// each group is its own `Vec`.
crate_test_fn! {mod_power_of_2_compose_truncated_divide_and_conquer(
    xs: &[Natural],
    ys: &[Natural],
    n: usize,
    pow: u64,
) -> Vec<Natural> {
    let mut groups: Vec<Vec<Natural>> = xs
        .chunks(2)
        .map(|pair| {
            let mut group = if let [_, x1] = pair {
                mul_truncated(ys, core::slice::from_ref(x1), n, pow)
            } else {
                Vec::new()
            };
            add_constant_assign(&mut group, &pair[0], pow);
            group
        })
        .collect();
    let mut power = mul_truncated(ys, ys, n, pow);
    loop {
        let mut merged = Vec::with_capacity(groups.len().div_ceil(2));
        let mut groups_iter = groups.into_iter();
        while let Some(mut lo) = groups_iter.next() {
            if let Some(hi) = groups_iter.next() {
                add_assign_val(&mut lo, mul_truncated(&power, &hi, n, pow), pow);
            }
            merged.push(lo);
        }
        groups = merged;
        if groups.len() == 1 {
            break;
        }
        power = mul_truncated(&power, &power, n, pow);
    }
    groups.pop().unwrap()
}}

// The first `n` coefficients of the composition, modulo $2^k$, where $k$ is `pow`, of the
// polynomials with coefficients `xs` and `ys`, reduced, where `ys[0]` is 0 and `2 <= xs.len() <=
// n`, by the Brent–Kung algorithm: with $m = \lfloor \sqrt n \rfloor + 1$, the products of the
// $m$ blocks of $m$ coefficients of `xs` with the first $m$ powers of `ys` are the rows of a matrix
// product, and they are combined by Horner's rule in $y^m$. The result may be shorter than `n`.
//
// This is equivalent to `_gr_poly_compose_series_brent_kung` from
// `gr_poly/compose_series_brent_kung.c`, FLINT 3.6.0, with coefficients modulo $2^k$, except that
// the powers of `ys` are always computed by multiplying by `ys`.
crate_test_fn! {mod_power_of_2_compose_truncated_brent_kung(
    xs: &[Natural],
    ys: &[Natural],
    n: usize,
    pow: u64,
) -> Vec<Natural> {
    let m = n.floor_sqrt() + 1;
    // The rows of B are the blocks of xs.
    let mut b = vec![Natural::ZERO; m * m];
    for (i, chunk) in xs.chunks(m).enumerate() {
        b[i * m..i * m + chunk.len()].clone_from_slice(chunk);
    }
    // The rows of A are the powers of ys.
    let mut a = vec![Natural::ZERO; m * n];
    a[0] = Natural::ONE;
    a[n..n + ys.len()].clone_from_slice(ys);
    for i in 2..m {
        let (previous, current) = a.split_at_mut(i * n);
        for (c, p) in current.iter_mut().zip(mul_truncated(&previous[(i - 1) * n..], ys, n, pow)) {
            *c = p;
        }
    }
    let mut c = matrix_mul(&b, &a, ProductShape { ar: m, br: m, bc: n });
    for x in &mut c {
        x.mod_power_of_2_assign(pow);
    }
    // Combine the rows by Horner's rule in h = ys^m.
    let h = mul_truncated(&a[(m - 1) * n..], ys, n, pow);
    let mut acc: Vec<Natural> = c.drain((m - 1) * n..).collect();
    for i in (0..m - 1).rev() {
        acc = mul_truncated(&acc, &h, n, pow);
        add_assign_val(&mut acc, c.drain(i * n..).collect(), pow);
    }
    acc
}}

// The first `n` coefficients of $p(cx^v)$ modulo $2^k$, where $k$ is `pow`, `xs` holds the
// coefficients of $p$, and $v \geq 1$: $p_i c^i$ at $x^{iv}$.
fn mod_power_of_2_compose_truncated_monomial(
    xs: &[Natural],
    c: &Natural,
    v: usize,
    n: usize,
    pow: u64,
) -> Vec<Natural> {
    let mut out = vec![Natural::ZERO; n];
    let mut power = Natural::ONE;
    for (o, x) in out.iter_mut().step_by(v).zip(xs) {
        *o = x.mod_power_of_2_mul(&power, pow);
        power.mod_power_of_2_mul_assign(c, pow);
    }
    out
}

// Whether a composition with an outer polynomial of `len1` coefficients is done by Horner's rule.
// As measured on this machine, divide and conquer is at least as fast from 8 coefficients on,
// whatever the other lengths and `pow`, so Horner's rule is kept only for the shortest outer
// polynomials, which divide and conquer, needing at least 3, would barely split. FLINT uses
// Horner's rule up to 7 coefficients, or up to a result of 7 coefficients.
const fn horner_preferred(len1: usize) -> bool {
    len1 <= 3
}

// Whether a composition with an outer polynomial of `len1` coefficients and an inner polynomial of
// `len2` coefficients, truncated to `n` coefficients, is done by divide and conquer rather than by
// the Brent–Kung algorithm. As measured on this machine, divide and conquer is faster when the
// outer polynomial is short compared with `n`, $\ell_1^2 \leq 4n$, or the untruncated composition
// is shorter than $4n$; FLINT chooses it when $\ell_1^2 < n$ or the untruncated composition is
// shorter than $4n$.
const fn divide_and_conquer_preferred(len1: usize, len2: usize, n: usize) -> bool {
    len1.saturating_mul(len1) <= n << 2 || (len1 - 1).saturating_mul(len2 - 1) + 1 < n << 2
}

// The first `n` coefficients of the composition, modulo $2^k$, where $k$ is `pow`, of the
// polynomials with coefficients `xs` and `ys`, reduced, where `xs.len() >= 2` and `ys[0]` is 0:
// directly if `ys` has a single nonzero coefficient, and otherwise by Horner's rule, divide and
// conquer, or the Brent–Kung algorithm.
//
// This is equivalent to `_gr_poly_compose_series` from `gr_poly/compose_series.c`, FLINT 3.6.0,
// with coefficients modulo $2^k$, except that the monomial case allows a monomial of any degree,
// that the choice of algorithm was measured (see `horner_preferred` and
// `divide_and_conquer_preferred`), and that the Kinoshita–Li algorithm is not used.
fn mod_power_of_2_compose_truncated_dispatch(
    xs: &[Natural],
    ys: &[Natural],
    n: usize,
    pow: u64,
) -> Vec<Natural> {
    let v = ys.iter().position(|y| *y != 0u32).unwrap();
    if ys[v + 1..].iter().all(|y| *y == 0u32) {
        mod_power_of_2_compose_truncated_monomial(xs, &ys[v], v, n, pow)
    } else if horner_preferred(xs.len()) {
        mod_power_of_2_compose_truncated_horner(xs, ys, n, pow)
    } else if divide_and_conquer_preferred(xs.len(), ys.len(), n) {
        mod_power_of_2_compose_truncated_divide_and_conquer(xs, ys, n, pow)
    } else {
        mod_power_of_2_compose_truncated_brent_kung(xs, ys, n, pow)
    }
}

// The composition, modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, reduced, keeping only the coefficients of $x^i$ for $i$ less than `len`, with `kernel`
// doing the work in the general case. If $v$ is the index of the first nonzero coefficient of `ys`,
// the term $p_i q^i$ is divisible by $x^{iv}$, so only the first $\lceil n/v \rceil$ coefficients
// of `xs` are read.
//
// This is equivalent to `nmod_poly_compose_series` from `nmod_poly/compose_series.c`, FLINT 3.6.0,
// with modulus $2^k$, except that `xs` is truncated using $v$.
crate_test_fn! {mod_power_of_2_compose_truncated_with(
    xs: &[Natural],
    ys: &[Natural],
    len: u64,
    pow: u64,
    kernel: fn(&[Natural], &[Natural], usize, u64) -> Vec<Natural>,
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
                coefficients: kernel(xs, ys, lenr, pow),
            };
            p.trim();
            return p;
        }
    }
    NaturalPolynomial::from(xs[0].clone())
}}

// The composition, modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, both reduced, keeping only the coefficients of $x^i$ for $i$ less than `len`, in wrapping
// arithmetic in `T`, which $2^k$ must fit in.
fn mod_power_of_2_compose_truncated_word<T: PrimitiveUnsigned + for<'a> ExactFrom<&'a Natural>>(
    xs: &[Natural],
    ys: &[Natural],
    len: u64,
    pow: u64,
) -> NaturalPolynomial
where
    Natural: From<T>,
{
    // Only the first `len` coefficients of either polynomial can matter, but at least one, so that
    // a nonzero constant coefficient of the inner polynomial is still rejected.
    let kept = max(usize::try_from(len).unwrap_or(usize::MAX), 1);
    let xs: Vec<T> = xs.iter().take(kept).map(T::exact_from).collect();
    let ys: Vec<T> = ys.iter().take(kept).map(T::exact_from).collect();
    NaturalPolynomial {
        coefficients: mod_power_of_2_compose_truncated_vec(&xs, &ys, len, pow)
            .into_iter()
            .map(Natural::from)
            .collect(),
    }
}

// The composition, modulo $2^k$, where $k$ is `pow`, of the polynomials with coefficients `xs` and
// `ys`, both reduced, keeping only the coefficients of $x^i$ for $i$ less than `len`. When $2^k$
// fits in one or two limbs, this is done in wrapping word arithmetic, which is several times faster
// than arithmetic on `Natural`s.
fn mod_power_of_2_compose_truncated_helper(
    xs: &[Natural],
    ys: &[Natural],
    len: u64,
    pow: u64,
) -> NaturalPolynomial {
    if pow <= Limb::WIDTH {
        mod_power_of_2_compose_truncated_word::<Limb>(xs, ys, len, pow)
    } else if pow <= DoubleLimb::WIDTH {
        mod_power_of_2_compose_truncated_word::<DoubleLimb>(xs, ys, len, pow)
    } else {
        mod_power_of_2_compose_truncated_with(
            xs,
            ys,
            len,
            pow,
            mod_power_of_2_compose_truncated_dispatch,
        )
    }
}

impl ModPowerOf2ComposeTruncated<Self> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo $2^k$ and truncates the result to its first `len`
    /// coefficients, taking both by value. The inner polynomial must have constant coefficient 0,
    /// and the coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (p(q(x)) \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $2^k$ fits in two limbs, the composition
    /// is done in wrapping word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \\log (nk) \\log\\log (nk))$
    ///
    /// $M(n, k) = O(n^{3/2} k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_compose_truncated(
    ///             NaturalPolynomial::from_str("x^2+5*x").unwrap(),
    ///             4,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "11*x^3+12*x^2+5*x+2"
    /// );
    /// // Modulo 4, the coefficient of x vanishes.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+2*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_compose_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x").unwrap(),
    ///             3,
    ///             2
    ///         )
    ///         .to_string(),
    ///     "2*x^2+1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_compose_truncated(self, other: Self, len: u64, pow: u64) -> Self {
        (&self).mod_power_of_2_compose_truncated(&other, len, pow)
    }
}

impl ModPowerOf2ComposeTruncated<&Self> for NaturalPolynomial {
    type Output = Self;

    /// Composes two [`NaturalPolynomial`]s modulo $2^k$ and truncates the result to its first `len`
    /// coefficients, taking the first by value and the second by reference. The inner polynomial
    /// must have constant coefficient 0, and the coefficients of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (p(q(x)) \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $2^k$ fits in two limbs, the composition
    /// is done in wrapping word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \\log (nk) \\log\\log (nk))$
    ///
    /// $M(n, k) = O(n^{3/2} k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_compose_truncated(
    ///             &NaturalPolynomial::from_str("x^2+5*x").unwrap(),
    ///             4,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "11*x^3+12*x^2+5*x+2"
    /// );
    /// // Modulo 4, the coefficient of x vanishes.
    /// assert_eq!(
    ///     NaturalPolynomial::from_str("x^2+2*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_compose_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x").unwrap(),
    ///             3,
    ///             2
    ///         )
    ///         .to_string(),
    ///     "2*x^2+1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_compose_truncated(self, other: &Self, len: u64, pow: u64) -> Self {
        (&self).mod_power_of_2_compose_truncated(other, len, pow)
    }
}

impl ModPowerOf2ComposeTruncated<NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo $2^k$ and truncates the result to its first `len`
    /// coefficients, taking the first by reference and the second by value. The inner polynomial
    /// must have constant coefficient 0, and the coefficients of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (p(q(x)) \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $2^k$ fits in two limbs, the composition
    /// is done in wrapping word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \\log (nk) \\log\\log (nk))$
    ///
    /// $M(n, k) = O(n^{3/2} k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_power_of_2_compose_truncated(
    ///             NaturalPolynomial::from_str("x^2+5*x").unwrap(),
    ///             4,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "11*x^3+12*x^2+5*x+2"
    /// );
    /// // Modulo 4, the coefficient of x vanishes.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+2*x+1").unwrap())
    ///         .mod_power_of_2_compose_truncated(
    ///             NaturalPolynomial::from_str("x^2+2*x").unwrap(),
    ///             3,
    ///             2
    ///         )
    ///         .to_string(),
    ///     "2*x^2+1"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_compose_truncated(
        self,
        other: NaturalPolynomial,
        len: u64,
        pow: u64,
    ) -> NaturalPolynomial {
        self.mod_power_of_2_compose_truncated(&other, len, pow)
    }
}

impl ModPowerOf2ComposeTruncated<&NaturalPolynomial> for &NaturalPolynomial {
    type Output = NaturalPolynomial;

    /// Composes two [`NaturalPolynomial`]s modulo $2^k$ and truncates the result to its first `len`
    /// coefficients, taking both by reference. The inner polynomial must have constant coefficient
    /// 0, and the coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (p(q(x)) \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $2^k$ fits in two limbs, the composition
    /// is done in wrapping word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \\log (nk) \\log\\log (nk))$
    ///
    /// $M(n, k) = O(n^{3/2} k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeTruncated;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_power_of_2_compose_truncated(
    ///             &NaturalPolynomial::from_str("x^2+5*x").unwrap(),
    ///             4,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "11*x^3+12*x^2+5*x+2"
    /// );
    /// // Modulo 4, the coefficient of x vanishes.
    /// assert_eq!(
    ///     (&NaturalPolynomial::from_str("x^2+2*x+1").unwrap())
    ///         .mod_power_of_2_compose_truncated(
    ///             &NaturalPolynomial::from_str("x^2+2*x").unwrap(),
    ///             3,
    ///             2
    ///         )
    ///         .to_string(),
    ///     "2*x^2+1"
    /// );
    /// ```
    fn mod_power_of_2_compose_truncated(
        self,
        other: &NaturalPolynomial,
        len: u64,
        pow: u64,
    ) -> NaturalPolynomial {
        assert_reduced(self, other, pow);
        mod_power_of_2_compose_truncated_helper(&self.coefficients, &other.coefficients, len, pow)
    }
}

impl ModPowerOf2ComposeTruncatedAssign<Self> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another modulo $2^k$ in place and truncates the result
    /// to its first `len` coefficients, taking the other by value. The inner polynomial must have
    /// constant coefficient 0, and the coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// p \\gets (p(q(x)) \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $2^k$ fits in two limbs, the composition
    /// is done in wrapping word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \\log (nk) \\log\\log (nk))$
    ///
    /// $M(n, k) = O(n^{3/2} k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_power_of_2_compose_truncated_assign(
    ///     NaturalPolynomial::from_str("x^2+5*x").unwrap(),
    ///     4,
    ///     4,
    /// );
    /// assert_eq!(p.to_string(), "11*x^3+12*x^2+5*x+2");
    ///
    /// // Modulo 4, the coefficient of x vanishes.
    /// let mut p = NaturalPolynomial::from_str("x^2+2*x+1").unwrap();
    /// p.mod_power_of_2_compose_truncated_assign(
    ///     NaturalPolynomial::from_str("x^2+2*x").unwrap(),
    ///     3,
    ///     2,
    /// );
    /// assert_eq!(p.to_string(), "2*x^2+1");
    /// ```
    #[inline]
    fn mod_power_of_2_compose_truncated_assign(&mut self, other: Self, len: u64, pow: u64) {
        self.mod_power_of_2_compose_truncated_assign(&other, len, pow);
    }
}

impl ModPowerOf2ComposeTruncatedAssign<&Self> for NaturalPolynomial {
    /// Composes a [`NaturalPolynomial`] with another modulo $2^k$ in place and truncates the result
    /// to its first `len` coefficients, taking the other by reference. The inner polynomial must
    /// have constant coefficient 0, and the coefficients of both must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// p \\gets (p(q(x)) \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. When $2^k$ fits in two limbs, the composition
    /// is done in wrapping word arithmetic, as for an
    /// [`UnsignedPolynomial`](malachite_base::unsigned_polynomial::UnsignedPolynomial); otherwise
    /// the coefficients are kept reduced throughout. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \\log (nk) \\log\\log (nk))$
    ///
    /// $M(n, k) = O(n^{3/2} k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `len`, and $k$ is `pow`.
    ///
    /// # Panics
    /// Panics if any coefficient of `self` or `other` is greater than or equal to $2^k$, or if
    /// `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeTruncatedAssign;
    /// use malachite_nz::natural_polynomial::NaturalPolynomial;
    ///
    /// let mut p = NaturalPolynomial::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_power_of_2_compose_truncated_assign(
    ///     &NaturalPolynomial::from_str("x^2+5*x").unwrap(),
    ///     4,
    ///     4,
    /// );
    /// assert_eq!(p.to_string(), "11*x^3+12*x^2+5*x+2");
    ///
    /// // Modulo 4, the coefficient of x vanishes.
    /// let mut p = NaturalPolynomial::from_str("x^2+2*x+1").unwrap();
    /// p.mod_power_of_2_compose_truncated_assign(
    ///     &NaturalPolynomial::from_str("x^2+2*x").unwrap(),
    ///     3,
    ///     2,
    /// );
    /// assert_eq!(p.to_string(), "2*x^2+1");
    /// ```
    fn mod_power_of_2_compose_truncated_assign(&mut self, other: &Self, len: u64, pow: u64) {
        assert_reduced(self, other, pow);
        *self = mod_power_of_2_compose_truncated_helper(
            &take(&mut self.coefficients),
            &other.coefficients,
            len,
            pow,
        );
    }
}
