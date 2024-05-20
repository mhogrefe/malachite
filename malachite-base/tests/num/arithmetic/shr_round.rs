// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ArithmeticCheckedShl, ShlRound, ShrRound, ShrRoundAssign,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::WrappingFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    signed_rounding_mode_pair_gen, signed_signed_rounding_mode_triple_gen_var_3,
    signed_unsigned_pair_gen_var_1, signed_unsigned_pair_gen_var_16,
    signed_unsigned_pair_gen_var_17, signed_unsigned_pair_gen_var_8,
    signed_unsigned_rounding_mode_triple_gen_var_2, unsigned_pair_gen_var_14,
    unsigned_pair_gen_var_2, unsigned_pair_gen_var_21, unsigned_rounding_mode_pair_gen,
    unsigned_signed_rounding_mode_triple_gen_var_1,
    unsigned_unsigned_rounding_mode_triple_gen_var_4,
};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

#[test]
fn test_shr_round() {
    fn test<T: PrimitiveInt + ShrRound<U, Output = T> + ShrRoundAssign<U>, U: PrimitiveInt>(
        t: T,
        u: U,
        rm: RoundingMode,
        out: T,
        o: Ordering,
    ) {
        assert_eq!(t.shr_round(u, rm), (out, o));

        let mut t = t;
        assert_eq!(t.shr_round_assign(u, rm), o);
        assert_eq!(t, out);
    }
    test::<u8, u8>(0, 0, Down, 0, Equal);
    test::<u8, u8>(0, 0, Up, 0, Equal);
    test::<u8, u8>(0, 0, Floor, 0, Equal);
    test::<u8, u8>(0, 0, Ceiling, 0, Equal);
    test::<u8, u8>(0, 0, Nearest, 0, Equal);
    test::<u8, u8>(0, 0, Exact, 0, Equal);

    test::<u8, i16>(0, 10, Down, 0, Equal);
    test::<u8, i16>(0, 10, Up, 0, Equal);
    test::<u8, i16>(0, 10, Floor, 0, Equal);
    test::<u8, i16>(0, 10, Ceiling, 0, Equal);
    test::<u8, i16>(0, 10, Nearest, 0, Equal);
    test::<u8, i16>(0, 10, Exact, 0, Equal);

    test::<i8, u32>(123, 0, Down, 123, Equal);
    test::<i8, u32>(123, 0, Up, 123, Equal);
    test::<i8, u32>(123, 0, Floor, 123, Equal);
    test::<i8, u32>(123, 0, Ceiling, 123, Equal);
    test::<i8, u32>(123, 0, Nearest, 123, Equal);
    test::<i8, u32>(123, 0, Exact, 123, Equal);

    test::<u8, u64>(245, 1, Down, 122, Less);
    test::<u8, u64>(245, 1, Up, 123, Greater);
    test::<u8, u64>(245, 1, Floor, 122, Less);
    test::<u8, u64>(245, 1, Ceiling, 123, Greater);
    test::<u8, u64>(245, 1, Nearest, 122, Less);

    test::<u8, u128>(246, 1, Down, 123, Equal);
    test::<u8, u128>(246, 1, Up, 123, Equal);
    test::<u8, u128>(246, 1, Floor, 123, Equal);
    test::<u8, u128>(246, 1, Ceiling, 123, Equal);
    test::<u8, u128>(246, 1, Nearest, 123, Equal);
    test::<u8, u128>(246, 1, Exact, 123, Equal);

    test::<u8, usize>(247, 1, Down, 123, Less);
    test::<u8, usize>(247, 1, Up, 124, Greater);
    test::<u8, usize>(247, 1, Floor, 123, Less);
    test::<u8, usize>(247, 1, Ceiling, 124, Greater);
    test::<u8, usize>(247, 1, Nearest, 124, Greater);

    test::<i16, i8>(491, 2, Down, 122, Less);
    test::<i16, i8>(491, 2, Up, 123, Greater);
    test::<i16, i8>(491, 2, Floor, 122, Less);
    test::<i16, i8>(491, 2, Ceiling, 123, Greater);
    test::<i16, i8>(491, 2, Nearest, 123, Greater);

    test::<u16, i16>(492, 2, Down, 123, Equal);
    test::<u16, i16>(492, 2, Up, 123, Equal);
    test::<u16, i16>(492, 2, Floor, 123, Equal);
    test::<u16, i16>(492, 2, Ceiling, 123, Equal);
    test::<u16, i16>(492, 2, Nearest, 123, Equal);
    test::<u16, i16>(492, 2, Exact, 123, Equal);

    test::<i16, u32>(493, 2, Down, 123, Less);
    test::<i16, u32>(493, 2, Up, 124, Greater);
    test::<i16, u32>(493, 2, Floor, 123, Less);
    test::<i16, u32>(493, 2, Ceiling, 124, Greater);
    test::<i16, u32>(493, 2, Nearest, 123, Less);

    test::<u32, i8>(4127195135, 25, Down, 122, Less);
    test::<u32, i8>(4127195135, 25, Up, 123, Greater);
    test::<u32, i8>(4127195135, 25, Floor, 122, Less);
    test::<u32, i8>(4127195135, 25, Ceiling, 123, Greater);
    test::<u32, i8>(4127195135, 25, Nearest, 123, Greater);

    test::<u32, u16>(4127195136, 25, Down, 123, Equal);
    test::<u32, u16>(4127195136, 25, Up, 123, Equal);
    test::<u32, u16>(4127195136, 25, Floor, 123, Equal);
    test::<u32, u16>(4127195136, 25, Ceiling, 123, Equal);
    test::<u32, u16>(4127195136, 25, Nearest, 123, Equal);
    test::<u32, u16>(4127195136, 25, Exact, 123, Equal);

    test::<u32, i32>(4127195137, 25, Down, 123, Less);
    test::<u32, i32>(4127195137, 25, Up, 124, Greater);
    test::<u32, i32>(4127195137, 25, Floor, 123, Less);
    test::<u32, i32>(4127195137, 25, Ceiling, 124, Greater);
    test::<u32, i32>(4127195137, 25, Nearest, 123, Less);

    test::<i64, u8>(8254390271, 26, Down, 122, Less);
    test::<i64, u8>(8254390271, 26, Up, 123, Greater);
    test::<i64, u8>(8254390271, 26, Floor, 122, Less);
    test::<i64, u8>(8254390271, 26, Ceiling, 123, Greater);
    test::<i64, u8>(8254390271, 26, Nearest, 123, Greater);

    test::<u64, i16>(8254390272, 26, Down, 123, Equal);
    test::<u64, i16>(8254390272, 26, Up, 123, Equal);
    test::<u64, i16>(8254390272, 26, Floor, 123, Equal);
    test::<u64, i16>(8254390272, 26, Ceiling, 123, Equal);
    test::<u64, i16>(8254390272, 26, Nearest, 123, Equal);
    test::<u64, i16>(8254390272, 26, Exact, 123, Equal);

    test::<i64, u32>(8254390273, 26, Down, 123, Less);
    test::<i64, u32>(8254390273, 26, Up, 124, Greater);
    test::<i64, u32>(8254390273, 26, Floor, 123, Less);
    test::<i64, u32>(8254390273, 26, Ceiling, 124, Greater);
    test::<i64, u32>(8254390273, 26, Nearest, 123, Less);

    test::<i64, i64>(0xffffffff, 1, Down, 0x7fffffff, Less);
    test::<i64, i64>(0xffffffff, 1, Up, 0x80000000, Greater);
    test::<i64, i64>(0xffffffff, 1, Floor, 0x7fffffff, Less);
    test::<i64, i64>(0xffffffff, 1, Ceiling, 0x80000000, Greater);
    test::<i64, i64>(0xffffffff, 1, Nearest, 0x80000000, Greater);

    test::<u64, u64>(0x100000000, 1, Down, 0x80000000, Equal);
    test::<u64, u64>(0x100000000, 1, Up, 0x80000000, Equal);
    test::<u64, u64>(0x100000000, 1, Floor, 0x80000000, Equal);
    test::<u64, u64>(0x100000000, 1, Ceiling, 0x80000000, Equal);
    test::<u64, u64>(0x100000000, 1, Nearest, 0x80000000, Equal);
    test::<u64, u64>(0x100000000, 1, Exact, 0x80000000, Equal);

    test::<u64, i128>(0x100000001, 1, Down, 0x80000000, Less);
    test::<u64, i128>(0x100000001, 1, Up, 0x80000001, Greater);
    test::<u64, i128>(0x100000001, 1, Floor, 0x80000000, Less);
    test::<u64, i128>(0x100000001, 1, Ceiling, 0x80000001, Greater);
    test::<u64, i128>(0x100000001, 1, Nearest, 0x80000000, Less);

    test::<i64, usize>(1000000000000, 0, Down, 1000000000000, Equal);
    test::<i64, usize>(1000000000000, 0, Up, 1000000000000, Equal);
    test::<i64, usize>(1000000000000, 0, Floor, 1000000000000, Equal);
    test::<i64, usize>(1000000000000, 0, Ceiling, 1000000000000, Equal);
    test::<i64, usize>(1000000000000, 0, Nearest, 1000000000000, Equal);
    test::<i64, usize>(1000000000000, 0, Exact, 1000000000000, Equal);

    test::<i128, i8>(7999999999999, 3, Down, 999999999999, Less);
    test::<i128, i8>(7999999999999, 3, Up, 1000000000000, Greater);
    test::<i128, i8>(7999999999999, 3, Floor, 999999999999, Less);
    test::<i128, i8>(7999999999999, 3, Ceiling, 1000000000000, Greater);
    test::<i128, i8>(7999999999999, 3, Nearest, 1000000000000, Greater);

    test::<u128, u16>(8000000000000, 3, Down, 1000000000000, Equal);
    test::<u128, u16>(8000000000000, 3, Up, 1000000000000, Equal);
    test::<u128, u16>(8000000000000, 3, Floor, 1000000000000, Equal);
    test::<u128, u16>(8000000000000, 3, Ceiling, 1000000000000, Equal);
    test::<u128, u16>(8000000000000, 3, Nearest, 1000000000000, Equal);
    test::<u128, u16>(8000000000000, 3, Exact, 1000000000000, Equal);

    test::<u128, i32>(8000000000001, 3, Down, 1000000000000, Less);
    test::<u128, i32>(8000000000001, 3, Up, 1000000000001, Greater);
    test::<u128, i32>(8000000000001, 3, Floor, 1000000000000, Less);
    test::<u128, i32>(8000000000001, 3, Ceiling, 1000000000001, Greater);
    test::<u128, i32>(8000000000001, 3, Nearest, 1000000000000, Less);

    test::<i128, u64>(1000000000000, 10, Down, 976562500, Equal);
    test::<i128, u64>(1000000000000, 10, Up, 976562500, Equal);
    test::<i128, u64>(1000000000000, 10, Floor, 976562500, Equal);
    test::<i128, u64>(1000000000000, 10, Ceiling, 976562500, Equal);
    test::<i128, u64>(1000000000000, 10, Nearest, 976562500, Equal);
    test::<i128, u64>(1000000000000, 10, Exact, 976562500, Equal);

    test::<u128, i128>(980657949, 72, Down, 0, Less);
    test::<u128, i128>(980657949, 72, Up, 1, Greater);
    test::<u128, i128>(980657949, 72, Floor, 0, Less);
    test::<u128, i128>(980657949, 72, Ceiling, 1, Greater);
    test::<u128, i128>(980657949, 72, Nearest, 0, Less);

    test::<i128, isize>(0xffffffff, 31, Down, 1, Less);
    test::<i128, isize>(0xffffffff, 31, Up, 2, Greater);
    test::<i128, isize>(0xffffffff, 31, Floor, 1, Less);
    test::<i128, isize>(0xffffffff, 31, Ceiling, 2, Greater);
    test::<i128, isize>(0xffffffff, 31, Nearest, 2, Greater);

    test::<u32, u128>(0xffffffff, 32, Down, 0, Less);
    test::<u32, u128>(0xffffffff, 32, Up, 1, Greater);
    test::<u32, u128>(0xffffffff, 32, Floor, 0, Less);
    test::<u32, u128>(0xffffffff, 32, Ceiling, 1, Greater);
    test::<u32, u128>(0xffffffff, 32, Nearest, 1, Greater);

    test::<u64, i8>(0x100000000, 32, Down, 1, Equal);
    test::<u64, i8>(0x100000000, 32, Up, 1, Equal);
    test::<u64, i8>(0x100000000, 32, Floor, 1, Equal);
    test::<u64, i8>(0x100000000, 32, Ceiling, 1, Equal);
    test::<u64, i8>(0x100000000, 32, Nearest, 1, Equal);
    test::<u64, i8>(0x100000000, 32, Exact, 1, Equal);

    test::<i64, u16>(0x100000000, 33, Down, 0, Less);
    test::<i64, u16>(0x100000000, 33, Up, 1, Greater);
    test::<i64, u16>(0x100000000, 33, Floor, 0, Less);
    test::<i64, u16>(0x100000000, 33, Ceiling, 1, Greater);
    test::<i64, u16>(0x100000000, 33, Nearest, 0, Less);

    test::<u8, i8>(0, -10, Exact, 0, Equal);
    test::<u8, i16>(123, -1, Exact, 246, Equal);
    test::<u16, i32>(123, -2, Exact, 492, Equal);
    test::<u64, i64>(123, -25, Exact, 4127195136, Equal);
    test::<u128, i128>(123, -26, Exact, 8254390272, Equal);
    test::<u8, isize>(123, -100, Exact, 0, Equal);

    test::<u64, i8>(0x80000000, -1, Exact, 0x100000000, Equal);
    test::<i64, i16>(1000000000000, -3, Exact, 8000000000000, Equal);
    test::<u64, i8>(1000000000000, -24, Exact, 16777216000000000000, Equal);
    test::<i128, i16>(1000000000000, -25, Exact, 33554432000000000000, Equal);
    test::<u128, i32>(1000000000000, -31, Exact, 2147483648000000000000, Equal);
    test::<i128, i64>(1000000000000, -32, Exact, 4294967296000000000000, Equal);
    test::<u128, i128>(1000000000000, -33, Exact, 8589934592000000000000, Equal);
    test::<i64, isize>(1000000000000, -100, Exact, 0, Equal);

    test::<i8, u8>(-123, 0, Down, -123, Equal);
    test::<i8, u8>(-123, 0, Up, -123, Equal);
    test::<i8, u8>(-123, 0, Floor, -123, Equal);
    test::<i8, u8>(-123, 0, Ceiling, -123, Equal);
    test::<i8, u8>(-123, 0, Nearest, -123, Equal);
    test::<i8, u8>(-123, 0, Exact, -123, Equal);

    test::<i16, i8>(-245, 1, Down, -122, Greater);
    test::<i16, i8>(-245, 1, Up, -123, Less);
    test::<i16, i8>(-245, 1, Floor, -123, Less);
    test::<i16, i8>(-245, 1, Ceiling, -122, Greater);
    test::<i16, i8>(-245, 1, Nearest, -122, Greater);

    test::<i16, u16>(-246, 1, Down, -123, Equal);
    test::<i16, u16>(-246, 1, Up, -123, Equal);
    test::<i16, u16>(-246, 1, Floor, -123, Equal);
    test::<i16, u16>(-246, 1, Ceiling, -123, Equal);
    test::<i16, u16>(-246, 1, Nearest, -123, Equal);
    test::<i16, u16>(-246, 1, Exact, -123, Equal);

    test::<i16, i32>(-247, 1, Down, -123, Greater);
    test::<i16, i32>(-247, 1, Up, -124, Less);
    test::<i16, i32>(-247, 1, Floor, -124, Less);
    test::<i16, i32>(-247, 1, Ceiling, -123, Greater);
    test::<i16, i32>(-247, 1, Nearest, -124, Less);

    test::<i16, u64>(-491, 2, Down, -122, Greater);
    test::<i16, u64>(-491, 2, Up, -123, Less);
    test::<i16, u64>(-491, 2, Floor, -123, Less);
    test::<i16, u64>(-491, 2, Ceiling, -122, Greater);
    test::<i16, u64>(-491, 2, Nearest, -123, Less);

    test::<i16, i128>(-492, 2, Down, -123, Equal);
    test::<i16, i128>(-492, 2, Up, -123, Equal);
    test::<i16, i128>(-492, 2, Floor, -123, Equal);
    test::<i16, i128>(-492, 2, Ceiling, -123, Equal);
    test::<i16, i128>(-492, 2, Nearest, -123, Equal);
    test::<i16, i128>(-492, 2, Exact, -123, Equal);

    test::<i16, usize>(-493, 2, Down, -123, Greater);
    test::<i16, usize>(-493, 2, Up, -124, Less);
    test::<i16, usize>(-493, 2, Floor, -124, Less);
    test::<i16, usize>(-493, 2, Ceiling, -123, Greater);
    test::<i16, usize>(-493, 2, Nearest, -123, Greater);

    test::<i64, i8>(-4127195135, 25, Down, -122, Greater);
    test::<i64, i8>(-4127195135, 25, Up, -123, Less);
    test::<i64, i8>(-4127195135, 25, Floor, -123, Less);
    test::<i64, i8>(-4127195135, 25, Ceiling, -122, Greater);
    test::<i64, i8>(-4127195135, 25, Nearest, -123, Less);

    test::<i64, u16>(-4127195136, 25, Down, -123, Equal);
    test::<i64, u16>(-4127195136, 25, Up, -123, Equal);
    test::<i64, u16>(-4127195136, 25, Floor, -123, Equal);
    test::<i64, u16>(-4127195136, 25, Ceiling, -123, Equal);
    test::<i64, u16>(-4127195136, 25, Nearest, -123, Equal);
    test::<i64, u16>(-4127195136, 25, Exact, -123, Equal);

    test::<i64, i32>(-4127195137, 25, Down, -123, Greater);
    test::<i64, i32>(-4127195137, 25, Up, -124, Less);
    test::<i64, i32>(-4127195137, 25, Floor, -124, Less);
    test::<i64, i32>(-4127195137, 25, Ceiling, -123, Greater);
    test::<i64, i32>(-4127195137, 25, Nearest, -123, Greater);

    test::<i64, u64>(-8254390271, 26, Down, -122, Greater);
    test::<i64, u64>(-8254390271, 26, Up, -123, Less);
    test::<i64, u64>(-8254390271, 26, Floor, -123, Less);
    test::<i64, u64>(-8254390271, 26, Ceiling, -122, Greater);
    test::<i64, u64>(-8254390271, 26, Nearest, -123, Less);

    test::<i64, i128>(-8254390272, 26, Down, -123, Equal);
    test::<i64, i128>(-8254390272, 26, Up, -123, Equal);
    test::<i64, i128>(-8254390272, 26, Floor, -123, Equal);
    test::<i64, i128>(-8254390272, 26, Ceiling, -123, Equal);
    test::<i64, i128>(-8254390272, 26, Nearest, -123, Equal);
    test::<i64, i128>(-8254390272, 26, Exact, -123, Equal);

    test::<i64, usize>(-8254390273, 26, Down, -123, Greater);
    test::<i64, usize>(-8254390273, 26, Up, -124, Less);
    test::<i64, usize>(-8254390273, 26, Floor, -124, Less);
    test::<i64, usize>(-8254390273, 26, Ceiling, -123, Greater);
    test::<i64, usize>(-8254390273, 26, Nearest, -123, Greater);

    test::<i128, i8>(-0xffffffff, 1, Down, -0x7fffffff, Greater);
    test::<i128, i8>(-0xffffffff, 1, Up, -0x80000000, Less);
    test::<i128, i8>(-0xffffffff, 1, Floor, -0x80000000, Less);
    test::<i128, i8>(-0xffffffff, 1, Ceiling, -0x7fffffff, Greater);
    test::<i128, i8>(-0xffffffff, 1, Nearest, -0x80000000, Less);

    test::<i128, u16>(-0x100000000, 1, Down, -0x80000000, Equal);
    test::<i128, u16>(-0x100000000, 1, Up, -0x80000000, Equal);
    test::<i128, u16>(-0x100000000, 1, Floor, -0x80000000, Equal);
    test::<i128, u16>(-0x100000000, 1, Ceiling, -0x80000000, Equal);
    test::<i128, u16>(-0x100000000, 1, Nearest, -0x80000000, Equal);
    test::<i128, u16>(-0x100000000, 1, Exact, -0x80000000, Equal);

    test::<i128, i32>(-0x100000001, 1, Down, -0x80000000, Greater);
    test::<i128, i32>(-0x100000001, 1, Up, -0x80000001, Less);
    test::<i128, i32>(-0x100000001, 1, Floor, -0x80000001, Less);
    test::<i128, i32>(-0x100000001, 1, Ceiling, -0x80000000, Greater);
    test::<i128, i32>(-0x100000001, 1, Nearest, -0x80000000, Greater);

    test::<i128, u64>(-1000000000000, 0, Down, -1000000000000, Equal);
    test::<i128, u64>(-1000000000000, 0, Up, -1000000000000, Equal);
    test::<i128, u64>(-1000000000000, 0, Floor, -1000000000000, Equal);
    test::<i128, u64>(-1000000000000, 0, Ceiling, -1000000000000, Equal);
    test::<i128, u64>(-1000000000000, 0, Nearest, -1000000000000, Equal);
    test::<i128, u64>(-1000000000000, 0, Exact, -1000000000000, Equal);

    test::<i128, i128>(-7999999999999, 3, Down, -999999999999, Greater);
    test::<i128, i128>(-7999999999999, 3, Up, -1000000000000, Less);
    test::<i128, i128>(-7999999999999, 3, Floor, -1000000000000, Less);
    test::<i128, i128>(-7999999999999, 3, Ceiling, -999999999999, Greater);
    test::<i128, i128>(-7999999999999, 3, Nearest, -1000000000000, Less);

    test::<i128, usize>(-8000000000000, 3, Down, -1000000000000, Equal);
    test::<i128, usize>(-8000000000000, 3, Up, -1000000000000, Equal);
    test::<i128, usize>(-8000000000000, 3, Floor, -1000000000000, Equal);
    test::<i128, usize>(-8000000000000, 3, Ceiling, -1000000000000, Equal);
    test::<i128, usize>(-8000000000000, 3, Nearest, -1000000000000, Equal);
    test::<i128, usize>(-8000000000000, 3, Exact, -1000000000000, Equal);

    test::<i64, i8>(-8000000000001, 3, Down, -1000000000000, Greater);
    test::<i64, i8>(-8000000000001, 3, Up, -1000000000001, Less);
    test::<i64, i8>(-8000000000001, 3, Floor, -1000000000001, Less);
    test::<i64, i8>(-8000000000001, 3, Ceiling, -1000000000000, Greater);
    test::<i64, i8>(-8000000000001, 3, Nearest, -1000000000000, Greater);

    test::<i128, u16>(-16777216000000000000, 24, Down, -1000000000000, Equal);
    test::<i128, u16>(-16777216000000000000, 24, Up, -1000000000000, Equal);
    test::<i128, u16>(-16777216000000000000, 24, Floor, -1000000000000, Equal);
    test::<i128, u16>(-16777216000000000000, 24, Ceiling, -1000000000000, Equal);
    test::<i128, u16>(-16777216000000000000, 24, Nearest, -1000000000000, Equal);
    test::<i128, u16>(-16777216000000000000, 24, Exact, -1000000000000, Equal);

    test::<i128, i32>(-33554432000000000000, 25, Down, -1000000000000, Equal);
    test::<i128, i32>(-33554432000000000000, 25, Up, -1000000000000, Equal);
    test::<i128, i32>(-33554432000000000000, 25, Floor, -1000000000000, Equal);
    test::<i128, i32>(-33554432000000000000, 25, Ceiling, -1000000000000, Equal);
    test::<i128, i32>(-33554432000000000000, 25, Nearest, -1000000000000, Equal);
    test::<i128, i32>(-33554432000000000000, 25, Exact, -1000000000000, Equal);

    test::<i128, u64>(-2147483648000000000000, 31, Down, -1000000000000, Equal);
    test::<i128, u64>(-2147483648000000000000, 31, Up, -1000000000000, Equal);
    test::<i128, u64>(-2147483648000000000000, 31, Floor, -1000000000000, Equal);
    test::<i128, u64>(-2147483648000000000000, 31, Ceiling, -1000000000000, Equal);
    test::<i128, u64>(-2147483648000000000000, 31, Nearest, -1000000000000, Equal);
    test::<i128, u64>(-2147483648000000000000, 31, Exact, -1000000000000, Equal);

    test::<i128, i128>(-4294967296000000000000, 32, Down, -1000000000000, Equal);
    test::<i128, i128>(-4294967296000000000000, 32, Up, -1000000000000, Equal);
    test::<i128, i128>(-4294967296000000000000, 32, Floor, -1000000000000, Equal);
    test::<i128, i128>(-4294967296000000000000, 32, Ceiling, -1000000000000, Equal);
    test::<i128, i128>(-4294967296000000000000, 32, Nearest, -1000000000000, Equal);
    test::<i128, i128>(-4294967296000000000000, 32, Exact, -1000000000000, Equal);

    test::<i128, usize>(-8589934592000000000000, 33, Down, -1000000000000, Equal);
    test::<i128, usize>(-8589934592000000000000, 33, Up, -1000000000000, Equal);
    test::<i128, usize>(-8589934592000000000000, 33, Floor, -1000000000000, Equal);
    test::<i128, usize>(-8589934592000000000000, 33, Ceiling, -1000000000000, Equal);
    test::<i128, usize>(-8589934592000000000000, 33, Nearest, -1000000000000, Equal);
    test::<i128, usize>(-8589934592000000000000, 33, Exact, -1000000000000, Equal);

    test::<i64, i8>(-1000000000000, 10, Down, -976562500, Equal);
    test::<i64, i8>(-1000000000000, 10, Up, -976562500, Equal);
    test::<i64, i8>(-1000000000000, 10, Floor, -976562500, Equal);
    test::<i64, i8>(-1000000000000, 10, Ceiling, -976562500, Equal);
    test::<i64, i8>(-1000000000000, 10, Nearest, -976562500, Equal);
    test::<i64, i8>(-1000000000000, 10, Exact, -976562500, Equal);

    test::<i64, u16>(-980657949, 72, Down, 0, Greater);
    test::<i64, u16>(-980657949, 72, Up, -1, Less);
    test::<i64, u16>(-980657949, 72, Floor, -1, Less);
    test::<i64, u16>(-980657949, 72, Ceiling, 0, Greater);
    test::<i64, u16>(-980657949, 72, Nearest, 0, Greater);

    test::<i64, i32>(-0xffffffff, 31, Down, -1, Greater);
    test::<i64, i32>(-0xffffffff, 31, Up, -2, Less);
    test::<i64, i32>(-0xffffffff, 31, Floor, -2, Less);
    test::<i64, i32>(-0xffffffff, 31, Ceiling, -1, Greater);
    test::<i64, i32>(-0xffffffff, 31, Nearest, -2, Less);

    test::<i64, u64>(-0xffffffff, 32, Down, 0, Greater);
    test::<i64, u64>(-0xffffffff, 32, Up, -1, Less);
    test::<i64, u64>(-0xffffffff, 32, Floor, -1, Less);
    test::<i64, u64>(-0xffffffff, 32, Ceiling, 0, Greater);
    test::<i64, u64>(-0xffffffff, 32, Nearest, -1, Less);

    test::<i64, i128>(-0x100000000, 32, Down, -1, Equal);
    test::<i64, i128>(-0x100000000, 32, Up, -1, Equal);
    test::<i64, i128>(-0x100000000, 32, Floor, -1, Equal);
    test::<i64, i128>(-0x100000000, 32, Ceiling, -1, Equal);
    test::<i64, i128>(-0x100000000, 32, Nearest, -1, Equal);
    test::<i64, i128>(-0x100000000, 32, Exact, -1, Equal);

    test::<i64, usize>(-0x100000000, 33, Down, 0, Greater);
    test::<i64, usize>(-0x100000000, 33, Up, -1, Less);
    test::<i64, usize>(-0x100000000, 33, Floor, -1, Less);
    test::<i64, usize>(-0x100000000, 33, Ceiling, 0, Greater);
    test::<i64, usize>(-0x100000000, 33, Nearest, 0, Greater);

    test::<i16, i8>(-123, -1, Exact, -246, Equal);
    test::<i16, i16>(-123, -2, Exact, -492, Equal);
    test::<i64, i8>(-123, -25, Exact, -4127195136, Equal);
    test::<i64, i16>(-123, -26, Exact, -8254390272, Equal);
    test::<i64, i32>(-0x80000000, -1, Exact, -0x100000000, Equal);
    test::<i64, i64>(-1000000000000, -3, Exact, -8000000000000, Equal);
    test::<i128, i128>(-1000000000000, -24, Exact, -16777216000000000000, Equal);
    test::<i128, isize>(-1000000000000, -25, Exact, -33554432000000000000, Equal);
    test::<i128, i8>(-1000000000000, -31, Exact, -2147483648000000000000, Equal);
    test::<i128, i16>(-1000000000000, -32, Exact, -4294967296000000000000, Equal);
    test::<i128, i32>(-1000000000000, -33, Exact, -8589934592000000000000, Equal);
}

fn shr_round_fail_helper<
    T: PrimitiveInt + ShrRound<U, Output = T> + ShrRoundAssign<U>,
    U: PrimitiveInt,
>() {
    assert_panic!(T::exact_from(123).shr_round(U::ONE, Exact));
    assert_panic!(T::exact_from(123).shr_round(U::exact_from(100), Exact));
    assert_panic!(T::exact_from(123).shr_round_assign(U::ONE, Exact));
    assert_panic!(T::exact_from(123).shr_round_assign(U::exact_from(100), Exact));
}

#[test]
fn shr_round_fail() {
    apply_fn_to_primitive_ints_and_primitive_ints!(shr_round_fail_helper);
}

fn shr_round_properties_helper_unsigned_unsigned<
    T: ArithmeticCheckedShl<U, Output = T>
        + PrimitiveUnsigned
        + ShrRound<U, Output = T>
        + ShrRoundAssign<U>,
    U: PrimitiveUnsigned,
>() {
    unsigned_unsigned_rounding_mode_triple_gen_var_4::<T, U>().test_properties(|(n, u, rm)| {
        let mut mut_n = n;
        let o = mut_n.shr_round_assign(u, rm);
        let shifted = mut_n;

        assert_eq!(n.shr_round(u, rm), (shifted, o));
        assert!(shifted <= n);
        assert_eq!(n.divisible_by_power_of_2(u.exact_into()), o == Equal);
        if let Some(m) = shifted.arithmetic_checked_shl(u) {
            assert_eq!(m.cmp(&n), o);
        }
    });

    unsigned_pair_gen_var_2::<T, U>().test_properties(|(n, u)| {
        if u < U::exact_from(T::WIDTH) {
            let no = (n, Equal);
            if let Some(shifted) = n.arithmetic_checked_shl(u) {
                assert_eq!(shifted.shr_round(u, Down), no);
                assert_eq!(shifted.shr_round(u, Up), no);
                assert_eq!(shifted.shr_round(u, Floor), no);
                assert_eq!(shifted.shr_round(u, Ceiling), no);
                assert_eq!(shifted.shr_round(u, Nearest), no);
                assert_eq!(shifted.shr_round(u, Exact), no);
            }
        }
    });

    unsigned_pair_gen_var_14::<T, U>().test_properties(|(n, u)| {
        let down = n.shr_round(u, Down);
        assert_eq!(down.1, Less);
        if let Some(up) = down.0.checked_add(T::ONE) {
            let up = (up, Greater);
            assert_eq!(n.shr_round(u, Up), up);
            assert_eq!(n.shr_round(u, Floor), down);
            assert_eq!(n.shr_round(u, Ceiling), up);
            let nearest = n.shr_round(u, Nearest);
            assert!(nearest == down || nearest == up);
        }
    });

    unsigned_pair_gen_var_21::<T, U>().test_properties(|(t, u)| {
        if let Some(shift) = u.checked_add(U::exact_from(T::WIDTH)) {
            assert_eq!(t.shr_round(shift, Down), (T::ZERO, Less));
            assert_eq!(t.shr_round(shift, Floor), (T::ZERO, Less));
            assert_eq!(t.shr_round(shift, Up), (T::ONE, Greater));
            assert_eq!(t.shr_round(shift, Ceiling), (T::ONE, Greater));
            if let Some(extra_shift) = shift.checked_add(U::ONE) {
                assert_eq!(t.shr_round(extra_shift, Nearest), (T::ZERO, Less));
            }
        }
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(n, rm)| {
        assert_eq!(n.shr_round(U::ZERO, rm), (n, Equal));
    });

    unsigned_rounding_mode_pair_gen::<U>().test_properties(|(u, rm)| {
        assert_eq!(T::ZERO.shr_round(u, rm), (T::ZERO, Equal));
    });
}

fn shr_round_properties_helper_unsigned_signed<
    T: ArithmeticCheckedShl<U, Output = T>
        + PrimitiveUnsigned
        + ShlRound<U, Output = T>
        + ShrRound<U, Output = T>
        + ShrRoundAssign<U>,
    U: PrimitiveSigned,
>() {
    unsigned_signed_rounding_mode_triple_gen_var_1::<T, U>().test_properties(|(n, i, rm)| {
        let mut mut_n = n;
        let o = mut_n.shr_round_assign(i, rm);
        let shifted = mut_n;

        assert_eq!(n.shr_round(i, rm), (shifted, o));
        if i >= U::ZERO {
            assert!(shifted <= n);
        }
        if i != U::MIN {
            assert_eq!(n.shl_round(-i, rm), (shifted, o));
        }
        assert_eq!(
            i <= U::ZERO || n.divisible_by_power_of_2(i.exact_into()),
            o == Equal
        );
        if i >= U::ZERO {
            if let Some(m) = shifted.arithmetic_checked_shl(i) {
                assert_eq!(m.cmp(&n), o);
            }
        }
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(n, rm)| {
        assert_eq!(n.shr_round(U::ZERO, rm), (n, Equal));
    });

    signed_rounding_mode_pair_gen::<U>().test_properties(|(i, rm)| {
        assert_eq!(T::ZERO.shr_round(i, rm), (T::ZERO, Equal));
    });
}

fn shr_round_properties_helper_signed_unsigned<
    V: PrimitiveUnsigned,
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: ArithmeticCheckedShl<V, Output = S>
        + PrimitiveSigned
        + ShrRound<V, Output = S>
        + ShrRoundAssign<V>
        + WrappingFrom<U>,
>() {
    signed_unsigned_rounding_mode_triple_gen_var_2::<S, V>().test_properties(|(n, u, rm)| {
        let mut mut_n = n;
        let o = mut_n.shr_round_assign(u, rm);
        let shifted = mut_n;
        assert_eq!(n.shr_round(u, rm), (shifted, o));

        assert!(shifted.le_abs(&n));
        if n != S::MIN {
            let (x, o_alt) = (-n).shr_round(u, -rm);
            if x != S::MIN {
                assert_eq!(-x, shifted);
            }
            assert_eq!(o_alt, o.reverse());
        }
        assert_eq!(n.divisible_by_power_of_2(u.exact_into()), o == Equal);
        if let Some(m) = shifted.arithmetic_checked_shl(u) {
            assert_eq!(m.cmp(&n), o);
        }
    });

    signed_unsigned_pair_gen_var_1::<S, V>().test_properties(|(n, u)| {
        if u < V::exact_from(S::WIDTH) {
            let no = (n, Equal);
            if let Some(shifted) = n.arithmetic_checked_shl(u) {
                assert_eq!(shifted.shr_round(u, Down), no);
                assert_eq!(shifted.shr_round(u, Up), no);
                assert_eq!(shifted.shr_round(u, Floor), no);
                assert_eq!(shifted.shr_round(u, Ceiling), no);
                assert_eq!(shifted.shr_round(u, Nearest), no);
                assert_eq!(shifted.shr_round(u, Exact), no);
            }
        }
    });

    signed_unsigned_pair_gen_var_8::<S, V>().test_properties(|(n, u)| {
        let floor = n.shr_round(u, Floor);
        assert_eq!(floor.1, Less);
        if let Some(ceiling) = floor.0.checked_add(S::ONE) {
            let ceiling = (ceiling, Greater);
            assert_eq!(n.shr_round(u, Ceiling), ceiling);
            if n >= S::ZERO {
                assert_eq!(n.shr_round(u, Up), ceiling);
                assert_eq!(n.shr_round(u, Down), floor);
            } else {
                assert_eq!(n.shr_round(u, Up), floor);
                assert_eq!(n.shr_round(u, Down), ceiling);
            }
            let nearest = n.shr_round(u, Nearest);
            assert!(nearest == floor || nearest == ceiling);
        }
    });

    signed_unsigned_pair_gen_var_16::<S, V>().test_properties(|(i, u)| {
        if let Some(shift) = u.checked_add(V::exact_from(S::WIDTH - 1)) {
            assert_eq!(i.shr_round(shift, Down), (S::ZERO, Less));
            assert_eq!(i.shr_round(shift, Floor), (S::ZERO, Less));
            assert_eq!(i.shr_round(shift, Up), (S::ONE, Greater));
            assert_eq!(i.shr_round(shift, Ceiling), (S::ONE, Greater));
            if let Some(extra_shift) = shift.checked_add(V::ONE) {
                assert_eq!(i.shr_round(extra_shift, Nearest), (S::ZERO, Less));
            }
        }
    });

    signed_unsigned_pair_gen_var_17::<U, S, V>().test_properties(|(i, u)| {
        if let Some(shift) = u.checked_add(V::exact_from(S::WIDTH - 1)) {
            assert_eq!(i.shr_round(shift, Down), (S::ZERO, Greater));
            assert_eq!(i.shr_round(shift, Floor), (S::NEGATIVE_ONE, Less));
            assert_eq!(i.shr_round(shift, Up), (S::NEGATIVE_ONE, Less));
            assert_eq!(i.shr_round(shift, Ceiling), (S::ZERO, Greater));
            if let Some(extra_shift) = shift.checked_add(V::ONE) {
                assert_eq!(i.shr_round(extra_shift, Nearest), (S::ZERO, Greater));
            }
        }
    });

    signed_rounding_mode_pair_gen::<S>().test_properties(|(n, rm)| {
        assert_eq!(n.shr_round(V::ZERO, rm), (n, Equal));
    });

    unsigned_rounding_mode_pair_gen::<V>().test_properties(|(u, rm)| {
        assert_eq!(S::ZERO.shr_round(u, rm), (S::ZERO, Equal));
    });
}

fn shr_round_properties_helper_signed_signed<
    T: ArithmeticCheckedShl<U, Output = T>
        + PrimitiveSigned
        + ShlRound<U, Output = T>
        + ShrRound<U, Output = T>
        + ShrRoundAssign<U>,
    U: PrimitiveSigned,
>() {
    signed_signed_rounding_mode_triple_gen_var_3::<T, U>().test_properties(|(n, i, rm)| {
        let mut mut_n = n;
        let o = mut_n.shr_round_assign(i, rm);
        let shifted = mut_n;

        assert_eq!(n.shr_round(i, rm), (shifted, o));
        if i >= U::ZERO {
            assert!(shifted.le_abs(&n));
        }
        if i != U::MIN {
            assert_eq!(n.shl_round(-i, rm), (shifted, o));
        }
        assert_eq!(
            i <= U::ZERO || n.divisible_by_power_of_2(i.exact_into()),
            o == Equal
        );
        if i >= U::ZERO {
            if let Some(m) = shifted.arithmetic_checked_shl(i) {
                assert_eq!(m.cmp(&n), o);
            }
        }
    });

    signed_rounding_mode_pair_gen::<T>().test_properties(|(n, rm)| {
        assert_eq!(n.shr_round(U::ZERO, rm), (n, Equal));
    });

    signed_rounding_mode_pair_gen::<U>().test_properties(|(i, rm)| {
        assert_eq!(T::ZERO.shr_round(i, rm), (T::ZERO, Equal));
    });
}

#[test]
fn shr_round_properties() {
    apply_fn_to_unsigneds_and_unsigneds!(shr_round_properties_helper_unsigned_unsigned);
    apply_fn_to_unsigneds_and_signeds!(shr_round_properties_helper_unsigned_signed);
    apply_fn_to_unsigneds_and_unsigned_signed_pairs!(shr_round_properties_helper_signed_unsigned);
    apply_fn_to_signeds_and_signeds!(shr_round_properties_helper_signed_signed);
}
