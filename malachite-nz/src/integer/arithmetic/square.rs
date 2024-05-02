// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};

impl Square for Integer {
    type Output = Integer;

    /// Squares an [`Integer`], taking it by value.
    ///
    /// $$
    /// f(x) = x^2.
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
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::ZERO.square(), 0);
    /// assert_eq!(Integer::from(123).square(), 15129);
    /// assert_eq!(Integer::from(-123).square(), 15129);
    /// ```
    #[inline]
    fn square(mut self) -> Integer {
        self.square_assign();
        self
    }
}

impl<'a> Square for &'a Integer {
    type Output = Integer;

    /// Squares an [`Integer`], taking it by reference.
    ///
    /// $$
    /// f(x) = x^2.
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
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::ZERO).square(), 0);
    /// assert_eq!((&Integer::from(123)).square(), 15129);
    /// assert_eq!((&Integer::from(-123)).square(), 15129);
    /// ```
    #[inline]
    fn square(self) -> Integer {
        Integer {
            sign: true,
            abs: (&self.abs).square(),
        }
    }
}

impl SquareAssign for Integer {
    /// Squares an [`Integer`] in place.
    ///
    /// $$
    /// x \gets x^2.
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
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::ZERO;
    /// x.square_assign();
    /// assert_eq!(x, 0);
    ///
    /// let mut x = Integer::from(123);
    /// x.square_assign();
    /// assert_eq!(x, 15129);
    ///
    /// let mut x = Integer::from(-123);
    /// x.square_assign();
    /// assert_eq!(x, 15129);
    /// ```
    fn square_assign(&mut self) {
        self.sign = true;
        self.abs.square_assign();
    }
}
