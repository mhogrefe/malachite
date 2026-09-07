// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2001-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

// Port of MPFR's sine. `mpfr_sin` (`sin.c`) reduces an argument with |x| >= 2 modulo 2 pi using
// `mpfr_remainder`, which also settles the sign of the result, and then computes sin(x) = ±sqrt(1
// - cos(x)^2) from the cosine, all inside a Ziv loop. The `mpfr_sin_fast` tier, used for precisions
// at or above `MPFR_SINCOS_THRESHOLD` and built on `mpfr_sincos_fast`, is not ported yet.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::cos::{
    NEAR_ZERO_MIN_CANCEL, TrigStep, half_constant, phi_minus_1_prec_round, reduce_huge,
    round_bracket, sin_bound, trig_near_zero, trig_rational_near_zero, trig_turns_near_zero,
};
use crate::float::arithmetic::round_near_x::float_round_near_x;
use crate::{Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::{max, min};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    Abs, CeilingLogBase2, Mod, NegAssign, PowerOf2, Sin, SinAssign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    NaN as NaNTrait, NegativeZero as NegativeZeroTrait, One, Zero as ZeroTrait,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{
    self, Ceiling, Down, Exact, Floor, Nearest, Up,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// One iteration of the Ziv loop at working precision `m`, which the cancellation checks may raise
// for the next iteration (the caller applies the generic increase on `Retry`).
fn sin_ziv_step(
    x: &Float,
    exp_x: i64,
    prec: u64,
    rm: RoundingMode,
    reduce: bool,
    m: &mut u64,
) -> TrigStep {
    // The near-zero path is taken for a cancellation of at least this many bits.
    let near_zero_threshold = max(NEAR_ZERO_MIN_CANCEL, prec >> 4);
    // first perform argument reduction modulo 2*Pi (if needed), also helps to determine the sign of
    // sin(x)
    let xr;
    let xx = if reduce {
        let c_prec = u64::exact_from(exp_x) + *m - 1;
        let pi = Float::pi_prec(c_prec).0;
        xr = x.ieee_remainder_prec_ref_val(&pi << 1u32, *m).0;
        // The analysis is similar to that of cos.c: |xr - x - 2kPi| <= 2^(2-m). Thus we can decide
        // the sign of sin(x) if xr is at distance at least 2^(2-m) of both 0 and +/-Pi.
        //
        // Since c approximates Pi with an error <= 2^(2-expx-m) <= 2^(-m), it suffices to check
        // that c - |xr| >= 2^(2-m).
        let c = pi.sub_prec_round((&xr).abs(), c_prec, Down).0;
        let threshold = 3 - i64::exact_from(*m);
        if xr == 0u32
            || i64::from(xr.get_exponent().unwrap()) < threshold
            || c == 0u32
            || i64::from(c.get_exponent().unwrap()) < threshold
        {
            // x is within 2^(4-m) of a multiple of pi (if |xr| is small, of 2k pi; if c is small,
            // of (2k + 1) pi), so |sin(x)| < 2^(5-m), and with m already above prec by a margin,
            // the near-zero path resolves the result directly. MPFR instead keeps raising m until
            // the reduced argument is resolved.
            let cancel = *m - 4;
            return if cancel >= near_zero_threshold {
                TrigStep::NearZero(cancel)
            } else {
                TrigStep::Retry
            };
        }
        // |xr - x - 2kPi| <= 2^(2-m), thus |sin(xr) - sin(x)| <= 2^(2-m)
        &xr
    } else {
        // the input argument is already reduced
        x
    };
    let sign = *xx < 0u32;
    // now that the argument is reduced, precision m is enough. c = cos(x) rounded away, squared
    // rounding away, then 1 - c^2 and its square root rounding toward zero
    let c = xx
        .cos_prec_round_ref(*m, Up)
        .0
        .square_prec_round(*m, Ceiling)
        .0;
    let mut c = Float::ONE
        .sub_prec_round(c, *m, Down)
        .0
        .sqrt_prec_round(*m, Down)
        .0;
    if sign {
        c.neg_assign();
    }
    // Warning: c may be 0!
    if c == 0u32 {
        // 1 - cos(xx)^2 rounded to zero, so sin(xx)^2 is below 2^(3-m) and |sin(x)| below 2^(3-m)/2
        // + 2^(2-m)
        let cancel = (*m >> 1).saturating_sub(3);
        if reduce && cancel >= near_zero_threshold {
            return TrigStep::NearZero(cancel);
        }
        // Huge cancellation: increase prec a lot!
        *m = max(*m, x.significant_bits()) << 1;
        return TrigStep::Retry;
    }
    // the absolute error on c is at most 2^(3-m-EXP(c)), plus 2^(2-m) if there was an argument
    // reduction. Since EXP(c) <= 1, 3-m-EXP(c) >= 2-m, thus the error is at most 2^(3-m-EXP(c)) in
    // case of argument reduction.
    let exp_c = i64::from(c.get_exponent().unwrap());
    let err = (exp_c << 1) + i64::exact_from(*m) - 3 - i64::from(reduce);
    if err > 0 && float_can_round(c.significand_ref().unwrap(), u64::exact_from(err), prec, rm) {
        return TrigStep::Done(c);
    }
    // |sin(x)| < 2^bound_exp, since |sin(x)| <= |c| + 2^(4-m-EXP(c))
    let bound_exp = max(exp_c, 4 - i64::exact_from(*m) - exp_c) + 1;
    if reduce && bound_exp < 0 {
        let cancel = u64::exact_from(-bound_exp);
        if cancel >= near_zero_threshold {
            return TrigStep::NearZero(cancel);
        }
    }
    // check for huge cancellation (Near 0)
    if err < i64::exact_from(prec) {
        *m += u64::exact_from(i64::exact_from(prec) - err);
    }
    // MPFR also doubles m here "if near 1", when EXP(c) = 1. That cannot happen: the squared cosine
    // is positive, so 1 - c^2 rounded toward zero is below 1, and so is its square root rounded
    // toward zero.
    assert_ne!(exp_c, 1);
    TrigStep::Retry
}

// Brackets sin(x) for a nonzero `Rational` x, small enough that its series converges in a few
// terms, between partial sums of that series, tightening the bracket until both ends round the same
// way. This also covers inputs too small to be `Float`s, whose sines underflow, since everything is
// done in `Rational` arithmetic.
fn sin_rational_series(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let mut w = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let lo = sin_bound(x, w, false);
        let hi = sin_bound(x, w, true);
        if let Some(result) = round_bracket(&lo, &hi, prec, rm) {
            return result;
        }
        w += increment;
        increment = w >> 1;
    }
}

// Computes sin(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding mode `rm`.
// (sin(0) = 0 is handled by the caller.) The sine of a nonzero rational is transcendental, so the
// result is never exactly representable and `rm` must not be `Exact`.
//
// A small x is handled by its series. Otherwise, as in `cos_rational_helper`, x is rounded to a
// `Float` y_f at a working precision w, its correctly rounded sine s_f is taken, and sin(x) is
// bracketed using |sin(x) - sin(y_f)| <= |x - y_f|, the rounding error of s_f, and, for an x too
// large to be a `Float`, the error of a `Rational` reduction modulo 2 pi. The bracket is rounded in
// `Rational` arithmetic, and w is raised until both ends agree.
pub(crate) fn sin_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact sin");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // With |x| < 2^exp_x, the kth term of the series is below |x| 2^(2k exp_x), so when -exp_x is
    // at least a sixteenth of the working precision, about 8 terms suffice, which is cheaper than a
    // `Float` sine at that precision. This also covers every x too small to be a `Float`.
    if exp_x < const { Float::MIN_EXPONENT_I64 - 1 } {
        // |sin(x)| < |x| < 2^(MIN_EXPONENT - 2), a quarter of the smallest positive Float, so the
        // result is zero or that Float, by the rounding mode alone, and no 2^30-bit arithmetic is
        // needed.
        return underflowed(*x > 0u32, prec, rm);
    }
    if exp_x < 0 && u64::exact_from(-exp_x) << 4 >= prec + 10 {
        return sin_rational_series(x, prec, rm);
    }
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
            fail_on_untested_path("sin_rational_helper, reduced argument is zero");
        } else {
            let (y_f, y_o) = Float::from_rational_prec_ref(y, w);
            if !huge && y_o == Equal {
                // x is exactly representable at w bits, so sin(x) is simply its sine
                return sin_prec_round_normal_ref(&y_f, prec, rm);
            }
            let s_f = (&y_f).sin();
            // The exponents of y and s_f, as `Float`s would have them (s_f is zero only if it
            // underflowed, which counts as complete cancellation).
            let exp_y = y.floor_log_base_2_abs() + 1;
            let exp_s = s_f
                .get_exponent()
                .map_or(Float::MIN_EXPONENT_I64, i64::from);
            // |sin(y)| < 2^exp_s (up to the bracket width): heavy cancellation means y is close to
            // a multiple of pi, where the bracket below would have to be far narrower than 2^-w.
            if exp_s < 0 {
                let cancel = u64::exact_from(-exp_s);
                if cancel >= max(NEAR_ZERO_MIN_CANCEL, prec >> 4) {
                    return trig_rational_near_zero(y, exp_y, prec, rm, extra, w, false);
                }
            }
            // |s_f - sin(y_f)| <= 2^(exp_s - w) (half an ulp, doubled for safety), and |sin(y) -
            // sin(y_f)| <= |y - y_f| <= 2^(exp_y - w)
            let w_i = i64::exact_from(w);
            let mut delta = Rational::power_of_2(exp_s - w_i) + Rational::power_of_2(exp_y - w_i);
            if let Some(extra) = extra {
                delta += Rational::power_of_2(extra);
            }
            let s = Rational::exact_from(&s_f);
            if let Some(result) = round_bracket(&(&s - &delta), &(s + delta), prec, rm) {
                return result;
            }
        }
        w += increment;
        increment = w >> 1;
    }
}

// The result of a function whose exact value is nonzero, has the given sign, and is below a quarter
// of the smallest positive `Float` in magnitude: zero or that `Float`, by the rounding mode alone.
fn underflowed(positive: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let away = match rm {
        Ceiling => positive,
        Floor => !positive,
        Up => true,
        _ => false,
    };
    let min_positive = Float::min_positive_value_prec(prec);
    match (positive, away) {
        (true, true) => (min_positive, Greater),
        (true, false) => (Float::ZERO, Less),
        (false, true) => (-min_positive, Less),
        (false, false) => (Float::NEGATIVE_ZERO, Greater),
    }
}

// MPFR computes 2 pi x/u inside a widened exponent range, so it never underflows there. Here, for
// an x/u within 2^66 of the bottom of the range, the computation is scaled up by 2^64 and the
// underflow decided by hand: a division that rounded up to the smallest positive Float would
// otherwise make the Ziv loop retry forever, since sin of that power of 2 can never be certified.
const SCALE: u64 = 64;
// The exponent of the scaled smallest positive Float, 2^(MIN_EXPONENT - 1) * 2^SCALE.
const MIN_SCALED_EXPONENT: i64 = Float::MIN_EXPONENT_I64 + SCALE as i64;
// Inputs with at most this exponent are scaled.
pub(crate) const SCALED_INPUT_EXPONENT: i64 = Float::MIN_EXPONENT_I64 + 66;

// Given t = 2^SCALE * 2 pi x/u to within a relative 2^(2 - prec), returns the result if the true
// value, and so its sine, which is just below it, is below the smallest positive Float: zero or
// that Float, by the rounding mode alone.
fn scaled_underflow(
    t: &Float,
    positive: bool,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    let exp_t = i64::from(t.get_exponent().unwrap());
    if exp_t >= MIN_SCALED_EXPONENT {
        return None;
    }
    // to nearest, the smallest positive Float wins from half of it upward, i.e. from one exponent
    // below (the value cannot be exactly half, being transcendental)
    let away = match rm {
        Ceiling => positive,
        Floor => !positive,
        Up => true,
        Nearest => exp_t == const { MIN_SCALED_EXPONENT - 1 },
        _ => false,
    };
    let min_positive = Float::min_positive_value_prec(prec);
    Some(match (positive, away) {
        (true, true) => (min_positive, Greater),
        (true, false) => (Float::ZERO, Less),
        (false, true) => (-min_positive, Less),
        (false, false) => (Float::NEGATIVE_ZERO, Greater),
    })
}

// The closed-form cases of sin(2 pi x / u), keyed by the denominator d of x/u in lowest terms (with
// |x| < u, so the numerator n is the angle in units of 1/d of a turn). MPFR's exact cases are (a) d
// dividing 4, where the sine is 0 (with the sign of x, following IEEE 754-2019's sinPi, so that the
// function is odd), 1, or -1, and (b) d = 12, where it is 1/2 or -1/2. Beyond MPFR, the algebraic
// cases are dispatched to a single correctly rounded constant: d = 3 or 6 gives sqrt(3)/2, d = 8
// gives sqrt(2)/2, and d = 20 gives phi/2 or (phi - 1)/2, up to sign. (Fifths and tenths of a turn
// have no such form for the sine.) Those constants are never exact, so they return `None` for
// `Exact`.
pub(crate) fn sin_turns_special_case(
    q: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    let d = q.denominator_ref();
    if *d > 20u32 {
        return None;
    }
    let d = u64::exact_from(d);
    // the angle in units of 1/d of a turn (the numerator of a `Rational` is unsigned, so the sign
    // is restored before reducing modulo d)
    let n = u64::exact_from(
        &Integer::from_sign_and_abs_ref(*q >= 0u32, q.numerator_ref()).mod_op(Integer::from(d)),
    );
    // the sine is negative in the second half of the turn
    let negative = n > d >> 1;
    match d {
        // sin(0) = sin(180°) = 0, with the sign of x
        1 | 2 => Some((
            if *q < 0u32 {
                Float::NEGATIVE_ZERO
            } else {
                Float::ZERO
            },
            Equal,
        )),
        // sin(90°) = 1, sin(270°) = -1
        4 => Some((
            if negative {
                -Float::one_prec(prec)
            } else {
                Float::one_prec(prec)
            },
            Equal,
        )),
        // sin(30°) = sin(150°) = 1/2, sin(210°) = sin(330°) = -1/2
        12 => Some((
            if negative {
                -(Float::one_prec(prec) >> 1u32)
            } else {
                Float::one_prec(prec) >> 1u32
            },
            Equal,
        )),
        _ if rm == Exact => None,
        // sin(60°) = sin(120°) = sqrt(3)/2, sin(240°) = sin(300°) = -sqrt(3)/2
        3 | 6 => Some(half_constant(
            |prec, rm| const { Float::const_from_unsigned(3) }.sqrt_prec_round(prec, rm),
            negative,
            prec,
            rm,
        )),
        // sin(45°) = sin(135°) = sqrt(2)/2, sin(225°) = sin(315°) = -sqrt(2)/2
        8 => Some(half_constant(Float::sqrt_2_prec_round, negative, prec, rm)),
        // sin(18°) = sin(162°) = (phi - 1)/2, sin(54°) = sin(126°) = phi/2, and their negatives
        // at 198°, 342°, 234°, and 306°
        20 => Some(if n == 1 || n == 9 || n == 11 || n == 19 {
            half_constant(phi_minus_1_prec_round, negative, prec, rm)
        } else {
            half_constant(Float::phi_prec_round, negative, prec, rm)
        }),
        _ => None,
    }
}

// Computes sin(2 pi x / u) for a finite nonzero `Float` x and a nonzero u, rounded to precision
// `prec` with rounding mode `rm`. `rm` may be `Exact` only in the exact cases (see
// `sin_turns_special_case`).
//
// This is mpfr_sinu from sinu.c, MPFR 4.2.2, with the additional near-zero path.
pub(crate) fn sin_with_period_prec_round_normal_ref(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // Range reduction. We do not need to reduce the argument if it is already reduced (|x| < u).
    // Note that the case |x| = u is better in the "else" branch as it will give xr = 0.
    let xr;
    let xp = if x.lt_abs(&u) {
        x
    } else {
        // xr = x mod u, with the sign of x, exactly: its precision is the size of u plus the length
        // of the fractional part of x.
        let p = i64::exact_from(x.get_prec().unwrap()) - i64::from(x.get_exponent().unwrap());
        let (r, o) =
            x.rem_unsigned_prec_round_ref(u, u64::WIDTH + u64::exact_from(max(p, 0)), Exact);
        assert_eq!(o, Equal);
        if r == 0u32 {
            // x is a multiple of u: the sine is zero, with the sign of x (IEEE 754-2019's sinPi)
            return (
                if *x < 0u32 {
                    Float::NEGATIVE_ZERO
                } else {
                    Float::ZERO
                },
                Equal,
            );
        }
        xr = r;
        &xr
    };
    // now |xp/u| < 1
    let exp_x = i64::from(xp.get_exponent().unwrap());
    // The special cases need |x/u| >= 1/20, so the exponent test skips the `Rational` construction
    // for the small x that would make it expensive (a tiny x has a huge power-of-2 denominator).
    let u_bits = i64::exact_from(u.significant_bits());
    if exp_x >= u_bits - 5
        && let Some(result) =
            sin_turns_special_case(&(Rational::exact_from(xp) / Rational::from(u)), prec, rm)
    {
        return result;
    }
    // Only the exact cases can be rounded exactly
    assert_ne!(rm, Exact, "Inexact sin_with_period");
    // For x large, since argument reduction is expensive, we want to avoid any failure in Ziv's
    // strategy, thus we take into account expx too.
    let mut prec_t =
        prec + u64::exact_from(max(exp_x, i64::exact_from(prec.ceiling_log_base_2()))) + 8;
    let mut increment = Limb::WIDTH;
    let u_float = Float::from(u);
    let scaled = exp_x <= SCALED_INPUT_EXPONENT;
    let xs;
    let xp_scaled = if scaled {
        xs = xp << SCALE;
        &xs
    } else {
        xp
    };
    loop {
        // We first compute an approximation t of 2*pi*x/u, then call sin(t). If t = 2*pi*x/u + s,
        // then |sin(t) - sin(2*pi*x/u)| <= |s|. t = 2*pi * (1 + theta1) where |theta1| <= 2^-prec
        let mut t = Float::pi_prec(prec_t).0 << 1u32;
        // t = 2*pi*x * (1 + theta2)^2 where |theta2| <= 2^-prec
        t.mul_prec_assign_ref(xp_scaled, prec_t);
        // t = 2*pi*x/u * (1 + theta3)^3 where |theta3| <= 2^-prec
        t.div_prec_assign_ref(&u_float, prec_t);
        if scaled {
            if let Some(result) = scaled_underflow(&t, *xp > 0u32, prec, rm) {
                return result;
            }
            t >>= SCALE;
        }
        // since prec >= 2, |(1 + theta3)^3 - 1| <= 4*theta3 <= 2^(2-prec)
        let exp_t = i64::from(t.get_exponent().unwrap());
        // we have |s| <= 2^(expt + 2 - prec)
        let prec_t_i = i64::exact_from(prec_t);
        let mut err = exp_t + 2 - prec_t_i;
        // rounding away from zero, so that t cannot be zero here: we excluded t = 0 before, which
        // is the only exact case where sin(t) = 0
        t.sin_prec_round_assign(prec_t, Up);
        let exp_t = i64::from(t.get_exponent().unwrap());
        // A tiny sine with x/u not itself tiny means x/u is close to a multiple of 1/2, which the
        // near-zero path resolves exactly; the Ziv loop would need its precision raised by the
        // whole cancellation. (For a tiny x/u the sine is simply close to 2 pi x/u, with no
        // cancellation, and the `Rational` construction would be expensive.)
        if exp_t < 0 && exp_x >= u_bits - 2 {
            let cancel = u64::exact_from(-exp_t);
            if cancel >= max(NEAR_ZERO_MIN_CANCEL, prec >> 4)
                && let Some(result) = trig_turns_near_zero(
                    &(Rational::exact_from(xp) / Rational::from(u)),
                    prec,
                    rm,
                    false,
                )
            {
                return result;
            }
        }
        // the total error is bounded by 2^err + ulp(t) = 2^err + 2^(expt-prec) thus if err <=
        // expt-prec, it is bounded by 2^(expt-prec+1), otherwise it is bounded by 2^(err+1).
        err = if err <= exp_t - prec_t_i {
            exp_t - prec_t_i + 1
        } else {
            err + 1
        };
        // normalize err for mpfr_can_round
        err = exp_t - err;
        if err > 0 && float_can_round(t.significand_ref().unwrap(), u64::exact_from(err), prec, rm)
        {
            return Float::from_float_prec_round(t, prec, rm);
        }
        // (MPFR checks its exact cases here, after the first level of Ziv's strategy; the special
        // cases above cover them before the loop, since the check is cheap.)
        prec_t += increment;
        increment = prec_t >> 1;
    }
}

// Computes sin(2 pi q) for a nonzero `Rational` fraction of a turn q in (-1, 1), rounded to
// precision `prec` with rounding mode `rm`. `rm` may be `Exact` only in the exact cases (see
// `sin_turns_special_case`). This is the `Float` algorithm with the fraction of a turn taken
// directly: since q is exact, only pi and the product are rounded.
pub(crate) fn sin_turns_helper(q: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let exp_q = q.floor_log_base_2_abs() + 1;
    // The special cases need |q| >= 1/20
    if exp_q >= -4
        && let Some(result) = sin_turns_special_case(q, prec, rm)
    {
        return result;
    }
    // Only the exact cases can be rounded exactly
    assert_ne!(rm, Exact, "Inexact sin_with_period");
    let mut w = prec + prec.ceiling_log_base_2() + 8;
    let mut increment = Limb::WIDTH;
    let scaled = exp_q <= SCALED_INPUT_EXPONENT;
    let qs;
    let q_scaled = if scaled {
        qs = q << SCALE;
        &qs
    } else {
        q
    };
    loop {
        // t = 2*pi*q * (1 + theta)^3 where |theta| <= 2^-w, from rounding q, pi, and the product
        let mut t = Float::pi_prec(w).0 << 1u32;
        t.mul_prec_assign(Float::from_rational_prec_ref(q_scaled, w).0, w);
        if scaled {
            if let Some(result) = scaled_underflow(&t, *q > 0u32, prec, rm) {
                return result;
            }
            t >>= SCALE;
        }
        // since w >= 2, |(1 + theta)^3 - 1| <= 4*theta <= 2^(2-w), and |sin(t) - sin(2 pi q)| <=
        // |s| <= 2^(EXP(t) + 2 - w)
        let exp_t = i64::from(t.get_exponent().unwrap());
        let w_i = i64::exact_from(w);
        let mut err = exp_t + 2 - w_i;
        t.sin_prec_round_assign(w, Up);
        let exp_t = i64::from(t.get_exponent().unwrap());
        // a tiny sine with q not itself tiny means q is close to a multiple of 1/2
        if exp_t < 0 && exp_q >= -2 {
            let cancel = u64::exact_from(-exp_t);
            if cancel >= max(NEAR_ZERO_MIN_CANCEL, prec >> 4)
                && let Some(result) = trig_turns_near_zero(q, prec, rm, false)
            {
                return result;
            }
        }
        // the total error is at most 2^err + ulp(t), bounded by 2^(EXP(t)-w+1) if err <= EXP(t)-w
        // and by 2^(err+1) otherwise; then normalized for can_round
        err = if err <= exp_t - w_i {
            exp_t - w_i + 1
        } else {
            err + 1
        };
        err = exp_t - err;
        if err > 0 && float_can_round(t.significand_ref().unwrap(), u64::exact_from(err), prec, rm)
        {
            return Float::from_float_prec_round(t, prec, rm);
        }
        w += increment;
        increment = w >> 1;
    }
}

// This is mpfr_sin from sin.c, MPFR 4.2.2, without the `mpfr_sin_fast` tier for precisions at or
// above `MPFR_SINCOS_THRESHOLD`, which depends on `mpfr_sincos_fast`.
fn sin_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact sin");
    let exp_x = i64::from(x.get_exponent().unwrap());
    let err1 = -(exp_x << 1);
    // sin(x) = x - x^3/6 + ... so the error is < 2^(3*EXP(x)-2)
    //
    // MPFR_FAST_COMPUTE_IF_SMALL_INPUT (y, x, err1, 2, 0, rnd_mode, {});
    if err1 > 0 {
        let err = u64::exact_from(err1) + 2;
        if err > prec + 1 {
            // The error bound only has to clear prec + 1; passing an enormous err (a tiny x has one
            // around 2^31) would make float_round_near_x do work proportional to it. This can fail
            // to round, for instance for a power of 2 stored at a precision above the error bound,
            // whose bits within the error window are all zero; the general algorithm then takes
            // over, as in MPFR.
            if let Some(result) = float_round_near_x(x, min(err, prec + 2), false, prec, rm) {
                return result;
            }
        }
    }
    // Compute initial precision. For x large, since argument reduction is expensive, we want to
    // avoid any failure in Ziv's strategy, thus we take into account expx too.
    let mut m = prec + max(prec, u64::try_from(exp_x).unwrap_or(0)).ceiling_log_base_2() + 8;
    // since we compute sin(x) as sqrt(1-cos(x)^2), and for x small we have cos(x)^2 ~ 1 - x^2, when
    // subtracting cos(x)^2 from 1 we will lose about -2*expx bits if expx < 0
    if exp_x < 0 {
        m += u64::exact_from(err1);
    }
    // MPFR reduces every |x| >= 2, noting that for 2 <= |x| < pi it could avoid the reduction. For
    // 2 <= |x| < 3, sin(x) has the sign of x and the cosine handles |x| < 4 unreduced, so the
    // reduction (a pi computation and a remainder) is skipped.
    let reduce = exp_x > 2 || (exp_x == 2 && x.ge_abs(&3u32));
    let mut increment = Limb::WIDTH;
    let c = loop {
        match sin_ziv_step(x, exp_x, prec, rm, reduce, &mut m) {
            TrigStep::Done(c) => break c,
            TrigStep::NearZero(cancel) => return trig_near_zero(x, prec, rm, cancel, false),
            TrigStep::Retry => {}
        }
        // ziv_next: Else generic increase
        m += increment;
        increment = m >> 1;
    };
    // inexact cannot be 0, since this would mean that c was representable within the target
    // precision, but in that case mpfr_can_round will fail
    Float::from_float_prec_round(c, prec, rm)
}

impl Float {
    /// Computes $\sin x$, the sine of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded sine is less than, equal to, or greater than
    /// the exact sine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes
    /// more than $2^{30}$ bits of precision, or an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::sin_round`] instead. If both of these things are true, consider using
    /// [`Float::sin`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, from which the sine is derived, costs
    /// the first term, and for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input. Unlike most functions,
    /// `sin` therefore gets slower as the magnitude of its input grows, not just as the precision
    /// does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.84147072");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.84147167");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .sin_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.84147072");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sin_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.sin_prec_round_ref(prec, rm)
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded sine is less than, equal to, or greater
    /// than the exact sine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=\pm0.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes
    /// more than $2^{30}$ bits of precision, or an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sin_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).sin()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, from which the sine is derived, costs
    /// the first term, and for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input. Unlike most functions,
    /// `sin` therefore gets slower as the magnitude of its input grows, not just as the precision
    /// does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.84147072");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.84147167");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.84147072");
    /// assert_eq!(o, Less);
    /// ```
    pub fn sin_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // sin(+0) = +0, sin(-0) = -0
            Zero { .. } => (self.clone(), Equal),
            Finite { .. } => sin_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded sine is less than, equal to, or greater than the exact sine.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the sine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes
    /// more than $2^{30}$ bits of precision, or an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::sin`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, from which the sine is derived, costs
    /// the first term, and for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input. Unlike most functions,
    /// `sin` therefore gets slower as the magnitude of its input grows, not just as the precision
    /// does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sin_prec(5);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sin_prec(20);
    /// assert_eq!(c.to_string(), "0.84147072");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sin_prec(self, prec: u64) -> (Self, Ordering) {
        self.sin_prec_round(prec, Nearest)
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded sine is less than, equal to, or greater than the exact sine.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the sine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes
    /// more than $2^{30}$ bits of precision, or an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).sin()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, from which the sine is derived, costs
    /// the first term, and for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input. Unlike most functions,
    /// `sin` therefore gets slower as the magnitude of its input grows, not just as the precision
    /// does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.84147072");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sin_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.sin_prec_round_ref(prec, Nearest)
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded sine is less than, equal to, or greater than the exact sine. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes
    /// more than $2^{30}$ bits of precision, or an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you want to specify an output precision, consider using [`Float::sin_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sin`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `sin`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sin_round(Floor);
    /// assert_eq!(c.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sin_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.84147098480789650665250232163084");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.sin_round(Nearest);
    /// assert_eq!(c.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sin_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.sin_prec_round(prec, rm)
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded sine is less than, equal to, or greater than the exact sine. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\pm\infty,m)=\text{NaN}$
    /// - $f(\pm0.0,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// Underflow requires an input within $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes
    /// more than $2^{30}$ bits of precision, or an input of magnitude $2^{-2^{30}}$, the smallest
    /// positive [`Float`], rounded toward zero.
    ///
    /// If you want to specify an output precision, consider using [`Float::sin_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).sin()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `sin`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.84147098480789650665250232163084");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).sin_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.84147098480789650665250232163005");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sin_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.sin_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded sine is less than, equal to, or
    /// greater than the exact sine. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sin_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_prec_assign`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::sin_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::sin_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, from which the sine is derived, costs
    /// the first term, and for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input. Unlike most functions,
    /// `sin` therefore gets slower as the magnitude of its input grows, not just as the precision
    /// does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.812");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.844");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_round_assign(5, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.844");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.84147072");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.84147167");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.84147072");
    /// ```
    #[inline]
    pub fn sin_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.sin_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The [`Float`] is replaced by the result, and an [`Ordering`] is
    /// returned, indicating whether the rounded sine is less than, equal to, or greater than the
    /// exact sine. Although `NaN`s are not comparable to any [`Float`], whenever this function sets
    /// a `NaN` it also returns `Equal`.
    ///
    /// If the sine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sin_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::sin_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the cosine at working precision $n$, from which the sine is derived, costs
    /// the first term, and for $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires
    /// $\pi$ to about $n + e$ bits and a remainder of the $m$-bit input. Unlike most functions,
    /// `sin` therefore gets slower as the magnitude of its input grows, not just as the precision
    /// does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_assign(5), Greater);
    /// assert_eq!(x.to_string(), "0.844");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.84147072");
    /// ```
    #[inline]
    pub fn sin_prec_assign(&mut self, prec: u64) -> Ordering {
        self.sin_prec_round_assign(prec, Nearest)
    }

    /// Computes $\sin x$, the sine of a [`Float`], rounding the result with the specified rounding
    /// mode. The [`Float`] is replaced by the result, and an [`Ordering`] is returned, indicating
    /// whether the rounded sine is less than, equal to, or greater than the exact sine. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function sets a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::sin_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::sin_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::sin_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `sin`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the sine of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.84147098480789650665250232163005");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.84147098480789650665250232163084");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_round_assign(Nearest), Less);
    /// assert_eq!(x.to_string(), "0.84147098480789650665250232163005");
    /// ```
    #[inline]
    pub fn sin_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.sin_prec_round_assign(prec, rm)
    }
}

impl Float {
    /// Computes $\sin x$, the sine of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded sine is less than, equal to, or greater than the exact sine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sin x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires an input of magnitude about $2^{-2^{30}}$ or less, or one within
    /// $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes more than $2^{30}$ bits.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] sine taken there, which for $|x| \geq 3$
    /// reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n + e$ bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::sin_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sin_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::sin_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "0.56464195");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sin_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.56464291");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::sin_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\sin x$, the sine of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded sine is less than, equal to, or greater than the exact sine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \sin x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=0$.
    ///
    /// See the [`Float::sin_rational_prec_round`] documentation for information on overflow and
    /// underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] sine taken there, which for $|x| \geq 3$
    /// reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n + e$ bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case for every nonzero input).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) =
    ///     Float::sin_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::sin_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.594");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::sin_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "0.56464195");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::sin_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.56464291");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn sin_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // sin(0) = 0, exactly
            return (Self::ZERO, Equal);
        }
        sin_rational_helper(x, prec, rm)
    }

    /// Computes $\sin x$, the sine of a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded sine is less
    /// than, equal to, or greater than the exact sine.
    ///
    /// If the sine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sin x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin x|\rfloor-p}$ (unless the result
    /// underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0$.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin x|\leq 1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an input of magnitude about $2^{-2^{30}}$ or less, or one within
    /// $2^{-2^{30}}$ of a nonzero multiple of $\pi$, which takes more than $2^{30}$ bits.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] sine taken there, which for $|x| \geq 3$
    /// reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n + e$ bits.
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
    /// let (c, o) = Float::sin_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sin_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "0.56464291");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::sin_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\sin x$, the sine of a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded sine is less
    /// than, equal to, or greater than the exact sine.
    ///
    /// If the sine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \sin x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin x|\rfloor-p}$ (unless the result
    /// underflows).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=0$.
    ///
    /// See the [`Float::sin_rational_prec`] documentation for information on overflow and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] sine taken there, which for $|x| \geq 3$
    /// reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n + e$ bits.
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
    /// let (c, o) = Float::sin_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sin_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "0.56464291");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sin_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::sin_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Float {
    /// Computes $\sin(2\pi x/u)$, the sine of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded sine is
    /// less than, equal to, or greater than the exact sine. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \sin(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $x$ is not finite or $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p+1}$.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(\pm0.0,u,p,m)=\pm0.0$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $0.0$ with the sign of $x$
    ///   (following IEEE 754-2019's `sinPi`, so that the function is odd); if it is an odd multiple
    ///   of $1/4$, the result is exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo
    ///   $1$, the result is exactly $1/2$ or $-1/2$.
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 20, the result is $\pm\sqrt3/2$,
    /// $\pm\sqrt2/2$, $\pm\varphi/2$, or $\pm(\varphi-1)/2$, and is computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine, which is far faster.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$, the result never overflows.
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,u,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,u,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,u,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,u,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,u,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires $x/u$ within $2^{-2^{30}}$ of a multiple of $1/2$ without being one,
    /// which takes more than $2^{30}$ bits of precision, or an $x$ so small that $2\pi x/u$ is
    /// below $2^{-2^{30}}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_with_period_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::sin_with_period_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the argument is reduced modulo $u$ exactly, and the sine of $2\pi x/u$ is
    /// then taken at a working precision of about $n + e$ bits, which needs $\pi$ to that many
    /// bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or is
    /// $\pm1/12$ or $\pm5/12$ modulo $1$, or $x$ is zero or not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::ONE.sin_with_period_prec_round(7, 10, Floor);
    /// assert_eq!(c.to_string(), "0.78125");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::ONE.sin_with_period_prec_round(7, 10, Ceiling);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::ONE.sin_with_period_prec_round(7, 10, Nearest);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// // a twelfth of a turn is exact
    /// let (c, o) = Float::from(30u32).sin_with_period_prec_round(360, 10, Exact);
    /// assert_eq!(c.to_string(), "0.50000");
    /// assert_eq!(o, Equal);
    ///
    /// // a half turn is exactly zero
    /// let (c, o) = Float::from(180u32).sin_with_period_prec_round(360, 10, Nearest);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn sin_with_period_prec_round(
        self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.sin_with_period_prec_round_ref(u, prec, rm)
    }

    /// Computes $\sin(2\pi x/u)$, the sine of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded sine is
    /// less than, equal to, or greater than the exact sine. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \sin(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $x$ is not finite or $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p+1}$.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(\pm\infty,u,p,m)=\text{NaN}$
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(\pm0.0,u,p,m)=\pm0.0$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $0.0$ with the sign of $x$
    ///   (following IEEE 754-2019's `sinPi`, so that the function is odd); if it is an odd multiple
    ///   of $1/4$, the result is exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo
    ///   $1$, the result is exactly $1/2$ or $-1/2$.
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 20, the result is $\pm\sqrt3/2$,
    /// $\pm\sqrt2/2$, $\pm\varphi/2$, or $\pm(\varphi-1)/2$, and is computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine, which is far faster.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$, the result never overflows.
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,u,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,u,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,u,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,u,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,u,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires $x/u$ within $2^{-2^{30}}$ of a multiple of $1/2$ without being one,
    /// which takes more than $2^{30}$ bits of precision, or an $x$ so small that $2\pi x/u$ is
    /// below $2^{-2^{30}}$.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_with_period_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::sin_with_period_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the argument is reduced modulo $u$ exactly, and the sine of $2\pi x/u$ is
    /// then taken at a working precision of about $n + e$ bits, which needs $\pi$ to that many
    /// bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or is
    /// $\pm1/12$ or $\pm5/12$ modulo $1$, or $x$ is zero or not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::ONE).sin_with_period_prec_round_ref(7, 10, Floor);
    /// assert_eq!(c.to_string(), "0.78125");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::ONE).sin_with_period_prec_round_ref(7, 10, Ceiling);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::ONE).sin_with_period_prec_round_ref(7, 10, Nearest);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// // a twelfth of a turn is exact
    /// let (c, o) = (&Float::from(30u32)).sin_with_period_prec_round_ref(360, 10, Exact);
    /// assert_eq!(c.to_string(), "0.50000");
    /// assert_eq!(o, Equal);
    ///
    /// // a half turn is exactly zero
    /// let (c, o) = (&Float::from(180u32)).sin_with_period_prec_round_ref(360, 10, Nearest);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn sin_with_period_prec_round_ref(
        &self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            // for u=0, return NaN
            _ if u == 0 => (Self::NAN, Equal),
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // x is zero: sin(±0) = ±0
            Zero { .. } => (self.clone(), Equal),
            Finite { .. } => sin_with_period_prec_round_normal_ref(self, u, prec, rm),
        }
    }

    /// Computes $\sin(2\pi x/u)$, the sine of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded sine is less than, equal
    /// to, or greater than the exact sine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,u,p) = \sin(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $x$ is not finite or $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $u\neq 0$, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin(2\pi
    ///   x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=\text{NaN}$
    /// - $f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,u,p)=\pm0.0$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $0.0$ with the sign of $x$
    ///   (following IEEE 754-2019's `sinPi`, so that the function is odd); if it is an odd multiple
    ///   of $1/4$, the result is exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo
    ///   $1$, the result is exactly $1/2$ or $-1/2$.
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 20, the result is $\pm\sqrt3/2$,
    /// $\pm\sqrt2/2$, $\pm\varphi/2$, or $\pm(\varphi-1)/2$, and is computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine, which is far faster.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$, the result never overflows.
    /// - If $0<f(x,u,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,u,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,u,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires $x/u$ within $2^{-2^{30}}$ of a multiple of $1/2$ without being one,
    /// which takes more than $2^{30}$ bits of precision, or an $x$ so small that $2\pi x/u$ is
    /// below $2^{-2^{30}}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_with_period_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::sin_with_period_round`] with `Nearest`
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the argument is reduced modulo $u$ exactly, and the sine of $2\pi x/u$ is
    /// then taken at a working precision of about $n + e$ bits, which needs $\pi$ to that many
    /// bits.
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
    /// let (c, o) = Float::ONE.sin_with_period_prec(7, 10);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::ONE.sin_with_period_prec(360, 53);
    /// assert_eq!(c.to_string(), "0.017452406437283512");
    /// assert_eq!(o, Less);
    ///
    /// // an eighth of a turn: sqrt(2)/2
    /// let (c, o) = Float::ONE.sin_with_period_prec(8, 10);
    /// assert_eq!(c.to_string(), "0.70703");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sin_with_period_prec(self, u: u64, prec: u64) -> (Self, Ordering) {
        self.sin_with_period_prec_round(u, prec, Nearest)
    }

    /// Computes $\sin(2\pi x/u)$, the sine of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded sine is less
    /// than, equal to, or greater than the exact sine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the sine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,u,p) = \sin(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $x$ is not finite or $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $u\neq 0$, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin(2\pi
    ///   x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,p)=\text{NaN}$
    /// - $f(\pm\infty,u,p)=\text{NaN}$
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(\pm0.0,u,p)=\pm0.0$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $0.0$ with the sign of $x$
    ///   (following IEEE 754-2019's `sinPi`, so that the function is odd); if it is an odd multiple
    ///   of $1/4$, the result is exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo
    ///   $1$, the result is exactly $1/2$ or $-1/2$.
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 20, the result is $\pm\sqrt3/2$,
    /// $\pm\sqrt2/2$, $\pm\varphi/2$, or $\pm(\varphi-1)/2$, and is computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine, which is far faster.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$, the result never overflows.
    /// - If $0<f(x,u,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,u,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,u,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires $x/u$ within $2^{-2^{30}}$ of a multiple of $1/2$ without being one,
    /// which takes more than $2^{30}$ bits of precision, or an $x$ so small that $2\pi x/u$ is
    /// below $2^{-2^{30}}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_with_period_prec_round_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using [`Float::sin_with_period_round_ref`] with
    /// `Nearest` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the argument is reduced modulo $u$ exactly, and the sine of $2\pi x/u$ is
    /// then taken at a working precision of about $n + e$ bits, which needs $\pi$ to that many
    /// bits.
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
    /// let (c, o) = (&Float::ONE).sin_with_period_prec_ref(7, 10);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::ONE).sin_with_period_prec_ref(360, 53);
    /// assert_eq!(c.to_string(), "0.017452406437283512");
    /// assert_eq!(o, Less);
    ///
    /// // an eighth of a turn: sqrt(2)/2
    /// let (c, o) = (&Float::ONE).sin_with_period_prec_ref(8, 10);
    /// assert_eq!(c.to_string(), "0.70703");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sin_with_period_prec_ref(&self, u: u64, prec: u64) -> (Self, Ordering) {
        self.sin_with_period_prec_round_ref(u, prec, Nearest)
    }

    /// Computes $\sin(2\pi x/u)$, the sine of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sine is less than, equal to,
    /// or greater than the exact sine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,m) = \sin(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $x$ is not finite or $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,m)=\text{NaN}$
    /// - $f(\pm\infty,u,m)=\text{NaN}$
    /// - $f(x,0,m)=\text{NaN}$
    /// - $f(\pm0.0,u,m)=\pm0.0$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $0.0$ with the sign of $x$
    ///   (following IEEE 754-2019's `sinPi`, so that the function is odd); if it is an odd multiple
    ///   of $1/4$, the result is exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo
    ///   $1$, the result is exactly $1/2$ or $-1/2$.
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 20, the result is $\pm\sqrt3/2$,
    /// $\pm\sqrt2/2$, $\pm\varphi/2$, or $\pm(\varphi-1)/2$, and is computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine, which is far faster.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$, the result never overflows.
    /// - If $0<f(x,u,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,u,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,u,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,u,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,u,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,u,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,u,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires $x/u$ within $2^{-2^{30}}$ of a multiple of $1/2$ without being one,
    /// which takes more than $2^{30}$ bits of precision, or an $x$ so small that $2\pi x/u$ is
    /// below $2^{-2^{30}}$.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sin_with_period_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::sin_with_period_prec`] with the input's precision
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the argument is
    /// reduced modulo $u$ exactly, and the sine of $2\pi x/u$ is then taken at a working precision
    /// of about $n + e$ bits, which needs $\pi$ to that many bits.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (which is the case unless $x/u$ is a multiple of $1/4$ or $1/6$, or $x$ is zero or
    /// not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 10)
    ///     .0
    ///     .sin_with_period_round(7, Floor);
    /// assert_eq!(c.to_string(), "0.78125");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 10)
    ///     .0
    ///     .sin_with_period_round(7, Ceiling);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 10)
    ///     .0
    ///     .sin_with_period_round(7, Nearest);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sin_with_period_round(self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.sin_with_period_prec_round(u, prec, rm)
    }

    /// Computes $\sin(2\pi x/u)$, the sine of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result with the specified rounding mode. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sine is less than, equal to,
    /// or greater than the exact sine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,m) = \sin(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $x$ is not finite or $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},u,m)=\text{NaN}$
    /// - $f(\pm\infty,u,m)=\text{NaN}$
    /// - $f(x,0,m)=\text{NaN}$
    /// - $f(\pm0.0,u,m)=\pm0.0$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $0.0$ with the sign of $x$
    ///   (following IEEE 754-2019's `sinPi`, so that the function is odd); if it is an odd multiple
    ///   of $1/4$, the result is exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo
    ///   $1$, the result is exactly $1/2$ or $-1/2$.
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 20, the result is $\pm\sqrt3/2$,
    /// $\pm\sqrt2/2$, $\pm\varphi/2$, or $\pm(\varphi-1)/2$, and is computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine, which is far faster.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$, the result never overflows.
    /// - If $0<f(x,u,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,u,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,u,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,u,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,u,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,u,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,u,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires $x/u$ within $2^{-2^{30}}$ of a multiple of $1/2$ without being one,
    /// which takes more than $2^{30}$ bits of precision, or an $x$ so small that $2\pi x/u$ is
    /// below $2^{-2^{30}}$.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sin_with_period_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::sin_with_period_prec_ref`] with the input's precision
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the argument is
    /// reduced modulo $u$ exactly, and the sine of $2\pi x/u$ is then taken at a working precision
    /// of about $n + e$ bits, which needs $\pi$ to that many bits.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (which is the case unless $x/u$ is a multiple of $1/4$ or $1/6$, or $x$ is zero or
    /// not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 10).0).sin_with_period_round_ref(7, Floor);
    /// assert_eq!(c.to_string(), "0.78125");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 10).0).sin_with_period_round_ref(7, Ceiling);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 10).0).sin_with_period_round_ref(7, Nearest);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sin_with_period_round_ref(&self, u: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.sin_with_period_prec_round_ref(u, self.significant_bits(), rm)
    }

    /// Computes $\sin(2\pi x/u)$, the sine of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result to the specified precision and with the specified rounding mode. The [`Float`] is
    /// replaced by the result, and an [`Ordering`] is returned, indicating whether the rounded sine
    /// is less than, equal to, or greater than the exact sine. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sin(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $x$ is not finite or $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p+1}$.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sin_with_period_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::sin_with_period_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::sin_with_period_round_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the argument is reduced modulo $u$ exactly, and the sine of $2\pi x/u$ is
    /// then taken at a working precision of about $n + e$ bits, which needs $\pi$ to that many
    /// bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or is
    /// $\pm1/12$ or $\pm5/12$ modulo $1$, or $x$ is zero or not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_with_period_prec_round_assign(7, 10, Floor), Less);
    /// assert_eq!(x.to_string(), "0.78125");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_with_period_prec_round_assign(7, 10, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.78223");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_with_period_prec_round_assign(7, 10, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.78223");
    /// ```
    #[inline]
    pub fn sin_with_period_prec_round_assign(
        &mut self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let o;
        (*self, o) = self.sin_with_period_prec_round_ref(u, prec, rm);
        o
    }

    /// Computes $\sin(2\pi x/u)$, the sine of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result to the nearest value of the specified precision. The [`Float`] is replaced by the
    /// result, and an [`Ordering`] is returned, indicating whether the rounded sine is less than,
    /// equal to, or greater than the exact sine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// If the sine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \sin(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $x$ is not finite or $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $u\neq 0$, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin(2\pi
    ///   x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::sin_with_period_prec`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_with_period_prec_round_assign`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::sin_with_period_round_assign`] with
    /// `Nearest` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the argument is reduced modulo $u$ exactly, and the sine of $2\pi x/u$ is
    /// then taken at a working precision of about $n + e$ bits, which needs $\pi$ to that many
    /// bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_with_period_prec_assign(7, 10), Greater);
    /// assert_eq!(x.to_string(), "0.78223");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.sin_with_period_prec_assign(8, 10), Less);
    /// assert_eq!(x.to_string(), "0.70703");
    /// ```
    #[inline]
    pub fn sin_with_period_prec_assign(&mut self, u: u64, prec: u64) -> Ordering {
        self.sin_with_period_prec_round_assign(u, prec, Nearest)
    }

    /// Computes $\sin(2\pi x/u)$, the sine of a [`Float`] measured in $u$ths of a turn, rounding
    /// the result with the specified rounding mode. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded sine is less than, equal to, or
    /// greater than the exact sine. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \sin(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $x$ is not finite or $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite, $u\neq 0$, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::sin_with_period_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::sin_with_period_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::sin_with_period_prec_assign`] with the
    /// input's precision instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the argument is
    /// reduced modulo $u$ exactly, and the sine of $2\pi x/u$ is then taken at a working precision
    /// of about $n + e$ bits, which needs $\pi$ to that many bits.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision (which is the case unless $x/u$ is a multiple of $1/4$ or $1/6$, or $x$ is zero or
    /// not finite, or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0;
    /// assert_eq!(x.sin_with_period_round_assign(7, Floor), Less);
    /// assert_eq!(x.to_string(), "0.78125");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0;
    /// assert_eq!(x.sin_with_period_round_assign(7, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.78223");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 10).0;
    /// assert_eq!(x.sin_with_period_round_assign(7, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.78223");
    /// ```
    #[inline]
    pub fn sin_with_period_round_assign(&mut self, u: u64, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.sin_with_period_prec_round_assign(u, prec, rm)
    }
}

impl Float {
    /// Computes $\sin(2\pi x/u)$, the sine of a [`Rational`] measured in $u$ths of a turn, rounding
    /// the result to the specified precision and with the specified rounding mode, and returning
    /// the result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded sine is less than, equal to, or greater than the
    /// exact sine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \sin(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $u\neq 0$ and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin(2\pi
    ///   x/u)|\rfloor-p+1}$.
    /// - If $u\neq 0$ and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin(2\pi
    ///   x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=0$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $0.0$ with the sign of $x$
    ///   (following IEEE 754-2019's `sinPi`, so that the function is odd); if it is an odd multiple
    ///   of $1/4$, the result is exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo
    ///   $1$, the result is exactly $1/2$ or $-1/2$.
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 20, the result is $\pm\sqrt3/2$,
    /// $\pm\sqrt2/2$, $\pm\varphi/2$, or $\pm(\varphi-1)/2$, and is computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine, which is far faster.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$, the result never overflows.
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,u,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,u,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,u,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,u,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,u,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires $x/u$ within $2^{-2^{30}}$ of a multiple of $1/2$ without being one,
    /// which takes a denominator of more than $2^{30}$ bits, or an $x/u$ so small that $2\pi x/u$
    /// is below $2^{-2^{30}}$.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::sin_with_period_rational_prec`] instead.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or is
    /// $\pm1/12$ or $\pm5/12$ modulo $1$, or $x$ or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::sin_with_period_rational_prec_round(Rational::ONE, 7, 10, Floor);
    /// assert_eq!(c.to_string(), "0.78125");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sin_with_period_rational_prec_round(Rational::ONE, 7, 10, Ceiling);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::sin_with_period_rational_prec_round(Rational::ONE, 7, 10, Nearest);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// // a twelfth of a turn is exact
    /// let (c, o) = Float::sin_with_period_rational_prec_round(
    ///     Rational::from_unsigneds(1u8, 12),
    ///     1,
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(c.to_string(), "0.50000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_with_period_rational_prec_round(
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::sin_with_period_rational_prec_round_ref(&x, u, prec, rm)
    }

    /// Computes $\sin(2\pi x/u)$, the sine of a [`Rational`] measured in $u$ths of a turn, rounding
    /// the result to the specified precision and with the specified rounding mode, and returning
    /// the result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded sine is less than, equal to, or greater than the
    /// exact sine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,u,p,m) = \sin(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $u\neq 0$ and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin(2\pi
    ///   x/u)|\rfloor-p+1}$.
    /// - If $u\neq 0$ and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\sin(2\pi
    ///   x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,0,p,m)=\text{NaN}$
    /// - $f(0,u,p,m)=0$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $0.0$ with the sign of $x$
    ///   (following IEEE 754-2019's `sinPi`, so that the function is odd); if it is an odd multiple
    ///   of $1/4$, the result is exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo
    ///   $1$, the result is exactly $1/2$ or $-1/2$.
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 20, the result is $\pm\sqrt3/2$,
    /// $\pm\sqrt2/2$, $\pm\varphi/2$, or $\pm(\varphi-1)/2$, and is computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine, which is far faster.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$, the result never overflows.
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,u,p,m)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,u,p,m)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,u,p,m)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,u,p,m)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,u,p,m)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is
    ///   returned instead.
    ///
    /// Underflow requires $x/u$ within $2^{-2^{30}}$ of a multiple of $1/2$ without being one,
    /// which takes a denominator of more than $2^{30}$ bits, or an $x/u$ so small that $2\pi x/u$
    /// is below $2^{-2^{30}}$.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::sin_with_period_rational_prec_ref`] instead.
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
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless $x/u$ is a multiple of $1/4$, or is
    /// $\pm1/12$ or $\pm5/12$ modulo $1$, or $x$ or $u$ is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::sin_with_period_rational_prec_round_ref(&Rational::ONE, 7, 10, Floor);
    /// assert_eq!(c.to_string(), "0.78125");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::sin_with_period_rational_prec_round_ref(&Rational::ONE, 7, 10, Ceiling);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::sin_with_period_rational_prec_round_ref(&Rational::ONE, 7, 10, Nearest);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// // a twelfth of a turn is exact
    /// let (c, o) = Float::sin_with_period_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(1u8, 12),
    ///     1,
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(c.to_string(), "0.50000");
    /// assert_eq!(o, Equal);
    /// ```
    pub fn sin_with_period_rational_prec_round_ref(
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        // for u = 0, return NaN
        if u == 0 {
            return (Self::NAN, Equal);
        }
        // sin(0) = 0 (a `Rational` zero has no sign)
        if *x == 0u32 {
            return (Self::ZERO, Equal);
        }
        // q = x/u, reduced to (-1, 1) with the sign of x: sin(2 pi q) has period 1 in q, and a
        // multiple of u gives a zero with the sign of x (IEEE 754-2019's sinPi)
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
                Equal,
            );
        }
        sin_turns_helper(&q, prec, rm)
    }

    /// Computes $\sin(2\pi x/u)$, the sine of a [`Rational`] measured in $u$ths of a turn, rounding
    /// the result to the nearest value of the specified precision, and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded sine is less than, equal to, or greater than the exact sine. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// If the sine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,u,p) = \sin(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $u\neq 0$, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(0,u,p)=0$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $0.0$ with the sign of $x$
    ///   (following IEEE 754-2019's `sinPi`, so that the function is odd); if it is an odd multiple
    ///   of $1/4$, the result is exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo
    ///   $1$, the result is exactly $1/2$ or $-1/2$.
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 20, the result is $\pm\sqrt3/2$,
    /// $\pm\sqrt2/2$, $\pm\varphi/2$, or $\pm(\varphi-1)/2$, and is computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine, which is far faster.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$, the result never overflows.
    /// - If $0<f(x,u,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,u,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,u,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires $x/u$ within $2^{-2^{30}}$ of a multiple of $1/2$ without being one,
    /// which takes a denominator of more than $2^{30}$ bits, or an $x/u$ so small that $2\pi x/u$
    /// is below $2^{-2^{30}}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_with_period_rational_prec_round`] instead.
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
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::sin_with_period_rational_prec(Rational::ONE, 7, 10);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::sin_with_period_rational_prec(Rational::ONE, 7, 53);
    /// assert_eq!(c.to_string(), "0.78183148246802980");
    /// assert_eq!(o, Less);
    ///
    /// // an eighth of a turn: sqrt(2)/2
    /// let (c, o) = Float::sin_with_period_rational_prec(Rational::from_unsigneds(1u8, 8), 1, 53);
    /// assert_eq!(c.to_string(), "0.70710678118654757");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_with_period_rational_prec(x: Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::sin_with_period_rational_prec_round_ref(&x, u, prec, Nearest)
    }

    /// Computes $\sin(2\pi x/u)$, the sine of a [`Rational`] measured in $u$ths of a turn, rounding
    /// the result to the nearest value of the specified precision, and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded sine is less than, equal to, or greater than the exact sine.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// If the sine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,u,p) = \sin(2\pi x/u)+\varepsilon.
    /// $$
    /// - If $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $u\neq 0$, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(x,0,p)=\text{NaN}$
    /// - $f(0,u,p)=0$
    /// - If $x/u$ is a multiple of $1/2$, the result is exactly $0.0$ with the sign of $x$
    ///   (following IEEE 754-2019's `sinPi`, so that the function is odd); if it is an odd multiple
    ///   of $1/4$, the result is exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo
    ///   $1$, the result is exactly $1/2$ or $-1/2$.
    ///
    /// When $x/u$ in lowest terms has denominator 3, 6, 8, or 20, the result is $\pm\sqrt3/2$,
    /// $\pm\sqrt2/2$, $\pm\varphi/2$, or $\pm(\varphi-1)/2$, and is computed from a single
    /// correctly rounded constant rather than from $\pi$ and a sine, which is far faster.
    ///
    /// Overflow and underflow:
    /// - Since $|\sin(2\pi x/u)|\leq 1$, the result never overflows.
    /// - If $0<f(x,u,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,u,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,u,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,u,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires $x/u$ within $2^{-2^{30}}$ of a multiple of $1/2$ without being one,
    /// which takes a denominator of more than $2^{30}$ bits, or an $x/u$ so small that $2\pi x/u$
    /// is below $2^{-2^{30}}$.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_with_period_rational_prec_round_ref`] instead.
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
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::sin_with_period_rational_prec_ref(&Rational::ONE, 7, 10);
    /// assert_eq!(c.to_string(), "0.78223");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::sin_with_period_rational_prec_ref(&Rational::ONE, 7, 53);
    /// assert_eq!(c.to_string(), "0.78183148246802980");
    /// assert_eq!(o, Less);
    ///
    /// // an eighth of a turn: sqrt(2)/2
    /// let (c, o) =
    ///     Float::sin_with_period_rational_prec_ref(&Rational::from_unsigneds(1u8, 8), 1, 53);
    /// assert_eq!(c.to_string(), "0.70710678118654757");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sin_with_period_rational_prec_ref(x: &Rational, u: u64, prec: u64) -> (Self, Ordering) {
        Self::sin_with_period_rational_prec_round_ref(x, u, prec, Nearest)
    }
}

impl Float {
    /// Computes $\sin(\pi x)$, the sine of a [`Float`] measured in half-turns, rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded sine is less than,
    /// equal to, or greater than the exact sine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `sin_with_period` with a period of 2: see [`Float::sin_with_period_prec_round`] for
    /// the error bounds, the special and closed-form cases (integers give $\pm0.0$ with the sign of
    /// the input, half-integers give $\pm1$, odd multiples of $1/6$ give $\pm1/2$, and multiples of
    /// $1/3$, $1/4$, and $1/10$ have closed forms), overflow and underflow, and the complexity,
    /// with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(0.1f64).sin_pi_prec_round(10, Floor);
    /// assert_eq!(c.to_string(), "0.30859");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from(0.1f64).sin_pi_prec_round(10, Ceiling);
    /// assert_eq!(c.to_string(), "0.30908");
    /// assert_eq!(o, Greater);
    ///
    /// // a half-turn is exactly zero
    /// let (c, o) = Float::ONE.sin_pi_prec_round(10, Exact);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn sin_pi_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.sin_with_period_prec_round(2, prec, rm)
    }

    /// Computes $\sin(\pi x)$, the sine of a [`Float`] measured in half-turns, rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded sine is less
    /// than, equal to, or greater than the exact sine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `sin_with_period` with a period of 2: see [`Float::sin_with_period_prec_round_ref`]
    /// for the error bounds, the special and closed-form cases (integers give $\pm0.0$ with the
    /// sign of the input, half-integers give $\pm1$, odd multiples of $1/6$ give $\pm1/2$, and
    /// multiples of $1/3$, $1/4$, and $1/10$ have closed forms), overflow and underflow, and the
    /// complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::from(0.1f64)).sin_pi_prec_round_ref(10, Floor);
    /// assert_eq!(c.to_string(), "0.30859");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::from(0.1f64)).sin_pi_prec_round_ref(10, Ceiling);
    /// assert_eq!(c.to_string(), "0.30908");
    /// assert_eq!(o, Greater);
    ///
    /// // a half-turn is exactly zero
    /// let (c, o) = (&Float::ONE).sin_pi_prec_round_ref(10, Exact);
    /// assert_eq!(c.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    pub fn sin_pi_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.sin_with_period_prec_round_ref(2, prec, rm)
    }

    /// Computes $\sin(\pi x)$, the sine of a [`Float`] measured in half-turns, rounding the result
    /// to the nearest value of the specified precision. The [`Float`] is taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded sine is less than, equal to,
    /// or greater than the exact sine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `sin_with_period` with a period of 2: see [`Float::sin_with_period_prec`] for the
    /// error bounds, the special and closed-form cases (integers give $\pm1$, half-integers give
    /// $+0.0$, and multiples of $1/3$, $1/4$, $1/5$, $1/6$, and $1/10$ have closed forms), overflow
    /// and underflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(0.1f64).sin_pi_prec(10);
    /// assert_eq!(c.to_string(), "0.30908");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from(0.1f64).sin_pi_prec(53);
    /// assert_eq!(c.to_string(), "0.30901699437494745");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sin_pi_prec(self, prec: u64) -> (Self, Ordering) {
        self.sin_with_period_prec(2, prec)
    }

    /// Computes $\sin(\pi x)$, the sine of a [`Float`] measured in half-turns, rounding the result
    /// to the nearest value of the specified precision. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded sine is less than, equal to,
    /// or greater than the exact sine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `sin_with_period` with a period of 2: see [`Float::sin_with_period_prec_ref`] for
    /// the error bounds, the special and closed-form cases (integers give $\pm0.0$ with the sign of
    /// the input, half-integers give $\pm1$, odd multiples of $1/6$ give $\pm1/2$, and multiples of
    /// $1/3$, $1/4$, and $1/10$ have closed forms), overflow and underflow, and the complexity,
    /// with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::from(0.1f64)).sin_pi_prec_ref(10);
    /// assert_eq!(c.to_string(), "0.30908");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (Float::from(0.1f64)).sin_pi_prec_ref(53);
    /// assert_eq!(c.to_string(), "0.30901699437494745");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sin_pi_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.sin_with_period_prec_ref(2, prec)
    }

    /// Computes $\sin(\pi x)$, the sine of a [`Float`] measured in half-turns, rounding the result
    /// with the specified rounding mode. The precision of the output is the precision of the input.
    /// The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded sine is less than, equal to, or greater than the exact sine. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `sin_with_period` with a period of 2: see [`Float::sin_with_period_round`] for the
    /// error bounds, the special and closed-form cases (integers give $\pm1$, half-integers give
    /// $+0.0$, and multiples of $1/3$, $1/4$, $1/5$, $1/6$, and $1/10$ have closed forms), overflow
    /// and underflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from(0.1f64).sin_pi_round(Floor);
    /// assert_eq!(c.to_string(), "0.30901699437494734");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from(0.1f64).sin_pi_round(Nearest);
    /// assert_eq!(c.to_string(), "0.30901699437494745");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sin_pi_round(self, rm: RoundingMode) -> (Self, Ordering) {
        self.sin_with_period_round(2, rm)
    }

    /// Computes $\sin(\pi x)$, the sine of a [`Float`] measured in half-turns, rounding the result
    /// with the specified rounding mode. The precision of the output is the precision of the input.
    /// The [`Float`] is taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded sine is less than, equal to, or greater than the exact sine. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// This is `sin_with_period` with a period of 2: see [`Float::sin_with_period_round_ref`] for
    /// the error bounds, the special and closed-form cases (integers give $\pm0.0$ with the sign of
    /// the input, half-integers give $\pm1$, odd multiples of $1/6$ give $\pm1/2$, and multiples of
    /// $1/3$, $1/4$, and $1/10$ have closed forms), overflow and underflow, and the complexity,
    /// with $u = 2$.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (Float::from(0.1f64)).sin_pi_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.30901699437494734");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (Float::from(0.1f64)).sin_pi_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.30901699437494745");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sin_pi_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.sin_with_period_round_ref(2, rm)
    }

    /// Computes $\sin(\pi x)$, the sine of a [`Float`] measured in half-turns, rounding the result
    /// to the specified precision and with the specified rounding mode. The [`Float`] is replaced
    /// by the result, and an [`Ordering`] is returned, indicating whether the rounded sine is less
    /// than, equal to, or greater than the exact sine. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// This is `sin_with_period` with a period of 2: see
    /// [`Float::sin_with_period_prec_round_assign`] for the error bounds, the special and
    /// closed-form cases (integers give $\pm1$, half-integers give $+0.0$, and multiples of $1/3$,
    /// $1/4$, $1/5$, $1/6$, and $1/10$ have closed forms), overflow and underflow, and the
    /// complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.1f64);
    /// assert_eq!(x.sin_pi_prec_round_assign(10, Floor), Less);
    /// assert_eq!(x.to_string(), "0.30859");
    ///
    /// let mut x = Float::from(0.1f64);
    /// assert_eq!(x.sin_pi_prec_round_assign(10, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.30908");
    /// ```
    #[inline]
    pub fn sin_pi_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        self.sin_with_period_prec_round_assign(2, prec, rm)
    }

    /// Computes $\sin(\pi x)$, the sine of a [`Float`] measured in half-turns, rounding the result
    /// to the nearest value of the specified precision. The [`Float`] is replaced by the result,
    /// and an [`Ordering`] is returned, indicating whether the rounded sine is less than, equal to,
    /// or greater than the exact sine. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function sets a `NaN` it also returns `Equal`.
    ///
    /// This is `sin_with_period` with a period of 2: see [`Float::sin_with_period_prec_assign`] for
    /// the error bounds, the special and closed-form cases (integers give $\pm0.0$ with the sign of
    /// the input, half-integers give $\pm1$, odd multiples of $1/6$ give $\pm1/2$, and multiples of
    /// $1/3$, $1/4$, and $1/10$ have closed forms), overflow and underflow, and the complexity,
    /// with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.1f64);
    /// assert_eq!(x.sin_pi_prec_assign(10), Greater);
    /// assert_eq!(x.to_string(), "0.30908");
    /// ```
    #[inline]
    pub fn sin_pi_prec_assign(&mut self, prec: u64) -> Ordering {
        self.sin_with_period_prec_assign(2, prec)
    }

    /// Computes $\sin(\pi x)$, the sine of a [`Float`] measured in half-turns, rounding the result
    /// with the specified rounding mode. The precision of the output is the precision of the input.
    /// The [`Float`] is replaced by the result, and an [`Ordering`] is returned, indicating whether
    /// the rounded sine is less than, equal to, or greater than the exact sine. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function sets a `NaN` it also returns
    /// `Equal`.
    ///
    /// This is `sin_with_period` with a period of 2: see [`Float::sin_with_period_round_assign`]
    /// for the error bounds, the special and closed-form cases (integers give $\pm0.0$ with the
    /// sign of the input, half-integers give $\pm1$, odd multiples of $1/6$ give $\pm1/2$, and
    /// multiples of $1/3$, $1/4$, and $1/10$ have closed forms), overflow and underflow, and the
    /// complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(0.1f64);
    /// assert_eq!(x.sin_pi_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.30901699437494734");
    /// ```
    #[inline]
    pub fn sin_pi_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        self.sin_with_period_round_assign(2, rm)
    }

    /// Computes $\sin(\pi x)$, the sine of a [`Rational`] measured in half-turns, rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded sine is less than, equal to, or greater than the exact sine.
    ///
    /// This is `sin_with_period_rational` with a period of 2: see
    /// [`Float::sin_with_period_rational_prec_round`] for the error bounds, the special and
    /// closed-form cases, overflow and underflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::sin_pi_rational_prec_round(Rational::from_unsigneds(1u8, 7), 10, Floor);
    /// assert_eq!(c.to_string(), "0.43359");
    /// assert_eq!(o, Less);
    ///
    /// // a sixth of a half-turn is exactly 1/2
    /// let (c, o) = Float::sin_pi_rational_prec_round(Rational::from_unsigneds(1u8, 6), 10, Exact);
    /// assert_eq!(c.to_string(), "0.50000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_pi_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::sin_with_period_rational_prec_round_ref(&x, 2, prec, rm)
    }

    /// Computes $\sin(\pi x)$, the sine of a [`Rational`] measured in half-turns, rounding the
    /// result to the specified precision and with the specified rounding mode and returning the
    /// result as a [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded sine is less than, equal to, or greater than the
    /// exact sine.
    ///
    /// This is `sin_with_period_rational` with a period of 2: see
    /// [`Float::sin_with_period_rational_prec_round_ref`] for the error bounds, the special and
    /// closed-form cases, overflow and underflow, and the complexity, with $u = 2$.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) =
    ///     Float::sin_pi_rational_prec_round_ref(&Rational::from_unsigneds(1u8, 7), 10, Ceiling);
    /// assert_eq!(c.to_string(), "0.43408");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn sin_pi_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::sin_with_period_rational_prec_round_ref(x, 2, prec, rm)
    }

    /// Computes $\sin(\pi x)$, the sine of a [`Rational`] measured in half-turns, rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded sine is less than, equal to, or greater than the exact sine.
    ///
    /// This is `sin_with_period_rational` with a period of 2: see
    /// [`Float::sin_with_period_rational_prec`] for the error bounds, the special and closed-form
    /// cases, overflow and underflow, and the complexity, with $u = 2$.
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
    /// let (c, o) = Float::sin_pi_rational_prec(Rational::from_unsigneds(1u8, 7), 53);
    /// assert_eq!(c.to_string(), "0.43388373911755812");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn sin_pi_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::sin_with_period_rational_prec_ref(&x, 2, prec)
    }

    /// Computes $\sin(\pi x)$, the sine of a [`Rational`] measured in half-turns, rounding the
    /// result to the nearest value of the specified precision and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded sine is less than, equal to, or greater than the exact sine.
    ///
    /// This is `sin_with_period_rational` with a period of 2: see
    /// [`Float::sin_with_period_rational_prec_ref`] for the error bounds, the special and
    /// closed-form cases, overflow and underflow, and the complexity, with $u = 2$.
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
    /// let (c, o) = Float::sin_pi_rational_prec_ref(&Rational::from_unsigneds(1u8, 7), 53);
    /// assert_eq!(c.to_string(), "0.43388373911755812");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn sin_pi_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::sin_with_period_rational_prec_ref(x, 2, prec)
    }
}

impl Sin for Float {
    type Output = Self;

    /// Computes $\sin x$, the sine of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the sine is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// See the [`Float::sin_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::sin_round`]
    /// instead. If you want to specify the output precision, consider using [`Float::sin_prec`]. If
    /// you want both of these things, consider using [`Float::sin_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `sin`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sin;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.sin().is_nan());
    /// assert!(Float::INFINITY.sin().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.sin().is_nan());
    /// assert_eq!(Float::ZERO.sin().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.sin().to_string(), "-0.0");
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.sin().to_string(),
    ///     "0.84147098480789650665250232163005"
    /// );
    /// assert_eq!(
    ///     Float::from_unsigned_prec(100u32, 100).0.sin().to_string(),
    ///     "-0.50636564110975879365655761045969"
    /// );
    /// ```
    #[inline]
    fn sin(self) -> Self {
        let prec = self.significant_bits();
        self.sin_prec_round(prec, Nearest).0
    }
}

impl Sin for &Float {
    type Output = Float;

    /// Computes $\sin x$, the sine of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the sine is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x) = \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=\pm0.0$
    ///
    /// See the [`Float::sin_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::sin_prec_ref`]. If you want both of these things, consider using
    /// [`Float::sin_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `sin`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sin;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.sin().is_nan());
    /// assert!(Float::INFINITY.sin().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.sin().is_nan());
    /// assert_eq!(Float::ZERO.sin().to_string(), "0.0");
    /// assert_eq!(Float::NEGATIVE_ZERO.sin().to_string(), "-0.0");
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).sin().to_string(),
    ///     "0.84147098480789650665250232163005"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(100u32, 100).0)
    ///         .sin()
    ///         .to_string(),
    ///     "-0.50636564110975879365655761045969"
    /// );
    /// ```
    #[inline]
    fn sin(self) -> Float {
        self.sin_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl SinAssign for Float {
    /// Computes $\sin x$, the sine of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the sine is equidistant
    /// from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in its binary
    /// expansion is chosen. See [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \sin x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// See the [`Float::sin`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::sin_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::sin_prec_assign`]. If you want both of these things, consider using
    /// [`Float::sin_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `sin`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SinAssign;
    /// use malachite_base::num::basic::traits::*;
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.sin_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.sin_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.sin_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.sin_assign();
    /// assert_eq!(x.to_string(), "0.0");
    ///
    /// let mut x = Float::NEGATIVE_ZERO;
    /// x.sin_assign();
    /// assert_eq!(x.to_string(), "-0.0");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.sin_assign();
    /// assert_eq!(x.to_string(), "0.84147098480789650665250232163005");
    ///
    /// let mut x = Float::from_unsigned_prec(100u32, 100).0;
    /// x.sin_assign();
    /// assert_eq!(x.to_string(), "-0.50636564110975879365655761045969");
    /// ```
    #[inline]
    fn sin_assign(&mut self) {
        let prec = self.significant_bits();
        self.sin_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\sin x$, the sine of a primitive float. Using this function is more accurate than
/// using the default `sin` function or the one provided by `libm`.
///
/// $$
/// f(x) = \sin x+\varepsilon.
/// $$
/// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\text{NaN}$
/// - $f(\pm0.0)=\pm0.0$
///
/// Overflow is not possible, since the result lies in $[-1, 1]$. The result is subnormal only when
/// $x$ is, and then it is $x$ itself: no [`f32`] or [`f64`] is close enough to a nonzero multiple
/// of $\pi$ for its sine to be subnormal.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin::primitive_float_sin;
///
/// assert!(primitive_float_sin(f32::NAN).is_nan());
/// assert!(primitive_float_sin(f32::INFINITY).is_nan());
/// assert!(primitive_float_sin(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(NiceFloat(primitive_float_sin(0.0f32)), NiceFloat(0.0));
/// assert_eq!(NiceFloat(primitive_float_sin(-0.0f32)), NiceFloat(-0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_sin(1.0f32)),
///     NiceFloat(0.84147096)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sin(1.0f64)),
///     NiceFloat(0.8414709848078965)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::sin_prec, x)
}

/// Computes $\sin x$, the sine of a [`Rational`], returning the result as a primitive float.
///
/// $$
/// f(x) = \sin x+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\sin x|\rfloor-p}$, and $p$ is the precision of the
/// output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(0)=0$
///
/// Overflow is not possible, since the result lies in $[-1, 1]$. The result underflows, to a
/// subnormal or to zero, when $x$ is tiny, since $\sin x$ is then very close to $x$; a [`Rational`]
/// close enough to a nonzero multiple of $\pi$ for its sine to be subnormal would need a
/// denominator of more than 100 bits, in which case the result is still correctly rounded.
///
/// # Worst-case complexity
/// $T(m, e) = O((m+e) (\log (m+e))^2 \log\log (m+e))$
///
/// $M(m, e) = O((m+e) \log (m+e))$
///
/// where $T$ is time, $M$ is additional memory, $m$ is `x.significant_bits()`, and $e$ is
/// `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): for $|x| \geq 3$ the
/// argument is reduced modulo $2\pi$, which needs $\pi$ to about $e$ bits.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin::primitive_float_sin_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_sin_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sin_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.32719469679615226)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sin_rational::<f32>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.3271947)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sin_rational::<f64>(&Rational::from(10000))),
///     NiceFloat(-0.30561438888825215)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::sin_rational_prec_ref, x)
}

/// Computes $\sin(2\pi x/u)$, the sine of a primitive float measured in $u$ths of a turn (so that
/// `u = 360` is degrees).
///
/// $$
/// f(x,u) = \sin(2\pi x/u)+\varepsilon.
/// $$
/// - If $x$ is not finite or $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x$ is finite and $u\neq 0$, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin(2\pi
///   x/u)|\rfloor-p}$, where $p$ is the precision of the output (24 if `T` is a [`f32`] and 53 if
///   `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN},u)=\text{NaN}$
/// - $f(\pm\infty,u)=\text{NaN}$
/// - $f(x,0)=\text{NaN}$
/// - $f(\pm0.0,u)=\pm0.0$
/// - If $x/u$ is a multiple of $1/2$, the result is exactly $0.0$ with the sign of $x$ (following
///   IEEE 754-2019's `sinPi`, so that the function is odd); if it is an odd multiple of $1/4$, the
///   result is exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo $1$, the result is
///   exactly $1/2$ or $-1/2$.
///
/// Overflow is not possible, since the result lies in $[-1, 1]$. The result underflows, to a
/// subnormal or to zero, only when $2\pi x/u$ does, which takes a subnormal $x$ or a large $u$; no
/// [`f32`] or [`f64`] is close enough to a half turn, without being one, for its sine to be
/// subnormal.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin::primitive_float_sin_with_period;
///
/// assert!(primitive_float_sin_with_period(f32::NAN, 360).is_nan());
/// assert!(primitive_float_sin_with_period(f32::INFINITY, 360).is_nan());
/// assert!(primitive_float_sin_with_period(1.0f32, 0).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_sin_with_period(-0.0f32, 360)),
///     NiceFloat(-0.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sin_with_period(90.0f32, 360)),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sin_with_period(30.0f64, 360)),
///     NiceFloat(0.5)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sin_with_period(1.0f32, 7)),
///     NiceFloat(0.7818315)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sin_with_period(1.0f64, 7)),
///     NiceFloat(0.7818314824680298)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin_with_period<T: PrimitiveFloat>(x: T, u: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(|x, prec| Float::sin_with_period_prec(x, u, prec), x)
}

/// Computes $\sin(2\pi x/u)$, the sine of a [`Rational`] measured in $u$ths of a turn (so that `u =
/// 360` is degrees), returning the result as a primitive float.
///
/// $$
/// f(x,u) = \sin(2\pi x/u)+\varepsilon.
/// $$
/// - If $u=0$, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $u\neq 0$, then $|\varepsilon| < 2^{\lfloor\log_2 |\sin(2\pi x/u)|\rfloor-p}$, where $p$ is
///   the precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(x,0)=\text{NaN}$
/// - $f(0,u)=0$
/// - If $x/u$ is a multiple of $1/2$, the result is exactly $0.0$ with the sign of $x$ (following
///   IEEE 754-2019's `sinPi`, so that the function is odd); if it is an odd multiple of $1/4$, the
///   result is exactly $1$ or $-1$; and if it is $\pm1/12$ or $\pm5/12$ modulo $1$, the result is
///   exactly $1/2$ or $-1/2$.
///
/// Overflow is not possible, since the result lies in $[-1, 1]$. The result underflows, to a
/// subnormal or to zero, only when $2\pi x/u$ does, for a tiny $x/u$; a [`Rational`] close enough
/// to a half turn, without being one, for its sine to be subnormal would need a denominator of more
/// than 100 bits, in which case the result is still correctly rounded.
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
/// use malachite_float::float::arithmetic::sin::primitive_float_sin_with_period_rational;
/// use malachite_q::Rational;
///
/// assert!(primitive_float_sin_with_period_rational::<f64>(&Rational::ZERO, 0).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_sin_with_period_rational::<f64>(
///         &Rational::ZERO,
///         360
///     )),
///     NiceFloat(0.0)
/// );
/// // a twelfth of a turn is exactly 1/2
/// assert_eq!(
///     NiceFloat(primitive_float_sin_with_period_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 12),
///         1
///     )),
///     NiceFloat(0.5)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sin_with_period_rational::<f32>(
///         &Rational::from_unsigneds(1u8, 7),
///         1
///     )),
///     NiceFloat(0.7818315)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sin_with_period_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 7),
///         1
///     )),
///     NiceFloat(0.7818314824680298)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin_with_period_rational<T: PrimitiveFloat>(x: &Rational, u: u64) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(
        |x, prec| Float::sin_with_period_rational_prec_ref(x, u, prec),
        x,
    )
}

/// Computes $\sin(\pi x)$, the sine of a primitive float measured in half-turns.
///
/// This is `primitive_float_sin_with_period` with a period of 2: see
/// [`primitive_float_sin_with_period`] for the error bound and the special cases, with $u = 2$.
/// Half-integers give exactly $\pm1$ and integers exactly $\pm0.0$ with the sign of the input.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin::primitive_float_sin_pi;
///
/// assert!(primitive_float_sin_pi(f32::NAN).is_nan());
/// assert_eq!(NiceFloat(primitive_float_sin_pi(0.5f32)), NiceFloat(1.0));
/// assert_eq!(NiceFloat(primitive_float_sin_pi(1.0f64)), NiceFloat(0.0));
/// assert_eq!(
///     NiceFloat(primitive_float_sin_pi(0.1f32)),
///     NiceFloat(0.309017)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sin_pi(0.1f64)),
///     NiceFloat(0.30901699437494745)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin_pi<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_sin_with_period(x, 2)
}

/// Computes $\sin(\pi x)$, the sine of a [`Rational`] measured in half-turns, returning the result
/// as a primitive float.
///
/// This is `primitive_float_sin_with_period_rational` with a period of 2: see
/// [`primitive_float_sin_with_period_rational`] for the error bound, the special cases, and the
/// complexity, with $u = 2$.
///
/// # Worst-case complexity
/// $T(m) = O(m (\log m)^2 \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `x.significant_bits()`.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::sin::primitive_float_sin_pi_rational;
/// use malachite_q::Rational;
///
/// // a sixth of a half-turn is exactly 1/2
/// assert_eq!(
///     NiceFloat(primitive_float_sin_pi_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 6)
///     )),
///     NiceFloat(0.5)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_sin_pi_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 7)
///     )),
///     NiceFloat(0.4338837391175581)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_sin_pi_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_sin_with_period_rational(x, 2)
}
