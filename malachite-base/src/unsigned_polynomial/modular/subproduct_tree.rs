// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2012 William Hart Copyright © 2011, 2012 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Parity, PowerOf2};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::num::exponential::traits::CeilingLogBase2;
use crate::num::logic::traits::SignificantBits;
use crate::unsigned_polynomial::modular::mod_div_rem::{mod_div_rem_dispatch, remainder};
use crate::unsigned_polynomial::modular::mod_div_rem_precomputed::{
    mod_quotient_preinv, reversed_inverse,
};
use crate::unsigned_polynomial::modular::mod_evaluate::mod_evaluate_slice;
use crate::unsigned_polynomial::modular::mod_mul::mod_mul_to_out;
use crate::unsigned_polynomial::modular::mod_power_of_2_div_rem::{
    mod_power_of_2_div_rem_dispatch, remainder as mod_power_of_2_remainder,
};
use crate::unsigned_polynomial::modular::mod_power_of_2_div_rem_precomputed::{
    mod_power_of_2_quotient_preinv, reversed_inverse as mod_power_of_2_reversed_inverse,
};
use crate::unsigned_polynomial::modular::mod_power_of_2_mul::{
    mask_coefficients, mod_power_of_2_mul_to_out,
};
use alloc::vec;
use alloc::vec::Vec;
use core::cmp::{max, min};

/// The arithmetic on coefficients of type `C`, modulo $m$ or modulo $2^k$, that building a
/// subproduct tree, and evaluating by remainders down it, need. Every input is reduced, and so is
/// every output.
///
/// This is not part of the public API; it is public so that `malachite-nz` can build subproduct
/// trees of `NaturalPolynomial`s.
#[doc(hidden)]
pub trait TreeModulus<C: Clone> {
    fn zero(&self) -> C;

    fn one(&self) -> C;

    fn neg(&self, x: &C) -> C;

    fn add(&self, x: &C, y: &C) -> C;

    fn sub(&self, x: &C, y: &C) -> C;

    fn mul(&self, x: &C, y: &C) -> C;

    /// Writes the product of `xs` and `ys` into `out`, which has length `xs.len() + ys.len() - 1`.
    fn mul_to_out(&self, out: &mut [C], xs: &[C], ys: &[C]);

    /// The remainder of `xs` by the monic `ys`, where `xs` is at least as long as `ys`; it may be
    /// shorter than `ys.len() - 1`.
    fn rem_monic(&self, xs: &[C], ys: &[C]) -> Vec<C>;

    /// The inverse of the reversal of the monic `ys`, as a power series truncated to `ys.len()`
    /// coefficients.
    fn reversed_inverse(&self, ys: &[C]) -> Vec<C>;

    /// The remainder of `xs` by the monic `ys`, where `xs` is at least as long as `ys` and has at
    /// most `2 * ys.len() - 2` coefficients, and `inverse` is `self.reversed_inverse(ys)`; it may
    /// be shorter than `ys.len() - 1`.
    fn rem_monic_preinv(&self, xs: &[C], ys: &[C], inverse: &[C]) -> Vec<C>;

    /// The value of the polynomial with coefficients `poly` at `x`.
    fn evaluate(&self, poly: &[C], x: &C) -> C;
}

/// A modulus $m > 1$ of type `T`.
///
/// This is not part of the public API.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct Modulus<T>(pub T);

impl<T: PrimitiveUnsigned> TreeModulus<T> for Modulus<T> {
    #[inline]
    fn zero(&self) -> T {
        T::ZERO
    }

    #[inline]
    fn one(&self) -> T {
        T::ONE
    }

    #[inline]
    fn neg(&self, x: &T) -> T {
        x.mod_neg(self.0)
    }

    #[inline]
    fn add(&self, x: &T, y: &T) -> T {
        x.mod_add(*y, self.0)
    }

    #[inline]
    fn sub(&self, x: &T, y: &T) -> T {
        x.mod_sub(*y, self.0)
    }

    #[inline]
    fn mul(&self, x: &T, y: &T) -> T {
        x.mod_mul(*y, self.0)
    }

    #[inline]
    fn mul_to_out(&self, out: &mut [T], xs: &[T], ys: &[T]) {
        mod_mul_to_out(out, xs, ys, self.0);
    }

    #[inline]
    fn rem_monic(&self, xs: &[T], ys: &[T]) -> Vec<T> {
        mod_div_rem_dispatch(xs, ys, self.0, T::ONE).1
    }

    #[inline]
    fn reversed_inverse(&self, ys: &[T]) -> Vec<T> {
        reversed_inverse(ys, self.0, T::ONE)
    }

    #[inline]
    fn rem_monic_preinv(&self, xs: &[T], ys: &[T], inverse: &[T]) -> Vec<T> {
        remainder(
            xs,
            ys,
            &mod_quotient_preinv(xs, ys, inverse, self.0),
            self.0,
        )
    }

    #[inline]
    fn evaluate(&self, poly: &[T], x: &T) -> T {
        mod_evaluate_slice(poly, *x, self.0)
    }
}

/// A modulus $2^k$, where $0 < k \leq$ `T::WIDTH`, for coefficients of type `T`; the field is $k$.
///
/// This is not part of the public API.
#[doc(hidden)]
#[derive(Clone, Copy)]
pub struct PowerOf2Modulus(pub u64);

impl<T: PrimitiveUnsigned> TreeModulus<T> for PowerOf2Modulus {
    #[inline]
    fn zero(&self) -> T {
        T::ZERO
    }

    #[inline]
    fn one(&self) -> T {
        T::ONE
    }

    #[inline]
    fn neg(&self, x: &T) -> T {
        x.mod_power_of_2_neg(self.0)
    }

    #[inline]
    fn add(&self, x: &T, y: &T) -> T {
        x.mod_power_of_2_add(*y, self.0)
    }

    #[inline]
    fn sub(&self, x: &T, y: &T) -> T {
        x.mod_power_of_2_sub(*y, self.0)
    }

    #[inline]
    fn mul(&self, x: &T, y: &T) -> T {
        x.mod_power_of_2_mul(*y, self.0)
    }

    #[inline]
    fn mul_to_out(&self, out: &mut [T], xs: &[T], ys: &[T]) {
        mod_power_of_2_mul_to_out(out, xs, ys, self.0);
    }

    // A monic divisor needs no inverse, so this works modulo $2^k$ although most leading
    // coefficients are not invertible.
    fn rem_monic(&self, xs: &[T], ys: &[T]) -> Vec<T> {
        let mut rs = mod_power_of_2_div_rem_dispatch(xs, ys, self.0, T::ONE).1;
        mask_coefficients(&mut rs, self.0);
        rs
    }

    #[inline]
    fn reversed_inverse(&self, ys: &[T]) -> Vec<T> {
        mod_power_of_2_reversed_inverse(ys, self.0, T::ONE)
    }

    fn rem_monic_preinv(&self, xs: &[T], ys: &[T], inverse: &[T]) -> Vec<T> {
        let qs = mod_power_of_2_quotient_preinv(xs, ys, inverse, self.0);
        let mut rs = mod_power_of_2_remainder(xs, ys, &qs, self.0);
        mask_coefficients(&mut rs, self.0);
        rs
    }

    fn evaluate(&self, poly: &[T], x: &T) -> T {
        let mut value = T::ZERO;
        for &c in poly.iter().rev() {
            value = value.wrapping_mul(*x).wrapping_add(c);
        }
        value.mod_power_of_2(self.0)
    }
}

// Whether evaluating a polynomial of length `n` at `k` points modulo `m` uses remainders down a
// subproduct tree rather than Horner's rule in blocks of points. Horner's rule with Shoup's
// multiplication, and fastest with its lazy form, which needs `m` to be at most `T::MAX / 3`, is so
// fast that the tree only wins for long polynomials and many points; when the top bit of `m` is
// set, Shoup's multiplication is unavailable and the tree wins much sooner. The tree's cost grows
// with the number of points much faster than Horner's rule's does, so with a polynomial that is
// only moderately long and many more points than coefficients, Horner's rule wins again; for a
// longer polynomial the tree wins with any number of points, as it must asymptotically. Measured on
// an Apple M-series machine, 2026-11, for `u64` with moduli of 30, 63, a little under 64, and 64
// bits; FLINT switches at a length of 32 whatever the number of points. The boundaries for other
// types are those for `u64`, without being measured.
crate_test_fn! {mod_evaluate_many_tree_preferred<T: PrimitiveUnsigned>(
    n: usize,
    k: usize,
    m: T,
) -> bool {
    let bits = m.significant_bits();
    if bits == T::WIDTH {
        k >= 32 && (n >= 1024 || n >= 256 && k <= n << 2)
    } else if m > T::MAX / T::from(3u8) {
        n >= 4096 && k >= 4096
    } else if bits << 1 > T::WIDTH {
        n >= 8192 && k >= 4096
    } else {
        n >= 4096 && k >= 1024
    }
}}

// Whether evaluating a polynomial of length `n` at `k` points modulo a power of 2 uses remainders
// down a subproduct tree rather than wrapping Horner's rule in blocks of points. As modulo $m$, a
// moderately long polynomial keeps Horner's rule when there are many more points than coefficients.
// Measured on an Apple M-series machine, 2026-11, for `u64` with moduli of 2^32 and 2^64.
crate_test_fn! {
#[allow(clippy::missing_const_for_fn)] // crate_test_fn! cannot declare a const fn
mod_power_of_2_evaluate_many_tree_preferred(n: usize, k: usize) -> bool {
    n >= 1024 && k >= 256 && (n >= 2048 || k <= n << 3)
}}

// ceil(log_2(len)), for len > 0, as FLINT's FLINT_CLOG2.
fn clog2(len: usize) -> usize {
    usize::exact_from(len.ceiling_log_base_2())
}

/// The subproduct tree of the linear factors $x - r_i$, laid out as described at
/// `ModEvaluateManyData`. The roots must be reduced.
///
/// This is not part of the public API; it is public so that `malachite-nz` can build subproduct
/// trees of `NaturalPolynomial`s.
///
/// This is equivalent to `_nmod_poly_tree_alloc` and `_nmod_poly_tree_build` from
/// `nmod_poly/tree.c`, FLINT 3.6.0.
#[doc(hidden)]
pub fn subproduct_tree<C: Clone, M: TreeModulus<C>>(roots: &[C], m: &M) -> Vec<Vec<C>> {
    let len = roots.len();
    if len == 0 {
        return Vec::new();
    }
    let height = clog2(len);
    // The top level, the product of all the factors, is never used, so it is not allocated; FLINT
    // allocates it but leaves it unset. A single factor still needs level 0.
    let mut tree: Vec<Vec<C>> = (0..max(height, 1))
        .map(|i| vec![m.zero(); len + (len >> i) + 1])
        .collect();
    // zeroth level, (x - a)
    for (i, r) in roots.iter().enumerate() {
        tree[0][(i << 1) + 1] = m.one();
        tree[0][i << 1] = m.neg(r);
    }
    // first level, (x - a)(x - b) = x^2 + (-a - b)x + ab
    if height > 1 {
        let pa = &mut tree[1];
        for i in 0..len >> 1 {
            let a = &roots[i << 1];
            let b = &roots[(i << 1) + 1];
            pa[3 * i] = m.mul(a, b);
            pa[3 * i + 1] = m.neg(&m.add(a, b));
            pa[3 * i + 2] = m.one();
        }
        if len.odd() {
            pa[3 * (len >> 1)] = m.neg(&roots[len - 1]);
            pa[3 * (len >> 1) + 1] = m.one();
        }
    }
    for i in 1..height.saturating_sub(1) {
        let mut left = len;
        let pow = usize::power_of_2(u64::exact_from(i));
        let (lower, upper) = tree.split_at_mut(i + 1);
        let pa = &lower[i];
        let pb = &mut upper[0];
        let mut a = 0;
        let mut b = 0;
        while left >= pow << 1 {
            m.mul_to_out(
                &mut pb[b..=(b + (pow << 1))],
                &pa[a..=(a + pow)],
                &pa[a + pow + 1..a + (pow << 1) + 2],
            );
            left -= pow << 1;
            a += (pow << 1) + 2;
            b += (pow << 1) + 1;
        }
        if left > pow {
            m.mul_to_out(
                &mut pb[b..=(b + left)],
                &pa[a..=(a + pow)],
                &pa[a + pow + 1..a + left + 2],
            );
        } else if left > 0 {
            pb[b..=(b + left)].clone_from_slice(&pa[a..=(a + left)]);
        }
    }
    tree
}

// Writes into `out` the remainder of the polynomial with coefficients `xs` by the monic polynomial
// with coefficients `ys`; `out` has length `ys.len() - 1`, and is zero-padded. The dividend is
// always longer than the divisor's degree: the initial reduction picks a level whose nodes have
// degree less than the polynomial's length, and every later dividend is a pair of remainders by the
// node above.
//
// When `inverse`, the inverse of the reversal of `ys`, is given and the dividend is short enough,
// the quotient is one truncated multiplication rather than a power series division.
fn rem_monic_to_out<C: Clone, M: TreeModulus<C>>(
    out: &mut [C],
    xs: &[C],
    ys: &[C],
    inverse: Option<&[C]>,
    m: &M,
) {
    debug_assert!(xs.len() >= ys.len());
    if xs.len() == 2 {
        // a linear divisor x + b: the remainder is a_0 - a_1 b
        out[0] = m.sub(&xs[0], &m.mul(&xs[1], &ys[0]));
    } else {
        let rs = match inverse {
            Some(inverse) if xs.len() <= (ys.len() << 1) - 2 => m.rem_monic_preinv(xs, ys, inverse),
            _ => m.rem_monic(xs, ys),
        };
        let n = rs.len();
        for (o, r) in out.iter_mut().zip(rs) {
            *o = r;
        }
        out[n..].fill(m.zero());
    }
}

/// The values of the polynomial with coefficients `poly` at the `len` points of the subproduct tree
/// `tree`, built by [`subproduct_tree`]. The coefficients must be reduced. `inverses` is either
/// empty or built by [`subproduct_tree_inverses`], in which case most remainders take one truncated
/// multiplication rather than a power series division.
///
/// This is not part of the public API; it is public so that `malachite-nz` can evaluate
/// `NaturalPolynomial`s at many points.
///
/// This is equivalent to `_nmod_poly_evaluate_nmod_vec_fast_precomp` from
/// `nmod_poly/evaluate_nmod_vec.c`, FLINT 3.6.0.
#[doc(hidden)]
pub fn evaluate_many_fast_precomputed<C: Clone, M: TreeModulus<C>>(
    poly: &[C],
    tree: &[Vec<C>],
    inverses: &[Vec<C>],
    len: usize,
    m: &M,
) -> Vec<C> {
    let inverse = |level: usize, start: usize, end: usize| {
        inverses
            .get(level)
            .filter(|level| !level.is_empty())
            .map(|level| &level[start..end])
    };
    let plen = poly.len();
    // avoid worrying about some degenerate cases
    match (len, plen) {
        (0, _) => return Vec::new(),
        (1, _) => return vec![m.evaluate(poly, &m.neg(&tree[0][0]))],
        (_, 0) => return vec![m.zero(); len],
        (_, 1) => return vec![poly[0].clone(); len],
        _ => {}
    }
    let mut t = vec![m.zero(); len];
    let mut u = vec![m.zero(); len];
    // Initial reduction. We allow the polynomial to be larger or smaller than the number of points.
    let height = min(
        usize::exact_from((plen - 1).significant_bits()) - 1,
        clog2(len) - 1,
    );
    let pow = usize::power_of_2(u64::exact_from(height));
    let mut i = 0;
    let mut j = 0;
    while i < len {
        let tlen = if i + pow <= len { pow } else { len % pow };
        rem_monic_to_out(
            &mut t[i..i + tlen],
            poly,
            &tree[height][j..=(j + tlen)],
            inverse(height, j, j + tlen + 1),
            m,
        );
        i += pow;
        j += pow + 1;
    }
    for i in (0..height).rev() {
        let pow = usize::power_of_2(u64::exact_from(i));
        let mut left = len;
        let pa = &tree[i];
        let mut a = 0;
        let mut bc = 0;
        while left >= pow << 1 {
            let pb = &t[bc..bc + (pow << 1)];
            rem_monic_to_out(
                &mut u[bc..bc + pow],
                pb,
                &pa[a..=(a + pow)],
                inverse(i, a, a + pow + 1),
                m,
            );
            rem_monic_to_out(
                &mut u[bc + pow..bc + (pow << 1)],
                pb,
                &pa[a + pow + 1..a + (pow << 1) + 2],
                inverse(i, a + pow + 1, a + (pow << 1) + 2),
                m,
            );
            a += (pow << 1) + 2;
            bc += pow << 1;
            left -= pow << 1;
        }
        if left > pow {
            let pb = &t[bc..bc + left];
            rem_monic_to_out(
                &mut u[bc..bc + pow],
                pb,
                &pa[a..=(a + pow)],
                inverse(i, a, a + pow + 1),
                m,
            );
            rem_monic_to_out(
                &mut u[bc + pow..bc + left],
                pb,
                &pa[a + pow + 1..a + left + 2],
                inverse(i, a + pow + 1, a + left + 2),
                m,
            );
        } else if left > 0 {
            u[bc..bc + left].clone_from_slice(&t[bc..bc + left]);
        }
        core::mem::swap(&mut t, &mut u);
    }
    t
}

/// The values of the polynomial with coefficients `poly` at the points `xs`, by building their
/// subproduct tree. The coefficients and the points must be reduced.
///
/// This is not part of the public API; it is public so that `malachite-nz` can evaluate
/// `NaturalPolynomial`s at many points.
///
/// This is equivalent to `_nmod_poly_evaluate_nmod_vec_fast` from `nmod_poly/evaluate_nmod_vec.c`,
/// FLINT 3.6.0.
#[doc(hidden)]
pub fn evaluate_many_fast<C: Clone, M: TreeModulus<C>>(poly: &[C], xs: &[C], m: &M) -> Vec<C> {
    let tree = subproduct_tree(xs, m);
    evaluate_many_fast_precomputed(poly, &tree, &[], xs.len(), m)
}

/// The inverses of the reversals of the nodes of the subproduct tree `tree` of `len` points, each
/// truncated to its node's length and laid out like the tree. Level 0 is left empty: its nodes are
/// linear, and a remainder by a linear node is found directly.
///
/// This is not part of the public API; it is public so that `malachite-nz` can precompute data for
/// evaluating `NaturalPolynomial`s at many points.
#[doc(hidden)]
#[inline]
pub fn subproduct_tree_inverses<C: Clone, M: TreeModulus<C>>(
    tree: &[Vec<C>],
    len: usize,
    m: &M,
) -> Vec<Vec<C>> {
    tree.iter()
        .enumerate()
        .map(|(level, nodes)| {
            if level == 0 {
                return Vec::new();
            }
            let pow = usize::power_of_2(u64::exact_from(level));
            let mut inverses = vec![m.zero(); nodes.len()];
            let mut i = 0;
            let mut j = 0;
            while i < len {
                let node_len = min(pow, len - i) + 1;
                inverses[j..j + node_len]
                    .clone_from_slice(&m.reversed_inverse(&nodes[j..j + node_len]));
                i += pow;
                j += pow + 1;
            }
            inverses
        })
        .collect()
}
