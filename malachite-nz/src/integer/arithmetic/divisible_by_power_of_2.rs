// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::DivisibleByPowerOf2;

impl DivisibleByPowerOf2 for &Integer {
    /// Returns whether an [`Integer`] is divisible by $2^k$.
    ///
    /// $f(x, k) = (2^k|x)$.
    ///
    /// $f(x, k) = (\exists n \in \N : \ x = n2^k)$.
    ///
    /// If `self` is 0, the result is always true; otherwise, it is equivalent to
    /// `self.trailing_zeros().unwrap() <= pow`, but more efficient.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `min(pow, self.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{DivisibleByPowerOf2, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.divisible_by_power_of_2(100), true);
    /// assert_eq!(Integer::from(-100).divisible_by_power_of_2(2), true);
    /// assert_eq!(Integer::from(100u32).divisible_by_power_of_2(3), false);
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).divisible_by_power_of_2(12),
    ///     true
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12)).divisible_by_power_of_2(13),
    ///     false
    /// );
    /// ```
    fn divisible_by_power_of_2(self, pow: u64) -> bool {
        self.abs.divisible_by_power_of_2(pow)
    }
}
