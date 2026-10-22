// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{CeilingSqrt, Square};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_matrix_pair_gen_var_7;
use malachite_nz::test_util::integer_matrix::arithmetic::determinant_bound::*;
use std::panic::catch_unwind;

#[test]
fn test_solve_bounds() {
    let test = |a, b, out_n, out_d| {
        let a = IntegerMatrix::from_str(a).unwrap();
        let b = IntegerMatrix::from_str(b).unwrap();
        let (n, d) = a.solve_bounds(&b);
        assert_eq!(n.to_string(), out_n);
        assert_eq!(d.to_string(), out_d);
    };
    test("[]_{0,0}", "[]_{0,2}", "0", "1");
    test("[1]", "[5]", "10", "2");
    test("[1, 2; 3, 4]", "[5; 6]", "96", "12");
    test("[1, 2; 3, 4]", "[5, 0; 6, 0]", "96", "12");
    test("[2, 0; 0, 2]", "[]_{2,0}", "0", "5");
    test("[1, 2; 3, 4]", "[0; 0]", "0", "12");
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "[1267650600228229401496703205376; 1]",
        "20370360066886875272954624215275773932371555430551145863300108038156959859215417541342003\
        20",
        "1606938068204233101571475504190334901745783987825591619420160",
    );
    test("[0, 0; 0, 0]", "[1; 1]", "0", "0");
}

#[test]
fn solve_bounds_fail() {
    let a = IntegerMatrix::from_str("[1, 2]").unwrap();
    let b = IntegerMatrix::from_str("[1]").unwrap();
    assert_panic!(a.solve_bounds(&b));
    let a = IntegerMatrix::from_str("[1, 2; 3, 4]").unwrap();
    let b = IntegerMatrix::from_str("[1; 2; 3]").unwrap();
    assert_panic!(a.solve_bounds(&b));
}

#[test]
fn solve_bounds_properties() {
    integer_matrix_pair_gen_var_7().test_properties(|(a, b)| {
        let (n, d) = a.solve_bounds(&b);
        assert_eq!(d, a.determinant_bound());
        let max = (0..b.width())
            .map(|j| {
                b.column_iter(j)
                    .map(|x| x.unsigned_abs_ref().square())
                    .sum::<Natural>()
            })
            .max()
            .unwrap_or_default();
        assert_eq!(n, &d * max.ceiling_sqrt());
        let (n_exact, d_exact) = integer_matrix_solve_bounds_naive(&a, &b);
        assert!(n >= n_exact);
        assert!(d >= d_exact);
    });
}
