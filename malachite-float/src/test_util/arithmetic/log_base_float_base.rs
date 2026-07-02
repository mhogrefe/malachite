// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::arithmetic::log_base_rational_rational_base::rational_log_base_rational_rational_base;
use crate::test_util::arithmetic::log_base::simplest_dyadic_in;
use crate::test_util::common::{rounding_mode_from_rug_round, rug_float_significant_bits};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

// Returns `Some(log_base(x))` if the rug `Float`s `x` and `base` (both finite, positive, not 1) are
// commensurable. Independent of the implementation's exact-detection only in spelling; it reuses
// `rational_log_base_rational_rational_base` (itself oracle-checked) with the `log_b = -log_{1/b}`
// reduction for a base below 1. Balloon-safe via the same size bound.
fn rug_log_base_float_base_exact(x: &rug::Float, base: &rug::Float) -> Option<Rational> {
    let mx = Float::from(x);
    let mb = Float::from(base);
    let bound = mx.get_prec()?.max(mb.get_prec()?).saturating_mul(64);
    if i64::from(mx.get_exponent()?).unsigned_abs() > bound
        || i64::from(mb.get_exponent()?).unsigned_abs() > bound
        || mx.significant_bits() > bound
        || mb.significant_bits() > bound
    {
        return None;
    }
    let xr = Rational::exact_from(&mx);
    let br = Rational::exact_from(&mb);
    if br > 1u32 {
        rational_log_base_rational_rational_base(&xr, &br)
    } else {
        rational_log_base_rational_rational_base(&xr, &(Rational::ONE / br)).map(|q| -q)
    }
}

// log_base(x) = ln(x) / ln(base) for `Float` `x` and `base`. For inputs outside the normal domain
// (x or base not finite-positive, or base 1) the result is the IEEE quotient of the natural logs --
// always exact (0, +-infinity, or NaN) -- computed directly with rug. For the normal domain rug
// brackets ln(x) and ln(base) down and up (both native, no Rational conversion) and divides the
// intervals (taking the min/max over the four corners, since a base below 1 makes ln(base)
// negative); the exact commensurable case is short-circuited and a straddled dyadic result is
// recovered past `exact_threshold`.
pub fn rug_log_base_float_base_prec_round(
    x: &rug::Float,
    base: &rug::Float,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    let target_prec = u32::exact_from(prec);
    let normal = x.is_finite() && *x > 0 && base.is_finite() && *base > 0 && *base != 1;
    if !normal {
        // The result is ln(x)/ln(base) under IEEE: 0, +-infinity, or NaN -- exact.
        let wp = max(target_prec, 64);
        let a = rug::Float::with_val(wp, x.ln_ref());
        let b = rug::Float::with_val(wp, base.ln_ref());
        let mut r = rug::Float::with_val(target_prec, 0);
        r.assign_round(&a / &b, rm);
        return (r, Equal);
    }
    // log_base(1) = 0, with the sign of 1 / ln(base).
    if *x == 1 {
        let mut r = rug::Float::with_val(target_prec, 0);
        if *base < 1 {
            r = -r;
        }
        return (r, Equal);
    }
    if let Some(q) = rug_log_base_float_base_exact(x, base) {
        let (l, o) = Float::from_rational_prec_round(q, prec, rounding_mode_from_rug_round(rm));
        return (rug::Float::exact_from(&l), o);
    }
    let mut working_prec = (prec << 1) + 128 + (rug_float_significant_bits(x) << 1);
    let exact_threshold = (prec << 1) + 512;
    loop {
        let wp = u32::exact_from(working_prec);
        // a_lo <= ln(x) <= a_hi
        let mut a_lo = rug::Float::with_val(wp, 0);
        a_lo.assign_round(x.ln_ref(), Round::Down);
        let mut a_hi = rug::Float::with_val(wp, 0);
        a_hi.assign_round(x.ln_ref(), Round::Up);
        // b_lo <= ln(base) <= b_hi (both the same nonzero sign: positive for base > 1, negative for
        // a base in (0, 1))
        let mut b_lo = rug::Float::with_val(wp, 0);
        b_lo.assign_round(base.ln_ref(), Round::Down);
        let mut b_hi = rug::Float::with_val(wp, 0);
        b_hi.assign_round(base.ln_ref(), Round::Up);
        // q_lo <= ln(x) / ln(base) <= q_hi, via the min/max of the four corner quotients (handles
        // either sign of ln(base) and an ln(x) interval that straddles 0).
        let mut q_lo: Option<rug::Float> = None;
        let mut q_hi: Option<rug::Float> = None;
        for a in [&a_lo, &a_hi] {
            for b in [&b_lo, &b_hi] {
                let mut lo = rug::Float::with_val(wp, 0);
                lo.assign_round(a / b, Round::Down);
                let mut hi = rug::Float::with_val(wp, 0);
                hi.assign_round(a / b, Round::Up);
                q_lo = Some(match q_lo {
                    Some(q) => {
                        if lo < q {
                            lo
                        } else {
                            q
                        }
                    }
                    None => lo,
                });
                q_hi = Some(match q_hi {
                    Some(q) => {
                        if hi > q {
                            hi
                        } else {
                            q
                        }
                    }
                    None => hi,
                });
            }
        }
        let q_lo = q_lo.unwrap();
        let q_hi = q_hi.unwrap();
        let mut l_lo = rug::Float::with_val(target_prec, 0);
        let mut o_lo = l_lo.assign_round(&q_lo, rm);
        let mut l_hi = rug::Float::with_val(target_prec, 0);
        let mut o_hi = l_hi.assign_round(&q_hi, rm);
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

pub fn rug_log_base_float_base_prec(
    x: &rug::Float,
    base: &rug::Float,
    prec: u64,
) -> (rug::Float, Ordering) {
    rug_log_base_float_base_prec_round(x, base, prec, Round::Nearest)
}

pub fn rug_log_base_float_base_round(
    x: &rug::Float,
    base: &rug::Float,
    rm: Round,
) -> (rug::Float, Ordering) {
    rug_log_base_float_base_prec_round(x, base, rug_float_significant_bits(x), rm)
}

pub fn rug_log_base_float_base(x: &rug::Float, base: &rug::Float) -> rug::Float {
    rug_log_base_float_base_prec_round(x, base, rug_float_significant_bits(x), Round::Nearest).0
}
