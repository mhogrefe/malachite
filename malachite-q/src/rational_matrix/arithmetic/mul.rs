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

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use alloc::vec::Vec;
use core::iter::repeat_n;
use core::mem;
use core::ops::{Mul, MulAssign};
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;

// Divides each element of `numerators`, in row-major order, by the corresponding denominator, and
// reduces it.
fn divide_by_denominators(
    numerators: IntegerMatrix,
    height: usize,
    width: usize,
    denominators: impl Iterator<Item = Natural>,
) -> RationalMatrix {
    let elements: Vec<Rational> = numerators
        .elements
        .into_iter()
        .zip(denominators)
        .map(|(x, d)| {
            if d == 1u32 {
                Rational::from(x)
            } else {
                Rational::from_sign_and_naturals(x >= 0u32, x.unsigned_abs(), d)
            }
        })
        .collect();
    RationalMatrix {
        elements,
        height,
        width,
    }
}

// Divides each row of `numerators` by the corresponding element of `denominators`, and reduces each
// element.
pub(crate) fn divide_rows(
    numerators: IntegerMatrix,
    height: usize,
    width: usize,
    denominators: &NaturalVector,
) -> RationalMatrix {
    divide_by_denominators(
        numerators,
        height,
        width,
        denominators
            .elements
            .iter()
            .flat_map(|d| repeat_n(d, width))
            .cloned(),
    )
}

// Divides each column of `numerators` by the corresponding element of `denominators`, and reduces
// each element.
pub(crate) fn divide_columns(
    numerators: IntegerMatrix,
    height: usize,
    width: usize,
    denominators: &NaturalVector,
) -> RationalMatrix {
    divide_by_denominators(
        numerators,
        height,
        width,
        denominators.elements.iter().cycle().cloned(),
    )
}

pub(crate) fn assert_product_compatible(a_width: usize, b_height: usize) {
    assert_eq!(
        a_width, b_height,
        "the width of the first matrix must equal the height of the second"
    );
}

// Takes the elements of `a`, leaving it with none, so that an in-place product can consume it.
pub(crate) fn take_matrix(a: &mut RationalMatrix) -> RationalMatrix {
    RationalMatrix {
        elements: mem::take(&mut a.elements),
        height: a.height,
        width: a.width,
    }
}

// The product of two matrices whose rows, and columns, have been cleared of denominators: the
// integer matrices are multiplied, and element $(i, j)$ of the product is divided by the
// denominators of row $i$ of the first matrix and column $j$ of the second, and reduced.
//
// This is equivalent to `fmpq_mat_mul_cleared` from `fmpq_mat/mul_cleared.c`, FLINT 3.6.0, where
// `fmpq_mat_get_fmpz_mat_rowwise` and `fmpq_mat_get_fmpz_mat_colwise` have already cleared the
// denominators.
fn mul_cleared(
    (a_numerators, a_denominators): (IntegerMatrix, NaturalVector),
    (b_numerators, b_denominators): (IntegerMatrix, NaturalVector),
) -> RationalMatrix {
    let (height, width) = (a_denominators.elements.len(), b_denominators.elements.len());
    divide_by_denominators(
        a_numerators * b_numerators,
        height,
        width,
        a_denominators
            .elements
            .iter()
            .flat_map(|d| b_denominators.elements.iter().map(move |e| d * e)),
    )
}

// The product of `a` and `b` when the width of `a`, and the height of `b`, is 1: element $(i, j)$
// is the single product $A_{i0}B_{0j}$, computed in rational arithmetic. With no sums to put over a
// common denominator, clearing denominators gains nothing, and it is up to three times slower.
//
// This is equivalent to `fmpq_mat_mul_direct` from `fmpq_mat/mul_direct.c`, FLINT 3.6.0, for an
// inner dimension of 1.
private_test_fn! {mul_outer(a: &RationalMatrix, b: &RationalMatrix) -> RationalMatrix {
    assert_eq!(a.width, 1);
    assert_eq!(b.height, 1);
    RationalMatrix {
        elements: a
            .elements
            .iter()
            .flat_map(|x| b.elements.iter().map(move |y| x * y))
            .collect(),
        height: a.height,
        width: b.width,
    }
}}

// The product of `a` and `b`, always by clearing denominators, whatever the inner dimension.
private_test_fn! {mul_by_clearing_denominators(
    a: &RationalMatrix,
    b: &RationalMatrix,
) -> RationalMatrix {
    assert_product_compatible(a.width, b.height);
    mul_cleared(
        a.to_numerators_and_row_denominators(),
        b.to_numerators_and_column_denominators(),
    )
}}

impl Mul<Self> for RationalMatrix {
    type Output = Self;

    /// Multiplies two [`RationalMatrix`]s, taking both by value.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix. When $k$ is 1, each element of
    /// the product is a single product of elements, computed in rational arithmetic.
    ///
    /// Otherwise, each row of $A$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_row_denominators`](RationalMatrix::to_numerators_and_row_denominators),
    /// and so is each column of $B$, as by
    /// [`to_numerators_and_column_denominators`][column_denominators]; the two integer matrices are
    /// multiplied as by the [`Mul`] implementation for [`IntegerMatrix`]s, and each element of the
    /// product is divided by its row's and column's denominators and reduced once. A matrix taken
    /// by value is cleared in place, without cloning its numerators.
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
    /// least common multiple of the denominators in any row of the first matrix or column of the
    /// second.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// let b = RationalMatrix::from_str("[2, 1; -1, 1/2; 1/4, 3]").unwrap();
    /// assert_eq!((a * b).to_string(), "[0, 1; 1/6, 37/6]");
    ///
    /// let a = RationalMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let b = RationalMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!((a * b).to_string(), "[-3, 11; 14, -23]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3/4, 1]").unwrap();
    /// let b = RationalMatrix::from_str("[2/3, 0; 1/4, 6]").unwrap();
    /// assert_eq!((a * b).to_string(), "[1/6, -4; 3/4, 6]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul` from `fmpq_mat/mul.c`, FLINT 3.6.0, which calls
    /// `fmpq_mat_mul_cleared` from `fmpq_mat/mul_cleared.c`, except that it panics if the
    /// dimensions are incompatible, which FLINT does not check.
    fn mul(self, other: Self) -> Self {
        assert_product_compatible(self.width, other.height);
        if self.width == 1 {
            mul_outer(&self, &other)
        } else {
            mul_cleared(
                self.into_numerators_and_row_denominators(),
                other.into_numerators_and_column_denominators(),
            )
        }
    }
}

impl Mul<&Self> for RationalMatrix {
    type Output = Self;

    /// Multiplies two [`RationalMatrix`]s, taking the first by value and the second by reference.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix. When $k$ is 1, each element of
    /// the product is a single product of elements, computed in rational arithmetic.
    ///
    /// Otherwise, each row of $A$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_row_denominators`](RationalMatrix::to_numerators_and_row_denominators),
    /// and so is each column of $B$, as by
    /// [`to_numerators_and_column_denominators`][column_denominators]; the two integer matrices are
    /// multiplied as by the [`Mul`] implementation for [`IntegerMatrix`]s, and each element of the
    /// product is divided by its row's and column's denominators and reduced once. A matrix taken
    /// by value is cleared in place, without cloning its numerators.
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
    /// least common multiple of the denominators in any row of the first matrix or column of the
    /// second.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// let b = RationalMatrix::from_str("[2, 1; -1, 1/2; 1/4, 3]").unwrap();
    /// assert_eq!((a * &b).to_string(), "[0, 1; 1/6, 37/6]");
    ///
    /// let a = RationalMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let b = RationalMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!((a * &b).to_string(), "[-3, 11; 14, -23]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3/4, 1]").unwrap();
    /// let b = RationalMatrix::from_str("[2/3, 0; 1/4, 6]").unwrap();
    /// assert_eq!((a * &b).to_string(), "[1/6, -4; 3/4, 6]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul` from `fmpq_mat/mul.c`, FLINT 3.6.0, which calls
    /// `fmpq_mat_mul_cleared` from `fmpq_mat/mul_cleared.c`, except that it panics if the
    /// dimensions are incompatible, which FLINT does not check.
    fn mul(self, other: &Self) -> Self {
        assert_product_compatible(self.width, other.height);
        if self.width == 1 {
            mul_outer(&self, other)
        } else {
            mul_cleared(
                self.into_numerators_and_row_denominators(),
                other.to_numerators_and_column_denominators(),
            )
        }
    }
}

impl Mul<RationalMatrix> for &RationalMatrix {
    type Output = RationalMatrix;

    /// Multiplies two [`RationalMatrix`]s, taking the first by reference and the second by value.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix. When $k$ is 1, each element of
    /// the product is a single product of elements, computed in rational arithmetic.
    ///
    /// Otherwise, each row of $A$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_row_denominators`](RationalMatrix::to_numerators_and_row_denominators),
    /// and so is each column of $B$, as by
    /// [`to_numerators_and_column_denominators`][column_denominators]; the two integer matrices are
    /// multiplied as by the [`Mul`] implementation for [`IntegerMatrix`]s, and each element of the
    /// product is divided by its row's and column's denominators and reduced once. A matrix taken
    /// by value is cleared in place, without cloning its numerators.
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
    /// least common multiple of the denominators in any row of the first matrix or column of the
    /// second.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// let b = RationalMatrix::from_str("[2, 1; -1, 1/2; 1/4, 3]").unwrap();
    /// assert_eq!((&a * b).to_string(), "[0, 1; 1/6, 37/6]");
    ///
    /// let a = RationalMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let b = RationalMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!((&a * b).to_string(), "[-3, 11; 14, -23]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3/4, 1]").unwrap();
    /// let b = RationalMatrix::from_str("[2/3, 0; 1/4, 6]").unwrap();
    /// assert_eq!((&a * b).to_string(), "[1/6, -4; 3/4, 6]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul` from `fmpq_mat/mul.c`, FLINT 3.6.0, which calls
    /// `fmpq_mat_mul_cleared` from `fmpq_mat/mul_cleared.c`, except that it panics if the
    /// dimensions are incompatible, which FLINT does not check.
    fn mul(self, other: RationalMatrix) -> RationalMatrix {
        assert_product_compatible(self.width, other.height);
        if self.width == 1 {
            mul_outer(self, &other)
        } else {
            mul_cleared(
                self.to_numerators_and_row_denominators(),
                other.into_numerators_and_column_denominators(),
            )
        }
    }
}

impl Mul<&RationalMatrix> for &RationalMatrix {
    type Output = RationalMatrix;

    /// Multiplies two [`RationalMatrix`]s, taking both by reference.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix. When $k$ is 1, each element of
    /// the product is a single product of elements, computed in rational arithmetic.
    ///
    /// Otherwise, each row of $A$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_row_denominators`](RationalMatrix::to_numerators_and_row_denominators),
    /// and so is each column of $B$, as by
    /// [`to_numerators_and_column_denominators`][column_denominators]; the two integer matrices are
    /// multiplied as by the [`Mul`] implementation for [`IntegerMatrix`]s, and each element of the
    /// product is divided by its row's and column's denominators and reduced once. A matrix taken
    /// by value is cleared in place, without cloning its numerators.
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
    /// least common multiple of the denominators in any row of the first matrix or column of the
    /// second.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// let b = RationalMatrix::from_str("[2, 1; -1, 1/2; 1/4, 3]").unwrap();
    /// assert_eq!((&a * &b).to_string(), "[0, 1; 1/6, 37/6]");
    ///
    /// let a = RationalMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let b = RationalMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!((&a * &b).to_string(), "[-3, 11; 14, -23]");
    ///
    /// let a = RationalMatrix::from_str("[1/2, -2/3; 3/4, 1]").unwrap();
    /// let b = RationalMatrix::from_str("[2/3, 0; 1/4, 6]").unwrap();
    /// assert_eq!((&a * &b).to_string(), "[1/6, -4; 3/4, 6]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul` from `fmpq_mat/mul.c`, FLINT 3.6.0, which calls
    /// `fmpq_mat_mul_cleared` from `fmpq_mat/mul_cleared.c`, except that it panics if the
    /// dimensions are incompatible, which FLINT does not check.
    fn mul(self, other: &RationalMatrix) -> RationalMatrix {
        assert_product_compatible(self.width, other.height);
        if self.width == 1 {
            mul_outer(self, other)
        } else {
            mul_cleared(
                self.to_numerators_and_row_denominators(),
                other.to_numerators_and_column_denominators(),
            )
        }
    }
}

impl MulAssign<Self> for RationalMatrix {
    /// Multiplies a [`RationalMatrix`] by another [`RationalMatrix`] in place, taking the second by
    /// value.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix. When $k$ is 1, each element of
    /// the product is a single product of elements, computed in rational arithmetic.
    ///
    /// Otherwise, each row of $A$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_row_denominators`](RationalMatrix::to_numerators_and_row_denominators),
    /// and so is each column of $B$, as by
    /// [`to_numerators_and_column_denominators`][column_denominators]; the two integer matrices are
    /// multiplied as by the [`Mul`] implementation for [`IntegerMatrix`]s, and each element of the
    /// product is divided by its row's and column's denominators and reduced once. A matrix taken
    /// by value is cleared in place, without cloning its numerators.
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
    /// least common multiple of the denominators in any row of the first matrix or column of the
    /// second.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, -2/3; 3/4, 1]").unwrap();
    /// a *= RationalMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// assert_eq!(a.to_string(), "[-2/3, 1/2; 1, 3/4]");
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// a *= RationalMatrix::from_str("[2, 1; -1, 1/2; 1/4, 3]").unwrap();
    /// assert_eq!(a.to_string(), "[0, 1; 1/6, 37/6]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul` from `fmpq_mat/mul.c`, FLINT 3.6.0, which calls
    /// `fmpq_mat_mul_cleared` from `fmpq_mat/mul_cleared.c`, except that it panics if the
    /// dimensions are incompatible, which FLINT does not check.
    fn mul_assign(&mut self, other: Self) {
        assert_product_compatible(self.width, other.height);
        *self = if self.width == 1 {
            mul_outer(self, &other)
        } else {
            mul_cleared(
                take_matrix(self).into_numerators_and_row_denominators(),
                other.into_numerators_and_column_denominators(),
            )
        };
    }
}

impl MulAssign<&Self> for RationalMatrix {
    /// Multiplies a [`RationalMatrix`] by another [`RationalMatrix`] in place, taking the second by
    /// reference.
    ///
    /// The product $AB$ of an $h \times k$ matrix $A$ and a $k \times w$ matrix $B$ is the $h
    /// \times w$ matrix whose element in row $i$ and column $j$ is the dot product of row $i$ of
    /// $A$ with column $j$ of $B$:
    ///
    /// $$
    /// f(A, B)_{ij} = \sum_{l=0}^{k-1} A_{il}B_{lj}.
    /// $$
    ///
    /// When $k$ is 0, the product is the $h \times w$ zero matrix. When $k$ is 1, each element of
    /// the product is a single product of elements, computed in rational arithmetic.
    ///
    /// Otherwise, each row of $A$ is written as integers over a common denominator, as by
    /// [`to_numerators_and_row_denominators`](RationalMatrix::to_numerators_and_row_denominators),
    /// and so is each column of $B$, as by
    /// [`to_numerators_and_column_denominators`][column_denominators]; the two integer matrices are
    /// multiplied as by the [`Mul`] implementation for [`IntegerMatrix`]s, and each element of the
    /// product is divided by its row's and column's denominators and reduced once. A matrix taken
    /// by value is cleared in place, without cloning its numerators.
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
    /// least common multiple of the denominators in any row of the first matrix or column of the
    /// second.
    ///
    /// # Panics
    /// Panics if the width of the first matrix is not the height of the second.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, -2/3; 3/4, 1]").unwrap();
    /// a *= &RationalMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// assert_eq!(a.to_string(), "[-2/3, 1/2; 1, 3/4]");
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, 1, 0; 0, 1/3, 2]").unwrap();
    /// a *= &RationalMatrix::from_str("[2, 1; -1, 1/2; 1/4, 3]").unwrap();
    /// assert_eq!(a.to_string(), "[0, 1; 1/6, 37/6]");
    /// ```
    ///
    /// This is equivalent to `fmpq_mat_mul` from `fmpq_mat/mul.c`, FLINT 3.6.0, which calls
    /// `fmpq_mat_mul_cleared` from `fmpq_mat/mul_cleared.c`, except that it panics if the
    /// dimensions are incompatible, which FLINT does not check.
    fn mul_assign(&mut self, other: &Self) {
        assert_product_compatible(self.width, other.height);
        *self = if self.width == 1 {
            mul_outer(self, other)
        } else {
            mul_cleared(
                take_matrix(self).into_numerators_and_row_denominators(),
                other.to_numerators_and_column_denominators(),
            )
        };
    }
}
