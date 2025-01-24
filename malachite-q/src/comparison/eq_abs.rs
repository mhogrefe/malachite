// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::comparison::traits::EqAbs;

impl EqAbs for Rational {
    /// Determines whether the absolute values of two [`Rational`]s are equal.
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
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).eq_abs(&Rational::from_signeds(-23, 7)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).eq_abs(&Rational::from_signeds(-24, 7)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).eq_abs(&Rational::from_signeds(22, 7)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7).eq_abs(&Rational::from_signeds(-22, 7)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).eq_abs(&Rational::from_signeds(22, 7)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(-22, 7).eq_abs(&Rational::from_signeds(-22, 7)),
    ///     true
    /// );
    /// ```
    #[inline]
    fn eq_abs(&self, other: &Rational) -> bool {
        self.numerator == other.numerator && self.denominator == other.denominator
    }
}
