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
    signed_unsigned_bool_triple_gen_var_1, unsigned_unsigned_bool_triple_gen_var_1,
};
use std::panic::catch_unwind;

fn assign_bit_helper_unsigned<T: PrimitiveInt>() {
    let test = |n: u64, index, bit, out: u64| {
        let mut n = T::exact_from(n);
        n.assign_bit(index, bit);
        assert_eq!(n, T::exact_from(out));
    };

    test(100, 0, true, 101);
    test(0, 10, false, 0);
    test(0, 100, false, 0);
    test(101, 0, false, 100);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, true, 1024);
        test(1024, 10, false, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1000000000000, 10, true, 1000000001024);
        test(1000000001024, 10, false, 1000000000000);
        test(1000000001024, 100, false, 1000000001024);
    }
}

fn assign_bit_helper_signed<T: PrimitiveSigned>() {
    assign_bit_helper_unsigned::<T>();

    let test = |n: i64, index, bit, out: i64| {
        let mut n = T::exact_from(n);
        n.assign_bit(index, bit);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, true, -1);
    test(-1, 100, true, -1);
    test(-33, 5, true, -1);
    test(-32, 0, true, -31);
    test(-1, 5, false, -33);
    test(-31, 0, false, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-1000000000000, 10, true, -999999998976);
        test(-1000000000000, 100, true, -1000000000000);
        test(-999999998976, 10, false, -1000000000000);
    }

    let mut n = T::ZERO;
    n.assign_bit(T::WIDTH - 1, true);
    assert_eq!(n, T::MIN);

    let mut n = T::MIN;
    n.assign_bit(T::WIDTH - 1, false);
    assert_eq!(n, T::ZERO);
}

#[test]
fn test_assign_bit() {
    apply_fn_to_unsigneds!(assign_bit_helper_unsigned);
    apply_fn_to_signeds!(assign_bit_helper_signed);
}

fn assign_bit_fail_helper<T: PrimitiveInt>() {
    assert_panic!({
        let mut n = T::exact_from(5);
        n.assign_bit(200, true);
    });
}

fn assign_bit_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!({
        let mut n = T::NEGATIVE_ONE;
        n.assign_bit(200, false);
    });
}

#[test]
fn assign_bit_fail() {
    apply_fn_to_primitive_ints!(assign_bit_fail_helper);
    apply_fn_to_signeds!(assign_bit_fail_helper_signed);
}

fn properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_unsigned_bool_triple_gen_var_1::<T>().test_properties(|(mut n, index, bit)| {
        n.assign_bit(index, bit);
    });
}

fn properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_bool_triple_gen_var_1::<T>().test_properties(|(mut n, index, bit)| {
        n.assign_bit(index, bit);
    });
}

#[test]
fn assign_bit_properties() {
    apply_fn_to_unsigneds!(properties_helper_unsigned);
    apply_fn_to_signeds!(properties_helper_signed);
}
