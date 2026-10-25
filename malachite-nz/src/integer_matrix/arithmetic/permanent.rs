// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2025 Fredrik Johansson
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use crate::integer_polynomial::arithmetic::coefficient::SMALL_FMPZ_BITCOUNT_MAX;
use crate::integer_vector::arithmetic::max_bits::vec_max_bits;
use crate::natural::arithmetic::add::limbs_slice_add_greater_in_place_left;
use crate::natural::arithmetic::mul::limb::limbs_slice_mul_limb_in_place;
use crate::natural::arithmetic::sub::limbs_sub_greater_in_place_left;
use crate::natural::{HALF_WIDTH, QUARTER_WIDTH, WIDTH_MINUS_1};
use crate::platform::{Limb, SignedLimb};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::*;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::num::logic::traits::{NotAssign, SignificantBits};

// The largest dimension for which Glynn's formula can be used: the sign vectors are indexed by a
// `u64`, and there are $2^{n-1}$ of them.
const GLYNN_MAX_DIMENSION: usize = 64;

// The permanent of the matrix formed by the rows `rows` and the columns `col..` of the square
// matrix `a`, by cofactor expansion along the first column. There are at least 3 rows. `scratch`
// holds the row lists of the deeper levels, so it must have room for $3 + 4 + \cdots + (n - 1)$ of
// them.
//
// This is equivalent to `_gr_mat_permanent_cofactor_recursive` from `gr_mat/permanent.c`, FLINT
// 3.6.0.
fn permanent_cofactor_recursive(
    a: &IntegerMatrix,
    rows: &[usize],
    col: usize,
    scratch: &mut [usize],
) -> Integer {
    let n = rows.len();
    let w = a.width;
    let e = |i: usize, j: usize| &a.elements[rows[i] * w + col + j];
    if n == 3 {
        (e(1, 0).mul_add_mul(e(2, 1), e(1, 1), e(2, 0)) * e(0, 2))
            .add_mul(e(1, 2).mul_add_mul(e(2, 0), e(1, 0), e(2, 2)), e(0, 1))
            .add_mul(e(1, 1).mul_add_mul(e(2, 2), e(1, 2), e(2, 1)), e(0, 0))
    } else {
        let (r, rest) = scratch.split_at_mut(n - 1);
        let mut res = Integer::ZERO;
        for i in 0..n {
            r[..i].copy_from_slice(&rows[..i]);
            r[i..].copy_from_slice(&rows[i + 1..]);
            res.add_mul_assign(e(i, 0), permanent_cofactor_recursive(a, r, col + 1, rest));
        }
        res
    }
}

// The permanent of a square matrix by cofactor expansion along the first column, in time $O(n \cdot
// n!)$.
//
// This is equivalent to `gr_mat_permanent_cofactor` from `gr_mat/permanent.c`, FLINT 3.6.0, except
// that it has no limit on the dimension.
crate_test_fn! {
permanent_cofactor(a: &IntegerMatrix) -> Integer {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let e = |i: usize, j: usize| &a.elements[i * a.width + j];
    match a.height {
        0 => Integer::ONE,
        1 => e(0, 0).clone(),
        2 => e(0, 0).mul_add_mul(e(1, 1), e(0, 1), e(1, 0)),
        n => {
            let rows: Vec<usize> = (0..n).collect();
            permanent_cofactor_recursive(a, &rows, 0, &mut vec![0; (n * n) >> 1])
        }
    }
}}

// The permanent of a square matrix of dimension at least 1 and at most 64 by Glynn's formula,
// $$
// \operatorname{perm}(A) = \frac{1}{2^{n-1}} \sum_\delta \left ( \prod_k \delta_k \right )
//     \prod_i \sum_j \delta_j a_{ij},
// $$
// where $\delta$ runs over the sign vectors with $\delta_0 = 1$, visited in Gray-code order so that
// each step changes one sign and the row sums are updated by adding or subtracting twice a column.
//
// This is equivalent to `gr_mat_permanent_glynn` from `gr_mat/permanent.c`, FLINT 3.6.0.
crate_test_fn! {
permanent_glynn(a: &IntegerMatrix) -> Integer {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    assert!(
        (1..=GLYNN_MAX_DIMENSION).contains(&n),
        "the dimension must be between 1 and 64"
    );
    // The columns of `a`, doubled.
    let mut a2 = vec![Integer::ZERO; n * n];
    for i in 0..n {
        for j in 0..n {
            a2[j * n + i] = &a.elements[i * n + j] << 1u32;
        }
    }
    let mut s: Vec<Integer> = a.elements.chunks_exact(n).map(|row| row.iter().sum()).collect();
    let mut res: Integer = s.iter().product();
    let mut j_prev = 0u64;
    for i in 1..u64::power_of_2(u64::wrapping_from(n - 1)) {
        let j = i ^ (i >> 1);
        let d = usize::exact_from((j ^ j_prev).trailing_zeros());
        let column = &a2[d * n..(d + 1) * n];
        if j > j_prev {
            for (x, y) in s.iter_mut().zip(column) {
                *x -= y;
            }
        } else {
            for (x, y) in s.iter_mut().zip(column) {
                *x += y;
            }
        }
        let p: Integer = s.iter().product();
        if i.odd() {
            res -= p;
        } else {
            res += p;
        }
        j_prev = j;
    }
    res >> (n - 1)
}}

// `rs` (two's complement, wrapping) += (-1)^negative * (xs[0] * ... * xs[n - 1]), assuming that
// |xs[i]| < 2^s_bits and that the product fits in `rs`.
//
// This is equivalent to `_nn_addproduct_1` from `fmpz_mat/permanent.c`, FLINT 3.6.0.
fn limbs_add_product(rs: &mut [Limb], xs: &[SignedLimb], s_bits: u64, mut negative: bool) {
    // The number of factors multiplied in one `SignedLimb` before multiplying the product in.
    let group = if s_bits < const { Limb::WIDTH >> 3 } {
        8
    } else if s_bits < QUARTER_WIDTH {
        4
    } else if s_bits < HALF_WIDTH {
        2
    } else {
        1
    };
    let n = xs.len();
    let mut p = [0; Limb::WIDTH as usize];
    let first = match n % group {
        0 => group,
        r => r,
    };
    let a: SignedLimb = xs[n - first..].iter().product();
    if a < 0 {
        negative.not_assign();
    }
    p[0] = a.unsigned_abs();
    let mut p_len = 1;
    for chunk in xs[..n - first].chunks_exact(group) {
        let a: SignedLimb = chunk.iter().product();
        if a < 0 {
            negative.not_assign();
        }
        let carry = limbs_slice_mul_limb_in_place(&mut p[..p_len], a.unsigned_abs());
        p[p_len] = carry;
        if carry != 0 {
            p_len += 1;
        }
    }
    if negative {
        limbs_sub_greater_in_place_left(rs, &p[..p_len]);
    } else {
        limbs_slice_add_greater_in_place_left(rs, &p[..p_len]);
    }
}

// The permanent of a square matrix of dimension at least 1 and at most `Limb::WIDTH - 2` by Glynn's
// formula, with the row sums held in single words: every element must have at most `Limb::WIDTH -
// 2` bits, every row sum fewer than `s_bits`, `s_bits` must be less than `Limb::WIDTH`, and the sum
// of the products must fit in `r_len` limbs.
//
// This is equivalent to `fmpz_mat_permanent_glynn1` from `fmpz_mat/permanent.c`, FLINT 3.6.0.
crate_test_fn! {
permanent_glynn_word(a: &IntegerMatrix, s_bits: u64, r_len: usize) -> Integer {
    let n = a.height;
    let xs: Vec<SignedLimb> = a.elements.iter().map(SignedLimb::exact_from).collect();
    // The columns of `a`, doubled.
    let mut a2 = vec![0; n * n];
    for (i, row) in xs.chunks_exact(n).enumerate() {
        for (j, &x) in row.iter().enumerate() {
            a2[j * n + i] = x << 1;
        }
    }
    let mut s: Vec<SignedLimb> = xs.chunks_exact(n).map(|row| row.iter().sum()).collect();
    let mut rs = vec![0; r_len];
    limbs_add_product(&mut rs, &s, s_bits, false);
    let mut j_prev = 0u64;
    for i in 1..u64::power_of_2(u64::wrapping_from(n - 1)) {
        let j = i ^ (i >> 1);
        let d = usize::exact_from((j ^ j_prev).trailing_zeros());
        let column = &a2[d * n..(d + 1) * n];
        if j > j_prev {
            for (x, y) in s.iter_mut().zip(column) {
                *x -= y;
            }
        } else {
            for (x, y) in s.iter_mut().zip(column) {
                *x += y;
            }
        }
        limbs_add_product(&mut rs, &s, s_bits, i.odd());
        j_prev = j;
    }
    Integer::from_twos_complement_limbs_asc(&rs) >> (n - 1)
}}

// The permanent of a square matrix of dimension at most 64, without looking for structure: directly
// up to dimension 2, by cofactor expansion for dimension 3 and up to 4 when the elements are large,
// and otherwise by Glynn's formula, on single words when the elements are small enough.
//
// This is equivalent to `fmpz_mat_permanent` from `fmpz_mat/permanent.c`, FLINT 3.6.0, without the
// threaded variants and with the limit on the dimension raised from 62 to 64.
crate_test_fn! {
permanent_dense(a: &IntegerMatrix) -> Integer {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    if n <= 2 {
        return permanent_cofactor(a);
    }
    let generic = || {
        if n <= 4 {
            permanent_cofactor(a)
        } else {
            permanent_glynn(a)
        }
    };
    let width_minus_2 = const { (Limb::WIDTH - 2) as usize };
    if n == 3 || n > width_minus_2 {
        return generic();
    }
    let a_bits = vec_max_bits(&a.elements).0;
    // A bound on the bits of a sum of n elements with signs.
    let s_bits = a_bits + u64::wrapping_from(n).significant_bits();
    if a_bits > SMALL_FMPZ_BITCOUNT_MAX || s_bits > WIDTH_MINUS_1 {
        return generic();
    }
    // A bound on the bits of the sum of 2^(n - 1) products of n sums, plus one for the sign.
    let r_bits = u64::wrapping_from(n) * s_bits + u64::wrapping_from(n);
    permanent_glynn_word(a, s_bits, usize::exact_from(r_bits.div_ceil(Limb::WIDTH)))
}}

// The submatrix of `a` formed by the rows `rows` and the columns `cols`, in the given orders.
fn submatrix(a: &IntegerMatrix, rows: &[usize], cols: &[usize]) -> IntegerMatrix {
    let mut elements = Vec::with_capacity(rows.len() * cols.len());
    for &i in rows {
        elements.extend(cols.iter().map(|&j| a.elements[i * a.width + j].clone()));
    }
    IntegerMatrix {
        elements,
        height: rows.len(),
        width: cols.len(),
    }
}

// The matrix `a` without row `i` and column `j`.
fn minor(a: &IntegerMatrix, i: usize, j: usize) -> IntegerMatrix {
    let n = a.height;
    let rows: Vec<usize> = (0..n).filter(|&k| k != i).collect();
    let cols: Vec<usize> = (0..n).filter(|&k| k != j).collect();
    submatrix(a, &rows, &cols)
}

// The matrix whose permanent equals that of `a`, a square matrix whose row `i` has exactly two
// nonzero elements, $x$ in column `j_1` and $y$ in column `j_2`: row `i` and column `j_2` are
// removed, and column `j_1` is replaced by $y$ times itself plus $x$ times column `j_2`. Expanding
// along row `i` gives $x \operatorname{perm}(A_{i, j_1}) + y \operatorname{perm}(A_{i, j_2})$, and
// the two minors differ only in the column they keep, so by linearity in that column, and since the
// permanent does not depend on the order of the columns, the sum is the permanent of the result.
fn contract_row(a: &IntegerMatrix, i: usize, j_1: usize, j_2: usize) -> IntegerMatrix {
    let n = a.height;
    let x = &a.elements[i * n + j_1];
    let y = &a.elements[i * n + j_2];
    let mut elements = Vec::with_capacity((n - 1) * (n - 1));
    for k in (0..n).filter(|&k| k != i) {
        let row = &a.elements[k * n..(k + 1) * n];
        for (j, z) in row.iter().enumerate() {
            if j == j_1 {
                elements.push(y.mul_add_mul(z, x, &row[j_2]));
            } else if j != j_2 {
                elements.push(z.clone());
            }
        }
    }
    IntegerMatrix {
        elements,
        height: n - 1,
        width: n - 1,
    }
}

// The connected components of the bipartite graph whose vertices are the rows and columns of the
// square matrix `a` and whose edges are its nonzero elements, as pairs of row and column lists.
fn components(a: &IntegerMatrix) -> Vec<(Vec<usize>, Vec<usize>)> {
    let n = a.height;
    // Vertices 0..n are rows and n..2n are columns.
    let mut component = vec![usize::MAX; n << 1];
    let mut result = Vec::new();
    for start in 0..n << 1 {
        if component[start] != usize::MAX {
            continue;
        }
        let c = result.len();
        let mut rows = Vec::new();
        let mut cols = Vec::new();
        let mut stack = vec![start];
        component[start] = c;
        while let Some(v) = stack.pop() {
            if v < n {
                rows.push(v);
                for j in 0..n {
                    if a.elements[v * n + j] != 0u32 && component[n + j] == usize::MAX {
                        component[n + j] = c;
                        stack.push(n + j);
                    }
                }
            } else {
                let j = v - n;
                cols.push(j);
                for (i, ci) in component[..n].iter_mut().enumerate() {
                    if a.elements[i * n + j] != 0u32 && *ci == usize::MAX {
                        *ci = c;
                        stack.push(i);
                    }
                }
            }
        }
        rows.sort_unstable();
        cols.sort_unstable();
        result.push((rows, cols));
    }
    result
}

// The permanent of a square matrix, using its structure before falling back to `permanent_dense`. A
// zero row or column gives 0. A row or column with a single nonzero element `x` gives `x` times the
// permanent of the minor. If the nonzero elements split into independent blocks (connected
// components of the bipartite graph of rows and columns), the permanent is the product of the
// blocks' permanents, or 0 if some block is not square. A row or column with two nonzero elements
// is contracted, lowering the dimension by 1 without branching (see `contract_row`). Otherwise, if
// the dimension is too large for Glynn's formula, the permanent is expanded along the sparsest row
// or column; if not, `permanent_dense` computes it.
crate_test_fn! {
permanent_structured(a: IntegerMatrix) -> Integer {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let mut a = a;
    let mut factor = Integer::ONE;
    loop {
        let n = a.height;
        if n <= 2 {
            return factor * permanent_dense(&a);
        }
        let row_counts: Vec<usize> = a
            .elements
            .chunks_exact(n)
            .map(|row| row.iter().filter(|x| **x != 0u32).count())
            .collect();
        let col_counts: Vec<usize> = (0..n)
            .map(|j| (0..n).filter(|&i| a.elements[i * n + j] != 0u32).count())
            .collect();
        if row_counts.contains(&0) || col_counts.contains(&0) {
            return Integer::ZERO;
        }
        if let Some(i) = row_counts.iter().position(|&c| c == 1) {
            let j = (0..n).find(|&j| a.elements[i * n + j] != 0u32).unwrap();
            factor *= &a.elements[i * n + j];
            a = minor(&a, i, j);
            continue;
        }
        if let Some(j) = col_counts.iter().position(|&c| c == 1) {
            let i = (0..n).find(|&i| a.elements[i * n + j] != 0u32).unwrap();
            factor *= &a.elements[i * n + j];
            a = minor(&a, i, j);
            continue;
        }
        let blocks = components(&a);
        if blocks.len() > 1 {
            for (rows, cols) in &blocks {
                if rows.len() != cols.len() {
                    return Integer::ZERO;
                }
            }
            for (rows, cols) in &blocks {
                factor *= permanent_structured(submatrix(&a, rows, cols));
                if factor == 0u32 {
                    return factor;
                }
            }
            return factor;
        }
        if let Some(i) = row_counts.iter().position(|&c| c == 2) {
            let mut nonzero = (0..n).filter(|&j| a.elements[i * n + j] != 0u32);
            let j_1 = nonzero.next().unwrap();
            let j_2 = nonzero.next().unwrap();
            a = contract_row(&a, i, j_1, j_2);
            continue;
        }
        if let Some(j) = col_counts.iter().position(|&c| c == 2) {
            let mut nonzero = (0..n).filter(|&i| a.elements[i * n + j] != 0u32);
            let i_1 = nonzero.next().unwrap();
            let i_2 = nonzero.next().unwrap();
            a = contract_row(&a.transpose(), j, i_1, i_2);
            continue;
        }
        let (min_row, &min_row_count) =
            row_counts.iter().enumerate().min_by_key(|&(_, c)| *c).unwrap();
        let (min_col, &min_col_count) =
            col_counts.iter().enumerate().min_by_key(|&(_, c)| *c).unwrap();
        if n > GLYNN_MAX_DIMENSION {
            let mut sum = Integer::ZERO;
            if min_row_count <= min_col_count {
                let i = min_row;
                for j in 0..n {
                    let x = &a.elements[i * n + j];
                    if *x != 0u32 {
                        sum.add_mul_assign(x, permanent_structured(minor(&a, i, j)));
                    }
                }
            } else {
                let j = min_col;
                for i in 0..n {
                    let x = &a.elements[i * n + j];
                    if *x != 0u32 {
                        sum.add_mul_assign(x, permanent_structured(minor(&a, i, j)));
                    }
                }
            }
            return factor * sum;
        }
        return factor * permanent_dense(&a);
    }
}}

// Returns the permanent of the square matrix `a`.
fn permanent_helper(a: &IntegerMatrix) -> Integer {
    assert_eq!(a.height, a.width, "the matrix must be square");
    if a.height <= 2 {
        permanent_cofactor(a)
    } else {
        permanent_structured(a.clone())
    }
}

impl Permanent for IntegerMatrix {
    type Output = Integer;

    /// Computes the permanent of a square [`IntegerMatrix`], taking it by value.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. The permanent of the $0
    /// \times 0$ matrix is 1.
    ///
    /// The matrix's structure is used first: a zero row or column gives 0, a row or column with a
    /// single nonzero element contributes that element times the permanent of the remaining minor,
    /// a row or column with two nonzero elements is merged into a smaller matrix with the same
    /// permanent, and independent blocks of nonzero elements contribute the product of their
    /// permanents. What remains is computed by cofactor expansion when it is small or by Glynn's
    /// formula, a sum over $2^{n-1}$ sign vectors, on single words when the elements are small
    /// enough. A remaining matrix of dimension greater than 64 is expanded along its sparsest row
    /// or column. The permanent is always computed, however large the matrix, but for a dense
    /// matrix the time grows exponentially with the dimension.
    ///
    /// $$
    /// f(A) = \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(2^n nm \log(nm) \log\log(nm))$ for a dense matrix of dimension at most 64
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::num::arithmetic::traits::Permanent;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!(a.clone().permanent().to_string(), "10");
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; 7, 8, 9]").unwrap();
    /// assert_eq!(a.clone().permanent().to_string(), "210");
    ///
    /// // The permanent of the 4 by 4 matrix of ones is 4!.
    /// let a =
    ///     IntegerMatrix::from_str("[1, 1, 1, 1; 1, 1, 1, 1; 1, 1, 1, 1; 1, 1, 1, 1]").unwrap();
    /// assert_eq!(a.clone().permanent().to_string(), "24");
    ///
    /// // Large matrices with structure are fast.
    /// let a = IntegerMatrix::identity(100);
    /// assert_eq!(a.clone().permanent().to_string(), "1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_permanent` from `fmpz_mat/permanent.c`, FLINT 3.6.0, except
    /// that it always succeeds, while FLINT's fails above dimension 62, and that it panics if the
    /// matrix is not square.
    #[inline]
    fn permanent(self) -> Integer {
        if self.height <= 2 {
            permanent_cofactor(&self)
        } else {
            permanent_structured(self)
        }
    }
}

impl Permanent for &IntegerMatrix {
    type Output = Integer;

    /// Computes the permanent of a square [`IntegerMatrix`], taking it by reference.
    ///
    /// The permanent is the determinant's formula without the signs: the sum, over all permutations
    /// $\sigma$ of the rows, of the products of the elements they select. The permanent of the $0
    /// \times 0$ matrix is 1.
    ///
    /// The matrix's structure is used first: a zero row or column gives 0, a row or column with a
    /// single nonzero element contributes that element times the permanent of the remaining minor,
    /// a row or column with two nonzero elements is merged into a smaller matrix with the same
    /// permanent, and independent blocks of nonzero elements contribute the product of their
    /// permanents. What remains is computed by cofactor expansion when it is small or by Glynn's
    /// formula, a sum over $2^{n-1}$ sign vectors, on single words when the elements are small
    /// enough. A remaining matrix of dimension greater than 64 is expanded along its sparsest row
    /// or column. The permanent is always computed, however large the matrix, but for a dense
    /// matrix the time grows exponentially with the dimension.
    ///
    /// $$
    /// f(A) = \sum_{\sigma \in S_n} \prod_{i=0}^{n-1} a_{i, \sigma(i)}.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(2^n nm \log(nm) \log\log(nm))$ for a dense matrix of dimension at most 64
    ///
    /// $M(n, m) = O(n^2 m)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is the dimension of the matrix, and $m$ is
    /// the largest number of significant bits of any element.
    ///
    /// # Panics
    /// Panics if the matrix is not square.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::matrix::Matrix;
    /// use malachite_base::num::arithmetic::traits::Permanent;
    /// use malachite_nz::integer_matrix::IntegerMatrix;
    ///
    /// let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    /// assert_eq!((&a).permanent().to_string(), "10");
    ///
    /// let a = IntegerMatrix::from_str("[1, -2, 3; 4, 5, -6; 7, 8, 9]").unwrap();
    /// assert_eq!((&a).permanent().to_string(), "210");
    ///
    /// // The permanent of the 4 by 4 matrix of ones is 4!.
    /// let a =
    ///     IntegerMatrix::from_str("[1, 1, 1, 1; 1, 1, 1, 1; 1, 1, 1, 1; 1, 1, 1, 1]").unwrap();
    /// assert_eq!((&a).permanent().to_string(), "24");
    ///
    /// // Large matrices with structure are fast.
    /// let a = IntegerMatrix::identity(100);
    /// assert_eq!((&a).permanent().to_string(), "1");
    /// ```
    ///
    /// This is equivalent to `fmpz_mat_permanent` from `fmpz_mat/permanent.c`, FLINT 3.6.0, except
    /// that it always succeeds, while FLINT's fails above dimension 62, and that it panics if the
    /// matrix is not square.
    #[inline]
    fn permanent(self) -> Integer {
        permanent_helper(self)
    }
}
