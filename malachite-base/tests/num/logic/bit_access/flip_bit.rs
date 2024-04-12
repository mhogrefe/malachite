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
    signed_unsigned_pair_gen_var_2, unsigned_pair_gen_var_3,
};
use std::panic::catch_unwind;

fn flip_bit_helper_unsigned<T: PrimitiveInt>() {
    let test = |n: u64, index, out: u64| {
        let mut n = T::exact_from(n);
        n.flip_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(100, 0, 101);
    test(101, 0, 100);
    if T::WIDTH >= u16::WIDTH {
        test(0, 10, 1024);
        test(1024, 10, 0);
    }
    if T::WIDTH >= u64::WIDTH {
        test(1000000000000, 10, 1000000001024);
        test(1000000001024, 10, 1000000000000);
    }
}

fn flip_bit_helper_signed<T: PrimitiveSigned>() {
    flip_bit_helper_unsigned::<T>();

    let test = |n: i64, index, out: i64| {
        let mut n = T::exact_from(n);
        n.flip_bit(index);
        assert_eq!(n, T::exact_from(out));
    };

    test(-1, 5, -33);
    test(-33, 5, -1);
    test(-32, 0, -31);
    test(-31, 0, -32);

    if T::WIDTH >= u64::WIDTH {
        test(-1000000000000, 10, -999999998976);
        test(-999999998976, 10, -1000000000000);
    }

    let mut n = T::ZERO;
    n.flip_bit(T::WIDTH - 1);
    assert_eq!(n, T::MIN);

    let mut n = T::MIN;
    n.flip_bit(T::WIDTH - 1);
    assert_eq!(n, T::ZERO);
}

#[test]
fn test_flip_bit() {
    apply_fn_to_unsigneds!(flip_bit_helper_unsigned);
    apply_fn_to_signeds!(flip_bit_helper_signed);
}

fn flip_bit_fail_helper_unsigned<T: PrimitiveUnsigned>() {
    assert_panic!(T::exact_from(5).flip_bit(200));
}

fn flip_bit_fail_helper_signed<T: PrimitiveSigned>() {
    assert_panic!(T::exact_from(5).flip_bit(200));
    assert_panic!({
        let mut n = T::NEGATIVE_ONE;
        n.flip_bit(200);
    });
}

#[test]
fn flip_bit_fail() {
    apply_fn_to_unsigneds!(flip_bit_fail_helper_unsigned);
    apply_fn_to_signeds!(flip_bit_fail_helper_signed);
}

fn properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_3::<T>().test_properties(|(mut n, index)| {
        let old_n = n;
        n.flip_bit(index);
        assert_ne!(n, old_n);

        n.flip_bit(index);
        assert_eq!(n, old_n);
    });
}

fn properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen_var_2::<T>().test_properties(|(mut n, index)| {
        let old_n = n;
        n.flip_bit(index);
        assert_ne!(n, old_n);

        n.flip_bit(index);
        assert_eq!(n, old_n);
    });
}

#[test]
fn flip_bit_properties() {
    apply_fn_to_unsigneds!(properties_helper_unsigned);
    apply_fn_to_signeds!(properties_helper_signed);
}
