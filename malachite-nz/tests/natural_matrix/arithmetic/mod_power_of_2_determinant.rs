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
    Determinant, ModDeterminant, ModPowerOf2, ModPowerOf2Determinant, PowerOf2,
};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_4;

#[test]
fn test_mod_power_of_2_determinant() {
    let test = |s: &str, pow: u64, out: &str| {
        let a = NaturalMatrix::from_str(s).unwrap();
        assert_eq!((&a).mod_power_of_2_determinant(pow).to_string(), out);
    };
    test("[]_{0,0}", 100, "1");
    test("[0, 0; 0, 0]", 100, "0");
    test(
        "[633825300114114700748351602688]",
        100,
        "633825300114114700748351602688",
    );
    test("[2, 4; 6, 0]", 100, "1267650600228229401496703205352");
    test("[3, 5; 7, 1]", 100, "1267650600228229401496703205344");
    test(
        "[633825300114114700748351602688, 6; 4, 2]",
        100,
        "1267650600228229401496703205352",
    );
    test("[12, 8; 4, 6]", 4, "8");
    test("[2, 4; 6, 0]", 64, "18446744073709551592");
    // - the valuations of the pivots add up to at least pow, so the determinant is 0
    test("[1152921504606846976, 0; 0, 1152921504606846976]", 100, "0");
}

#[test]
fn mod_power_of_2_determinant_properties() {
    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        if !a.is_square() {
            return;
        }
        let d = (&a).mod_power_of_2_determinant(pow);
        assert_eq!(a.clone().mod_power_of_2_determinant(pow), d);
        assert_eq!((&a).mod_determinant(Natural::power_of_2(pow)), d);
        assert_eq!(IntegerMatrix::from(&a).determinant().mod_power_of_2(pow), d);
    });

    // The word-sized determinants, including modulo 2^64, agree with those of the corresponding
    // NaturalMatrix.
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        if !a.is_square() {
            return;
        }
        assert_eq!(
            Natural::from((&a).mod_power_of_2_determinant(pow)),
            NaturalMatrix::from(a).mod_power_of_2_determinant(pow)
        );
    });
}
