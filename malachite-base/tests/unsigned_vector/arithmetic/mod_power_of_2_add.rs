// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Add, ModPowerOf2AddAssign, ModPowerOf2IsReduced, ModPowerOf2Neg,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_pair_gen_var_4,
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1,
    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_1,
};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_mod_power_of_2_add() {
    let test = |s, t, pow, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = UnsignedVector::<u8>::from_str(t).unwrap();
        let r = (&v).mod_power_of_2_add(&w, pow);
        assert_eq!(r.to_string(), out);
        assert_eq!((&v).mod_power_of_2_add(w.clone(), pow), r);
        assert_eq!(v.clone().mod_power_of_2_add(&w, pow), r);
        assert_eq!(v.clone().mod_power_of_2_add(w.clone(), pow), r);
        let mut x = v.clone();
        x.mod_power_of_2_add_assign(&w, pow);
        assert_eq!(x, r);
        let mut x = v;
        x.mod_power_of_2_add_assign(w, pow);
        assert_eq!(x, r);
    };
    test("()", "()", 0, "()");
    test("()", "()", 5, "()");
    test("(0)", "(0)", 0, "(0)");
    test("(5, 1, 3)", "(4, 7, 0)", 3, "(1, 0, 3)");
    test("(1, 2)", "(3, 4)", 8, "(4, 6)");
    test("(255, 128)", "(1, 128)", 8, "(0, 0)");
}

#[test]
fn test_mod_power_of_2_add_u64() {
    let v = UnsignedVector::<u64>::from_str("(18446744073709551615, 1)").unwrap();
    let w = UnsignedVector::<u64>::from_str("(1, 18446744073709551615)").unwrap();
    assert_eq!(v.mod_power_of_2_add(w, 64).to_string(), "(0, 0)");
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_1() {
    // An element of self is not reduced.
    (&UnsignedVector::<u8>::from_str("(8, 1)").unwrap())
        .mod_power_of_2_add(&UnsignedVector::<u8>::from_str("(1, 1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_2() {
    // An element of other is not reduced.
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_power_of_2_add(&UnsignedVector::<u8>::from_str("(1, 8)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_3() {
    // The dimensions differ.
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_power_of_2_add(&UnsignedVector::<u8>::from_str("(1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_4() {
    UnsignedVector::<u8>::from_str("(8, 1)")
        .unwrap()
        .mod_power_of_2_add(UnsignedVector::<u8>::from_str("(1, 1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_assign_fail_1() {
    let mut v = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    v.mod_power_of_2_add_assign(UnsignedVector::<u8>::from_str("(1, 8)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_assign_fail_2() {
    let mut v = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    v.mod_power_of_2_add_assign(&UnsignedVector::<u8>::from_str("(1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_add_fail_5() {
    // pow is too large.
    (&UnsignedVector::<u8>::from_str("()").unwrap())
        .mod_power_of_2_add(&UnsignedVector::<u8>::from_str("()").unwrap(), 9);
}

fn mod_power_of_2_add_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1::<T>().test_properties(
        |(v, w, pow)| {
            let r = (&v).mod_power_of_2_add(&w, pow);
            assert_eq!((&v).mod_power_of_2_add(w.clone(), pow), r);
            assert_eq!(v.clone().mod_power_of_2_add(&w, pow), r);
            assert_eq!(v.clone().mod_power_of_2_add(w.clone(), pow), r);
            let mut x = v.clone();
            x.mod_power_of_2_add_assign(&w, pow);
            assert_eq!(x, r);
            let mut x = v.clone();
            x.mod_power_of_2_add_assign(w.clone(), pow);
            assert_eq!(x, r);

            // The result is reduced, has the same dimension, and is the elementwise modular sum.
            assert!(r.mod_power_of_2_is_reduced(pow));
            assert_eq!(r.dimension(), v.dimension());
            for ((&x, &y), &z) in v.elements.iter().zip(&w.elements).zip(&r.elements) {
                assert_eq!(z, x.mod_power_of_2_add(y, pow));
            }
            // Addition is commutative, and adding the negation gives zero.
            assert_eq!((&w).mod_power_of_2_add(&v, pow), r);
            assert!(
                (&v).mod_power_of_2_add((&v).mod_power_of_2_neg(pow), pow)
                    .elements
                    .iter()
                    .all(|&x| x == T::ZERO)
            );
        },
    );

    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_1::<T>()
        .test_properties(|(u, v, w, pow)| {
            // Addition is associative.
            assert_eq!(
                (&u).mod_power_of_2_add(&v, pow).mod_power_of_2_add(&w, pow),
                (&u).mod_power_of_2_add((&v).mod_power_of_2_add(&w, pow), pow)
            );
        });

    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        // The zero vector of the same dimension is the identity.
        let zero = UnsignedVector {
            elements: vec![T::ZERO; v.elements.len()],
        };
        assert_eq!((&v).mod_power_of_2_add(&zero, pow), v);
        // Adding a vector to itself is reducing its double.
        let doubled = UnsignedVector {
            elements: v.elements.iter().map(|&x| x.wrapping_add(x)).collect(),
        };
        assert_eq!(
            (&v).mod_power_of_2_add(&v, pow),
            doubled.mod_power_of_2(pow)
        );
    });
}

#[test]
fn mod_power_of_2_add_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_add_properties_helper);
}
