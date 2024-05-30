// Copyright © 2024 Mikhail Hogrefe
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

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{AbsDiff, AbsDiffAssign};

impl AbsDiff<Natural> for Natural {
    type Output = Natural;

    /// Computes the absolute value of the difference between two [`Natural`]s, taking both by
    /// value.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32).abs_diff(Natural::ZERO), 123);
    /// assert_eq!(Natural::ZERO.abs_diff(Natural::from(123u32)), 123);
    /// assert_eq!(Natural::from(456u32).abs_diff(Natural::from(123u32)), 333);
    /// assert_eq!(Natural::from(123u32).abs_diff(Natural::from(456u32)), 333);
    /// assert_eq!(
    ///     (Natural::from(10u32).pow(12) * Natural::from(3u32))
    ///         .abs_diff(Natural::from(10u32).pow(12)),
    ///     2000000000000u64
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .abs_diff(Natural::from(10u32).pow(12) * Natural::from(3u32)),
    ///     2000000000000u64
    /// );
    /// ```
    fn abs_diff(self, other: Natural) -> Natural {
        if self >= other {
            self - other
        } else {
            other - self
        }
    }
}

impl<'a> AbsDiff<&'a Natural> for Natural {
    type Output = Natural;

    /// Computes the absolute value of the difference between two [`Natural`]s, taking the first by
    /// value and the second by reference.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(123u32).abs_diff(&Natural::ZERO), 123);
    /// assert_eq!(Natural::ZERO.abs_diff(&Natural::from(123u32)), 123);
    /// assert_eq!(Natural::from(456u32).abs_diff(&Natural::from(123u32)), 333);
    /// assert_eq!(Natural::from(123u32).abs_diff(&Natural::from(456u32)), 333);
    /// assert_eq!(
    ///     (Natural::from(10u32).pow(12) * Natural::from(3u32))
    ///         .abs_diff(&Natural::from(10u32).pow(12)),
    ///     2000000000000u64
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .abs_diff(&(Natural::from(10u32).pow(12) * Natural::from(3u32))),
    ///     2000000000000u64
    /// );
    /// ```
    fn abs_diff(self, other: &'a Natural) -> Natural {
        if self >= *other {
            self - other
        } else {
            other - self
        }
    }
}

impl<'a> AbsDiff<Natural> for &'a Natural {
    type Output = Natural;

    /// Computes the absolute value of the difference between two [`Natural`]s, taking the first by
    /// reference and the second by value.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(123u32)).abs_diff(Natural::ZERO), 123);
    /// assert_eq!((&Natural::ZERO).abs_diff(Natural::from(123u32)), 123);
    /// assert_eq!(
    ///     (&Natural::from(456u32)).abs_diff(Natural::from(123u32)),
    ///     333
    /// );
    /// assert_eq!(
    ///     (&Natural::from(123u32)).abs_diff(Natural::from(456u32)),
    ///     333
    /// );
    /// assert_eq!(
    ///     (&(Natural::from(10u32).pow(12) * Natural::from(3u32)))
    ///         .abs_diff(Natural::from(10u32).pow(12)),
    ///     2000000000000u64
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12))
    ///         .abs_diff(Natural::from(10u32).pow(12) * Natural::from(3u32)),
    ///     2000000000000u64
    /// );
    /// ```
    fn abs_diff(self, other: Natural) -> Natural {
        if *self >= other {
            self - other
        } else {
            other - self
        }
    }
}

impl<'a, 'b> AbsDiff<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Computes the absolute value of the difference between two [`Natural`]s, taking both by
    /// reference.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(123u32)).abs_diff(&Natural::ZERO), 123);
    /// assert_eq!((&Natural::ZERO).abs_diff(&Natural::from(123u32)), 123);
    /// assert_eq!(
    ///     (&Natural::from(456u32)).abs_diff(&Natural::from(123u32)),
    ///     333
    /// );
    /// assert_eq!(
    ///     (&Natural::from(123u32)).abs_diff(&Natural::from(456u32)),
    ///     333
    /// );
    /// assert_eq!(
    ///     (&(Natural::from(10u32).pow(12) * Natural::from(3u32)))
    ///         .abs_diff(&Natural::from(10u32).pow(12)),
    ///     2000000000000u64
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12))
    ///         .abs_diff(&(Natural::from(10u32).pow(12) * Natural::from(3u32))),
    ///     2000000000000u64
    /// );
    /// ```
    fn abs_diff(self, other: &'a Natural) -> Natural {
        if self >= other {
            self - other
        } else {
            other - self
        }
    }
}

impl AbsDiffAssign<Natural> for Natural {
    /// Subtracts a [`Natural`] by another [`Natural`] in place and takes the absolute value, taking
    /// the [`Natural`] on the right-hand side by value.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(123u32);
    /// x.abs_diff_assign(Natural::ZERO);
    /// assert_eq!(x, 123);
    ///
    /// let mut x = Natural::ZERO;
    /// x.abs_diff_assign(Natural::from(123u32));
    /// assert_eq!(x, 123);
    ///
    /// let mut x = Natural::from(456u32);
    /// x.abs_diff_assign(Natural::from(123u32));
    /// assert_eq!(x, 333);
    ///
    /// let mut x = Natural::from(123u32);
    /// x.abs_diff_assign(Natural::from(456u32));
    /// assert_eq!(x, 333);
    ///
    /// let mut x = Natural::from(10u32).pow(12) * Natural::from(3u32);
    /// x.abs_diff_assign(Natural::from(10u32).pow(12));
    /// assert_eq!(x, 2000000000000u64);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.abs_diff_assign(Natural::from(10u32).pow(12) * Natural::from(3u32));
    /// assert_eq!(x, 2000000000000u64);
    /// ```
    fn abs_diff_assign(&mut self, other: Natural) {
        if *self >= other {
            *self -= other;
        } else {
            self.sub_right_assign_no_panic(&other);
        }
    }
}

impl<'a> AbsDiffAssign<&'a Natural> for Natural {
    /// Subtracts a [`Natural`] by another [`Natural`] in place and takes the absolute value, taking
    /// the [`Natural`] on the right-hand side by reference.
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
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(123u32);
    /// x.abs_diff_assign(&Natural::ZERO);
    /// assert_eq!(x, 123);
    ///
    /// let mut x = Natural::ZERO;
    /// x.abs_diff_assign(&Natural::from(123u32));
    /// assert_eq!(x, 123);
    ///
    /// let mut x = Natural::from(456u32);
    /// x.abs_diff_assign(&Natural::from(123u32));
    /// assert_eq!(x, 333);
    ///
    /// let mut x = Natural::from(123u32);
    /// x.abs_diff_assign(&Natural::from(456u32));
    /// assert_eq!(x, 333);
    ///
    /// let mut x = Natural::from(10u32).pow(12) * Natural::from(3u32);
    /// x.abs_diff_assign(&Natural::from(10u32).pow(12));
    /// assert_eq!(x, 2000000000000u64);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.abs_diff_assign(&Natural::from(10u32).pow(12) * Natural::from(3u32));
    /// assert_eq!(x, 2000000000000u64);
    /// ```
    fn abs_diff_assign(&mut self, other: &'a Natural) {
        if *self >= *other {
            *self -= other;
        } else {
            self.sub_right_assign_no_panic(other);
        }
    }
}
