// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::logic::traits::CountOnes;

impl Integer {
    /// Counts the number of ones in the binary expansion of an [`Integer`]. If the [`Integer`] is
    /// negative, then the number of ones is infinite, so `None` is returned.
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
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.checked_count_ones(), Some(0));
    /// // 105 = 1101001b
    /// assert_eq!(Integer::from(105).checked_count_ones(), Some(4));
    /// assert_eq!(Integer::from(-105).checked_count_ones(), None);
    /// // 10^12 = 1110100011010100101001010001000000000000b
    /// assert_eq!(Integer::from(10u32).pow(12).checked_count_ones(), Some(13));
    /// ```
    pub fn checked_count_ones(&self) -> Option<u64> {
        if self.sign {
            Some(self.abs.count_ones())
        } else {
            None
        }
    }
}
