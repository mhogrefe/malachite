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
    Gcd, Mod, ModIsReduced, ModPowerOf2SmithNormalForm, ModSmithNormalForm,
    ModSmithNormalFormAssign, PowerOf2, SmithNormalForm,
};
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_4, unsigned_matrix_unsigned_pair_gen_var_9,
};
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_pair_gen_var_4, natural_matrix_unsigned_pair_gen_var_4,
};

// The Smith normal form of `a` modulo `m` from its Smith normal form over the integers: each
// diagonal element d becomes gcd(d, m) modulo m.
fn snf_from_integer_snf(a: &NaturalMatrix, m: &Natural) -> NaturalMatrix {
    let mut s = NaturalMatrix::try_from(IntegerMatrix::from(a).smith_normal_form()).unwrap();
    let d = usize::try_from(s.height().min(s.width())).unwrap();
    for i in 0..d {
        let x = &mut s[(i, i)];
        *x = (&*x).gcd(m).mod_op(m);
    }
    s
}

fn verify(a: &NaturalMatrix, m: &Natural) {
    let s = a.mod_smith_normal_form(m);
    assert!(s.is_valid());
    assert_eq!(a.clone().mod_smith_normal_form(m.clone()), s);
    let mut b = a.clone();
    b.mod_smith_normal_form_assign(m);
    assert_eq!(b, s);
    assert!(s.mod_is_reduced(m));
    assert_eq!((s.height(), s.width()), (a.height(), a.width()));
    assert_eq!(snf_from_integer_snf(a, m), s);
    assert_eq!((&s).mod_smith_normal_form(m), s);
    assert_eq!(a.transpose().mod_smith_normal_form(m), s.transpose());
}

#[test]
fn test_mod_smith_normal_form() {
    let test = |s: &str, m: &str, out: &str| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!((&a).mod_smith_normal_form(&m).to_string(), out);
    };
    test("[]_{0,0}", "1267650600228229401496703205377", "[]_{0,0}");
    test(
        "[0, 0; 0, 0]",
        "1267650600228229401496703205377",
        "[0, 0; 0, 0]",
    );
    test("[2, 3; 4, 6]", "12", "[1, 0; 0, 0]");
    test(
        "[0, 2, 8, 2, 11; 6, 11, 11, 11, 11; 6, 0, 9, 9, 3]",
        "12",
        "[1, 0, 0, 0, 0; 0, 3, 0, 0, 0; 0, 0, 3, 0, 0]",
    );
    test(
        "[633825300114114700748351602688, 6; 4, 2]",
        "1267650600228229401496703205377",
        "[1, 0; 0, 1]",
    );
    test(
        "[1267650600228229401496703205376, 2; 4, 8]",
        "1267650600228229401496703205377",
        "[1, 0; 0, 1]",
    );
    test(
        "[3458764513820540928, 2305843009213693952; 4611686018427387904, 5764607523034234880]",
        "55340232221128654848",
        "[1152921504606846976, 0; 0, 1152921504606846976]",
    );
}

#[test]
fn mod_smith_normal_form_properties() {
    natural_matrix_natural_pair_gen_var_4().test_properties(|(a, m)| {
        verify(&a, &m);
    });

    // Modulo 2^pow, the form is that of `ModPowerOf2SmithNormalForm`.
    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        assert_eq!(
            (&a).mod_smith_normal_form(Natural::power_of_2(pow)),
            a.mod_power_of_2_smith_normal_form(pow)
        );
    });

    // The word-sized forms agree with those of the corresponding NaturalMatrix.
    unsigned_matrix_unsigned_pair_gen_var_9().test_properties(|(a, m)| {
        assert_eq!(
            NaturalMatrix::from(a.clone().mod_smith_normal_form(m)),
            NaturalMatrix::from(a).mod_smith_normal_form(Natural::from(m))
        );
    });
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        if pow < 64 {
            assert_eq!(
                (&a).mod_smith_normal_form(1u64 << pow),
                a.mod_power_of_2_smith_normal_form(pow)
            );
        }
    });
}
