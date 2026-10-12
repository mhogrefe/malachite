// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModDot;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_dot() {
    fn test<T: PrimitiveUnsigned>(a: &str, b: &str, m: T, out: T) {
        let v = UnsignedVector::<T>::from_str(a).unwrap();
        let w = UnsignedVector::<T>::from_str(b).unwrap();
        let d = (&v).mod_dot(&w, m);
        assert_eq!(d, out);
        assert_eq!((&v).mod_dot(w.clone(), m), d);
        assert_eq!(v.clone().mod_dot(&w, m), d);
        assert_eq!(v.mod_dot(w, m), d);
    }
    test::<u8>("()", "()", 1, 0);
    test::<u8>("()", "()", 7, 0);
    test::<u8>("(0, 0)", "(0, 0)", 1, 0);
    test::<u8>("(1, 2, 3)", "(4, 5, 7)", 10, 5);
    // - the sum fits in one word
    test::<u8>("(3, 4)", "(5, 6)", 7, 4);
    // - the sum needs two words
    test::<u8>("(254, 254)", "(254, 254)", 255, 2);
    // - the sum needs three words
    test::<u64>(
        "(18446744073709551614, 18446744073709551614, 18446744073709551614)",
        "(18446744073709551614, 18446744073709551614, 18446744073709551614)",
        18446744073709551615,
        3,
    );
}

fn mod_dot_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T>::from_str("(1, 2)").unwrap();
    assert_panic!((&v).mod_dot(&v, T::ZERO));
    let w = UnsignedVector::<T>::from_str("(1)").unwrap();
    assert_panic!((&v).mod_dot(&w, T::exact_from(7u8)));
    let w = UnsignedVector::<T>::from_str("(1, 7)").unwrap();
    assert_panic!((&v).mod_dot(&w, T::exact_from(7u8)));
}

#[test]
fn mod_dot_fail() {
    apply_fn_to_unsigneds!(mod_dot_fail_helper);
}

fn mod_dot_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_2::<T>().test_properties(
        |(v, w, m)| {
            let d = (&v).mod_dot(&w, m);
            assert!(d < m);
            assert_eq!((&v).mod_dot(w.clone(), m), d);
            assert_eq!(v.clone().mod_dot(&w, m), d);
            assert_eq!(v.clone().mod_dot(w.clone(), m), d);
            // It is symmetric, and it is the products added one at a time modulo `m`.
            assert_eq!((&w).mod_dot(&v, m), d);
            assert_eq!(
                v.elements
                    .iter()
                    .zip(&w.elements)
                    .fold(T::ZERO, |s, (&x, &y)| s.mod_add(x.mod_mul(y, m), m)),
                d
            );
        },
    );
}

#[test]
fn mod_dot_properties() {
    apply_fn_to_unsigneds!(mod_dot_properties_helper);
}
