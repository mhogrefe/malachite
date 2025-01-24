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
use malachite_base::test_util::generators::{signed_unsigned_pair_gen, unsigned_pair_gen};

#[test]
fn test_checked_pow() {
    fn test<T: PrimitiveInt>(x: T, y: u64, out: Option<T>) {
        assert_eq!(x.checked_pow(y), out);
    }
    test::<u8>(0, 0, Some(1));
    test::<u64>(123, 0, Some(1));
    test::<u64>(123, 1, Some(123));
    test::<u16>(0, 123, Some(0));
    test::<u16>(1, 123, Some(1));
    test::<i16>(-1, 123, Some(-1));
    test::<i16>(-1, 124, Some(1));
    test::<u8>(3, 3, Some(27));
    test::<i32>(-10, 9, Some(-1000000000));
    test::<i32>(-10, 10, None);
    test::<i16>(-10, 9, None);
    test::<i16>(10, 9, None);
    test::<i64>(123, 456, None);
    test::<u64>(0, u64::MAX, Some(0));
    test::<u64>(1, u64::MAX, Some(1));
    test::<u64>(123, u64::MAX, None);
    test::<i64>(0, u64::MAX, Some(0));
    test::<i64>(1, u64::MAX, Some(1));
    test::<i64>(-1, u64::MAX, Some(-1));
    test::<i64>(-1, u64::MAX - 1, Some(1));
    test::<i64>(123, u64::MAX, None);
    test::<i64>(-123, u64::MAX, None);
    test::<i64>(-123, u64::MAX - 1, None);
}

fn checked_pow_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen::<T, u64>().test_properties(|(x, y)| {
        let power = x.checked_pow(y);
        if let Some(power) = power {
            if y != 0 {
                assert!(power >= x);
            }
            assert_eq!(power, x.pow(y));
        }
    });
}

fn checked_pow_properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen::<T, u64>().test_properties(|(x, y)| {
        let power = x.checked_pow(y);
        if let Some(power) = power {
            assert_eq!(power, x.pow(y));
        }
    });
}

#[test]
fn checked_pow_properties() {
    apply_fn_to_unsigneds!(checked_pow_properties_helper_unsigned);
    apply_fn_to_signeds!(checked_pow_properties_helper_signed);
}
