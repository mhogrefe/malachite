// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::u64_polynomial::U64Polynomial;
use alloc::vec;

impl<T: Into<u64>> From<T> for U64Polynomial {
    /// Converts a value to a constant [`U64Polynomial`].
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
    /// use malachite_base::u64_polynomial::U64Polynomial;
    ///
    /// assert_eq!(U64Polynomial::from(123u32).to_string(), "123");
    /// assert_eq!(U64Polynomial::from(123u64).to_string(), "123");
    /// assert_eq!(U64Polynomial::from(true).to_string(), "1");
    ///
    /// // Zero is the zero polynomial, which has no coefficients.
    /// assert_eq!(U64Polynomial::from(0u32), U64Polynomial::default());
    /// ```
    #[inline]
    fn from(x: T) -> Self {
        Self::from_coefficients_asc(vec![x.into()])
    }
}
