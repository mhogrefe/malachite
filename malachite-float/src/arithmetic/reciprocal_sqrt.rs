// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright 2008-2024 Free Software Foundation, Inc.
//
//      Contributed by the AriC and Caramba projects, INRIA.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::arithmetic::sqrt::generic_sqrt_rational;
use crate::conversion::from_natural::{
    from_natural_prec_round_zero_exponent_ref, from_natural_zero_exponent,
    from_natural_zero_exponent_ref,
};
use crate::conversion::from_rational::FROM_RATIONAL_THRESHOLD;
use crate::{
    Float, emulate_float_to_float_fn, emulate_rational_to_float_fn, float_either_zero,
    float_infinity, float_nan, float_zero, significand_bits,
};
use core::cmp::Ordering::{self, *};
use core::cmp::max;
use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, CheckedSqrt, FloorLogBase2, IsPowerOf2, NegAssign, NegModPowerOf2, Parity,
    PowerOf2, Reciprocal, ReciprocalAssign, ReciprocalSqrt, ReciprocalSqrtAssign,
    RoundToMultipleOfPowerOf2, Sqrt, UnsignedAbs,
};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeZero, Zero as ZeroTrait,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom, SaturatingFrom};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::natural::LIMB_HIGH_BIT;
use malachite_nz::natural::arithmetic::float_extras::{
    float_can_round, limbs_float_can_round, limbs_significand_slice_add_limb_in_place,
};
use malachite_nz::natural::arithmetic::float_reciprocal_sqrt::limbs_reciprocal_sqrt;
use malachite_nz::natural::{Natural, bit_to_limb_count_ceiling, limb_to_bit_count};
use malachite_nz::platform::Limb;
use malachite_q::Rational;

fn from_reciprocal_rational_prec_round_ref_direct(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let sign = *x >= 0;
    if let Some(pow) = x.numerator_ref().checked_log_base_2() {
        let n = x.denominator_ref();
        let n_bits = n.significant_bits();
        let (mut y, mut o) =
            from_natural_prec_round_zero_exponent_ref(n, prec, if sign { rm } else { -rm });
        o = y.shr_prec_round_assign_helper(
            i128::from(pow) - i128::from(n_bits),
            prec,
            if sign { rm } else { -rm },
            o,
        );
        assert!(
            rm != Exact || o == Equal,
            "Inexact conversion from Rational to Float"
        );
        if sign { (y, o) } else { (-y, o.reverse()) }
    } else {
        let x = x.reciprocal();
        let mut exponent = i32::saturating_from(x.floor_log_base_2_abs());
        if exponent >= Float::MAX_EXPONENT {
            return match (sign, rm) {
                (true, Up | Ceiling | Nearest) => (Float::INFINITY, Greater),
                (true, Floor | Down) => (Float::max_finite_value_with_prec(prec), Less),
                (false, Up | Floor | Nearest) => (Float::NEGATIVE_INFINITY, Less),
                (false, Ceiling | Down) => (-Float::max_finite_value_with_prec(prec), Greater),
                (_, Exact) => panic!("Inexact conversion from Rational to Float"),
            };
        }
        let (significand, o) =
            Integer::rounding_from(x << (i128::exact_from(prec) - i128::from(exponent) - 1), rm);
        let sign = significand >= 0;
        let mut significand = significand.unsigned_abs();
        let away_from_0 = if sign { Greater } else { Less };
        if o == away_from_0 && significand.is_power_of_2() {
            exponent += 1;
            if exponent >= Float::MAX_EXPONENT {
                return if sign {
                    (Float::INFINITY, Greater)
                } else {
                    (Float::NEGATIVE_INFINITY, Less)
                };
            }
        }
        exponent += 1;
        if exponent < Float::MIN_EXPONENT {
            assert!(rm != Exact, "Inexact conversion from Rational to Float");
            return if rm == Nearest
                && exponent == Float::MIN_EXPONENT - 1
                && (o == away_from_0.reverse() || !significand.is_power_of_2())
            {
                if sign {
                    (Float::min_positive_value_prec(prec), Greater)
                } else {
                    (-Float::min_positive_value_prec(prec), Less)
                }
            } else {
                match (sign, rm) {
                    (true, Up | Ceiling) => (Float::min_positive_value_prec(prec), Greater),
                    (true, Floor | Down | Nearest) => (Float::ZERO, Less),
                    (false, Up | Floor) => (-Float::min_positive_value_prec(prec), Less),
                    (false, Ceiling | Down | Nearest) => (Float::NEGATIVE_ZERO, Greater),
                    (_, Exact) => unreachable!(),
                }
            };
        }
        significand <<= significand
            .significant_bits()
            .neg_mod_power_of_2(Limb::LOG_WIDTH);
        let target_bits = prec
            .round_to_multiple_of_power_of_2(Limb::LOG_WIDTH, Ceiling)
            .0;
        let current_bits = significand_bits(&significand);
        if current_bits > target_bits {
            significand >>= current_bits - target_bits;
        }
        (
            Float(Finite {
                sign,
                exponent,
                precision: prec,
                significand,
            }),
            o,
        )
    }
}

fn from_reciprocal_rational_prec_round_ref_using_div(
    x: &Rational,
    prec: u64,
    mut rm: RoundingMode,
) -> (Float, Ordering) {
    let sign = *x >= 0;
    if !sign {
        rm.neg_assign();
    }
    let (d, n) = x.numerator_and_denominator_ref();
    let is_zero = *n == 0;
    let (f, o) = match (
        if is_zero {
            None
        } else {
            n.checked_log_base_2()
        },
        d.checked_log_base_2(),
    ) {
        (Some(log_n), Some(log_d)) => Float::power_of_2_prec_round(
            i64::saturating_from(i128::from(log_n) - i128::from(log_d)),
            prec,
            rm,
        ),
        (None, Some(log_d)) => {
            let (mut f, mut o) = from_natural_prec_round_zero_exponent_ref(n, prec, rm);
            o = f.shr_prec_round_assign_helper(
                i128::from(log_d) - i128::from(n.significant_bits()),
                prec,
                rm,
                o,
            );
            (f, o)
        }
        (Some(log_n), None) => {
            let (mut f, mut o) = from_natural_zero_exponent_ref(d).reciprocal_prec_round(prec, rm);
            o = f.shl_prec_round_assign_helper(
                i128::from(log_n) - i128::from(d.significant_bits()),
                prec,
                rm,
                o,
            );
            (f, o)
        }
        (None, None) => {
            let (mut f, mut o) = from_natural_zero_exponent_ref(n).div_prec_round(
                from_natural_zero_exponent_ref(d),
                prec,
                rm,
            );
            o = f.shl_prec_round_assign_helper(
                i128::from(n.significant_bits()) - i128::from(d.significant_bits()),
                prec,
                rm,
                o,
            );
            (f, o)
        }
    };
    if sign { (f, o) } else { (-f, o.reverse()) }
}

pub_crate_test! {
#[inline]
from_reciprocal_rational_prec_round_ref(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    if max(x.significant_bits(), prec) < FROM_RATIONAL_THRESHOLD {
        from_reciprocal_rational_prec_round_ref_direct(x, prec, rm)
    } else {
        from_reciprocal_rational_prec_round_ref_using_div(x, prec, rm)
    }
}}

pub_crate_test! {
generic_reciprocal_sqrt_rational_ref(
    x: &Rational,
    prec: u64,
    rm: RoundingMode
) -> (Float, Ordering) {
    let mut working_prec = prec + 10;
    let mut increment = Limb::WIDTH;
    let mut end_shift = x.floor_log_base_2();
    let x2;
    let reduced_x: &Rational;
    if end_shift.gt_abs(&0x3fff_0000) {
        end_shift &= !1;
        x2 = x >> end_shift;
        reduced_x = &x2;
    } else {
        end_shift = 0;
        reduced_x = x;
    }
    loop {
        let sqrt = from_reciprocal_rational_prec_round_ref(reduced_x, working_prec, Floor).0.sqrt();
        // See algorithms.tex. Since we rounded down when computing fx, the absolute error of the
        // square root is bounded by (c_sqrt + k_fx)ulp(sqrt) <= 2ulp(sqrt).
        //
        // Experiments suggest that `working_prec` is low enough (that is, that the error is at most
        // 1 ulp), but I can only prove `working_prec - 1`.
        if float_can_round(sqrt.significand_ref().unwrap(), working_prec - 1, prec, rm) {
            let (mut sqrt, mut o) = Float::from_float_prec_round(sqrt, prec, rm);
            if end_shift != 0 {
                o = sqrt.shr_prec_round_assign_helper(end_shift >> 1, prec, rm, o);
            }
            return (sqrt, o);
        }
        working_prec += increment;
        increment = working_prec >> 1;
    }
}}

impl Float {
    /// Computes the reciprocal of the square root of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by value.
    /// An [`Ordering`] is also returned, indicating whether the rounded reciprocal square root is
    /// less than, equal to, or greater than the exact square root. Although `NaN`s are not
    /// comparable to any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// Using this function is more accurate than taking the square root and then the reciprocal, or
    /// vice versa.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p+1}$.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::reciprocal_sqrt_prec`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::reciprocal_sqrt_round`] instead. If both of these things are true, consider
    /// using [`Float::reciprocal_sqrt`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round(5, Floor);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round(5, Ceiling);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.59");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round(5, Nearest);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round(20, Floor);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round(20, Ceiling);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56419");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round(20, Nearest);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56419");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec_round(self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        self.reciprocal_sqrt_prec_round_ref(prec, rm)
    }

    /// Computes the reciprocal of the square root of a [`Float`], rounding the result to the
    /// specified precision and with the specified rounding mode. The [`Float`] is taken by
    /// reference. An [`Ordering`] is also returned, indicating whether the rounded reciprocal
    /// square root is less than, equal to, or greater than the exact square root. Although `NaN`s
    /// are not comparable to any [`Float`], whenever this function returns a `NaN` it also returns
    /// `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// Using this function is more accurate than taking the square root and then the reciprocal, or
    /// vice versa.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p+1}$.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p,m)=\text{NaN}$
    /// - $f(\infty,p,m)=0.0$
    /// - $f(-\infty,p,m)=\text{NaN}$
    /// - $f(0.0,p,m)=\infty$
    /// - $f(-0.0,p,m)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::reciprocal_sqrt_prec_ref`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::reciprocal_sqrt_round_ref`] instead. If both of these things are true,
    /// consider using `(&Float).reciprocal_sqrt()`instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round_ref(5, Floor);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round_ref(5, Ceiling);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.59");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round_ref(5, Nearest);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round_ref(20, Floor);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round_ref(20, Ceiling);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56419");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_round_ref(20, Nearest);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56419");
    /// assert_eq!(o, Greater);
    /// ```
    ///
    /// This is mpfr_rec_sqrt from rec_sqrt.c, MPFR 4.3.0.
    #[inline]
    pub fn reciprocal_sqrt_prec_round_ref(&self, prec: u64, rm: RoundingMode) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        match self {
            Self(NaN | Infinity { sign: false }) => (float_nan!(), Equal),
            float_infinity!() => (float_zero!(), Equal),
            float_either_zero!() => (float_infinity!(), Equal),
            Self(Finite {
                sign,
                exponent: x_exp,
                precision: x_prec,
                significand: x,
                ..
            }) => {
                if !sign {
                    return (float_nan!(), Equal);
                }
                // Let u = U*2^e, where e = EXP(u), and 1/2 <= U < 1. If e is even, we compute an
                // approximation of X of (4U)^{-1/2}, and the result is X*2^(-(e-2)/2) [case s=1].
                // If e is odd, we compute an approximation of X of (2U)^{-1/2}, and the result is
                // X*2^(-(e-1)/2) [case s=0].
                //
                // parity of the exponent of u
                let mut s = i32::from(x_exp.even());
                let in_len = bit_to_limb_count_ceiling(prec);
                // for the first iteration, if rp + 11 fits into rn limbs, we round up up to a full
                // limb to maximize the chance of rounding, while avoiding to allocate extra space
                let mut working_prec = max(prec + 11, limb_to_bit_count(in_len));
                let mut increment = Limb::WIDTH;
                let mut out;
                loop {
                    let working_limbs = bit_to_limb_count_ceiling(working_prec);
                    out = alloc::vec![0; working_limbs];
                    limbs_reciprocal_sqrt(
                        &mut out,
                        working_prec,
                        x.as_limbs_asc(),
                        *x_prec,
                        s == 1,
                    );
                    // If the input was not truncated, the error is at most one ulp; if the input
                    // was truncated, the error is at most two ulps (see algorithms.tex).
                    if limbs_float_can_round(
                        &out,
                        working_prec - u64::from(working_prec < *x_prec),
                        prec,
                        rm,
                    ) {
                        assert_ne!(rm, Exact, "Inexact float reciprocal square root");
                        break;
                    }
                    // We detect only now the exact case where u = 2 ^ (2e), to avoid slowing down
                    // the average case. This can happen only when the mantissa is exactly 1 / 2 and
                    // the exponent is odd.
                    if s == 0 && x.is_power_of_2() {
                        let pl = limb_to_bit_count(working_limbs) - working_prec;
                        // we should have x=111...111
                        limbs_significand_slice_add_limb_in_place(&mut out, Limb::power_of_2(pl));
                        *out.last_mut().unwrap() = LIMB_HIGH_BIT;
                        s = 2;
                        break;
                    }
                    working_prec += increment;
                    increment = working_prec >> 1;
                }
                let reciprocal_sqrt = Self(Finite {
                    sign: true,
                    exponent: (s + 1 - x_exp) >> 1,
                    precision: working_prec,
                    significand: Natural::from_owned_limbs_asc(out),
                });
                Self::from_float_prec_round(reciprocal_sqrt, prec, rm)
            }
        }
    }

    /// Computes the reciprocal of the square root of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by value. An [`Ordering`]
    /// is also returned, indicating whether the rounded reciprocal square root is less than, equal
    /// to, or greater than the exact square root. Although `NaN`s are not comparable to any
    /// [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// Using this function is more accurate than taking the square root and then the reciprocal, or
    /// vice versa.
    ///
    /// If the reciprocal square root is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   1/\sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_prec_round`] instead. If you know that your target precision is the
    /// precision of the input, consider using [`Float::reciprocal_sqrt`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec(5);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec(20);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56419");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec(self, prec: u64) -> (Self, Ordering) {
        self.reciprocal_sqrt_prec_round(prec, Nearest)
    }

    /// Computes the reciprocal of the square root of a [`Float`], rounding the result to the
    /// nearest value of the specified precision. The [`Float`] is taken by reference. An
    /// [`Ordering`] is also returned, indicating whether the rounded reciprocal square root is less
    /// than, equal to, or greater than the exact square root. Although `NaN`s are not comparable to
    /// any [`Float`], whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// Using this function is more accurate than taking the square root and then the reciprocal, or
    /// vice versa.
    ///
    /// If the reciprocal square root is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   1/\sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(\text{NaN},p)=\text{NaN}$
    /// - $f(\infty,p)=0.0$
    /// - $f(-\infty,p)=\text{NaN}$
    /// - $f(0.0,p)=\infty$
    /// - $f(-0.0,p)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_prec_round_ref`] instead. If you know that your target precision is
    /// the precision of the input, consider using `(&Float).reciprocal_sqrt()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_ref(5);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_prec_ref(20);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.56419");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec_ref(&self, prec: u64) -> (Self, Ordering) {
        self.reciprocal_sqrt_prec_round_ref(prec, Nearest)
    }

    /// Computes the reciprocal of the square root of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by value. An [`Ordering`] is also returned,
    /// indicating whether the rounded reciprocal square root is less than, equal to, or greater
    /// than the exact square root. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function returns a `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// Using this function is more accurate than taking the square root and then the reciprocal, or
    /// vice versa.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::reciprocal_sqrt_prec_round`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using [`Float::reciprocal_sqrt`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_round(Floor);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189583547756");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_round(Ceiling);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189583547757");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_round(Nearest);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189583547757");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_round(self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.reciprocal_sqrt_prec_round(prec, rm)
    }

    /// Computes the reciprocal of the square root of a [`Float`], rounding the result with the
    /// specified rounding mode. The [`Float`] is taken by reference. An [`Ordering`] is also
    /// returned, indicating whether the rounded reciprocal square root is less than, equal to, or
    /// greater than the exact square root. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// Using this function is more accurate than taking the square root and then the reciprocal, or
    /// vice versa.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// f(x,m) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p+1}$, where $p$ is the precision of the input.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p}$, where $p$ is the precision of the input.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// Special cases:
    /// - $f(\text{NaN},m)=\text{NaN}$
    /// - $f(\infty,m)=0.0$
    /// - $f(-\infty,m)=\text{NaN}$
    /// - $f(0.0,m)=\infty$
    /// - $f(-0.0,m)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::reciprocal_sqrt_prec_round_ref`] instead. If you know you'll be using the `Nearest`
    /// rounding mode, consider using `(&Float).reciprocal_sqrt()` instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_round_ref(Floor);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189583547756");
    /// assert_eq!(o, Less);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_round_ref(Ceiling);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189583547757");
    /// assert_eq!(o, Greater);
    ///
    /// let (reciprocal_sqrt, o) = Float::from(PI).reciprocal_sqrt_round_ref(Nearest);
    /// assert_eq!(reciprocal_sqrt.to_string(), "0.564189583547757");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_round_ref(&self, rm: RoundingMode) -> (Self, Ordering) {
        let prec = self.significant_bits();
        self.reciprocal_sqrt_prec_round_ref(prec, rm)
    }

    /// Computes the reciprocal of the square root of a [`Float`] in place, rounding the result to
    /// the specified precision and with the specified rounding mode. An [`Ordering`] is returned,
    /// indicating whether the rounded reciprocal square root is less than, equal to, or greater
    /// than the exact square root. Although `NaN`s are not comparable to any [`Float`], whenever
    /// this function sets the [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// Using this function is more accurate than taking the square root and then the reciprocal, or
    /// vice versa.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 |xy|\rfloor-p+1}$.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::reciprocal_sqrt_prec_round`] documentation for information on special
    /// cases, overflow, and underflow.
    ///
    /// If you know you'll be using `Nearest`, consider using [`Float::reciprocal_sqrt_prec_assign`]
    /// instead. If you know that your target precision is the precision of the input, consider
    /// using [`Float::reciprocal_sqrt_round_assign`] instead. If both of these things are true,
    /// consider using [`Float::reciprocal_sqrt_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(5, Floor), Less);
    /// assert_eq!(x.to_string(), "0.56");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(5, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.59");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(5, Nearest), Less);
    /// assert_eq!(x.to_string(), "0.56");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(20, Floor), Less);
    /// assert_eq!(x.to_string(), "0.564189");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(20, Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.56419");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_round_assign(20, Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.56419");
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec_round_assign(&mut self, prec: u64, rm: RoundingMode) -> Ordering {
        let (reciprocal_sqrt, o) = self.reciprocal_sqrt_prec_round_ref(prec, rm);
        *self = reciprocal_sqrt;
        o
    }

    /// Computes the reciprocal of the square root of a [`Float`] in place, rounding the result to
    /// the nearest value of the specified precision. An [`Ordering`] is returned, indicating
    /// whether the rounded square root is less than, equal to, or greater than the exact square
    /// root. Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// Using this function is more accurate than taking the square root and then the reciprocal, or
    /// vice versa.
    ///
    /// If the reciprocal square root is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// x \gets 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   1/\sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// See the [`Float::reciprocal_sqrt_prec`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_prec_round_assign`] instead. If you know that your target precision
    /// is the precision of the input, consider using [`Float::reciprocal_sqrt`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_assign(5), Less);
    /// assert_eq!(x.to_string(), "0.56");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_prec_assign(20), Greater);
    /// assert_eq!(x.to_string(), "0.56419");
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_prec_assign(&mut self, prec: u64) -> Ordering {
        self.reciprocal_sqrt_prec_round_assign(prec, Nearest)
    }

    /// Computes the reciprocal of the square root of a [`Float`] in place, rounding the result with
    /// the specified rounding mode. An [`Ordering`] is returned, indicating whether the rounded
    /// reciprocal square root is less than, equal to, or greater than the exact square root.
    /// Although `NaN`s are not comparable to any [`Float`], whenever this function sets the
    /// [`Float`] to `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// Using this function is more accurate than taking the square root and then the reciprocal, or
    /// vice versa.
    ///
    /// The precision of the output is the precision of the input. See [`RoundingMode`] for a
    /// description of the possible rounding modes.
    ///
    /// $$
    /// x \gets 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p+1}$, where $p$ is the maximum precision of the
    ///   inputs.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// If the output has a precision, it is the precision of the input.
    ///
    /// See the [`Float::reciprocal_sqrt_round`] documentation for information on special cases,
    /// overflow, and underflow.
    ///
    /// If you want to specify an output precision, consider using
    /// [`Float::reciprocal_sqrt_prec_round_assign`] instead. If you know you'll be using the
    /// `Nearest` rounding mode, consider using [`Float::reciprocal_sqrt_assign`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the input
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use core::f64::consts::PI;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_round_assign(Floor), Less);
    /// assert_eq!(x.to_string(), "0.564189583547756");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_round_assign(Ceiling), Greater);
    /// assert_eq!(x.to_string(), "0.564189583547757");
    ///
    /// let mut x = Float::from(PI);
    /// assert_eq!(x.reciprocal_sqrt_round_assign(Nearest), Greater);
    /// assert_eq!(x.to_string(), "0.564189583547757");
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_round_assign(&mut self, rm: RoundingMode) -> Ordering {
        let prec = self.significant_bits();
        self.reciprocal_sqrt_prec_round_assign(prec, rm)
    }

    /// Computes the reciprocal of the square root of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating
    /// whether the rounded reciprocal square root is less than, equal to, or greater than the exact
    /// reciprocal square root. Although `NaN`s are not comparable to any [`Float`], whenever this
    /// function returns a `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0.0,p,m)=\infty$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
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
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_rational_prec`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) =
    ///     Float::reciprocal_sqrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 5, Floor);
    /// assert_eq!(sqrt.to_string(), "1.25");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::reciprocal_sqrt_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(sqrt.to_string(), "1.3");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::reciprocal_sqrt_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(sqrt.to_string(), "1.3");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) =
    ///     Float::reciprocal_sqrt_rational_prec_round(Rational::from_unsigneds(3u8, 5), 20, Floor);
    /// assert_eq!(sqrt.to_string(), "1.290993");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::reciprocal_sqrt_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(sqrt.to_string(), "1.290995");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::reciprocal_sqrt_rational_prec_round(
    ///     Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(sqrt.to_string(), "1.290995");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn reciprocal_sqrt_rational_prec_round(
        mut x: Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if x == 0u32 {
            return (Self::INFINITY, Equal);
        } else if x < 0u32 {
            return (Self::NAN, Equal);
        }
        x.reciprocal_assign();
        if let Some(sqrt) = (&x).checked_sqrt() {
            return Self::from_rational_prec_round(sqrt, prec, rm);
        }
        let (n, d) = x.numerator_and_denominator_ref();
        match (n.checked_log_base_2(), d.checked_log_base_2()) {
            (_, Some(log_d)) if log_d.even() => {
                let n = x.into_numerator();
                let n_exp = n.significant_bits();
                let mut n = from_natural_zero_exponent(n);
                if n_exp.odd() {
                    n <<= 1u32;
                }
                let (mut sqrt, o) = Self::exact_from(n).sqrt_prec_round(prec, rm);
                let o = sqrt.shr_prec_round_assign_helper(
                    i128::from(log_d >> 1) - i128::from(n_exp >> 1),
                    prec,
                    rm,
                    o,
                );
                (sqrt, o)
            }
            (Some(log_n), _) if log_n.even() => {
                let d = x.into_denominator();
                let d_exp = d.significant_bits();
                let mut d = from_natural_zero_exponent(d);
                if d_exp.odd() {
                    d <<= 1u32;
                }
                let (mut reciprocal_sqrt, o) =
                    Self::exact_from(d).reciprocal_sqrt_prec_round(prec, rm);
                let o = reciprocal_sqrt.shl_prec_round_assign_helper(
                    i128::from(log_n >> 1) - i128::from(d_exp >> 1),
                    prec,
                    rm,
                    o,
                );
                (reciprocal_sqrt, o)
            }
            _ => generic_sqrt_rational(x, prec, rm),
        }
    }

    /// Computes the reciprocal of the square root of a [`Rational`], rounding the result to the
    /// specified precision and with the specified rounding mode and returning the result as a
    /// [`Float`]. The [`Rational`] is taken by reference. An [`Ordering`] is also returned,
    /// indicating whether the rounded reciprocal square root is less than, equal to, or greater
    /// than the exact reciprocal square root. Although `NaN`s are not comparable to any [`Float`],
    /// whenever this function returns a `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// See [`RoundingMode`] for a description of the possible rounding modes.
    ///
    /// $$
    /// f(x,p,m) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p+1}$.
    /// - If $\sqrt{x}$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| \leq
    ///   2^{\lfloor\log_2 1/\sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0.0,p,m)=\infty$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is
    ///   returned instead.
    /// - If $f(x,p,m)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$
    ///   is returned instead, where `p` is the precision of the input.
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
    /// If you know you'll be using `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_rational_prec_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` but the result cannot be represented exactly with the given
    /// precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) = Float::reciprocal_sqrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Floor,
    /// );
    /// assert_eq!(sqrt.to_string(), "1.25");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::reciprocal_sqrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Ceiling,
    /// );
    /// assert_eq!(sqrt.to_string(), "1.3");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::reciprocal_sqrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     5,
    ///     Nearest,
    /// );
    /// assert_eq!(sqrt.to_string(), "1.3");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::reciprocal_sqrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Floor,
    /// );
    /// assert_eq!(sqrt.to_string(), "1.290993");
    /// assert_eq!(o, Less);
    ///
    /// let (sqrt, o) = Float::reciprocal_sqrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Ceiling,
    /// );
    /// assert_eq!(sqrt.to_string(), "1.290995");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::reciprocal_sqrt_rational_prec_round_ref(
    ///     &Rational::from_unsigneds(3u8, 5),
    ///     20,
    ///     Nearest,
    /// );
    /// assert_eq!(sqrt.to_string(), "1.290995");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn reciprocal_sqrt_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        assert_ne!(prec, 0);
        if *x == 0u32 {
            return (Self::INFINITY, Equal);
        } else if *x < 0u32 {
            return (Self::NAN, Equal);
        }
        if let Some(sqrt) = x.checked_sqrt() {
            return Self::from_rational_prec_round(sqrt.reciprocal(), prec, rm);
        }
        let (d, n) = x.numerator_and_denominator_ref();
        match (n.checked_log_base_2(), d.checked_log_base_2()) {
            (_, Some(log_d)) if log_d.even() => {
                let n_exp = n.significant_bits();
                let mut n = from_natural_zero_exponent_ref(n);
                if n_exp.odd() {
                    n <<= 1u32;
                }
                let (mut sqrt, o) = Self::exact_from(n).sqrt_prec_round(prec, rm);
                let o = sqrt.shr_prec_round_assign_helper(
                    i128::from(log_d >> 1) - i128::from(n_exp >> 1),
                    prec,
                    rm,
                    o,
                );
                (sqrt, o)
            }
            (Some(log_n), _) if log_n.even() => {
                let d_exp = d.significant_bits();
                let mut d = from_natural_zero_exponent_ref(d);
                if d_exp.odd() {
                    d <<= 1u32;
                }
                let (mut reciprocal_sqrt, o) =
                    Self::exact_from(d).reciprocal_sqrt_prec_round(prec, rm);
                let o = reciprocal_sqrt.shl_prec_round_assign_helper(
                    i128::from(log_n >> 1) - i128::from(d_exp >> 1),
                    prec,
                    rm,
                    o,
                );
                (reciprocal_sqrt, o)
            }
            _ => generic_reciprocal_sqrt_rational_ref(x, prec, rm),
        }
    }

    /// Computes the reciprocal of the square root of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by value. An [`Ordering`] is also returned, indicating whether the
    /// rounded reciprocal square root is less than, equal to, or greater than the exact reciprocal
    /// square root. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// If the reciprocal square root is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   1/\sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0.0,p)=\infty$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is returned
    ///   instead.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,p)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_rational_prec_round`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) = Float::reciprocal_sqrt_rational_prec(Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(sqrt.to_string(), "1.3");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) = Float::reciprocal_sqrt_rational_prec(Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(sqrt.to_string(), "1.290995");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_rational_prec(x: Rational, prec: u64) -> (Self, Ordering) {
        Self::reciprocal_sqrt_rational_prec_round(x, prec, Nearest)
    }

    /// Computes the reciprocal of the square root of a [`Rational`], rounding the result to the
    /// nearest value of the specified precision and returning the result as a [`Float`]. The
    /// [`Rational`] is taken by reference. An [`Ordering`] is also returned, indicating whether the
    /// rounded reciprocal square root is less than, equal to, or greater than the exact reciprocal
    /// square root. Although `NaN`s are not comparable to any [`Float`], whenever this function
    /// returns a `NaN` it also returns `Equal`.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// If the reciprocal square root is equidistant from two [`Float`]s with the specified
    /// precision, the [`Float`] with fewer 1s in its binary expansion is chosen. See
    /// [`RoundingMode`] for a description of the `Nearest` rounding mode.
    ///
    /// $$
    /// f(x,p) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   1/\sqrt{x}\rfloor-p}$.
    ///
    /// If the output has a precision, it is `prec`.
    ///
    /// Special cases:
    /// - $f(0.0,p)=\infty$
    ///
    /// Overflow and underflow:
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling`, `Up`, or `Nearest`, $\infty$ is
    ///   returned instead.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Floor` or `Down`, $(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Floor`, `Up`, or `Nearest`, $-\infty$ is returned
    ///   instead.
    /// - If $f(x,p)\geq 2^{2^{30}-1}$ and $m$ is `Ceiling` or `Down`, $-(1-(1/2)^p)2^{2^{30}-1}$ is
    ///   returned instead, where `p` is the precision of the input.
    /// - If $0<f(x,p)<2^{-2^{30}}$, and $m$ is `Floor` or `Down`, $0.0$ is returned instead.
    /// - If $0<f(x,p)<2^{-2^{30}}$, and $m$ is `Ceiling` or `Up`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $0<f(x,p)\leq2^{-2^{30}-1}$, and $m$ is `Nearest`, $0.0$ is returned instead.
    /// - If $2^{-2^{30}-1}<f(x,p,m)<2^{-2^{30}}$, and $m$ is `Nearest`, $2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}}<f(x,p)<0$, and $m$ is `Ceiling` or `Down`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<0$, and $m$ is `Floor` or `Up`, $-2^{-2^{30}}$ is returned
    ///   instead.
    /// - If $-2^{-2^{30}-1}\leq f(x,p)<0$, and $m$ is `Nearest`, $-0.0$ is returned instead.
    /// - If $-2^{-2^{30}}<f(x,p)<-2^{-2^{30}-1}$, and $m$ is `Nearest`, $-2^{-2^{30}}$ is returned
    ///   instead.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_rational_prec_round_ref`] instead.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (sqrt, o) =
    ///     Float::reciprocal_sqrt_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 5);
    /// assert_eq!(sqrt.to_string(), "1.3");
    /// assert_eq!(o, Greater);
    ///
    /// let (sqrt, o) =
    ///     Float::reciprocal_sqrt_rational_prec_ref(&Rational::from_unsigneds(3u8, 5), 20);
    /// assert_eq!(sqrt.to_string(), "1.290995");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn reciprocal_sqrt_rational_prec_ref(x: &Rational, prec: u64) -> (Self, Ordering) {
        Self::reciprocal_sqrt_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl ReciprocalSqrt for Float {
    type Output = Self;

    /// Computes the reciprocal of the square root of a [`Float`], taking it by value.
    ///
    /// If the output has a precision, it is the precision of the input. If the reciprocal square
    /// root is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// Using this function is more accurate than taking the square root and then the reciprocal, or
    /// vice versa.
    ///
    /// $$
    /// f(x) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   1/\sqrt{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_prec`] instead. If you want to specify the output precision,
    /// consider using [`Float::reciprocal_sqrt_round`]. If you want both of these things, consider
    /// using [`Float::reciprocal_sqrt_prec_round`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ReciprocalSqrt;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!(Float::NAN.reciprocal_sqrt().is_nan());
    /// assert_eq!(Float::INFINITY.reciprocal_sqrt(), Float::ZERO);
    /// assert!(Float::NEGATIVE_INFINITY.reciprocal_sqrt().is_nan());
    /// assert_eq!(Float::from(1.5).reciprocal_sqrt().to_string(), "0.8");
    /// assert!(Float::from(-1.5).reciprocal_sqrt().is_nan());
    /// ```
    #[inline]
    fn reciprocal_sqrt(self) -> Self {
        let prec = self.significant_bits();
        self.reciprocal_sqrt_prec_round(prec, Nearest).0
    }
}

impl ReciprocalSqrt for &Float {
    type Output = Float;

    /// Computes the reciprocal of the square root of a [`Float`], taking it by reference.
    ///
    /// If the output has a precision, it is the precision of the input. If the reciprocal square
    /// root is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// Using this function is more accurate than taking the square root and then the reciprocal, or
    /// vice versa.
    ///
    /// $$
    /// f(x) = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   1/\sqrt{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// Special cases:
    /// - $f(\text{NaN})=\text{NaN}$
    /// - $f(\infty)=0.0$
    /// - $f(-\infty)=\text{NaN}$
    /// - $f(0.0)=\infty$
    /// - $f(-0.0)=\infty$
    ///
    /// Neither overflow nor underflow is possible.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_prec_ref`] instead. If you want to specify the output precision,
    /// consider using [`Float::reciprocal_sqrt_round_ref`]. If you want both of these things,
    /// consider using [`Float::reciprocal_sqrt_prec_round_ref`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ReciprocalSqrt;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// assert!((&Float::NAN).reciprocal_sqrt().is_nan());
    /// assert_eq!((&Float::INFINITY).reciprocal_sqrt(), Float::ZERO);
    /// assert!((&Float::NEGATIVE_INFINITY).reciprocal_sqrt().is_nan());
    /// assert_eq!((&Float::from(1.5)).reciprocal_sqrt().to_string(), "0.8");
    /// assert!((&Float::from(-1.5)).reciprocal_sqrt().is_nan());
    /// ```
    #[inline]
    fn reciprocal_sqrt(self) -> Float {
        let prec = self.significant_bits();
        self.reciprocal_sqrt_prec_round_ref(prec, Nearest).0
    }
}

impl ReciprocalSqrtAssign for Float {
    /// Computes the reciprocal of the square root of a [`Float`] in place.
    ///
    /// If the output has a precision, it is the precision of the input. If the reciprocal square
    /// root is equidistant from two [`Float`]s with the specified precision, the [`Float`] with
    /// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
    /// `Nearest` rounding mode.
    ///
    /// The reciprocal square root of any nonzero negative number is `NaN`.
    ///
    /// Using this function is more accurate than taking the square root and then the reciprocal, or
    /// vice versa.
    ///
    /// $$
    /// x\gets = 1/\sqrt{x}+\varepsilon.
    /// $$
    /// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be
    ///   0.
    /// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
    ///   1/\sqrt{x}\rfloor-p}$, where $p$ is the maximum precision of the inputs.
    ///
    /// See the [`Float::reciprocal_sqrt`] documentation for information on special cases, overflow,
    /// and underflow.
    ///
    /// If you want to use a rounding mode other than `Nearest`, consider using
    /// [`Float::reciprocal_sqrt_prec_assign`] instead. If you want to specify the output precision,
    /// consider using [`Float::reciprocal_sqrt_round_assign`]. If you want both of these things,
    /// consider using [`Float::reciprocal_sqrt_prec_round_assign`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.get_prec()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ReciprocalSqrtAssign;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, Zero};
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::NAN;
    /// x.reciprocal_sqrt_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::INFINITY;
    /// x.reciprocal_sqrt_assign();
    /// assert_eq!(x, Float::ZERO);
    ///
    /// let mut x = Float::NEGATIVE_INFINITY;
    /// x.reciprocal_sqrt_assign();
    /// assert!(x.is_nan());
    ///
    /// let mut x = Float::from(1.5);
    /// x.reciprocal_sqrt_assign();
    /// assert_eq!(x.to_string(), "0.8");
    ///
    /// let mut x = Float::from(-1.5);
    /// x.reciprocal_sqrt_assign();
    /// assert!(x.is_nan());
    /// ```
    #[inline]
    fn reciprocal_sqrt_assign(&mut self) {
        let prec = self.significant_bits();
        self.reciprocal_sqrt_prec_round_assign(prec, Nearest);
    }
}

/// Computes the reciprocal of the square root of a primitive float. Using this function is more
/// accurate than using `powf(0.5)` or taking the square root and then the reciprocal, or vice
/// versa.
///
/// If the reciprocal square root is equidistant from two primitive floats, the primitive float with
/// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
/// `Nearest` rounding mode.
///
/// The reciprocal square root of any nonzero negative number is `NaN`.
///
/// $$
/// f(x) = 1/\sqrt{x}+\varepsilon.
/// $$
/// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   1/\sqrt{x}\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a [`f32`]
///   and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(\text{NaN})=\text{NaN}$
/// - $f(\infty)=0.0$
/// - $f(-\infty)=\text{NaN}$
/// - $f(0.0)=\infty$
/// - $f(-0.0)=\infty$
///
/// Neither overflow nor underflow is possible.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::NegativeInfinity;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::reciprocal_sqrt::primitive_float_reciprocal_sqrt;
///
/// assert!(primitive_float_reciprocal_sqrt(f32::NAN).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_sqrt(f32::INFINITY)),
///     NiceFloat(0.0)
/// );
/// assert!(primitive_float_reciprocal_sqrt(f32::NEGATIVE_INFINITY).is_nan());
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_sqrt(3.0f32)),
///     NiceFloat(0.57735026)
/// );
/// assert!(primitive_float_reciprocal_sqrt(-3.0f32).is_nan());
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_reciprocal_sqrt<T: PrimitiveFloat>(x: T) -> T
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_float_to_float_fn(Float::reciprocal_sqrt_prec, x)
}

/// Computes the reciprocal of the square root of a [`Rational`], returning a primitive float
/// result.
///
/// If the reciprocal square root is equidistant from two primitive floats, the primitive float with
/// fewer 1s in its binary expansion is chosen. See [`RoundingMode`] for a description of the
/// `Nearest` rounding mode.
///
/// The reciprocal square root of any negative number is `NaN`.
///
/// $$
/// f(x) = 1/\sqrt{x}+\varepsilon.
/// $$
/// - If $1/\sqrt{x}$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to be 0.
/// - If $1/\sqrt{x}$ is finite and nonzero, then $|\varepsilon| < 2^{\lfloor\log_2
///   1/\sqrt{x}\rfloor-p}$, where $p$ is precision of the output (typically 24 if `T` is a [`f32`]
///   and 53 if `T` is a [`f64`], but less if the output is subnormal).
///
/// Special cases:
/// - $f(0)=\infty$
///
/// Overflow:
/// - If the absolute value of the result is too large to represent, $\infty$ is returned instead.
/// - If the absolute value of the result is too small to represent, 0.0 is returned instead.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::num::basic::traits::Zero;
/// use malachite_base::num::float::NiceFloat;
/// use malachite_float::arithmetic::reciprocal_sqrt::primitive_float_reciprocal_sqrt_rational;
/// use malachite_q::Rational;
///
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_sqrt_rational::<f64>(
///         &Rational::ZERO
///     )),
///     NiceFloat(f64::INFINITY)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_sqrt_rational::<f64>(
///         &Rational::from_unsigneds(1u8, 3)
///     )),
///     NiceFloat(1.7320508075688772)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_sqrt_rational::<f64>(
///         &Rational::from(10000)
///     )),
///     NiceFloat(0.01)
/// );
/// assert_eq!(
///     NiceFloat(primitive_float_reciprocal_sqrt_rational::<f64>(
///         &Rational::from(-10000)
///     )),
///     NiceFloat(f64::NAN)
/// );
/// ```
#[inline]
#[allow(clippy::type_repetition_in_bounds)]
pub fn primitive_float_reciprocal_sqrt_rational<T: PrimitiveFloat>(x: &Rational) -> T
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    emulate_rational_to_float_fn(Float::reciprocal_sqrt_rational_prec_ref, x)
}
