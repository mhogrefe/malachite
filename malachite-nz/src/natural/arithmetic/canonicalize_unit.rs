// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{CanonicalizeUnit, CanonicalizeUnitAssign};

impl CanonicalizeUnit for Natural {
    type Output = Self;

    /// Brings a [`Natural`] into canonical unit form, taking it by value. A [`Natural`] is already
    /// in canonical unit form, so this is the identity.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnit;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32).canonicalize_unit(), 123);
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> Self {
        self
    }
}

impl CanonicalizeUnit for &Natural {
    type Output = Natural;

    /// Brings a [`Natural`] into canonical unit form, taking it by reference. A [`Natural`] is
    /// already in canonical unit form, so this is the identity.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(123u32)).canonicalize_unit(), 123);
    /// ```
    #[inline]
    fn canonicalize_unit(self) -> Natural {
        self.clone()
    }
}

impl CanonicalizeUnitAssign for Natural {
    /// Replaces a [`Natural`] with its canonical unit form. A [`Natural`] is already in canonical
    /// unit form, so this is the identity.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::CanonicalizeUnitAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(123u32);
    /// x.canonicalize_unit_assign();
    /// assert_eq!(x, 123);
    /// ```
    #[inline]
    fn canonicalize_unit_assign(&mut self) {}
}
