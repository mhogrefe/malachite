// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use core::num::FpCategory;

impl BigExponentFloat {
    /// Determines whether a [`BigExponentFloat`] is NaN.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{NaN, One};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.is_nan(), true);
    /// assert_eq!(BigExponentFloat::ONE.is_nan(), false);
    /// ```
    #[inline]
    pub const fn is_nan(&self) -> bool {
        matches!(self, Self(NaN))
    }

    /// Determines whether a [`BigExponentFloat`] is finite.
    ///
    /// NaN is not finite.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.is_finite(), false);
    /// assert_eq!(BigExponentFloat::INFINITY.is_finite(), false);
    /// assert_eq!(BigExponentFloat::ONE.is_finite(), true);
    /// ```
    #[inline]
    pub const fn is_finite(&self) -> bool {
        matches!(self, Self(Zero | Finite { .. }))
    }

    /// Determines whether a [`BigExponentFloat`] is infinite.
    ///
    /// NaN is not infinite.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.is_infinite(), false);
    /// assert_eq!(BigExponentFloat::INFINITY.is_infinite(), true);
    /// assert_eq!(BigExponentFloat::ONE.is_infinite(), false);
    /// ```
    #[inline]
    pub const fn is_infinite(&self) -> bool {
        matches!(self, Self(Infinity { .. }))
    }

    /// Determines whether a [`BigExponentFloat`] is zero. There is only one zero, which is
    /// unsigned.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.is_zero(), false);
    /// assert_eq!(BigExponentFloat::INFINITY.is_zero(), false);
    /// assert_eq!(BigExponentFloat::ONE.is_zero(), false);
    /// assert_eq!(BigExponentFloat::ZERO.is_zero(), true);
    /// ```
    #[inline]
    pub const fn is_zero(&self) -> bool {
        matches!(self, Self(Zero))
    }

    /// Determines whether a [`BigExponentFloat`] is normal, that is, finite and nonzero.
    ///
    /// The exponent is unbounded, so there are no subnormal [`BigExponentFloat`]s.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.is_normal(), false);
    /// assert_eq!(BigExponentFloat::INFINITY.is_normal(), false);
    /// assert_eq!(BigExponentFloat::ZERO.is_normal(), false);
    /// assert_eq!(BigExponentFloat::ONE.is_normal(), true);
    /// ```
    #[inline]
    pub const fn is_normal(&self) -> bool {
        matches!(self, Self(Finite { .. }))
    }

    /// Determines whether a [`BigExponentFloat`]'s sign is positive.
    ///
    /// A NaN has no sign, so this function returns false when given a NaN. Zero is treated as
    /// positive, like a [`Float`](crate::Float)'s positive zero.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero,
    /// };
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.is_sign_positive(), false);
    /// assert_eq!(BigExponentFloat::INFINITY.is_sign_positive(), true);
    /// assert_eq!(
    ///     BigExponentFloat::NEGATIVE_INFINITY.is_sign_positive(),
    ///     false
    /// );
    /// assert_eq!(BigExponentFloat::ZERO.is_sign_positive(), true);
    /// assert_eq!(BigExponentFloat::ONE.is_sign_positive(), true);
    /// assert_eq!(BigExponentFloat::NEGATIVE_ONE.is_sign_positive(), false);
    /// ```
    pub const fn is_sign_positive(&self) -> bool {
        match self {
            Self(NaN) => false,
            Self(Zero) => true,
            Self(Infinity { sign } | Finite { sign, .. }) => *sign,
        }
    }

    /// Determines whether a [`BigExponentFloat`]'s sign is negative.
    ///
    /// A NaN has no sign, so this function returns false when given a NaN. Zero is treated as
    /// positive, so this function returns false when given zero.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero,
    /// };
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(BigExponentFloat::NAN.is_sign_negative(), false);
    /// assert_eq!(BigExponentFloat::INFINITY.is_sign_negative(), false);
    /// assert_eq!(BigExponentFloat::NEGATIVE_INFINITY.is_sign_negative(), true);
    /// assert_eq!(BigExponentFloat::ZERO.is_sign_negative(), false);
    /// assert_eq!(BigExponentFloat::ONE.is_sign_negative(), false);
    /// assert_eq!(BigExponentFloat::NEGATIVE_ONE.is_sign_negative(), true);
    /// ```
    pub const fn is_sign_negative(&self) -> bool {
        match self {
            Self(NaN | Zero) => false,
            Self(Infinity { sign } | Finite { sign, .. }) => !*sign,
        }
    }

    /// Classifies a [`BigExponentFloat`] into one of several categories.
    ///
    /// The exponent is unbounded, so the result is never [`FpCategory::Subnormal`].
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero,
    /// };
    /// use malachite_float::BigExponentFloat;
    /// use std::num::FpCategory;
    ///
    /// assert_eq!(BigExponentFloat::NAN.classify(), FpCategory::Nan);
    /// assert_eq!(BigExponentFloat::INFINITY.classify(), FpCategory::Infinite);
    /// assert_eq!(
    ///     BigExponentFloat::NEGATIVE_INFINITY.classify(),
    ///     FpCategory::Infinite
    /// );
    /// assert_eq!(BigExponentFloat::ZERO.classify(), FpCategory::Zero);
    /// assert_eq!(BigExponentFloat::ONE.classify(), FpCategory::Normal);
    /// assert_eq!(
    ///     BigExponentFloat::NEGATIVE_ONE.classify(),
    ///     FpCategory::Normal
    /// );
    /// ```
    pub const fn classify(&self) -> FpCategory {
        match self {
            Self(NaN) => FpCategory::Nan,
            Self(Infinity { .. }) => FpCategory::Infinite,
            Self(Zero) => FpCategory::Zero,
            Self(Finite { .. }) => FpCategory::Normal,
        }
    }

    /// Turns a NaN into a `None` and wraps any non-NaN [`BigExponentFloat`] with a `Some`. The
    /// [`BigExponentFloat`] is taken by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::NAN.into_non_nan()),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::INFINITY.into_non_nan()),
    ///     "Some(Infinity)"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::ZERO.into_non_nan()),
    ///     "Some(0x0.0)"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::ONE.into_non_nan()),
    ///     "Some(0x1.0)"
    /// );
    /// ```
    #[allow(clippy::missing_const_for_fn)] // destructor doesn't work with const
    pub fn into_non_nan(self) -> Option<Self> {
        match self {
            Self(NaN) => None,
            x => Some(x),
        }
    }

    /// Turns a NaN into a `None` and wraps any non-NaN [`BigExponentFloat`] with a `Some`. The
    /// [`BigExponentFloat`] is taken by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(format!("{:?}", BigExponentFloat::NAN.to_non_nan()), "None");
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::INFINITY.to_non_nan()),
    ///     "Some(Infinity)"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::ZERO.to_non_nan()),
    ///     "Some(0x0.0)"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::ONE.to_non_nan()),
    ///     "Some(0x1.0)"
    /// );
    /// ```
    pub fn to_non_nan(&self) -> Option<Self> {
        match self {
            Self(NaN) => None,
            x => Some(x.clone()),
        }
    }

    /// Turns any [`BigExponentFloat`] that's NaN or infinite into a `None` and wraps any finite
    /// [`BigExponentFloat`] with a `Some`. The [`BigExponentFloat`] is taken by value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(format!("{:?}", BigExponentFloat::NAN.into_finite()), "None");
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::INFINITY.into_finite()),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::ZERO.into_finite()),
    ///     "Some(0x0.0)"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::ONE.into_finite()),
    ///     "Some(0x1.0)"
    /// );
    /// ```
    #[allow(clippy::missing_const_for_fn)] // destructor doesn't work with const
    pub fn into_finite(self) -> Option<Self> {
        match self {
            Self(NaN | Infinity { .. }) => None,
            x => Some(x),
        }
    }

    /// Turns any [`BigExponentFloat`] that's NaN or infinite into a `None` and wraps any finite
    /// [`BigExponentFloat`] with a `Some`. The [`BigExponentFloat`] is taken by reference.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.complexity()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, One, Zero};
    /// use malachite_float::BigExponentFloat;
    ///
    /// assert_eq!(format!("{:?}", BigExponentFloat::NAN.to_finite()), "None");
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::INFINITY.to_finite()),
    ///     "None"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::ZERO.to_finite()),
    ///     "Some(0x0.0)"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::ONE.to_finite()),
    ///     "Some(0x1.0)"
    /// );
    /// ```
    pub fn to_finite(&self) -> Option<Self> {
        match self {
            Self(NaN | Infinity { .. }) => None,
            x => Some(x.clone()),
        }
    }
}
