// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use core::cmp::Ordering::{self, *};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_log_base_2_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut log_base_2 = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = log_base_2.assign_round(x.log2_ref(), rm);
    (log_base_2, o)
}

pub fn rug_log_base_2_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_log_base_2_prec_round(x, prec, Round::Nearest)
}

pub fn rug_log_base_2_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_log_base_2_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_log_base_2(x: &rug::Float) -> rug::Float {
    rug_log_base_2_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

// There is no rug (MPFR) function that computes the base-2 logarithm of a rational number directly,
// but rug can still serve as an independent oracle by bracketing: round the rational down and up to
// a `rug::Float` at a generous working precision, take the base-2 logarithm of both endpoints, and
// accept the result if both brackets agree on the value and the ternary. Otherwise, increase the
// working precision and retry.
//
// Since `Rational`s may have magnitudes outside the exponent range of `rug::Float` (MPFR's default
// exponent range is smaller than `Float`'s), a positive x is first written as x = 2^k * x' with x'
// in [1, 2), using log_2(x) = k + log_2(x'). The lower bracketing chain rounds every step down and
// the upper chain rounds every step up, so the true result always lies between the two chains'
// values.
pub fn rug_log_base_2_rational_prec_round(
    x: &Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let (k, shifted) = if *x > 0 {
        let k = x.floor_log_base_2_abs();
        (k, x >> k)
    } else {
        (0, x.clone())
    };
    let rug_x = rug::Rational::from(&shifted);
    let mut working_prec = (prec << 1) + 64;
    loop {
        let wp = u32::exact_from(working_prec);
        let mut lo = rug::Float::with_val(wp, 0);
        let conversion_o = lo.assign_round(&rug_x, Round::Down);
        if k == 0 && conversion_o == Equal {
            // The conversion was exact, so MPFR's log2 gives the correctly-rounded result directly.
            let mut l_lo = rug::Float::with_val(u32::exact_from(prec), 0);
            let o_lo = l_lo.assign_round(lo.log2_ref(), rm);
            return (l_lo, o_lo);
        }
        let mut hi = rug::Float::with_val(wp, 0);
        hi.assign_round(&rug_x, Round::Up);
        // t_lo <= log_2(x') <= t_hi
        let mut t_lo = rug::Float::with_val(wp, 0);
        t_lo.assign_round(lo.log2_ref(), Round::Down);
        let mut t_hi = rug::Float::with_val(wp, 0);
        t_hi.assign_round(hi.log2_ref(), Round::Up);
        // s_lo <= k + log_2(x') = log_2(x) <= s_hi
        let mut s_lo = rug::Float::with_val(wp, 0);
        s_lo.assign_round(&t_lo + k, Round::Down);
        let mut s_hi = rug::Float::with_val(wp, 0);
        s_hi.assign_round(&t_hi + k, Round::Up);
        let mut l_lo = rug::Float::with_val(u32::exact_from(prec), 0);
        let mut o_lo = l_lo.assign_round(&s_lo, rm);
        let mut l_hi = rug::Float::with_val(u32::exact_from(prec), 0);
        let mut o_hi = l_hi.assign_round(&s_hi, rm);
        if l_lo.is_nan() && l_hi.is_nan() {
            // x is negative, so the result is NaN. (NaN != NaN, so the equality test below would
            // never succeed.)
            return (l_lo, Equal);
        }
        // If an endpoint's rounding was exact but the other endpoint's wasn't, log_2(x) is
        // irrational and strictly between the chains, so the exact endpoint adopts the other
        // endpoint's ternary.
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if l_lo == l_hi && o_lo == o_hi {
            return (l_lo, o_lo);
        }
        working_prec += working_prec >> 1;
    }
}

pub fn rug_log_base_2_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_log_base_2_rational_prec_round(x, prec, Round::Nearest)
}
