// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::{MulAddMul, Permanent};
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;

// Returns the permanent of the square matrix `a`: directly for dimensions up to 2, and otherwise as
// the permanent of the `IntegerMatrix` with the same elements.
fn permanent_helper(a: &NaturalMatrix) -> Natural {
    assert_eq!(a.height, a.width, "the matrix must be square");
    match a.elements.as_slice() {
        [] => Natural::ONE,
        [a00] => a00.clone(),
        [a00, a01, a10, a11] => a00.mul_add_mul(a11, a01, a10),
        _ => Natural::exact_from(IntegerMatrix::from(a).permanent()),
    }
}

impl Permanent for NaturalMatrix {
    type Output = Natural;

    /// Computes the permanent of a square [`NaturalMatrix`], taking it by value.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. The permanent of the $0
    /// \times 0$ matrix is 1. Matrices of dimension greater than 2 are handled as the
    /// [`IntegerMatrix`] with the same elements.
    ///
    /// $$
    /// f(A) = \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)}.
    /// $$
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `permanent` for an [`IntegerMatrix`] with the same elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Permanent;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.clone().permanent().to_string(), "10");
    ///
    /// let a = NaturalMatrix::from_str("[2, 1, 1; 1, 2, 1; 1, 1, 2]").unwrap();
    /// assert_eq!(a.clone().permanent().to_string(), "16");
    /// ```
    #[inline]
    fn permanent(self) -> Natural {
        permanent_helper(&self)
    }
}

impl Permanent for &NaturalMatrix {
    type Output = Natural;

    /// Computes the permanent of a square [`NaturalMatrix`], taking it by reference.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. The permanent of the $0
    /// \times 0$ matrix is 1. Matrices of dimension greater than 2 are handled as the
    /// [`IntegerMatrix`] with the same elements.
    ///
    /// $$
    /// f(A) = \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)}.
    /// $$
    ///
    /// # Worst-case complexity
    /// Same as the complexity of `permanent` for an [`IntegerMatrix`] with the same elements.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Permanent;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).permanent().to_string(), "10");
    ///
    /// let a = NaturalMatrix::from_str("[2, 1, 1; 1, 2, 1; 1, 1, 2]").unwrap();
    /// assert_eq!((&a).permanent().to_string(), "16");
    /// ```
    #[inline]
    fn permanent(self) -> Natural {
        permanent_helper(self)
    }
}
