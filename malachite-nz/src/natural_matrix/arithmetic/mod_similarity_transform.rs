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

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_lu_decomposition::assert_reduced;
use malachite_base::num::arithmetic::traits::{
    ModSimilarityTransform, ModSimilarityTransformAssign,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::ModRing;
use malachite_base::unsigned_matrix::arithmetic::similarity_over_ring::*;

// Replaces the square matrix `a`, whose elements are reduced modulo `m`, with $P^{-1}AP$ modulo
// `m`, as `similarity_transform_in_place` describes.
//
// This is equivalent to `fmpz_mod_mat_similarity` from `fmpz_mod_mat/similarity.c`, FLINT 3.6.0,
// except that it panics if the matrix is not square, `r` is not a row index, or the inputs are not
// reduced, none of which FLINT checks.
fn mod_similarity_transform_helper(a: &mut NaturalMatrix, r: u64, d: &Natural, m: &Natural) {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    assert!(
        r < u64::exact_from(n),
        "r must be less than the dimension of the matrix, {n}, but it is {r}"
    );
    assert_reduced(a, m);
    assert!(d < m, "d must be reduced mod m, but {d} >= {m}");
    similarity_transform_in_place(
        &ModRing::new(m.clone()),
        &mut a.elements,
        n,
        usize::exact_from(r),
        d,
    );
}

impl ModSimilarityTransformAssign<Natural, Natural> for NaturalMatrix {
    /// Applies a similarity transform to a square [`NaturalMatrix`] modulo $m$, in place, taking
    /// the scalar by value and the modulus by value.
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
    /// This is the transform that FLINT's `fmpz_mod_mat_similarity` computes. FLINT's manual
    /// describes $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this
    /// follows, also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r >
    /// 0$. Both are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \log k \log\log k)$
    ///
    /// $M(n, k) = O(k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from_str("123456789012345678901234567890").unwrap();
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// a.mod_similarity_transform_assign(0, d, m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[641975320864197532086419753325, 176801747611645297120561041482, \
    ///     806431380574608260416857337870; 4, 493827156049382715604938271565, \
    ///     493827156049382715604938271566; 7, 864197523086419752308641975238, \
    ///     864197523086419752308641975240]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from(5u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// a.mod_similarity_transform_assign(2, d, m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[16, 2, 3; 34, 5, 6; 1000000000000000000000000000034, \
    ///     1000000000000000000000000000055, 1000000000000000000000000000052]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let mut a = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// let d = Natural::from(4u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// a.mod_similarity_transform_assign(1, d, m);
    /// assert_eq!(a.to_string(), "[5, 6; 7, 8]");
    /// ```
    #[inline]
    fn mod_similarity_transform_assign(&mut self, r: u64, d: Natural, m: Natural) {
        mod_similarity_transform_helper(self, r, &d, &m);
    }
}

impl ModSimilarityTransformAssign<Natural, &Natural> for NaturalMatrix {
    /// Applies a similarity transform to a square [`NaturalMatrix`] modulo $m$, in place, taking
    /// the scalar by value and the modulus by reference.
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
    /// This is the transform that FLINT's `fmpz_mod_mat_similarity` computes. FLINT's manual
    /// describes $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this
    /// follows, also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r >
    /// 0$. Both are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \log k \log\log k)$
    ///
    /// $M(n, k) = O(k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from_str("123456789012345678901234567890").unwrap();
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// a.mod_similarity_transform_assign(0, d, &m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[641975320864197532086419753325, 176801747611645297120561041482, \
    ///     806431380574608260416857337870; 4, 493827156049382715604938271565, \
    ///     493827156049382715604938271566; 7, 864197523086419752308641975238, \
    ///     864197523086419752308641975240]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from(5u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// a.mod_similarity_transform_assign(2, d, &m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[16, 2, 3; 34, 5, 6; 1000000000000000000000000000034, \
    ///     1000000000000000000000000000055, 1000000000000000000000000000052]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let mut a = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// let d = Natural::from(4u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// a.mod_similarity_transform_assign(1, d, &m);
    /// assert_eq!(a.to_string(), "[5, 6; 7, 8]");
    /// ```
    #[inline]
    fn mod_similarity_transform_assign(&mut self, r: u64, d: Natural, m: &Natural) {
        mod_similarity_transform_helper(self, r, &d, m);
    }
}

impl ModSimilarityTransformAssign<&Natural, Natural> for NaturalMatrix {
    /// Applies a similarity transform to a square [`NaturalMatrix`] modulo $m$, in place, taking
    /// the scalar by reference and the modulus by value.
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
    /// This is the transform that FLINT's `fmpz_mod_mat_similarity` computes. FLINT's manual
    /// describes $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this
    /// follows, also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r >
    /// 0$. Both are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \log k \log\log k)$
    ///
    /// $M(n, k) = O(k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from_str("123456789012345678901234567890").unwrap();
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// a.mod_similarity_transform_assign(0, &d, m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[641975320864197532086419753325, 176801747611645297120561041482, \
    ///     806431380574608260416857337870; 4, 493827156049382715604938271565, \
    ///     493827156049382715604938271566; 7, 864197523086419752308641975238, \
    ///     864197523086419752308641975240]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from(5u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// a.mod_similarity_transform_assign(2, &d, m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[16, 2, 3; 34, 5, 6; 1000000000000000000000000000034, \
    ///     1000000000000000000000000000055, 1000000000000000000000000000052]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let mut a = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// let d = Natural::from(4u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// a.mod_similarity_transform_assign(1, &d, m);
    /// assert_eq!(a.to_string(), "[5, 6; 7, 8]");
    /// ```
    #[inline]
    fn mod_similarity_transform_assign(&mut self, r: u64, d: &Natural, m: Natural) {
        mod_similarity_transform_helper(self, r, d, &m);
    }
}

impl ModSimilarityTransformAssign<&Natural, &Natural> for NaturalMatrix {
    /// Applies a similarity transform to a square [`NaturalMatrix`] modulo $m$, in place, taking
    /// the scalar by reference and the modulus by reference.
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
    /// This is the transform that FLINT's `fmpz_mod_mat_similarity` computes. FLINT's manual
    /// describes $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this
    /// follows, also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r >
    /// 0$. Both are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \log k \log\log k)$
    ///
    /// $M(n, k) = O(k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from_str("123456789012345678901234567890").unwrap();
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// a.mod_similarity_transform_assign(0, &d, &m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[641975320864197532086419753325, 176801747611645297120561041482, \
    ///     806431380574608260416857337870; 4, 493827156049382715604938271565, \
    ///     493827156049382715604938271566; 7, 864197523086419752308641975238, \
    ///     864197523086419752308641975240]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let mut a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from(5u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// a.mod_similarity_transform_assign(2, &d, &m);
    /// assert_eq!(
    ///     a.to_string(),
    ///     "[16, 2, 3; 34, 5, 6; 1000000000000000000000000000034, \
    ///     1000000000000000000000000000055, 1000000000000000000000000000052]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let mut a = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// let d = Natural::from(4u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// a.mod_similarity_transform_assign(1, &d, &m);
    /// assert_eq!(a.to_string(), "[5, 6; 7, 8]");
    /// ```
    #[inline]
    fn mod_similarity_transform_assign(&mut self, r: u64, d: &Natural, m: &Natural) {
        mod_similarity_transform_helper(self, r, d, m);
    }
}

impl ModSimilarityTransform<Natural, Natural> for NaturalMatrix {
    type Output = Self;

    /// Applies a similarity transform to a square [`NaturalMatrix`] modulo $m$, taking the matrix
    /// by value, the scalar by value and the modulus by value.
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
    /// This is the transform that FLINT's `fmpz_mod_mat_similarity` computes. FLINT's manual
    /// describes $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this
    /// follows, also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r >
    /// 0$. Both are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \log k \log\log k)$
    ///
    /// $M(n, k) = O(n^2 k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from_str("123456789012345678901234567890").unwrap();
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(0, d, m).to_string(),
    ///     "[641975320864197532086419753325, 176801747611645297120561041482, \
    ///     806431380574608260416857337870; 4, 493827156049382715604938271565, \
    ///     493827156049382715604938271566; 7, 864197523086419752308641975238, \
    ///     864197523086419752308641975240]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from(5u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(2, d, m).to_string(),
    ///     "[16, 2, 3; 34, 5, 6; 1000000000000000000000000000034, \
    ///     1000000000000000000000000000055, 1000000000000000000000000000052]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let a = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// let d = Natural::from(4u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(1, d, m).to_string(),
    ///     "[5, 6; 7, 8]"
    /// );
    /// ```
    #[inline]
    fn mod_similarity_transform(mut self, r: u64, d: Natural, m: Natural) -> Self {
        mod_similarity_transform_helper(&mut self, r, &d, &m);
        self
    }
}

impl ModSimilarityTransform<Natural, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Applies a similarity transform to a square [`NaturalMatrix`] modulo $m$, taking the matrix
    /// by value, the scalar by value and the modulus by reference.
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
    /// This is the transform that FLINT's `fmpz_mod_mat_similarity` computes. FLINT's manual
    /// describes $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this
    /// follows, also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r >
    /// 0$. Both are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \log k \log\log k)$
    ///
    /// $M(n, k) = O(n^2 k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from_str("123456789012345678901234567890").unwrap();
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(0, d, &m).to_string(),
    ///     "[641975320864197532086419753325, 176801747611645297120561041482, \
    ///     806431380574608260416857337870; 4, 493827156049382715604938271565, \
    ///     493827156049382715604938271566; 7, 864197523086419752308641975238, \
    ///     864197523086419752308641975240]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from(5u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(2, d, &m).to_string(),
    ///     "[16, 2, 3; 34, 5, 6; 1000000000000000000000000000034, \
    ///     1000000000000000000000000000055, 1000000000000000000000000000052]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let a = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// let d = Natural::from(4u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(1, d, &m).to_string(),
    ///     "[5, 6; 7, 8]"
    /// );
    /// ```
    #[inline]
    fn mod_similarity_transform(mut self, r: u64, d: Natural, m: &Natural) -> Self {
        mod_similarity_transform_helper(&mut self, r, &d, m);
        self
    }
}

impl ModSimilarityTransform<&Natural, Natural> for NaturalMatrix {
    type Output = Self;

    /// Applies a similarity transform to a square [`NaturalMatrix`] modulo $m$, taking the matrix
    /// by value, the scalar by reference and the modulus by value.
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
    /// This is the transform that FLINT's `fmpz_mod_mat_similarity` computes. FLINT's manual
    /// describes $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this
    /// follows, also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r >
    /// 0$. Both are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \log k \log\log k)$
    ///
    /// $M(n, k) = O(n^2 k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from_str("123456789012345678901234567890").unwrap();
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(0, &d, m).to_string(),
    ///     "[641975320864197532086419753325, 176801747611645297120561041482, \
    ///     806431380574608260416857337870; 4, 493827156049382715604938271565, \
    ///     493827156049382715604938271566; 7, 864197523086419752308641975238, \
    ///     864197523086419752308641975240]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from(5u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(2, &d, m).to_string(),
    ///     "[16, 2, 3; 34, 5, 6; 1000000000000000000000000000034, \
    ///     1000000000000000000000000000055, 1000000000000000000000000000052]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let a = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// let d = Natural::from(4u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(1, &d, m).to_string(),
    ///     "[5, 6; 7, 8]"
    /// );
    /// ```
    #[inline]
    fn mod_similarity_transform(mut self, r: u64, d: &Natural, m: Natural) -> Self {
        mod_similarity_transform_helper(&mut self, r, d, &m);
        self
    }
}

impl ModSimilarityTransform<&Natural, &Natural> for NaturalMatrix {
    type Output = Self;

    /// Applies a similarity transform to a square [`NaturalMatrix`] modulo $m$, taking the matrix
    /// by value, the scalar by reference and the modulus by reference.
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
    /// This is the transform that FLINT's `fmpz_mod_mat_similarity` computes. FLINT's manual
    /// describes $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this
    /// follows, also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r >
    /// 0$. Both are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \log k \log\log k)$
    ///
    /// $M(n, k) = O(n^2 k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from_str("123456789012345678901234567890").unwrap();
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(0, &d, &m).to_string(),
    ///     "[641975320864197532086419753325, 176801747611645297120561041482, \
    ///     806431380574608260416857337870; 4, 493827156049382715604938271565, \
    ///     493827156049382715604938271566; 7, 864197523086419752308641975238, \
    ///     864197523086419752308641975240]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from(5u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(2, &d, &m).to_string(),
    ///     "[16, 2, 3; 34, 5, 6; 1000000000000000000000000000034, \
    ///     1000000000000000000000000000055, 1000000000000000000000000000052]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let a = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// let d = Natural::from(4u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     a.clone().mod_similarity_transform(1, &d, &m).to_string(),
    ///     "[5, 6; 7, 8]"
    /// );
    /// ```
    #[inline]
    fn mod_similarity_transform(mut self, r: u64, d: &Natural, m: &Natural) -> Self {
        mod_similarity_transform_helper(&mut self, r, d, m);
        self
    }
}

impl ModSimilarityTransform<&Natural, &Natural> for &NaturalMatrix {
    type Output = NaturalMatrix;

    /// Applies a similarity transform to a square [`NaturalMatrix`] modulo $m$, taking the matrix,
    /// the scalar, and the modulus by reference.
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
    /// This is the transform that FLINT's `fmpz_mod_mat_similarity` computes. FLINT's manual
    /// describes $P$ as having $d$ in every column of row $r$ but column $r$; its code, which this
    /// follows, also leaves the element of $P$ in row $r$ and column $r - 1$ equal to 0 when $r >
    /// 0$. Both are similarity transforms.
    ///
    /// # Worst-case complexity
    /// $T(n, k) = O(n^2 k \log k \log\log k)$
    ///
    /// $M(n, k) = O(n^2 k)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $k$ is
    /// `m.significant_bits()`.
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
    /// use malachite_nz::natural::Natural;
    /// use malachite_nz::natural_matrix::NaturalMatrix;
    ///
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from_str("123456789012345678901234567890").unwrap();
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     (&a).mod_similarity_transform(0, &d, &m).to_string(),
    ///     "[641975320864197532086419753325, 176801747611645297120561041482, \
    ///     806431380574608260416857337870; 4, 493827156049382715604938271565, \
    ///     493827156049382715604938271566; 7, 864197523086419752308641975238, \
    ///     864197523086419752308641975240]"
    /// );
    ///
    /// // With r = 2, P leaves column r - 1 = 1 out, so that column changes only in row r.
    /// let a = NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6; 7, 8, 10]").unwrap();
    /// let d = Natural::from(5u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     (&a).mod_similarity_transform(2, &d, &m).to_string(),
    ///     "[16, 2, 3; 34, 5, 6; 1000000000000000000000000000034, \
    ///     1000000000000000000000000000055, 1000000000000000000000000000052]"
    /// );
    ///
    /// // Here S is empty, so nothing changes.
    /// let a = NaturalMatrix::from_str("[5, 6; 7, 8]").unwrap();
    /// let d = Natural::from(4u32);
    /// let m = Natural::from_str("1000000000000000000000000000057").unwrap();
    /// assert_eq!(
    ///     (&a).mod_similarity_transform(1, &d, &m).to_string(),
    ///     "[5, 6; 7, 8]"
    /// );
    /// ```
    #[inline]
    fn mod_similarity_transform(self, r: u64, d: &Natural, m: &Natural) -> NaturalMatrix {
        let mut a = self.clone();
        mod_similarity_transform_helper(&mut a, r, d, m);
        a
    }
}
