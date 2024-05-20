// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{CheckedSub, SaturatingSub, SaturatingSubAssign};
use malachite_base::num::basic::traits::Zero;

impl SaturatingSub<Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by another [`Natural`], taking both by value and returning 0 if the
    /// result is negative.
    ///
    /// $$
    /// f(x, y) = \max(x - y, 0).
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
    /// use malachite_base::num::arithmetic::traits::{Pow, SaturatingSub};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.saturating_sub(Natural::from(123u32)), 0);
    /// assert_eq!(Natural::from(123u32).saturating_sub(Natural::ZERO), 123);
    /// assert_eq!(
    ///     Natural::from(456u32).saturating_sub(Natural::from(123u32)),
    ///     333
    /// );
    /// assert_eq!(
    ///     (Natural::from(10u32).pow(12) * Natural::from(3u32))
    ///         .saturating_sub(Natural::from(10u32).pow(12)),
    ///     2000000000000u64
    /// );
    /// ```
    #[inline]
    fn saturating_sub(self, other: Natural) -> Natural {
        CheckedSub::checked_sub(self, other).unwrap_or(Natural::ZERO)
    }
}

impl<'a> SaturatingSub<&'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by another [`Natural`], taking the first by value and the second by
    /// reference and returning 0 if the result is negative.
    ///
    /// $$
    /// f(x, y) = \max(x - y, 0).
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
    /// use malachite_base::num::arithmetic::traits::{Pow, SaturatingSub};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.saturating_sub(&Natural::from(123u32)), 0);
    /// assert_eq!(Natural::from(123u32).saturating_sub(&Natural::ZERO), 123);
    /// assert_eq!(
    ///     Natural::from(456u32).saturating_sub(&Natural::from(123u32)),
    ///     333
    /// );
    /// assert_eq!(
    ///     (Natural::from(10u32).pow(12) * Natural::from(3u32))
    ///         .saturating_sub(&Natural::from(10u32).pow(12)),
    ///     2000000000000u64
    /// );
    /// ```
    #[inline]
    fn saturating_sub(self, other: &'a Natural) -> Natural {
        CheckedSub::checked_sub(self, other).unwrap_or(Natural::ZERO)
    }
}

impl<'a> SaturatingSub<Natural> for &'a Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by another [`Natural`], taking the first by reference and the second
    /// by value and returning 0 if the result is negative.
    ///
    /// $$
    /// f(x, y) = \max(x - y, 0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SaturatingSub};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).saturating_sub(Natural::from(123u32)), 0);
    /// assert_eq!((&Natural::from(123u32)).saturating_sub(Natural::ZERO), 123);
    /// assert_eq!(
    ///     (&Natural::from(456u32)).saturating_sub(Natural::from(123u32)),
    ///     333
    /// );
    /// assert_eq!(
    ///     (&(Natural::from(10u32).pow(12) * Natural::from(3u32)))
    ///         .saturating_sub(Natural::from(10u32).pow(12)),
    ///     2000000000000u64
    /// );
    /// ```
    #[inline]
    fn saturating_sub(self, other: Natural) -> Natural {
        CheckedSub::checked_sub(self, other).unwrap_or(Natural::ZERO)
    }
}

impl<'a, 'b> SaturatingSub<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by another [`Natural`], taking both by reference and returning 0 if
    /// the result is negative.
    ///
    /// $$
    /// f(x, y) = \max(x - y, 0).
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
    /// use malachite_base::num::arithmetic::traits::{Pow, SaturatingSub};
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).saturating_sub(&Natural::from(123u32)), 0);
    /// assert_eq!((&Natural::from(123u32)).saturating_sub(&Natural::ZERO), 123);
    /// assert_eq!(
    ///     (&Natural::from(456u32)).saturating_sub(&Natural::from(123u32)),
    ///     333
    /// );
    /// assert_eq!(
    ///     (&(Natural::from(10u32).pow(12) * Natural::from(3u32)))
    ///         .saturating_sub(&Natural::from(10u32).pow(12)),
    ///     2000000000000u64
    /// );
    /// ```
    #[inline]
    fn saturating_sub(self, other: &'a Natural) -> Natural {
        CheckedSub::checked_sub(self, other).unwrap_or(Natural::ZERO)
    }
}

impl SaturatingSubAssign<Natural> for Natural {
    /// Subtracts a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by value and setting the left-hand side to 0 if the result is negative.
    ///
    /// $$
    /// x \gets \max(x - y, 0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SaturatingSubAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(123u32);
    /// x.saturating_sub_assign(Natural::from(123u32));
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Natural::from(123u32);
    /// x.saturating_sub_assign(Natural::ZERO);
    /// assert_eq!(x, 123);
    ///
    /// let mut x = Natural::from(456u32);
    /// x.saturating_sub_assign(Natural::from(123u32));
    /// assert_eq!(x, 333);
    ///
    /// let mut x = Natural::from(123u32);
    /// x.saturating_sub_assign(Natural::from(456u32));
    /// assert_eq!(x, 0);
    /// ```
    #[inline]
    fn saturating_sub_assign(&mut self, other: Natural) {
        if self.sub_assign_ref_no_panic(&other) {
            *self = Natural::ZERO;
        }
    }
}

impl<'a> SaturatingSubAssign<&'a Natural> for Natural {
    /// Subtracts a [`Natural`] by another [`Natural`] in place, taking the [`Natural`] on the
    /// right-hand side by reference and setting the left-hand side to 0 if the result is negative.
    ///
    /// $$
    /// x \gets \max(x - y, 0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `other` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SaturatingSubAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(123u32);
    /// x.saturating_sub_assign(&Natural::from(123u32));
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Natural::from(123u32);
    /// x.saturating_sub_assign(&Natural::ZERO);
    /// assert_eq!(x, 123);
    ///
    /// let mut x = Natural::from(456u32);
    /// x.saturating_sub_assign(&Natural::from(123u32));
    /// assert_eq!(x, 333);
    ///
    /// let mut x = Natural::from(123u32);
    /// x.saturating_sub_assign(&Natural::from(456u32));
    /// assert_eq!(x, 0);
    /// ```
    #[inline]
    fn saturating_sub_assign(&mut self, other: &'a Natural) {
        if self.sub_assign_ref_no_panic(other) {
            *self = Natural::ZERO;
        }
    }
}
