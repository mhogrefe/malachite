// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{unsigned_pair_gen_var_16, unsigned_triple_gen_var_12};
use std::panic::catch_unwind;

fn mod_sub_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, m, out| {
        assert_eq!(x.mod_sub(y, m), out);

        let mut x = x;
        x.mod_sub_assign(y, m);
        assert_eq!(x, out);
    };
    test(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    test(T::exact_from(4), T::exact_from(3), T::exact_from(5), T::ONE);
    test(
        T::exact_from(7),
        T::exact_from(9),
        T::exact_from(10),
        T::exact_from(8),
    );
    test(
        T::exact_from(100),
        T::exact_from(120),
        T::exact_from(123),
        T::exact_from(103),
    );
    test(T::ZERO, T::ONE, T::MAX, T::MAX - T::ONE);
    test(T::MAX - T::TWO, T::MAX - T::ONE, T::MAX, T::MAX - T::ONE);
}

#[test]
fn test_mod_sub() {
    apply_fn_to_unsigneds!(mod_sub_helper);
}

fn mod_sub_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.mod_sub(T::ZERO, T::ZERO));
    assert_panic!(T::from(123u8).mod_sub(T::from(200u8), T::from(200u8)));
    assert_panic!(T::from(200u8).mod_sub(T::from(123u8), T::from(200u8)));
}

#[test]
fn mod_sub_fail() {
    apply_fn_to_unsigneds!(mod_sub_fail_helper);
}

fn mod_sub_assign_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut x = T::ZERO;
        x.mod_sub_assign(T::ZERO, T::ZERO);
    });
    assert_panic!({
        let mut x = T::from(123u8);
        x.mod_sub_assign(T::from(200u8), T::from(200u8));
    });
    assert_panic!({
        let mut x = T::from(200u8);
        x.mod_sub_assign(T::from(123u8), T::from(200u8));
    });
}

#[test]
fn mod_sub_assign_fail() {
    apply_fn_to_unsigneds!(mod_sub_assign_fail_helper);
}

fn mod_sub_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_12::<T>().test_properties(|(x, y, m)| {
        assert!(x.mod_is_reduced(&m));
        assert!(y.mod_is_reduced(&m));
        let diff = x.mod_sub(y, m);
        assert!(diff.mod_is_reduced(&m));

        let mut x_alt = x;
        x_alt.mod_sub_assign(y, m);
        assert_eq!(x_alt, diff);

        assert_eq!(diff.mod_add(y, m), x);
        assert_eq!(diff.mod_sub(x, m), y.mod_neg(m));
        assert_eq!(y.mod_sub(x, m), diff.mod_neg(m));
        assert_eq!(x.mod_add(y.mod_neg(m), m), diff);
    });

    unsigned_pair_gen_var_16::<T>().test_properties(|(x, m)| {
        assert_eq!(x.mod_sub(T::ZERO, m), x);
        assert_eq!(T::ZERO.mod_sub(x, m), x.mod_neg(m));
        assert_eq!(x.mod_sub(x, m), T::ZERO);
    });
}

#[test]
fn mod_sub_properties() {
    apply_fn_to_unsigneds!(mod_sub_properties_helper);
}
