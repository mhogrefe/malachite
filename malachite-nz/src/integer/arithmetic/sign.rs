// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Sign;

impl Sign for Integer {
    /// Compares an [`Integer`] to zero.
    ///
    /// Returns `Greater`, `Equal`, or `Less`, depending on whether the [`Integer`] is positive,
    /// zero, or negative, respectively.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::Sign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.sign(), Equal);
    /// assert_eq!(Integer::from(123).sign(), Greater);
    /// assert_eq!(Integer::from(-123).sign(), Less);
    /// ```
    fn sign(&self) -> Ordering {
        if self.sign {
            if self.abs == 0 {
                Equal
            } else {
                Greater
            }
        } else {
            Less
        }
    }
}
