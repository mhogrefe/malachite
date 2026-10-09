// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::integer_vector::arithmetic::sum_max_bits::vec_sum_abs_significant_bits;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{L1Norm, UnsignedAbs};

impl L1Norm for IntegerVector {
    type Output = Natural;

    /// Returns the $\ell^1$ norm of an [`IntegerVector`]: the sum of the absolute values of its elements, taking the vector by
    /// reference.
    ///
    /// The 0-dimensional vector has no elements, and its norm is 0.
    ///
    /// $$
    /// f(v) = \|v\|_1 = \sum_i |v_i|.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::L1Norm;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(IntegerVector::from_str("(1, -5, 2)").unwrap().to_l1_norm(), 8);
    /// assert_eq!(IntegerVector::from_str("()").unwrap().to_l1_norm(), 0);
    /// ```
    #[inline]
    fn to_l1_norm(&self) -> Natural {
        self.elements.iter().map(Integer::unsigned_abs_ref).sum()
    }

    /// Returns the $\ell^1$ norm of an [`IntegerVector`]: the sum of the absolute values of its elements, taking the vector by value.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::L1Norm;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(IntegerVector::from_str("(1, -5, 2)").unwrap().into_l1_norm(), 8);
    /// assert_eq!(IntegerVector::from_str("()").unwrap().into_l1_norm(), 0);
    /// ```
    #[inline]
    fn into_l1_norm(self) -> Natural {
        self.elements.into_iter().map(Integer::unsigned_abs).sum()
    }

    /// Returns the number of significant bits of the $\ell^1$ norm of an [`IntegerVector`].
    ///
    /// While every element fits in one limb, the absolute values are added in two limbs, so nothing is
    /// allocated, as in FLINT's `_fmpz_vec_sum_max_bits`.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::L1Norm;
    /// use malachite_nz::integer_vector::IntegerVector;
    ///
    /// assert_eq!(
    ///     IntegerVector::from_str("(1, -5, 2)")
    ///         .unwrap()
    ///         .l1_norm_significant_bits(),
    ///     4
    /// );
    /// assert_eq!(
    ///     IntegerVector::from_str("()")
    ///         .unwrap()
    ///         .l1_norm_significant_bits(),
    ///     0
    /// );
    /// ```
    #[inline]
    fn l1_norm_significant_bits(&self) -> u64 {
        vec_sum_abs_significant_bits(&self.elements)
    }
}
