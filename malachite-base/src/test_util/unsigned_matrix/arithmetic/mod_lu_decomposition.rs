// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::matrix::Matrix;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_matrix::arithmetic::mod_lu_decomposition::{
    LuBlock, lu, lu_classical, lu_classical_delayed, lu_recursive,
};
use alloc::vec;
use alloc::vec::Vec;

type LuFn<T> = fn(&mut [usize], &mut LuBlock<'_, T>, T, bool) -> usize;

fn packed_lu<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
    rank_check: bool,
    f: LuFn<T>,
) -> (Vec<usize>, usize, UnsignedMatrix<T>) {
    let mut a = a.clone();
    let height = usize::exact_from(a.height());
    let width = usize::exact_from(a.width());
    let mut perm = vec![0; height];
    let rank = f(
        &mut perm,
        &mut LuBlock {
            xs: &mut a.elements,
            stride: width,
            height,
            width,
        },
        m,
        rank_check,
    );
    (perm, rank, a)
}

/// The packed LU decomposition of `a` modulo the prime `m`, as computed by FLINT's `nmod_mat_lu`:
/// the permutation, the rank, and the matrix holding U and the multipliers of L.
pub fn unsigned_matrix_mod_lu_packed<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
    rank_check: bool,
) -> (Vec<usize>, usize, UnsignedMatrix<T>) {
    packed_lu(a, m, rank_check, lu)
}

/// `unsigned_matrix_mod_lu_packed`, always with the classical algorithm.
pub fn unsigned_matrix_mod_lu_packed_classical<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
    rank_check: bool,
) -> (Vec<usize>, usize, UnsignedMatrix<T>) {
    packed_lu(a, m, rank_check, lu_classical)
}

/// `unsigned_matrix_mod_lu_packed`, always with the classical algorithm with delayed reduction.
pub fn unsigned_matrix_mod_lu_packed_delayed<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
    rank_check: bool,
) -> (Vec<usize>, usize, UnsignedMatrix<T>) {
    packed_lu(a, m, rank_check, lu_classical_delayed)
}

/// `unsigned_matrix_mod_lu_packed`, always with the recursive algorithm at the top level.
pub fn unsigned_matrix_mod_lu_packed_recursive<T: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    m: T,
    rank_check: bool,
) -> (Vec<usize>, usize, UnsignedMatrix<T>) {
    packed_lu(a, m, rank_check, lu_recursive)
}
