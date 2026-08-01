// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use core::cmp::Ordering::Less;
use malachite_base::num::arithmetic::traits::{BalancedMod, BalancedModAssign, ModEuclidean};
use malachite_base::num::comparison::traits::OrdDouble;

// The Euclidean remainder already lies in [0, |m|), so the balanced one is that remainder when it
// is at most half the modulus, and that remainder less the modulus otherwise. A remainder of
// exactly half the modulus stays positive, which puts the endpoint at the top of the range.
fn balanced_mod_helper(x: &Integer, m: &Integer) -> Integer {
    let r: Natural = x.mod_euclidean(m);
    let abs_m = m.unsigned_abs_ref();
    // `r <= abs_m >> 1` is exactly `2r <= abs_m`: for an integer `r`, `r <= floor(x)` iff `r <= x`.
    // Phrasing it as the latter lets `cmp_double` answer it without building either value.
    if abs_m.cmp_double(&r) != Less {
        Integer::from(r)
    } else {
        Integer::from(r) - Integer::from(abs_m)
    }
}

impl BalancedMod<Self> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], returning the balanced remainder: the
    /// representative of `self` modulo `other` that is closest to zero, taking both [`Integer`]s by
    /// value.
    ///
    /// The remainder $r$ satisfies $-|y|/2 < r \leq |y|/2$ and $r \equiv x \bmod y$, which
    /// determine it uniquely. A remainder of exactly $|y|/2$ is positive. Only the magnitude of
    /// `other` matters, so negating it leaves the result unchanged.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(23).balanced_mod(Integer::from(10)), 3);
    /// // 7 is more than half of 10, so the representative closest to zero is negative
    /// assert_eq!(Integer::from(27).balanced_mod(Integer::from(10)), -3);
    /// // only the magnitude of the modulus matters
    /// assert_eq!(Integer::from(27).balanced_mod(Integer::from(-10)), -3);
    /// ```
    #[inline]
    fn balanced_mod(self, other: Self) -> Integer {
        balanced_mod_helper(&self, &other)
    }
}

impl BalancedMod<&Self> for Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], returning the balanced remainder: the
    /// representative of `self` modulo `other` that is closest to zero, taking the first
    /// [`Integer`] by value and the second by reference.
    ///
    /// The remainder $r$ satisfies $-|y|/2 < r \leq |y|/2$ and $r \equiv x \bmod y$, which
    /// determine it uniquely. A remainder of exactly $|y|/2$ is positive. Only the magnitude of
    /// `other` matters, so negating it leaves the result unchanged.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(23).balanced_mod(&Integer::from(10)), 3);
    /// // 7 is more than half of 10, so the representative closest to zero is negative
    /// assert_eq!(Integer::from(27).balanced_mod(&Integer::from(10)), -3);
    /// // only the magnitude of the modulus matters
    /// assert_eq!(Integer::from(27).balanced_mod(&Integer::from(-10)), -3);
    /// ```
    #[inline]
    fn balanced_mod(self, other: &Self) -> Integer {
        balanced_mod_helper(&self, other)
    }
}

impl BalancedMod<Integer> for &Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], returning the balanced remainder: the
    /// representative of `self` modulo `other` that is closest to zero, taking the first
    /// [`Integer`] by reference and the second by value.
    ///
    /// The remainder $r$ satisfies $-|y|/2 < r \leq |y|/2$ and $r \equiv x \bmod y$, which
    /// determine it uniquely. A remainder of exactly $|y|/2$ is positive. Only the magnitude of
    /// `other` matters, so negating it leaves the result unchanged.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(23)).balanced_mod(Integer::from(10)), 3);
    /// // 7 is more than half of 10, so the representative closest to zero is negative
    /// assert_eq!((&Integer::from(27)).balanced_mod(Integer::from(10)), -3);
    /// // only the magnitude of the modulus matters
    /// assert_eq!((&Integer::from(27)).balanced_mod(Integer::from(-10)), -3);
    /// ```
    #[inline]
    fn balanced_mod(self, other: Integer) -> Integer {
        balanced_mod_helper(self, &other)
    }
}

impl BalancedMod<&Integer> for &Integer {
    type Output = Integer;

    /// Divides an [`Integer`] by another [`Integer`], returning the balanced remainder: the
    /// representative of `self` modulo `other` that is closest to zero, taking both [`Integer`]s by
    /// reference.
    ///
    /// The remainder $r$ satisfies $-|y|/2 < r \leq |y|/2$ and $r \equiv x \bmod y$, which
    /// determine it uniquely. A remainder of exactly $|y|/2$ is positive. Only the magnitude of
    /// `other` matters, so negating it leaves the result unchanged.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(23)).balanced_mod(&Integer::from(10)), 3);
    /// // 7 is more than half of 10, so the representative closest to zero is negative
    /// assert_eq!((&Integer::from(27)).balanced_mod(&Integer::from(10)), -3);
    /// // only the magnitude of the modulus matters
    /// assert_eq!((&Integer::from(27)).balanced_mod(&Integer::from(-10)), -3);
    /// ```
    #[inline]
    fn balanced_mod(self, other: &Integer) -> Integer {
        balanced_mod_helper(self, other)
    }
}

impl BalancedModAssign<Self> for Integer {
    /// Divides an [`Integer`] by another [`Integer`], replacing the first [`Integer`] by the
    /// balanced remainder: the representative of `self` modulo `other` that is closest to zero. The
    /// [`Integer`] on the right-hand side is taken by value.
    ///
    /// The remainder $r$ satisfies $-|y|/2 < r \leq |y|/2$; a remainder of exactly $|y|/2$ is
    /// positive.
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
    /// use malachite_base::num::arithmetic::traits::BalancedModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(27);
    /// x.balanced_mod_assign(Integer::from(10));
    /// assert_eq!(x, -3);
    /// ```
    #[inline]
    fn balanced_mod_assign(&mut self, other: Self) {
        *self = balanced_mod_helper(self, &other);
    }
}

impl BalancedModAssign<&Self> for Integer {
    /// Divides an [`Integer`] by another [`Integer`], replacing the first [`Integer`] by the
    /// balanced remainder: the representative of `self` modulo `other` that is closest to zero. The
    /// [`Integer`] on the right-hand side is taken by reference.
    ///
    /// The remainder $r$ satisfies $-|y|/2 < r \leq |y|/2$; a remainder of exactly $|y|/2$ is
    /// positive.
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
    /// use malachite_base::num::arithmetic::traits::BalancedModAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(27);
    /// x.balanced_mod_assign(&Integer::from(10));
    /// assert_eq!(x, -3);
    /// ```
    #[inline]
    fn balanced_mod_assign(&mut self, other: &Self) {
        *self = balanced_mod_helper(self, other);
    }
}
