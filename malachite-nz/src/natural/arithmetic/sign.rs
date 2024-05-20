// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Sign;

impl Sign for Natural {
    /// Compares a [`Natural`] to zero.
    ///
    /// Returns `Greater` or `Equal` depending on whether the [`Natural`] is positive or zero,
    /// respectively.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use core::cmp::Ordering::*;
    /// use malachite_base::num::arithmetic::traits::Sign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.sign(), Equal);
    /// assert_eq!(Natural::from(123u32).sign(), Greater);
    /// ```
    fn sign(&self) -> Ordering {
        if *self == 0 {
            Equal
        } else {
            Greater
        }
    }
}
