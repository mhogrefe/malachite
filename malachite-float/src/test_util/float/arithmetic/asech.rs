// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::test_util::common::rug_float_significant_bits;
use crate::test_util::float::arithmetic::acosh::rug_acosh_rational_prec_round;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Reciprocal, Sign};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_q::Rational;
use rug::float::{Round, Special};
use rug::ops::AssignRound;

// MPFR has no asech, so this oracle uses MPFR in two ways. For 1/2 <= x <= 1 it evaluates
// acosh(1/x), the reciprocal of x being an exact `Rational` (see `rug_acosh_rational_prec_round`
// for the extra input bits, which cover the ill-conditioning near x = 1). For 0 < x < 1/2, where
// 1/x can overflow MPFR's exponent range but asech is well-conditioned, it evaluates ln(1 + sqrt(1
// - x^2)) - ln(x), a sum of two positive terms, with 128 extra bits before the final rounding.
pub fn rug_asech_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let p = u32::exact_from(prec);
    if x.is_nan() || x.is_infinite() || (x.is_sign_negative() && !x.is_zero()) || *x > 1u32 {
        return (rug::Float::with_val(p, Special::Nan), Equal);
    }
    if x.is_zero() {
        return (rug::Float::with_val(p, Special::Infinity), Equal);
    }
    if x.get_exp().unwrap() >= 0 {
        return rug_acosh_rational_prec_round(
            &Rational::exact_from(&Float::from(x)).reciprocal(),
            prec,
            rm,
        );
    }
    let wp = u32::exact_from(prec + 128);
    let one_minus_x2 = rug::Float::with_val(wp, 1u32 - rug::Float::with_val(wp, x * x));
    let s = rug::Float::with_val(wp, one_minus_x2.sqrt_ref());
    let ln_x = rug::Float::with_val(wp, x.ln_ref());
    let t = rug::Float::with_val(wp, rug::Float::with_val(wp, s + 1u32).ln() - ln_x);
    let mut c = rug::Float::with_val(p, 0);
    let o = c.assign_round(&t, rm);
    (c, o)
}

pub fn rug_asech_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_asech_prec_round(x, prec, Round::Nearest)
}

pub fn rug_asech_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_asech_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_asech(x: &rug::Float) -> rug::Float {
    rug_asech_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

pub fn rug_asech_rational_prec_round(x: &Rational, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let p = u32::exact_from(prec);
    match x.sign() {
        Less => (rug::Float::with_val(p, Special::Nan), Equal),
        Equal => (rug::Float::with_val(p, Special::Infinity), Equal),
        Greater => rug_acosh_rational_prec_round(&x.reciprocal(), prec, rm),
    }
}

pub fn rug_asech_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_asech_rational_prec_round(x, prec, Round::Nearest)
}
