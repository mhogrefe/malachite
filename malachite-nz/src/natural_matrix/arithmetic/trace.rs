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
use malachite_base::num::arithmetic::traits::Trace;

impl Trace for NaturalMatrix {
    type Output = Natural;

    /// Returns the trace of a square [`NaturalMatrix`], the sum of its diagonal elements, taking
    /// the matrix by reference.
    ///
    /// The $0 \times 0$ matrix has no diagonal elements, and its trace is 0.
    ///
    /// $$
    /// f(A) = \sum_{i=0}^{n-1} A_{ii}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// diagonal elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Trace;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[1, 2; 3, 4]")
    ///         .unwrap()
    ///         .to_trace()
    ///         .to_string(),
    ///     "5"
    /// );
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .to_trace()
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_trace` from `fmpz_mat/trace.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square.
    fn to_trace(&self) -> Natural {
        assert!(
            self.is_square(),
            "the matrix must be square, but it is {} by {}",
            self.height,
            self.width
        );
        // The diagonal elements are `width + 1` apart in row-major order.
        self.elements.iter().step_by(self.width + 1).sum()
    }

    /// Returns the trace of a square [`NaturalMatrix`], the sum of its diagonal elements, taking
    /// the matrix by value.
    ///
    /// The $0 \times 0$ matrix has no diagonal elements, and its trace is 0.
    ///
    /// $$
    /// f(A) = \sum_{i=0}^{n-1} A_{ii}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// diagonal elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Trace;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[1, 2; 3, 4]")
    ///         .unwrap()
    ///         .into_trace()
    ///         .to_string(),
    ///     "5"
    /// );
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .into_trace()
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_trace` from `fmpz_mat/trace.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square.
    fn into_trace(self) -> Natural {
        assert!(
            self.is_square(),
            "the matrix must be square, but it is {} by {}",
            self.height,
            self.width
        );
        // The diagonal elements are `width + 1` apart in row-major order.
        self.elements.into_iter().step_by(self.width + 1).sum()
    }
}
