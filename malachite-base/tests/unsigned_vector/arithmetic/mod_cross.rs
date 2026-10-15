// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModCross, ModCrossAssign, ModDot, ModIsReduced, ModNeg,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::test_util::unsigned_vector::arithmetic::mod_cross::*;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

fn check<T: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    w: &UnsignedVector<T>,
    m: T,
) -> UnsignedVector<T> {
    let r = v.mod_cross(w, m);
    assert!(r.mod_is_reduced(&m));
    assert_eq!(v.mod_cross(w.clone(), m), r);
    assert_eq!(v.clone().mod_cross(w, m), r);
    assert_eq!(v.clone().mod_cross(w.clone(), m), r);
    let mut s = v.clone();
    s.mod_cross_assign(w, m);
    assert_eq!(s, r);
    let mut s = v.clone();
    s.mod_cross_assign(w.clone(), m);
    assert_eq!(s, r);
    r
}

fn test_mod_cross_helper<T: PrimitiveUnsigned>() {
    let test = |s, t, m: u8, out| {
        let v = UnsignedVector::<T>::from_str(s).unwrap();
        let w = UnsignedVector::<T>::from_str(t).unwrap();
        let m = T::from(m);
        let r = check(&v, &w, m);
        assert_eq!(r.to_string(), out);
        assert_eq!(unsigned_vector_mod_cross_naive(&v, &w, m), r);
    };
    test("(0, 0, 0)", "(1, 2, 3)", 5, "(0, 0, 0)");
    test("(0, 0, 0)", "(0, 0, 0)", 1, "(0, 0, 0)");
    test("(1, 0, 0)", "(0, 1, 0)", 2, "(0, 0, 1)");
    test("(1, 0, 0)", "(0, 1, 0)", 7, "(0, 0, 1)");
    test("(0, 1, 0)", "(1, 0, 0)", 7, "(0, 0, 6)");
    test("(1, 2, 3)", "(4, 5, 6)", 7, "(4, 6, 4)");
    test("(1, 2, 3)", "(4, 5, 6)", 12, "(9, 6, 9)");
    test("(3, 5, 7)", "(7, 5, 3)", 10, "(0, 0, 0)");
    test("(1, 2, 3)", "(2, 4, 6)", 251, "(0, 0, 0)");
    test("(254, 254, 254)", "(1, 2, 3)", 255, "(254, 2, 254)");
    test("(200, 100, 50)", "(25, 75, 125)", 251, "(216, 95, 201)");
    test("(1, 1, 1)", "(1, 1, 1)", 2, "(0, 0, 0)");
    test("(5, 0, 3)", "(2, 4, 1)", 6, "(0, 1, 2)");
}

#[test]
fn test_mod_cross() {
    apply_fn_to_unsigneds!(test_mod_cross_helper);
}

#[test]
#[should_panic]
fn mod_cross_fail_1() {
    // The vectors are not 3-dimensional.
    UnsignedVector::<u8>::from_str("(1, 2)")
        .unwrap()
        .mod_cross(UnsignedVector::from_str("(1, 2)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_cross_fail_2() {
    // An element is not reduced.
    UnsignedVector::<u8>::from_str("(1, 2, 7)")
        .unwrap()
        .mod_cross(UnsignedVector::from_str("(1, 2, 3)").unwrap(), 7);
}

#[test]
#[should_panic]
fn mod_cross_fail_3() {
    // The modulus is 0.
    UnsignedVector::<u8>::from_str("(0, 0, 0)")
        .unwrap()
        .mod_cross(UnsignedVector::from_str("(0, 0, 0)").unwrap(), 0);
}

#[test]
#[should_panic]
fn mod_cross_assign_fail() {
    let mut v = UnsignedVector::<u8>::from_str("(1, 2, 3, 4)").unwrap();
    v.mod_cross_assign(UnsignedVector::from_str("(1, 2, 3, 4)").unwrap(), 7);
}

fn mod_cross_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_4::<T>().test_properties(
        |(v, w, m)| {
            let r = check(&v, &w, m);
            assert_eq!(unsigned_vector_mod_cross_naive(&v, &w, m), r);
            // Swapping the vectors negates the product.
            assert_eq!((&w).mod_cross(&v, m), (&r).mod_neg(m));
            // The product is perpendicular to both vectors.
            assert_eq!((&r).mod_dot(&v, m), T::ZERO);
            assert_eq!((&r).mod_dot(&w, m), T::ZERO);
            // Lagrange's identity: |v x w|^2 = |v|^2 |w|^2 - (v . w)^2, modulo m.
            let vw = (&v).mod_dot(&w, m);
            assert_eq!(
                (&r).mod_dot(&r, m),
                (&v).mod_dot(&v, m)
                    .mod_mul((&w).mod_dot(&w, m), m)
                    .mod_sub(vw.mod_square(m), m)
            );
        },
    );

    unsigned_vector_unsigned_pair_gen_var_10::<T>().test_properties(|(v, m)| {
        // A vector's product with itself, or with zero, is zero.
        let zero = UnsignedVector::<T>::zero(3);
        assert!((&v).mod_cross(&v, m).is_zero());
        assert!((&v).mod_cross(&zero, m).is_zero());
        assert!((&zero).mod_cross(&v, m).is_zero());
    });
}

#[test]
fn mod_cross_properties() {
    apply_fn_to_unsigneds!(mod_cross_properties_helper);
}
