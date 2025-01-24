// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::logic::traits::NotAssign;

impl NotAssign for bool {
    /// Replaces a [`bool`] by its opposite.
    ///
    /// $b \gets \lnot b$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::NotAssign;
    ///
    /// let mut b = false;
    /// b.not_assign();
    /// assert_eq!(b, true);
    ///
    /// let mut b = true;
    /// b.not_assign();
    /// assert_eq!(b, false);
    /// ```
    #[inline]
    fn not_assign(&mut self) {
        *self = !*self;
    }
}
