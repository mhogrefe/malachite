// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Sign;

impl Sign for Rational {
    /// Compares a [`Rational`] to zero.
    ///
    /// Returns `Greater`, `Equal`, or `Less`, depending on whether the [`Rational`] is positive,
    /// zero, or negative, respectively.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_q::Rational;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(Rational::ZERO.sign(), Equal);
    /// assert_eq!(Rational::from_signeds(22, 7).sign(), Greater);
    /// assert_eq!(Rational::from_signeds(-22, 7).sign(), Less);
    /// ```
    fn sign(&self) -> Ordering {
        if self.sign {
            if self.numerator == 0 {
                Equal
            } else {
                Greater
            }
        } else {
            Less
        }
    }
}
