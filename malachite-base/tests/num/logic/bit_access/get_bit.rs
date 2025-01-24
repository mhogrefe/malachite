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
    signed_unsigned_pair_gen_var_1, unsigned_gen, unsigned_pair_gen_var_2,
};

fn test_helper_primitive_int<T: PrimitiveInt>() {
    let test = |n: u64, index, out| {
        assert_eq!(T::exact_from(n).get_bit(index), out);
    };

    test(0, 0, false);
    test(0, 100, false);
    test(123, 2, false);
    test(123, 3, true);
    test(123, 100, false);
    if T::WIDTH >= u64::WIDTH {
        test(1000000000000, 12, true);
        test(1000000000000, 100, false);
    }
}

fn test_helper_signed<T: PrimitiveSigned>() {
    let test = |n: i64, index, out| {
        assert_eq!(T::exact_from(n).get_bit(index), out);
    };

    test(-123, 0, true);
    test(-123, 1, false);
    test(-123, 100, true);
    if T::WIDTH >= u64::WIDTH {
        test(-1000000000000, 12, true);
        test(-1000000000000, 100, true);
        test(-i64::from(u32::MAX), 0, true);
        test(-i64::from(u32::MAX), 1, false);
        test(-i64::from(u32::MAX), 31, false);
        test(-i64::from(u32::MAX), 32, true);
        test(-i64::from(u32::MAX), 33, true);
        test(-i64::from(u32::MAX) - 1, 0, false);
        test(-i64::from(u32::MAX) - 1, 31, false);
        test(-i64::from(u32::MAX) - 1, 32, true);
        test(-i64::from(u32::MAX) - 1, 33, true);
    }
}

#[test]
fn test_get_bit() {
    apply_fn_to_primitive_ints!(test_helper_primitive_int);
    apply_fn_to_signeds!(test_helper_signed);
}

fn properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_2::<T, _>().test_properties(|(n, index)| {
        let bit = n.get_bit(index);
        if index >= T::WIDTH {
            assert!(!bit);
        } else {
            assert_eq!(bit, !(!n).get_bit(index));
        }
    });

    unsigned_gen::<T>().test_properties(|n| {
        let significant_bits = n.significant_bits();
        assert!(!n.get_bit(significant_bits));
        if n != T::ZERO {
            assert!(n.get_bit(significant_bits - 1));
        }
    });
}

fn properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen_var_1::<T, _>().test_properties(|(n, index)| {
        let bit = n.get_bit(index);
        if index >= T::WIDTH {
            assert_eq!(bit, n < T::ZERO);
        } else {
            assert_eq!(bit, !(!n).get_bit(index));
        }
    });
}

#[test]
fn get_bit_properties() {
    apply_fn_to_unsigneds!(properties_helper_unsigned);
    apply_fn_to_signeds!(properties_helper_signed);
}
