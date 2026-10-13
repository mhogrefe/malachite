// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModAdd, ModEntrywiseMul, ModEntrywiseMulAssign, ModIsReduced,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_2,
    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_2,
};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_mod_entrywise_mul() {
    let test = |s, t, m, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = UnsignedVector::<u8>::from_str(t).unwrap();
        let r = (&v).mod_entrywise_mul(&w, m);
        assert_eq!(r.to_string(), out);
        assert_eq!((&v).mod_entrywise_mul(w.clone(), m), r);
        assert_eq!(v.clone().mod_entrywise_mul(&w, m), r);
        assert_eq!(v.clone().mod_entrywise_mul(w.clone(), m), r);
        let mut x = v.clone();
        x.mod_entrywise_mul_assign(&w, m);
        assert_eq!(x, r);
        let mut x = v;
        x.mod_entrywise_mul_assign(w, m);
        assert_eq!(x, r);
    };
    test("()", "()", 1, "()");
    test("()", "()", 5, "()");
    test("(0)", "(0)", 1, "(0)");
    test("(5, 1, 3)", "(4, 6, 5)", 7, "(6, 6, 1)");
    test("(1, 2)", "(3, 4)", 7, "(3, 1)");
    test("(254, 200)", "(254, 100)", 255, "(1, 110)");
}

#[test]
fn test_mod_entrywise_mul_u64() {
    let v = UnsignedVector::<u64>::from_str("(18446744073709551614, 2)").unwrap();
    let w =
        UnsignedVector::<u64>::from_str("(18446744073709551614, 18446744073709551613)").unwrap();
    assert_eq!(
        v.mod_entrywise_mul(w, u64::MAX).to_string(),
        "(1, 18446744073709551611)"
    );
}

#[test]
#[should_panic]
fn mod_entrywise_mul_fail_1() {
    // An element of self is not reduced.
    (&UnsignedVector::<u8>::from_str("(7, 1)").unwrap())
        .mod_entrywise_mul(&UnsignedVector::<u8>::from_str("(1, 1)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_entrywise_mul_fail_2() {
    // An element of other is not reduced.
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_entrywise_mul(&UnsignedVector::<u8>::from_str("(1, 7)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_entrywise_mul_fail_3() {
    // The dimensions differ.
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_entrywise_mul(&UnsignedVector::<u8>::from_str("(1)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_entrywise_mul_fail_4() {
    // The modulus is zero.
    (&UnsignedVector::<u8>::from_str("()").unwrap())
        .mod_entrywise_mul(&UnsignedVector::<u8>::from_str("()").unwrap(), 0);
}

#[test]
#[should_panic]
fn mod_entrywise_mul_fail_5() {
    UnsignedVector::<u8>::from_str("(7, 1)")
        .unwrap()
        .mod_entrywise_mul(UnsignedVector::<u8>::from_str("(1, 1)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_entrywise_mul_fail_6() {
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_entrywise_mul(UnsignedVector::<u8>::from_str("(1, 7)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_entrywise_mul_assign_fail_1() {
    let mut v = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    v.mod_entrywise_mul_assign(UnsignedVector::<u8>::from_str("(1, 7)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_entrywise_mul_assign_fail_2() {
    let mut v = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    v.mod_entrywise_mul_assign(&UnsignedVector::<u8>::from_str("(1)").unwrap(), 7);
}

fn mod_entrywise_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_2::<T>().test_properties(
        |(v, w, m)| {
            let r = (&v).mod_entrywise_mul(&w, m);
            assert_eq!((&v).mod_entrywise_mul(w.clone(), m), r);
            assert_eq!(v.clone().mod_entrywise_mul(&w, m), r);
            assert_eq!(v.clone().mod_entrywise_mul(w.clone(), m), r);
            let mut x = v.clone();
            x.mod_entrywise_mul_assign(&w, m);
            assert_eq!(x, r);
            let mut x = v.clone();
            x.mod_entrywise_mul_assign(w.clone(), m);
            assert_eq!(x, r);

            // The result is reduced, has the same dimension, and is the elementwise modular
            // product.
            assert!(r.mod_is_reduced(&m));
            assert_eq!(r.dimension(), v.dimension());
            for ((&x, &y), &z) in v.elements.iter().zip(&w.elements).zip(&r.elements) {
                assert_eq!(z, x.mod_mul(y, m));
            }
            // It is commutative.
            assert_eq!((&w).mod_entrywise_mul(&v, m), r);
            // The vector of 1s is the identity, and the zero vector absorbs everything.
            let ones = UnsignedVector {
                elements: vec![T::ONE % m; v.elements.len()],
            };
            assert_eq!((&v).mod_entrywise_mul(&ones, m), v);
            let zero = UnsignedVector {
                elements: vec![T::ZERO; v.elements.len()],
            };
            assert_eq!((&v).mod_entrywise_mul(&zero, m), zero);
        },
    );

    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_2::<T>()
        .test_properties(|(u, v, w, m)| {
            // It is associative, and it distributes over modular addition.
            assert_eq!(
                (&u).mod_entrywise_mul(&v, m).mod_entrywise_mul(&w, m),
                (&u).mod_entrywise_mul((&v).mod_entrywise_mul(&w, m), m)
            );
            assert_eq!(
                (&u).mod_entrywise_mul((&v).mod_add(&w, m), m),
                (&u).mod_entrywise_mul(&v, m)
                    .mod_add((&u).mod_entrywise_mul(&w, m), m)
            );
        });
}

#[test]
fn mod_entrywise_mul_properties() {
    apply_fn_to_unsigneds!(mod_entrywise_mul_properties_helper);
}
