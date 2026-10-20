// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::arithmetic::pow::matrix_pow;
use crate::integer_matrix::arithmetic::square::assert_square;
use crate::natural_matrix::NaturalMatrix;
use malachite_base::num::arithmetic::traits::{Pow, PowAssign};

impl Pow<u64> for NaturalMatrix {
    type Output = Self;

    /// Raises a square [`NaturalMatrix`] to a power, taking it by value.
    ///
    /// The power $A^k$ of an $n \times n$ matrix $A$ is the product of $k$ copies of $A$; $A^0$ is
    /// the $n \times n$ identity matrix.
    ///
    /// $$
    /// f(A, k) = A^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O(n^3 km \log (km) \log\log (km))$
    ///
    /// $M(n, m, k) = O(n^2 km)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is the
    /// largest number of significant bits of any of its elements, and $k$ is `exp`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!(a.pow(10).to_string(), "[89, 55; 55, 34]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.pow(3).to_string(), "[37, 54; 81, 118]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.pow(0).to_string(), "[1, 0; 0, 1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_pow` from `fmpz_mat/pow.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square, which FLINT does not check.
    fn pow(self, exp: u64) -> Self {
        if exp == 1 {
            assert_square(&self);
            self
        } else {
            (&self).pow(exp)
        }
    }
}

impl Pow<u64> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Raises a square [`NaturalMatrix`] to a power, taking it by reference.
    ///
    /// The power $A^k$ of an $n \times n$ matrix $A$ is the product of $k$ copies of $A$; $A^0$ is
    /// the $n \times n$ identity matrix.
    ///
    /// $$
    /// f(A, k) = A^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O(n^3 km \log (km) \log\log (km))$
    ///
    /// $M(n, m, k) = O(n^2 km)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is the
    /// largest number of significant bits of any of its elements, and $k$ is `exp`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Pow;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// assert_eq!((&a).pow(10).to_string(), "[89, 55; 55, 34]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).pow(3).to_string(), "[37, 54; 81, 118]");
    ///
    /// let a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).pow(0).to_string(), "[1, 0; 0, 1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_pow` from `fmpz_mat/pow.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square, which FLINT does not check.
    fn pow(self, exp: u64) -> NaturalMatrix {
        assert_square(self);
        NaturalMatrix {
            elements: matrix_pow(&self.elements, self.height, exp),
            height: self.height,
            width: self.width,
        }
    }
}

impl PowAssign<u64> for NaturalMatrix {
    /// Raises a square [`NaturalMatrix`] to a power in place.
    ///
    /// The power $A^k$ of an $n \times n$ matrix $A$ is the product of $k$ copies of $A$; $A^0$ is
    /// the $n \times n$ identity matrix.
    ///
    /// $$
    /// f(A, k) = A^k.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m, k) = O(n^3 km \log (km) \log\log (km))$
    ///
    /// $M(n, m, k) = O(n^2 km)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, $m$ is the
    /// largest number of significant bits of any of its elements, and $k$ is `exp`.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::PowAssign;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 1; 1, 0]").unwrap();
    /// a.pow_assign(10);
    /// assert_eq!(a.to_string(), "[89, 55; 55, 34]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.pow_assign(3);
    /// assert_eq!(a.to_string(), "[37, 54; 81, 118]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a.pow_assign(0);
    /// assert_eq!(a.to_string(), "[1, 0; 0, 1]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_pow` from `fmpz_mat/pow.c`, FLINT 3.6.0, except that it
    /// panics if the matrix is not square, which FLINT does not check.
    fn pow_assign(&mut self, exp: u64) {
        assert_square(self);
        if exp != 1 {
            self.elements = matrix_pow(&self.elements, self.height, exp);
        }
    }
}
