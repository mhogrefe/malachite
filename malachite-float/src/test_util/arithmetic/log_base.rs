// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::test_util::common::{rounding_mode_from_rug_round, rug_float_significant_bits};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{CheckedLogBase, Floor};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use rug::float::Round;
use rug::ops::AssignRound;

// The simplest-denominator dyadic rational strictly inside the open interval `(lo, hi)` (which is
// assumed nonempty). Used to recover an exactly-representable result that the bracketing straddles;
// such results are always dyadic (a non-dyadic rational like log_8(4) = 2/3 is strictly between
// `Float`s and so the brackets converge normally).
fn simplest_dyadic_in(lo: &Rational, hi: &Rational) -> Rational {
    let mut k = 0u64;
    loop {
        let m = (lo << k).floor() + Integer::from(1u32); // smallest integer m with m / 2^k > lo
        let candidate = Rational::from(m) >> k;
        if candidate < *hi {
            return candidate;
        }
        k += 1;
    }
}

// Returns `Some(n)` if the rug `Float` `x` (finite and positive, and not 1) equals `base^n` for some
// integer `n >= 1`. This short-circuits the exact case, which the bracketing Ziv loop could never
// resolve (`log_base(base^n) = n` is an exactly-representable integer). It is balloon-safe: `x` is
// materialized as an integer only when its exponent is within `64 * prec` of being a representable
// power, the same bound used by the implementation under test.
fn rug_log_base_exact(x: &rug::Float, base: u64) -> Option<u64> {
    // Cheap rejection of the overwhelmingly common non-integer case before the (more expensive)
    // conversion to a malachite `Float` and `Natural`.
    if !x.is_integer() {
        return None;
    }
    let mx = Float::from(x);
    let e = i64::from(mx.get_exponent()?);
    if e < 1 || u64::exact_from(e) > mx.get_prec()?.saturating_mul(64) {
        return None;
    }
    let n = Natural::try_from(&mx).ok()?;
    (&n).checked_log_base(&Natural::from(base))
}

// log_base(x, base) = ln(x) / ln(base). rug serves as an independent oracle by bracketing: compute
// ln(x) and ln(base) at a generous working precision rounding down and up, divide the brackets, and
// round both to `prec`. The true result lies between the two chains, so when both agree on the value
// and the ternary the answer is correctly rounded. Because ln(base) > 0 (base > 1), ln(x)/ln(base)
// increases with ln(x), so each bound divides the corresponding ln(x) bracket by the ln(base)
// bracket endpoint that extremizes it (which depends on the sign of ln(x)). The exact case
// x = base^n is detected up front, since then the result is an exactly-representable integer the
// brackets could never resolve.
pub fn rug_log_base_prec_round(
    x: &rug::Float,
    base: u64,
    prec: u64,
    rm: Round,
) -> (rug::Float, Ordering) {
    if x.is_finite() && x.is_sign_positive() && *x != 1u32 {
        if let Some(n) = rug_log_base_exact(x, base) {
            return rug::Float::with_val_round(u32::exact_from(prec), n, rm);
        }
    }
    let target_prec = u32::exact_from(prec);
    let mut working_prec = (prec << 1) + 128 + (rug_float_significant_bits(x) << 1);
    // A genuinely irrational log_base(x) converges at a working precision close to the target. If the
    // precision grows far past that, the true value must be an exactly-representable dyadic that the
    // brackets straddle (for example log_4(2) = 1/2, or log_9(27) = 3/2) -- a case the integer-power
    // detection above does not catch. This threshold is far above where any irrational result for
    // the (bounded) test inputs converges, and far below the runaway the straddle would cause.
    let exact_threshold = (prec << 1) + 512;
    loop {
        let wp = u32::exact_from(working_prec);
        // a_lo <= ln(x) <= a_hi
        let mut a_lo = rug::Float::with_val(wp, 0);
        a_lo.assign_round(x.ln_ref(), Round::Down);
        let mut a_hi = rug::Float::with_val(wp, 0);
        a_hi.assign_round(x.ln_ref(), Round::Up);
        // 0 < b_lo <= ln(base) <= b_hi
        let base_float = rug::Float::with_val(wp, base);
        let mut b_lo = rug::Float::with_val(wp, 0);
        b_lo.assign_round(base_float.ln_ref(), Round::Down);
        let mut b_hi = rug::Float::with_val(wp, 0);
        b_hi.assign_round(base_float.ln_ref(), Round::Up);
        // q_lo <= ln(x) / ln(base) <= q_hi
        let q_lo_den = if a_lo.is_sign_negative() { &b_lo } else { &b_hi };
        let q_hi_den = if a_hi.is_sign_negative() { &b_hi } else { &b_lo };
        let mut q_lo = rug::Float::with_val(wp, 0);
        q_lo.assign_round(&a_lo / q_lo_den, Round::Down);
        let mut q_hi = rug::Float::with_val(wp, 0);
        q_hi.assign_round(&a_hi / q_hi_den, Round::Up);
        let mut l_lo = rug::Float::with_val(target_prec, 0);
        let mut o_lo = l_lo.assign_round(&q_lo, rm);
        let mut l_hi = rug::Float::with_val(target_prec, 0);
        let mut o_hi = l_hi.assign_round(&q_hi, rm);
        if l_lo.is_nan() && l_hi.is_nan() {
            // x <= 0, so the result is NaN. (NaN != NaN, so equality tests against it never succeed.)
            return (l_lo, Equal);
        }
        // If one endpoint's rounding was exact but the other's wasn't, the result is irrational and
        // strictly between the chains, so the exact endpoint adopts the other's ternary.
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if l_lo == l_hi && o_lo == o_hi {
            return (l_lo, o_lo);
        }
        // The chains disagree (on the value under directed rounding, or only on the ternary), and the
        // precision has grown well past where any irrational result would converge for these bounded
        // inputs: the true value is a dyadic rational the brackets straddle (for example log_4(2) =
        // 1/2). Recover it exactly and round it.
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

pub fn rug_log_base_prec(x: &rug::Float, base: u64, prec: u64) -> (rug::Float, Ordering) {
    rug_log_base_prec_round(x, base, prec, Round::Nearest)
}

pub fn rug_log_base_round(x: &rug::Float, base: u64, rm: Round) -> (rug::Float, Ordering) {
    rug_log_base_prec_round(x, base, rug_float_significant_bits(x), rm)
}

pub fn rug_log_base(x: &rug::Float, base: u64) -> rug::Float {
    rug_log_base_prec_round(x, base, rug_float_significant_bits(x), Round::Nearest).0
}
