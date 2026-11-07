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

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::natural_matrix::special::stirling_matrix::stirling_matrix;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::number_theory::traits::StirlingMatrixOfTheFirstKind;

impl StirlingMatrixOfTheFirstKind for IntegerMatrix {
    /// Computes the `height`-by-`width` matrix of Stirling numbers of the first kind, whose entry
    /// in row $n$ and column $k$ is $s(n, k) = (-1)^{n + k} \left[{n \atop k}\right]$.
    ///
    /// Row $n$ holds the coefficients of the falling factorial $x(x - 1) \cdots (x - n + 1)$, so
    /// the entries above the diagonal are zero. The square matrix is the inverse of the square
    /// matrix of Stirling numbers of the second kind.
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
    /// use malachite_base::num::number_theory::traits::StirlingMatrixOfTheFirstKind;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// assert_eq!(
    ///     IntegerMatrix::stirling_matrix_of_the_first_kind(4, 4).to_string(),
    ///     "[1, 0, 0, 0; 0, 1, 0, 0; 0, -1, 1, 0; 0, 2, -3, 1]"
    /// );
    /// assert_eq!(
    ///     IntegerMatrix::stirling_matrix_of_the_first_kind(5, 3).to_string(),
    ///     "[1, 0, 0; 0, 1, 0; 0, -1, 1; 0, 2, -3; 0, -6, 11]"
    /// );
    /// ```
    ///
    /// This is equivalent to `arith_stirling_matrix_1` from `arith/stirlingmat.c`, FLINT 3.6.0: the
    /// unsigned matrix, with the sign of each entry with $n + k$ odd reversed.
    fn stirling_matrix_of_the_first_kind(height: u64, width: u64) -> Self {
        let unsigned = stirling_matrix(height, width, true);
        let mut m = Self::zero(height, width);
        let w = usize::exact_from(width);
        for (i, x) in unsigned.elements.into_iter().enumerate() {
            let x = Integer::from(x);
            m.elements[i] = if (i / w + i % w).even() { x } else { -x };
        }
        m
    }
}
