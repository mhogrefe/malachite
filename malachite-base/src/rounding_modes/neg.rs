// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::NegAssign;
use crate::rounding_modes::RoundingMode::{self, *};
use core::ops::Neg;

/// Returns the negative of a [`RoundingMode`].
///
/// The negative is defined so that if a [`RoundingMode`] $m$ is used to round the result of an odd
/// function $f$, then $f(x, -m) = -f(-x, m)$. `Floor` and `Ceiling` are swapped, and the other
/// modes are unchanged.
///
/// # Worst-case complexity
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::rounding_modes::RoundingMode::*;
///
/// assert_eq!(-Down, Down);
/// assert_eq!(-Up, Up);
/// assert_eq!(-Floor, Ceiling);
/// assert_eq!(-Ceiling, Floor);
/// assert_eq!(-Nearest, Nearest);
/// assert_eq!(-Exact, Exact);
/// ```
impl Neg for RoundingMode {
    type Output = RoundingMode;

    #[inline]
    fn neg(self) -> RoundingMode {
        match self {
            Floor => Ceiling,
            Ceiling => Floor,
            rm => rm,
        }
    }
}

impl NegAssign for RoundingMode {
    /// Replaces a [`RoundingMode`] with its negative.
    ///
    /// The negative is defined so that if a [`RoundingMode`] $m$ is used to round the result of an
    /// odd function $f$, then $f(x, -m) = -f(-x, m)$. `Floor` and `Ceiling` are swapped, and the
    /// other modes are unchanged.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::NegAssign;
    /// use malachite_base::rounding_modes::RoundingMode::*;
    ///
    /// let mut rm = Down;
    /// rm.neg_assign();
    /// assert_eq!(rm, Down);
    ///
    /// let mut rm = Floor;
    /// rm.neg_assign();
    /// assert_eq!(rm, Ceiling);
    /// ```
    #[inline]
    fn neg_assign(&mut self) {
        if *self == Floor {
            *self = Ceiling;
        } else if *self == Ceiling {
            *self = Floor;
        }
    }
}
