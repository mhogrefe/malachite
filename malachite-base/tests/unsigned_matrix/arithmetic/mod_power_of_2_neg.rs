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
    ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2NegAssign,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_power_of_2_neg() {
    fn test<T: PrimitiveUnsigned>(s: &str, pow: u64, out: &str) {
        let a = UnsignedMatrix::<T>::from_str(s).unwrap();
        let b = (&a).mod_power_of_2_neg(pow);
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().mod_power_of_2_neg(pow), b);
        let mut x = a;
        x.mod_power_of_2_neg_assign(pow);
        assert_eq!(x, b);
    }
    test::<u8>("[]_{0,0}", 0, "[]_{0,0}");
    test::<u8>("[]_{0,0}", 3, "[]_{0,0}");
    test::<u8>("[0]", 0, "[0]");
    test::<u8>("[1]", 1, "[1]");
    test::<u8>("[]_{3,0}", 3, "[]_{3,0}");
    test::<u8>("[5, 1, 3]", 3, "[3, 7, 5]");
    test::<u8>("[5, 1; 3, 0]", 3, "[3, 7; 5, 0]");
    test::<u8>("[1; 2; 3]", 2, "[3; 2; 1]");
    // A zero element stays zero, wherever it is.
    test::<u8>("[0, 1]", 8, "[0, 255]");
    test::<u8>("[1, 0]", 8, "[255, 0]");
    test::<u8>("[4, 4]", 3, "[4, 4]");
    test::<u64>("[3, 5]", 64, "[18446744073709551613, 18446744073709551611]");
    test::<u128>("[1]", 100, "[1267650600228229401496703205375]");
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_fail_1() {
    // An element is not reduced.
    (&UnsignedMatrix::<u8>::from_str("[8, 1]").unwrap()).mod_power_of_2_neg(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_fail_2() {
    // pow is too large.
    (&UnsignedMatrix::<u8>::from_str("[1]").unwrap()).mod_power_of_2_neg(9);
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_assign_fail() {
    // An element is not reduced.
    let mut a = UnsignedMatrix::<u8>::from_str("[8, 1]").unwrap();
    a.mod_power_of_2_neg_assign(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_empty_matrix_fail() {
    // pow is too large, even though there are no elements to check.
    UnsignedMatrix::<u8>::from_str("[]_{0,0}")
        .unwrap()
        .mod_power_of_2_neg(9);
}

#[test]
fn mod_power_of_2_neg_properties() {
    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        let b = (&a).mod_power_of_2_neg(pow);
        assert!(b.is_valid());
        assert_eq!(a.clone().mod_power_of_2_neg(pow), b);
        let mut x = a.clone();
        x.mod_power_of_2_neg_assign(pow);
        assert_eq!(x, b);

        // The result is reduced, has the same shape, and is the elementwise negation.
        assert!(b.mod_power_of_2_is_reduced(pow));
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for (&x, &y) in a.elements.iter().zip(&b.elements) {
            assert_eq!(y, x.mod_power_of_2_neg(pow));
            assert_eq!(x.mod_power_of_2_add(y, pow), 0);
        }
        // Negating twice gives the matrix back, and negation commutes with transposition.
        assert_eq!((&b).mod_power_of_2_neg(pow), a);
        assert_eq!(a.transpose().mod_power_of_2_neg(pow), b.transpose());
    });
}
