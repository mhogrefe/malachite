// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use crate::integer::Integer;
use crate::integer_vector::IntegerVector;
use crate::natural_vector::NaturalVector;

impl From<NaturalVector> for IntegerVector {
    /// Converts a [`NaturalVector`] to an [`IntegerVector`].
    ///
    /// Every [`Natural`](crate::natural::Natural) is an [`Integer`], so nothing is lost and nothing
    /// can fail. The elements are converted one by one, without copying their limbs, so the
    /// dimension is unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `v.dimension()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_vector::IntegerVector;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = NaturalVector::from_str("(1, 2, 3)").unwrap();
    /// assert_eq!(IntegerVector::from(v).to_string(), "(1, 2, 3)");
    ///
    /// assert_eq!(
    ///     IntegerVector::from(NaturalVector::from_str("()").unwrap()).to_string(),
    ///     "()"
    /// );
    /// ```
    #[inline]
    fn from(v: NaturalVector) -> Self {
        Self {
            elements: v.elements.into_iter().map(Integer::from).collect(),
        }
    }
}
