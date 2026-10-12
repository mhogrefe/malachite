// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModPowerOf2AbsSquared, ModPowerOf2Dot};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_4;
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_abs_squared() {
    fn test<T: PrimitiveUnsigned>(s: &str, pow: u64, out: T) {
        let v = UnsignedVector::<T>::from_str(s).unwrap();
        let n = (&v).mod_power_of_2_abs_squared(pow);
        assert_eq!(n, out);
        assert_eq!(v.mod_power_of_2_abs_squared(pow), n);
    }
    test::<u8>("()", 0, 0);
    test::<u8>("(0, 0)", 0, 0);
    test::<u8>("(1, 2, 3)", 3, 6);
    test::<u8>("(255, 255)", 8, 2);
    test::<u64>("(18446744073709551615)", 64, 1);
}

fn mod_power_of_2_abs_squared_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(
        UnsignedVector::<T>::from_str("()")
            .unwrap()
            .mod_power_of_2_abs_squared(T::WIDTH + 1)
    );
    assert_panic!(
        UnsignedVector::<T>::from_str("(1, 8)")
            .unwrap()
            .mod_power_of_2_abs_squared(3)
    );
}

#[test]
fn mod_power_of_2_abs_squared_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_abs_squared_fail_helper);
}

fn mod_power_of_2_abs_squared_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        let n = (&v).mod_power_of_2_abs_squared(pow);
        assert_eq!(v.clone().mod_power_of_2_abs_squared(pow), n);
        assert!(n.mod_power_of_2_is_reduced(pow));
        // It is the dot product of the vector with itself, and the squares added one at a time.
        assert_eq!((&v).mod_power_of_2_dot(&v, pow), n);
        assert_eq!(
            v.elements.iter().fold(T::ZERO, |s, &x| s
                .mod_power_of_2_add(x.mod_power_of_2_abs_squared(pow), pow)),
            n
        );
    });
}

#[test]
fn mod_power_of_2_abs_squared_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_abs_squared_properties_helper);
}
