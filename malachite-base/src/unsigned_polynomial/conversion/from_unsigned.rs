// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::traits::Zero;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_polynomial::UnsignedPolynomial;
use alloc::vec;

impl<T: PrimitiveUnsigned> From<T> for UnsignedPolynomial<T> {
    /// Converts a value to a constant [`UnsignedPolynomial`].
    ///
    /// This works for anything a [`u64`] can be converted from, and for a [`u64`] itself. The
    /// polynomial is the constant one, whose only coefficient is the value; zero becomes the zero
    /// polynomial, which has no coefficients at all.
    ///
    /// $f(x) = x$, read on the left as a number and on the right as a polynomial.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(UnsignedPolynomial::<u64>::from(123u64).to_string(), "123");
    /// assert_eq!(UnsignedPolynomial::<u64>::from(123u64).to_string(), "123");
    /// assert_eq!(UnsignedPolynomial::<u64>::from(true).to_string(), "1");
    ///
    /// // Zero is the zero polynomial, which has no coefficients.
    /// assert_eq!(
    ///     UnsignedPolynomial::<u64>::from(0u64),
    ///     UnsignedPolynomial::<u64>::default()
    /// );
    /// ```
    #[inline]
    fn from(x: T) -> Self {
        Self::from_coefficients_asc(vec![x])
    }
}

impl<T: PrimitiveUnsigned> From<bool> for UnsignedPolynomial<T> {
    /// Converts a [`bool`] to an [`UnsignedPolynomial`]: the constant polynomial 0 or 1.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(UnsignedPolynomial::<u64>::from(false).to_string(), "0");
    /// assert_eq!(UnsignedPolynomial::<u64>::from(true).to_string(), "1");
    /// ```
    #[inline]
    fn from(b: bool) -> Self {
        if b { Self::one() } else { Self::ZERO }
    }
}
