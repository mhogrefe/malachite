// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{DivEuclidean, DivEuclideanAssign};

impl DivEuclidean<Self> for Natural {
    type Output = Self;

    /// Divides a [`Natural`] by another [`Natural`], taking both by value and returning just the
    /// quotient.
    ///
    /// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < y$. For [`Natural`]s, the Euclidean quotient coincides with division.
    ///
    /// $$
    /// f(x, y) = \left \lfloor \frac{x}{y} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::DivEuclidean;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Natural::from(23u32).div_euclidean(Natural::from(10u32)), 2);
    /// ```
    #[inline]
    fn div_euclidean(self, other: Self) -> Self {
        self / other
    }
}

impl DivEuclidean<&Self> for Natural {
    type Output = Self;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference and returning just the quotient.
    ///
    /// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < y$. For [`Natural`]s, the Euclidean quotient coincides with division.
    ///
    /// $$
    /// f(x, y) = \left \lfloor \frac{x}{y} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::DivEuclidean;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(Natural::from(23u32).div_euclidean(&Natural::from(10u32)), 2);
    /// ```
    #[inline]
    fn div_euclidean(self, other: &Self) -> Self {
        self / other
    }
}

impl DivEuclidean<Natural> for &Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value and returning just the quotient.
    ///
    /// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < y$. For [`Natural`]s, the Euclidean quotient coincides with division.
    ///
    /// $$
    /// f(x, y) = \left \lfloor \frac{x}{y} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::DivEuclidean;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Natural::from(23u32)).div_euclidean(Natural::from(10u32)), 2);
    /// ```
    #[inline]
    fn div_euclidean(self, other: Natural) -> Natural {
        self / other
    }
}

impl DivEuclidean<&Natural> for &Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by reference and returning just
    /// the quotient.
    ///
    /// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < y$. For [`Natural`]s, the Euclidean quotient coincides with division.
    ///
    /// $$
    /// f(x, y) = \left \lfloor \frac{x}{y} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::DivEuclidean;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!((&Natural::from(23u32)).div_euclidean(&Natural::from(10u32)), 2);
    /// ```
    #[inline]
    fn div_euclidean(self, other: &Natural) -> Natural {
        self / other
    }
}

impl DivEuclideanAssign<Self> for Natural {
    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by value and keeping just the quotient.
    ///
    /// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < y$. For [`Natural`]s, the Euclidean quotient coincides with division.
    ///
    /// $$
    /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::DivEuclideanAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// x.div_euclidean_assign(Natural::from(10u32));
    /// assert_eq!(x, 2);
    /// ```
    #[inline]
    fn div_euclidean_assign(&mut self, other: Self) {
        *self /= other;
    }
}

impl DivEuclideanAssign<&Self> for Natural {
    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by reference and keeping just the quotient.
    ///
    /// If the remainder were computed, the quotient and remainder would satisfy $x = qy + r$ and $0
    /// \leq r < y$. For [`Natural`]s, the Euclidean quotient coincides with division.
    ///
    /// $$
    /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
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
    /// use malachite_base::num::arithmetic::traits::DivEuclideanAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// x.div_euclidean_assign(&Natural::from(10u32));
    /// assert_eq!(x, 2);
    /// ```
    #[inline]
    fn div_euclidean_assign(&mut self, other: &Self) {
        *self /= other;
    }
}
