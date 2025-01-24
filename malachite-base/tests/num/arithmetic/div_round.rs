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
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    signed_pair_gen_var_3, signed_pair_gen_var_5, signed_rounding_mode_pair_gen,
    signed_rounding_mode_pair_gen_var_1, signed_rounding_mode_pair_gen_var_2,
    signed_rounding_mode_pair_gen_var_3, signed_signed_rounding_mode_triple_gen_var_1,
    unsigned_pair_gen_var_11, unsigned_pair_gen_var_12, unsigned_pair_gen_var_13,
    unsigned_rounding_mode_pair_gen, unsigned_rounding_mode_pair_gen_var_1,
    unsigned_unsigned_rounding_mode_triple_gen_var_1,
};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

#[test]
fn test_div_round_unsigned() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, rm: RoundingMode, q: T, o: Ordering) {
        assert_eq!(n.div_round(d, rm), (q, o));

        let mut mut_n = n;
        assert_eq!(mut_n.div_round_assign(d, rm), o);
        assert_eq!(mut_n, q);
    }
    test::<u8>(0, 1, Down, 0, Equal);
    test::<u8>(0, 1, Floor, 0, Equal);
    test::<u8>(0, 1, Up, 0, Equal);
    test::<u8>(0, 1, Ceiling, 0, Equal);
    test::<u8>(0, 1, Nearest, 0, Equal);
    test::<u8>(0, 1, Exact, 0, Equal);

    test::<u16>(0, 123, Down, 0, Equal);
    test::<u16>(0, 123, Floor, 0, Equal);
    test::<u16>(0, 123, Up, 0, Equal);
    test::<u16>(0, 123, Ceiling, 0, Equal);
    test::<u16>(0, 123, Nearest, 0, Equal);
    test::<u16>(0, 123, Exact, 0, Equal);

    test::<u32>(1, 1, Down, 1, Equal);
    test::<u32>(1, 1, Floor, 1, Equal);
    test::<u32>(1, 1, Up, 1, Equal);
    test::<u32>(1, 1, Ceiling, 1, Equal);
    test::<u32>(1, 1, Nearest, 1, Equal);
    test::<u32>(1, 1, Exact, 1, Equal);

    test::<u64>(123, 1, Down, 123, Equal);
    test::<u64>(123, 1, Floor, 123, Equal);
    test::<u64>(123, 1, Up, 123, Equal);
    test::<u64>(123, 1, Ceiling, 123, Equal);
    test::<u64>(123, 1, Nearest, 123, Equal);
    test::<u64>(123, 1, Exact, 123, Equal);

    test::<u128>(123, 2, Down, 61, Less);
    test::<u128>(123, 2, Floor, 61, Less);
    test::<u128>(123, 2, Up, 62, Greater);
    test::<u128>(123, 2, Ceiling, 62, Greater);
    test::<u128>(123, 2, Nearest, 62, Greater);

    test::<usize>(125, 2, Down, 62, Less);
    test::<usize>(125, 2, Floor, 62, Less);
    test::<usize>(125, 2, Up, 63, Greater);
    test::<usize>(125, 2, Ceiling, 63, Greater);
    test::<usize>(125, 2, Nearest, 62, Less);

    test::<u8>(123, 123, Down, 1, Equal);
    test::<u8>(123, 123, Floor, 1, Equal);
    test::<u8>(123, 123, Up, 1, Equal);
    test::<u8>(123, 123, Ceiling, 1, Equal);
    test::<u8>(123, 123, Nearest, 1, Equal);
    test::<u8>(123, 123, Exact, 1, Equal);

    test::<u16>(123, 456, Down, 0, Less);
    test::<u16>(123, 456, Floor, 0, Less);
    test::<u16>(123, 456, Up, 1, Greater);
    test::<u16>(123, 456, Ceiling, 1, Greater);
    test::<u16>(123, 456, Nearest, 0, Less);

    test::<u64>(1000000000000, 1, Down, 1000000000000, Equal);
    test::<u64>(1000000000000, 1, Floor, 1000000000000, Equal);
    test::<u64>(1000000000000, 1, Up, 1000000000000, Equal);
    test::<u64>(1000000000000, 1, Ceiling, 1000000000000, Equal);
    test::<u64>(1000000000000, 1, Nearest, 1000000000000, Equal);
    test::<u64>(1000000000000, 1, Exact, 1000000000000, Equal);

    test::<u64>(1000000000000, 3, Down, 333333333333, Less);
    test::<u64>(1000000000000, 3, Floor, 333333333333, Less);
    test::<u64>(1000000000000, 3, Up, 333333333334, Greater);
    test::<u64>(1000000000000, 3, Ceiling, 333333333334, Greater);
    test::<u64>(1000000000000, 3, Nearest, 333333333333, Less);

    test::<u64>(999999999999, 2, Down, 499999999999, Less);
    test::<u64>(999999999999, 2, Floor, 499999999999, Less);
    test::<u64>(999999999999, 2, Up, 500000000000, Greater);
    test::<u64>(999999999999, 2, Ceiling, 500000000000, Greater);
    test::<u64>(999999999999, 2, Nearest, 500000000000, Greater);

    test::<u64>(1000000000001, 2, Down, 500000000000, Less);
    test::<u64>(1000000000001, 2, Floor, 500000000000, Less);
    test::<u64>(1000000000001, 2, Up, 500000000001, Greater);
    test::<u64>(1000000000001, 2, Ceiling, 500000000001, Greater);
    test::<u64>(1000000000001, 2, Nearest, 500000000000, Less);

    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        Down,
        232830643708079,
        Less,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        Floor,
        232830643708079,
        Less,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        Up,
        232830643708080,
        Greater,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        Ceiling,
        232830643708080,
        Greater,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        Nearest,
        232830643708080,
        Greater,
    );

    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        Down,
        1000000000000,
        Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        Floor,
        1000000000000,
        Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        Up,
        1000000000000,
        Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        Ceiling,
        1000000000000,
        Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        Nearest,
        1000000000000,
        Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        Exact,
        1000000000000,
        Equal,
    );

    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        Down,
        999999999999,
        Less,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        Floor,
        999999999999,
        Less,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        Up,
        1000000000000,
        Greater,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        Ceiling,
        1000000000000,
        Greater,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        Nearest,
        999999999999,
        Less,
    );

    test::<u128>(
        2999999999999999999999999,
        2000000000000000000000000,
        Nearest,
        1,
        Less,
    );
    test::<u128>(
        3000000000000000000000000,
        2000000000000000000000000,
        Nearest,
        2,
        Greater,
    );
    test::<u128>(
        3000000000000000000000001,
        2000000000000000000000000,
        Nearest,
        2,
        Greater,
    );
}

#[test]
fn test_div_round_signed() {
    fn test<T: PrimitiveSigned>(n: T, d: T, rm: RoundingMode, q: T, o: Ordering) {
        assert_eq!(n.div_round(d, rm), (q, o));

        let mut mut_n = n;
        assert_eq!(mut_n.div_round_assign(d, rm), o);
        assert_eq!(mut_n, q);
    }
    test::<i8>(0, 1, Down, 0, Equal);
    test::<i8>(0, 1, Floor, 0, Equal);
    test::<i8>(0, 1, Up, 0, Equal);
    test::<i8>(0, 1, Ceiling, 0, Equal);
    test::<i8>(0, 1, Nearest, 0, Equal);
    test::<i8>(0, 1, Exact, 0, Equal);

    test::<i16>(0, 123, Down, 0, Equal);
    test::<i16>(0, 123, Floor, 0, Equal);
    test::<i16>(0, 123, Up, 0, Equal);
    test::<i16>(0, 123, Ceiling, 0, Equal);
    test::<i16>(0, 123, Nearest, 0, Equal);
    test::<i16>(0, 123, Exact, 0, Equal);

    test::<i32>(1, 1, Down, 1, Equal);
    test::<i32>(1, 1, Floor, 1, Equal);
    test::<i32>(1, 1, Up, 1, Equal);
    test::<i32>(1, 1, Ceiling, 1, Equal);
    test::<i32>(1, 1, Nearest, 1, Equal);
    test::<i32>(1, 1, Exact, 1, Equal);

    test::<i64>(123, 1, Down, 123, Equal);
    test::<i64>(123, 1, Floor, 123, Equal);
    test::<i64>(123, 1, Up, 123, Equal);
    test::<i64>(123, 1, Ceiling, 123, Equal);
    test::<i64>(123, 1, Nearest, 123, Equal);
    test::<i64>(123, 1, Exact, 123, Equal);

    test::<i128>(123, 2, Down, 61, Less);
    test::<i128>(123, 2, Floor, 61, Less);
    test::<i128>(123, 2, Up, 62, Greater);
    test::<i128>(123, 2, Ceiling, 62, Greater);
    test::<i128>(123, 2, Nearest, 62, Greater);

    test::<isize>(125, 2, Down, 62, Less);
    test::<isize>(125, 2, Floor, 62, Less);
    test::<isize>(125, 2, Up, 63, Greater);
    test::<isize>(125, 2, Ceiling, 63, Greater);
    test::<isize>(125, 2, Nearest, 62, Less);

    test::<i8>(123, 123, Down, 1, Equal);
    test::<i8>(123, 123, Floor, 1, Equal);
    test::<i8>(123, 123, Up, 1, Equal);
    test::<i8>(123, 123, Ceiling, 1, Equal);
    test::<i8>(123, 123, Nearest, 1, Equal);
    test::<i8>(123, 123, Exact, 1, Equal);

    test::<i16>(123, 456, Down, 0, Less);
    test::<i16>(123, 456, Floor, 0, Less);
    test::<i16>(123, 456, Up, 1, Greater);
    test::<i16>(123, 456, Ceiling, 1, Greater);
    test::<i16>(123, 456, Nearest, 0, Less);

    test::<i64>(1000000000000, 1, Down, 1000000000000, Equal);
    test::<i64>(1000000000000, 1, Floor, 1000000000000, Equal);
    test::<i64>(1000000000000, 1, Up, 1000000000000, Equal);
    test::<i64>(1000000000000, 1, Ceiling, 1000000000000, Equal);
    test::<i64>(1000000000000, 1, Nearest, 1000000000000, Equal);
    test::<i64>(1000000000000, 1, Exact, 1000000000000, Equal);

    test::<i64>(1000000000000, 3, Down, 333333333333, Less);
    test::<i64>(1000000000000, 3, Floor, 333333333333, Less);
    test::<i64>(1000000000000, 3, Up, 333333333334, Greater);
    test::<i64>(1000000000000, 3, Ceiling, 333333333334, Greater);
    test::<i64>(1000000000000, 3, Nearest, 333333333333, Less);

    test::<i64>(999999999999, 2, Down, 499999999999, Less);
    test::<i64>(999999999999, 2, Floor, 499999999999, Less);
    test::<i64>(999999999999, 2, Up, 500000000000, Greater);
    test::<i64>(999999999999, 2, Ceiling, 500000000000, Greater);
    test::<i64>(999999999999, 2, Nearest, 500000000000, Greater);

    test::<i64>(1000000000001, 2, Down, 500000000000, Less);
    test::<i64>(1000000000001, 2, Floor, 500000000000, Less);
    test::<i64>(1000000000001, 2, Up, 500000000001, Greater);
    test::<i64>(1000000000001, 2, Ceiling, 500000000001, Greater);
    test::<i64>(1000000000001, 2, Nearest, 500000000000, Less);

    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        Down,
        232830643708079,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        Floor,
        232830643708079,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        Up,
        232830643708080,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        Ceiling,
        232830643708080,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        Nearest,
        232830643708080,
        Greater,
    );

    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Down,
        1000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Floor,
        1000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Up,
        1000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Ceiling,
        1000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Nearest,
        1000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Exact,
        1000000000000,
        Equal,
    );

    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        Down,
        999999999999,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        Floor,
        999999999999,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        Up,
        1000000000000,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        Ceiling,
        1000000000000,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        Nearest,
        999999999999,
        Less,
    );

    test::<i128>(
        2999999999999999999999999,
        2000000000000000000000000,
        Nearest,
        1,
        Less,
    );
    test::<i128>(
        3000000000000000000000000,
        2000000000000000000000000,
        Nearest,
        2,
        Greater,
    );
    test::<i128>(
        3000000000000000000000001,
        2000000000000000000000000,
        Nearest,
        2,
        Greater,
    );

    test::<i8>(0, -1, Down, 0, Equal);
    test::<i8>(0, -1, Floor, 0, Equal);
    test::<i8>(0, -1, Up, 0, Equal);
    test::<i8>(0, -1, Ceiling, 0, Equal);
    test::<i8>(0, -1, Nearest, 0, Equal);
    test::<i8>(0, -1, Exact, 0, Equal);

    test::<i16>(0, -123, Down, 0, Equal);
    test::<i16>(0, -123, Floor, 0, Equal);
    test::<i16>(0, -123, Up, 0, Equal);
    test::<i16>(0, -123, Ceiling, 0, Equal);
    test::<i16>(0, -123, Nearest, 0, Equal);
    test::<i16>(0, -123, Exact, 0, Equal);

    test::<i32>(1, -1, Down, -1, Equal);
    test::<i32>(1, -1, Floor, -1, Equal);
    test::<i32>(1, -1, Up, -1, Equal);
    test::<i32>(1, -1, Ceiling, -1, Equal);
    test::<i32>(1, -1, Nearest, -1, Equal);
    test::<i32>(1, -1, Exact, -1, Equal);

    test::<i64>(123, -1, Down, -123, Equal);
    test::<i64>(123, -1, Floor, -123, Equal);
    test::<i64>(123, -1, Up, -123, Equal);
    test::<i64>(123, -1, Ceiling, -123, Equal);
    test::<i64>(123, -1, Nearest, -123, Equal);
    test::<i64>(123, -1, Exact, -123, Equal);

    test::<i128>(123, -2, Down, -61, Greater);
    test::<i128>(123, -2, Floor, -62, Less);
    test::<i128>(123, -2, Up, -62, Less);
    test::<i128>(123, -2, Ceiling, -61, Greater);
    test::<i128>(123, -2, Nearest, -62, Less);

    test::<isize>(125, -2, Down, -62, Greater);
    test::<isize>(125, -2, Floor, -63, Less);
    test::<isize>(125, -2, Up, -63, Less);
    test::<isize>(125, -2, Ceiling, -62, Greater);
    test::<isize>(125, -2, Nearest, -62, Greater);

    test::<i8>(123, -123, Down, -1, Equal);
    test::<i8>(123, -123, Floor, -1, Equal);
    test::<i8>(123, -123, Up, -1, Equal);
    test::<i8>(123, -123, Ceiling, -1, Equal);
    test::<i8>(123, -123, Nearest, -1, Equal);
    test::<i8>(123, -123, Exact, -1, Equal);

    test::<i16>(123, -456, Down, 0, Greater);
    test::<i16>(123, -456, Floor, -1, Less);
    test::<i16>(123, -456, Up, -1, Less);
    test::<i16>(123, -456, Ceiling, 0, Greater);
    test::<i16>(123, -456, Nearest, 0, Greater);

    test::<i64>(1000000000000, -1, Down, -1000000000000, Equal);
    test::<i64>(1000000000000, -1, Floor, -1000000000000, Equal);
    test::<i64>(1000000000000, -1, Up, -1000000000000, Equal);
    test::<i64>(1000000000000, -1, Ceiling, -1000000000000, Equal);
    test::<i64>(1000000000000, -1, Nearest, -1000000000000, Equal);
    test::<i64>(1000000000000, -1, Exact, -1000000000000, Equal);

    test::<i64>(1000000000000, -3, Down, -333333333333, Greater);
    test::<i64>(1000000000000, -3, Floor, -333333333334, Less);
    test::<i64>(1000000000000, -3, Up, -333333333334, Less);
    test::<i64>(1000000000000, -3, Ceiling, -333333333333, Greater);
    test::<i64>(1000000000000, -3, Nearest, -333333333333, Greater);

    test::<i64>(999999999999, -2, Down, -499999999999, Greater);
    test::<i64>(999999999999, -2, Floor, -500000000000, Less);
    test::<i64>(999999999999, -2, Up, -500000000000, Less);
    test::<i64>(999999999999, -2, Ceiling, -499999999999, Greater);
    test::<i64>(999999999999, -2, Nearest, -500000000000, Less);

    test::<i64>(1000000000001, -2, Down, -500000000000, Greater);
    test::<i64>(1000000000001, -2, Floor, -500000000001, Less);
    test::<i64>(1000000000001, -2, Up, -500000000001, Less);
    test::<i64>(1000000000001, -2, Ceiling, -500000000000, Greater);
    test::<i64>(1000000000001, -2, Nearest, -500000000000, Greater);

    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        Down,
        -232830643708079,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        Floor,
        -232830643708080,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        Up,
        -232830643708080,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        Ceiling,
        -232830643708079,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        Nearest,
        -232830643708080,
        Less,
    );

    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        Down,
        -1000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        Floor,
        -1000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        Up,
        -1000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        Ceiling,
        -1000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        Nearest,
        -1000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        Exact,
        -1000000000000,
        Equal,
    );

    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        Down,
        -999999999999,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        Floor,
        -1000000000000,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        Up,
        -1000000000000,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        Ceiling,
        -999999999999,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        Nearest,
        -999999999999,
        Greater,
    );

    test::<i128>(
        2999999999999999999999999,
        -2000000000000000000000000,
        Nearest,
        -1,
        Greater,
    );
    test::<i128>(
        3000000000000000000000000,
        -2000000000000000000000000,
        Nearest,
        -2,
        Less,
    );
    test::<i128>(
        3000000000000000000000001,
        -2000000000000000000000000,
        Nearest,
        -2,
        Less,
    );

    test::<i8>(-1, 1, Down, -1, Equal);
    test::<i8>(-1, 1, Floor, -1, Equal);
    test::<i8>(-1, 1, Up, -1, Equal);
    test::<i8>(-1, 1, Ceiling, -1, Equal);
    test::<i8>(-1, 1, Nearest, -1, Equal);
    test::<i8>(-1, 1, Exact, -1, Equal);

    test::<i16>(-123, 1, Down, -123, Equal);
    test::<i16>(-123, 1, Floor, -123, Equal);
    test::<i16>(-123, 1, Up, -123, Equal);
    test::<i16>(-123, 1, Ceiling, -123, Equal);
    test::<i16>(-123, 1, Nearest, -123, Equal);
    test::<i16>(-123, 1, Exact, -123, Equal);

    test::<i32>(-123, 2, Down, -61, Greater);
    test::<i32>(-123, 2, Floor, -62, Less);
    test::<i32>(-123, 2, Up, -62, Less);
    test::<i32>(-123, 2, Ceiling, -61, Greater);
    test::<i32>(-123, 2, Nearest, -62, Less);

    test::<i64>(-125, 2, Down, -62, Greater);
    test::<i64>(-125, 2, Floor, -63, Less);
    test::<i64>(-125, 2, Up, -63, Less);
    test::<i64>(-125, 2, Ceiling, -62, Greater);
    test::<i64>(-125, 2, Nearest, -62, Greater);

    test::<i128>(-123, 123, Down, -1, Equal);
    test::<i128>(-123, 123, Floor, -1, Equal);
    test::<i128>(-123, 123, Up, -1, Equal);
    test::<i128>(-123, 123, Ceiling, -1, Equal);
    test::<i128>(-123, 123, Nearest, -1, Equal);
    test::<i128>(-123, 123, Exact, -1, Equal);

    test::<isize>(-123, 456, Down, 0, Greater);
    test::<isize>(-123, 456, Floor, -1, Less);
    test::<isize>(-123, 456, Up, -1, Less);
    test::<isize>(-123, 456, Ceiling, 0, Greater);
    test::<isize>(-123, 456, Nearest, 0, Greater);

    test::<i64>(-1000000000000, 1, Down, -1000000000000, Equal);
    test::<i64>(-1000000000000, 1, Floor, -1000000000000, Equal);
    test::<i64>(-1000000000000, 1, Up, -1000000000000, Equal);
    test::<i64>(-1000000000000, 1, Ceiling, -1000000000000, Equal);
    test::<i64>(-1000000000000, 1, Nearest, -1000000000000, Equal);
    test::<i64>(-1000000000000, 1, Exact, -1000000000000, Equal);

    test::<i64>(-1000000000000, 3, Down, -333333333333, Greater);
    test::<i64>(-1000000000000, 3, Floor, -333333333334, Less);
    test::<i64>(-1000000000000, 3, Up, -333333333334, Less);
    test::<i64>(-1000000000000, 3, Ceiling, -333333333333, Greater);
    test::<i64>(-1000000000000, 3, Nearest, -333333333333, Greater);

    test::<i64>(-999999999999, 2, Down, -499999999999, Greater);
    test::<i64>(-999999999999, 2, Floor, -500000000000, Less);
    test::<i64>(-999999999999, 2, Up, -500000000000, Less);
    test::<i64>(-999999999999, 2, Ceiling, -499999999999, Greater);
    test::<i64>(-999999999999, 2, Nearest, -500000000000, Less);

    test::<i64>(-1000000000001, 2, Down, -500000000000, Greater);
    test::<i64>(-1000000000001, 2, Floor, -500000000001, Less);
    test::<i64>(-1000000000001, 2, Up, -500000000001, Less);
    test::<i64>(-1000000000001, 2, Ceiling, -500000000000, Greater);
    test::<i64>(-1000000000001, 2, Nearest, -500000000000, Greater);

    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        Down,
        -232830643708079,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        Floor,
        -232830643708080,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        Up,
        -232830643708080,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        Ceiling,
        -232830643708079,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        Nearest,
        -232830643708080,
        Less,
    );

    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        Down,
        -1000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        Floor,
        -1000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        Up,
        -1000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        Ceiling,
        -1000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        Nearest,
        -1000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        Exact,
        -1000000000000,
        Equal,
    );

    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        Down,
        -999999999999,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        Floor,
        -1000000000000,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        Up,
        -1000000000000,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        Ceiling,
        -999999999999,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        Nearest,
        -999999999999,
        Greater,
    );

    test::<i128>(
        -2999999999999999999999999,
        2000000000000000000000000,
        Nearest,
        -1,
        Greater,
    );
    test::<i128>(
        -3000000000000000000000000,
        2000000000000000000000000,
        Nearest,
        -2,
        Less,
    );
    test::<i128>(
        -3000000000000000000000001,
        2000000000000000000000000,
        Nearest,
        -2,
        Less,
    );

    test::<i8>(-1, -1, Down, 1, Equal);
    test::<i8>(-1, -1, Floor, 1, Equal);
    test::<i8>(-1, -1, Up, 1, Equal);
    test::<i8>(-1, -1, Ceiling, 1, Equal);
    test::<i8>(-1, -1, Nearest, 1, Equal);
    test::<i8>(-1, -1, Exact, 1, Equal);

    test::<i16>(-123, -1, Down, 123, Equal);
    test::<i16>(-123, -1, Floor, 123, Equal);
    test::<i16>(-123, -1, Up, 123, Equal);
    test::<i16>(-123, -1, Ceiling, 123, Equal);
    test::<i16>(-123, -1, Nearest, 123, Equal);
    test::<i16>(-123, -1, Exact, 123, Equal);

    test::<i32>(-123, -2, Down, 61, Less);
    test::<i32>(-123, -2, Floor, 61, Less);
    test::<i32>(-123, -2, Up, 62, Greater);
    test::<i32>(-123, -2, Ceiling, 62, Greater);
    test::<i32>(-123, -2, Nearest, 62, Greater);

    test::<i64>(-125, -2, Down, 62, Less);
    test::<i64>(-125, -2, Floor, 62, Less);
    test::<i64>(-125, -2, Up, 63, Greater);
    test::<i64>(-125, -2, Ceiling, 63, Greater);
    test::<i64>(-125, -2, Nearest, 62, Less);

    test::<i128>(-123, -123, Down, 1, Equal);
    test::<i128>(-123, -123, Floor, 1, Equal);
    test::<i128>(-123, -123, Up, 1, Equal);
    test::<i128>(-123, -123, Ceiling, 1, Equal);
    test::<i128>(-123, -123, Nearest, 1, Equal);
    test::<i128>(-123, -123, Exact, 1, Equal);

    test::<isize>(-123, -456, Down, 0, Less);
    test::<isize>(-123, -456, Floor, 0, Less);
    test::<isize>(-123, -456, Up, 1, Greater);
    test::<isize>(-123, -456, Ceiling, 1, Greater);
    test::<isize>(-123, -456, Nearest, 0, Less);

    test::<i64>(-1000000000000, -1, Down, 1000000000000, Equal);
    test::<i64>(-1000000000000, -1, Floor, 1000000000000, Equal);
    test::<i64>(-1000000000000, -1, Up, 1000000000000, Equal);
    test::<i64>(-1000000000000, -1, Ceiling, 1000000000000, Equal);
    test::<i64>(-1000000000000, -1, Nearest, 1000000000000, Equal);
    test::<i64>(-1000000000000, -1, Exact, 1000000000000, Equal);

    test::<i64>(-1000000000000, -3, Down, 333333333333, Less);
    test::<i64>(-1000000000000, -3, Floor, 333333333333, Less);
    test::<i64>(-1000000000000, -3, Up, 333333333334, Greater);
    test::<i64>(-1000000000000, -3, Ceiling, 333333333334, Greater);
    test::<i64>(-1000000000000, -3, Nearest, 333333333333, Less);

    test::<i64>(-999999999999, -2, Down, 499999999999, Less);
    test::<i64>(-999999999999, -2, Floor, 499999999999, Less);
    test::<i64>(-999999999999, -2, Up, 500000000000, Greater);
    test::<i64>(-999999999999, -2, Ceiling, 500000000000, Greater);
    test::<i64>(-999999999999, -2, Nearest, 500000000000, Greater);

    test::<i64>(-1000000000001, -2, Down, 500000000000, Less);
    test::<i64>(-1000000000001, -2, Floor, 500000000000, Less);
    test::<i64>(-1000000000001, -2, Up, 500000000001, Greater);
    test::<i64>(-1000000000001, -2, Ceiling, 500000000001, Greater);
    test::<i64>(-1000000000001, -2, Nearest, 500000000000, Less);

    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        Down,
        232830643708079,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        Floor,
        232830643708079,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        Up,
        232830643708080,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        Ceiling,
        232830643708080,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        Nearest,
        232830643708080,
        Greater,
    );

    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        Down,
        1000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        Floor,
        1000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        Up,
        1000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        Ceiling,
        1000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        Nearest,
        1000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        Exact,
        1000000000000,
        Equal,
    );

    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        Down,
        999999999999,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        Floor,
        999999999999,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        Up,
        1000000000000,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        Ceiling,
        1000000000000,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        Nearest,
        999999999999,
        Less,
    );

    test::<i128>(
        -2999999999999999999999999,
        -2000000000000000000000000,
        Nearest,
        1,
        Less,
    );
    test::<i128>(
        -3000000000000000000000000,
        -2000000000000000000000000,
        Nearest,
        2,
        Greater,
    );
    test::<i128>(
        -3000000000000000000000001,
        -2000000000000000000000000,
        Nearest,
        2,
        Greater,
    );

    test::<i8>(-128, 1, Down, -128, Equal);
    test::<i8>(-128, 1, Up, -128, Equal);
    test::<i8>(-128, 1, Floor, -128, Equal);
    test::<i8>(-128, 1, Ceiling, -128, Equal);
    test::<i8>(-128, 1, Nearest, -128, Equal);
    test::<i8>(-128, 1, Exact, -128, Equal);
}

fn div_round_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::exact_from(10).div_round(T::ZERO, Floor));
    assert_panic!(T::exact_from(10).div_round(T::exact_from(3), Exact));
    assert_panic!(T::exact_from(10).div_round_assign(T::ZERO, Floor));
    assert_panic!(T::exact_from(10).div_round_assign(T::exact_from(3), Exact));
}

fn div_round_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::MIN.div_round(T::NEGATIVE_ONE, Floor));
    assert_panic!({
        let mut n = T::MIN;
        n.div_round_assign(T::NEGATIVE_ONE, Floor);
    });
}

#[test]
fn div_round_fail() {
    apply_fn_to_primitive_ints!(div_round_fail_helper);
    apply_fn_to_signeds!(div_round_signed_fail_helper);
}

fn div_round_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_unsigned_rounding_mode_triple_gen_var_1::<T>().test_properties(|(x, y, rm)| {
        let mut mut_x = x;
        let o = mut_x.div_round_assign(y, rm);
        let q = mut_x;

        assert_eq!(x.div_round(y, rm), (q, o));
        assert!(q <= x);
        assert_eq!(x.divisible_by(y), o == Equal);

        match rm {
            Floor | Down => assert_ne!(o, Greater),
            Ceiling | Up => assert_ne!(o, Less),
            Exact => assert_eq!(o, Equal),
            _ => {}
        }
        if let Some(product) = q.checked_mul(y) {
            assert_eq!(product.cmp(&x), o);
        }

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                assert_eq!(x.div_round(y, rm), (q, Equal));
            }
        } else {
            assert_panic!(x.div_round(y, Exact));
        }
    });

    unsigned_pair_gen_var_12::<T, T>().test_properties(|(x, y)| {
        assert_eq!(x.ceiling_div_neg_mod(y).0, x.div_round(y, Ceiling).0);
    });

    unsigned_pair_gen_var_11::<T>().test_properties(|(x, y)| {
        let q = x.div_exact(y);
        let o = Equal;
        assert_eq!(x.div_round(y, Down), (q, o));
        assert_eq!(x.div_round(y, Up), (q, o));
        assert_eq!(x.div_round(y, Floor), (q, o));
        assert_eq!(x.div_round(y, Ceiling), (q, o));
        assert_eq!(x.div_round(y, Nearest), (q, o));
        assert_eq!(x.div_round(y, Exact), (q, o));
    });

    unsigned_pair_gen_var_13::<T>().test_properties(|(x, y)| {
        let down = x.div_round(y, Down);
        assert_eq!(down.1, Less);
        let up = (down.0 + T::ONE, Greater);
        assert_eq!(x.div_round(y, Up), up);
        assert_eq!(x.div_round(y, Floor), down);
        assert_eq!(x.div_round(y, Ceiling), up);
        let nearest = x.div_round(y, Nearest);
        assert!(nearest == down || nearest == up);
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(x, rm)| {
        assert_eq!(x.div_round(T::ONE, rm), (x, Equal));
        assert_panic!(x.div_round(T::ZERO, rm));
        assert_panic!({
            let mut y = x;
            y.div_round_assign(T::ZERO, rm)
        });
    });

    unsigned_rounding_mode_pair_gen_var_1::<T>().test_properties(|(x, rm)| {
        assert_eq!(T::ZERO.div_round(x, rm), (T::ZERO, Equal));
        assert_eq!(x.div_round(x, rm), (T::ONE, Equal));
    });
}

fn div_round_properties_helper_signed<T: PrimitiveSigned>() {
    signed_signed_rounding_mode_triple_gen_var_1::<T>().test_properties(|(x, y, rm)| {
        let mut mut_x = x;
        let o = mut_x.div_round_assign(y, rm);
        let q = mut_x;

        assert_eq!(x.div_round(y, rm), (q, o));

        assert!(q.le_abs(&x));
        assert_eq!(x.divisible_by(y), o == Equal);
        if x != T::MIN {
            let (q_alt, o_alt) = (-x).div_round(y, -rm);
            assert_eq!(-q_alt, q);
            assert_eq!(o_alt.reverse(), o);
        }
        if y != T::MIN && (x != T::MIN || (y != T::ONE && y != T::NEGATIVE_ONE)) {
            let (q_alt, o_alt) = x.div_round(-y, -rm);
            assert_eq!(-q_alt, q);
            assert_eq!(o_alt.reverse(), o);
        }

        match ((x >= T::ZERO) == (y >= T::ZERO), rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }
        if let Some(product) = q.checked_mul(y) {
            assert_eq!(product.cmp(&x), if y >= T::ZERO { o } else { o.reverse() });
        }

        if o == Equal {
            for rm in exhaustive_rounding_modes() {
                assert_eq!(x.div_round(y, rm), (q, Equal));
            }
        } else {
            assert_panic!(x.div_round(y, Exact));
        }
    });

    signed_pair_gen_var_3::<T>().test_properties(|(x, y)| {
        let q = x.div_exact(y);
        let o = Equal;
        assert_eq!(x.div_round(y, Down), (q, o));
        assert_eq!(x.div_round(y, Up), (q, o));
        assert_eq!(x.div_round(y, Floor), (q, o));
        assert_eq!(x.div_round(y, Ceiling), (q, o));
        assert_eq!(x.div_round(y, Nearest), (q, o));
        assert_eq!(x.div_round(y, Exact), (q, o));
    });

    signed_pair_gen_var_5::<T>().test_properties(|(x, y)| {
        let down = x.div_round(y, Down);
        let up = if (x >= T::ZERO) == (y >= T::ZERO) {
            (down.0 + T::ONE, Greater)
        } else {
            (down.0 - T::ONE, Less)
        };
        let floor = x.div_round(y, Floor);
        let ceiling = (floor.0 + T::ONE, Greater);
        assert_eq!(x.div_round(y, Up), up);
        assert_eq!(x.div_round(y, Ceiling), ceiling);
        let nearest = x.div_round(y, Nearest);
        assert!(nearest == down || nearest == up);
    });

    signed_rounding_mode_pair_gen::<T>().test_properties(|(x, rm)| {
        assert_eq!(x.div_round(T::ONE, rm), (x, Equal));
        assert_panic!(x.div_round(T::ZERO, rm));
        assert_panic!({
            let mut y = x;
            y.div_round_assign(T::ZERO, rm)
        });
    });

    signed_rounding_mode_pair_gen_var_2::<T>().test_properties(|(x, rm)| {
        assert_eq!(x.div_round(T::NEGATIVE_ONE, rm), (-x, Equal));
    });

    signed_rounding_mode_pair_gen_var_1::<T>().test_properties(|(x, rm)| {
        assert_eq!(T::ZERO.div_round(x, rm), (T::ZERO, Equal));
        assert_eq!(x.div_round(x, rm), (T::ONE, Equal));
    });

    signed_rounding_mode_pair_gen_var_3::<T>().test_properties(|(x, rm)| {
        assert_eq!(x.div_round(-x, rm), (T::NEGATIVE_ONE, Equal));
        assert_eq!((-x).div_round(x, rm), (T::NEGATIVE_ONE, Equal));
    });
}

#[test]
fn div_round_properties() {
    apply_fn_to_unsigneds!(div_round_properties_helper_unsigned);
    apply_fn_to_signeds!(div_round_properties_helper_signed);
}
