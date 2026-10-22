// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_lu_decomposition::{
    NaturalLuBlock, lu, lu_classical, lu_recursive,
};
use alloc::vec;
use alloc::vec::Vec;
use malachite_base::matrix::Matrix;
use malachite_base::num::conversion::traits::ExactFrom;

type LuFn = fn(&mut [usize], &mut NaturalLuBlock<'_>, &Natural, bool) -> Option<usize>;

fn packed_lu(
    a: &NaturalMatrix,
    m: &Natural,
    rank_check: bool,
    f: LuFn,
) -> Option<(Vec<usize>, usize, NaturalMatrix)> {
    let mut a = a.clone();
    let height = usize::exact_from(a.height());
    let width = usize::exact_from(a.width());
    let mut perm = vec![0; height];
    let rank = f(
        &mut perm,
        &mut NaturalLuBlock {
            xs: &mut a.elements,
            stride: width,
            height,
            width,
        },
        m,
        rank_check,
    )?;
    Some((perm, rank, a))
}

/// The packed LU decomposition of `a` modulo `m`, as computed by FLINT's `fmpz_mod_mat_lu`: the
/// permutation, the rank, and the matrix holding U and the multipliers of L, or `None` if a pivot
/// is not invertible. The large-modulus algorithms are used even when `m` fits in a limb.
pub fn natural_matrix_mod_lu_packed(
    a: &NaturalMatrix,
    m: &Natural,
    rank_check: bool,
) -> Option<(Vec<usize>, usize, NaturalMatrix)> {
    packed_lu(a, m, rank_check, lu)
}

/// `natural_matrix_mod_lu_packed`, always with the classical algorithm.
pub fn natural_matrix_mod_lu_packed_classical(
    a: &NaturalMatrix,
    m: &Natural,
    rank_check: bool,
) -> Option<(Vec<usize>, usize, NaturalMatrix)> {
    packed_lu(a, m, rank_check, lu_classical)
}

/// `natural_matrix_mod_lu_packed`, always with the recursive algorithm at the top level.
pub fn natural_matrix_mod_lu_packed_recursive(
    a: &NaturalMatrix,
    m: &Natural,
    rank_check: bool,
) -> Option<(Vec<usize>, usize, NaturalMatrix)> {
    packed_lu(a, m, rank_check, lu_recursive)
}
