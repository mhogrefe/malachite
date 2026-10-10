// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, AddMulShl, AddMulShlAssign};

impl AddMulShl<Self, Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, taking
    /// all three by value.
    ///
    /// $f(x, y, z, k) = x + yz2^k$.
    ///
    /// When `bits` is 0, this is [`AddMul`].
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `max(self.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMulShl, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).add_mul_shl(Natural::from(3u32), Natural::from(4u32), 2),
    ///     58
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).add_mul_shl(
    ///         Natural::from(0x10000u32),
    ///         Natural::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     524289000000000000u64
    /// );
    /// ```
    #[inline]
    fn add_mul_shl(mut self, y: Self, z: Self, bits: u64) -> Self {
        self.add_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'b> AddMulShl<Self, &'b Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, taking
    /// the first [`Natural`] by value, the second by value, and the third by reference.
    ///
    /// $f(x, y, z, k) = x + yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `max(self.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMulShl, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).add_mul_shl(Natural::from(3u32), &Natural::from(4u32), 2),
    ///     58
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).add_mul_shl(
    ///         Natural::from(0x10000u32),
    ///         &Natural::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     524289000000000000u64
    /// );
    /// ```
    #[inline]
    fn add_mul_shl(mut self, y: Self, z: &'b Self, bits: u64) -> Self {
        self.add_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'a> AddMulShl<&'a Self, Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, taking
    /// the first [`Natural`] by value, the second by reference, and the third by value.
    ///
    /// $f(x, y, z, k) = x + yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `max(self.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMulShl, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).add_mul_shl(&Natural::from(3u32), Natural::from(4u32), 2),
    ///     58
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).add_mul_shl(
    ///         &Natural::from(0x10000u32),
    ///         Natural::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     524289000000000000u64
    /// );
    /// ```
    #[inline]
    fn add_mul_shl(mut self, y: &'a Self, z: Self, bits: u64) -> Self {
        self.add_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'a, 'b> AddMulShl<&'a Self, &'b Self> for Natural {
    type Output = Self;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, taking
    /// the first [`Natural`] by value, the second by reference, and the third by reference.
    ///
    /// $f(x, y, z, k) = x + yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `max(self.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMulShl, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).add_mul_shl(&Natural::from(3u32), &Natural::from(4u32), 2),
    ///     58
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).pow(12).add_mul_shl(
    ///         &Natural::from(0x10000u32),
    ///         &Natural::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     524289000000000000u64
    /// );
    /// ```
    #[inline]
    fn add_mul_shl(mut self, y: &'a Self, z: &'b Self, bits: u64) -> Self {
        self.add_mul_shl_assign(y, z, bits);
        self
    }
}

impl AddMulShl<&Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, taking
    /// all three by reference.
    ///
    /// $f(x, y, z, k) = x + yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `max(self.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMulShl, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).add_mul_shl(&Natural::from(3u32), &Natural::from(4u32), 2),
    ///     58
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12)).add_mul_shl(
    ///         &Natural::from(0x10000u32),
    ///         &Natural::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     524289000000000000u64
    /// );
    /// ```
    fn add_mul_shl(self, y: &Natural, z: &Natural, bits: u64) -> Natural {
        if bits == 0 {
            self.add_mul(y, z)
        } else {
            self + ((y * z) << bits)
        }
    }
}

impl AddMulShlAssign<Self, Self> for Natural {
    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, in
    /// place, taking both [`Natural`]s on the right-hand side by value.
    ///
    /// $x \gets x + yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `max(self.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMulShlAssign, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.add_mul_shl_assign(Natural::from(3u32), Natural::from(4u32), 2);
    /// assert_eq!(x, 58);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.add_mul_shl_assign(Natural::from(0x10000u32), Natural::from(10u32).pow(12), 3);
    /// assert_eq!(x, 524289000000000000u64);
    /// ```
    fn add_mul_shl_assign(&mut self, mut y: Self, z: Self, bits: u64) {
        if bits == 0 {
            self.add_mul_assign(y, z);
        } else {
            y *= z;
            y <<= bits;
            *self += y;
        }
    }
}

impl<'b> AddMulShlAssign<Self, &'b Self> for Natural {
    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, in
    /// place, taking the first [`Natural`] on the right-hand side by value and the second by
    /// reference.
    ///
    /// $x \gets x + yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `max(self.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMulShlAssign, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.add_mul_shl_assign(Natural::from(3u32), &Natural::from(4u32), 2);
    /// assert_eq!(x, 58);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.add_mul_shl_assign(Natural::from(0x10000u32), &Natural::from(10u32).pow(12), 3);
    /// assert_eq!(x, 524289000000000000u64);
    /// ```
    fn add_mul_shl_assign(&mut self, mut y: Self, z: &'b Self, bits: u64) {
        if bits == 0 {
            self.add_mul_assign(y, z);
        } else {
            y *= z;
            y <<= bits;
            *self += y;
        }
    }
}

impl<'a> AddMulShlAssign<&'a Self, Self> for Natural {
    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, in
    /// place, taking the first [`Natural`] on the right-hand side by reference and the second by
    /// value.
    ///
    /// $x \gets x + yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `max(self.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMulShlAssign, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.add_mul_shl_assign(&Natural::from(3u32), Natural::from(4u32), 2);
    /// assert_eq!(x, 58);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.add_mul_shl_assign(&Natural::from(0x10000u32), Natural::from(10u32).pow(12), 3);
    /// assert_eq!(x, 524289000000000000u64);
    /// ```
    fn add_mul_shl_assign(&mut self, y: &'a Self, mut z: Self, bits: u64) {
        if bits == 0 {
            self.add_mul_assign(y, z);
        } else {
            z *= y;
            z <<= bits;
            *self += z;
        }
    }
}

impl<'a, 'b> AddMulShlAssign<&'a Self, &'b Self> for Natural {
    /// Adds a [`Natural`] and the product of two other [`Natural`]s, shifted left by `bits`, in
    /// place, taking both [`Natural`]s on the right-hand side by reference.
    ///
    /// $x \gets x + yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `max(self.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{AddMulShlAssign, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.add_mul_shl_assign(&Natural::from(3u32), &Natural::from(4u32), 2);
    /// assert_eq!(x, 58);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.add_mul_shl_assign(&Natural::from(0x10000u32), &Natural::from(10u32).pow(12), 3);
    /// assert_eq!(x, 524289000000000000u64);
    /// ```
    fn add_mul_shl_assign(&mut self, y: &'a Self, z: &'b Self, bits: u64) {
        if bits == 0 {
            self.add_mul_assign(y, z);
        } else {
            *self += (y * z) << bits;
        }
    }
}
