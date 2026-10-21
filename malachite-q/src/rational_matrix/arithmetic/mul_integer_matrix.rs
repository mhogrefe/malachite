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
use crate::rational_matrix::arithmetic::mul::{
    assert_product_compatible, divide_rows, take_matrix,
};
use core::ops::{Mul, MulAssign};
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_nz::integer_matrix::IntegerMatrix;

impl Mul<IntegerMatrix> for RationalMatrix {
    type Output = Self;

    /// Multiplies a [`RationalMatrix`] by a [`IntegerMatrix`], taking the first by value and the
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
    /// Each row of $A$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_row_denominators`](RationalMatrix::to_numerators_and_row_denominators);
    /// the integer matrix is multiplied by $B$ as by the [`Mul`] implementation for
    /// [`IntegerMatrix`]s, and each row of the product is divided by its denominator and reduced
    /// once. Only the [`RationalMatrix`]'s denominators are cleared, so this is cheaper than
    /// converting $B$ to a [`RationalMatrix`] first.
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the sum of the largest number of
    /// significant bits of any element of either matrix and the number of significant bits of the
    /// least common multiple of the denominators in any row of the first matrix.
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
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// let b = IntegerMatrix::from_str("[2, 1; -1, 0; 1, 3]").unwrap();
    /// assert_eq!((a * b).to_string(), "[0, 1/2; 5/3, 6]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3/4, 1]").unwrap();
    /// let b = IntegerMatrix::from_str("[3, 0; 6, 4]").unwrap();
    /// assert_eq!((a * b).to_string(), "[-5/2, -8/3; 33/4, 4]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul_fmpz_mat` from `fmpq_mat/mul_fmpz_mat.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible, which FLINT does not check.
    fn mul(self, other: IntegerMatrix) -> Self {
        assert_product_compatible(self.width, usize::exact_from(other.height()));
        let (height, width) = (self.height, usize::exact_from(other.width()));
        let (numerators, denominators) = self.into_numerators_and_row_denominators();
        divide_rows(numerators * other, height, width, &denominators)
    }
}

impl Mul<&IntegerMatrix> for RationalMatrix {
    type Output = Self;

    /// Multiplies a [`RationalMatrix`] by a [`IntegerMatrix`], taking the first by value and the
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
    /// Each row of $A$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_row_denominators`](RationalMatrix::to_numerators_and_row_denominators);
    /// the integer matrix is multiplied by $B$ as by the [`Mul`] implementation for
    /// [`IntegerMatrix`]s, and each row of the product is divided by its denominator and reduced
    /// once. Only the [`RationalMatrix`]'s denominators are cleared, so this is cheaper than
    /// converting $B$ to a [`RationalMatrix`] first.
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the sum of the largest number of
    /// significant bits of any element of either matrix and the number of significant bits of the
    /// least common multiple of the denominators in any row of the first matrix.
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
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// let b = IntegerMatrix::from_str("[2, 1; -1, 0; 1, 3]").unwrap();
    /// assert_eq!((a * &b).to_string(), "[0, 1/2; 5/3, 6]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3/4, 1]").unwrap();
    /// let b = IntegerMatrix::from_str("[3, 0; 6, 4]").unwrap();
    /// assert_eq!((a * &b).to_string(), "[-5/2, -8/3; 33/4, 4]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul_fmpz_mat` from `fmpq_mat/mul_fmpz_mat.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible, which FLINT does not check.
    fn mul(self, other: &IntegerMatrix) -> Self {
        assert_product_compatible(self.width, usize::exact_from(other.height()));
        let (height, width) = (self.height, usize::exact_from(other.width()));
        let (numerators, denominators) = self.into_numerators_and_row_denominators();
        divide_rows(numerators * other, height, width, &denominators)
    }
}

impl Mul<IntegerMatrix> for &RationalMatrix {
    type Output = RationalMatrix;

    /// Multiplies a [`RationalMatrix`] by a [`IntegerMatrix`], taking the first by reference and
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
    /// Each row of $A$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_row_denominators`](RationalMatrix::to_numerators_and_row_denominators);
    /// the integer matrix is multiplied by $B$ as by the [`Mul`] implementation for
    /// [`IntegerMatrix`]s, and each row of the product is divided by its denominator and reduced
    /// once. Only the [`RationalMatrix`]'s denominators are cleared, so this is cheaper than
    /// converting $B$ to a [`RationalMatrix`] first.
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the sum of the largest number of
    /// significant bits of any element of either matrix and the number of significant bits of the
    /// least common multiple of the denominators in any row of the first matrix.
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
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// let b = IntegerMatrix::from_str("[2, 1; -1, 0; 1, 3]").unwrap();
    /// assert_eq!((&a * b).to_string(), "[0, 1/2; 5/3, 6]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3/4, 1]").unwrap();
    /// let b = IntegerMatrix::from_str("[3, 0; 6, 4]").unwrap();
    /// assert_eq!((&a * b).to_string(), "[-5/2, -8/3; 33/4, 4]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul_fmpz_mat` from `fmpq_mat/mul_fmpz_mat.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible, which FLINT does not check.
    fn mul(self, other: IntegerMatrix) -> RationalMatrix {
        assert_product_compatible(self.width, usize::exact_from(other.height()));
        let (height, width) = (self.height, usize::exact_from(other.width()));
        let (numerators, denominators) = self.to_numerators_and_row_denominators();
        divide_rows(numerators * other, height, width, &denominators)
    }
}

impl Mul<&IntegerMatrix> for &RationalMatrix {
    type Output = RationalMatrix;

    /// Multiplies a [`RationalMatrix`] by a [`IntegerMatrix`], taking the first by reference and
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
    /// Each row of $A$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_row_denominators`](RationalMatrix::to_numerators_and_row_denominators);
    /// the integer matrix is multiplied by $B$ as by the [`Mul`] implementation for
    /// [`IntegerMatrix`]s, and each row of the product is divided by its denominator and reduced
    /// once. Only the [`RationalMatrix`]'s denominators are cleared, so this is cheaper than
    /// converting $B$ to a [`RationalMatrix`] first.
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the sum of the largest number of
    /// significant bits of any element of either matrix and the number of significant bits of the
    /// least common multiple of the denominators in any row of the first matrix.
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
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// let b = IntegerMatrix::from_str("[2, 1; -1, 0; 1, 3]").unwrap();
    /// assert_eq!((&a * &b).to_string(), "[0, 1/2; 5/3, 6]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3/4, 1]").unwrap();
    /// let b = IntegerMatrix::from_str("[3, 0; 6, 4]").unwrap();
    /// assert_eq!((&a * &b).to_string(), "[-5/2, -8/3; 33/4, 4]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul_fmpz_mat` from `fmpq_mat/mul_fmpz_mat.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible, which FLINT does not check.
    fn mul(self, other: &IntegerMatrix) -> RationalMatrix {
        assert_product_compatible(self.width, usize::exact_from(other.height()));
        let (height, width) = (self.height, usize::exact_from(other.width()));
        let (numerators, denominators) = self.to_numerators_and_row_denominators();
        divide_rows(numerators * other, height, width, &denominators)
    }
}

impl MulAssign<IntegerMatrix> for RationalMatrix {
    /// Multiplies a [`RationalMatrix`] by an [`IntegerMatrix`] in place, taking the
    /// [`IntegerMatrix`] by value.
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
    /// Each row of $A$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_row_denominators`](RationalMatrix::to_numerators_and_row_denominators);
    /// the integer matrix is multiplied by $B$ as by the [`Mul`] implementation for
    /// [`IntegerMatrix`]s, and each row of the product is divided by its denominator and reduced
    /// once. Only the [`RationalMatrix`]'s denominators are cleared, so this is cheaper than
    /// converting $B$ to a [`RationalMatrix`] first.
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the sum of the largest number of
    /// significant bits of any element of either matrix and the number of significant bits of the
    /// least common multiple of the denominators in any row of the first matrix.
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
    /// let mut a = RationalMatrix::from_str("[1/2, -2/3; 3/4, 1]").unwrap();
    /// a *= IntegerMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// assert_eq!(a.to_string(), "[-2/3, 1/2; 1, 3/4]");
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// a *= IntegerMatrix::from_str("[2, 1; -1, 0; 1, 3]").unwrap();
    /// assert_eq!(a.to_string(), "[0, 1/2; 5/3, 6]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul_fmpz_mat` from `fmpq_mat/mul_fmpz_mat.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible, which FLINT does not check.
    fn mul_assign(&mut self, other: IntegerMatrix) {
        assert_product_compatible(self.width, usize::exact_from(other.height()));
        let (height, width) = (self.height, usize::exact_from(other.width()));
        let (numerators, denominators) = take_matrix(self).into_numerators_and_row_denominators();
        *self = divide_rows(numerators * other, height, width, &denominators);
    }
}

impl MulAssign<&IntegerMatrix> for RationalMatrix {
    /// Multiplies a [`RationalMatrix`] by an [`IntegerMatrix`] in place, taking the
    /// [`IntegerMatrix`] by reference.
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
    /// Each row of $A$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_row_denominators`](RationalMatrix::to_numerators_and_row_denominators);
    /// the integer matrix is multiplied by $B$ as by the [`Mul`] implementation for
    /// [`IntegerMatrix`]s, and each row of the product is divided by its denominator and reduced
    /// once. Only the [`RationalMatrix`]'s denominators are cleared, so this is cheaper than
    /// converting $B$ to a [`RationalMatrix`] first.
    ///
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the sum of the largest number of
    /// significant bits of any element of either matrix and the number of significant bits of the
    /// least common multiple of the denominators in any row of the first matrix.
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
    /// let mut a = RationalMatrix::from_str("[1/2, -2/3; 3/4, 1]").unwrap();
    /// a *= &IntegerMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// assert_eq!(a.to_string(), "[-2/3, 1/2; 1, 3/4]");
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// a *= &IntegerMatrix::from_str("[2, 1; -1, 0; 1, 3]").unwrap();
    /// assert_eq!(a.to_string(), "[0, 1/2; 5/3, 6]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul_fmpz_mat` from `fmpq_mat/mul_fmpz_mat.c`, FLINT 3.6.0,
    /// except that it panics if the dimensions are incompatible, which FLINT does not check.
    fn mul_assign(&mut self, other: &IntegerMatrix) {
        assert_product_compatible(self.width, usize::exact_from(other.height()));
        let (height, width) = (self.height, usize::exact_from(other.width()));
        let (numerators, denominators) = take_matrix(self).into_numerators_and_row_denominators();
        *self = divide_rows(numerators * other, height, width, &denominators);
    }
}
