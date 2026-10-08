// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModAdd, ModAddAssign, ModIsReduced, ModNeg};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_pair_gen_var_5,
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_2,
    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_2,
};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_mod_add() {
    let test = |s, t, m, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = UnsignedVector::<u8>::from_str(t).unwrap();
        let r = (&v).mod_add(&w, m);
        assert_eq!(r.to_string(), out);
        assert_eq!((&v).mod_add(w.clone(), m), r);
        assert_eq!(v.clone().mod_add(&w, m), r);
        assert_eq!(v.clone().mod_add(w.clone(), m), r);
        let mut x = v.clone();
        x.mod_add_assign(&w, m);
        assert_eq!(x, r);
        let mut x = v;
        x.mod_add_assign(w, m);
        assert_eq!(x, r);
    };
    test("()", "()", 1, "()");
    test("()", "()", 5, "()");
    test("(0)", "(0)", 1, "(0)");
    test("(5, 1, 3)", "(4, 7, 0)", 8, "(1, 0, 3)");
    test("(1, 2)", "(3, 4)", 7, "(4, 6)");
    test("(4, 5)", "(4, 6)", 7, "(1, 4)");
    test("(254, 200)", "(1, 100)", 255, "(0, 45)");
}

#[test]
fn test_mod_add_u64() {
    let v = UnsignedVector::<u64>::from_str("(18446744073709551614, 1)").unwrap();
    let w =
        UnsignedVector::<u64>::from_str("(18446744073709551614, 18446744073709551613)").unwrap();
    assert_eq!(
        v.mod_add(w, u64::MAX).to_string(),
        "(18446744073709551613, 18446744073709551614)"
    );
}

#[test]
#[should_panic]
fn mod_add_fail_1() {
    // An element of self is not reduced.
    (&UnsignedVector::<u8>::from_str("(7, 1)").unwrap())
        .mod_add(&UnsignedVector::<u8>::from_str("(1, 1)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_add_fail_2() {
    // An element of other is not reduced.
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_add(&UnsignedVector::<u8>::from_str("(1, 7)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_add_fail_3() {
    // The dimensions differ.
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_add(&UnsignedVector::<u8>::from_str("(1)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_add_fail_4() {
    // The modulus is zero.
    (&UnsignedVector::<u8>::from_str("()").unwrap())
        .mod_add(&UnsignedVector::<u8>::from_str("()").unwrap(), 0);
}

#[test]
#[should_panic]
fn mod_add_fail_5() {
    UnsignedVector::<u8>::from_str("(7, 1)")
        .unwrap()
        .mod_add(UnsignedVector::<u8>::from_str("(1, 1)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_add_fail_6() {
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_add(UnsignedVector::<u8>::from_str("(1, 7)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_add_assign_fail_1() {
    let mut v = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    v.mod_add_assign(UnsignedVector::<u8>::from_str("(1, 7)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_add_assign_fail_2() {
    let mut v = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    v.mod_add_assign(&UnsignedVector::<u8>::from_str("(1)").unwrap(), 7);
}

fn mod_add_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_2::<T>().test_properties(
        |(v, w, m)| {
            let r = (&v).mod_add(&w, m);
            assert_eq!((&v).mod_add(w.clone(), m), r);
            assert_eq!(v.clone().mod_add(&w, m), r);
            assert_eq!(v.clone().mod_add(w.clone(), m), r);
            let mut x = v.clone();
            x.mod_add_assign(&w, m);
            assert_eq!(x, r);
            let mut x = v.clone();
            x.mod_add_assign(w.clone(), m);
            assert_eq!(x, r);

            // The result is reduced, has the same dimension, and is the elementwise modular sum.
            assert!(r.mod_is_reduced(&m));
            assert_eq!(r.dimension(), v.dimension());
            for ((&x, &y), &z) in v.elements.iter().zip(&w.elements).zip(&r.elements) {
                assert_eq!(z, x.mod_add(y, m));
            }
            // Addition is commutative, and adding the negation gives zero.
            assert_eq!((&w).mod_add(&v, m), r);
            assert!(
                (&v).mod_add((&v).mod_neg(m), m)
                    .elements
                    .iter()
                    .all(|&x| x == T::ZERO)
            );
        },
    );

    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_2::<T>()
        .test_properties(|(u, v, w, m)| {
            // Addition is associative.
            assert_eq!(
                (&u).mod_add(&v, m).mod_add(&w, m),
                (&u).mod_add((&v).mod_add(&w, m), m)
            );
        });

    unsigned_vector_unsigned_pair_gen_var_5::<T>().test_properties(|(v, m)| {
        // The zero vector of the same dimension is the identity.
        let zero = UnsignedVector {
            elements: vec![T::ZERO; v.elements.len()],
        };
        assert_eq!((&v).mod_add(&zero, m), v);
    });
}

#[test]
fn mod_add_properties() {
    apply_fn_to_unsigneds!(mod_add_properties_helper);
}
