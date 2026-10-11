// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use malachite_base::num::arithmetic::traits::ElementSum;

impl ElementSum for RationalVector {
    type Output = Rational;

    /// Returns the sum of the elements of a [`RationalVector`], taking the vector by reference.
    ///
    /// The 0-dimensional vector has no elements, and their sum is 0.
    ///
    /// $$
    /// f(v) = \sum_i v_i.
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::ElementSum;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_str("(1/2, -1/3)")
    ///         .unwrap()
    ///         .to_element_sum()
    ///         .to_string(),
    ///     "1/6"
    /// );
    /// assert_eq!(RationalVector::from_str("()").unwrap().to_element_sum(), 0);
    /// ```
    #[inline]
    fn to_element_sum(&self) -> Rational {
        self.elements.iter().sum()
    }

    /// Returns the sum of the elements of a [`RationalVector`], taking the vector by value.
    ///
    /// The 0-dimensional vector has no elements, and their sum is 0.
    ///
    /// $$
    /// f(v) = \sum_i v_i.
    /// $$
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
    /// use malachite_base::num::arithmetic::traits::ElementSum;
    /// use malachite_q::rational_vector::RationalVector;
    ///
    /// assert_eq!(
    ///     RationalVector::from_str("(1/2, -1/3)")
    ///         .unwrap()
    ///         .into_element_sum()
    ///         .to_string(),
    ///     "1/6"
    /// );
    /// assert_eq!(
    ///     RationalVector::from_str("()").unwrap().into_element_sum(),
    ///     0
    /// );
    /// ```
    #[inline]
    fn into_element_sum(self) -> Rational {
        self.elements.into_iter().sum()
    }
}
