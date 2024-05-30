// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::InnerFloat::{Finite, Infinity, NaN, Zero};
use crate::{float_either_infinity, float_finite, float_nan, float_negative_zero, float_zero};
use crate::{float_either_zero, Float};
use core::num::FpCategory;

impl Float {
    /// Determines whether a [`Float`] is NaN.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.is_nan(), true);
    /// assert_eq!(Float::ONE.is_nan(), false);
    /// ```
    #[inline]
    pub const fn is_nan(&self) -> bool {
        matches!(self, float_nan!())
    }

    /// Determines whether a [`Float`] is finite.
    ///
    /// NaN is not finite.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.is_finite(), false);
    /// assert_eq!(Float::INFINITY.is_finite(), false);
    /// assert_eq!(Float::ONE.is_finite(), true);
    /// ```
    #[inline]
    pub const fn is_finite(&self) -> bool {
        matches!(self, Float(Zero { .. } | Finite { .. }))
    }

    /// Determines whether a [`Float`] is infinite.
    ///
    /// NaN is not infinite.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.is_infinite(), false);
    /// assert_eq!(Float::INFINITY.is_infinite(), true);
    /// assert_eq!(Float::ONE.is_infinite(), false);
    /// ```
    #[inline]
    pub const fn is_infinite(&self) -> bool {
        matches!(self, float_either_infinity!())
    }

    /// Determines whether a [`Float`] is positive zero.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeZero, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.is_positive_zero(), false);
    /// assert_eq!(Float::INFINITY.is_positive_zero(), false);
    /// assert_eq!(Float::ONE.is_positive_zero(), false);
    /// assert_eq!(Float::ZERO.is_positive_zero(), true);
    /// assert_eq!(Float::NEGATIVE_ZERO.is_positive_zero(), false);
    /// ```
    #[inline]
    pub const fn is_positive_zero(&self) -> bool {
        matches!(self, float_zero!())
    }

    /// Determines whether a [`Float`] is negative zero.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeZero, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.is_negative_zero(), false);
    /// assert_eq!(Float::INFINITY.is_negative_zero(), false);
    /// assert_eq!(Float::ONE.is_negative_zero(), false);
    /// assert_eq!(Float::ZERO.is_negative_zero(), false);
    /// assert_eq!(Float::NEGATIVE_ZERO.is_negative_zero(), true);
    /// ```
    #[inline]
    pub const fn is_negative_zero(&self) -> bool {
        matches!(self, float_negative_zero!())
    }

    /// Determines whether a [`Float`] is zero (positive or negative).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeZero, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.is_zero(), false);
    /// assert_eq!(Float::INFINITY.is_zero(), false);
    /// assert_eq!(Float::ONE.is_zero(), false);
    /// assert_eq!(Float::ZERO.is_zero(), true);
    /// assert_eq!(Float::NEGATIVE_ZERO.is_zero(), true);
    /// ```
    #[inline]
    pub const fn is_zero(&self) -> bool {
        matches!(self, float_either_zero!())
    }

    /// Determines whether a [`Float`] is normal, that is, finite and nonzero.
    ///
    /// There is no notion of subnormal [`Float`]s.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeZero, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.is_normal(), false);
    /// assert_eq!(Float::INFINITY.is_normal(), false);
    /// assert_eq!(Float::ZERO.is_normal(), false);
    /// assert_eq!(Float::NEGATIVE_ZERO.is_normal(), false);
    /// assert_eq!(Float::ONE.is_normal(), true);
    /// ```
    pub const fn is_normal(&self) -> bool {
        matches!(self, float_finite!())
    }

    /// Determines whether a [`Float`]'s sign is positive.
    ///
    /// A NaN has no sign, so this function returns false when given a NaN.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
    /// };
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.is_sign_positive(), false);
    /// assert_eq!(Float::INFINITY.is_sign_positive(), true);
    /// assert_eq!(Float::NEGATIVE_INFINITY.is_sign_positive(), false);
    /// assert_eq!(Float::ZERO.is_sign_positive(), true);
    /// assert_eq!(Float::NEGATIVE_ZERO.is_sign_positive(), false);
    /// assert_eq!(Float::ONE.is_sign_positive(), true);
    /// assert_eq!(Float::NEGATIVE_ONE.is_sign_positive(), false);
    /// ```
    pub const fn is_sign_positive(&self) -> bool {
        match self {
            float_nan!() => false,
            Float(Infinity { sign } | Finite { sign, .. } | Zero { sign, .. }) => *sign,
        }
    }

    /// Determines whether a [`Float`]'s sign is negative.
    ///
    /// A NaN has no sign, so this function returns false when given a NaN.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
    /// };
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.is_sign_negative(), false);
    /// assert_eq!(Float::INFINITY.is_sign_negative(), false);
    /// assert_eq!(Float::NEGATIVE_INFINITY.is_sign_negative(), true);
    /// assert_eq!(Float::ZERO.is_sign_negative(), false);
    /// assert_eq!(Float::NEGATIVE_ZERO.is_sign_negative(), true);
    /// assert_eq!(Float::ONE.is_sign_negative(), false);
    /// assert_eq!(Float::NEGATIVE_ONE.is_sign_negative(), true);
    /// ```
    pub const fn is_sign_negative(&self) -> bool {
        match self {
            float_nan!() => false,
            Float(Infinity { sign } | Finite { sign, .. } | Zero { sign, .. }) => !*sign,
        }
    }

    /// Classifies a [`Float`] into one of several categories.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
    /// };
    /// use malachite_float::Float;
    /// use std::num::FpCategory;
    ///
    /// assert_eq!(Float::NAN.classify(), FpCategory::Nan);
    /// assert_eq!(Float::INFINITY.classify(), FpCategory::Infinite);
    /// assert_eq!(Float::NEGATIVE_INFINITY.classify(), FpCategory::Infinite);
    /// assert_eq!(Float::ZERO.classify(), FpCategory::Zero);
    /// assert_eq!(Float::NEGATIVE_ZERO.classify(), FpCategory::Zero);
    /// assert_eq!(Float::ONE.classify(), FpCategory::Normal);
    /// assert_eq!(Float::NEGATIVE_ONE.classify(), FpCategory::Normal);
    /// ```
    pub const fn classify(&self) -> FpCategory {
        match self {
            float_nan!() => FpCategory::Nan,
            float_either_infinity!() => FpCategory::Infinite,
            Float(Zero { .. }) => FpCategory::Zero,
            _ => FpCategory::Normal,
        }
    }

    /// Turns a NaN into a `None` and wraps any non-NaN [`Float`] with a `Some`. The [`Float`] is
    /// taken by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeZero, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.into_non_nan(), None);
    /// assert_eq!(Float::INFINITY.into_non_nan(), Some(Float::INFINITY));
    /// assert_eq!(Float::ZERO.into_non_nan(), Some(Float::ZERO));
    /// assert_eq!(
    ///     Float::NEGATIVE_ZERO.into_non_nan(),
    ///     Some(Float::NEGATIVE_ZERO)
    /// );
    /// assert_eq!(Float::ONE.into_non_nan(), Some(Float::ONE));
    /// ```
    #[allow(clippy::missing_const_for_fn)] // destructor doesn't work with const
    pub fn into_non_nan(self) -> Option<Float> {
        match self {
            float_nan!() => None,
            x => Some(x),
        }
    }

    /// Turns a NaN into a `None` and wraps any non-NaN [`Float`] with a `Some`. The [`Float`] is
    /// taken by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeZero, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.to_non_nan(), None);
    /// assert_eq!(Float::INFINITY.to_non_nan(), Some(Float::INFINITY));
    /// assert_eq!(Float::ZERO.to_non_nan(), Some(Float::ZERO));
    /// assert_eq!(
    ///     Float::NEGATIVE_ZERO.to_non_nan(),
    ///     Some(Float::NEGATIVE_ZERO)
    /// );
    /// assert_eq!(Float::ONE.to_non_nan(), Some(Float::ONE));
    /// ```
    #[allow(clippy::missing_const_for_fn)] // destructor doesn't work with const
    pub fn to_non_nan(&self) -> Option<Float> {
        match self {
            float_nan!() => None,
            x => Some(x.clone()),
        }
    }

    /// Turns any [`Float`] that's NaN or infinite into a `None` and wraps any finite [`Float`] with
    /// a `Some`. The [`Float`] is taken by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeZero, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.into_finite(), None);
    /// assert_eq!(Float::INFINITY.into_finite(), None);
    /// assert_eq!(Float::ZERO.into_finite(), Some(Float::ZERO));
    /// assert_eq!(
    ///     Float::NEGATIVE_ZERO.into_finite(),
    ///     Some(Float::NEGATIVE_ZERO)
    /// );
    /// assert_eq!(Float::ONE.into_finite(), Some(Float::ONE));
    /// ```
    #[allow(clippy::missing_const_for_fn)] // destructor doesn't work with const
    pub fn into_finite(self) -> Option<Float> {
        match self {
            Float(NaN | Infinity { .. }) => None,
            x => Some(x),
        }
    }

    /// Turns any [`Float`] that's NaN or infinite into a `None` and wraps any finite [`Float`] with
    /// a `Some`. The [`Float`] is taken by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeZero, One, Zero};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.to_finite(), None);
    /// assert_eq!(Float::INFINITY.to_finite(), None);
    /// assert_eq!(Float::ZERO.to_finite(), Some(Float::ZERO));
    /// assert_eq!(Float::NEGATIVE_ZERO.to_finite(), Some(Float::NEGATIVE_ZERO));
    /// assert_eq!(Float::ONE.to_finite(), Some(Float::ONE));
    /// ```
    pub fn to_finite(&self) -> Option<Float> {
        match self {
            Float(NaN | Infinity { .. }) => None,
            x => Some(x.clone()),
        }
    }
}
