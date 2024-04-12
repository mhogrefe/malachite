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
    signed_gen, signed_gen_var_6, signed_pair_gen, signed_pair_gen_var_3, signed_pair_gen_var_5,
    unsigned_gen, unsigned_gen_var_1, unsigned_pair_gen_var_11, unsigned_pair_gen_var_13,
    unsigned_pair_gen_var_27,
};

#[test]
fn test_divisible_by() {
    fn test<T: PrimitiveInt>(x: T, y: T, out: bool) {
        assert_eq!(x.divisible_by(y), out);
    }
    test::<u8>(0, 0, true);
    test::<u16>(1, 0, false);
    test::<u64>(1000000000000, 0, false);
    test::<u32>(0, 1, true);
    test::<u128>(0, 123, true);
    test::<usize>(1, 1, true);
    test::<u8>(123, 1, true);
    test::<u16>(123, 123, true);
    test::<u32>(123, 456, false);
    test::<u64>(456, 123, false);
    test::<u128>(369, 123, true);
    test::<usize>(0xffffffff, 1, true);
    test::<u32>(u32::MAX, u32::MAX, true);
    test::<u64>(1000000000000, 1, true);
    test::<u64>(1000000000000, 3, false);
    test::<u64>(1000000000002, 3, true);
    test::<u64>(1000000000000, 123, false);
    test::<u64>(1000000000000, 0xffffffff, false);
    test::<u128>(1000000000000000000000000, 1, true);
    test::<u128>(1000000000000000000000000, 3, false);
    test::<u128>(1000000000000000000000002, 3, true);
    test::<u128>(1000000000000000000000000, 123, false);
    test::<u128>(1000000000000000000000000, 0xffffffff, false);
    test::<u128>(1000000000000000000000000, 1000000000000, true);
    test::<u128>(1000000000000000000000000, 1000000000001, false);

    test::<i64>(1000000000000, 0, false);
    test::<i32>(0, -1, true);
    test::<i128>(0, -123, true);
    test::<isize>(1, -1, true);
    test::<i8>(123, -1, true);
    test::<i16>(123, -123, true);
    test::<i32>(123, -456, false);
    test::<i64>(456, -123, false);
    test::<i128>(369, -123, true);
    test::<i64>(0xffffffff, -1, true);
    test::<i64>(0xffffffff, -0xffffffff, true);
    test::<i64>(1000000000000, -1, true);
    test::<i64>(1000000000000, -3, false);
    test::<i64>(1000000000002, -3, true);
    test::<i64>(1000000000000, -123, false);
    test::<i64>(1000000000000, -0xffffffff, false);
    test::<i128>(1000000000000000000000000, -1, true);
    test::<i128>(1000000000000000000000000, -3, false);
    test::<i128>(1000000000000000000000002, -3, true);
    test::<i128>(1000000000000000000000000, -123, false);
    test::<i128>(1000000000000000000000000, -0xffffffff, false);
    test::<i128>(1000000000000000000000000, -1000000000000, true);
    test::<i128>(1000000000000000000000000, -1000000000001, false);

    test::<i16>(-1, 0, false);
    test::<i64>(-1000000000000, 0, false);
    test::<isize>(-1, 1, true);
    test::<i8>(-123, 1, true);
    test::<i16>(-123, 123, true);
    test::<i32>(-123, 456, false);
    test::<i64>(-456, 123, false);
    test::<i128>(-369, 123, true);
    test::<i64>(-0xffffffff, 1, true);
    test::<i64>(-0xffffffff, 0xffffffff, true);
    test::<i64>(-1000000000000, 1, true);
    test::<i64>(-1000000000000, 3, false);
    test::<i64>(-1000000000002, 3, true);
    test::<i64>(-1000000000000, 123, false);
    test::<i64>(-1000000000000, 0xffffffff, false);
    test::<i128>(-1000000000000000000000000, 1, true);
    test::<i128>(-1000000000000000000000000, 3, false);
    test::<i128>(-1000000000000000000000002, 3, true);
    test::<i128>(-1000000000000000000000000, 123, false);
    test::<i128>(-1000000000000000000000000, 0xffffffff, false);
    test::<i128>(-1000000000000000000000000, 1000000000000, true);
    test::<i128>(-1000000000000000000000000, 1000000000001, false);

    test::<isize>(-1, -1, true);
    test::<i8>(-123, -1, true);
    test::<i16>(-123, -123, true);
    test::<i32>(-123, -456, false);
    test::<i64>(-456, -123, false);
    test::<i128>(-369, -123, true);
    test::<i64>(-0xffffffff, -1, true);
    test::<i64>(-0xffffffff, -0xffffffff, true);
    test::<i64>(-1000000000000, -1, true);
    test::<i64>(-1000000000000, -3, false);
    test::<i64>(-1000000000002, -3, true);
    test::<i64>(-1000000000000, -123, false);
    test::<i64>(-1000000000000, -0xffffffff, false);
    test::<i128>(-1000000000000000000000000, -1, true);
    test::<i128>(-1000000000000000000000000, -3, false);
    test::<i128>(-1000000000000000000000002, -3, true);
    test::<i128>(-1000000000000000000000000, -123, false);
    test::<i128>(-1000000000000000000000000, -0xffffffff, false);
    test::<i128>(-1000000000000000000000000, -1000000000000, true);
    test::<i128>(-1000000000000000000000000, -1000000000001, false);
}

fn divisible_by_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        let divisible = x.divisible_by(y);
        assert_eq!(x == T::ZERO || y != T::ZERO && x % y == T::ZERO, divisible);
    });

    unsigned_pair_gen_var_11::<T>().test_properties(|(x, y)| {
        assert!(x.divisible_by(y));
        assert!(x == T::ZERO || y != T::ZERO && x % y == T::ZERO);
    });

    unsigned_pair_gen_var_13::<T>().test_properties(|(x, y)| {
        assert!(!x.divisible_by(y));
        assert!(x != T::ZERO && (y == T::ZERO || x % y != T::ZERO));
    });

    unsigned_gen::<T>().test_properties(|n| {
        assert!(n.divisible_by(T::ONE));
    });

    unsigned_gen_var_1::<T>().test_properties(|n| {
        assert!(!n.divisible_by(T::ZERO));
        assert!(T::ZERO.divisible_by(n));
        if n > T::ONE {
            assert!(!T::ONE.divisible_by(n));
        }
        assert!(n.divisible_by(n));
    });
}

fn divisible_by_properties_helper_signed<T: PrimitiveSigned>() {
    signed_pair_gen::<T>().test_properties(|(x, y)| {
        let divisible = x.divisible_by(y);
        assert_eq!(
            x == T::ZERO || x == T::MIN && y == T::NEGATIVE_ONE || y != T::ZERO && x % y == T::ZERO,
            divisible
        );
        if x != T::MIN {
            assert_eq!((-x).divisible_by(y), divisible);
        }
        if y != T::MIN {
            assert_eq!(x.divisible_by(-y), divisible);
        }
    });

    signed_pair_gen_var_3::<T>().test_properties(|(x, y)| {
        assert!(x.divisible_by(y));
        assert!(
            x == T::ZERO || x == T::MIN && y == T::NEGATIVE_ONE || y != T::ZERO && x % y == T::ZERO
        );
    });

    signed_pair_gen_var_5::<T>().test_properties(|(x, y)| {
        assert!(!x.divisible_by(y));
        assert!(
            x != T::ZERO
                && (x != T::MIN || y != T::NEGATIVE_ONE)
                && (y == T::ZERO || x % y != T::ZERO)
        );
    });

    signed_gen::<T>().test_properties(|n| {
        assert!(n.divisible_by(T::ONE));
        assert!(n.divisible_by(T::NEGATIVE_ONE));
    });

    signed_gen_var_6::<T>().test_properties(|n| {
        assert!(!n.divisible_by(T::ZERO));
        assert!(T::ZERO.divisible_by(n));
        if n > T::ONE {
            assert!(!T::ONE.divisible_by(n));
        }
        assert!(n.divisible_by(n));
    });
}

#[test]
fn divisible_by_properties() {
    apply_fn_to_unsigneds!(divisible_by_properties_helper_unsigned);
    apply_fn_to_signeds!(divisible_by_properties_helper_signed);
}
