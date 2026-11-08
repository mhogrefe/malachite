// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::linear_algebra::traits::{
    ModAdjugate, ModPowerOf2Adjugate, ModPowerOf2Cofactor, ModPowerOf2Determinant,
};
use malachite_base::num::modular::traits::ModMul;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_16;
use malachite_base::unsigned_matrix::UnsignedMatrix;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_adjugate() {
    let test = |s, pow: u64, out: &str| {
        let a = UnsignedMatrix::<u64>::from_str(s).unwrap();
        let x = (&a).mod_power_of_2_adjugate(pow);
        assert_eq!(x.to_string(), out);
        assert_eq!(a.mod_power_of_2_adjugate(pow), x);
    };
    test("[]_{0,0}", 3, "[]_{0,0}");
    test("[5]", 3, "[1]");
    test("[1, 2; 3, 0]", 2, "[0, 2; 1, 1]");
    test(
        "[1, 2, 3; 4, 5, 6; 7, 0, 1]",
        3,
        "[5, 6, 5; 6, 4, 6; 5, 6, 5]",
    );
    test(
        "[2, 0, 1, 3; 3, 1, 0, 2; 0, 4, 2, 1; 1, 1, 1, 1]",
        3,
        "[5, 5, 4, 3; 1, 5, 4, 7; 2, 6, 4, 2; 0, 0, 4, 0]",
    );
    test("[0, 0; 0, 0]", 0, "[0, 0; 0, 0]");
    test(
        "[1, 2, 3; 2, 4, 6; 3, 6, 1]",
        3,
        "[0, 0, 0; 0, 0, 0; 0, 0, 0]",
    );
}

#[test]
fn mod_power_of_2_adjugate_fail() {
    assert_panic!(
        UnsignedMatrix::<u64>::from_str("[1, 2, 3; 4, 5, 6]")
            .unwrap()
            .mod_power_of_2_adjugate(3)
    );
    assert_panic!(
        UnsignedMatrix::<u64>::from_str("[9, 2; 3, 4]")
            .unwrap()
            .mod_power_of_2_adjugate(3)
    );
}

#[test]
fn mod_power_of_2_adjugate_properties() {
    unsigned_matrix_unsigned_pair_gen_var_16().test_properties(|(a, pow)| {
        let adj = (&a).mod_power_of_2_adjugate(pow);
        assert!(adj.is_valid());
        let n = a.height();
        for i in 0..n {
            for j in 0..n {
                assert_eq!(
                    *adj.get(i, j).unwrap(),
                    (&a).mod_power_of_2_cofactor(j, i, pow)
                );
            }
        }
        if pow < u64::WIDTH {
            let modulus = u64::power_of_2(pow);
            if modulus != 1 {
                let det = (&a).mod_power_of_2_determinant(pow);
                let scaled_identity = UnsignedMatrix::<u64>::identity(n).mod_mul(det, modulus);
                assert_eq!((&a).mod_mul(&adj, modulus), scaled_identity);
                assert_eq!((&adj).mod_mul(&a, modulus), scaled_identity);
            }
            assert_eq!((&a).mod_adjugate(modulus), adj);
        }
        assert_eq!(a.transpose().mod_power_of_2_adjugate(pow), adj.transpose());
    });
}
