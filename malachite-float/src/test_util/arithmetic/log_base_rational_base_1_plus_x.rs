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
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::factorization::traits::ExpressAsPower;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

// Returns `Some(m / e_base)` -- the value of `log_base(1 + x)` -- when the rug `Float` `x` is
// finite, in (-1, 0) or positive, and `1 + x = g^m` for the primitive root `g` of `base`. This
// short-circuits the exact case (which the bracketing Ziv loop could never resolve when the result
// is exactly representable), and is balloon-safe via the same size bound as the implementation
// under test. `x = 0` (sign-preserving zero) and `x <= -1` are left to the bracketing path.
fn rug_log_base_rational_base_1_plus_x_exact(x: &rug::Float, base: &Rational) -> Option<Rational> {
    if !x.is_finite() {
        return None;
    }
    let mx = Float::from(x);
    if mx == 0u32 || mx <= -1i32 {
        return None;
    }
    let bound = mx.get_prec()?.saturating_mul(64);
    let e = i64::from(mx.get_exponent()?);
    if e.unsigned_abs() > bound || base.significant_bits() > bound {
        return None;
    }
    let (root, e_base) = base.express_as_power().unwrap_or_else(|| (base.clone(), 1));
    let m = (Rational::exact_from(&mx) + Rational::ONE).checked_log_base(&root)?;
    Some(Rational::from_signeds(m, i64::exact_from(e_base)))
}

// log_base(1 + x) = ln(1 + x) / ln(base) for a `Rational` base. rug serves as an independent oracle
// by bracketing: ln(1 + x) is computed via rug's native `ln_1p` (accurate for x near 0) rounding
// down and up, and ln(base) -- since `base` is generally not representable as a rug `Float` -- by
// bracketing `base` to rug `Float` bounds rounding down and up, then taking their (monotonic) logs.
// The brackets are divided and both rounded to `prec`. The exact case 1 + x = g^m is detected up
// front; a dyadic result the brackets straddle is recovered past `exact_threshold`. Sign cases fall
// out of `ln_1p`: ln_1p(-1) = -inf gives -inf, ln_1p of a value below -1 gives NaN, and ln_1p
// preserves the sign of a zero.
pub fn rug_log_base_rational_base_1_plus_x_prec_round(
    x: &rug::Float,
    base: &Rational,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    if let Some(q) = rug_log_base_rational_base_1_plus_x_exact(x, base) {
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
        // a_lo <= ln(1 + x) <= a_hi
        let mut a_lo = rug::Float::with_val(wp, 0);
        a_lo.assign_round(x.ln_1p_ref(), Round::Down);
        let mut a_hi = rug::Float::with_val(wp, 0);
        a_hi.assign_round(x.ln_1p_ref(), Round::Up);
        // base_lo <= base <= base_hi, so (monotonic) 0 < b_lo <= ln(base) <= b_hi
        let mut base_lo = rug::Float::with_val(wp, 0);
        base_lo.assign_round(&rug_base, Round::Down);
        let mut base_hi = rug::Float::with_val(wp, 0);
        base_hi.assign_round(&rug_base, Round::Up);
        let mut b_lo = rug::Float::with_val(wp, 0);
        b_lo.assign_round(base_lo.ln_ref(), Round::Down);
        let mut b_hi = rug::Float::with_val(wp, 0);
        b_hi.assign_round(base_hi.ln_ref(), Round::Up);
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

pub fn rug_log_base_rational_base_1_plus_x_prec(
    x: &rug::Float,
    base: &Rational,
    prec: u64,
) -> (rug::Float, Ordering) {
    rug_log_base_rational_base_1_plus_x_prec_round(x, base, prec, Round::Nearest)
}

pub fn rug_log_base_rational_base_1_plus_x_round(
    x: &rug::Float,
    base: &Rational,
    rm: Round,
) -> (rug::Float, Ordering) {
    rug_log_base_rational_base_1_plus_x_prec_round(x, base, rug_float_significant_bits(x), rm)
}

pub fn rug_log_base_rational_base_1_plus_x(x: &rug::Float, base: &Rational) -> rug::Float {
    rug_log_base_rational_base_1_plus_x_prec_round(
        x,
        base,
        rug_float_significant_bits(x),
        Round::Nearest,
    )
    .0
}
