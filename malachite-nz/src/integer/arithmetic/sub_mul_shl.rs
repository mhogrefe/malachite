// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{SubMul, SubMulAssign, SubMulShl, SubMulShlAssign};

impl SubMulShl<Self, Self> for Integer {
    type Output = Self;

    /// Subtracts the product of two [`Integer`]s, shifted left by `bits`, from an [`Integer`],
    /// taking all three by value.
    ///
    /// $f(x, y, z, k) = x - yz2^k$.
    ///
    /// When `bits` is 0, this is [`SubMul`].
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
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulShl};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10).sub_mul_shl(Integer::from(3), Integer::from(4), 2),
    ///     -38
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).sub_mul_shl(Integer::from(3), Integer::from(-4), 2),
    ///     38
    /// );
    /// assert_eq!(
    ///     Integer::from(10u32).pow(12).sub_mul_shl(
    ///         Integer::from(0x10000),
    ///         -Integer::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     524289000000000000i64
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl(mut self, y: Self, z: Self, bits: u64) -> Self {
        self.sub_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'b> SubMulShl<Self, &'b Self> for Integer {
    type Output = Self;

    /// Subtracts the product of two [`Integer`]s, shifted left by `bits`, from an [`Integer`],
    /// taking the first [`Integer`] by value, the second by value, and the third by reference.
    ///
    /// $f(x, y, z, k) = x - yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulShl};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10).sub_mul_shl(Integer::from(3), &Integer::from(4), 2),
    ///     -38
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).sub_mul_shl(Integer::from(3), &Integer::from(-4), 2),
    ///     38
    /// );
    /// assert_eq!(
    ///     Integer::from(10u32).pow(12).sub_mul_shl(
    ///         Integer::from(0x10000),
    ///         &-Integer::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     524289000000000000i64
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl(mut self, y: Self, z: &'b Self, bits: u64) -> Self {
        self.sub_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'a> SubMulShl<&'a Self, Self> for Integer {
    type Output = Self;

    /// Subtracts the product of two [`Integer`]s, shifted left by `bits`, from an [`Integer`],
    /// taking the first [`Integer`] by value, the second by reference, and the third by value.
    ///
    /// $f(x, y, z, k) = x - yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulShl};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10).sub_mul_shl(&Integer::from(3), Integer::from(4), 2),
    ///     -38
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).sub_mul_shl(&Integer::from(3), Integer::from(-4), 2),
    ///     38
    /// );
    /// assert_eq!(
    ///     Integer::from(10u32).pow(12).sub_mul_shl(
    ///         &Integer::from(0x10000),
    ///         -Integer::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     524289000000000000i64
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl(mut self, y: &'a Self, z: Self, bits: u64) -> Self {
        self.sub_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'a, 'b> SubMulShl<&'a Self, &'b Self> for Integer {
    type Output = Self;

    /// Subtracts the product of two [`Integer`]s, shifted left by `bits`, from an [`Integer`],
    /// taking the first [`Integer`] by value, the second by reference, and the third by reference.
    ///
    /// $f(x, y, z, k) = x - yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulShl};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10).sub_mul_shl(&Integer::from(3), &Integer::from(4), 2),
    ///     -38
    /// );
    /// assert_eq!(
    ///     Integer::from(-10).sub_mul_shl(&Integer::from(3), &Integer::from(-4), 2),
    ///     38
    /// );
    /// assert_eq!(
    ///     Integer::from(10u32).pow(12).sub_mul_shl(
    ///         &Integer::from(0x10000),
    ///         &-Integer::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     524289000000000000i64
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl(mut self, y: &'a Self, z: &'b Self, bits: u64) -> Self {
        self.sub_mul_shl_assign(y, z, bits);
        self
    }
}

impl SubMulShl<&Integer, &Integer> for &Integer {
    type Output = Integer;

    /// Subtracts the product of two [`Integer`]s, shifted left by `bits`, from an [`Integer`],
    /// taking all three by reference.
    ///
    /// $f(x, y, z, k) = x - yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulShl};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(10)).sub_mul_shl(&Integer::from(3), &Integer::from(4), 2),
    ///     -38
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-10)).sub_mul_shl(&Integer::from(3), &Integer::from(-4), 2),
    ///     38
    /// );
    /// assert_eq!(
    ///     (&Integer::from(10u32).pow(12)).sub_mul_shl(
    ///         &Integer::from(0x10000),
    ///         &-Integer::from(10u32).pow(12),
    ///         3
    ///     ),
    ///     524289000000000000i64
    /// );
    /// ```
    fn sub_mul_shl(self, y: &Integer, z: &Integer, bits: u64) -> Integer {
        if bits == 0 {
            self.sub_mul(y, z)
        } else {
            self - ((y * z) << bits)
        }
    }
}

impl SubMulShlAssign<Self, Self> for Integer {
    /// Subtracts the product of two [`Integer`]s, shifted left by `bits`, from an [`Integer`], in
    /// place, taking both [`Integer`]s on the right-hand side by value.
    ///
    /// $x \gets x - yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulShlAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10);
    /// x.sub_mul_shl_assign(Integer::from(3), Integer::from(4), 2);
    /// assert_eq!(x, -38);
    ///
    /// let mut x = Integer::from(-10);
    /// x.sub_mul_shl_assign(Integer::from(3), Integer::from(-4), 2);
    /// assert_eq!(x, 38);
    ///
    /// let mut x = Integer::from(10u32).pow(12);
    /// x.sub_mul_shl_assign(Integer::from(0x10000), -Integer::from(10u32).pow(12), 3);
    /// assert_eq!(x, 524289000000000000i64);
    /// ```
    fn sub_mul_shl_assign(&mut self, mut y: Self, z: Self, bits: u64) {
        if bits == 0 {
            self.sub_mul_assign(y, z);
        } else {
            y *= z;
            y <<= bits;
            *self -= y;
        }
    }
}

impl<'b> SubMulShlAssign<Self, &'b Self> for Integer {
    /// Subtracts the product of two [`Integer`]s, shifted left by `bits`, from an [`Integer`], in
    /// place, taking the first [`Integer`] on the right-hand side by value and the second by
    /// reference.
    ///
    /// $x \gets x - yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulShlAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10);
    /// x.sub_mul_shl_assign(Integer::from(3), &Integer::from(4), 2);
    /// assert_eq!(x, -38);
    ///
    /// let mut x = Integer::from(-10);
    /// x.sub_mul_shl_assign(Integer::from(3), &Integer::from(-4), 2);
    /// assert_eq!(x, 38);
    ///
    /// let mut x = Integer::from(10u32).pow(12);
    /// x.sub_mul_shl_assign(Integer::from(0x10000), &-Integer::from(10u32).pow(12), 3);
    /// assert_eq!(x, 524289000000000000i64);
    /// ```
    fn sub_mul_shl_assign(&mut self, mut y: Self, z: &'b Self, bits: u64) {
        if bits == 0 {
            self.sub_mul_assign(y, z);
        } else {
            y *= z;
            y <<= bits;
            *self -= y;
        }
    }
}

impl<'a> SubMulShlAssign<&'a Self, Self> for Integer {
    /// Subtracts the product of two [`Integer`]s, shifted left by `bits`, from an [`Integer`], in
    /// place, taking the first [`Integer`] on the right-hand side by reference and the second by
    /// value.
    ///
    /// $x \gets x - yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulShlAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10);
    /// x.sub_mul_shl_assign(&Integer::from(3), Integer::from(4), 2);
    /// assert_eq!(x, -38);
    ///
    /// let mut x = Integer::from(-10);
    /// x.sub_mul_shl_assign(&Integer::from(3), Integer::from(-4), 2);
    /// assert_eq!(x, 38);
    ///
    /// let mut x = Integer::from(10u32).pow(12);
    /// x.sub_mul_shl_assign(&Integer::from(0x10000), -Integer::from(10u32).pow(12), 3);
    /// assert_eq!(x, 524289000000000000i64);
    /// ```
    fn sub_mul_shl_assign(&mut self, y: &'a Self, mut z: Self, bits: u64) {
        if bits == 0 {
            self.sub_mul_assign(y, z);
        } else {
            z *= y;
            z <<= bits;
            *self -= z;
        }
    }
}

impl<'a, 'b> SubMulShlAssign<&'a Self, &'b Self> for Integer {
    /// Subtracts the product of two [`Integer`]s, shifted left by `bits`, from an [`Integer`], in
    /// place, taking both [`Integer`]s on the right-hand side by reference.
    ///
    /// $x \gets x - yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulShlAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10);
    /// x.sub_mul_shl_assign(&Integer::from(3), &Integer::from(4), 2);
    /// assert_eq!(x, -38);
    ///
    /// let mut x = Integer::from(-10);
    /// x.sub_mul_shl_assign(&Integer::from(3), &Integer::from(-4), 2);
    /// assert_eq!(x, 38);
    ///
    /// let mut x = Integer::from(10u32).pow(12);
    /// x.sub_mul_shl_assign(&Integer::from(0x10000), &-Integer::from(10u32).pow(12), 3);
    /// assert_eq!(x, 524289000000000000i64);
    /// ```
    fn sub_mul_shl_assign(&mut self, y: &'a Self, z: &'b Self, bits: u64) {
        if bits == 0 {
            self.sub_mul_assign(y, z);
        } else {
            *self -= (y * z) << bits;
        }
    }
}
