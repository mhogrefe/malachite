// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering::*;
use malachite_nz::integer::Integer;

impl PartialEq<Integer> for BigExponentFloat {
    /// Determines whether a [`BigExponentFloat`] is equal to an [`Integer`].
    ///
    /// $\infty$, $-\infty$, and NaN are not equal to any [`Integer`]. The [`BigExponentFloat`] zero
    /// is equal to the [`Integer`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.complexity(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(BigExponentFloat::from(123) == Integer::from(123));
    /// assert!(BigExponentFloat::from(-123) == Integer::from(-123));
    /// assert!(BigExponentFloat::ONE_HALF != Integer::ONE);
    /// ```
    #[inline]
    fn eq(&self, other: &Integer) -> bool {
        self.partial_cmp(other) == Some(Equal)
    }
}

impl PartialEq<BigExponentFloat> for Integer {
    /// Determines whether an [`Integer`] is equal to a [`BigExponentFloat`].
    ///
    /// No [`Integer`] is equal to $\infty$, $-\infty$, or NaN. The [`Integer`] zero is equal to the
    /// [`BigExponentFloat`] zero.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.complexity())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::{One, OneHalf};
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert!(Integer::from(123) == BigExponentFloat::from(123));
    /// assert!(Integer::from(-123) == BigExponentFloat::from(-123));
    /// assert!(Integer::ONE != BigExponentFloat::ONE_HALF);
    /// ```
    #[inline]
    fn eq(&self, other: &BigExponentFloat) -> bool {
        other == self
    }
}
