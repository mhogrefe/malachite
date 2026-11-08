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
use malachite_base::num::linear_algebra::traits::{ModCofactor, ModDeterminant, ModMinor};
use malachite_base::num::modular::traits::{ModAdd, ModMul, ModNeg};
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_15,
    unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_3,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;
use std::panic::catch_unwind;

#[test]
fn test_mod_cofactor() {
    let test = |s, i, j, m: u64, out: &str| {
        let a = UnsignedMatrix::<u64>::from_str(s).unwrap();
        let x = (&a).mod_cofactor(i, j, m);
        assert_eq!(x.to_string(), out);
        assert_eq!(a.mod_cofactor(i, j, m), x);
    };
    test("[5]", 0, 0, 7, "1");
    test("[1, 2; 3, 4]", 0, 0, 5, "4");
    test("[1, 2; 3, 4]", 0, 1, 5, "2");
    test("[1, 2; 3, 4]", 1, 0, 5, "3");
    test("[1, 2; 3, 4]", 1, 1, 5, "1");
    test("[1, 2, 3; 4, 5, 6; 0, 1, 6]", 0, 0, 7, "3");
    test("[1, 2, 3; 4, 5, 6; 0, 1, 6]", 0, 1, 7, "4");
    test("[1, 2, 3; 4, 5, 6; 0, 1, 6]", 0, 2, 7, "4");
    test("[1, 2, 3; 4, 5, 6; 0, 1, 6]", 1, 0, 7, "5");
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        0,
        6,
        "5",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        1,
        6,
        "1",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        2,
        6,
        "2",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        3,
        6,
        "4",
    );
    test("[1, 2, 3; 2, 4, 0; 1, 0, 1]", 0, 0, 6, "4");
    test("[1, 2, 3; 2, 4, 0; 1, 0, 1]", 0, 1, 6, "4");
    test("[1, 2, 3; 2, 4, 0; 1, 0, 1]", 0, 2, 6, "2");
    test("[1, 2, 3; 2, 4, 0; 1, 0, 1]", 1, 0, 6, "4");
    test("[0, 0; 0, 0]", 0, 0, 1, "0");
    test("[0, 0; 0, 0]", 0, 1, 1, "0");
    test("[0, 0; 0, 0]", 1, 0, 1, "0");
    test("[0, 0; 0, 0]", 1, 1, 1, "0");
}

#[test]
fn mod_cofactor_fail() {
    assert_panic!(
        UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]")
            .unwrap()
            .mod_cofactor(0, 0, 7)
    );
    assert_panic!(
        UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]")
            .unwrap()
            .mod_cofactor(2, 0, 7)
    );
    assert_panic!(
        UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]")
            .unwrap()
            .mod_cofactor(0, 2, 7)
    );
    assert_panic!(
        UnsignedMatrix::<u64>::from_str("[9, 2; 3, 4]")
            .unwrap()
            .mod_cofactor(0, 0, 7)
    );
    assert_panic!(
        UnsignedMatrix::<u64>::from_str("[1, 2; 3, 4]")
            .unwrap()
            .mod_cofactor(0, 0, 0)
    );
}

#[test]
fn mod_cofactor_properties() {
    unsigned_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_3().test_properties(
        |(a, i, j, m)| {
            let x = (&a).mod_cofactor(i, j, m);
            let x2 = (&a).mod_minor(i, j, m);
            assert_eq!(x, if (i + j).even() { x2 } else { x2.mod_neg(m) });
        },
    );

    // Expanding along any row gives the determinant.
    unsigned_matrix_unsigned_pair_gen_var_15().test_properties(|(a, m)| {
        let n = a.height();
        let det = (&a).mod_determinant(m);
        for i in 0..n {
            let mut sum = 0u64;
            for j in 0..n {
                let c = (&a).mod_cofactor(i, j, m);
                sum = sum.mod_add((*a.get(i, j).unwrap()).mod_mul(c, m), m);
            }
            assert_eq!(sum, det);
        }
    });
}
