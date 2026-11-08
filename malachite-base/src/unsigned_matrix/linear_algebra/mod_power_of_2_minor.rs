// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::linear_algebra::traits::{ModPowerOf2Determinant, ModPowerOf2Minor};
use crate::num::modular::traits::ModPowerOf2IsReduced;
use crate::unsigned_matrix::UnsignedMatrix;

// Panics unless the matrix is square and its elements are reduced modulo 2^pow.
pub(crate) fn check_mod_power_of_2<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, pow: u64) {
    assert!(a.is_square(), "minors are only defined for square matrices");
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "each element of the matrix must be reduced modulo 2^{pow}"
    );
}

impl<T: PrimitiveUnsigned> ModPowerOf2Minor for UnsignedMatrix<T> {
    type Output = T;

    /// Computes the minor of a square [`UnsignedMatrix`] at row `i` and column `j` modulo $2^k$,
    /// taking the matrix by value.
    ///
    /// The minor is the determinant modulo $2^k$ of the matrix obtained by deleting row `i` and
    /// column `j`:
    ///
    /// $$
    /// f(A, i, j) = M_{i, j} = \det A_{(i, j)} \bmod 2^k.
    /// $$
    ///
    /// The minor of a $1 \times 1$ matrix is $1 \bmod 2^k$.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of
    /// [`mod_power_of_2_determinant`](ModPowerOf2Determinant::mod_power_of_2_determinant) for a
    /// square [`UnsignedMatrix`] of height $n - 1$ with the same elements, where $n$ is the height
    /// of the matrix.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if `i` or `j` is not less than its height, or if any
    /// element of the matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModPowerOf2Minor;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 0, 1]").unwrap();
    /// assert_eq!(a.clone().mod_power_of_2_minor(0, 0, 3).to_string(), "5");
    /// assert_eq!(a.clone().mod_power_of_2_minor(1, 2, 3).to_string(), "2");
    /// ```
    fn mod_power_of_2_minor(self, i: u64, j: u64, pow: u64) -> T {
        check_mod_power_of_2(&self, pow);
        self.into_without_row_and_column(i, j)
            .mod_power_of_2_determinant(pow)
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Minor for &UnsignedMatrix<T> {
    type Output = T;

    /// Computes the minor of a square [`UnsignedMatrix`] at row `i` and column `j` modulo $2^k$,
    /// taking the matrix by reference.
    ///
    /// The minor is the determinant modulo $2^k$ of the matrix obtained by deleting row `i` and
    /// column `j`:
    ///
    /// $$
    /// f(A, i, j) = M_{i, j} = \det A_{(i, j)} \bmod 2^k.
    /// $$
    ///
    /// The minor of a $1 \times 1$ matrix is $1 \bmod 2^k$.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of
    /// [`mod_power_of_2_determinant`](ModPowerOf2Determinant::mod_power_of_2_determinant) for a
    /// square [`UnsignedMatrix`] of height $n - 1$ with the same elements, where $n$ is the height
    /// of the matrix.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if `i` or `j` is not less than its height, or if any
    /// element of the matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModPowerOf2Minor;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 0, 1]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_minor(0, 0, 3).to_string(), "5");
    /// assert_eq!((&a).mod_power_of_2_minor(1, 2, 3).to_string(), "2");
    /// ```
    fn mod_power_of_2_minor(self, i: u64, j: u64, pow: u64) -> T {
        check_mod_power_of_2(self, pow);
        self.without_row_and_column(i, j)
            .mod_power_of_2_determinant(pow)
    }
}
