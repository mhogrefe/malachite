// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use crate::big_exponent_float::InnerBigExponentFloat::{Finite, Infinity, NaN, Zero};
use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::Sign;

impl Sign for BigExponentFloat {
    /// Returns the sign of a [`BigExponentFloat`].
    ///
    /// Returns `Greater` if the [`BigExponentFloat`] is positive, `Less` if it is negative, and
    /// `Equal` if it is zero. $\infty$ is positive and $-\infty$ is negative; a
    /// [`BigExponentFloat`] has no negative zero.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is NaN.
    ///
    /// This is FLINT's `arf_sgn`, except that FLINT returns 0 for NaN.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::Sign;
    /// use malachite_base::num::basic::traits::{
    ///     Infinity, NegativeInfinity, NegativeOne, One, Zero,
    /// };
    /// use malachite_float::BigExponentFloat;
    /// use std::cmp::Ordering::*;
    ///
    /// assert_eq!(BigExponentFloat::INFINITY.sign(), Greater);
    /// assert_eq!(BigExponentFloat::NEGATIVE_INFINITY.sign(), Less);
    /// assert_eq!(BigExponentFloat::ZERO.sign(), Equal);
    /// assert_eq!(BigExponentFloat::ONE.sign(), Greater);
    /// assert_eq!(BigExponentFloat::NEGATIVE_ONE.sign(), Less);
    /// ```
    fn sign(&self) -> Ordering {
        match self {
            Self(NaN) => panic!("Cannot take the sign of NaN"),
            Self(Zero) => Equal,
            Self(Infinity { sign } | Finite { sign, .. }) => {
                if *sign {
                    Greater
                } else {
                    Less
                }
            }
        }
    }
}
