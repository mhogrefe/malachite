// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::integer_matrix::arithmetic::determinant::{
    determinant_bareiss, determinant_cofactor,
};
use malachite_nz::test_util::generators::{integer_matrix_gen_var_1, integer_matrix_gen_var_2};
use malachite_nz::test_util::integer_matrix::arithmetic::determinant::*;
use std::panic::catch_unwind;

#[test]
fn test_determinant_bareiss() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let det = determinant_bareiss(&a);
        assert_eq!(det.to_string(), out);
        assert_eq!(integer_matrix_determinant_naive(&a), det);
    };
    test("[]_{0,0}", "1");
    test("[0]", "0");
    test("[-7]", "-7");
    test("[1, 2; 3, 4]", "-2");
    test("[0, 1; 1, 0]", "-1");
    test("[2, 4; 1, 2]", "0");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", "-3");
    test("[0, 0, 1; 0, 1, 0; 1, 0, 0]", "-1");
    test("[2, 0, 0, 0; 0, 3, 0, 0; 0, 0, 5, 0; 0, 0, 0, 7]", "210");
    test("[1, 2, 3, 4; 5, 6, 7, 8; 2, 6, 4, 8; 3, 1, 1, 2]", "72");
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "1606938044258990275541962092341162602522202993782792835301375",
    );
    test(
        "[0, 2, 0, 0, 0; 1, 0, 0, 0, 0; 0, 0, 3, 0, 0; 0, 0, 0, 0, 5; 0, 0, 0, 4, 0]",
        "120",
    );
}

#[test]
fn test_determinant_cofactor() {
    let test = |s, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let det = determinant_cofactor(&a);
        assert_eq!(det.to_string(), out);
        assert_eq!(integer_matrix_determinant_naive(&a), det);
    };
    test("[]_{0,0}", "1");
    test("[0]", "0");
    test("[-7]", "-7");
    test("[1, 2; 3, 4]", "-2");
    test("[0, 1; 1, 0]", "-1");
    test("[2, 4; 1, 2]", "0");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", "-3");
    test("[0, 0, 1; 0, 1, 0; 1, 0, 0]", "-1");
    test("[2, 0, 0, 0; 0, 3, 0, 0; 0, 0, 5, 0; 0, 0, 0, 7]", "210");
    test("[1, 2, 3, 4; 5, 6, 7, 8; 2, 6, 4, 8; 3, 1, 1, 2]", "72");
    test(
        "[1267650600228229401496703205376, 1; 1, 1267650600228229401496703205376]",
        "1606938044258990275541962092341162602522202993782792835301375",
    );
}

#[test]
fn determinant_fail() {
    let a = IntegerMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(determinant_bareiss(&a));
    assert_panic!(determinant_cofactor(&a));
    let a = IntegerMatrix::from_str(
        "[1, 0, 0, 0, 0; 0, 1, 0, 0, 0; 0, 0, 1, 0, 0; 0, 0, 0, 1, 0; 0, 0, 0, 0, 1]",
    )
    .unwrap();
    assert_panic!(determinant_cofactor(&a));
}

#[test]
fn determinant_properties() {
    integer_matrix_gen_var_1().test_properties(|a| {
        let det = determinant_bareiss(&a);
        assert_eq!(integer_matrix_determinant_naive(&a), det);
        assert_eq!(determinant_bareiss(&a.transpose()), det);
        assert!(*det.unsigned_abs_ref() <= a.determinant_bound());
    });

    integer_matrix_gen_var_2().test_properties(|a| {
        assert_eq!(determinant_cofactor(&a), determinant_bareiss(&a));
    });
}
