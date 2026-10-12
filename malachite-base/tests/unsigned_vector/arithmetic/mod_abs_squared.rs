// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModAbsSquared, ModDot, ModPowerOf2AbsSquared};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_pair_gen_var_4, unsigned_vector_unsigned_pair_gen_var_5,
};
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_abs_squared() {
    fn test<T: PrimitiveUnsigned>(s: &str, m: T, out: T) {
        let v = UnsignedVector::<T>::from_str(s).unwrap();
        let n = (&v).mod_abs_squared(m);
        assert_eq!(n, out);
        assert_eq!(v.mod_abs_squared(m), n);
    }
    test::<u8>("()", 1, 0);
    test::<u8>("()", 7, 0);
    test::<u8>("(0, 0)", 1, 0);
    test::<u8>("(1, 2, 3)", 10, 4);
    test::<u8>("(254, 254)", 255, 2);
    test::<u64>(
        "(18446744073709551614, 18446744073709551614, 18446744073709551614)",
        18446744073709551615,
        3,
    );
}

fn mod_abs_squared_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(
        UnsignedVector::<T>::from_str("()")
            .unwrap()
            .mod_abs_squared(T::ZERO)
    );
    assert_panic!(
        UnsignedVector::<T>::from_str("(1, 7)")
            .unwrap()
            .mod_abs_squared(T::exact_from(7u8))
    );
}

#[test]
fn mod_abs_squared_fail() {
    apply_fn_to_unsigneds!(mod_abs_squared_fail_helper);
}

fn mod_abs_squared_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_5::<T>().test_properties(|(v, m)| {
        let n = (&v).mod_abs_squared(m);
        assert_eq!(v.clone().mod_abs_squared(m), n);
        assert!(n < m);
        // It is the dot product of the vector with itself, and the squares added one at a time.
        assert_eq!((&v).mod_dot(&v, m), n);
        assert_eq!(
            v.elements
                .iter()
                .fold(T::ZERO, |s, &x| s.mod_add(x.mod_abs_squared(m), m)),
            n
        );
    });

    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        // For a power-of-2 modulus, this is `mod_power_of_2_abs_squared`.
        if pow < T::WIDTH {
            assert_eq!(
                (&v).mod_abs_squared(T::power_of_2(pow)),
                v.mod_power_of_2_abs_squared(pow)
            );
        }
    });
}

#[test]
fn mod_abs_squared_properties() {
    apply_fn_to_unsigneds!(mod_abs_squared_properties_helper);
}
