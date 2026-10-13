// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModDot, ModDotReversed};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

fn reversed<T: PrimitiveUnsigned>(w: &UnsignedVector<T>) -> UnsignedVector<T> {
    UnsignedVector::from_owned_elements(w.elements.iter().rev().copied().collect())
}

#[test]
fn test_mod_dot_reversed() {
    fn test<T: PrimitiveUnsigned>(v: &str, w: &str, m: T, out: T) {
        let v = UnsignedVector::<T>::from_str(v).unwrap();
        let w = UnsignedVector::<T>::from_str(w).unwrap();
        let d = (&v).mod_dot_reversed(&w, m);
        assert_eq!(d, out);
        assert_eq!((&v).mod_dot_reversed(w.clone(), m), d);
        assert_eq!(v.clone().mod_dot_reversed(&w, m), d);
        assert_eq!(v.mod_dot_reversed(w, m), d);
    }
    test::<u8>("()", "()", 1, 0);
    test::<u8>("(1, 2, 3)", "(4, 5, 7)", 11, 7);
    test::<u8>("(254, 254)", "(254, 254)", 255, 2);
    test::<u64>(
        "(18446744073709551614, 18446744073709551614, 18446744073709551614)",
        "(18446744073709551614, 18446744073709551614, 18446744073709551613)",
        18446744073709551615,
        4,
    );
}

fn mod_dot_reversed_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T>::from_str("(1, 2)").unwrap();
    assert_panic!((&v).mod_dot_reversed(&v, T::ZERO));
    let w = UnsignedVector::<T>::from_str("(1)").unwrap();
    assert_panic!((&v).mod_dot_reversed(&w, T::exact_from(7u8)));
    let w = UnsignedVector::<T>::from_str("(1, 7)").unwrap();
    assert_panic!((&v).mod_dot_reversed(&w, T::exact_from(7u8)));
}

#[test]
fn mod_dot_reversed_fail() {
    apply_fn_to_unsigneds!(mod_dot_reversed_fail_helper);
}

fn mod_dot_reversed_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_2::<T>().test_properties(
        |(v, w, _, m)| {
            let d = (&v).mod_dot_reversed(&w, m);
            assert!(d < m);
            assert_eq!((&v).mod_dot_reversed(w.clone(), m), d);
            assert_eq!(v.clone().mod_dot_reversed(&w, m), d);
            // It is the dot product with the second vector reversed, and it is symmetric.
            assert_eq!((&v).mod_dot(&reversed(&w), m), d);
            assert_eq!((&w).mod_dot_reversed(&v, m), d);
            assert_eq!(reversed(&v).mod_dot_reversed(&reversed(&w), m), d);
        },
    );
}

#[test]
fn mod_dot_reversed_properties() {
    apply_fn_to_unsigneds!(mod_dot_reversed_properties_helper);
}
