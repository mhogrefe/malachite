// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2005-2026 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use core::cmp::Ordering::{self, *};
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::arithmetic::float::round::float_can_round;

// Steps `y` to the next `prec`-precision value away from zero, like `mpfr_nexttoinf` (note that
// `Float::increment` is not suitable: it does not preserve precision when crossing a power of 2).
// Multiplying by 1 + 2^-(prec+1) yields a value strictly between y and its successor in magnitude
// — including across power-of-2 boundaries — so rounding away from zero produces exactly that
// successor. Overflow produces an infinity.
fn step_away_from_zero(y: Float, prec: u64) -> Float {
    let mult = Float::one_prec(prec + 2)
        .add_prec(Float::power_of_2(-i64::exact_from(prec) - 1), prec + 2)
        .0;
    y.mul_prec_round(mult, prec, Up).0
}

// Steps `y` to the next `prec`-precision value toward zero, like `mpfr_nexttozero`. Multiplying by
// 1 - 2^-(prec+1) yields a value strictly between y and its predecessor in magnitude — including
// across power-of-2 boundaries, where the spacing halves — so rounding toward zero produces
// exactly that predecessor. Underflow produces a zero with y's sign.
fn step_toward_zero(y: Float, prec: u64) -> Float {
    let mult = Float::one_prec(prec + 1)
        .sub_prec(Float::power_of_2(-i64::exact_from(prec) - 1), prec + 1)
        .0;
    y.mul_prec_round(mult, prec, Down).0
}

// Helper for fast rounding when a function's value is known to be very close to its argument (or to
// another easily-computed value). Assuming the true value is f(x) = v + g(x) with |g(x)| <
// 2^(EXP(v) - err), and that f(x) is not exactly representable, tries to determine round(f(x),
// prec, rm) from v alone.
//
// If the error bound is too large to round correctly, returns `None`, and the caller must compute
// f(x) the expensive way. Otherwise, returns the correctly rounded value and the (nonzero) ternary
// value, as an `Ordering`.
//
// `v` must be finite and nonzero. If `dir` is `false`, the error term g(x) brings f(x) toward zero
// (|f(x)| < |v|); if `dir` is `true`, away from zero (|f(x)| > |v|).
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `v.significant_bits()`.
//
// # Panics
// Panics if `v` is NaN, infinite, or zero, or if `rm` is `Exact` (the result is never exact).
//
// This is equivalent to `mpfr_round_near_x` from `round_near_x.c`, MPFR 4.3.0, where the result is
// returned along with the ternary value, and a `None` return corresponds to a 0 return in C. MPFR's
// MPFR_FAST_COMPUTE_IF_SMALL_INPUT, for a function whose value is x plus a correction of order x^3:
// returns the correctly rounded result directly when x is small enough for that correction to be
// invisible at the target precision, and `None` when the caller must compute the function properly.
//
// `err1` and `err2` are the two halves of MPFR's error exponent, which together bound the
// correction by 2^(EXP(x) - err1 - err2); `dir` says whether the correction carries the value away
// from zero. A nonpositive `err1` means x is too large for the shortcut.
//
// The bound handed to `float_round_near_x` is capped rather than passed at its true size, which for
// a tiny x runs to about 2^31: a pointlessly large bound makes it round a huge value, and the
// general path its callers fall back to works at a precision of about -EXP(x) bits. The cap sits
// above the input's own precision where it can, which spares `float_round_near_x` its
// `float_can_round` test; where the true bound is smaller than that the test does run and can
// decline, but only for an x large enough that the general path is cheap.
pub(crate) fn small_input_shortcut(
    x: &Float,
    err1: i64,
    err2: u64,
    dir: bool,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    if err1 <= 0 {
        return None;
    }
    let err = u64::exact_from(err1) + err2;
    if err <= prec + 1 {
        return None;
    }
    let cap = max(prec + 2, x.get_prec().unwrap() + 1);
    float_round_near_x(x, min(err, cap), dir, prec, rm)
}

crate_test_fn! {float_round_near_x(
    v: &Float,
    err: u64,
    dir: bool,
    prec: u64,
    rm: RoundingMode
) -> Option<(Float, Ordering)> {
    assert_ne!(rm, Exact, "Inexact float_round_near_x");
    let sign = if v > &0u32 { Greater } else { Less };
    let prec_v = v.get_prec().unwrap();
    // First check if we can round. The test is more restrictive than necessary. (The C version
    // calls the raw mpfr_round_p and adds 1 to the precision for Nearest itself; float_can_round is
    // the MPFR_CAN_ROUND macro, which already does that internally.)
    if !(err > prec + 1
        && (err > prec_v || float_can_round(v.significand_ref().unwrap(), err, prec, rm)))
    {
        // If we can't round, the caller must compute the function the expensive way.
        return None;
    }
    // Round v to the target precision. In C this is MPFR_RNDRAW_GEN with a custom halfway-case
    // hook; here, the halfway case is detected separately: v rounds to a tie at `prec` iff it is
    // exactly representable in prec + 1 bits but not in prec bits.
    if rm == Nearest {
        let (y_wide, o_wide) = Float::from_float_prec_round_ref(v, prec + 1, Down);
        if o_wide == Equal {
            let (y_trunc, o_trunc) = Float::from_float_prec_round(y_wide, prec, Down);
            if o_trunc != Equal {
                // Halfway case. Instead of rounding to even, the error direction breaks the tie: if
                // the error is toward zero, the true value is below the midpoint, so truncate;
                // otherwise it is above, so round away from zero.
                return Some(if dir {
                    (step_away_from_zero(y_trunc, prec), sign)
                } else {
                    (y_trunc, sign.reverse())
                });
            }
        }
    }
    let (mut y, mut o) = Float::from_float_prec_round_ref(v, prec, rm);
    // If o == Equal, setting y from v was exact, but the error term hasn't been taken into account
    // yet; the result is still inexact, and some rounding modes require a final nudge.
    if o == Equal {
        if dir {
            // The error term brings f(x) away from zero.
            o = sign.reverse();
            let rounds_away = match rm {
                Floor => sign == Less,
                Ceiling => sign == Greater,
                Up => true,
                _ => false,
            };
            if rounds_away {
                o = sign;
                y = step_away_from_zero(y, prec);
            }
        } else {
            // The error term brings f(x) toward zero.
            o = sign;
            let rounds_to_zero = match rm {
                Floor => sign == Greater,
                Ceiling => sign == Less,
                Down => true,
                _ => false,
            };
            if rounds_to_zero {
                o = sign.reverse();
                y = step_toward_zero(y, prec);
            }
        }
    }
    assert_ne!(o, Equal);
    Some((y, o))
}}
