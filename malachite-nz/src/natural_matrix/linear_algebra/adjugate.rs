// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::linear_algebra::minor::assert_square;
use malachite_base::matrix::Matrix;
#[cfg(doc)]
use malachite_base::num::linear_algebra::traits::Minor;
use malachite_base::num::linear_algebra::traits::{Adjugate, Cofactor};

impl Adjugate for NaturalMatrix {
    type Output = IntegerMatrix;

    /// Computes the adjugate of a square [`NaturalMatrix`], taking the matrix by value.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert_eq!(
    ///     (NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap())
    ///         .adjugate()
    ///         .to_string(),
    ///     "[2, 4, -3; 2, -11, 6; -3, 6, -3]"
    /// );
    /// assert_eq!(
    ///     (NaturalMatrix::from_str("[5]").unwrap())
    ///         .adjugate()
    ///         .to_string(),
    ///     "[1]"
    /// );
    /// assert_eq!(
    ///     (NaturalMatrix::from_str("[]_{0,0}").unwrap())
    ///         .adjugate()
    ///         .to_string(),
    ///     "[]_{0,0}"
    /// );
    /// ```
    #[inline]
    fn adjugate(self) -> IntegerMatrix {
        (&self).adjugate()
    }
}

impl Adjugate for &NaturalMatrix {
    type Output = IntegerMatrix;

    /// Computes the adjugate of a square [`NaturalMatrix`], taking the matrix by reference.
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
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert_eq!(
    ///     (&NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap())
    ///         .adjugate()
    ///         .to_string(),
    ///     "[2, 4, -3; 2, -11, 6; -3, 6, -3]"
    /// );
    /// assert_eq!(
    ///     (&NaturalMatrix::from_str("[5]").unwrap())
    ///         .adjugate()
    ///         .to_string(),
    ///     "[1]"
    /// );
    /// assert_eq!(
    ///     (&NaturalMatrix::from_str("[]_{0,0}").unwrap())
    ///         .adjugate()
    ///         .to_string(),
    ///     "[]_{0,0}"
    /// );
    /// ```
    fn adjugate(self) -> IntegerMatrix {
        assert_square(self);
        let n = self.height();
        IntegerMatrix::from_owned_elements(
            n,
            n,
            (0..n)
                .flat_map(|i| (0..n).map(move |j| self.cofactor(j, i)))
                .collect(),
        )
    }
}
