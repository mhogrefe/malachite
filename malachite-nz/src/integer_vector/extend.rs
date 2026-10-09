// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;

impl Extend<Integer> for IntegerVector {
    /// Appends the elements produced by an iterator to the end of an [`IntegerVector`], increasing
    /// its dimension by their number.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()` plus the number
    /// of elements produced, not counting the cost of producing them.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// let mut v = IntegerVector::from_str("(1, 2)").unwrap();
    /// v.extend([-3, 4].map(Integer::from));
    /// assert_eq!(v.to_string(), "(1, 2, -3, 4)");
    /// ```
    #[inline]
    fn extend<I: IntoIterator<Item = Integer>>(&mut self, xs: I) {
        self.elements.extend(xs);
    }
}
