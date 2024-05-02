// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShr, ShlRound, ShlRoundAssign, ShrRound, UnsignedAbs,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::{
    signed_rounding_mode_pair_gen, signed_signed_rounding_mode_triple_gen_var_4,
    unsigned_rounding_mode_pair_gen, unsigned_signed_rounding_mode_triple_gen_var_2,
};
use std::cmp::Ordering;
use std::panic::catch_unwind;

#[test]
fn test_shl_round() {
    fn test<T: PrimitiveInt + ShlRound<U, Output = T> + ShlRoundAssign<U>, U: PrimitiveInt>(
        t: T,
        u: U,
        rm: RoundingMode,
        out: T,
        o: Ordering,
    ) {
        assert_eq!(t.shl_round(u, rm), (out, o));

        let mut t = t;
        assert_eq!(t.shl_round_assign(u, rm), o);
        assert_eq!(t, out);
    }
    test::<u8, i8>(0, 0, RoundingMode::Down, 0, Ordering::Equal);
    test::<u8, i8>(0, 0, RoundingMode::Up, 0, Ordering::Equal);
    test::<u8, i8>(0, 0, RoundingMode::Floor, 0, Ordering::Equal);
    test::<u8, i8>(0, 0, RoundingMode::Ceiling, 0, Ordering::Equal);
    test::<u8, i8>(0, 0, RoundingMode::Nearest, 0, Ordering::Equal);
    test::<u8, i8>(0, 0, RoundingMode::Exact, 0, Ordering::Equal);

    test::<u8, i16>(0, -10, RoundingMode::Down, 0, Ordering::Equal);
    test::<u8, i16>(0, -10, RoundingMode::Up, 0, Ordering::Equal);
    test::<u8, i16>(0, -10, RoundingMode::Floor, 0, Ordering::Equal);
    test::<u8, i16>(0, -10, RoundingMode::Ceiling, 0, Ordering::Equal);
    test::<u8, i16>(0, -10, RoundingMode::Nearest, 0, Ordering::Equal);
    test::<u8, i16>(0, -10, RoundingMode::Exact, 0, Ordering::Equal);

    test::<i8, i32>(123, 0, RoundingMode::Down, 123, Ordering::Equal);
    test::<i8, i32>(123, 0, RoundingMode::Up, 123, Ordering::Equal);
    test::<i8, i32>(123, 0, RoundingMode::Floor, 123, Ordering::Equal);
    test::<i8, i32>(123, 0, RoundingMode::Ceiling, 123, Ordering::Equal);
    test::<i8, i32>(123, 0, RoundingMode::Nearest, 123, Ordering::Equal);
    test::<i8, i32>(123, 0, RoundingMode::Exact, 123, Ordering::Equal);

    test::<u8, i64>(245, -1, RoundingMode::Down, 122, Ordering::Less);
    test::<u8, i64>(245, -1, RoundingMode::Up, 123, Ordering::Greater);
    test::<u8, i64>(245, -1, RoundingMode::Floor, 122, Ordering::Less);
    test::<u8, i64>(245, -1, RoundingMode::Ceiling, 123, Ordering::Greater);
    test::<u8, i64>(245, -1, RoundingMode::Nearest, 122, Ordering::Less);

    test::<u8, i128>(246, -1, RoundingMode::Down, 123, Ordering::Equal);
    test::<u8, i128>(246, -1, RoundingMode::Up, 123, Ordering::Equal);
    test::<u8, i128>(246, -1, RoundingMode::Floor, 123, Ordering::Equal);
    test::<u8, i128>(246, -1, RoundingMode::Ceiling, 123, Ordering::Equal);
    test::<u8, i128>(246, -1, RoundingMode::Nearest, 123, Ordering::Equal);
    test::<u8, i128>(246, -1, RoundingMode::Exact, 123, Ordering::Equal);

    test::<u8, isize>(247, -1, RoundingMode::Down, 123, Ordering::Less);
    test::<u8, isize>(247, -1, RoundingMode::Up, 124, Ordering::Greater);
    test::<u8, isize>(247, -1, RoundingMode::Floor, 123, Ordering::Less);
    test::<u8, isize>(247, -1, RoundingMode::Ceiling, 124, Ordering::Greater);
    test::<u8, isize>(247, -1, RoundingMode::Nearest, 124, Ordering::Greater);

    test::<i16, i8>(491, -2, RoundingMode::Down, 122, Ordering::Less);
    test::<i16, i8>(491, -2, RoundingMode::Up, 123, Ordering::Greater);
    test::<i16, i8>(491, -2, RoundingMode::Floor, 122, Ordering::Less);
    test::<i16, i8>(491, -2, RoundingMode::Ceiling, 123, Ordering::Greater);
    test::<i16, i8>(491, -2, RoundingMode::Nearest, 123, Ordering::Greater);

    test::<u16, i16>(492, -2, RoundingMode::Down, 123, Ordering::Equal);
    test::<u16, i16>(492, -2, RoundingMode::Up, 123, Ordering::Equal);
    test::<u16, i16>(492, -2, RoundingMode::Floor, 123, Ordering::Equal);
    test::<u16, i16>(492, -2, RoundingMode::Ceiling, 123, Ordering::Equal);
    test::<u16, i16>(492, -2, RoundingMode::Nearest, 123, Ordering::Equal);
    test::<u16, i16>(492, -2, RoundingMode::Exact, 123, Ordering::Equal);

    test::<i16, i32>(493, -2, RoundingMode::Down, 123, Ordering::Less);
    test::<i16, i32>(493, -2, RoundingMode::Up, 124, Ordering::Greater);
    test::<i16, i32>(493, -2, RoundingMode::Floor, 123, Ordering::Less);
    test::<i16, i32>(493, -2, RoundingMode::Ceiling, 124, Ordering::Greater);
    test::<i16, i32>(493, -2, RoundingMode::Nearest, 123, Ordering::Less);

    test::<u32, i8>(4127195135, -25, RoundingMode::Down, 122, Ordering::Less);
    test::<u32, i8>(4127195135, -25, RoundingMode::Up, 123, Ordering::Greater);
    test::<u32, i8>(4127195135, -25, RoundingMode::Floor, 122, Ordering::Less);
    test::<u32, i8>(
        4127195135,
        -25,
        RoundingMode::Ceiling,
        123,
        Ordering::Greater,
    );
    test::<u32, i8>(
        4127195135,
        -25,
        RoundingMode::Nearest,
        123,
        Ordering::Greater,
    );

    test::<u32, i16>(4127195136, -25, RoundingMode::Down, 123, Ordering::Equal);
    test::<u32, i16>(4127195136, -25, RoundingMode::Up, 123, Ordering::Equal);
    test::<u32, i16>(4127195136, -25, RoundingMode::Floor, 123, Ordering::Equal);
    test::<u32, i16>(4127195136, -25, RoundingMode::Ceiling, 123, Ordering::Equal);
    test::<u32, i16>(4127195136, -25, RoundingMode::Nearest, 123, Ordering::Equal);
    test::<u32, i16>(4127195136, -25, RoundingMode::Exact, 123, Ordering::Equal);

    test::<u32, i32>(4127195137, -25, RoundingMode::Down, 123, Ordering::Less);
    test::<u32, i32>(4127195137, -25, RoundingMode::Up, 124, Ordering::Greater);
    test::<u32, i32>(4127195137, -25, RoundingMode::Floor, 123, Ordering::Less);
    test::<u32, i32>(
        4127195137,
        -25,
        RoundingMode::Ceiling,
        124,
        Ordering::Greater,
    );
    test::<u32, i32>(4127195137, -25, RoundingMode::Nearest, 123, Ordering::Less);

    test::<i64, i8>(8254390271, -26, RoundingMode::Down, 122, Ordering::Less);
    test::<i64, i8>(8254390271, -26, RoundingMode::Up, 123, Ordering::Greater);
    test::<i64, i8>(8254390271, -26, RoundingMode::Floor, 122, Ordering::Less);
    test::<i64, i8>(
        8254390271,
        -26,
        RoundingMode::Ceiling,
        123,
        Ordering::Greater,
    );
    test::<i64, i8>(
        8254390271,
        -26,
        RoundingMode::Nearest,
        123,
        Ordering::Greater,
    );

    test::<u64, i16>(8254390272, -26, RoundingMode::Down, 123, Ordering::Equal);
    test::<u64, i16>(8254390272, -26, RoundingMode::Up, 123, Ordering::Equal);
    test::<u64, i16>(8254390272, -26, RoundingMode::Floor, 123, Ordering::Equal);
    test::<u64, i16>(8254390272, -26, RoundingMode::Ceiling, 123, Ordering::Equal);
    test::<u64, i16>(8254390272, -26, RoundingMode::Nearest, 123, Ordering::Equal);
    test::<u64, i16>(8254390272, -26, RoundingMode::Exact, 123, Ordering::Equal);

    test::<i64, i32>(8254390273, -26, RoundingMode::Down, 123, Ordering::Less);
    test::<i64, i32>(8254390273, -26, RoundingMode::Up, 124, Ordering::Greater);
    test::<i64, i32>(8254390273, -26, RoundingMode::Floor, 123, Ordering::Less);
    test::<i64, i32>(
        8254390273,
        -26,
        RoundingMode::Ceiling,
        124,
        Ordering::Greater,
    );
    test::<i64, i32>(8254390273, -26, RoundingMode::Nearest, 123, Ordering::Less);

    test::<i64, i64>(
        0xffffffff,
        -1,
        RoundingMode::Down,
        0x7fffffff,
        Ordering::Less,
    );
    test::<i64, i64>(
        0xffffffff,
        -1,
        RoundingMode::Up,
        0x80000000,
        Ordering::Greater,
    );
    test::<i64, i64>(
        0xffffffff,
        -1,
        RoundingMode::Floor,
        0x7fffffff,
        Ordering::Less,
    );
    test::<i64, i64>(
        0xffffffff,
        -1,
        RoundingMode::Ceiling,
        0x80000000,
        Ordering::Greater,
    );
    test::<i64, i64>(
        0xffffffff,
        -1,
        RoundingMode::Nearest,
        0x80000000,
        Ordering::Greater,
    );

    test::<u64, i64>(
        0x100000000,
        -1,
        RoundingMode::Down,
        0x80000000,
        Ordering::Equal,
    );
    test::<u64, i64>(
        0x100000000,
        -1,
        RoundingMode::Up,
        0x80000000,
        Ordering::Equal,
    );
    test::<u64, i64>(
        0x100000000,
        -1,
        RoundingMode::Floor,
        0x80000000,
        Ordering::Equal,
    );
    test::<u64, i64>(
        0x100000000,
        -1,
        RoundingMode::Ceiling,
        0x80000000,
        Ordering::Equal,
    );
    test::<u64, i64>(
        0x100000000,
        -1,
        RoundingMode::Nearest,
        0x80000000,
        Ordering::Equal,
    );
    test::<u64, i64>(
        0x100000000,
        -1,
        RoundingMode::Exact,
        0x80000000,
        Ordering::Equal,
    );

    test::<u64, i128>(
        0x100000001,
        -1,
        RoundingMode::Down,
        0x80000000,
        Ordering::Less,
    );
    test::<u64, i128>(
        0x100000001,
        -1,
        RoundingMode::Up,
        0x80000001,
        Ordering::Greater,
    );
    test::<u64, i128>(
        0x100000001,
        -1,
        RoundingMode::Floor,
        0x80000000,
        Ordering::Less,
    );
    test::<u64, i128>(
        0x100000001,
        -1,
        RoundingMode::Ceiling,
        0x80000001,
        Ordering::Greater,
    );
    test::<u64, i128>(
        0x100000001,
        -1,
        RoundingMode::Nearest,
        0x80000000,
        Ordering::Less,
    );

    test::<i64, isize>(
        1000000000000,
        0,
        RoundingMode::Down,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64, isize>(
        1000000000000,
        0,
        RoundingMode::Up,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64, isize>(
        1000000000000,
        0,
        RoundingMode::Floor,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64, isize>(
        1000000000000,
        0,
        RoundingMode::Ceiling,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64, isize>(
        1000000000000,
        0,
        RoundingMode::Nearest,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64, isize>(
        1000000000000,
        0,
        RoundingMode::Exact,
        1000000000000,
        Ordering::Equal,
    );

    test::<i128, i8>(
        7999999999999,
        -3,
        RoundingMode::Down,
        999999999999,
        Ordering::Less,
    );
    test::<i128, i8>(
        7999999999999,
        -3,
        RoundingMode::Up,
        1000000000000,
        Ordering::Greater,
    );
    test::<i128, i8>(
        7999999999999,
        -3,
        RoundingMode::Floor,
        999999999999,
        Ordering::Less,
    );
    test::<i128, i8>(
        7999999999999,
        -3,
        RoundingMode::Ceiling,
        1000000000000,
        Ordering::Greater,
    );
    test::<i128, i8>(
        7999999999999,
        -3,
        RoundingMode::Nearest,
        1000000000000,
        Ordering::Greater,
    );

    test::<u128, i16>(
        8000000000000,
        -3,
        RoundingMode::Down,
        1000000000000,
        Ordering::Equal,
    );
    test::<u128, i16>(
        8000000000000,
        -3,
        RoundingMode::Up,
        1000000000000,
        Ordering::Equal,
    );
    test::<u128, i16>(
        8000000000000,
        -3,
        RoundingMode::Floor,
        1000000000000,
        Ordering::Equal,
    );
    test::<u128, i16>(
        8000000000000,
        -3,
        RoundingMode::Ceiling,
        1000000000000,
        Ordering::Equal,
    );
    test::<u128, i16>(
        8000000000000,
        -3,
        RoundingMode::Nearest,
        1000000000000,
        Ordering::Equal,
    );
    test::<u128, i16>(
        8000000000000,
        -3,
        RoundingMode::Exact,
        1000000000000,
        Ordering::Equal,
    );

    test::<u128, i32>(
        8000000000001,
        -3,
        RoundingMode::Down,
        1000000000000,
        Ordering::Less,
    );
    test::<u128, i32>(
        8000000000001,
        -3,
        RoundingMode::Up,
        1000000000001,
        Ordering::Greater,
    );
    test::<u128, i32>(
        8000000000001,
        -3,
        RoundingMode::Floor,
        1000000000000,
        Ordering::Less,
    );
    test::<u128, i32>(
        8000000000001,
        -3,
        RoundingMode::Ceiling,
        1000000000001,
        Ordering::Greater,
    );
    test::<u128, i32>(
        8000000000001,
        -3,
        RoundingMode::Nearest,
        1000000000000,
        Ordering::Less,
    );

    test::<i128, i64>(
        1000000000000,
        -10,
        RoundingMode::Down,
        976562500,
        Ordering::Equal,
    );
    test::<i128, i64>(
        1000000000000,
        -10,
        RoundingMode::Up,
        976562500,
        Ordering::Equal,
    );
    test::<i128, i64>(
        1000000000000,
        -10,
        RoundingMode::Floor,
        976562500,
        Ordering::Equal,
    );
    test::<i128, i64>(
        1000000000000,
        -10,
        RoundingMode::Ceiling,
        976562500,
        Ordering::Equal,
    );
    test::<i128, i64>(
        1000000000000,
        -10,
        RoundingMode::Nearest,
        976562500,
        Ordering::Equal,
    );
    test::<i128, i64>(
        1000000000000,
        -10,
        RoundingMode::Exact,
        976562500,
        Ordering::Equal,
    );

    test::<u128, i128>(980657949, -72, RoundingMode::Down, 0, Ordering::Less);
    test::<u128, i128>(980657949, -72, RoundingMode::Up, 1, Ordering::Greater);
    test::<u128, i128>(980657949, -72, RoundingMode::Floor, 0, Ordering::Less);
    test::<u128, i128>(980657949, -72, RoundingMode::Ceiling, 1, Ordering::Greater);
    test::<u128, i128>(980657949, -72, RoundingMode::Nearest, 0, Ordering::Less);

    test::<i128, isize>(0xffffffff, -31, RoundingMode::Down, 1, Ordering::Less);
    test::<i128, isize>(0xffffffff, -31, RoundingMode::Up, 2, Ordering::Greater);
    test::<i128, isize>(0xffffffff, -31, RoundingMode::Floor, 1, Ordering::Less);
    test::<i128, isize>(0xffffffff, -31, RoundingMode::Ceiling, 2, Ordering::Greater);
    test::<i128, isize>(0xffffffff, -31, RoundingMode::Nearest, 2, Ordering::Greater);

    test::<u32, i128>(0xffffffff, -32, RoundingMode::Down, 0, Ordering::Less);
    test::<u32, i128>(0xffffffff, -32, RoundingMode::Up, 1, Ordering::Greater);
    test::<u32, i128>(0xffffffff, -32, RoundingMode::Floor, 0, Ordering::Less);
    test::<u32, i128>(0xffffffff, -32, RoundingMode::Ceiling, 1, Ordering::Greater);
    test::<u32, i128>(0xffffffff, -32, RoundingMode::Nearest, 1, Ordering::Greater);

    test::<u64, i8>(0x100000000, -32, RoundingMode::Down, 1, Ordering::Equal);
    test::<u64, i8>(0x100000000, -32, RoundingMode::Up, 1, Ordering::Equal);
    test::<u64, i8>(0x100000000, -32, RoundingMode::Floor, 1, Ordering::Equal);
    test::<u64, i8>(0x100000000, -32, RoundingMode::Ceiling, 1, Ordering::Equal);
    test::<u64, i8>(0x100000000, -32, RoundingMode::Nearest, 1, Ordering::Equal);
    test::<u64, i8>(0x100000000, -32, RoundingMode::Exact, 1, Ordering::Equal);

    test::<i64, i16>(0x100000000, -33, RoundingMode::Down, 0, Ordering::Less);
    test::<i64, i16>(0x100000000, -33, RoundingMode::Up, 1, Ordering::Greater);
    test::<i64, i16>(0x100000000, -33, RoundingMode::Floor, 0, Ordering::Less);
    test::<i64, i16>(
        0x100000000,
        -33,
        RoundingMode::Ceiling,
        1,
        Ordering::Greater,
    );
    test::<i64, i16>(0x100000000, -33, RoundingMode::Nearest, 0, Ordering::Less);

    test::<u8, i8>(0, 10, RoundingMode::Exact, 0, Ordering::Equal);
    test::<u8, i16>(123, 1, RoundingMode::Exact, 246, Ordering::Equal);
    test::<u16, i32>(123, 2, RoundingMode::Exact, 492, Ordering::Equal);
    test::<u64, i64>(123, 25, RoundingMode::Exact, 4127195136, Ordering::Equal);
    test::<u128, i128>(123, 26, RoundingMode::Exact, 8254390272, Ordering::Equal);
    test::<u8, isize>(123, 100, RoundingMode::Exact, 0, Ordering::Equal);

    test::<u64, i8>(
        0x80000000,
        1,
        RoundingMode::Exact,
        0x100000000,
        Ordering::Equal,
    );
    test::<i64, i16>(
        1000000000000,
        3,
        RoundingMode::Exact,
        8000000000000,
        Ordering::Equal,
    );
    test::<u64, i8>(
        1000000000000,
        24,
        RoundingMode::Exact,
        16777216000000000000,
        Ordering::Equal,
    );
    test::<i128, i16>(
        1000000000000,
        25,
        RoundingMode::Exact,
        33554432000000000000,
        Ordering::Equal,
    );
    test::<u128, i32>(
        1000000000000,
        31,
        RoundingMode::Exact,
        2147483648000000000000,
        Ordering::Equal,
    );
    test::<i128, i64>(
        1000000000000,
        32,
        RoundingMode::Exact,
        4294967296000000000000,
        Ordering::Equal,
    );
    test::<u128, i128>(
        1000000000000,
        33,
        RoundingMode::Exact,
        8589934592000000000000,
        Ordering::Equal,
    );
    test::<i64, isize>(1000000000000, 100, RoundingMode::Exact, 0, Ordering::Equal);

    test::<i8, i8>(-123, 0, RoundingMode::Down, -123, Ordering::Equal);
    test::<i8, i8>(-123, 0, RoundingMode::Up, -123, Ordering::Equal);
    test::<i8, i8>(-123, 0, RoundingMode::Floor, -123, Ordering::Equal);
    test::<i8, i8>(-123, 0, RoundingMode::Ceiling, -123, Ordering::Equal);
    test::<i8, i8>(-123, 0, RoundingMode::Nearest, -123, Ordering::Equal);
    test::<i8, i8>(-123, 0, RoundingMode::Exact, -123, Ordering::Equal);

    test::<i16, i8>(-245, -1, RoundingMode::Down, -122, Ordering::Greater);
    test::<i16, i8>(-245, -1, RoundingMode::Up, -123, Ordering::Less);
    test::<i16, i8>(-245, -1, RoundingMode::Floor, -123, Ordering::Less);
    test::<i16, i8>(-245, -1, RoundingMode::Ceiling, -122, Ordering::Greater);
    test::<i16, i8>(-245, -1, RoundingMode::Nearest, -122, Ordering::Greater);

    test::<i16, i16>(-246, -1, RoundingMode::Down, -123, Ordering::Equal);
    test::<i16, i16>(-246, -1, RoundingMode::Up, -123, Ordering::Equal);
    test::<i16, i16>(-246, -1, RoundingMode::Floor, -123, Ordering::Equal);
    test::<i16, i16>(-246, -1, RoundingMode::Ceiling, -123, Ordering::Equal);
    test::<i16, i16>(-246, -1, RoundingMode::Nearest, -123, Ordering::Equal);
    test::<i16, i16>(-246, -1, RoundingMode::Exact, -123, Ordering::Equal);

    test::<i16, i32>(-247, -1, RoundingMode::Down, -123, Ordering::Greater);
    test::<i16, i32>(-247, -1, RoundingMode::Up, -124, Ordering::Less);
    test::<i16, i32>(-247, -1, RoundingMode::Floor, -124, Ordering::Less);
    test::<i16, i32>(-247, -1, RoundingMode::Ceiling, -123, Ordering::Greater);
    test::<i16, i32>(-247, -1, RoundingMode::Nearest, -124, Ordering::Less);

    test::<i16, i64>(-491, -2, RoundingMode::Down, -122, Ordering::Greater);
    test::<i16, i64>(-491, -2, RoundingMode::Up, -123, Ordering::Less);
    test::<i16, i64>(-491, -2, RoundingMode::Floor, -123, Ordering::Less);
    test::<i16, i64>(-491, -2, RoundingMode::Ceiling, -122, Ordering::Greater);
    test::<i16, i64>(-491, -2, RoundingMode::Nearest, -123, Ordering::Less);

    test::<i16, i128>(-492, -2, RoundingMode::Down, -123, Ordering::Equal);
    test::<i16, i128>(-492, -2, RoundingMode::Up, -123, Ordering::Equal);
    test::<i16, i128>(-492, -2, RoundingMode::Floor, -123, Ordering::Equal);
    test::<i16, i128>(-492, -2, RoundingMode::Ceiling, -123, Ordering::Equal);
    test::<i16, i128>(-492, -2, RoundingMode::Nearest, -123, Ordering::Equal);
    test::<i16, i128>(-492, -2, RoundingMode::Exact, -123, Ordering::Equal);

    test::<i16, isize>(-493, -2, RoundingMode::Down, -123, Ordering::Greater);
    test::<i16, isize>(-493, -2, RoundingMode::Up, -124, Ordering::Less);
    test::<i16, isize>(-493, -2, RoundingMode::Floor, -124, Ordering::Less);
    test::<i16, isize>(-493, -2, RoundingMode::Ceiling, -123, Ordering::Greater);
    test::<i16, isize>(-493, -2, RoundingMode::Nearest, -123, Ordering::Greater);

    test::<i64, i8>(
        -4127195135,
        -25,
        RoundingMode::Down,
        -122,
        Ordering::Greater,
    );
    test::<i64, i8>(-4127195135, -25, RoundingMode::Up, -123, Ordering::Less);
    test::<i64, i8>(-4127195135, -25, RoundingMode::Floor, -123, Ordering::Less);
    test::<i64, i8>(
        -4127195135,
        -25,
        RoundingMode::Ceiling,
        -122,
        Ordering::Greater,
    );
    test::<i64, i8>(
        -4127195135,
        -25,
        RoundingMode::Nearest,
        -123,
        Ordering::Less,
    );

    test::<i64, i16>(-4127195136, -25, RoundingMode::Down, -123, Ordering::Equal);
    test::<i64, i16>(-4127195136, -25, RoundingMode::Up, -123, Ordering::Equal);
    test::<i64, i16>(-4127195136, -25, RoundingMode::Floor, -123, Ordering::Equal);
    test::<i64, i16>(
        -4127195136,
        -25,
        RoundingMode::Ceiling,
        -123,
        Ordering::Equal,
    );
    test::<i64, i16>(
        -4127195136,
        -25,
        RoundingMode::Nearest,
        -123,
        Ordering::Equal,
    );
    test::<i64, i16>(-4127195136, -25, RoundingMode::Exact, -123, Ordering::Equal);

    test::<i64, i32>(
        -4127195137,
        -25,
        RoundingMode::Down,
        -123,
        Ordering::Greater,
    );
    test::<i64, i32>(-4127195137, -25, RoundingMode::Up, -124, Ordering::Less);
    test::<i64, i32>(-4127195137, -25, RoundingMode::Floor, -124, Ordering::Less);
    test::<i64, i32>(
        -4127195137,
        -25,
        RoundingMode::Ceiling,
        -123,
        Ordering::Greater,
    );
    test::<i64, i32>(
        -4127195137,
        -25,
        RoundingMode::Nearest,
        -123,
        Ordering::Greater,
    );

    test::<i64, i64>(
        -8254390271,
        -26,
        RoundingMode::Down,
        -122,
        Ordering::Greater,
    );
    test::<i64, i64>(-8254390271, -26, RoundingMode::Up, -123, Ordering::Less);
    test::<i64, i64>(-8254390271, -26, RoundingMode::Floor, -123, Ordering::Less);
    test::<i64, i64>(
        -8254390271,
        -26,
        RoundingMode::Ceiling,
        -122,
        Ordering::Greater,
    );
    test::<i64, i64>(
        -8254390271,
        -26,
        RoundingMode::Nearest,
        -123,
        Ordering::Less,
    );

    test::<i64, i128>(-8254390272, -26, RoundingMode::Down, -123, Ordering::Equal);
    test::<i64, i128>(-8254390272, -26, RoundingMode::Up, -123, Ordering::Equal);
    test::<i64, i128>(-8254390272, -26, RoundingMode::Floor, -123, Ordering::Equal);
    test::<i64, i128>(
        -8254390272,
        -26,
        RoundingMode::Ceiling,
        -123,
        Ordering::Equal,
    );
    test::<i64, i128>(
        -8254390272,
        -26,
        RoundingMode::Nearest,
        -123,
        Ordering::Equal,
    );
    test::<i64, i128>(-8254390272, -26, RoundingMode::Exact, -123, Ordering::Equal);

    test::<i64, isize>(
        -8254390273,
        -26,
        RoundingMode::Down,
        -123,
        Ordering::Greater,
    );
    test::<i64, isize>(-8254390273, -26, RoundingMode::Up, -124, Ordering::Less);
    test::<i64, isize>(-8254390273, -26, RoundingMode::Floor, -124, Ordering::Less);
    test::<i64, isize>(
        -8254390273,
        -26,
        RoundingMode::Ceiling,
        -123,
        Ordering::Greater,
    );
    test::<i64, isize>(
        -8254390273,
        -26,
        RoundingMode::Nearest,
        -123,
        Ordering::Greater,
    );

    test::<i128, i8>(
        -0xffffffff,
        -1,
        RoundingMode::Down,
        -0x7fffffff,
        Ordering::Greater,
    );
    test::<i128, i8>(
        -0xffffffff,
        -1,
        RoundingMode::Up,
        -0x80000000,
        Ordering::Less,
    );
    test::<i128, i8>(
        -0xffffffff,
        -1,
        RoundingMode::Floor,
        -0x80000000,
        Ordering::Less,
    );
    test::<i128, i8>(
        -0xffffffff,
        -1,
        RoundingMode::Ceiling,
        -0x7fffffff,
        Ordering::Greater,
    );
    test::<i128, i8>(
        -0xffffffff,
        -1,
        RoundingMode::Nearest,
        -0x80000000,
        Ordering::Less,
    );

    test::<i128, i16>(
        -0x100000000,
        -1,
        RoundingMode::Down,
        -0x80000000,
        Ordering::Equal,
    );
    test::<i128, i16>(
        -0x100000000,
        -1,
        RoundingMode::Up,
        -0x80000000,
        Ordering::Equal,
    );
    test::<i128, i16>(
        -0x100000000,
        -1,
        RoundingMode::Floor,
        -0x80000000,
        Ordering::Equal,
    );
    test::<i128, i16>(
        -0x100000000,
        -1,
        RoundingMode::Ceiling,
        -0x80000000,
        Ordering::Equal,
    );
    test::<i128, i16>(
        -0x100000000,
        -1,
        RoundingMode::Nearest,
        -0x80000000,
        Ordering::Equal,
    );
    test::<i128, i16>(
        -0x100000000,
        -1,
        RoundingMode::Exact,
        -0x80000000,
        Ordering::Equal,
    );

    test::<i128, i32>(
        -0x100000001,
        -1,
        RoundingMode::Down,
        -0x80000000,
        Ordering::Greater,
    );
    test::<i128, i32>(
        -0x100000001,
        -1,
        RoundingMode::Up,
        -0x80000001,
        Ordering::Less,
    );
    test::<i128, i32>(
        -0x100000001,
        -1,
        RoundingMode::Floor,
        -0x80000001,
        Ordering::Less,
    );
    test::<i128, i32>(
        -0x100000001,
        -1,
        RoundingMode::Ceiling,
        -0x80000000,
        Ordering::Greater,
    );
    test::<i128, i32>(
        -0x100000001,
        -1,
        RoundingMode::Nearest,
        -0x80000000,
        Ordering::Greater,
    );

    test::<i128, i64>(
        -1000000000000,
        0,
        RoundingMode::Down,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i64>(
        -1000000000000,
        0,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i64>(
        -1000000000000,
        0,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i64>(
        -1000000000000,
        0,
        RoundingMode::Ceiling,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i64>(
        -1000000000000,
        0,
        RoundingMode::Nearest,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i64>(
        -1000000000000,
        0,
        RoundingMode::Exact,
        -1000000000000,
        Ordering::Equal,
    );

    test::<i128, i128>(
        -7999999999999,
        -3,
        RoundingMode::Down,
        -999999999999,
        Ordering::Greater,
    );
    test::<i128, i128>(
        -7999999999999,
        -3,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Less,
    );
    test::<i128, i128>(
        -7999999999999,
        -3,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Less,
    );
    test::<i128, i128>(
        -7999999999999,
        -3,
        RoundingMode::Ceiling,
        -999999999999,
        Ordering::Greater,
    );
    test::<i128, i128>(
        -7999999999999,
        -3,
        RoundingMode::Nearest,
        -1000000000000,
        Ordering::Less,
    );

    test::<i128, isize>(
        -8000000000000,
        -3,
        RoundingMode::Down,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, isize>(
        -8000000000000,
        -3,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, isize>(
        -8000000000000,
        -3,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, isize>(
        -8000000000000,
        -3,
        RoundingMode::Ceiling,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, isize>(
        -8000000000000,
        -3,
        RoundingMode::Nearest,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, isize>(
        -8000000000000,
        -3,
        RoundingMode::Exact,
        -1000000000000,
        Ordering::Equal,
    );

    test::<i64, i8>(
        -8000000000001,
        -3,
        RoundingMode::Down,
        -1000000000000,
        Ordering::Greater,
    );
    test::<i64, i8>(
        -8000000000001,
        -3,
        RoundingMode::Up,
        -1000000000001,
        Ordering::Less,
    );
    test::<i64, i8>(
        -8000000000001,
        -3,
        RoundingMode::Floor,
        -1000000000001,
        Ordering::Less,
    );
    test::<i64, i8>(
        -8000000000001,
        -3,
        RoundingMode::Ceiling,
        -1000000000000,
        Ordering::Greater,
    );
    test::<i64, i8>(
        -8000000000001,
        -3,
        RoundingMode::Nearest,
        -1000000000000,
        Ordering::Greater,
    );

    test::<i128, i16>(
        -16777216000000000000,
        -24,
        RoundingMode::Down,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i16>(
        -16777216000000000000,
        -24,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i16>(
        -16777216000000000000,
        -24,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i16>(
        -16777216000000000000,
        -24,
        RoundingMode::Ceiling,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i16>(
        -16777216000000000000,
        -24,
        RoundingMode::Nearest,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i16>(
        -16777216000000000000,
        -24,
        RoundingMode::Exact,
        -1000000000000,
        Ordering::Equal,
    );

    test::<i128, i32>(
        -33554432000000000000,
        -25,
        RoundingMode::Down,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i32>(
        -33554432000000000000,
        -25,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i32>(
        -33554432000000000000,
        -25,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i32>(
        -33554432000000000000,
        -25,
        RoundingMode::Ceiling,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i32>(
        -33554432000000000000,
        -25,
        RoundingMode::Nearest,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i32>(
        -33554432000000000000,
        -25,
        RoundingMode::Exact,
        -1000000000000,
        Ordering::Equal,
    );

    test::<i128, i64>(
        -2147483648000000000000,
        -31,
        RoundingMode::Down,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i64>(
        -2147483648000000000000,
        -31,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i64>(
        -2147483648000000000000,
        -31,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i64>(
        -2147483648000000000000,
        -31,
        RoundingMode::Ceiling,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i64>(
        -2147483648000000000000,
        -31,
        RoundingMode::Nearest,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i64>(
        -2147483648000000000000,
        -31,
        RoundingMode::Exact,
        -1000000000000,
        Ordering::Equal,
    );

    test::<i128, i128>(
        -4294967296000000000000,
        -32,
        RoundingMode::Down,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i128>(
        -4294967296000000000000,
        -32,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i128>(
        -4294967296000000000000,
        -32,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i128>(
        -4294967296000000000000,
        -32,
        RoundingMode::Ceiling,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i128>(
        -4294967296000000000000,
        -32,
        RoundingMode::Nearest,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, i128>(
        -4294967296000000000000,
        -32,
        RoundingMode::Exact,
        -1000000000000,
        Ordering::Equal,
    );

    test::<i128, isize>(
        -8589934592000000000000,
        -33,
        RoundingMode::Down,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, isize>(
        -8589934592000000000000,
        -33,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, isize>(
        -8589934592000000000000,
        -33,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, isize>(
        -8589934592000000000000,
        -33,
        RoundingMode::Ceiling,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, isize>(
        -8589934592000000000000,
        -33,
        RoundingMode::Nearest,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128, isize>(
        -8589934592000000000000,
        -33,
        RoundingMode::Exact,
        -1000000000000,
        Ordering::Equal,
    );

    test::<i64, i8>(
        -1000000000000,
        -10,
        RoundingMode::Down,
        -976562500,
        Ordering::Equal,
    );
    test::<i64, i8>(
        -1000000000000,
        -10,
        RoundingMode::Up,
        -976562500,
        Ordering::Equal,
    );
    test::<i64, i8>(
        -1000000000000,
        -10,
        RoundingMode::Floor,
        -976562500,
        Ordering::Equal,
    );
    test::<i64, i8>(
        -1000000000000,
        -10,
        RoundingMode::Ceiling,
        -976562500,
        Ordering::Equal,
    );
    test::<i64, i8>(
        -1000000000000,
        -10,
        RoundingMode::Nearest,
        -976562500,
        Ordering::Equal,
    );
    test::<i64, i8>(
        -1000000000000,
        -10,
        RoundingMode::Exact,
        -976562500,
        Ordering::Equal,
    );

    test::<i64, i16>(-980657949, -72, RoundingMode::Down, 0, Ordering::Greater);
    test::<i64, i16>(-980657949, -72, RoundingMode::Up, -1, Ordering::Less);
    test::<i64, i16>(-980657949, -72, RoundingMode::Floor, -1, Ordering::Less);
    test::<i64, i16>(-980657949, -72, RoundingMode::Ceiling, 0, Ordering::Greater);
    test::<i64, i16>(-980657949, -72, RoundingMode::Nearest, 0, Ordering::Greater);

    test::<i64, i32>(-0xffffffff, -31, RoundingMode::Down, -1, Ordering::Greater);
    test::<i64, i32>(-0xffffffff, -31, RoundingMode::Up, -2, Ordering::Less);
    test::<i64, i32>(-0xffffffff, -31, RoundingMode::Floor, -2, Ordering::Less);
    test::<i64, i32>(
        -0xffffffff,
        -31,
        RoundingMode::Ceiling,
        -1,
        Ordering::Greater,
    );
    test::<i64, i32>(-0xffffffff, -31, RoundingMode::Nearest, -2, Ordering::Less);

    test::<i64, i64>(-0xffffffff, -32, RoundingMode::Down, 0, Ordering::Greater);
    test::<i64, i64>(-0xffffffff, -32, RoundingMode::Up, -1, Ordering::Less);
    test::<i64, i64>(-0xffffffff, -32, RoundingMode::Floor, -1, Ordering::Less);
    test::<i64, i64>(
        -0xffffffff,
        -32,
        RoundingMode::Ceiling,
        0,
        Ordering::Greater,
    );
    test::<i64, i64>(-0xffffffff, -32, RoundingMode::Nearest, -1, Ordering::Less);

    test::<i64, i128>(-0x100000000, -32, RoundingMode::Down, -1, Ordering::Equal);
    test::<i64, i128>(-0x100000000, -32, RoundingMode::Up, -1, Ordering::Equal);
    test::<i64, i128>(-0x100000000, -32, RoundingMode::Floor, -1, Ordering::Equal);
    test::<i64, i128>(
        -0x100000000,
        -32,
        RoundingMode::Ceiling,
        -1,
        Ordering::Equal,
    );
    test::<i64, i128>(
        -0x100000000,
        -32,
        RoundingMode::Nearest,
        -1,
        Ordering::Equal,
    );
    test::<i64, i128>(-0x100000000, -32, RoundingMode::Exact, -1, Ordering::Equal);

    test::<i64, isize>(-0x100000000, -33, RoundingMode::Down, 0, Ordering::Greater);
    test::<i64, isize>(-0x100000000, -33, RoundingMode::Up, -1, Ordering::Less);
    test::<i64, isize>(-0x100000000, -33, RoundingMode::Floor, -1, Ordering::Less);
    test::<i64, isize>(
        -0x100000000,
        -33,
        RoundingMode::Ceiling,
        0,
        Ordering::Greater,
    );
    test::<i64, isize>(
        -0x100000000,
        -33,
        RoundingMode::Nearest,
        0,
        Ordering::Greater,
    );

    test::<i16, i8>(-123, 1, RoundingMode::Exact, -246, Ordering::Equal);
    test::<i16, i16>(-123, 2, RoundingMode::Exact, -492, Ordering::Equal);
    test::<i64, i8>(-123, 25, RoundingMode::Exact, -4127195136, Ordering::Equal);
    test::<i64, i16>(-123, 26, RoundingMode::Exact, -8254390272, Ordering::Equal);
    test::<i64, i32>(
        -0x80000000,
        1,
        RoundingMode::Exact,
        -0x100000000,
        Ordering::Equal,
    );
    test::<i64, i64>(
        -1000000000000,
        3,
        RoundingMode::Exact,
        -8000000000000,
        Ordering::Equal,
    );
    test::<i128, i128>(
        -1000000000000,
        24,
        RoundingMode::Exact,
        -16777216000000000000,
        Ordering::Equal,
    );
    test::<i128, isize>(
        -1000000000000,
        25,
        RoundingMode::Exact,
        -33554432000000000000,
        Ordering::Equal,
    );
    test::<i128, i8>(
        -1000000000000,
        31,
        RoundingMode::Exact,
        -2147483648000000000000,
        Ordering::Equal,
    );
    test::<i128, i16>(
        -1000000000000,
        32,
        RoundingMode::Exact,
        -4294967296000000000000,
        Ordering::Equal,
    );
    test::<i128, i32>(
        -1000000000000,
        33,
        RoundingMode::Exact,
        -8589934592000000000000,
        Ordering::Equal,
    );
}

fn shl_round_fail_helper<
    T: PrimitiveInt + ShlRound<U, Output = T> + ShlRoundAssign<U>,
    U: PrimitiveSigned,
>() {
    assert_panic!(T::exact_from(123).shl_round(U::NEGATIVE_ONE, RoundingMode::Exact));
    assert_panic!(T::exact_from(123).shl_round(U::exact_from(-100), RoundingMode::Exact));
    assert_panic!(T::exact_from(123).shl_round_assign(U::NEGATIVE_ONE, RoundingMode::Exact));
    assert_panic!(T::exact_from(123).shl_round_assign(U::exact_from(-100), RoundingMode::Exact));
}

#[test]
fn shl_round_fail() {
    apply_fn_to_primitive_ints_and_signeds!(shl_round_fail_helper);
}

fn shl_round_properties_helper_unsigned_signed<
    T: ArithmeticCheckedShr<U, Output = T>
        + PrimitiveUnsigned
        + ShlRound<U, Output = T>
        + ShlRoundAssign<U>
        + ShrRound<U, Output = T>,
    U: PrimitiveSigned,
>()
where
    u64: TryFrom<<U as UnsignedAbs>::Output>,
{
    unsigned_signed_rounding_mode_triple_gen_var_2::<T, U>().test_properties(|(n, i, rm)| {
        let mut mut_n = n;
        let o = mut_n.shl_round_assign(i, rm);
        let shifted = mut_n;

        assert_eq!(n.shl_round(i, rm), (shifted, o));
        if i < U::ZERO {
            assert!(shifted <= n);
        }
        if i != U::MIN {
            assert_eq!(n.shr_round(-i, rm), (shifted, o));
        }
        assert_eq!(
            i >= U::ZERO || n.divisible_by_power_of_2(u64::exact_from(i.unsigned_abs())),
            o == Ordering::Equal
        );
        if i < U::ZERO {
            if let Some(m) = shifted.arithmetic_checked_shr(i) {
                assert_eq!(m.cmp(&n), o);
            }
        }
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(n, rm)| {
        assert_eq!(n.shl_round(U::ZERO, rm), (n, Ordering::Equal));
    });

    signed_rounding_mode_pair_gen::<U>().test_properties(|(i, rm)| {
        assert_eq!(T::ZERO.shl_round(i, rm), (T::ZERO, Ordering::Equal));
    });
}

fn shl_round_properties_helper_signed_signed<
    T: ArithmeticCheckedShr<U, Output = T>
        + PrimitiveSigned
        + ShlRound<U, Output = T>
        + ShlRoundAssign<U>
        + ShrRound<U, Output = T>,
    U: PrimitiveSigned,
>()
where
    u64: TryFrom<<U as UnsignedAbs>::Output>,
{
    signed_signed_rounding_mode_triple_gen_var_4::<T, U>().test_properties(|(n, i, rm)| {
        let mut mut_n = n;
        let o = mut_n.shl_round_assign(i, rm);
        let shifted = mut_n;

        assert_eq!(n.shl_round(i, rm), (shifted, o));
        if i < U::ZERO {
            assert!(shifted.le_abs(&n));
        }
        if i != U::MIN {
            assert_eq!(n.shr_round(-i, rm), (shifted, o));
        }
        assert_eq!(
            i >= U::ZERO || n.divisible_by_power_of_2(u64::exact_from(i.unsigned_abs())),
            o == Ordering::Equal
        );
        if i < U::ZERO {
            if let Some(m) = shifted.arithmetic_checked_shr(i) {
                assert_eq!(m.cmp(&n), o);
            }
        }
    });

    signed_rounding_mode_pair_gen::<T>().test_properties(|(n, rm)| {
        assert_eq!(n.shl_round(U::ZERO, rm), (n, Ordering::Equal));
    });

    signed_rounding_mode_pair_gen::<U>().test_properties(|(i, rm)| {
        assert_eq!(T::ZERO.shl_round(i, rm), (T::ZERO, Ordering::Equal));
    });
}

#[test]
fn shl_round_properties() {
    apply_fn_to_unsigneds_and_signeds!(shl_round_properties_helper_unsigned_signed);
    apply_fn_to_signeds_and_signeds!(shl_round_properties_helper_signed_signed);
}
