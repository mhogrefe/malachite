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

use crate::num::arithmetic::traits::{ModSimilarityTransform, ModSimilarityTransformAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_lu_decomposition::assert_reduced;
use crate::unsigned_matrix::arithmetic::permanent_over_ring::ModRing;
use crate::unsigned_matrix::arithmetic::similarity_over_ring::similarity_transform_in_place;

// Replaces the square matrix `a`, whose elements are reduced modulo `m`, with $P^{-1}AP$ modulo
// `m`, as `similarity_transform_in_place` describes.
//
// This is equivalent to `nmod_mat_similarity` from `nmod_mat/similarity.c`, FLINT 3.6.0, except
// that it panics if the matrix is not square, `r` is not a row index, or the inputs are not
// reduced, none of which FLINT checks.
fn mod_similarity_transform_helper<T: PrimitiveUnsigned>(
    a: &mut UnsignedMatrix<T>,
    r: u64,
    d: T,
    m: T,
) {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    assert!(
        r < u64::exact_from(n),
        "r must be less than the dimension of the matrix, {n}, but it is {r}"
    );
    assert_reduced(a, m);
    assert!(d < m, "d must be reduced mod m, but {d} >= {m}");
    similarity_transform_in_place(
        &ModRing::new(m),
        &mut a.elements,
        n,
        usize::exact_from(r),
        &d,
    );
}

impl<T: PrimitiveUnsigned> ModSimilarityTransformAssign<T> for UnsignedMatrix<T> {
    /// Applies a similarity transform to a square [`UnsignedMatrix`] modulo $m$, in place.
    ///
    /// Let $A$ be the $n \times n$ matrix, $r$ a row index, $0 \leq r < n$, and $d$ the scalar. The
    /// modulus is $m$, and the elements of $A$ and $d$ are reduced modulo $m$. Let $S$ be the set
    /// of column indices $j$ with $0 \leq j < n$, $j \neq r$, and $j \neq r - 1$; so for $r = 0$ it
    /// is every index but 0, and for $r > 0$ it leaves out both $r - 1$ and $r$. Let $P$ be the
    /// identity matrix with the element in row $r$ and column $j$ replaced by $d$ for every $j \in
    /// S$. Then $P^{-1}$ is the same matrix with $m - d$ (or 0, if $d = 0$) in place of $d$, and
    /// the result is
    /// $$
    /// f(A, r, d, m) = P^{-1}AP \bmod m.
    /// $$
    /// Element by element: writing $C$ for the product $AP$ reduced modulo $m$,
    /// $$
    /// C_{ij} = \begin{cases}
    ///     (A_{ij} + dA_{ir}) \bmod m & \text{if} \quad j \in S, \\\\
    ///     A_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times column $r$ is added to each column in $S$; and then the result $B$ is
    /// $$
    /// B_{ij} = \begin{cases}
    ///     (C_{ij} - d\sum_{k \in S} C_{kj}) \bmod m & \text{if} \quad i = r, \\\\
    ///     C_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times each row in $S$ is subtracted from row $r$.
    ///
    /// It is similar to $A$ over $\Z/m\Z$, so it has the same determinant, trace, characteristic
    /// polynomial, and minimal polynomial modulo $m$. If $d = 0$, or if $S$ is empty (when $n = 1$,
    /// or $n = 2$ and $r = 1$), the result is $A$.
    ///
    /// This is the transform that FLINT's `nmod_mat_similarity` computes. FLINT's manual describes
    /// $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this follows,
    /// also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r > 0$. Both
    /// are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if `r` is greater than or equal to its dimension (so
    /// always for the $0 \times 0$ matrix), if `m` is 0, or if `d` or any element of the matrix is
    /// greater than or equal to `m`. FLINT checks none of these.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSimilarityTransformAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// a.mod_similarity_transform_assign(0, 2, 11);
    /// assert_eq!(a.to_string(), "[1, 0, 6; 4, 2, 3; 7, 0, 2]");
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// a.mod_similarity_transform_assign(2, 5, 11);
    /// assert_eq!(a.to_string(), "[5, 2, 3; 1, 5, 6; 10, 9, 6]");
    ///
    /// // Here S is empty, so nothing changes.
    /// let mut a = UnsignedMatrix::<u8>::from_str("[5, 6; 7, 8]").unwrap();
    /// a.mod_similarity_transform_assign(1, 4, 11);
    /// assert_eq!(a.to_string(), "[5, 6; 7, 8]");
    /// ```
    #[inline]
    fn mod_similarity_transform_assign(&mut self, r: u64, d: T, m: T) {
        mod_similarity_transform_helper(self, r, d, m);
    }
}

impl<T: PrimitiveUnsigned> ModSimilarityTransform<T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Applies a similarity transform to a square [`UnsignedMatrix`] modulo $m$, taking the matrix
    /// by value.
    ///
    /// Let $A$ be the $n \times n$ matrix, $r$ a row index, $0 \leq r < n$, and $d$ the scalar. The
    /// modulus is $m$, and the elements of $A$ and $d$ are reduced modulo $m$. Let $S$ be the set
    /// of column indices $j$ with $0 \leq j < n$, $j \neq r$, and $j \neq r - 1$; so for $r = 0$ it
    /// is every index but 0, and for $r > 0$ it leaves out both $r - 1$ and $r$. Let $P$ be the
    /// identity matrix with the element in row $r$ and column $j$ replaced by $d$ for every $j \in
    /// S$. Then $P^{-1}$ is the same matrix with $m - d$ (or 0, if $d = 0$) in place of $d$, and
    /// the result is
    /// $$
    /// f(A, r, d, m) = P^{-1}AP \bmod m.
    /// $$
    /// Element by element: writing $C$ for the product $AP$ reduced modulo $m$,
    /// $$
    /// C_{ij} = \begin{cases}
    ///     (A_{ij} + dA_{ir}) \bmod m & \text{if} \quad j \in S, \\\\
    ///     A_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times column $r$ is added to each column in $S$; and then the result $B$ is
    /// $$
    /// B_{ij} = \begin{cases}
    ///     (C_{ij} - d\sum_{k \in S} C_{kj}) \bmod m & \text{if} \quad i = r, \\\\
    ///     C_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times each row in $S$ is subtracted from row $r$.
    ///
    /// It is similar to $A$ over $\Z/m\Z$, so it has the same determinant, trace, characteristic
    /// polynomial, and minimal polynomial modulo $m$. If $d = 0$, or if $S$ is empty (when $n = 1$,
    /// or $n = 2$ and $r = 1$), the result is $A$.
    ///
    /// This is the transform that FLINT's `nmod_mat_similarity` computes. FLINT's manual describes
    /// $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this follows,
    /// also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r > 0$. Both
    /// are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if `r` is greater than or equal to its dimension (so
    /// always for the $0 \times 0$ matrix), if `m` is 0, or if `d` or any element of the matrix is
    /// greater than or equal to `m`. FLINT checks none of these.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSimilarityTransform;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(0, 2, 11).to_string(),
    ///     "[1, 0, 6; 4, 2, 3; 7, 0, 2]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(2, 5, 11).to_string(),
    ///     "[5, 2, 3; 1, 5, 6; 10, 9, 6]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(1, 4, 11).to_string(),
    ///     "[5, 6; 7, 8]"
    /// );
    /// ```
    #[inline]
    fn mod_similarity_transform(mut self, r: u64, d: T, m: T) -> Self {
        mod_similarity_transform_helper(&mut self, r, d, m);
        self
    }
}

impl<T: PrimitiveUnsigned> ModSimilarityTransform<T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Applies a similarity transform to a square [`UnsignedMatrix`] modulo $m$, taking the matrix
    /// by reference.
    ///
    /// Let $A$ be the $n \times n$ matrix, $r$ a row index, $0 \leq r < n$, and $d$ the scalar. The
    /// modulus is $m$, and the elements of $A$ and $d$ are reduced modulo $m$. Let $S$ be the set
    /// of column indices $j$ with $0 \leq j < n$, $j \neq r$, and $j \neq r - 1$; so for $r = 0$ it
    /// is every index but 0, and for $r > 0$ it leaves out both $r - 1$ and $r$. Let $P$ be the
    /// identity matrix with the element in row $r$ and column $j$ replaced by $d$ for every $j \in
    /// S$. Then $P^{-1}$ is the same matrix with $m - d$ (or 0, if $d = 0$) in place of $d$, and
    /// the result is
    /// $$
    /// f(A, r, d, m) = P^{-1}AP \bmod m.
    /// $$
    /// Element by element: writing $C$ for the product $AP$ reduced modulo $m$,
    /// $$
    /// C_{ij} = \begin{cases}
    ///     (A_{ij} + dA_{ir}) \bmod m & \text{if} \quad j \in S, \\\\
    ///     A_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times column $r$ is added to each column in $S$; and then the result $B$ is
    /// $$
    /// B_{ij} = \begin{cases}
    ///     (C_{ij} - d\sum_{k \in S} C_{kj}) \bmod m & \text{if} \quad i = r, \\\\
    ///     C_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times each row in $S$ is subtracted from row $r$.
    ///
    /// It is similar to $A$ over $\Z/m\Z$, so it has the same determinant, trace, characteristic
    /// polynomial, and minimal polynomial modulo $m$. If $d = 0$, or if $S$ is empty (when $n = 1$,
    /// or $n = 2$ and $r = 1$), the result is $A$.
    ///
    /// This is the transform that FLINT's `nmod_mat_similarity` computes. FLINT's manual describes
    /// $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this follows,
    /// also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r > 0$. Both
    /// are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n^2)$
    ///
    /// $M(n) = O(n^2)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is the dimension of the matrix.
    ///
    /// # Panics
    /// Panics if the matrix is not square, if `r` is greater than or equal to its dimension (so
    /// always for the $0 \times 0$ matrix), if `m` is 0, or if `d` or any element of the matrix is
    /// greater than or equal to `m`. FLINT checks none of these.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModSimilarityTransform;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_similarity_transform(0, 2, 11).to_string(),
    ///     "[1, 0, 6; 4, 2, 3; 7, 0, 2]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_similarity_transform(2, 5, 11).to_string(),
    ///     "[5, 2, 3; 1, 5, 6; 10, 9, 6]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_similarity_transform(1, 4, 11).to_string(),
    ///     "[5, 6; 7, 8]"
    /// );
    /// ```
    #[inline]
    fn mod_similarity_transform(self, r: u64, d: T, m: T) -> UnsignedMatrix<T> {
        let mut a = self.clone();
        mod_similarity_transform_helper(&mut a, r, d, m);
        a
    }
}
