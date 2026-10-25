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
    Gcd, ModPowerOf2, ModPowerOf2IsReduced, ModPowerOf2SmithNormalForm,
    ModPowerOf2SmithNormalFormAssign, PowerOf2, SmithNormalForm,
};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_4;

// The Smith normal form of `a` modulo 2^pow from its Smith normal form over the integers: each
// diagonal element d becomes gcd(d, 2^pow) modulo 2^pow.
fn snf_from_integer_snf(a: &NaturalMatrix, pow: u64) -> NaturalMatrix {
    let mut s = NaturalMatrix::try_from(IntegerMatrix::from(a).smith_normal_form()).unwrap();
    let w = usize::try_from(s.width()).unwrap();
    let d = usize::try_from(s.height().min(s.width())).unwrap();
    let m = Natural::power_of_2(pow);
    for i in 0..d {
        let x = &mut s.elements[i * (w + 1)];
        *x = (&*x).gcd(&m).mod_power_of_2(pow);
    }
    s
}

fn verify(a: &NaturalMatrix, pow: u64) {
    let s = a.mod_power_of_2_smith_normal_form(pow);
    assert!(s.is_valid());
    assert_eq!(a.clone().mod_power_of_2_smith_normal_form(pow), s);
    let mut b = a.clone();
    b.mod_power_of_2_smith_normal_form_assign(pow);
    assert_eq!(b, s);
    assert!(s.mod_power_of_2_is_reduced(pow));
    assert_eq!((s.height(), s.width()), (a.height(), a.width()));
    assert_eq!(snf_from_integer_snf(a, pow), s);
    assert_eq!((&s).mod_power_of_2_smith_normal_form(pow), s);
    assert_eq!(
        a.transpose().mod_power_of_2_smith_normal_form(pow),
        s.transpose()
    );
}

#[test]
fn test_mod_power_of_2_smith_normal_form() {
    let test = |s: &str, pow: u64, out: &str| {
        let a = NaturalMatrix::from_str(s).unwrap();
        assert_eq!((&a).mod_power_of_2_smith_normal_form(pow).to_string(), out);
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
    test("[3, 5; 7, 1]", 100, "[1, 0; 0, 32]");
    test(
        "[633825300114114700748351602688, 6; 4, 2]",
        100,
        "[2, 0; 0, 4]",
    );
    test(
        "[633825300114114700748351602688, 2, 6; 2, 0, 4]",
        100,
        "[2, 0, 0; 0, 2, 0]",
    );
    test("[12, 8; 4, 6]", 4, "[2, 0; 0, 4]");
    test("[2, 4; 6, 0]", 64, "[2, 0; 0, 4]");
}

#[test]
fn mod_power_of_2_smith_normal_form_properties() {
    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        verify(&a, pow);
    });

    // The word-sized forms, including modulo 2^64, agree with those of the corresponding
    // NaturalMatrix.
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        let n = NaturalMatrix::from(a.clone());
        assert_eq!(
            NaturalMatrix::from(a.mod_power_of_2_smith_normal_form(pow)),
            n.mod_power_of_2_smith_normal_form(pow)
        );
    });
}
