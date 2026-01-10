// Copyright © 2025 Mikhail Hogrefe
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
use malachite_base::num::arithmetic::traits::{
    IsPowerOf2, NegModPowerOf2, PowerOf2, RoundToMultipleOfPowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity as InfinityTrait, NaN as NaNTrait, NegativeInfinity, NegativeOne, NegativeZero, One,
    OneHalf, Two, Zero as ZeroTrait,
};
use malachite_base::num::logic::traits::{BitScan, LowMask};
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
    const ZERO: Self = float_zero!();
}

/// The constant 1.0, with precision 1.
impl One for Float {
    const ONE: Self = float_one!();
}

/// The constant 2.0, with precision 1.
impl Two for Float {
    const TWO: Self = float_two!();
}

/// The constant -1.0, with precision 1.
impl NegativeOne for Float {
    const NEGATIVE_ONE: Self = float_negative_one!();
}

/// The constant 0.5, with precision 1.
impl OneHalf for Float {
    const ONE_HALF: Self = float_one_half!();
}

/// The constant -0.0, with precision 1.
impl NegativeZero for Float {
    const NEGATIVE_ZERO: Self = float_negative_zero!();
}

/// The constant $\infty$.
impl InfinityTrait for Float {
    const INFINITY: Self = float_infinity!();
}

/// The constant $-\infty$.
impl NegativeInfinity for Float {
    const NEGATIVE_INFINITY: Self = float_negative_infinity!();
}

/// The constant NaN.
impl NaNTrait for Float {
    const NAN: Self = float_nan!();
}

impl Default for Float {
    /// The default value of a [`Float`], NaN.
    fn default() -> Self {
        Self::NAN
    }
}

/// The lowest value representable by this type, $-\infty$.
impl Min for Float {
    const MIN: Self = Self::NEGATIVE_INFINITY;
}

/// The highest value representable by this type, $\infty$.
impl Max for Float {
    const MAX: Self = Self::INFINITY;
}

// Implements `Named` for `Float`.
impl_named!(Float);

impl Float {
    /// The minimum representable positive value, or $2^{-2^{30}}$, with precision 1.
    pub const MIN_POSITIVE: Self = Self(Finite {
        sign: true,
        exponent: Self::MIN_EXPONENT,
        precision: 1,
        significand: Natural::HIGH_BIT,
    });

    /// Returns the minimum representable positive value, or $2^{-2^{30}}$, with the given
    /// precision.
    ///
    /// $$
    /// f(p) = 2^{-2^{30}},
    /// $$
    ///
    /// and the output has precision `prec`.
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
    ///
    /// assert_eq!(Float::min_positive_value_prec(1).to_string(), "too_small");
    /// assert_eq!(Float::min_positive_value_prec(10).to_string(), "too_small");
    /// assert_eq!(Float::min_positive_value_prec(100).to_string(), "too_small");
    ///
    /// assert_eq!(Float::min_positive_value_prec(1).get_prec(), Some(1));
    /// assert_eq!(Float::min_positive_value_prec(10).get_prec(), Some(10));
    /// assert_eq!(Float::min_positive_value_prec(100).get_prec(), Some(100));
    /// ```
    pub fn min_positive_value_prec(prec: u64) -> Self {
        assert_ne!(prec, 0);
        Self(Finite {
            sign: true,
            exponent: Self::MIN_EXPONENT,
            precision: prec,
            significand: Natural::power_of_2(
                prec.round_to_multiple_of_power_of_2(Limb::LOG_WIDTH, Ceiling)
                    .0
                    - 1,
            ),
        })
    }

    /// Returns whether the absolute value of a `Float` is equal to the minimum representable
    /// positive value, or $2^{-2^{30}}$.
    ///
    /// $$
    /// f(x) = (|x|=2^{-2^{30}}).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert!(Float::min_positive_value_prec(100).abs_is_min_positive_value());
    /// assert!((-Float::min_positive_value_prec(100)).abs_is_min_positive_value());
    /// assert!(!(Float::min_positive_value_prec(100) << 1u32).abs_is_min_positive_value());
    /// ```
    pub fn abs_is_min_positive_value(&self) -> bool {
        self.get_exponent() == Some(Self::MIN_EXPONENT)
            && self.significand_ref().unwrap().is_power_of_2()
    }

    /// There is no maximum finite [`Float`], but there is one for any given precision. This
    /// function returns that [`Float`].
    ///
    /// $$
    /// f(p) = (1-(1/2)^p)2^{2^{30}-1},
    /// $$
    /// where $p$ is `prec`. The output has precision `prec`.
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
    ///
    /// assert_eq!(Float::max_finite_value_with_prec(1).to_string(), "too_big");
    /// assert_eq!(Float::max_finite_value_with_prec(10).to_string(), "too_big");
    /// assert_eq!(
    ///     Float::max_finite_value_with_prec(100).to_string(),
    ///     "too_big"
    /// );
    ///
    /// assert_eq!(Float::max_finite_value_with_prec(1).get_prec(), Some(1));
    /// assert_eq!(Float::max_finite_value_with_prec(10).get_prec(), Some(10));
    /// assert_eq!(Float::max_finite_value_with_prec(100).get_prec(), Some(100));
    /// ```
    pub fn max_finite_value_with_prec(prec: u64) -> Self {
        assert_ne!(prec, 0);
        Self(Finite {
            sign: true,
            exponent: Self::MAX_EXPONENT,
            precision: prec,
            significand: Natural::low_mask(prec) << prec.neg_mod_power_of_2(Limb::LOG_WIDTH),
        })
    }

    /// Returns whether the absolute value of a `Float` is equal to the maximum representable finite
    /// value with that precision.
    ///
    /// $$
    /// f(x) = (|x|=(1-(1/2)^p)2^{2^{30}-1}),
    /// $$
    /// where $p$ is the precision of the $x$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::Float;
    ///
    /// assert!(Float::max_finite_value_with_prec(100).abs_is_max_finite_value_with_prec());
    /// assert!((-Float::max_finite_value_with_prec(100)).abs_is_max_finite_value_with_prec());
    /// assert!(
    ///     !(Float::max_finite_value_with_prec(100) >> 1u32).abs_is_max_finite_value_with_prec()
    /// );
    /// ```
    pub fn abs_is_max_finite_value_with_prec(&self) -> bool {
        if self.get_exponent() != Some(Self::MAX_EXPONENT) {
            return false;
        }
        let prec = self.get_prec().unwrap();
        let lowest_1_index = prec.neg_mod_power_of_2(Limb::LOG_WIDTH);
        self.significand_ref()
            .unwrap()
            .index_of_next_false_bit(lowest_1_index)
            .unwrap()
            == prec
                .round_to_multiple_of_power_of_2(Limb::LOG_WIDTH, Ceiling)
                .0
    }

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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
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
    pub fn one_prec(prec: u64) -> Self {
        assert_ne!(prec, 0);
        Self(Finite {
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
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
    pub fn two_prec(prec: u64) -> Self {
        assert_ne!(prec, 0);
        Self(Finite {
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
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
    pub fn negative_one_prec(prec: u64) -> Self {
        assert_ne!(prec, 0);
        Self(Finite {
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
    /// where $T$ is time, $M$ is additional memory, and $n$ is `prec`.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
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
    pub fn one_half_prec(prec: u64) -> Self {
        assert_ne!(prec, 0);
        Self(Finite {
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
