// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::One;

impl PowerOf2<u64> for Integer {
    /// Raises 2 to an integer power.
    ///
    /// $f(k) = 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::PowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::power_of_2(0), 1);
    /// assert_eq!(Integer::power_of_2(3), 8);
    /// assert_eq!(
    ///     Integer::power_of_2(100).to_string(),
    ///     "1267650600228229401496703205376"
    /// );
    /// ```
    #[inline]
    fn power_of_2(pow: u64) -> Integer {
        Integer::ONE << pow
    }
}
