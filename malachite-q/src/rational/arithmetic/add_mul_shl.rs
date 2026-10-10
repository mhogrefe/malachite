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
use malachite_base::num::arithmetic::traits::{AddMulShl, AddMulShlAssign};

impl AddMulShl<Self, Self> for Rational {
    type Output = Self;

    /// Adds a [`Rational`] and the product of two other [`Rational`]s, shifted left by `bits`,
    /// taking all three by value.
    ///
    /// $f(x, y, z, k) = x + yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::AddMulShl;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .add_mul_shl(
    ///             Rational::from_signeds(2, 3),
    ///             Rational::from_signeds(3, 4),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "5/2"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .add_mul_shl(
    ///             Rational::from_signeds(-1, 2),
    ///             Rational::from_signeds(1, 3),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "38/21"
    /// );
    /// ```
    #[inline]
    fn add_mul_shl(mut self, y: Self, z: Self, bits: u64) -> Self {
        self.add_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'b> AddMulShl<Self, &'b Self> for Rational {
    type Output = Self;

    /// Adds a [`Rational`] and the product of two other [`Rational`]s, shifted left by `bits`,
    /// taking the first [`Rational`] by value, the second by value, and the third by reference.
    ///
    /// $f(x, y, z, k) = x + yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::AddMulShl;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .add_mul_shl(
    ///             Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "5/2"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .add_mul_shl(
    ///             Rational::from_signeds(-1, 2),
    ///             &Rational::from_signeds(1, 3),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "38/21"
    /// );
    /// ```
    #[inline]
    fn add_mul_shl(mut self, y: Self, z: &'b Self, bits: u64) -> Self {
        self.add_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'a> AddMulShl<&'a Self, Self> for Rational {
    type Output = Self;

    /// Adds a [`Rational`] and the product of two other [`Rational`]s, shifted left by `bits`,
    /// taking the first [`Rational`] by value, the second by reference, and the third by value.
    ///
    /// $f(x, y, z, k) = x + yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::AddMulShl;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .add_mul_shl(
    ///             &Rational::from_signeds(2, 3),
    ///             Rational::from_signeds(3, 4),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "5/2"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .add_mul_shl(
    ///             &Rational::from_signeds(-1, 2),
    ///             Rational::from_signeds(1, 3),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "38/21"
    /// );
    /// ```
    #[inline]
    fn add_mul_shl(mut self, y: &'a Self, z: Self, bits: u64) -> Self {
        self.add_mul_shl_assign(y, z, bits);
        self
    }
}

impl<'a, 'b> AddMulShl<&'a Self, &'b Self> for Rational {
    type Output = Self;

    /// Adds a [`Rational`] and the product of two other [`Rational`]s, shifted left by `bits`,
    /// taking the first [`Rational`] by value, the second by reference, and the third by reference.
    ///
    /// $f(x, y, z, k) = x + yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::AddMulShl;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     Rational::ONE_HALF
    ///         .add_mul_shl(
    ///             &Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "5/2"
    /// );
    /// assert_eq!(
    ///     Rational::from_signeds(22, 7)
    ///         .add_mul_shl(
    ///             &Rational::from_signeds(-1, 2),
    ///             &Rational::from_signeds(1, 3),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "38/21"
    /// );
    /// ```
    #[inline]
    fn add_mul_shl(mut self, y: &'a Self, z: &'b Self, bits: u64) -> Self {
        self.add_mul_shl_assign(y, z, bits);
        self
    }
}

impl AddMulShl<&Rational, &Rational> for &Rational {
    type Output = Rational;

    /// Adds a [`Rational`] and the product of two other [`Rational`]s, shifted left by `bits`,
    /// taking all three by reference.
    ///
    /// $f(x, y, z, k) = x + yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::AddMulShl;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// assert_eq!(
    ///     (&Rational::ONE_HALF)
    ///         .add_mul_shl(
    ///             &Rational::from_signeds(2, 3),
    ///             &Rational::from_signeds(3, 4),
    ///             2
    ///         )
    ///         .to_string(),
    ///     "5/2"
    /// );
    /// assert_eq!(
    ///     (&Rational::from_signeds(22, 7))
    ///         .add_mul_shl(
    ///             &Rational::from_signeds(-1, 2),
    ///             &Rational::from_signeds(1, 3),
    ///             3
    ///         )
    ///         .to_string(),
    ///     "38/21"
    /// );
    /// ```
    #[inline]
    fn add_mul_shl(self, y: &Rational, z: &Rational, bits: u64) -> Rational {
        self + ((y * z) << bits)
    }
}

impl AddMulShlAssign<Self, Self> for Rational {
    /// Adds a [`Rational`] and the product of two other [`Rational`]s, shifted left by `bits`, in
    /// place, taking both [`Rational`]s on the right-hand side by value.
    ///
    /// $x \gets x + yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::AddMulShlAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.add_mul_shl_assign(
    ///     Rational::from_signeds(2, 3),
    ///     Rational::from_signeds(3, 4),
    ///     2,
    /// );
    /// assert_eq!(x.to_string(), "5/2");
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.add_mul_shl_assign(
    ///     Rational::from_signeds(-1, 2),
    ///     Rational::from_signeds(1, 3),
    ///     3,
    /// );
    /// assert_eq!(x.to_string(), "38/21");
    /// ```
    #[inline]
    fn add_mul_shl_assign(&mut self, mut y: Self, z: Self, bits: u64) {
        y *= z;
        y <<= bits;
        *self += y;
    }
}

impl<'b> AddMulShlAssign<Self, &'b Self> for Rational {
    /// Adds a [`Rational`] and the product of two other [`Rational`]s, shifted left by `bits`, in
    /// place, taking the first [`Rational`] on the right-hand side by value and the second by
    /// reference.
    ///
    /// $x \gets x + yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::AddMulShlAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.add_mul_shl_assign(
    ///     Rational::from_signeds(2, 3),
    ///     &Rational::from_signeds(3, 4),
    ///     2,
    /// );
    /// assert_eq!(x.to_string(), "5/2");
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.add_mul_shl_assign(
    ///     Rational::from_signeds(-1, 2),
    ///     &Rational::from_signeds(1, 3),
    ///     3,
    /// );
    /// assert_eq!(x.to_string(), "38/21");
    /// ```
    #[inline]
    fn add_mul_shl_assign(&mut self, mut y: Self, z: &'b Self, bits: u64) {
        y *= z;
        y <<= bits;
        *self += y;
    }
}

impl<'a> AddMulShlAssign<&'a Self, Self> for Rational {
    /// Adds a [`Rational`] and the product of two other [`Rational`]s, shifted left by `bits`, in
    /// place, taking the first [`Rational`] on the right-hand side by reference and the second by
    /// value.
    ///
    /// $x \gets x + yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::AddMulShlAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.add_mul_shl_assign(
    ///     &Rational::from_signeds(2, 3),
    ///     Rational::from_signeds(3, 4),
    ///     2,
    /// );
    /// assert_eq!(x.to_string(), "5/2");
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.add_mul_shl_assign(
    ///     &Rational::from_signeds(-1, 2),
    ///     Rational::from_signeds(1, 3),
    ///     3,
    /// );
    /// assert_eq!(x.to_string(), "38/21");
    /// ```
    #[inline]
    fn add_mul_shl_assign(&mut self, y: &'a Self, mut z: Self, bits: u64) {
        z *= y;
        z <<= bits;
        *self += z;
    }
}

impl<'a, 'b> AddMulShlAssign<&'a Self, &'b Self> for Rational {
    /// Adds a [`Rational`] and the product of two other [`Rational`]s, shifted left by `bits`, in
    /// place, taking both [`Rational`]s on the right-hand side by reference.
    ///
    /// $x \gets x + yz2^k$.
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
    /// use malachite_base::num::arithmetic::traits::AddMulShlAssign;
    /// use malachite_base::num::basic::traits::OneHalf;
    /// use malachite_q::Rational;
    ///
    /// let mut x = Rational::ONE_HALF;
    /// x.add_mul_shl_assign(
    ///     &Rational::from_signeds(2, 3),
    ///     &Rational::from_signeds(3, 4),
    ///     2,
    /// );
    /// assert_eq!(x.to_string(), "5/2");
    ///
    /// let mut x = Rational::from_signeds(22, 7);
    /// x.add_mul_shl_assign(
    ///     &Rational::from_signeds(-1, 2),
    ///     &Rational::from_signeds(1, 3),
    ///     3,
    /// );
    /// assert_eq!(x.to_string(), "38/21");
    /// ```
    #[inline]
    fn add_mul_shl_assign(&mut self, y: &'a Self, z: &'b Self, bits: u64) {
        *self += (y * z) << bits;
    }
}
