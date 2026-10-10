// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use malachite_base::num::arithmetic::traits::{SubMulShl, SubMulShlAssign};

impl SubMulShl<Self, Self> for Rational {
    type Output = Self;

    /// Subtracts the product of two [`Rational`]s, shifted left by `bits`, from a [`Rational`],
    /// taking all three by value.
    ///
    /// $f(x, y, z, k) = x - yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .sub_mul_shl(
    ///             Rational::from_signeds(2, 3),
    ///             Rational::from_signeds(3, 4),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "-3/2"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .sub_mul_shl(
    ///             Rational::from_signeds(-1, 2),
    ///             Rational::from_signeds(1, 3),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "94/21"
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl(mut self, y: Self, z: Self, bits: u64) -> Self {
        self.sub_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'b> SubMulShl<Self, &'b Self> for Rational {
    type Output = Self;

    /// Subtracts the product of two [`Rational`]s, shifted left by `bits`, from a [`Rational`],
    /// taking the first [`Rational`] by value, the second by value, and the third by reference.
    ///
    /// $f(x, y, z, k) = x - yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .sub_mul_shl(
    ///             Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "-3/2"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .sub_mul_shl(
    ///             Rational::from_signeds(-1, 2),
    ///             &Rational::from_signeds(1, 3),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "94/21"
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl(mut self, y: Self, z: &'b Self, bits: u64) -> Self {
        self.sub_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'a> SubMulShl<&'a Self, Self> for Rational {
    type Output = Self;

    /// Subtracts the product of two [`Rational`]s, shifted left by `bits`, from a [`Rational`],
    /// taking the first [`Rational`] by value, the second by reference, and the third by value.
    ///
    /// $f(x, y, z, k) = x - yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .sub_mul_shl(
    ///             &Rational::from_signeds(2, 3),
    ///             Rational::from_signeds(3, 4),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "-3/2"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .sub_mul_shl(
    ///             &Rational::from_signeds(-1, 2),
    ///             Rational::from_signeds(1, 3),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "94/21"
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl(mut self, y: &'a Self, z: Self, bits: u64) -> Self {
        self.sub_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'a, 'b> SubMulShl<&'a Self, &'b Self> for Rational {
    type Output = Self;

    /// Subtracts the product of two [`Rational`]s, shifted left by `bits`, from a [`Rational`],
    /// taking the first [`Rational`] by value, the second by reference, and the third by reference.
    ///
    /// $f(x, y, z, k) = x - yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .sub_mul_shl(
    ///             &Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "-3/2"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .sub_mul_shl(
    ///             &Rational::from_signeds(-1, 2),
    ///             &Rational::from_signeds(1, 3),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "94/21"
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl(mut self, y: &'a Self, z: &'b Self, bits: u64) -> Self {
        self.sub_mul_shl_assign(y, z, bits);
        self
    }
}

impl SubMulShl<&Rational, &Rational> for &Rational {
    type Output = Rational;

    /// Subtracts the product of two [`Rational`]s, shifted left by `bits`, from a [`Rational`],
    /// taking all three by reference.
    ///
    /// $f(x, y, z, k) = x - yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMulShl;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::ONE_HALF)
    ///         .sub_mul_shl(
    ///             &Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "-3/2"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7))
    ///         .sub_mul_shl(
    ///             &Rational::from_signeds(-1, 2),
    ///             &Rational::from_signeds(1, 3),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "94/21"
    /// );
    /// ```
    #[inline]
    fn sub_mul_shl(self, y: &Rational, z: &Rational, bits: u64) -> Rational {
        self - ((y * z) << bits)
    }
}

impl SubMulShlAssign<Self, Self> for Rational {
    /// Subtracts the product of two [`Rational`]s, shifted left by `bits`, from a [`Rational`], in
    /// place, taking both [`Rational`]s on the right-hand side by value.
    ///
    /// $x \gets x - yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMulShlAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.sub_mul_shl_assign(
    ///     Rational::from_signeds(2, 3),
    ///     Rational::from_signeds(3, 4),
    ///     2,
    /// );
    /// assert_eq!(x.to_string(), "-3/2");
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.sub_mul_shl_assign(
    ///     Rational::from_signeds(-1, 2),
    ///     Rational::from_signeds(1, 3),
    ///     3,
    /// );
    /// assert_eq!(x.to_string(), "94/21");
    /// ```
    #[inline]
    fn sub_mul_shl_assign(&mut self, mut y: Self, z: Self, bits: u64) {
        y *= z;
        y <<= bits;
        *self -= y;
    }
}

impl<'b> SubMulShlAssign<Self, &'b Self> for Rational {
    /// Subtracts the product of two [`Rational`]s, shifted left by `bits`, from a [`Rational`], in
    /// place, taking the first [`Rational`] on the right-hand side by value and the second by
    /// reference.
    ///
    /// $x \gets x - yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMulShlAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.sub_mul_shl_assign(
    ///     Rational::from_signeds(2, 3),
    ///     &Rational::from_signeds(3, 4),
    ///     2,
    /// );
    /// assert_eq!(x.to_string(), "-3/2");
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.sub_mul_shl_assign(
    ///     Rational::from_signeds(-1, 2),
    ///     &Rational::from_signeds(1, 3),
    ///     3,
    /// );
    /// assert_eq!(x.to_string(), "94/21");
    /// ```
    #[inline]
    fn sub_mul_shl_assign(&mut self, mut y: Self, z: &'b Self, bits: u64) {
        y *= z;
        y <<= bits;
        *self -= y;
    }
}

impl<'a> SubMulShlAssign<&'a Self, Self> for Rational {
    /// Subtracts the product of two [`Rational`]s, shifted left by `bits`, from a [`Rational`], in
    /// place, taking the first [`Rational`] on the right-hand side by reference and the second by
    /// value.
    ///
    /// $x \gets x - yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMulShlAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.sub_mul_shl_assign(
    ///     &Rational::from_signeds(2, 3),
    ///     Rational::from_signeds(3, 4),
    ///     2,
    /// );
    /// assert_eq!(x.to_string(), "-3/2");
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.sub_mul_shl_assign(
    ///     &Rational::from_signeds(-1, 2),
    ///     Rational::from_signeds(1, 3),
    ///     3,
    /// );
    /// assert_eq!(x.to_string(), "94/21");
    /// ```
    #[inline]
    fn sub_mul_shl_assign(&mut self, y: &'a Self, mut z: Self, bits: u64) {
        z *= y;
        z <<= bits;
        *self -= z;
    }
}

impl<'a, 'b> SubMulShlAssign<&'a Self, &'b Self> for Rational {
    /// Subtracts the product of two [`Rational`]s, shifted left by `bits`, from a [`Rational`], in
    /// place, taking both [`Rational`]s on the right-hand side by reference.
    ///
    /// $x \gets x - yz2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// y.significant_bits(), z.significant_bits(), bits)`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::SubMulShlAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.sub_mul_shl_assign(
    ///     &Rational::from_signeds(2, 3),
    ///     &Rational::from_signeds(3, 4),
    ///     2,
    /// );
    /// assert_eq!(x.to_string(), "-3/2");
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.sub_mul_shl_assign(
    ///     &Rational::from_signeds(-1, 2),
    ///     &Rational::from_signeds(1, 3),
    ///     3,
    /// );
    /// assert_eq!(x.to_string(), "94/21");
    /// ```
    #[inline]
    fn sub_mul_shl_assign(&mut self, y: &'a Self, z: &'b Self, bits: u64) {
        *self -= (y * z) << bits;
    }
}
