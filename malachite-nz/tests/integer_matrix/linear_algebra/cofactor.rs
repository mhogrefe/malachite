// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::linear_algebra::traits::{Cofactor, Determinant, Minor};
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::test_util::generators::{
    integer_matrix_gen_var_4, integer_matrix_unsigned_unsigned_triple_gen_var_4,
};
use std::panic::catch_unwind;

#[test]
fn test_cofactor() {
    let test = |s, i, j, out| {
        let a = IntegerMatrix::from_str(s).unwrap();
        let c = (&a).cofactor(i, j);
        assert_eq!(c.to_string(), out);
        assert_eq!(a.cofactor(i, j), c);
    };
    test("[5]", 0, 0, "1");
    test("[1, 2; 3, 4]", 0, 0, "4");
    test("[1, 2; 3, 4]", 0, 1, "-3");
    test("[1, 2; 3, 4]", 1, 0, "-2");
    test("[1, 2; 3, 4]", 1, 1, "1");
    test("[1, -2, 3; 4, 5, -6; 7, -8, 9]", 0, 0, "-3");
    test("[1, -2, 3; 4, 5, -6; 7, -8, 9]", 0, 1, "-78");
    test("[1, -2, 3; 4, 5, -6; 7, -8, 9]", 0, 2, "-67");
    test("[1, -2, 3; 4, 5, -6; 7, -8, 9]", 1, 0, "-6");
    test(
        "[2, 0, 1, -1; 3, 1, 0, 2; 0, -4, 2, 1; 1, 1, 1, 1]",
        0,
        0,
        "-11",
    );
    test(
        "[2, 0, 1, -1; 3, 1, 0, 2; 0, -4, 2, 1; 1, 1, 1, 1]",
        0,
        1,
        "1",
    );
    test(
        "[2, 0, 1, -1; 3, 1, 0, 2; 0, -4, 2, 1; 1, 1, 1, 1]",
        0,
        2,
        "-6",
    );
    test(
        "[2, 0, 1, -1; 3, 1, 0, 2; 0, -4, 2, 1; 1, 1, 1, 1]",
        0,
        3,
        "16",
    );
    test("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 0, 0, "4");
    test("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 0, 1, "4");
    test("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 0, 2, "-4");
    test("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 1, 0, "-2");
}

#[test]
fn cofactor_fail() {
    assert_panic!(
        IntegerMatrix::from_str("[1, 2, 3; 4, 5, 6]")
            .unwrap()
            .cofactor(0, 0)
    );
    assert_panic!(
        IntegerMatrix::from_str("[1, 2; 3, 4]")
            .unwrap()
            .cofactor(2, 0)
    );
    assert_panic!(
        IntegerMatrix::from_str("[1, 2; 3, 4]")
            .unwrap()
            .cofactor(0, 2)
    );
}

#[test]
fn cofactor_properties() {
    integer_matrix_unsigned_unsigned_triple_gen_var_4().test_properties(|(a, i, j)| {
        let c = (&a).cofactor(i, j);
        assert_eq!(a.clone().cofactor(i, j), c);
        let m = (&a).minor(i, j);
        assert_eq!(c, if (i + j).even() { m } else { -m });
    });

    // Expanding along any row or column gives the determinant.
    integer_matrix_gen_var_4().test_properties(|a| {
        let n = a.height();
        let det = (&a).determinant();
        for i in 0..n {
            let mut row_sum = Integer::ZERO;
            let mut column_sum = Integer::ZERO;
            for j in 0..n {
                let x = a.get(i, j).unwrap().clone();
                row_sum += x * (&a).cofactor(i, j);
                let x = a.get(j, i).unwrap().clone();
                column_sum += x * (&a).cofactor(j, i);
            }
            assert_eq!(row_sum, det);
            assert_eq!(column_sum, det);
        }
    });
}
