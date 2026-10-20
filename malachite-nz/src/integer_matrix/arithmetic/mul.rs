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

use crate::integer_matrix::IntegerMatrix;
use crate::integer_polynomial::arithmetic::coefficient::PolynomialCoefficient;
use crate::integer_vector::arithmetic::dot_general::vec_dot_general_strided;
use alloc::vec;
use alloc::vec::Vec;
use core::ops::{Mul, MulAssign};

// Appends to `out` the product of `row`, a row of `br` elements, and the `br` by `bc` matrix with
// elements `ys` in row-major order: one row of a matrix product. `br` must be positive.
fn mul_row<C: PolynomialCoefficient>(row: &[C], ys: &[C], bc: usize, out: &mut Vec<C>) {
    match *row {
        [ref x] => {
            for y in ys {
                out.push(x.mul_ref(y));
            }
        }
        [ref x_0, ref x_1] => {
            let (ys_0, ys_1) = ys.split_at(bc);
            for (y_0, y_1) in ys_0.iter().zip(ys_1) {
                let mut c = x_0.mul_ref(y_0);
                c.add_mul_assign(x_1, y_1);
                out.push(c);
            }
        }
        _ => {
            // Column `j` of the second matrix starts at element `j` and continues every `bc`
            // elements.
            for j in 0..bc {
                out.push(vec_dot_general_strided(
                    None,
                    false,
                    row,
                    &ys[j..],
                    bc,
                    false,
                ));
            }
        }
    }
}

// Returns the elements of the product of the `ar` by `br` matrix with elements `xs` and the `br` by
// `bc` matrix with elements `ys`, both in row-major order.
//
// This is equivalent to `fmpz_mat_mul_classical` from `fmpz_mat/mul_classical.c`, FLINT 3.6.0,
// except that the columns of the second matrix are read in place, at a stride, rather than through
// a transposed array of pointers.
crate_test_fn! {matrix_mul_classical<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    ar: usize,
    br: usize,
    bc: usize,
) -> Vec<C> {
    if ar == 0 || br == 0 || bc == 0 {
        return vec![C::ZERO; ar * bc];
    }
    let mut out = Vec::with_capacity(ar * bc);
    for row in xs.chunks_exact(br) {
        mul_row(row, ys, bc, &mut out);
    }
    out
}}

// Replaces the elements `xs` of an `ar` by `br` matrix with those of its product with the `br` by
// `bc` matrix with elements `ys`, both in row-major order, reusing the storage of `xs`.
//
// Row `i` of the product depends only on row `i` of the first matrix, so each row is computed into
// a one-row buffer and then written back. When the product is no wider than the first matrix, the
// rows are written from the top, and row `i` only overwrites rows that have already been used; when
// it is wider, the elements are extended first and the rows are written from the bottom, for the
// same reason. When the second matrix is square, the shape is unchanged and the only additional
// memory is the buffer.
//
// This is equivalent to `fmpz_mat_mul_classical` from `fmpz_mat/mul_classical.c`, FLINT 3.6.0, with
// the output the same as the first input.
crate_test_fn! {matrix_mul_classical_assign<C: PolynomialCoefficient>(
    xs: &mut Vec<C>,
    ys: &[C],
    ar: usize,
    br: usize,
    bc: usize,
) {
    if ar == 0 || bc == 0 {
        xs.clear();
        return;
    }
    if br == 0 {
        xs.resize(ar * bc, C::ZERO);
        return;
    }
    let mut row = Vec::with_capacity(bc);
    let mut write_row = |xs: &mut Vec<C>, i: usize| {
        mul_row(&xs[i * br..(i + 1) * br], ys, bc, &mut row);
        for (x, y) in xs[i * bc..(i + 1) * bc].iter_mut().zip(row.drain(..)) {
            *x = y;
        }
    };
    if bc <= br {
        for i in 0..ar {
            write_row(xs, i);
        }
        xs.truncate(ar * bc);
    } else {
        xs.resize(ar * bc, C::ZERO);
        for i in (0..ar).rev() {
            write_row(xs, i);
        }
    }
}}

// Returns the elements of the product of the `ar` by `br` matrix with elements `xs` and the `br` by
// `bc` matrix with elements `ys`, both in row-major order, choosing an algorithm.
//
// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, restricted to the
// algorithms ported so far: only the classical one.
#[inline]
pub(crate) fn matrix_mul<C: PolynomialCoefficient>(
    xs: &[C],
    ys: &[C],
    ar: usize,
    br: usize,
    bc: usize,
) -> Vec<C> {
    matrix_mul_classical(xs, ys, ar, br, bc)
}

// Replaces the elements `xs` of an `ar` by `br` matrix with those of its product with the `br` by
// `bc` matrix with elements `ys`, choosing an algorithm.
//
// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, with the output the same
// as the first input, restricted to the algorithms ported so far: only the classical one.
#[inline]
pub(crate) fn matrix_mul_assign<C: PolynomialCoefficient>(
    xs: &mut Vec<C>,
    ys: &[C],
    ar: usize,
    br: usize,
    bc: usize,
) {
    matrix_mul_classical_assign(xs, ys, ar, br, bc);
}

fn assert_mul_compatible(a: &IntegerMatrix, b: &IntegerMatrix) {
    assert_eq!(
        a.width, b.height,
        "the width of the first matrix must equal the height of the second"
    );
}

fn mul_helper(a: &IntegerMatrix, b: &IntegerMatrix) -> IntegerMatrix {
    assert_mul_compatible(a, b);
    IntegerMatrix {
        elements: matrix_mul(&a.elements, &b.elements, a.height, b.height, b.width),
        height: a.height,
        width: b.width,
    }
}

fn mul_assign_helper(a: &mut IntegerMatrix, b: &IntegerMatrix) {
    assert_mul_compatible(a, b);
    matrix_mul_assign(&mut a.elements, &b.elements, a.height, b.height, b.width);
    a.width = b.width;
}

impl Mul<Self> for IntegerMatrix {
    type Output = Self;

    /// Multiplies two [`IntegerMatrix`]s, taking both by value.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let b = IntegerMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!((a * b).to_string(), "[-3, 11; 14, -23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that it
    /// panics if the dimensions are incompatible, which FLINT checks only in debug builds.
    #[inline]
    fn mul(self, other: Self) -> Self {
        mul_helper(&self, &other)
    }
}

impl Mul<&Self> for IntegerMatrix {
    type Output = Self;

    /// Multiplies two [`IntegerMatrix`]s, taking the first by value and the second by reference.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let b = IntegerMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!((a * &b).to_string(), "[-3, 11; 14, -23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that it
    /// panics if the dimensions are incompatible, which FLINT checks only in debug builds.
    #[inline]
    fn mul(self, other: &Self) -> Self {
        mul_helper(&self, other)
    }
}

impl Mul<IntegerMatrix> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Multiplies two [`IntegerMatrix`]s, taking the first by reference and the second by value.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let b = IntegerMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!((&a * b).to_string(), "[-3, 11; 14, -23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that it
    /// panics if the dimensions are incompatible, which FLINT checks only in debug builds.
    #[inline]
    fn mul(self, other: IntegerMatrix) -> IntegerMatrix {
        mul_helper(self, &other)
    }
}

impl Mul<&IntegerMatrix> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Multiplies two [`IntegerMatrix`]s, taking both by reference.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// let b = IntegerMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!((&a * &b).to_string(), "[-3, 11; 14, -23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that it
    /// panics if the dimensions are incompatible, which FLINT checks only in debug builds.
    #[inline]
    fn mul(self, other: &IntegerMatrix) -> IntegerMatrix {
        mul_helper(self, other)
    }
}

impl MulAssign<Self> for IntegerMatrix {
    /// Multiplies an [`IntegerMatrix`] by another [`IntegerMatrix`] in place, taking the second by
    /// value.
    ///
    /// The product is computed one row at a time and written back into the storage of `self`, so
    /// when the product is no wider than `self`, and in particular when the second matrix is
    /// square, the only additional memory is one row.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// a *= IntegerMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// assert_eq!(a.to_string(), "[-2, 1; 4, 3]");
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// a *= IntegerMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!(a.to_string(), "[-3, 11; 14, -23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that it
    /// panics if the dimensions are incompatible, which FLINT checks only in debug builds.
    #[inline]
    fn mul_assign(&mut self, other: Self) {
        mul_assign_helper(self, &other);
    }
}

impl MulAssign<&Self> for IntegerMatrix {
    /// Multiplies an [`IntegerMatrix`] by another [`IntegerMatrix`] in place, taking the second by
    /// reference.
    ///
    /// The product is computed one row at a time and written back into the storage of `self`, so
    /// when the product is no wider than `self`, and in particular when the second matrix is
    /// square, the only additional memory is one row.
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
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2; 3, 4]").unwrap();
    /// a *= &IntegerMatrix::from_str("[0, 1; 1, 0]").unwrap();
    /// assert_eq!(a.to_string(), "[-2, 1; 4, 3]");
    ///
    /// let mut a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6]").unwrap();
    /// a *= &IntegerMatrix::from_str("[1, 0; 2, -1; 0, 3]").unwrap();
    /// assert_eq!(a.to_string(), "[-3, 11; 14, -23]");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_mul` from `fmpz_mat/mul.c`, FLINT 3.6.0, except that it
    /// panics if the dimensions are incompatible, which FLINT checks only in debug builds.
    #[inline]
    fn mul_assign(&mut self, other: &Self) {
        mul_assign_helper(self, other);
    }
}
