// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::ModPowerOf2IsReduced;
use malachite_base::num::logic::traits::SignificantBits;

impl ModPowerOf2IsReduced for Natural {
    /// Returns whether a [`Natural`] is reduced modulo 2^k$; in other words, whether it has no more
    /// than $k$ significant bits.
    ///
    /// $f(x, k) = (x < 2^k)$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModPowerOf2IsReduced, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_power_of_2_is_reduced(5), true);
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).mod_power_of_2_is_reduced(39),
    ///     false
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).mod_power_of_2_is_reduced(40),
    ///     true
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_is_reduced(&self, pow: u64) -> bool {
        self.significant_bits() <= pow
    }
}
