// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Parity, PowerOf2};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::linear_algebra::traits::{
    ModCofactor, ModPowerOf2Cofactor, ModPowerOf2Determinant, ModPowerOf2Minor,
};
use malachite_base::num::modular::traits::{ModPowerOf2Add, ModPowerOf2Mul, ModPowerOf2Neg};
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_unsigned_pair_gen_var_8,
    natural_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_1,
};
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_cofactor() {
    let test = |s, i, j, pow: u64, out: &str| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let x = (&a).mod_power_of_2_cofactor(i, j, pow);
        assert_eq!(x.to_string(), out);
        assert_eq!(a.mod_power_of_2_cofactor(i, j, pow), x);
    };
    test("[5]", 0, 0, 3, "1");
    test("[1, 2; 3, 0]", 0, 0, 2, "0");
    test("[1, 2; 3, 0]", 0, 1, 2, "1");
    test("[1, 2; 3, 0]", 1, 0, 2, "2");
    test("[1, 2; 3, 0]", 1, 1, 2, "1");
    test("[1, 2, 3; 4, 5, 6; 7, 0, 1]", 0, 0, 3, "5");
    test("[1, 2, 3; 4, 5, 6; 7, 0, 1]", 0, 1, 3, "6");
    test("[1, 2, 3; 4, 5, 6; 7, 0, 1]", 0, 2, 3, "5");
    test("[1, 2, 3; 4, 5, 6; 7, 0, 1]", 1, 0, 3, "6");
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        0,
        3,
        "5",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        1,
        3,
        "1",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        2,
        3,
        "2",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        0,
        3,
        3,
        "0",
    );
    test("[0, 0; 0, 0]", 0, 0, 0, "0");
    test("[0, 0; 0, 0]", 0, 1, 0, "0");
    test("[0, 0; 0, 0]", 1, 0, 0, "0");
    test("[0, 0; 0, 0]", 1, 1, 0, "0");
}

#[test]
fn mod_power_of_2_cofactor_fail() {
    assert_panic!(
        NaturalMatrix::from_str("[1, 2, 3; 4, 5, 6]")
            .unwrap()
            .mod_power_of_2_cofactor(0, 0, 3)
    );
    assert_panic!(
        NaturalMatrix::from_str("[1, 2; 3, 4]")
            .unwrap()
            .mod_power_of_2_cofactor(2, 0, 3)
    );
    assert_panic!(
        NaturalMatrix::from_str("[1, 2; 3, 4]")
            .unwrap()
            .mod_power_of_2_cofactor(0, 2, 3)
    );
    assert_panic!(
        NaturalMatrix::from_str("[9, 2; 3, 4]")
            .unwrap()
            .mod_power_of_2_cofactor(0, 0, 3)
    );
}

#[test]
fn mod_power_of_2_cofactor_properties() {
    natural_matrix_unsigned_unsigned_unsigned_quadruple_gen_var_1().test_properties(
        |(a, i, j, pow)| {
            let x = (&a).mod_power_of_2_cofactor(i, j, pow);
            let x2 = (&a).mod_power_of_2_minor(i, j, pow);
            assert_eq!(
                x,
                if (i + j).even() {
                    x2
                } else {
                    x2.mod_power_of_2_neg(pow)
                }
            );
            assert_eq!((&a).mod_cofactor(i, j, Natural::power_of_2(pow)), x);
            assert_eq!(
                x,
                NaturalMatrix::mod_cofactor(a, i, j, Natural::power_of_2(pow))
            );
        },
    );

    // Expanding along any row gives the determinant.
    natural_matrix_unsigned_pair_gen_var_8().test_properties(|(a, pow)| {
        let n = a.height();
        let det = (&a).mod_power_of_2_determinant(pow);
        for i in 0..n {
            let mut sum = Natural::ZERO;
            for j in 0..n {
                let c = (&a).mod_power_of_2_cofactor(i, j, pow);
                sum = sum.mod_power_of_2_add(a.get(i, j).unwrap().mod_power_of_2_mul(c, pow), pow);
            }
            assert_eq!(sum, det);
        }
    });
}
