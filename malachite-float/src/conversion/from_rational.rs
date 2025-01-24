// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::arithmetic::shl_round::shl_prec_round_assign_helper;
use crate::arithmetic::shr_round::shr_prec_round_assign_helper;
use crate::conversion::from_integer::{
    from_integer_prec_round_zero_exponent, from_integer_zero_exponent,
};
use crate::conversion::from_natural::{
    from_natural_prec_round_zero_exponent, from_natural_prec_round_zero_exponent_ref,
    from_natural_zero_exponent, from_natural_zero_exponent_ref,
};
use crate::InnerFloat::Finite;
use crate::{significand_bits, Float};
use core::cmp::max;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, IsPowerOf2, NegAssign, NegModPowerOf2, RoundToMultipleOfPowerOf2, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::{
    ConvertibleFrom, ExactFrom, RoundingFrom, SaturatingFrom,
};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::integer::Integer;
use malachite_nz::platform::Limb;
use malachite_q::conversion::primitive_float_from_rational::FloatConversionError;
use malachite_q::Rational;

pub_test! {from_rational_prec_round_direct(
    x: Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let sign = x >= 0;
    if let Some(pow) = x.denominator_ref().checked_log_base_2() {
        let n = x.into_numerator();
        let n_bits = n.significant_bits();
        let (mut y, mut o) =
            from_integer_prec_round_zero_exponent(Integer::from_sign_and_abs(sign, n), prec, rm);
        o = shr_prec_round_assign_helper(&mut y, i128::from(pow) - i128::from(n_bits), prec, rm, o);
        assert!(rm != Exact || o == Equal, "Inexact conversion from Rational to Float");
        (y, o)
    } else {
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
}}

pub_test! {from_rational_prec_round_using_div(
    x: Rational,
    prec: u64,
    mut rm: RoundingMode,
) -> (Float, Ordering) {
    let sign = x >= 0;
    if !sign {
        rm.neg_assign();
    }
    let (n, d) = x.into_numerator_and_denominator();
    let is_zero = n == 0;
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
            let bits = n.significant_bits();
            let (mut f, mut o) = from_natural_prec_round_zero_exponent(n, prec, rm);
            o = shr_prec_round_assign_helper(
                &mut f,
                i128::from(log_d) - i128::from(bits),
                prec,
                rm,
                o,
            );
            (f, o)
        }
        (Some(log_n), None) => {
            let bits = d.significant_bits();
            let (mut f, mut o) =
                from_natural_zero_exponent(d).reciprocal_prec_round(prec, rm);
            o = shl_prec_round_assign_helper(
                &mut f,
                i128::from(log_n) - i128::from(bits),
                prec,
                rm,
                o,
            );
            (f, o)
        }
        (None, None) => {
            let n_bits = n.significant_bits();
            let d_bits = d.significant_bits();
            let (mut f, mut o) = from_natural_zero_exponent(n).div_prec_round(
                from_natural_zero_exponent(d),
                prec,
                rm,
            );
            o = shl_prec_round_assign_helper(
                &mut f,
                i128::from(n_bits) - i128::from(d_bits),
                prec,
                rm,
                o,
            );
            (f, o)
        }
    };
    if sign {
        (f, o)
    } else {
        (-f, o.reverse())
    }
}}

pub_test! {from_rational_prec_round_ref_direct(
    x: &Rational,
    prec: u64,
    rm: RoundingMode,
) -> (Float, Ordering) {
    assert_ne!(prec, 0);
    let sign = *x >= 0;
    if let Some(pow) = x.denominator_ref().checked_log_base_2() {
        let n = x.numerator_ref();
        let n_bits = n.significant_bits();
        let (mut y, mut o) =
            from_natural_prec_round_zero_exponent_ref(n, prec, if sign { rm } else { -rm });
        o = shr_prec_round_assign_helper(
            &mut y,
            i128::from(pow) - i128::from(n_bits),
            prec,
            if sign { rm } else { -rm },
            o,
        );
        assert!(rm != Exact || o == Equal, "Inexact conversion from Rational to Float");
        if sign {
            (y, o)
        } else {
            (-y, o.reverse())
        }
    } else {
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
}}

pub_test! {from_rational_prec_round_ref_using_div(
    x: &Rational,
    prec: u64,
    mut rm: RoundingMode,
) -> (Float, Ordering) {
    let sign = *x >= 0;
    if !sign {
        rm.neg_assign();
    }
    let (n, d) = x.numerator_and_denominator_ref();
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
            o = shr_prec_round_assign_helper(
                &mut f,
                i128::from(log_d) - i128::from(n.significant_bits()),
                prec,
                rm,
                o,
            );
            (f, o)
        }
        (Some(log_n), None) => {
            let (mut f, mut o) =
                from_natural_zero_exponent_ref(d).reciprocal_prec_round(prec, rm);
            o = shl_prec_round_assign_helper(
                &mut f,
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
            o = shl_prec_round_assign_helper(
                &mut f,
                i128::from(n.significant_bits()) - i128::from(d.significant_bits()),
                prec,
                rm,
                o,
            );
            (f, o)
        }
    };
    if sign {
        (f, o)
    } else {
        (-f, o.reverse())
    }
}}

const FROM_RATIONAL_THRESHOLD: u64 = 100;

impl Float {
    /// Converts a [`Rational`] to a [`Float`], taking the [`Rational`] by value. If the [`Float`]
    /// is nonzero, it has the specified precision. If rounding is needed, the specified rounding
    /// mode is used. An [`Ordering`] is also returned, indicating whether the returned value is
    /// less than, equal to, or greater than the original value.
    ///
    /// If you're only using [`Nearest`], try using [`Float::from_rational_prec`] instead.
    ///
    /// - If the [`Rational`] rounds to a value greater than or equal to $2^{2^{30}-1}$), this
    ///   function overflows to $\infty$ if `rm` is `Ceiling`, `Up`, or `Nearest`, and rounds down
    ///   to $(1-(1/2)^p)2^{2^{30}-1}$ otherwise, where $p$ is `prec`.
    /// - If the [`Rational`] rounds to a value less than or equal to $-2^{2^{30}-1}$), this
    ///   function overflows to $-\infty$ if `rm` is `Floor`, `Up`, or `Nearest`, and rounds up to
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ otherwise, where $p$ is `prec`.
    /// - If the [`Rational`] rounds to a positive value less than $2^{-2^{30}}$), this function
    ///   underflows to positive zero if `rm` is `Floor` or `Down`, rounds up to $2^{-2^{30}}$ if
    ///   `rm` is `Ceiling` or `Up`, underflows to positive zero if `rm` is `Nearest` and the
    ///   [`Rational`] rounds to a value less than or equal to $2^{-2^{30}-1}$, and rounds up to
    ///   $2^{-2^{30}}$ if `rm` is `Nearest` and the [`Rational`] rounds to a value greater than
    ///   $2^{-2^{30}-1}$.
    /// - If the [`Rational`] rounds to a negative value greater than $-2^{-2^{30}}$), this function
    ///   underflows to negative zero if `rm` is `Ceiling` or `Down`, rounds down to $-2^{-2^{30}}$
    ///   if `rm` is `Floor` or `Up`, underflows to negative zero if `rm` is `Nearest` and the
    ///   [`Rational`] rounds to a value greater than or equal to $-2^{-2^{30}-1}$, and rounds down
    ///   to $-2^{-2^{30}}$ if `rm` is `Nearest` and the [`Rational`] rounds to a value less than
    ///   $-2^{-2^{30}-1}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(n.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is exact and the `Rational` cannot be exactly
    /// represented with the specified precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_rational_prec_round(Rational::from_signeds(1, 3), 10, Floor);
    /// assert_eq!(x.to_string(), "0.333");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_rational_prec_round(Rational::from_signeds(1, 3), 10, Ceiling);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec_round(Rational::from_signeds(1, 3), 10, Nearest);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec_round(Rational::from_signeds(-1, 3), 10, Floor);
    /// assert_eq!(x.to_string(), "-0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_rational_prec_round(Rational::from_signeds(-1, 3), 10, Ceiling);
    /// assert_eq!(x.to_string(), "-0.333");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec_round(Rational::from_signeds(-1, 3), 10, Nearest);
    /// assert_eq!(x.to_string(), "-0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_rational_prec_round(x: Rational, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        if max(x.significant_bits(), prec) < FROM_RATIONAL_THRESHOLD {
            from_rational_prec_round_direct(x, prec, rm)
        } else {
            from_rational_prec_round_using_div(x, prec, rm)
        }
    }

    /// Converts a [`Rational`] to a [`Float`], taking the [`Rational`] by value. If the [`Float`]
    /// is nonzero, it has the specified precision. An [`Ordering`] is also returned, indicating
    /// whether the returned value is less than, equal to, or greater than the original value.
    ///
    /// If the [`Rational`] is dyadic (its denominator is a power of 2), then you can convert it to
    /// a [`Float`] using `try_from` instead. The precision of the resulting [`Float`] will be the
    /// number of significant bits of the [`Rational`]'s numerator.
    ///
    /// Rounding may occur, in which case [`Nearest`] is used by default. To specify a rounding mode
    /// as well as a precision, try [`Float::from_rational_prec_round`].
    ///
    /// - If the [`Rational`] rounds to a value greater than or equal to $2^{2^{30}-1}$), this
    ///   function overflows to $\infty$.
    /// - If the [`Rational`] rounds to a value less than or equal to $-2^{2^{30}-1}$), this
    ///   function overflows to $-\infty$.
    /// - If the [`Rational`] rounds to a positive value less than $2^{-2^{30}}$), this function
    ///   underflows to positive zero if the [`Rational`] rounds to a value less than or equal to
    ///   $2^{-2^{30}-1}$ and rounds up to $2^{-2^{30}}$ if the [`Rational`] rounds to a value
    ///   greater than $2^{-2^{30}-1}$.
    /// - If the [`Rational`] rounds to a negative value greater than $2^{-2^{30}}$), this function
    ///   underflows to negative zero if the [`Rational`] rounds to a value greater than or equal to
    ///   $-2^{-2^{30}-1}$ and rounds down to $-2^{-2^{30}}$ if the [`Rational`] rounds to a value
    ///   less than $-2^{-2^{30}-1}$.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(), prec)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_rational_prec(Rational::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_rational_prec(Rational::from_signeds(1, 3), 10);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec(Rational::from_signeds(1, 3), 100);
    /// assert_eq!(x.to_string(), "0.3333333333333333333333333333335");
    /// assert_eq!(x.get_prec(), Some(100));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec(Rational::from_signeds(-1, 3), 10);
    /// assert_eq!(x.to_string(), "-0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_rational_prec(Rational::from_signeds(-1, 3), 100);
    /// assert_eq!(x.to_string(), "-0.3333333333333333333333333333335");
    /// assert_eq!(x.get_prec(), Some(100));
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_rational_prec(x: Rational, prec: u64) -> (Float, Ordering) {
        Float::from_rational_prec_round(x, prec, Nearest)
    }

    /// Converts a [`Rational`] to a [`Float`], taking the [`Rational`] by reference. If the
    /// [`Float`] is nonzero, it has the specified precision. If rounding is needed, the specified
    /// rounding mode is used. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// If you're only using [`Nearest`], try using [`Float::from_rational_prec_ref`] instead.
    ///
    /// - If the [`Rational`] rounds to a value greater than or equal to $2^{2^{30}-1}$), this
    ///   function overflows to $\infty$ if `rm` is `Ceiling`, `Up`, or `Nearest`, and rounds down
    ///   to $(1-(1/2)^p)2^{2^{30}-1}$ otherwise, where $p$ is `prec`.
    /// - If the [`Rational`] rounds to a value less than or equal to $-2^{2^{30}-1}$), this
    ///   function overflows to $-\infty$ if `rm` is `Floor`, `Up`, or `Nearest`, and rounds up to
    ///   $-(1-(1/2)^p)2^{2^{30}-1}$ otherwise, where $p$ is `prec`.
    /// - If the [`Rational`] rounds to a positive value less than $2^{-2^{30}}$), this function
    ///   underflows to positive zero if `rm` is `Floor` or `Down`, rounds up to $2^{-2^{30}}$ if
    ///   `rm` is `Ceiling` or `Up`, underflows to positive zero if `rm` is `Nearest` and the
    ///   [`Rational`] rounds to a value less than or equal to $2^{-2^{30}-1}$, and rounds up to
    ///   $2^{-2^{30}}$ if `rm` is `Nearest` and the [`Rational`] rounds to a value greater than
    ///   $2^{-2^{30}-1}$.
    /// - If the [`Rational`] rounds to a negative value greater than $-2^{-2^{30}}$), this function
    ///   underflows to negative zero if `rm` is `Ceiling` or `Down`, rounds down to $-2^{-2^{30}}$
    ///   if `rm` is `Floor` or `Up`, underflows to negative zero if `rm` is `Nearest` and the
    ///   [`Rational`] rounds to a value greater than or equal to $-2^{-2^{30}-1}$, and rounds down
    ///   to $-2^{-2^{30}}$ if `rm` is `Nearest` and the [`Rational`] rounds to a value less than
    ///   $-2^{-2^{30}-1}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is exact and the `Rational` cannot be exactly
    /// represented with the specified precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_rational_prec_round_ref(&Rational::from_signeds(1, 3), 10, Floor);
    /// assert_eq!(x.to_string(), "0.333");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) =
    ///     Float::from_rational_prec_round_ref(&Rational::from_signeds(1, 3), 10, Ceiling);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) =
    ///     Float::from_rational_prec_round_ref(&Rational::from_signeds(1, 3), 10, Nearest);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec_round_ref(&Rational::from_signeds(-1, 3), 10, Floor);
    /// assert_eq!(x.to_string(), "-0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) =
    ///     Float::from_rational_prec_round_ref(&Rational::from_signeds(-1, 3), 10, Ceiling);
    /// assert_eq!(x.to_string(), "-0.333");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) =
    ///     Float::from_rational_prec_round_ref(&Rational::from_signeds(-1, 3), 10, Nearest);
    /// assert_eq!(x.to_string(), "-0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_rational_prec_round_ref(
        x: &Rational,
        prec: u64,
        rm: RoundingMode,
    ) -> (Float, Ordering) {
        if max(x.significant_bits(), prec) < FROM_RATIONAL_THRESHOLD {
            from_rational_prec_round_ref_direct(x, prec, rm)
        } else {
            from_rational_prec_round_ref_using_div(x, prec, rm)
        }
    }

    /// Converts a [`Rational`] to a [`Float`], taking the [`Rational`] by reference. If the
    /// [`Float`] is nonzero, it has the specified precision. An [`Ordering`] is also returned,
    /// indicating whether the returned value is less than, equal to, or greater than the original
    /// value.
    ///
    /// If the [`Rational`] is dyadic (its denominator is a power of 2), then you can convert it to
    /// a [`Float`] using `try_from` instead. The precision of the resulting [`Float`] will be the
    /// number of significant bits of the [`Rational`]'s numerator.
    ///
    /// Rounding may occur, in which case [`Nearest`] is used by default. To specify a rounding mode
    /// as well as a precision, try [`Float::from_rational_prec_round_ref`].
    ///
    /// - If the [`Rational`] rounds to a value greater than or equal to $2^{2^{30}-1}$), this
    ///   function overflows to $\infty$.
    /// - If the [`Rational`] rounds to a value less than or equal to $-2^{2^{30}-1}$), this
    ///   function overflows to $-\infty$.
    /// - If the [`Rational`] rounds to a positive value less than $2^{-2^{30}}$), this function
    ///   underflows to positive zero if the [`Rational`] rounds to a value less than or equal to
    ///   $2^{-2^{30}-1}$ and rounds up to $2^{-2^{30}}$ if the [`Rational`] rounds to a value
    ///   greater than $2^{-2^{30}-1}$.
    /// - If the [`Rational`] rounds to a negative value greater than $2^{-2^{30}}$), this function
    ///   underflows to negative zero if the [`Rational`] rounds to a value greater than or equal to
    ///   $-2^{-2^{30}-1}$ and rounds down to $-2^{-2^{30}}$ if the [`Rational`] rounds to a value
    ///   less than $-2^{-2^{30}-1}$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(x.significant_bits(), prec)`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = Float::from_rational_prec_ref(&Rational::ZERO, 10);
    /// assert_eq!(x.to_string(), "0.0");
    /// assert_eq!(o, Equal);
    ///
    /// let (x, o) = Float::from_rational_prec_ref(&Rational::from_signeds(1, 3), 10);
    /// assert_eq!(x.to_string(), "0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec_ref(&Rational::from_signeds(1, 3), 100);
    /// assert_eq!(x.to_string(), "0.3333333333333333333333333333335");
    /// assert_eq!(x.get_prec(), Some(100));
    /// assert_eq!(o, Greater);
    ///
    /// let (x, o) = Float::from_rational_prec_ref(&Rational::from_signeds(-1, 3), 10);
    /// assert_eq!(x.to_string(), "-0.3335");
    /// assert_eq!(x.get_prec(), Some(10));
    /// assert_eq!(o, Less);
    ///
    /// let (x, o) = Float::from_rational_prec_ref(&Rational::from_signeds(-1, 3), 100);
    /// assert_eq!(x.to_string(), "-0.3333333333333333333333333333335");
    /// assert_eq!(x.get_prec(), Some(100));
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_rational_prec_ref(x: &Rational, prec: u64) -> (Float, Ordering) {
        Float::from_rational_prec_round_ref(x, prec, Nearest)
    }
}

impl TryFrom<Rational> for Float {
    type Error = FloatConversionError;

    /// Converts a [`Rational`] to an [`Float`], taking the [`Rational`] by value. If the
    /// [`Rational`]'s denominator is not a power of 2, or if the [`Rational`] is too far from zero
    /// or too close to zero to be represented as a [`Float`], an error is returned.
    ///
    /// The [`Float`]'s precision is the minimum number of bits needed to exactly represent the
    /// [`Rational`].
    ///
    /// - If the [`Rational`] is greater than or equal to $2^{2^{30}-1}$), this function returns an
    ///   overflow error.
    /// - If the [`Rational`] is less than or equal to $-2^{2^{30}-1}$), this function returns an
    ///   overflow error.
    /// - If the [`Rational`] is positive and less than $2^{-2^{30}}$), this function returns an
    ///   underflow error.
    /// - If the [`Rational`] is negative and greater than $-2^{-2^{30}}$), this function returns an
    ///   underflow error.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_q::conversion::primitive_float_from_rational::FloatConversionError;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Float::try_from(Rational::ZERO).unwrap(), 0);
    /// assert_eq!(
    ///     Float::try_from(Rational::from_signeds(1, 8)).unwrap(),
    ///     0.125
    /// );
    /// assert_eq!(
    ///     Float::try_from(Rational::from_signeds(-1, 8)).unwrap(),
    ///     -0.125
    /// );
    ///
    /// assert_eq!(
    ///     Float::try_from(Rational::from_signeds(1, 3)),
    ///     Err(FloatConversionError::Inexact)
    /// );
    /// assert_eq!(
    ///     Float::try_from(Rational::from_signeds(-1, 3)),
    ///     Err(FloatConversionError::Inexact)
    /// );
    /// ```
    fn try_from(x: Rational) -> Result<Float, Self::Error> {
        if x == 0u32 {
            return Ok(Float::ZERO);
        }
        if let Some(log_denominator) = x.denominator_ref().checked_log_base_2() {
            let exponent = i32::saturating_from(x.floor_log_base_2_abs()).saturating_add(1);
            if exponent > Float::MAX_EXPONENT {
                return Err(FloatConversionError::Overflow);
            } else if exponent < Float::MIN_EXPONENT {
                return Err(FloatConversionError::Underflow);
            }
            let n = Integer::from_sign_and_abs(x >= 0u32, x.into_numerator());
            let n_bits = n.significant_bits();
            Ok(from_integer_zero_exponent(n) >> (i128::from(log_denominator) - i128::from(n_bits)))
        } else {
            Err(FloatConversionError::Inexact)
        }
    }
}

impl TryFrom<&Rational> for Float {
    type Error = FloatConversionError;

    /// Converts a [`Rational`] to an [`Float`], taking the [`Rational`] by reference. If the
    /// [`Rational`]'s denominator is not a power of 2, or if the [`Rational`] is too far from zero
    /// or too close to zero to be represented as a [`Float`], an error is returned.
    ///
    /// The [`Float`]'s precision is the minimum number of bits needed to exactly represent the
    /// [`Rational`].
    ///
    /// - If the [`Rational`] is greater than or equal to $2^{2^{30}-1}$), this function returns an
    ///   overflow error.
    /// - If the [`Rational`] is less than or equal to $-2^{2^{30}-1}$), this function returns an
    ///   overflow error.
    /// - If the [`Rational`] is positive and less than $2^{-2^{30}}$), this function returns an
    ///   underflow error.
    /// - If the [`Rational`] is negative and greater than $-2^{-2^{30}}$), this function returns an
    ///   underflow error.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_float::Float;
    /// use malachite_q::conversion::primitive_float_from_rational::FloatConversionError;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Float::try_from(&Rational::ZERO).unwrap(), 0);
    /// assert_eq!(
    ///     Float::try_from(&Rational::from_signeds(1, 8)).unwrap(),
    ///     0.125
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Rational::from_signeds(-1, 8)).unwrap(),
    ///     -0.125
    /// );
    ///
    /// assert_eq!(
    ///     Float::try_from(&Rational::from_signeds(1, 3)),
    ///     Err(FloatConversionError::Inexact)
    /// );
    /// assert_eq!(
    ///     Float::try_from(&Rational::from_signeds(-1, 3)),
    ///     Err(FloatConversionError::Inexact)
    /// );
    /// ```
    fn try_from(x: &Rational) -> Result<Float, Self::Error> {
        if *x == 0u32 {
            return Ok(Float::ZERO);
        }
        if let Some(log_denominator) = x.denominator_ref().checked_log_base_2() {
            let exponent = i32::saturating_from(x.floor_log_base_2_abs()).saturating_add(1);
            if exponent > Float::MAX_EXPONENT {
                return Err(FloatConversionError::Overflow);
            } else if exponent < Float::MIN_EXPONENT {
                return Err(FloatConversionError::Underflow);
            }
            let n = x.numerator_ref();
            let n_bits = n.significant_bits();
            let mut n = from_natural_zero_exponent_ref(n);
            if *x < 0 {
                n.neg_assign();
            }
            Ok(n >> (i128::from(log_denominator) - i128::from(n_bits)))
        } else {
            Err(FloatConversionError::Inexact)
        }
    }
}

impl ConvertibleFrom<&Rational> for Float {
    /// Determines whether a [`Rational`] can be converted to an [`Float`], taking the [`Rational`]
    /// by reference.
    ///
    /// The [`Rational`]s that are convertible to [`Float`]s are precisely those whose denominators
    /// are powers of two, and would not overflow or underflow.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `x.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::conversion::traits::ConvertibleFrom;
    /// use malachite_float::Float;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Float::convertible_from(&Rational::ZERO), true);
    /// assert_eq!(Float::convertible_from(&Rational::from_signeds(3, 8)), true);
    /// assert_eq!(
    ///     Float::convertible_from(&Rational::from_signeds(-3, 8)),
    ///     true
    /// );
    ///
    /// assert_eq!(
    ///     Float::convertible_from(&Rational::from_signeds(1, 3)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Float::convertible_from(&Rational::from_signeds(-1, 3)),
    ///     false
    /// );
    /// ```
    #[inline]
    fn convertible_from(x: &Rational) -> bool {
        *x == 0
            || x.denominator_ref().is_power_of_2()
                && (Float::MIN_EXPONENT..=Float::MAX_EXPONENT)
                    .contains(&i32::saturating_from(x.floor_log_base_2_abs()).saturating_add(1))
    }
}
