// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use malachite_base::num::arithmetic::traits::IsUnit;

impl IsUnit for GaussianInteger {
    /// Determines whether a [`GaussianInteger`] is a unit: one of $1$, $-1$, $i$, and $-i$, the
    /// four elements of $\mathbb{Z}[i]$ with a multiplicative inverse in $\mathbb{Z}[i]$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::IsUnit;
    /// use malachite_base::num::basic::traits::{I, NegativeI, NegativeOne, One, Two, Zero};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianInteger::ONE.is_unit(), true);
    /// assert_eq!(GaussianInteger::NEGATIVE_ONE.is_unit(), true);
    /// assert_eq!(GaussianInteger::I.is_unit(), true);
    /// assert_eq!(GaussianInteger::NEGATIVE_I.is_unit(), true);
    /// assert_eq!(GaussianInteger::ZERO.is_unit(), false);
    /// assert_eq!(GaussianInteger::from_str("1+i").unwrap().is_unit(), false);
    /// assert_eq!(GaussianInteger::TWO.is_unit(), false);
    /// ```
    fn is_unit(&self) -> bool {
        if self.imaginary == 0u32 {
            self.real == 1u32 || self.real == -1i32
        } else if self.real == 0u32 {
            self.imaginary == 1u32 || self.imaginary == -1i32
        } else {
            false
        }
    }
}
