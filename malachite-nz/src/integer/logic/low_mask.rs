// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::logic::traits::LowMask;

impl LowMask for Integer {
    /// Returns an [`Integer`] whose least significant $b$ bits are `true` and whose other bits are
    /// `false`.
    ///
    /// $f(b) = 2^b - 1$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `bits`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::LowMask;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::low_mask(0), 0);
    /// assert_eq!(Integer::low_mask(3), 7);
    /// assert_eq!(
    ///     Integer::low_mask(100).to_string(),
    ///     "1267650600228229401496703205375"
    /// );
    /// ```
    #[inline]
    fn low_mask(bits: u64) -> Integer {
        Integer::from(Natural::low_mask(bits))
    }
}
