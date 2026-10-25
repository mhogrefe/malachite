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
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_matrix_unsigned_matrix_unsigned_quadruple_gen_var_1,
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_1,
    unsigned_matrix_unsigned_pair_gen_var_4,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_power_of_2_add() {
    let test = |s, t, pow, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let b = UnsignedMatrix::<u8>::from_str(t).unwrap();
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
    test("[255, 128]", "[1, 128]", 8, "[0, 0]");
}

#[test]
fn test_mod_power_of_2_add_u64() {
    let a = UnsignedMatrix::<u64>::from_str("[18446744073709551615, 1]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[1, 18446744073709551615]").unwrap();
    assert_eq!(a.mod_power_of_2_add(b, 64).to_string(), "[0, 0]");
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_1() {
    // An element of self is not reduced.
    (&UnsignedMatrix::<u8>::from_str("[8, 1]").unwrap())
        .mod_power_of_2_add(&UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_2() {
    // An element of other is not reduced.
    (&UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap())
        .mod_power_of_2_add(&UnsignedMatrix::<u8>::from_str("[1, 8]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_3() {
    // The shapes differ.
    (&UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap())
        .mod_power_of_2_add(&UnsignedMatrix::<u8>::from_str("[1]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_4() {
    UnsignedMatrix::<u8>::from_str("[8, 1]")
        .unwrap()
        .mod_power_of_2_add(UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_assign_fail_1() {
    let mut a = UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap();
    a.mod_power_of_2_add_assign(UnsignedMatrix::<u8>::from_str("[1, 8]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_assign_fail_2() {
    let mut a = UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap();
    a.mod_power_of_2_add_assign(&UnsignedMatrix::<u8>::from_str("[1]").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_5() {
    // pow is too large.
    (&UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap())
        .mod_power_of_2_add(&UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap(), 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_shape_fail() {
    // The same number of elements is not enough: the shapes must match.
    (&UnsignedMatrix::<u64>::from_str("[1, 1]").unwrap())
        .mod_power_of_2_add(&UnsignedMatrix::<u64>::from_str("[1; 1]").unwrap(), 3);
}

#[test]
fn mod_power_of_2_add_properties() {
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_1().test_properties(|(a, b, pow)| {
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
        for ((&x, &y), &z) in a
            .elements_ref()
            .iter()
            .zip(b.elements_ref())
            .zip(r.elements_ref())
        {
            assert_eq!(z, x.mod_power_of_2_add(y, pow));
        }
        // Addition is commutative, and adding the negation gives zero.
        assert_eq!((&b).mod_power_of_2_add(&a, pow), r);
        assert!(
            (&a).mod_power_of_2_add((&a).mod_power_of_2_neg(pow), pow)
                .is_zero()
        );
    });

    unsigned_matrix_unsigned_matrix_unsigned_matrix_unsigned_quadruple_gen_var_1().test_properties(
        |(c, a, b, pow)| {
            // Addition is associative.
            assert_eq!(
                (&c).mod_power_of_2_add(&a, pow).mod_power_of_2_add(&b, pow),
                (&c).mod_power_of_2_add((&a).mod_power_of_2_add(&b, pow), pow)
            );
        },
    );

    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        // The zero matrix of the same shape is the identity.
        let zero = UnsignedMatrix::<u64>::zero(a.height(), a.width());
        assert_eq!((&a).mod_power_of_2_add(&zero, pow), a);
        // Adding a matrix to itself is reducing its double.
        let doubled = UnsignedMatrix::from_owned_elements(
            a.height(),
            a.width(),
            a.elements_ref()
                .iter()
                .map(|&x| x.wrapping_add(x))
                .collect(),
        );
        assert_eq!(
            (&a).mod_power_of_2_add(&a, pow),
            doubled.mod_power_of_2(pow)
        );
    });
}
