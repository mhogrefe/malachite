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
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::{
    PermanentRing, permanent_structured as permanent_structured_over_ring,
};

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

// The integers, for `permanent_structured`.
struct IntegerRing;

impl PermanentRing for IntegerRing {
    type Element = Integer;

    const DENSE_MAX_DIMENSION: usize = GLYNN_MAX_DIMENSION;

    #[inline]
    fn zero(&self) -> Integer {
        Integer::ZERO
    }

    #[inline]
    fn one(&self) -> Integer {
        Integer::ONE
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
    fn mul_assign(&self, x: &mut Integer, y: &Integer) {
        *x *= y;
    }

    #[inline]
    fn mul_add_mul(&self, x: &Integer, y: &Integer, z: &Integer, w: &Integer) -> Integer {
        x.mul_add_mul(y, z, w)
    }

    fn permanent_dense(&self, xs: &[Integer], n: usize) -> Integer {
        let n = u64::exact_from(n);
        permanent_dense(&IntegerMatrix::from_elements(n, n, xs))
    }
}

// The permanent of a square matrix, using its structure before falling back to `permanent_dense`,
// by the structural reductions of `permanent_structured` from malachite-base, which are shared with
// the modular permanents: zero rows and columns, rows and columns with one or two nonzero elements,
// independent blocks, and, above dimension 64, expansion along the sparsest row or column.
crate_test_fn! {
permanent_structured(a: IntegerMatrix) -> Integer {
    assert_eq!(a.height, a.width, "the matrix must be square");
    let n = a.height;
    permanent_structured_over_ring(&IntegerRing, a.elements, n)
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
