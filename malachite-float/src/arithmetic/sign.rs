// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::{Finite, Infinity, Zero};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Sign;

impl Sign for Float {
    /// Returns the sign of a [`Float`].
    ///
    /// Returns `Greater` if the sign is positive and `Less` if the sign is negative. Never returns
    /// `Equal`. $\infty$ and positive zero have a positive sign, and $-\infty$ and negative zero
    /// have a negative sign.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is NaN.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sign;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
    /// };
    /// use malachite_float::Float;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(Float::INFINITY.sign(), Greater);
    /// assert_eq!(Float::NEGATIVE_INFINITY.sign(), Less);
    /// assert_eq!(Float::ZERO.sign(), Greater);
    /// assert_eq!(Float::NEGATIVE_ZERO.sign(), Less);
    /// assert_eq!(Float::ONE.sign(), Greater);
    /// assert_eq!(Float::NEGATIVE_ONE.sign(), Less);
    /// ```
    fn sign(&self) -> Ordering {
        if let Float(Infinity { sign } | Zero { sign } | Finite { sign, .. }) = self {
            if *sign {
                Greater
            } else {
                Less
            }
        } else {
            panic!()
        }
    }
}
