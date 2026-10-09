// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::{Abs, AbsAssign, EntrywiseAbs, EntrywiseAbsAssign};

impl EntrywiseAbs for RationalVector {
    type Output = Self;

    /// Replaces every element of a [`RationalVector`] by its absolute value, taking the vector by
    /// value.
    ///
    /// The dimension is unchanged. This is not [`Abs`], which for a vector would be its Euclidean
    /// length.
    ///
    /// $$
    /// f(v) = (|v_0|, |v_1|, \ldots, |v_{n-1}|).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseAbs;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 0, -3)").unwrap();
    /// assert_eq!(v.clone().entrywise_abs().to_string(), "(1/2, 2/3, 0, 3)");
    /// ```
    #[inline]
    fn entrywise_abs(mut self) -> Self {
        self.entrywise_abs_assign();
        self
    }
}

impl EntrywiseAbs for &RationalVector {
    type Output = RationalVector;

    /// Replaces every element of a [`RationalVector`] by its absolute value, taking the vector by
    /// reference.
    ///
    /// See the documentation for the [`EntrywiseAbs`] implementation on [`RationalVector`] that
    /// takes the vector by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseAbs;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let v = RationalVector::from_str("(1/2, -2/3, 0, -3)").unwrap();
    /// assert_eq!((&v).entrywise_abs().to_string(), "(1/2, 2/3, 0, 3)");
    /// ```
    fn entrywise_abs(self) -> RationalVector {
        RationalVector {
            elements: self.elements.iter().map(Abs::abs).collect(),
        }
    }
}

impl EntrywiseAbsAssign for RationalVector {
    /// Replaces every element of a [`RationalVector`] by its absolute value, in place.
    ///
    /// See the documentation for the [`EntrywiseAbs`] implementation on [`RationalVector`] that
    /// takes the vector by value for details.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EntrywiseAbsAssign;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// let mut v = RationalVector::from_str("(1/2, -2/3, 0, -3)").unwrap();
    /// v.entrywise_abs_assign();
    /// assert_eq!(v.to_string(), "(1/2, 2/3, 0, 3)");
    /// ```
    fn entrywise_abs_assign(&mut self) {
        for x in &mut self.elements {
            x.abs_assign();
        }
    }
}
