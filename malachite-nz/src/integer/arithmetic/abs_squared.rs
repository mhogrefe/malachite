// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{AbsSquared, AbsSquaredAssign, Square, SquareAssign};

impl AbsSquared for Integer {
    type Output = Self;

    /// Computes the squared absolute value of a [`Integer`], taking it by value. For real types
    /// this is the same as squaring.
    ///
    /// $$
    /// f(x) = |x|^2 = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsSquared;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.abs_squared(), 0);
    /// assert_eq!(Integer::from(123).abs_squared(), 15129);
    /// assert_eq!(Integer::from(-123).abs_squared(), 15129);
    /// ```
    #[inline]
    fn abs_squared(self) -> Self {
        self.square()
    }
}

impl AbsSquared for &Integer {
    type Output = Integer;

    /// Computes the squared absolute value of a [`Integer`], taking it by reference. For real types
    /// this is the same as squaring.
    ///
    /// $$
    /// f(x) = |x|^2 = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsSquared;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::ZERO).abs_squared(), 0);
    /// assert_eq!((&Integer::from(123)).abs_squared(), 15129);
    /// assert_eq!((&Integer::from(-123)).abs_squared(), 15129);
    /// ```
    #[inline]
    fn abs_squared(self) -> Integer {
        self.square()
    }
}

impl AbsSquaredAssign for Integer {
    /// Replaces a [`Integer`] with its squared absolute value. For real types this is the same as
    /// squaring in place.
    ///
    /// $$
    /// x \gets |x|^2 = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsSquaredAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(-123);
    /// x.abs_squared_assign();
    /// assert_eq!(x, 15129);
    /// ```
    #[inline]
    fn abs_squared_assign(&mut self) {
        self.square_assign();
    }
}
