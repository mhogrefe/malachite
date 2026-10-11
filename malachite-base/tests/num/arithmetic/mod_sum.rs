// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_16, unsigned_vector_unsigned_pair_gen_var_5,
};
use std::panic::catch_unwind;

fn mod_sum_helper<T: PrimitiveUnsigned>() {
    let test = |xs: &[u8], m: u8, out: u8| {
        let xs: Vec<T> = xs.iter().map(|&x| T::from(x)).collect();
        let m = T::from(m);
        let out = T::from(out);
        assert_eq!(T::mod_sum(xs.iter().copied(), m), out);
        assert_eq!(T::mod_sum(xs.iter(), m), out);
    };
    test(&[], 1, 0);
    test(&[], 7, 0);
    test(&[0], 1, 0);
    test(&[6], 7, 6);
    test(&[2, 3, 5, 7], 10, 7);
    test(&[200, 100, 50], 255, 95);
    test(&[254, 254, 254], 255, 252);
    test(&[6, 1], 7, 0);
    let m = T::MAX;
    let x = m - T::ONE;
    assert_eq!(T::mod_sum([x, x, T::ONE].into_iter(), m), x);
}

#[test]
fn test_mod_sum() {
    apply_fn_to_unsigneds!(mod_sum_helper);
}

fn mod_sum_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::mod_sum(core::iter::empty::<T>(), T::ZERO));
    assert_panic!(T::mod_sum([T::ONE].into_iter(), T::ONE));
    assert_panic!(T::mod_sum(
        [T::ZERO, T::exact_from(7u8)].into_iter(),
        T::exact_from(7u8)
    ));
}

#[test]
fn mod_sum_fail() {
    apply_fn_to_unsigneds!(mod_sum_fail_helper);
}

fn mod_sum_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_vector_unsigned_pair_gen_var_5::<T>().test_properties(|(v, m)| {
        let xs = v.elements;
        let sum = T::mod_sum(xs.iter().copied(), m);
        assert!(sum.mod_is_reduced(&m));
        assert_eq!(T::mod_sum(xs.iter(), m), sum);
        // It is the modular sum taken one element at a time, in any order.
        assert_eq!(xs.iter().fold(T::ZERO, |s, &x| s.mod_add(x, m)), sum);
        assert_eq!(T::mod_sum(xs.iter().rev(), m), sum);
    });

    unsigned_pair_gen_var_16::<T>().test_properties(|(x, m)| {
        assert_eq!(T::mod_sum([x].into_iter(), m), x);
        assert_eq!(T::mod_sum([x, T::ZERO].into_iter(), m), x);
        assert_eq!(T::mod_sum([x, x.mod_neg(m)].into_iter(), m), T::ZERO);
    });
}

#[test]
fn mod_sum_properties() {
    apply_fn_to_unsigneds!(mod_sum_properties_helper);
}
