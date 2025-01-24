// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{unsigned_pair_gen_var_17, unsigned_triple_gen_var_11};
use std::panic::catch_unwind;

fn mod_power_of_2_sub_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, pow, out| {
        assert_eq!(x.mod_power_of_2_sub(y, pow), out);

        let mut x = x;
        x.mod_power_of_2_sub_assign(y, pow);
        assert_eq!(x, out);
    };
    test(T::ZERO, T::ZERO, 0, T::ZERO);
    test(T::ZERO, T::ONE, 1, T::ONE);
    test(T::ONE, T::ONE, 1, T::ZERO);
    test(T::exact_from(5), T::TWO, 5, T::exact_from(3));
    test(T::exact_from(10), T::exact_from(14), 4, T::exact_from(12));
    test(
        T::exact_from(100),
        T::exact_from(200),
        8,
        T::exact_from(156),
    );
    test(T::ZERO, T::ONE, T::WIDTH, T::MAX);
    test(T::ONE, T::MAX, T::WIDTH, T::TWO);
}

#[test]
fn test_mod_power_of_2_sub() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_helper);
}

fn mod_power_of_2_sub_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.mod_power_of_2_sub(T::ZERO, 0));
    assert_panic!(T::ZERO.mod_power_of_2_sub(T::ONE, 0));
    assert_panic!(T::from(123u8).mod_power_of_2_sub(T::from(200u8), 7));
    assert_panic!(T::from(200u8).mod_power_of_2_sub(T::from(123u8), 7));
}

#[test]
fn mod_power_of_2_sub_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_fail_helper);
}

fn mod_power_of_2_sub_assign_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut x = T::ONE;
        x.mod_power_of_2_sub_assign(T::ZERO, 0);
    });
    assert_panic!({
        let mut x = T::ZERO;
        x.mod_power_of_2_sub_assign(T::ONE, 0);
    });
    assert_panic!({
        let mut x = T::from(123u8);
        x.mod_power_of_2_sub_assign(T::from(200u8), 7);
    });
    assert_panic!({
        let mut x = T::from(200u8);
        x.mod_power_of_2_sub_assign(T::from(123u8), 7);
    });
}

#[test]
fn mod_power_of_2_sub_assign_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_assign_fail_helper);
}

fn mod_power_of_2_sub_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_11::<T>().test_properties(|(x, y, pow)| {
        assert!(x.mod_power_of_2_is_reduced(pow));
        assert!(y.mod_power_of_2_is_reduced(pow));
        let diff = x.mod_power_of_2_sub(y, pow);
        assert!(diff.mod_power_of_2_is_reduced(pow));

        let mut x_alt = x;
        x_alt.mod_power_of_2_sub_assign(y, pow);
        assert_eq!(x_alt, diff);

        assert_eq!(diff.mod_power_of_2_add(y, pow), x);
        assert_eq!(diff.mod_power_of_2_sub(x, pow), y.mod_power_of_2_neg(pow));
        assert_eq!(y.mod_power_of_2_sub(x, pow), diff.mod_power_of_2_neg(pow));
        assert_eq!(x.mod_power_of_2_add(y.mod_power_of_2_neg(pow), pow), diff);
    });

    unsigned_pair_gen_var_17::<T>().test_properties(|(x, pow)| {
        assert_eq!(x.mod_power_of_2_sub(T::ZERO, pow), x);
        assert_eq!(
            T::ZERO.mod_power_of_2_sub(x, pow),
            x.mod_power_of_2_neg(pow)
        );
        assert_eq!(x.mod_power_of_2_sub(x, pow), T::ZERO);
    });
}

#[test]
fn mod_power_of_2_sub_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_sub_properties_helper);
}
