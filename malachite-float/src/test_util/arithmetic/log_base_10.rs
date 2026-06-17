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
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

// Unlike the general `log_base`, rug (MPFR) exposes a native base-10 logarithm, so the oracle is a
// direct correctly-rounded call rather than a bracketed `ln(x) / ln(10)`.
pub fn rug_log_base_10_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut log_base_10 = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = log_base_10.assign_round(x.log10_ref(), rm);
    (log_base_10, o)
}

pub fn rug_log_base_10_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_log_base_10_prec_round(x, prec, Round::Nearest)
}

pub fn rug_log_base_10_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_log_base_10_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_log_base_10(x: &rug::Float) -> rug::Float {
    rug_log_base_10_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}

// log10(x) for a `Rational` x. rug has a native base-10 logarithm, but a `Rational` is generally
// not representable as a rug `Float`, so log10(x) is bracketed: convert x (exactly, as a
// `rug::Rational`) to rug `Float` bounds rounding down and up, then take their (monotonic) base-10
// logarithms. Sign cases fall out: log10(0) = -inf gives -inf, and log10 of a negative gives NaN.
// The exact case x = 10^m is an integer the brackets straddle; it is recovered past
// `exact_threshold`.
pub fn rug_log_base_10_rational_prec_round(
    x: &Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let rug_x = rug::Rational::from(x);
    let target_prec = u32::exact_from(prec);
    let mut working_prec = (prec << 1) + 128 + (x.significant_bits() << 1);
    let exact_threshold = (prec << 1) + 512;
    loop {
        let wp = u32::exact_from(working_prec);
        // x_lo <= x <= x_hi
        let mut x_lo = rug::Float::with_val(wp, 0);
        x_lo.assign_round(&rug_x, Round::Down);
        let mut x_hi = rug::Float::with_val(wp, 0);
        x_hi.assign_round(&rug_x, Round::Up);
        // log10 is increasing, so a_lo <= log10(x) <= a_hi
        let mut a_lo = rug::Float::with_val(wp, 0);
        a_lo.assign_round(x_lo.log10_ref(), Round::Down);
        let mut a_hi = rug::Float::with_val(wp, 0);
        a_hi.assign_round(x_hi.log10_ref(), Round::Up);
        let mut l_lo = rug::Float::with_val(target_prec, 0);
        let mut o_lo = l_lo.assign_round(&a_lo, rm);
        let mut l_hi = rug::Float::with_val(target_prec, 0);
        let mut o_hi = l_hi.assign_round(&a_hi, rm);
        if l_lo.is_nan() && l_hi.is_nan() {
            // x < 0, so the result is NaN.
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
            let lo = Rational::try_from(&Float::from(&a_lo)).unwrap();
            let hi = Rational::try_from(&Float::from(&a_hi)).unwrap();
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

pub fn rug_log_base_10_rational_prec(x: &Rational, prec: u64) -> (rug::Float, Ordering) {
    rug_log_base_10_rational_prec_round(x, prec, Round::Nearest)
}
