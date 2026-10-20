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

use crate::integer_matrix::arithmetic::mul::{ProductShape, matrix_mul, matrix_mul_assign};
use crate::natural_matrix::NaturalMatrix;
use core::ops::{Mul, MulAssign};

fn assert_mul_compatible(a: &NaturalMatrix, b: &NaturalMatrix) {
    assert_eq!(
        a.width, b.height,
        "the width of the first matrix must equal the height of the second"
    );
}

const fn product_shape(a: &NaturalMatrix, b: &NaturalMatrix) -> ProductShape {
    ProductShape {
        ar: a.height,
        br: b.height,
        bc: b.width,
    }
}

fn mul_helper(a: &NaturalMatrix, b: &NaturalMatrix) -> NaturalMatrix {
    assert_mul_compatible(a, b);
    NaturalMatrix {
        elements: matrix_mul(&a.elements, &b.elements, product_shape(a, b)),
        height: a.height,
        width: b.width,
    }
}

fn mul_assign_helper(a: &mut NaturalMatrix, b: &NaturalMatrix) {
    assert_mul_compatible(a, b);
    let shape = product_shape(a, b);
    matrix_mul_assign(&mut a.elements, &b.elements, shape);
    a.width = b.width;
}

impl Mul<Self> for NaturalMatrix {
    type Output = Self;

    /// Multiplies two [`NaturalMatrix`]s, taking both by value.
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
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the largest number of significant bits of
    /// any element of either matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((a * b).to_string(), "[5, 11; 14, 23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, for matrices with
    /// non-negative elements, except that it panics if the dimensions are incompatible, which FLINT
    /// checks only in debug builds.
    #[inline]
    fn mul(self, other: Self) -> Self {
        mul_helper(&self, &other)
    }
}

impl Mul<&Self> for NaturalMatrix {
    type Output = Self;

    /// Multiplies two [`NaturalMatrix`]s, taking the first by value and the second by reference.
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
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the largest number of significant bits of
    /// any element of either matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((a * &b).to_string(), "[5, 11; 14, 23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, for matrices with
    /// non-negative elements, except that it panics if the dimensions are incompatible, which FLINT
    /// checks only in debug builds.
    #[inline]
    fn mul(self, other: &Self) -> Self {
        mul_helper(&self, other)
    }
}

impl Mul<NaturalMatrix> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies two [`NaturalMatrix`]s, taking the first by reference and the second by value.
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
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the largest number of significant bits of
    /// any element of either matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((&a * b).to_string(), "[5, 11; 14, 23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, for matrices with
    /// non-negative elements, except that it panics if the dimensions are incompatible, which FLINT
    /// checks only in debug builds.
    #[inline]
    fn mul(self, other: NaturalMatrix) -> NaturalMatrix {
        mul_helper(self, &other)
    }
}

impl Mul<&NaturalMatrix> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Multiplies two [`NaturalMatrix`]s, taking both by reference.
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
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the largest number of significant bits of
    /// any element of either matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// let b = NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!((&a * &b).to_string(), "[5, 11; 14, 23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, for matrices with
    /// non-negative elements, except that it panics if the dimensions are incompatible, which FLINT
    /// checks only in debug builds.
    #[inline]
    fn mul(self, other: &NaturalMatrix) -> NaturalMatrix {
        mul_helper(self, other)
    }
}

impl MulAssign<Self> for NaturalMatrix {
    /// Multiplies a [`NaturalMatrix`] by another [`NaturalMatrix`] in place, taking the second by
    /// value.
    ///
    /// The product is written back into the storage of `self`. When every element fits in two
    /// limbs, the elements of `self` are first copied out as words, and the product is written
    /// directly. For large matrices with large elements, Waksman's or Strassen's algorithm computes
    /// the product separately, and it is then moved into the storage of `self`. Otherwise the
    /// product is computed one row at a time, so when it is no wider than `self`, and in particular
    /// when the second matrix is square, the only additional memory is one row.
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
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the largest number of significant bits of
    /// any element of either matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a *= NaturalMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// assert_eq!(a.to_string(), "[2, 1; 4, 3]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// a *= NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!(a.to_string(), "[5, 11; 14, 23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, for matrices with
    /// non-negative elements, except that it panics if the dimensions are incompatible, which FLINT
    /// checks only in debug builds.
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        mul_assign_helper(self, &other);
    }
}

impl MulAssign<&Self> for NaturalMatrix {
    /// Multiplies a [`NaturalMatrix`] by another [`NaturalMatrix`] in place, taking the second by
    /// reference.
    ///
    /// The product is written back into the storage of `self`. When every element fits in two
    /// limbs, the elements of `self` are first copied out as words, and the product is written
    /// directly. For large matrices with large elements, Waksman's or Strassen's algorithm computes
    /// the product separately, and it is then moved into the storage of `self`. Otherwise the
    /// product is computed one row at a time, so when it is no wider than `self`, and in particular
    /// when the second matrix is square, the only additional memory is one row.
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
    /// # Worst-case complexity
    /// $T(n, k, m) = O(nkm \log m \log\log m)$
    ///
    /// $M(n, m) = O(nm)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the number of elements of the product,
    /// $k$ is the width of the first matrix, and $m$ is the largest number of significant bits of
    /// any element of either matrix.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// a *= &NaturalMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// assert_eq!(a.to_string(), "[2, 1; 4, 3]");
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]").unwrap();
    /// a *= &NaturalMatrix::from_str("[1, 0; 2, 1; 0, 3]").unwrap();
    /// assert_eq!(a.to_string(), "[5, 11; 14, 23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, for matrices with
    /// non-negative elements, except that it panics if the dimensions are incompatible, which FLINT
    /// checks only in debug builds.
    #[inline]
    fn mul_assign(&mut self, other: &Self) {
        mul_assign_helper(self, other);
    }
}
