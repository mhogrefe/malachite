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
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2NegAssign,
};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_4;

#[test]
fn test_mod_power_of_2_neg() {
    let test = |s, pow, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let b = (&a).mod_power_of_2_neg(pow);
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().mod_power_of_2_neg(pow), b);
        let mut x = a;
        x.mod_power_of_2_neg_assign(pow);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", 0, "[]_{0,0}");
    test("[]_{0,0}", 3, "[]_{0,0}");
    test("[0]", 0, "[0]");
    test("[1]", 1, "[1]");
    test("[]_{3,0}", 3, "[]_{3,0}");
    test("[5, 1, 3]", 3, "[3, 7, 5]");
    test("[5, 1; 3, 0]", 3, "[3, 7; 5, 0]");
    test("[1; 2; 3]", 2, "[3; 2; 1]");
    // A zero element stays zero, wherever it is.
    test("[0, 1]", 8, "[0, 255]");
    test("[1, 0]", 8, "[255, 0]");
    test("[4, 4]", 3, "[4, 4]");
    test(
        "[3, 5]",
        100,
        "[1267650600228229401496703205373, 1267650600228229401496703205371]",
    );
    test(
        "[1000000000000000000000, 1]",
        70,
        "[180591620717411303424, 1180591620717411303423]",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_fail() {
    // An element is not reduced.
    (&NaturalMatrix::from_str("[8, 1]").unwrap()).mod_power_of_2_neg(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_assign_fail() {
    // An element is not reduced.
    let mut a = NaturalMatrix::from_str("[8, 1]").unwrap();
    a.mod_power_of_2_neg_assign(3);
}

#[test]
fn mod_power_of_2_neg_properties() {
    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        let b = (&a).mod_power_of_2_neg(pow);
        assert!(b.is_valid());
        assert_eq!(a.clone().mod_power_of_2_neg(pow), b);
        let mut x = a.clone();
        x.mod_power_of_2_neg_assign(pow);
        assert_eq!(x, b);

        // The result is reduced, has the same shape, and is the elementwise negation.
        assert!(b.mod_power_of_2_is_reduced(pow));
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for (x, y) in a.elements_ref().iter().zip(b.elements_ref()) {
            assert_eq!(*y, x.mod_power_of_2_neg(pow));
            assert_eq!(x.mod_power_of_2_add(y, pow), 0u32);
        }
        // Negating twice gives the matrix back, and negation commutes with transposition.
        assert_eq!((&b).mod_power_of_2_neg(pow), a);
        assert_eq!(a.transpose().mod_power_of_2_neg(pow), b.transpose());
        // It is the negation over the integers, reduced.
        let negated = -IntegerMatrix::from(a.clone());
        assert_eq!(
            b.elements_ref(),
            negated
                .elements_ref()
                .iter()
                .map(|x| x.mod_power_of_2(pow))
                .collect::<Vec<Natural>>()
        );
    });

    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        // The `u64` and `Natural` versions agree.
        assert_eq!(
            NaturalMatrix::from((&a).mod_power_of_2_neg(pow)),
            NaturalMatrix::from(a).mod_power_of_2_neg(pow)
        );
    });
}
