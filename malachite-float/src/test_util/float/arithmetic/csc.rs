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
use malachite_base::num::arithmetic::traits::{Abs, PowerOf2, Reciprocal};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Down};
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_csc_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut c = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = c.assign_round(x.csc_ref(), rm);
    (c, o)
}

pub fn rug_csc_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_csc_prec_round(x, prec, Round::Nearest)
}

pub fn rug_csc_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_csc_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_csc(x: &rug::Float) -> rug::Float {
    rug_csc_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

// As for the secant oracle, the input carries its denominator's worth of extra bits: the cosecant
// of a fraction close to a short dyadic depends on that closeness.
pub fn rug_csc_rational_prec_round(x: &Rational, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let exponent_bits = if *x == 0u32 {
        0
    } else {
        x.floor_log_base_2_abs().unsigned_abs()
    };
    let denominator_bits = x.denominator_ref().significant_bits();
    let rx = rug::Float::with_val(
        u32::exact_from(prec + 128 + exponent_bits + denominator_bits),
        rug::Rational::exact_from(x),
    );
    rug_csc_prec_round(&rx, prec, rm)
}

pub fn rug_csc_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_csc_rational_prec_round(x, prec, Round::Nearest)
}

// The cosecant in uths of a turn, from a bracket on the sine: `sin_with_period` rounded toward zero
// at a wider precision puts the true sine in [|s|, |s| + ulp), and so the cosecant in the
// reciprocal interval. Both ends are rounded independently, and the result is returned only when
// they agree and neither is exact, which pins the cosecant's rounding. This is an oracle for
// `csc_with_period`, which has no MPFR counterpart; `None` means the bracket did not settle it, and
// the caller skips.
pub fn csc_with_period_naive(
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
    if s == 0u32 || !s.is_finite() {
        return None;
    }
    let negative = s.is_sign_negative();
    let lo = Rational::exact_from(&s).abs();
    let hi = &lo + Rational::power_of_2(i64::from(s.get_exponent().unwrap()) - i64::exact_from(w));
    let (b_lo, b_hi) = if negative {
        (-lo.reciprocal(), -hi.reciprocal())
    } else {
        (hi.reciprocal(), lo.reciprocal())
    };
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

// The cosecant of a `Rational` fraction of a turn, bracketed from the sine exactly as
// `csc_with_period_naive` does for a `Float`. `None` means the bracket did not settle it.
pub fn csc_with_period_rational_naive(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    // the bracket's ends are not representable, so `Exact` has nothing to say here
    if rm == RoundingMode::Exact {
        return None;
    }
    let w = prec + 64;
    let s = Float::sin_with_period_rational_prec_round_ref(x, u, w, Down).0;
    if s == 0u32 || !s.is_finite() {
        return None;
    }
    let negative = s.is_sign_negative();
    let lo = Rational::exact_from(&s).abs();
    let hi = &lo + Rational::power_of_2(i64::from(s.get_exponent().unwrap()) - i64::exact_from(w));
    let (b_lo, b_hi) = if negative {
        (-lo.reciprocal(), -hi.reciprocal())
    } else {
        (hi.reciprocal(), lo.reciprocal())
    };
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
