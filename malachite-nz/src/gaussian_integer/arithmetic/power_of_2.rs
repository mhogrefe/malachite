// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::PowerOf2;

impl PowerOf2<u64> for GaussianInteger {
    /// Raises 2 to an integer power, producing a purely real [`GaussianInteger`].
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
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    ///
    /// assert_eq!(GaussianInteger::power_of_2(0).to_string(), "1");
    /// assert_eq!(GaussianInteger::power_of_2(3).to_string(), "8");
    /// assert_eq!(
    ///     GaussianInteger::power_of_2(100).to_string(),
    ///     "1267650600228229401496703205376"
    /// );
    /// ```
    #[inline]
    fn power_of_2(pow: u64) -> Self {
        Self::from(Integer::power_of_2(pow))
    }
}
