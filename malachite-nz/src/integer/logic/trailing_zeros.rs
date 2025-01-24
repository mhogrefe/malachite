// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;

impl Integer {
    /// Returns the number of trailing zeros in the binary expansion of an [`Integer`]
    /// (equivalently, the multiplicity of 2 in its prime factorization), or `None` is the
    /// [`Integer`] is 0.
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
    /// assert_eq!(Integer::ZERO.trailing_zeros(), None);
    /// assert_eq!(Integer::from(3).trailing_zeros(), Some(0));
    /// assert_eq!(Integer::from(-72).trailing_zeros(), Some(3));
    /// assert_eq!(Integer::from(100).trailing_zeros(), Some(2));
    /// assert_eq!((-Integer::from(10u32).pow(12)).trailing_zeros(), Some(12));
    /// ```
    pub fn trailing_zeros(&self) -> Option<u64> {
        self.abs.trailing_zeros()
    }
}
