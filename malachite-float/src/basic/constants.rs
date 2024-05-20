// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use malachite_base::comparison::traits::{Max, Min};
use malachite_base::named::Named;
use malachite_base::num::arithmetic::traits::{PowerOf2, RoundToMultipleOfPowerOf2};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeOne, NegativeZero, One,
    OneHalf, Two, Zero as ZeroTrait,
};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

#[doc(hidden)]
#[macro_export]
macro_rules! float_zero {
    () => {
        Float(Zero { sign: true })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! float_one {
    () => {
        Float(Finite {
            sign: true,
            exponent: 1,
            precision: 1,
            significand: Natural::HIGH_BIT,
        })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! float_two {
    () => {
        Float(Finite {
            sign: true,
            exponent: 2,
            precision: 1,
            significand: Natural::HIGH_BIT,
        })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! float_negative_one {
    () => {
        Float(Finite {
            sign: false,
            exponent: 1,
            precision: 1,
            significand: Natural::HIGH_BIT,
        })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! float_one_half {
    () => {
        Float(Finite {
            sign: true,
            exponent: 0,
            precision: 1,
            significand: Natural::HIGH_BIT,
        })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! float_negative_zero {
    () => {
        Float(Zero { sign: false })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! float_infinity {
    () => {
        Float(Infinity { sign: true })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! float_negative_infinity {
    () => {
        Float(Infinity { sign: false })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! float_nan {
    () => {
        Float(NaN)
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! float_finite {
    () => {
        Float(Finite { .. })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! float_either_infinity {
    () => {
        Float(Infinity { .. })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! float_either_zero {
    () => {
        Float(Zero { .. })
    };
}

/// The constant 0.0 (positive zero), with precision 1.
impl ZeroTrait for Float {
    const ZERO: Float = float_zero!();
}

/// The constant 1.0, with precision 1.
impl One for Float {
    const ONE: Float = float_one!();
}

/// The constant 2.0, with precision 1.
impl Two for Float {
    const TWO: Float = float_two!();
}

/// The constant -1.0, with precision 1.
impl NegativeOne for Float {
    const NEGATIVE_ONE: Float = float_negative_one!();
}

/// The constant 0.5, with precision 1.
impl OneHalf for Float {
    const ONE_HALF: Float = float_one_half!();
}

/// The constant -0.0, with precision 1.
impl NegativeZero for Float {
    const NEGATIVE_ZERO: Float = float_negative_zero!();
}

/// The constant Infinity.
impl InfinityTrait for Float {
    const INFINITY: Float = float_infinity!();
}

/// The constant -Infinity.
impl NegativeInfinity for Float {
    const NEGATIVE_INFINITY: Float = float_negative_infinity!();
}

/// The constant NaN.
impl NaNTrait for Float {
    const NAN: Float = float_nan!();
}

impl Default for Float {
    /// The default value of a [`Float`], NaN.
    fn default() -> Float {
        Float::NAN
    }
}

/// The lowest value representable by this type, negative infinity.
impl Min for Float {
    const MIN: Float = Float::NEGATIVE_INFINITY;
}

/// The highest value representable by this type, positive infinity.
impl Max for Float {
    const MAX: Float = Float::INFINITY;
}

// Implements `Named` for `Float`.
impl_named!(Float);

impl Float {
    /// Returns the number 1, with the given precision.
    ///
    /// $$
    /// f(p) = 1,
    /// $$
    ///
    /// and the output has precision $p$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `p`.
    ///
    /// # Panics
    /// Panics if `p` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::one_prec(1), 1);
    /// assert_eq!(Float::one_prec(10), 1);
    /// assert_eq!(Float::one_prec(100), 1);
    ///
    /// assert_eq!(Float::one_prec(1).get_prec(), Some(1));
    /// assert_eq!(Float::one_prec(10).get_prec(), Some(10));
    /// assert_eq!(Float::one_prec(100).get_prec(), Some(100));
    /// ```
    pub fn one_prec(prec: u64) -> Float {
        assert_ne!(prec, 0);
        Float(Finite {
            sign: true,
            exponent: 1,
            precision: prec,
            significand: Natural::power_of_2(
                prec.round_to_multiple_of_power_of_2(Limb::LOG_WIDTH, Ceiling)
                    .0
                    - 1,
            ),
        })
    }

    /// Returns the number 2, with the given precision.
    ///
    /// $$
    /// f(p) = 2,
    /// $$
    ///
    /// and the output has precision $p$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `p`.
    ///
    /// # Panics
    /// Panics if `p` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::two_prec(1), 2);
    /// assert_eq!(Float::two_prec(10), 2);
    /// assert_eq!(Float::two_prec(100), 2);
    ///
    /// assert_eq!(Float::two_prec(1).get_prec(), Some(1));
    /// assert_eq!(Float::two_prec(10).get_prec(), Some(10));
    /// assert_eq!(Float::two_prec(100).get_prec(), Some(100));
    /// ```
    pub fn two_prec(prec: u64) -> Float {
        assert_ne!(prec, 0);
        Float(Finite {
            sign: true,
            exponent: 2,
            precision: prec,
            significand: Natural::power_of_2(
                prec.round_to_multiple_of_power_of_2(Limb::LOG_WIDTH, Ceiling)
                    .0
                    - 1,
            ),
        })
    }

    /// Returns the number $-1$, with the given precision.
    ///
    /// $$
    /// f(p) = -1,
    /// $$
    ///
    /// and the output has precision $p$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `p`.
    ///
    /// # Panics
    /// Panics if `p` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::negative_one_prec(1), -1);
    /// assert_eq!(Float::negative_one_prec(10), -1);
    /// assert_eq!(Float::negative_one_prec(100), -1);
    ///
    /// assert_eq!(Float::negative_one_prec(1).get_prec(), Some(1));
    /// assert_eq!(Float::negative_one_prec(10).get_prec(), Some(10));
    /// assert_eq!(Float::negative_one_prec(100).get_prec(), Some(100));
    /// ```
    pub fn negative_one_prec(prec: u64) -> Float {
        assert_ne!(prec, 0);
        Float(Finite {
            sign: false,
            exponent: 1,
            precision: prec,
            significand: Natural::power_of_2(
                prec.round_to_multiple_of_power_of_2(Limb::LOG_WIDTH, Ceiling)
                    .0
                    - 1,
            ),
        })
    }

    /// Returns the number 0.5, with the given precision.
    ///
    /// $$
    /// f(p) = 0.5,
    /// $$
    ///
    /// and the output has precision $p$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `p`.
    ///
    /// # Panics
    /// Panics if `p` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::one_half_prec(1), 0.5);
    /// assert_eq!(Float::one_half_prec(10), 0.5);
    /// assert_eq!(Float::one_half_prec(100), 0.5);
    ///
    /// assert_eq!(Float::one_half_prec(1).get_prec(), Some(1));
    /// assert_eq!(Float::one_half_prec(10).get_prec(), Some(10));
    /// assert_eq!(Float::one_half_prec(100).get_prec(), Some(100));
    /// ```
    pub fn one_half_prec(prec: u64) -> Float {
        assert_ne!(prec, 0);
        Float(Finite {
            sign: true,
            exponent: 0,
            precision: prec,
            significand: Natural::power_of_2(
                prec.round_to_multiple_of_power_of_2(Limb::LOG_WIDTH, Ceiling)
                    .0
                    - 1,
            ),
        })
    }
}
