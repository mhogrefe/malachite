// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2005-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::float::arithmetic::atan::{
    atan_rational_helper, atan_with_period_rational_helper, scaled_unsigned,
};
use crate::float::arithmetic::sin::{SCALE, SCALED_INPUT_EXPONENT, scaled_underflow};
use crate::{Float, emulate_float_float_to_float_fn, emulate_rational_rational_to_float_fn};
use core::cmp::Ordering::{self, Equal, Greater, Less};
use core::cmp::{max, min};
use malachite_base::num::arithmetic::traits::{
    Abs, AbsAssign, Atan2, Atan2Assign, CeilingLogBase2, IsPowerOf2,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    NaN as NaNTrait, NegativeZero as NegativeZeroTrait, Zero as ZeroTrait,
};
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{SignificantBits, TrailingZeros};
use malachite_base::rounding_modes::RoundingMode::{
    self, Ceiling, Down, Exact, Floor, Nearest, Up,
};
use malachite_nz::natural::arithmetic::float::round::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// pi/2^i, negated when `neg`. This is pi_div_2ui from atan2.c, MPFR 4.2.2; the shift is exact, so
// it does not disturb the ternary value.
fn pi_div_2ui(i: u32, neg: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact atan2");
    let (pi, o) = Float::pi_prec_round(prec, if neg { -rm } else { rm });
    let q = pi >> i;
    if neg { (-q, o.reverse()) } else { (q, o) }
}

// +-3 pi/4, for an infinite y over a negative infinite x. MPFR gives this its own Ziv loop, since
// unlike the other quadrant boundaries it is not a power of 2 times pi.
fn three_pi_over_4(neg: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact atan2");
    let mut w = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        // error <= 2 ulps
        let mut t = Float::pi_prec(w)
            .0
            .mul_prec(const { Float::const_from_unsigned(3) }, w)
            .0;
        t >>= 2u32;
        if float_can_round(t.significand_ref().unwrap(), w - 2, prec, rm) {
            let t = if neg { -t } else { t };
            return Float::from_float_prec_round(t, prec, rm);
        }
        w += increment;
        increment = w >> 1;
    }
}

// The result of a computation that underflowed: a signed zero or the smallest positive `Float`, by
// the rounding mode alone. This is mpfr_underflow from mpfr-impl.h, MPFR 4.2.2, where `Nearest`
// rounds away from zero; the caller substitutes `Down` for the cases where it must not.
fn underflow(positive: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let away = match rm {
        Ceiling => positive,
        Floor => !positive,
        Up | Nearest => true,
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

// Whether |y/x| is below 2^(MIN_EXPONENT - 1), the smallest positive `Float`, so that the quotient
// underflows. MPFR reads this off the division's underflow flag; its exponent range is wide enough
// that the case never arises for representable inputs, while here it does.
//
// |y/x| = (my/mx) 2^d, where d is the difference of the exponents and my and mx, the significands,
// both lie in [1/2, 1). Only the middle binade needs the two significands compared, which the
// shifts below do exactly.
fn quotient_underflows(y: &Float, x: &Float, exp_y: i64, exp_x: i64) -> bool {
    match (exp_y - exp_x).cmp(&(Float::MIN_EXPONENT_I64 - 1)) {
        Less => true,
        Greater => false,
        Equal => (y >> exp_y).lt_abs(&(x >> exp_x)),
    }
}

// atan2(y, x) when |y/x| is beyond the top of the exponent range, so that the quotient is not a
// `Float`. MPFR widens its range for the whole computation and never meets this case; here the
// arctangent has to be taken from its limit instead.
//
// For z > 0, pi/2 - 1/z < atan z < pi/2. With |y/x| > 2^k the result is therefore atan|y/x| = pi/2
// - delta for x > 0, and pi - atan|y/x| = pi/2 + delta for x < 0, where 0 < delta < 2^-k: either
// way it is pi/2 perturbed by less than 2^-k, carrying the sign of y. Since k is at least
// MAX_EXPONENT - 1, that perturbation is far below the rounding error of pi itself at any usable
// precision, and the loop below is the ordinary one for pi/2.
fn atan2_huge_quotient(k: u64, negative: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let mut w = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        // |v - pi/2| <= 2^-w, and EXP(v) = 1, so v is good to min(w, k) - 1 bits once delta is
        // counted too
        let v = Float::pi_prec(w).0 >> 1u32;
        if float_can_round(v.significand_ref().unwrap(), min(w, k) - 1, prec, rm) {
            return Float::from_float_prec_round(if negative { -v } else { v }, prec, rm);
        }
        w += increment;
        increment = w >> 1;
    }
}

// Computes atan2(y, x) for finite nonzero y and x, rounded to precision `prec` with rounding mode
// `rm`.
//
// This is mpfr_atan2 from atan2.c, MPFR 4.2.2, past the special cases.
fn atan2_prec_round_normal_ref(
    y: &Float,
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact atan2");
    let exp_y = i64::from(y.get_exponent().unwrap());
    let exp_x = i64::from(x.get_exponent().unwrap());
    let x_positive = *x > 0u32;
    // When x is a power of two, y/x is exact, so atan takes it directly. The shift is exact only if
    // it stays inside the exponent range, which MPFR checks through the division's flags.
    if x_positive && x.significand_ref().unwrap().is_power_of_2() {
        let shifted = exp_y - exp_x + 1;
        if (Float::MIN_EXPONENT_I64..=Float::MAX_EXPONENT_I64).contains(&shifted) {
            return (y >> (exp_x - 1)).atan_prec_round(prec, rm);
        }
    }
    let y_negative = *y < 0u32;
    // |y/x| lies in (2^(d - 1), 2^(d + 1)), so a d this large puts it beyond the top of the range
    if exp_y - exp_x >= Float::MAX_EXPONENT_I64 {
        return atan2_huge_quotient(u64::exact_from(exp_y - exp_x - 1), y_negative, prec, rm);
    }
    let mut w = prec + 3 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    if x_positive {
        // atan2(y, x) = atan(y/x)
        loop {
            let (t, div_o) = y.div_prec_ref_ref(x, w);
            if div_o == Equal {
                // the quotient is exact, so its arctangent is the whole answer
                return t.atan_prec_round(prec, rm);
            }
            // error <= 1 ulp, except on underflow or overflow
            if quotient_underflows(y, x, exp_y, exp_x) {
                // |atan z| < |z|, so an underflowing quotient gives an underflowing result MPFR
                // takes the sign from the quotient; in this branch x is positive, so it is the sign
                // of y. With `Nearest` a quotient that rounded to zero is below a quarter of the
                // smallest positive `Float`, and rounds toward zero rather than away.
                let rm = if rm == Nearest && t == 0u32 { Down } else { rm };
                return underflow(!y_negative, prec, rm);
            }
            // error <= 2 ulps, since |atan'| <= 1
            let mut t = t;
            t.atan_prec_assign(w);
            if float_can_round(t.significand_ref().unwrap(), w - 2, prec, rm) {
                return Float::from_float_prec_round(t, prec, rm);
            }
            w += increment;
            increment = w >> 1;
        }
    } else {
        // atan2(y, x) = sign(y) (pi - atan|y/x|)
        loop {
            // error <= 1 ulp
            let mut t = y.div_prec_ref_ref(x, w).0.abs();
            // error <= 2 ulps, since |atan'| <= 1
            t.atan_prec_assign(w);
            // error <= 1/2 ulp
            let pi = Float::pi_prec(w).0;
            // if the quotient was zero, so is its arctangent, and |y/x| was below 2^(MIN_EXPONENT -
            // 1)
            let e = if t == 0u32 {
                Float::MIN_EXPONENT_I64 - 1
            } else {
                i64::from(t.get_exponent().unwrap())
            };
            let exp_pi = i64::from(pi.get_exponent().unwrap());
            let t = pi.sub_prec(t, w).0;
            let t = if y_negative { -t } else { t };
            let exp_t = i64::from(t.get_exponent().unwrap());
            // error(t) is at most (1/2 + 2^(EXP(pi) - EXP(t) - 1) + 2^(e - EXP(t) + 1)) ulps, and
            // so at most 2^(max(max(EXP(pi) - EXP(t) - 1, e - EXP(t) + 1), -1) + 2) ulps
            let e = max(max(exp_pi - exp_t - 1, e - exp_t + 1), -1) + 2;
            if e < i64::exact_from(w)
                && float_can_round(
                    t.significand_ref().unwrap(),
                    w - u64::exact_from(e),
                    prec,
                    rm,
                )
            {
                return Float::from_float_prec_round(t, prec, rm);
            }
            w += increment;
            increment = w >> 1;
        }
    }
}

// Computes atan2(y, x) for nonzero `Rational`s y and x, rounded to precision `prec` with rounding
// mode `rm`. (The zero cases are handled by the caller.)
//
// The quotient y/x is exact here, so nothing corresponds to the `Float` case's division, its
// underflow, or its overflow beyond the exponent range: `atan_rational_helper` already covers every
// magnitude, including the two ends where the quotient is not a `Float` at all. Only the negative-x
// reflection needs a loop of its own, and it is MPFR's, with the arctangent taken from the
// `Rational` directly rather than from a rounded quotient.
fn atan2_rational_prec_round_normal_ref(
    y: &Rational,
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact atan2_rational");
    let q = y / x;
    if *x > 0u32 {
        // atan2(y, x) = atan(y/x)
        return atan_rational_helper(&q, prec, rm);
    }
    // atan2(y, x) = sign(y) (pi - atan|y/x|)
    let y_negative = *y < 0u32;
    let aq = q.abs();
    let mut w = prec + 3 + prec.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        // correctly rounded, so the error is at most 1/2 ulp
        let t = atan_rational_helper(&aq, w, Nearest).0;
        // error <= 1/2 ulp
        let pi = Float::pi_prec(w).0;
        let exp_pi = i64::from(pi.get_exponent().unwrap());
        // if the arctangent underflowed to zero, |y/x| was below 2^(MIN_EXPONENT - 1)
        let e = if t == 0u32 {
            Float::MIN_EXPONENT_I64 - 1
        } else {
            i64::from(t.get_exponent().unwrap())
        };
        // pi - atan|y/x| lies in [pi/2, pi], so it is never zero and never cancels
        let t = pi.sub_prec(t, w).0;
        let t = if y_negative { -t } else { t };
        let exp_t = i64::from(t.get_exponent().unwrap());
        // the same bound as the `Float` case, which is conservative here since the arctangent is
        // correctly rounded rather than two ulps out
        let e = max(max(exp_pi - exp_t - 1, e - exp_t + 1), -1) + 2;
        if e < i64::exact_from(w)
            && float_can_round(
                t.significand_ref().unwrap(),
                w - u64::exact_from(e),
                prec,
                rm,
            )
        {
            return Float::from_float_prec_round(t, prec, rm);
        }
        w += increment;
        increment = w >> 1;
    }
}

// The number of bits in MPFR's unsigned long, which bounds u.
const ULSIZE: u64 = 64;
// Wide enough to hold 3u exactly, and so u/2 and u/4 as well.
const AUX_PREC: u64 = ULSIZE + 2;

// z = s 3u 2^-k, with k between 1 and 3. This is mpfr_atan2u_aux2 from atan2u.c, MPFR 4.2.2.
fn atan2u_aux2(u: u64, k: u32, positive: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    // 3u needs at most ULSIZE + 2 bits, so t is exact
    let t = Float::from_unsigned_prec_round(u, AUX_PREC, Exact)
        .0
        .mul_prec_round(const { Float::const_from_unsigned(3) }, AUX_PREC, Exact)
        .0
        >> k;
    Float::from_float_prec_round(if positive { t } else { -t }, prec, rm)
}

// round(s (u/2 - eps)), where eps < 1/2 ulp(u/2). This is mpfr_atan2u_aux3.
fn atan2u_aux3(u: u64, positive: bool, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    // exact, since the working precision is at least ULSIZE
    let mut t = Float::from_unsigned_prec_round(u, max(prec + 2, ULSIZE), Exact).0 >> 1u32;
    // u/2 - 1/4 ulp_p(u/2) <= t <= u/2 for p = prec, which makes t round like u/2 - eps
    t.decrement();
    Float::from_float_prec_round(if positive { t } else { -t }, prec, rm)
}

// round(sign(y) (u/4 - sign(x) eps)), where eps < 1/2 ulp(u/4). This is mpfr_atan2u_aux4.
fn atan2u_aux4(
    u: u64,
    x_positive: bool,
    y_positive: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let w = if prec > ULSIZE { prec + 2 } else { AUX_PREC };
    // exact
    let mut t = Float::from_unsigned_prec_round(u, w, Exact).0 >> 2u32;
    if x_positive {
        t.decrement();
    } else {
        t.increment();
    }
    Float::from_float_prec_round(if y_positive { t } else { -t }, prec, rm)
}

// atan2u(y, x, u) when |y/x| is below the bottom of the exponent range and x is positive.
//
// MPFR reaches this only when the result underflows too, and asserts as much; here a large u can
// lift |y/x| u/(2 pi) back into the range, since Malachite's range is so much narrower. For a |y/x|
// this small atan|y/x| is its own leading term, so the quotient is formed from the numerator scaled
// up by 2^SCALE, exactly as `sin_with_period` and `atan_with_period_rational` do, and the underflow
// that remains is decided by the rounding mode alone.
fn atan2u_tiny(
    y: &Float,
    x: &Float,
    u: u64,
    positive: bool,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // |y| 2^SCALE stays well inside the range: this branch needs EXP(y) <= EXP(x) + MIN_EXPONENT,
    // and EXP(x) is at most MAX_EXPONENT = -MIN_EXPONENT, so EXP(y) is at most 1 x is positive in
    // this branch, so the quotient carries the sign of y; keeping it here rather than taking
    // absolute values is what makes `Up` mean away from zero and lets the rounding mode see the
    // sign it must round with
    let ys = y << SCALE;
    let xa = x.clone();
    let mut w = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    let u_float = Float::from(u);
    loop {
        // rounded away from zero throughout, so each step is a relative 1 + theta with |theta| <=
        // 2^(1 - w)
        let mut t = ys.div_prec_round_ref_ref(&xa, w, Up).0;
        t.mul_prec_round_assign_ref(&u_float, w, Up);
        // 2 pi rounded toward zero, so that the quotient rounds away
        let two_pi = Float::pi_prec_round(w, Down).0 << 1u32;
        t.div_prec_round_assign(two_pi, w, Up);
        if let Some(result) = scaled_underflow(&t, positive, prec, rm) {
            return result;
        }
        let t = t >> SCALE;
        if float_can_round(t.significand_ref().unwrap(), w - 4, prec, rm) {
            return Float::from_float_prec_round(t, prec, rm);
        }
        w += increment;
        increment = w >> 1;
    }
}

// Computes atan2u(y, x, u) = atan2(y, x) u/(2 pi) for finite nonzero y and x with |y| != |x| and
// nonzero u, rounded to precision `prec` with rounding mode `rm`.
//
// This is mpfr_atan2u from atan2u.c, MPFR 4.2.2, past the special cases.
fn atan2_with_period_prec_round_normal_ref(
    y: &Float,
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact atan2_with_period");
    let x_positive = *x > 0u32;
    let y_positive = *y > 0u32;
    // When |y/x| is extreme the result lies astronomically close to a quadrant boundary: u/4 as
    // |y/x| grows without bound, and u/2 as it shrinks to nothing with x negative. If that boundary
    // is also a rounding boundary at the target precision -- that is, if u is representable in prec
    // + 1 bits, so that u/4 and u/2 are either representable or exactly halfway between two
    // representable numbers -- the loop below cannot settle the rounding until its working
    // precision passes |EXP(y) - EXP(x)|, which the exponent range allows to be about 2^31. The two
    // helpers answer such cases directly.
    //
    // MPFR reaches those helpers only when the division returns zero or an infinity, which is a
    // much rarer condition than the situation itself, so mpfr_atan2u hangs here; this is the one
    // place where the port deliberately departs from its structure. Where u is not representable in
    // prec + 1 bits the loop settles quickly, since the boundary then lies strictly inside a
    // rounding interval.
    let exp_y = i64::from(y.get_exponent().unwrap());
    let exp_x = i64::from(x.get_exponent().unwrap());
    let d = exp_y - exp_x;
    let p = i64::exact_from(prec);
    if i64::exact_from(u.significant_bits() - TrailingZeros::trailing_zeros(u)) <= p + 1 {
        // |y/x| >= 2^(d - 1) and u/(2 pi) < 2^(EXP(u) - 2), so u/(2 pi |y/x|) is below half an ulp
        // of u/4 once d >= p + 2
        if d >= p + 2 {
            return atan2u_aux4(u, x_positive, y_positive, prec, rm);
        }
        // |y/x| < 2^(d + 1), so atanu(|y/x|) is below half an ulp of u/2 once d <= -p - 1; for a
        // negative x the result is then just below u/2. For a positive x it is just above zero,
        // which the loop handles, since there the limit is approached relatively rather than
        // absolutely.
        if !x_positive && d < -p {
            return atan2u_aux3(u, y_positive, prec, rm);
        }
    }
    // The periodic arctangent underflows for a tiny quotient with a small u, which MPFR's wider
    // exponent range never sees. This is decided from the exponents rather than from the computed
    // value: an arctangent that rounded up to the smallest positive `Float` is not zero, so a test
    // on the value misses it, and no working precision can ever certify it, so the loop below would
    // spin forever. The bound is the one `sin_with_period` scales at; past it |y/x| is above
    // 2^(MIN_EXPONENT + 65), whose arctangent in u ths of a turn is far clear of the bottom.
    if d <= SCALED_INPUT_EXPONENT {
        return if x_positive {
            atan2u_tiny(y, x, u, y_positive, prec, rm)
        } else {
            // u/2 minus a quantity this small rounds like u/2 stepped one ulp toward zero, whether
            // or not u/2 lies on a rounding boundary
            atan2u_aux3(u, y_positive, prec, rm)
        };
    }
    let log_u = u.ceiling_log_base_2();
    let mut w = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    loop {
        // In atan2pi units the four quadrants are [0, 1/2], [1/2, 1], [-1, -1/2] and [-1/2, 0];
        // here they are [0, u/4], [u/4, u/2], [-u/2, -u/4] and [-u/4, 0].
        let t = y.div_prec_ref_ref(x, w).0;
        // the quotient can still overflow, which MPFR's range does not let it do
        if !t.is_finite() {
            return atan2u_aux4(u, x_positive, y_positive, prec, rm);
        }
        let mut t = t;
        t.abs_assign();
        let exp_t = i64::from(t.get_exponent().unwrap());
        // |t - |y/x|| <= e1 := 1/2 ulp(t) = 2^(exp_t - w - 1)
        t.atan_with_period_prec_assign(u, w);
        // the derivative of atanu(s) is u/(1 + s^2)/(2 pi), so the new t is within 1/2 ulp(t) + e1
        // u/(1 + s^2)/4 of atanu(|y/x|)
        let e = if exp_t < 1 { 0 } else { exp_t - 1 };
        // max(1, |t|) >= 2^e, so 1/(1 + t^2) <= 2^(-2 e)
        let mut e = exp_t - (e << 1) + i64::exact_from(log_u) - 2;
        // now e1 u/(1 + t^2)/4 <= 2^(e - w - 1), so |t - atanu(y/x)| <= 2^(e - w)
        let mut exp_t = i64::from(t.get_exponent().unwrap());
        e = max(e, exp_t);
        if !x_positive {
            // compute u/2 - t
            t <<= 1u32; // error <= 2^(e + 1 - w)
            t = Float::from(u).sub_prec(t, w).0;
            exp_t = i64::from(t.get_exponent().unwrap());
            // error <= 2^(exp_t - w - 1) + 2^(e + 1 - w)
            e = max(exp_t - 1, e + 1);
            // error <= 2^(e + 1 - w)
            t >>= 1u32;
            // error <= 2^(e - w)
            exp_t = i64::from(t.get_exponent().unwrap());
        }
        // either way the error is at most 2^(e - w); expressed relative to t, that is 2^(exp_t - w
        // + err) with err = e - exp_t
        e -= exp_t;
        // atan2u is odd with respect to y
        let t = if y_positive { t } else { -t };
        // a negative e claims better than half-ulp accuracy, which cannot beat t's own precision
        let err = min(i64::exact_from(w), i64::exact_from(w) - e);
        if err > 0 && float_can_round(t.significand_ref().unwrap(), u64::exact_from(err), prec, rm)
        {
            return Float::from_float_prec_round(t, prec, rm);
        }
        w += increment;
        increment = w >> 1;
    }
}

// Computes atan2u(y, x, u) = atan2(y, x) u/(2 pi) for nonzero `Rational`s y and x with |y| != |x|
// and nonzero u, rounded to precision `prec` with rounding mode `rm`. (The rest is handled by the
// caller.)
//
// The quotient y/x is exact here, so nothing corresponds to the `Float` case's division or to its
// underflow and overflow: for a positive x the whole computation is the `Rational` arctangent in u
// ths of a turn, which already covers every magnitude. Only the negative-x reflection needs a loop,
// and it is MPFR's, with the arctangent taken from the `Rational` directly.
fn atan2_with_period_rational_prec_round_normal_ref(
    y: &Rational,
    x: &Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(rm, Exact, "Inexact atan2_with_period_rational");
    let q = y / x;
    if *x > 0u32 {
        // atan2u(y, x, u) = atanu(y/x, u)
        return atan_with_period_rational_helper(&q, u, prec, rm);
    }
    // atan2u(y, x, u) = sign(y) (u/2 - atanu(|y/x|, u))
    let y_positive = *y > 0u32;
    let aq = q.abs();
    let d = aq.floor_log_base_2_abs() + 1;
    let p = i64::exact_from(prec);
    // An arctangent this small underflows, and would leave the loop below with a value it can never
    // certify; u/2 minus it rounds like u/2 stepped one ulp toward zero either way.
    if d <= SCALED_INPUT_EXPONENT {
        return atan2u_aux3(u, y_positive, prec, rm);
    }
    // As in the `Float` case, an extreme quotient puts the result astronomically close to a
    // quadrant boundary, which the loop cannot settle when that boundary is also a rounding
    // boundary. Here |y/x| growing takes the result to u/4 from above, and |y/x| shrinking takes it
    // to u/2 from below.
    if i64::exact_from(u.significant_bits() - TrailingZeros::trailing_zeros(u)) <= p + 1 {
        if d >= p + 2 {
            return atan2u_aux4(u, false, y_positive, prec, rm);
        }
        if d < -p {
            return atan2u_aux3(u, y_positive, prec, rm);
        }
    }
    let mut w = prec + prec.ceiling_log_base_2() + 10;
    let mut increment = Limb::WIDTH;
    loop {
        // correctly rounded, so the error is under an ulp: e below is EXP(t), which states it as
        // 2^(e - w)
        let t = atan_with_period_rational_helper(&aq, u, w, Nearest).0;
        let mut e = i64::from(t.get_exponent().unwrap());
        // u/2 - t, formed as (u - 2 t)/2 so that u stays an integer
        let t = Float::from(u).sub_prec(t << 1u32, w).0;
        let exp_t = i64::from(t.get_exponent().unwrap());
        // error <= 2^(exp_t - w - 1) + 2^(e + 1 - w) <= 2^(e + 1 - w) for the e below
        e = max(exp_t - 1, e + 1);
        let t = t >> 1u32;
        let exp_t = i64::from(t.get_exponent().unwrap());
        // the error is at most 2^(e - w); relative to t that is 2^(exp_t - w + err)
        e -= exp_t;
        // atan2u is odd with respect to y
        let t = if y_positive { t } else { -t };
        let err = min(i64::exact_from(w), i64::exact_from(w) - e);
        if err > 0 && float_can_round(t.significand_ref().unwrap(), u64::exact_from(err), prec, rm)
        {
            return Float::from_float_prec_round(t, prec, rm);
        }
        w += increment;
        increment = w >> 1;
    }
}

// A signed zero, exactly.
const fn signed_zero(negative: bool) -> (Float, Ordering) {
    (
        if negative {
            Float::NEGATIVE_ZERO
        } else {
            Float::ZERO
        },
        Equal,
    )
}

impl Float {
    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Float`]s are both taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded angle is less than, equal to, or greater than the exact
    /// angle. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless the result is a zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::ONE).atan2_prec_round_ref_ref(&Float::ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    ///
    /// // a negative x with a zero y is half a turn
    /// let (t, o) = (&Float::ZERO).atan2_prec_round_ref_ref(&Float::NEGATIVE_ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "3.1406");
    /// assert_eq!(o, Less);
    /// ```
    pub fn atan2_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        let (y, x) = (self, other);
        // atan2 is NaN if either argument is
        if y.is_nan() || x.is_nan() {
            return (Self::NAN, Equal);
        }
        // the quadrant is chosen by the sign bits, so a signed zero behaves like a signed number
        let y_negative = y.is_sign_negative();
        let x_negative = x.is_sign_negative();
        // atan2(+-0, x) = +-pi for x < 0 (or -0.0), and +-0 for x > 0 (or +0.0)
        if *y == 0u32 {
            return if x_negative {
                pi_div_2ui(0, y_negative, prec, rm)
            } else {
                signed_zero(y_negative)
            };
        }
        // atan2(y, +-0) = +-pi/2, with the sign of y
        if *x == 0u32 {
            return pi_div_2ui(1, y_negative, prec, rm);
        }
        if !y.is_finite() {
            // atan2(+-infinity, x) = +-pi/2 for finite x, +-pi/4 for +infinity, +-3pi/4 for
            // -infinity
            return if x.is_finite() {
                pi_div_2ui(1, y_negative, prec, rm)
            } else if x_negative {
                three_pi_over_4(y_negative, prec, rm)
            } else {
                pi_div_2ui(2, y_negative, prec, rm)
            };
        }
        // atan2(+-y, -infinity) = +-pi, atan2(+-y, +infinity) = +-0, for finite nonzero y
        if !x.is_finite() {
            return if x_negative {
                pi_div_2ui(0, y_negative, prec, rm)
            } else {
                signed_zero(y_negative)
            };
        }
        atan2_prec_round_normal_ref(y, x, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`]s are both taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // an eighth of a turn
    /// let (t, o) =
    ///     (&Float::ONE).atan2_with_period_prec_round_ref_ref(&Float::ONE, 360, 10, Exact);
    /// assert_eq!(t.to_string(), "45.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) =
    ///     (&Float::ONE).atan2_with_period_prec_round_ref_ref(&Float::TWO, 360, 10, Floor);
    /// assert_eq!(t.to_string(), "26.562");
    /// assert_eq!(o, Less);
    /// ```
    pub fn atan2_with_period_prec_round_ref_ref(
        &self,
        other: &Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        let (y, x) = (self, other);
        // atan2u is NaN if either argument is
        if y.is_nan() || x.is_nan() {
            return (Self::NAN, Equal);
        }
        // the quadrant is chosen by the sign bits, so a signed zero behaves like a signed number
        let y_positive = y.is_sign_positive();
        let x_positive = x.is_sign_positive();
        if !x.is_finite() {
            if !y.is_finite() {
                return if x_positive {
                    // atan2u(+-infinity, +infinity, u) = +-u/8
                    scaled_unsigned(u, 3, y_positive, prec, rm)
                } else {
                    // atan2u(+-infinity, -infinity, u) = +-3u/8
                    atan2u_aux2(u, 3, y_positive, prec, rm)
                };
            }
            // atan2u(+-y, -infinity, u) = +-u/2 and atan2u(+-y, +infinity, u) = +-0, which are also
            // the IEEE 754-2019 answers for a zero y against a nonzero x
            return if x_positive {
                signed_zero(!y_positive)
            } else {
                scaled_unsigned(u, 1, y_positive, prec, rm)
            };
        }
        // atan2u(+-infinity, x, u) = +-u/4 for a finite x
        if !y.is_finite() {
            return scaled_unsigned(u, 2, y_positive, prec, rm);
        }
        if *y == 0u32 {
            return if x_positive {
                // atan2u(+-0.0, x, u) = +-0.0 for a positive-signed x
                signed_zero(!y_positive)
            } else {
                // atan2u(+-0.0, x, u) = +-u/2 for a negative-signed x
                scaled_unsigned(u, 1, y_positive, prec, rm)
            };
        }
        // atan2u(y, +-0.0, u) = +-u/4, with the sign of y
        if *x == 0u32 {
            return scaled_unsigned(u, 2, y_positive, prec, rm);
        }
        // |y| = |x| puts the angle on a quadrant diagonal, an exact eighth or three eighths of a
        // turn
        if y.eq_abs(x) {
            return if x_positive {
                scaled_unsigned(u, 3, y_positive, prec, rm)
            } else {
                atan2u_aux2(u, 3, y_positive, prec, rm)
            };
        }
        // Every angle measures zero units when the whole turn does. MPFR returns +-1 here for a
        // negative x, which disagrees with its own definition, with the formula it uses for that
        // quadrant (u/2 - atanu, which is 0 - 0), and with the branches above, all of which return
        // zero for u = 0.
        if u == 0 {
            return signed_zero(!y_positive);
        }
        atan2_with_period_prec_round_normal_ref(y, x, u, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified precision
    /// and with the specified rounding mode. The [`Float`]s are both taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,u,p,m) = \operatorname{atan2}(y,x)u/(2\pi)+\varepsilon.
    /// $$
    /// - If $y$ or $x$ is NaN, or the result is one of the exact cases below, $\varepsilon$ may be
    ///   ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)u/(2\pi)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)u/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases, in which the sign of a zero argument selects the quadrant:
    /// - $f(\text{NaN},x,u,p,m)=f(y,\text{NaN},u,p,m)=\text{NaN}$
    /// - $f(\pm\infty,+\infty,u,p,m)=\pm u/8$ and $f(\pm\infty,-\infty,u,p,m)=\pm3u/8$
    /// - $f(\pm\infty,x,u,p,m)=\pm u/4$ for finite $x$
    /// - $f(y,+\infty,u,p,m)=\pm0.0$ and $f(y,-\infty,u,p,m)=\pm u/2$, with the sign of $y$
    /// - $f(\pm0.0,x,u,p,m)=\pm0.0$ if $x$ is positive or $+0.0$, and $\pm u/2$ if $x$ is negative
    ///   or $-0.0$
    /// - $f(y,\pm0.0,u,p,m)=\pm u/4$, with the sign of $y$, for nonzero $y$
    /// - $f(\pm x,x,u,p,m)=\pm u/8$ for positive $x$, and $\pm3u/8$ for negative $x$
    /// - $f(y,x,0,p,m)=\pm0.0$, with the sign of $y$
    ///
    /// These are the only exact cases, and the turn fractions are exact only when $p$ is large
    /// enough to hold them.
    ///
    /// The last is a deliberate divergence from MPFR, whose `mpfr_atan2u` returns $\pm1$ for a
    /// negative $x$ when $u$ is zero. That disagrees with the function's own definition, with the
    /// formula MPFR uses for that quadrant, and with MPFR's own answers when $y$ is zero or
    /// infinite or $|y|=|x|$, all of which are zero.
    ///
    /// Overflow is not possible, since $|f(y,x,u,p,m)| \leq u/2 < 2^{63}$. The result underflows
    /// only for a positive $x$ with $|y/x|$ tiny and $u$ small, where it is about $yu/(2\pi x)$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atan2_with_period_prec`]
    /// instead. If you know that your target precision is the precision of the inputs, consider
    /// using [`Float::atan2_with_period_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`: the quotient is formed at a
    /// working precision of about $n$ bits and its periodic arctangent taken there, which costs the
    /// first term; the second covers the inputs. The magnitudes of the inputs do not drive the
    /// cost.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // an eighth of a turn
    /// let (t, o) = Float::ONE.atan2_with_period_prec_round(Float::ONE, 360, 10, Exact);
    /// assert_eq!(t.to_string(), "45.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) = Float::ONE.atan2_with_period_prec_round(Float::TWO, 360, 10, Floor);
    /// assert_eq!(t.to_string(), "26.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_prec_round(
        self,
        other: Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.atan2_with_period_prec_round_ref_ref(&other, u, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified precision
    /// and with the specified rounding mode. The first [`Float`] is taken by value and the second
    /// by reference. An [`Ordering`] is also returned, indicating whether the rounded angle is less
    /// than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // an eighth of a turn
    /// let (t, o) = Float::ONE.atan2_with_period_prec_round_val_ref(&Float::ONE, 360, 10, Exact);
    /// assert_eq!(t.to_string(), "45.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) = Float::ONE.atan2_with_period_prec_round_val_ref(&Float::TWO, 360, 10, Floor);
    /// assert_eq!(t.to_string(), "26.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_prec_round_val_ref(
        self,
        other: &Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.atan2_with_period_prec_round_ref_ref(other, u, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified precision
    /// and with the specified rounding mode. The first [`Float`] is taken by reference and the
    /// second by value. An [`Ordering`] is also returned, indicating whether the rounded angle is
    /// less than, equal to, or greater than the exact angle. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // an eighth of a turn
    /// let (t, o) = (&Float::ONE).atan2_with_period_prec_round_ref_val(Float::ONE, 360, 10, Exact);
    /// assert_eq!(t.to_string(), "45.000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) = (&Float::ONE).atan2_with_period_prec_round_ref_val(Float::TWO, 360, 10, Floor);
    /// assert_eq!(t.to_string(), "26.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_prec_round_ref_val(
        &self,
        other: Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.atan2_with_period_prec_round_ref_ref(&other, u, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the nearest value of the
    /// specified precision. The [`Float`]s are both taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.atan2_with_period_prec(Float::TWO, 360, 10);
    /// assert_eq!(t.to_string(), "26.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_prec(self, other: Self, u: u64, prec: u64) -> (Self, Ordering) {
        self.atan2_with_period_prec_ref_ref(&other, u, prec)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the nearest value of the
    /// specified precision. The first [`Float`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.atan2_with_period_prec_val_ref(&Float::TWO, 360, 10);
    /// assert_eq!(t.to_string(), "26.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_prec_val_ref(
        self,
        other: &Self,
        u: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        self.atan2_with_period_prec_ref_ref(other, u, prec)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the nearest value of the
    /// specified precision. The first [`Float`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::ONE).atan2_with_period_prec_ref_val(Float::TWO, 360, 10);
    /// assert_eq!(t.to_string(), "26.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_prec_ref_val(
        &self,
        other: Self,
        u: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        self.atan2_with_period_prec_ref_ref(&other, u, prec)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the nearest value of the
    /// specified precision. The [`Float`]s are both taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::ONE).atan2_with_period_prec_ref_ref(&Float::TWO, 360, 10);
    /// assert_eq!(t.to_string(), "26.562");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_with_period_prec_ref_ref(
        &self,
        other: &Self,
        u: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        self.atan2_with_period_prec_round_ref_ref(other, u, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified rounding
    /// mode. The [`Float`]s are both taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function is that one with `prec` the maximum input
    /// precision.
    ///
    /// If you want to specify the output precision, consider using
    /// [`Float::atan2_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::from(0.3f64).atan2_with_period_round(Float::from(0.4f64), 360, Floor);
    /// assert_eq!(t.to_string(), "36.869897645844013");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_round(
        self,
        other: Self,
        u: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.atan2_with_period_prec_round_ref_ref(&other, u, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified rounding
    /// mode. The first [`Float`] is taken by value and the second by reference. An [`Ordering`] is
    /// also returned, indicating whether the rounded angle is less than, equal to, or greater than
    /// the exact angle. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function is that one with `prec` the maximum input
    /// precision.
    ///
    /// If you want to specify the output precision, consider using
    /// [`Float::atan2_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) =
    ///     Float::from(0.3f64).atan2_with_period_round_val_ref(&Float::from(0.4f64), 360, Floor);
    /// assert_eq!(t.to_string(), "36.869897645844013");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_round_val_ref(
        self,
        other: &Self,
        u: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.atan2_with_period_prec_round_ref_ref(other, u, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified rounding
    /// mode. The first [`Float`] is taken by reference and the second by value. An [`Ordering`] is
    /// also returned, indicating whether the rounded angle is less than, equal to, or greater than
    /// the exact angle. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function is that one with `prec` the maximum input
    /// precision.
    ///
    /// If you want to specify the output precision, consider using
    /// [`Float::atan2_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) =
    ///     (&Float::from(0.3f64)).atan2_with_period_round_ref_val(Float::from(0.4f64), 360, Floor);
    /// assert_eq!(t.to_string(), "36.869897645844013");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_round_ref_val(
        &self,
        other: Self,
        u: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.atan2_with_period_prec_round_ref_ref(&other, u, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified rounding
    /// mode. The [`Float`]s are both taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded angle is less than, equal to, or greater than the exact
    /// angle. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function is that one with `prec` the maximum input
    /// precision.
    ///
    /// If you want to specify the output precision, consider using
    /// [`Float::atan2_with_period_prec_round`] instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let y = Float::from(0.3f64);
    /// let x = Float::from(0.4f64);
    /// let (t, o) = (&y).atan2_with_period_round_ref_ref(&x, 360, Floor);
    /// assert_eq!(t.to_string(), "36.869897645844013");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_with_period_round_ref_ref(
        &self,
        other: &Self,
        u: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.atan2_with_period_prec_round_ref_ref(other, u, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified precision
    /// and with the specified rounding mode. The first [`Float`] is replaced by the result, and the
    /// second is taken by value. An [`Ordering`] is returned, indicating whether the rounded angle
    /// is less than, equal to, or greater than the exact angle.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign(Float::TWO, 360, 10, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "26.562");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_prec_round_assign(
        &mut self,
        other: Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (t, o) = self.atan2_with_period_prec_round_ref_ref(&other, u, prec, rm);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified precision
    /// and with the specified rounding mode. The first [`Float`] is replaced by the result, and the
    /// second is taken by reference. An [`Ordering`] is returned, indicating whether the rounded
    /// angle is less than, equal to, or greater than the exact angle.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(
    ///     y.atan2_with_period_prec_round_assign_ref(&Float::TWO, 360, 10, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "26.562");
    /// ```
    #[inline]
    pub fn atan2_with_period_prec_round_assign_ref(
        &mut self,
        other: &Self,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (t, o) = self.atan2_with_period_prec_round_ref_ref(other, u, prec, rm);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the nearest value of the
    /// specified precision. The first [`Float`] is replaced by the result, and the second is taken
    /// by value. An [`Ordering`] is returned, indicating whether the rounded angle is less than,
    /// equal to, or greater than the exact angle.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_with_period_prec_assign(Float::TWO, 360, 10), Less);
    /// assert_eq!(y.to_string(), "26.562");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_prec_assign(&mut self, other: Self, u: u64, prec: u64) -> Ordering {
        let (t, o) = self.atan2_with_period_prec_ref_ref(&other, u, prec);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the nearest value of the
    /// specified precision. The first [`Float`] is replaced by the result, and the second is taken
    /// by reference. An [`Ordering`] is returned, indicating whether the rounded angle is less
    /// than, equal to, or greater than the exact angle.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(
    ///     y.atan2_with_period_prec_assign_ref(&Float::TWO, 360, 10),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "26.562");
    /// ```
    #[inline]
    pub fn atan2_with_period_prec_assign_ref(
        &mut self,
        other: &Self,
        u: u64,
        prec: u64,
    ) -> Ordering {
        let (t, o) = self.atan2_with_period_prec_ref_ref(other, u, prec);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified rounding
    /// mode. The first [`Float`] is replaced by the result, and the second is taken by value. An
    /// [`Ordering`] is returned, indicating whether the rounded angle is less than, equal to, or
    /// greater than the exact angle.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::from(0.3f64);
    /// assert_eq!(
    ///     y.atan2_with_period_round_assign(Float::from(0.4f64), 360, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "36.869897645844013");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_round_assign(
        &mut self,
        other: Self,
        u: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        let (t, o) = self.atan2_with_period_prec_round_ref_ref(&other, u, prec, rm);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified rounding
    /// mode. The first [`Float`] is replaced by the result, and the second is taken by reference.
    /// An [`Ordering`] is returned, indicating whether the rounded angle is less than, equal to, or
    /// greater than the exact angle.
    ///
    /// See [`Float::atan2_with_period_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::from(0.3f64);
    /// assert_eq!(
    ///     y.atan2_with_period_round_assign_ref(&Float::from(0.4f64), 360, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "36.869897645844013");
    /// ```
    #[inline]
    pub fn atan2_with_period_round_assign_ref(
        &mut self,
        other: &Self,
        u: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        let (t, o) = self.atan2_with_period_prec_round_ref_ref(other, u, prec, rm);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Float`]s are both taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded angle is less than, equal to, or greater than the exact
    /// angle. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If $y$ or $x$ is NaN, or the result is a zero, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// Special cases, in which the sign of a zero argument selects the quadrant:
    /// - $f(\text{NaN},x,p,m)=f(y,\text{NaN},p,m)=\text{NaN}$
    /// - $f(\pm0.0,x,p,m)=\pm0.0$ if $x$ is positive or $+0.0$
    /// - $f(\pm0.0,x,p,m)=\pm\pi$ if $x$ is negative or $-0.0$
    /// - $f(y,\pm0.0,p,m)=\pm\pi/2$, with the sign of $y$, for nonzero $y$
    /// - $f(\pm\infty,x,p,m)=\pm\pi/2$ for finite $x$
    /// - $f(\pm\infty,+\infty,p,m)=\pm\pi/4$
    /// - $f(\pm\infty,-\infty,p,m)=\pm3\pi/4$
    /// - $f(y,+\infty,p,m)=\pm0.0$, with the sign of $y$, for finite nonzero $y$
    /// - $f(y,-\infty,p,m)=\pm\pi$, with the sign of $y$, for finite nonzero $y$
    ///
    /// The zeros are the only exact cases; every other result is a nonzero multiple of $\pi$ or an
    /// arctangent, and so is irrational.
    ///
    /// Overflow is not possible, since $|\operatorname{atan2}(y,x)| \leq \pi$. The result
    /// underflows only for a positive $x$ with $|y/x|$ below $2^{-2^{30}}$, where it is about
    /// $y/x$; there $0.0$ or $\pm2^{-2^{30}}$ is returned instead, by the rounding mode alone.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atan2_prec`] instead. If you
    /// know that your target precision is the precision of the inputs, consider using
    /// [`Float::atan2_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(self.significant_bits(), other.significant_bits())`: the quotient is formed at a
    /// working precision of about $n$ bits and its arctangent taken there, which costs the first
    /// term; the second covers the inputs. The magnitudes of the inputs do not drive the cost.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless the result is a zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.atan2_prec_round(Float::ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    ///
    /// // a negative x with a zero y is half a turn
    /// let (t, o) = Float::ZERO.atan2_prec_round(Float::NEGATIVE_ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "3.1406");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(&other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode. The first [`Float`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless the result is a zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.atan2_prec_round_val_ref(&Float::ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    ///
    /// // a negative x with a zero y is half a turn
    /// let (t, o) = Float::ZERO.atan2_prec_round_val_ref(&Float::NEGATIVE_ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "3.1406");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode. The first [`Float`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless the result is a zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::ONE).atan2_prec_round_ref_val(Float::ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    ///
    /// // a negative x with a zero y is half a turn
    /// let (t, o) = (&Float::ZERO).atan2_prec_round_ref_val(Float::NEGATIVE_ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "3.1406");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(&other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision. The
    /// [`Float`]s are both taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_prec_round`] instead.
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
    /// let (t, o) = Float::ONE.atan2_prec(Float::ONE, 10);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(&other, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision. The
    /// first [`Float`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_prec_round`] instead.
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
    /// let (t, o) = Float::ONE.atan2_prec_val_ref(&Float::ONE, 10);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision. The
    /// first [`Float`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_prec_round`] instead.
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
    /// let (t, o) = (&Float::ONE).atan2_prec_ref_val(Float::ONE, 10);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(&other, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision. The
    /// [`Float`]s are both taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_prec_round`] instead.
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
    /// let (t, o) = (&Float::ONE).atan2_prec_ref_ref(&Float::ONE, 10);
    /// assert_eq!(t.to_string(), "0.78516");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.atan2_prec_round_ref_ref(other, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result with the specified rounding mode. The [`Float`]s are
    /// both taken by value. An [`Ordering`] is also returned, indicating whether the rounded angle
    /// is less than, equal to, or greater than the exact angle. Although `NaN`s are not comparable
    /// to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `prec` the maximum input precision.
    ///
    /// If you want to specify the output precision, consider using [`Float::atan2_prec_round`]
    /// instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::from(0.3f64).atan2_round(Float::from(0.4f64), Floor);
    /// assert_eq!(t.to_string(), "0.64350110879328426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.atan2_prec_round_ref_ref(&other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result with the specified rounding mode. The first [`Float`]
    /// is taken by value and the second by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `prec` the maximum input precision.
    ///
    /// If you want to specify the output precision, consider using [`Float::atan2_prec_round`]
    /// instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::from(0.3f64).atan2_round_val_ref(&Float::from(0.4f64), Floor);
    /// assert_eq!(t.to_string(), "0.64350110879328426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.atan2_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result with the specified rounding mode. The first [`Float`]
    /// is taken by reference and the second by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle. Although
    /// `NaN`s are not comparable to any [`Float`], whenever this function returns a `NaN` it also
    /// returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `prec` the maximum input precision.
    ///
    /// If you want to specify the output precision, consider using [`Float::atan2_prec_round`]
    /// instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::from(0.3f64)).atan2_round_ref_val(Float::from(0.4f64), Floor);
    /// assert_eq!(t.to_string(), "0.64350110879328426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.atan2_prec_round_ref_ref(&other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result with the specified rounding mode. The [`Float`]s are
    /// both taken by reference. An [`Ordering`] is also returned, indicating whether the rounded
    /// angle is less than, equal to, or greater than the exact angle. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs. See
    /// [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function is that one with `prec` the maximum input precision.
    ///
    /// If you want to specify the output precision, consider using [`Float::atan2_prec_round`]
    /// instead.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::from(0.3f64)).atan2_round_ref_ref(&Float::from(0.4f64), Floor);
    /// assert_eq!(t.to_string(), "0.64350110879328426");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = max(self.significant_bits(), other.significant_bits());
        self.atan2_prec_round_ref_ref(other, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode. The first [`Float`] is replaced by the result, and the second is taken by
    /// value. An [`Ordering`] is returned, indicating whether the rounded angle is less than, equal
    /// to, or greater than the exact angle.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless the result is a zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_prec_round_assign(Float::ONE, 10, Floor), Less);
    /// assert_eq!(y.to_string(), "0.78516");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_round_assign(
        &mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (t, o) = self.atan2_prec_round_ref_ref(&other, prec, rm);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode. The first [`Float`] is replaced by the result, and the second is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded angle is less than,
    /// equal to, or greater than the exact angle.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless the result is a zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_prec_round_assign_ref(&Float::ONE, 10, Floor), Less);
    /// assert_eq!(y.to_string(), "0.78516");
    /// ```
    #[inline]
    pub fn atan2_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (t, o) = self.atan2_prec_round_ref_ref(other, prec, rm);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision. The
    /// first [`Float`] is replaced by the result, and the second is taken by value. An [`Ordering`]
    /// is returned, indicating whether the rounded angle is less than, equal to, or greater than
    /// the exact angle.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
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
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_prec_assign(Float::ONE, 10), Less);
    /// assert_eq!(y.to_string(), "0.78516");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        let (t, o) = self.atan2_prec_ref_ref(&other, prec);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision. The
    /// first [`Float`] is replaced by the result, and the second is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded angle is less than, equal to, or
    /// greater than the exact angle.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
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
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_prec_assign_ref(&Float::ONE, 10), Less);
    /// assert_eq!(y.to_string(), "0.78516");
    /// ```
    #[inline]
    pub fn atan2_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        let (t, o) = self.atan2_prec_ref_ref(other, prec);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified rounding mode. The first [`Float`]
    /// is replaced by the result, and the second is taken by value. An [`Ordering`] is returned,
    /// indicating whether the rounded angle is less than, equal to, or greater than the exact
    /// angle.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::from(0.3f64);
    /// assert_eq!(y.atan2_round_assign(Float::from(0.4f64), Floor), Less);
    /// assert_eq!(y.to_string(), "0.64350110879328426");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        let (t, o) = self.atan2_prec_round_ref_ref(&other, prec, rm);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified rounding mode. The first [`Float`]
    /// is replaced by the result, and the second is taken by reference. An [`Ordering`] is
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity; this function behaves the same way.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::from(0.3f64);
    /// assert_eq!(y.atan2_round_assign_ref(&Float::from(0.4f64), Floor), Less);
    /// assert_eq!(y.to_string(), "0.64350110879328426");
    /// ```
    #[inline]
    pub fn atan2_round_assign_ref(&mut self, other: &Self, rm: RoundingMode) -> Ordering {
        let prec = max(self.significant_bits(), other.significant_bits());
        let (t, o) = self.atan2_prec_round_ref_ref(other, prec, rm);
        *self = t;
        o
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode and returning the result as a [`Float`]. The [`Rational`]s are both taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded angle is less than,
    /// equal to, or greater than the exact angle.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,p,m) = \operatorname{atan2}(y,x)+\varepsilon.
    /// $$
    /// - If the result is zero, $\varepsilon$ may be ignored or assumed to be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)|\rfloor-p}$.
    ///
    /// The output has precision `prec`.
    ///
    /// Special cases:
    /// - $f(0,x,p,m)=0.0$ if $x \geq 0$, and $\pi$ if $x < 0$
    /// - $f(y,0,p,m)=\pm\pi/2$, with the sign of $y$, for nonzero $y$
    ///
    /// A [`Rational`] has no signed zeros and no infinities, so the quadrant-selecting sign of a
    /// zero argument has no counterpart here: the zero result is a positive zero, and it is the
    /// only exact case.
    ///
    /// Overflow is not possible, since $|\operatorname{atan2}(y,x)| \leq \pi$. The result
    /// underflows only for a positive $x$ with $|y/x|$ below $2^{-2^{30}}$, where it is about
    /// $y/x$; there $0.0$ or $\pm2^{-2^{30}}$ is returned instead, by the rounding mode alone.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::atan2_rational_prec`]
    /// instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`: the quotient is formed exactly, then
    /// rounded once and its [`Float`] arctangent taken at a working precision of about $n$ bits,
    /// which costs the first term; the second covers the inputs. The magnitudes of the inputs do
    /// not drive the cost.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision (which is the case unless the result is zero).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NegativeOne, Zero};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) =
    ///     Float::atan2_rational_prec_round(Rational::from(3), Rational::from(4), 10, Floor);
    /// assert_eq!(t.to_string(), "0.64258");
    /// assert_eq!(o, Less);
    ///
    /// // a negative x with a zero y is half a turn
    /// let (t, o) =
    ///     Float::atan2_rational_prec_round(Rational::ZERO, Rational::NEGATIVE_ONE, 10, Floor);
    /// assert_eq!(t.to_string(), "3.1406");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_rational_prec_round(
        y: Rational,
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::atan2_rational_prec_round_ref(&y, &x, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the specified precision and with the specified
    /// rounding mode and returning the result as a [`Float`]. The [`Rational`]s are both taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded angle is less
    /// than, equal to, or greater than the exact angle.
    ///
    /// See [`Float::atan2_rational_prec_round`] for the error bounds, the special cases, underflow,
    /// and the complexity; this function behaves the same way.
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
    /// let (t, o) = Float::atan2_rational_prec_round_ref(
    ///     &Rational::from(3),
    ///     &Rational::from(4),
    ///     10,
    ///     Ceiling,
    /// );
    /// assert_eq!(t.to_string(), "0.64355");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn atan2_rational_prec_round_ref(
        y: &Rational,
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        // atan2(0, x) = 0 for a nonnegative x and pi for a negative one; a `Rational` zero is
        // unsigned, so there is no negative-zero branch as there is for `Float`s
        if *y == 0u32 {
            return if *x < 0u32 {
                pi_div_2ui(0, false, prec, rm)
            } else {
                (Self::ZERO, Equal)
            };
        }
        // atan2(y, 0) = +-pi/2, with the sign of y
        if *x == 0u32 {
            return pi_div_2ui(1, *y < 0u32, prec, rm);
        }
        atan2_rational_prec_round_normal_ref(y, x, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision and
    /// returning the result as a [`Float`]. The [`Rational`]s are both taken by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_rational_prec_round`] for the error bounds, the special cases, underflow,
    /// and the complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_rational_prec_round`] instead.
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
    /// let (t, o) = Float::atan2_rational_prec(Rational::from(3), Rational::from(4), 10);
    /// assert_eq!(t.to_string(), "0.64355");
    /// assert_eq!(o, Greater);
    ///
    /// let (t, o) = Float::atan2_rational_prec(Rational::from(3), Rational::from(4), 53);
    /// assert_eq!(t.to_string(), "0.64350110879328437");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_rational_prec(y: Rational, x: Rational, prec: u64) -> (Self, Ordering) {
        Self::atan2_rational_prec_round_ref(&y, &x, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, rounding the result to the nearest value of the specified precision and
    /// returning the result as a [`Float`]. The [`Rational`]s are both taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle.
    ///
    /// See [`Float::atan2_rational_prec`] for the error bounds, the special cases, underflow, and
    /// the complexity; this function behaves the same way.
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
    /// let (t, o) = Float::atan2_rational_prec_ref(&Rational::from(3), &Rational::from(4), 53);
    /// assert_eq!(t.to_string(), "0.64350110879328437");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_rational_prec_ref(y: &Rational, x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::atan2_rational_prec_round_ref(y, x, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified precision
    /// and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`]s are both taken by value. An [`Ordering`] is also returned, indicating whether
    /// the rounded angle is less than, equal to, or greater than the exact angle.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(y,x,u,p,m) = \operatorname{atan2}(y,x)u/(2\pi)+\varepsilon.
    /// $$
    /// - If the result is one of the exact cases below, $\varepsilon$ may be ignored or assumed to
    ///   be 0.
    /// - Otherwise, if $m$ is not `Nearest`, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)u/(2\pi)|\rfloor-p+1}$.
    /// - Otherwise, if $m$ is `Nearest`, then $|\varepsilon| \leq 2^{\lfloor\log_2
    ///   |\operatorname{atan2}(y,x)u/(2\pi)|\rfloor-p}$.
    ///
    /// Special cases:
    /// - $f(0,x,u,p,m)=0.0$ if $x \geq 0$, and $u/2$ if $x < 0$
    /// - $f(y,0,u,p,m)=\pm u/4$, with the sign of $y$, for nonzero $y$
    /// - $f(\pm x,x,u,p,m)=\pm u/8$ for positive $x$, and $\pm3u/8$ for negative $x$
    /// - $f(y,x,0,p,m)=0.0$
    ///
    /// These are the only exact cases, and the turn fractions are exact only when $p$ is large
    /// enough to hold them. A [`Rational`] has no NaN, no infinities, and no signed zeros, so the
    /// quadrant-selecting sign of a zero argument has no counterpart here. As in the [`Float`]
    /// case, $u = 0$ gives a zero throughout, where MPFR's `mpfr_atan2u` returns $\pm1$ for a
    /// negative $x$.
    ///
    /// Overflow is not possible, since $|f(y,x,u,p,m)| \leq u/2 < 2^{63}$. The result underflows
    /// only for a positive $x$ with $|y/x|$ tiny and $u$ small.
    ///
    /// The output has precision `prec`.
    ///
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::atan2_with_period_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n (\log n)^3 \log\log n + m (\log m)^2 \log\log m)$
    ///
    /// $M(n, m) = O(n \log n + m \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `prec`, and $m$ is
    /// `max(y.significant_bits(), x.significant_bits())`: the quotient is formed exactly, then
    /// rounded once and its periodic arctangent taken at a working precision of about $n$ bits,
    /// which costs the first term; the second covers the inputs.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::atan2_with_period_rational_prec_round(
    ///     Rational::from(3),
    ///     Rational::from(4),
    ///     360,
    ///     10,
    ///     Floor,
    /// );
    /// assert_eq!(t.to_string(), "36.812");
    /// assert_eq!(o, Less);
    ///
    /// // the first quadrant's diagonal is an eighth of a turn
    /// let (t, o) = Float::atan2_with_period_rational_prec_round(
    ///     Rational::ONE,
    ///     Rational::ONE,
    ///     360,
    ///     10,
    ///     Exact,
    /// );
    /// assert_eq!(t.to_string(), "45.000");
    /// assert_eq!(o, Equal);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_rational_prec_round(
        y: Rational,
        x: Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::atan2_with_period_rational_prec_round_ref(&y, &x, u, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the specified precision
    /// and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`]s are both taken by reference. An [`Ordering`] is also returned, indicating
    /// whether the rounded angle is less than, equal to, or greater than the exact angle.
    ///
    /// See [`Float::atan2_with_period_rational_prec_round`] for the error bounds, the special
    /// cases, underflow, and the complexity; this function behaves the same way.
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
    /// let (t, o) = Float::atan2_with_period_rational_prec_round_ref(
    ///     &Rational::from(3),
    ///     &Rational::from(4),
    ///     360,
    ///     10,
    ///     Ceiling,
    /// );
    /// assert_eq!(t.to_string(), "36.875");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn atan2_with_period_rational_prec_round_ref(
        y: &Rational,
        x: &Rational,
        u: u64,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        // atan2u(0, x, u) = 0 for a nonnegative x and u/2 for a negative one
        if *y == 0u32 {
            return if *x < 0u32 {
                scaled_unsigned(u, 1, true, prec, rm)
            } else {
                (Self::ZERO, Equal)
            };
        }
        let y_positive = *y > 0u32;
        // atan2u(y, 0, u) = +-u/4, with the sign of y
        if *x == 0u32 {
            return scaled_unsigned(u, 2, y_positive, prec, rm);
        }
        // |y| = |x| puts the angle on a quadrant diagonal, an exact eighth or three eighths of a
        // turn
        if y.eq_abs(x) {
            return if *x > 0u32 {
                scaled_unsigned(u, 3, y_positive, prec, rm)
            } else {
                atan2u_aux2(u, 3, y_positive, prec, rm)
            };
        }
        // every angle measures zero units when the whole turn does; see the `Float` version for why
        // this departs from MPFR
        if u == 0 {
            return (Self::ZERO, Equal);
        }
        atan2_with_period_rational_prec_round_normal_ref(y, x, u, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the nearest value of the
    /// specified precision and returning the result as a [`Float`]. The [`Rational`]s are both
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded angle is
    /// less than, equal to, or greater than the exact angle.
    ///
    /// If the angle is equidistant from two [`Float`]s with the specified precision, the [`Float`]
    /// with fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of
    /// the `Nearest` rounding mode.
    ///
    /// See [`Float::atan2_with_period_rational_prec_round`] for the error bounds, the special
    /// cases, underflow, and the complexity; this function is that one with `Nearest`.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::atan2_with_period_rational_prec_round`] instead.
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
    /// let (t, o) =
    ///     Float::atan2_with_period_rational_prec(Rational::from(3), Rational::from(4), 360, 53);
    /// assert_eq!(t.to_string(), "36.869897645844020");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_with_period_rational_prec(
        y: Rational,
        x: Rational,
        u: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        Self::atan2_with_period_rational_prec_round_ref(&y, &x, u, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from
    /// the positive $x$-axis in $u$ths of a turn, rounding the result to the nearest value of the
    /// specified precision and returning the result as a [`Float`]. The [`Rational`]s are both
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded angle
    /// is less than, equal to, or greater than the exact angle.
    ///
    /// See [`Float::atan2_with_period_rational_prec`] for the error bounds, the special cases,
    /// underflow, and the complexity; this function behaves the same way.
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
    /// let (t, o) = Float::atan2_with_period_rational_prec_ref(
    ///     &Rational::from(3),
    ///     &Rational::from(4),
    ///     360,
    ///     53,
    /// );
    /// assert_eq!(t.to_string(), "36.869897645844020");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_with_period_rational_prec_ref(
        y: &Rational,
        x: &Rational,
        u: u64,
        prec: u64,
    ) -> (Self, Ordering) {
        Self::atan2_with_period_rational_prec_round_ref(y, x, u, prec, Nearest)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified precision and with the
    /// specified rounding mode. The [`Float`]s are both taken by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // the first quadrant's diagonal is a quarter turn
    /// let (t, o) = Float::ONE.atan2_pi_prec_round(Float::ONE, 10, Exact);
    /// assert_eq!(t.to_string(), "0.25000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) = Float::ONE.atan2_pi_prec_round(Float::TWO, 10, Floor);
    /// assert_eq!(t.to_string(), "0.14746");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_prec_round(self, other: Self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.atan2_with_period_prec_round(other, 2, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Float`] is taken by value and the second by reference.
    /// An [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal
    /// to, or greater than the exact angle. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // the first quadrant's diagonal is a quarter turn
    /// let (t, o) = Float::ONE.atan2_pi_prec_round_val_ref(&Float::ONE, 10, Exact);
    /// assert_eq!(t.to_string(), "0.25000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) = Float::ONE.atan2_pi_prec_round_val_ref(&Float::TWO, 10, Floor);
    /// assert_eq!(t.to_string(), "0.14746");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_prec_round_val_ref(
        self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.atan2_with_period_prec_round_val_ref(other, 2, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Float`] is taken by reference and the second by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal
    /// to, or greater than the exact angle. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // the first quadrant's diagonal is a quarter turn
    /// let (t, o) = (&Float::ONE).atan2_pi_prec_round_ref_val(Float::ONE, 10, Exact);
    /// assert_eq!(t.to_string(), "0.25000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) = (&Float::ONE).atan2_pi_prec_round_ref_val(Float::TWO, 10, Floor);
    /// assert_eq!(t.to_string(), "0.14746");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_prec_round_ref_val(
        &self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.atan2_with_period_prec_round_ref_val(other, 2, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified precision and with the
    /// specified rounding mode. The [`Float`]s are both taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// // the first quadrant's diagonal is a quarter turn
    /// let (t, o) = (&Float::ONE).atan2_pi_prec_round_ref_ref(&Float::ONE, 10, Exact);
    /// assert_eq!(t.to_string(), "0.25000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) = (&Float::ONE).atan2_pi_prec_round_ref_ref(&Float::TWO, 10, Floor);
    /// assert_eq!(t.to_string(), "0.14746");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_pi_prec_round_ref_ref(
        &self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        self.atan2_with_period_prec_round_ref_ref(other, 2, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the nearest value of the specified
    /// precision. The [`Float`]s are both taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded angle is less than, equal to, or greater than the exact
    /// angle. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.atan2_pi_prec(Float::TWO, 10);
    /// assert_eq!(t.to_string(), "0.14771");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_prec(self, other: Self, prec: u64) -> (Self, Ordering) {
        self.atan2_with_period_prec(other, 2, prec)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the nearest value of the specified
    /// precision. The first [`Float`] is taken by value and the second by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::ONE.atan2_pi_prec_val_ref(&Float::TWO, 10);
    /// assert_eq!(t.to_string(), "0.14771");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_prec_val_ref(self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.atan2_with_period_prec_val_ref(other, 2, prec)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the nearest value of the specified
    /// precision. The first [`Float`] is taken by reference and the second by value. An
    /// [`Ordering`] is also returned, indicating whether the rounded angle is less than, equal to,
    /// or greater than the exact angle. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::ONE).atan2_pi_prec_ref_val(Float::TWO, 10);
    /// assert_eq!(t.to_string(), "0.14771");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_prec_ref_val(&self, other: Self, prec: u64) -> (Self, Ordering) {
        self.atan2_with_period_prec_ref_val(other, 2, prec)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the nearest value of the specified
    /// precision. The [`Float`]s are both taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded angle is less than, equal to, or greater than the exact
    /// angle. Although `NaN`s are not comparable to any [`Float`], whenever this function returns a
    /// `NaN` it also returns `Equal`.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::ONE).atan2_pi_prec_ref_ref(&Float::TWO, 10);
    /// assert_eq!(t.to_string(), "0.14771");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan2_pi_prec_ref_ref(&self, other: &Self, prec: u64) -> (Self, Ordering) {
        self.atan2_with_period_prec_ref_ref(other, 2, prec)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified rounding mode. The
    /// [`Float`]s are both taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s are
    /// not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::from(0.3f64).atan2_pi_round(Float::from(0.4f64), Floor);
    /// assert_eq!(t.to_string(), "0.20483276469913342");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_round(self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        self.atan2_with_period_round(other, 2, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified rounding mode. The
    /// first [`Float`] is taken by value and the second by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = Float::from(0.3f64).atan2_pi_round_val_ref(&Float::from(0.4f64), Floor);
    /// assert_eq!(t.to_string(), "0.20483276469913342");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_round_val_ref(self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        self.atan2_with_period_round_val_ref(other, 2, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified rounding mode. The
    /// first [`Float`] is taken by reference and the second by value. An [`Ordering`] is also
    /// returned, indicating whether the rounded angle is less than, equal to, or greater than the
    /// exact angle. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::from(0.3f64)).atan2_pi_round_ref_val(Float::from(0.4f64), Floor);
    /// assert_eq!(t.to_string(), "0.20483276469913342");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_round_ref_val(&self, other: Self, rm: RoundingMode) -> (Self, Ordering) {
        self.atan2_with_period_round_ref_val(other, 2, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified rounding mode. The
    /// [`Float`]s are both taken by reference. An [`Ordering`] is also returned, indicating whether
    /// the rounded angle is less than, equal to, or greater than the exact angle. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (t, o) = (&Float::from(0.3f64)).atan2_pi_round_ref_ref(&Float::from(0.4f64), Floor);
    /// assert_eq!(t.to_string(), "0.20483276469913342");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_pi_round_ref_ref(&self, other: &Self, rm: RoundingMode) -> (Self, Ordering) {
        self.atan2_with_period_round_ref_ref(other, 2, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Float`] is replaced by the result, and the second is
    /// taken by value. An [`Ordering`] is returned, indicating whether the rounded angle is less
    /// than, equal to, or greater than the exact angle.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_pi_prec_round_assign(Float::TWO, 10, Floor), Less);
    /// assert_eq!(y.to_string(), "0.14746");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_prec_round_assign(
        &mut self,
        other: Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        self.atan2_with_period_prec_round_assign(other, 2, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified precision and with the
    /// specified rounding mode. The first [`Float`] is replaced by the result, and the second is
    /// taken by reference. An [`Ordering`] is returned, indicating whether the rounded angle is
    /// less than, equal to, or greater than the exact angle.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but the result cannot be represented exactly
    /// with the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(
    ///     y.atan2_pi_prec_round_assign_ref(&Float::TWO, 10, Floor),
    ///     Less
    /// );
    /// assert_eq!(y.to_string(), "0.14746");
    /// ```
    #[inline]
    pub fn atan2_pi_prec_round_assign_ref(
        &mut self,
        other: &Self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        self.atan2_with_period_prec_round_assign_ref(other, 2, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the nearest value of the specified
    /// precision. The first [`Float`] is replaced by the result, and the second is taken by value.
    /// An [`Ordering`] is returned, indicating whether the rounded angle is less than, equal to, or
    /// greater than the exact angle.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_pi_prec_assign(Float::TWO, 10), Greater);
    /// assert_eq!(y.to_string(), "0.14771");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_prec_assign(&mut self, other: Self, prec: u64) -> Ordering {
        self.atan2_with_period_prec_assign(other, 2, prec)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the nearest value of the specified
    /// precision. The first [`Float`] is replaced by the result, and the second is taken by
    /// reference. An [`Ordering`] is returned, indicating whether the rounded angle is less than,
    /// equal to, or greater than the exact angle.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_pi_prec_assign_ref(&Float::TWO, 10), Greater);
    /// assert_eq!(y.to_string(), "0.14771");
    /// ```
    #[inline]
    pub fn atan2_pi_prec_assign_ref(&mut self, other: &Self, prec: u64) -> Ordering {
        self.atan2_with_period_prec_assign_ref(other, 2, prec)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified rounding mode. The
    /// first [`Float`] is replaced by the result, and the second is taken by value. An [`Ordering`]
    /// is returned, indicating whether the rounded angle is less than, equal to, or greater than
    /// the exact angle.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_pi_round_assign(Float::TWO, Floor), Less);
    /// assert_eq!(y.to_string(), "0.12");
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_round_assign(&mut self, other: Self, rm: RoundingMode) -> Ordering {
        self.atan2_with_period_round_assign(other, 2, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified rounding mode. The
    /// first [`Float`] is replaced by the result, and the second is taken by reference. An
    /// [`Ordering`] is returned, indicating whether the rounded angle is less than, equal to, or
    /// greater than the exact angle.
    ///
    /// This is `atan2_with_period` with a period of 2: see [`Float::atan2_with_period_prec_round`]
    /// for the error bounds, the special cases, underflow, and the complexity, with $u = 2$. An
    /// infinite $y$ gives $\pm1/4$ against $+\infty$ and $\pm3/4$ against $-\infty$, and $\pm1/2$
    /// against a finite $x$; a zero $y$ gives $\pm0.0$ for a positive-signed $x$ and $\pm1$ for a
    /// negative-signed one; a zero $x$ gives $\pm1/2$; and the quadrant diagonals give $\pm1/4$ and
    /// $\pm3/4$. All of those are exact at every precision except $\pm3/4$, which needs two bits.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the precision of
    /// the inputs.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Two};
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut y = Float::ONE;
    /// assert_eq!(y.atan2_pi_round_assign_ref(&Float::TWO, Floor), Less);
    /// assert_eq!(y.to_string(), "0.12");
    /// ```
    #[inline]
    pub fn atan2_pi_round_assign_ref(&mut self, other: &Self, rm: RoundingMode) -> Ordering {
        self.atan2_with_period_round_assign_ref(other, 2, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified precision and with the
    /// specified rounding mode and returning the result as a [`Float`]. The [`Rational`]s are both
    /// taken by value. An [`Ordering`] is also returned, indicating whether the rounded angle is
    /// less than, equal to, or greater than the exact angle.
    ///
    /// This is `atan2_with_period_rational` with a period of 2: see
    /// [`Float::atan2_with_period_rational_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity, with $u = 2$. A zero $y$ gives $0.0$ for a nonnegative $x$
    /// and $1$ for a negative one, a zero $x$ gives $\pm1/2$ with the sign of $y$, and the quadrant
    /// diagonals give $\pm1/4$ and $\pm3/4$. All of those are exact at every precision except
    /// $\pm3/4$, which needs two bits.
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
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// // the first quadrant's diagonal is a quarter turn
    /// let (t, o) = Float::atan2_pi_rational_prec_round(Rational::ONE, Rational::ONE, 10, Exact);
    /// assert_eq!(t.to_string(), "0.25000");
    /// assert_eq!(o, Equal);
    ///
    /// let (t, o) =
    ///     Float::atan2_pi_rational_prec_round(Rational::from(3), Rational::from(4), 10, Floor);
    /// assert_eq!(t.to_string(), "0.20459");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_rational_prec_round(
        y: Rational,
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::atan2_with_period_rational_prec_round(y, x, 2, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the specified precision and with the
    /// specified rounding mode and returning the result as a [`Float`]. The [`Rational`]s are both
    /// taken by reference. An [`Ordering`] is also returned, indicating whether the rounded angle
    /// is less than, equal to, or greater than the exact angle.
    ///
    /// This is `atan2_with_period_rational` with a period of 2: see
    /// [`Float::atan2_with_period_rational_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity, with $u = 2$. A zero $y$ gives $0.0$ for a nonnegative $x$
    /// and $1$ for a negative one, a zero $x$ gives $\pm1/2$ with the sign of $y$, and the quadrant
    /// diagonals give $\pm1/4$ and $\pm3/4$. All of those are exact at every precision except
    /// $\pm3/4$, which needs two bits.
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
    /// let (t, o) = Float::atan2_pi_rational_prec_round_ref(
    ///     &Rational::from(3),
    ///     &Rational::from(4),
    ///     10,
    ///     Ceiling,
    /// );
    /// assert_eq!(t.to_string(), "0.20483");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn atan2_pi_rational_prec_round_ref(
        y: &Rational,
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::atan2_with_period_rational_prec_round_ref(y, x, 2, prec, rm)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the nearest value of the specified
    /// precision and returning the result as a [`Float`]. The [`Rational`]s are both taken by
    /// value. An [`Ordering`] is also returned, indicating whether the rounded angle is less than,
    /// equal to, or greater than the exact angle.
    ///
    /// This is `atan2_with_period_rational` with a period of 2: see
    /// [`Float::atan2_with_period_rational_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity, with $u = 2$. A zero $y$ gives $0.0$ for a nonnegative $x$
    /// and $1$ for a negative one, a zero $x$ gives $\pm1/2$ with the sign of $y$, and the quadrant
    /// diagonals give $\pm1/4$ and $\pm3/4$. All of those are exact at every precision except
    /// $\pm3/4$, which needs two bits.
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
    /// let (t, o) = Float::atan2_pi_rational_prec(Rational::from(3), Rational::from(4), 53);
    /// assert_eq!(t.to_string(), "0.20483276469913345");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    #[allow(clippy::needless_pass_by_value)]
    pub fn atan2_pi_rational_prec(y: Rational, x: Rational, prec: u64) -> (Self, Ordering) {
        Self::atan2_with_period_rational_prec(y, x, 2, prec)
    }

    /// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis in half-turns, rounding the result to the nearest value of the specified
    /// precision and returning the result as a [`Float`]. The [`Rational`]s are both taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded angle is less
    /// than, equal to, or greater than the exact angle.
    ///
    /// This is `atan2_with_period_rational` with a period of 2: see
    /// [`Float::atan2_with_period_rational_prec_round`] for the error bounds, the special cases,
    /// underflow, and the complexity, with $u = 2$. A zero $y$ gives $0.0$ for a nonnegative $x$
    /// and $1$ for a negative one, a zero $x$ gives $\pm1/2$ with the sign of $y$, and the quadrant
    /// diagonals give $\pm1/4$ and $\pm3/4$. All of those are exact at every precision except
    /// $\pm3/4$, which needs two bits.
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
    /// let (t, o) = Float::atan2_pi_rational_prec_ref(&Rational::from(3), &Rational::from(4), 53);
    /// assert_eq!(t.to_string(), "0.20483276469913345");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn atan2_pi_rational_prec_ref(y: &Rational, x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::atan2_with_period_rational_prec_ref(y, x, 2, prec)
    }
}

impl Atan2<Self> for Float {
    type Output = Self;

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, taking both [`Float`]s by value.
    ///
    /// The precision of the output is the maximum of the precisions of the inputs, and the result
    /// is rounded to nearest. See [`Float::atan2_prec_round`] for the error bounds, the special
    /// cases, underflow, and the complexity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan2;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     Float::from(0.3f64).atan2(Float::from(0.4f64)).to_string(),
    ///     "0.64350110879328437"
    /// );
    /// ```
    #[inline]
    fn atan2(self, other: Self) -> Self {
        self.atan2_round_ref_ref(&other, Nearest).0
    }
}

impl Atan2<&Self> for Float {
    type Output = Self;

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, taking the first [`Float`] by value and the second by reference.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan2;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     Float::from(0.3f64).atan2(&Float::from(0.4f64)).to_string(),
    ///     "0.64350110879328437"
    /// );
    /// ```
    #[inline]
    fn atan2(self, other: &Self) -> Self {
        self.atan2_round_ref_ref(other, Nearest).0
    }
}

impl Atan2<Float> for &Float {
    type Output = Float;

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, taking the first [`Float`] by reference and the second by value.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan2;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     (&Float::from(0.3f64))
    ///         .atan2(Float::from(0.4f64))
    ///         .to_string(),
    ///     "0.64350110879328437"
    /// );
    /// ```
    #[inline]
    fn atan2(self, other: Float) -> Float {
        self.atan2_round_ref_ref(&other, Nearest).0
    }
}

impl Atan2<&Float> for &Float {
    type Output = Float;

    /// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the
    /// positive $x$-axis, taking both [`Float`]s by reference.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan2;
    /// use malachite_float::Float;
    ///
    /// assert_eq!(
    ///     (&Float::from(0.3f64))
    ///         .atan2(&Float::from(0.4f64))
    ///         .to_string(),
    ///     "0.64350110879328437"
    /// );
    /// ```
    #[inline]
    fn atan2(self, other: &Float) -> Float {
        self.atan2_round_ref_ref(other, Nearest).0
    }
}

impl Atan2Assign<Self> for Float {
    /// Replaces a [`Float`] $y$ with $\operatorname{atan2}(y,x)$, taking $x$ by value.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan2Assign;
    /// use malachite_float::Float;
    ///
    /// let mut y = Float::from(0.3f64);
    /// y.atan2_assign(Float::from(0.4f64));
    /// assert_eq!(y.to_string(), "0.64350110879328437");
    /// ```
    #[inline]
    fn atan2_assign(&mut self, other: Self) {
        self.atan2_round_assign_ref(&other, Nearest);
    }
}

impl Atan2Assign<&Self> for Float {
    /// Replaces a [`Float`] $y$ with $\operatorname{atan2}(y,x)$, taking $x$ by reference.
    ///
    /// See [`Float::atan2_prec_round`] for the error bounds, the special cases, underflow, and the
    /// complexity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Atan2Assign;
    /// use malachite_float::Float;
    ///
    /// let mut y = Float::from(0.3f64);
    /// y.atan2_assign(&Float::from(0.4f64));
    /// assert_eq!(y.to_string(), "0.64350110879328437");
    /// ```
    #[inline]
    fn atan2_assign(&mut self, other: &Self) {
        self.atan2_round_assign_ref(other, Nearest);
    }
}

/// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the positive
/// $x$-axis, for primitive floats.
///
/// $$
/// f(y,x) = \operatorname{atan2}(y,x)+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p}$ and $p$ is the
/// precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]); the special cases
/// below are exact.
///
/// Special cases, in which the sign of a zero argument selects the quadrant:
/// - $f(\text{NaN},x)=f(y,\text{NaN})=\text{NaN}$
/// - $f(\pm0.0,x)=\pm0.0$ if $x$ is positive or $+0.0$, and $\pm\pi$ if $x$ is negative or $-0.0$
/// - $f(y,\pm0.0)=\pm\pi/2$, with the sign of $y$, for nonzero $y$
/// - $f(\pm\infty,x)=\pm\pi/2$ for finite $x$, $\pm\pi/4$ for $+\infty$, and $\pm3\pi/4$ for
///   $-\infty$
/// - $f(y,+\infty)=\pm0.0$ and $f(y,-\infty)=\pm\pi$, with the sign of $y$, for finite nonzero $y$
///
/// Overflow is not possible, since $|\operatorname{atan2}(y,x)| \leq \pi$. The result is subnormal,
/// or zero, only for a positive $x$ with $|y/x|$ subnormal or smaller.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::atan2::primitive_float_atan2;
///
/// assert!(primitive_float_atan2(f32::NAN, 1.0).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_atan2(1.0f32, 1.0)),
///     NiceFloat(0.7853982)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atan2(1.0f64, 1.0)),
///     NiceFloat(0.7853981633974483)
/// );
/// // a negative x with a zero y is half a turn
/// assert_eq!(
///     NiceFloat(primitive_float_atan2(0.0f64, -1.0)),
///     NiceFloat(3.141592653589793)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atan2(-0.0f64, -1.0)),
///     NiceFloat(-3.141592653589793)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_atan2<T: PrimitiveFloat>(y: T, x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_to_float_fn(|y, x, prec| y.atan2_prec_ref_ref(&x, prec), y, x)
}

/// Computes $\operatorname{atan2}(y,x)$, the angle of the point $(x,y)$ measured from the positive
/// $x$-axis, for [`Rational`]s, returning the result as a primitive float.
///
/// $$
/// f(y,x) = \operatorname{atan2}(y,x)+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)|\rfloor-p}$ and $p$ is the
/// precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]); the zero case below
/// is exact.
///
/// Special cases:
/// - $f(0,x)=0.0$ if $x \geq 0$, and $\pi$ if $x < 0$
/// - $f(y,0)=\pm\pi/2$, with the sign of $y$, for nonzero $y$
///
/// Overflow is not possible, since $|\operatorname{atan2}(y,x)| \leq \pi$. The result is subnormal,
/// or zero, only for a positive $x$ with $|y/x|$ subnormal or smaller.
///
/// # Worst-case complexity
/// $T(m) = O(m \log m \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `max(y.significant_bits(),
/// x.significant_bits())`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeOne, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::atan2::primitive_float_atan2_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_rational::<f64>(
///         &Rational::from(3),
///         &Rational::from(4)
///     )),
///     NiceFloat(0.6435011087932844)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_rational::<f32>(
///         &Rational::from(3),
///         &Rational::from(4)
///     )),
///     NiceFloat(0.6435011)
/// );
/// // a negative x with a zero y is half a turn
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_rational::<f64>(
///         &Rational::ZERO,
///         &Rational::NEGATIVE_ONE
///     )),
///     NiceFloat(3.141592653589793)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_atan2_rational<T: PrimitiveFloat>(y: &Rational, x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_rational_rational_to_float_fn(Float::atan2_rational_prec_ref, y, x)
}

/// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from the
/// positive $x$-axis in $u$ths of a turn (so that `u = 360` gives degrees), for primitive floats.
///
/// $$
/// f(y,x,u) = \operatorname{atan2}(y,x)u/(2\pi)+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)u/(2\pi)|\rfloor-p}$ and $p$
/// is the precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]); the special
/// cases below are exact when the output can hold them.
///
/// Special cases, in which the sign of a zero argument selects the quadrant:
/// - $f(\text{NaN},x,u)=f(y,\text{NaN},u)=\text{NaN}$
/// - $f(\pm\infty,+\infty,u)=\pm u/8$ and $f(\pm\infty,-\infty,u)=\pm3u/8$
/// - $f(\pm\infty,x,u)=\pm u/4$ for finite $x$
/// - $f(y,+\infty,u)=\pm0.0$ and $f(y,-\infty,u)=\pm u/2$, with the sign of $y$
/// - $f(\pm0.0,x,u)=\pm0.0$ if $x$ is positive or $+0.0$, and $\pm u/2$ otherwise
/// - $f(y,\pm0.0,u)=\pm u/4$, with the sign of $y$, for nonzero $y$
/// - $f(\pm x,x,u)=\pm u/8$ for positive $x$, and $\pm3u/8$ for negative $x$
/// - $f(y,x,0)=\pm0.0$, with the sign of $y$
///
/// Overflow is not possible, since $|f(y,x,u)| \leq u/2 < 2^{63}$. The result is subnormal, or
/// zero, only for a positive $x$ with $|y/x|$ tiny and $u$ small.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::atan2::primitive_float_atan2_with_period;
///
/// assert!(primitive_float_atan2_with_period(f32::NAN, 1.0, 360).is_nan());
/// // the first quadrant's diagonal is an eighth of a turn
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_with_period(1.0f32, 1.0, 360)),
///     NiceFloat(45.0)
/// );
/// // the second quadrant's diagonal is three eighths
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_with_period(1.0f32, -1.0, 360)),
///     NiceFloat(135.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_with_period(3.0f64, 4.0, 360)),
///     NiceFloat(36.86989764584402)
/// );
/// // a negative x with a zero y is half a turn
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_with_period(0.0f64, -1.0, 360)),
///     NiceFloat(180.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_atan2_with_period<T: PrimitiveFloat>(y: T, x: T, u: u64) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_float_float_to_float_fn(
        |y, x, prec| y.atan2_with_period_prec_ref_ref(&x, u, prec),
        y,
        x,
    )
}

/// Computes $\operatorname{atan2}(y,x)u/(2\pi)$, the angle of the point $(x,y)$ measured from the
/// positive $x$-axis in $u$ths of a turn (so that `u = 360` gives degrees), for [`Rational`]s,
/// returning the result as a primitive float.
///
/// $$
/// f(y,x,u) = \operatorname{atan2}(y,x)u/(2\pi)+\varepsilon,
/// $$
/// where $|\varepsilon| < 2^{\lfloor\log_2 |\operatorname{atan2}(y,x)u/(2\pi)|\rfloor-p}$ and $p$
/// is the precision of the output (24 if `T` is a [`f32`] and 53 if `T` is a [`f64`]); the special
/// cases below are exact when the output can hold them.
///
/// Special cases:
/// - $f(0,x,u)=0.0$ if $x \geq 0$, and $u/2$ if $x < 0$
/// - $f(y,0,u)=\pm u/4$, with the sign of $y$, for nonzero $y$
/// - $f(\pm x,x,u)=\pm u/8$ for positive $x$, and $\pm3u/8$ for negative $x$
/// - $f(y,x,0)=0.0$
///
/// Overflow is not possible, since $|f(y,x,u)| \leq u/2 < 2^{63}$. The result is subnormal, or
/// zero, only for a positive $x$ with $|y/x|$ tiny and $u$ small.
///
/// # Worst-case complexity
/// $T(m) = O(m \log m \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `max(y.significant_bits(),
/// x.significant_bits())`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeOne, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::atan2::primitive_float_atan2_with_period_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_with_period_rational::<f64>(
///         &Rational::from(3),
///         &Rational::from(4),
///         360
///     )),
///     NiceFloat(36.86989764584402)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_with_period_rational::<f32>(
///         &Rational::from(3),
///         &Rational::from(4),
///         360
///     )),
///     NiceFloat(36.869896)
/// );
/// // a negative x with a zero y is half a turn
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_with_period_rational::<f64>(
///         &Rational::ZERO,
///         &Rational::NEGATIVE_ONE,
///         360
///     )),
///     NiceFloat(180.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_atan2_with_period_rational<T: PrimitiveFloat>(
    y: &Rational,
    x: &Rational,
    u: u64,
) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    emulate_rational_rational_to_float_fn(
        |y, x, prec| Float::atan2_with_period_rational_prec_ref(y, x, u, prec),
        y,
        x,
    )
}

/// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
/// positive $x$-axis in half-turns, for primitive floats.
///
/// This is `primitive_float_atan2_with_period` with a period of 2: see
/// [`primitive_float_atan2_with_period`] for the error bound and the special cases, with $u = 2$.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::atan2::primitive_float_atan2_pi;
///
/// assert!(primitive_float_atan2_pi(f32::NAN, 1.0).is_nan());
/// // the first quadrant's diagonal is a quarter turn
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_pi(1.0f32, 1.0)),
///     NiceFloat(0.25)
/// );
/// // the second quadrant's is three quarters
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_pi(1.0f32, -1.0)),
///     NiceFloat(0.75)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_pi(3.0f64, 4.0)),
///     NiceFloat(0.20483276469913345)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_atan2_pi<T: PrimitiveFloat>(y: T, x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    primitive_float_atan2_with_period(y, x, 2)
}

/// Computes $\operatorname{atan2}(y,x)/\pi$, the angle of the point $(x,y)$ measured from the
/// positive $x$-axis in half-turns, for [`Rational`]s, returning the result as a primitive float.
///
/// This is `primitive_float_atan2_with_period_rational` with a period of 2: see
/// [`primitive_float_atan2_with_period_rational`] for the error bound and the special cases, with
/// $u = 2$.
///
/// # Worst-case complexity
/// $T(m) = O(m \log m \log\log m)$
///
/// $M(m) = O(m \log m)$
///
/// where $T$ is time, $M$ is additional memory, and $m$ is `max(y.significant_bits(),
/// x.significant_bits())`.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::{NegativeOne, Zero};
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::float::arithmetic::atan2::primitive_float_atan2_pi_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_pi_rational::<f64>(
///         &Rational::from(3),
///         &Rational::from(4)
///     )),
///     NiceFloat(0.20483276469913345)
/// );
/// // a negative x with a zero y is half a turn
/// assert_eq!(
///     NiceFloat(primitive_float_atan2_pi_rational::<f64>(
///         &Rational::ZERO,
///         &Rational::NEGATIVE_ONE
///     )),
///     NiceFloat(1.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_atan2_pi_rational<T: PrimitiveFloat>(y: &Rational, x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float>,
{
    primitive_float_atan2_with_period_rational(y, x, 2)
}
