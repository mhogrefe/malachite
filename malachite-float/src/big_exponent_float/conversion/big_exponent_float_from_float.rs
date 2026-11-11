// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use crate::{Float, InnerFloat};
use malachite_nz::integer::Integer;

impl From<Float> for BigExponentFloat {
    /// Converts a [`Float`] to a [`BigExponentFloat`], taking the [`Float`] by value.
    ///
    /// The conversion is exact: every [`Float`] has the same value as some [`BigExponentFloat`].
    /// The precision of the [`Float`] is not kept; the result's precision is the [`Float`]'s
    /// minimum precision. Both zeros become zero, since a [`BigExponentFloat`] has no negative
    /// zero.
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
    /// use malachite_base::num::basic::traits::{NegativeZero, One};
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert_eq!(format!("{:?}", BigExponentFloat::from(Float::ONE)), "0x1.0");
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::from(Float::from(-1.5))),
    ///     "-0x1.8"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::from(Float::NEGATIVE_ZERO)),
    ///     "0x0.0"
    /// );
    /// ```
    fn from(x: Float) -> Self {
        match x {
            Float(InnerFloat::NaN) => Self(NaN),
            Float(InnerFloat::Infinity { sign }) => Self(Infinity { sign }),
            Float(InnerFloat::Zero { .. }) => Self(Zero),
            Float(InnerFloat::Finite {
                sign,
                exponent,
                mut significand,
                ..
            }) => {
                Self::trim_significand(&mut significand);
                Self(Finite {
                    sign,
                    exponent: Integer::from(exponent),
                    significand,
                })
            }
        }
    }
}

impl From<&Float> for BigExponentFloat {
    /// Converts a [`Float`] to a [`BigExponentFloat`], taking the [`Float`] by reference.
    ///
    /// The conversion is exact: every [`Float`] has the same value as some [`BigExponentFloat`].
    /// The precision of the [`Float`] is not kept; the result's precision is the [`Float`]'s
    /// minimum precision. Both zeros become zero, since a [`BigExponentFloat`] has no negative
    /// zero.
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
    /// use malachite_base::num::basic::traits::{Infinity, One};
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::from(&Float::ONE)),
    ///     "0x1.0"
    /// );
    /// assert_eq!(
    ///     format!("{:?}", BigExponentFloat::from(&Float::INFINITY)),
    ///     "Infinity"
    /// );
    /// ```
    #[inline]
    fn from(x: &Float) -> Self {
        Self::from(x.clone())
    }
}
