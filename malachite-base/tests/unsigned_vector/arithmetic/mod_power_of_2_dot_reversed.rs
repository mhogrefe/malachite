// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Dot, ModPowerOf2DotReversed};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use std::panic::catch_unwind;

fn reversed<T: PrimitiveUnsigned>(w: &UnsignedVector<T>) -> UnsignedVector<T> {
    UnsignedVector::from_owned_elements(w.elements.iter().rev().copied().collect())
}

#[test]
fn test_mod_power_of_2_dot_reversed() {
    fn test<T: PrimitiveUnsigned>(v: &str, w: &str, pow: u64, out: T) {
        let v = UnsignedVector::<T>::from_str(v).unwrap();
        let w = UnsignedVector::<T>::from_str(w).unwrap();
        let d = (&v).mod_power_of_2_dot_reversed(&w, pow);
        assert_eq!(d, out);
        assert_eq!((&v).mod_power_of_2_dot_reversed(w.clone(), pow), d);
        assert_eq!(v.clone().mod_power_of_2_dot_reversed(&w, pow), d);
        assert_eq!(v.mod_power_of_2_dot_reversed(w, pow), d);
    }
    test::<u8>("()", "()", 0, 0);
    test::<u8>("(1, 2, 3)", "(4, 5, 7)", 4, 13);
    test::<u8>("(255, 255)", "(255, 1)", 8, 0);
    test::<u64>(
        "(18446744073709551615, 2)",
        "(3, 18446744073709551615)",
        64,
        7,
    );
}

fn mod_power_of_2_dot_reversed_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T>::from_str("(1, 2)").unwrap();
    assert_panic!((&v).mod_power_of_2_dot_reversed(&v, T::WIDTH + 1));
    let w = UnsignedVector::<T>::from_str("(1)").unwrap();
    assert_panic!((&v).mod_power_of_2_dot_reversed(&w, 3));
    let w = UnsignedVector::<T>::from_str("(1, 8)").unwrap();
    assert_panic!((&v).mod_power_of_2_dot_reversed(&w, 3));
}

#[test]
fn mod_power_of_2_dot_reversed_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_dot_reversed_fail_helper);
}

fn mod_power_of_2_dot_reversed_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_unsigned_quadruple_gen_var_1::<T>().test_properties(
        |(v, w, _, pow)| {
            let d = (&v).mod_power_of_2_dot_reversed(&w, pow);
            assert!(d.mod_power_of_2_is_reduced(pow));
            assert_eq!((&v).mod_power_of_2_dot_reversed(w.clone(), pow), d);
            assert_eq!(v.clone().mod_power_of_2_dot_reversed(&w, pow), d);
            // It is the dot product with the second vector reversed, and it is symmetric.
            assert_eq!((&v).mod_power_of_2_dot(&reversed(&w), pow), d);
            assert_eq!((&w).mod_power_of_2_dot_reversed(&v, pow), d);
            assert_eq!(
                reversed(&v).mod_power_of_2_dot_reversed(&reversed(&w), pow),
                d
            );
        },
    );
}

#[test]
fn mod_power_of_2_dot_reversed_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_dot_reversed_properties_helper);
}
