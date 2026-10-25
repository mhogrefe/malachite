// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{
    ModDeterminant, ModPowerOf2Determinant, ModPowerOf2Mul,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_power_of_2_determinant() {
    let test = |s: &str, pow: u64, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        assert_eq!((&a).mod_power_of_2_determinant(pow).to_string(), out);
    };
    test("[]_{0,0}", 3, "1");
    test("[0, 0; 0, 0]", 3, "0");
    test("[0]", 0, "0");
    test("[5]", 3, "5");
    test("[6]", 3, "6");
    test("[2, 4; 6, 0]", 3, "0");
    test("[1, 2; 3, 4]", 3, "6");
    test("[2, 6; 4, 2]", 3, "4");
    test("[3, 5; 7, 1]", 4, "0");
    test("[12, 8; 4, 6]", 4, "8");
    test("[1, 2; 3, 5]", 3, "7");
    test("[3, 1; 2, 1]", 8, "1");
}

#[test]
fn mod_power_of_2_determinant_properties() {
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        if !a.is_square() {
            return;
        }
        let d = (&a).mod_power_of_2_determinant(pow);
        assert_eq!(a.clone().mod_power_of_2_determinant(pow), d);
        assert!(pow == u64::WIDTH || d < 1 << pow);
        if pow < u64::WIDTH {
            assert_eq!((&a).mod_determinant(1 << pow), d);
        }
        assert_eq!(a.transpose().mod_power_of_2_determinant(pow), d);
        let s = (&a).mod_power_of_2_mul(&a, pow);
        assert_eq!(
            s.mod_power_of_2_determinant(pow),
            d.mod_power_of_2_mul(d, pow)
        );
    });
}
