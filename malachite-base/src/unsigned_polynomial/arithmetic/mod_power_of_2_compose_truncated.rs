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
use crate::polynomial::{ModPowerOf2ComposeTruncated, ModPowerOf2ComposeTruncatedAssign};
use crate::unsigned_matrix::arithmetic::mod_mul_matrix::ProductShape;
use crate::unsigned_matrix::arithmetic::mod_power_of_2_mul_matrix::matrix_wrapping_mul;
use crate::unsigned_polynomial::UnsignedPolynomial;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_add::assert_reduced;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul::{
    add_wrapping_assign, from_coefficients_trimmed, mask_coefficients,
};
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_mul_truncated::*;
use crate::unsigned_polynomial::arithmetic::mod_power_of_2_square_truncated::*;
use crate::unsigned_polynomial::arithmetic::ntt::{FixedFactor, NttModulus};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::min;
use core::mem::{swap, take};

// The first `n` coefficients of the composition of the polynomials with coefficients `xs` and `ys`,
// correct modulo $2^k$, where $k$ is `pow`, `xs.len() >= 2`, and `ys[0]` is 0, by Horner's rule.
// The running polynomial and the scratch product trade places at every step. The result is not
// reduced.
//
// This is equivalent to `_gr_poly_compose_series_horner` from `gr_poly/compose_series_horner.c`,
// FLINT 3.6.0, in wrapping word arithmetic, except that the product is not copied back.
crate_test_fn! {mod_power_of_2_compose_truncated_horner<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    n: usize,
    pow: u64,
) -> Vec<T> {
    let (&leading, rest) = xs.split_last().unwrap();
    let mut i = rest.len() - 1;
    let mut acc = Vec::with_capacity(n);
    acc.extend(ys.iter().map(|&y| y.wrapping_mul(leading)));
    acc[0].wrapping_add_assign(xs[i]);
    let mut scratch = Vec::with_capacity(n);
    let factor = FixedFactor::new(ys, n, NttModulus::PowerOf2(pow), i);
    while i > 0 {
        i -= 1;
        // The product overwrites every element, so stale ones may stay.
        scratch.resize(min(acc.len() + ys.len() - 1, n), T::ZERO);
        factor.mul_truncated(&mut scratch, &acc);
        swap(&mut acc, &mut scratch);
        acc[0].wrapping_add_assign(xs[i]);
    }
    acc.resize(n, T::ZERO);
    acc
}}

// The first `n` coefficients of `xs + ys * zs`, correct modulo $2^k$, where $k$ is `pow`, and `ys`
// and `zs` are nonempty.
fn add_mul_truncated_wrapping<T: PrimitiveUnsigned>(
    mut xs: Vec<T>,
    ys: &[T],
    zs: &[T],
    n: usize,
    pow: u64,
) -> Vec<T> {
    let mut product = vec![T::ZERO; min(ys.len() + zs.len() - 1, n)];
    mul_truncated_wrapping(&mut product, ys, zs, pow);
    if product.len() > xs.len() {
        swap(&mut xs, &mut product);
    }
    add_wrapping_assign(&mut xs, &product);
    xs
}

// The first `n` coefficients of the composition of the polynomials with coefficients `xs` and `ys`,
// correct modulo $2^k$, where $k$ is `pow`, `xs.len() >= 3`, and `ys[0]` is 0, by divide and
// conquer: neighboring pairs of coefficients $x_{2i} + x_{2i+1}y$ start as groups; at every level,
// neighboring groups $g_{2i}$ and $g_{2i+1}$ merge into $g_{2i} + y^{2^j} g_{2i+1}$, and the power
// of `ys` is squared, until one group is left, everything truncated to `n` coefficients. The result
// is not reduced.
//
// This is equivalent to `_gr_poly_compose_series_divconquer` from
// `gr_poly/compose_series_divconquer.c`, FLINT 3.6.0, in wrapping word arithmetic, except that each
// group is its own `Vec`.
crate_test_fn! {mod_power_of_2_compose_truncated_divide_and_conquer<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    n: usize,
    pow: u64,
) -> Vec<T> {
    let mut groups: Vec<Vec<T>> = xs
        .chunks(2)
        .map(|pair| {
            if let &[x0, x1] = pair {
                let mut group: Vec<T> = ys.iter().map(|&y| y.wrapping_mul(x1)).collect();
                group[0].wrapping_add_assign(x0);
                group
            } else {
                vec![pair[0]]
            }
        })
        .collect();
    let mut power = vec![T::ZERO; min((ys.len() << 1) - 1, n)];
    square_truncated_wrapping(&mut power, ys, pow);
    loop {
        let mut merged = Vec::with_capacity(groups.len().div_ceil(2));
        let mut groups_iter = groups.into_iter();
        while let Some(lo) = groups_iter.next() {
            merged.push(if let Some(hi) = groups_iter.next() {
                add_mul_truncated_wrapping(lo, &power, &hi, n, pow)
            } else {
                lo
            });
        }
        groups = merged;
        if groups.len() == 1 {
            break;
        }
        let mut square = vec![T::ZERO; min((power.len() << 1) - 1, n)];
        square_truncated_wrapping(&mut square, &power, pow);
        power = square;
    }
    let mut out = groups.pop().unwrap();
    out.resize(n, T::ZERO);
    out
}}

// The first `n` coefficients of the composition of the polynomials with coefficients `xs` and `ys`,
// correct modulo $2^k$, where $k$ is `pow`, `ys[0]` is 0, and `2 <= xs.len() <= n`, by the
// Brent–Kung algorithm: with $m = \lfloor \sqrt n \rfloor + 1$, the products of the $m$ blocks of
// $m$ coefficients of `xs` with the first $m$ powers of `ys` are the rows of a matrix product,
// computed modulo $2^W$, and they are combined by Horner's rule in $y^m$. The result is not
// reduced.
//
// This is equivalent to `_gr_poly_compose_series_brent_kung` from
// `gr_poly/compose_series_brent_kung.c`, FLINT 3.6.0, in wrapping word arithmetic, except that the
// powers of `ys` are always computed by multiplying by `ys`, and that the running polynomial and
// the scratch product trade places at every step.
crate_test_fn! {mod_power_of_2_compose_truncated_brent_kung<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    n: usize,
    pow: u64,
) -> Vec<T> {
    let m = n.floor_sqrt() + 1;
    // The rows of B are the blocks of xs.
    let mut b = vec![T::ZERO; m * m];
    for (i, chunk) in xs.chunks(m).enumerate() {
        b[i * m..i * m + chunk.len()].copy_from_slice(chunk);
    }
    // The rows of A are the powers of ys.
    let mut a = vec![T::ZERO; m * n];
    a[0] = T::ONE;
    a[n..n + ys.len()].copy_from_slice(ys);
    // The powers of `ys` and then `ys^m` itself: m - 1 products.
    let factor = FixedFactor::new(ys, n, NttModulus::PowerOf2(pow), m - 1);
    for i in 2..m {
        let (previous, current) = a.split_at_mut(i * n);
        factor.mul_truncated(&mut current[..n], &previous[(i - 1) * n..]);
    }
    let c = matrix_wrapping_mul(&b, &a, ProductShape { ar: m, br: m, bc: n });
    // Combine the rows by Horner's rule in h = ys^m.
    let mut h = vec![T::ZERO; n];
    factor.mul_truncated(&mut h, &a[(m - 1) * n..]);
    let h_factor = FixedFactor::new(&h, n, NttModulus::PowerOf2(pow), m - 1);
    let mut acc = c[(m - 1) * n..].to_vec();
    let mut scratch = vec![T::ZERO; n];
    for row in c.chunks_exact(n).take(m - 1).rev() {
        h_factor.mul_truncated(&mut scratch, &acc);
        add_wrapping_assign(&mut scratch, row);
        swap(&mut acc, &mut scratch);
    }
    acc
}}

// The first `n` coefficients of $p(cx^v)$, modulo $2^W$, where `xs` holds the coefficients of $p$
// and $v \geq 1$: $p_i c^i$ at $x^{iv}$.
fn mod_power_of_2_compose_truncated_monomial<T: PrimitiveUnsigned>(
    xs: &[T],
    c: T,
    v: usize,
    n: usize,
) -> Vec<T> {
    let mut out = vec![T::ZERO; n];
    let mut power = T::ONE;
    for (o, &x) in out.iter_mut().step_by(v).zip(xs) {
        *o = x.wrapping_mul(power);
        power.wrapping_mul_assign(c);
    }
    out
}

// Whether a composition with an outer polynomial of `len1` coefficients, truncated to `n`
// coefficients, is done by Horner's rule: as measured on this machine, when the outer polynomial
// has at most 7 coefficients, as FLINT chooses, or when the product of its length and `n` is at
// most 128, rather than FLINT's `n <= 7`.
const fn horner_preferred(len1: usize, n: usize) -> bool {
    len1 <= 7 || len1.saturating_mul(n) <= 128
}

// Whether a composition with an outer polynomial of `len1` coefficients and an inner polynomial of
// `len2` coefficients, truncated to `n` coefficients, is done by divide and conquer rather than by
// the Brent–Kung algorithm. As measured on this machine, divide and conquer is faster when the
// outer polynomial is short compared with `n`, $\ell_1^2 \leq 2n$, or, beyond 16 coefficients, when
// the untruncated composition is shorter than $2n$. FLINT chooses it when $\ell_1^2 < n$ or the
// untruncated composition is shorter than $4n$.
const fn divide_and_conquer_preferred(len1: usize, len2: usize, n: usize) -> bool {
    len1.saturating_mul(len1) <= n << 1
        || (n > 16 && (len1 - 1).saturating_mul(len2 - 1) + 1 < n << 1)
}

// The first `n` coefficients of the composition of the polynomials with coefficients `xs` and `ys`,
// correct modulo $2^k$, where $k$ is `pow`, `xs.len() >= 2`, and `ys[0]` is 0: directly if `ys` has
// a single nonzero coefficient, and otherwise by Horner's rule, divide and conquer, or the
// Brent–Kung algorithm.
//
// This is equivalent to `_gr_poly_compose_series` from `gr_poly/compose_series.c`, FLINT 3.6.0, in
// wrapping word arithmetic, except that the monomial case allows a monomial of any degree, that the
// choice of algorithm was measured (see `horner_preferred` and `divide_and_conquer_preferred`), and
// that the Kinoshita–Li algorithm is not used.
fn mod_power_of_2_compose_truncated_dispatch<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    n: usize,
    pow: u64,
) -> Vec<T> {
    let v = ys.iter().position(|&y| y != T::ZERO).unwrap();
    if ys[v + 1..].iter().all(|&y| y == T::ZERO) {
        mod_power_of_2_compose_truncated_monomial(xs, ys[v], v, n)
    } else if horner_preferred(xs.len(), n) {
        mod_power_of_2_compose_truncated_horner(xs, ys, n, pow)
    } else if divide_and_conquer_preferred(xs.len(), ys.len(), n) {
        mod_power_of_2_compose_truncated_divide_and_conquer(xs, ys, n, pow)
    } else {
        mod_power_of_2_compose_truncated_brent_kung(xs, ys, n, pow)
    }
}

// The coefficients of the composition, modulo $2^k$, of the polynomials with coefficients `xs` and
// `ys`, keeping only the coefficients of $x^i$ for $i$ less than `len`, with `kernel` doing the
// work in the general case. The result is reduced and trimmed. If $v$ is the index of the first
// nonzero coefficient of `ys`, the term $p_i q^i$ is divisible by $x^{iv}$, so only the first
// $\lceil n/v \rceil$ coefficients of `xs` are read.
//
// This is equivalent to `nmod_poly_compose_series` from `nmod_poly/compose_series.c`, FLINT 3.6.0,
// with modulus $2^k$, except that `xs` is truncated using $v$.
crate_test_fn! {mod_power_of_2_compose_truncated_with<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    len: u64,
    pow: u64,
    kernel: fn(&[T], &[T], usize, u64) -> Vec<T>,
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
            let mut out = kernel(xs, ys, lenr, pow);
            mask_coefficients(&mut out, pow);
            return from_coefficients_trimmed(out).coefficients;
        }
    }
    from_coefficients_trimmed(vec![xs[0]]).coefficients
}}

/// The coefficients of the composition, modulo $2^k$, of the polynomials with coefficients `xs` and
/// `ys`, both reduced modulo $2^k$, keeping only the coefficients of $x^i$ for $i$ less than `len`,
/// trimmed. `ys` must have constant coefficient 0, and `pow` must be no greater than `T::WIDTH`.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compose
/// `NaturalPolynomial`s with word-sized coefficients modulo $2^k$.
#[doc(hidden)]
#[inline]
pub fn mod_power_of_2_compose_truncated_vec<T: PrimitiveUnsigned>(
    xs: &[T],
    ys: &[T],
    len: u64,
    pow: u64,
) -> Vec<T> {
    mod_power_of_2_compose_truncated_with(
        xs,
        ys,
        len,
        pow,
        mod_power_of_2_compose_truncated_dispatch,
    )
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeTruncated<Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo $2^k$ and truncates the result to its first
    /// `len` coefficients, taking both by value. The inner polynomial must have constant
    /// coefficient 0, and the coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (p(q(x)) \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. The arithmetic is done in wrapping word
    /// arithmetic and reduced modulo $2^k$ at the end, which gives the right result because $2^k$
    /// divides $2^W$. If $q$ has a single nonzero coefficient, the coefficients of $p$ are scaled
    /// and spread out; otherwise short compositions use Horner's rule; those whose outer polynomial
    /// is short compared with the result, or whose untruncated result is not much longer than $n$,
    /// use divide and conquer; and the others use the Brent–Kung algorithm. The result is
    /// trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_compose_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x^2+5*x").unwrap(),
    ///             4,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "11*x^3+12*x^2+5*x+2"
    /// );
    /// // Modulo 4, the coefficient of x vanishes.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+2*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_compose_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap(),
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

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeTruncated<&Self> for UnsignedPolynomial<T> {
    type Output = Self;

    /// Composes two [`UnsignedPolynomial`]s modulo $2^k$ and truncates the result to its first
    /// `len` coefficients, taking the first by value and the second by reference. The inner
    /// polynomial must have constant coefficient 0, and the coefficients of both must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (p(q(x)) \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. The arithmetic is done in wrapping word
    /// arithmetic and reduced modulo $2^k$ at the end, which gives the right result because $2^k$
    /// divides $2^W$. If $q$ has a single nonzero coefficient, the coefficients of $p$ are scaled
    /// and spread out; otherwise short compositions use Horner's rule; those whose outer polynomial
    /// is short compared with the result, or whose untruncated result is not much longer than $n$,
    /// use divide and conquer; and the others use the Brent–Kung algorithm. The result is
    /// trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+x+2")
    ///         .unwrap()
    ///         .mod_power_of_2_compose_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+5*x").unwrap(),
    ///             4,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "11*x^3+12*x^2+5*x+2"
    /// );
    /// // Modulo 4, the coefficient of x vanishes.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+2*x+1")
    ///         .unwrap()
    ///         .mod_power_of_2_compose_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap(),
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

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeTruncated<UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo $2^k$ and truncates the result to its first
    /// `len` coefficients, taking the first by reference and the second by value. The inner
    /// polynomial must have constant coefficient 0, and the coefficients of both must already be
    /// reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (p(q(x)) \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. The arithmetic is done in wrapping word
    /// arithmetic and reduced modulo $2^k$ at the end, which gives the right result because $2^k$
    /// divides $2^W$. If $q$ has a single nonzero coefficient, the coefficients of $p$ are scaled
    /// and spread out; otherwise short compositions use Horner's rule; those whose outer polynomial
    /// is short compared with the result, or whose untruncated result is not much longer than $n$,
    /// use divide and conquer; and the others use the Brent–Kung algorithm. The result is
    /// trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_power_of_2_compose_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x^2+5*x").unwrap(),
    ///             4,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "11*x^3+12*x^2+5*x+2"
    /// );
    /// // Modulo 4, the coefficient of x vanishes.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+2*x+1").unwrap())
    ///         .mod_power_of_2_compose_truncated(
    ///             UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap(),
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
        other: UnsignedPolynomial<T>,
        len: u64,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        self.mod_power_of_2_compose_truncated(&other, len, pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeTruncated<&UnsignedPolynomial<T>>
    for &UnsignedPolynomial<T>
{
    type Output = UnsignedPolynomial<T>;

    /// Composes two [`UnsignedPolynomial`]s modulo $2^k$ and truncates the result to its first
    /// `len` coefficients, taking both by reference. The inner polynomial must have constant
    /// coefficient 0, and the coefficients of both must already be reduced modulo $2^k$.
    ///
    /// $$
    /// f(p, q, n, k) = (p(q(x)) \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. The arithmetic is done in wrapping word
    /// arithmetic and reduced modulo $2^k$ at the end, which gives the right result because $2^k$
    /// divides $2^W$. If $q$ has a single nonzero coefficient, the coefficients of $p$ are scaled
    /// and spread out; otherwise short compositions use Horner's rule; those whose outer polynomial
    /// is short compared with the result, or whose untruncated result is not much longer than $n$,
    /// use divide and conquer; and the others use the Brent–Kung algorithm. The result is
    /// trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeTruncated;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+x+2").unwrap())
    ///         .mod_power_of_2_compose_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+5*x").unwrap(),
    ///             4,
    ///             4
    ///         )
    ///         .to_string(),
    ///     "11*x^3+12*x^2+5*x+2"
    /// );
    /// // Modulo 4, the coefficient of x vanishes.
    /// assert_eq!(
    ///     (&UnsignedPolynomial::<u8>::from_str("x^2+2*x+1").unwrap())
    ///         .mod_power_of_2_compose_truncated(
    ///             &UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap(),
    ///             3,
    ///             2
    ///         )
    ///         .to_string(),
    ///     "2*x^2+1"
    /// );
    /// ```
    fn mod_power_of_2_compose_truncated(
        self,
        other: &UnsignedPolynomial<T>,
        len: u64,
        pow: u64,
    ) -> UnsignedPolynomial<T> {
        assert_reduced(self, other, pow);
        UnsignedPolynomial {
            coefficients: mod_power_of_2_compose_truncated_vec(
                &self.coefficients,
                &other.coefficients,
                len,
                pow,
            ),
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeTruncatedAssign<Self> for UnsignedPolynomial<T> {
    /// Composes an [`UnsignedPolynomial`] with another modulo $2^k$ in place and truncates the
    /// result to its first `len` coefficients, taking the other by value. The inner polynomial must
    /// have constant coefficient 0, and the coefficients of both must already be reduced modulo
    /// $2^k$.
    ///
    /// $$
    /// p \\gets (p(q(x)) \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. The arithmetic is done in wrapping word
    /// arithmetic and reduced modulo $2^k$ at the end, which gives the right result because $2^k$
    /// divides $2^W$. If $q$ has a single nonzero coefficient, the coefficients of $p$ are scaled
    /// and spread out; otherwise short compositions use Horner's rule; those whose outer polynomial
    /// is short compared with the result, or whose untruncated result is not much longer than $n$,
    /// use divide and conquer; and the others use the Brent–Kung algorithm. The result is
    /// trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_power_of_2_compose_truncated_assign(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+5*x").unwrap(),
    ///     4,
    ///     4,
    /// );
    /// assert_eq!(p.to_string(), "11*x^3+12*x^2+5*x+2");
    ///
    /// // Modulo 4, the coefficient of x vanishes.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+2*x+1").unwrap();
    /// p.mod_power_of_2_compose_truncated_assign(
    ///     UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap(),
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

impl<T: PrimitiveUnsigned> ModPowerOf2ComposeTruncatedAssign<&Self> for UnsignedPolynomial<T> {
    /// Composes an [`UnsignedPolynomial`] with another modulo $2^k$ in place and truncates the
    /// result to its first `len` coefficients, taking the other by reference. The inner polynomial
    /// must have constant coefficient 0, and the coefficients of both must already be reduced
    /// modulo $2^k$.
    ///
    /// $$
    /// p \\gets (p(q(x)) \\bmod x^n) \\bmod 2^k.
    /// $$
    ///
    /// Since $q(0) = 0$, the coefficients of $x^i$ for $i \\geq n$ of $p$ and $q$ do not affect the
    /// result, and if the first nonzero coefficient of $q$ is that of $x^v$, only the first
    /// $\\lceil n/v \\rceil$ coefficients of $p$ do. The arithmetic is done in wrapping word
    /// arithmetic and reduced modulo $2^k$ at the end, which gives the right result because $2^k$
    /// divides $2^W$. If $q$ has a single nonzero coefficient, the coefficients of $p$ are scaled
    /// and spread out; otherwise short compositions use Horner's rule; those whose outer polynomial
    /// is short compared with the result, or whose untruncated result is not much longer than $n$,
    /// use divide and conquer; and the others use the Brent–Kung algorithm. The result is
    /// trimmed.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n^{3/2})$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `len`.
    ///
    /// # Panics
    /// Panics if `pow` is greater than `T::WIDTH`, if any coefficient of `self` or `other` is
    /// greater than or equal to $2^k$, or if `other` has a nonzero constant coefficient.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::polynomial::ModPowerOf2ComposeTruncatedAssign;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^3+3*x^2+x+2").unwrap();
    /// p.mod_power_of_2_compose_truncated_assign(
    ///     &UnsignedPolynomial::<u8>::from_str("x^2+5*x").unwrap(),
    ///     4,
    ///     4,
    /// );
    /// assert_eq!(p.to_string(), "11*x^3+12*x^2+5*x+2");
    ///
    /// // Modulo 4, the coefficient of x vanishes.
    /// let mut p = UnsignedPolynomial::<u8>::from_str("x^2+2*x+1").unwrap();
    /// p.mod_power_of_2_compose_truncated_assign(
    ///     &UnsignedPolynomial::<u8>::from_str("x^2+2*x").unwrap(),
    ///     3,
    ///     2,
    /// );
    /// assert_eq!(p.to_string(), "2*x^2+1");
    /// ```
    fn mod_power_of_2_compose_truncated_assign(&mut self, other: &Self, len: u64, pow: u64) {
        assert_reduced(self, other, pow);
        self.coefficients = mod_power_of_2_compose_truncated_vec(
            &take(&mut self.coefficients),
            &other.coefficients,
            len,
            pow,
        );
    }
}
