// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModPowerOf2Dot;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::*;
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_dot() {
    fn test<T: PrimitiveUnsigned>(a: &str, b: &str, pow: u64, out: T) {
        let v = UnsignedVector::<T>::from_str(a).unwrap();
        let w = UnsignedVector::<T>::from_str(b).unwrap();
        let d = (&v).mod_power_of_2_dot(&w, pow);
        assert_eq!(d, out);
        assert_eq!((&v).mod_power_of_2_dot(w.clone(), pow), d);
        assert_eq!(v.clone().mod_power_of_2_dot(&w, pow), d);
        assert_eq!(v.mod_power_of_2_dot(w, pow), d);
    }
    test::<u8>("()", "()", 0, 0);
    test::<u8>("()", "()", 3, 0);
    test::<u8>("(0, 0)", "(0, 0)", 0, 0);
    test::<u8>("(1, 2, 3)", "(4, 5, 7)", 4, 3);
    test::<u8>("(255, 255)", "(255, 255)", 8, 2);
    test::<u64>(
        "(18446744073709551615, 18446744073709551615)",
        "(18446744073709551615, 1)",
        64,
        0,
    );
}

fn mod_power_of_2_dot_fail_helper<T: PrimitiveUnsigned>() {
    let v = UnsignedVector::<T>::from_str("(1, 2)").unwrap();
    assert_panic!((&v).mod_power_of_2_dot(&v, T::WIDTH + 1));
    let w = UnsignedVector::<T>::from_str("(1)").unwrap();
    assert_panic!((&v).mod_power_of_2_dot(&w, 3));
    let w = UnsignedVector::<T>::from_str("(1, 8)").unwrap();
    assert_panic!((&v).mod_power_of_2_dot(&w, 3));
}

#[test]
fn mod_power_of_2_dot_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_dot_fail_helper);
}

fn mod_power_of_2_dot_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_vector_unsigned_triple_gen_var_1::<T>().test_properties(
        |(v, w, pow)| {
            let d = (&v).mod_power_of_2_dot(&w, pow);
            assert!(d.mod_power_of_2_is_reduced(pow));
            assert_eq!((&v).mod_power_of_2_dot(w.clone(), pow), d);
            assert_eq!(v.clone().mod_power_of_2_dot(&w, pow), d);
            assert_eq!(v.clone().mod_power_of_2_dot(w.clone(), pow), d);
            // It is symmetric, and it is the products added one at a time modulo $2^k$.
            assert_eq!((&w).mod_power_of_2_dot(&v, pow), d);
            assert_eq!(
                v.elements
                    .iter()
                    .zip(&w.elements)
                    .fold(T::ZERO, |s, (&x, &y)| {
                        s.mod_power_of_2_add(x.mod_power_of_2_mul(y, pow), pow)
                    }),
                d
            );
        },
    );
}

#[test]
fn mod_power_of_2_dot_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_dot_properties_helper);
}
