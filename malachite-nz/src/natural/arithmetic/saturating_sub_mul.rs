// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    CheckedSubMul, SaturatingSubMul, SaturatingSubMulAssign,
};
use malachite_base::num::basic::traits::Zero;

impl SaturatingSubMul<Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking all three by value
    /// and returning 0 if the result is negative.
    ///
    /// $$
    /// f(x, y, z) = \max(x - yz, 0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SaturatingSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32).saturating_sub_mul(Natural::from(3u32), Natural::from(4u32)),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).saturating_sub_mul(Natural::from(3u32), Natural::from(4u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .saturating_sub_mul(Natural::from(0x10000u32), Natural::from(0x10000u32)),
    ///     995705032704u64
    /// );
    /// ```
    #[inline]
    fn saturating_sub_mul(self, y: Natural, z: Natural) -> Natural {
        self.checked_sub_mul(y, z).unwrap_or(Natural::ZERO)
    }
}

impl SaturatingSubMul<Natural, &Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking the first two by
    /// value and the third by reference and returning 0 if the result is negative.
    ///
    /// $$
    /// f(x, y, z) = \max(x - yz, 0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SaturatingSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32).saturating_sub_mul(Natural::from(3u32), &Natural::from(4u32)),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).saturating_sub_mul(Natural::from(3u32), &Natural::from(4u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .saturating_sub_mul(Natural::from(0x10000u32), &Natural::from(0x10000u32)),
    ///     995705032704u64
    /// );
    /// ```
    #[inline]
    fn saturating_sub_mul(self, y: Natural, z: &Natural) -> Natural {
        self.checked_sub_mul(y, z).unwrap_or(Natural::ZERO)
    }
}

impl SaturatingSubMul<&Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking the first and third
    /// by value and the second by reference and returning 0 if the result is negative.
    ///
    /// $$
    /// f(x, y, z) = \max(x - yz, 0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SaturatingSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32).saturating_sub_mul(&Natural::from(3u32), Natural::from(4u32)),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).saturating_sub_mul(&Natural::from(3u32), Natural::from(4u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .saturating_sub_mul(&Natural::from(0x10000u32), Natural::from(0x10000u32)),
    ///     995705032704u64
    /// );
    /// ```
    #[inline]
    fn saturating_sub_mul(self, y: &Natural, z: Natural) -> Natural {
        self.checked_sub_mul(y, z).unwrap_or(Natural::ZERO)
    }
}

impl SaturatingSubMul<&Natural, &Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking the first by value
    /// and the second and third by reference and returning 0 if the result is negative.
    ///
    /// $$
    /// f(x, y, z) = \max(x - yz, 0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SaturatingSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32).saturating_sub_mul(&Natural::from(3u32), &Natural::from(4u32)),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).saturating_sub_mul(&Natural::from(3u32), &Natural::from(4u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .saturating_sub_mul(&Natural::from(0x10000u32), &Natural::from(0x10000u32)),
    ///     995705032704u64
    /// );
    /// ```
    #[inline]
    fn saturating_sub_mul(self, y: &Natural, z: &Natural) -> Natural {
        self.checked_sub_mul(y, z).unwrap_or(Natural::ZERO)
    }
}

impl SaturatingSubMul<&Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking all three by
    /// reference and returning 0 if the result is negative.
    ///
    /// $$
    /// f(x, y, z) = \max(x - yz, 0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SaturatingSubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(20u32)).saturating_sub_mul(&Natural::from(3u32), &Natural::from(4u32)),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).saturating_sub_mul(&Natural::from(3u32), &Natural::from(4u32)),
    ///     0
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12))
    ///         .saturating_sub_mul(&Natural::from(0x10000u32), &Natural::from(0x10000u32)),
    ///     995705032704u64
    /// );
    /// ```
    #[inline]
    fn saturating_sub_mul(self, y: &Natural, z: &Natural) -> Natural {
        self.checked_sub_mul(y, z).unwrap_or(Natural::ZERO)
    }
}

impl SaturatingSubMulAssign<Natural, Natural> for Natural {
    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s in place, taking both
    /// [`Natural`]s on the right-hand side by value and replacing the left-hand side [`Natural`]
    /// with 0 if the result is negative.
    ///
    /// $$
    /// x \gets \max(x - yz, 0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SaturatingSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(20u32);
    /// x.saturating_sub_mul_assign(Natural::from(3u32), Natural::from(4u32));
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.saturating_sub_mul_assign(Natural::from(3u32), Natural::from(4u32));
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.saturating_sub_mul_assign(Natural::from(0x10000u32), Natural::from(0x10000u32));
    /// assert_eq!(x, 995705032704u64);
    /// ```
    #[inline]
    fn saturating_sub_mul_assign(&mut self, y: Natural, z: Natural) {
        if self.sub_mul_assign_no_panic(y, z) {
            *self = Natural::ZERO;
        }
    }
}

impl SaturatingSubMulAssign<Natural, &Natural> for Natural {
    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s in place, taking the first
    /// [`Natural`] on the right-hand side by value and the second by reference and replacing the
    /// left-hand side [`Natural`] with 0 if the result is negative.
    ///
    /// $$
    /// x \gets \max(x - yz, 0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SaturatingSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(20u32);
    /// x.saturating_sub_mul_assign(Natural::from(3u32), &Natural::from(4u32));
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.saturating_sub_mul_assign(Natural::from(3u32), &Natural::from(4u32));
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.saturating_sub_mul_assign(Natural::from(0x10000u32), &Natural::from(0x10000u32));
    /// assert_eq!(x, 995705032704u64);
    /// ```
    #[inline]
    fn saturating_sub_mul_assign(&mut self, y: Natural, z: &Natural) {
        if self.sub_mul_assign_val_ref_no_panic(y, z) {
            *self = Natural::ZERO;
        }
    }
}

impl SaturatingSubMulAssign<&Natural, Natural> for Natural {
    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s in place, taking the first
    /// [`Natural`] on the right-hand side by reference and the second by value and replacing the
    /// left-hand side [`Natural`] with 0 if the result is negative.
    ///
    /// $$
    /// x \gets \max(x - yz, 0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SaturatingSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(20u32);
    /// x.saturating_sub_mul_assign(&Natural::from(3u32), Natural::from(4u32));
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.saturating_sub_mul_assign(&Natural::from(3u32), Natural::from(4u32));
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.saturating_sub_mul_assign(&Natural::from(0x10000u32), Natural::from(0x10000u32));
    /// assert_eq!(x, 995705032704u64);
    /// ```
    #[inline]
    fn saturating_sub_mul_assign(&mut self, y: &Natural, z: Natural) {
        if self.sub_mul_assign_ref_val_no_panic(y, z) {
            *self = Natural::ZERO;
        }
    }
}

impl SaturatingSubMulAssign<&Natural, &Natural> for Natural {
    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s in place, taking both
    /// [`Natural`]s on the right-hand side by reference and replacing the left-hand side
    /// [`Natural`] with 0 if the result is negative.
    ///
    /// $$
    /// x \gets \max(x - yz, 0).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SaturatingSubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(20u32);
    /// x.saturating_sub_mul_assign(&Natural::from(3u32), &Natural::from(4u32));
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.saturating_sub_mul_assign(&Natural::from(3u32), &Natural::from(4u32));
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.saturating_sub_mul_assign(&Natural::from(0x10000u32), &Natural::from(0x10000u32));
    /// assert_eq!(x, 995705032704u64);
    /// ```
    #[inline]
    fn saturating_sub_mul_assign(&mut self, y: &Natural, z: &Natural) {
        if self.sub_mul_assign_ref_ref_no_panic(y, z) {
            *self = Natural::ZERO;
        }
    }
}
