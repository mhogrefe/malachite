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
fn test_overflowing_pow() {
    fn test<T: PrimitiveInt>(x: T, y: u64, out: T, overflow: bool) {
        assert_eq!(x.overflowing_pow(y), (out, overflow));

        let mut x = x;
        assert_eq!(x.overflowing_pow_assign(y), overflow);
        assert_eq!(x, out);
    }
    test::<u8>(0, 0, 1, false);
    test::<u64>(123, 0, 1, false);
    test::<u64>(123, 1, 123, false);
    test::<u16>(0, 123, 0, false);
    test::<u16>(1, 123, 1, false);
    test::<i16>(-1, 123, -1, false);
    test::<i16>(-1, 124, 1, false);
    test::<u8>(3, 3, 27, false);
    test::<i32>(-10, 9, -1000000000, false);
    test::<i32>(-10, 10, 1410065408, true);
    test::<i16>(-10, 9, 13824, true);
    test::<i16>(10, 9, -13824, true);
    test::<i64>(123, 456, 2409344748064316129, true);
    test::<u64>(0, u64::MAX, 0, false);
    test::<u64>(1, u64::MAX, 1, false);
    test::<u64>(123, u64::MAX, 3449391168254631603, true);
    test::<i64>(0, u64::MAX, 0, false);
    test::<i64>(1, u64::MAX, 1, false);
    test::<i64>(-1, u64::MAX, -1, false);
    test::<i64>(-1, u64::MAX - 1, 1, false);
    test::<i64>(123, u64::MAX, 3449391168254631603, true);
    test::<i64>(-123, u64::MAX, -3449391168254631603, true);
    test::<i64>(-123, u64::MAX - 1, 4527249702272692521, true);
}

fn overflowing_pow_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen::<T, u64>().test_properties(|(x, y)| {
        let mut power = x;
        let overflow = power.overflowing_pow_assign(y);
        assert_eq!((power, overflow), x.overflowing_pow(y));
        assert_eq!(x.wrapping_pow(y), power);
        assert_eq!(x.checked_pow(y).is_none(), overflow);
        if !overflow {
            assert_eq!(power, x.pow(y));
        }
    });
}

fn overflowing_pow_properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen::<T, u64>().test_properties(|(x, y)| {
        let mut power = x;
        let overflow = power.overflowing_pow_assign(y);
        assert_eq!((power, overflow), x.overflowing_pow(y));
        assert_eq!(x.wrapping_pow(y), power);
        assert_eq!(x.checked_pow(y).is_none(), overflow);
        if !overflow {
            assert_eq!(power, x.pow(y));
        }
    });
}

#[test]
fn overflowing_pow_properties() {
    apply_fn_to_unsigneds!(overflowing_pow_properties_helper_unsigned);
    apply_fn_to_signeds!(overflowing_pow_properties_helper_signed);
}
