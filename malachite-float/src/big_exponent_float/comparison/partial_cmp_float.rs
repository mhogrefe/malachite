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
use core::cmp::Ordering::{self, *};

impl PartialOrd<Float> for BigExponentFloat {
    /// Compares a [`BigExponentFloat`] to a [`Float`].
    ///
    /// This implementation follows the IEEE 754 standard: NaN is not comparable to anything. A
    /// [`BigExponentFloat`] has no stored precision and a [`Float`] does; values with different
    /// precisions are equal if they represent the same number. Both [`Float`] zeros are equal to
    /// the [`BigExponentFloat`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeZero, One, OneHalf, Zero};
    /// use malachite_base::num::conversion::traits::IntegerMantissaAndExponent;
    /// use malachite_float::{BigExponentFloat, Float};
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(BigExponentFloat::ONE_HALF < Float::ONE);
    /// assert!(BigExponentFloat::ZERO == Float::NEGATIVE_ZERO);
    /// assert!(BigExponentFloat::INFINITY > Float::max_finite_value_with_prec(10));
    /// assert_eq!(BigExponentFloat::NAN.partial_cmp(&Float::NAN), None);
    ///
    /// let huge = BigExponentFloat::from_integer_mantissa_and_exponent(
    ///     Natural::ONE,
    ///     Integer::from_str("100000000000000000000").unwrap(),
    /// )
    /// .unwrap();
    /// assert!(huge > Float::max_finite_value_with_prec(10));
    /// ```
    fn partial_cmp(&self, other: &Float) -> Option<Ordering> {
        match (self, other) {
            (Self(NaN), _) | (_, Float(InnerFloat::NaN)) => None,
            (Self(Infinity { sign: s_x }), Float(InnerFloat::Infinity { sign: s_y })) => {
                Some(s_x.cmp(s_y))
            }
            (Self(Zero), Float(InnerFloat::Zero { .. })) => Some(Equal),
            (Self(Infinity { sign }), _)
            | (Self(Finite { sign, .. }), Float(InnerFloat::Zero { .. })) => {
                Some(if *sign { Greater } else { Less })
            }
            (_, Float(InnerFloat::Infinity { sign }))
            | (Self(Zero), Float(InnerFloat::Finite { sign, .. })) => {
                Some(if *sign { Less } else { Greater })
            }
            (
                Self(Finite {
                    sign: s_x,
                    exponent: e_x,
                    significand: x,
                }),
                Float(InnerFloat::Finite {
                    sign: s_y,
                    exponent: e_y,
                    significand: y,
                    ..
                }),
            ) => Some(s_x.cmp(s_y).then_with(|| {
                let abs_cmp = e_x
                    .partial_cmp(e_y)
                    .unwrap()
                    .then_with(|| x.cmp_normalized_no_shift(y));
                if *s_x { abs_cmp } else { abs_cmp.reverse() }
            })),
        }
    }
}

impl PartialOrd<BigExponentFloat> for Float {
    /// Compares a [`Float`] to a [`BigExponentFloat`].
    ///
    /// This implementation follows the IEEE 754 standard: NaN is not comparable to anything. A
    /// [`BigExponentFloat`] has no stored precision and a [`Float`] does; values with different
    /// precisions are equal if they represent the same number. Both [`Float`] zeros are equal to
    /// the [`BigExponentFloat`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeZero, One, OneHalf, Zero};
    /// use malachite_float::{BigExponentFloat, Float};
    ///
    /// assert!(Float::ONE > BigExponentFloat::ONE_HALF);
    /// assert!(Float::NEGATIVE_ZERO == BigExponentFloat::ZERO);
    /// assert!(Float::max_finite_value_with_prec(10) < BigExponentFloat::INFINITY);
    /// assert_eq!(Float::NAN.partial_cmp(&BigExponentFloat::NAN), None);
    /// ```
    #[inline]
    fn partial_cmp(&self, other: &BigExponentFloat) -> Option<Ordering> {
        other.partial_cmp(self).map(Ordering::reverse)
    }
}
