// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ModPowerOf2ElementProduct;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_vector_unsigned_pair_gen_var_4;
use malachite_base::unsigned_vector::UnsignedVector;
use std::panic::catch_unwind;

#[test]
fn test_mod_power_of_2_element_product() {
    fn test<T: PrimitiveUnsigned>(s: &str, pow: u64, out: T) {
        let v = UnsignedVector::<T>::from_str(s).unwrap();
        let product = v.to_mod_power_of_2_element_product(pow);
        assert_eq!(product, out);
        assert_eq!(v.into_mod_power_of_2_element_product(pow), product);
    }
    test::<u8>("()", 0, 0);
    test::<u8>("()", 3, 1);
    test::<u8>("(0, 0)", 0, 0);
    test::<u8>("(3, 5, 7)", 4, 9);
    test::<u8>("(255, 255)", 8, 1);
    // The partial product becomes zero, and the unreduced element after it is not examined.
    test::<u8>("(2, 4, 9)", 3, 0);
    test::<u64>("(18446744073709551615, 18446744073709551614)", 64, 2);
}

fn mod_power_of_2_element_product_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(
        UnsignedVector::<T>::from_str("()")
            .unwrap()
            .to_mod_power_of_2_element_product(T::WIDTH + 1)
    );
    assert_panic!(
        UnsignedVector::<T>::from_str("(1, 8)")
            .unwrap()
            .to_mod_power_of_2_element_product(3)
    );
    // With pow = 0 the product starts at 0, but the first element is still checked.
    assert_panic!(
        UnsignedVector::<T>::from_str("(1)")
            .unwrap()
            .to_mod_power_of_2_element_product(0)
    );
}

#[test]
fn mod_power_of_2_element_product_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_element_product_fail_helper);
}

fn mod_power_of_2_element_product_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        let product = v.to_mod_power_of_2_element_product(pow);
        assert_eq!(v.clone().into_mod_power_of_2_element_product(pow), product);
        assert!(product.mod_power_of_2_is_reduced(pow));
        // It is the elements multiplied one at a time with `mod_power_of_2_mul`.
        assert_eq!(
            v.elements
                .iter()
                .fold(T::ONE.mod_power_of_2(pow), |acc, &x| {
                    acc.mod_power_of_2_mul(x, pow)
                }),
            product
        );
    });
}

#[test]
fn mod_power_of_2_element_product_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_element_product_properties_helper);
}
