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
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Sum, ModPowerOf2Trace,
};

fn check_args(a: &NaturalMatrix, pow: u64) {
    assert!(
        a.is_square(),
        "the matrix must be square, but it is {} by {}",
        a.height,
        a.width
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {a} has an element >= 2^{pow}"
    );
}

impl ModPowerOf2Trace for NaturalMatrix {
    type Output = Natural;

    /// Returns the trace of a square [`NaturalMatrix`] modulo $2^k$, the sum of its diagonal
    /// elements modulo $2^k$, taking the matrix by reference. The elements must already be reduced.
    ///
    /// The $0 \times 0$ matrix has no diagonal elements, and its trace is 0. Every element is
    /// checked to be reduced, not only the diagonal ones.
    ///
    /// $$
    /// f(A, k) = \sum_{i=0}^{n-1} A_{ii} \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Trace;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // 5 + 7 = 12, which is 4 mod 8.
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[5, 1; 2, 7]")
    ///         .unwrap()
    ///         .to_mod_power_of_2_trace(3)
    ///         .to_string(),
    ///     "4"
    /// );
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .to_mod_power_of_2_trace(3)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    fn to_mod_power_of_2_trace(&self, pow: u64) -> Natural {
        check_args(self, pow);
        // The diagonal elements are `width + 1` apart in row-major order.
        Natural::mod_power_of_2_sum(self.elements.iter().step_by(self.width + 1), pow)
    }

    /// Returns the trace of a square [`NaturalMatrix`] modulo $2^k$, the sum of its diagonal
    /// elements modulo $2^k$, taking the matrix by value. The elements must already be reduced.
    ///
    /// The $0 \times 0$ matrix has no diagonal elements, and its trace is 0. Every element is
    /// checked to be reduced, not only the diagonal ones.
    ///
    /// $$
    /// f(A, k) = \sum_{i=0}^{n-1} A_{ii} \bmod 2^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the total number of bits of the
    /// elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square, or if any element is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Trace;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// // 5 + 7 = 12, which is 4 mod 8.
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[5, 1; 2, 7]")
    ///         .unwrap()
    ///         .into_mod_power_of_2_trace(3)
    ///         .to_string(),
    ///     "4"
    /// );
    /// assert_eq!(
    ///     NaturalMatrix::from_str("[]_{0,0}")
    ///         .unwrap()
    ///         .into_mod_power_of_2_trace(3)
    ///         .to_string(),
    ///     "0"
    /// );
    /// ```
    fn into_mod_power_of_2_trace(self, pow: u64) -> Natural {
        check_args(&self, pow);
        let step = self.width + 1;
        Natural::mod_power_of_2_sum(self.elements.into_iter().step_by(step), pow)
    }
}
