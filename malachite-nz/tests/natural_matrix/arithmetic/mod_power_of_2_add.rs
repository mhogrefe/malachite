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
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2AddAssign, ModPowerOf2IsReduced, ModPowerOf2Neg,
};
use malachite_base::test_util::generators::*;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_matrix_natural_matrix_unsigned_quadruple_gen_var_1,
    natural_matrix_natural_matrix_unsigned_triple_gen_var_1,
    natural_matrix_unsigned_pair_gen_var_4,
};

#[test]
fn test_mod_power_of_2_add() {
    let test = |s, t, pow, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let b = NaturalMatrix::from_str(t).unwrap();
        let r = (&a).mod_power_of_2_add(&b, pow);
        assert_eq!(r.to_string(), out);
        assert_eq!((&a).mod_power_of_2_add(b.clone(), pow), r);
        assert_eq!(a.clone().mod_power_of_2_add(&b, pow), r);
        assert_eq!(a.clone().mod_power_of_2_add(b.clone(), pow), r);
        let mut x = a.clone();
        x.mod_power_of_2_add_assign(&b, pow);
        assert_eq!(x, r);
        let mut x = a;
        x.mod_power_of_2_add_assign(b, pow);
        assert_eq!(x, r);
    };
    test("[]_{0,0}", "[]_{0,0}", 0, "[]_{0,0}");
    test("[]_{0,0}", "[]_{0,0}", 5, "[]_{0,0}");
    test("[0]", "[0]", 0, "[0]");
    test("[]_{3,0}", "[]_{3,0}", 3, "[]_{3,0}");
    test("[5, 1; 3, 2]", "[4, 7; 0, 6]", 3, "[1, 0; 3, 0]");
    test("[5, 1, 3]", "[4, 7, 0]", 3, "[1, 0, 3]");
    test("[1, 2]", "[3, 4]", 8, "[4, 6]");
    test(
        "[1000000000000000000000, 1]",
        "[180591620717411303424, 1]",
        70,
        "[0, 2]",
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_1() {
    // An element of self is not reduced.
    (&NaturalMatrix::from_str("[8, 1]").unwrap())
        .mod_power_of_2_add(&NaturalMatrix::from_str("[1, 1]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_2() {
    // An element of other is not reduced.
    (&NaturalMatrix::from_str("[1, 1]").unwrap())
        .mod_power_of_2_add(&NaturalMatrix::from_str("[1, 8]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_3() {
    // The shapes differ.
    (&NaturalMatrix::from_str("[1, 1]").unwrap())
        .mod_power_of_2_add(&NaturalMatrix::from_str("[1]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_4() {
    NaturalMatrix::from_str("[8, 1]")
        .unwrap()
        .mod_power_of_2_add(NaturalMatrix::from_str("[1, 1]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_assign_fail_1() {
    let mut a = NaturalMatrix::from_str("[1, 1]").unwrap();
    a.mod_power_of_2_add_assign(NaturalMatrix::from_str("[1, 8]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_assign_fail_2() {
    let mut a = NaturalMatrix::from_str("[1, 1]").unwrap();
    a.mod_power_of_2_add_assign(&NaturalMatrix::from_str("[1]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_shape_fail() {
    // The same number of elements is not enough: the shapes must match.
    (&NaturalMatrix::from_str("[1, 1]").unwrap())
        .mod_power_of_2_add(&NaturalMatrix::from_str("[1; 1]").unwrap(), 3);
}

#[test]
fn mod_power_of_2_add_properties() {
    natural_matrix_natural_matrix_unsigned_triple_gen_var_1().test_properties(|(a, b, pow)| {
        let r = (&a).mod_power_of_2_add(&b, pow);
        assert_eq!((&a).mod_power_of_2_add(b.clone(), pow), r);
        assert_eq!(a.clone().mod_power_of_2_add(&b, pow), r);
        assert_eq!(a.clone().mod_power_of_2_add(b.clone(), pow), r);
        let mut x = a.clone();
        x.mod_power_of_2_add_assign(&b, pow);
        assert_eq!(x, r);
        let mut x = a.clone();
        x.mod_power_of_2_add_assign(b.clone(), pow);
        assert_eq!(x, r);

        // The operation commutes with transposition.

        assert_eq!(
            a.transpose().mod_power_of_2_add(b.transpose(), pow),
            r.transpose()
        );

        // The result is reduced, has the same shape, and is the elementwise modular sum.
        assert!(r.mod_power_of_2_is_reduced(pow));
        assert_eq!((r.height(), r.width()), (a.height(), a.width()));
        for ((x, y), z) in a.elements.iter().zip(&b.elements).zip(&r.elements) {
            assert_eq!(*z, x.mod_power_of_2_add(y, pow));
        }
        // It is the ordinary sum, reduced.
        assert_eq!((&a + &b).mod_power_of_2(pow), r);
        // Addition is commutative, and adding the negation gives zero.
        assert_eq!((&b).mod_power_of_2_add(&a, pow), r);
        assert!(
            (&a).mod_power_of_2_add((&a).mod_power_of_2_neg(pow), pow)
                .is_zero()
        );
    });

    natural_matrix_natural_matrix_natural_matrix_unsigned_quadruple_gen_var_1().test_properties(
        |(c, a, b, pow)| {
            // Addition is associative.
            assert_eq!(
                (&c).mod_power_of_2_add(&a, pow).mod_power_of_2_add(&b, pow),
                (&c).mod_power_of_2_add((&a).mod_power_of_2_add(&b, pow), pow)
            );
        },
    );

    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        // The zero matrix of the same shape is the identity.
        let zero = (&a).mod_power_of_2(0);
        assert_eq!((&a).mod_power_of_2_add(&zero, pow), a);
    });

    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_1().test_properties(|(a, b, pow)| {
        // The u64 and Natural versions agree.
        assert_eq!(
            NaturalMatrix::from((&a).mod_power_of_2_add(&b, pow)),
            NaturalMatrix::from(a).mod_power_of_2_add(NaturalMatrix::from(b), pow)
        );
    });
}
