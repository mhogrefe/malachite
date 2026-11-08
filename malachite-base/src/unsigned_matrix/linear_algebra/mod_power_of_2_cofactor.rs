// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::Parity;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
#[cfg(doc)]
use crate::num::linear_algebra::traits::ModPowerOf2Determinant;
use crate::num::linear_algebra::traits::{ModPowerOf2Cofactor, ModPowerOf2Minor};
use crate::unsigned_matrix::UnsignedMatrix;

impl<T: PrimitiveUnsigned> ModPowerOf2Cofactor for UnsignedMatrix<T> {
    type Output = T;

    /// Computes the cofactor of a square [`UnsignedMatrix`] at row `i` and column `j` modulo $2^k$,
    /// taking the matrix by value.
    ///
    /// $$
    /// f(A, i, j) = C_{i, j} = (-1)^{i + j} M_{i, j} \bmod 2^k,
    /// $$
    ///
    /// where $M_{i, j}$ is the [minor](crate::num::linear_algebra::traits::ModPowerOf2Minor) modulo
    /// $2^k$.
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
    /// use malachite_base::num::linear_algebra::traits::ModPowerOf2Cofactor;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 0, 1]").unwrap();
    /// assert_eq!(a.clone().mod_power_of_2_cofactor(1, 2, 3).to_string(), "6");
    /// assert_eq!(a.clone().mod_power_of_2_cofactor(0, 1, 3).to_string(), "6");
    /// ```
    fn mod_power_of_2_cofactor(self, i: u64, j: u64, pow: u64) -> T {
        let c = self.mod_power_of_2_minor(i, j, pow);
        if (i + j).even() {
            c
        } else {
            c.mod_power_of_2_neg(pow)
        }
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2Cofactor for &UnsignedMatrix<T> {
    type Output = T;

    /// Computes the cofactor of a square [`UnsignedMatrix`] at row `i` and column `j` modulo $2^k$,
    /// taking the matrix by reference.
    ///
    /// $$
    /// f(A, i, j) = C_{i, j} = (-1)^{i + j} M_{i, j} \bmod 2^k,
    /// $$
    ///
    /// where $M_{i, j}$ is the [minor](crate::num::linear_algebra::traits::ModPowerOf2Minor) modulo
    /// $2^k$.
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
    /// use malachite_base::num::linear_algebra::traits::ModPowerOf2Cofactor;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 7, 0, 1]").unwrap();
    /// assert_eq!((&a).mod_power_of_2_cofactor(1, 2, 3).to_string(), "6");
    /// assert_eq!((&a).mod_power_of_2_cofactor(0, 1, 3).to_string(), "6");
    /// ```
    fn mod_power_of_2_cofactor(self, i: u64, j: u64, pow: u64) -> T {
        let c = self.mod_power_of_2_minor(i, j, pow);
        if (i + j).even() {
            c
        } else {
            c.mod_power_of_2_neg(pow)
        }
    }
}
