// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::test_util::common::rug_float_significant_bits;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use rug::float::Round;
use rug::ops::AssignRound;
use std::cmp::Ordering::{self, *};
use std::cmp::{max, min};

// A naive implementation of mpfr_sum to test against. Each partial sum is computed with enough
// precision to be exact, and the result is rounded only once, at the end. The singular rules (NaN,
// infinities, and the signs of zero results) are the same as mpfr_sum's.
pub fn naive_sum_prec_round(xs: &[Float], prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let mut sign_inf = 0i8;
    let mut pos_zero = false;
    let mut neg_zero = false;
    let mut regulars: Vec<&Float> = Vec::new();
    for x in xs {
        if x.is_nan() {
            return (Float::NAN, Equal);
        } else if x.is_infinite() {
            let s: i8 = if *x > 0u32 { 1 } else { -1 };
            if sign_inf == 0 {
                sign_inf = s;
            } else if sign_inf != s {
                return (Float::NAN, Equal);
            }
        } else if *x == 0u32 {
            if x.is_negative_zero() {
                neg_zero = true;
            } else {
                pos_zero = true;
            }
        } else {
            regulars.push(x);
        }
    }
    if sign_inf != 0 {
        return (
            if sign_inf > 0 {
                Float::INFINITY
            } else {
                Float::NEGATIVE_INFINITY
            },
            Equal,
        );
    }
    if regulars.is_empty() {
        // All inputs are zeros, or there are no inputs (in which case the sum is +0). If all the
        // zeros have the same sign, the sum has that sign; otherwise the sign is determined by the
        // rounding mode.
        return (
            if neg_zero && (!pos_zero || rm == Floor) {
                Float::NEGATIVE_ZERO
            } else {
                Float::ZERO
            },
            Equal,
        );
    }
    let mut acc = regulars[0].clone();
    for &x in &regulars[1..] {
        if acc == 0u32 {
            // exact intermediate cancellation; the sum so far is just x
            acc = x.clone();
            continue;
        }
        let ea = i64::from(acc.get_exponent().unwrap());
        let ex = i64::from(x.get_exponent().unwrap());
        let pa = i64::exact_from(acc.get_prec().unwrap());
        let px = i64::exact_from(x.get_prec().unwrap());
        // All the bits of the exact sum lie in [min(ea - pa, ex - px), max(ea, ex) + 1), so this
        // precision makes the addition exact.
        let p = u64::exact_from(max(ea, ex) + 1 - min(ea - pa, ex - px));
        let (s, o) = acc.add_prec_round_ref_ref(x, p, Exact);
        assert_eq!(o, Equal);
        acc = s;
    }
    if acc == 0u32 {
        // Nonzero inputs summing to exactly zero give +0, except under Floor.
        (
            if rm == Floor {
                Float::NEGATIVE_ZERO
            } else {
                Float::ZERO
            },
            Equal,
        )
    } else {
        Float::from_float_prec_round(acc, prec, rm)
    }
}

#[inline]
pub fn naive_sum_prec(xs: &[Float], prec: u64) -> (Float, Ordering) {
    naive_sum_prec_round(xs, prec, Nearest)
}

fn naive_max_prec(xs: &[Float]) -> u64 {
    xs.iter()
        .map(SignificantBits::significant_bits)
        .max()
        .unwrap_or(1)
}

#[inline]
pub fn naive_sum_round(xs: &[Float], rm: RoundingMode) -> (Float, Ordering) {
    naive_sum_prec_round(xs, naive_max_prec(xs), rm)
}

#[inline]
pub fn naive_sum(xs: &[Float]) -> Float {
    naive_sum_prec_round(xs, naive_max_prec(xs), Nearest).0
}

pub fn rug_sum_prec_round(xs: &[rug::Float], prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut sum = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = sum.assign_round(rug::Float::sum(xs.iter()), rm);
    (sum, o)
}

#[inline]
pub fn rug_sum_prec(xs: &[rug::Float], prec: u64) -> (rug::Float, Ordering) {
    rug_sum_prec_round(xs, prec, Round::Nearest)
}

fn rug_max_prec(xs: &[rug::Float]) -> u64 {
    xs.iter().map(rug_float_significant_bits).max().unwrap_or(1)
}

#[inline]
pub fn rug_sum_round(xs: &[rug::Float], rm: Round) -> (rug::Float, Ordering) {
    rug_sum_prec_round(xs, rug_max_prec(xs), rm)
}

pub fn rug_sum(xs: &[rug::Float]) -> rug::Float {
    rug_sum_prec_round(xs, rug_max_prec(xs), Round::Nearest).0
}
