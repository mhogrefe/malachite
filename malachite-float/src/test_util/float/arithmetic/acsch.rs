// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::test_util::common::{EXPONENT_GATE, rug_float_significant_bits};
use crate::test_util::float::arithmetic::asinh::rug_asinh_rational_prec_round;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Reciprocal, Sign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_q::Rational;
use rug::float::{Round, Special};
use rug::ops::AssignRound;

// MPFR has no acsch, so this oracle uses MPFR in three ways, on |x| (acsch being odd):
// - For an exponent within `EXPONENT_GATE` it evaluates asinh(1/x), the reciprocal of x being an
//   exact `Rational` (see `rug_asinh_rational_prec_round` for the extra input bits).
// - For a tiny x, where 1/x can overflow MPFR's exponent range, it evaluates ln(1 + sqrt(1 + x^2))
//   - ln|x|, a sum of two positive terms, with 128 extra bits before the final rounding.
// - For a huge x it evaluates asinh(1/x) with 1/x rounded to 128 bits more than both the target
//   precision and x's own precision, or exactly when x is a power of 2.
pub fn rug_acsch_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let p = u32::exact_from(prec);
    if x.is_nan() {
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
    if x.is_zero() {
        let infinity = if x.is_sign_positive() {
            Special::Infinity
        } else {
            Special::NegInfinity
        };
        return (rug::Float::with_val(p, infinity), Equal);
    }
    let exp_x = x.get_exp().unwrap();
    if i64::from(exp_x).abs() <= EXPONENT_GATE {
        return rug_asinh_rational_prec_round(
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
    let x_abs = rug::Float::with_val(x.prec(), x.abs_ref());
    let wp = u32::exact_from(prec + 128) + x.prec();
    let t = if exp_x < 0 {
        let one_plus_x2 =
            rug::Float::with_val(wp, 1u32 + rug::Float::with_val(wp, &x_abs * &x_abs));
        let s = rug::Float::with_val(wp, one_plus_x2.sqrt_ref());
        let ln_x = rug::Float::with_val(wp, x_abs.ln_ref());
        rug::Float::with_val(wp, rug::Float::with_val(wp, s + 1u32).ln() - ln_x)
    } else {
        let y = rug::Float::with_val(wp, x_abs.recip_ref());
        rug::Float::with_val(wp, y.asinh_ref())
    };
    let mut c = rug::Float::with_val(p, 0);
    let o = c.assign_round(&t, rm_abs);
    if negative { (-c, o.reverse()) } else { (c, o) }
}

pub fn rug_acsch_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_acsch_prec_round(x, prec, Round::Nearest)
}

pub fn rug_acsch_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_acsch_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_acsch(x: &rug::Float) -> rug::Float {
    rug_acsch_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

pub fn rug_acsch_rational_prec_round(x: &Rational, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    match x.sign() {
        Equal => (
            rug::Float::with_val(u32::exact_from(prec), Special::Infinity),
            Equal,
        ),
        _ => rug_asinh_rational_prec_round(&x.reciprocal(), prec, rm),
    }
}

pub fn rug_acsch_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_acsch_rational_prec_round(x, prec, Round::Nearest)
}
