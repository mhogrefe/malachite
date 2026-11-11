// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::big_exponent_float::BigExponentFloat;
use core::cmp::Ordering::*;
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_nz::natural::Natural;

impl EqAbs<Natural> for BigExponentFloat {
    /// Determines whether the absolute value of a [`BigExponentFloat`] is equal to a [`Natural`].
    ///
    /// $\infty$, $-\infty$, and NaN are not equal to any [`Natural`]. The [`BigExponentFloat`] zero
    /// is equal to the [`Natural`] zero.
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
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(BigExponentFloat::from(123).eq_abs(&Natural::from(123u32)));
    /// assert!(BigExponentFloat::from(-123).eq_abs(&Natural::from(123u32)));
    /// assert!(BigExponentFloat::ONE_HALF.ne_abs(&Natural::ONE));
    /// ```
    #[inline]
    fn eq_abs(&self, other: &Natural) -> bool {
        self.partial_cmp_abs(other) == Some(Equal)
    }
}

impl EqAbs<BigExponentFloat> for Natural {
    /// Determines whether a [`Natural`] is equal to the absolute value of a [`BigExponentFloat`].
    ///
    /// No [`Natural`] is equal to $\infty$, $-\infty$, or NaN. The [`Natural`] zero is equal to the
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
    /// use malachite_base::num::comparison::traits::EqAbs;
    /// use malachite_float::BigExponentFloat;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert!(Natural::from(123u32).eq_abs(&BigExponentFloat::from(123)));
    /// assert!(Natural::from(123u32).eq_abs(&BigExponentFloat::from(-123)));
    /// assert!(Natural::ONE.ne_abs(&BigExponentFloat::ONE_HALF));
    /// ```
    #[inline]
    fn eq_abs(&self, other: &BigExponentFloat) -> bool {
        other.eq_abs(self)
    }
}
