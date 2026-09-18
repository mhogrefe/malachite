// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use core::cmp::Ordering;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

// MPFR has no arccotangent. For |x| >= 1 the oracle is its arctangent of the reciprocal, taken with
// the input's precision to spare: that reciprocal lies between 1 and twice the smallest positive
// `Float`, so it neither overflows nor underflows, and the arctangent amplifies nothing. Below 1
// the reciprocal of a tiny x would overflow, so the oracle is pi/2 minus the arctangent of |x|
// instead, with the sign of x, everything carrying 128 bits to spare.
pub fn rug_acot_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let w = u32::exact_from(prec + 128 + rug_float_significant_bits(x));
    let mut a = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = if x.is_nan() || x.clone().abs() >= 1u32 {
        let mut t = rug::Float::with_val(w, 0);
        t.assign_round(x.recip_ref(), Round::Nearest);
        a.assign_round(t.atan_ref(), rm)
    } else {
        let t = rug::Float::with_val(w, x.abs_ref());
        let at = rug::Float::with_val(w, t.atan_ref());
        let half_pi = rug::Float::with_val(w, rug::float::Constant::Pi) / 2u32;
        let mut d = rug::Float::with_val(w, &half_pi - &at);
        if x.is_sign_negative() {
            d = -d;
        }
        a.assign_round(&d, rm)
    };
    (a, o)
}

pub fn rug_acot_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_acot_prec_round(x, prec, Round::Nearest)
}

pub fn rug_acot(x: &rug::Float) -> rug::Float {
    rug_acot_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

// As for the other `Rational` oracles, the input carries its denominator's worth of extra bits, and
// twice its exponent's; `rug_acot_prec_round` then widens again for the reciprocal.
pub fn rug_acot_rational_prec_round(x: &Rational, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let exponent_bits = if *x == 0u32 {
        0
    } else {
        x.floor_log_base_2_abs().unsigned_abs()
    };
    let denominator_bits = x.denominator_ref().significant_bits();
    let rx = rug::Float::with_val(
        u32::exact_from(prec + 128 + (exponent_bits << 1) + denominator_bits),
        rug::Rational::exact_from(x),
    );
    rug_acot_prec_round(&rx, prec, rm)
}

pub fn rug_acot_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_acot_rational_prec_round(x, prec, Round::Nearest)
}

// As for the plain arccotangent, with a period: the arctangent of the reciprocal for |x| >= 1, and
// u/4 minus the arctangent of |x| below that, with the sign of x. Here the side matters, u/4 being
// exactly representable: a tiny x puts the true value a hair below it. So the arctangent is rounded
// away from zero, which keeps it from underflowing to nothing, and the subtraction toward zero,
// which keeps the difference strictly below u/4 -- in the same rounding cell as the true value,
// since 128 spare bits separate that cell's edges from u/4.
pub fn rug_acot_with_period_prec_round(
    x: &rug::Float,
    u: u64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let w = u32::exact_from(prec + 128 + rug_float_significant_bits(x));
    let u = u32::exact_from(u);
    let mut a = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = if x.is_nan() || x.clone().abs() >= 1u32 {
        let mut t = rug::Float::with_val(w, 0);
        t.assign_round(x.recip_ref(), Round::Nearest);
        a.assign_round(t.atan_u_ref(u), rm)
    } else {
        let t = rug::Float::with_val(w, x.abs_ref());
        let mut at = rug::Float::with_val(w, 0);
        at.assign_round(t.atan_u_ref(u), Round::Up);
        let quarter = rug::Float::with_val(w, u) / 4u32;
        let mut d = rug::Float::with_val(w, 0);
        d.assign_round(&quarter - &at, Round::Zero);
        if x.is_sign_negative() {
            d = -d;
        }
        a.assign_round(&d, rm)
    };
    (a, o)
}

pub fn rug_acot_with_period_prec(x: &rug::Float, u: u64, prec: u64) -> (rug::Float, Ordering) {
    rug_acot_with_period_prec_round(x, u, prec, Round::Nearest)
}

// The `Rational` widening of `rug_acot_rational_prec_round`, with a period.
pub fn rug_acot_with_period_rational_prec_round(
    x: &Rational,
    u: u64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let exponent_bits = if *x == 0u32 {
        0
    } else {
        x.floor_log_base_2_abs().unsigned_abs()
    };
    let denominator_bits = x.denominator_ref().significant_bits();
    let rx = rug::Float::with_val(
        u32::exact_from(prec + 128 + (exponent_bits << 1) + denominator_bits),
        rug::Rational::exact_from(x),
    );
    rug_acot_with_period_prec_round(&rx, u, prec, rm)
}

pub fn rug_acot_with_period_rational_prec(
    x: &Rational,
    u: u64,
    prec: u64,
) -> (rug::Float, Ordering) {
    rug_acot_with_period_rational_prec_round(x, u, prec, Round::Nearest)
}
