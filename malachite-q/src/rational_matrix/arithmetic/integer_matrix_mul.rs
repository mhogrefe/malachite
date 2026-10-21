// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use crate::rational_matrix::arithmetic::mul::{assert_product_compatible, divide_columns};
use core::ops::Mul;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer_matrix::IntegerMatrix;

impl Mul<RationalMatrix> for IntegerMatrix {
    type Output = RationalMatrix;

    /// Multiplies a [`IntegerMatrix`] by a [`RationalMatrix`], taking the first by value and the
    /// second by value.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// Each column of $B$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_column_denominators`][column_denominators]; $A$ is multiplied by the
    /// integer matrix as by the [`Mul`] implementation for [`IntegerMatrix`]s, and each column of
    /// the product is divided by its denominator and reduced once. Only the [`RationalMatrix`]'s
    /// denominators are cleared, so this is cheaper than converting $A$ to a [`RationalMatrix`]
    /// first.
    ///
    /// [column_denominators]: RationalMatrix::to_numerators_and_column_denominators
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the sum of the largest number of
    /// significant bits of any element of either matrix and the number of significant bits of the
    /// least common multiple of the denominators in any column of the second matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// let b = RationalMatrix::from_str("[1/2, 1/3; 1/4, 1/5]").unwrap();
    /// assert_eq!((a * b).to_string(), "[0, -1/15; 5/2, 9/5]");
    ///
    /// let a = IntegerMatrix::from_str("[2, 0, 1]").unwrap();
    /// let b = RationalMatrix::from_str("[1/2; 1/3; 1/4]").unwrap();
    /// assert_eq!((a * b).to_string(), "[5/4]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul_r_fmpz_mat` from `fmpq_mat/mul_r_fmpz_mat.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible, which FLINT does not check.
    fn mul(self, other: RationalMatrix) -> RationalMatrix {
        assert_product_compatible(usize::exact_from(self.width()), other.height);
        let (height, width) = (usize::exact_from(self.height()), other.width);
        let (numerators, denominators) = other.into_numerators_and_column_denominators();
        divide_columns(self * numerators, height, width, &denominators)
    }
}

impl Mul<&RationalMatrix> for IntegerMatrix {
    type Output = RationalMatrix;

    /// Multiplies a [`IntegerMatrix`] by a [`RationalMatrix`], taking the first by value and the
    /// second by reference.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// Each column of $B$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_column_denominators`][column_denominators]; $A$ is multiplied by the
    /// integer matrix as by the [`Mul`] implementation for [`IntegerMatrix`]s, and each column of
    /// the product is divided by its denominator and reduced once. Only the [`RationalMatrix`]'s
    /// denominators are cleared, so this is cheaper than converting $A$ to a [`RationalMatrix`]
    /// first.
    ///
    /// [column_denominators]: RationalMatrix::to_numerators_and_column_denominators
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the sum of the largest number of
    /// significant bits of any element of either matrix and the number of significant bits of the
    /// least common multiple of the denominators in any column of the second matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// let b = RationalMatrix::from_str("[1/2, 1/3; 1/4, 1/5]").unwrap();
    /// assert_eq!((a * &b).to_string(), "[0, -1/15; 5/2, 9/5]");
    ///
    /// let a = IntegerMatrix::from_str("[2, 0, 1]").unwrap();
    /// let b = RationalMatrix::from_str("[1/2; 1/3; 1/4]").unwrap();
    /// assert_eq!((a * &b).to_string(), "[5/4]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul_r_fmpz_mat` from `fmpq_mat/mul_r_fmpz_mat.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible, which FLINT does not check.
    fn mul(self, other: &RationalMatrix) -> RationalMatrix {
        assert_product_compatible(usize::exact_from(self.width()), other.height);
        let (height, width) = (usize::exact_from(self.height()), other.width);
        let (numerators, denominators) = other.to_numerators_and_column_denominators();
        divide_columns(self * numerators, height, width, &denominators)
    }
}

impl Mul<RationalMatrix> for &IntegerMatrix {
    type Output = RationalMatrix;

    /// Multiplies a [`IntegerMatrix`] by a [`RationalMatrix`], taking the first by reference and
    /// the second by value.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// Each column of $B$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_column_denominators`][column_denominators]; $A$ is multiplied by the
    /// integer matrix as by the [`Mul`] implementation for [`IntegerMatrix`]s, and each column of
    /// the product is divided by its denominator and reduced once. Only the [`RationalMatrix`]'s
    /// denominators are cleared, so this is cheaper than converting $A$ to a [`RationalMatrix`]
    /// first.
    ///
    /// [column_denominators]: RationalMatrix::to_numerators_and_column_denominators
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the sum of the largest number of
    /// significant bits of any element of either matrix and the number of significant bits of the
    /// least common multiple of the denominators in any column of the second matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// let b = RationalMatrix::from_str("[1/2, 1/3; 1/4, 1/5]").unwrap();
    /// assert_eq!((&a * b).to_string(), "[0, -1/15; 5/2, 9/5]");
    ///
    /// let a = IntegerMatrix::from_str("[2, 0, 1]").unwrap();
    /// let b = RationalMatrix::from_str("[1/2; 1/3; 1/4]").unwrap();
    /// assert_eq!((&a * b).to_string(), "[5/4]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul_r_fmpz_mat` from `fmpq_mat/mul_r_fmpz_mat.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible, which FLINT does not check.
    fn mul(self, other: RationalMatrix) -> RationalMatrix {
        assert_product_compatible(usize::exact_from(self.width()), other.height);
        let (height, width) = (usize::exact_from(self.height()), other.width);
        let (numerators, denominators) = other.into_numerators_and_column_denominators();
        divide_columns(self * numerators, height, width, &denominators)
    }
}

impl Mul<&RationalMatrix> for &IntegerMatrix {
    type Output = RationalMatrix;

    /// Multiplies a [`IntegerMatrix`] by a [`RationalMatrix`], taking the first by reference and
    /// the second by reference.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix.
    ///
    /// Each column of $B$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_column_denominators`][column_denominators]; $A$ is multiplied by the
    /// integer matrix as by the [`Mul`] implementation for [`IntegerMatrix`]s, and each column of
    /// the product is divided by its denominator and reduced once. Only the [`RationalMatrix`]'s
    /// denominators are cleared, so this is cheaper than converting $A$ to a [`RationalMatrix`]
    /// first.
    ///
    /// [column_denominators]: RationalMatrix::to_numerators_and_column_denominators
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the sum of the largest number of
    /// significant bits of any element of either matrix and the number of significant bits of the
    /// least common multiple of the denominators in any column of the second matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// let b = RationalMatrix::from_str("[1/2, 1/3; 1/4, 1/5]").unwrap();
    /// assert_eq!((&a * &b).to_string(), "[0, -1/15; 5/2, 9/5]");
    ///
    /// let a = IntegerMatrix::from_str("[2, 0, 1]").unwrap();
    /// let b = RationalMatrix::from_str("[1/2; 1/3; 1/4]").unwrap();
    /// assert_eq!((&a * &b).to_string(), "[5/4]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul_r_fmpz_mat` from `fmpq_mat/mul_r_fmpz_mat.c`, FLINT
    /// 3.6.0, except that it panics if the dimensions are incompatible, which FLINT does not check.
    fn mul(self, other: &RationalMatrix) -> RationalMatrix {
        assert_product_compatible(usize::exact_from(self.width()), other.height);
        let (height, width) = (usize::exact_from(self.height()), other.width);
        let (numerators, denominators) = other.to_numerators_and_column_denominators();
        divide_columns(self * numerators, height, width, &denominators)
    }
}
