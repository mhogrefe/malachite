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
use malachite_base::test_util::generators::unsigned_gen_var_9;
use std::panic::catch_unwind;

fn low_mask_primitive_helper<T: PrimitiveInt>() {
    let test = |bits, out| {
        assert_eq!(T::low_mask(bits), out);
    };
    test(0, T::ZERO);
    test(1, T::ONE);
    test(2, T::exact_from(3));
    test(3, T::exact_from(7));
}

fn low_mask_unsigned_helper<T: PrimitiveUnsigned>() {
    let test = |bits, out| {
        assert_eq!(T::low_mask(bits), out);
    };
    test(T::WIDTH - 1, (T::ONE << (T::WIDTH - 1)) - T::ONE);
    test(T::WIDTH, T::MAX);
}

fn low_mask_signed_helper<T: PrimitiveSigned>() {
    let test = |bits, out| {
        assert_eq!(T::low_mask(bits), out);
    };
    test(T::WIDTH - 1, T::MAX);
    test(T::WIDTH, T::NEGATIVE_ONE);
}

#[test]
fn test_low_mask() {
    apply_fn_to_primitive_ints!(low_mask_primitive_helper);
    apply_fn_to_unsigneds!(low_mask_unsigned_helper);
    apply_fn_to_signeds!(low_mask_signed_helper);
}

fn low_mask_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::low_mask(T::WIDTH + 1));
}

#[test]
fn low_mask_fail() {
    apply_fn_to_primitive_ints!(low_mask_fail_helper);
}

fn low_mask_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_gen_var_9::<T>().test_properties(|bits| {
        let n = T::low_mask(bits);
        assert_eq!(n.count_ones(), bits);
        assert_eq!(n.index_of_next_false_bit(0), Some(bits));
    });
}

fn low_mask_properties_helper_signed<T: PrimitiveSigned>() {
    unsigned_gen_var_9::<T>().test_properties(|bits| {
        let n = T::low_mask(bits);
        assert_eq!(n.count_ones(), bits);
        assert_eq!(
            n.index_of_next_false_bit(0),
            if bits == T::WIDTH { None } else { Some(bits) }
        );
    });
}

#[test]
fn low_mask_properties() {
    apply_fn_to_unsigneds!(low_mask_properties_helper_unsigned);
    apply_fn_to_signeds!(low_mask_properties_helper_signed);
}
