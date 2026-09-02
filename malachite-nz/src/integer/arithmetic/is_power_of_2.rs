// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::IsPowerOf2;

impl IsPowerOf2 for Integer {
    /// Determines whether an [`Integer`] is an integer power of 2.
    ///
    /// Negative values are never powers of 2.
    ///
    /// $f(x) = (\exists n \in \N : 2^n = x)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::{IsPowerOf2, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.is_power_of_2(), false);
    /// assert_eq!(Integer::from(123).is_power_of_2(), false);
    /// assert_eq!(Integer::from(0x80).is_power_of_2(), true);
    /// assert_eq!(Integer::from(-0x80).is_power_of_2(), false);
    /// assert_eq!(Integer::from(10).pow(12).is_power_of_2(), false);
    /// assert_eq!(
    ///     Integer::from_str("1099511627776").unwrap().is_power_of_2(),
    ///     true
    /// );
    /// ```
    #[inline]
    fn is_power_of_2(&self) -> bool {
        self.sign && self.abs.is_power_of_2()
    }
}
