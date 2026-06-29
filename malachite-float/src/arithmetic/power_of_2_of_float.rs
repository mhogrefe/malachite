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

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::arithmetic::exp::{exp_overflow, exp_underflow};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CeilingLogBase2, IsPowerOf2, PowerOf2, PowerOf2Assign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, Zero as ZeroTrait,
};
use malachite_base::num::conversion::traits::{ExactFrom, IsInteger, RoundingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::float_extras::float_can_round;
use malachite_nz::platform::{Limb, SignedLimb};

// This is mpfr_exp2 from exp2.c, MPFR 4.2.2, where the input is finite and nonzero.
fn power_of_2_of_float_prec_round_normal(
    x: &Float,
    precy: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    // 2^x overflows once x >= MAX_EXPONENT, and underflows once x <= MIN_EXPONENT - 2 (the smallest
    // representable positive value is 2^(MIN_EXPONENT - 1)).
    if *x >= const { Float::const_from_signed(Float::MAX_EXPONENT as SignedLimb) } {
        return exp_overflow(precy, rm);
    }
    if *x <= const { Float::const_from_signed((Float::MIN_EXPONENT as SignedLimb) - 2) } {
        return exp_underflow(precy, rm);
    }
    // We now know that MIN_EXPONENT - 2 < x < MAX_EXPONENT, so the integer part fits in an i64.
    let xint = i64::exact_from(&Integer::rounding_from(x, Down).0); // trunc(x), toward zero
    // If x is an integer, 2^x is a power of 2, hence exact.
    if x.is_integer() {
        return Float::power_of_2_prec_round(xint, precy, rm);
    }
    // 2^x for a non-integer Float is transcendental, hence never exactly representable.
    assert_ne!(rm, Exact, "Inexact power_of_2_of_float");
    // 2^x = 2^xint * 2^xfrac, where xfrac = x - xint and |xfrac| < 1. We compute 2^xfrac =
    // exp(xfrac * ln(2)) and then multiply by 2^xint by shifting the result's exponent.
    let p = x.get_prec().unwrap();
    let xfrac = if xint == 0 {
        x.clone()
    } else {
        // x - xint is exact: the difference has fewer significant bits than x.
        let xint_f = Float::from_integer_prec(Integer::from(xint), p).0;
        x.sub_prec_round_ref_val(xint_f, p, Floor).0
    };
    // The working precision and error estimate are from algorithms.tex (mpfr_exp2).
    let mut working_prec = precy + 5 + precy.ceiling_log_base_2();
    let mut increment = Limb::WIDTH;
    loop {
        let ln_2 = Float::ln_2_prec_round(working_prec, Up).0;
        let t = xfrac.mul_prec_round_ref_val(ln_2, working_prec, Up).0; // xfrac * ln(2)
        // Error estimate (cf. mpfr_exp2): the relative error of t (computed with two roundings) is
        // bounded so that exp(t) is correct to `err` bits.
        let err = u64::exact_from(
            i64::exact_from(working_prec) - (i64::from(t.get_exponent().unwrap()) + 2),
        );
        let (t, _) = t.exp_prec_round(working_prec, Nearest); // exp(xfrac * ln(2))
        if float_can_round(t.significand_ref().unwrap(), err, precy, rm) {
            let (y, inexact) = Float::from_float_prec_round(t, precy, rm);
            // Multiply by 2^xint. Special case (mpfr_exp2): if `Nearest` rounded 2^xfrac down to
            // 1/2 and xint = MIN_EXPONENT - 1, the unrounded result is the midpoint between 0 and
            // the smallest positive Float, which is a double-rounding problem: round up to that
            // smallest value instead of underflowing.
            if rm == Nearest
                && xint == const { (Float::MIN_EXPONENT as i64) - 1 }
                && y.get_exponent() == Some(0)
                && (&y).is_power_of_2()
            {
                return (Float::min_positive_value_prec(precy), Greater);
            }
            return (y << xint, inexact);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}

impl Float {
    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the specified precision and
    /// with the specified rounding mode. The exponent is taken by value.
    #[inline]
    pub fn power_of_2_of_float_prec_round(
        pow: Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        Self::power_of_2_of_float_prec_round_ref(&pow, prec, rm)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the specified precision and
    /// with the specified rounding mode. The exponent is taken by reference.
    pub fn power_of_2_of_float_prec_round_ref(
        pow: &Float,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match &pow.0 {
            NaN => (Self::NAN, Equal),
            // 2^(+inf) = +inf; 2^(-inf) = +0
            Infinity { sign } => {
                if *sign {
                    (Self::INFINITY, Equal)
                } else {
                    (Self::ZERO, Equal)
                }
            }
            // 2^(+0) = 2^(-0) = 1
            Zero { .. } => (Self::one_prec(prec), Equal),
            Finite { .. } => power_of_2_of_float_prec_round_normal(pow, prec, rm),
        }
    }

    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The exponent is taken by value.
    #[inline]
    pub fn power_of_2_of_float_prec(pow: Float, prec: u64) -> (Self, Ordering) {
        Self::power_of_2_of_float_prec_round_ref(&pow, prec, Nearest)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the nearest value of the
    /// specified precision. The exponent is taken by reference.
    #[inline]
    pub fn power_of_2_of_float_prec_ref(pow: &Float, prec: u64) -> (Self, Ordering) {
        Self::power_of_2_of_float_prec_round_ref(pow, prec, Nearest)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result with the specified rounding
    /// mode. The output precision is the precision of the exponent. The exponent is taken by value.
    #[inline]
    pub fn power_of_2_of_float_round(pow: Float, rm: RoundingMode) -> (Self, Ordering) {
        let prec = pow.significant_bits();
        Self::power_of_2_of_float_prec_round_ref(&pow, prec, rm)
    }

    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result with the specified rounding
    /// mode. The output precision is the precision of the exponent. The exponent is taken by
    /// reference.
    #[inline]
    pub fn power_of_2_of_float_round_ref(pow: &Float, rm: RoundingMode) -> (Self, Ordering) {
        let prec = pow.significant_bits();
        Self::power_of_2_of_float_prec_round_ref(pow, prec, rm)
    }

    /// Replaces $x$ with $2^x$, rounding the result to the specified precision and with the
    /// specified rounding mode.
    #[inline]
    pub fn power_of_2_of_float_prec_round_assign(
        &mut self,
        prec: u64,
        rm: RoundingMode,
    ) -> Ordering {
        let (result, o) = Self::power_of_2_of_float_prec_round_ref(self, prec, rm);
        *self = result;
        o
    }

    /// Replaces $x$ with $2^x$, rounding the result to the nearest value of the specified precision.
    #[inline]
    pub fn power_of_2_of_float_prec_assign(&mut self, prec: u64) -> Ordering {
        self.power_of_2_of_float_prec_round_assign(prec, Nearest)
    }

    /// Replaces $x$ with $2^x$, rounding the result with the specified rounding mode. The precision
    /// of $x$ is unchanged.
    #[inline]
    pub fn power_of_2_of_float_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.power_of_2_of_float_prec_round_assign(prec, rm)
    }
}

impl PowerOf2<Float> for Float {
    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the nearest value with the
    /// precision of $x$. The exponent is taken by value.
    #[inline]
    fn power_of_2(pow: Float) -> Float {
        Float::power_of_2_of_float_round(pow, Nearest).0
    }
}

impl PowerOf2<&Float> for Float {
    /// Computes $2^x$, where $x$ is a [`Float`], rounding the result to the nearest value with the
    /// precision of $x$. The exponent is taken by reference.
    #[inline]
    fn power_of_2(pow: &Float) -> Float {
        Float::power_of_2_of_float_round_ref(pow, Nearest).0
    }
}

impl PowerOf2Assign for Float {
    /// Replaces $x$ with $2^x$, rounding the result to the nearest value with the precision of $x$.
    #[inline]
    fn power_of_2_assign(&mut self) {
        self.power_of_2_of_float_round_assign(Nearest);
    }
}
