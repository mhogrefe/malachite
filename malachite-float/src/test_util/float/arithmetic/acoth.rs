// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::test_util::common::{EXPONENT_GATE, rug_float_significant_bits};
use crate::test_util::float::arithmetic::atanh::rug_atanh_rational_prec_round;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Reciprocal;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_q::Rational;
use rug::float::{Round, Special};
use rug::ops::AssignRound;

// MPFR has no acoth, so this oracle evaluates atanh(1/x) with MPFR. For an exponent within
// `EXPONENT_GATE` the reciprocal of x is passed as an exact `Rational` (see
// `rug_atanh_rational_prec_round` for the extra input bits, which cover the ill-conditioning near
// |x| = 1); for a huge x, 1/x is rounded to 128 bits more than both the target precision and x's
// own precision, and is exact when x is a power of 2.
pub fn rug_acoth_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let p = u32::exact_from(prec);
    if x.is_nan() || x.is_zero() {
        return (rug::Float::with_val(p, Special::Nan), Equal);
    }
    if x.is_infinite() {
        let zero = if x.is_sign_positive() {
            Special::Zero
        } else {
            Special::NegZero
        };
        return (rug::Float::with_val(p, zero), Equal);
    }
    let x_abs = rug::Float::with_val(x.prec(), x.abs_ref());
    if x_abs < 1u32 {
        return (rug::Float::with_val(p, Special::Nan), Equal);
    }
    if x_abs == 1u32 {
        let infinity = if x.is_sign_positive() {
            Special::Infinity
        } else {
            Special::NegInfinity
        };
        return (rug::Float::with_val(p, infinity), Equal);
    }
    let exp_x = x.get_exp().unwrap();
    if i64::from(exp_x) <= EXPONENT_GATE {
        return rug_atanh_rational_prec_round(
            &Rational::exact_from(&Float::from(x)).reciprocal(),
            prec,
            rm,
        );
    }
    let negative = x.is_sign_negative();
    let rm_abs = match (negative, rm) {
        (true, Round::Up) => Round::Down,
        (true, Round::Down) => Round::Up,
        _ => rm,
    };
    let wp = u32::exact_from(prec + 128) + x.prec();
    let y = rug::Float::with_val(wp, x_abs.recip_ref());
    let t = rug::Float::with_val(wp, y.atanh_ref());
    let mut c = rug::Float::with_val(p, 0);
    let o = c.assign_round(&t, rm_abs);
    if negative { (-c, o.reverse()) } else { (c, o) }
}

pub fn rug_acoth_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_acoth_prec_round(x, prec, Round::Nearest)
}

pub fn rug_acoth_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_acoth_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_acoth(x: &rug::Float) -> rug::Float {
    rug_acoth_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

pub fn rug_acoth_rational_prec_round(x: &Rational, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    if *x == 0u32 {
        (
            rug::Float::with_val(u32::exact_from(prec), Special::Nan),
            Equal,
        )
    } else {
        rug_atanh_rational_prec_round(&x.reciprocal(), prec, rm)
    }
}

pub fn rug_acoth_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_acoth_rational_prec_round(x, prec, Round::Nearest)
}
