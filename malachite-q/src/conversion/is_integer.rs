// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::conversion::traits::IsInteger;

impl<'a> IsInteger for &'a Rational {
    /// Determines whether a [`Rational`] is an integer.
    ///
    /// $f(x) = x \in \Z$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::num::conversion::traits::IsInteger;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.is_integer(), true);
    /// assert_eq!(Rational::ONE.is_integer(), true);
    /// assert_eq!(Rational::from(100).is_integer(), true);
    /// assert_eq!(Rational::from(-100).is_integer(), true);
    /// assert_eq!(Rational::from_signeds(22, 7).is_integer(), false);
    /// assert_eq!(Rational::from_signeds(-22, 7).is_integer(), false);
    /// ```
    #[inline]
    fn is_integer(self) -> bool {
        self.denominator == 1u32
    }
}
