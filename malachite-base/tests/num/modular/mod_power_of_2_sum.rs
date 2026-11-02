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

fn mod_power_of_2_sum_helper<T: PrimitiveUnsigned>() {
    let test = |xs: &[u8], pow, out: u8| {
        let xs: Vec<T> = xs.iter().map(|&x| T::from(x)).collect();
        let out = T::from(out);
        assert_eq!(T::mod_power_of_2_sum(xs.iter().copied(), pow), out);
        assert_eq!(T::mod_power_of_2_sum(xs.iter(), pow), out);
    };
    test(&[], 0, 0);
    test(&[], 5, 0);
    test(&[0], 0, 0);
    test(&[1], 1, 1);
    test(&[2, 3, 5, 7], 4, 1);
    test(&[2, 3, 5, 7], 5, 17);
    test(&[200, 100, 50], 8, 94);
    test(&[255, 255, 255], 8, 253);
    test(&[127, 1], 7, 0);
    let max = T::MAX;
    assert_eq!(
        T::mod_power_of_2_sum([max, max, T::ONE].into_iter(), T::WIDTH),
        max
    );
}

#[test]
fn test_mod_power_of_2_sum() {
    apply_fn_to_unsigneds!(mod_power_of_2_sum_helper);
}

fn mod_power_of_2_sum_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::mod_power_of_2_sum([T::ONE].into_iter(), 0));
    assert_panic!(T::mod_power_of_2_sum(
        [T::ZERO, T::exact_from(8u8)].into_iter(),
        3
    ));
    assert_panic!(T::mod_power_of_2_sum(
        core::iter::empty::<T>(),
        T::WIDTH + 1
    ));
}

#[test]
fn mod_power_of_2_sum_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_sum_fail_helper);
}

fn mod_power_of_2_sum_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_4::<T>().test_properties(|(v, pow)| {
        let xs = v.elements;
        let sum = T::mod_power_of_2_sum(xs.iter().copied(), pow);
        assert!(sum.mod_power_of_2_is_reduced(pow));
        assert_eq!(T::mod_power_of_2_sum(xs.iter(), pow), sum);
        // It is the modular sum taken one element at a time, in any order.
        assert_eq!(
            xs.iter()
                .fold(T::ZERO, |s, &x| s.mod_power_of_2_add(x, pow)),
            sum
        );
        assert_eq!(T::mod_power_of_2_sum(xs.iter().rev(), pow), sum);
    });

    unsigned_pair_gen_var_17::<T>().test_properties(|(x, pow)| {
        assert_eq!(T::mod_power_of_2_sum([x].into_iter(), pow), x);
        assert_eq!(T::mod_power_of_2_sum([x, T::ZERO].into_iter(), pow), x);
        assert_eq!(
            T::mod_power_of_2_sum([x, x.mod_power_of_2_neg(pow)].into_iter(), pow),
            T::ZERO
        );
    });
}

#[test]
fn mod_power_of_2_sum_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_sum_properties_helper);
}
