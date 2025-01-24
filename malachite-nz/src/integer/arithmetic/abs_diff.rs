// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991-2018, 2020 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{AbsAssign, AbsDiff, AbsDiffAssign, UnsignedAbs};

impl AbsDiff<Integer> for Integer {
    type Output = Natural;

    /// Computes the absolute value of the difference between two [`Integer`]s, taking both by
    /// value. A [`Natural`] is returned.
    ///
    /// $$
    /// f(x, y) = |x - y|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AbsDiff, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(123).abs_diff(Integer::ZERO), 123);
    /// assert_eq!(Integer::ZERO.abs_diff(Integer::from(123)), 123);
    /// assert_eq!(Integer::from(456).abs_diff(Integer::from(-123)), 579);
    /// assert_eq!(Integer::from(123).abs_diff(Integer::from(-456)), 579);
    /// assert_eq!(
    ///     (Integer::from(10).pow(12) * Integer::from(3)).abs_diff(Integer::from(10).pow(12)),
    ///     2000000000000u64
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10).pow(12)).abs_diff(-Integer::from(10).pow(12) * Integer::from(3)),
    ///     2000000000000u64
    /// );
    /// ```
    #[inline]
    fn abs_diff(self, other: Integer) -> Natural {
        (self - other).unsigned_abs()
    }
}

impl AbsDiff<&Integer> for Integer {
    type Output = Natural;

    /// Computes the absolute value of the difference between two [`Integer`]s, taking the first by
    /// value and the second by reference. A [`Natural`] is returned.
    ///
    /// $$
    /// f(x, y) = |x - y|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AbsDiff, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(123).abs_diff(&Integer::ZERO), 123);
    /// assert_eq!(Integer::ZERO.abs_diff(&Integer::from(123)), 123);
    /// assert_eq!(Integer::from(456).abs_diff(&Integer::from(-123)), 579);
    /// assert_eq!(Integer::from(123).abs_diff(&Integer::from(-456)), 579);
    /// assert_eq!(
    ///     (Integer::from(10).pow(12) * Integer::from(3)).abs_diff(&Integer::from(10).pow(12)),
    ///     2000000000000u64
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10).pow(12)).abs_diff(&(-Integer::from(10).pow(12) * Integer::from(3))),
    ///     2000000000000u64
    /// );
    /// ```
    #[inline]
    fn abs_diff(self, other: &Integer) -> Natural {
        (self - other).unsigned_abs()
    }
}

impl AbsDiff<Integer> for &Integer {
    type Output = Natural;

    /// Computes the absolute value of the difference between two [`Integer`]s, taking the first by
    /// reference and the second by value. A [`Natural`] is returned.
    ///
    /// $$
    /// f(x, y) = |x - y|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AbsDiff, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(123)).abs_diff(Integer::ZERO), 123);
    /// assert_eq!((&Integer::ZERO).abs_diff(Integer::from(123)), 123);
    /// assert_eq!((&Integer::from(456)).abs_diff(Integer::from(-123)), 579);
    /// assert_eq!((&Integer::from(123)).abs_diff(Integer::from(-456)), 579);
    /// assert_eq!(
    ///     (&(Integer::from(10).pow(12) * Integer::from(3))).abs_diff(Integer::from(10).pow(12)),
    ///     2000000000000u64
    /// );
    /// assert_eq!(
    ///     (&(-Integer::from(10).pow(12))).abs_diff(-Integer::from(10).pow(12) * Integer::from(3)),
    ///     2000000000000u64
    /// );
    /// ```
    #[inline]
    fn abs_diff(self, other: Integer) -> Natural {
        (self - other).unsigned_abs()
    }
}

impl AbsDiff<&Integer> for &Integer {
    type Output = Natural;

    /// Computes the absolute value of the difference between two [`Integer`]s, taking both by
    /// reference. A [`Natural`] is returned.
    ///
    /// $$
    /// f(x, y) = |x - y|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AbsDiff, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(123)).abs_diff(&Integer::ZERO), 123);
    /// assert_eq!((&Integer::ZERO).abs_diff(&Integer::from(123)), 123);
    /// assert_eq!((&Integer::from(456)).abs_diff(&Integer::from(-123)), 579);
    /// assert_eq!((&Integer::from(123)).abs_diff(&Integer::from(-456)), 579);
    /// assert_eq!(
    ///     (&(Integer::from(10).pow(12) * Integer::from(3))).abs_diff(&Integer::from(10).pow(12)),
    ///     2000000000000u64
    /// );
    /// assert_eq!(
    ///     (&(-Integer::from(10).pow(12)))
    ///         .abs_diff(&(-Integer::from(10).pow(12) * Integer::from(3))),
    ///     2000000000000u64
    /// );
    /// ```
    #[inline]
    fn abs_diff(self, other: &Integer) -> Natural {
        (self - other).unsigned_abs()
    }
}

impl AbsDiffAssign<Integer> for Integer {
    /// Subtracts an [`Integer`] by another [`Integer`] in place and takes the absolute value,
    /// taking the [`Integer`] on the right-hand side by value.
    ///
    /// $$
    /// x \gets |x - y|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AbsDiffAssign, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(123);
    /// x.abs_diff_assign(Integer::ZERO);
    /// assert_eq!(x, 123);
    ///
    /// let mut x = Integer::ZERO;
    /// x.abs_diff_assign(Integer::from(123));
    /// assert_eq!(x, 123);
    ///
    /// let mut x = Integer::from(456);
    /// x.abs_diff_assign(Integer::from(-123));
    /// assert_eq!(x, 579);
    ///
    /// let mut x = Integer::from(-123);
    /// x.abs_diff_assign(Integer::from(456));
    /// assert_eq!(x, 579);
    ///
    /// let mut x = Integer::from(10).pow(12) * Integer::from(3);
    /// x.abs_diff_assign(Integer::from(10u32).pow(12));
    /// assert_eq!(x, 2000000000000u64);
    ///
    /// let mut x = -Integer::from(10u32).pow(12);
    /// x.abs_diff_assign(-(Integer::from(10).pow(12) * Integer::from(3)));
    /// assert_eq!(x, 2000000000000u64);
    /// ```
    #[inline]
    fn abs_diff_assign(&mut self, other: Integer) {
        *self -= other;
        self.abs_assign();
    }
}

impl<'a> AbsDiffAssign<&'a Integer> for Integer {
    /// Subtracts an [`Integer`] by another [`Integer`] in place and takes the absolute value,
    /// taking the [`Integer`] on the right-hand side by reference.
    ///
    /// $$
    /// x \gets |x - y|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AbsDiffAssign, Pow};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(123);
    /// x.abs_diff_assign(&Integer::ZERO);
    /// assert_eq!(x, 123);
    ///
    /// let mut x = Integer::ZERO;
    /// x.abs_diff_assign(&Integer::from(123));
    /// assert_eq!(x, 123);
    ///
    /// let mut x = Integer::from(456);
    /// x.abs_diff_assign(&Integer::from(-123));
    /// assert_eq!(x, 579);
    ///
    /// let mut x = Integer::from(-123);
    /// x.abs_diff_assign(&Integer::from(456));
    /// assert_eq!(x, 579);
    ///
    /// let mut x = Integer::from(10).pow(12) * Integer::from(3);
    /// x.abs_diff_assign(&Integer::from(10u32).pow(12));
    /// assert_eq!(x, 2000000000000u64);
    ///
    /// let mut x = -Integer::from(10u32).pow(12);
    /// x.abs_diff_assign(&(-(Integer::from(10).pow(12) * Integer::from(3))));
    /// assert_eq!(x, 2000000000000u64);
    /// ```
    #[inline]
    fn abs_diff_assign(&mut self, other: &'a Integer) {
        *self -= other;
        self.abs_assign();
    }
}
