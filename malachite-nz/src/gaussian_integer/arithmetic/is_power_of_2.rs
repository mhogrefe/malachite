// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use malachite_base::num::arithmetic::traits::IsPowerOf2;

impl IsPowerOf2 for GaussianInteger {
    /// Determines whether a [`GaussianInteger`] is an integer power of 2.
    ///
    /// Only purely real, positive values qualify; in particular, $i$ and its multiples are not
    /// powers of 2.
    ///
    /// $f(x) = (\exists n \in \N : 2^n = x)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.real.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::IsPowerOf2;
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianInteger::from(0x80).is_power_of_2(), true);
    /// assert_eq!(GaussianInteger::from(-0x80).is_power_of_2(), false);
    /// assert_eq!(GaussianInteger::from(0x81).is_power_of_2(), false);
    /// assert_eq!(
    ///     GaussianInteger::from_str("128i").unwrap().is_power_of_2(),
    ///     false
    /// );
    /// assert_eq!(
    ///     GaussianInteger::from_str("128+i").unwrap().is_power_of_2(),
    ///     false
    /// );
    /// ```
    #[inline]
    fn is_power_of_2(&self) -> bool {
        self.imaginary == 0u32 && self.real.is_power_of_2()
    }
}
