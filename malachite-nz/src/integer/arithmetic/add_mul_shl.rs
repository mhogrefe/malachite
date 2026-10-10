// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, AddMulShl, AddMulShlAssign};

impl AddMulShl<Self, Self> for Integer {
    type Output = Self;

    /// Adds an [`Integer`] and the product of two other [`Integer`]s, shifted left by `bits`,
    /// taking all three by value.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10).add_mul_shl(Integer::from(3), Integer::from(4), 2),
    ///     58
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).add_mul_shl(Integer::from(3), Integer::from(-4), 2),
    ///     -58
    /// );
    /// assert_eq!(
    ///     Integer::from(10u32).pow(12).add_mul_shl(
    ///         Integer::from(0x10000),
    ///         -Integer::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     -524287000000000000i64
    /// );
    /// ```
    #[inline]
    fn add_mul_shl(mut self, y: Self, z: Self, bits: u64) -> Self {
        self.add_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'b> AddMulShl<Self, &'b Self> for Integer {
    type Output = Self;

    /// Adds an [`Integer`] and the product of two other [`Integer`]s, shifted left by `bits`,
    /// taking the first [`Integer`] by value, the second by value, and the third by reference.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10).add_mul_shl(Integer::from(3), &Integer::from(4), 2),
    ///     58
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).add_mul_shl(Integer::from(3), &Integer::from(-4), 2),
    ///     -58
    /// );
    /// assert_eq!(
    ///     Integer::from(10u32).pow(12).add_mul_shl(
    ///         Integer::from(0x10000),
    ///         &-Integer::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     -524287000000000000i64
    /// );
    /// ```
    #[inline]
    fn add_mul_shl(mut self, y: Self, z: &'b Self, bits: u64) -> Self {
        self.add_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'a> AddMulShl<&'a Self, Self> for Integer {
    type Output = Self;

    /// Adds an [`Integer`] and the product of two other [`Integer`]s, shifted left by `bits`,
    /// taking the first [`Integer`] by value, the second by reference, and the third by value.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10).add_mul_shl(&Integer::from(3), Integer::from(4), 2),
    ///     58
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).add_mul_shl(&Integer::from(3), Integer::from(-4), 2),
    ///     -58
    /// );
    /// assert_eq!(
    ///     Integer::from(10u32).pow(12).add_mul_shl(
    ///         &Integer::from(0x10000),
    ///         -Integer::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     -524287000000000000i64
    /// );
    /// ```
    #[inline]
    fn add_mul_shl(mut self, y: &'a Self, z: Self, bits: u64) -> Self {
        self.add_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'a, 'b> AddMulShl<&'a Self, &'b Self> for Integer {
    type Output = Self;

    /// Adds an [`Integer`] and the product of two other [`Integer`]s, shifted left by `bits`,
    /// taking the first [`Integer`] by value, the second by reference, and the third by reference.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10).add_mul_shl(&Integer::from(3), &Integer::from(4), 2),
    ///     58
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).add_mul_shl(&Integer::from(3), &Integer::from(-4), 2),
    ///     -58
    /// );
    /// assert_eq!(
    ///     Integer::from(10u32).pow(12).add_mul_shl(
    ///         &Integer::from(0x10000),
    ///         &-Integer::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     -524287000000000000i64
    /// );
    /// ```
    #[inline]
    fn add_mul_shl(mut self, y: &'a Self, z: &'b Self, bits: u64) -> Self {
        self.add_mul_shl_assign(y, z, bits);
        self
    }
}

impl AddMulShl<&Integer, &Integer> for &Integer {
    type Output = Integer;

    /// Adds an [`Integer`] and the product of two other [`Integer`]s, shifted left by `bits`,
    /// taking all three by reference.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(10)).add_mul_shl(&Integer::from(3), &Integer::from(4), 2),
    ///     58
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).add_mul_shl(&Integer::from(3), &Integer::from(-4), 2),
    ///     -58
    /// );
    /// assert_eq!(
    ///     (&Integer::from(10u32).pow(12)).add_mul_shl(
    ///         &Integer::from(0x10000),
    ///         &-Integer::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     -524287000000000000i64
    /// );
    /// ```
    fn add_mul_shl(self, y: &Integer, z: &Integer, bits: u64) -> Integer {
        if bits == 0 {
            self.add_mul(y, z)
        } else {
            self + ((y * z) << bits)
        }
    }
}

impl AddMulShlAssign<Self, Self> for Integer {
    /// Adds an [`Integer`] and the product of two other [`Integer`]s, shifted left by `bits`, in
    /// place, taking both [`Integer`]s on the right-hand side by value.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10);
    /// x.add_mul_shl_assign(Integer::from(3), Integer::from(4), 2);
    /// assert_eq!(x, 58);
    ///
    /// let mut x = Integer::from(-10);
    /// x.add_mul_shl_assign(Integer::from(3), Integer::from(-4), 2);
    /// assert_eq!(x, -58);
    ///
    /// let mut x = Integer::from(10u32).pow(12);
    /// x.add_mul_shl_assign(Integer::from(0x10000), -Integer::from(10u32).pow(12), 3);
    /// assert_eq!(x, -524287000000000000i64);
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

impl<'b> AddMulShlAssign<Self, &'b Self> for Integer {
    /// Adds an [`Integer`] and the product of two other [`Integer`]s, shifted left by `bits`, in
    /// place, taking the first [`Integer`] on the right-hand side by value and the second by
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
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10);
    /// x.add_mul_shl_assign(Integer::from(3), &Integer::from(4), 2);
    /// assert_eq!(x, 58);
    ///
    /// let mut x = Integer::from(-10);
    /// x.add_mul_shl_assign(Integer::from(3), &Integer::from(-4), 2);
    /// assert_eq!(x, -58);
    ///
    /// let mut x = Integer::from(10u32).pow(12);
    /// x.add_mul_shl_assign(Integer::from(0x10000), &-Integer::from(10u32).pow(12), 3);
    /// assert_eq!(x, -524287000000000000i64);
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

impl<'a> AddMulShlAssign<&'a Self, Self> for Integer {
    /// Adds an [`Integer`] and the product of two other [`Integer`]s, shifted left by `bits`, in
    /// place, taking the first [`Integer`] on the right-hand side by reference and the second by
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
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10);
    /// x.add_mul_shl_assign(&Integer::from(3), Integer::from(4), 2);
    /// assert_eq!(x, 58);
    ///
    /// let mut x = Integer::from(-10);
    /// x.add_mul_shl_assign(&Integer::from(3), Integer::from(-4), 2);
    /// assert_eq!(x, -58);
    ///
    /// let mut x = Integer::from(10u32).pow(12);
    /// x.add_mul_shl_assign(&Integer::from(0x10000), -Integer::from(10u32).pow(12), 3);
    /// assert_eq!(x, -524287000000000000i64);
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

impl<'a, 'b> AddMulShlAssign<&'a Self, &'b Self> for Integer {
    /// Adds an [`Integer`] and the product of two other [`Integer`]s, shifted left by `bits`, in
    /// place, taking both [`Integer`]s on the right-hand side by reference.
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
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10);
    /// x.add_mul_shl_assign(&Integer::from(3), &Integer::from(4), 2);
    /// assert_eq!(x, 58);
    ///
    /// let mut x = Integer::from(-10);
    /// x.add_mul_shl_assign(&Integer::from(3), &Integer::from(-4), 2);
    /// assert_eq!(x, -58);
    ///
    /// let mut x = Integer::from(10u32).pow(12);
    /// x.add_mul_shl_assign(&Integer::from(0x10000), &-Integer::from(10u32).pow(12), 3);
    /// assert_eq!(x, -524287000000000000i64);
    /// ```
    fn add_mul_shl_assign(&mut self, y: &'a Self, z: &'b Self, bits: u64) {
        if bits == 0 {
            self.add_mul_assign(y, z);
        } else {
            *self += (y * z) << bits;
        }
    }
}
