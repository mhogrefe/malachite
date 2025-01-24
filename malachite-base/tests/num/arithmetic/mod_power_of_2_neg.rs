// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_pair_gen_var_17;
use std::panic::catch_unwind;

fn mod_power_of_2_neg_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.mod_power_of_2_neg(pow), out);

        let mut n = n;
        n.mod_power_of_2_neg_assign(pow);
        assert_eq!(n, out);
    };
    test(T::ZERO, 5, T::ZERO);
    test(T::exact_from(10), 4, T::exact_from(6));
    test(T::exact_from(100), 8, T::exact_from(156));
    test(T::ONE, T::WIDTH, T::MAX);
    test(T::MAX, T::WIDTH, T::ONE);
}

#[test]
fn test_mod_power_of_2_neg() {
    apply_fn_to_unsigneds!(mod_power_of_2_neg_helper);
}

fn mod_power_of_2_neg_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ONE.mod_power_of_2_neg(0));
    assert_panic!(T::from(200u8).mod_power_of_2_neg(7));
}

#[test]
fn mod_power_of_2_neg_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_neg_fail_helper);
}

fn mod_power_of_2_neg_assign_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut x = T::ONE;
        x.mod_power_of_2_neg_assign(0);
    });
    assert_panic!({
        let mut x = T::from(200u8);
        x.mod_power_of_2_neg_assign(7);
    });
}

#[test]
fn mod_power_of_2_neg_assign_fail() {
    apply_fn_to_unsigneds!(mod_power_of_2_neg_assign_fail_helper);
}

fn mod_power_of_2_neg_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_17::<T>().test_properties(|(n, pow)| {
        assert!(n.mod_power_of_2_is_reduced(pow));
        let neg = n.mod_power_of_2_neg(pow);
        assert!(neg.mod_power_of_2_is_reduced(pow));

        let mut n_alt = n;
        n_alt.mod_power_of_2_neg_assign(pow);
        assert_eq!(n_alt, neg);

        assert_eq!(neg, n.wrapping_neg().mod_power_of_2(pow));
        assert_eq!(neg.mod_power_of_2_neg(pow), n);
        assert_eq!(n.mod_power_of_2_add(neg, pow), T::ZERO);
        assert_eq!(n == neg, n == T::ZERO || n == T::power_of_2(pow - 1));
    });
}

#[test]
fn mod_power_of_2_neg_properties() {
    apply_fn_to_unsigneds!(mod_power_of_2_neg_properties_helper);
}
