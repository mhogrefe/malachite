// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Permanent;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_gen_var_2;
use malachite_nz::test_util::integer_matrix::arithmetic::permanent::integer_matrix_permanent_naive;
use std::panic::catch_unwind;

fn verify(a: &NaturalMatrix) -> String {
    let p = a.clone().permanent();
    assert_eq!(a.permanent(), p);
    let b = IntegerMatrix::from(a);
    assert_eq!((&b).permanent(), Integer::from(&p));
    if b.height() <= 8 {
        assert_eq!(integer_matrix_permanent_naive(&b), Integer::from(&p));
    }
    p.to_string()
}

#[test]
fn test_permanent() {
    let test = |s, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        assert_eq!(verify(&a), out);
    };
    // - dimension other than 2
    test("[]_{0,0}", "1");
    test("[7]", "7");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", "463");
    test("[2, 1, 1, 1; 1, 2, 1, 1; 1, 1, 2, 1; 1, 1, 1, 2]", "65");
    test(
        "[18446744073709551615, 1, 0; 0, 18446744073709551615, 1; 1, 0, 18446744073709551615]",
        "6277101735386680762814942322444851025767571854389858533376",
    );
    // - dimension 2
    test("[1, 2; 3, 4]", "10");
}

#[test]
fn permanent_fail() {
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.clone().permanent());
    assert_panic!((&a).permanent());
}

#[test]
fn permanent_properties() {
    natural_matrix_gen_var_2().test_properties(|a| {
        verify(&a);
    });
}
