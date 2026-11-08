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
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_gen_var_2, natural_matrix_unsigned_unsigned_triple_gen_var_6,
};
use std::panic::catch_unwind;

#[test]
fn test_cofactor() {
    let test = |s, i, j, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let c = (&a).cofactor(i, j);
        assert_eq!(c.to_string(), out);
        assert_eq!(a.cofactor(i, j), c);
    };
    test("[5]", 0, 0, "1");
    test("[1, 2; 3, 4]", 0, 0, "4");
    test("[1, 2; 3, 4]", 0, 1, "-3");
    test("[1, 2; 3, 4]", 1, 0, "-2");
    test("[1, 2; 3, 4]", 1, 1, "1");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", 0, 0, "2");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", 0, 1, "2");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", 0, 2, "-3");
    test("[1, 2, 3; 4, 5, 6; 7, 8, 10]", 1, 0, "4");
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        0,
        "5",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        1,
        "1",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        2,
        "2",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        3,
        "-8",
    );
    test("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 0, 0, "4");
    test("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 0, 1, "4");
    test("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 0, 2, "-4");
    test("[1, 2, 3; 2, 4, 6; 1, 0, 1]", 1, 0, "-2");
}

#[test]
fn cofactor_fail() {
    assert_panic!(
        NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]")
            .unwrap()
            .cofactor(0, 0)
    );
    assert_panic!(
        NaturalMatrix::from_str("[1, 2; 3, 4]")
            .unwrap()
            .cofactor(2, 0)
    );
    assert_panic!(
        NaturalMatrix::from_str("[1, 2; 3, 4]")
            .unwrap()
            .cofactor(0, 2)
    );
}

#[test]
fn cofactor_properties() {
    natural_matrix_unsigned_unsigned_triple_gen_var_6().test_properties(|(a, i, j)| {
        let c = (&a).cofactor(i, j);
        assert_eq!(a.clone().cofactor(i, j), c);
        let m = (&a).minor(i, j);
        assert_eq!(c, if (i + j).even() { m } else { -m });
        assert_eq!(IntegerMatrix::from(&a).cofactor(i, j), c);
    });

    // Expanding along any row or column gives the determinant.
    natural_matrix_gen_var_2().test_properties(|a| {
        let n = a.height();
        let det = (&a).determinant();
        for i in 0..n {
            let mut row_sum = Integer::ZERO;
            let mut column_sum = Integer::ZERO;
            for j in 0..n {
                let x = a.get(i, j).unwrap().clone();
                row_sum += Integer::from(x) * (&a).cofactor(i, j);
                let x = a.get(j, i).unwrap().clone();
                column_sum += Integer::from(x) * (&a).cofactor(j, i);
            }
            assert_eq!(row_sum, det);
            assert_eq!(column_sum, det);
        }
    });
}
