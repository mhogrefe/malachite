// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::NegAssign;
use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::is_in_reduced_row_echelon_form_with_rank::*;
use malachite_nz::test_util::generators::integer_matrix_integer_pair_gen_var_1;

#[test]
fn test_is_in_reduced_row_echelon_form_with_rank() {
    let test = |a, den, rank, out| {
        let a = IntegerMatrix::from_str(a).unwrap();
        let den = Integer::from_str(den).unwrap();
        assert_eq!(
            is_in_reduced_row_echelon_form_with_rank(&a, &den, rank),
            out
        );
    };
    test("[]_{0,0}", "1", 0, true);
    test("[]_{2,0}", "5", 0, true);
    test("[0, 0; 0, 0]", "3", 0, true);
    test("[0, 0; 0, 1]", "1", 0, false);
    test("[2, 0, 4; 0, 2, 6]", "2", 2, true);
    test("[2, 0, 4; 0, 2, 6]", "2", 1, false);
    test("[2, 0, 4; 0, 3, 6]", "2", 2, false);
    test("[0, 2, 4; 2, 0, 6]", "2", 2, false);
    test("[2, 1, 4; 0, 2, 6]", "2", 2, false);
    test("[0, 5, 0, 1; 0, 0, 5, 2; 0, 0, 0, 0]", "5", 2, true);
    test("[0, 5, 0, 1; 0, 0, 5, 2; 0, 0, 0, 1]", "5", 2, false);
    test("[-3, 0; 0, -3]", "-3", 2, true);
    test("[-3, 0; 0, -3]", "3", 2, false);
    test("[1, 2, 3]", "1", 1, true);
    test("[0, 0, 0]", "1", 1, true);
}

#[test]
fn is_in_reduced_row_echelon_form_with_rank_properties() {
    integer_matrix_integer_pair_gen_var_1().test_properties(|(a, c)| {
        let rows = usize::try_from(a.height()).unwrap();
        for rank in 0..=rows {
            for den in [Integer::ONE, c.clone()] {
                let result = is_in_reduced_row_echelon_form_with_rank(&a, &den, rank);
                // Scaling the matrix and the denominator by the same nonzero factor, or negating
                // both, preserves the form.
                assert_eq!(
                    is_in_reduced_row_echelon_form_with_rank(&(&a * &c), &(&den * &c), rank),
                    result
                );
                let b = -&a;
                let mut neg_den = den.clone();
                neg_den.neg_assign();
                assert_eq!(
                    is_in_reduced_row_echelon_form_with_rank(&b, &neg_den, rank),
                    result
                );
            }
        }
    });
}
