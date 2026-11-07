// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2021 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::AddMulAssign;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::number_theory::traits::{
    StirlingMatrixOfTheSecondKind, UnsignedStirlingMatrixOfTheFirstKind,
};

// The `height`-by-`width` truncation of the triangle of [n, k], if `first_kind` is true, or of S(n,
// k), filled row by row with [n, k] = [n - 1, k - 1] + (n - 1)[n - 1, k] or S(n, k) = S(n - 1, k
// - 1) + kS(n - 1, k).
//
// This is `gr_mat_stirling` from `gr_mat/stirling.c`, FLINT 3.6.0, with `kind` equal to 0 or 2.
pub(crate) fn stirling_matrix(height: u64, width: u64, first_kind: bool) -> NaturalMatrix {
    let mut m = NaturalMatrix::zero(height, width);
    let w = usize::exact_from(width);
    if w == 0 {
        return m;
    }
    for n in 0..usize::exact_from(height) {
        if n < w {
            m.elements[n * w + n] = Natural::ONE;
        }
        if n == 0 {
            continue;
        }
        let (before, after) = m.elements.split_at_mut(n * w);
        let previous = &before[(n - 1) * w..];
        for k in (1..n.min(w)).rev() {
            let multiplier = Natural::from(u64::exact_from(if first_kind { n - 1 } else { k }));
            after[k] = previous[k - 1].clone();
            after[k].add_mul_assign(&previous[k], multiplier);
        }
    }
    m
}

impl UnsignedStirlingMatrixOfTheFirstKind for NaturalMatrix {
    /// Computes the `height`-by-`width` matrix of unsigned Stirling numbers of the first kind,
    /// whose entry in row $n$ and column $k$ is $\left[{n \atop k}\right]$, the number of
    /// permutations of $n$ elements with $k$ cycles.
    ///
    /// Row $n$ holds the coefficients of the rising factorial $x(x + 1) \cdots (x + n - 1)$, so the
    /// entries above the diagonal are zero. The rows are found in turn by the recurrence $\left[{n
    /// \atop k}\right] = \left[{n - 1 \atop k - 1}\right] + (n - 1)\left[{n - 1 \atop k}\right]$.
    ///
    /// # Worst-case complexity
    /// $T(m, n) = O(m^2 n \log m)$
    ///
    /// $M(m, n) = O(m^2 n \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `height`, and $n$ is `width`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::number_theory::traits::UnsignedStirlingMatrixOfTheFirstKind;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert_eq!(
    ///     NaturalMatrix::unsigned_stirling_matrix_of_the_first_kind(4, 4).to_string(),
    ///     "[1, 0, 0, 0; 0, 1, 0, 0; 0, 1, 1, 0; 0, 2, 3, 1]"
    /// );
    /// assert_eq!(
    ///     NaturalMatrix::unsigned_stirling_matrix_of_the_first_kind(5, 3).to_string(),
    ///     "[1, 0, 0; 0, 1, 0; 0, 1, 1; 0, 2, 3; 0, 6, 11]"
    /// );
    /// ```
    ///
    /// This is equivalent to `arith_stirling_matrix_1u` from `arith/stirlingmat.c`, FLINT 3.6.0.
    #[inline]
    fn unsigned_stirling_matrix_of_the_first_kind(height: u64, width: u64) -> Self {
        stirling_matrix(height, width, true)
    }
}

impl StirlingMatrixOfTheSecondKind for NaturalMatrix {
    /// Computes the `height`-by-`width` matrix of Stirling numbers of the second kind, whose entry
    /// in row $n$ and column $k$ is $S(n, k)$, the number of ways to partition a set of $n$
    /// elements into $k$ nonempty subsets.
    ///
    /// Row $n$ holds the coefficients of the $n$th Touchard polynomial, so the entries above the
    /// diagonal are zero. The rows are found in turn by the recurrence $S(n, k) = S(n - 1, k - 1) +
    /// kS(n - 1, k)$. The square matrix is the inverse of the square matrix of Stirling numbers of
    /// the first kind.
    ///
    /// # Worst-case complexity
    /// $T(m, n) = O(m^2 n \log m)$
    ///
    /// $M(m, n) = O(m^2 n \log m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $m$ is `height`, and $n$ is `width`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::number_theory::traits::StirlingMatrixOfTheSecondKind;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert_eq!(
    ///     NaturalMatrix::stirling_matrix_of_the_second_kind(4, 4).to_string(),
    ///     "[1, 0, 0, 0; 0, 1, 0, 0; 0, 1, 1, 0; 0, 1, 3, 1]"
    /// );
    /// assert_eq!(
    ///     NaturalMatrix::stirling_matrix_of_the_second_kind(5, 3).to_string(),
    ///     "[1, 0, 0; 0, 1, 0; 0, 1, 1; 0, 1, 3; 0, 1, 7]"
    /// );
    /// ```
    ///
    /// This is equivalent to `arith_stirling_matrix_2` from `arith/stirlingmat.c`, FLINT 3.6.0.
    #[inline]
    fn stirling_matrix_of_the_second_kind(height: u64, width: u64) -> Self {
        stirling_matrix(height, width, false)
    }
}
