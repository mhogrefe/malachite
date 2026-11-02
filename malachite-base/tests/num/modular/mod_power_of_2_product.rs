// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_17, unsigned_vector_unsigned_pair_gen_var_4,
};
use std::panic::catch_unwind;

fn mod_power_of_2_product_helper<T: PrimitiveUnsigned>() {
    let test = |xs: &[u8], pow, out: u8| {
        let xs: Vec<T> = xs.iter().map(|&x| T::from(x)).collect();
        let out = T::from(out);
        assert_eq!(T::mod_power_of_2_product(xs.iter().copied(), pow), out);
        assert_eq!(T::mod_power_of_2_product(xs.iter(), pow), out);
    };
    test(&[], 0, 0);
    test(&[], 5, 1);
    test(&[0], 0, 0);
    test(&[1], 1, 1);
    test(&[2, 3, 5, 7], 4, 2);
    test(&[2, 3, 5, 7], 8, 210);
    test(&[200, 100, 50], 8, 64);
    test(&[255, 255, 255], 8, 255);
    test(&[3, 5, 0], 3, 0);
    let max = T::MAX;
    assert_eq!(
        T::mod_power_of_2_product([max, max, max].into_iter(), T::WIDTH),
        max
    );
    // Once the partial product is zero, the remaining factors are not examined, so these unreduced
    // factors do not panic.
    assert_eq!(
        T::mod_power_of_2_product([T::ZERO, T::MAX].into_iter(), 3),
        T::ZERO
    );
    assert_eq!(
        T::mod_power_of_2_product([T::TWO, T::exact_from(4u8), T::MAX].into_iter(), 3),
        T::ZERO
    );
}

#[test]
fn test_mod_power_of_2_product() {
    apply_fn_to_unsigneds!(mod_power_of_2_product_helper);
}

fn mod_power_of_2_product_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::mod_power_of_2_product([T::ONE].into_iter(), 0));
    assert_panic!(T::mod_power_of_2_product(
        [T::ONE, T::exact_from(8u8)].into_iter(),
        3
    ));
    assert_panic!(T::mod_power_of_2_product(
        core::iter::empty::<T>(),
        T::WIDTH + 1
    ));
}

#[test]
fn mod_power_of_2_product_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_product_fail_helper);
}

fn mod_power_of_2_product_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        let xs = v.elements;
        let product = T::mod_power_of_2_product(xs.iter().copied(), pow);
        assert!(product.mod_power_of_2_is_reduced(pow));
        assert_eq!(T::mod_power_of_2_product(xs.iter(), pow), product);
        // It is the modular product taken one element at a time, in any order.
        assert_eq!(
            xs.iter().fold(T::ONE.mod_power_of_2(pow), |p, &x| p
                .mod_power_of_2_mul(x, pow)),
            product
        );
        assert_eq!(T::mod_power_of_2_product(xs.iter().rev(), pow), product);
    });

    unsigned_pair_gen_var_17::<T>().test_properties(|(x, pow)| {
        assert_eq!(T::mod_power_of_2_product([x].into_iter(), pow), x);
        assert_eq!(
            T::mod_power_of_2_product([x, x].into_iter(), pow),
            x.mod_power_of_2_square(pow)
        );
        assert_eq!(
            T::mod_power_of_2_product([x, T::ZERO].into_iter(), pow),
            T::ZERO
        );
        if pow != 0 {
            assert_eq!(T::mod_power_of_2_product([x, T::ONE].into_iter(), pow), x);
        }
        if x.odd() {
            assert_eq!(
                T::mod_power_of_2_product(
                    [x, x.mod_power_of_2_inverse(pow).unwrap()].into_iter(),
                    pow
                ),
                T::ONE
            );
        }
    });
}

#[test]
fn mod_power_of_2_product_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_product_properties_helper);
}
