// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    SaturatingMulSubMul, SaturatingMulSubMulAssign, SaturatingSubMul, SaturatingSubMulAssign,
};

impl SaturatingMulSubMul<Self, Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning 0
    /// if the result would be negative, taking all four by value.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).saturating_mul_sub_mul(
    ///         Natural::from(3u32),
    ///         Natural::from(4u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).saturating_mul_sub_mul(
    ///         Natural::from(1u32),
    ///         Natural::from(2u32),
    ///         Natural::from(2u32)
    ///     ),
    ///     0
    /// );
    /// ```
    #[inline]
    fn saturating_mul_sub_mul(self, y: Self, z: Self, w: Self) -> Self {
        (self * y).saturating_sub_mul(z, w)
    }
}

impl SaturatingMulSubMul<Self, Self, &Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning 0
    /// if the result would be negative, taking $x$, $y$ and $z$ by value and $w$ by reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).saturating_mul_sub_mul(
    ///         Natural::from(3u32),
    ///         Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).saturating_mul_sub_mul(
    ///         Natural::from(1u32),
    ///         Natural::from(2u32),
    ///         &Natural::from(2u32)
    ///     ),
    ///     0
    /// );
    /// ```
    #[inline]
    fn saturating_mul_sub_mul(self, y: Self, z: Self, w: &Self) -> Self {
        (self * y).saturating_sub_mul(z, w)
    }
}

impl SaturatingMulSubMul<Self, &Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning 0
    /// if the result would be negative, taking $x$, $y$ and $w$ by value and $z$ by reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).saturating_mul_sub_mul(
    ///         Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).saturating_mul_sub_mul(
    ///         Natural::from(1u32),
    ///         &Natural::from(2u32),
    ///         Natural::from(2u32)
    ///     ),
    ///     0
    /// );
    /// ```
    #[inline]
    fn saturating_mul_sub_mul(self, y: Self, z: &Self, w: Self) -> Self {
        (self * y).saturating_sub_mul(z, w)
    }
}

impl SaturatingMulSubMul<Self, &Self, &Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning 0
    /// if the result would be negative, taking $x$ and $y$ by value and $z$ and $w$ by reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).saturating_mul_sub_mul(
    ///         Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).saturating_mul_sub_mul(
    ///         Natural::from(1u32),
    ///         &Natural::from(2u32),
    ///         &Natural::from(2u32)
    ///     ),
    ///     0
    /// );
    /// ```
    #[inline]
    fn saturating_mul_sub_mul(self, y: Self, z: &Self, w: &Self) -> Self {
        (self * y).saturating_sub_mul(z, w)
    }
}

impl SaturatingMulSubMul<&Self, Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning 0
    /// if the result would be negative, taking $x$, $z$ and $w$ by value and $y$ by reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).saturating_mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         Natural::from(4u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).saturating_mul_sub_mul(
    ///         &Natural::from(1u32),
    ///         Natural::from(2u32),
    ///         Natural::from(2u32)
    ///     ),
    ///     0
    /// );
    /// ```
    #[inline]
    fn saturating_mul_sub_mul(self, y: &Self, z: Self, w: Self) -> Self {
        (self * y).saturating_sub_mul(z, w)
    }
}

impl SaturatingMulSubMul<&Self, Self, &Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning 0
    /// if the result would be negative, taking $x$ and $z$ by value and $y$ and $w$ by reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).saturating_mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).saturating_mul_sub_mul(
    ///         &Natural::from(1u32),
    ///         Natural::from(2u32),
    ///         &Natural::from(2u32)
    ///     ),
    ///     0
    /// );
    /// ```
    #[inline]
    fn saturating_mul_sub_mul(self, y: &Self, z: Self, w: &Self) -> Self {
        (self * y).saturating_sub_mul(z, w)
    }
}

impl SaturatingMulSubMul<&Self, &Self, Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning 0
    /// if the result would be negative, taking $x$ and $w$ by value and $y$ and $z$ by reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).saturating_mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).saturating_mul_sub_mul(
    ///         &Natural::from(1u32),
    ///         &Natural::from(2u32),
    ///         Natural::from(2u32)
    ///     ),
    ///     0
    /// );
    /// ```
    #[inline]
    fn saturating_mul_sub_mul(self, y: &Self, z: &Self, w: Self) -> Self {
        (self * y).saturating_sub_mul(z, w)
    }
}

impl SaturatingMulSubMul<&Self, &Self, &Self> for Natural {
    type Output = Self;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning 0
    /// if the result would be negative, taking $x$ by value and $y$, $z$ and $w$ by reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).saturating_mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// assert_eq!(
    ///     Natural::from(1u32).saturating_mul_sub_mul(
    ///         &Natural::from(1u32),
    ///         &Natural::from(2u32),
    ///         &Natural::from(2u32)
    ///     ),
    ///     0
    /// );
    /// ```
    #[inline]
    fn saturating_mul_sub_mul(self, y: &Self, z: &Self, w: &Self) -> Self {
        (self * y).saturating_sub_mul(z, w)
    }
}

impl SaturatingMulSubMul<&Natural, &Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Subtracts the product of one pair of [`Natural`]s from the product of another, returning 0
    /// if the result would be negative, taking all four by reference.
    ///
    /// $$
    /// f(x, y, z, w) = \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).saturating_mul_sub_mul(
    ///         &Natural::from(3u32),
    ///         &Natural::from(4u32),
    ///         &Natural::from(5u32)
    ///     ),
    ///     10
    /// );
    /// assert_eq!(
    ///     (&Natural::from(1u32)).saturating_mul_sub_mul(
    ///         &Natural::from(1u32),
    ///         &Natural::from(2u32),
    ///         &Natural::from(2u32)
    ///     ),
    ///     0
    /// );
    /// ```
    #[inline]
    fn saturating_mul_sub_mul(self, y: &Natural, z: &Natural, w: &Natural) -> Natural {
        (self * y).saturating_sub_mul(z, w)
    }
}

impl SaturatingMulSubMulAssign<Self, Self, Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// saturating at 0, taking all four by value.
    ///
    /// $$
    /// x \gets \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(1u32);
    /// x.saturating_mul_sub_mul_assign(
    ///     Natural::from(1u32),
    ///     Natural::from(2u32),
    ///     Natural::from(2u32),
    /// );
    /// assert_eq!(x, 0);
    /// ```
    #[inline]
    fn saturating_mul_sub_mul_assign(&mut self, y: Self, z: Self, w: Self) {
        *self *= y;
        self.saturating_sub_mul_assign(z, w);
    }
}

impl SaturatingMulSubMulAssign<Self, Self, &Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// saturating at 0, taking $x$, $y$ and $z$ by value and $w$ by reference.
    ///
    /// $$
    /// x \gets \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(1u32);
    /// x.saturating_mul_sub_mul_assign(
    ///     Natural::from(1u32),
    ///     Natural::from(2u32),
    ///     &Natural::from(2u32),
    /// );
    /// assert_eq!(x, 0);
    /// ```
    #[inline]
    fn saturating_mul_sub_mul_assign(&mut self, y: Self, z: Self, w: &Self) {
        *self *= y;
        self.saturating_sub_mul_assign(z, w);
    }
}

impl SaturatingMulSubMulAssign<Self, &Self, Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// saturating at 0, taking $x$, $y$ and $w$ by value and $z$ by reference.
    ///
    /// $$
    /// x \gets \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(1u32);
    /// x.saturating_mul_sub_mul_assign(
    ///     Natural::from(1u32),
    ///     &Natural::from(2u32),
    ///     Natural::from(2u32),
    /// );
    /// assert_eq!(x, 0);
    /// ```
    #[inline]
    fn saturating_mul_sub_mul_assign(&mut self, y: Self, z: &Self, w: Self) {
        *self *= y;
        self.saturating_sub_mul_assign(z, w);
    }
}

impl SaturatingMulSubMulAssign<Self, &Self, &Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// saturating at 0, taking $x$ and $y$ by value and $z$ and $w$ by reference.
    ///
    /// $$
    /// x \gets \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(1u32);
    /// x.saturating_mul_sub_mul_assign(
    ///     Natural::from(1u32),
    ///     &Natural::from(2u32),
    ///     &Natural::from(2u32),
    /// );
    /// assert_eq!(x, 0);
    /// ```
    #[inline]
    fn saturating_mul_sub_mul_assign(&mut self, y: Self, z: &Self, w: &Self) {
        *self *= y;
        self.saturating_sub_mul_assign(z, w);
    }
}

impl SaturatingMulSubMulAssign<&Self, Self, Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// saturating at 0, taking $x$, $z$ and $w$ by value and $y$ by reference.
    ///
    /// $$
    /// x \gets \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(1u32);
    /// x.saturating_mul_sub_mul_assign(
    ///     &Natural::from(1u32),
    ///     Natural::from(2u32),
    ///     Natural::from(2u32),
    /// );
    /// assert_eq!(x, 0);
    /// ```
    #[inline]
    fn saturating_mul_sub_mul_assign(&mut self, y: &Self, z: Self, w: Self) {
        *self *= y;
        self.saturating_sub_mul_assign(z, w);
    }
}

impl SaturatingMulSubMulAssign<&Self, Self, &Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// saturating at 0, taking $x$ and $z$ by value and $y$ and $w$ by reference.
    ///
    /// $$
    /// x \gets \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(1u32);
    /// x.saturating_mul_sub_mul_assign(
    ///     &Natural::from(1u32),
    ///     Natural::from(2u32),
    ///     &Natural::from(2u32),
    /// );
    /// assert_eq!(x, 0);
    /// ```
    #[inline]
    fn saturating_mul_sub_mul_assign(&mut self, y: &Self, z: Self, w: &Self) {
        *self *= y;
        self.saturating_sub_mul_assign(z, w);
    }
}

impl SaturatingMulSubMulAssign<&Self, &Self, Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// saturating at 0, taking $x$ and $w$ by value and $y$ and $z$ by reference.
    ///
    /// $$
    /// x \gets \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(1u32);
    /// x.saturating_mul_sub_mul_assign(
    ///     &Natural::from(1u32),
    ///     &Natural::from(2u32),
    ///     Natural::from(2u32),
    /// );
    /// assert_eq!(x, 0);
    /// ```
    #[inline]
    fn saturating_mul_sub_mul_assign(&mut self, y: &Self, z: &Self, w: Self) {
        *self *= y;
        self.saturating_sub_mul_assign(z, w);
    }
}

impl SaturatingMulSubMulAssign<&Self, &Self, &Self> for Natural {
    /// Subtracts the product of one pair of [`Natural`]s from the product of another, in place,
    /// saturating at 0, taking $x$ by value and $y$, $z$ and $w$ by reference.
    ///
    /// $$
    /// x \gets \begin{cases}
    ///     xy - zw & \text{if} \quad xy \geq zw \\\\
    ///     0 & \text{otherwise}
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
    /// use malachite_base::num::arithmetic::traits::{SaturatingMulSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(1u32);
    /// x.saturating_mul_sub_mul_assign(
    ///     &Natural::from(1u32),
    ///     &Natural::from(2u32),
    ///     &Natural::from(2u32),
    /// );
    /// assert_eq!(x, 0);
    /// ```
    #[inline]
    fn saturating_mul_sub_mul_assign(&mut self, y: &Self, z: &Self, w: &Self) {
        *self *= y;
        self.saturating_sub_mul_assign(z, w);
    }
}
