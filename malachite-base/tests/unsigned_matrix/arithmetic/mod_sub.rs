// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModAdd, ModIsReduced, ModNeg, ModSub, ModSubAssign};
use malachite_base::test_util::generators::{
    unsigned_matrix_unsigned_matrix_unsigned_matrix_unsigned_quadruple_gen_var_2,
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_2,
    unsigned_matrix_unsigned_pair_gen_var_5,
};
use malachite_base::unsigned_matrix::UnsignedMatrix;

#[test]
fn test_mod_sub() {
    let test = |s, t, m, out| {
        let a = UnsignedMatrix::<u8>::from_str(s).unwrap();
        let b = UnsignedMatrix::<u8>::from_str(t).unwrap();
        let r = (&a).mod_sub(&b, m);
        assert_eq!(r.to_string(), out);
        assert_eq!((&a).mod_sub(b.clone(), m), r);
        assert_eq!(a.clone().mod_sub(&b, m), r);
        assert_eq!(a.clone().mod_sub(b.clone(), m), r);
        let mut x = a.clone();
        x.mod_sub_assign(&b, m);
        assert_eq!(x, r);
        let mut x = a;
        x.mod_sub_assign(b, m);
        assert_eq!(x, r);
    };
    test("[]_{0,0}", "[]_{0,0}", 1, "[]_{0,0}");
    test("[]_{0,0}", "[]_{0,0}", 5, "[]_{0,0}");
    test("[0]", "[0]", 1, "[0]");
    test("[]_{3,0}", "[]_{3,0}", 8, "[]_{3,0}");
    test("[5, 1; 3, 2]", "[4, 7; 0, 6]", 8, "[1, 2; 3, 4]");
    test("[5, 1, 3]", "[4, 7, 0]", 8, "[1, 2, 3]");
    test("[1, 2]", "[3, 4]", 7, "[5, 5]");
    test("[4, 6]", "[4, 5]", 7, "[0, 1]");
    test("[0, 200]", "[254, 100]", 255, "[1, 100]");
}

#[test]
fn test_mod_sub_u64() {
    let a = UnsignedMatrix::<u64>::from_str("[0, 18446744073709551614]").unwrap();
    let b = UnsignedMatrix::<u64>::from_str("[18446744073709551614, 1]").unwrap();
    assert_eq!(
        a.mod_sub(b, u64::MAX).to_string(),
        "[1, 18446744073709551613]"
    );
}

#[test]
#[should_panic]
fn mod_sub_fail_1() {
    // An element of self is not reduced.
    (&UnsignedMatrix::<u8>::from_str("[7, 1]").unwrap())
        .mod_sub(&UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_sub_fail_2() {
    // An element of other is not reduced.
    (&UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap())
        .mod_sub(&UnsignedMatrix::<u8>::from_str("[1, 7]").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_sub_fail_3() {
    // The shapes differ.
    (&UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap())
        .mod_sub(&UnsignedMatrix::<u8>::from_str("[1]").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_sub_fail_4() {
    // The modulus is zero.
    (&UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap())
        .mod_sub(&UnsignedMatrix::<u8>::from_str("[]_{0,0}").unwrap(), 0);
}

#[test]
#[should_panic]
fn mod_sub_fail_5() {
    UnsignedMatrix::<u8>::from_str("[7, 1]")
        .unwrap()
        .mod_sub(UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_sub_fail_6() {
    (&UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap())
        .mod_sub(UnsignedMatrix::<u8>::from_str("[1, 7]").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_sub_fail_7() {
    // The shapes differ, with other taken by value.
    (&UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap())
        .mod_sub(UnsignedMatrix::<u8>::from_str("[1]").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_sub_assign_fail_1() {
    let mut a = UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap();
    a.mod_sub_assign(UnsignedMatrix::<u8>::from_str("[1, 7]").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_sub_assign_fail_2() {
    let mut a = UnsignedMatrix::<u8>::from_str("[1, 1]").unwrap();
    a.mod_sub_assign(&UnsignedMatrix::<u8>::from_str("[1]").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_sub_shape_fail() {
    // The same number of elements is not enough: the shapes must match.
    (&UnsignedMatrix::<u64>::from_str("[1, 1]").unwrap())
        .mod_sub(&UnsignedMatrix::<u64>::from_str("[1; 1]").unwrap(), 3);
}

#[test]
fn mod_sub_properties() {
    unsigned_matrix_unsigned_matrix_unsigned_triple_gen_var_2().test_properties(|(a, b, m)| {
        let r = (&a).mod_sub(&b, m);
        assert_eq!((&a).mod_sub(b.clone(), m), r);
        assert_eq!(a.clone().mod_sub(&b, m), r);
        assert_eq!(a.clone().mod_sub(b.clone(), m), r);
        let mut x = a.clone();
        x.mod_sub_assign(&b, m);
        assert_eq!(x, r);
        let mut x = a.clone();
        x.mod_sub_assign(b.clone(), m);
        assert_eq!(x, r);

        // The operation commutes with transposition.

        assert_eq!(a.transpose().mod_sub(b.transpose(), m), r.transpose());

        // The result is reduced, has the same shape, and is the elementwise modular difference.
        assert!(r.mod_is_reduced(&m));
        assert_eq!((r.height(), r.width()), (a.height(), a.width()));
        for ((&x, &y), &z) in a.elements.iter().zip(&b.elements).zip(&r.elements) {
            assert_eq!(z, x.mod_sub(y, m));
        }
        // Subtracting is adding the negation, swapping the operands negates the difference, and
        // adding back the subtrahend recovers the minuend.
        assert_eq!((&a).mod_add((&b).mod_neg(m), m), r);
        assert_eq!((&b).mod_sub(&a, m), (&r).mod_neg(m));
        assert_eq!((&r).mod_add(&b, m), a);
    });

    unsigned_matrix_unsigned_matrix_unsigned_matrix_unsigned_quadruple_gen_var_2().test_properties(
        |(c, a, b, m)| {
            assert_eq!(
                (&c).mod_sub(&a, m).mod_sub(&b, m),
                (&c).mod_sub((&a).mod_add(&b, m), m)
            );
        },
    );

    unsigned_matrix_unsigned_pair_gen_var_5().test_properties(|(a, m)| {
        let zero = UnsignedMatrix::<u64>::zero(a.height(), a.width());
        assert_eq!((&a).mod_sub(&zero, m), a);
        assert_eq!((&zero).mod_sub(&a, m), (&a).mod_neg(m));
        assert!((&a).mod_sub(&a, m).is_zero());
    });
}
