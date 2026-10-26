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
use malachite_base::num::arithmetic::traits::{SimilarityTransform, SimilarityTransformAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::unsigned_matrix::arithmetic::similarity_over_ring::{
    SimilarityRing, similarity_transform_in_place,
};

// The integers, for `similarity_transform_in_place`.
struct IntegerRing;

impl SimilarityRing for IntegerRing {
    type Element = Integer;

    #[inline]
    fn zero(&self) -> Integer {
        Integer::ZERO
    }

    #[inline]
    fn is_zero(&self, x: &Integer) -> bool {
        *x == 0u32
    }

    #[inline]
    fn add_assign(&self, x: &mut Integer, y: &Integer) {
        *x += y;
    }

    #[inline]
    fn sub_assign(&self, x: &mut Integer, y: &Integer) {
        *x -= y;
    }

    #[inline]
    fn mul(&self, x: &Integer, y: &Integer) -> Integer {
        x * y
    }
}

// Replaces the square matrix `a` with $P^{-1}AP$, as `similarity_transform_in_place` describes.
//
// This is equivalent to `fmpz_mat_similarity` from `fmpz_mat/similarity.c`, FLINT 3.6.0, except
// that it panics if the matrix is not square or `r` is not a row index, which FLINT does not check.
fn similarity_transform_helper(a: &mut IntegerMatrix, r: u64, d: &Integer) {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    assert!(
        r < u64::exact_from(n),
        "r must be less than the dimension of the matrix, {n}, but it is {r}"
    );
    similarity_transform_in_place(&IntegerRing, &mut a.elements, n, usize::exact_from(r), d);
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
