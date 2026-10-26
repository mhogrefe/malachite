// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2015 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use malachite_base::num::arithmetic::traits::{
    SimilarityTransform, SimilarityTransformAssign, SubMulAssign,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;

// Replaces the square matrix `a` with $P^{-1}AP$, where $P$ is the identity matrix with the
// elements of row `r` in the columns other than `r - 1` and `r` replaced by `d`: first `d` times
// column `r` is added to each of those columns, and then `d` times each of the corresponding rows
// is subtracted from row `r`.
//
// This is equivalent to `fmpz_mat_similarity` from `fmpz_mat/similarity.c`, FLINT 3.6.0, except
// that it panics if the matrix is not square or `r` is not a row index, which FLINT does not check,
// and that it multiplies by `d` once per row in the first step and once per column in the second,
// rather than once per element.
fn similarity_transform_helper(a: &mut IntegerMatrix, r: u64, d: &Integer) {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    assert!(
        r < u64::exact_from(n),
        "r must be less than the dimension of the matrix, {n}, but it is {r}"
    );
    if *d == 0u32 {
        return;
    }
    let r = usize::exact_from(r);
    // The columns, and rows, that P mixes with column, and row, r.
    let mixed = |j: usize| j != r && j + 1 != r;
    // A := AP
    for row in a.elements.chunks_exact_mut(n) {
        if row[r] == 0u32 {
            continue;
        }
        let x = &row[r] * d;
        for (j, y) in row.iter_mut().enumerate() {
            if mixed(j) {
                *y += &x;
            }
        }
    }
    // A := P^{-1}A, where P^{-1} is P with -d in place of d
    for j in 0..n {
        let mut sum = Integer::ZERO;
        for k in (0..n).filter(|&k| mixed(k)) {
            sum += &a.elements[k * n + j];
        }
        a.elements[r * n + j].sub_mul_assign(sum, d);
    }
}

impl SimilarityTransformAssign<Integer> for IntegerMatrix {
    /// Applies a similarity transform to a square [`IntegerMatrix`] in place, taking the scalar by
    /// value.
    ///
    /// Let $A$ be the $n \times n$ matrix, $r$ a row index, $0 \leq r < n$, and $d$ the scalar. Let
    /// $S$ be the set of column indices $j$ with $0 \leq j < n$, $j \neq r$, and $j \neq r - 1$; so
    /// for $r = 0$ it is every index but 0, and for $r > 0$ it leaves out both $r - 1$ and $r$. Let
    /// $P$ be the identity matrix with the element in row $r$ and column $j$ replaced by $d$ for
    /// every $j \in S$. Then $P^{-1}$ is the same matrix with $-d$ in place of $d$, and the result
    /// is
    /// $$
    /// f(A, r, d) = P^{-1}AP.
    /// $$
    /// Element by element: writing $C = AP$,
    /// $$
    /// C_{ij} = \begin{cases}
    ///     A_{ij} + dA_{ir} & \text{if} \quad j \in S, \\\\
    ///     A_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times column $r$ is added to each column in $S$; and then the result $B$ is
    /// $$
    /// B_{ij} = \begin{cases}
    ///     C_{ij} - d\sum_{k \in S} C_{kj} & \text{if} \quad i = r, \\\\
    ///     C_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times each row in $S$ is subtracted from row $r$.
    ///
    /// The result is similar to $A$, so it has the same determinant, trace, rank, characteristic
    /// polynomial, and minimal polynomial. If $d = 0$, or if $S$ is empty (when $n = 1$, or $n = 2$
    /// and $r = 1$), the result is $A$.
    ///
    /// This is the transform that FLINT's `fmpz_mat_similarity` computes. FLINT's manual describes
    /// $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this follows,
    /// also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r > 0$. Both
    /// are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the sum of the largest number of significant bits of its elements and the number of
    /// significant bits of `d`.
    ///
    /// # Panics
    /// Panics if the matrix is not square or if `r` is greater than or equal to its dimension, so
    /// always for the $0 \times 0$ matrix. FLINT checks neither.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SimilarityTransformAssign;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// a.similarity_transform_assign(0, Integer::TWO);
    /// assert_eq!(a.to_string(), "[-21, -66, -71; 4, 13, 14; 7, 22, 24]");
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let mut a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// a.similarity_transform_assign(2, Integer::from(5));
    /// assert_eq!(a.to_string(), "[16, 2, 3; 34, 5, 6; -23, -2, -5]");
    ///
    /// // Here S is empty, so nothing changes.
    /// let mut a = IntegerMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// a.similarity_transform_assign(1, Integer::from(4));
    /// assert_eq!(a.to_string(), "[5, 6; 7, 8]");
    /// ```
    #[inline]
    fn similarity_transform_assign(&mut self, r: u64, d: Integer) {
        similarity_transform_helper(self, r, &d);
    }
}

impl SimilarityTransformAssign<&Integer> for IntegerMatrix {
    /// Applies a similarity transform to a square [`IntegerMatrix`] in place, taking the scalar by
    /// reference.
    ///
    /// Let $A$ be the $n \times n$ matrix, $r$ a row index, $0 \leq r < n$, and $d$ the scalar. Let
    /// $S$ be the set of column indices $j$ with $0 \leq j < n$, $j \neq r$, and $j \neq r - 1$; so
    /// for $r = 0$ it is every index but 0, and for $r > 0$ it leaves out both $r - 1$ and $r$. Let
    /// $P$ be the identity matrix with the element in row $r$ and column $j$ replaced by $d$ for
    /// every $j \in S$. Then $P^{-1}$ is the same matrix with $-d$ in place of $d$, and the result
    /// is
    /// $$
    /// f(A, r, d) = P^{-1}AP.
    /// $$
    /// Element by element: writing $C = AP$,
    /// $$
    /// C_{ij} = \begin{cases}
    ///     A_{ij} + dA_{ir} & \text{if} \quad j \in S, \\\\
    ///     A_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times column $r$ is added to each column in $S$; and then the result $B$ is
    /// $$
    /// B_{ij} = \begin{cases}
    ///     C_{ij} - d\sum_{k \in S} C_{kj} & \text{if} \quad i = r, \\\\
    ///     C_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times each row in $S$ is subtracted from row $r$.
    ///
    /// The result is similar to $A$, so it has the same determinant, trace, rank, characteristic
    /// polynomial, and minimal polynomial. If $d = 0$, or if $S$ is empty (when $n = 1$, or $n = 2$
    /// and $r = 1$), the result is $A$.
    ///
    /// This is the transform that FLINT's `fmpz_mat_similarity` computes. FLINT's manual describes
    /// $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this follows,
    /// also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r > 0$. Both
    /// are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the sum of the largest number of significant bits of its elements and the number of
    /// significant bits of `d`.
    ///
    /// # Panics
    /// Panics if the matrix is not square or if `r` is greater than or equal to its dimension, so
    /// always for the $0 \times 0$ matrix. FLINT checks neither.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SimilarityTransformAssign;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let mut a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// a.similarity_transform_assign(0, &Integer::TWO);
    /// assert_eq!(a.to_string(), "[-21, -66, -71; 4, 13, 14; 7, 22, 24]");
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let mut a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// a.similarity_transform_assign(2, &Integer::from(5));
    /// assert_eq!(a.to_string(), "[16, 2, 3; 34, 5, 6; -23, -2, -5]");
    ///
    /// // Here S is empty, so nothing changes.
    /// let mut a = IntegerMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// a.similarity_transform_assign(1, &Integer::from(4));
    /// assert_eq!(a.to_string(), "[5, 6; 7, 8]");
    /// ```
    #[inline]
    fn similarity_transform_assign(&mut self, r: u64, d: &Integer) {
        similarity_transform_helper(self, r, d);
    }
}

impl SimilarityTransform<Integer> for IntegerMatrix {
    type Output = Self;

    /// Applies a similarity transform to a square [`IntegerMatrix`], taking the matrix by value and
    /// the scalar by value.
    ///
    /// Let $A$ be the $n \times n$ matrix, $r$ a row index, $0 \leq r < n$, and $d$ the scalar. Let
    /// $S$ be the set of column indices $j$ with $0 \leq j < n$, $j \neq r$, and $j \neq r - 1$; so
    /// for $r = 0$ it is every index but 0, and for $r > 0$ it leaves out both $r - 1$ and $r$. Let
    /// $P$ be the identity matrix with the element in row $r$ and column $j$ replaced by $d$ for
    /// every $j \in S$. Then $P^{-1}$ is the same matrix with $-d$ in place of $d$, and the result
    /// is
    /// $$
    /// f(A, r, d) = P^{-1}AP.
    /// $$
    /// Element by element: writing $C = AP$,
    /// $$
    /// C_{ij} = \begin{cases}
    ///     A_{ij} + dA_{ir} & \text{if} \quad j \in S, \\\\
    ///     A_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times column $r$ is added to each column in $S$; and then the result $B$ is
    /// $$
    /// B_{ij} = \begin{cases}
    ///     C_{ij} - d\sum_{k \in S} C_{kj} & \text{if} \quad i = r, \\\\
    ///     C_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times each row in $S$ is subtracted from row $r$.
    ///
    /// The result is similar to $A$, so it has the same determinant, trace, rank, characteristic
    /// polynomial, and minimal polynomial. If $d = 0$, or if $S$ is empty (when $n = 1$, or $n = 2$
    /// and $r = 1$), the result is $A$.
    ///
    /// This is the transform that FLINT's `fmpz_mat_similarity` computes. FLINT's manual describes
    /// $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this follows,
    /// also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r > 0$. Both
    /// are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the sum of the largest number of significant bits of its elements and the number of
    /// significant bits of `d`.
    ///
    /// # Panics
    /// Panics if the matrix is not square or if `r` is greater than or equal to its dimension, so
    /// always for the $0 \times 0$ matrix. FLINT checks neither.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SimilarityTransform;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     a.clone().similarity_transform(0, Integer::TWO).to_string(),
    ///     "[-21, -66, -71; 4, 13, 14; 7, 22, 24]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .similarity_transform(2, Integer::from(5))
    ///         .to_string(),
    ///     "[16, 2, 3; 34, 5, 6; -23, -2, -5]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let a = IntegerMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .similarity_transform(1, Integer::from(4))
    ///         .to_string(),
    ///     "[5, 6; 7, 8]"
    /// );
    /// ```
    #[inline]
    fn similarity_transform(mut self, r: u64, d: Integer) -> Self {
        similarity_transform_helper(&mut self, r, &d);
        self
    }
}

impl SimilarityTransform<&Integer> for IntegerMatrix {
    type Output = Self;

    /// Applies a similarity transform to a square [`IntegerMatrix`], taking the matrix by value and
    /// the scalar by reference.
    ///
    /// Let $A$ be the $n \times n$ matrix, $r$ a row index, $0 \leq r < n$, and $d$ the scalar. Let
    /// $S$ be the set of column indices $j$ with $0 \leq j < n$, $j \neq r$, and $j \neq r - 1$; so
    /// for $r = 0$ it is every index but 0, and for $r > 0$ it leaves out both $r - 1$ and $r$. Let
    /// $P$ be the identity matrix with the element in row $r$ and column $j$ replaced by $d$ for
    /// every $j \in S$. Then $P^{-1}$ is the same matrix with $-d$ in place of $d$, and the result
    /// is
    /// $$
    /// f(A, r, d) = P^{-1}AP.
    /// $$
    /// Element by element: writing $C = AP$,
    /// $$
    /// C_{ij} = \begin{cases}
    ///     A_{ij} + dA_{ir} & \text{if} \quad j \in S, \\\\
    ///     A_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times column $r$ is added to each column in $S$; and then the result $B$ is
    /// $$
    /// B_{ij} = \begin{cases}
    ///     C_{ij} - d\sum_{k \in S} C_{kj} & \text{if} \quad i = r, \\\\
    ///     C_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times each row in $S$ is subtracted from row $r$.
    ///
    /// The result is similar to $A$, so it has the same determinant, trace, rank, characteristic
    /// polynomial, and minimal polynomial. If $d = 0$, or if $S$ is empty (when $n = 1$, or $n = 2$
    /// and $r = 1$), the result is $A$.
    ///
    /// This is the transform that FLINT's `fmpz_mat_similarity` computes. FLINT's manual describes
    /// $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this follows,
    /// also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r > 0$. Both
    /// are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the sum of the largest number of significant bits of its elements and the number of
    /// significant bits of `d`.
    ///
    /// # Panics
    /// Panics if the matrix is not square or if `r` is greater than or equal to its dimension, so
    /// always for the $0 \times 0$ matrix. FLINT checks neither.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SimilarityTransform;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     a.clone().similarity_transform(0, &Integer::TWO).to_string(),
    ///     "[-21, -66, -71; 4, 13, 14; 7, 22, 24]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .similarity_transform(2, &Integer::from(5))
    ///         .to_string(),
    ///     "[16, 2, 3; 34, 5, 6; -23, -2, -5]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let a = IntegerMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .similarity_transform(1, &Integer::from(4))
    ///         .to_string(),
    ///     "[5, 6; 7, 8]"
    /// );
    /// ```
    #[inline]
    fn similarity_transform(mut self, r: u64, d: &Integer) -> Self {
        similarity_transform_helper(&mut self, r, d);
        self
    }
}

impl SimilarityTransform<Integer> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Applies a similarity transform to a square [`IntegerMatrix`], taking the matrix by reference
    /// and the scalar by value.
    ///
    /// Let $A$ be the $n \times n$ matrix, $r$ a row index, $0 \leq r < n$, and $d$ the scalar. Let
    /// $S$ be the set of column indices $j$ with $0 \leq j < n$, $j \neq r$, and $j \neq r - 1$; so
    /// for $r = 0$ it is every index but 0, and for $r > 0$ it leaves out both $r - 1$ and $r$. Let
    /// $P$ be the identity matrix with the element in row $r$ and column $j$ replaced by $d$ for
    /// every $j \in S$. Then $P^{-1}$ is the same matrix with $-d$ in place of $d$, and the result
    /// is
    /// $$
    /// f(A, r, d) = P^{-1}AP.
    /// $$
    /// Element by element: writing $C = AP$,
    /// $$
    /// C_{ij} = \begin{cases}
    ///     A_{ij} + dA_{ir} & \text{if} \quad j \in S, \\\\
    ///     A_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times column $r$ is added to each column in $S$; and then the result $B$ is
    /// $$
    /// B_{ij} = \begin{cases}
    ///     C_{ij} - d\sum_{k \in S} C_{kj} & \text{if} \quad i = r, \\\\
    ///     C_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times each row in $S$ is subtracted from row $r$.
    ///
    /// The result is similar to $A$, so it has the same determinant, trace, rank, characteristic
    /// polynomial, and minimal polynomial. If $d = 0$, or if $S$ is empty (when $n = 1$, or $n = 2$
    /// and $r = 1$), the result is $A$.
    ///
    /// This is the transform that FLINT's `fmpz_mat_similarity` computes. FLINT's manual describes
    /// $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this follows,
    /// also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r > 0$. Both
    /// are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the sum of the largest number of significant bits of its elements and the number of
    /// significant bits of `d`.
    ///
    /// # Panics
    /// Panics if the matrix is not square or if `r` is greater than or equal to its dimension, so
    /// always for the $0 \times 0$ matrix. FLINT checks neither.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SimilarityTransform;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     (&a).similarity_transform(0, Integer::TWO).to_string(),
    ///     "[-21, -66, -71; 4, 13, 14; 7, 22, 24]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     (&a).similarity_transform(2, Integer::from(5)).to_string(),
    ///     "[16, 2, 3; 34, 5, 6; -23, -2, -5]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let a = IntegerMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(
    ///     (&a).similarity_transform(1, Integer::from(4)).to_string(),
    ///     "[5, 6; 7, 8]"
    /// );
    /// ```
    #[inline]
    fn similarity_transform(self, r: u64, d: Integer) -> IntegerMatrix {
        let mut a = self.clone();
        similarity_transform_helper(&mut a, r, &d);
        a
    }
}

impl SimilarityTransform<&Integer> for &IntegerMatrix {
    type Output = IntegerMatrix;

    /// Applies a similarity transform to a square [`IntegerMatrix`], taking the matrix by reference
    /// and the scalar by reference.
    ///
    /// Let $A$ be the $n \times n$ matrix, $r$ a row index, $0 \leq r < n$, and $d$ the scalar. Let
    /// $S$ be the set of column indices $j$ with $0 \leq j < n$, $j \neq r$, and $j \neq r - 1$; so
    /// for $r = 0$ it is every index but 0, and for $r > 0$ it leaves out both $r - 1$ and $r$. Let
    /// $P$ be the identity matrix with the element in row $r$ and column $j$ replaced by $d$ for
    /// every $j \in S$. Then $P^{-1}$ is the same matrix with $-d$ in place of $d$, and the result
    /// is
    /// $$
    /// f(A, r, d) = P^{-1}AP.
    /// $$
    /// Element by element: writing $C = AP$,
    /// $$
    /// C_{ij} = \begin{cases}
    ///     A_{ij} + dA_{ir} & \text{if} \quad j \in S, \\\\
    ///     A_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times column $r$ is added to each column in $S$; and then the result $B$ is
    /// $$
    /// B_{ij} = \begin{cases}
    ///     C_{ij} - d\sum_{k \in S} C_{kj} & \text{if} \quad i = r, \\\\
    ///     C_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times each row in $S$ is subtracted from row $r$.
    ///
    /// The result is similar to $A$, so it has the same determinant, trace, rank, characteristic
    /// polynomial, and minimal polynomial. If $d = 0$, or if $S$ is empty (when $n = 1$, or $n = 2$
    /// and $r = 1$), the result is $A$.
    ///
    /// This is the transform that FLINT's `fmpz_mat_similarity` computes. FLINT's manual describes
    /// $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this follows,
    /// also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r > 0$. Both
    /// are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(n^2 m \log m \log\log m)$
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the sum of the largest number of significant bits of its elements and the number of
    /// significant bits of `d`.
    ///
    /// # Panics
    /// Panics if the matrix is not square or if `r` is greater than or equal to its dimension, so
    /// always for the $0 \times 0$ matrix. FLINT checks neither.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::SimilarityTransform;
    /// use malachite_base::num::basic::traits::Two;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     (&a).similarity_transform(0, &Integer::TWO).to_string(),
    ///     "[-21, -66, -71; 4, 13, 14; 7, 22, 24]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let a = IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     (&a).similarity_transform(2, &Integer::from(5)).to_string(),
    ///     "[16, 2, 3; 34, 5, 6; -23, -2, -5]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let a = IntegerMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(
    ///     (&a).similarity_transform(1, &Integer::from(4)).to_string(),
    ///     "[5, 6; 7, 8]"
    /// );
    /// ```
    #[inline]
    fn similarity_transform(self, r: u64, d: &Integer) -> IntegerMatrix {
        let mut a = self.clone();
        similarity_transform_helper(&mut a, r, d);
        a
    }
}
