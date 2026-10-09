// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::arithmetic::balanced_mod_power_of_2::balanced_from_mod_power_of_2;
use core::mem::take;
use malachite_base::num::arithmetic::traits::{
    BalancedModPowerOf2, BalancedModPowerOf2Assign, ModPowerOf2,
};

impl BalancedModPowerOf2 for Integer {
    type Output = Self;

    /// Divides an [`Integer`] by $2^k$, returning the balanced remainder: the representative of
    /// `self` modulo $2^k$ that is closest to zero. The [`Integer`] is taken by value.
    ///
    /// The remainder $r$ satisfies $-2^{k-1} < r \leq 2^{k-1}$ and $r \equiv x \bmod 2^k$, which
    /// determine it uniquely; for $k = 0$ it is 0. A remainder of exactly $2^{k-1}$ is positive.
    /// This is [`balanced_mod`](malachite_base::num::arithmetic::traits::BalancedMod::balanced_mod)
    /// with modulus $2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedModPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(23).balanced_mod_power_of_2(3), -1);
    /// // a negative value is reduced into the same range
    /// assert_eq!(Integer::from(-19).balanced_mod_power_of_2(3), -3);
    /// // exactly half the modulus stays positive
    /// assert_eq!(Integer::from(-20).balanced_mod_power_of_2(3), 4);
    /// ```
    #[inline]
    fn balanced_mod_power_of_2(self, pow: u64) -> Self {
        balanced_from_mod_power_of_2(self.mod_power_of_2(pow), pow)
    }
}

impl BalancedModPowerOf2 for &Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by $2^k$, returning the balanced remainder: the representative of
    /// `self` modulo $2^k$ that is closest to zero. The [`Integer`] is taken by reference.
    ///
    /// See the [`BalancedModPowerOf2`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedModPowerOf2;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(23)).balanced_mod_power_of_2(3), -1);
    /// assert_eq!((&Integer::from(-19)).balanced_mod_power_of_2(3), -3);
    /// assert_eq!((&Integer::from(-20)).balanced_mod_power_of_2(3), 4);
    /// ```
    #[inline]
    fn balanced_mod_power_of_2(self, pow: u64) -> Integer {
        balanced_from_mod_power_of_2(self.mod_power_of_2(pow), pow)
    }
}

impl BalancedModPowerOf2Assign for Integer {
    /// Divides an [`Integer`] by $2^k$, replacing it by the balanced remainder: the representative
    /// of `self` modulo $2^k$ that is closest to zero.
    ///
    /// See the [`BalancedModPowerOf2`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedModPowerOf2Assign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(23);
    /// x.balanced_mod_power_of_2_assign(3);
    /// assert_eq!(x, -1);
    /// ```
    #[inline]
    fn balanced_mod_power_of_2_assign(&mut self, pow: u64) {
        *self = take(self).balanced_mod_power_of_2(pow);
    }
}
