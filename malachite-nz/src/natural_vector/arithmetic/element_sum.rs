// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use malachite_base::num::arithmetic::traits::ElementSum;

impl ElementSum for NaturalVector {
    type Output = Natural;

    /// Returns the sum of the elements of a [`NaturalVector`], taking the vector by reference.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 3, 2)")
    ///         .unwrap()
    ///         .to_element_sum(),
    ///     6
    /// );
    /// assert_eq!(NaturalVector::from_str("()").unwrap().to_element_sum(), 0);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_sum` from `fmpz_vec/sum.c`, FLINT 3.6.0.
    #[inline]
    fn to_element_sum(&self) -> Natural {
        self.elements.iter().sum()
    }

    /// Returns the sum of the elements of a [`NaturalVector`], taking the vector by value.
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
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// assert_eq!(
    ///     NaturalVector::from_str("(1, 3, 2)")
    ///         .unwrap()
    ///         .into_element_sum(),
    ///     6
    /// );
    /// assert_eq!(NaturalVector::from_str("()").unwrap().into_element_sum(), 0);
    /// ```
    ///
    /// This is equivalent to `_fmpz_vec_sum` from `fmpz_vec/sum.c`, FLINT 3.6.0.
    #[inline]
    fn into_element_sum(self) -> Natural {
        self.elements.into_iter().sum()
    }
}
