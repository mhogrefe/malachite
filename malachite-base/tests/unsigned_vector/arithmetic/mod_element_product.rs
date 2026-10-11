// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ModElementProduct, ModPowerOf2ElementProduct};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_vector_unsigned_pair_gen_var_4, unsigned_vector_unsigned_pair_gen_var_5,
};
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_element_product() {
    fn test<T: PrimitiveUnsigned>(s: &str, m: T, out: T) {
        let v = UnsignedVector::<T>::from_str(s).unwrap();
        let product = v.to_mod_element_product(m);
        assert_eq!(product, out);
        assert_eq!(v.into_mod_element_product(m), product);
    }
    test::<u8>("()", 1, 0);
    test::<u8>("()", 7, 1);
    test::<u8>("(0, 0)", 1, 0);
    test::<u8>("(3, 5, 7)", 10, 5);
    test::<u8>("(254, 254)", 255, 1);
    // The partial product becomes zero, and the unreduced element after it is not examined.
    test::<u8>("(2, 5, 11)", 10, 0);
    test::<u64>(
        "(18446744073709551614, 18446744073709551613)",
        18446744073709551615,
        2,
    );
}

fn mod_element_product_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(
        UnsignedVector::<T>::from_str("()")
            .unwrap()
            .to_mod_element_product(T::ZERO)
    );
    assert_panic!(
        UnsignedVector::<T>::from_str("(1, 7)")
            .unwrap()
            .to_mod_element_product(T::exact_from(7u8))
    );
    // With m = 1 the product starts at 0, but the first element is still checked.
    assert_panic!(
        UnsignedVector::<T>::from_str("(1)")
            .unwrap()
            .to_mod_element_product(T::ONE)
    );
}

#[test]
fn mod_element_product_fail() {
    apply_fn_to_unsigneds!(mod_element_product_fail_helper);
}

fn mod_element_product_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_5::<T>().test_properties(|(v, m)| {
        let product = v.to_mod_element_product(m);
        assert_eq!(v.clone().into_mod_element_product(m), product);
        assert!(product.mod_is_reduced(&m));
        // It is the elements multiplied one at a time with `mod_mul`.
        assert_eq!(
            v.elements
                .iter()
                .fold(T::ONE.mod_op(m), |acc, &x| acc.mod_mul(x, m)),
            product
        );
    });

    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        // For a power-of-2 modulus, this is `mod_power_of_2_element_product`.
        if pow < T::WIDTH {
            assert_eq!(
                v.to_mod_element_product(T::power_of_2(pow)),
                v.to_mod_power_of_2_element_product(pow)
            );
        }
    });
}

#[test]
fn mod_element_product_properties() {
    apply_fn_to_unsigneds!(mod_element_product_properties_helper);
}
