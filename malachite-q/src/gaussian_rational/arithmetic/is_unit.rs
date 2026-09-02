// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::arithmetic::traits::IsUnit;

impl IsUnit for GaussianRational {
    /// Determines whether a [`GaussianRational`] is a unit: nonzero. $\mathbb{Q}(i)$ is a field, so
    /// every nonzero element has a multiplicative inverse and the only non-unit is zero. This
    /// differs from the four units of $\mathbb{Z}[i]$, which are the only
    /// [`GaussianInteger`](malachite_nz::gaussian_integer::GaussianInteger) units.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::IsUnit;
    /// use malachite_base::num::basic::traits::{I, NegativeI, NegativeOne, One, Zero};
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianRational::ONE.is_unit(), true);
    /// assert_eq!(GaussianRational::NEGATIVE_ONE.is_unit(), true);
    /// assert_eq!(GaussianRational::I.is_unit(), true);
    /// assert_eq!(GaussianRational::NEGATIVE_I.is_unit(), true);
    /// assert_eq!(GaussianRational::from_str("1/2+i").unwrap().is_unit(), true);
    /// assert_eq!(GaussianRational::ZERO.is_unit(), false);
    /// ```
    fn is_unit(&self) -> bool {
        self.real != 0u32 || self.imaginary != 0u32
    }
}
