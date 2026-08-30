// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::gaussian_rational::GaussianRational;
use malachite_base::num::arithmetic::traits::{AbsSquared, Square};

impl AbsSquared for GaussianRational {
    type Output = Rational;

    /// Computes the squared absolute value of a [`GaussianRational`], taking it by value. This is
    /// the sum of the squares of the real and imaginary parts, also known as the norm. It is always
    /// a non-negative [`Rational`].
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
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianRational::ZERO.abs_squared(), 0);
    /// assert_eq!(GaussianRational::I.abs_squared(), 1);
    /// assert_eq!(GaussianRational::from_str("2-3i").unwrap().abs_squared(), 13);
    /// assert_eq!(
    ///     GaussianRational::from_str("2/3-5i/6").unwrap()
    ///         .abs_squared()
    ///         .to_string(),
    ///     "41/36"
    /// );
    /// ```
    #[inline]
    fn abs_squared(self) -> Rational {
        self.real.square() + self.imaginary.square()
    }
}

impl AbsSquared for &GaussianRational {
    type Output = Rational;

    /// Computes the squared absolute value of a [`GaussianRational`], taking it by reference. This
    /// is the sum of the squares of the real and imaginary parts, also known as the norm. It is
    /// always a non-negative [`Rational`].
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
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!((&GaussianRational::ZERO).abs_squared(), 0);
    /// assert_eq!((&GaussianRational::I).abs_squared(), 1);
    /// let x = GaussianRational::from_str("2/3-5i/6").unwrap();
    /// assert_eq!((&x).abs_squared().to_string(), "41/36");
    /// ```
    #[inline]
    fn abs_squared(self) -> Rational {
        (&self.real).square() + (&self.imaginary).square()
    }
}
