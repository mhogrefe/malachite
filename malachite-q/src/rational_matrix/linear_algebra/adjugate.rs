// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use crate::rational_matrix::linear_algebra::minor::assert_square;
use malachite_base::matrix::Matrix;
#[cfg(doc)]
use malachite_base::num::linear_algebra::traits::Minor;
use malachite_base::num::linear_algebra::traits::{Adjugate, Cofactor};

impl Adjugate for RationalMatrix {
    type Output = Self;

    /// Computes the adjugate of a square [`RationalMatrix`], taking the matrix by value.
    ///
    /// The adjugate is the transpose of the matrix of [cofactors](Cofactor::cofactor):
    ///
    /// $$
    /// f(A)_{i, j} = C_{j, i},
    /// $$
    ///
    /// so that $A \operatorname{adj}(A) = \operatorname{adj}(A) A = (\det A) I$. Each element is
    /// found as a separate cofactor.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 T_D(n - 1))$
    ///
    /// $M(n) = O(n^2 + M_D(n - 1))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $T_D$ and
    /// $M_D$ are the complexities of [`minor`](Minor::minor) for a square matrix of the given
    /// height with the same elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::Adjugate;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// assert_eq!(
    ///     (RationalMatrix::from_str("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]").unwrap())
    ///         .adjugate()
    ///         .to_string(),
    ///     "[-33, -6, 7; -78, -33/2, 15; -131/3, -10, 53/6]"
    /// );
    /// assert_eq!(
    ///     (RationalMatrix::from_str("[5]").unwrap())
    ///         .adjugate()
    ///         .to_string(),
    ///     "[1]"
    /// );
    /// assert_eq!(
    ///     (RationalMatrix::from_str("[]_{0,0}").unwrap())
    ///         .adjugate()
    ///         .to_string(),
    ///     "[]_{0,0}"
    /// );
    /// ```
    #[inline]
    fn adjugate(self) -> Self {
        (&self).adjugate()
    }
}

impl Adjugate for &RationalMatrix {
    type Output = RationalMatrix;

    /// Computes the adjugate of a square [`RationalMatrix`], taking the matrix by reference.
    ///
    /// The adjugate is the transpose of the matrix of [cofactors](Cofactor::cofactor):
    ///
    /// $$
    /// f(A)_{i, j} = C_{j, i},
    /// $$
    ///
    /// so that $A \operatorname{adj}(A) = \operatorname{adj}(A) A = (\det A) I$. Each element is
    /// found as a separate cofactor.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2 T_D(n - 1))$
    ///
    /// $M(n) = O(n^2 + M_D(n - 1))$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the height of the matrix, and $T_D$ and
    /// $M_D$ are the complexities of [`minor`](Minor::minor) for a square matrix of the given
    /// height with the same elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::Adjugate;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// assert_eq!(
    ///     (&RationalMatrix::from_str("[1/2, -2, 3; 4, 5/3, -6; 7, -8, 9]").unwrap())
    ///         .adjugate()
    ///         .to_string(),
    ///     "[-33, -6, 7; -78, -33/2, 15; -131/3, -10, 53/6]"
    /// );
    /// assert_eq!(
    ///     (&RationalMatrix::from_str("[5]").unwrap())
    ///         .adjugate()
    ///         .to_string(),
    ///     "[1]"
    /// );
    /// assert_eq!(
    ///     (&RationalMatrix::from_str("[]_{0,0}").unwrap())
    ///         .adjugate()
    ///         .to_string(),
    ///     "[]_{0,0}"
    /// );
    /// ```
    fn adjugate(self) -> RationalMatrix {
        assert_square(self);
        let n = self.height();
        RationalMatrix::from_owned_elements(
            n,
            n,
            (0..n)
                .flat_map(|i| (0..n).map(move |j| self.cofactor(j, i)))
                .collect(),
        )
    }
}
