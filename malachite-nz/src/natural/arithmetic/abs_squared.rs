// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{AbsSquared, AbsSquaredAssign, Square, SquareAssign};

impl AbsSquared for Natural {
    type Output = Self;

    /// Computes the squared absolute value of a [`Natural`], taking it by value. For real types
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::ZERO.abs_squared(), 0);
    /// assert_eq!(Natural::from(123u32).abs_squared(), 15129);
    /// ```
    #[inline]
    fn abs_squared(self) -> Self {
        self.square()
    }
}

impl AbsSquared for &Natural {
    type Output = Natural;

    /// Computes the squared absolute value of a [`Natural`], taking it by reference. For real types
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
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::ZERO).abs_squared(), 0);
    /// assert_eq!((&Natural::from(123u32)).abs_squared(), 15129);
    /// ```
    #[inline]
    fn abs_squared(self) -> Natural {
        self.square()
    }
}

impl AbsSquaredAssign for Natural {
    /// Replaces a [`Natural`] with its squared absolute value. For real types this is the same as
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
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(123u32);
    /// x.abs_squared_assign();
    /// assert_eq!(x, 15129);
    /// ```
    #[inline]
    fn abs_squared_assign(&mut self) {
        self.square_assign();
    }
}
