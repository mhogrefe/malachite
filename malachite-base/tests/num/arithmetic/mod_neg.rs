// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_gen_var_1, unsigned_gen_var_6, unsigned_pair_gen_var_16,
};
use std::panic::catch_unwind;

fn mod_neg_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, m, out| {
        assert_eq!(n.mod_neg(m), out);

        let mut n = n;
        n.mod_neg_assign(m);
        assert_eq!(n, out);
    };

    test(T::ZERO, T::exact_from(5), T::ZERO);
    test(T::exact_from(7), T::exact_from(10), T::exact_from(3));
    test(T::exact_from(100), T::exact_from(101), T::ONE);
    test(T::MAX - T::ONE, T::MAX, T::ONE);
    test(T::ONE, T::MAX, T::MAX - T::ONE);
}

#[test]
fn test_mod_neg() {
    apply_fn_to_unsigneds!(mod_neg_helper);
}

fn mod_neg_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.mod_neg(T::ZERO));
    assert_panic!(T::from(123u8).mod_neg(T::from(123u8)));
}

#[test]
fn mod_neg_fail() {
    apply_fn_to_unsigneds!(mod_neg_fail_helper);
}

fn mod_neg_assign_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!({
        let mut x = T::ZERO;
        x.mod_neg_assign(T::ZERO);
    });
    assert_panic!({
        let mut x = T::from(123u8);
        x.mod_neg_assign(T::from(123u8));
    });
}

#[test]
fn mod_neg_assign_fail() {
    apply_fn_to_unsigneds!(mod_neg_assign_fail_helper);
}

fn mod_neg_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_16::<T>().test_properties(|(n, m)| {
        assert!(n.mod_is_reduced(&m));
        let neg = n.mod_neg(m);
        assert!(neg.mod_is_reduced(&m));

        let mut n_alt = n;
        n_alt.mod_neg_assign(m);
        assert_eq!(n_alt, neg);

        assert_eq!(neg.mod_neg(m), n);
        assert_eq!(n.mod_add(neg, m), T::ZERO);
        assert_eq!(n == neg, n == T::ZERO || m.even() && n == m >> 1);
    });

    unsigned_gen_var_1::<T>().test_properties(|m| {
        assert_eq!(T::ZERO.mod_neg(m), T::ZERO);
    });

    unsigned_gen_var_6::<T>().test_properties(|m| {
        assert_eq!(T::ONE.mod_neg(m), m - T::ONE);
        assert_eq!((m - T::ONE).mod_neg(m), T::ONE);
    });
}

#[test]
fn mod_neg_properties() {
    apply_fn_to_unsigneds!(mod_neg_properties_helper);
}
