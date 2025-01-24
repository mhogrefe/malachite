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
use malachite_base::test_util::generators::{signed_gen, unsigned_gen};

fn significant_bits_helper<T: PrimitiveInt>() {
    let test = |n, out| {
        assert_eq!(T::exact_from(n).significant_bits(), out);
    };
    test(0, 0);
    test(1, 1);
    test(2, 2);
    test(3, 2);
    test(4, 3);
    test(5, 3);
    test(100, 7);
    test(63, 6);
    test(64, 7);
}

fn significant_bits_helper_unsigned<T: PrimitiveUnsigned>() {
    let test = |n: T, out: u64| {
        assert_eq!(n.significant_bits(), out);
    };
    test(T::MAX, T::WIDTH);
}

fn significant_bits_helper_signed<T: PrimitiveSigned>() {
    let test = |n: T, out: u64| {
        assert_eq!(n.significant_bits(), out);
    };
    test(T::MAX, T::WIDTH - 1);
    test(T::MIN, T::WIDTH);
}

#[test]
fn test_significant_bits() {
    apply_fn_to_primitive_ints!(significant_bits_helper);
    apply_fn_to_unsigneds!(significant_bits_helper_unsigned);
    apply_fn_to_signeds!(significant_bits_helper_signed);
}

fn significant_bits_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen::<T>().test_properties(|u| {
        let significant_bits = u.significant_bits();
        assert!(significant_bits <= T::WIDTH);
        assert_eq!(significant_bits == 0, u == T::ZERO);
        if u != T::ZERO {
            assert_eq!(significant_bits, u.floor_log_base_2() + 1);
        }
    });
}

fn significant_bits_properties_helper_signed<T: PrimitiveSigned>() {
    signed_gen::<T>().test_properties(|i| {
        let significant_bits = i.significant_bits();
        assert!(significant_bits <= T::WIDTH);
        assert_eq!(significant_bits == 0, i == T::ZERO);
        assert_eq!(significant_bits == T::WIDTH, i == T::MIN);
        assert_eq!(i.wrapping_neg().significant_bits(), significant_bits);
    });
}

#[test]
fn significant_bits_properties() {
    apply_fn_to_unsigneds!(significant_bits_properties_helper_unsigned);
    apply_fn_to_signeds!(significant_bits_properties_helper_signed);
}
