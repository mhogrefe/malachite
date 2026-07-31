// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{BalancedMod, Mod};

// The balanced remainder is the ordinary one when it is at most half the modulus, and that
// remainder less the modulus otherwise. A remainder of exactly half the modulus stays positive,
// which is what puts the endpoint at the top of the range rather than the bottom.
fn balanced_mod_helper(x: &Natural, m: &Natural) -> Integer {
    let r = x.mod_op(m);
    if r <= m >> 1u32 {
        Integer::from(r)
    } else {
        Integer::from(r) - Integer::from(m)
    }
}

impl BalancedMod<Self> for Natural {
    type Output = Integer;

    /// Divides a [`Natural`] by another [`Natural`], returning the balanced remainder: the
    /// representative of `self` modulo `other` that is closest to zero. Both [`Natural`]s are taken
    /// by value.
    ///
    /// The remainder $r$ satisfies $-y/2 < r \leq y/2$ and $r \equiv x \bmod y$, which determine it
    /// uniquely. A remainder of exactly $y/2$ is positive, so the result may be negative and is
    /// returned as an [`Integer`].
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(23u32).balanced_mod(Natural::from(10u32)), 3);
    /// // 7 is more than half of 10, so the representative closest to zero is negative
    /// assert_eq!(Natural::from(27u32).balanced_mod(Natural::from(10u32)), -3);
    /// // exactly half the modulus stays positive
    /// assert_eq!(Natural::from(25u32).balanced_mod(Natural::from(10u32)), 5);
    /// ```
    #[inline]
    fn balanced_mod(self, other: Self) -> Integer {
        balanced_mod_helper(&self, &other)
    }
}

impl BalancedMod<&Self> for Natural {
    type Output = Integer;

    /// Divides a [`Natural`] by another [`Natural`], returning the balanced remainder: the
    /// representative of `self` modulo `other` that is closest to zero. The first [`Natural`] is
    /// taken by value and the second by reference.
    ///
    /// See the [`BalancedMod`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(27u32).balanced_mod(&Natural::from(10u32)), -3);
    /// ```
    #[inline]
    fn balanced_mod(self, other: &Self) -> Integer {
        balanced_mod_helper(&self, other)
    }
}

impl BalancedMod<Natural> for &Natural {
    type Output = Integer;

    /// Divides a [`Natural`] by another [`Natural`], returning the balanced remainder: the
    /// representative of `self` modulo `other` that is closest to zero. The first [`Natural`] is
    /// taken by reference and the second by value.
    ///
    /// See the [`BalancedMod`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(27u32)).balanced_mod(Natural::from(10u32)),
    ///     -3
    /// );
    /// ```
    #[inline]
    fn balanced_mod(self, other: Natural) -> Integer {
        balanced_mod_helper(self, &other)
    }
}

impl BalancedMod<&Natural> for &Natural {
    type Output = Integer;

    /// Divides a [`Natural`] by another [`Natural`], returning the balanced remainder: the
    /// representative of `self` modulo `other` that is closest to zero. Both [`Natural`]s are taken
    /// by reference.
    ///
    /// See the [`BalancedMod`] documentation for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is zero.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::BalancedMod;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(27u32)).balanced_mod(&Natural::from(10u32)),
    ///     -3
    /// );
    /// ```
    #[inline]
    fn balanced_mod(self, other: &Natural) -> Integer {
        balanced_mod_helper(self, other)
    }
}
