// Copyright © 2024 Mikhail Hogrefe
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
    signed_unsigned_pair_gen_var_4, unsigned_pair_gen_var_2,
};
use std::panic::catch_unwind;

fn clear_bit_helper_unsigned<T: PrimitiveInt>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::exact_from(n);
        n.clear_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(0, 10, 0);
    test(0, 100, 0);
    test(101, 0, 100);
    if T::WIDTH >= u16::WIDTH {
        test(1024, 10, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1000000001024, 10, 1000000000000);
        test(1000000001024, 100, 1000000001024);
    }
}

fn clear_bit_helper_signed<T: PrimitiveSigned>() {
    clear_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out: i64| {
        let mut n = T::exact_from(n);
        n.clear_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, -33);
    test(-31, 0, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-999999998976, 10, -1000000000000);
    }

    let mut n = T::MIN;
    n.clear_bit(T::WIDTH - 1);
    assert_eq!(n, T::ZERO);
}

#[test]
fn test_clear_bit() {
    apply_fn_to_unsigneds!(clear_bit_helper_unsigned);
    apply_fn_to_signeds!(clear_bit_helper_signed);
}

fn clear_bit_fail_helper<T: PrimitiveSigned>() {
    assert_panic!({
        let mut n = T::NEGATIVE_ONE;
        n.clear_bit(200);
    });
}

#[test]
fn clear_bit_fail() {
    apply_fn_to_signeds!(clear_bit_fail_helper);
}

fn properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_2::<T, u64>().test_properties(|(mut n, index)| {
        let old_n = n;
        n.clear_bit(index);
        let result = n;

        let mut n = old_n;
        n.assign_bit(index, false);
        assert_eq!(n, result);

        assert!(result <= old_n);
        if old_n.get_bit(index) {
            assert_ne!(result, old_n);
            let mut n = result;
            n.set_bit(index);
            assert_eq!(n, old_n);
        } else {
            assert_eq!(result, old_n);
        }
    });
}

fn properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen_var_4::<T>().test_properties(|(mut n, index)| {
        let old_n = n;
        n.clear_bit(index);
        let result = n;

        let mut n = old_n;
        n.assign_bit(index, false);
        assert_eq!(n, result);

        if old_n < T::ZERO && index == T::WIDTH - 1 {
            assert!(result >= T::ZERO);
        } else {
            assert!(result <= old_n);
        }
        if old_n.get_bit(index) {
            assert_ne!(result, old_n);
            let mut n = result;
            n.set_bit(index);
            assert_eq!(n, old_n);
        } else {
            assert_eq!(result, old_n);
        }

        let mut n = !old_n;
        n.set_bit(index);
        n.not_assign();
        assert_eq!(n, result);
    });
}

#[test]
fn clear_bit_properties() {
    apply_fn_to_unsigneds!(properties_helper_unsigned);
    apply_fn_to_signeds!(properties_helper_signed);
}
