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

use crate::num::arithmetic::traits::FloorSqrt;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::polynomial::{ModComposeTruncated, ModComposeTruncatedAssign};
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::{ProductShape, matrix_mod_mul};
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_add::assert_reduced;
use crate::unsigned_polynomial::arithmetic::mod_mul_truncated::mod_mul_truncated_to_out;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::from_coefficients_trimmed;
use crate::unsigned_polynomial::arithmetic::mod_square_truncated::mod_square_truncated_to_out;
use crate::unsigned_polynomial::arithmetic::ntt::{FixedFactor, NttModulus};
use crate::unsigned_vector::arithmetic::mod_add::mod_add_assign_slice;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::{swap, take};

// The first `n` coefficients of the composition, modulo `m`, of the polynomials with coefficients
// `xs` and `ys`, reduced, where `xs.len() >= 2` and `ys[0]` is 0, by Horner's rule. The running
// polynomial and the scratch product trade places at every step, and every product is by `ys`, so
// its transforms are computed once when they are used (see `FixedFactor`).
//
// This is equivalent to `_gr_poly_compose_series_horner` from `gr_poly/compose_series_horner.c`,
// FLINT 3.6.0, with coefficients modulo `m`, except that the product is not copied back and that
// the transforms of `ys` are reused.
crate_test_fn! {mod_compose_truncated_horner<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    n: usize,
    m: T,
) -> Vec<T> {
    let (&leading, rest) = xs.split_last().unwrap();
    let mut i = rest.len() - 1;
    let data = T::precompute_mod_mul_data(&m);
    let mut acc = Vec::with_capacity(n);
    acc.extend(ys.iter().map(|&y| y.mod_mul_precomputed(leading, m, &data)));
    acc[0].mod_add_assign(xs[i], m);
    let mut scratch = Vec::with_capacity(n);
    let factor = FixedFactor::new(ys, n, NttModulus::Mod(m), i);
    while i > 0 {
        i -= 1;
        // The product overwrites every element, so stale ones may stay.
        scratch.resize(min(acc.len() + ys.len() - 1, n), T::ZERO);
        factor.mul_truncated(&mut scratch, &acc);
        swap(&mut acc, &mut scratch);
        acc[0].mod_add_assign(xs[i], m);
    }
    acc.resize(n, T::ZERO);
    acc
}}

// The first `n` coefficients of `xs + ys * zs`, modulo `m`, where `ys` and `zs` are nonempty.
fn mod_add_mul_truncated<T: PrimitiveUnsigned>(
    mut xs: Vec<T>,
    ys: &[T],
    zs: &[T],
    n: usize,
    m: T,
) -> Vec<T> {
    let mut product = vec![T::ZERO; min(ys.len() + zs.len() - 1, n)];
    mod_mul_truncated_to_out(&mut product, ys, zs, m);
    if product.len() > xs.len() {
        swap(&mut xs, &mut product);
    }
    mod_add_assign_slice(&mut xs, &product, m);
    xs
}

// The first `n` coefficients of the composition, modulo `m`, of the polynomials with coefficients
// `xs` and `ys`, reduced, where `xs.len() >= 3` and `ys[0]` is 0, by divide and conquer:
// neighboring pairs of coefficients $x_{2i} + x_{2i+1}y$ start as groups; at every level,
// neighboring groups $g_{2i}$ and $g_{2i+1}$ merge into $g_{2i} + y^{2^j} g_{2i+1}$, and the power
// of `ys` is squared, until one group is left, everything truncated to `n` coefficients.
//
// This is equivalent to `_gr_poly_compose_series_divconquer` from
// `gr_poly/compose_series_divconquer.c`, FLINT 3.6.0, with coefficients modulo `m`, except that
// each group is its own `Vec`.
crate_test_fn! {mod_compose_truncated_divide_and_conquer<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    n: usize,
    m: T,
) -> Vec<T> {
    let data = T::precompute_mod_mul_data(&m);
    let mut groups: Vec<Vec<T>> = xs
        .chunks(2)
        .map(|pair| {
            if let &[x0, x1] = pair {
                let mut group: Vec<T> =
                    ys.iter().map(|&y| y.mod_mul_precomputed(x1, m, &data)).collect();
                group[0].mod_add_assign(x0, m);
                group
            } else {
                vec![pair[0]]
            }
        })
        .collect();
    let mut power = vec![T::ZERO; min((ys.len() << 1) - 1, n)];
    mod_square_truncated_to_out(&mut power, ys, m);
    loop {
        let mut merged = Vec::with_capacity(groups.len().div_ceil(2));
        let mut groups_iter = groups.into_iter();
        while let Some(lo) = groups_iter.next() {
            merged.push(if let Some(hi) = groups_iter.next() {
                mod_add_mul_truncated(lo, &power, &hi, n, m)
            } else {
                lo
            });
        }
        groups = merged;
        if groups.len() == 1 {
            break;
        }
        let mut square = vec![T::ZERO; min((power.len() << 1) - 1, n)];
        mod_square_truncated_to_out(&mut square, &power, m);
        power = square;
    }
    let mut out = groups.pop().unwrap();
    out.resize(n, T::ZERO);
    out
}}

// The first `n` coefficients of the composition, modulo `m`, of the polynomials with coefficients
// `xs` and `ys`, reduced, where `ys[0]` is 0 and `2 <= xs.len() <= n`, by the Brent–Kung
// algorithm: with $m' = \lfloor \sqrt n \rfloor + 1$, the products of the $m'$ blocks of $m'$
// coefficients of `xs` with the first $m'$ powers of `ys` are the rows of a matrix product modulo
// `m`, and they are combined by Horner's rule in $y^{m'}$.
//
// This is equivalent to `_gr_poly_compose_series_brent_kung` from
// `gr_poly/compose_series_brent_kung.c`, FLINT 3.6.0, with coefficients modulo `m`, except that the
// powers of `ys` are always computed by multiplying by `ys`, that the transforms of `ys` and of
// $y^{m'}$ are each computed once (see `FixedFactor`), and that the running polynomial and the
// scratch product trade places at every step.
crate_test_fn! {mod_compose_truncated_brent_kung<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    n: usize,
    m: T,
) -> Vec<T> {
    let k = n.floor_sqrt() + 1;
    // The rows of B are the blocks of xs.
    let mut b = vec![T::ZERO; k * k];
    for (i, chunk) in xs.chunks(k).enumerate() {
        b[i * k..i * k + chunk.len()].copy_from_slice(chunk);
    }
    // The rows of A are the powers of ys.
    let mut a = vec![T::ZERO; k * n];
    a[0] = T::ONE;
    a[n..n + ys.len()].copy_from_slice(ys);
    // The powers of `ys` and then `ys^k` itself: k - 1 products.
    let modulus = NttModulus::Mod(m);
    let factor = FixedFactor::new(ys, n, modulus, k - 1);
    for i in 2..k {
        let (previous, current) = a.split_at_mut(i * n);
        factor.mul_truncated(&mut current[..n], &previous[(i - 1) * n..]);
    }
    let c = matrix_mod_mul(&b, &a, ProductShape { ar: k, br: k, bc: n }, m);
    // Combine the rows by Horner's rule in h = ys^k.
    let mut h = vec![T::ZERO; n];
    factor.mul_truncated(&mut h, &a[(k - 1) * n..]);
    let h_factor = FixedFactor::new(&h, n, modulus, k - 1);
    let mut acc = c[(k - 1) * n..].to_vec();
    let mut scratch = vec![T::ZERO; n];
    for row in c.chunks_exact(n).take(k - 1).rev() {
        h_factor.mul_truncated(&mut scratch, &acc);
        mod_add_assign_slice(&mut scratch, row, m);
        swap(&mut acc, &mut scratch);
    }
    acc
}}

// The first `n` coefficients of $p(cx^v)$ modulo `m`, where `xs` holds the coefficients of $p$ and
// $v \geq 1$: $p_i c^i$ at $x^{iv}$.
fn mod_compose_truncated_monomial<T: PrimitiveUnsigned>(
    xs: &[T],
    c: T,
    v: usize,
    n: usize,
    m: T,
) -> Vec<T> {
    let data = T::precompute_mod_mul_data(&m);
    let mut out = vec![T::ZERO; n];
    let mut power = T::ONE;
    for (o, &x) in out.iter_mut().step_by(v).zip(xs) {
        *o = x.mod_mul_precomputed(power, m, &data);
        power.mod_mul_precomputed_assign(c, m, &data);
    }
    out
}

// Whether a composition with an outer polynomial of `len1` coefficients, truncated to `n`
// coefficients, is done by Horner's rule: as measured on this machine, when the outer polynomial
// has at most 3 coefficients, or when the product of its length and `n` is at most 256, rather than
// FLINT's `len1 <= 7 || n <= 7`.
const fn horner_preferred(len1: usize, n: usize) -> bool {
    len1 <= 3 || len1.saturating_mul(n) <= 256
}

// Whether a composition with an outer polynomial of `len1` coefficients and an inner polynomial of
// `len2` coefficients, truncated to `n` coefficients, is done by divide and conquer rather than by
// the Brent–Kung algorithm. Divide and conquer costs about $\min(\ell_1, \ell/n)$ products of
// length $n$, where $\ell$ is the length of the untruncated composition, and the Brent–Kung
// algorithm about $2\sqrt n$ of them plus a matrix product; as measured on this machine, divide and
// conquer is faster when $\ell_1^2 \leq 16n$ or $\ell/n \leq 0.8\sqrt n$. FLINT chooses it when
// $\ell_1^2 < n$ or $\ell < 4n$.
const fn divide_and_conquer_preferred(len1: usize, len2: usize, n: usize) -> bool {
    let q = (len1 - 1)
        .saturating_mul(len2 - 1)
        .saturating_add(1)
        .saturating_mul(5)
        / n;
    len1.saturating_mul(len1) <= n << 4 || q.saturating_mul(q) <= n << 4
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
fn mod_compose_truncated_dispatch<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    n: usize,
    m: T,
) -> Vec<T> {
    let v = ys.iter().position(|&y| y != T::ZERO).unwrap();
    if ys[v + 1..].iter().all(|&y| y == T::ZERO) {
        mod_compose_truncated_monomial(xs, ys[v], v, n, m)
    } else if horner_preferred(xs.len(), n) {
        mod_compose_truncated_horner(xs, ys, n, m)
    } else if divide_and_conquer_preferred(xs.len(), ys.len(), n) {
        mod_compose_truncated_divide_and_conquer(xs, ys, n, m)
    } else {
        mod_compose_truncated_brent_kung(xs, ys, n, m)
    }
}

// The coefficients of the composition, modulo `m`, of the polynomials with coefficients `xs` and
// `ys`, reduced, keeping only the coefficients of $x^i$ for $i$ less than `len`, with `kernel`
// doing the work in the general case. The result is trimmed. If $v$ is the index of the first
// nonzero coefficient of `ys`, the term $p_i q^i$ is divisible by $x^{iv}$, so only the first
// $\lceil n/v \rceil$ coefficients of `xs` are read.
//
// This is equivalent to `nmod_poly_compose_series` from `nmod_poly/compose_series.c`, FLINT 3.6.0,
// except that `xs` is truncated using $v$.
crate_test_fn! {mod_compose_truncated_with<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    len: u64,
    m: T,
    kernel: fn(&[T], &[T], usize, T) -> Vec<T>,
) -> Vec<T> {
    if let Some(&y) = ys.first() {
        assert_eq!(y, T::ZERO, "the inner polynomial must have constant coefficient 0");
    }
    let n = usize::try_from(len).unwrap_or(usize::MAX);
    if xs.is_empty() || n == 0 {
        return Vec::new();
    }
    // The inner polynomial may have zeros at the end, or be all zeros, if it was truncated.
    if let Some(v) = ys.iter().position(|&y| y != T::ZERO) {
        let xs = &xs[..min(xs.len(), (n - 1) / v + 1)];
        if xs.len() > 1 {
            let lenr = min((xs.len() - 1).saturating_mul(ys.len() - 1) + 1, n);
            let xs = &xs[..min(xs.len(), lenr)];
            let ys = &ys[..min(ys.len(), lenr)];
            return from_coefficients_trimmed(kernel(xs, ys, lenr, m)).coefficients;
        }
    }
    from_coefficients_trimmed(vec![xs[0]]).coefficients
}}

/// The coefficients of the composition, modulo `m`, of the polynomials with coefficients `xs` and
/// `ys`, both reduced modulo `m`, keeping only the coefficients of $x^i$ for $i$ less than `len`,
/// trimmed. `ys` must have constant coefficient 0, and `m` must be positive.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compose
/// `NaturalPolynomial`s modulo a word.
#[doc(hidden)]
#[inline]
pub fn mod_compose_truncated_vec<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    len: u64,
    m: T,
) -> Vec<T> {
    mod_compose_truncated_with(xs, ys, len, m, mod_compose_truncated_dispatch)
}

impl<T: PrimitiveUnsigned> ModComposeTruncated<Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo $m$ and truncates the result to its first `len`
    /// coefficients, taking both by value. The inner polynomial must have constant coefficient 0,
    /// and the coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{1/2 + \\log_2 3})$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// $m$, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_compose_truncated(UnsignedPolynomial::<u8>::from_str("x^2+5*x").unwrap(), 4, 7)
    ///         .to_string(),
    ///     "x^3+6*x^2+5*x+2"
    /// );
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+2*x+1")
    ///         .unwrap()
    ///         .mod_compose_truncated(UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap(), 3, 6)
    ///         .to_string(),
    ///     "4*x+1"
    /// );
    /// ```
    #[inline]
    fn mod_compose_truncated(self, other: Self, len: u64, m: T) -> Self {
        (&self).mod_compose_truncated(&other, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModComposeTruncated<&Self, T> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo $m$ and truncates the result to its first `len`
    /// coefficients, taking the first by value and the second by reference. The inner polynomial
    /// must have constant coefficient 0, and the coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{1/2 + \\log_2 3})$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// $m$, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_compose_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+5*x").unwrap(),
    ///             4,
    ///             7
    ///         )
    ///         .to_string(),
    ///     "x^3+6*x^2+5*x+2"
    /// );
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+2*x+1")
    ///         .unwrap()
    ///         .mod_compose_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap(),
    ///             3,
    ///             6
    ///         )
    ///         .to_string(),
    ///     "4*x+1"
    /// );
    /// ```
    #[inline]
    fn mod_compose_truncated(self, other: &Self, len: u64, m: T) -> Self {
        (&self).mod_compose_truncated(other, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModComposeTruncated<UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo $m$ and truncates the result to its first `len`
    /// coefficients, taking the first by reference and the second by value. The inner polynomial
    /// must have constant coefficient 0, and the coefficients of both must already be reduced
    /// modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{1/2 + \\log_2 3})$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// $m$, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_compose_truncated(UnsignedPolynomial::<u8>::from_str("x^2+5*x").unwrap(), 4, 7)
    ///         .to_string(),
    ///     "x^3+6*x^2+5*x+2"
    /// );
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+2*x+1").unwrap())
    ///         .mod_compose_truncated(UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap(), 3, 6)
    ///         .to_string(),
    ///     "4*x+1"
    /// );
    /// ```
    #[inline]
    fn mod_compose_truncated(
        self,
        other: UnsignedPolynomial<T>,
        len: u64,
        m: T,
    ) -> UnsignedPolynomial<T> {
        self.mod_compose_truncated(&other, len, m)
    }
}

impl<T: PrimitiveUnsigned> ModComposeTruncated<&UnsignedPolynomial<T>, T>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo $m$ and truncates the result to its first `len`
    /// coefficients, taking both by reference. The inner polynomial must have constant coefficient
    /// 0, and the coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// f(p, q, n, m) = (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{1/2 + \\log_2 3})$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// $m$, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_compose_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+5*x").unwrap(),
    ///             4,
    ///             7
    ///         )
    ///         .to_string(),
    ///     "x^3+6*x^2+5*x+2"
    /// );
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+2*x+1").unwrap())
    ///         .mod_compose_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap(),
    ///             3,
    ///             6
    ///         )
    ///         .to_string(),
    ///     "4*x+1"
    /// );
    /// ```
    fn mod_compose_truncated(
        self,
        other: &UnsignedPolynomial<T>,
        len: u64,
        m: T,
    ) -> UnsignedPolynomial<T> {
        assert_reduced(self, other, m);
        UnsignedPolynomial {
            coefficients: mod_compose_truncated_vec(
                &self.coefficients,
                &other.coefficients,
                len,
                m,
            ),
        }
    }
}

impl<T: PrimitiveUnsigned> ModComposeTruncatedAssign<Self, T> for UnsignedPolynomial<T> {
    /// Composes an [`UnsignedPolynomial`] with another modulo $m$ in place and truncates the result
    /// to its first `len` coefficients, taking the other by value. The inner polynomial must have
    /// constant coefficient 0, and the coefficients of both must already be reduced modulo $m$.
    ///
    /// $$
    /// p \\gets (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{1/2 + \\log_2 3})$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// $m$, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_compose_truncated_assign(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+5*x").unwrap(),
    ///     4,
    ///     7,
    /// );
    /// assert_eq!(p.to_string(), "x^3+6*x^2+5*x+2");
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+2*x+1").unwrap();
    /// p.mod_compose_truncated_assign(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap(),
    ///     3,
    ///     6,
    /// );
    /// assert_eq!(p.to_string(), "4*x+1");
    /// ```
    #[inline]
    fn mod_compose_truncated_assign(&mut self, other: Self, len: u64, m: T) {
        self.mod_compose_truncated_assign(&other, len, m);
    }
}

impl<T: PrimitiveUnsigned> ModComposeTruncatedAssign<&Self, T> for UnsignedPolynomial<T> {
    /// Composes an [`UnsignedPolynomial`] with another modulo $m$ in place and truncates the result
    /// to its first `len` coefficients, taking the other by reference. The inner polynomial must
    /// have constant coefficient 0, and the coefficients of both must already be reduced modulo
    /// $m$.
    ///
    /// $$
    /// p \\gets (p(q(x)) \\bmod x^n) \\bmod m.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. If $q$ has a single nonzero coefficient, the
    /// coefficients of $p$ are scaled and spread out; otherwise short compositions use Horner's
    /// rule; those whose outer polynomial is short compared with the result, or whose untruncated
    /// result is not much longer than $n$, use divide and conquer; and the others use the
    /// Brent–Kung algorithm. The result is trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^{1/2 + \\log_2 3})$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `m` is zero, if any coefficient of `self` or `other` is greater than or equal to
    /// $m$, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModComposeTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_compose_truncated_assign(
    ///     &UnsignedPolynomial::<u8>::from_str("x^2+5*x").unwrap(),
    ///     4,
    ///     7,
    /// );
    /// assert_eq!(p.to_string(), "x^3+6*x^2+5*x+2");
    ///
    /// // Modulo 6, the coefficient of x^2 vanishes.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+2*x+1").unwrap();
    /// p.mod_compose_truncated_assign(
    ///     &UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap(),
    ///     3,
    ///     6,
    /// );
    /// assert_eq!(p.to_string(), "4*x+1");
    /// ```
    fn mod_compose_truncated_assign(&mut self, other: &Self, len: u64, m: T) {
        assert_reduced(self, other, m);
        self.coefficients =
            mod_compose_truncated_vec(&take(&mut self.coefficients), &other.coefficients, len, m);
    }
}
