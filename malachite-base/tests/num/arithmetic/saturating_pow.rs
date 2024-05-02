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
use malachite_base::test_util::generators::{signed_unsigned_pair_gen, unsigned_pair_gen};

#[test]
fn test_saturating_pow() {
    fn test<T: PrimitiveInt>(x: T, y: u64, out: T) {
        assert_eq!(x.saturating_pow(y), out);

        let mut x = x;
        x.saturating_pow_assign(y);
        assert_eq!(x, out);
    }
    test::<u8>(0, 0, 1);
    test::<u64>(123, 0, 1);
    test::<u64>(123, 1, 123);
    test::<u16>(0, 123, 0);
    test::<u16>(1, 123, 1);
    test::<i16>(-1, 123, -1);
    test::<i16>(-1, 124, 1);
    test::<u8>(3, 3, 27);
    test::<i32>(-10, 9, -1000000000);
    test::<i32>(-10, 10, i32::MAX);
    test::<i16>(-10, 9, i16::MIN);
    test::<i16>(10, 9, i16::MAX);
    test::<i64>(123, 456, i64::MAX);
    test::<u64>(0, u64::MAX, 0);
    test::<u64>(1, u64::MAX, 1);
    test::<u64>(123, u64::MAX, u64::MAX);
    test::<i64>(0, u64::MAX, 0);
    test::<i64>(1, u64::MAX, 1);
    test::<i64>(-1, u64::MAX, -1);
    test::<i64>(-1, u64::MAX - 1, 1);
    test::<i64>(123, u64::MAX, i64::MAX);
    test::<i64>(-123, u64::MAX, i64::MIN);
    test::<i64>(-123, u64::MAX - 1, i64::MAX);
}

fn saturating_pow_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen::<T, u64>().test_properties(|(x, y)| {
        let mut power = x;
        power.saturating_pow_assign(y);
        assert_eq!(power, x.saturating_pow(y));
        if y != 0 {
            assert!(power >= x);
        }
        if power < T::MAX {
            assert_eq!(power, x.pow(y));
        }
    });
}

fn saturating_pow_properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen::<T, u64>().test_properties(|(x, y)| {
        let mut power = x;
        power.saturating_pow_assign(y);
        assert_eq!(power, x.saturating_pow(y));
        if power > T::MIN && power < T::MAX {
            assert_eq!(power, x.pow(y));
        }
    });
}

#[test]
fn saturating_pow_properties() {
    apply_fn_to_unsigneds!(saturating_pow_properties_helper_unsigned);
    apply_fn_to_signeds!(saturating_pow_properties_helper_signed);
}
