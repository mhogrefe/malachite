// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::IsUnit;

impl IsUnit for Rational {
    /// Determines whether a [`Rational`] is a unit: whether it is nonzero, since every nonzero
    /// [`Rational`] has a multiplicative inverse.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::IsUnit;
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ONE.is_unit(), true);
    /// assert_eq!(Rational::from_signeds(-22, 7).is_unit(), true);
    /// assert_eq!(Rational::ZERO.is_unit(), false);
    /// ```
    #[inline]
    fn is_unit(&self) -> bool {
        *self != 0u32
    }
}
