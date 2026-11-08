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
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::linear_algebra::traits::{
    Cofactor, ModCofactor, ModDeterminant, ModMinor,
};
use malachite_base::num::modular::traits::{Mod, ModAdd, ModMul, ModNeg};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_pair_gen_var_9,
    natural_matrix_unsigned_unsigned_natural_quadruple_gen_var_1,
};
use std::panic::catch_unwind;

#[test]
fn test_mod_cofactor() {
    let test = |s, i, j, m: u32, out: &str| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from(m);
        let x = (&a).mod_cofactor(i, j, &m);
        assert_eq!(x.to_string(), out);
        assert_eq!((&a).mod_cofactor(i, j, m.clone()), x);
        assert_eq!(a.clone().mod_cofactor(i, j, &m), x);
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
        NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]")
            .unwrap()
            .mod_cofactor(0, 0, Natural::from(7u32))
    );
    assert_panic!(
        NaturalMatrix::from_str("[1, 2; 3, 4]")
            .unwrap()
            .mod_cofactor(2, 0, Natural::from(7u32))
    );
    assert_panic!(
        NaturalMatrix::from_str("[1, 2; 3, 4]")
            .unwrap()
            .mod_cofactor(0, 2, Natural::from(7u32))
    );
    assert_panic!(
        NaturalMatrix::from_str("[9, 2; 3, 4]")
            .unwrap()
            .mod_cofactor(0, 0, Natural::from(7u32))
    );
    assert_panic!(
        NaturalMatrix::from_str("[1, 2; 3, 4]")
            .unwrap()
            .mod_cofactor(0, 0, Natural::ZERO)
    );
}

#[test]
fn mod_cofactor_properties() {
    natural_matrix_unsigned_unsigned_natural_quadruple_gen_var_1().test_properties(
        |(a, i, j, m)| {
            let x = (&a).mod_cofactor(i, j, &m);
            let x2 = (&a).mod_minor(i, j, &m);
            assert_eq!(x, if (i + j).even() { x2 } else { x2.mod_neg(&m) });
            assert_eq!(
                Natural::exact_from((&a).cofactor(i, j).mod_op(Integer::from(&m))),
                x
            );
        },
    );

    // Expanding along any row gives the determinant.
    natural_matrix_natural_pair_gen_var_9().test_properties(|(a, m)| {
        let n = a.height();
        let det = (&a).mod_determinant(&m);
        for i in 0..n {
            let mut sum = Natural::ZERO;
            for j in 0..n {
                let c = (&a).mod_cofactor(i, j, &m);
                sum = sum.mod_add(a.get(i, j).unwrap().mod_mul(c, &m), &m);
            }
            assert_eq!(sum, det);
        }
    });
}
