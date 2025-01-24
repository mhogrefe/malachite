// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::test_util::generators::{signed_unsigned_pair_gen, unsigned_pair_gen};

#[test]
fn test_wrapping_pow() {
    fn test<T: PrimitiveInt>(x: T, y: u64, out: T) {
        assert_eq!(x.wrapping_pow(y), out);

        let mut x = x;
        x.wrapping_pow_assign(y);
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
    test::<i32>(-10, 10, 1410065408);
    test::<i16>(-10, 9, 13824);
    test::<i16>(10, 9, -13824);
    test::<i64>(123, 456, 2409344748064316129);
    test::<u64>(0, u64::MAX, 0);
    test::<u64>(1, u64::MAX, 1);
    test::<u64>(123, u64::MAX, 3449391168254631603);
    test::<i64>(0, u64::MAX, 0);
    test::<i64>(1, u64::MAX, 1);
    test::<i64>(-1, u64::MAX, -1);
    test::<i64>(-1, u64::MAX - 1, 1);
    test::<i64>(123, u64::MAX, 3449391168254631603);
    test::<i64>(-123, u64::MAX, -3449391168254631603);
    test::<i64>(-123, u64::MAX - 1, 4527249702272692521);
}

fn wrapping_pow_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen::<T, u64>().test_properties(|(x, y)| {
        let mut power = x;
        power.wrapping_pow_assign(y);
        assert_eq!(power, x.wrapping_pow(y));
    });
}

fn wrapping_pow_properties_helper_signed<T: PrimitiveSigned>() {
    signed_unsigned_pair_gen::<T, u64>().test_properties(|(x, y)| {
        let mut power = x;
        power.wrapping_pow_assign(y);
        assert_eq!(power, x.wrapping_pow(y));
        if x != T::MIN {
            let neg_pow = (-x).wrapping_pow(y);
            if y.even() {
                assert_eq!(neg_pow, power);
            } else {
                assert_eq!(neg_pow, power.wrapping_neg());
            }
        }
    });
}

#[test]
fn wrapping_pow_properties() {
    apply_fn_to_unsigneds!(wrapping_pow_properties_helper_unsigned);
    apply_fn_to_signeds!(wrapping_pow_properties_helper_signed);
}
