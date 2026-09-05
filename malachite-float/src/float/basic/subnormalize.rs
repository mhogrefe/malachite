// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the GNU MPFR Library.
//
//      Copyright © 2001-2025 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{IsPowerOf2, NegModPowerOf2, PowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeZero, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, LowMask, SignificantBits};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

// Steps a finite nonzero Float's magnitude down by one ulp of its own precision, with
// mpfr_nexttozero's behavior at binade boundaries: from a power of 2, the step is into the lower
// binade, by that binade's smaller ulp. (Float::decrement instead subtracts the departed binade's
// larger ulp there, and for negative values the direction of Float::increment reverses, so neither
// is usable directly.)
fn magnitude_step_toward_zero(x: &mut Float) {
    let Float(Finite {
        exponent,
        precision,
        significand,
        ..
    }) = x
    else {
        panic!();
    };
    let total = precision.neg_mod_power_of_2(Limb::LOG_WIDTH) + *precision;
    if significand.is_power_of_2() {
        *significand = Natural::low_mask(*precision) << (total - *precision);
        *exponent -= 1;
    } else {
        *significand -= Natural::power_of_2(total - *precision);
    }
}

// Steps a finite nonzero Float's magnitude up by one ulp of its own precision.
fn magnitude_step_away_from_zero(x: &mut Float) {
    let Float(Finite {
        precision,
        significand,
        ..
    }) = x
    else {
        panic!();
    };
    let total = precision.neg_mod_power_of_2(Limb::LOG_WIDTH) + *precision;
    *significand += Natural::power_of_2(total - *precision);
    // The only caller steps away from a result of rounding to even, whose significand's lowest kept
    // bit is 0, so the addition cannot carry out of the significand.
    debug_assert!(significand.significant_bits() <= total);
}

// The minimum positive value of the emulated format, +/- 2^(sub_exp_min - 1), at precision `prec`.
fn min_subnormal(sign: bool, sub_exp_min: i64, prec: u64) -> Float {
    Float(Finite {
        sign,
        exponent: i32::exact_from(sub_exp_min),
        precision: prec,
        significand: Natural::power_of_2(prec.neg_mod_power_of_2(Limb::LOG_WIDTH) + prec - 1),
    })
}

impl Float {
    // This is a translation of mpfr_subnormalize from subnormal.c, MPFR 4.2.2, with two
    // differences. First, since Malachite has no global exponent range, the minimum normal exponent
    // of the emulated format is an explicit argument, as in rug's subnormalize_round; values with
    // exponents in [normal_exp_min - prec + 1, normal_exp_min) are subnormal in the emulated
    // format. Second, since a preceding Malachite computation runs in the full exponent range
    // rather than underflowing at the format's minimum, values below the smallest subnormal are
    // also handled here (mirroring rug), instead of being clamped by the preceding operation.
    /// Emulates gradual underflow, adjusting a rounded result as if it had been computed in a
    /// floating-point format with a limited exponent range and subnormal numbers, such as an IEEE
    /// 754 format.
    ///
    /// `self` should be the result of a computation correctly rounded to its own precision, with
    /// `o` indicating whether that result is less than, equal to, or greater than the exact value,
    /// and `rm` the rounding mode that was used. If the value is at least
    /// $2^{\\text{{normal\\_exp\\_min}}-1}$ in absolute value (or is `NaN`, infinite, or zero), it
    /// is returned unchanged along with `o`. Otherwise it lies in the emulated format's subnormal
    /// range, where fewer than `prec` significand bits are available, and it is rounded again to
    /// the available precision, with a correction that makes the result identical to what a single
    /// rounding of the exact value into the subnormal format would have produced. The returned
    /// [`Ordering`] compares the final result to the exact value. Values smaller than half the
    /// minimum subnormal round to zero.
    ///
    /// The precision of the result equals the precision of the input, except that zero results
    /// carry no precision.
    ///
    /// To emulate a standard format, pass the format's minimum normal exponent: for example, $-125$
    /// for IEEE 754 binary32 and $-1021$ for binary64, using the convention in which the
    /// significand lies in $[1/2, 1)$. This function is the analogue of `mpfr_subnormalize`, with
    /// the exponent range passed explicitly rather than set globally, and with values below the
    /// smallest subnormal handled here rather than by the preceding operation's underflow.
    ///
    /// The [`Float`] is modified in place.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the value is not exactly representable in the emulated format.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// // In a format with 4 significand bits and minimum normal exponent -5, the value 13 * 2^-10
    /// // is subnormal, with only 3 significand bits available; rounding ties to even.
    /// let mut x = Float::from_natural_prec(Natural::from(13u32), 4).0 >> 10u32;
    /// assert_eq!(x.to_string(), "0.0127");
    /// let o = x.subnormalize_assign(Equal, -5, Nearest);
    /// assert_eq!(x.to_string(), "0.0117");
    /// assert_eq!(x.get_prec(), Some(4));
    /// assert_eq!(o, Less);
    /// ```
    pub fn subnormalize_assign(
        &mut self,
        o: Ordering,
        normal_exp_min: i64,
        rm: RoundingMode,
    ) -> Ordering {
        let Self(Finite {
            sign,
            exponent,
            precision,
            significand,
        }) = &*self
        else {
            return o;
        };
        let sign = *sign;
        let prec = *precision;
        let exp = i64::from(*exponent);
        let sub_exp_min = normal_exp_min - i64::exact_from(prec) + 1;
        if exp >= normal_exp_min {
            return o;
        }
        if exp < sub_exp_min {
            // Below the smallest subnormal 2^(sub_exp_min - 1). The rounding tie is at
            // 2^(sub_exp_min - 2), which the value equals exactly if and only if its exponent is
            // sub_exp_min - 1 and it is a power of 2; in that case the direction of the exact
            // result is recovered from the ternary value.
            let away = match rm {
                Floor => !sign,
                Ceiling => sign,
                Down => false,
                Up => true,
                Nearest => {
                    if exp == sub_exp_min - 1 {
                        if significand.is_power_of_2() {
                            // exactly at the tie; round away only if the exact value is beyond the
                            // approximation
                            if sign { o == Less } else { o == Greater }
                        } else {
                            true
                        }
                    } else {
                        false
                    }
                }
                Exact => panic!(
                    "subnormalize with Exact: value is below the smallest subnormal of the \
                     emulated format"
                ),
            };
            return if away {
                *self = min_subnormal(sign, sub_exp_min, prec);
                if sign { Greater } else { Less }
            } else {
                *self = if sign {
                    Self::ZERO
                } else {
                    Self::NEGATIVE_ZERO
                };
                if sign { Less } else { Greater }
            };
        }
        // The value is in the subnormal range, with q available bits.
        let q = u64::exact_from(exp - sub_exp_min + 1);
        let min_prec = self.get_min_prec().unwrap();
        if min_prec <= q {
            // exactly representable in the emulated format; no second rounding occurs
            return o;
        }
        assert!(
            rm != Exact,
            "subnormalize with Exact: value is not exactly representable in the emulated format"
        );
        if q == 1 {
            // Only one bit is available. The rounding bit is the second-highest bit of the
            // significand and the sticky bit is the disjunction of the rest; this mirrors
            // mpfr_subnormalize's table for rounding to nearest, in which ties round to the even
            // multiple of the smallest subnormal, that is, upward.
            let away = match rm {
                Floor => !sign,
                Ceiling => sign,
                Down => false,
                Up => true,
                Nearest => {
                    let sig_bits = prec.neg_mod_power_of_2(Limb::LOG_WIDTH) + prec;
                    if !significand.get_bit(sig_bits - 2) {
                        false
                    } else if min_prec > 2 {
                        // the sticky bit is set
                        true
                    } else {
                        // rounding bit 1, sticky bit 0: the value is exactly the tie; round away
                        // unless the exact result is toward zero from here
                        if sign { o != Greater } else { o != Less }
                    }
                }
                Exact => unreachable!(),
            };
            return if away {
                *self = min_subnormal(sign, sub_exp_min + 1, prec);
                if sign { Greater } else { Less }
            } else {
                *self = min_subnormal(sign, sub_exp_min, prec);
                if sign { Less } else { Greater }
            };
        }
        // The general case: round again to q bits and correct for double rounding.
        let (mut rounded, mut o2) = Self::from_float_prec_round_ref(self, q, rm);
        // Since values exactly representable at q bits returned early, the second rounding is
        // always inexact here, so (unlike in mpfr_subnormalize) there is no exact case whose
        // ternary needs to be replaced by the first rounding's. The correction applies when the
        // second rounding hit an exact midpoint and applied the even rule, in the same direction
        // that the first rounding had already taken: the result has then drifted a full ulp from
        // the exact value, so step back and reverse the reported direction.
        if o != Equal && rm == Nearest && min_prec == q + 1 && o2 == o {
            if (o2 == Greater) == sign {
                // the result was rounded away from zero; step toward zero
                magnitude_step_toward_zero(&mut rounded);
            } else {
                magnitude_step_away_from_zero(&mut rounded);
            }
            o2 = o2.reverse();
        }
        *self = Self::from_float_prec(rounded, prec).0;
        o2
    }

    /// Emulates gradual underflow, adjusting a rounded result as if it had been computed in a
    /// floating-point format with a limited exponent range and subnormal numbers, such as an IEEE
    /// 754 format.
    ///
    /// `self` should be the result of a computation correctly rounded to its own precision, with
    /// `o` indicating whether that result is less than, equal to, or greater than the exact value,
    /// and `rm` the rounding mode that was used. If the value is at least
    /// $2^{\\text{{normal\\_exp\\_min}}-1}$ in absolute value (or is `NaN`, infinite, or zero), it
    /// is returned unchanged along with `o`. Otherwise it lies in the emulated format's subnormal
    /// range, where fewer than `prec` significand bits are available, and it is rounded again to
    /// the available precision, with a correction that makes the result identical to what a single
    /// rounding of the exact value into the subnormal format would have produced. The returned
    /// [`Ordering`] compares the final result to the exact value. Values smaller than half the
    /// minimum subnormal round to zero.
    ///
    /// The precision of the result equals the precision of the input, except that zero results
    /// carry no precision.
    ///
    /// To emulate a standard format, pass the format's minimum normal exponent: for example, $-125$
    /// for IEEE 754 binary32 and $-1021$ for binary64, using the convention in which the
    /// significand lies in $[1/2, 1)$. This function is the analogue of `mpfr_subnormalize`, with
    /// the exponent range passed explicitly rather than set globally, and with values below the
    /// smallest subnormal handled here rather than by the preceding operation's underflow.
    ///
    /// The [`Float`] is taken by value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the value is not exactly representable in the emulated format.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_base::num::basic::traits::One;
    /// use malachite_base::num::conversion::traits::ExactFrom;
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// // A value at least 2^(normal_exp_min - 1) in absolute value is unchanged.
    /// let x = Float::from_natural_prec(Natural::from(8u32), 4).0 >> 4u32;
    /// let (y, o) = x.subnormalize(Equal, -5, Nearest);
    /// assert_eq!(y.to_string(), "0.500");
    /// assert_eq!(o, Equal);
    ///
    /// // Emulating IEEE 754 binary64: (2^52 + 1) * 2^-1125 rounds to the second-smallest
    /// // subnormal double.
    /// let x =
    ///     Float::from_natural_prec(Natural::power_of_2(52u64) + Natural::ONE, 53).0 >> 1125u32;
    /// let (y, o) = x.subnormalize(Equal, -1021, Nearest);
    /// assert_eq!(y.to_string(), "9.8813129168249309e-324");
    /// assert_eq!(NiceFloat(f64::exact_from(&y)), NiceFloat(1.0e-323));
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn subnormalize(
        mut self,
        o: Ordering,
        normal_exp_min: i64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let o2 = self.subnormalize_assign(o, normal_exp_min, rm);
        (self, o2)
    }

    /// Emulates gradual underflow, adjusting a rounded result as if it had been computed in a
    /// floating-point format with a limited exponent range and subnormal numbers, such as an IEEE
    /// 754 format.
    ///
    /// `self` should be the result of a computation correctly rounded to its own precision, with
    /// `o` indicating whether that result is less than, equal to, or greater than the exact value,
    /// and `rm` the rounding mode that was used. If the value is at least
    /// $2^{\\text{{normal\\_exp\\_min}}-1}$ in absolute value (or is `NaN`, infinite, or zero), it
    /// is returned unchanged along with `o`. Otherwise it lies in the emulated format's subnormal
    /// range, where fewer than `prec` significand bits are available, and it is rounded again to
    /// the available precision, with a correction that makes the result identical to what a single
    /// rounding of the exact value into the subnormal format would have produced. The returned
    /// [`Ordering`] compares the final result to the exact value. Values smaller than half the
    /// minimum subnormal round to zero.
    ///
    /// The precision of the result equals the precision of the input, except that zero results
    /// carry no precision.
    ///
    /// To emulate a standard format, pass the format's minimum normal exponent: for example, $-125$
    /// for IEEE 754 binary32 and $-1021$ for binary64, using the convention in which the
    /// significand lies in $[1/2, 1)$. This function is the analogue of `mpfr_subnormalize`, with
    /// the exponent range passed explicitly rather than set globally, and with values below the
    /// smallest subnormal handled here rather than by the preceding operation's underflow.
    ///
    /// The [`Float`] is taken by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `rm` is `Exact` and the value is not exactly representable in the emulated format.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use malachite_nz::natural::Natural;
    ///
    /// // Values smaller than half the minimum subnormal round to zero.
    /// let x = Float::from_natural_prec(Natural::from(8u32), 4).0 >> 15u32;
    /// assert_eq!(x.to_string(), "0.000244");
    /// let (y, o) = x.subnormalize_ref(Equal, -5, Nearest);
    /// assert_eq!(y.to_string(), "0.0");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn subnormalize_ref(
        &self,
        o: Ordering,
        normal_exp_min: i64,
        rm: RoundingMode,
    ) -> (Self, Ordering) {
        let mut x = self.clone();
        let o2 = x.subnormalize_assign(o, normal_exp_min, rm);
        (x, o2)
    }
}
