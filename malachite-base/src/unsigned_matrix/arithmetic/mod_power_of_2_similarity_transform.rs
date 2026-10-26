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

use crate::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2SimilarityTransform, ModPowerOf2SimilarityTransformAssign,
};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::permanent_over_ring::ModPowerOf2Ring;
use crate::unsigned_matrix::arithmetic::similarity_over_ring::similarity_transform_in_place;

// Replaces the square matrix `a`, whose elements are reduced modulo 2^pow, with $P^{-1}AP$ modulo
// 2^pow, as `similarity_transform_in_place` describes.
fn mod_power_of_2_similarity_transform_helper<T: PrimitiveUnsigned>(
    a: &mut UnsignedMatrix<T>,
    r: u64,
    d: T,
    pow: u64,
) {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    assert!(
        r < u64::exact_from(n),
        "r must be less than the dimension of the matrix, {n}, but it is {r}"
    );
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    assert!(
        a.mod_power_of_2_is_reduced(pow),
        "the matrix must be reduced mod 2^pow, but {a} has an element >= 2^{pow}"
    );
    assert!(
        d.mod_power_of_2_is_reduced(pow),
        "d must be reduced mod 2^pow, but {d} >= 2^{pow}"
    );
    similarity_transform_in_place(
        &ModPowerOf2Ring::new(pow),
        &mut a.elements,
        n,
        usize::exact_from(r),
        &d,
    );
}

impl<T: PrimitiveUnsigned> ModPowerOf2SimilarityTransformAssign<T> for UnsignedMatrix<T> {
    /// Applies a similarity transform to a square [`UnsignedMatrix`] modulo $2^k$, in place.
    ///
    /// Let $A$ be the $n \times n$ matrix, $r$ a row index, $0 \leq r < n$, and $d$ the scalar. The
    /// modulus is $2^k$, and the elements of $A$ and $d$ are reduced modulo $2^k$. Let $S$ be the
    /// set of column indices $j$ with $0 \leq j < n$, $j \neq r$, and $j \neq r - 1$; so for $r =
    /// 0$ it is every index but 0, and for $r > 0$ it leaves out both $r - 1$ and $r$. Let $P$ be
    /// the identity matrix with the element in row $r$ and column $j$ replaced by $d$ for every $j
    /// \in S$. Then $P^{-1}$ is the same matrix with $2^k - d$ (or 0, if $d = 0$) in place of $d$,
    /// and the result is
    /// $$
    /// f(A, r, d, k) = P^{-1}AP \bmod 2^k.
    /// $$
    /// Element by element: writing $C$ for the product $AP$ reduced modulo $2^k$,
    /// $$
    /// C_{ij} = \begin{cases}
    ///     (A_{ij} + dA_{ir}) \bmod 2^k & \text{if} \quad j \in S, \\\\
    ///     A_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times column $r$ is added to each column in $S$; and then the result $B$ is
    /// $$
    /// B_{ij} = \begin{cases}
    ///     (C_{ij} - d\sum_{l \in S} C_{lj}) \bmod 2^k & \text{if} \quad i = r, \\\\
    ///     C_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times each row in $S$ is subtracted from row $r$.
    ///
    /// It is similar to $A$ over $\Z/2^k\Z$, so it has the same determinant, trace, characteristic
    /// polynomial, and minimal polynomial modulo $2^k$. If $d = 0$, or if $S$ is empty (when $n =
    /// 1$, or $n = 2$ and $r = 1$), the result is $A$.
    ///
    /// FLINT has no version modulo a power of 2; this is the transform of
    /// [`mod_similarity_transform`]( crate::num::arithmetic::traits::ModSimilarityTransform) with
    /// $m = 2^k$, which is what FLINT's `nmod_mat_similarity` computes for that modulus. FLINT's
    /// manual describes $P$ as having $d$ in every column of row $r$ but column $r$; its code,
    /// which this follows, also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0
    /// when $r > 0$. Both are similarity transforms.
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
    /// always for the $0 \times 0$ matrix), if `pow` is greater than `T::WIDTH`, or if `d` or any
    /// element of the matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SimilarityTransformAssign;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// a.mod_power_of_2_similarity_transform_assign(0, 2, 4);
    /// assert_eq!(a.to_string(), "[11, 14, 9; 4, 13, 14; 7, 6, 8]");
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let mut a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// a.mod_power_of_2_similarity_transform_assign(2, 5, 4);
    /// assert_eq!(a.to_string(), "[0, 2, 3; 2, 5, 6; 9, 14, 11]");
    ///
    /// // Here S is empty, so nothing changes.
    /// let mut a = UnsignedMatrix::<u8>::from_str("[5, 6; 7, 8]").unwrap();
    /// a.mod_power_of_2_similarity_transform_assign(1, 4, 4);
    /// assert_eq!(a.to_string(), "[5, 6; 7, 8]");
    /// ```
    #[inline]
    fn mod_power_of_2_similarity_transform_assign(&mut self, r: u64, d: T, pow: u64) {
        mod_power_of_2_similarity_transform_helper(self, r, d, pow);
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SimilarityTransform<T> for UnsignedMatrix<T> {
    type Output = Self;

    /// Applies a similarity transform to a square [`UnsignedMatrix`] modulo $2^k$, taking the
    /// matrix by value.
    ///
    /// Let $A$ be the $n \times n$ matrix, $r$ a row index, $0 \leq r < n$, and $d$ the scalar. The
    /// modulus is $2^k$, and the elements of $A$ and $d$ are reduced modulo $2^k$. Let $S$ be the
    /// set of column indices $j$ with $0 \leq j < n$, $j \neq r$, and $j \neq r - 1$; so for $r =
    /// 0$ it is every index but 0, and for $r > 0$ it leaves out both $r - 1$ and $r$. Let $P$ be
    /// the identity matrix with the element in row $r$ and column $j$ replaced by $d$ for every $j
    /// \in S$. Then $P^{-1}$ is the same matrix with $2^k - d$ (or 0, if $d = 0$) in place of $d$,
    /// and the result is
    /// $$
    /// f(A, r, d, k) = P^{-1}AP \bmod 2^k.
    /// $$
    /// Element by element: writing $C$ for the product $AP$ reduced modulo $2^k$,
    /// $$
    /// C_{ij} = \begin{cases}
    ///     (A_{ij} + dA_{ir}) \bmod 2^k & \text{if} \quad j \in S, \\\\
    ///     A_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times column $r$ is added to each column in $S$; and then the result $B$ is
    /// $$
    /// B_{ij} = \begin{cases}
    ///     (C_{ij} - d\sum_{l \in S} C_{lj}) \bmod 2^k & \text{if} \quad i = r, \\\\
    ///     C_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times each row in $S$ is subtracted from row $r$.
    ///
    /// It is similar to $A$ over $\Z/2^k\Z$, so it has the same determinant, trace, characteristic
    /// polynomial, and minimal polynomial modulo $2^k$. If $d = 0$, or if $S$ is empty (when $n =
    /// 1$, or $n = 2$ and $r = 1$), the result is $A$.
    ///
    /// FLINT has no version modulo a power of 2; this is the transform of
    /// [`mod_similarity_transform`]( crate::num::arithmetic::traits::ModSimilarityTransform) with
    /// $m = 2^k$, which is what FLINT's `nmod_mat_similarity` computes for that modulus. FLINT's
    /// manual describes $P$ as having $d$ in every column of row $r$ but column $r$; its code,
    /// which this follows, also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0
    /// when $r > 0$. Both are similarity transforms.
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
    /// always for the $0 \times 0$ matrix), if `pow` is greater than `T::WIDTH`, or if `d` or any
    /// element of the matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SimilarityTransform;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_power_of_2_similarity_transform(0, 2, 4)
    ///         .to_string(),
    ///     "[11, 14, 9; 4, 13, 14; 7, 6, 8]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_power_of_2_similarity_transform(2, 5, 4)
    ///         .to_string(),
    ///     "[0, 2, 3; 2, 5, 6; 9, 14, 11]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(
    ///     a.clone()
    ///         .mod_power_of_2_similarity_transform(1, 4, 4)
    ///         .to_string(),
    ///     "[5, 6; 7, 8]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_similarity_transform(mut self, r: u64, d: T, pow: u64) -> Self {
        mod_power_of_2_similarity_transform_helper(&mut self, r, d, pow);
        self
    }
}

impl<T: PrimitiveUnsigned> ModPowerOf2SimilarityTransform<T> for &UnsignedMatrix<T> {
    type Output = UnsignedMatrix<T>;

    /// Applies a similarity transform to a square [`UnsignedMatrix`] modulo $2^k$, taking the
    /// matrix by reference.
    ///
    /// Let $A$ be the $n \times n$ matrix, $r$ a row index, $0 \leq r < n$, and $d$ the scalar. The
    /// modulus is $2^k$, and the elements of $A$ and $d$ are reduced modulo $2^k$. Let $S$ be the
    /// set of column indices $j$ with $0 \leq j < n$, $j \neq r$, and $j \neq r - 1$; so for $r =
    /// 0$ it is every index but 0, and for $r > 0$ it leaves out both $r - 1$ and $r$. Let $P$ be
    /// the identity matrix with the element in row $r$ and column $j$ replaced by $d$ for every $j
    /// \in S$. Then $P^{-1}$ is the same matrix with $2^k - d$ (or 0, if $d = 0$) in place of $d$,
    /// and the result is
    /// $$
    /// f(A, r, d, k) = P^{-1}AP \bmod 2^k.
    /// $$
    /// Element by element: writing $C$ for the product $AP$ reduced modulo $2^k$,
    /// $$
    /// C_{ij} = \begin{cases}
    ///     (A_{ij} + dA_{ir}) \bmod 2^k & \text{if} \quad j \in S, \\\\
    ///     A_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times column $r$ is added to each column in $S$; and then the result $B$ is
    /// $$
    /// B_{ij} = \begin{cases}
    ///     (C_{ij} - d\sum_{l \in S} C_{lj}) \bmod 2^k & \text{if} \quad i = r, \\\\
    ///     C_{ij} & \textrm{otherwise},
    /// \end{cases}
    /// $$
    /// so $d$ times each row in $S$ is subtracted from row $r$.
    ///
    /// It is similar to $A$ over $\Z/2^k\Z$, so it has the same determinant, trace, characteristic
    /// polynomial, and minimal polynomial modulo $2^k$. If $d = 0$, or if $S$ is empty (when $n =
    /// 1$, or $n = 2$ and $r = 1$), the result is $A$.
    ///
    /// FLINT has no version modulo a power of 2; this is the transform of
    /// [`mod_similarity_transform`]( crate::num::arithmetic::traits::ModSimilarityTransform) with
    /// $m = 2^k$, which is what FLINT's `nmod_mat_similarity` computes for that modulus. FLINT's
    /// manual describes $P$ as having $d$ in every column of row $r$ but column $r$; its code,
    /// which this follows, also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0
    /// when $r > 0$. Both are similarity transforms.
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
    /// always for the $0 \times 0$ matrix), if `pow` is greater than `T::WIDTH`, or if `d` or any
    /// element of the matrix is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2SimilarityTransform;
    /// use malachite_base::unsigned_matrix::UnsignedMatrix;
    ///
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_similarity_transform(0, 2, 4)
    ///         .to_string(),
    ///     "[11, 14, 9; 4, 13, 14; 7, 6, 8]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let a = UnsignedMatrix::<u8>::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_similarity_transform(2, 5, 4)
    ///         .to_string(),
    ///     "[0, 2, 3; 2, 5, 6; 9, 14, 11]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let a = UnsignedMatrix::<u8>::from_str("[5, 6; 7, 8]").unwrap();
    /// assert_eq!(
    ///     (&a).mod_power_of_2_similarity_transform(1, 4, 4)
    ///         .to_string(),
    ///     "[5, 6; 7, 8]"
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_similarity_transform(self, r: u64, d: T, pow: u64) -> UnsignedMatrix<T> {
        let mut a = self.clone();
        mod_power_of_2_similarity_transform_helper(&mut a, r, d, pow);
        a
    }
}
