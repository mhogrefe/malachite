// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{Mod, ModAssign, ModEuclidean, ModEuclideanAssign};

impl ModEuclidean<Self> for Natural {
    type Output = Self;

    /// Divides a [`Natural`] by another [`Natural`], taking both by value and returning just the
    /// remainder.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < y$. For [`Natural`]s, the Euclidean remainder coincides with
    /// [`mod_op`](Mod::mod_op).
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::ModEuclidean;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Natural::from(23u32).mod_euclidean(Natural::from(10u32)), 3);
    /// ```
    #[inline]
    fn mod_euclidean(self, other: Self) -> Self {
        self.mod_op(other)
    }
}

impl ModEuclidean<&Self> for Natural {
    type Output = Self;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference and returning just the remainder.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < y$. For [`Natural`]s, the Euclidean remainder coincides with
    /// [`mod_op`](Mod::mod_op).
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::ModEuclidean;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Natural::from(23u32).mod_euclidean(&Natural::from(10u32)), 3);
    /// ```
    #[inline]
    fn mod_euclidean(self, other: &Self) -> Self {
        self.mod_op(other)
    }
}

impl ModEuclidean<Natural> for &Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value and returning just the remainder.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < y$. For [`Natural`]s, the Euclidean remainder coincides with
    /// [`mod_op`](Mod::mod_op).
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::ModEuclidean;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     (&Natural::from(23u32)).mod_euclidean(Natural::from(10u32)),
    ///     3
    /// );
    /// ```
    #[inline]
    fn mod_euclidean(self, other: Natural) -> Natural {
        self.mod_op(other)
    }
}

impl ModEuclidean<&Natural> for &Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by reference and returning just
    /// the remainder.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < y$. For [`Natural`]s, the Euclidean remainder coincides with
    /// [`mod_op`](Mod::mod_op).
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::ModEuclidean;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     (&Natural::from(23u32)).mod_euclidean(&Natural::from(10u32)),
    ///     3
    /// );
    /// ```
    #[inline]
    fn mod_euclidean(self, other: &Natural) -> Natural {
        self.mod_op(other)
    }
}

impl ModEuclideanAssign<Self> for Natural {
    /// Divides a [`Natural`] by another [`Natural`], taking the [`Natural`] on the right-hand side
    /// by value and replacing the first [`Natural`] by the remainder.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < y$. For [`Natural`]s, the Euclidean remainder coincides with
    /// [`mod_op`](Mod::mod_op).
    ///
    /// $$
    /// x \gets x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::ModEuclideanAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// x.mod_euclidean_assign(Natural::from(10u32));
    /// assert_eq!(x, 3);
    /// ```
    #[inline]
    fn mod_euclidean_assign(&mut self, other: Self) {
        self.mod_assign(other);
    }
}

impl ModEuclideanAssign<&Self> for Natural {
    /// Divides a [`Natural`] by another [`Natural`], taking the [`Natural`] on the right-hand side
    /// by reference and replacing the first [`Natural`] by the remainder.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < y$. For [`Natural`]s, the Euclidean remainder coincides with
    /// [`mod_op`](Mod::mod_op).
    ///
    /// $$
    /// x \gets x - y\left \lfloor \frac{x}{y} \right \rfloor.
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::ModEuclideanAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// x.mod_euclidean_assign(&Natural::from(10u32));
    /// assert_eq!(x, 3);
    /// ```
    #[inline]
    fn mod_euclidean_assign(&mut self, other: &Self) {
        self.mod_assign(other);
    }
}
