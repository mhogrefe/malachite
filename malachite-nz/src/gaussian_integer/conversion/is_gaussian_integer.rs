// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use malachite_base::num::conversion::traits::IsGaussianInteger;

impl IsGaussianInteger for &GaussianInteger {
    /// Determines whether a [`GaussianInteger`] is a Gaussian integer. It always returns `true`.
    ///
    /// $f(x) = \textrm{true}$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{I, One, Zero};
    /// use malachite_base::num::conversion::traits::IsGaussianInteger;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(GaussianInteger::ZERO.is_gaussian_integer(), true);
    /// assert_eq!(GaussianInteger::ONE.is_gaussian_integer(), true);
    /// assert_eq!(GaussianInteger::I.is_gaussian_integer(), true);
    /// ```
    #[inline]
    fn is_gaussian_integer(self) -> bool {
        true
    }
}
