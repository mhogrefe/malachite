// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::log_base::{ceiling_log_base_naive, checked_log_base_naive};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    unsigned_gen_var_1, unsigned_gen_var_6, unsigned_pair_gen_var_24,
};
use std::panic::catch_unwind;

fn floor_log_base_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, base, out| {
        assert_eq!(n.floor_log_base(base), out);
    };
    test(T::ONE, T::TWO, 0);
    test(T::ONE, T::exact_from(5), 0);
    test(T::TWO, T::TWO, 1);
    test(T::TWO, T::exact_from(3), 0);
    test(T::exact_from(3), T::TWO, 1);
    test(T::exact_from(3), T::exact_from(3), 1);
    test(T::exact_from(3), T::exact_from(4), 0);
    test(T::exact_from(100), T::exact_from(2), 6);
    test(T::exact_from(100), T::exact_from(3), 4);
    test(T::exact_from(100), T::exact_from(4), 3);
    test(T::exact_from(100), T::exact_from(5), 2);
    test(T::exact_from(100), T::exact_from(10), 2);
    test(T::exact_from(100), T::exact_from(11), 1);
}

#[test]
fn test_floor_log_base() {
    apply_fn_to_unsigneds!(floor_log_base_helper);
}

fn floor_log_base_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.floor_log_base(T::TWO));
    assert_panic!(T::TWO.floor_log_base(T::ZERO));
    assert_panic!(T::TWO.floor_log_base(T::ONE));
}

#[test]
fn floor_log_base_fail() {
    apply_fn_to_unsigneds!(floor_log_base_fail_helper);
}

fn ceiling_log_base_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.ceiling_log_base(pow), out);
    };
    test(T::ONE, T::TWO, 0);
    test(T::ONE, T::exact_from(5), 0);
    test(T::TWO, T::TWO, 1);
    test(T::TWO, T::exact_from(3), 1);
    test(T::exact_from(3), T::TWO, 2);
    test(T::exact_from(3), T::exact_from(3), 1);
    test(T::exact_from(3), T::exact_from(4), 1);
    test(T::exact_from(100), T::exact_from(2), 7);
    test(T::exact_from(100), T::exact_from(3), 5);
    test(T::exact_from(100), T::exact_from(4), 4);
    test(T::exact_from(100), T::exact_from(5), 3);
    test(T::exact_from(100), T::exact_from(10), 2);
    test(T::exact_from(100), T::exact_from(11), 2);
}

#[test]
fn test_ceiling_log_base() {
    apply_fn_to_unsigneds!(ceiling_log_base_helper);
}

fn ceiling_log_base_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.ceiling_log_base(T::TWO));
    assert_panic!(T::TWO.ceiling_log_base(T::ZERO));
    assert_panic!(T::TWO.ceiling_log_base(T::ONE));
}

#[test]
fn ceiling_log_base_fail() {
    apply_fn_to_unsigneds!(ceiling_log_base_fail_helper);
}

fn checked_log_base_helper<T: PrimitiveUnsigned>() {
    let test = |n: T, pow, out| {
        assert_eq!(n.checked_log_base(pow), out);
    };
    test(T::ONE, T::TWO, Some(0));
    test(T::ONE, T::exact_from(5), Some(0));
    test(T::TWO, T::TWO, Some(1));
    test(T::TWO, T::exact_from(3), None);
    test(T::exact_from(3), T::TWO, None);
    test(T::exact_from(3), T::exact_from(3), Some(1));
    test(T::exact_from(3), T::exact_from(4), None);
    test(T::exact_from(100), T::exact_from(2), None);
    test(T::exact_from(100), T::exact_from(3), None);
    test(T::exact_from(100), T::exact_from(4), None);
    test(T::exact_from(100), T::exact_from(5), None);
    test(T::exact_from(100), T::exact_from(10), Some(2));
    test(T::exact_from(100), T::exact_from(11), None);
}

#[test]
fn test_checked_log_base() {
    apply_fn_to_unsigneds!(checked_log_base_helper);
}

fn checked_log_base_fail_helper<T: PrimitiveUnsigned>() {
    assert_panic!(T::ZERO.checked_log_base(T::TWO));
    assert_panic!(T::TWO.checked_log_base(T::ZERO));
    assert_panic!(T::TWO.checked_log_base(T::ONE));
}

#[test]
fn checked_log_base_fail() {
    apply_fn_to_unsigneds!(checked_log_base_fail_helper);
}

fn floor_log_base_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_24::<T, T>().test_properties(|(n, base)| {
        let floor_log = n.floor_log_base(base);
        assert!(floor_log < T::WIDTH);
        assert_eq!(floor_log == 0, n < base);

        if let Some(pow) = base.checked_pow(floor_log) {
            assert!(pow <= n);
        }
        if let Some(pow) = base.checked_pow(floor_log + 1) {
            assert!(pow > n);
        }
        let ceiling_log = n.ceiling_log_base(base);
        assert!(ceiling_log == floor_log || ceiling_log == floor_log + 1);
    });

    unsigned_gen_var_1::<T>().test_properties(|n| {
        assert_eq!(n.floor_log_base(T::TWO), n.floor_log_base_2());
    });

    unsigned_gen_var_6().test_properties(|n| {
        assert_eq!(T::ONE.floor_log_base(n), 0);
        assert_eq!(n.floor_log_base(n), 1);
    });
}

#[test]
fn floor_log_base_properties() {
    apply_fn_to_unsigneds!(floor_log_base_properties_helper);
}

fn ceiling_log_base_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_24::<T, T>().test_properties(|(n, base)| {
        let ceiling_log = n.ceiling_log_base(base);
        assert!(ceiling_log <= T::WIDTH);
        assert_eq!(ceiling_log, ceiling_log_base_naive(n, base));
        assert_eq!(ceiling_log == 0, n == T::ONE);

        if let Some(pow) = base.checked_pow(ceiling_log) {
            assert!(pow >= n);
        }
        if n != T::ONE {
            if let Some(pow) = base.checked_pow(ceiling_log - 1) {
                assert!(pow < n);
            }
        }

        let floor_log = n.floor_log_base(base);
        assert!(ceiling_log == floor_log || ceiling_log == floor_log + 1);
    });

    unsigned_gen_var_1::<T>().test_properties(|n| {
        assert_eq!(n.ceiling_log_base(T::TWO), n.ceiling_log_base_2());
    });

    unsigned_gen_var_6().test_properties(|n| {
        assert_eq!(T::ONE.ceiling_log_base(n), 0);
        assert_eq!(n.ceiling_log_base(n), 1);
    });
}

#[test]
fn ceiling_log_base_properties() {
    apply_fn_to_unsigneds!(ceiling_log_base_properties_helper);
}

fn checked_log_base_properties_helper<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_24::<T, T>().test_properties(|(n, base)| {
        let checked_log = n.checked_log_base(base);
        assert_eq!(checked_log, checked_log_base_naive(n, base));
        if let Some(log) = checked_log {
            assert_eq!(base.pow(log), n);
            assert!(log <= T::WIDTH);
            assert_eq!(log == 0, n == T::ONE);
            assert_eq!(n.floor_log_base(base), log);
            assert_eq!(n.ceiling_log_base(base), log);
        }
    });

    unsigned_gen_var_1::<T>().test_properties(|n| {
        assert_eq!(n.checked_log_base(T::TWO), n.checked_log_base_2());
    });

    unsigned_gen_var_6().test_properties(|n| {
        assert_eq!(T::ONE.checked_log_base(n), Some(0));
        assert_eq!(n.checked_log_base(n), Some(1));
    });
}

#[test]
fn checked_log_base_properties() {
    apply_fn_to_unsigneds!(checked_log_base_properties_helper);
}
