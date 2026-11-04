// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2007, 2008 William Hart
//
//      Copyright © 2011 Sebastian Pancratz
//
//      Copyright © 2023 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Parity;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_polynomial::modular::hgcd::{div_rem, hgcd, pow};
use crate::unsigned_polynomial::modular::mod_gcd::GcdModulus;
use alloc::vec::Vec;

// Whether the product of the lengths' contributions to the resultant's sign is negative: when both
// lengths are even.
fn both_even(len_a: usize, len_b: usize) -> bool {
    (len_a | len_b).even()
}

// The resultant of the polynomials with coefficients `xs` and `ys`, where `xs` is at least as long
// as `ys` and neither is empty or ends with a zero, by a closed formula when `ys` is short, or
// `None` if there is none.
//
// This is equivalent to `_gr_poly_resultant_small` from `gr_poly/resultant_small.c`, FLINT 3.6.0.
fn resultant_small<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Option<C> {
    match (xs.len(), ys.len()) {
        (1, 1) => Some(m.one()),
        (2, 1) => Some(ys[0].clone()),
        (len1, 1) => Some(pow(&ys[0], u64::exact_from(len1 - 1), m)),
        (2, 2) => Some(m.sub(&m.mul(&xs[1], &ys[0]), &m.mul(&xs[0], &ys[1]))),
        (3, 2) => {
            let t = m.sub(&m.mul(&xs[0], &ys[1]), &m.mul(&xs[1], &ys[0]));
            let t = m.mul(&t, &ys[1]);
            let u = m.mul(&m.mul(&ys[0], &ys[0]), &xs[2]);
            Some(m.add(&t, &u))
        }
        (len1, 2) => {
            // The powers of b0, scaled by the complementary powers of -b1, dotted with `xs`.
            let mut t = Vec::with_capacity(len1);
            t.push(m.one());
            for i in 1..len1 {
                t.push(m.mul(&t[i - 1], &ys[0]));
            }
            let mut u = m.neg(&ys[1]);
            let v = u.clone();
            for i in 1..len1 {
                t[len1 - i - 1] = m.mul(&t[len1 - i - 1], &u);
                if i < len1 - 1 {
                    u = m.mul(&u, &v);
                }
            }
            let mut res = m.zero();
            for (x, y) in xs.iter().zip(&t) {
                res = m.add(&res, &m.mul(x, y));
            }
            Some(res)
        }
        (3, 3) => {
            let t0 = m.mul(&xs[0], &xs[2]);
            let t1 = m.mul(&ys[0], &ys[2]);
            let t2 = m.mul(&xs[0], &ys[2]);
            let t3 = m.mul(&xs[2], &ys[0]);
            let t7 = m.mul(&xs[1], &ys[1]);
            let t4 = m.sub(&m.mul(&ys[1], &ys[1]), &m.add(&t1, &t1));
            let t5 = m.mul(&xs[1], &xs[1]);
            let t6 = m.sub(&t2, &t7);
            let t7 = m.sub(&t3, &t7);
            let mut res = m.mul(&t0, &t4);
            res = m.add(&res, &m.mul(&t1, &t5));
            res = m.add(&res, &m.mul(&t2, &t6));
            Some(m.add(&res, &m.mul(&t3, &t7)))
        }
        _ => None,
    }
}

/// The resultant of the polynomials with coefficients `xs` and `ys`, where `xs` is at least as long
/// as `ys` and neither is empty or ends with a zero, by Euclid's algorithm; or `None` if a leading
/// coefficient met along the way has no inverse.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compute resultants
/// of `NaturalPolynomial`s.
///
/// This is equivalent to `_gr_poly_resultant_euclidean` from `gr_poly/resultant_euclidean.c`, FLINT
/// 3.6.0.
#[doc(hidden)]
pub fn resultant_euclidean<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Option<C> {
    if ys.len() == 1 {
        return resultant_small(xs, ys, m);
    }
    let mut res = m.one();
    let mut u = xs.to_vec();
    let mut v = ys.to_vec();
    loop {
        let l0 = u.len();
        let l1 = v.len();
        let lc = v.last().unwrap().clone();
        let r = div_rem(&u, &v, m)?.1;
        let l2 = r.len();
        u = core::mem::replace(&mut v, r);
        if l2 == 0 {
            return Some(if l1 == 1 {
                m.mul(&res, &pow(&lc, u64::exact_from(l0 - 1), m))
            } else {
                m.zero()
            });
        }
        res = m.mul(&res, &pow(&lc, u64::exact_from(l0 - l2), m));
        if both_even(l0, l1) {
            res = m.neg(&res);
        }
    }
}

/// The resultant of the polynomials with coefficients `xs` and `ys`, where `xs` is at least as long
/// as `ys` and neither is empty or ends with a zero, by the half-GCD algorithm down to `cutoff`
/// coefficients and Euclid's algorithm below that, with `inner_cutoff` passed on to the half-GCD;
/// or `None` if a leading coefficient met along the way has no inverse.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compute resultants
/// of `NaturalPolynomial`s.
///
/// This is equivalent to `_gr_poly_resultant_hgcd` from `gr_poly/resultant_hgcd.c`, FLINT 3.6.0.
#[doc(hidden)]
pub fn resultant_hgcd<C: Clone, M: GcdModulus<C>>(
    xs: &[C],
    ys: &[C],
    inner_cutoff: usize,
    cutoff: usize,
    m: &M,
) -> Option<C> {
    let mut res = m.one();
    let lc = ys.last().unwrap().clone();
    let r = div_rem(xs, ys, m)?.1;
    if r.is_empty() {
        return Some(if ys.len() == 1 {
            pow(&lc, u64::exact_from(xs.len() - 1), m)
        } else {
            m.zero()
        });
    }
    res = m.mul(&res, &pow(&lc, u64::exact_from(xs.len() - r.len()), m));
    if both_even(xs.len(), ys.len()) {
        res = m.neg(&res);
    }
    let step = hgcd(ys, &r, inner_cutoff, false, m, Some(&mut res))?;
    let mut g = step.a;
    let mut j = step.b;
    while !j.is_empty() {
        let lc = j.last().unwrap().clone();
        let r = div_rem(&g, &j, m)?.1;
        if r.is_empty() {
            return Some(if j.len() == 1 {
                m.mul(&res, &pow(&lc, u64::exact_from(g.len() - 1), m))
            } else {
                m.zero()
            });
        }
        res = m.mul(&res, &pow(&lc, u64::exact_from(g.len() - r.len()), m));
        if both_even(g.len(), j.len()) {
            res = m.neg(&res);
        }
        if j.len() < cutoff {
            return Some(m.mul(&res, &resultant_euclidean(&j, &r, m)?));
        }
        let step = hgcd(&j, &r, inner_cutoff, false, m, Some(&mut res))?;
        g = step.a;
        j = step.b;
    }
    Some(res)
}

/// The resultant of the polynomials with coefficients `xs` and `ys`, where `xs` is at least as long
/// as `ys` and neither is empty or ends with a zero; or `None` if a leading coefficient met along
/// the way has no inverse, which never happens when the modulus is prime.
///
/// This is not part of the public API; it is public so that `malachite-nz` can compute resultants
/// of `IntegerPolynomial`s.
///
/// This is equivalent to `_nmod_poly_resultant` from `nmod_poly/resultant.c`, FLINT 3.6.0.
#[doc(hidden)]
pub fn checked_resultant<C: Clone, M: GcdModulus<C>>(xs: &[C], ys: &[C], m: &M) -> Option<C> {
    if let Some(res) = resultant_small(xs, ys, m) {
        return Some(res);
    }
    let cutoffs = m.gcd_cutoffs();
    if ys.len() < cutoffs.gcd_hgcd {
        resultant_euclidean(xs, ys, m)
    } else {
        resultant_hgcd(xs, ys, cutoffs.hgcd_iter, cutoffs.hgcd_outer, m)
    }
}
