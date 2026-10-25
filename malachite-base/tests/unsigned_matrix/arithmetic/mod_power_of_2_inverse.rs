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
    ModPowerOf2, ModPowerOf2Determinant, ModPowerOf2Inverse, ModPowerOf2Mul, Parity,
};
use malachite_base::strings::ToDebugString;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_power_of_2_inverse() {
    let test = |s: &str, pow: u64, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        assert_eq!((&a).mod_power_of_2_inverse(pow).to_debug_string(), out);
    };
    test("[]_{0,0}", 3, "Some([]_{0,0})");
    test("[0, 0; 0, 0]", 3, "None");
    test("[0]", 0, "Some([0])");
    test("[5]", 3, "Some([5])");
    test("[6]", 3, "None");
    test("[2, 4; 6, 0]", 3, "None");
    test("[1, 2; 3, 4]", 3, "None");
    test("[2, 6; 4, 2]", 3, "None");
    test("[3, 5; 7, 1]", 4, "None");
    test("[12, 8; 4, 6]", 4, "None");
    test("[1, 2; 3, 5]", 3, "Some([3, 2; 3, 7])");
    test("[3, 1; 2, 1]", 8, "Some([1, 255; 254, 3])");
}

#[test]
fn mod_power_of_2_inverse_properties() {
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        if !a.is_square() {
            return;
        }
        let inv = (&a).mod_power_of_2_inverse(pow);
        assert_eq!(a.clone().mod_power_of_2_inverse(pow), inv);
        let det = (&a).mod_power_of_2_determinant(pow);
        assert_eq!(inv.is_some(), pow == 0 || det.odd());
        if let Some(inv) = inv {
            assert!(inv.is_valid());
            let identity = UnsignedMatrix::<u64>::identity(a.height()).mod_power_of_2(pow);
            assert_eq!((&a).mod_power_of_2_mul(&inv, pow), identity);
            assert_eq!((&inv).mod_power_of_2_mul(&a, pow), identity);
            assert_eq!((&inv).mod_power_of_2_inverse(pow), Some(a));
        }
    });
}
