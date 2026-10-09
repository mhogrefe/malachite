// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;

impl Extend<Rational> for RationalVector {
    /// Appends the elements produced by an iterator to the end of a [`RationalVector`], increasing
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
    /// use malachite_q::Rational;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1, 2)").unwrap();
    /// v.extend([-3, 4].map(Rational::from));
    /// assert_eq!(v.to_string(), "(1, 2, -3, 4)");
    /// ```
    #[inline]
    fn extend<I: IntoIterator<Item = Rational>>(&mut self, xs: I) {
        self.elements.extend(xs);
    }
}
