// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use malachite_base::num::arithmetic::traits::{AbsSquared, AbsSquaredAssign, Square, SquareAssign};

impl AbsSquared for Float {
    type Output = Self;

    /// Computes the squared absolute value of a [`Float`], taking it by value. For real types this
    /// is the same as squaring: the output has the precision of the input and is rounded to
    /// nearest. See [`Float::square`] for more details, and the `square_prec` and `square_round`
    /// families for more control over the result.
    ///
    /// $$
    /// f(x) = |x|^2 = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsSquared;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert_eq!(Float::NAN.abs_squared().to_string(), "NaN");
    /// assert_eq!(Float::INFINITY.abs_squared().to_string(), "Infinity");
    /// assert_eq!(Float::NEGATIVE_INFINITY.abs_squared().to_string(), "Infinity");
    /// assert_eq!(Float::from(4.0).abs_squared().to_string(), "16.0");
    /// assert_eq!(Float::from(-1.5).abs_squared().to_string(), "2.0");
    /// ```
    #[inline]
    fn abs_squared(self) -> Self {
        self.square()
    }
}

impl AbsSquared for &Float {
    type Output = Float;

    /// Computes the squared absolute value of a [`Float`], taking it by reference. For real types
    /// this is the same as squaring: the output has the precision of the input and is rounded to
    /// nearest. See [`Float::square`] for more details, and the `square_prec` and `square_round`
    /// families for more control over the result.
    ///
    /// $$
    /// f(x) = |x|^2 = x^2.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::AbsSquared;
    /// use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity};
    /// use malachite_float::Float;
    ///
    /// assert_eq!((&Float::NAN).abs_squared().to_string(), "NaN");
    /// assert_eq!((&Float::INFINITY).abs_squared().to_string(), "Infinity");
    /// assert_eq!((&Float::NEGATIVE_INFINITY).abs_squared().to_string(), "Infinity");
    /// assert_eq!((&Float::from(4.0)).abs_squared().to_string(), "16.0");
    /// assert_eq!((&Float::from(-1.5)).abs_squared().to_string(), "2.0");
    /// ```
    #[inline]
    fn abs_squared(self) -> Float {
        self.square()
    }
}

impl AbsSquaredAssign for Float {
    /// Replaces a [`Float`] with its squared absolute value. For real types this is the same as
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
    /// use malachite_float::Float;
    ///
    /// let mut x = Float::from(-1.5);
    /// x.abs_squared_assign();
    /// assert_eq!(x.to_string(), "2.0");
    /// ```
    #[inline]
    fn abs_squared_assign(&mut self) {
        self.square_assign();
    }
}
