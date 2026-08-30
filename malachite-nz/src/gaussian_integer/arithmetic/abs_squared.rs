// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_integer::GaussianInteger;
use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{AbsSquared, Square};

impl AbsSquared for GaussianInteger {
    type Output = Integer;

    /// Computes the squared absolute value of a [`GaussianInteger`], taking it by value. This is
    /// the sum of the squares of the real and imaginary parts, also known as the norm. It is always
    /// a non-negative [`Integer`].
    ///
    /// $$
    /// f(x) = |x|^2 = \Re(x)^2 + \Im(x)^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsSquared;
    /// use malachite_base::num::basic::traits::{I, Zero};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianInteger::ZERO.abs_squared(), 0);
    /// assert_eq!(GaussianInteger::I.abs_squared(), 1);
    /// assert_eq!(GaussianInteger::from_str("2-3i").unwrap().abs_squared(), 13);
    /// assert_eq!(GaussianInteger::from(-123).abs_squared(), 15129);
    /// ```
    #[inline]
    fn abs_squared(self) -> Integer {
        self.real.square() + self.imaginary.square()
    }
}

impl AbsSquared for &GaussianInteger {
    type Output = Integer;

    /// Computes the squared absolute value of a [`GaussianInteger`], taking it by reference. This
    /// is the sum of the squares of the real and imaginary parts, also known as the norm. It is
    /// always a non-negative [`Integer`].
    ///
    /// $$
    /// f(x) = |x|^2 = \Re(x)^2 + \Im(x)^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the maximum number of significant
    /// bits of the real and imaginary parts.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsSquared;
    /// use malachite_base::num::basic::traits::{I, Zero};
    /// use malachite_nz::gaussian_integer::GaussianInteger;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&GaussianInteger::ZERO).abs_squared(), 0);
    /// assert_eq!((&GaussianInteger::I).abs_squared(), 1);
    /// let x = GaussianInteger::from_str("2-3i").unwrap();
    /// assert_eq!((&x).abs_squared(), 13);
    /// ```
    #[inline]
    fn abs_squared(self) -> Integer {
        (&self.real).square() + (&self.imaginary).square()
    }
}
