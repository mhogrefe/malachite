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
use crate::{Float, float_infinity, float_nan};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, PowerOf2XMinus1, PowerOf2XMinus1Assign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeZero, One, Zero as ZeroTrait};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
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
        // Integer x: 2^x is exact, so the result is simply round(2^x - 1).
        if o1 == Equal {
            return t.sub_prec_round(Float::ONE, prec, rm);
        }
        // 2^x is inexact, so x is not an integer and 2^x - 1 is transcendental: never exact.
        assert_ne!(rm, Exact, "Inexact power_of_2_x_minus_1");
        // 2^x cannot underflow here: that would require x < MIN_EXPONENT - 1, but then the huge-
        // negative case above would already have returned.
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
            Self(Infinity { sign: false }) => (Float::from_signed_prec(-1i32, prec).0, Equal),
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
