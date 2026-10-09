// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    BalancedModPowerOf2, IsPowerOf2, ModPowerOf2, NegModPowerOf2,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::BitAccess;

// The balanced remainder modulo $2^k$ of a number whose ordinary remainder is `r`. It is `r` when
// `r` is at most $2^{k-1}$, and $r - 2^k$ otherwise; `r` exceeds $2^{k-1}$ exactly when bit $k - 1$
// is set and `r` is not $2^{k-1}$ itself.
pub(crate) fn balanced_from_mod_power_of_2(r: Natural, pow: u64) -> Integer {
    if pow == 0 {
        Integer::ZERO
    } else if r.get_bit(pow - 1) && !r.is_power_of_2() {
        -Integer::from(r.neg_mod_power_of_2(pow))
    } else {
        Integer::from(r)
    }
}

impl BalancedModPowerOf2 for Natural {
    type Output = Integer;

    /// Divides a [`Natural`] by $2^k$, returning the balanced remainder: the representative of
    /// `self` modulo $2^k$ that is closest to zero. The [`Natural`] is taken by value.
    ///
    /// The remainder $r$ satisfies $-2^{k-1} < r \leq 2^{k-1}$ and $r \equiv x \bmod 2^k$, which
    /// determine it uniquely; for $k = 0$ it is 0. A remainder of exactly $2^{k-1}$ is positive, so
    /// the result may be negative and is returned as an [`Integer`]. This is
    /// [`balanced_mod`](malachite_base::num::arithmetic::traits::BalancedMod::balanced_mod) with
    /// modulus $2^k$.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(19u32).balanced_mod_power_of_2(3), 3);
    /// // 7 is more than half of 8, so the representative closest to zero is negative
    /// assert_eq!(Natural::from(23u32).balanced_mod_power_of_2(3), -1);
    /// // exactly half the modulus stays positive
    /// assert_eq!(Natural::from(20u32).balanced_mod_power_of_2(3), 4);
    /// ```
    #[inline]
    fn balanced_mod_power_of_2(self, pow: u64) -> Integer {
        balanced_from_mod_power_of_2(self.mod_power_of_2(pow), pow)
    }
}

impl BalancedModPowerOf2 for &Natural {
    type Output = Integer;

    /// Divides a [`Natural`] by $2^k$, returning the balanced remainder: the representative of
    /// `self` modulo $2^k$ that is closest to zero. The [`Natural`] is taken by reference.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(19u32)).balanced_mod_power_of_2(3), 3);
    /// assert_eq!((&Natural::from(23u32)).balanced_mod_power_of_2(3), -1);
    /// assert_eq!((&Natural::from(20u32)).balanced_mod_power_of_2(3), 4);
    /// ```
    #[inline]
    fn balanced_mod_power_of_2(self, pow: u64) -> Integer {
        balanced_from_mod_power_of_2(self.mod_power_of_2(pow), pow)
    }
}
