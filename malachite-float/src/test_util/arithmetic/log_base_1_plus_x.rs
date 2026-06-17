// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::test_util::arithmetic::log_base::simplest_dyadic_in;
use crate::test_util::common::{rounding_mode_from_rug_round, rug_float_significant_bits};
use core::cmp::Ordering::{self, *};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

// log_base_1_plus_x(x, base) = ln(1 + x) / ln(base). Same bracketing oracle as
// `rug_log_base_prec_round`, but using rug's `ln_1p` so that ln(1 + x) is accurate when x is near
// 0. `ln_1p` resolves the correctly-rounded log even when 1 + x is astronomically close to a power
// (the down/up brackets are then a single ulp apart, not separated around a minuscule deviation),
// so no precision blowup occurs. The exact case 1 + x = g^m is a dyadic the brackets straddle; it
// is recovered past `exact_threshold`. Sign cases fall out: ln_1p(-1) = -inf gives -inf, and ln_1p
// of a value below -1 gives NaN.
pub fn rug_log_base_1_plus_x_prec_round(
    x: &rug::Float,
    base: u64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let target_prec = u32::exact_from(prec);
    let mut working_prec = (prec << 1) + 128 + (rug_float_significant_bits(x) << 1);
    let exact_threshold = (prec << 1) + 512;
    loop {
        let wp = u32::exact_from(working_prec);
        // a_lo <= ln(1 + x) <= a_hi
        let mut a_lo = rug::Float::with_val(wp, 0);
        a_lo.assign_round(x.ln_1p_ref(), Round::Down);
        let mut a_hi = rug::Float::with_val(wp, 0);
        a_hi.assign_round(x.ln_1p_ref(), Round::Up);
        // 0 < b_lo <= ln(base) <= b_hi
        let base_float = rug::Float::with_val(wp, base);
        let mut b_lo = rug::Float::with_val(wp, 0);
        b_lo.assign_round(base_float.ln_ref(), Round::Down);
        let mut b_hi = rug::Float::with_val(wp, 0);
        b_hi.assign_round(base_float.ln_ref(), Round::Up);
        // q_lo <= ln(1 + x) / ln(base) <= q_hi
        let q_lo_den = if a_lo.is_sign_negative() {
            &b_lo
        } else {
            &b_hi
        };
        let q_hi_den = if a_hi.is_sign_negative() {
            &b_hi
        } else {
            &b_lo
        };
        let mut q_lo = rug::Float::with_val(wp, 0);
        q_lo.assign_round(&a_lo / q_lo_den, Round::Down);
        let mut q_hi = rug::Float::with_val(wp, 0);
        q_hi.assign_round(&a_hi / q_hi_den, Round::Up);
        let mut l_lo = rug::Float::with_val(target_prec, 0);
        let mut o_lo = l_lo.assign_round(&q_lo, rm);
        let mut l_hi = rug::Float::with_val(target_prec, 0);
        let mut o_hi = l_hi.assign_round(&q_hi, rm);
        if l_lo.is_nan() && l_hi.is_nan() {
            // x < -1, so the result is NaN.
            return (l_lo, Equal);
        }
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if l_lo == l_hi && o_lo == o_hi {
            return (l_lo, o_lo);
        }
        if working_prec > exact_threshold {
            let lo = Rational::try_from(&Float::from(&q_lo)).unwrap();
            let hi = Rational::try_from(&Float::from(&q_hi)).unwrap();
            let (l, o) = Float::from_rational_prec_round(
                simplest_dyadic_in(&lo, &hi),
                prec,
                rounding_mode_from_rug_round(rm),
            );
            return (rug::Float::exact_from(&l), o);
        }
        working_prec += working_prec >> 1;
    }
}

pub fn rug_log_base_1_plus_x_prec(x: &rug::Float, base: u64, prec: u64) -> (rug::Float, Ordering) {
    rug_log_base_1_plus_x_prec_round(x, base, prec, Round::Nearest)
}

pub fn rug_log_base_1_plus_x_round(x: &rug::Float, base: u64, rm: Round) -> (rug::Float, Ordering) {
    rug_log_base_1_plus_x_prec_round(x, base, rug_float_significant_bits(x), rm)
}

pub fn rug_log_base_1_plus_x(x: &rug::Float, base: u64) -> rug::Float {
    rug_log_base_1_plus_x_prec_round(x, base, rug_float_significant_bits(x), Round::Nearest).0
}
