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
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

// log_{2^pow}(x) = log_2(x) / pow. rug serves as an independent oracle by bracketing: compute
// log_2(x) at a generous working precision rounding down and up, divide each bracket by `pow`, and
// round both to `prec`. The true result lies between the two chains, so when both agree on the
// value and the ternary the answer is correctly rounded. Bracketing (rather than a single
// high-precision evaluation) is necessary because, when `x` is astronomically close to a power of
// 2, a fixed-precision `log_2(x)` rounds to an exact integer and would wrongly report the division
// as exact; the brackets detect the inexactness instead. The working precision is scaled by `x`'s
// significant bits so the brackets straddle that near-power-of-2 deviation. When `pow < 0` the
// division reverses the sense of "down" and "up", so the lower and upper log_2 brackets are
// swapped.
pub fn rug_log_base_power_of_2_prec_round(
    x: &rug::Float,
    pow: i64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let target_prec = u32::exact_from(prec);
    let mut working_prec = (prec << 1) + 128 + (rug_float_significant_bits(x) << 1);
    loop {
        let wp = u32::exact_from(working_prec);
        // t_lo <= log_2(x) <= t_hi
        let mut t_lo = rug::Float::with_val(wp, 0);
        t_lo.assign_round(x.log2_ref(), Round::Down);
        let mut t_hi = rug::Float::with_val(wp, 0);
        t_hi.assign_round(x.log2_ref(), Round::Up);
        let pow_float = rug::Float::with_val(wp, pow);
        // q_lo <= log_2(x) / pow <= q_hi
        let (num_lo, num_hi) = if pow > 0 {
            (&t_lo, &t_hi)
        } else {
            (&t_hi, &t_lo)
        };
        let mut q_lo = rug::Float::with_val(wp, 0);
        q_lo.assign_round(num_lo / &pow_float, Round::Down);
        let mut q_hi = rug::Float::with_val(wp, 0);
        q_hi.assign_round(num_hi / &pow_float, Round::Up);
        let mut l_lo = rug::Float::with_val(target_prec, 0);
        let mut o_lo = l_lo.assign_round(&q_lo, rm);
        let mut l_hi = rug::Float::with_val(target_prec, 0);
        let mut o_hi = l_hi.assign_round(&q_hi, rm);
        if l_lo.is_nan() && l_hi.is_nan() {
            // x is negative, so the result is NaN. (NaN != NaN, so the equality test below would
            // never succeed.)
            return (l_lo, Equal);
        }
        // If one endpoint's rounding was exact but the other's wasn't, the result is irrational and
        // strictly between the chains, so the exact endpoint adopts the other's ternary.
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

pub fn rug_log_base_power_of_2_prec(x: &rug::Float, pow: i64, prec: u64) -> (rug::Float, Ordering) {
    rug_log_base_power_of_2_prec_round(x, pow, prec, Round::Nearest)
}

pub fn rug_log_base_power_of_2_round(
    x: &rug::Float,
    pow: i64,
    rm: Round,
) -> (rug::Float, Ordering) {
    rug_log_base_power_of_2_prec_round(x, pow, rug_float_significant_bits(x), rm)
}

pub fn rug_log_base_power_of_2(x: &rug::Float, pow: i64) -> rug::Float {
    rug_log_base_power_of_2_prec_round(x, pow, rug_float_significant_bits(x), Round::Nearest).0
}

// log_{2^pow}(x) = log_2(x) / pow, where x is a `Rational`. rug serves as an independent oracle by
// bracketing: bracket log_2(x), divide each bracket by `pow`, and round both to `prec`. The true
// result lies between the two chains, so when both agree on the value and the ternary the answer is
// correctly rounded. Bracketing (rather than a single high-precision evaluation) is essential
// because, when `x` is astronomically close to a power of 2, a fixed-precision `log_2(x)` rounds to
// an exact integer and would wrongly report the division as exact; the brackets detect the
// inexactness instead.
//
// Since `Rational`s may have magnitudes outside the exponent range of `rug::Float` (MPFR's default
// exponent range is smaller than `Float`'s), a positive x is first written as x = 2^k * x' with x'
// in [1, 2), using log_2(x) = k + log_2(x'). The lower bracketing chain rounds every step down and
// the upper chain rounds every step up, so the true value of log_2(x) always lies between the two
// chains. When `pow < 0` the division reverses the sense of "down" and "up", so the lower and upper
// log_2 brackets are swapped before dividing by `pow`. The working precision is scaled by `x`'s
// significant bits so the brackets straddle any near-power-of-2 deviation.
pub fn rug_log_base_power_of_2_rational_prec_round(
    x: &Rational,
    pow: i64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let target_prec = u32::exact_from(prec);
    let (k, shifted) = if *x > 0 {
        let k = x.floor_log_base_2_abs();
        (k, x >> k)
    } else {
        (0, x.clone())
    };
    let rug_x = rug::Rational::from(&shifted);
    let mut working_prec = (prec << 1) + 128 + ((&shifted).significant_bits() << 1);
    loop {
        let wp = u32::exact_from(working_prec);
        let mut lo = rug::Float::with_val(wp, 0);
        lo.assign_round(&rug_x, Round::Down);
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
        let pow_float = rug::Float::with_val(wp, pow);
        // q_lo <= log_2(x) / pow <= q_hi
        let (num_lo, num_hi) = if pow > 0 {
            (&s_lo, &s_hi)
        } else {
            (&s_hi, &s_lo)
        };
        let mut q_lo = rug::Float::with_val(wp, 0);
        q_lo.assign_round(num_lo / &pow_float, Round::Down);
        let mut q_hi = rug::Float::with_val(wp, 0);
        q_hi.assign_round(num_hi / &pow_float, Round::Up);
        let mut l_lo = rug::Float::with_val(target_prec, 0);
        let mut o_lo = l_lo.assign_round(&q_lo, rm);
        let mut l_hi = rug::Float::with_val(target_prec, 0);
        let mut o_hi = l_hi.assign_round(&q_hi, rm);
        if l_lo.is_nan() && l_hi.is_nan() {
            // x is negative, so the result is NaN. (NaN != NaN, so the equality test below would
            // never succeed.)
            return (l_lo, Equal);
        }
        // If one endpoint's rounding was exact but the other's wasn't, the result is irrational and
        // strictly between the chains, so the exact endpoint adopts the other's ternary.
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

pub fn rug_log_base_power_of_2_rational_prec(
    x: &Rational,
    pow: i64,
    prec: u64,
) -> (rug::Float, Ordering) {
    rug_log_base_power_of_2_rational_prec_round(x, pow, prec, Round::Nearest)
}
