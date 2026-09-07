// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2002-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Port of MPFR's simultaneous sine and cosine. `mpfr_sin_cos` (`sin_cos.c`) reduces an argument
// with |x| >= 2 modulo 2 pi, takes the cosine of the reduced argument, and derives the sine as
// ±sqrt(1 - cos^2), all inside one Ziv loop that must certify both results. The `mpfr_sincos_fast`
// tier, used for precisions at or above `MPFR_SINCOS_THRESHOLD`, is not ported yet.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::cos::{
    NEAR_ZERO_MIN_CANCEL, cos_rational_helper, cos_rational_tiny, cos_turns_helper,
    cos_turns_special_case, cos_with_period_prec_round_normal_ref, reduce_huge, round_bracket,
    trig_near_zero, trig_rational_near_zero, trig_turns_near_zero,
};
use crate::float::arithmetic::round_near_x::float_round_near_x;
use crate::float::arithmetic::sin::{
    SCALED_INPUT_EXPONENT, sin_rational_helper, sin_turns_helper, sin_turns_special_case,
    sin_with_period_prec_round_normal_ref,
};
use crate::{Float, emulate_float_to_float_pair_fn, emulate_rational_to_float_pair_fn};
use core::cmp::Ordering::{self, Equal};
use core::cmp::{max, min};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, NegAssign, PowerOf2, SinCos, SinCosAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    NaN as NaNTrait, NegativeZero as NegativeZeroTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Ceiling, Down, Exact, Nearest, Up};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// The outcome of one iteration of the Ziv loop in `sin_cos_prec_round_normal_ref`.
enum SinCosStep {
    // The working precision could not decide both results; retry at a higher one.
    Retry,
    // The sine and cosine at the working precision, ready for the final rounding.
    Done(Float, Float),
    // The input is within about 2^-cancel of an odd multiple of pi/2, so the cosine is tiny and the
    // sine is within 2^-2cancel of ±1, with the given sign.
    NearZeroCos { cancel: u64, sin_negative: bool },
    // The input is within about 2^-cancel of a nonzero multiple of pi, so the sine is tiny and the
    // cosine is within 2^-2cancel of ±1, with the given sign.
    NearZeroSin { cancel: u64, cos_negative: bool },
}

// One iteration of the Ziv loop at working precision `m`, which the cancellation checks may raise
// for the next iteration (the caller applies the generic increase on `Retry`).
fn sin_cos_ziv_step(
    x: &Float,
    exp_x: i64,
    prec: u64,
    rm: RoundingMode,
    reduce: bool,
    m: &mut u64,
) -> SinCosStep {
    // A cancellation of this many bits sends a result to the near-zero path, and leaves the other
    // one within 2^-(prec + 2) of ±1, so that it rounds from ±1 alone.
    let near_zero_threshold = max(NEAR_ZERO_MIN_CANCEL, (prec >> 1) + 1);
    let m_i = i64::exact_from(*m);
    let xr;
    let xx = if reduce {
        // As in `mpfr_sin`: reduce x modulo 2 pi to xr, and check that xr is at least 2^(2-m) away
        // from 0 and from ±pi, which settles the sign of the sine.
        let c_prec = u64::exact_from(exp_x) + *m - 1;
        let pi = Float::pi_prec(c_prec).0;
        xr = x.ieee_remainder_prec_ref_val(&pi << 1u32, *m).0;
        let c = pi.sub_prec_round((&xr).abs(), c_prec, Down).0;
        let threshold = 3 - m_i;
        let xr_small = xr == 0u32 || i64::from(xr.get_exponent().unwrap()) < threshold;
        let c_small = c == 0u32 || i64::from(c.get_exponent().unwrap()) < threshold;
        if xr_small || c_small {
            // x is within 2^(4-m) of a multiple of pi (an even one if xr is small, an odd one if c
            // is small), so |sin(x)| < 2^(5-m); the near-zero path resolves the sine directly, and
            // the cosine is then ±1 to within 2^-2cancel.
            let cancel = *m - 4;
            return if cancel >= near_zero_threshold {
                SinCosStep::NearZeroSin {
                    cancel,
                    cos_negative: c_small,
                }
            } else {
                SinCosStep::Retry
            };
        }
        &xr
    } else {
        x
    };
    // the sign of the sine
    let sign = *xx < 0u32;
    // c = cos(xx) rounded toward zero
    let c = xx.cos_prec_round_ref(*m, Down).0;
    // If no argument reduction was performed, the error is at most ulp(c), otherwise it is at most
    // ulp(c) + 2^(2-m). Since |c| < 1, we have ulp(c) <= 2^(-m), thus the error is bounded by
    // 2^(3-m) in that later case.
    let exp_c = c.get_exponent().map_or(Float::MIN_EXPONENT_I64, i64::from);
    // |cos(x)| < 2^bound_exp
    let bound_exp = if reduce { max(exp_c, 2 - m_i) } else { exp_c } + 1;
    if bound_exp < 0 && exp_x >= 1 {
        let cancel = u64::exact_from(-bound_exp);
        if cancel >= near_zero_threshold {
            return SinCosStep::NearZeroCos {
                cancel,
                sin_negative: sign,
            };
        }
    }
    let err = if reduce { exp_c + m_i - 3 } else { m_i };
    if c == 0u32
        || err <= 0
        || !float_can_round(c.significand_ref().unwrap(), u64::exact_from(err), prec, rm)
    {
        return SinCosStep::Retry;
    }
    // s = sqrt(1 - c^2): the square rounds up, so its absolute error is bounded by 2^(5-m) if
    // reduce, and by 2^(2-m) otherwise; 1 - c^2 rounds to nearest, for 2^(6-m) or 2^(3-m); the
    // square root, also to nearest, has absolute error 2^(6-m-EXP(s)) or 2^(3-m-EXP(s)).
    let mut s = Float::ONE.sub_prec(c.square_round_ref(Ceiling).0, *m).0;
    if s == 0u32 {
        // 1 - c^2 rounded to zero, so sin(xx)^2 is below 2^-(m + 1): x is near a multiple of pi
        let cancel = (*m >> 1).saturating_sub(1);
        if reduce && cancel >= near_zero_threshold {
            return SinCosStep::NearZeroSin {
                cancel,
                cos_negative: c < 0u32,
            };
        }
        fail_on_untested_path("sin_cos_ziv_step, 1 - c^2 rounded to zero");
        *m = max(*m, x.significant_bits()) << 1;
        return SinCosStep::Retry;
    }
    s.sqrt_prec_assign(*m);
    let exp_s = i64::from(s.get_exponent().unwrap());
    // the absolute error on s is at most 2^(err - m)
    let err = 3 + if reduce { 3 } else { 0 } - exp_s;
    if sign {
        s.neg_assign();
    }
    // |sin(x)| < 2^bound_exp
    let bound_exp = max(exp_s, err - m_i) + 1;
    if reduce && bound_exp < 0 {
        let cancel = u64::exact_from(-bound_exp);
        if cancel >= near_zero_threshold {
            return SinCosStep::NearZeroSin {
                cancel,
                cos_negative: c < 0u32,
            };
        }
    }
    // put the error in the form 2^(EXP(s) - err)
    let err = exp_s + m_i - err;
    if err > 0 && float_can_round(s.significand_ref().unwrap(), u64::exact_from(err), prec, rm) {
        return SinCosStep::Done(s, c);
    }
    if err < i64::exact_from(prec) {
        *m += u64::exact_from(i64::exact_from(prec) - err);
    }
    // s is exactly ±1 (its square root rounded to nearest), so the sine is within an ulp of ±1
    // and the working precision is doubled
    if exp_s == 1 && s.eq_abs(&1u32) {
        *m <<= 1;
    }
    SinCosStep::Retry
}

// ±1 rounded to `prec` bits under `rm`, as the value of a function known to lie within 2^(1 - err)
// of ±1 on the side toward zero, with the ternary value; the negative case reuses the positive one
// with the rounding mode mirrored.
fn near_one(err: u64, negative: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let err = min(err, prec + 2);
    if negative {
        let (r, o) = float_round_near_x(&Float::ONE, err, false, prec, -rm).unwrap();
        (-r, o.reverse())
    } else {
        float_round_near_x(&Float::ONE, err, false, prec, rm).unwrap()
    }
}

// Computes sin(x) and cos(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding
// mode `rm`. (x = 0 is handled by the caller.) Neither result is ever exactly representable, so
// `rm` must not be `Exact`.
//
// This shares the work of `sin_rational_helper` and `cos_rational_helper`: x is rounded once to a
// `Float` y_f at a working precision w, both functions of y_f are taken together, and both are
// bracketed using the Lipschitz bound |f(x) - f(y_f)| <= |x - y_f|, the rounding errors, and, for
// an x too large to be a `Float`, the error of a single `Rational` reduction modulo 2 pi, which for
// such an x is the dominant cost. The brackets are rounded in `Rational` arithmetic, and w is
// raised until both resolve.
fn sin_cos_rational_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Float, Ordering, Ordering) {
    assert_ne!(rm, Exact, "Inexact sin_cos");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // For an x so small that the cosine rounds to 1, both results come cheaply from the separate
    // paths: the sine from its series (or the underflow rule, or, for a precision beyond 2^31 bits,
    // the general path), and the cosine from 1.
    if 1 - (exp_x << 1) > i64::exact_from(prec) {
        let (s, o_s) = sin_rational_helper(x, prec, rm);
        let (c, o_c) = cos_rational_tiny(prec, rm);
        return (s, c, o_s, o_c);
    }
    // an x too small to be a `Float` at a precision that does not round its cosine to 1 needs the
    // series paths of both functions, which is only reachable beyond 2^31 bits of precision
    if exp_x <= Float::MIN_EXPONENT_I64 {
        fail_on_untested_path("sin_cos_rational_helper, series paths");
        let (s, o_s) = sin_rational_helper(x, prec, rm);
        let (c, o_c) = cos_rational_helper(x, prec, rm);
        return (s, c, o_s, o_c);
    }
    let near_zero_threshold = max(NEAR_ZERO_MIN_CANCEL, (prec >> 1) + 1);
    let huge = exp_x >= Float::MAX_EXPONENT_I64;
    let mut w = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let reduced;
        let (y, extra) = if huge {
            reduced = reduce_huge(x, exp_x, w);
            (&reduced, Some(2 - i64::exact_from(w)))
        } else {
            (x, None)
        };
        if *y == 0u32 {
            // x is an exact multiple of 2 pi at the working precision; a higher precision breaks
            // the coincidence
            fail_on_untested_path("sin_cos_rational_helper, reduced argument is zero");
        } else {
            let (y_f, y_o) = Float::from_rational_prec_ref(y, w);
            if !huge && y_o == Equal {
                // x is exactly representable at w bits, so its sine and cosine are simply those
                return sin_cos_prec_round_normal_ref(&y_f, prec, rm);
            }
            let (s_f, c_f, _, _) = y_f.sin_cos_round_ref(Nearest);
            // The exponents of y, s_f, and c_f as `Float`s would have them (a zero result means
            // complete cancellation).
            let exp_y = y.floor_log_base_2_abs() + 1;
            let exp_s = s_f
                .get_exponent()
                .map_or(Float::MIN_EXPONENT_I64, i64::from);
            let exp_c = c_f
                .get_exponent()
                .map_or(Float::MIN_EXPONENT_I64, i64::from);
            let w_i = i64::exact_from(w);
            // |f(y) - f_f| <= 2^(exp_f - w) (half an ulp, doubled for safety) + |y - y_f| <=
            // 2^(exp_y - w), plus the reduction error; so |f(y)| < 2^(bound + 2) with bound the
            // largest of those exponents. Heavy cancellation in either function means y is close to
            // one of its zeros, which its near-zero path resolves exactly, while the other function
            // is then within 2^-2cancel of ±1 and rounds from ±1 alone.
            let error_exp = max(exp_y - w_i, extra.unwrap_or(i64::MIN));
            let bound_s = max(exp_s, error_exp) + 2;
            let bound_c = max(exp_c, error_exp) + 2;
            if bound_s < 0 {
                let cancel = u64::exact_from(-bound_s);
                if cancel >= near_zero_threshold {
                    let (s, o_s) = trig_rational_near_zero(y, exp_y, prec, rm, extra, w, false);
                    // 1 - |cos(x)| <= sin(x)^2 / 2 < 2^(2 bound_s - 1)
                    let (c, o_c) = near_one((cancel << 1) + 2, c_f < 0u32, prec, rm);
                    return (s, c, o_s, o_c);
                }
            }
            if bound_c < 0 {
                let cancel = u64::exact_from(-bound_c);
                if cancel >= near_zero_threshold {
                    let (c, o_c) = trig_rational_near_zero(y, exp_y, prec, rm, extra, w, true);
                    // 1 - |sin(x)| <= cos(x)^2 < 2^(2 bound_c)
                    let (s, o_s) = near_one((cancel << 1) + 1, s_f < 0u32, prec, rm);
                    return (s, c, o_s, o_c);
                }
            }
            let mut delta_s = Rational::power_of_2(exp_s - w_i) + Rational::power_of_2(exp_y - w_i);
            let mut delta_c = Rational::power_of_2(exp_c - w_i) + Rational::power_of_2(exp_y - w_i);
            if let Some(extra) = extra {
                let e = Rational::power_of_2(extra);
                delta_s += &e;
                delta_c += e;
            }
            let s = Rational::exact_from(&s_f);
            let c = Rational::exact_from(&c_f);
            if let Some((s, o_s)) = round_bracket(&(&s - &delta_s), &(s + delta_s), prec, rm)
                && let Some((c, o_c)) = round_bracket(&(&c - &delta_c), &(c + delta_c), prec, rm)
            {
                return (s, c, o_s, o_c);
            }
        }
        w += increment;
        increment = w >> 1;
    }
}

// This is mpfr_sin_cos from sin_cos.c, MPFR 4.2.2, without the `mpfr_sincos_fast` tier for
// precisions at or above `MPFR_SINCOS_THRESHOLD`, with the near-zero paths of `sin` and `cos` added
// for inputs extremely close to a zero of either function. Both results have precision `prec`,
// where MPFR allows two precisions and works at the larger.
fn sin_cos_prec_round_normal_ref(
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Float, Ordering, Ordering) {
    assert_ne!(rm, Exact, "Inexact sin_cos");
    let exp_x = i64::from(x.get_exponent().unwrap());
    let mut m = prec + prec.ceiling_log_base_2() + 13;
    // When x is close to 0, say 2^(-k), then there is a cancellation of about 2k bits in
    // 1-cos(x)^2, and both results may round from x and 1 alone: sin(x) = x - x^3/6 + ... has error
    // below 2^(3 EXP(x) - 2), and cos(x) = 1 - x^2/2 + ... has error below 2^(2 EXP(x) - 1). MPFR
    // tries the sine first and then the cosine; here the cosine's bound is the weaker one, and the
    // reference value 1 always rounds, so it decides.
    if exp_x < 0 {
        let neg_two_exp = u64::exact_from(-(exp_x << 1));
        let err_cos = neg_two_exp + 1;
        if err_cos > prec + 1
            && let Some((s, o_s)) =
                float_round_near_x(x, min(neg_two_exp + 2, prec + 2), false, prec, rm)
        {
            let (c, o_c) = near_one(err_cos, false, prec, rm);
            return (s, c, o_s, o_c);
        }
        m += neg_two_exp;
    }
    let reduce = exp_x >= 2;
    let mut increment = Limb::WIDTH;
    loop {
        match sin_cos_ziv_step(x, exp_x, prec, rm, reduce, &mut m) {
            SinCosStep::Done(s, c) => {
                let (s, o_s) = Float::from_float_prec_round(s, prec, rm);
                let (c, o_c) = Float::from_float_prec_round(c, prec, rm);
                return (s, c, o_s, o_c);
            }
            SinCosStep::NearZeroCos {
                cancel,
                sin_negative,
            } => {
                // 1 - |sin(x)| <= cos(x)^2 < 2^-2cancel
                let (c, o_c) = trig_near_zero(x, prec, rm, cancel, true);
                let (s, o_s) = near_one((cancel << 1) + 1, sin_negative, prec, rm);
                return (s, c, o_s, o_c);
            }
            SinCosStep::NearZeroSin {
                cancel,
                cos_negative,
            } => {
                // 1 - |cos(x)| <= sin(x)^2 / 2 < 2^-(2cancel + 1)
                let (s, o_s) = trig_near_zero(x, prec, rm, cancel, false);
                let (c, o_c) = near_one((cancel << 1) + 2, cos_negative, prec, rm);
                return (s, c, o_s, o_c);
            }
            SinCosStep::Retry => {}
        }
        m += increment;
        increment = m >> 1;
    }
}

// One Ziv iteration for the sine and cosine of a fraction of a turn q, given t = 2 pi q (1 +
// theta)^3 with |theta| <= 2^-w, rounded to w bits. Returns both results when they are settled,
// either from the values at the working precision or, for a result tiny enough, from the exact
// near-zero path, with the other then rounded from ±1; returns `None` if the working precision
// must be raised. `q` produces the exact fraction for the near-zero path, and `sin_near_zero` says
// whether q is large enough for a tiny sine to mean cancellation rather than a tiny q.
fn sin_cos_turns_step(
    t: &Float,
    w: u64,
    prec: u64,
    rm: RoundingMode,
    sin_near_zero: bool,
    q: impl Fn() -> Rational,
) -> Option<(Float, Float, Ordering, Ordering)> {
    // A cancellation of this many bits sends a result to the near-zero path, and leaves the other
    // one within 2^-(prec + 2) of ±1, so that it rounds from ±1 alone.
    let near_zero_threshold = max(NEAR_ZERO_MIN_CANCEL, (prec >> 1) + 1);
    // since w >= 2, |(1 + theta)^3 - 1| <= 4 theta, so t = 2 pi q + e with |e| <= 2^(EXP(t) + 2 -
    // w), and both sin and cos move by at most |e|
    let w_i = i64::exact_from(w);
    let err_t = i64::from(t.get_exponent().unwrap()) + 2 - w_i;
    // Both rounded away from zero, so that neither is zero (t is not a multiple of pi/2, being a
    // nonzero `Float`) and the computed magnitudes bound the true ones.
    let (s, c, _, _) = t.sin_cos_prec_round_ref(w, Up);
    let exp_s = i64::from(s.get_exponent().unwrap());
    let exp_c = i64::from(c.get_exponent().unwrap());
    // |sin(2 pi q)| <= |s| + |e| < 2^bound_s, and likewise for the cosine
    let bound_s = max(exp_s, err_t) + 1;
    let bound_c = max(exp_c, err_t) + 1;
    // A tiny sine with q not itself tiny means q is close to a multiple of 1/2, and a tiny cosine
    // means it is close to an odd multiple of 1/4. Either is resolved exactly by the near-zero
    // path, where the Ziv loop would need its precision raised by the whole cancellation, and the
    // other function is then within 2^-2cancel of ±1 and rounds from ±1 alone.
    if bound_s < 0 && sin_near_zero {
        let cancel = u64::exact_from(-bound_s);
        if cancel >= near_zero_threshold
            && let Some((s, o_s)) = trig_turns_near_zero(&q(), prec, rm, false)
        {
            // 1 - |cos(2 pi q)| <= sin(2 pi q)^2 / 2 < 2^(2 bound_s - 1)
            let (c, o_c) = near_one((cancel << 1) + 2, c < 0u32, prec, rm);
            return Some((s, c, o_s, o_c));
        }
    }
    if bound_c < 0 {
        let cancel = u64::exact_from(-bound_c);
        if cancel >= near_zero_threshold
            && let Some((c, o_c)) = trig_turns_near_zero(&q(), prec, rm, true)
        {
            // 1 - |sin(2 pi q)| <= cos(2 pi q)^2 < 2^(2 bound_c)
            let (s, o_s) = near_one((cancel << 1) + 1, s < 0u32, prec, rm);
            return Some((s, c, o_s, o_c));
        }
    }
    // The total error on each result is at most |e| + ulp, bounded by 2^(EXP + 1 - w) if err_t <=
    // EXP - w and by 2^(err_t + 1) otherwise; then normalized for can_round. For the sine, |sin(t)|
    // <= |t| gives EXP(s) <= EXP(t) + 1, so its ulp is at most 2^err_t / 2 and the second bound
    // always applies.
    let err_s = exp_s - err_t - 1;
    let err_c = exp_c
        - if err_t <= exp_c - w_i {
            exp_c - w_i + 1
        } else {
            err_t + 1
        };
    if err_s > 0
        && err_c > 0
        && float_can_round(
            s.significand_ref().unwrap(),
            u64::exact_from(err_s),
            prec,
            rm,
        )
        && float_can_round(
            c.significand_ref().unwrap(),
            u64::exact_from(err_c),
            prec,
            rm,
        )
    {
        let (s, o_s) = Float::from_float_prec_round(s, prec, rm);
        let (c, o_c) = Float::from_float_prec_round(c, prec, rm);
        return Some((s, c, o_s, o_c));
    }
    None
}

// Computes sin(2 pi x/u) and cos(2 pi x/u) for a finite nonzero `Float` x and a nonzero u, rounded
// to precision `prec` with rounding mode `rm`. `rm` may be `Exact` only when both results are
// exact, that is, when x/u is a multiple of 1/4.
//
// MPFR has no combined function here. This joins the `mpfr_sinu` and `mpfr_cosu` ports (see
// `sin_with_period_prec_round_normal_ref` and `cos_with_period_prec_round_normal_ref`) around one
// approximation of 2 pi x/u per Ziv iteration, and one `sin_cos` of it, with the near-zero paths of
// both.
fn sin_cos_with_period_prec_round_normal_ref(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Float, Ordering, Ordering) {
    // Range reduction, as in the sine: xr = x mod u, with the sign of x, exactly.
    let xr;
    let xp = if x.lt_abs(&u) {
        x
    } else {
        let p = i64::exact_from(x.get_prec().unwrap()) - i64::from(x.get_exponent().unwrap());
        let (r, o) =
            x.rem_unsigned_prec_round_ref(u, u64::WIDTH + u64::exact_from(max(p, 0)), Exact);
        assert_eq!(o, Equal);
        if r == 0u32 {
            // x is a multiple of u: the sine is zero, with the sign of x, and the cosine is 1
            return (
                if *x < 0u32 {
                    Float::NEGATIVE_ZERO
                } else {
                    Float::ZERO
                },
                Float::one_prec(prec),
                Equal,
                Equal,
            );
        }
        xr = r;
        &xr
    };
    // now |xp/u| < 1
    let exp_x = i64::from(xp.get_exponent().unwrap());
    // For x/u small, the cosine rounds from 1 alone: |cos(2 pi x/u) - 1| < 2^5 (x/u)^2 <= 2^(5 + 2
    // EXP(x) - 2 log2u), with u >= 2^log2u, as in the cosine. The sine has no such shortcut, being
    // close to 2 pi x/u, which must still be computed, so it takes its own path; there is nothing
    // to share.
    let log2u = if u == 1 {
        0
    } else {
        i64::exact_from(u.ceiling_log_base_2()) - 1
    };
    let err = ((log2u - exp_x) << 1) - 5;
    if err > 0 {
        let err = u64::exact_from(err);
        if err > prec + 1 {
            // such a small x/u is never a special case, and its cosine is never exact
            assert_ne!(rm, Exact, "Inexact sin_cos_with_period");
            let (s, o_s) = sin_with_period_prec_round_normal_ref(xp, u, prec, rm);
            let (c, o_c) = near_one(err, false, prec, rm);
            return (s, c, o_s, o_c);
        }
    }
    let u_bits = i64::exact_from(u.significant_bits());
    // The special cases need |x/u| >= 1/20, so the exponent test skips the `Rational` construction
    // for the small x that would make it expensive. Only a fraction of a turn with both closed
    // forms (a multiple of 1/4, or a denominator of 3, 6, 8, or 12) is taken from them; a fifth,
    // tenth, or twentieth of a turn has only one, and goes through the loop like any other input.
    if exp_x >= u_bits - 5 {
        let q = Rational::exact_from(xp) / Rational::from(u);
        if let Some((s, o_s)) = sin_turns_special_case(&q, prec, rm)
            && let Some((c, o_c)) = cos_turns_special_case(&q, prec, rm)
        {
            return (s, c, o_s, o_c);
        }
    }
    // Only the exact cases can be rounded exactly
    assert_ne!(rm, Exact, "Inexact sin_cos_with_period");
    if exp_x <= SCALED_INPUT_EXPONENT {
        // 2 pi x/u is within a few bits of the bottom of the exponent range, where the sine may
        // underflow while the cosine has not rounded to 1, which needs a precision beyond 2^31
        // bits: the separate functions handle each.
        fail_on_untested_path("sin_cos_with_period_prec_round_normal_ref, tiny x/u");
        let (s, o_s) = sin_with_period_prec_round_normal_ref(xp, u, prec, rm);
        let (c, o_c) = cos_with_period_prec_round_normal_ref(xp, u, prec, rm);
        return (s, c, o_s, o_c);
    }
    // For x large, since argument reduction is expensive, we want to avoid any failure in Ziv's
    // strategy, thus we take into account expx too.
    let mut prec_t =
        prec + u64::exact_from(max(exp_x, i64::exact_from(prec.ceiling_log_base_2()))) + 8;
    let mut increment = Limb::WIDTH;
    let u_float = Float::from(u);
    // A tiny sine with x/u not itself tiny means cancellation; for a tiny x/u the sine is simply
    // close to 2 pi x/u, and its `Rational` form would be expensive.
    let sin_near_zero = exp_x >= u_bits - 2;
    loop {
        // t = 2*pi*x/u * (1 + theta)^3 where |theta| <= 2^-prec_t, from rounding pi, the product,
        // and the quotient
        let mut t = Float::pi_prec(prec_t).0 << 1u32;
        t.mul_prec_assign_ref(xp, prec_t);
        t.div_prec_assign_ref(&u_float, prec_t);
        if let Some(result) = sin_cos_turns_step(&t, prec_t, prec, rm, sin_near_zero, || {
            Rational::exact_from(xp) / Rational::from(u)
        }) {
            return result;
        }
        prec_t += increment;
        increment = prec_t >> 1;
    }
}

impl Float {
    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, rounding both
    /// results to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`
    /// for it.
    ///
    /// The results are the same as those of [`Float::sin_prec_round`] and
    /// [`Float::cos_prec_round`], but the argument reduction and most of the work are shared, so
    /// this is faster than the two calls when both values are needed.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sin x+\varepsilon_s, \cos x+\varepsilon_c).
    /// $$
    /// - If $x$ is not finite, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or assumed to be
    ///   0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$ and $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon_s| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$ and $|\varepsilon_c| \leq 2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// If the outputs have a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=(\text{NaN},\text{NaN})$
    /// - $f(\pm\infty,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(\pm0.0,p,m)=(\pm0.0,1.0)$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$ and $|\cos x|\leq 1$, the results never overflow.
    /// - Each result underflows exactly as [`Float::sin_prec_round`] or [`Float::cos_prec_round`]
    ///   does: the sine for an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$ or of
    ///   magnitude $2^{-2^{30}}$ rounded toward zero, and the cosine for an input within
    ///   $2^{-2^{30}}$ of an odd multiple of $\pi/2$, either of which takes more than $2^{30}$ bits
    ///   of precision. See those functions for the values returned.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_cos_prec`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sin_cos_round`] instead. If both of these things are true, consider using
    /// [`Float::sin_cos`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, from which the sine is derived, costs
    /// the first term, and for $|x| \geq 2$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine and cosine of a finite nonzero [`Float`] are never
    /// exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_cos_prec_round(5, Floor);
    /// assert_eq!(s.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_cos_prec_round(5, Ceiling);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_cos_prec_round(20, Nearest);
    /// assert_eq!(s.to_string(), "0.84147072");
    /// assert_eq!(c.to_string(), "0.54030228");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_prec_round(
        self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_prec_round_ref(prec, rm)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, rounding both
    /// results to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. Two [`Ordering`]s are also returned, indicating whether the rounded sine
    /// and cosine are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_prec_round`] for the error bounds, the special cases, overflow and
    /// underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine and cosine of a finite nonzero [`Float`] are never
    /// exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let x = Float::from_unsigned_prec(1u32, 100).0;
    /// let (s, c, o_s, o_c) = x.sin_cos_prec_round_ref(5, Floor);
    /// assert_eq!(s.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = x.sin_cos_prec_round_ref(20, Nearest);
    /// assert_eq!(s.to_string(), "0.84147072");
    /// assert_eq!(c.to_string(), "0.54030228");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    pub fn sin_cos_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN | Infinity { .. } => (Self::NAN, Self::NAN, Equal, Equal),
            // sin(±0) = ±0 and cos(±0) = 1, exactly
            Zero { .. } => (self.clone(), Self::one_prec(prec), Equal, Equal),
            Finite { .. } => sin_cos_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, rounding both
    /// results to the nearest value of the specified precision. The [`Float`] is taken by value.
    /// Two [`Ordering`]s are also returned, indicating whether the rounded sine and cosine are less
    /// than, equal to, or greater than the exact values.
    ///
    /// If a result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sin_cos_prec_round`] for the error bounds, the special cases, overflow and
    /// underflow, and the complexity; this function behaves the same way with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_cos_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::sin_cos`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100).0.sin_cos_prec(5);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100).0.sin_cos_prec(20);
    /// assert_eq!(s.to_string(), "0.84147072");
    /// assert_eq!(c.to_string(), "0.54030228");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_prec(self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_prec_round_ref(prec, Nearest)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, rounding both
    /// results to the nearest value of the specified precision. The [`Float`] is taken by
    /// reference. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_prec`] and [`Float::sin_cos_prec_round`]; this function behaves the
    /// same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 100).0.sin_cos_prec_ref(5);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_prec_ref(&self, prec: u64) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_prec_round_ref(prec, Nearest)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, rounding both
    /// results to the precision of the input and with the specified rounding mode. The [`Float`] is
    /// taken by value. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_prec_round`] for the error bounds, the special cases, overflow and
    /// underflow, and the complexity; this function behaves the same way with `prec` equal to the
    /// precision of the input.
    ///
    /// If you want to specify an output precision, consider using [`Float::sin_cos_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sin_cos`] instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine and cosine of a finite nonzero [`Float`] are never
    /// exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 5).0.sin_cos_round(Floor);
    /// assert_eq!(s.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 5).0.sin_cos_round(Ceiling);
    /// assert_eq!(s.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sin_cos_round(self, rm: RoundingMode) -> (Self, Self, Ordering, Ordering) {
        let prec = self.significant_bits();
        self.sin_cos_prec_round_ref(prec, rm)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, rounding both
    /// results to the precision of the input and with the specified rounding mode. The [`Float`] is
    /// taken by reference. Two [`Ordering`]s are also returned, indicating whether the rounded sine
    /// and cosine are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_round`] and [`Float::sin_cos_prec_round`]; this function behaves the
    /// same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine and cosine of a finite nonzero [`Float`] are never
    /// exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 5)
    ///     .0
    ///     .sin_cos_round_ref(Floor);
    /// assert_eq!(s.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_round_ref(&self, rm: RoundingMode) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_prec_round_ref(self.significant_bits(), rm)
    }

    /// Replaces a [`Float`] with its sine and writes its cosine to `cos`, rounding both results to
    /// the specified precision and with the specified rounding mode. The previous value of `cos` is
    /// discarded. Two [`Ordering`]s are returned, indicating whether the rounded sine and cosine
    /// are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_prec_round`] for the error bounds, the special cases, overflow and
    /// underflow, and the complexity; this function behaves the same way.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_cos_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::sin_cos_round_assign`] instead. If both of these things are true, consider
    /// using [`Float::sin_cos_assign`] instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine and cosine of a finite nonzero [`Float`] are never
    /// exactly representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// let mut c = Float::NAN;
    /// assert_eq!(x.sin_cos_prec_round_assign(&mut c, 5, Floor), (Less, Less));
    /// assert_eq!(x.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// ```
    #[inline]
    pub fn sin_cos_prec_round_assign(
        &mut self,
        cos: &mut Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let (s, c, o_s, o_c) = self.sin_cos_prec_round_ref(prec, rm);
        *self = s;
        *cos = c;
        (o_s, o_c)
    }

    /// Replaces a [`Float`] with its sine and writes its cosine to `cos`, rounding both results to
    /// the nearest value of the specified precision. The previous value of `cos` is discarded. Two
    /// [`Ordering`]s are returned, indicating whether the rounded sine and cosine are less than,
    /// equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_prec`] and [`Float::sin_cos_prec_round`]; this function behaves the
    /// same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// let mut c = Float::NAN;
    /// assert_eq!(x.sin_cos_prec_assign(&mut c, 5), (Greater, Less));
    /// assert_eq!(x.to_string(), "0.844");
    /// assert_eq!(c.to_string(), "0.531");
    /// ```
    #[inline]
    pub fn sin_cos_prec_assign(&mut self, cos: &mut Self, prec: u64) -> (Ordering, Ordering) {
        self.sin_cos_prec_round_assign(cos, prec, Nearest)
    }

    /// Replaces a [`Float`] with its sine and writes its cosine to `cos`, rounding both results to
    /// the precision of the input and with the specified rounding mode. The previous value of `cos`
    /// is discarded. Two [`Ordering`]s are returned, indicating whether the rounded sine and cosine
    /// are less than, equal to, or greater than the exact values.
    ///
    /// See [`Float::sin_cos_round`] and [`Float::sin_cos_prec_round`]; this function behaves the
    /// same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine and cosine of a finite nonzero [`Float`] are never
    /// exactly representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 5).0;
    /// let mut c = Float::NAN;
    /// assert_eq!(x.sin_cos_round_assign(&mut c, Floor), (Less, Less));
    /// assert_eq!(x.to_string(), "0.812");
    /// assert_eq!(c.to_string(), "0.531");
    /// ```
    #[inline]
    pub fn sin_cos_round_assign(
        &mut self,
        cos: &mut Self,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let prec = self.significant_bits();
        self.sin_cos_prec_round_assign(cos, prec, rm)
    }
}

impl Float {
    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Rational`], together, rounding
    /// both results to the specified precision and with the specified rounding mode, and returning
    /// the results as [`Float`]s. The [`Rational`] is taken by value. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values.
    ///
    /// The results are the same as those of [`Float::sin_rational_prec_round`] and
    /// [`Float::cos_rational_prec_round`], but the rounding of the input, the argument reduction,
    /// and most of the work are shared, so this is faster than the two calls when both values are
    /// needed.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = (\sin x+\varepsilon_s, \cos x+\varepsilon_c).
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin x|\rfloor-p+1}$
    ///   and $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon_s| \leq 2^{\lfloor\log_2 |\sin x|\rfloor-p}$ and
    ///   $|\varepsilon_c| \leq 2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// These bounds do not apply when a result underflows.
    ///
    /// The outputs have precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=(0,1)$.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$ and $|\cos x|\leq 1$, the results never overflow.
    /// - Each result underflows exactly as [`Float::sin_rational_prec_round`] or
    ///   [`Float::cos_rational_prec_round`] does; see those functions for the inputs concerned and
    ///   the values returned.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_cos_rational_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] sine and cosine taken there together,
    /// which for $|x| \geq 2$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n +
    /// e$ bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(s.to_string(), "0.562");
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(s.to_string(), "0.594");
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_cos_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Rational`], together, rounding
    /// both results to the specified precision and with the specified rounding mode, and returning
    /// the results as [`Float`]s. The [`Rational`] is taken by reference. Two [`Ordering`]s are
    /// also returned, indicating whether the rounded sine and cosine are less than, equal to, or
    /// greater than the exact values.
    ///
    /// See [`Float::sin_cos_rational_prec_round`] for the error bounds, the special cases, overflow
    /// and underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(s.to_string(), "0.56464195");
    /// assert_eq!(c.to_string(), "0.82533550");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    pub fn sin_cos_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // sin(0) = 0 and cos(0) = 1, exactly
            return (Self::ZERO, Self::one_prec(prec), Equal, Equal);
        }
        sin_cos_rational_helper(x, prec, rm)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Rational`], together, rounding
    /// both results to the nearest value of the specified precision, and returning the results as
    /// [`Float`]s. The [`Rational`] is taken by value. Two [`Ordering`]s are also returned,
    /// indicating whether the rounded sine and cosine are less than, equal to, or greater than the
    /// exact values.
    ///
    /// If a result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sin_cos_rational_prec_round`] for the error bounds, the special cases, overflow
    /// and underflow, and the complexity; this function behaves the same way with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_cos_rational_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::sin_cos_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(s.to_string(), "0.562");
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::sin_cos_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(s.to_string(), "0.56464291");
    /// assert_eq!(c.to_string(), "0.82533550");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_cos_rational_prec(x: Rational, prec: u64) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Rational`], together, rounding
    /// both results to the nearest value of the specified precision, and returning the results as
    /// [`Float`]s. The [`Rational`] is taken by reference. Two [`Ordering`]s are also returned,
    /// indicating whether the rounded sine and cosine are less than, equal to, or greater than the
    /// exact values.
    ///
    /// See [`Float::sin_cos_rational_prec`] and [`Float::sin_cos_rational_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(s.to_string(), "0.562");
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_rational_prec_round_ref(x, prec, Nearest)
    }
}

// Computes sin(2 pi q) and cos(2 pi q) for a nonzero `Rational` fraction of a turn q in (-1, 1),
// rounded to precision `prec` with rounding mode `rm`. `rm` may be `Exact` only when both results
// are exact, that is, when q is a multiple of 1/4. This is the `Float` algorithm with the fraction
// of a turn taken directly: since q is exact, only pi and the product are rounded.
fn sin_cos_turns_helper(
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Float, Ordering, Ordering) {
    let exp_q = q.floor_log_base_2_abs() + 1;
    // for q small, the cosine rounds from 1 alone: |cos(2 pi q) - 1| < 1/2 (2 pi q)^2 < 2^(5 + 2
    // EXP(q)); the sine takes its own path, as in the `Float` version
    let err = -(exp_q << 1) - 5;
    if err > 0 {
        let err = u64::exact_from(err);
        if err > prec + 1 {
            assert_ne!(rm, Exact, "Inexact sin_cos_with_period");
            let (s, o_s) = sin_turns_helper(q, prec, rm);
            let (c, o_c) = near_one(err, false, prec, rm);
            return (s, c, o_s, o_c);
        }
    }
    // The special cases need |q| >= 1/20; only a q with both closed forms is taken from them
    if exp_q >= -4
        && let Some((s, o_s)) = sin_turns_special_case(q, prec, rm)
        && let Some((c, o_c)) = cos_turns_special_case(q, prec, rm)
    {
        return (s, c, o_s, o_c);
    }
    // Only the exact cases can be rounded exactly
    assert_ne!(rm, Exact, "Inexact sin_cos_with_period");
    if exp_q <= SCALED_INPUT_EXPONENT {
        // as in the `Float` version, only reachable beyond 2^31 bits of precision
        fail_on_untested_path("sin_cos_turns_helper, tiny q");
        let (s, o_s) = sin_turns_helper(q, prec, rm);
        let (c, o_c) = cos_turns_helper(q, prec, rm);
        return (s, c, o_s, o_c);
    }
    let mut w = prec + prec.ceiling_log_base_2() + 8;
    let mut increment = Limb::WIDTH;
    let sin_near_zero = exp_q >= -2;
    loop {
        // t = 2*pi*q * (1 + theta)^3 where |theta| <= 2^-w, from rounding q, pi, and the product
        let t = (Float::pi_prec(w).0 << 1u32)
            .mul_prec(Float::from_rational_prec_ref(q, w).0, w)
            .0;
        if let Some(result) = sin_cos_turns_step(&t, w, prec, rm, sin_near_zero, || q.clone()) {
            return result;
        }
        w += increment;
        increment = w >> 1;
    }
}

impl Float {
    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Float`] measured
    /// in $u$ths of a turn, together, rounding both results to the specified precision and with the
    /// specified rounding mode. The [`Float`] is taken by value. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal` for it.
    ///
    /// The results are the same as those of [`Float::sin_with_period_prec_round`] and
    /// [`Float::cos_with_period_prec_round`], but the argument reduction, the computation of $2\pi
    /// x/u$, and most of the work are shared, so this is faster than the two calls when both values
    /// are needed.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = (\sin(2\pi x/u)+\varepsilon_s, \cos(2\pi x/u)+\varepsilon_c).
    /// $$
    /// - If $x$ is not finite or $u=0$, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or
    ///   assumed to be 0.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is not `Nearest`, then $|\varepsilon_s| <
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p+1}$ and $|\varepsilon_c| < 2^{\lfloor\log_2
    ///   |\cos(2\pi x/u)|\rfloor-p+1}$.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is `Nearest`, then $|\varepsilon_s| \leq
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p}$ and $|\varepsilon_c| \leq 2^{\lfloor\log_2
    ///   |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// If the outputs have a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(\pm\infty,u,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(x,0,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(\pm0.0,u,p,m)=(\pm0.0,1.0)$
    /// - If $x/u$ is a multiple of $1/4$, both results are exact: the sine is $0.0$ with the sign
    ///   of $x$, $1$, or $-1$, and the cosine is $1$, $0.0$, or $-1$, as for
    ///   [`Float::sin_with_period_prec_round`] and [`Float::cos_with_period_prec_round`].
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 12, one result is exactly $\pm1/2$ or
    /// both are $\pm\sqrt2/2$, and the other is $\pm\sqrt3/2$; these are computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine and cosine, which is far
    /// faster. (A fifth, tenth, or twentieth of a turn has a closed form for only one of the two,
    /// and is computed like any other input.)
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$ and $|\cos(2\pi x/u)|\leq 1$, the results never overflow.
    /// - Each result underflows exactly as [`Float::sin_with_period_prec_round`] or
    ///   [`Float::cos_with_period_prec_round`] does: the sine for $x/u$ within $2^{-2^{30}}$ of a
    ///   multiple of $1/2$ without being one, or for an $x$ so small that $2\pi x/u$ is below
    ///   $2^{-2^{30}}$, and the cosine for $x/u$ within $2^{-2^{30}}$ of an odd multiple of $1/4$
    ///   without being one, which takes more than $2^{30}$ bits of precision. See those functions
    ///   for the values returned.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_cos_with_period_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::sin_cos_with_period_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the argument is reduced modulo $u$ exactly, and the sine and cosine of
    /// $2\pi x/u$ are then taken together at a working precision of about $n + e$ bits, which needs
    /// $\pi$ to that many bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or
    /// $x$ is zero or not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec_round(7, 10, Floor);
    /// assert_eq!(s.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec_round(7, 10, Ceiling);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62402");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec_round(7, 10, Nearest);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// // a quarter turn is exact
    /// let (s, c, o_s, o_c) = Float::from(90u32).sin_cos_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(s.to_string(), "1.0000");
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o_s, Equal);
    /// assert_eq!(o_c, Equal);
    ///
    /// // a twelfth of a turn: 1/2 exactly, and sqrt(3)/2
    /// let (s, c, o_s, o_c) = Float::from(30u32).sin_cos_with_period_prec_round(360, 10, Nearest);
    /// assert_eq!(s.to_string(), "0.50000");
    /// assert_eq!(c.to_string(), "0.86621");
    /// assert_eq!(o_s, Equal);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_prec_round_ref(u, prec, rm)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Float`] measured
    /// in $u$ths of a turn, together, rounding both results to the specified precision and with the
    /// specified rounding mode. The [`Float`] is taken by reference. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_prec_round`] for the error bounds, the special cases,
    /// overflow and underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or
    /// $x$ is zero or not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec_round_ref(7, 10, Floor);
    /// assert_eq!(s.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec_round_ref(7, 10, Ceiling);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62402");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    pub fn sin_cos_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // for u=0, return NaN
            _ if u == 0 => (Self::NAN, Self::NAN, Equal, Equal),
            NaN | Infinity { .. } => (Self::NAN, Self::NAN, Equal, Equal),
            // x is zero: sin(±0) = ±0 and cos(±0) = 1
            Zero { .. } => (self.clone(), Self::one_prec(prec), Equal, Equal),
            Finite { .. } => sin_cos_with_period_prec_round_normal_ref(self, u, prec, rm),
        }
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Float`] measured
    /// in $u$ths of a turn, together, rounding both results to the nearest value of the specified
    /// precision. The [`Float`] is taken by value. Two [`Ordering`]s are also returned, indicating
    /// whether the rounded sine and cosine are less than, equal to, or greater than the exact
    /// values. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal` for it.
    ///
    /// If a result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sin_cos_with_period_prec_round`] for the error bounds, the special cases,
    /// overflow and underflow, and the complexity; this function behaves the same way with
    /// `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_cos_with_period_prec_round`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::sin_cos_with_period_round`] with
    /// `Nearest` instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec(7, 10);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec(360, 53);
    /// assert_eq!(s.to_string(), "0.017452406437283512");
    /// assert_eq!(c.to_string(), "0.99984769515639127");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    ///
    /// // an eighth of a turn: sqrt(2)/2 for both
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec(8, 10);
    /// assert_eq!(s.to_string(), "0.70703");
    /// assert_eq!(c.to_string(), "0.70703");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec(self, u: u64, prec: u64) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_prec_round_ref(u, prec, Nearest)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Float`] measured
    /// in $u$ths of a turn, together, rounding both results to the nearest value of the specified
    /// precision. The [`Float`] is taken by reference. Two [`Ordering`]s are also returned,
    /// indicating whether the rounded sine and cosine are less than, equal to, or greater than the
    /// exact values. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_prec`] and [`Float::sin_cos_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::ONE.sin_cos_with_period_prec_ref(7, 10);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec_ref(
        &self,
        u: u64,
        prec: u64,
    ) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_prec_round_ref(u, prec, Nearest)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Float`] measured
    /// in $u$ths of a turn, together, rounding both results to the precision of the input and with
    /// the specified rounding mode. The [`Float`] is taken by value. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_prec_round`] for the error bounds, the special cases,
    /// overflow and underflow, and the complexity; this function behaves the same way with `prec`
    /// equal to the precision of the input.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sin_cos_with_period_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::sin_cos_with_period_prec`] with the precision of the
    /// input instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the results cannot be represented exactly with the precision
    /// of the input (which is the case unless $x/u$ is a multiple of $1/4$, or $x$ is zero or not
    /// finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 10)
    ///     .0
    ///     .sin_cos_with_period_round(7, Floor);
    /// assert_eq!(s.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 10)
    ///     .0
    ///     .sin_cos_with_period_round(7, Ceiling);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62402");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_round(
        self,
        u: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        let prec = self.significant_bits();
        self.sin_cos_with_period_prec_round_ref(u, prec, rm)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Float`] measured
    /// in $u$ths of a turn, together, rounding both results to the precision of the input and with
    /// the specified rounding mode. The [`Float`] is taken by reference. Two [`Ordering`]s are also
    /// returned, indicating whether the rounded sine and cosine are less than, equal to, or greater
    /// than the exact values. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_round`] and [`Float::sin_cos_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the results cannot be represented exactly with the precision
    /// of the input (which is the case unless $x/u$ is a multiple of $1/4$, or $x$ is zero or not
    /// finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::from_unsigned_prec(1u32, 10)
    ///     .0
    ///     .sin_cos_with_period_round_ref(7, Floor);
    /// assert_eq!(s.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_round_ref(
        &self,
        u: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        self.sin_cos_with_period_prec_round_ref(u, self.significant_bits(), rm)
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its sine and writes its cosine to
    /// `cos`, rounding both results to the specified precision and with the specified rounding
    /// mode. The previous value of `cos` is discarded. Two [`Ordering`]s are returned, indicating
    /// whether the rounded sine and cosine are less than, equal to, or greater than the exact
    /// values. Although `NaN`s are not comparable to any [`Float`], whenever this function sets a
    /// `NaN` it also returns `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_prec_round`] for the error bounds, the special cases,
    /// overflow and underflow, and the complexity; this function behaves the same way.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::sin_cos_with_period_prec_assign`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::sin_cos_with_period_round_assign`]
    /// instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or
    /// $x$ is zero or not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::ONE;
    /// let mut c = Float::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_prec_round_assign(&mut c, 7, 10, Floor),
    ///     (Less, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec_round_assign(
        &mut self,
        cos: &mut Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let (s, c, o_s, o_c) = self.sin_cos_with_period_prec_round_ref(u, prec, rm);
        *self = s;
        *cos = c;
        (o_s, o_c)
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its sine and writes its cosine to
    /// `cos`, rounding both results to the nearest value of the specified precision. The previous
    /// value of `cos` is discarded. Two [`Ordering`]s are returned, indicating whether the rounded
    /// sine and cosine are less than, equal to, or greater than the exact values. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function sets a `NaN` it also returns
    /// `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_prec`] and [`Float::sin_cos_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::ONE;
    /// let mut c = Float::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_prec_assign(&mut c, 7, 10),
    ///     (Greater, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62305");
    /// ```
    #[inline]
    pub fn sin_cos_with_period_prec_assign(
        &mut self,
        cos: &mut Self,
        u: u64,
        prec: u64,
    ) -> (Ordering, Ordering) {
        self.sin_cos_with_period_prec_round_assign(cos, u, prec, Nearest)
    }

    /// Replaces a [`Float`] measured in $u$ths of a turn with its sine and writes its cosine to
    /// `cos`, rounding both results to the precision of the input and with the specified rounding
    /// mode. The previous value of `cos` is discarded. Two [`Ordering`]s are returned, indicating
    /// whether the rounded sine and cosine are less than, equal to, or greater than the exact
    /// values. Although `NaN`s are not comparable to any [`Float`], whenever this function sets a
    /// `NaN` it also returns `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_round`] and [`Float::sin_cos_with_period_prec_round`]; this
    /// function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the results cannot be represented exactly with the precision
    /// of the input (which is the case unless $x/u$ is a multiple of $1/4$, or $x$ is zero or not
    /// finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0;
    /// let mut c = Float::NAN;
    /// assert_eq!(
    ///     x.sin_cos_with_period_round_assign(&mut c, 7, Floor),
    ///     (Less, Less)
    /// );
    /// assert_eq!(x.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// ```
    #[inline]
    pub fn sin_cos_with_period_round_assign(
        &mut self,
        cos: &mut Self,
        u: u64,
        rm: RoundingMode,
    ) -> (Ordering, Ordering) {
        let prec = self.significant_bits();
        self.sin_cos_with_period_prec_round_assign(cos, u, prec, rm)
    }
}

impl Float {
    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`]
    /// measured in $u$ths of a turn, together, rounding both results to the specified precision and
    /// with the specified rounding mode, and returning the results as [`Float`]s. The [`Rational`]
    /// is taken by value. Two [`Ordering`]s are also returned, indicating whether the rounded sine
    /// and cosine are less than, equal to, or greater than the exact values. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal` for it.
    ///
    /// The results are the same as those of [`Float::sin_with_period_rational_prec_round`] and
    /// [`Float::cos_with_period_rational_prec_round`], but the reduction of the fraction of a turn,
    /// the computation of $2\pi x/u$, and most of the work are shared, so this is faster than the
    /// two calls when both values are needed.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = (\sin(2\pi x/u)+\varepsilon_s, \cos(2\pi x/u)+\varepsilon_c).
    /// $$
    /// - If $u=0$, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or assumed to be 0.
    /// - If $u\neq 0$ and $m$ is not `Nearest`, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin(2\pi
    ///   x/u)|\rfloor-p+1}$ and $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p+1}$.
    /// - If $u\neq 0$ and $m$ is `Nearest`, then $|\varepsilon_s| \leq 2^{\lfloor\log_2 |\sin(2\pi
    ///   x/u)|\rfloor-p}$ and $|\varepsilon_c| \leq 2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p}$.
    ///
    /// If the outputs have a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=(\text{NaN},\text{NaN})$
    /// - $f(0,u,p,m)=(0,1)$
    /// - If $x/u$ is a multiple of $1/4$, both results are exact: the sine is $0.0$ with the sign
    ///   of $x$, $1$, or $-1$, and the cosine is $1$, $0.0$, or $-1$, as for
    ///   [`Float::sin_with_period_rational_prec_round`] and
    ///   [`Float::cos_with_period_rational_prec_round`].
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 12, one result is exactly $\pm1/2$ or
    /// both are $\pm\sqrt2/2$, and the other is $\pm\sqrt3/2$; these are computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine and cosine, which is far
    /// faster. (A fifth, tenth, or twentieth of a turn has a closed form for only one of the two,
    /// and is computed like any other input.)
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$ and $|\cos(2\pi x/u)|\leq 1$, the results never overflow.
    /// - Each result underflows exactly as [`Float::sin_with_period_rational_prec_round`] or
    ///   [`Float::cos_with_period_rational_prec_round`] does: the sine for $x/u$ within
    ///   $2^{-2^{30}}$ of a multiple of $1/2$ without being one, or for an $x/u$ so small that
    ///   $2\pi x/u$ is below $2^{-2^{30}}$, and the cosine for $x/u$ within $2^{-2^{30}}$ of an odd
    ///   multiple of $1/4$ without being one, which takes a denominator of more than $2^{30}$ bits.
    ///   See those functions for the values returned.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::sin_cos_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^{3/2} \log n \log\log n + (n+m) (\log (n+m))^2 \log\log (n+m))$
    ///
    /// $M(n, m) = O((n+m) \log (n+m))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `x.significant_bits()`: the fraction of a turn is reduced modulo 1 exactly, so only its size
    /// and the precision drive the cost, not the magnitude of $x$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or
    /// $x$ or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_with_period_rational_prec_round(Rational::ONE, 7, 10, Floor);
    /// assert_eq!(s.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_with_period_rational_prec_round(Rational::ONE, 7, 10, Ceiling);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62402");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Greater);
    ///
    /// // a quarter turn is exact
    /// let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 4),
    ///     1,
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(s.to_string(), "1.0000");
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o_s, Equal);
    /// assert_eq!(o_c, Equal);
    ///
    /// // a twelfth of a turn: 1/2 exactly, and sqrt(3)/2
    /// let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 12),
    ///     1,
    ///     10,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.50000");
    /// assert_eq!(c.to_string(), "0.86621");
    /// assert_eq!(o_s, Equal);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_cos_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_with_period_rational_prec_round_ref(&x, u, prec, rm)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`]
    /// measured in $u$ths of a turn, together, rounding both results to the specified precision and
    /// with the specified rounding mode, and returning the results as [`Float`]s. The [`Rational`]
    /// is taken by reference. Two [`Ordering`]s are also returned, indicating whether the rounded
    /// sine and cosine are less than, equal to, or greater than the exact values. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal` for it.
    ///
    /// See [`Float::sin_cos_with_period_rational_prec_round`] for the error bounds, the special
    /// cases, overflow and underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the results cannot be represented
    /// exactly with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or
    /// $x$ or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) =
    ///     Float::sin_cos_with_period_rational_prec_round_ref(&Rational::ONE, 7, 10, Floor);
    /// assert_eq!(s.to_string(), "0.78125");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    ///
    /// // an eighth of a turn: sqrt(2)/2 for both
    /// let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 8),
    ///     1,
    ///     10,
    ///     Nearest,
    /// );
    /// assert_eq!(s.to_string(), "0.70703");
    /// assert_eq!(c.to_string(), "0.70703");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Less);
    /// ```
    pub fn sin_cos_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Self, Ordering, Ordering) {
        assert_ne!(prec, 0);
        // for u = 0, return NaN
        if u == 0 {
            return (Self::NAN, Self::NAN, Equal, Equal);
        }
        // sin(0) = 0 (a `Rational` zero has no sign) and cos(0) = 1
        if *x == 0u32 {
            return (Self::ZERO, Self::one_prec(prec), Equal, Equal);
        }
        // q = x/u, reduced to (-1, 1) with the sign of x: both functions have period 1 in q, and a
        // multiple of u gives a sine of zero with the sign of x (IEEE 754-2019's sinPi) and a
        // cosine of 1
        let q = x / Rational::from(u);
        let whole = Rational::from(Integer::rounding_from(&q, Down).0);
        let q = q - whole;
        if q == 0u32 {
            return (
                if *x < 0u32 {
                    Self::NEGATIVE_ZERO
                } else {
                    Self::ZERO
                },
                Self::one_prec(prec),
                Equal,
                Equal,
            );
        }
        sin_cos_turns_helper(&q, prec, rm)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`]
    /// measured in $u$ths of a turn, together, rounding both results to the nearest value of the
    /// specified precision, and returning the results as [`Float`]s. The [`Rational`] is taken by
    /// value. Two [`Ordering`]s are also returned, indicating whether the rounded sine and cosine
    /// are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`
    /// for it.
    ///
    /// If a result is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::sin_cos_with_period_rational_prec_round`] for the error bounds, the special
    /// cases, overflow and underflow, and the complexity; this function behaves the same way with
    /// `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_cos_with_period_rational_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec(Rational::ONE, 7, 10);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    ///
    /// let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec(Rational::ONE, 360, 53);
    /// assert_eq!(s.to_string(), "0.017452406437283512");
    /// assert_eq!(c.to_string(), "0.99984769515639127");
    /// assert_eq!(o_s, Less);
    /// assert_eq!(o_c, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_cos_with_period_rational_prec(
        x: Rational,
        u: u64,
        prec: u64,
    ) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_with_period_rational_prec_round_ref(&x, u, prec, Nearest)
    }

    /// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`]
    /// measured in $u$ths of a turn, together, rounding both results to the nearest value of the
    /// specified precision, and returning the results as [`Float`]s. The [`Rational`] is taken by
    /// reference. Two [`Ordering`]s are also returned, indicating whether the rounded sine and
    /// cosine are less than, equal to, or greater than the exact values. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`
    /// for it.
    ///
    /// See [`Float::sin_cos_with_period_rational_prec`] and
    /// [`Float::sin_cos_with_period_rational_prec_round`]; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (s, c, o_s, o_c) = Float::sin_cos_with_period_rational_prec_ref(&Rational::ONE, 7, 10);
    /// assert_eq!(s.to_string(), "0.78223");
    /// assert_eq!(c.to_string(), "0.62305");
    /// assert_eq!(o_s, Greater);
    /// assert_eq!(o_c, Less);
    /// ```
    #[inline]
    pub fn sin_cos_with_period_rational_prec_ref(
        x: &Rational,
        u: u64,
        prec: u64,
    ) -> (Self, Self, Ordering, Ordering) {
        Self::sin_cos_with_period_rational_prec_round_ref(x, u, prec, Nearest)
    }
}

impl SinCos for Float {
    type Output = Self;

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, taking it by
    /// value.
    ///
    /// If the outputs have a precision, it is the precision of the input. If a result is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = (\sin x+\varepsilon_s, \cos x+\varepsilon_c).
    /// $$
    /// - If $x$ is not finite, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or assumed to be
    ///   0.
    /// - If $x$ is finite, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$ and
    ///   $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos x|\rfloor-p}$, where $p$ is the precision of the
    ///   input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=(\text{NaN},\text{NaN})$
    /// - $f(\pm\infty)=(\text{NaN},\text{NaN})$
    /// - $f(\pm0.0)=(\pm0.0,1.0)$
    ///
    /// See [`Float::sin_cos_prec_round`] for overflow, underflow, and the complexity.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_cos_round`] instead. If you want to specify an output precision, consider using
    /// [`Float::sin_cos_prec`] instead. If you want both of these things, consider using
    /// [`Float::sin_cos_prec_round`] instead.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SinCos;
    /// use malachite_base::num::basic::traits::{NaN, NegativeZero, Zero};
    /// use malachite_float::Float;
    ///
    /// let (s, c) = Float::NAN.sin_cos();
    /// assert!(s.is_nan());
    /// assert!(c.is_nan());
    ///
    /// let (s, c) = Float::ZERO.sin_cos();
    /// assert_eq!(s.to_string(), "0.0");
    /// assert_eq!(c.to_string(), "1.0");
    ///
    /// let (s, c) = Float::NEGATIVE_ZERO.sin_cos();
    /// assert_eq!(s.to_string(), "-0.0");
    /// assert_eq!(c.to_string(), "1.0");
    ///
    /// let (s, c) = Float::from_unsigned_prec(1u32, 100).0.sin_cos();
    /// assert_eq!(s.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(c.to_string(), "0.54030230586813971740093660744335");
    /// ```
    #[inline]
    fn sin_cos(self) -> (Self, Self) {
        let prec = self.significant_bits();
        let (s, c, _, _) = self.sin_cos_prec_round_ref(prec, Nearest);
        (s, c)
    }
}

impl SinCos for &Float {
    type Output = Float;

    /// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Float`], together, taking it by
    /// reference.
    ///
    /// See [`Float::sin_cos`]; this function behaves the same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SinCos;
    /// use malachite_float::Float;
    ///
    /// let (s, c) = (&Float::from_unsigned_prec(1u32, 100).0).sin_cos();
    /// assert_eq!(s.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(c.to_string(), "0.54030230586813971740093660744335");
    /// ```
    #[inline]
    fn sin_cos(self) -> (Float, Float) {
        let (s, c, _, _) = self.sin_cos_prec_round_ref(self.significant_bits(), Nearest);
        (s, c)
    }
}

impl SinCosAssign for Float {
    /// Replaces a [`Float`] with its sine and writes its cosine to `cos`, rounding both results to
    /// the nearest value of the input's precision. The previous value of `cos` is discarded.
    ///
    /// See [`Float::sin_cos`]; this function behaves the same way.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SinCosAssign;
    /// use malachite_base::num::basic::traits::NaN;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// let mut c = Float::NAN;
    /// x.sin_cos_assign(&mut c);
    /// assert_eq!(x.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(c.to_string(), "0.54030230586813971740093660744335");
    /// ```
    #[inline]
    fn sin_cos_assign(&mut self, cos: &mut Self) {
        let prec = self.significant_bits();
        self.sin_cos_prec_round_assign(cos, prec, Nearest);
    }
}

/// Computes $\sin x$ and $\cos x$, the sine and cosine of a primitive float, together. Using this
/// function is more accurate than using the default `sin_cos` function or the ones provided by
/// `libm`.
///
/// The results are those of
/// [`primitive_float_sin`](crate::float::arithmetic::sin::primitive_float_sin) and
/// [`primitive_float_cos`](crate::float::arithmetic::cos::primitive_float_cos), but the argument
/// reduction and most of the work are shared, so this is faster than the two calls when both values
/// are needed.
///
/// $$
/// f(x) = (\sin x+\varepsilon_s, \cos x+\varepsilon_c).
/// $$
/// - If $x$ is not finite, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or assumed to be 0.
/// - If $x$ is finite, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$ and
///   $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos x|\rfloor-p}$, where $p$ is the precision of the
///   output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=(\text{NaN},\text{NaN})$
/// - $f(\pm\infty)=(\text{NaN},\text{NaN})$
/// - $f(\pm0.0)=(\pm0.0,1.0)$
///
/// Overflow is not possible, since the results lie in $[-1, 1]$. The sine is subnormal only when
/// $x$ is, and then it is $x$ itself; the cosine is never subnormal. See
/// [`primitive_float_sin`](crate::float::arithmetic::sin::primitive_float_sin) and
/// [`primitive_float_cos`](crate::float::arithmetic::cos::primitive_float_cos).
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin_cos::primitive_float_sin_cos;
///
/// let (s, c) = primitive_float_sin_cos(f32::NAN);
/// assert!(s.is_nan());
/// assert!(c.is_nan());
///
/// let (s, c) = primitive_float_sin_cos(0.0f32);
/// assert_eq!(NiceFloat(s), NiceFloat(0.0));
/// assert_eq!(NiceFloat(c), NiceFloat(1.0));
///
/// let (s, c) = primitive_float_sin_cos(1.0f32);
/// assert_eq!(NiceFloat(s), NiceFloat(0.84147096));
/// assert_eq!(NiceFloat(c), NiceFloat(0.5403023));
///
/// let (s, c) = primitive_float_sin_cos(1.0f64);
/// assert_eq!(NiceFloat(s), NiceFloat(0.8414709848078965));
/// assert_eq!(NiceFloat(c), NiceFloat(0.5403023058681398));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin_cos<T: PrimitiveFloat>(x: T) -> (T, T)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_pair_fn(Float::sin_cos_prec, x)
}

/// Computes $\sin x$ and $\cos x$, the sine and cosine of a [`Rational`], together, returning the
/// results as primitive floats.
///
/// The results are those of
/// [`primitive_float_sin_rational`](crate::float::arithmetic::sin::primitive_float_sin_rational)
/// and
/// [`primitive_float_cos_rational`](crate::float::arithmetic::cos::primitive_float_cos_rational),
/// but the rounding of the input, the argument reduction, and most of the work are shared, so this
/// is faster than the two calls when both values are needed.
///
/// $$
/// f(x) = (\sin x+\varepsilon_s, \cos x+\varepsilon_c),
/// $$
/// where $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$ and $|\varepsilon_c| <
/// 2^{\lfloor\log_2 |\cos x|\rfloor-p}$, and $p$ is the precision of the output (24 if `T` is a
/// [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(0)=(0,1)$
///
/// Overflow is not possible, since the results lie in $[-1, 1]$. The sine underflows, to a
/// subnormal or to zero, when $x$ is tiny, since $\sin x$ is then very close to $x$; the cosine is
/// never subnormal. See
/// [`primitive_float_sin_rational`](crate::float::arithmetic::sin::primitive_float_sin_rational)
/// and
/// [`primitive_float_cos_rational`](crate::float::arithmetic::cos::primitive_float_cos_rational).
///
/// # Worst-case complexity
/// $T(m, e) = O((m+e) (\log (m+e))^2 \log\log (m+e))$
///
/// $M(m, e) = O((m+e) \log (m+e))$
///
/// where $T$ is time, $M$ is additional memory, $m$ is `x.significant_bits()`, and $e$ is
/// `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): for $|x| \geq 2$ the
/// argument is reduced modulo $2\pi$, which needs $\pi$ to about $e$ bits.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin_cos::primitive_float_sin_cos_rational;
/// use malachite_q::Rational;
///
/// let (s, c) = primitive_float_sin_cos_rational::<f64>(&Rational::ZERO);
/// assert_eq!(NiceFloat(s), NiceFloat(0.0));
/// assert_eq!(NiceFloat(c), NiceFloat(1.0));
///
/// let (s, c) = primitive_float_sin_cos_rational::<f64>(&Rational::from_unsigneds(1u8, 3));
/// assert_eq!(NiceFloat(s), NiceFloat(0.32719469679615226));
/// assert_eq!(NiceFloat(c), NiceFloat(0.9449569463147377));
///
/// let (s, c) = primitive_float_sin_cos_rational::<f32>(&Rational::from_unsigneds(1u8, 3));
/// assert_eq!(NiceFloat(s), NiceFloat(0.3271947));
/// assert_eq!(NiceFloat(c), NiceFloat(0.94495696));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin_cos_rational<T: PrimitiveFloat>(x: &Rational) -> (T, T)
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_pair_fn(Float::sin_cos_rational_prec_ref, x)
}

/// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a primitive float
/// measured in $u$ths of a turn (so that `u = 360` is degrees), together.
///
/// The results are those of
/// [`primitive_float_sin_with_period`](super::sin::primitive_float_sin_with_period) and
/// [`primitive_float_cos_with_period`](super::cos::primitive_float_cos_with_period), but the
/// argument reduction and most of the work are shared, so this is faster than the two calls when
/// both values are needed.
///
/// $$
/// f(x,u) = (\sin(2\pi x/u)+\varepsilon_s, \cos(2\pi x/u)+\varepsilon_c).
/// $$
/// - If $x$ is not finite or $u=0$, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or assumed
///   to be 0.
/// - If $x$ is finite and $u\neq 0$, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin(2\pi
///   x/u)|\rfloor-p}$ and $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p}$, where
///   $p$ is the precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN},u)=(\text{NaN},\text{NaN})$
/// - $f(\pm\infty,u)=(\text{NaN},\text{NaN})$
/// - $f(x,0)=(\text{NaN},\text{NaN})$
/// - $f(\pm0.0,u)=(\pm0.0,1.0)$
/// - If $x/u$ is a multiple of $1/4$, both results are exact: the sine is $0.0$ with the sign of
///   $x$, $1$, or $-1$, and the cosine is $1$, $0.0$, or $-1$.
///
/// Overflow is not possible, since the results lie in $[-1, 1]$. The sine underflows, to a
/// subnormal or to zero, only when $2\pi x/u$ does, which takes a subnormal $x$ or a large $u$; the
/// cosine is never subnormal. See
/// [`primitive_float_sin_with_period`](super::sin::primitive_float_sin_with_period) and
/// [`primitive_float_cos_with_period`](super::cos::primitive_float_cos_with_period).
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin_cos::primitive_float_sin_cos_with_period;
///
/// let (s, c) = primitive_float_sin_cos_with_period(f32::NAN, 360);
/// assert!(s.is_nan());
/// assert!(c.is_nan());
///
/// let (s, c) = primitive_float_sin_cos_with_period(1.0f32, 0);
/// assert!(s.is_nan());
/// assert!(c.is_nan());
///
/// let (s, c) = primitive_float_sin_cos_with_period(90.0f32, 360);
/// assert_eq!(NiceFloat(s), NiceFloat(1.0));
/// assert_eq!(NiceFloat(c), NiceFloat(0.0));
///
/// let (s, c) = primitive_float_sin_cos_with_period(30.0f64, 360);
/// assert_eq!(NiceFloat(s), NiceFloat(0.5));
/// assert_eq!(NiceFloat(c), NiceFloat(0.8660254037844386));
///
/// let (s, c) = primitive_float_sin_cos_with_period(1.0f32, 7);
/// assert_eq!(NiceFloat(s), NiceFloat(0.7818315));
/// assert_eq!(NiceFloat(c), NiceFloat(0.6234898));
///
/// let (s, c) = primitive_float_sin_cos_with_period(1.0f64, 7);
/// assert_eq!(NiceFloat(s), NiceFloat(0.7818314824680298));
/// assert_eq!(NiceFloat(c), NiceFloat(0.6234898018587335));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin_cos_with_period<T: PrimitiveFloat>(x: T, u: u64) -> (T, T)
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_pair_fn(|x, prec| Float::sin_cos_with_period_prec(x, u, prec), x)
}

/// Computes $\sin(2\pi x/u)$ and $\cos(2\pi x/u)$, the sine and cosine of a [`Rational`] measured
/// in $u$ths of a turn (so that `u = 360` is degrees), together, returning the results as primitive
/// floats.
///
/// The results are those of
/// [`primitive_float_sin_with_period_rational`](super::sin::primitive_float_sin_with_period_rational)
/// and
/// [`primitive_float_cos_with_period_rational`](super::cos::primitive_float_cos_with_period_rational),
/// but the reduction of the fraction of a turn and most of the work are shared, so this is faster
/// than the two calls when both values are needed.
///
/// $$
/// f(x,u) = (\sin(2\pi x/u)+\varepsilon_s, \cos(2\pi x/u)+\varepsilon_c).
/// $$
/// - If $u=0$, $\varepsilon_s$ and $\varepsilon_c$ may be ignored or assumed to be 0.
/// - If $u\neq 0$, then $|\varepsilon_s| < 2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p}$ and
///   $|\varepsilon_c| < 2^{\lfloor\log_2 |\cos(2\pi x/u)|\rfloor-p}$, where $p$ is the precision of
///   the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(x,0)=(\text{NaN},\text{NaN})$
/// - $f(0,u)=(0,1)$
/// - If $x/u$ is a multiple of $1/4$, both results are exact: the sine is $0.0$ with the sign of
///   $x$, $1$, or $-1$, and the cosine is $1$, $0.0$, or $-1$.
///
/// Overflow is not possible, since the results lie in $[-1, 1]$. The sine underflows, to a
/// subnormal or to zero, only when $2\pi x/u$ does, for a tiny $x/u$; the cosine is never
/// subnormal. See
/// [`primitive_float_sin_with_period_rational`](super::sin::primitive_float_sin_with_period_rational)
/// and
/// [`primitive_float_cos_with_period_rational`](super::cos::primitive_float_cos_with_period_rational).
///
/// # Worst-case complexity
/// $T(m) = O(m (\log m)^2 \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`: the fraction of
/// a turn is reduced modulo 1 exactly, so the magnitude of $x$ does not drive the cost.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin_cos::primitive_float_sin_cos_with_period_rational;
/// use malachite_q::Rational;
///
/// let (s, c) = primitive_float_sin_cos_with_period_rational::<f64>(&Rational::ZERO, 0);
/// assert!(s.is_nan());
/// assert!(c.is_nan());
///
/// let (s, c) = primitive_float_sin_cos_with_period_rational::<f64>(&Rational::ZERO, 360);
/// assert_eq!(NiceFloat(s), NiceFloat(0.0));
/// assert_eq!(NiceFloat(c), NiceFloat(1.0));
///
/// // a twelfth of a turn: exactly 1/2, and sqrt(3)/2
/// let (s, c) =
///     primitive_float_sin_cos_with_period_rational::<f64>(&Rational::from_unsigneds(1u8, 12), 1);
/// assert_eq!(NiceFloat(s), NiceFloat(0.5));
/// assert_eq!(NiceFloat(c), NiceFloat(0.8660254037844386));
///
/// let (s, c) =
///     primitive_float_sin_cos_with_period_rational::<f32>(&Rational::from_unsigneds(1u8, 7), 1);
/// assert_eq!(NiceFloat(s), NiceFloat(0.7818315));
/// assert_eq!(NiceFloat(c), NiceFloat(0.6234898));
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
pub fn primitive_float_sin_cos_with_period_rational<T: PrimitiveFloat>(
    x: &Rational,
    u: u64,
) -> (T, T)
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_pair_fn(
        |x, prec| Float::sin_cos_with_period_rational_prec_ref(x, u, prec),
        x,
    )
}
