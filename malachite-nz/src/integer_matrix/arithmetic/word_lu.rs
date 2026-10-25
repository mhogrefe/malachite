// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2010, 2011, 2026 Fredrik Johansson
//
//      Copyright © 2020 William Hart
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer_matrix::IntegerMatrix;
use crate::integer_matrix::arithmetic::fraction_matrix::RightHandSide;
use crate::natural_matrix::arithmetic::mod_determinant::permutation_is_odd;
use crate::platform::Limb;
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    Mod, ModMul, ModMulAssign, ModNegAssign, ModSolveLowerTriangular, ModSolveUpperTriangular,
};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use malachite_base::unsigned_matrix::arithmetic::mod_lu_decomposition::mod_lu_decomposition_packed;

// An LU decomposition PA = LU of an integer matrix A reduced modulo a word-sized prime p.
pub(crate) struct WordLu {
    pub(crate) p: Limb,
    // Row i of PA is row perm[i] of A.
    pub(crate) perm: Vec<usize>,
    pub(crate) rank: usize,
    // The pivot columns, in increasing order, followed by the other columns, in increasing order.
    pub(crate) pivots_nonpivots: Vec<usize>,
    // The packed decomposition: the elements of L, other than its unit diagonal, are below the
    // diagonal of the first rank columns, and those of U are on and above the diagonal of the first
    // rank rows.
    lu: UnsignedMatrix<Limb>,
}

// The rows of the matrix with elements `xs` and width `width`, permuted: row `i` of the result is
// row `perm[i]` of the matrix.
fn permute_rows(xs: &[Limb], width: usize, perm: &[usize]) -> Vec<Limb> {
    let mut out = Vec::with_capacity(perm.len() * width);
    for &p in perm {
        out.extend_from_slice(&xs[p * width..(p + 1) * width]);
    }
    out
}

fn from_elements(height: usize, width: usize, xs: Vec<Limb>) -> UnsignedMatrix<Limb> {
    UnsignedMatrix::from_owned_elements(u64::exact_from(height), u64::exact_from(width), xs)
}

impl WordLu {
    // Computes the LU decomposition of `a` modulo the prime `p`, with the pivots chosen as by
    // `nmod_mat_lu` with `rank_check = 0`.
    //
    // This is equivalent to `nmod_mat_lu_with_pivots` from `nmod_mat/lu_with_pivots.c`, FLINT
    // 3.6.0, applied to `a` reduced modulo `p`.
    pub(crate) fn new(a: &IntegerMatrix, p: Limb) -> Self {
        let width = a.width;
        let (perm, rank, lu) = mod_lu_decomposition_packed(a.mod_op(p), p);
        let mut pivots_nonpivots = Vec::with_capacity(width);
        if rank == 0 || rank == width {
            pivots_nonpivots.extend(0..width);
        } else {
            let mut nonpivots = Vec::with_capacity(width - rank);
            let mut j = 0;
            for row in lu.elements_ref().chunks_exact(width).take(rank) {
                while row[j] == 0 {
                    nonpivots.push(j);
                    j += 1;
                }
                pivots_nonpivots.push(j);
                j += 1;
            }
            nonpivots.extend(j..width);
            pivots_nonpivots.extend(nonpivots);
        }
        Self {
            p,
            perm,
            rank,
            pivots_nonpivots,
            lu,
        }
    }

    // The determinant modulo p of a square matrix A: the product of the diagonal of U, negated if
    // the row permutation is odd. When A is singular modulo p, a diagonal element is zero.
    //
    // This is equivalent to the determinant in `fmpz_mat_det_modular_given_divisor` from
    // `fmpz_mat/det_modular_given_divisor.c`, FLINT 3.6.0.
    pub(crate) fn determinant(&self) -> Limb {
        let n = self.perm.len();
        let mut det = 1;
        for i in 0..n {
            det.mod_mul_assign(self.lu.elements_ref()[i * (n + 1)], self.p);
        }
        if permutation_is_odd(&self.perm) {
            det.mod_neg_assign(self.p);
        }
        det
    }

    // Solves AX = B modulo p for a square matrix A of full rank, where `pb` holds the elements of
    // PB, which is n by k. Returns X. The triangular solves read only the parts of the packed
    // decomposition that hold L and U.
    fn solve_square_permuted(&self, pb: Vec<Limb>, k: usize) -> UnsignedMatrix<Limb> {
        let y =
            (&self.lu).mod_solve_lower_triangular(from_elements(self.rank, k, pb), self.p, true);
        (&self.lu).mod_solve_upper_triangular(y, self.p, false)
    }

    // Solves AX = B modulo p for a square matrix A of full rank.
    //
    // This is equivalent to the solve in `fmpz_mat_find_good_prime_and_solve` from
    // `fmpq_mat/solve_multi_mod.c`, FLINT 3.6.0.
    pub(crate) fn solve_square(&self, b: &UnsignedMatrix<Limb>) -> UnsignedMatrix<Limb> {
        let k = usize::exact_from(b.width());
        self.solve_square_permuted(permute_rows(b.elements_ref(), k, &self.perm), k)
    }

    // The inverse modulo p of a square matrix A of full rank.
    //
    // This is equivalent to the inversion in `fmpz_mat_find_good_prime_and_invert` from
    // `fmpz_mat/solve_dixon.c`, FLINT 3.6.0.
    pub(crate) fn inverse(&self) -> UnsignedMatrix<Limb> {
        let n = self.rank;
        let mut pb = vec![0; n * n];
        for (i, &p) in self.perm.iter().enumerate() {
            pb[i * n + p] = 1;
        }
        self.solve_square_permuted(pb, n)
    }
}

// The rank, the row permutation, and the pivot and non-pivot columns of the LU decomposition of `a`
// modulo the prime `p`.
//
// This is equivalent to `nmod_mat_lu_with_pivots` from `nmod_mat/lu_with_pivots.c`, FLINT 3.6.0,
// applied to `a` reduced modulo `p`.
crate_test_fn! {
lu_with_pivots(a: &IntegerMatrix, p: Limb) -> (usize, Vec<usize>, Vec<usize>) {
    let lu = WordLu::new(a, p);
    (lu.rank, lu.perm, lu.pivots_nonpivots)
}}

// Solves AX = B modulo the prime `p`, where A is h by w and B is h by k, returning the rank of A
// modulo `p` and a solution X, or `None` if there is none; the rows of X for the columns of A that
// are not pivot columns are zero. The permutation of the LU decomposition is written to `perm`, and
// the pivot columns to the first rank elements of `pivots`; the other elements of `pivots` are left
// alone. A and B must have at least one row, and A at least one column.
//
// This is equivalent to `nmod_mat_can_solve_inner` from `nmod_mat/can_solve.c`, FLINT 3.6.0,
// applied to `a` and `b` reduced modulo `p`.
pub(crate) fn can_solve_inner(
    a: &IntegerMatrix,
    b: RightHandSide<'_>,
    p: Limb,
    perm: &mut [usize],
    pivots: &mut [isize],
) -> (usize, Option<UnsignedMatrix<Limb>>) {
    let (h, w, k) = (a.height, a.width, b.width);
    let lu = WordLu::new(a, p);
    let rank = lu.rank;
    perm.copy_from_slice(&lu.perm);
    for (pivot, &column) in pivots.iter_mut().zip(&lu.pivots_nonpivots[..rank]) {
        *pivot = isize::exact_from(column);
    }
    let pb = permute_rows(&b.mod_word(p).elements_ref(), k, &lu.perm);
    // The lower triangular solve reads only the part of this block that holds L.
    let l_top = lu
        .lu
        .window(0..u64::exact_from(rank), 0..u64::exact_from(rank));
    let y =
        l_top.mod_solve_lower_triangular(from_elements(rank, k, pb[..rank * k].to_vec()), p, true);
    if h > rank {
        // The remaining rows of L, times Y, must equal the remaining rows of PB.
        let l_rest = lu.lu.window(
            u64::exact_from(rank)..u64::exact_from(h),
            0..u64::exact_from(rank),
        );
        if (&l_rest).mod_mul(&y, p).elements_ref() != &pb[rank * k..] {
            return (rank, None);
        }
    }
    // The rank by rank upper triangular matrix made of the pivot columns of U. The upper triangular
    // solve reads only its elements on and above the diagonal, which are those of U.
    let mut u_pivots = Vec::with_capacity(rank * rank);
    for row in lu.lu.elements_ref().chunks_exact(w).take(rank) {
        u_pivots.extend(lu.pivots_nonpivots[..rank].iter().map(|&j| row[j]));
    }
    let y = from_elements(rank, rank, u_pivots).mod_solve_upper_triangular(y, p, false);
    let mut x = vec![0; w * k];
    for (row, &j) in y.elements_ref().chunks_exact(k).zip(&lu.pivots_nonpivots[..rank]) {
        x[j * k..(j + 1) * k].copy_from_slice(row);
    }
    (rank, Some(from_elements(w, k, x)))
}
