// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::gaussian_rational::GaussianRational;
use malachite_base::num::logic::traits::SignificantBits;

impl SignificantBits for &GaussianRational {
    /// Returns the sum of the numbers of significant bits of the real and imaginary parts of a
    /// [`GaussianRational`], where each part's count is the sum of the bits of its numerator and
    /// denominator, as for [`Rational`](crate::Rational#impl-SignificantBits-for-%26Rational).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::traits::Zero;
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_q::gaussian_rational::GaussianRational;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(GaussianRational::ZERO.significant_bits(), 2);
    /// assert_eq!(GaussianRational::from(100).significant_bits(), 9);
    /// assert_eq!(
    ///     GaussianRational::from_str("1/2+i/3")
    ///         .unwrap()
    ///         .significant_bits(),
    ///     6
    /// );
    /// assert_eq!(
    ///     GaussianRational::from_str("-100/101+i")
    ///         .unwrap()
    ///         .significant_bits(),
    ///     16
    /// );
    /// ```
    #[inline]
    fn significant_bits(self) -> u64 {
        self.real.significant_bits() + self.imaginary.significant_bits()
    }
}
