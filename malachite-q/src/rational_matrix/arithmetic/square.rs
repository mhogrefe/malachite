// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::rational_matrix::RationalMatrix;
use crate::rational_matrix::arithmetic::mul::{divide_all, mul_cleared, mul_outer, take_matrix};
use malachite_base::num::arithmetic::traits::{Square, SquareAssign};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;

fn assert_square(a: &RationalMatrix) {
    assert_eq!(
        a.height, a.width,
        "the matrix must be square, but it is {} by {}",
        a.height, a.width
    );
}

// The common denominator of every row and every column, if they all have the same one.
fn uniform_denominator<'a>(rows: &'a [Natural], columns: &[Natural]) -> Option<&'a Natural> {
    let d = rows.first()?;
    if rows.iter().chain(columns).all(|e| e == d) {
        Some(d)
    } else {
        None
    }
}

// The square of a matrix with a uniform denominator $d$, given its numerators $N$: $N^2/d^2$.
fn square_uniform(numerators: &IntegerMatrix, n: usize, d: &Natural) -> RationalMatrix {
    divide_all(numerators.square(), n, n, &d.square())
}

// The square of `a`. When every row and column has the same denominator, the row-cleared and
// column-cleared integer matrices are one matrix, which is squared; otherwise the matrix is
// multiplied by itself.
fn square_ref(a: &RationalMatrix) -> RationalMatrix {
    assert_square(a);
    if a.width == 1 {
        return mul_outer(a, a);
    }
    let row_denominators = a.row_denominators();
    let column_denominators = a.column_denominators();
    if let Some(d) = uniform_denominator(&row_denominators, &column_denominators) {
        square_uniform(&a.scale_by_row_denominators(&row_denominators), a.height, d)
    } else {
        let rows = a.scale_by_row_denominators(&row_denominators);
        let columns = a.scale_by_column_denominators(&column_denominators);
        mul_cleared(
            (
                rows,
                NaturalVector {
                    elements: row_denominators,
                },
            ),
            (
                columns,
                NaturalVector {
                    elements: column_denominators,
                },
            ),
        )
    }
}

// The square of `a`, taking it by value, so that its numerators are moved into one of the integer
// matrices rather than cloned.
fn square_owned(a: RationalMatrix) -> RationalMatrix {
    assert_square(&a);
    if a.width == 1 {
        return mul_outer(&a, &a);
    }
    let row_denominators = a.row_denominators();
    let column_denominators = a.column_denominators();
    if let Some(d) = uniform_denominator(&row_denominators, &column_denominators) {
        let n = a.height;
        square_uniform(&a.into_scaled_by_row_denominators(&row_denominators), n, d)
    } else {
        let rows = a.scale_by_row_denominators(&row_denominators);
        let columns = a.into_scaled_by_column_denominators(&column_denominators);
        mul_cleared(
            (
                rows,
                NaturalVector {
                    elements: row_denominators,
                },
            ),
            (
                columns,
                NaturalVector {
                    elements: column_denominators,
                },
            ),
        )
    }
}

impl Square for RationalMatrix {
    type Output = Self;

    /// Squares a square [`RationalMatrix`], taking it by value.
    ///
    /// The square $A^2 = AA$ of an $n \times n$ matrix $A$ is the $n \times n$ matrix whose element
    /// in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of $A$:
    ///
    /// $$
    /// f(A)_{ij} = \sum_{k=0}^{n-1} A_{ik}A_{kj}.
    /// $$
    ///
    /// When every row and every column of $A$ has the same common denominator $d$, as when $A$ is
    /// an integer matrix divided by $d$, $A$ is $N/d$ for an integer matrix $N$, and the square is
    /// $N^2/d^2$, with $N^2$ computed as by the [`Square`] implementation for [`IntegerMatrix`]s,
    /// which uses Bodrato's algorithm for small matrices. Otherwise the square is the product of
    /// the matrix with itself, computed as by [`Mul`](core::ops::Mul): each row of $A$ and each
    /// column of $A$ is written as integers over a common denominator, the two integer matrices are
    /// multiplied, and each element of the product is divided by its row's and column's
    /// denominators and reduced once. A $1 \times 1$ matrix's element is squared directly.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the sum of the largest number of significant bits of any of its elements and the number of
    /// significant bits of the least common multiple of the denominators in any of its rows or
    /// columns.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// assert_eq!(a.square().to_string(), "[-5/12, 5/12; -5/2, -5/48]");
    ///
    /// let a = RationalMatrix::from_str("[2, 0, 1/2; 1, 1/3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     a.square().to_string(),
    ///     "[4, 1/2, 3/2; 7/3, 1/9, 1/2; 1, 4/3, 1]"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[1/6, -5/6; 7/6, 1/6]").unwrap();
    /// assert_eq!(a.square().to_string(), "[-17/18, -5/18; 7/18, -17/18]");
    ///
    /// let a = RationalMatrix::from_str("[-2/3]").unwrap();
    /// assert_eq!(a.square().to_string(), "[4/9]");
    /// ```
    #[inline]
    fn square(self) -> Self {
        square_owned(self)
    }
}

impl Square for &RationalMatrix {
    type Output = RationalMatrix;

    /// Squares a square [`RationalMatrix`], taking it by reference.
    ///
    /// The square $A^2 = AA$ of an $n \times n$ matrix $A$ is the $n \times n$ matrix whose element
    /// in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of $A$:
    ///
    /// $$
    /// f(A)_{ij} = \sum_{k=0}^{n-1} A_{ik}A_{kj}.
    /// $$
    ///
    /// When every row and every column of $A$ has the same common denominator $d$, as when $A$ is
    /// an integer matrix divided by $d$, $A$ is $N/d$ for an integer matrix $N$, and the square is
    /// $N^2/d^2$, with $N^2$ computed as by the [`Square`] implementation for [`IntegerMatrix`]s,
    /// which uses Bodrato's algorithm for small matrices. Otherwise the square is the product of
    /// the matrix with itself, computed as by [`Mul`](core::ops::Mul): each row of $A$ and each
    /// column of $A$ is written as integers over a common denominator, the two integer matrices are
    /// multiplied, and each element of the product is divided by its row's and column's
    /// denominators and reduced once. A $1 \times 1$ matrix's element is squared directly.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the sum of the largest number of significant bits of any of its elements and the number of
    /// significant bits of the least common multiple of the denominators in any of its rows or
    /// columns.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::Square;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// assert_eq!((&a).square().to_string(), "[-5/12, 5/12; -5/2, -5/48]");
    ///
    /// let a = RationalMatrix::from_str("[2, 0, 1/2; 1, 1/3, 0; 0, 1, 1]").unwrap();
    /// assert_eq!(
    ///     (&a).square().to_string(),
    ///     "[4, 1/2, 3/2; 7/3, 1/9, 1/2; 1, 4/3, 1]"
    /// );
    ///
    /// let a = RationalMatrix::from_str("[1/6, -5/6; 7/6, 1/6]").unwrap();
    /// assert_eq!((&a).square().to_string(), "[-17/18, -5/18; 7/18, -17/18]");
    ///
    /// let a = RationalMatrix::from_str("[-2/3]").unwrap();
    /// assert_eq!((&a).square().to_string(), "[4/9]");
    /// ```
    #[inline]
    fn square(self) -> RationalMatrix {
        square_ref(self)
    }
}

impl SquareAssign for RationalMatrix {
    /// Squares a square [`RationalMatrix`] in place.
    ///
    /// The square $A^2 = AA$ of an $n \times n$ matrix $A$ is the $n \times n$ matrix whose element
    /// in row $i$ and column $j$ is the dot product of row $i$ of $A$ with column $j$ of $A$:
    ///
    /// $$
    /// f(A)_{ij} = \sum_{k=0}^{n-1} A_{ik}A_{kj}.
    /// $$
    ///
    /// When every row and every column of $A$ has the same common denominator $d$, as when $A$ is
    /// an integer matrix divided by $d$, $A$ is $N/d$ for an integer matrix $N$, and the square is
    /// $N^2/d^2$, with $N^2$ computed as by the [`Square`] implementation for [`IntegerMatrix`]s,
    /// which uses Bodrato's algorithm for small matrices. Otherwise the square is the product of
    /// the matrix with itself, computed as by [`Mul`](core::ops::Mul): each row of $A$ and each
    /// column of $A$ is written as integers over a common denominator, the two integer matrices are
    /// multiplied, and each element of the product is divided by its row's and column's
    /// denominators and reduced once. A $1 \times 1$ matrix's element is squared directly.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^3 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the sum of the largest number of significant bits of any of its elements and the number of
    /// significant bits of the least common multiple of the denominators in any of its rows or
    /// columns.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SquareAssign;
    /// use malachite_q::rational_matrix::RationalMatrix;
    ///
    /// let mut a = RationalMatrix::from_str("[1/2, 1/3; -2, 3/4]").unwrap();
    /// a.square_assign();
    /// assert_eq!(a.to_string(), "[-5/12, 5/12; -5/2, -5/48]");
    ///
    /// let mut a = RationalMatrix::from_str("[2, 0, 1/2; 1, 1/3, 0; 0, 1, 1]").unwrap();
    /// a.square_assign();
    /// assert_eq!(a.to_string(), "[4, 1/2, 3/2; 7/3, 1/9, 1/2; 1, 4/3, 1]");
    ///
    /// let mut a = RationalMatrix::from_str("[1/6, -5/6; 7/6, 1/6]").unwrap();
    /// a.square_assign();
    /// assert_eq!(a.to_string(), "[-17/18, -5/18; 7/18, -17/18]");
    ///
    /// let mut a = RationalMatrix::from_str("[-2/3]").unwrap();
    /// a.square_assign();
    /// assert_eq!(a.to_string(), "[4/9]");
    /// ```
    #[inline]
    fn square_assign(&mut self) {
        *self = square_owned(take_matrix(self));
    }
}
