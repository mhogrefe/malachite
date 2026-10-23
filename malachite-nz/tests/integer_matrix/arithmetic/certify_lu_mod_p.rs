// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::FractionFreeLuDecomposition;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::certify_lu_mod_p::*;
use malachite_nz::integer_matrix::arithmetic::is_in_reduced_row_echelon_form_with_rank::*;
use malachite_nz::integer_matrix::arithmetic::word_lu::lu_with_pivots;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{integer_matrix_gen, integer_matrix_pair_gen_var_4};

// Checks the certificate for the LU decomposition of `a` modulo `p`: if it is some (E, d), then E
// is in fraction-free reduced row echelon form with denominator d and the rank modulo p, which is
// the rank over the integers; if the rank modulo p is less than that, there is none. Returns it.
fn verify_certificate(a: &IntegerMatrix, p: Limb) -> String {
    let (rank, perm, pivots_nonpivots) = lu_with_pivots(a, p);
    let result = rref_upper_certify_lu_mod_p(a, rank, &perm, &pivots_nonpivots);
    assert_eq!(
        rank_certify_lu_mod_p(a, rank, &perm, &pivots_nonpivots),
        result.is_some()
    );
    let true_rank = usize::try_from(a.fraction_free_lu_decomposition().1.width()).unwrap();
    assert!(rank <= true_rank);
    if let Some((e, den)) = &result {
        assert!(e.is_valid());
        assert_eq!(rank, true_rank);
        assert_eq!(e.width(), a.width());
        assert_eq!(usize::try_from(e.height()).unwrap(), rank);
        assert!(is_in_reduced_row_echelon_form_with_rank(e, den, rank));
    }
    result.map_or_else(
        || "None".to_string(),
        |(e, den)| format!("Some(({e}, {den}))"),
    )
}

#[test]
fn test_rref_upper_certify_lu_mod_p() {
    let test = |a, p, out| {
        let a = IntegerMatrix::from_str(a).unwrap();
        assert_eq!(verify_certificate(&a, p), out);
    };
    test("[]_{0,3}", 2, "Some(([]_{0,3}, 1))");
    test("[]_{2,0}", 2, "Some(([]_{0,0}, 1))");
    test("[0, 0; 0, 0]", 3, "Some(([]_{0,2}, 1))");
    test("[1, 2; 3, 4]", 2, "None");
    test("[1, 2; 3, 4]", 3, "Some(([1, 0; 0, 1], 1))");
    test("[2, 4; 1, 2]", 3, "Some(([2, 4], 2))");
    test("[2, 4; 1, 2]", 2, "Some(([1, 2], 1))");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 9]",
        5,
        "Some(([-3, 0, 3; 0, -3, -6], -3))",
    );
    test(
        "[1, 2, 3; 2, 4, 6; 1, 0, 1]",
        7,
        "Some(([-2, 0, -2; 0, -2, -2], -2))",
    );
    test("[3, 1; 6, 2; 1, 5]", 3, "Some(([1, 0; 0, 1], 1))");
    test("[2, 1; 4, 2; 6, 3]", 5, "Some(([2, 1], 2))");
    test("[1, 2, 3; 2, 4, 6]", 7, "Some(([1, 2, 3], 1))");
    test("[2, 2; 1, 3]", 2, "None");
}

#[test]
fn rref_upper_certify_lu_mod_p_properties() {
    integer_matrix_gen().test_properties(|a| {
        for p in [2, 3, 5, 7] {
            verify_certificate(&a, p);
        }
    });

    integer_matrix_pair_gen_var_4().test_properties(|(a, b)| {
        let a = a * b;
        for p in [2, 3, 5, 7] {
            verify_certificate(&a, p);
        }
    });
}
