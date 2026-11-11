// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Zero};
use core::cmp::Ordering::*;
use malachite_base::num::exponential::traits::CheckedLogBase2;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_q::Rational;

// Determines whether the absolute value of a finite nonzero `BigExponentFloat`, given by its
// exponent and significand, equals a nonzero `Rational`. Equality requires a power-of-2
// denominator, after which no multiplication is needed.
pub(crate) fn finite_eq_abs_nonzero_rational(
    exponent: &Integer,
    significand: &Natural,
    other: &Rational,
) -> bool {
    if let Some(log_d) = other.denominator_ref().checked_log_base_2() {
        let n = other.numerator_ref();
        *exponent == Integer::from(n.significant_bits()) - Integer::from(log_d)
            && significand.cmp_normalized(n) == Equal
    } else {
        false
    }
}

impl PartialEq<Rational> for BigExponentFloat {
    /// Determines whether a [`BigExponentFloat`] is equal to a [`Rational`].
    ///
    /// $\infty$, $-\infty$, and NaN are not equal to any [`Rational`]. The [`BigExponentFloat`]
    /// zero is equal to the [`Rational`] zero.
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    ///
    /// assert!(BigExponentFloat::from(2) == Rational::from(2));
    /// assert!(BigExponentFloat::from(-1.5) == Rational::from_signeds(-3, 2));
    /// assert!(BigExponentFloat::ONE_HALF != Rational::from_signeds(1, 3));
    /// ```
    fn eq(&self, other: &Rational) -> bool {
        match self {
            Self(Zero) => *other == 0u32,
            Self(Finite {
                sign,
                exponent,
                significand,
            }) => {
                *other != 0u32
                    && *sign == (*other > 0u32)
                    && finite_eq_abs_nonzero_rational(exponent, significand, other)
            }
            _ => false,
        }
    }
}

impl PartialEq<BigExponentFloat> for Rational {
    /// Determines whether a [`Rational`] is equal to a [`BigExponentFloat`].
    ///
    /// No [`Rational`] is equal to $\infty$, $-\infty$, or NaN. The [`Rational`] zero is equal to
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
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_q::Rational;
    ///
    /// assert!(Rational::from(2) == BigExponentFloat::from(2));
    /// assert!(Rational::from_signeds(-3, 2) == BigExponentFloat::from(-1.5));
    /// assert!(Rational::from_signeds(1, 3) != BigExponentFloat::ONE_HALF);
    /// ```
    #[inline]
    fn eq(&self, other: &BigExponentFloat) -> bool {
        other == self
    }
}
