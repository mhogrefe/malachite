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
    CeilingLogBase2, Pow, PowerOf10XMinus1, PowerOf10XMinus1Assign, Reciprocal,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, One};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::Limb;
use malachite_q::Rational;

const MAX_EXPONENT_U64: u64 = Float::MAX_EXPONENT as u64;
const MAX_EXPONENT_I64: i64 = Float::MAX_EXPONENT as i64;
const MIN_EXPONENT_I64: i64 = Float::MIN_EXPONENT as i64;
const TEN: Rational = Rational::const_from_unsigned(10);

// The outcome of the "small |x|" branch of `mpfr_exp10m1`.
enum Small {
    // x * ln(10) enables correct rounding; the contained `Float` is the approximation to round.
    Round(Float),
    // The small approximation did not enable correct rounding.
    NotSmall,
}

// Decides `x * log2(10) >= bound` exactly, where `x` is a nonzero `Rational` and `bound` an
// integer. Since log2(10) is irrational and `x` is a nonzero rational, `x * log2(10)` is irrational
// and hence never equals the integer `bound`, so the widening bracket always resolves. Used to
// place a `Float` x against the overflow (10^x >= 2^MAX_EXPONENT) and underflow (10^x <
// 2^MIN_EXPONENT) boundaries of 10^x, which occur at x = MAX_EXPONENT / log2(10) and x =
// MIN_EXPONENT / log2(10) respectively.
fn x_log_2_10_ge(x: &Rational, bound: i64) -> bool {
    let bound = Rational::from(bound);
    let positive = *x > 0u32;
    let mut p = const { Limb::WIDTH << 1 };
    loop {
        let (lo, hi) = floor_and_ceiling(Float::log_2_10_prec_round(p, Floor));
        let lo = Rational::exact_from(lo);
        let hi = Rational::exact_from(hi);
        // log2(10) in [lo, hi], so x * log2(10) in [a_lo, a_hi].
        let (a_lo, a_hi) = if positive {
            (x * lo, x * hi)
        } else {
            (x * hi, x * lo)
        };
        if a_lo >= bound {
            return true;
        }
        if a_hi < bound {
            return false;
        }
        p <<= 1;
    }
}

// In case x is small in absolute value, 10^x - 1 ~ x * ln(10). If this is enough to deduce correct
// rounding, return the approximation that will be rounded to get the result; otherwise signal that
// the small case does not apply. (Unlike 2^x - 1, no underflow can occur, since ln(10) > 1, so x *
// ln(10) is always at least as large as x in magnitude and hence representable.)
//
// This is mpfr_exp10m1_small from exp10m1.c, MPFR 4.3.0.
fn power_of_10_x_minus_1_small(x: &Float, prec: u64, working_prec: u64, rm: RoundingMode) -> Small {
    let ex = i64::from(x.get_exponent().unwrap());
    // For |x| < 0.25, we have |10^x - 1 - x * ln(10)| < 4 * x^2. Otherwise the approximation is not
    // accurate enough.
    if ex > -2 {
        return Small::NotSmall;
    }
    // t = ln(10) * x * (1 + theta)^2 with |theta| <= 2^(-working_prec).
    let t = Float::ln_10_prec(working_prec)
        .0
        .mul_prec_val_ref(x, working_prec)
        .0;
    let exp_t = i64::from(t.get_exponent().unwrap());
    // |t - x * ln(10)| < 3 * 2^(EXP(t) - working_prec), and |4 * x^2| < 2^e * 2^(EXP(t) -
    // working_prec), so |t - (10^x - 1)| < 2^e * 2^(EXP(t) - working_prec).
    let e = (ex << 1) + 2 + i64::exact_from(working_prec) - exp_t;
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

// This is mpfr_exp10m1 from exp10m1.c, MPFR 4.3.0, where the input is finite and nonzero.
fn power_of_10_x_minus_1_prec_round_normal(
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // Huge negative x: if |x| > 2 + (prec - 1) / 3 then 3 |x| >= prec + 3, so 10^x < 8^x <=
    // 2^(-prec-3) <= (1/2) ulp(-1); thus 10^x - 1 rounds to -1 (for Floor, Up, and Nearest) or to
    // nextabove(-1) (for Ceiling and Down).
    if x.is_sign_negative() && x.gt_abs(&(2 + (prec - 1) / 3)) {
        return match rm {
            Ceiling | Down => (-one_neighbor(prec, false), Greater),
            Floor | Up | Nearest => (Float::negative_one_prec(prec), Less),
            Exact => panic!("Inexact power_of_10_x_minus_1"),
        };
    }
    // Deeply negative x: 10^x lies below the smallest positive Float (x * log2(10) < MIN_EXPONENT),
    // so the loop's `power_of_10` would return 0 or the minimum positive value with unbounded
    // relative error, and the result could never be certified. Since the huge-negative shortcut
    // above did not fire, |x| <= 2 + (prec - 1) / 3, so the bits of 10^x land within the output's
    // prec-bit window and must be computed for real. This is only reachable at enormous prec: a
    // sub-`MIN_EXPONENT` 10^x needs |x| > -MIN_EXPONENT / log2(10) > -MIN_EXPONENT / 4, and the
    // shortcut leaves only |x| <= 2 + (prec - 1) / 3. The cheap `prec` gate below skips the (up to
    // 128 MB) `Rational::exact_from(x)` in the common case, where the branch cannot fire.
    if x.is_sign_negative()
        && 2 + (prec - 1) / 3 >= const { MAX_EXPONENT_U64 >> 2 }
        && !x_log_2_10_ge(&Rational::exact_from(x), MIN_EXPONENT_I64)
    {
        return power_of_10_x_minus_1_deep_negative(x, prec, rm);
    }
    // Compute the precision of the intermediary variable: the optimal number of bits, see
    // algorithms.tex.
    let mut working_prec = prec + prec.ceiling_log_base_2() + 6;
    let mut increment = Limb::WIDTH;
    loop {
        // 10^x may overflow.
        let (mut t, o1) = Float::power_of_10_of_float_prec_ref(x, working_prec);
        if t.is_infinite() {
            // 10^x overflowed at the working precision. For prec < MAX_EXPONENT this decides the
            // result: 10^x >= (1 - 2^-working_prec) * 2^MAX_EXPONENT, so 10^x - 1 exceeds the
            // largest prec-bit value and rounds exactly as an overflow does. At prec >=
            // MAX_EXPONENT the values just below 2^MAX_EXPONENT are representable, and the
            // intermediate overflow no longer implies one in the result:
            // - x * log2(10) >= MAX_EXPONENT: a true overflow. (Unlike base 2, 10^x is never
            //   exactly 2^MAX_EXPONENT, so there is no representable boundary value to
            //   materialize.)
            // - x * log2(10) < MAX_EXPONENT: 10^x < 2^MAX_EXPONENT strictly, so the overflow is an
            //   artifact of rounding 10^x up at the working precision; growing it far enough (past
            //   the distance from x * log2(10) to MAX_EXPONENT) makes 10^x finite, and the loop
            //   proceeds normally.
            if prec < MAX_EXPONENT_U64 || x_log_2_10_ge(&Rational::exact_from(x), MAX_EXPONENT_I64)
            {
                return exp_overflow(prec, rm);
            }
            working_prec += increment;
            increment = working_prec >> 1;
            continue;
        }
        // 10^x cannot underflow here: that would require x * log2(10) < MIN_EXPONENT, but then the
        // deep-negative case above would already have returned. Integer x: 10^x is exact, so the
        // result is simply round(10^x - 1).
        if o1 == Equal {
            return t.sub_prec_round(Float::ONE, prec, rm);
        }
        // 10^x is inexact, so x is not an integer and 10^x - 1 is transcendental: never exact.
        assert_ne!(rm, Exact, "Inexact power_of_10_x_minus_1");
        let exp_te = i64::from(t.get_exponent().unwrap());
        t.sub_prec_assign(Float::ONE, working_prec); // 10^x - 1
        if t != 0u32 {
            let exp_t = i64::from(t.get_exponent().unwrap());
            // The error estimate (cf. exp10m1.c): err = max(EXP(10^x) - EXP(10^x - 1), 0) + 1.
            let err = u64::exact_from(max(exp_te - exp_t, 0) + 1);
            if float_can_round(t.significand_ref().unwrap(), working_prec - err, prec, rm) {
                return Float::from_float_prec_round(t, prec, rm);
            }
        }
        // For small |x|, 10^x - 1 ~ x * ln(10); this may enable correct rounding when the
        // cancellation in 10^x - 1 above does not. We must retry it at each Ziv step, since the
        // multiplication x * ln(10) might not give correct rounding at the first loop.
        match power_of_10_x_minus_1_small(x, prec, working_prec, rm) {
            Small::Round(t) => return Float::from_float_prec_round(t, prec, rm),
            Small::NotSmall => {}
        }
        // Increase the precision.
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

// Computes 10^x - 1 for a Float x with x * log2(10) < MIN_EXPONENT (so that 10^x is smaller than
// the smallest positive Float) and |x| <= 2 + (prec - 1) / 3 (larger |x| is handled by the caller's
// -1-rounding shortcut). The result is -1 + 10^x, and since |x| is bounded the bits of 10^x land
// within the output's prec-bit window, even though 10^x itself is not representable. Split x = -s +
// f with integer s >= 1 and f in [0, 1): 10^x = 10^f / 10^s, where 10^f is a normal Float in [1,
// 10); the division by 10^s and the subtraction of 1 are exact over `Rational`s, whose size stays
// O(prec) because 10^s has about 3.32 * s <= prec bits. For integer x the result is an exact
// rational; otherwise 10^f is bracketed and the bracket is tightened Ziv-style. The initial working
// precision is small: the leading ~3.32 * s bits of the result are a run of ones, so only about
// prec - 3.32 * s bits of 10^f are needed.
fn power_of_10_x_minus_1_deep_negative(
    x: &Float,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let xr = Rational::exact_from(x);
    let neg_n = -Integer::rounding_from(&xr, Floor).0; // = |floor(x)| = s, since x < 0
    let shift = u64::exact_from(&neg_n);
    let ten_pow_s = TEN.pow(shift); // 10^s
    let frac = xr + Rational::from(neg_n); // x - floor(x), in [0, 1)
    if frac == 0u32 {
        // x is an integer, so -1 + 10^x = -1 + 1 / 10^s is an exact rational.
        return Float::from_rational_prec_round(ten_pow_s.reciprocal() - Rational::ONE, prec, rm);
    }
    // x is not an integer, so 10^x - 1 is irrational: never exact.
    assert_ne!(rm, Exact, "Inexact power_of_10_x_minus_1");
    // The fractional part of the Float x is a dyadic rational, exactly representable with as many
    // bits as its numerator has.
    let frac_prec = frac.numerator_ref().significant_bits();
    let frac = Float::from_rational_prec_round(frac, frac_prec, Exact).0;
    let mut working_prec = prec.saturating_sub(3 * shift) + Limb::WIDTH;
    let mut increment = Limb::WIDTH;
    loop {
        // 10^frac is in [1, 10) and irrational, so the Floor rounding is strict: the true value
        // lies strictly between u_lo and u_hi.
        let u = Float::power_of_10_of_float_prec_round_ref(&frac, working_prec, Floor);
        let (u_lo, u_hi) = floor_and_ceiling(u);
        let (f_lo, mut o_lo) = Float::from_rational_prec_round(
            Rational::exact_from(u_lo) / &ten_pow_s - Rational::ONE,
            prec,
            rm,
        );
        let (f_hi, mut o_hi) = Float::from_rational_prec_round(
            Rational::exact_from(u_hi) / &ten_pow_s - Rational::ONE,
            prec,
            rm,
        );
        // A bound that is exactly representable at `prec` rounds with `Equal`, but the true value
        // lies strictly between the bounds, so the other bound's ordering is the true one; treat
        // the exact bound as agreeing with it.
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

// Computes 10^x - 1 for a nonzero `Rational` `x` with x * log2(10) < MIN_EXPONENT (so 10^x is too
// small to bracket between `Float`s). Since 10^x - 1 = expm1(x ln(10)), bracketing ln(10) between
// two `Rational`s (from a single directed ln(10) computation) and applying
// `exp_x_minus_1_rational_near_zero` to each product brackets the result. expm1 is increasing, and
// x ln(10) is increasing in ln(10) for x > 0 and decreasing for x < 0, so the bracket ends order
// accordingly.
fn power_of_10_x_minus_1_rational_near_zero(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    let positive = *x > 0u32;
    let mut working_prec = prec + Limb::WIDTH;
    let mut increment = Limb::WIDTH;
    loop {
        // ln_10_lo <= ln(10) <= ln_10_hi, as exact Rationals, from a single ln(10) computation.
        let (ln_10_lo, ln_10_hi) = floor_and_ceiling(Float::ln_10_prec_round(working_prec, Floor));
        let ln_10_lo = Rational::exact_from(ln_10_lo);
        let ln_10_hi = Rational::exact_from(ln_10_hi);
        let (a_lo, a_hi) = if positive {
            (x * ln_10_lo, x * ln_10_hi)
        } else {
            (x * ln_10_hi, x * ln_10_lo)
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

// Computes 10^x - 1 for a nonzero `Rational` `x`, rounded to precision `prec` with rounding mode
// `rm`. (10^0 - 1 = 0 is handled by the caller.) Unlike expm1, the result is exactly representable
// for some inputs: a nonnegative integer x makes 10^x - 1 an exact integer, and a negative integer
// x makes it an exact (non-dyadic) rational, so those are computed directly (the Ziv squeeze below
// could never certify one); every other rational x gives an irrational result.
fn power_of_10_x_minus_1_rational_helper(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    if x.is_integer() {
        let n = Integer::exact_from(x);
        return if n > 0 {
            // 10^n - 1 overflows when 10^n - 1 >= 2^MAX_EXPONENT, i.e. n * log2(10) >=
            // MAX_EXPONENT.
            if x_log_2_10_ge(x, MAX_EXPONENT_I64) {
                exp_overflow(prec, rm)
            } else {
                // 10^n - 1 is an exact integer with at most MAX_EXPONENT bits; round it directly.
                let value = TEN.pow(u64::exact_from(&n)) - Rational::ONE;
                Float::from_rational_prec_round(value, prec, rm)
            }
        } else {
            // n < 0: 10^n - 1 = (1 - 10^|n|) / 10^|n| lies in (-1, 0) and is never exactly
            // representable (its denominator 10^|n| has a factor of 5^|n|).
            assert_ne!(rm, Exact, "Inexact power_of_10_x_minus_1");
            // If 10^n >= 2^(-prec-2), then |n| log2(10) = O(prec), so materializing 10^|n| stays
            // cheap. Otherwise 10^n < 2^(-prec-2) (i.e. n * log2(10) < -prec-2), so the result is
            // within (1/4) ulp(-1) of -1: round directly from -1 with err = prec + 2, which
            // satisfies `float_round_near_x`'s |result + 1| = 10^n < 2^(1 - err) and err > prec +
            // 1.
            if x_log_2_10_ge(x, -i64::exact_from(prec) - 2) {
                let s = u64::exact_from(&-&n);
                let value = TEN.pow(s).reciprocal() - Rational::ONE;
                Float::from_rational_prec_round(value, prec, rm)
            } else {
                float_round_near_x(&Float::NEGATIVE_ONE, prec + 2, false, prec, rm).unwrap()
            }
        };
    }
    // The result for a non-integer rational is irrational, hence never exactly representable.
    assert_ne!(rm, Exact, "Inexact power_of_10_x_minus_1");
    let positive = *x > 0u32;
    let exp_x = x.floor_log_base_2_abs() + 1; // the MPFR-style exponent of x
    // x is too small to be represented as a normal Float (|x| < 2^MIN_EXPONENT). The squeeze below
    // cannot bracket it (its Float bounds would be 0), so use the ln(10)-bracketing helper, which
    // reduces 10^x - 1 to expm1(x ln(10)) for the tiny (near-zero) argument.
    if exp_x <= MIN_EXPONENT_I64 {
        return power_of_10_x_minus_1_rational_near_zero(x, prec, rm);
    }
    // |x| is too large to be a finite Float. For x > 0, 10^x - 1 overflows; for x < 0 it tends to
    // -1. Smaller x that still overflow or round to -1 are handled by the Float function inside the
    // squeeze below.
    if exp_x >= MAX_EXPONENT_I64 {
        if positive {
            return exp_overflow(prec, rm);
        }
        // 10^x is far below ulp(-1) at any precision, so 10^x - 1 rounds to -1 or its toward-zero
        // neighbor.
        if let Some(result) =
            float_round_near_x(&Float::NEGATIVE_ONE, MAX_EXPONENT_U64, false, prec, rm)
        {
            return result;
        }
        // `prec` is enormous (>= MAX_EXPONENT), so `float_round_near_x` cannot resolve the
        // rounding; but 10^x is still far below ulp(-1), so -1 rounds the same way.
        return match rm {
            Ceiling | Down => (-one_neighbor(prec, false), Greater), // -1 + ulp (toward zero)
            _ => (-Float::one_prec(prec), Less),                     // -1
        };
    }
    // General case: bracket x between the Floats x_lo <= x <= x_hi, apply 10^x - 1 to both, and
    // increase the working precision until the two bounds round to the same result. 10^x - 1 is
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
            return x_lo.power_of_10_x_minus_1_prec_round(prec, rm);
        }
        let (x_lo, x_hi) = floor_and_ceiling((x_lo, x_o));
        let (e_lo, o_lo) = x_lo.power_of_10_x_minus_1_prec_round_ref(prec, rm);
        let (e_hi, o_hi) = x_hi.power_of_10_x_minus_1_prec_round_ref(prec, rm);
        if o_lo == o_hi && e_lo == e_hi {
            return (e_lo, o_lo);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $10^x-1$, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Float`] is taken by value.
    #[inline]
    pub fn power_of_10_x_minus_1_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.power_of_10_x_minus_1_prec_round_ref(prec, rm)
    }

    /// Computes $10^x-1$, rounding the result to the specified precision and with the specified
    /// rounding mode. The [`Float`] is taken by reference.
    pub fn power_of_10_x_minus_1_prec_round_ref(
        &self,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN) => (float_nan!(), Equal),
            float_infinity!() => (float_infinity!(), Equal),
            // 10^(-inf) - 1 = -1
            Self(Infinity { sign: false }) => (Self::from_signed_prec(-1i32, prec).0, Equal),
            // 10^(±0) - 1 = ±0
            Self(Zero { .. }) => (self.clone(), Equal),
            _ => power_of_10_x_minus_1_prec_round_normal(self, prec, rm),
        }
    }

    /// Computes $10^x-1$, rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by value.
    #[inline]
    pub fn power_of_10_x_minus_1_prec(self, prec: u64) -> (Self, Ordering) {
        self.power_of_10_x_minus_1_prec_round(prec, Nearest)
    }

    /// Computes $10^x-1$, rounding the result to the nearest value of the specified precision. The
    /// [`Float`] is taken by reference.
    #[inline]
    pub fn power_of_10_x_minus_1_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.power_of_10_x_minus_1_prec_round_ref(prec, Nearest)
    }

    /// Computes $10^x-1$, rounding the result with the specified rounding mode. The precision of
    /// the output is the precision of the input. The [`Float`] is taken by value.
    #[inline]
    pub fn power_of_10_x_minus_1_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.power_of_10_x_minus_1_prec_round(prec, rm)
    }

    /// Computes $10^x-1$, rounding the result with the specified rounding mode. The precision of
    /// the output is the precision of the input. The [`Float`] is taken by reference.
    #[inline]
    pub fn power_of_10_x_minus_1_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        self.power_of_10_x_minus_1_prec_round_ref(self.significant_bits(), rm)
    }

    /// Computes $10^x-1$ in place, rounding the result to the specified precision and with the
    /// specified rounding mode.
    #[inline]
    pub fn power_of_10_x_minus_1_prec_round_assign(
        &mut self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = core::mem::take(self).power_of_10_x_minus_1_prec_round(prec, rm);
        *self = result;
        o
    }

    /// Computes $10^x-1$ in place, rounding the result to the nearest value of the specified
    /// precision.
    #[inline]
    pub fn power_of_10_x_minus_1_prec_assign(&mut self, prec: u64) -> Ordering {
        self.power_of_10_x_minus_1_prec_round_assign(prec, Nearest)
    }

    /// Computes $10^x-1$ in place, rounding the result with the specified rounding mode. The
    /// precision of the output is the precision of the input.
    #[inline]
    pub fn power_of_10_x_minus_1_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.power_of_10_x_minus_1_prec_round_assign(prec, rm)
    }

    #[allow(clippy::needless_pass_by_value)]
    /// Computes $10^x-1$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value.
    #[inline]
    pub fn power_of_10_x_minus_1_rational_prec_round(
        x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::power_of_10_x_minus_1_rational_prec_round_ref(&x, prec, rm)
    }

    /// Computes $10^x-1$, where $x$ is a [`Rational`], rounding the result to the specified
    /// precision and with the specified rounding mode and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference.
    #[inline]
    pub fn power_of_10_x_minus_1_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            // 10^0 - 1 = 0, exactly.
            return (float_zero!(), Equal);
        }
        power_of_10_x_minus_1_rational_helper(x, prec, rm)
    }

    /// Computes $10^x-1$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by value.
    #[inline]
    pub fn power_of_10_x_minus_1_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::power_of_10_x_minus_1_rational_prec_round(x, prec, Nearest)
    }

    /// Computes $10^x-1$, where $x$ is a [`Rational`], rounding the result to the nearest value of
    /// the specified precision and returning the result as a [`Float`]. The [`Rational`] is taken
    /// by reference.
    #[inline]
    pub fn power_of_10_x_minus_1_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::power_of_10_x_minus_1_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl PowerOf10XMinus1 for Float {
    type Output = Self;

    /// Computes $10^x-1$, where $x$ is a [`Float`], taking the [`Float`] by value.
    #[inline]
    fn power_of_10_x_minus_1(self) -> Self {
        let prec = self.significant_bits();
        self.power_of_10_x_minus_1_prec(prec).0
    }
}

impl PowerOf10XMinus1 for &Float {
    type Output = Float;

    /// Computes $10^x-1$, where $x$ is a [`Float`], taking the [`Float`] by reference.
    #[inline]
    fn power_of_10_x_minus_1(self) -> Float {
        self.power_of_10_x_minus_1_prec_round_ref(self.significant_bits(), Nearest)
            .0
    }
}

impl PowerOf10XMinus1Assign for Float {
    /// Computes $10^x-1$, where $x$ is a [`Float`], in place.
    #[inline]
    fn power_of_10_x_minus_1_assign(&mut self) {
        let prec = self.significant_bits();
        self.power_of_10_x_minus_1_prec_round_assign(prec, Nearest);
    }
}

/// Computes $10^x-1$ for a primitive float. The result is correctly rounded. Using this function is
/// more accurate than computing `10.0.powf(x) - 1.0` with the primitive float functions: that
/// subtraction loses all precision when $x$ is small (where $10^x-1\approx x\ln 10$ but $10^x$
/// rounds to 1), and the standard library's `powf` is not correctly rounded to begin with.
///
/// $$
/// f(x) = 10^x-1+\varepsilon.
/// $$
/// - If $10^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $10^x-1$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |10^x-1|\rfloor-p}$,
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
/// use malachite_float::arithmetic::power_of_10_x_minus_1::primitive_float_power_of_10_x_minus_1;
///
/// assert!(primitive_float_power_of_10_x_minus_1(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_10_x_minus_1(f32::INFINITY)),
///     NiceFloat(f32::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_10_x_minus_1(
///         f32::NEGATIVE_INFINITY
///     )),
///     NiceFloat(-1.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_10_x_minus_1(3.0f32)),
///     NiceFloat(999.0)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_power_of_10_x_minus_1<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::power_of_10_x_minus_1_prec, x)
}

/// Computes $10^x-1$, where $x$ is a [`Rational`], returning the result as a primitive float. The
/// result is correctly rounded.
///
/// $$
/// f(x) = 10^x-1+\varepsilon.
/// $$
/// - If $10^x-1$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $10^x-1$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2 |10^x-1|\rfloor-p}$,
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
/// use malachite_float::arithmetic::power_of_10_x_minus_1::primitive_float_power_of_10_x_minus_1_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_10_x_minus_1_rational::<f32>(
///         &Rational::from(3u32)
///     )),
///     NiceFloat(999.0)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_power_of_10_x_minus_1_rational::<f32>(
///         &Rational::from_signeds(-1i32, 2)
///     )),
///     NiceFloat(-0.6837722)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
#[cfg_attr(dylint_lib = "malachite_lints", expect(long_lines))]
pub fn primitive_float_power_of_10_x_minus_1_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::power_of_10_x_minus_1_rational_prec_ref, x)
}
