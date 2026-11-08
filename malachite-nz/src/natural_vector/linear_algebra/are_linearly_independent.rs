// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural_matrix::{NaturalMatrix, common_dimension};
use crate::natural_vector::NaturalVector;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::{AreLinearlyIndependent, Rank};

impl AreLinearlyIndependent for NaturalVector {
    /// Determines whether [`NaturalVector`]s are linearly independent over the rationals: whether
    /// no nontrivial linear combination of them, with rational coefficients, is zero.
    ///
    /// The vectors must all have the same dimension. They are linearly independent exactly when the
    /// matrix whose rows they are has rank equal to their number. An empty slice is linearly
    /// independent.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `rank` for the [`NaturalMatrix`] whose rows are the vectors.
    ///
    /// # Panics
    /// Panics if the vectors do not all have the same dimension.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::AreLinearlyIndependent;
    /// use malachite_nz::natural_vector::NaturalVector;
    ///
    /// let v = |s| NaturalVector::from_str(s).unwrap();
    /// assert!(NaturalVector::are_linearly_independent(&[
    ///     v("(1, 2)"),
    ///     v("(3, 4)")
    /// ]));
    /// assert!(!NaturalVector::are_linearly_independent(&[
    ///     v("(1, 2)"),
    ///     v("(2, 4)")
    /// ]));
    /// assert!(!NaturalVector::are_linearly_independent(&[v("(0, 0)")]));
    /// assert!(NaturalVector::are_linearly_independent(&[]));
    /// ```
    fn are_linearly_independent(xs: &[Self]) -> bool {
        // more vectors than their dimension are dependent
        xs.len() <= common_dimension(xs, "vectors")
            && NaturalMatrix::from_rows(xs).rank() == u64::exact_from(xs.len())
    }
}
