// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 William Hart
//
//      Copyright © 2011 Sebastian Pancratz
//
//      Copyright © 2013 Mike Hansen
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::fail_on_untested_path;
use crate::num::arithmetic::traits::{NegAssign, Parity};
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_polynomial::modular::mod_extended_gcd::xgcd_euclidean;
use crate::unsigned_polynomial::modular::mod_gcd::{GcdModulus, gcd_euclidean, normalize};
use alloc::vec::Vec;
use core::cmp::min;

// A 2 x 2 matrix of polynomials, in the order [[0, 1], [2, 3]].
pub(crate) type PolynomialMatrix<C> = [Vec<C>; 4];

// The result of a half-GCD step: the sign of the matrix's determinant, the matrix (when it was
// asked for), and the two remainders it reaches.
pub(crate) struct Hgcd<C> {
    pub sign: i8,
    pub matrix: Option<PolynomialMatrix<C>>,
    pub a: Vec<C>,
    pub b: Vec<C>,
}

fn identity<C: Clone, M: GcdModulus<C>>(m: &M) -> PolynomialMatrix<C> {
    [alloc::vec![m.one()], Vec::new(), Vec::new(), alloc::vec![m.one()]]
}

pub(crate) fn mul<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Vec<C> {
    if xs.is_empty() || ys.is_empty() {
        return Vec::new();
    }
    let mut out = m.mul_polynomials(xs, ys);
    normalize(&mut out, m);
    out
}

// `xs += ys`, without zeros at the end.
pub(crate) fn add_assign<C: Clone, M: GcdModulus<C>>(xs: &mut Vec<C>, ys: &[C], m: &M) {
    if xs.len() < ys.len() {
        xs.resize(ys.len(), m.zero());
    }
    for (x, y) in xs.iter_mut().zip(ys) {
        *x = m.add(x, y);
    }
    normalize(xs, m);
}

// `xs -= ys`, without zeros at the end.
pub(crate) fn sub_assign<C: Clone, M: GcdModulus<C>>(xs: &mut Vec<C>, ys: &[C], m: &M) {
    if xs.len() < ys.len() {
        xs.resize(ys.len(), m.zero());
    }
    for (x, y) in xs.iter_mut().zip(ys) {
        *x = m.sub(x, y);
    }
    normalize(xs, m);
}

// `xs = ys - xs`, without zeros at the end.
pub(crate) fn sub_from<C: Clone, M: GcdModulus<C>>(xs: &mut Vec<C>, ys: &[C], m: &M) {
    if xs.len() < ys.len() {
        xs.resize(ys.len(), m.zero());
    }
    for (i, x) in xs.iter_mut().enumerate() {
        *x = match ys.get(i) {
            Some(y) => m.sub(y, x),
            None => m.neg(x),
        };
    }
    normalize(xs, m);
}

// The quotient and remainder of `xs` by `ys`, both without zeros at the end, or `None` if the
// leading coefficient of `ys` has no inverse. A dividend shorter than the divisor is its own
// remainder.
pub(crate) fn div_rem<C: Clone, M: GcdModulus<C>>(
    xs: &[C],
    ys: &[C],
    m: &M,
) -> Option<(Vec<C>, Vec<C>)> {
    if xs.len() < ys.len() {
        // Every caller divides by a polynomial no longer than the dividend; FLINT's `__divrem`
        // allows otherwise.
        fail_on_untested_path("hgcd div_rem, xs.len() < ys.len()");
        return Some((Vec::new(), xs.to_vec()));
    }
    let inverse = m.inverse(ys.last().unwrap())?;
    let (q, mut r) = m.div_rem(xs, ys, &inverse);
    normalize(&mut r, m);
    Some((q, r))
}

// The coefficients of `xs` from `k` on: the quotient by x^k.
fn shift<C: Clone>(xs: &[C], k: usize) -> &[C] {
    if xs.len() >= k { &xs[k..] } else { &[] }
}

// The coefficients of `xs` below `k`, without zeros at the end: the remainder by x^k.
fn truncate<C: Clone, M: GcdModulus<C>>(xs: &[C], k: usize, m: &M) -> Vec<C> {
    let mut out = xs[..min(k, xs.len())].to_vec();
    normalize(&mut out, m);
    out
}

// `xs + x^k * ys`, without zeros at the end.
fn add_shifted<C: Clone, M: GcdModulus<C>>(mut xs: Vec<C>, ys: &[C], k: usize, m: &M) -> Vec<C> {
    if xs.len() < k + ys.len() {
        xs.resize(k + ys.len(), m.zero());
    }
    for (x, y) in xs[k..].iter_mut().zip(ys) {
        *x = m.add(x, y);
    }
    normalize(&mut xs, m);
    xs
}

// The product of two matrices by the classical method.
//
// This is equivalent to `__mat_mul_classical` from `gr_poly/hgcd.c`, FLINT 3.6.0.
fn mat_mul_classical<C: Clone, M: GcdModulus<C>>(
    a: &PolynomialMatrix<C>,
    b: &PolynomialMatrix<C>,
    m: &M,
) -> PolynomialMatrix<C> {
    let entry = |i: usize, j: usize| {
        let mut c = mul(&a[i], &b[j], m);
        add_assign(&mut c, &mul(&a[i + 1], &b[j + 2], m), m);
        c
    };
    [entry(0, 0), entry(0, 1), entry(2, 0), entry(2, 1)]
}

// The product of two matrices by Strassen's method.
//
// This is equivalent to `__mat_mul_strassen` from `gr_poly/hgcd.c`, FLINT 3.6.0.
fn mat_mul_strassen<C: Clone, M: GcdModulus<C>>(
    a: &PolynomialMatrix<C>,
    b: &PolynomialMatrix<C>,
    m: &M,
) -> PolynomialMatrix<C> {
    let mut t0 = a[0].clone();
    sub_assign(&mut t0, &a[2], m);
    let mut t1 = b[3].clone();
    sub_assign(&mut t1, &b[1], m);
    let mut c2 = mul(&t0, &t1, m);
    t0.clone_from(&a[2]);
    add_assign(&mut t0, &a[3], m);
    t1.clone_from(&b[1]);
    sub_assign(&mut t1, &b[0], m);
    let mut c3 = mul(&t0, &t1, m);
    sub_assign(&mut t0, &a[0], m);
    sub_from(&mut t1, &b[3], m);
    let mut c1 = mul(&t0, &t1, m);
    sub_from(&mut t0, &a[1], m);
    let mut c0 = mul(&t0, &b[3], m);
    t0 = mul(&a[0], &b[0], m);
    add_assign(&mut c1, &t0, m);
    add_assign(&mut c2, &c1, m);
    add_assign(&mut c1, &c3, m);
    add_assign(&mut c3, &c2, m);
    add_assign(&mut c1, &c0, m);
    sub_assign(&mut t1, &b[2], m);
    c0 = mul(&a[3], &t1, m);
    sub_assign(&mut c2, &c0, m);
    c0 = mul(&a[1], &b[2], m);
    add_assign(&mut c0, &t0, m);
    [c0, c1, c2, c3]
}

// The product of two matrices, by Strassen's method when every entry has at least 20 coefficients.
//
// This is equivalent to `__mat_mul` from `gr_poly/hgcd.c`, FLINT 3.6.0.
fn mat_mul<C: Clone, M: GcdModulus<C>>(
    a: &PolynomialMatrix<C>,
    b: &PolynomialMatrix<C>,
    m: &M,
) -> PolynomialMatrix<C> {
    let shortest = a.iter().chain(b.iter()).map(Vec::len).min().unwrap();
    if shortest < 20 {
        mat_mul_classical(a, b, m)
    } else {
        mat_mul_strassen(a, b, m)
    }
}

// The state for tracking the resultant through the half-GCD: the resultant so far, the leading
// coefficient of the current divisor, the lengths of the current pair of remainders, and the offset
// by which the current subproblem is shifted.
//
// This is equivalent to `gr_poly_res_struct` from `gr_poly.h`, FLINT 3.6.0.
pub(crate) struct ResultantState<C> {
    pub res: C,
    pub lc: C,
    pub len0: i64,
    pub len1: i64,
    pub off: i64,
}

pub(crate) fn pow<C: Clone, M: GcdModulus<C>>(x: &C, mut e: u64, m: &M) -> C {
    let mut result = m.one();
    let mut base = x.clone();
    while e != 0 {
        if e.odd() {
            result = m.mul(&result, &base);
        }
        e >>= 1;
        if e != 0 {
            base = m.mul(&base, &base);
        }
    }
    result
}

fn len(xs: &[impl Clone]) -> i64 {
    i64::exact_from(xs.len())
}

impl<C: Clone> ResultantState<C> {
    // Multiplies the resultant by the leading coefficient raised to `e`, negating it if `len_a +
    // off` and `len_b + off` are both even.
    fn update<M: GcdModulus<C>>(&mut self, e: i64, len_a: i64, len_b: i64, m: &M) {
        self.lc = pow(&self.lc, u64::exact_from(e), m);
        self.res = m.mul(&self.res, &self.lc);
        if ((len_a + self.off) | (len_b + self.off)).even() {
            self.res = m.neg(&self.res);
        }
    }
}

// The iterative half-GCD step, where `a` is longer than `b` and `b` is nonzero: Euclid's algorithm
// until the remainder is shorter than half of `a`, recording the quotients in the matrix and, if
// `res` is present, tracking the resultant.
//
// This is equivalent to `_gr_poly_hgcd_recursive_iter` from `gr_poly/hgcd.c`, FLINT 3.6.0.
fn hgcd_iter<C: Clone, M: GcdModulus<C>>(
    a: &[C],
    b: &[C],
    m: &M,
    res: &mut Option<ResultantState<C>>,
) -> Option<Hgcd<C>> {
    let half = a.len() >> 1;
    let mut b = b.to_vec();
    normalize(&mut b, m);
    let mut matrix = identity(m);
    let mut a = a.to_vec();
    let mut sign = 1;
    while b.len() > half {
        if let Some(res) = res {
            res.lc = b.last().unwrap().clone();
        }
        let (q, r) = div_rem(&a, &b, m)?;
        if let Some(res) = res {
            if r.len() > half {
                res.update(len(&a) - len(&r), len(&a), len(&b), m);
            } else {
                res.len0 = len(&a);
                res.len1 = len(&b);
            }
        }
        a = core::mem::replace(&mut b, r);
        let mut new_2 = mul(&q, &matrix[2], m);
        add_assign(&mut new_2, &matrix[3], m);
        matrix[3] = core::mem::replace(&mut matrix[2], new_2);
        let mut new_0 = mul(&q, &matrix[0], m);
        add_assign(&mut new_0, &matrix[1], m);
        matrix[1] = core::mem::replace(&mut matrix[0], new_0);
        sign.neg_assign();
    }
    Some(Hgcd {
        sign,
        matrix: Some(matrix),
        a,
        b,
    })
}

// Shifts the resultant state into a subproblem whose polynomials are divided by $x^k$.
fn enter<C>(res: &mut Option<ResultantState<C>>, k: usize) {
    if let Some(res) = res {
        let k = i64::exact_from(k);
        res.len0 -= k;
        res.len1 -= k;
        res.off += k;
    }
}

// Shifts the resultant state back out of a subproblem whose polynomials were divided by $x^k$.
fn leave<C>(res: &mut Option<ResultantState<C>>, k: usize) {
    if let Some(res) = res {
        let k = i64::exact_from(k);
        res.len0 += k;
        res.len1 += k;
        res.off -= k;
    }
}

// The recursive half-GCD, where `a` is longer than `b` and `b` is nonzero, computing the matrix
// only when `flag` is set and, if `res` is present, tracking the resultant.
//
// This is equivalent to `_gr_poly_hgcd_recursive` from `gr_poly/hgcd.c`, FLINT 3.6.0.
fn hgcd_recursive<C: Clone, M: GcdModulus<C>>(
    a: &[C],
    b: &[C],
    cutoff: usize,
    flag: bool,
    m: &M,
    res: &mut Option<ResultantState<C>>,
) -> Option<Hgcd<C>> {
    let half = a.len() >> 1;
    if b.len() <= half {
        return Some(Hgcd {
            sign: 1,
            matrix: if flag { Some(identity(m)) } else { None },
            a: a.to_vec(),
            b: b.to_vec(),
        });
    }
    let a0 = shift(a, half);
    let b0 = shift(b, half);
    if let Some(res) = res {
        res.lc = b.last().unwrap().clone();
    }
    enter(res, half);
    let first = if a0.len() < cutoff {
        hgcd_iter(a0, b0, m, res)?
    } else {
        hgcd_recursive(a0, b0, cutoff, true, m, res)?
    };
    leave(res, half);
    let r = first.matrix.unwrap();
    let s = truncate(a, half, m);
    let t = truncate(b, half, m);
    let mut b2 = mul(&r[2], &s, m);
    let t0 = mul(&r[0], &t, m);
    if first.sign < 0 {
        sub_assign(&mut b2, &t0, m);
    } else {
        sub_from(&mut b2, &t0, m);
    }
    let b2 = add_shifted(b2, &first.b, half, m);
    let mut a2 = mul(&r[3], &s, m);
    let t0 = mul(&r[1], &t, m);
    if first.sign < 0 {
        sub_from(&mut a2, &t0, m);
    } else {
        sub_assign(&mut a2, &t0, m);
    }
    let a2 = add_shifted(a2, &first.a, half, m);
    if b2.len() <= half {
        return Some(Hgcd {
            sign: first.sign,
            matrix: if flag { Some(r) } else { None },
            a: a2,
            b: b2,
        });
    }
    let k = (half << 1) + 1 - b2.len();
    if let Some(res) = res {
        // Something happened in the first half.
        if b2.len() < b.len() {
            let e = res.len0 - len(&b2);
            let (len0, len1) = (res.len0, res.len1);
            res.update(e, len0, len1, m);
        }
        res.lc = b2.last().unwrap().clone();
        res.len0 = len(&a2);
        res.len1 = len(&b2);
    }
    let (q, d) = div_rem(&a2, &b2, m)?;
    let c0 = shift(&b2, k);
    let d0 = shift(&d, k);
    if let Some(res) = res
        && d.len() > half
    {
        res.update(len(&a2) - len(&d), len(&a2), len(&b2), m);
        res.len0 = len(&b2);
        res.len1 = len(&d);
    }
    enter(res, k);
    let second = if c0.len() < cutoff {
        hgcd_iter(c0, d0, m, res)?
    } else {
        hgcd_recursive(c0, d0, cutoff, true, m, res)?
    };
    leave(res, k);
    let mut s_matrix = second.matrix.unwrap();
    let s = truncate(&b2, k, m);
    let t = truncate(&d, k, m);
    let mut new_b = mul(&s_matrix[2], &s, m);
    let t0 = mul(&s_matrix[0], &t, m);
    if second.sign < 0 {
        sub_assign(&mut new_b, &t0, m);
    } else {
        sub_from(&mut new_b, &t0, m);
    }
    let new_b = add_shifted(new_b, &second.b, k, m);
    let mut new_a = mul(&s_matrix[3], &s, m);
    let t0 = mul(&s_matrix[1], &t, m);
    if second.sign < 0 {
        sub_from(&mut new_a, &t0, m);
    } else {
        sub_assign(&mut new_a, &t0, m);
    }
    let new_a = add_shifted(new_a, &second.a, k, m);
    let matrix = if flag {
        s_matrix.swap(0, 2);
        s_matrix.swap(1, 3);
        let product = mul(&s_matrix[2], &q, m);
        add_assign(&mut s_matrix[0], &product, m);
        let product = mul(&s_matrix[3], &q, m);
        add_assign(&mut s_matrix[1], &product, m);
        Some(mat_mul(&r, &s_matrix, m))
    } else {
        None
    };
    Some(Hgcd {
        sign: -(first.sign * second.sign),
        matrix,
        a: new_a,
        b: new_b,
    })
}

// The half-GCD of `a` and `b`, where `a` is longer than `b` and both are nonzero, or `None` if a
// leading coefficient met along the way has no inverse. The matrix is computed only when
// `want_matrix` is set. If `r` is present, it is multiplied by the factor of the resultant of `a`
// and `b` accounted for by the steps taken, so that it becomes the resultant once multiplied by the
// resultant of the two remainders returned.
//
// This is equivalent to `_gr_poly_hgcd` from `gr_poly/hgcd.c`, FLINT 3.6.0.
pub(crate) fn hgcd<C: Clone, M: GcdModulus<C>>(
    a: &[C],
    b: &[C],
    cutoff: usize,
    want_matrix: bool,
    m: &M,
    r: Option<&mut C>,
) -> Option<Hgcd<C>> {
    let Some(r) = r else {
        return hgcd_recursive(a, b, cutoff, want_matrix, m, &mut None);
    };
    let mut res = Some(ResultantState {
        res: r.clone(),
        lc: b.last().unwrap().clone(),
        len0: len(a),
        len1: len(b),
        off: 0,
    });
    let result = hgcd_recursive(a, b, cutoff, want_matrix, m, &mut res)?;
    let mut res = res.unwrap();
    // Make sure something happened.
    if result.b.len() < b.len() {
        if result.b.is_empty() {
            res.res = if res.len1 == 1 {
                // Reaching 0 means dividing by a constant, which the half-GCD never does: a
                // constant `b` is returned at once, and each subproblem's `b` is longer than half
                // its `a`.
                fail_on_untested_path("hgcd, res.len1 == 1");
                res.lc = pow(&res.lc, u64::exact_from(res.len0 - 1), m);
                m.mul(&res.res, &res.lc)
            } else {
                m.zero()
            };
        } else {
            let e = res.len0 - len(&result.b);
            let (len0, len1) = (res.len0, res.len1);
            res.update(e, len0, len1, m);
        }
    }
    *r = res.res;
    Some(result)
}

/// A GCD, not normalized, of the polynomials with coefficients `xs` and `ys`, where `xs` is at
/// least as long as `ys` and neither is empty or ends with a zero, by the half-GCD algorithm down
/// to `cutoff` coefficients and Euclid's algorithm below that, with `inner_cutoff` passed on to the
/// half-GCD; or `None` if a leading coefficient met along the way has no inverse.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compute GCDs of
/// `NaturalPolynomial`s.
///
/// This is equivalent to `_gr_poly_gcd_hgcd` from `gr_poly/gcd_hgcd.c`, FLINT 3.6.0.
#[doc(hidden)]
pub fn gcd_hgcd<C: Clone, M: GcdModulus<C>>(
    xs: &[C],
    ys: &[C],
    inner_cutoff: usize,
    cutoff: usize,
    m: &M,
) -> Option<Vec<C>> {
    let r = div_rem(xs, ys, m)?.1;
    if r.is_empty() {
        return Some(ys.to_vec());
    }
    let step = hgcd(ys, &r, inner_cutoff, false, m, None)?;
    let mut g = step.a;
    let mut j = step.b;
    while !j.is_empty() {
        let r = div_rem(&g, &j, m)?.1;
        if r.is_empty() {
            return Some(j);
        }
        if j.len() < cutoff {
            return gcd_euclidean(&j, &r, m);
        }
        let step = hgcd(&j, &r, inner_cutoff, false, m, None)?;
        g = step.a;
        j = step.b;
    }
    Some(g)
}

fn neg<C: Clone, M: GcdModulus<C>>(xs: &[C], m: &M) -> Vec<C> {
    xs.iter().map(|x| m.neg(x)).collect()
}

// The exact quotient of `xs` by `ys`, where the last coefficient of `ys` is known to have an
// inverse.
fn div_exact<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Vec<C> {
    div_rem(xs, ys, m).unwrap().0
}

/// A GCD $g$, not normalized, of the polynomials $a$ and $b$ with coefficients `xs` and `ys`,
/// together with cofactors $s$ and $t$ such that $sa + tb = g$, where `xs` is at least as long as
/// `ys`, `ys` has at least two coefficients, and neither ends with a zero; by the half-GCD
/// algorithm, switching to Euclid's algorithm below `cutoff` coefficients and passing
/// `inner_cutoff` on to the half-GCD; or `None` if a leading coefficient met along the way has no
/// inverse.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compute extended
/// GCDs of `NaturalPolynomial`s.
///
/// This is equivalent to `_gr_poly_xgcd_hgcd` from `gr_poly/xgcd_hgcd.c`, FLINT 3.6.0.
#[doc(hidden)]
#[allow(clippy::type_complexity)]
pub fn xgcd_hgcd<C: Clone, M: GcdModulus<C>>(
    xs: &[C],
    ys: &[C],
    inner_cutoff: usize,
    cutoff: usize,
    m: &M,
) -> Option<(Vec<C>, Vec<C>, Vec<C>)> {
    let r = div_rem(xs, ys, m)?.1;
    if r.is_empty() {
        return Some((ys.to_vec(), Vec::new(), alloc::vec![m.one()]));
    }
    let step = hgcd(ys, &r, inner_cutoff, true, m, None)?;
    let matrix = step.matrix.unwrap();
    let (mut s, mut t) = if step.sign > 0 {
        (neg(&matrix[1], m), matrix[0].clone())
    } else {
        (matrix[1].clone(), neg(&matrix[0], m))
    };
    let mut h = step.a;
    let mut j = step.b;
    let g = loop {
        if j.is_empty() {
            break h;
        }
        let (q, r) = div_rem(&h, &j, m)?;
        let v = mul(&q, &t, m);
        core::mem::swap(&mut s, &mut t);
        if r.is_empty() {
            break j;
        }
        sub_assign(&mut t, &v, m);
        if j.len() < cutoff {
            let (g, u0, u1) = xgcd_euclidean(&j, &r, m)?;
            let mut s_new = mul(&s, &u0, m);
            add_assign(&mut s_new, &mul(&t, &u1, m), m);
            s = s_new;
            break g;
        }
        let step = hgcd(&j, &r, inner_cutoff, true, m, None)?;
        let matrix = step.matrix.unwrap();
        let v = mul(&matrix[1], &t, m);
        let w = mul(&matrix[2], &s, m);
        s = mul(&s, &matrix[3], m);
        t = mul(&t, &matrix[0], m);
        if step.sign > 0 {
            sub_assign(&mut s, &v, m);
            sub_assign(&mut t, &w, m);
        } else {
            sub_from(&mut s, &v, m);
            sub_from(&mut t, &w, m);
        }
        h = step.a;
        j = step.b;
    };
    let mut w = mul(&s, xs, m);
    sub_from(&mut w, &g, m);
    let t = div_exact(&w, ys, m);
    Some((g, s, t))
}
