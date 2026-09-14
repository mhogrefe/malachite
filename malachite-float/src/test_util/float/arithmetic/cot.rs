// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use crate::{ComparableFloat, Float};
use core::cmp::Ordering;
use malachite_base::num::arithmetic::traits::{Abs, PowerOf2};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Down};
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_cot_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = c.assign_round(x.cot_ref(), rm);
    (c, o)
}

pub fn rug_cot_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_cot_prec_round(x, prec, Round::Nearest)
}

pub fn rug_cot_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_cot_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_cot(x: &rug::Float) -> rug::Float {
    rug_cot_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

// As for the secant and cosecant oracles, the input carries its denominator's worth of extra bits:
// the cotangent of a fraction close to a short dyadic depends on that closeness.
pub fn rug_cot_rational_prec_round(x: &Rational, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let exponent_bits = if *x == 0u32 {
        0
    } else {
        u64::try_from(x.floor_log_base_2_abs()).unwrap_or(0)
    };
    let denominator_bits = x.denominator_ref().significant_bits();
    let rx = rug::Float::with_val(
        u32::exact_from(prec + 128 + exponent_bits + denominator_bits),
        rug::Rational::exact_from(x),
    );
    rug_cot_prec_round(&rx, prec, rm)
}

pub fn rug_cot_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_cot_rational_prec_round(x, prec, Round::Nearest)
}

// The cotangent in uths of a turn, from brackets on the sine and the cosine: each rounded toward
// zero at a wider precision lies in [|v|, |v| + ulp), so the quotient lies in the interval formed
// by dividing the ends the other way about. Both ends are rounded independently, and the result is
// returned only when they agree and neither is exact, which pins the cotangent's rounding. This is
// an oracle for `cot_with_period`, which has no MPFR counterpart; `None` means the bracket did not
// settle it, and the caller skips.
pub fn cot_with_period_naive(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    // the bracket's ends are not representable, so `Exact` has nothing to say here
    if rm == RoundingMode::Exact {
        return None;
    }
    let w = prec + 64;
    let s = x.sin_with_period_prec_round_ref(u, w, Down).0;
    let c = x.cos_with_period_prec_round_ref(u, w, Down).0;
    if s == 0u32 || c == 0u32 || !s.is_finite() || !c.is_finite() {
        return None;
    }
    let negative = s.is_sign_negative() != c.is_sign_negative();
    let s_lo = Rational::exact_from(&s).abs();
    let s_hi =
        &s_lo + Rational::power_of_2(i64::from(s.get_exponent().unwrap()) - i64::exact_from(w));
    let c_lo = Rational::exact_from(&c).abs();
    let c_hi =
        &c_lo + Rational::power_of_2(i64::from(c.get_exponent().unwrap()) - i64::exact_from(w));
    let (lo, hi) = (c_lo / s_hi, c_hi / s_lo);
    let (b_lo, b_hi) = if negative { (-hi, -lo) } else { (lo, hi) };
    let (f_lo, o_lo) = Float::from_rational_prec_round_ref(&b_lo, prec, rm);
    let (f_hi, o_hi) = Float::from_rational_prec_round_ref(&b_hi, prec, rm);
    if o_lo == o_hi
        && o_lo != Ordering::Equal
        && ComparableFloat(f_lo) == ComparableFloat(f_hi.clone())
    {
        Some((f_hi, o_hi))
    } else {
        None
    }
}
