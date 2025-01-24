// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::unsigned_pair_gen_var_12;
use std::panic::catch_unwind;

fn mod_is_reduced_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, m, out| {
        assert_eq!(n.mod_is_reduced(&m), out);
    };
    test(T::ZERO, T::exact_from(5), true);
    test(T::exact_from(100), T::exact_from(100), false);
    test(T::exact_from(100), T::exact_from(101), true);
    test(T::MAX - T::ONE, T::MAX - T::ONE, false);
    test(T::MAX - T::ONE, T::MAX, true);
    test(T::MAX, T::MAX, false);
}

#[test]
fn test_mod_is_reduced() {
    apply_fn_to_unsigneds!(mod_is_reduced_helper);
}

fn mod_is_reduced_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.mod_is_reduced(&T::ZERO));
}

#[test]
fn mod_is_reduced_fail() {
    apply_fn_to_unsigneds!(mod_is_reduced_fail_helper);
}

fn mod_is_reduced_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_12::<T, T>().test_properties(|(n, m)| {
        assert_eq!(n.mod_is_reduced(&m), n % m == n);
    });
}

#[test]
fn mod_is_reduced_properties() {
    apply_fn_to_unsigneds!(mod_is_reduced_properties_helper);
}
