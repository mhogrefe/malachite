// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{
    signed_unsigned_pair_gen_var_3, unsigned_pair_gen_var_3,
};
use std::panic::catch_unwind;

fn set_bit_helper_unsigned<T: PrimitiveInt>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::exact_from(n);
        n.set_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(100, 0, 101);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, 1024);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1000000000000, 10, 1000000001024);
    }
}

fn set_bit_helper_signed<T: PrimitiveSigned>() {
    set_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out: i64| {
        let mut n = T::exact_from(n);
        n.set_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, -1);
    test(-1, 100, -1);
    test(-33, 5, -1);
    test(-32, 0, -31);

    if T::WIDTH >= u64::WIDTH {
        test(-1000000000000, 10, -999999998976);
        test(-1000000000000, 100, -1000000000000);
    }

    let mut n = T::ZERO;
    n.set_bit(T::WIDTH - 1);
    assert_eq!(n, T::MIN);
}

#[test]
fn test_set_bit() {
    apply_fn_to_unsigneds!(set_bit_helper_unsigned);
    apply_fn_to_signeds!(set_bit_helper_signed);
}

fn set_bit_fail_helper<T: PrimitiveInt>() {
    assert_panic!({
        let mut n = T::exact_from(5);
        n.set_bit(200);
    });
}

#[test]
fn set_bit_fail() {
    apply_fn_to_primitive_ints!(set_bit_fail_helper);
}

fn properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_3::<T>().test_properties(|(mut n, index)| {
        let old_n = n;
        n.set_bit(index);
        let result = n;

        let mut n = old_n;
        n.assign_bit(index, true);
        assert_eq!(n, result);

        assert_ne!(result, T::ZERO);
        assert!(result >= old_n);
        if old_n.get_bit(index) {
            assert_eq!(result, old_n);
        } else {
            assert_ne!(result, old_n);
            let mut n = result;
            n.clear_bit(index);
            assert_eq!(n, old_n);
        }
    });
}

fn properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen_var_3::<T>().test_properties(|(mut n, index)| {
        let old_n = n;
        n.set_bit(index);
        let result = n;

        let mut n = old_n;
        n.assign_bit(index, true);
        assert_eq!(n, result);

        assert_ne!(result, T::ZERO);
        if old_n >= T::ZERO && index == T::WIDTH - 1 {
            assert!(result < T::ZERO);
        } else {
            assert!(result >= old_n);
        }
        if old_n.get_bit(index) {
            assert_eq!(result, old_n);
        } else {
            assert_ne!(result, old_n);
            let mut n = result;
            n.clear_bit(index);
            assert_eq!(n, old_n);
        }

        let mut n = !old_n;
        n.clear_bit(index);
        n.not_assign();
        assert_eq!(n, result);
    });
}

#[test]
fn set_bit_properties() {
    apply_fn_to_unsigneds!(properties_helper_unsigned);
    apply_fn_to_signeds!(properties_helper_signed);
}
