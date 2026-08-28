// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::conversion::traits::{IsGaussianInteger, IsInteger};

impl IsGaussianInteger for &Rational {
    /// Determines whether a [`Rational`] is a Gaussian integer.
    ///
    /// A [`Rational`] is real, so it is a Gaussian integer if and only if it is an integer.
    ///
    /// $f(x) = x \in \Z\[i\]$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_base::num::conversion::traits::IsGaussianInteger;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(Rational::ZERO.is_gaussian_integer(), true);
    /// assert_eq!(Rational::ONE.is_gaussian_integer(), true);
    /// assert_eq!(Rational::from(-100).is_gaussian_integer(), true);
    /// assert_eq!(Rational::from_signeds(22, 7).is_gaussian_integer(), false);
    /// ```
    #[inline]
    fn is_gaussian_integer(self) -> bool {
        self.is_integer()
    }
}
