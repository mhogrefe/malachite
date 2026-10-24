// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use malachite_base::num::arithmetic::traits::Rank;

// The rank of a matrix: that of the integer matrix found by clearing the denominators of each row,
// which has the same rank, since scaling a row by a nonzero number keeps it.
fn rank_helper(a: &RationalMatrix) -> u64 {
    a.to_numerators_and_row_denominators().0.rank()
}

impl Rank for RationalMatrix {
    /// Computes the rank of a [`RationalMatrix`]: the number of linearly independent rows, or
    /// equivalently columns, taking the matrix by value.
    ///
    /// The denominators of each row are cleared, which keeps the rank, and the rank of the
    /// resulting [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix) is computed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of the numerators and
    /// denominators of its elements: clearing the denominators of the rows gives integer elements
    /// of $O(nb)$ bits.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Rank;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.clone().rank(), 2);
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3, 1/4; 1, 2/3, 1/2]").unwrap();
    /// assert_eq!(a.clone().rank(), 1);
    /// ```
    #[inline]
    fn rank(self) -> u64 {
        self.into_numerators_and_row_denominators().0.rank()
    }
}

impl Rank for &RationalMatrix {
    /// Computes the rank of a [`RationalMatrix`]: the number of linearly independent rows, or
    /// equivalently columns, taking the matrix by reference.
    ///
    /// The denominators of each row are cleared, which keeps the rank, and the rank of the
    /// resulting [`IntegerMatrix`](malachite_nz::integer_matrix::IntegerMatrix) is computed.
    ///
    /// # Worst-case complexity
    /// $T(n, b) = O(n^6 b^2 + n^5 b \log^2 (nb) \log\log (nb))$
    ///
    /// $M(n, b) = O(n^4 b)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the maximum of the height and width of
    /// the matrix, and $b$ is the maximum number of significant bits of the numerators and
    /// denominators of its elements: clearing the denominators of the rows gives integer elements
    /// of $O(nb)$ bits.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Rank;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 9]").unwrap();
    /// assert_eq!((&a).rank(), 2);
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3, 1/4; 1, 2/3, 1/2]").unwrap();
    /// assert_eq!((&a).rank(), 1);
    /// ```
    #[inline]
    fn rank(self) -> u64 {
        rank_helper(self)
    }
}
