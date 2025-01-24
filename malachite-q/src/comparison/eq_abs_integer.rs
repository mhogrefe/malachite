// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::comparison::traits::EqAbs;
use malachite_nz::integer::Integer;

impl EqAbs<Integer> for Rational {
    /// Determines whether the absolute values of a [`Rational`] and an [`Integer`] are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::from(-123).eq_abs(&Integer::from(122)), false);
    /// assert_eq!(Rational::from(-123).eq_abs(&Integer::from(124)), false);
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).eq_abs(&Integer::from(123)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).eq_abs(&Integer::from(123)),
    ///     false
    /// );
    /// assert_eq!(Rational::from(123).eq_abs(&Integer::from(123)), true);
    /// assert_eq!(Rational::from(-123).eq_abs(&Integer::from(123)), true);
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).eq_abs(&Integer::from(123)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).eq_abs(&Integer::from(123)),
    ///     false
    /// );
    /// assert_eq!(Rational::from(-123).eq_abs(&Integer::from(-122)), false);
    /// assert_eq!(Rational::from(-123).eq_abs(&Integer::from(-124)), false);
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).eq_abs(&Integer::from(-123)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).eq_abs(&Integer::from(1 - 23)),
    ///     false
    /// );
    /// assert_eq!(Rational::from(123).eq_abs(&Integer::from(-123)), true);
    /// assert_eq!(Rational::from(-123).eq_abs(&Integer::from(-123)), true);
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).eq_abs(&Integer::from(-123)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).eq_abs(&Integer::from(-123)),
    ///     false
    /// );
    /// ```
    #[inline]
    fn eq_abs(&self, other: &Integer) -> bool {
        self.denominator == 1 && self.numerator == *other.unsigned_abs_ref()
    }
}

impl EqAbs<Rational> for Integer {
    /// Determines whether the absolute values of a [`Rational`] and an [`Integer`] are equal.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_nz::integer::Integer;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Integer::from(122).eq_abs(&Rational::from(-123)), false);
    /// assert_eq!(Integer::from(124).eq_abs(&Rational::from(-123)), false);
    /// assert_eq!(
    ///     Integer::from(124).eq_abs(&Rational::from_signeds(22, 7)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Integer::from(124).eq_abs(&Rational::from_signeds(-22, 7)),
    ///     false
    /// );
    /// assert_eq!(Integer::from(123).eq_abs(&Rational::from(123)), true);
    /// assert_eq!(Integer::from(123).eq_abs(&Rational::from(-123)), true);
    /// assert_eq!(
    ///     Integer::from(123).eq_abs(&Rational::from_signeds(22, 7)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Integer::from(123).eq_abs(&Rational::from_signeds(-22, 7)),
    ///     false
    /// );
    /// assert_eq!(Integer::from(-122).eq_abs(&Rational::from(-123)), false);
    /// assert_eq!(Integer::from(-124).eq_abs(&Rational::from(-123)), false);
    /// assert_eq!(
    ///     Integer::from(-124).eq_abs(&Rational::from_signeds(22, 7)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Integer::from(-124).eq_abs(&Rational::from_signeds(-22, 7)),
    ///     false
    /// );
    /// assert_eq!(Integer::from(-123).eq_abs(&Rational::from(123)), true);
    /// assert_eq!(Integer::from(-123).eq_abs(&Rational::from(-123)), true);
    /// assert_eq!(
    ///     Integer::from(-123).eq_abs(&Rational::from_signeds(22, 7)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Integer::from(-123).eq_abs(&Rational::from_signeds(-22, 7)),
    ///     false
    /// );
    /// ```
    #[inline]
    fn eq_abs(&self, other: &Rational) -> bool {
        other.denominator == 1 && other.numerator == *self.unsigned_abs_ref()
    }
}
