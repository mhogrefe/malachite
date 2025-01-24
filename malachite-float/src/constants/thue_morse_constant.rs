// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use alloc::vec;
use core::cmp::Ordering::{self, *};
use malachite_base::iterators::thue_morse_sequence;
use malachite_base::num::arithmetic::traits::{NegModPowerOf2, PowerOf2, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::OneHalf;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
const LIMB_0: Limb = 0xd32d2cd2;
#[cfg(feature = "32_bit_limbs")]
const LIMB_1: Limb = 0x2cd2d32c;

#[cfg(not(feature = "32_bit_limbs"))]
const LIMB_0: Limb = 0xd32d2cd32cd2d32c;
#[cfg(not(feature = "32_bit_limbs"))]
const LIMB_1: Limb = 0x2cd2d32cd32d2cd2;

impl Float {
    /// Returns an approximation to the Thue-Morse constant, with the given precision and rounded
    /// using the given [`RoundingMode`]. An [`Ordering`] is also returned, indicating whether the
    /// rounded value is less than or greater than the exact value of the constant. (Since the
    /// constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// The Thue-Morse constant is the real number whose bits are the Thue-Morse sequence. That is,
    /// $$
    /// \tau = \sum_{k=0}^\infty\frac{t_n}{2^{n+1}},
    /// $$
    /// where $t_n$ is the Thue-Morse sequence.
    ///
    /// An alternative expression, from <https://mathworld.wolfram.com/Thue-MorseConstant.html>, is
    /// $$
    /// \tau = \frac{1}{4}\left[2-\prod_{k=0}^\infty\left(1-\frac{1}{2^{2^k}}\right)\right].
    /// $$
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero or if `rm` is `Exact`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (tmc, o) = Float::thue_morse_constant_prec_round(100, Floor);
    /// assert_eq!(tmc.to_string(), "0.4124540336401075977833613682584");
    /// assert_eq!(o, Less);
    ///
    /// let (tmc, o) = Float::thue_morse_constant_prec_round(100, Ceiling);
    /// assert_eq!(tmc.to_string(), "0.4124540336401075977833613682588");
    /// assert_eq!(o, Greater);
    /// ```
    pub fn thue_morse_constant_prec_round(prec: u64, rm: RoundingMode) -> (Float, Ordering) {
        assert_ne!(prec, 0);
        assert_ne!(rm, Exact);
        // If the result is 1/2 then the exponent is 0 rather than -1, so we handle that case
        // separately.
        if prec == 1 && (rm == Nearest || rm == Ceiling || rm == Up) {
            return (Float::ONE_HALF, Greater);
        } else if prec == 2 && (rm == Ceiling || rm == Up) {
            // TODO implement const_from_unsigned_prec_times_power_of_2
            return (Float::one_half_prec(2), Greater);
        }
        let len = usize::exact_from(prec.shr_round(Limb::LOG_WIDTH, Ceiling).0);
        let mut limbs = vec![0; len];
        let mut tms = thue_morse_sequence();
        for (i, b) in (0..len).rev().zip(&mut tms) {
            limbs[i] = if b {
                limbs[i + 1] |= 1;
                LIMB_1
            } else {
                LIMB_0
            };
        }
        let lsb = Limb::power_of_2(prec.neg_mod_power_of_2(Limb::LOG_WIDTH));
        let mut next_tms = false;
        if lsb == 1 {
            next_tms = tms.next().unwrap();
            if next_tms {
                limbs[0] |= 1;
            }
        }
        let increment = match rm {
            Up | Ceiling => true,
            Down | Floor => false,
            Nearest => match lsb {
                1 => !next_tms,
                2 => tms.next().unwrap(),
                _ => limbs[0] & (lsb >> 1) != 0,
            },
            Exact => unreachable!(),
        };
        limbs[0] &= !(lsb - 1);
        let mut significand = Natural::from_owned_limbs_asc(limbs);
        if increment {
            significand += Natural::from(lsb);
        }
        (
            Float(Finite {
                sign: true,
                exponent: -1,
                precision: prec,
                significand,
            }),
            if increment { Greater } else { Less },
        )
    }

    /// Returns an approximation to the Thue-Morse constant, with the given precision and rounded to
    /// the nearest [`Float`] of that precision. An [`Ordering`] is also returned, indicating
    /// whether the rounded value is less than or greater than the exact value of the constant.
    /// (Since the constant is irrational, the rounded value is never equal to the exact value.)
    ///
    /// The Thue-Morse constant is the real number whose bits are the Thue-Morse sequence. That is,
    /// $$
    /// \tau = \sum_{k=0}^\infty\frac{t_n}{2^{n+1}},
    /// $$
    /// where $t_n$ is the Thue-Morse sequence.
    ///
    /// An alternative expression, from <https://mathworld.wolfram.com/Thue-MorseConstant.html>, is
    /// $$
    /// \tau = \frac{1}{4}\left[2-\prod_{k=0}^\infty\left(1-\frac{1}{2^{2^k}}\right)\right].
    /// $$
    ///
    /// The constant is irrational and transcendental.
    ///
    /// The output has precision `prec`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// let (tmc, o) = Float::thue_morse_constant_prec(1);
    /// assert_eq!(tmc.to_string(), "0.5");
    /// assert_eq!(o, Greater);
    ///
    /// let (tmc, o) = Float::thue_morse_constant_prec(10);
    /// assert_eq!(tmc.to_string(), "0.4126");
    /// assert_eq!(o, Greater);
    ///
    /// let (tmc, o) = Float::thue_morse_constant_prec(100);
    /// assert_eq!(tmc.to_string(), "0.4124540336401075977833613682584");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn thue_morse_constant_prec(prec: u64) -> (Float, Ordering) {
        Float::thue_morse_constant_prec_round(prec, Nearest)
    }
}
