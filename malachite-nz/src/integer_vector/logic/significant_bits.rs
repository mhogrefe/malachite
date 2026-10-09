// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_vector::IntegerVector;
use malachite_base::num::logic::traits::SignificantBits;

impl SignificantBits for &IntegerVector {
    /// Returns the sum of the numbers of significant bits of the elements of an [`IntegerVector`].
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(IntegerVector::from_str("(1, -5, 2)").unwrap().significant_bits(), 6);
    /// assert_eq!(IntegerVector::from_str("()").unwrap().significant_bits(), 0);
    /// ```
    #[inline]
    fn significant_bits(self) -> u64 {
        self.elements
            .iter()
            .map(SignificantBits::significant_bits)
            .sum()
    }
}
