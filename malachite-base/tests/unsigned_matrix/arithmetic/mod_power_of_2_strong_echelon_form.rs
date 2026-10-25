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
    ModHowellForm, ModPowerOf2HowellForm, ModPowerOf2HowellFormAssign, ModPowerOf2IsReduced,
    ModPowerOf2StrongEchelonForm, ModPowerOf2StrongEchelonFormAssign, ModStrongEchelonForm,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::unsigned_matrix::UnsignedMatrix;

// Checks the strong echelon and Howell forms of `a` modulo 2^pow: the ways of computing them agree,
// they are reduced, have max(h, w) rows, are unchanged by recomputation, the Howell form has the
// nonzero rows of the strong echelon form in order, and, when 2^pow fits in a u64, both are the
// general forms modulo 2^pow.
fn verify(a: &UnsignedMatrix<u64>, pow: u64) {
    let s = a.mod_power_of_2_strong_echelon_form(pow);
    assert!(s.is_valid());
    assert_eq!(a.clone().mod_power_of_2_strong_echelon_form(pow), s);
    let mut b = a.clone();
    b.mod_power_of_2_strong_echelon_form_assign(pow);
    assert_eq!(b, s);
    assert!(s.mod_power_of_2_is_reduced(pow));
    assert_eq!(s.height(), a.height().max(a.width()));
    assert_eq!(s.width(), a.width());
    assert_eq!((&s).mod_power_of_2_strong_echelon_form(pow), s);
    let h = a.mod_power_of_2_howell_form(pow);
    assert!(h.is_valid());
    assert_eq!(a.clone().mod_power_of_2_howell_form(pow), h);
    let mut b = a.clone();
    b.mod_power_of_2_howell_form_assign(pow);
    assert_eq!(b, h);
    assert_eq!((&h).mod_power_of_2_howell_form(pow), h);
    let w = usize::try_from(a.width()).unwrap();
    if w != 0 {
        let nonzero = |m: &UnsignedMatrix<u64>| -> Vec<Vec<u64>> {
            m.elements
                .chunks_exact(w)
                .filter(|row| row.iter().any(|&x| x != 0))
                .map(<[u64]>::to_vec)
                .collect()
        };
        assert_eq!(nonzero(&h), nonzero(&s));
    }
    if pow < u64::WIDTH {
        assert_eq!(a.mod_strong_echelon_form(1 << pow), s);
        assert_eq!(a.mod_howell_form(1 << pow), h);
    }
}

#[test]
fn test_mod_power_of_2_strong_echelon_form() {
    let test = |s: &str, pow: u64, out: &str| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        assert_eq!(
            (&a).mod_power_of_2_strong_echelon_form(pow).to_string(),
            out
        );
    };
    test("[]_{0,0}", 3, "[]_{0,0}");
    test("[]_{2,0}", 3, "[]_{2,0}");
    test("[]_{0,2}", 3, "[0, 0; 0, 0]");
    test("[0, 0; 0, 0]", 3, "[0, 0; 0, 0]");
    test("[0]", 0, "[0]");
    test("[5]", 3, "[1]");
    test("[6]", 3, "[2]");
    test("[2, 4; 6, 0]", 3, "[2, 0; 0, 4]");
    test("[1, 2; 3, 4]", 3, "[1, 0; 0, 2]");
    test("[2, 6; 4, 2]", 3, "[2, 0; 0, 2]");
    test("[4, 2, 6; 2, 0, 4]", 3, "[2, 0, 4; 0, 2, 6; 0, 0, 0]");
    test("[0, 2; 0, 4; 0, 6]", 3, "[0, 0; 0, 2; 0, 0]");
    test("[3, 5; 7, 1]", 4, "[1, 7; 0, 0]");
    test("[12, 8; 4, 6]", 4, "[4, 0; 0, 2]");
}

#[test]
fn mod_power_of_2_strong_echelon_form_properties() {
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        verify(&a, pow);
    });
}
