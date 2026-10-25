// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::unsigned_matrix::UnsignedMatrix;

impl<T: PrimitiveUnsigned> From<UnsignedMatrix<T>> for NaturalMatrix
where
    Natural: From<T>,
{
    /// Converts an [`UnsignedMatrix`] to a [`NaturalMatrix`].
    ///
    /// Every unsigned primitive is a [`Natural`], so nothing is lost and nothing can fail. The
    /// elements are converted one by one, so the height and width are unchanged.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `m.height() * m.width()`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let m = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// assert_eq!(NaturalMatrix::from(m).to_string(), "[1, 2, 3; 4, 5, 6]");
    ///
    /// assert_eq!(
    ///     NaturalMatrix::from(UnsignedMatrix::<u8>::from_str("[]_{3,0}").unwrap()).to_string(),
    ///     "[]_{3,0}"
    /// );
    /// ```
    #[inline]
    fn from(m: UnsignedMatrix<T>) -> Self {
        Self::from_owned_elements(
            m.height(),
            m.width(),
            m.into_elements().into_iter().map(Natural::from).collect(),
        )
    }
}
