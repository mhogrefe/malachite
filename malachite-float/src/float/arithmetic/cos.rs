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

// Port of MPFR's cosine. `mpfr_cos` (`cos.c`) reduces an argument with |x| >= 4 modulo 2 pi using
// `mpfr_remainder`, halves the (squared) reduced argument K times, sums the Taylor series of cos in
// integer arithmetic (`mpfr_cos2_aux`), and undoes the halvings with cos(2x) = 2cos^2(x) - 1, all
// inside a Ziv loop. The `mpfr_cos_fast` tier, used for precisions at or above
// `MPFR_SINCOS_THRESHOLD` and built on `mpfr_sincos_fast`, is not ported yet.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::float::arithmetic::exp::{get_z_2exp, one_neighbor};
use crate::float::arithmetic::round_near_x::float_round_near_x;
use crate::{ComparableFloatRef, Float, emulate_float_to_float_fn, emulate_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::{max, min};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, Cos, CosAssign, DivRoundAssign, FloorSqrt, ModPowerOf2, NegAssign, Parity,
    PowerOf2, Square, SubMul, UnsignedAbs,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NaN as NaNTrait, One};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, Ceiling, Down, Exact, Floor, Nearest};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// f <- 1 - r/2! + r^2/4! + ... + (-1)^l r^l/(2l)! + ...
//
// Assumes |r| < 1/2, and f, r have the same precision. Returns e such that the error on f is
// bounded by 2^e ulps.
//
// The smallest i such that i*(i+1) might not fit in a u64. Reaching it would take 2^32 terms, so
// the two-step division below is never exercised in practice.
const MAXI: u64 = 1 << (u64::WIDTH >> 1);

// This is mpfr_cos2_aux from cos.c, MPFR 4.2.2.
fn cos2_aux(r: &Float, p: u64) -> (Float, u64) {
    let exp_r = i64::from(r.get_exponent().unwrap());
    assert!(exp_r <= -1);
    let (mut x, mut ex) = get_z_2exp(r.clone()); // r = x*2^ex
    // Remove trailing zeroes. Since x comes from a regular MPFR number, due to the constraints on
    // the exponent and the precision, there can be no integer overflow below.
    let l = x.trailing_zeros().unwrap();
    ex += i64::exact_from(l);
    x >>= l;
    // since |r| < 1, r = x*2^ex, and x is an integer, necessarily ex < 0 bound for number of
    // iterations
    let mut imax = p / u64::exact_from(-exp_r);
    imax += u64::from(imax == 0);
    let q = (imax.ceiling_log_base_2() << 1) + 4; // bound for (3l)^2
    let mut s = Integer::ONE << (p + q); // initialize sum with 1, scaled by 2^(p+q)
    let mut t = s.clone(); // invariant: t is previous term
    let mut i: u64 = 1;
    loop {
        let m = t.significant_bits();
        if m < q {
            break;
        }
        // adjust precision of x to that of t
        let mut l = x.significant_bits();
        if l > m {
            l -= m;
            x >>= l;
            ex += i64::exact_from(l);
        }
        // multiply t by r
        t *= &x;
        t >>= u64::exact_from(-ex);
        // divide t by i*(i+1)
        if i < MAXI {
            t.div_round_assign(Integer::from(i * (i + 1)), Floor);
        } else {
            t.div_round_assign(Integer::from(i), Floor);
            t.div_round_assign(Integer::from(i + 1), Floor);
        }
        // if m is the (current) number of bits of t, we can consider that all operations on t so
        // far had precision >= m, so we can prove by induction that the relative error on t is of
        // the form (1+u)^(3l)-1, where |u| <= 2^(-m), and l=(i+1)/2 is the # of loops. Since
        // |(1+x^2)^(1/x) - 1| <= 4x/3 for |x| <= 1/2, for |u| <= 1/(3l)^2, the absolute error is
        // bounded by 4/3*(3l)*2^(-m)*t <= 4*l since |t| < 2^m. Therefore the error on s is bounded
        // by 2*l*(l+1).
        //
        // add or subtract to s
        if i % 4 == 1 {
            s -= &t;
        } else {
            s += &t;
        }
        i += 2;
    }
    let f = Float::from_integer_prec(s, p).0 >> (p + q);
    let l = (i - 1) >> 1; // number of iterations
    (f, ((l + 1).ceiling_log_base_2() << 1) + 1) // bound is 2l(l+1)
}

// Rounds both ends of a bracket [lo, hi] known to contain a transcendental value; if the two ends
// round to the same `Float` on the same side of it, that settles the result. A bound that is
// exactly representable rounds with `Equal`, which is merged with the other bound's `Ordering`. The
// comparison is sign-sensitive, so a bracket straddling zero is never accepted.
fn round_bracket(
    lo: &Rational,
    hi: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    let (f_lo, mut o_lo) = Float::from_rational_prec_round_ref(lo, prec, rm);
    let (f_hi, mut o_hi) = Float::from_rational_prec_round_ref(hi, prec, rm);
    if o_lo == Equal {
        o_lo = o_hi;
    }
    if o_hi == Equal {
        o_hi = o_lo;
    }
    (o_lo == o_hi && ComparableFloatRef(&f_lo) == ComparableFloatRef(&f_hi)).then_some((f_lo, o_lo))
}

// cos(x) for a nonzero x so small that 1 - x^2/2 <= cos(x) < 1 lies within half an ulp of 1 at
// precision `prec`: the result is 1, or its predecessor for rounding toward zero.
fn cos_rational_tiny(prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    match rm {
        Floor | Down => (one_neighbor(prec, false), Less),
        _ => (Float::one_prec(prec), Greater),
    }
}

// Sums the cosine series 1 - x^2/2! + x^4/4! - ... in `Rational` arithmetic for a nonzero |x| < 1
// too small to be a `Float`. The terms alternate in sign with decreasing magnitude, so cos(x) lies
// between consecutive partial sums, and the bracket is tightened until both ends round the same
// way. Only reachable for a precision beyond 2^31 bits: any smaller precision takes the tiny path.
fn cos_rational_series(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    fail_on_untested_path("cos_rational_series");
    let x_squared = x.square();
    let mut s = Rational::ONE;
    let mut term = Rational::ONE;
    let mut k = 1u64;
    loop {
        term *= &x_squared;
        term /= Rational::from((k << 1) * ((k << 1) - 1));
        term.neg_assign();
        let s_next = &s + &term;
        let (lo, hi) = if s < s_next {
            (&s, &s_next)
        } else {
            (&s_next, &s)
        };
        if let Some(result) = round_bracket(lo, hi, prec, rm) {
            return result;
        }
        s = s_next;
        k += 1;
    }
}

// Reduces a `Rational` too large to be a `Float` modulo 2 pi, using pi to exp_x + w bits, so that
// the reduced value y satisfies |x - 2 pi k - y| <= 2^(2 - w): |k| < 2^exp_x, and 2 pi is known to
// within 2^(2 - exp_x - w).
fn reduce_huge(x: &Rational, exp_x: i64, w: u64) -> Rational {
    let two_pi = Rational::exact_from(&(Float::pi_prec(u64::exact_from(exp_x) + w).0 << 1u32));
    let k = Integer::rounding_from(x / &two_pi, Nearest).0;
    x.sub_mul(&two_pi, &Rational::from(k))
}

// cos(y) for a `Rational` y within about 2^-cancel of an odd multiple of pi/2 (cancel >= 64 and
// cancel >= prec / 16), where the general bracket would need its working precision raised by
// `cancel` bits. As in `cos_near_zero`, write y = n pi/2 + delta with n odd, so that cos(y) =
// -sin(delta) if n = 1 mod 4 and sin(delta) otherwise; delta is computed exactly in `Rational`
// arithmetic from pi to exp_y + w bits (error at most 2^(1 - w), plus `extra` for a reduced y), and
// sin(delta) is bracketed by t - t^3/6 and t. The bracket is rounded in `Rational` arithmetic, so
// the result underflows correctly when it must.
fn cos_rational_near_zero(
    y: &Rational,
    exp_y: i64,
    prec: u64,
    rm: RoundingMode,
    extra: Option<i64>,
    mut w: u64,
) -> (Float, Ordering) {
    let mut increment = Limb::WIDTH;
    // A rational y = a/b is typically no closer to n pi/2 than about 1/b (a dyadic approximation of
    // pi/2 to k bits, for instance, is off by about 2^-k), so a precision that resolves delta at
    // that scale is a good first target: as in `cos_near_zero`, w at least grows by half each time
    // and by up to 8 times to reach the hint, so that the early iterations cost a small fraction of
    // the last one.
    let w_hint = y.denominator_ref().significant_bits() + prec + 64;
    loop {
        let pi = Rational::exact_from(&Float::pi_prec(u64::exact_from(max(exp_y, 1)) + w).0);
        let n = Integer::rounding_from((y / &pi) << 1u32, Nearest).0;
        assert!(n.odd());
        let negate = (&n).mod_power_of_2(2) == 1u32;
        let half_pi = pi >> 1u32;
        let delta = y.sub_mul(&half_pi, &Rational::from(&n));
        let mut e = Rational::power_of_2(1 - i64::exact_from(w));
        if let Some(extra) = extra {
            e += Rational::power_of_2(extra);
        }
        let d_lo = &delta - &e;
        let d_hi = delta + e;
        // for |t| <= 1, t - t^3/6 <= sin(t) <= t when t >= 0, and t <= sin(t) <= t - t^3/6 when t <
        // 0; sin is increasing on [-1, 1]
        let cubic = |t: &Rational| t - t.square() * t / const { Rational::const_from_unsigned(6) };
        let sin_lo = if d_lo >= 0u32 { cubic(&d_lo) } else { d_lo };
        let sin_hi = if d_hi >= 0u32 { d_hi } else { cubic(&d_hi) };
        let (lo, hi) = if negate {
            (-sin_hi, -sin_lo)
        } else {
            (sin_lo, sin_hi)
        };
        if let Some(result) = round_bracket(&lo, &hi, prec, rm) {
            return result;
        }
        w = max(w + increment, min(w_hint, w << 3));
        increment = w >> 1;
    }
}

// Computes cos(x) for a nonzero `Rational` x, rounded to precision `prec` with rounding mode `rm`.
// (cos(0) = 1 is handled by the caller.) The cosine of a nonzero rational is transcendental, so the
// result is never exactly representable and `rm` must not be `Exact`.
//
// The general case rounds x to a `Float` y_f at a working precision w, takes its correctly rounded
// cosine c_f, and brackets cos(x) using |cos(x) - cos(y_f)| <= |x - y_f|, the rounding error of
// c_f, and, for an x too large to be a `Float`, the error of a `Rational` reduction modulo 2 pi.
// The bracket is rounded in `Rational` arithmetic, and w is raised until both ends agree. Unlike
// `exp_rational_helper`'s bracket of x itself, this needs no monotonicity.
fn cos_rational_helper(x: &Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact cos");
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // 1 - cos(x) <= x^2/2 < 2^(2 exp_x - 1): when that is at most 2^(-prec - 1), cos(x) rounds to 1
    if 1 - (exp_x << 1) > i64::exact_from(prec) {
        return cos_rational_tiny(prec, rm);
    }
    // x is too small to be a `Float` but `prec` is so large that cos(x) does not round to 1
    if exp_x <= Float::MIN_EXPONENT_I64 {
        return cos_rational_series(x, prec, rm);
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
        let (y_f, y_o) = Float::from_rational_prec_ref(y, w);
        if !huge && y_o == Equal {
            // x is exactly representable at w bits, so cos(x) is simply its cosine
            return cos_prec_round_normal_ref(&y_f, prec, rm);
        }
        let c_f = (&y_f).cos();
        // The exponents of y and c_f, as `Float`s would have them (y is nonzero, and c_f is zero
        // only if it underflowed, which counts as complete cancellation).
        let exp_y = y.floor_log_base_2_abs() + 1;
        let exp_c = c_f
            .get_exponent()
            .map_or(Float::MIN_EXPONENT_I64, i64::from);
        // |cos(y)| < 2^exp_c (up to the bracket width): heavy cancellation means y is close to an
        // odd multiple of pi/2, where the bracket below would have to be far narrower than 2^-w.
        if exp_c < 0 {
            let cancel = u64::exact_from(-exp_c);
            if cancel >= max(NEAR_ZERO_MIN_CANCEL, prec >> 4) {
                return cos_rational_near_zero(y, exp_y, prec, rm, extra, w);
            }
        }
        // |c_f - cos(y_f)| <= 2^(exp_c - w) (half an ulp, doubled for safety), and |cos(y) -
        // cos(y_f)| <= |y - y_f| <= 2^(exp_y - w)
        let w_i = i64::exact_from(w);
        let mut delta = Rational::power_of_2(exp_c - w_i) + Rational::power_of_2(exp_y - w_i);
        if let Some(extra) = extra {
            delta += Rational::power_of_2(extra);
        }
        let c = Rational::exact_from(&c_f);
        if let Some(result) = round_bracket(&(&c - &delta), &(c + delta), prec, rm) {
            return result;
        }
        w += increment;
        increment = w >> 1;
    }
}

// The least number of bits of cancellation (|cos(x)| < 2^-cancel) that sends an input to
// `cos_near_zero`; the cancellation must also be at least prec / 16, so that the Taylor series
// there needs only a handful of terms.
const NEAR_ZERO_MIN_CANCEL: u64 = 64;

// Computes cos(x) for an x within about 2^-cancel of an odd multiple of pi/2, so that |cos(x)| <
// 2^-cancel, where cancel >= 64 and cancel >= prec / 16.
//
// The Ziv loop in `cos_prec_round_normal_ref` would have to raise its working precision by `cancel`
// bits to resolve such a result, and its halving-plus-Taylor-series scheme becomes prohibitively
// slow long before `cancel` reaches 2^30, where the result underflows. Instead, write x = n pi/2 +
// delta with n odd, so that cos(x) = -sin(delta) if n = 1 mod 4 and sin(delta) if n = 3 mod 4.
// delta is computed exactly in integer arithmetic from x and an approximation of pi, and
// sin(delta)/delta from its Taylor series, which converges very quickly since |delta| is tiny.
// Everything is done in integers scaled by explicit powers of 2, so values far below the exponent
// range are no problem, and only the final `shl_prec_round` can underflow.
//
// This has no MPFR counterpart: MPFR's exponent range is so wide that cos never underflows there,
// and `mpfr_cos` simply keeps raising its working precision.
fn cos_near_zero(x: &Float, prec: u64, rm: RoundingMode, cancel: u64) -> (Float, Ordering) {
    // |x| > 1, so its exponent is positive
    let e = u64::exact_from(x.get_exponent().unwrap());
    // n = round(2x / pi). Since 2x / pi is within 2^-62 of an odd integer, a quotient with 16 bits
    // after the binary point suffices to identify it.
    let x_low = Float::from_float_prec_ref(x, e + 16).0 << 1u32;
    let q = x_low.div_prec(Float::pi_prec(e + 16).0, e + 16).0;
    let n = Integer::rounding_from(q, Nearest).0;
    assert!(n.odd());
    let negate = (&n).mod_power_of_2(2) == 1u32;
    // x = x_sig * 2^x_exp exactly
    let x_sig = x.significand_ref().unwrap();
    let x_bits = x_sig.significant_bits();
    let x_exp = i64::from(x.get_exponent().unwrap()) - i64::exact_from(x_bits);
    // The working precision: delta and sin(delta) / delta are computed to w bits.
    let w = prec + 64;
    // The precision of pi: since |delta| < 2^(1 - cancel), delta is resolved to w bits once p
    // exceeds w + cancel, unless delta is even smaller than the cancellation suggests. In that
    // case, x is typically n pi/2 rounded to its own precision, so that |delta| is about 2^(e -
    // x_bits), and p_hint is the precision that resolves that. The precision at least doubles each
    // time, and grows by up to 8 times to reach p_hint, so that the cost of the early iterations is
    // a small fraction of that of the last one, without wildly overshooting if x is only stored at
    // a higher precision than the one it agrees with n pi/2 to.
    let mut p = w + cancel + 2;
    let p_hint = (x_bits + w + 2).saturating_sub(e);
    loop {
        // pi_p = pi_sig * 2^pi_exp, with |pi_p - pi| <= 2^(1 - e - p), so |n pi_p / 2 - n pi / 2| <
        // 2^-p, since |n| < 2^e.
        let (pi_sig, pi_exp) = get_z_2exp(Float::pi_prec(e + p).0);
        let pi_exp = pi_exp - 1; // n pi_p / 2 = n pi_sig * 2^pi_exp
        // delta = d * 2^d_exp, up to the error in pi
        let d_exp = min(x_exp, pi_exp);
        let a = Integer::from_sign_and_abs_ref(x > &0u32, x_sig) << u64::exact_from(x_exp - d_exp);
        let b = (&n * pi_sig) << u64::exact_from(pi_exp - d_exp);
        let d = a - b;
        let d_neg = d < 0u32;
        let mut d_abs = d.unsigned_abs();
        let d_bits = d_abs.significant_bits();
        // 2^(delta_exp - 1) <= |delta| < 2^delta_exp. Since x has bits beyond the precision of pi,
        // d is practically never zero; if it is, the bound below is trivially true.
        let delta_exp = d_exp + i64::exact_from(d_bits);
        // delta must be resolved to w bits, i.e. its error 2^-p must be below 2^(delta_exp - w)
        if delta_exp + i64::exact_from(p) <= i64::exact_from(w) {
            p = max(
                max(p << 1, min(p_hint, p << 3)),
                u64::exact_from(i64::exact_from(w) - delta_exp + 2),
            );
            continue;
        }
        // Truncate d to w bits: |delta| = d_abs * 2^d_exp, with |d_abs| < 2^w, up to 1 unit of
        // truncation error and 2^(-p - d_exp) < 1 unit of error from pi.
        let mut d_exp = d_exp;
        if d_bits > w {
            let shift = d_bits - w;
            d_abs >>= shift;
            d_exp += i64::exact_from(shift);
        }
        // q = delta^2 * 2^w = d_abs^2 * 2^(2 d_exp + w), a fixed-point number with w fractional
        // bits, up to 1 unit. Since d_exp <= -p < -w, the shift is always to the right.
        let q = Integer::from(
            (&d_abs).square() >> u64::exact_from(-((d_exp << 1) + i64::exact_from(w))),
        );
        // r = (sin(delta) / delta) * 2^w = 2^w - q / 3! + q^2 / (5! 2^w) - ..., each term computed
        // with at most 2 units of rounding error. Since |delta| < 2^-63, each term is at most
        // 2^-126 times the previous one, so the error on r is at most 2 * terms + 2 units,
        // including the tail after the last nonzero term.
        let mut r = Integer::power_of_2(w);
        let mut term = Integer::power_of_2(w);
        let mut k = 1u64;
        let mut terms = 0u64;
        loop {
            term *= &q;
            term >>= w;
            term.div_round_assign(Integer::from((k << 1) * ((k << 1) + 1)), Floor);
            term.neg_assign();
            if term == 0u32 {
                break;
            }
            r += &term;
            k += 1;
            terms += 1;
        }
        // m = d_abs * r approximates |sin(delta)| * 2^(w - d_exp). Its error is at most d_abs * (2
        // terms + 2) + r * 2 < 2^w (2 terms + 4) < 2^(w + 2) (terms + 2) units.
        let mut m = Integer::from(d_abs) * r;
        if d_neg != negate {
            m.neg_assign();
        }
        let m_bits = m.significant_bits();
        assert!(m_bits <= const { Float::MAX_EXPONENT as u64 });
        let err = m_bits - w - 2 - (terms + 2).ceiling_log_base_2();
        let s = Float::from_integer_prec(m, m_bits).0;
        if float_can_round(s.significand_ref().unwrap(), err, prec, rm) {
            // cos(x) = m * 2^(d_exp - w). `float_can_round` guarantees that s and the exact value
            // round the same way and lie on the same side of every power of 2, so the single
            // rounding in `shl_prec_round`, including its underflow handling, gives the correct
            // result and `Ordering`.
            return s.shl_prec_round(d_exp - i64::exact_from(w), prec, rm);
        }
        fail_on_untested_path("cos_near_zero, cannot round");
        p <<= 1;
    }
}

// This is mpfr_cos from cos.c, MPFR 4.2.2, without the `mpfr_cos_fast` tier for precisions at or
// above `MPFR_SINCOS_THRESHOLD`, which depends on `mpfr_sincos_fast`.
fn cos_prec_round_normal_ref(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact cos");
    // cos(x) = 1-x^2/2 + ..., so error < 2^(2*EXP(x)-1)
    let exp_x = i64::from(x.get_exponent().unwrap());
    // MPFR_SMALL_INPUT_AFTER_SAVE_EXPO (y, __gmpfr_one, -2 * expx, 1, 0, rnd_mode, expo, {});
    let neg_err = -(exp_x << 1);
    if neg_err > 0 {
        let err = u64::exact_from(neg_err) + 1;
        if err > prec + 1 {
            // The reference value 1 has precision 1 < err, so float_round_near_x always succeeds.
            return float_round_near_x(&Float::ONE, err, false, prec, rm).unwrap();
        }
    }
    // Compute initial precision
    let k0 = (prec / 3).floor_sqrt();
    let mut m = prec + (prec.ceiling_log_base_2() << 1) + (k0 << 1) + 4;
    let reduce = exp_x >= 3;
    let mut cancel: i64 = 0;
    let mut increment = Limb::WIDTH;
    let s = loop {
        // If |x| >= 4, first reduce x cmod (2*Pi) into xr, using mpfr_remainder: let e = EXP(x) >=
        // 3, and m the target precision:
        // ```
        // (1) c <- 2*Pi              [precision e+m-1, nearest]
        // (2) xr <- remainder (x, c) [precision m, nearest]
        // We have |c - 2*Pi| <= 1/2ulp(c) = 2^(3-e-m)
        //         |xr - x - k c| <= 1/2ulp(xr) <= 2^(1-m)
        //         |k| <= |x|/(2*Pi) <= 2^(e-2)
        // Thus |xr - x - 2kPi| <= |k| |c - 2Pi| + 2^(1-m) <= 2^(2-m).
        // It follows |cos(xr) - cos(x)| <= 2^(2-m).
        // ```
        let mut goto_ziv_next = false;
        let mut r = if reduce {
            let c = Float::pi_prec(u64::exact_from(exp_x) + m - 1).0 << 1u32; // 2Pi
            let xr = x.ieee_remainder_prec_ref_val(c, m).0;
            if xr == 0u32 {
                goto_ziv_next = true;
                Float::one_prec(m)
            } else {
                // now |xr| <= 4, thus r <= 16 below
                xr.square_round(Ceiling).0 // err <= 1 ulp
            }
        } else {
            x.square_prec_round_ref(m, Ceiling).0 // err <= 1 ulp
        };
        let mut result = None;
        if !goto_ziv_next {
            // now |x| < 4 (or xr if reduce = 1), thus |r| <= 16 we need |r| < 1/2 for
            // mpfr_cos2_aux, i.e., EXP(r) - 2K <= -1
            let exp_r = i64::from(r.get_exponent().unwrap());
            let k = k0 + 1 + (u64::exact_from(max(0, exp_r)) >> 1);
            // since K0 >= 0, if EXP(r) < 0, then K >= 1, thus EXP(r) - 2K <= -3; otherwise if
            // EXP(r) >= 0, then K >= 1/2 + EXP(r)/2, thus EXP(r) - 2K <= -1
            r >>= k << 1; // Can't overflow!
            // s <- 1 - r/2! + ... + (-1)^l r^l/(2l)!
            let (mut s, err_ulps) = cos2_aux(&r, m);
            // err_ulps is the error bound in ulps on s
            let one = Float::one_prec(m);
            for _ in 0..k {
                s.square_prec_round_assign(m, Ceiling); // err <= 2*olderr
                s <<= 1u32; // Can't overflow
                s.sub_prec_assign_ref(&one, m); // err <= 4*olderr
                if s == 0u32 {
                    fail_on_untested_path("cos_prec_round_normal_ref, s == 0 after doubling");
                    goto_ziv_next = true;
                    break;
                }
                assert!(s.get_exponent().unwrap() <= 1);
            }
            if !goto_ziv_next {
                // The absolute error on s is bounded by (2l+1/3)*2^(2K-m) 2l+1/3 <= 2l+1. If |x| >=
                // 4, we need to add 2^(2-m) for the argument reduction by 2Pi: if K = 0, this
                // amounts to add 4 to 2l+1/3, i.e., to add 2 to l; if K >= 1, this amounts to add 1
                // to 2*l+1/3. (K >= 1 always holds here, since K0 >= 0, so the K = 0 case in the C
                // code is dead.)
                let mut err_ulps = (err_ulps << 1) + 1;
                if reduce {
                    err_ulps += 1;
                }
                let err_bits = err_ulps.ceiling_log_base_2() + (k << 1);
                // now the error is bounded by 2^(err_bits-m) = 2^(EXP(s)-err)
                let exp_s = i64::from(s.get_exponent().unwrap());
                let err = exp_s + i64::exact_from(m) - i64::exact_from(err_bits);
                if err > 0
                    && float_can_round(s.significand_ref().unwrap(), u64::exact_from(err), prec, rm)
                {
                    result = Some(s);
                } else if exp_s == 1
                    && m > err_bits
                    && m - err_bits >= prec + u64::from(rm == Nearest)
                {
                    // s = 1 or -1, and except x=0 which was already checked above, cos(x) cannot be
                    // 1 or -1, so we can round if the error is less than 2^(-precy) for directed
                    // rounding, or 2^(-precy-1) for rounding to nearest.
                    //
                    // If round to nearest or away, result is s = 1 or -1, otherwise it is
                    // round(nexttoward (s, 0)). However, in order to have the inexact flag
                    // correctly set below, we set |s| to 1 - 2^(-m) in all cases.
                    let neighbor = one_neighbor(m, false);
                    result = Some(if s < 0u32 { -neighbor } else { neighbor });
                } else {
                    // |cos(x)| < 2^bound
                    let bound = max(exp_s, i64::exact_from(err_bits) - i64::exact_from(m)) + 1;
                    if bound < 0 {
                        let c = u64::exact_from(-bound);
                        if c >= max(NEAR_ZERO_MIN_CANCEL, prec >> 4) {
                            return cos_near_zero(x, prec, rm, c);
                        }
                    }
                    if exp_s < cancel {
                        m += u64::exact_from(cancel - exp_s);
                        cancel = exp_s;
                    }
                }
            }
        }
        if let Some(s) = result {
            break s;
        }
        // ziv_next: MPFR_ZIV_NEXT (loop, m);
        m += increment;
        increment = m >> 1;
    };
    Float::from_float_prec_round(s, prec, rm)
}

impl Float {
    /// Computes $\cos x$, the cosine of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded cosine is less than, equal to, or greater than
    /// the exact cosine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\cos x|\leq 1$, the result never overflows.
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
    /// Underflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes
    /// more than $2^{30}$ bits of precision.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cos_prec`] instead. If you
    /// know that your target precision is the precision of the input, consider using
    /// [`Float::cos_round`] instead. If both of these things are true, consider using
    /// [`Float::cos`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the Taylor series at working precision $n$ costs the first term, and for
    /// $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$
    /// bits and a remainder of the $m$-bit input. Unlike most functions, `cos` therefore gets
    /// slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cosine of a finite nonzero [`Float`] is never exactly
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
    ///     .cos_prec_round(5, Floor);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cos_prec_round(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cos_prec_round(5, Nearest);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cos_prec_round(20, Floor);
    /// assert_eq!(c.to_string(), "0.54030228");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cos_prec_round(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.54030323");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100)
    ///     .0
    ///     .cos_prec_round(20, Nearest);
    /// assert_eq!(c.to_string(), "0.54030228");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.cos_prec_round_ref(prec, rm)
    }

    /// Computes $\cos x$, the cosine of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`]
    /// is also returned, indicating whether the rounded cosine is less than, equal to, or greater
    /// than the exact cosine. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm\infty,p,m)=\text{NaN}$
    /// - $f(\pm0.0,p,m)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\cos x|\leq 1$, the result never overflows.
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
    /// Underflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes
    /// more than $2^{30}$ bits of precision.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cos_prec_ref`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::cos_round_ref`] instead. If both of these things are true, consider using
    /// `(&Float).cos()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the Taylor series at working precision $n$ costs the first term, and for
    /// $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$
    /// bits and a remainder of the $m$-bit input. Unlike most functions, `cos` therefore gets
    /// slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cosine of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cos_prec_round_ref(5, Floor);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cos_prec_round_ref(5, Ceiling);
    /// assert_eq!(c.to_string(), "0.562");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cos_prec_round_ref(5, Nearest);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cos_prec_round_ref(20, Floor);
    /// assert_eq!(c.to_string(), "0.54030228");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cos_prec_round_ref(20, Ceiling);
    /// assert_eq!(c.to_string(), "0.54030323");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cos_prec_round_ref(20, Nearest);
    /// assert_eq!(c.to_string(), "0.54030228");
    /// assert_eq!(o, Less);
    /// ```
    pub fn cos_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &self.0 {
            NaN | Infinity { .. } => (Self::NAN, Equal),
            // cos(+0) = cos(-0) = 1
            Zero { .. } => (Self::one_prec(prec), Equal),
            Finite { .. } => cos_prec_round_normal_ref(self, prec, rm),
        }
    }

    /// Computes $\cos x$, the cosine of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded cosine is less than, equal to, or greater than the exact
    /// cosine. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// If the cosine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cos x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\cos x|\leq 1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes
    /// more than $2^{30}$ bits of precision.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cos_prec_round`] instead. If you know that your target precision is the precision
    /// of the input, consider using [`Float::cos`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the Taylor series at working precision $n$ costs the first term, and for
    /// $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$
    /// bits and a remainder of the $m$-bit input. Unlike most functions, `cos` therefore gets
    /// slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cos_prec(5);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cos_prec(20);
    /// assert_eq!(c.to_string(), "0.54030228");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_prec(self, prec: u64) -> (Self, Ordering) {
        self.cos_prec_round(prec, Nearest)
    }

    /// Computes $\cos x$, the cosine of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded cosine is less than, equal to, or greater than the
    /// exact cosine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the cosine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cos x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\pm\infty,p)=\text{NaN}$
    /// - $f(\pm0.0,p)=1.0$
    ///
    /// Overflow and underflow:
    /// - Since $|\cos x|\leq 1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes
    /// more than $2^{30}$ bits of precision.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cos_prec_round_ref`] instead. If you know that your target precision is the
    /// precision of the input, consider using `(&Float).cos()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the Taylor series at working precision $n$ costs the first term, and for
    /// $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$
    /// bits and a remainder of the $m$-bit input. Unlike most functions, `cos` therefore gets
    /// slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cos_prec_ref(5);
    /// assert_eq!(c.to_string(), "0.531");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cos_prec_ref(20);
    /// assert_eq!(c.to_string(), "0.54030228");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.cos_prec_round_ref(prec, Nearest)
    }

    /// Computes $\cos x$, the cosine of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded cosine is less than, equal to, or greater than the exact cosine.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN`
    /// it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \cos x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos
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
    /// - Since $|\cos x|\leq 1$, the result never overflows.
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
    /// Underflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes
    /// more than $2^{30}$ bits of precision.
    ///
    /// If you want to specify an output precision, consider using [`Float::cos_prec_round`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::cos`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `cos`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cosine of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cos_round(Floor);
    /// assert_eq!(c.to_string(), "0.54030230586813971740093660744256");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cos_round(Ceiling);
    /// assert_eq!(c.to_string(), "0.54030230586813971740093660744335");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::from_unsigned_prec(1u32, 100).0.cos_round(Nearest);
    /// assert_eq!(c.to_string(), "0.54030230586813971740093660744335");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cos_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.cos_prec_round(prec, rm)
    }

    /// Computes $\cos x$, the cosine of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded cosine is less than, equal to, or greater than the exact
    /// cosine. Although `NaN`s are not comparable to any [`Float`], whenever this function returns
    /// a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = \cos x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos
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
    /// - Since $|\cos x|\leq 1$, the result never overflows.
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
    /// Underflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes
    /// more than $2^{30}$ bits of precision.
    ///
    /// If you want to specify an output precision, consider using [`Float::cos_prec_round_ref`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// `(&Float).cos()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `cos`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cosine of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cos_round_ref(Floor);
    /// assert_eq!(c.to_string(), "0.54030230586813971740093660744256");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cos_round_ref(Ceiling);
    /// assert_eq!(c.to_string(), "0.54030230586813971740093660744335");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = (&Float::from_unsigned_prec(1u32, 100).0).cos_round_ref(Nearest);
    /// assert_eq!(c.to_string(), "0.54030230586813971740093660744335");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn cos_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.cos_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $\cos x$, the cosine of a [`Float`], rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`] is replaced by the result, and an
    /// [`Ordering`] is returned, indicating whether the rounded cosine is less than, equal to, or
    /// greater than the exact cosine. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \cos x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p+1}$.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::cos_prec_round`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cos_prec_assign`] instead. If
    /// you know that your target precision is the precision of the input, consider using
    /// [`Float::cos_round_assign`] instead. If both of these things are true, consider using
    /// [`Float::cos_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the Taylor series at working precision $n$ costs the first term, and for
    /// $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$
    /// bits and a remainder of the $m$-bit input. Unlike most functions, `cos` therefore gets
    /// slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cosine of a finite nonzero [`Float`] is never exactly
    /// representable, or if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cos_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.531");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cos_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.562");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cos_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.531");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cos_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.54030228");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cos_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.54030323");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cos_prec_round_assign(20, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.54030228");
    /// ```
    #[inline]
    pub fn cos_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let o;
        (*self, o) = self.cos_prec_round_ref(prec, rm);
        o
    }

    /// Computes $\cos x$, the cosine of a [`Float`], rounding the result to the nearest value of
    /// the specified precision. The [`Float`] is replaced by the result, and an [`Ordering`] is
    /// returned, indicating whether the rounded cosine is less than, equal to, or greater than the
    /// exact cosine. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// sets a `NaN` it also returns `Equal`.
    ///
    /// If the cosine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets \cos x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::cos_prec`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cos_prec_round_assign`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::cos_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is
    /// `self.significant_bits()`, and $e$ is the exponent of `self` (0 if `self` has no exponent or
    /// a negative one): the Taylor series at working precision $n$ costs the first term, and for
    /// $|x| \geq 4$ the argument is reduced modulo $2\pi$, which requires $\pi$ to about $n + e$
    /// bits and a remainder of the $m$-bit input. Unlike most functions, `cos` therefore gets
    /// slower as the magnitude of its input grows, not just as the precision does.
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
    /// assert_eq!(x.cos_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.531");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cos_prec_assign(20), Less);
    /// assert_eq!(x.to_string(), "0.54030228");
    /// ```
    #[inline]
    pub fn cos_prec_assign(&mut self, prec: u64) -> Ordering {
        self.cos_prec_round_assign(prec, Nearest)
    }

    /// Computes $\cos x$, the cosine of a [`Float`], rounding the result with the specified
    /// rounding mode. The [`Float`] is replaced by the result, and an [`Ordering`] is returned,
    /// indicating whether the rounded cosine is less than, equal to, or greater than the exact
    /// cosine. Although `NaN`s are not comparable to any [`Float`], whenever this function sets a
    /// `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets \cos x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite and $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $x$ is finite and $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos
    ///   x|\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::cos_round`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to specify an output precision, consider using [`Float::cos_prec_round_assign`]
    /// instead. If you know you'll be using the `Nearest` rounding mode, consider using
    /// [`Float::cos_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `cos`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact`, since the cosine of a finite nonzero [`Float`] is never exactly
    /// representable.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cos_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.54030230586813971740093660744256");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cos_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.54030230586813971740093660744335");
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// assert_eq!(x.cos_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.54030230586813971740093660744335");
    /// ```
    #[inline]
    pub fn cos_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.cos_prec_round_assign(prec, rm)
    }
}

impl Float {
    /// Computes $\cos x$, the cosine of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded cosine is less than, equal to, or greater than the exact cosine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows; see below.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
    ///
    /// Overflow and underflow:
    /// - Since $|\cos x|\leq 1$, the result never overflows.
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
    /// Underflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes
    /// more than $2^{30}$ bits.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cos_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] cosine taken there, which for $|x| \geq
    /// 4$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n + e$ bits.
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
    /// let (c, o) = Float::cos_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::cos_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) = Float::cos_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "0.82533550");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::cos_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.82533646");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn cos_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        Self::cos_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $\cos x$, the cosine of a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded cosine is less than, equal to, or greater than the exact cosine.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = \cos x+\varepsilon.
    /// $$
    /// - If $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos x|\rfloor-p+1}$.
    /// - If $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos x|\rfloor-p}$.
    ///
    /// These bounds do not apply when the result underflows.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p,m)=1$.
    ///
    /// See the [`Float::cos_rational_prec_round`] documentation for information on overflow and
    /// underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::cos_rational_prec_ref`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] cosine taken there, which for $|x| \geq
    /// 4$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n + e$ bits.
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
    ///     Float::cos_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::cos_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 5, Ceiling);
    /// assert_eq!(c.to_string(), "0.844");
    /// assert_eq!(o, Greater);
    ///
    /// let (c, o) =
    ///     Float::cos_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(c.to_string(), "0.82533550");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) =
    ///     Float::cos_rational_prec_round_ref(&Rational::from_unsigneds(3u8, 5), 20, Ceiling);
    /// assert_eq!(c.to_string(), "0.82533646");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn cos_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // cos(0) = 1, exactly
            return (Self::one_prec(prec), Equal);
        }
        cos_rational_helper(x, prec, rm)
    }

    /// Computes $\cos x$, the cosine of a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by value. An [`Ordering`] is also returned, indicating whether the rounded cosine is less
    /// than, equal to, or greater than the exact cosine.
    ///
    /// If the cosine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cos x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos x|\rfloor-p}$ (unless the result
    /// underflows; see below).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// Overflow and underflow:
    /// - Since $|\cos x|\leq 1$, the result never overflows.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p)<2^{-2^{30}}$, $2^{-2^{30}}$ is returned instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, $-2^{-2^{30}}$ is returned instead.
    ///
    /// Underflow requires an input within $2^{-2^{30}}$ of an odd multiple of $\pi/2$, which takes
    /// more than $2^{30}$ bits.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cos_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] cosine taken there, which for $|x| \geq
    /// 4$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n + e$ bits.
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
    /// let (c, o) = Float::cos_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::cos_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "0.82533550");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn cos_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::cos_rational_prec_round_ref(&x, prec, Nearest)
    }

    /// Computes $\cos x$, the cosine of a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded cosine is
    /// less than, equal to, or greater than the exact cosine.
    ///
    /// If the cosine is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = \cos x+\varepsilon,
    /// $$
    /// where $|\varepsilon| \leq 2^{\lfloor\log_2 |\cos x|\rfloor-p}$ (unless the result
    /// underflows).
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,p)=1$.
    ///
    /// See the [`Float::cos_rational_prec`] documentation for information on overflow and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cos_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m, e) = O(n^{3/2} \log n \log\log n + (n+m+e) (\log (n+m+e))^2 \log\log (n+m+e))$
    ///
    /// $M(n, m, e) = O((n+m+e) \log (n+m+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, $m$ is `x.significant_bits()`,
    /// and $e$ is `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): the input
    /// is rounded to a working precision and the [`Float`] cosine taken there, which for $|x| \geq
    /// 4$ reduces the argument modulo $2\pi$ and so needs $\pi$ to about $n + e$ bits.
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
    /// let (c, o) = Float::cos_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(c.to_string(), "0.812");
    /// assert_eq!(o, Less);
    ///
    /// let (c, o) = Float::cos_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(c.to_string(), "0.82533550");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn cos_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::cos_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl Cos for Float {
    type Output = Self;

    /// Computes $\cos x$, the cosine of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the cosine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \cos x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::cos_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using [`Float::cos_round`]
    /// instead. If you want to specify the output precision, consider using [`Float::cos_prec`]. If
    /// you want both of these things, consider using [`Float::cos_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `cos`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Cos;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.cos().is_nan());
    /// assert!(Float::INFINITY.cos().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.cos().is_nan());
    /// assert_eq!(Float::ZERO.cos(), Float::ONE);
    /// assert_eq!(
    ///     Float::from_unsigned_prec(1u32, 100).0.cos().to_string(),
    ///     "0.54030230586813971740093660744335"
    /// );
    /// assert_eq!(
    ///     Float::from_unsigned_prec(100u32, 100).0.cos().to_string(),
    ///     "0.86231887228768393410193851395099"
    /// );
    /// ```
    #[inline]
    fn cos(self) -> Self {
        let prec = self.significant_bits();
        self.cos_prec_round(prec, Nearest).0
    }
}

impl Cos for &Float {
    type Output = Float;

    /// Computes $\cos x$, the cosine of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the cosine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// f(x) = \cos x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\pm\infty)=\text{NaN}$
    /// - $f(\pm0.0)=1.0$
    ///
    /// See the [`Float::cos_round`] documentation for information on overflow and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cos_round_ref`] instead. If you want to specify the output precision, consider
    /// using [`Float::cos_prec_ref`]. If you want both of these things, consider using
    /// [`Float::cos_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `cos`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Cos;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.cos().is_nan());
    /// assert!(Float::INFINITY.cos().is_nan());
    /// assert!(Float::NEGATIVE_INFINITY.cos().is_nan());
    /// assert_eq!(Float::ZERO.cos(), Float::ONE);
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(1u32, 100).0).cos().to_string(),
    ///     "0.54030230586813971740093660744335"
    /// );
    /// assert_eq!(
    ///     (&Float::from_unsigned_prec(100u32, 100).0)
    ///         .cos()
    ///         .to_string(),
    ///     "0.86231887228768393410193851395099"
    /// );
    /// ```
    #[inline]
    fn cos(self) -> Float {
        self.cos_prec_round_ref(self.significant_bits(), Nearest).0
    }
}

impl CosAssign for Float {
    /// Computes $\cos x$, the cosine of a [`Float`], in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the cosine is
    /// equidistant from two [`Float`]s with the specified precision, the [`Float`] with fewer 1s in
    /// its binary expansion is chosen. See [`RoundingMode`] for a description of the `Nearest`
    /// rounding mode.
    ///
    /// $$
    /// x \gets \cos x+\varepsilon.
    /// $$
    /// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
    /// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos x|\rfloor-p}$, where $p$ is
    ///   the precision of the input.
    ///
    /// See the [`Float::cos`] documentation for information on special cases, overflow, and
    /// underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::cos_round_assign`] instead. If you want to specify the output precision, consider
    /// using [`Float::cos_prec_assign`]. If you want both of these things, consider using
    /// [`Float::cos_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n, e) = O(n^{3/2} \log n \log\log n + (n+e) (\log (n+e))^2 \log\log (n+e))$
    ///
    /// $M(n, e) = O((n+e) \log (n+e))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and $e$ is
    /// the exponent of `self` (0 if `self` has no exponent or a negative one): the Taylor series at
    /// working precision $n$ costs the first term, and for $|x| \geq 4$ the argument is reduced
    /// modulo $2\pi$, which requires $\pi$ to about $n + e$ bits. Unlike most functions, `cos`
    /// therefore gets slower as the magnitude of its input grows, not just as the precision does.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CosAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, Zero};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.cos_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.cos_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.cos_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::ZERO;
    /// x.cos_assign();
    /// assert_eq!(x, Float::ONE);
    ///
    /// let mut x = Float::from_unsigned_prec(1u32, 100).0;
    /// x.cos_assign();
    /// assert_eq!(x.to_string(), "0.54030230586813971740093660744335");
    ///
    /// let mut x = Float::from_unsigned_prec(100u32, 100).0;
    /// x.cos_assign();
    /// assert_eq!(x.to_string(), "0.86231887228768393410193851395099");
    /// ```
    #[inline]
    fn cos_assign(&mut self) {
        let prec = self.significant_bits();
        self.cos_prec_round_assign(prec, Nearest);
    }
}

/// Computes $\cos x$, the cosine of a primitive float. Using this function is more accurate than
/// using the default `cos` function or the one provided by `libm`.
///
/// $$
/// f(x) = \cos x+\varepsilon.
/// $$
/// - If $x$ is not finite, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $x$ is finite, then $|\varepsilon| < 2^{\lfloor\log_2 |\cos x|\rfloor-p}$, where $p$ is the
///   precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\pm\infty)=\text{NaN}$
/// - $f(\pm0.0)=1.0$
///
/// Overflow and underflow are not possible: the result lies in $[-1, 1]$, and no [`f32`] or [`f64`]
/// is close enough to an odd multiple of $\pi/2$ for its cosine to be subnormal.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::cos::primitive_float_cos;
///
/// assert!(primitive_float_cos(f32::NAN).is_nan());
/// assert!(primitive_float_cos(f32::INFINITY).is_nan());
/// assert!(primitive_float_cos(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(NiceFloat(primitive_float_cos(0.0f32)), NiceFloat(1.0));
/// assert_eq!(NiceFloat(primitive_float_cos(1.0f32)), NiceFloat(0.5403023));
/// assert_eq!(
///     NiceFloat(primitive_float_cos(1.0f64)),
///     NiceFloat(0.5403023058681398)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_cos<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::cos_prec, x)
}

/// Computes $\cos x$, the cosine of a [`Rational`], returning the result as a primitive float.
///
/// $$
/// f(x) = \cos x+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\cos x|\rfloor-p}$, and $p$ is the precision of the
/// output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]).
///
/// Special cases:
/// - $f(0)=1$
///
/// Overflow and underflow are not possible: the result lies in $[-1, 1]$, and a [`Rational`] close
/// enough to an odd multiple of $\pi/2$ for its cosine to be subnormal would need a denominator of
/// more than 100 bits, in which case the result is still correctly rounded.
///
/// # Worst-case complexity
/// $T(m, e) = O((m+e) (\log (m+e))^2 \log\log (m+e))$
///
/// $M(m, e) = O((m+e) \log (m+e))$
///
/// where $T$ is time, $M$ is additional memory, $m$ is `x.significant_bits()`, and $e$ is
/// `x.floor_log_base_2_abs()` (taken as 0 when it is negative or $x = 0$): for $|x| \geq 4$ the
/// argument is reduced modulo $2\pi$, which needs $\pi$ to about $e$ bits.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::cos::primitive_float_cos_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_cos_rational::<f64>(&Rational::ZERO)),
///     NiceFloat(1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cos_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.9449569463147377)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cos_rational::<f32>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(0.94495696)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_cos_rational::<f64>(&Rational::from(10000))),
///     NiceFloat(-0.9521553682590148)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_cos_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::cos_rational_prec_ref, x)
}
