// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2Cross, ModPowerOf2CrossAssign, ModPowerOf2Dot, ModPowerOf2IsReduced, ModPowerOf2Neg,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_vector::arithmetic::mod_power_of_2_cross::*;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

fn check<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    w: &UnsignedVector<T>,
    pow: u64,
) -> UnsignedVector<T> {
    let r = v.mod_power_of_2_cross(w, pow);
    assert!(r.mod_power_of_2_is_reduced(pow));
    assert_eq!(v.mod_power_of_2_cross(w.clone(), pow), r);
    assert_eq!(v.clone().mod_power_of_2_cross(w, pow), r);
    assert_eq!(v.clone().mod_power_of_2_cross(w.clone(), pow), r);
    let mut s = v.clone();
    s.mod_power_of_2_cross_assign(w, pow);
    assert_eq!(s, r);
    let mut s = v.clone();
    s.mod_power_of_2_cross_assign(w.clone(), pow);
    assert_eq!(s, r);
    r
}

fn test_mod_power_of_2_cross_helper<T: PrimitiveUnsigned>() {
    let test = |s, t, pow, out| {
        let v = UnsignedVector::<T>::from_str(s).unwrap();
        let w = UnsignedVector::<T>::from_str(t).unwrap();
        let r = check(&v, &w, pow);
        assert_eq!(r.to_string(), out);
        assert_eq!(unsigned_vector_mod_power_of_2_cross_naive(&v, &w, pow), r);
    };
    test("(0, 0, 0)", "(1, 2, 3)", 2, "(0, 0, 0)");
    test("(0, 0, 0)", "(0, 0, 0)", 0, "(0, 0, 0)");
    test("(1, 0, 0)", "(0, 1, 0)", 1, "(0, 0, 1)");
    test("(1, 0, 0)", "(0, 1, 0)", 8, "(0, 0, 1)");
    test("(0, 1, 0)", "(1, 0, 0)", 8, "(0, 0, 255)");
    test("(1, 2, 3)", "(4, 5, 6)", 4, "(13, 6, 13)");
    test("(1, 2, 3)", "(4, 5, 6)", 8, "(253, 6, 253)");
    test("(3, 5, 7)", "(7, 5, 3)", 3, "(4, 0, 4)");
    test("(1, 2, 3)", "(2, 4, 6)", 8, "(0, 0, 0)");
    test("(255, 255, 255)", "(1, 2, 3)", 8, "(255, 2, 255)");
    test("(200, 100, 50)", "(25, 75, 125)", 8, "(46, 58, 212)");
    test("(1, 1, 1)", "(1, 1, 1)", 1, "(0, 0, 0)");
}

#[test]
fn test_mod_power_of_2_cross() {
    apply_fn_to_unsigneds!(test_mod_power_of_2_cross_helper);
}

#[test]
#[should_panic]
fn mod_power_of_2_cross_fail_1() {
    // The vectors are not 3-dimensional.
    UnsignedVector::<u8>::from_str("(1, 2)")
        .unwrap()
        .mod_power_of_2_cross(UnsignedVector::from_str("(1, 2)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_cross_fail_2() {
    // An element is not reduced.
    UnsignedVector::<u8>::from_str("(1, 2, 8)")
        .unwrap()
        .mod_power_of_2_cross(UnsignedVector::from_str("(1, 2, 3)").unwrap(), 3);
}

#[test]
#[should_panic]
fn mod_power_of_2_cross_fail_3() {
    // The modulus is too large for the element type.
    UnsignedVector::<u8>::from_str("(1, 2, 3)")
        .unwrap()
        .mod_power_of_2_cross(UnsignedVector::from_str("(1, 2, 3)").unwrap(), 9);
}

#[test]
#[should_panic]
fn mod_power_of_2_cross_assign_fail() {
    let mut v = UnsignedVector::<u8>::from_str("(1, 2, 3, 4)").unwrap();
    v.mod_power_of_2_cross_assign(UnsignedVector::from_str("(1, 2, 3, 4)").unwrap(), 3);
}

fn mod_power_of_2_cross_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_3::<T>().test_properties(
        |(v, w, pow)| {
            let r = check(&v, &w, pow);
            assert_eq!(unsigned_vector_mod_power_of_2_cross_naive(&v, &w, pow), r);
            // Swapping the vectors negates the product.
            assert_eq!(
                (&w).mod_power_of_2_cross(&v, pow),
                (&r).mod_power_of_2_neg(pow)
            );
            // The product is perpendicular to both vectors.
            assert_eq!((&r).mod_power_of_2_dot(&v, pow), T::ZERO);
            assert_eq!((&r).mod_power_of_2_dot(&w, pow), T::ZERO);
            // Lagrange's identity: |v x w|^2 = |v|^2 |w|^2 - (v . w)^2, modulo 2^pow.
            let vw = (&v).mod_power_of_2_dot(&w, pow);
            assert_eq!(
                (&r).mod_power_of_2_dot(&r, pow),
                (&v).mod_power_of_2_dot(&v, pow)
                    .mod_power_of_2_mul((&w).mod_power_of_2_dot(&w, pow), pow)
                    .mod_power_of_2_sub(vw.mod_power_of_2_square(pow), pow)
            );
        },
    );

    unsigned_vector_unsigned_pair_gen_var_9::<T>().test_properties(|(v, pow)| {
        // A vector's product with itself, or with zero, is zero.
        let zero = UnsignedVector::<T>::zero(3);
        assert!((&v).mod_power_of_2_cross(&v, pow).is_zero());
        assert!((&v).mod_power_of_2_cross(&zero, pow).is_zero());
        assert!((&zero).mod_power_of_2_cross(&v, pow).is_zero());
    });
}

#[test]
fn mod_power_of_2_cross_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_cross_properties_helper);
}
