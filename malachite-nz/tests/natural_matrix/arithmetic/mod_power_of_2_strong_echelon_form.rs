// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModHowellForm, ModPowerOf2HowellForm, ModPowerOf2HowellFormAssign, ModPowerOf2IsReduced,
    ModPowerOf2StrongEchelonForm, ModPowerOf2StrongEchelonFormAssign, ModStrongEchelonForm,
    PowerOf2,
};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_4;

// Checks the strong echelon and Howell forms of `a` modulo 2^pow: the ways of computing them agree,
// they are reduced, are unchanged by recomputation, and are the general forms modulo 2^pow.
fn verify(a: &NaturalMatrix, pow: u64) {
    let s = a.mod_power_of_2_strong_echelon_form(pow);
    assert!(s.is_valid());
    assert_eq!(a.clone().mod_power_of_2_strong_echelon_form(pow), s);
    let mut b = a.clone();
    b.mod_power_of_2_strong_echelon_form_assign(pow);
    assert_eq!(b, s);
    assert!(s.mod_power_of_2_is_reduced(pow));
    assert_eq!((&s).mod_power_of_2_strong_echelon_form(pow), s);
    let h = a.mod_power_of_2_howell_form(pow);
    assert!(h.is_valid());
    assert_eq!(a.clone().mod_power_of_2_howell_form(pow), h);
    let mut b = a.clone();
    b.mod_power_of_2_howell_form_assign(pow);
    assert_eq!(b, h);
    let m = Natural::power_of_2(pow);
    assert_eq!(a.mod_strong_echelon_form(&m), s);
    assert_eq!(a.mod_howell_form(&m), h);
}

#[test]
fn test_mod_power_of_2_strong_echelon_form() {
    let test = |s: &str, pow: u64, out: &str| {
        let a = NaturalMatrix::from_str(s).unwrap();
        assert_eq!(
            (&a).mod_power_of_2_strong_echelon_form(pow).to_string(),
            out
        );
    };
    test("[]_{0,0}", 100, "[]_{0,0}");
    test("[]_{2,0}", 100, "[]_{2,0}");
    test("[0, 0; 0, 0]", 100, "[0, 0; 0, 0]");
    test(
        "[633825300114114700748351602688]",
        100,
        "[633825300114114700748351602688]",
    );
    test("[2, 4; 6, 0]", 100, "[2, 0; 0, 4]");
    test("[3, 5; 7, 1]", 100, "[1, 23; 0, 32]");
    test(
        "[633825300114114700748351602688, 6; 4, 2]",
        100,
        "[4, 0; 0, 2]",
    );
    test(
        "[633825300114114700748351602688, 2, 6; 2, 0, 4]",
        100,
        "[2, 0, 4; 0, 2, 6; 0, 0, 0]",
    );
    test("[12, 8; 4, 6]", 4, "[4, 0; 0, 2]");
    test("[2, 4; 6, 0]", 64, "[2, 0; 0, 4]");
}

#[test]
fn mod_power_of_2_strong_echelon_form_properties() {
    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        verify(&a, pow);
    });

    // The word-sized forms, including modulo 2^64, which does not fit in a u64, agree with those of
    // the corresponding NaturalMatrix.
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        let n = NaturalMatrix::from(a.clone());
        assert_eq!(
            NaturalMatrix::from((&a).mod_power_of_2_strong_echelon_form(pow)),
            (&n).mod_power_of_2_strong_echelon_form(pow)
        );
        assert_eq!(
            NaturalMatrix::from(a.mod_power_of_2_howell_form(pow)),
            n.mod_howell_form(Natural::power_of_2(pow))
        );
    });
}
