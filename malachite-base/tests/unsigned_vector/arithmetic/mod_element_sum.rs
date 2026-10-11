// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModElementSum, ModPowerOf2ElementSum};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_pair_gen_var_4, unsigned_vector_unsigned_pair_gen_var_5,
};
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_element_sum() {
    fn test<T: PrimitiveUnsigned>(s: &str, m: T, out: T) {
        let v = UnsignedVector::<T>::from_str(s).unwrap();
        let sum = v.to_mod_element_sum(m);
        assert_eq!(sum, out);
        assert_eq!(v.into_mod_element_sum(m), sum);
    }
    test::<u8>("()", 1, 0);
    test::<u8>("()", 7, 0);
    test::<u8>("(0, 0)", 1, 0);
    test::<u8>("(5, 6, 7)", 10, 8);
    test::<u8>("(254, 254)", 255, 253);
    test::<u64>(
        "(18446744073709551614, 18446744073709551614, 1)",
        18446744073709551615,
        18446744073709551614,
    );
}

fn mod_element_sum_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(
        UnsignedVector::<T>::from_str("()")
            .unwrap()
            .to_mod_element_sum(T::ZERO)
    );
    assert_panic!(
        UnsignedVector::<T>::from_str("(1, 7)")
            .unwrap()
            .to_mod_element_sum(T::exact_from(7u8))
    );
}

#[test]
fn mod_element_sum_fail() {
    apply_fn_to_unsigneds!(mod_element_sum_fail_helper);
}

fn mod_element_sum_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_5::<T>().test_properties(|(v, m)| {
        let sum = v.to_mod_element_sum(m);
        assert_eq!(v.clone().into_mod_element_sum(m), sum);
        assert!(sum.mod_is_reduced(&m));
        // It is the elements added one at a time with `mod_add`.
        assert_eq!(
            v.elements.iter().fold(T::ZERO, |acc, &x| acc.mod_add(x, m)),
            sum
        );
    });

    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        // For a power-of-2 modulus, this is `mod_power_of_2_element_sum`.
        if pow < T::WIDTH {
            assert_eq!(
                v.to_mod_element_sum(T::power_of_2(pow)),
                v.to_mod_power_of_2_element_sum(pow)
            );
        }
    });
}

#[test]
fn mod_element_sum_properties() {
    apply_fn_to_unsigneds!(mod_element_sum_properties_helper);
}
