// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::Float;
use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::rounding_modes::RoundingMode::{self, *};

impl BigExponentFloat {
    /// Converts a primitive float to a [`BigExponentFloat`], rounding to a given precision with the
    /// specified rounding mode. An [`Ordering`] is also returned, indicating whether the returned
    /// value is less than, equal to, or greater than the original value.
    ///
    /// NaN and $\pm\infty$ become the corresponding [`BigExponentFloat`]s, and both zeros become
    /// zero, all with `Equal`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `prec` is zero, or if `rm` is `Exact` but `x` cannot be represented exactly with
    /// the given precision.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode::*;
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from_primitive_float_prec_round(1.75f64, 2, Floor);
    /// assert_eq!(format!("{x:?}"), "0x1.8");
    /// assert_eq!(o, Less);
    /// ```
    #[inline]
    pub fn from_primitive_float_prec_round<T: PrimitiveFloat>(
        x: T,
        prec: u64,
        rm: RoundingMode,
    ) -> (Self, Ordering)
    where
        Float: From<T>,
    {
        Self::from(Float::from(x)).round_to_prec_round(prec, rm)
    }

    /// Converts a primitive float to a [`BigExponentFloat`], rounding to the nearest value of a
    /// given precision. An [`Ordering`] is also returned, indicating whether the returned value is
    /// less than, equal to, or greater than the original value.
    ///
    /// NaN and $\pm\infty$ become the corresponding [`BigExponentFloat`]s, and both zeros become
    /// zero, all with `Equal`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `prec` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// let (x, o) = BigExponentFloat::from_primitive_float_prec(1.75f64, 2);
    /// assert_eq!(format!("{x:?}"), "0x2.0");
    /// assert_eq!(o, Greater);
    /// ```
    #[inline]
    pub fn from_primitive_float_prec<T: PrimitiveFloat>(x: T, prec: u64) -> (Self, Ordering)
    where
        Float: From<T>,
    {
        Self::from_primitive_float_prec_round(x, prec, Nearest)
    }
}

macro_rules! impl_from_primitive_float {
    ($t: ident) => {
        impl From<$t> for BigExponentFloat {
            /// Converts a primitive float to a [`BigExponentFloat`], exactly.
            ///
            /// NaN and $\pm\infty$ become the corresponding [`BigExponentFloat`]s. Both zeros
            /// become zero, since a [`BigExponentFloat`] has no negative zero. Otherwise, the
            /// result's precision is the number of bits from the most significant 1 of `x`'s
            /// significand to its least significant 1.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::from_primitive_float#from).
            #[inline]
            fn from(x: $t) -> BigExponentFloat {
                BigExponentFloat::from(Float::from(x))
            }
        }
    };
}
apply_to_primitive_floats!(impl_from_primitive_float);
