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
    ModIsReduced, ModNeg, ModNegAssign, ModPowerOf2Neg, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_pair_gen_var_4, unsigned_matrix_unsigned_pair_gen_var_5,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_neg() {
    fn test<T: PrimitiveUnsigned>(s: &str, m: T, out: &str) {
        let a = UnsignedMatrix::<T>::from_str(s).unwrap();
        let b = (&a).mod_neg(m);
        assert_eq!(b.to_string(), out);
        assert_eq!(a.clone().mod_neg(m), b);
        let mut x = a;
        x.mod_neg_assign(m);
        assert_eq!(x, b);
    }
    test::<u8>("[]_{0,0}", 1, "[]_{0,0}");
    test::<u8>("[]_{0,0}", 7, "[]_{0,0}");
    test::<u8>("[0]", 1, "[0]");
    test::<u8>("[]_{3,0}", 7, "[]_{3,0}");
    test::<u8>("[5, 1, 0]", 7, "[2, 6, 0]");
    test::<u8>("[5, 1; 0, 3]", 7, "[2, 6; 0, 4]");
    test::<u8>("[1; 2; 3]", 4, "[3; 2; 1]");
    // A zero element stays zero, wherever it is.
    test::<u8>("[0, 1]", 255, "[0, 254]");
    test::<u8>("[1, 0]", 255, "[254, 0]");
    test::<u8>("[3, 3]", 6, "[3, 3]");
    test::<u64>(
        "[1, 18446744073709551614]",
        18446744073709551615,
        "[18446744073709551614, 1]",
    );
}

#[test]
#[should_panic]
fn mod_neg_fail_1() {
    // An element is not reduced.
    (&UnsignedMatrix::<u8>::from_str("[7, 1]").unwrap()).mod_neg(7);
}

#[test]
#[should_panic]
fn mod_neg_fail_2() {
    // The modulus is zero.
    (&UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap()).mod_neg(0);
}

#[test]
#[should_panic]
fn mod_neg_assign_fail() {
    // An element is not reduced.
    let mut a = UnsignedMatrix::<u8>::from_str("[7, 1]").unwrap();
    a.mod_neg_assign(7);
}

#[test]
fn mod_neg_properties() {
    unsigned_matrix_unsigned_pair_gen_var_5().test_properties(|(a, m)| {
        let b = (&a).mod_neg(m);
        assert!(b.is_valid());
        assert_eq!(a.clone().mod_neg(m), b);
        let mut x = a.clone();
        x.mod_neg_assign(m);
        assert_eq!(x, b);

        // The result is reduced, has the same shape, and is the elementwise negation.
        assert!(b.mod_is_reduced(&m));
        assert_eq!((b.height(), b.width()), (a.height(), a.width()));
        for (&x, &y) in a.elements_ref().iter().zip(b.elements_ref()) {
            assert_eq!(y, x.mod_neg(m));
        }
        // Negating twice gives the matrix back, and negation commutes with transposition.
        assert_eq!((&b).mod_neg(m), a);
        assert_eq!(a.transpose().mod_neg(m), b.transpose());
    });

    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        // For a power-of-2 modulus that fits in a `u64`, this is `mod_power_of_2_neg`.
        if pow < u64::WIDTH {
            assert_eq!(
                (&a).mod_neg(u64::power_of_2(pow)),
                (&a).mod_power_of_2_neg(pow)
            );
        }
    });
}
