// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Neg, ModPowerOf2NegAssign,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_4;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;

#[test]
fn test_mod_power_of_2_neg() {
    fn test<T: PrimitiveUnsigned>(s: &str, pow: u64, out: &str) {
        let v = UnsignedVector::<T>::from_str(s).unwrap();
        let w = (&v).mod_power_of_2_neg(pow);
        assert_eq!(w.to_string(), out);
        assert_eq!(v.clone().mod_power_of_2_neg(pow), w);
        let mut x = v;
        x.mod_power_of_2_neg_assign(pow);
        assert_eq!(x, w);
    }
    test::<u8>("()", 0, "()");
    test::<u8>("()", 3, "()");
    test::<u8>("(0)", 0, "(0)");
    test::<u8>("(1)", 1, "(1)");
    test::<u8>("(5, 1, 3)", 3, "(3, 7, 5)");
    // A zero element stays zero, wherever it is.
    test::<u8>("(0, 1)", 8, "(0, 255)");
    test::<u8>("(1, 0)", 8, "(255, 0)");
    test::<u8>("(4, 4)", 3, "(4, 4)");
    test::<u64>("(3, 5)", 64, "(18446744073709551613, 18446744073709551611)");
    test::<u128>("(1)", 100, "(1267650600228229401496703205375)");
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_fail_1() {
    // An element is not reduced.
    (&UnsignedVector::<u8>::from_str("(8, 1)").unwrap()).mod_power_of_2_neg(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_fail_2() {
    // pow is too large.
    (&UnsignedVector::<u8>::from_str("(1)").unwrap()).mod_power_of_2_neg(9);
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_assign_fail() {
    // An element is not reduced.
    let mut v = UnsignedVector::<u8>::from_str("(8, 1)").unwrap();
    v.mod_power_of_2_neg_assign(3);
}

#[test]
#[should_panic]
fn mod_power_of_2_neg_empty_vector_fail() {
    // pow is too large, even though there are no elements to check.
    UnsignedVector::<u8>::from_str("()")
        .unwrap()
        .mod_power_of_2_neg(9);
}

fn mod_power_of_2_neg_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        let w = (&v).mod_power_of_2_neg(pow);
        assert_eq!(v.clone().mod_power_of_2_neg(pow), w);
        let mut x = v.clone();
        x.mod_power_of_2_neg_assign(pow);
        assert_eq!(x, w);

        // The result is reduced, has the same dimension, and is the elementwise negation.
        assert!(w.mod_power_of_2_is_reduced(pow));
        assert_eq!(w.dimension(), v.dimension());
        for (&x, &y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(y, x.mod_power_of_2_neg(pow));
            assert_eq!(x.mod_power_of_2_add(y, pow), T::ZERO);
        }
        // Negating twice gives the vector back.
        assert_eq!((&w).mod_power_of_2_neg(pow), v);
    });
}

#[test]
fn mod_power_of_2_neg_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_neg_properties_helper);
}
