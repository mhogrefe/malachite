// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use itertools::Itertools;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Mod, ModRank};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::word_lu::lu_with_pivots;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::integer_matrix_gen;

// Checks the LU decomposition of `a` modulo `p`: the row permutation and the pivot and non-pivot
// columns are permutations, the pivots and non-pivots are each increasing, and the rank is the rank
// modulo p. Returns them.
fn verify_lu(a: &IntegerMatrix, p: Limb) -> String {
    let (rank, perm, pivots_nonpivots) = lu_with_pivots(a, p);
    let h = usize::try_from(a.height()).unwrap();
    let w = usize::try_from(a.width()).unwrap();
    assert_eq!(
        perm.iter().copied().sorted().collect_vec(),
        (0..h).collect_vec()
    );
    assert_eq!(
        pivots_nonpivots.iter().copied().sorted().collect_vec(),
        (0..w).collect_vec()
    );
    let (pivots, nonpivots) = pivots_nonpivots.split_at(rank);
    assert!(pivots.is_sorted());
    assert!(nonpivots.is_sorted());
    assert_eq!(u64::try_from(rank).unwrap(), a.mod_op(p).mod_rank(p));
    format!("({rank}, {perm:?}, {pivots_nonpivots:?})")
}

#[test]
fn test_lu_with_pivots() {
    let test = |a, p, out| {
        let a = IntegerMatrix::from_str(a).unwrap();
        assert_eq!(verify_lu(&a, p), out);
    };
    test("[]_{0,3}", 2, "(0, [], [0, 1, 2])");
    test("[]_{2,0}", 2, "(0, [0, 1], [])");
    test("[0, 0; 0, 0]", 3, "(0, [0, 1], [0, 1])");
    test("[1, 2; 3, 4]", 2, "(1, [0, 1], [0, 1])");
    test("[1, 2; 3, 4]", 7, "(2, [0, 1], [0, 1])");
    test("[0, 1; 1, 0]", 7, "(2, [1, 0], [0, 1])");
    test("[2, 4, 1; 1, 2, 3]", 7, "(2, [0, 1], [0, 2, 1])");
    test(
        "[0, 2, 1; 0, 4, 3; 0, 1, 1]",
        5,
        "(2, [0, 1, 2], [1, 2, 0])",
    );
}

#[test]
fn lu_with_pivots_properties() {
    integer_matrix_gen().test_properties(|a| {
        for p in [2, 3, 5, 7, 1000000007] {
            verify_lu(&a, p);
        }
    });
}
