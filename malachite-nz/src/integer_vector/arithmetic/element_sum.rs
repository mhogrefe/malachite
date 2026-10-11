// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use malachite_base::num::arithmetic::traits::ElementSum;

impl ElementSum for IntegerVector {
    type Output = Integer;

    /// Returns the sum of the elements of an [`IntegerVector`], taking the vector by reference.
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_str("(1, -5, 2)")
    ///         .unwrap()
    ///         .to_element_sum(),
    ///     -2
    /// );
    /// assert_eq!(IntegerVector::from_str("()").unwrap().to_element_sum(), 0);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_sum` from `fmpz_vec/sum.c`, FLINT 3.6.0.
    #[inline]
    fn to_element_sum(&self) -> Integer {
        self.elements.iter().sum()
    }

    /// Returns the sum of the elements of an [`IntegerVector`], taking the vector by value.
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
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_str("(1, -5, 2)")
    ///         .unwrap()
    ///         .into_element_sum(),
    ///     -2
    /// );
    /// assert_eq!(IntegerVector::from_str("()").unwrap().into_element_sum(), 0);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_sum` from `fmpz_vec/sum.c`, FLINT 3.6.0.
    #[inline]
    fn into_element_sum(self) -> Integer {
        self.elements.into_iter().sum()
    }
}
