// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::ModIsReduced;
use malachite_base::num::basic::traits::Zero;

impl ModIsReduced for Natural {
    /// Returns whether a [`Natural`] is reduced modulo another [`Natural`] $m$; in other words,
    /// whether it is less than $m$.
    ///
    /// $m$ cannot be zero.
    ///
    /// $f(x, m) = (x < m)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `m` is 0.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{ModIsReduced, Pow};
    /// use malachite_base::num::basic::traits::{One, Zero};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.mod_is_reduced(&Natural::from(5u32)), true);
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .mod_is_reduced(&Natural::from(10u32).pow(12)),
    ///     false
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .mod_is_reduced(&(Natural::from(10u32).pow(12) + Natural::ONE)),
    ///     true
    /// );
    /// ```
    #[inline]
    fn mod_is_reduced(&self, m: &Natural) -> bool {
        assert_ne!(*m, Natural::ZERO);
        self < m
    }
}
