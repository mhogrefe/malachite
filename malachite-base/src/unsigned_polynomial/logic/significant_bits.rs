// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::logic::traits::SignificantBits;
use crate::unsigned_polynomial::UnsignedPolynomial;

impl<T: PrimitiveUnsigned> SignificantBits for &UnsignedPolynomial<T> {
    /// Returns the sum of the numbers of significant bits of the coefficients of an
    /// [`UnsignedPolynomial`].
    ///
    /// This is the number of bits needed to store all of the coefficients, and 0 for the zero
    /// polynomial. Zero coefficients below the leading one contribute nothing.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the number of coefficients.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_base::unsigned_polynomial::UnsignedPolynomial;
    ///
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("5*x^2+2*x+1")
    ///         .unwrap()
    ///         .significant_bits(),
    ///     6
    /// );
    /// assert_eq!(
    ///     UnsignedPolynomial::<u8>::from_str("0")
    ///         .unwrap()
    ///         .significant_bits(),
    ///     0
    /// );
    /// ```
    #[inline]
    fn significant_bits(self) -> u64 {
        self.coefficients
            .iter()
            .map(|&c| c.significant_bits())
            .sum()
    }
}
