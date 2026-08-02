// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{CheckedMulSubMul, CheckedSubMul};

impl CheckedMulSubMul<Self, Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning
    /// `None` if the result would be negative, taking all four by value.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     \operatorname{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).checked_mul_sub_mul(
    ///         Natural::from(3u32),
    ///         Natural::from(4u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     Some(Natural::from(10u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).checked_mul_sub_mul(
    ///         Natural::from(1u32),
    ///         Natural::from(2u32),
    ///         Natural::from(2u32)
    ///     ),
    ///     None
    /// );
    /// ```
    #[inline]
    fn checked_mul_sub_mul(self, y: Self, z: Self, w: Self) -> Option<Self> {
        (self * y).checked_sub_mul(z, w)
    }
}

impl CheckedMulSubMul<Self, Self, &Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning
    /// `None` if the result would be negative, taking $x$, $y$ and $z$ by value and $w$ by
    /// reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     \operatorname{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).checked_mul_sub_mul(
    ///         Natural::from(3u32),
    ///         Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     Some(Natural::from(10u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).checked_mul_sub_mul(
    ///         Natural::from(1u32),
    ///         Natural::from(2u32),
    ///         &Natural::from(2u32)
    ///     ),
    ///     None
    /// );
    /// ```
    #[inline]
    fn checked_mul_sub_mul(self, y: Self, z: Self, w: &Self) -> Option<Self> {
        (self * y).checked_sub_mul(z, w)
    }
}

impl CheckedMulSubMul<Self, &Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning
    /// `None` if the result would be negative, taking $x$, $y$ and $w$ by value and $z$ by
    /// reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     \operatorname{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).checked_mul_sub_mul(
    ///         Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     Some(Natural::from(10u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).checked_mul_sub_mul(
    ///         Natural::from(1u32),
    ///         &Natural::from(2u32),
    ///         Natural::from(2u32)
    ///     ),
    ///     None
    /// );
    /// ```
    #[inline]
    fn checked_mul_sub_mul(self, y: Self, z: &Self, w: Self) -> Option<Self> {
        (self * y).checked_sub_mul(z, w)
    }
}

impl CheckedMulSubMul<Self, &Self, &Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning
    /// `None` if the result would be negative, taking $x$ and $y$ by value and $z$ and $w$ by
    /// reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     \operatorname{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).checked_mul_sub_mul(
    ///         Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     Some(Natural::from(10u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).checked_mul_sub_mul(
    ///         Natural::from(1u32),
    ///         &Natural::from(2u32),
    ///         &Natural::from(2u32)
    ///     ),
    ///     None
    /// );
    /// ```
    #[inline]
    fn checked_mul_sub_mul(self, y: Self, z: &Self, w: &Self) -> Option<Self> {
        (self * y).checked_sub_mul(z, w)
    }
}

impl CheckedMulSubMul<&Self, Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning
    /// `None` if the result would be negative, taking $x$, $z$ and $w$ by value and $y$ by
    /// reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     \operatorname{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).checked_mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         Natural::from(4u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     Some(Natural::from(10u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).checked_mul_sub_mul(
    ///         &Natural::from(1u32),
    ///         Natural::from(2u32),
    ///         Natural::from(2u32)
    ///     ),
    ///     None
    /// );
    /// ```
    #[inline]
    fn checked_mul_sub_mul(self, y: &Self, z: Self, w: Self) -> Option<Self> {
        (self * y).checked_sub_mul(z, w)
    }
}

impl CheckedMulSubMul<&Self, Self, &Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning
    /// `None` if the result would be negative, taking $x$ and $z$ by value and $y$ and $w$ by
    /// reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     \operatorname{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).checked_mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     Some(Natural::from(10u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).checked_mul_sub_mul(
    ///         &Natural::from(1u32),
    ///         Natural::from(2u32),
    ///         &Natural::from(2u32)
    ///     ),
    ///     None
    /// );
    /// ```
    #[inline]
    fn checked_mul_sub_mul(self, y: &Self, z: Self, w: &Self) -> Option<Self> {
        (self * y).checked_sub_mul(z, w)
    }
}

impl CheckedMulSubMul<&Self, &Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning
    /// `None` if the result would be negative, taking $x$ and $w$ by value and $y$ and $z$ by
    /// reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     \operatorname{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).checked_mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     Some(Natural::from(10u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).checked_mul_sub_mul(
    ///         &Natural::from(1u32),
    ///         &Natural::from(2u32),
    ///         Natural::from(2u32)
    ///     ),
    ///     None
    /// );
    /// ```
    #[inline]
    fn checked_mul_sub_mul(self, y: &Self, z: &Self, w: Self) -> Option<Self> {
        (self * y).checked_sub_mul(z, w)
    }
}

impl CheckedMulSubMul<&Self, &Self, &Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning
    /// `None` if the result would be negative, taking $x$ by value and $y$, $z$ and $w$ by
    /// reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     \operatorname{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).checked_mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     Some(Natural::from(10u32))
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).checked_mul_sub_mul(
    ///         &Natural::from(1u32),
    ///         &Natural::from(2u32),
    ///         &Natural::from(2u32)
    ///     ),
    ///     None
    /// );
    /// ```
    #[inline]
    fn checked_mul_sub_mul(self, y: &Self, z: &Self, w: &Self) -> Option<Self> {
        (self * y).checked_sub_mul(z, w)
    }
}

impl CheckedMulSubMul<&Natural, &Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning
    /// `None` if the result would be negative, taking all four by reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     \operatorname{None} & \text{otherwise}
    /// \end{cases}
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), w.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{CheckedMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).checked_mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     Some(Natural::from(10u32))
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1u32)).checked_mul_sub_mul(
    ///         &Natural::from(1u32),
    ///         &Natural::from(2u32),
    ///         &Natural::from(2u32)
    ///     ),
    ///     None
    /// );
    /// ```
    #[inline]
    fn checked_mul_sub_mul(self, y: &Natural, z: &Natural, w: &Natural) -> Option<Natural> {
        (self * y).checked_sub_mul(z, w)
    }
}
