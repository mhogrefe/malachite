// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2014 Alex J. Best
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::DivisibleBy;

impl IntegerMatrix {
    /// Determines whether an [`IntegerMatrix`] is in Smith normal form: whether it is diagonal and
    /// its diagonal elements are nonnegative and each divide the next, so that, in particular, no
    /// nonzero diagonal element follows a zero one.
    ///
    /// # Worst-case complexity
    /// $T(h, w, b) = O(hw + \min(h, w) b \log b \log\log b)$
    ///
    /// $M(b) = O(b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $h$ is the height of the matrix, $w$ is its
    /// width, and $b$ is the maximum number of significant bits of its elements.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// assert!(
    ///     IntegerMatrix::from_str("[1, 0; 0, 2]")
    ///         .unwrap()
    ///         .is_in_smith_normal_form()
    /// );
    /// assert!(
    ///     IntegerMatrix::from_str("[2, 0; 0, 6; 0, 0]")
    ///         .unwrap()
    ///         .is_in_smith_normal_form()
    /// );
    ///
    /// // 2 does not divide 3
    /// assert!(
    ///     !IntegerMatrix::from_str("[2, 0; 0, 3]")
    ///         .unwrap()
    ///         .is_in_smith_normal_form()
    /// );
    /// // Not diagonal
    /// assert!(
    ///     !IntegerMatrix::from_str("[1, 1; 0, 2]")
    ///         .unwrap()
    ///         .is_in_smith_normal_form()
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_is_in_snf` from `fmpz_mat/is_in_snf.c`, FLINT 3.6.0.
    pub fn is_in_smith_normal_form(&self) -> bool {
        if !self.is_diagonal() {
            return false;
        }
        let w = self.width;
        let d = self.height.min(w);
        let mut previous: Option<&Integer> = None;
        for i in 0..d {
            let x = &self.elements[i * (w + 1)];
            if *x < 0u32 {
                return false;
            }
            if let Some(previous) = previous
                && (*x != 0u32 && *previous == 0u32 || !x.divisible_by(previous))
            {
                return false;
            }
            previous = Some(x);
        }
        true
    }
}
