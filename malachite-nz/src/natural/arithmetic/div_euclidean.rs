// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    DivAssignEuclidean, DivAssignMod, DivEuclidean, DivMod,
};

impl DivEuclidean<Self> for Natural {
    type DivOutput = Self;
    type ModOutput = Self;

    /// Divides a [`Natural`] by another [`Natural`], taking both by value and returning the
    /// quotient and remainder. The remainder is nonnegative.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$. For [`Natural`]s,
    /// Euclidean division coincides with [`div_mod`](DivMod::div_mod).
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
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
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     Natural::from(23u32)
    ///         .div_euclidean(Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    /// ```
    #[inline]
    fn div_euclidean(self, other: Self) -> (Self, Self) {
        self.div_mod(other)
    }
}

impl DivEuclidean<&Self> for Natural {
    type DivOutput = Self;
    type ModOutput = Self;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference and returning the quotient and remainder. The remainder is nonnegative.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$. For [`Natural`]s,
    /// Euclidean division coincides with [`div_mod`](DivMod::div_mod).
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
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
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 3 * 123 + 87 = 456
    /// assert_eq!(
    ///     Natural::from(456u32)
    ///         .div_euclidean(&Natural::from(123u32))
    ///         .to_debug_string(),
    ///     "(3, 87)"
    /// );
    /// ```
    #[inline]
    fn div_euclidean(self, other: &Self) -> (Self, Self) {
        self.div_mod(other)
    }
}

impl DivEuclidean<Natural> for &Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value and returning the quotient and remainder. The remainder is nonnegative.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$. For [`Natural`]s,
    /// Euclidean division coincides with [`div_mod`](DivMod::div_mod).
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
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
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 0 * 456 + 123 = 123
    /// assert_eq!(
    ///     (&Natural::from(123u32))
    ///         .div_euclidean(Natural::from(456u32))
    ///         .to_debug_string(),
    ///     "(0, 123)"
    /// );
    /// ```
    #[inline]
    fn div_euclidean(self, other: Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl DivEuclidean<&Natural> for &Natural {
    type DivOutput = Natural;
    type ModOutput = Natural;

    /// Divides a [`Natural`] by another [`Natural`], taking both by reference and returning the
    /// quotient and remainder. The remainder is nonnegative.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$. For [`Natural`]s,
    /// Euclidean division coincides with [`div_mod`](DivMod::div_mod).
    ///
    /// $$
    /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
    /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
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
    /// use malachite_base::strings::ToDebugString;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// assert_eq!(
    ///     (&Natural::from(23u32))
    ///         .div_euclidean(&Natural::from(10u32))
    ///         .to_debug_string(),
    ///     "(2, 3)"
    /// );
    /// ```
    #[inline]
    fn div_euclidean(self, other: &Natural) -> (Natural, Natural) {
        self.div_mod(other)
    }
}

impl DivAssignEuclidean<Self> for Natural {
    type ModOutput = Self;

    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by value and returning the remainder. The remainder is nonnegative.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$. For [`Natural`]s,
    /// Euclidean division coincides with [`div_mod`](DivMod::div_mod).
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::DivAssignEuclidean;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 10 + 3 = 23
    /// let mut x = Natural::from(23u32);
    /// assert_eq!(x.div_assign_euclidean(Natural::from(10u32)), 3);
    /// assert_eq!(x, 2);
    /// ```
    #[inline]
    fn div_assign_euclidean(&mut self, other: Self) -> Self {
        self.div_assign_mod(other)
    }
}

impl DivAssignEuclidean<&Self> for Natural {
    type ModOutput = Self;

    /// Divides a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by reference and returning the remainder. The remainder is nonnegative.
    ///
    /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$. For [`Natural`]s,
    /// Euclidean division coincides with [`div_mod`](DivMod::div_mod).
    ///
    /// $$
    /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::DivAssignEuclidean;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 3 * 123 + 87 = 456
    /// let mut x = Natural::from(456u32);
    /// assert_eq!(x.div_assign_euclidean(&Natural::from(123u32)), 87);
    /// assert_eq!(x, 3);
    /// ```
    #[inline]
    fn div_assign_euclidean(&mut self, other: &Self) -> Self {
        self.div_assign_mod(other)
    }
}
