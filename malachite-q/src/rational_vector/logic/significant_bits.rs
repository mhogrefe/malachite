// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_vector::RationalVector;
use malachite_base::num::logic::traits::SignificantBits;

impl SignificantBits for &RationalVector {
    /// Returns the sum of the numbers of significant bits of the elements of a [`RationalVector`],
    /// each element's count being the sum of the bits of its numerator and denominator, as for
    /// [`Rational`](crate::Rational#impl-SignificantBits-for-%26Rational).
    ///
    /// This is the number of bits needed to store all of the elements, and 0 for the
    /// 0-dimensional vector.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::logic::traits::SignificantBits;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(RationalVector::from_str("(1/2, -1/3)").unwrap().significant_bits(), 6);
    /// assert_eq!(RationalVector::from_str("()").unwrap().significant_bits(), 0);
    /// ```
    #[inline]
    fn significant_bits(self) -> u64 {
        self.elements
            .iter()
            .map(SignificantBits::significant_bits)
            .sum()
    }
}
