// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::linear_algebra::traits::{ModDeterminant, ModMinor};
use crate::num::modular::traits::ModIsReduced;
use crate::unsigned_matrix::UnsignedMatrix;

// Panics unless the modulus is nonzero, the matrix is square, and its elements are reduced modulo
// the modulus.
pub(crate) fn check_mod<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: &T) {
    assert!(*m != T::ZERO, "the modulus must be nonzero");
    assert!(a.is_square(), "minors are only defined for square matrices");
    assert!(
        a.mod_is_reduced(m),
        "each element of the matrix must be reduced modulo {m}"
    );
}

impl<T: PrimitiveUnsigned> ModMinor<T> for UnsignedMatrix<T>
where
    Self: ModDeterminant<T, Output = T>,
{
    type Output = T;

    /// Computes the minor of a square [`UnsignedMatrix`] at row `i` and column `j` modulo $m$,
    /// taking the matrix by value.
    ///
    /// The minor is the determinant modulo $m$ of the matrix obtained by deleting row `i` and
    /// column `j`:
    ///
    /// $$
    /// f(A, i, j, m) = M_{i, j} = \det A_{(i, j)} \bmod m.
    /// $$
    ///
    /// The minor of a $1 \times 1$ matrix is $1 \bmod m$. The modulus may be composite.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of [`mod_determinant`](ModDeterminant::mod_determinant) for a square
    /// [`UnsignedMatrix`] of height $n - 1$ with the same elements, where $n$ is the height of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, if `i` or `j` is not less than its
    /// height, or if any element of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModMinor;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 0, 1, 6]").unwrap();
    /// assert_eq!(a.clone().mod_minor(0, 0, 7).to_string(), "3");
    /// assert_eq!(a.clone().mod_minor(1, 2, 7).to_string(), "1");
    /// ```
    fn mod_minor(self, i: u64, j: u64, m: T) -> T {
        check_mod(&self, &m);
        self.into_without_row_and_column(i, j).mod_determinant(m)
    }
}

impl<T: PrimitiveUnsigned> ModMinor<T> for &UnsignedMatrix<T>
where
    UnsignedMatrix<T>: ModDeterminant<T, Output = T>,
{
    type Output = T;

    /// Computes the minor of a square [`UnsignedMatrix`] at row `i` and column `j` modulo $m$,
    /// taking the matrix by reference.
    ///
    /// The minor is the determinant modulo $m$ of the matrix obtained by deleting row `i` and
    /// column `j`:
    ///
    /// $$
    /// f(A, i, j, m) = M_{i, j} = \det A_{(i, j)} \bmod m.
    /// $$
    ///
    /// The minor of a $1 \times 1$ matrix is $1 \bmod m$. The modulus may be composite.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of [`mod_determinant`](ModDeterminant::mod_determinant) for a square
    /// [`UnsignedMatrix`] of height $n - 1$ with the same elements, where $n$ is the height of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, if `i` or `j` is not less than its
    /// height, or if any element of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModMinor;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6; 0, 1, 6]").unwrap();
    /// assert_eq!((&a).mod_minor(0, 0, 7).to_string(), "3");
    /// assert_eq!((&a).mod_minor(1, 2, 7).to_string(), "1");
    /// ```
    fn mod_minor(self, i: u64, j: u64, m: T) -> T {
        check_mod(self, &m);
        self.without_row_and_column(i, j).mod_determinant(m)
    }
}
