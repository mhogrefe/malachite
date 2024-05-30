// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_17, unsigned_quadruple_gen_var_3, unsigned_triple_gen_var_11,
};
use std::panic::catch_unwind;

fn mod_power_of_2_add_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, pow, out| {
        assert_eq!(x.mod_power_of_2_add(y, pow), out);

        let mut x = x;
        x.mod_power_of_2_add_assign(y, pow);
        assert_eq!(x, out);
    };
    test(T::ZERO, T::ZERO, 0, T::ZERO);
    test(T::ZERO, T::ONE, 1, T::ONE);
    test(T::ONE, T::ONE, 1, T::ZERO);
    test(T::ZERO, T::TWO, 5, T::TWO);
    test(T::exact_from(10), T::exact_from(14), 4, T::exact_from(8));
    test(T::exact_from(100), T::exact_from(200), 8, T::exact_from(44));
    test(T::MAX, T::ONE, T::WIDTH, T::ZERO);
    test(T::MAX, T::MAX, T::WIDTH, T::MAX - T::ONE);
}

#[test]
fn test_mod_power_of_2_add() {
    apply_fn_to_unsigneds!(mod_power_of_2_add_helper);
}

fn mod_power_of_2_add_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.mod_power_of_2_add(T::ZERO, 0));
    assert_panic!(T::ZERO.mod_power_of_2_add(T::ONE, 0));
    assert_panic!(T::from(123u8).mod_power_of_2_add(T::from(200u8), 7));
    assert_panic!(T::from(200u8).mod_power_of_2_add(T::from(123u8), 7));
}

#[test]
fn mod_power_of_2_add_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_add_fail_helper);
}

fn mod_power_of_2_add_assign_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut x = T::ONE;
        x.mod_power_of_2_add_assign(T::ZERO, 0);
    });
    assert_panic!({
        let mut x = T::ZERO;
        x.mod_power_of_2_add_assign(T::ONE, 0);
    });
    assert_panic!({
        let mut x = T::from(123u8);
        x.mod_power_of_2_add_assign(T::from(200u8), 7);
    });
    assert_panic!({
        let mut x = T::from(200u8);
        x.mod_power_of_2_add_assign(T::from(123u8), 7);
    });
}

#[test]
fn mod_power_of_2_add_assign_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_add_assign_fail_helper);
}

fn mod_power_of_2_add_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_11::<T>().test_properties(|(x, y, pow)| {
        assert!(x.mod_power_of_2_is_reduced(pow));
        assert!(y.mod_power_of_2_is_reduced(pow));
        let sum = x.mod_power_of_2_add(y, pow);
        assert!(sum.mod_power_of_2_is_reduced(pow));

        let mut x_alt = x;
        x_alt.mod_power_of_2_add_assign(y, pow);
        assert_eq!(x_alt, sum);

        assert_eq!(sum.mod_power_of_2_sub(y, pow), x);
        assert_eq!(sum.mod_power_of_2_sub(x, pow), y);
        assert_eq!(y.mod_power_of_2_add(x, pow), sum);
        assert_eq!(x.mod_power_of_2_sub(y.mod_power_of_2_neg(pow), pow), sum);
    });

    unsigned_pair_gen_var_17::<T>().test_properties(|(x, pow)| {
        assert_eq!(x.mod_power_of_2_add(T::ZERO, pow), x);
        assert_eq!(T::ZERO.mod_power_of_2_add(x, pow), x);
        assert_eq!(
            x.mod_power_of_2_add(x.mod_power_of_2_neg(pow), pow),
            T::ZERO
        );
    });

    unsigned_quadruple_gen_var_3::<T>().test_properties(|(x, y, z, pow)| {
        assert_eq!(
            x.mod_power_of_2_add(y, pow).mod_power_of_2_add(z, pow),
            x.mod_power_of_2_add(y.mod_power_of_2_add(z, pow), pow)
        );
    });
}

#[test]
fn mod_power_of_2_add_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_add_properties_helper);
}
