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
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_4;

#[test]
fn test_mod_power_of_2_inverse() {
    let test = |s: &str, pow: u64, out: &str| {
        let a = NaturalMatrix::from_str(s).unwrap();
        assert_eq!((&a).mod_power_of_2_inverse(pow).to_debug_string(), out);
    };
    test("[]_{0,0}", 100, "Some([]_{0,0})");
    test("[0, 0; 0, 0]", 100, "None");
    test("[633825300114114700748351602688]", 100, "None");
    test("[2, 4; 6, 0]", 100, "None");
    test("[3, 5; 7, 1]", 100, "None");
    test("[633825300114114700748351602688, 6; 4, 2]", 100, "None");
    test("[12, 8; 4, 6]", 4, "None");
    test("[2, 4; 6, 0]", 64, "None");
}

#[test]
fn mod_power_of_2_inverse_properties() {
    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        if !a.is_square() {
            return;
        }
        let inv = (&a).mod_power_of_2_inverse(pow);
        assert_eq!(a.clone().mod_power_of_2_inverse(pow), inv);
        let det = (&a).mod_power_of_2_determinant(pow);
        assert_eq!(inv.is_some(), pow == 0 || det.odd());
        if let Some(inv) = inv {
            assert!(inv.is_valid());
            let identity = NaturalMatrix::identity(a.height()).mod_power_of_2(pow);
            assert_eq!((&a).mod_power_of_2_mul(&inv, pow), identity);
            assert_eq!((&inv).mod_power_of_2_mul(&a, pow), identity);
            assert_eq!((&inv).mod_power_of_2_inverse(pow), Some(a));
        }
    });

    // The word-sized inverses, including modulo 2^64, agree with those of the corresponding
    // NaturalMatrix.
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        if !a.is_square() {
            return;
        }
        assert_eq!(
            (&a).mod_power_of_2_inverse(pow).map(NaturalMatrix::from),
            NaturalMatrix::from(a).mod_power_of_2_inverse(pow)
        );
    });
}
