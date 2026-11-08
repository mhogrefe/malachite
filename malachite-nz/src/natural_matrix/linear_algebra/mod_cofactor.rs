// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::Parity;
#[cfg(doc)]
use malachite_base::num::linear_algebra::traits::ModDeterminant;
use malachite_base::num::linear_algebra::traits::{ModCofactor, ModMinor};
use malachite_base::num::modular::traits::ModNeg;

impl ModCofactor<Natural> for NaturalMatrix {
    type Output = Natural;

    /// Computes the cofactor of a square [`NaturalMatrix`] at row `i` and column `j` modulo $m$,
    /// taking the matrix by value and the modulus by value.
    ///
    /// $$
    /// f(A, i, j, m) = C_{i, j} = (-1)^{i + j} M_{i, j} \bmod m,
    /// $$
    ///
    /// where $M_{i, j}$ is the [minor](malachite_base::num::linear_algebra::traits::ModMinor)
    /// modulo $m$.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of [`mod_determinant`](ModDeterminant::mod_determinant) for a square
    /// [`NaturalMatrix`] of height $n - 1$ with the same elements, where $n$ is the height of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, if `i` or `j` is not less than its
    /// height, or if any element of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModCofactor;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 0, 1, 6]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_cofactor(1, 2, Natural::from(7u32))
    ///         .to_string(),
    ///     "6"
    /// );
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_cofactor(0, 1, Natural::from(7u32))
    ///         .to_string(),
    ///     "4"
    /// );
    /// ```
    fn mod_cofactor(self, i: u64, j: u64, m: Natural) -> Natural {
        let c = self.mod_minor(i, j, &m);
        if (i + j).even() { c } else { c.mod_neg(&m) }
    }
}

impl ModCofactor<&Natural> for NaturalMatrix {
    type Output = Natural;

    /// Computes the cofactor of a square [`NaturalMatrix`] at row `i` and column `j` modulo $m$,
    /// taking the matrix by value and the modulus by reference.
    ///
    /// $$
    /// f(A, i, j, m) = C_{i, j} = (-1)^{i + j} M_{i, j} \bmod m,
    /// $$
    ///
    /// where $M_{i, j}$ is the [minor](malachite_base::num::linear_algebra::traits::ModMinor)
    /// modulo $m$.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of [`mod_determinant`](ModDeterminant::mod_determinant) for a square
    /// [`NaturalMatrix`] of height $n - 1$ with the same elements, where $n$ is the height of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, if `i` or `j` is not less than its
    /// height, or if any element of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModCofactor;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 0, 1, 6]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_cofactor(1, 2, &Natural::from(7u32))
    ///         .to_string(),
    ///     "6"
    /// );
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_cofactor(0, 1, &Natural::from(7u32))
    ///         .to_string(),
    ///     "4"
    /// );
    /// ```
    fn mod_cofactor(self, i: u64, j: u64, m: &Natural) -> Natural {
        let c = self.mod_minor(i, j, m);
        if (i + j).even() { c } else { c.mod_neg(m) }
    }
}

impl ModCofactor<Natural> for &NaturalMatrix {
    type Output = Natural;

    /// Computes the cofactor of a square [`NaturalMatrix`] at row `i` and column `j` modulo $m$,
    /// taking the matrix by reference and the modulus by value.
    ///
    /// $$
    /// f(A, i, j, m) = C_{i, j} = (-1)^{i + j} M_{i, j} \bmod m,
    /// $$
    ///
    /// where $M_{i, j}$ is the [minor](malachite_base::num::linear_algebra::traits::ModMinor)
    /// modulo $m$.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of [`mod_determinant`](ModDeterminant::mod_determinant) for a square
    /// [`NaturalMatrix`] of height $n - 1$ with the same elements, where $n$ is the height of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, if `i` or `j` is not less than its
    /// height, or if any element of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModCofactor;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 0, 1, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_cofactor(1, 2, Natural::from(7u32)).to_string(),
    ///     "6"
    /// );
    /// assert_eq!(
    ///     (&a).mod_cofactor(0, 1, Natural::from(7u32)).to_string(),
    ///     "4"
    /// );
    /// ```
    fn mod_cofactor(self, i: u64, j: u64, m: Natural) -> Natural {
        let c = self.mod_minor(i, j, &m);
        if (i + j).even() { c } else { c.mod_neg(&m) }
    }
}

impl ModCofactor<&Natural> for &NaturalMatrix {
    type Output = Natural;

    /// Computes the cofactor of a square [`NaturalMatrix`] at row `i` and column `j` modulo $m$,
    /// taking the matrix by reference and the modulus by reference.
    ///
    /// $$
    /// f(A, i, j, m) = C_{i, j} = (-1)^{i + j} M_{i, j} \bmod m,
    /// $$
    ///
    /// where $M_{i, j}$ is the [minor](malachite_base::num::linear_algebra::traits::ModMinor)
    /// modulo $m$.
    ///
    /// # Worst-case complexity
    /// Same as the complexity of [`mod_determinant`](ModDeterminant::mod_determinant) for a square
    /// [`NaturalMatrix`] of height $n - 1$ with the same elements, where $n$ is the height of the
    /// matrix.
    ///
    /// # Panics
    /// Panics if `m` is zero, if the matrix is not square, if `i` or `j` is not less than its
    /// height, or if any element of the matrix is greater than or equal to $m$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::linear_algebra::traits::ModCofactor;
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 0, 1, 6]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_cofactor(1, 2, &Natural::from(7u32)).to_string(),
    ///     "6"
    /// );
    /// assert_eq!(
    ///     (&a).mod_cofactor(0, 1, &Natural::from(7u32)).to_string(),
    ///     "4"
    /// );
    /// ```
    fn mod_cofactor(self, i: u64, j: u64, m: &Natural) -> Natural {
        let c = self.mod_minor(i, j, m);
        if (i + j).even() { c } else { c.mod_neg(m) }
    }
}
