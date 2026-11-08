// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::linear_algebra::traits::{Adjugate, Cofactor, Determinant};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_gen_var_2;
use std::panic::catch_unwind;

#[test]
fn test_adjugate() {
    let test = |s, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let adj = (&a).adjugate();
        assert!(adj.is_valid());
        assert_eq!(adj.to_string(), out);
        assert_eq!(a.adjugate(), adj);
    };
    test("[]_{0,0}", "[]_{0,0}");
    test("[5]", "[1]");
    test("[1, 2; 3, 4]", "[4, -2; -3, 1]");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 8, 10]",
        "[2, 4, -3; 2, -11, 6; -3, 6, -3]",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        "[5, -3, 4, -13; 1, -3, -4, 7; 2, 6, 4, -22; -8, 0, -4, 16]",
    );
    test(
        "[1, 2, 3; 2, 4, 6; 1, 0, 1]",
        "[4, -2, 0; 4, -2, 0; -4, 2, 0]",
    );
    test("[1, 2, 3; 2, 4, 6; 3, 6, 9]", "[0, 0, 0; 0, 0, 0; 0, 0, 0]");
}

#[test]
fn adjugate_fail() {
    assert_panic!(
        NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]")
            .unwrap()
            .adjugate()
    );
}

#[test]
fn adjugate_properties() {
    natural_matrix_gen_var_2().test_properties(|a| {
        let adj = (&a).adjugate();
        assert!(adj.is_valid());
        assert_eq!(a.clone().adjugate(), adj);
        let n = a.height();
        for i in 0..n {
            for j in 0..n {
                assert_eq!(*adj.get(i, j).unwrap(), (&a).cofactor(j, i));
            }
        }
        let det = (&a).determinant();
        let scaled_identity = &IntegerMatrix::identity(n) * &det;
        assert_eq!(&IntegerMatrix::from(&a) * &adj, scaled_identity);
        assert_eq!(&adj * &IntegerMatrix::from(&a), scaled_identity);
        assert_eq!(a.transpose().adjugate(), adj.transpose());
        assert_eq!(IntegerMatrix::from(&a).adjugate(), adj);
    });
}
