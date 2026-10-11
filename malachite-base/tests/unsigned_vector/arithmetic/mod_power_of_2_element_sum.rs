// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModPowerOf2ElementSum;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_4;
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_element_sum() {
    fn test<T: PrimitiveUnsigned>(s: &str, pow: u64, out: T) {
        let v = UnsignedVector::<T>::from_str(s).unwrap();
        let sum = v.to_mod_power_of_2_element_sum(pow);
        assert_eq!(sum, out);
        assert_eq!(v.into_mod_power_of_2_element_sum(pow), sum);
    }
    test::<u8>("()", 0, 0);
    test::<u8>("()", 3, 0);
    test::<u8>("(0, 0)", 0, 0);
    test::<u8>("(5, 6, 7)", 3, 2);
    test::<u8>("(255, 1)", 8, 0);
    test::<u64>(
        "(18446744073709551615, 18446744073709551615)",
        64,
        18446744073709551614,
    );
}

fn mod_power_of_2_element_sum_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(
        UnsignedVector::<T>::from_str("()")
            .unwrap()
            .to_mod_power_of_2_element_sum(T::WIDTH + 1)
    );
    assert_panic!(
        UnsignedVector::<T>::from_str("(1, 8)")
            .unwrap()
            .to_mod_power_of_2_element_sum(3)
    );
}

#[test]
fn mod_power_of_2_element_sum_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_element_sum_fail_helper);
}

fn mod_power_of_2_element_sum_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        let sum = v.to_mod_power_of_2_element_sum(pow);
        assert_eq!(v.clone().into_mod_power_of_2_element_sum(pow), sum);
        assert!(sum.mod_power_of_2_is_reduced(pow));
        // It is the elements added one at a time with `mod_power_of_2_add`.
        assert_eq!(
            v.elements
                .iter()
                .fold(T::ZERO, |acc, &x| acc.mod_power_of_2_add(x, pow)),
            sum
        );
    });
}

#[test]
fn mod_power_of_2_element_sum_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_element_sum_properties_helper);
}
