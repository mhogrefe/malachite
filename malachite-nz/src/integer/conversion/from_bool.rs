// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::basic::traits::{One, Zero};

impl From<bool> for Integer {
    /// Converts a [`bool`] to 0 or 1.
    ///
    /// This function is known as the [Iverson
    /// bracket](https://en.wikipedia.org/wiki/Iverson_bracket).
    ///
    /// $$
    /// f(P) = \[P\] = \\begin{cases}
    ///     1 & \text{if} \\quad P, \\\\
    ///     0 & \\text{otherwise}.
    /// \\end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(false), 0);
    /// assert_eq!(Integer::from(true), 1);
    /// ```
    #[inline]
    fn from(b: bool) -> Integer {
        if b {
            Integer::ONE
        } else {
            Integer::ZERO
        }
    }
}
