// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_q::rational_matrix::RationalMatrix;
use malachite_q::rational_matrix::conversion::to_numerators_and_row_denominators::*;
use malachite_q::test_util::generators::rational_matrix_pair_gen_var_3;
use std::panic::catch_unwind;

#[test]
fn test_to_numerators_and_row_denominators_2() {
    let test = |a, b, out_a, out_b, out_d| {
        let a = RationalMatrix::from_str(a).unwrap();
        let b = RationalMatrix::from_str(b).unwrap();
        let (na, nb, d) = to_numerators_and_row_denominators_2(&a, &b);
        assert_eq!(na.to_string(), out_a);
        assert_eq!(nb.to_string(), out_b);
        assert_eq!(d.to_string(), out_d);
    };
    test("[]_{0,0}", "[]_{0,2}", "[]_{0,0}", "[]_{0,2}", "()");
    test("[]_{2,0}", "[1/2; 1/3]", "[]_{2,0}", "[1; 1]", "(2, 3)");
    test(
        "[1/2, 1/3; 1, -1/4]",
        "[1/5; 2]",
        "[15, 10; 4, -1]",
        "[6; 8]",
        "(30, 4)",
    );
    test("[1, 2]", "[3, 4]", "[1, 2]", "[3, 4]", "(1)");
}

#[test]
fn to_numerators_and_row_denominators_2_fail() {
    let a = RationalMatrix::from_str("[1]").unwrap();
    let b = RationalMatrix::from_str("[1; 2]").unwrap();
    assert_panic!(to_numerators_and_row_denominators_2(&a, &b));
}

#[test]
fn to_numerators_and_row_denominators_2_properties() {
    rational_matrix_pair_gen_var_3().test_properties(|(a, b)| {
        let (na, nb, d) = to_numerators_and_row_denominators_2(&a, &b);
        // The result is that of the single-matrix conversion of the two matrices side by side.
        let joined = a.concat_horizontal(&b);
        let (n, e) = joined.to_numerators_and_row_denominators();
        assert_eq!(d, e);
        assert_eq!(na.concat_horizontal(&nb), n);
        assert_eq!(na.height(), a.height());
        assert_eq!(nb.width(), b.width());
    });
}
