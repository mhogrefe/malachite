// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_16, unsigned_pair_gen_var_27, unsigned_quadruple_gen_var_4,
    unsigned_triple_gen_var_12,
};
use std::panic::catch_unwind;

fn mod_add_helper<T: PrimitiveUnsigned>() {
    let test = |x: T, y: T, m, out| {
        assert_eq!(x.mod_add(y, m), out);

        let mut x = x;
        x.mod_add_assign(y, m);
        assert_eq!(x, out);
    };
    test(T::ZERO, T::ZERO, T::ONE, T::ZERO);
    test(
        T::ZERO,
        T::exact_from(3),
        T::exact_from(5),
        T::exact_from(3),
    );
    test(
        T::exact_from(7),
        T::exact_from(5),
        T::exact_from(10),
        T::TWO,
    );
    test(
        T::exact_from(100),
        T::exact_from(100),
        T::exact_from(123),
        T::exact_from(77),
    );
    test(T::MAX - T::ONE, T::ONE, T::MAX, T::ZERO);
    test(T::MAX - T::ONE, T::MAX - T::ONE, T::MAX, T::MAX - T::TWO);
}

#[test]
fn test_mod_add() {
    apply_fn_to_unsigneds!(mod_add_helper);
}

fn mod_add_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.mod_add(T::ZERO, T::ZERO));
    assert_panic!(T::from(123u8).mod_add(T::from(200u8), T::from(200u8)));
    assert_panic!(T::from(200u8).mod_add(T::from(123u8), T::from(200u8)));
}

#[test]
fn mod_add_fail() {
    apply_fn_to_unsigneds!(mod_add_fail_helper);
}

fn mod_add_assign_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut x = T::ZERO;
        x.mod_add_assign(T::ZERO, T::ZERO);
    });
    assert_panic!({
        let mut x = T::from(123u8);
        x.mod_add_assign(T::from(200u8), T::from(200u8));
    });
    assert_panic!({
        let mut x = T::from(200u8);
        x.mod_add_assign(T::from(123u8), T::from(200u8));
    });
}

#[test]
fn mod_add_assign_fail() {
    apply_fn_to_unsigneds!(mod_add_assign_fail_helper);
}

fn mod_add_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_triple_gen_var_12::<T>().test_properties(|(x, y, m)| {
        assert!(x.mod_is_reduced(&m));
        assert!(y.mod_is_reduced(&m));
        let sum = x.mod_add(y, m);
        assert!(sum.mod_is_reduced(&m));

        let mut x_alt = x;
        x_alt.mod_add_assign(y, m);
        assert_eq!(x_alt, sum);

        assert_eq!(sum.mod_sub(y, m), x);
        assert_eq!(sum.mod_sub(x, m), y);
        assert_eq!(y.mod_add(x, m), sum);
        assert_eq!(x.mod_sub(y.mod_neg(m), m), sum);
    });

    unsigned_pair_gen_var_16::<T>().test_properties(|(x, m)| {
        assert_eq!(x.mod_add(T::ZERO, m), x);
        assert_eq!(T::ZERO.mod_add(x, m), x);
        assert_eq!(x.mod_add(x.mod_neg(m), m), T::ZERO);
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        assert_panic!(x.mod_add(y, T::ZERO));
        assert_panic!({
            let mut x = x;
            x.mod_add_assign(y, T::ZERO);
        });
    });

    unsigned_quadruple_gen_var_4::<T>().test_properties(|(x, y, z, m)| {
        assert_eq!(x.mod_add(y, m).mod_add(z, m), x.mod_add(y.mod_add(z, m), m));
    });
}

#[test]
fn mod_add_properties() {
    apply_fn_to_unsigneds!(mod_add_properties_helper);
}
