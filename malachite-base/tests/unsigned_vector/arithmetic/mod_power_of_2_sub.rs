// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Add, ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2Sub, ModPowerOf2SubAssign,
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
fn test_mod_power_of_2_sub() {
    let test = |s, t, pow, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = UnsignedVector::<u8>::from_str(t).unwrap();
        let r = (&v).mod_power_of_2_sub(&w, pow);
        assert_eq!(r.to_string(), out);
        assert_eq!((&v).mod_power_of_2_sub(w.clone(), pow), r);
        assert_eq!(v.clone().mod_power_of_2_sub(&w, pow), r);
        assert_eq!(v.clone().mod_power_of_2_sub(w.clone(), pow), r);
        let mut x = v.clone();
        x.mod_power_of_2_sub_assign(&w, pow);
        assert_eq!(x, r);
        let mut x = v;
        x.mod_power_of_2_sub_assign(w, pow);
        assert_eq!(x, r);
    };
    test("()", "()", 0, "()");
    test("()", "()", 5, "()");
    test("(0)", "(0)", 0, "(0)");
    test("(5, 1, 3)", "(4, 7, 0)", 3, "(1, 2, 3)");
    test("(1, 2)", "(3, 4)", 8, "(254, 254)");
    test("(0, 128)", "(1, 128)", 8, "(255, 0)");
}

#[test]
fn test_mod_power_of_2_sub_u64() {
    let v = UnsignedVector::<u64>::from_str("(0, 1)").unwrap();
    let w = UnsignedVector::<u64>::from_str("(1, 18446744073709551615)").unwrap();
    assert_eq!(
        v.mod_power_of_2_sub(w, 64).to_string(),
        "(18446744073709551615, 2)"
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_fail_1() {
    // An element of self is not reduced.
    (&UnsignedVector::<u8>::from_str("(8, 1)").unwrap())
        .mod_power_of_2_sub(&UnsignedVector::<u8>::from_str("(1, 1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_fail_2() {
    // An element of other is not reduced.
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_power_of_2_sub(&UnsignedVector::<u8>::from_str("(1, 8)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_fail_3() {
    // The dimensions differ.
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_power_of_2_sub(&UnsignedVector::<u8>::from_str("(1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_fail_4() {
    UnsignedVector::<u8>::from_str("(8, 1)")
        .unwrap()
        .mod_power_of_2_sub(UnsignedVector::<u8>::from_str("(1, 1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_fail_5() {
    // pow is too large.
    (&UnsignedVector::<u8>::from_str("()").unwrap())
        .mod_power_of_2_sub(&UnsignedVector::<u8>::from_str("()").unwrap(), 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_fail_6() {
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_power_of_2_sub(UnsignedVector::<u8>::from_str("(1, 8)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_fail_7() {
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_power_of_2_sub(UnsignedVector::<u8>::from_str("(1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_assign_fail_1() {
    let mut v = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    v.mod_power_of_2_sub_assign(UnsignedVector::<u8>::from_str("(1, 8)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_sub_assign_fail_2() {
    let mut v = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    v.mod_power_of_2_sub_assign(&UnsignedVector::<u8>::from_str("(1)").unwrap(), 3);
}

fn mod_power_of_2_sub_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1::<T>().test_properties(
        |(v, w, pow)| {
            let r = (&v).mod_power_of_2_sub(&w, pow);
            assert_eq!((&v).mod_power_of_2_sub(w.clone(), pow), r);
            assert_eq!(v.clone().mod_power_of_2_sub(&w, pow), r);
            assert_eq!(v.clone().mod_power_of_2_sub(w.clone(), pow), r);
            let mut x = v.clone();
            x.mod_power_of_2_sub_assign(&w, pow);
            assert_eq!(x, r);
            let mut x = v.clone();
            x.mod_power_of_2_sub_assign(w.clone(), pow);
            assert_eq!(x, r);

            // The result is reduced, has the same dimension, and is the elementwise modular
            // difference.
            assert!(r.mod_power_of_2_is_reduced(pow));
            assert_eq!(r.dimension(), v.dimension());
            for ((&x, &y), &z) in v.elements.iter().zip(&w.elements).zip(&r.elements) {
                assert_eq!(z, x.mod_power_of_2_sub(y, pow));
            }
            // Subtracting is adding the negation, swapping the operands negates the difference, and
            // adding back the subtrahend recovers the minuend.
            assert_eq!(
                (&v).mod_power_of_2_add((&w).mod_power_of_2_neg(pow), pow),
                r
            );
            assert_eq!(
                (&w).mod_power_of_2_sub(&v, pow),
                (&r).mod_power_of_2_neg(pow)
            );
            assert_eq!((&r).mod_power_of_2_add(&w, pow), v);
        },
    );

    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_1::<T>()
        .test_properties(|(u, v, w, pow)| {
            assert_eq!(
                (&u).mod_power_of_2_sub(&v, pow).mod_power_of_2_sub(&w, pow),
                (&u).mod_power_of_2_sub((&v).mod_power_of_2_add(&w, pow), pow)
            );
        });

    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        let zero = UnsignedVector {
            elements: vec![T::ZERO; v.elements.len()],
        };
        assert_eq!((&v).mod_power_of_2_sub(&zero, pow), v);
        assert_eq!(
            (&zero).mod_power_of_2_sub(&v, pow),
            (&v).mod_power_of_2_neg(pow)
        );
        assert!(
            (&v).mod_power_of_2_sub(&v, pow)
                .elements
                .iter()
                .all(|&x| x == T::ZERO)
        );
    });
}

#[test]
fn mod_power_of_2_sub_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_properties_helper);
}
