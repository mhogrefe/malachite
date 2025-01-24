// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::logic::traits::SignificantBits;

impl SignificantBits for &Integer {
    /// Returns the number of significant bits of an [`Integer`]'s absolute value.
    ///
    /// $$
    /// f(n) = \\begin{cases}
    ///     0 & \text{if} \\quad n = 0, \\\\
    ///     \lfloor \log_2 |n| \rfloor + 1 & \\text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.significant_bits(), 0);
    /// assert_eq!(Integer::from(100).significant_bits(), 7);
    /// assert_eq!(Integer::from(-100).significant_bits(), 7);
    /// ```
    fn significant_bits(self) -> u64 {
        self.abs.significant_bits()
    }
}
