// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{
    Abs, AbsAssign, CanonicalizeUnit, CanonicalizeUnitAssign,
};

impl CanonicalizeUnit for Integer {
    type Output = Self;

    /// Brings a [`Integer`] into canonical unit form, taking it by value. The canonical unit form
    /// of an [`Integer`] is its absolute value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(-123).canonicalize_unit(), 123);
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> Self {
        self.abs()
    }
}

impl CanonicalizeUnit for &Integer {
    type Output = Integer;

    /// Brings a [`Integer`] into canonical unit form, taking it by reference. The canonical unit
    /// form of an [`Integer`] is its absolute value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(-123)).canonicalize_unit(), 123);
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> Integer {
        self.abs()
    }
}

impl CanonicalizeUnitAssign for Integer {
    /// Replaces a [`Integer`] with its canonical unit form. The canonical unit form of an
    /// [`Integer`] is its absolute value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-123);
    /// x.canonicalize_unit_assign();
    /// assert_eq!(x, 123);
    /// ```
    #[inline]
    fn canonicalize_unit_assign(&mut self) {
        self.abs_assign();
    }
}
