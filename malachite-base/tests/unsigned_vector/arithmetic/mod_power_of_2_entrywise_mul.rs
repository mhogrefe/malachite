// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModEntrywiseMul, ModPowerOf2Add, ModPowerOf2EntrywiseMul, ModPowerOf2EntrywiseMulAssign,
    ModPowerOf2IsReduced,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1,
    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_1,
};
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_mod_power_of_2_entrywise_mul() {
    let test = |s, t, pow, out| {
        let v = UnsignedVector::<u8>::from_str(s).unwrap();
        let w = UnsignedVector::<u8>::from_str(t).unwrap();
        let r = (&v).mod_power_of_2_entrywise_mul(&w, pow);
        assert_eq!(r.to_string(), out);
        assert_eq!((&v).mod_power_of_2_entrywise_mul(w.clone(), pow), r);
        assert_eq!(v.clone().mod_power_of_2_entrywise_mul(&w, pow), r);
        assert_eq!(v.clone().mod_power_of_2_entrywise_mul(w.clone(), pow), r);
        let mut x = v.clone();
        x.mod_power_of_2_entrywise_mul_assign(&w, pow);
        assert_eq!(x, r);
        let mut x = v;
        x.mod_power_of_2_entrywise_mul_assign(w, pow);
        assert_eq!(x, r);
    };
    test("()", "()", 0, "()");
    test("()", "()", 5, "()");
    test("(0)", "(0)", 0, "(0)");
    test("(5, 1, 3)", "(4, 7, 6)", 3, "(4, 7, 2)");
    test("(1, 2)", "(3, 4)", 3, "(3, 0)");
    test("(255, 200)", "(255, 100)", 8, "(1, 32)");
}

#[test]
fn test_mod_power_of_2_entrywise_mul_u64() {
    let v = UnsignedVector::<u64>::from_str("(18446744073709551615, 3)").unwrap();
    let w =
        UnsignedVector::<u64>::from_str("(18446744073709551615, 18446744073709551615)").unwrap();
    assert_eq!(
        v.mod_power_of_2_entrywise_mul(w, 64).to_string(),
        "(1, 18446744073709551613)"
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_mul_fail_1() {
    // pow is greater than the width of the type.
    (&UnsignedVector::<u8>::from_str("()").unwrap())
        .mod_power_of_2_entrywise_mul(&UnsignedVector::<u8>::from_str("()").unwrap(), 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_mul_fail_2() {
    // An element of self is not reduced.
    (&UnsignedVector::<u8>::from_str("(8, 1)").unwrap())
        .mod_power_of_2_entrywise_mul(&UnsignedVector::<u8>::from_str("(1, 1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_mul_fail_3() {
    // An element of other is not reduced.
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_power_of_2_entrywise_mul(&UnsignedVector::<u8>::from_str("(1, 8)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_mul_fail_4() {
    // The dimensions differ.
    (&UnsignedVector::<u8>::from_str("(1, 1)").unwrap())
        .mod_power_of_2_entrywise_mul(&UnsignedVector::<u8>::from_str("(1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_mul_fail_5() {
    UnsignedVector::<u8>::from_str("(8, 1)")
        .unwrap()
        .mod_power_of_2_entrywise_mul(UnsignedVector::<u8>::from_str("(1, 1)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_entrywise_mul_assign_fail() {
    let mut v = UnsignedVector::<u8>::from_str("(1, 1)").unwrap();
    v.mod_power_of_2_entrywise_mul_assign(UnsignedVector::<u8>::from_str("(1, 8)").unwrap(), 3);
}

fn mod_power_of_2_entrywise_mul_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1::<T>().test_properties(
        |(v, w, pow)| {
            let r = (&v).mod_power_of_2_entrywise_mul(&w, pow);
            assert_eq!((&v).mod_power_of_2_entrywise_mul(w.clone(), pow), r);
            assert_eq!(v.clone().mod_power_of_2_entrywise_mul(&w, pow), r);
            assert_eq!(v.clone().mod_power_of_2_entrywise_mul(w.clone(), pow), r);
            let mut x = v.clone();
            x.mod_power_of_2_entrywise_mul_assign(&w, pow);
            assert_eq!(x, r);
            let mut x = v.clone();
            x.mod_power_of_2_entrywise_mul_assign(w.clone(), pow);
            assert_eq!(x, r);

            // The result is reduced, has the same dimension, and is the elementwise modular
            // product.
            assert!(r.mod_power_of_2_is_reduced(pow));
            assert_eq!(r.dimension(), v.dimension());
            for ((&x, &y), &z) in v.elements.iter().zip(&w.elements).zip(&r.elements) {
                assert_eq!(z, x.mod_power_of_2_mul(y, pow));
            }
            // It is commutative.
            assert_eq!((&w).mod_power_of_2_entrywise_mul(&v, pow), r);
            // The vector of 1s is the identity, and the zero vector absorbs everything.
            let ones = UnsignedVector {
                elements: vec![T::ONE.mod_power_of_2(pow); v.elements.len()],
            };
            assert_eq!((&v).mod_power_of_2_entrywise_mul(&ones, pow), v);
            let zero = UnsignedVector {
                elements: vec![T::ZERO; v.elements.len()],
            };
            assert_eq!((&v).mod_power_of_2_entrywise_mul(&zero, pow), zero);
            // For a power of 2 below the width of `T`, this is `mod_entrywise_mul`.
            if pow < T::WIDTH {
                assert_eq!((&v).mod_entrywise_mul(&w, T::power_of_2(pow)), r);
            }
        },
    );

    unsigned_vector_unsigned_vector_unsigned_vector_unsigned_quadruple_gen_var_1::<T>()
        .test_properties(|(u, v, w, pow)| {
            // It is associative, and it distributes over modular addition.
            assert_eq!(
                (&u).mod_power_of_2_entrywise_mul(&v, pow)
                    .mod_power_of_2_entrywise_mul(&w, pow),
                (&u).mod_power_of_2_entrywise_mul((&v).mod_power_of_2_entrywise_mul(&w, pow), pow)
            );
            assert_eq!(
                (&u).mod_power_of_2_entrywise_mul((&v).mod_power_of_2_add(&w, pow), pow),
                (&u).mod_power_of_2_entrywise_mul(&v, pow)
                    .mod_power_of_2_add((&u).mod_power_of_2_entrywise_mul(&w, pow), pow)
            );
        });
}

#[test]
fn mod_power_of_2_entrywise_mul_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_entrywise_mul_properties_helper);
}
