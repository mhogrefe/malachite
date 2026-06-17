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
use malachite_base::num::arithmetic::traits::CheckedLogBase;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::ExpressAsPower;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

// Returns `Some(log_base(x))` if the rug `Float` `x` (finite, positive, not 1) equals `base^m` for a
// rational `m`. This short-circuits the exact case, which the bracketing Ziv loop could never
// resolve when the result is exactly representable (for example `log_3(9) = 2`). It is balloon-safe,
// using the same exponent/size bound as the implementation under test.
fn rug_log_base_rational_base_exact(x: &rug::Float, base: &Rational) -> Option<Rational> {
    let mx = Float::from(x);
    let bound = mx.get_prec()?.saturating_mul(64);
    let e = i64::from(mx.get_exponent()?);
    if e.unsigned_abs() > bound || base.significant_bits() > bound {
        return None;
    }
    let (root, e_base) = base.express_as_power().unwrap_or_else(|| (base.clone(), 1));
    let a = (&Rational::exact_from(&mx)).checked_log_base(&root)?;
    Some(Rational::from_signeds(a, i64::exact_from(e_base)))
}

// log_base(x) = ln(x) / ln(base) for a `Rational` base. rug serves as an independent oracle by
// bracketing: compute ln(x) and ln(base) at a generous working precision rounding down and up,
// divide the brackets, and round both to `prec`. Because `base` is generally not representable as a
// rug `Float`, ln(base) is itself bracketed: `base` is converted (exactly, as a `rug::Rational`) to
// rug `Float` bounds rounding down and up, whose (monotonic) natural logs bound ln(base). The exact
// case x = base^m is detected up front; a dyadic result the brackets straddle is recovered past
// `exact_threshold`.
pub fn rug_log_base_rational_base_prec_round(
    x: &rug::Float,
    base: &Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    if x.is_finite()
        && x.is_sign_positive()
        && *x != 1u32
        && let Some(q) = rug_log_base_rational_base_exact(x, base)
    {
        let (l, o) = Float::from_rational_prec_round(q, prec, rounding_mode_from_rug_round(rm));
        return (rug::Float::exact_from(&l), o);
    }
    let rug_base = rug::Rational::from(base);
    let target_prec = u32::exact_from(prec);
    let mut working_prec =
        (prec << 1) + 128 + (rug_float_significant_bits(x) << 1) + (base.significant_bits() << 1);
    let exact_threshold = (prec << 1) + 512;
    loop {
        let wp = u32::exact_from(working_prec);
        // a_lo <= ln(x) <= a_hi
        let mut a_lo = rug::Float::with_val(wp, 0);
        a_lo.assign_round(x.ln_ref(), Round::Down);
        let mut a_hi = rug::Float::with_val(wp, 0);
        a_hi.assign_round(x.ln_ref(), Round::Up);
        // base_lo <= base <= base_hi, so (monotonic) 0 < b_lo <= ln(base) <= b_hi
        let mut base_lo = rug::Float::with_val(wp, 0);
        base_lo.assign_round(&rug_base, Round::Down);
        let mut base_hi = rug::Float::with_val(wp, 0);
        base_hi.assign_round(&rug_base, Round::Up);
        let mut b_lo = rug::Float::with_val(wp, 0);
        b_lo.assign_round(base_lo.ln_ref(), Round::Down);
        let mut b_hi = rug::Float::with_val(wp, 0);
        b_hi.assign_round(base_hi.ln_ref(), Round::Up);
        // q_lo <= ln(x) / ln(base) <= q_hi
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
            // x <= 0, so the result is NaN.
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

pub fn rug_log_base_rational_base_prec(
    x: &rug::Float,
    base: &Rational,
    prec: u64,
) -> (rug::Float, Ordering) {
    rug_log_base_rational_base_prec_round(x, base, prec, Round::Nearest)
}

pub fn rug_log_base_rational_base_round(
    x: &rug::Float,
    base: &Rational,
    rm: Round,
) -> (rug::Float, Ordering) {
    rug_log_base_rational_base_prec_round(x, base, rug_float_significant_bits(x), rm)
}

pub fn rug_log_base_rational_base(x: &rug::Float, base: &Rational) -> rug::Float {
    rug_log_base_rational_base_prec_round(x, base, rug_float_significant_bits(x), Round::Nearest).0
}
