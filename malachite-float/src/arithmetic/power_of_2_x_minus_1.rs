// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2001-2025 Free Software Foundation, Inc.
//
//      Contributed by the Pascaline and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Infinity, NaN, Zero};
use crate::arithmetic::exp::{exp_overflow, one_neighbor};
use crate::arithmetic::exp_x_minus_1::exp_x_minus_1_rational_near_zero;
use crate::arithmetic::round_near_x::float_round_near_x;
use crate::{
    Float, emulate_float_to_float_fn, emulate_rational_to_float_fn, float_infinity, float_nan,
    float_zero, floor_and_ceiling,
};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, PowerOf2, PowerOf2XMinus1, PowerOf2XMinus1Assign,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, NegativeZero, One, Zero as ZeroTrait};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, RoundingFrom, SaturatingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

// The outcome of the "small |x|" branch of `mpfr_exp2m1`.
enum Small {
    // x * ln(2) enables correct rounding; the contained `Float` is the approximation to round.
    Round(Float),
    // The small approximation did not enable correct rounding.
    NotSmall,
}

// In case x is small in absolute value, 2^x - 1 ~ x * ln(2). If this is enough to deduce correct
// rounding, return the approximation that will be rounded to get the result; otherwise signal that
// the small case does not apply. (The underflow case, where x * ln(2) is smaller than the minimum
// positive `Float`, is handled up front in `power_of_2_x_minus_1_prec_round_normal`, so here x *
// ln(2) is always representable.)
//
// This is mpfr_exp2m1_small from exp2m1.c, MPFR 4.2.2.
fn power_of_2_x_minus_1_small(x: &Float, prec: u64, working_prec: u64, rm: RoundingMode) -> Small {
    let ex = i64::from(x.get_exponent().unwrap());
    // For |x| < 0.125, we have |2^x - 1 - x * ln(2)| < x^2 / 4. Otherwise the approximation is not
    // accurate enough.
    if ex > -3 {
        return Small::NotSmall;
    }
    // t = ln(2) * x * (1 + theta)^2 with |theta| <= 2^(-working_prec).
    let t = Float::ln_2_prec(working_prec)
        .0
        .mul_prec_val_ref(x, working_prec)
        .0;
    let exp_t = i64::from(t.get_exponent().unwrap());
    // |t - x * ln(2)| < 3 * 2^(EXP(t) - working_prec), and |x^2 / 4| < 2^e * 2^(EXP(t) -
    // working_prec), so |t - (2^x - 1)| < 2^e * 2^(EXP(t) - working_prec).
    let e = (ex << 1) - 2 + i64::exact_from(working_prec) - exp_t;
    let e = if e <= 1 { 2 + i64::from(e == 1) } else { e + 1 };
    if float_can_round(
        t.significand_ref().unwrap(),
        working_prec - u64::exact_from(e),
        prec,
        rm,
    ) {
        Small::Round(t)
    } else {
        Small::NotSmall
    }
}

// This is mpfr_exp2m1 from exp2m1.c, MPFR 4.2.2, where the input is finite and nonzero.
fn power_of_2_x_minus_1_prec_round_normal(
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // Huge negative x: if |x| > prec + 1 then 2^x < (1/4) ulp(-1), so 2^x - 1 rounds to -1 (for
    // Floor, Up, and Nearest) or to nextabove(-1) (for Ceiling and Down).
    if x.is_sign_negative() && x.gt_abs(&(prec + 1)) {
        return match rm {
            Ceiling | Down => (-one_neighbor(prec, false), Greater),
            Floor | Up | Nearest => (-Float::one_prec(prec), Less),
            Exact => panic!("Inexact power_of_2_x_minus_1"),
        };
    }
    // Tiny x: 2^x - 1 ~ x * ln(2). Because ln(2) < 1, this can be smaller than the minimum positive
    // `Float` even when x is not (an underflow that e^x - 1 ~ x never produces). Malachite's
    // bounded-exponent arithmetic cannot represent such a sub-minimal intermediate, so the Ziv loop
    // below would never converge; detect and round the underflow here. This is only possible in the
    // smallest binade (|x| >= min_positive), where |2^x - 1| ~ |x| ln(2) lies in (min_positive / 2,
    // min_positive): the lower bound is min_positive * ln(2) > min_positive / 2, so a `Nearest`
    // result always rounds away from zero. Rounding ln(2) to `x`'s precision (rather than the
    // output precision, which may be enormous) is enough to resolve which side of min_positive the
    // result lies on.
    let ex = i64::from(x.get_exponent().unwrap());
    if ex <= i64::from(Float::MIN_EXPONENT) {
        let ln_2 = Float::ln_2_prec(x.significant_bits() + Limb::WIDTH).0;
        // Scale x to exponent 1 before converting to a `Rational`: x itself sits in the smallest
        // binade, so an exact `Rational` for it would carry a ~2^30-bit power-of-2 denominator
        // (~128 MB). With x' = x * 2^(1 - EXP(x)), a small number, the test |x ln(2)| <
        // 2^(MIN_EXPONENT - 1) becomes |x' ln(2)| < 2^(MIN_EXPONENT - ex), which is a comparison of
        // floor(log_2 |x' ln(2)|) against the exponent -- no power-of-2 `Rational` needed.
        let x_scaled = x >> (ex - 1);
        let x_ln_2_scaled = Rational::exact_from(&x_scaled) * Rational::exact_from(&ln_2);
        if x_ln_2_scaled.floor_log_base_2_abs() < i64::from(Float::MIN_EXPONENT) - ex {
            let neg = x.is_sign_negative();
            // A magnitude in (min_positive / 2, min_positive) rounds either away from zero (to
            // +/-min_positive) or toward zero (to +/-0), depending on the rounding mode.
            let away = match rm {
                Up | Nearest => true,
                Ceiling => !neg,
                Floor => neg,
                Down => false,
                Exact => panic!("Inexact power_of_2_x_minus_1"),
            };
            return if away {
                let m = Float::min_positive_value_prec(prec);
                if neg { (-m, Less) } else { (m, Greater) }
            } else if neg {
                (Float::NEGATIVE_ZERO, Greater)
            } else {
                (Float::ZERO, Less)
            };
        }
    }
    // Deeply negative x: 2^x lies below the smallest positive Float, so the loop's
    // `power_of_2_of_float` would return 0 or the minimum positive value with unbounded relative
    // error, and the result could never be certified. Since the huge-negative shortcut above did
    // not fire, |x| <= prec + 1, so the bits of 2^x land within the output's prec-bit window and
    // must be computed for real.
    if *x < const { Float::MIN_EXPONENT as i64 - 1 } {
        return power_of_2_x_minus_1_deep_negative(x, prec, rm);
    }
    // Compute the precision of the intermediary variable: the optimal number of bits, see
    // algorithms.tex.
    let mut working_prec = prec + prec.ceiling_log_base_2() + 6;
    let mut increment = Limb::WIDTH;
    loop {
        // 2^x may overflow.
        let (mut t, o1) = Float::power_of_2_of_float_prec_ref(x, working_prec);
        if t.is_infinite() {
            return exp_overflow(prec, rm);
        }
        // 2^x cannot underflow here: that would require x < MIN_EXPONENT - 1, but then the deep-
        // negative case above would already have returned. Integer x: 2^x is exact, so the result
        // is simply round(2^x - 1).
        if o1 == Equal {
            return t.sub_prec_round(Float::ONE, prec, rm);
        }
        // 2^x is inexact, so x is not an integer and 2^x - 1 is transcendental: never exact.
        assert_ne!(rm, Exact, "Inexact power_of_2_x_minus_1");
        let exp_te = i64::from(t.get_exponent().unwrap());
        t.sub_prec_assign(Float::ONE, working_prec); // 2^x - 1
        if t != 0u32 {
            let exp_t = i64::from(t.get_exponent().unwrap());
            // The error estimate (cf. exp2m1.c): err = max(EXP(2^x) - EXP(2^x - 1), 0) + 1.
            let err = u64::exact_from(max(exp_te - exp_t, 0) + 1);
            if float_can_round(t.significand_ref().unwrap(), working_prec - err, prec, rm) {
                return Float::from_float_prec_round(t, prec, rm);
            }
        }
        // For small |x|, 2^x - 1 ~ x * ln(2); this may enable correct rounding when the
        // cancellation in 2^x - 1 above does not. We must retry it at each Ziv step, since the
        // multiplication x * ln(2) might not give correct rounding at the first loop.
        match power_of_2_x_minus_1_small(x, prec, working_prec, rm) {
            Small::Round(t) => return Float::from_float_prec_round(t, prec, rm),
            Small::NotSmall => {}
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes 2^x - 1 for a Float x < MIN_EXPONENT - 1 (so that 2^x is smaller than the smallest
// positive Float) with |x| <= prec + 1 (larger |x| is handled by the caller's -1-rounding
// shortcut). The result is -1 + 2^x, and since |x| <= prec + 1 the bits of 2^x land within the
// output's prec-bit window, even though 2^x itself is not representable. Split x = -s + f with
// integer s and f in [0, 1): 2^x = 2^f * 2^-s, where 2^f is a normal Float in [1, 2); the scaling
// by 2^-s and the subtraction of 1 are exact over `Rational`s, whose size stays O(prec) because s
// <= prec + 2. For integer x the result is an exact dyadic rational; otherwise 2^f is bracketed and
// the bracket is tightened Ziv-style. The initial working precision is small: the leading s - 1
// bits of the result are a run of ones, so only about prec - s bits of 2^f are needed.
fn power_of_2_x_minus_1_deep_negative(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    let xr = Rational::exact_from(x);
    let neg_n = -Integer::rounding_from(&xr, Floor).0; // = |floor(x)|, since x < 0
    // The conversion fails only for |x| >= 2^63; but |x| <= prec + 1, and a result of such a
    // precision could never be materialized at all.
    let shift = u64::exact_from(&neg_n);
    let frac = xr + Rational::from(neg_n); // x - floor(x), in [0, 1)
    if frac == 0u32 {
        // x is an integer, so -1 + 2^x is an exact dyadic rational needing |x| + 1 bits.
        return Float::from_rational_prec_round(
            Rational::power_of_2(-i64::exact_from(shift)) - Rational::ONE,
            prec,
            rm,
        );
    }
    // x is not an integer, so 2^x - 1 is irrational: never exact.
    assert_ne!(rm, Exact, "Inexact power_of_2_x_minus_1");
    // The fractional part of the Float x is a dyadic rational, exactly representable with as many
    // bits as its numerator has.
    let frac_prec = frac.numerator_ref().significant_bits();
    let frac = Float::from_rational_prec_round(frac, frac_prec, Exact).0;
    let mut working_prec = prec.saturating_sub(shift) + Limb::WIDTH;
    let mut increment = Limb::WIDTH;
    loop {
        // 2^frac is in (1, 2) and irrational, so the Floor rounding is strict: the true value lies
        // strictly between u_lo and u_hi.
        let u = Float::power_of_2_of_float_prec_round_ref(&frac, working_prec, Floor);
        let (u_lo, u_hi) = floor_and_ceiling(u);
        let (f_lo, mut o_lo) = Float::from_rational_prec_round(
            (Rational::exact_from(&u_lo) >> shift) - Rational::ONE,
            prec,
            rm,
        );
        let (f_hi, mut o_hi) = Float::from_rational_prec_round(
            (Rational::exact_from(&u_hi) >> shift) - Rational::ONE,
            prec,
            rm,
        );
        // A bound that is exactly representable at `prec` rounds with `Equal`, but the true value
        // lies strictly between the bounds, so the other bound's ordering is the true one; treat
        // the exact bound as agreeing with it. (Both cannot be `Equal` with equal values, since the
        // bounds are distinct.)
        if o_lo == Equal {
            o_lo = o_hi;
        }
        if o_hi == Equal {
            o_hi = o_lo;
        }
        if o_lo == o_hi && f_lo == f_hi {
            return (f_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes 2^x - 1 for a nonzero `Rational` `x` with `|x| < 2^MIN_EXPONENT`, too small to bracket
// between `Float`s. Since 2^x - 1 = expm1(x ln(2)), bracketing ln(2) between two `Rational`s (from
// a single directed ln(2) computation) and applying `exp_x_minus_1_rational_near_zero` to each
// product brackets the result; that helper's `from_rational_prec_round` calls perform the underflow
// rounding, which matters here because |2^x - 1| ~ |x| ln(2) can fall below the smallest positive
// `Float` (ln(2) < 1). expm1 is increasing, and x ln(2) is increasing in ln(2) for x > 0 and
// decreasing for x < 0, so the bracket ends order accordingly. The bracket width is about |x| 2^-w
// for a working ln(2) precision w, while the result's ulp is about 2^(EXP(x) - prec), so w ~ prec +
// slack suffices regardless of how tiny x is; the Ziv loop widens w if not.
fn power_of_2_x_minus_1_rational_near_zero(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let positive = *x > 0u32;
    let mut working_prec = prec + Limb::WIDTH;
    let mut increment = Limb::WIDTH;
    loop {
        // ln_2_lo <= ln(2) <= ln_2_hi, as exact Rationals, from a single ln(2) computation.
        let (ln_2_lo, ln_2_hi) = floor_and_ceiling(Float::ln_2_prec_round(working_prec, Floor));
        let ln_2_lo = Rational::exact_from(&ln_2_lo);
        let ln_2_hi = Rational::exact_from(&ln_2_hi);
        let (a_lo, a_hi) = if positive {
            (x * ln_2_lo, x * ln_2_hi)
        } else {
            (x * ln_2_hi, x * ln_2_lo)
        };
        let (f_lo, o_lo) = exp_x_minus_1_rational_near_zero(&a_lo, prec, rm);
        let (f_hi, o_hi) = exp_x_minus_1_rational_near_zero(&a_hi, prec, rm);
        if o_lo == o_hi && f_lo == f_hi {
            return (f_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes 2^x - 1 for a nonzero `Rational` `x`, rounded to precision `prec` with rounding mode
// `rm`. (2^0 - 1 = 0 is handled by the caller.) Unlike expm1, the result is exactly representable
// for some inputs: an integer x makes 2^x - 1 an exact dyadic rational, so those are computed
// directly (the Ziv squeeze below could never certify one); every other rational x gives an
// irrational result.
fn power_of_2_x_minus_1_rational_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    if x.is_integer() {
        let n = Integer::exact_from(x);
        return if n > const { Float::MAX_EXPONENT as i64 } {
            // 2^n - 1 has exponent n, beyond the maximum.
            exp_overflow(prec, rm)
        } else if n == const { Float::MAX_EXPONENT as i64 } {
            // 2^n is not representable, but 2^n - 1 (with exponent n) is -- though only with at
            // least n bits of precision. At smaller precisions it rounds exactly like an overflow:
            // down to the largest finite value, or up (and to nearest) past it.
            if prec >= const { Float::MAX_EXPONENT as u64 } {
                Float::from_rational_prec_round(
                    Rational::power_of_2(i64::exact_from(&n)) - Rational::ONE,
                    prec,
                    rm,
                )
            } else {
                exp_overflow(prec, rm)
            }
        } else if n >= i64::from(Float::MIN_EXPONENT) - 1 {
            // 2^n is representable, so round the exact difference 2^n - 1 directly.
            Float::power_of_2(i64::exact_from(&n)).sub_prec_round(Float::ONE, prec, rm)
        } else {
            // n <= MIN_EXPONENT - 2: the result is -1 + 2^n with 2^n below the smallest positive
            // Float. It is exactly representable with 1 - n bits; at prec = -n it sits exactly on
            // the midpoint between -1 and its toward-zero neighbor, so both cases materialize the
            // exact rational (proportional to prec). (The i64 conversion would fail only for |n| >=
            // 2^63, where prec >= |n| means the result could not be materialized at all.)
            let bits_needed = Integer::ONE - &n;
            if n >= -i64::exact_from(prec) {
                Float::from_rational_prec_round(
                    Rational::power_of_2(i64::exact_from(&n)) - Rational::ONE,
                    prec,
                    rm,
                )
            } else {
                // The result is within 2^n < (1/4) ulp(-1) of -1, and err = 1 - n > prec + 1, so
                // rounding from -1 always resolves.
                assert_ne!(rm, Exact, "Inexact power_of_2_x_minus_1");
                let err = u64::saturating_from(&bits_needed);
                float_round_near_x(&Float::NEGATIVE_ONE, err, false, prec, rm).unwrap()
            }
        };
    }
    // The result for a non-integer rational is irrational, hence never exactly representable.
    assert_ne!(rm, Exact, "Inexact power_of_2_x_minus_1");
    let positive = *x > 0u32;
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // x is too small to be represented as a normal Float (|x| < 2^MIN_EXPONENT). The squeeze below
    // cannot bracket it (its Float bounds would be 0), so use the ln(2)-bracketing helper, which
    // also performs the underflow rounding that |x| ln(2) may need.
    if exp_x <= const { Float::MIN_EXPONENT as i64 } {
        return power_of_2_x_minus_1_rational_near_zero(x, prec, rm);
    }
    // |x| is too large to be a finite Float. For x > 0, 2^x - 1 overflows; for x < 0 it tends to
    // -1. Smaller x that still overflow or round to -1 are handled by the Float function inside the
    // squeeze below.
    if exp_x >= const { Float::MAX_EXPONENT as i64 } {
        if positive {
            return exp_overflow(prec, rm);
        }
        // 2^x is far below ulp(-1) at any precision, so 2^x - 1 rounds to -1 or its toward-zero
        // neighbor.
        let err = const { Float::MAX_EXPONENT as u64 };
        if let Some(result) = float_round_near_x(&Float::NEGATIVE_ONE, err, false, prec, rm) {
            return result;
        }
        // `prec` is enormous (>= MAX_EXPONENT), so `float_round_near_x` cannot resolve the
        // rounding; but 2^x is still far below ulp(-1), so -1 rounds the same way.
        return match rm {
            Ceiling | Down => (-one_neighbor(prec, false), Greater), // -1 + ulp (toward zero)
            _ => (-Float::one_prec(prec), Less),                     // -1
        };
    }
    // General case: bracket x between the Floats x_lo <= x <= x_hi, apply 2^x - 1 to both, and
    // increase the working precision until the two bounds round to the same result. 2^x - 1 is
    // monotonic, so once the bounds agree the exact result (which lies between them) rounds the
    // same way. A bound that lands on an integer produces an `Equal` ordering, which cannot match
    // the other (irrational) bound's; the loop then simply tightens the bracket until neither bound
    // is an integer.
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    loop {
        let (x_lo, x_o) = Float::from_rational_prec_round_ref(x, working_prec, Floor);
        if x_o == Equal {
            // x is exactly representable at `working_prec`, so the result is simply that of the
            // Float function.
            return x_lo.power_of_2_x_minus_1_prec_round(prec, rm);
        }
        let (x_lo, x_hi) = floor_and_ceiling((x_lo, x_o));
        let (e_lo, o_lo) = x_lo.power_of_2_x_minus_1_prec_round_ref(prec, rm);
        let (e_hi, o_hi) = x_hi.power_of_2_x_minus_1_prec_round_ref(prec, rm);
        if o_lo == o_hi && e_lo == e_hi {
            return (e_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $2^x-1$, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Float`] is taken by value.
    #[inline]
    pub fn power_of_2_x_minus_1_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.power_of_2_x_minus_1_prec_round_ref(prec, rm)
    }

    /// Computes $2^x-1$, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Float`] is taken by reference.
    #[inline]
    pub fn power_of_2_x_minus_1_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            // 2^(-inf) - 1 = -1
            Self(Infinity { sign: false }) => (Self::from_signed_prec(-1i32, prec).0, Equal),
            // 2^(±0) - 1 = ±0
            Self(Zero { sign }) => (Self(Zero { sign: *sign }), Equal),
            _ => power_of_2_x_minus_1_prec_round_normal(self, prec, rm),
        }
    }

    /// Computes $2^x-1$, rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by value.
    #[inline]
    pub fn power_of_2_x_minus_1_prec(self, prec: u64) -> (Self, Ordering) {
        self.power_of_2_x_minus_1_prec_round(prec, Nearest)
    }

    /// Computes $2^x-1$, rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by reference.
    #[inline]
    pub fn power_of_2_x_minus_1_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.power_of_2_x_minus_1_prec_round_ref(prec, Nearest)
    }

    /// Computes $2^x-1$, rounding the result with the specified rounding mode. The precision of the
    /// output is the precision of the input. The [`Float`] is taken by value.
    #[inline]
    pub fn power_of_2_x_minus_1_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.power_of_2_x_minus_1_prec_round(prec, rm)
    }

    /// Computes $2^x-1$, rounding the result with the specified rounding mode. The precision of the
    /// output is the precision of the input. The [`Float`] is taken by reference.
    #[inline]
    pub fn power_of_2_x_minus_1_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.power_of_2_x_minus_1_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $2^x-1$ in place, rounding the result to the specified precision and with the
    /// specified rounding mode.
    #[inline]
    pub fn power_of_2_x_minus_1_prec_round_assign(
        &mut self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = core::mem::take(self).power_of_2_x_minus_1_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $2^x-1$ in place, rounding the result to the nearest value of the specified
    /// precision.
    #[inline]
    pub fn power_of_2_x_minus_1_prec_assign(&mut self, prec: u64) -> Ordering {
        self.power_of_2_x_minus_1_prec_round_assign(prec, Nearest)
    }

    /// Computes $2^x-1$ in place, rounding the result with the specified rounding mode. The
    /// precision of the output is the precision of the input.
    #[inline]
    pub fn power_of_2_x_minus_1_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.power_of_2_x_minus_1_prec_round_assign(prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $2^x-1$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value.
    #[inline]
    pub fn power_of_2_x_minus_1_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::power_of_2_x_minus_1_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $2^x-1$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference.
    #[inline]
    pub fn power_of_2_x_minus_1_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // 2^0 - 1 = 0, exactly.
            return (float_zero!(), Equal);
        }
        power_of_2_x_minus_1_rational_helper(x, prec, rm)
    }

    /// Computes $2^x-1$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by value.
    #[inline]
    pub fn power_of_2_x_minus_1_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::power_of_2_x_minus_1_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $2^x-1$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by reference.
    #[inline]
    pub fn power_of_2_x_minus_1_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::power_of_2_x_minus_1_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl PowerOf2XMinus1 for Float {
    type Output = Self;

    /// Computes $2^x-1$, where $x$ is a [`Float`], taking the [`Float`] by value.
    #[inline]
    fn power_of_2_x_minus_1(self) -> Self {
        let prec = self.significant_bits();
        self.power_of_2_x_minus_1_prec(prec).0
    }
}

impl PowerOf2XMinus1 for &Float {
    type Output = Float;

    /// Computes $2^x-1$, where $x$ is a [`Float`], taking the [`Float`] by reference.
    #[inline]
    fn power_of_2_x_minus_1(self) -> Float {
        self.power_of_2_x_minus_1_prec_round_ref(self.significant_bits(), Nearest)
            .0
    }
}

impl PowerOf2XMinus1Assign for Float {
    /// Computes $2^x-1$, where $x$ is a [`Float`], in place.
    #[inline]
    fn power_of_2_x_minus_1_assign(&mut self) {
        let prec = self.significant_bits();
        self.power_of_2_x_minus_1_prec_round_assign(prec, Nearest);
    }
}

/// Computes $2^x-1$ for a primitive float. The result is correctly rounded. Using this function is
/// more accurate than computing `x.exp2() - 1.0` with the primitive float functions: that
/// subtraction loses all precision when $x$ is small (where $2^x-1\approx x\ln 2$ but $2^x$ rounds
/// to 1), and the standard library's `exp2` is not correctly rounded to begin with.
///
/// $$
/// f(x) = 2^x-1+\varepsilon.
/// $$
/// - If $2^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $2^x-1$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |2^x-1|\rfloor-p}$,
///   where $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=\infty$
/// - $f(-\infty)=-1$
/// - $f(\pm0.0)=\pm0.0$
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::power_of_2_x_minus_1::primitive_float_power_of_2_x_minus_1;
///
/// assert!(primitive_float_power_of_2_x_minus_1(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2_x_minus_1(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2_x_minus_1(f32::NEGATIVE_INFINITY)),
///     NiceFloat(-1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2_x_minus_1(3.0f32)),
///     NiceFloat(7.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_power_of_2_x_minus_1<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::power_of_2_x_minus_1_prec, x)
}

/// Computes $2^x-1$, where $x$ is a [`Rational`], returning the result as a primitive float. The
/// result is correctly rounded.
///
/// $$
/// f(x) = 2^x-1+\varepsilon.
/// $$
/// - If $2^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $2^x-1$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |2^x-1|\rfloor-p}$,
///   where $p$ is the precision of the output (typically 24 if `T` is a [`f32`] and 53 if `T` is a
///   [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=0$
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::power_of_2_x_minus_1::primitive_float_power_of_2_x_minus_1_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2_x_minus_1_rational::<f32>(
///         &Rational::from(3u32)
///     )),
///     NiceFloat(7.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_2_x_minus_1_rational::<f32>(
///         &Rational::from_signeds(-1i32, 2)
///     )),
///     NiceFloat(-0.29289323)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
pub fn primitive_float_power_of_2_x_minus_1_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::power_of_2_x_minus_1_rational_prec_ref, x)
}
