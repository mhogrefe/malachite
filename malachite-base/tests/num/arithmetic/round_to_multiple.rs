// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ConvertibleFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    signed_pair_gen, signed_pair_gen_var_5, signed_rounding_mode_pair_gen,
    signed_signed_rounding_mode_triple_gen_var_2, unsigned_pair_gen_var_13,
    unsigned_pair_gen_var_27, unsigned_rounding_mode_pair_gen,
    unsigned_unsigned_rounding_mode_triple_gen_var_2,
};
use std::cmp::Ordering::{self, *};
use std::panic::catch_unwind;

#[test]
fn test_round_to_multiple_unsigned() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, rm: RoundingMode, q: T, o: Ordering) {
        assert_eq!(n.round_to_multiple(d, rm), (q, o));

        let mut mut_n = n;
        assert_eq!(mut_n.round_to_multiple_assign(d, rm), o);
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

    test::<u128>(123, 2, Down, 122, Less);
    test::<u128>(123, 2, Floor, 122, Less);
    test::<u128>(123, 2, Up, 124, Greater);
    test::<u128>(123, 2, Ceiling, 124, Greater);
    test::<u128>(123, 2, Nearest, 124, Greater);

    test::<usize>(125, 2, Down, 124, Less);
    test::<usize>(125, 2, Floor, 124, Less);
    test::<usize>(125, 2, Up, 126, Greater);
    test::<usize>(125, 2, Ceiling, 126, Greater);
    test::<usize>(125, 2, Nearest, 124, Less);

    test::<u8>(123, 123, Down, 123, Equal);
    test::<u8>(123, 123, Floor, 123, Equal);
    test::<u8>(123, 123, Up, 123, Equal);
    test::<u8>(123, 123, Ceiling, 123, Equal);
    test::<u8>(123, 123, Nearest, 123, Equal);
    test::<u8>(123, 123, Exact, 123, Equal);

    test::<u16>(123, 456, Down, 0, Less);
    test::<u16>(123, 456, Floor, 0, Less);
    test::<u16>(123, 456, Up, 456, Greater);
    test::<u16>(123, 456, Ceiling, 456, Greater);
    test::<u16>(123, 456, Nearest, 0, Less);

    test::<u64>(1000000000000, 1, Down, 1000000000000, Equal);
    test::<u64>(1000000000000, 1, Floor, 1000000000000, Equal);
    test::<u64>(1000000000000, 1, Up, 1000000000000, Equal);
    test::<u64>(1000000000000, 1, Ceiling, 1000000000000, Equal);
    test::<u64>(1000000000000, 1, Nearest, 1000000000000, Equal);
    test::<u64>(1000000000000, 1, Exact, 1000000000000, Equal);

    test::<u64>(1000000000000, 3, Down, 999999999999, Less);
    test::<u64>(1000000000000, 3, Floor, 999999999999, Less);
    test::<u64>(1000000000000, 3, Up, 1000000000002, Greater);
    test::<u64>(1000000000000, 3, Ceiling, 1000000000002, Greater);
    test::<u64>(1000000000000, 3, Nearest, 999999999999, Less);

    test::<u64>(999999999999, 2, Down, 999999999998, Less);
    test::<u64>(999999999999, 2, Floor, 999999999998, Less);
    test::<u64>(999999999999, 2, Up, 1000000000000, Greater);
    test::<u64>(999999999999, 2, Ceiling, 1000000000000, Greater);
    test::<u64>(999999999999, 2, Nearest, 1000000000000, Greater);

    test::<u64>(1000000000001, 2, Down, 1000000000000, Less);
    test::<u64>(1000000000001, 2, Floor, 1000000000000, Less);
    test::<u64>(1000000000001, 2, Up, 1000000000002, Greater);
    test::<u64>(1000000000001, 2, Ceiling, 1000000000002, Greater);
    test::<u64>(1000000000001, 2, Nearest, 1000000000000, Less);

    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        Down,
        999999999999996832276305,
        Less,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        Floor,
        999999999999996832276305,
        Less,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        Up,
        1000000000000001127243600,
        Greater,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        Ceiling,
        1000000000000001127243600,
        Greater,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        Nearest,
        1000000000000001127243600,
        Greater,
    );

    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        Down,
        1000000000000000000000000,
        Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        Floor,
        1000000000000000000000000,
        Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        Up,
        1000000000000000000000000,
        Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        Ceiling,
        1000000000000000000000000,
        Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        Nearest,
        1000000000000000000000000,
        Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        Exact,
        1000000000000000000000000,
        Equal,
    );

    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        Down,
        999999999999999999999999,
        Less,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        Floor,
        999999999999999999999999,
        Less,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        Up,
        1000000000001000000000000,
        Greater,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        Ceiling,
        1000000000001000000000000,
        Greater,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        Nearest,
        999999999999999999999999,
        Less,
    );

    test::<u128>(
        2999999999999999999999999,
        2000000000000000000000000,
        Nearest,
        2000000000000000000000000,
        Less,
    );
    test::<u128>(
        3000000000000000000000000,
        2000000000000000000000000,
        Nearest,
        4000000000000000000000000,
        Greater,
    );
    test::<u128>(
        3000000000000000000000001,
        2000000000000000000000000,
        Nearest,
        4000000000000000000000000,
        Greater,
    );

    test::<u8>(0, 0, Floor, 0, Equal);
    test::<u16>(0, 0, Ceiling, 0, Equal);
    test::<u32>(0, 0, Down, 0, Equal);
    test::<u64>(0, 0, Up, 0, Equal);
    test::<u128>(0, 0, Nearest, 0, Equal);
    test::<usize>(0, 0, Exact, 0, Equal);

    test::<u8>(2, 0, Floor, 0, Less);
    test::<u16>(2, 0, Down, 0, Less);
    test::<u32>(2, 0, Nearest, 0, Less);
}

#[test]
fn test_round_to_multiple_signed() {
    fn test<T: PrimitiveSigned>(n: T, d: T, rm: RoundingMode, q: T, o: Ordering) {
        assert_eq!(n.round_to_multiple(d, rm), (q, o));

        let mut mut_n = n;
        assert_eq!(mut_n.round_to_multiple_assign(d, rm), o);
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

    test::<i128>(123, 2, Down, 122, Less);
    test::<i128>(123, 2, Floor, 122, Less);
    test::<i128>(123, 2, Up, 124, Greater);
    test::<i128>(123, 2, Ceiling, 124, Greater);
    test::<i128>(123, 2, Nearest, 124, Greater);

    test::<isize>(125, 2, Down, 124, Less);
    test::<isize>(125, 2, Floor, 124, Less);
    test::<isize>(125, 2, Up, 126, Greater);
    test::<isize>(125, 2, Ceiling, 126, Greater);
    test::<isize>(125, 2, Nearest, 124, Less);

    test::<i8>(123, 123, Down, 123, Equal);
    test::<i8>(123, 123, Floor, 123, Equal);
    test::<i8>(123, 123, Up, 123, Equal);
    test::<i8>(123, 123, Ceiling, 123, Equal);
    test::<i8>(123, 123, Nearest, 123, Equal);
    test::<i8>(123, 123, Exact, 123, Equal);

    test::<i16>(123, 456, Down, 0, Less);
    test::<i16>(123, 456, Floor, 0, Less);
    test::<i16>(123, 456, Up, 456, Greater);
    test::<i16>(123, 456, Ceiling, 456, Greater);
    test::<i16>(123, 456, Nearest, 0, Less);

    test::<i64>(1000000000000, 1, Down, 1000000000000, Equal);
    test::<i64>(1000000000000, 1, Floor, 1000000000000, Equal);
    test::<i64>(1000000000000, 1, Up, 1000000000000, Equal);
    test::<i64>(1000000000000, 1, Ceiling, 1000000000000, Equal);
    test::<i64>(1000000000000, 1, Nearest, 1000000000000, Equal);
    test::<i64>(1000000000000, 1, Exact, 1000000000000, Equal);

    test::<i64>(1000000000000, 3, Down, 999999999999, Less);
    test::<i64>(1000000000000, 3, Floor, 999999999999, Less);
    test::<i64>(1000000000000, 3, Up, 1000000000002, Greater);
    test::<i64>(1000000000000, 3, Ceiling, 1000000000002, Greater);
    test::<i64>(1000000000000, 3, Nearest, 999999999999, Less);

    test::<i64>(999999999999, 2, Down, 999999999998, Less);
    test::<i64>(999999999999, 2, Floor, 999999999998, Less);
    test::<i64>(999999999999, 2, Up, 1000000000000, Greater);
    test::<i64>(999999999999, 2, Ceiling, 1000000000000, Greater);
    test::<i64>(999999999999, 2, Nearest, 1000000000000, Greater);

    test::<i64>(1000000000001, 2, Down, 1000000000000, Less);
    test::<i64>(1000000000001, 2, Floor, 1000000000000, Less);
    test::<i64>(1000000000001, 2, Up, 1000000000002, Greater);
    test::<i64>(1000000000001, 2, Ceiling, 1000000000002, Greater);
    test::<i64>(1000000000001, 2, Nearest, 1000000000000, Less);

    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        Down,
        999999999999996832276305,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        Floor,
        999999999999996832276305,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        Up,
        1000000000000001127243600,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        Ceiling,
        1000000000000001127243600,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        Nearest,
        1000000000000001127243600,
        Greater,
    );

    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Down,
        1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Floor,
        1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Up,
        1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Ceiling,
        1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Nearest,
        1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Exact,
        1000000000000000000000000,
        Equal,
    );

    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        Down,
        999999999999999999999999,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        Floor,
        999999999999999999999999,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        Up,
        1000000000001000000000000,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        Ceiling,
        1000000000001000000000000,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        Nearest,
        999999999999999999999999,
        Less,
    );

    test::<i128>(
        2999999999999999999999999,
        2000000000000000000000000,
        Nearest,
        2000000000000000000000000,
        Less,
    );
    test::<i128>(
        3000000000000000000000000,
        2000000000000000000000000,
        Nearest,
        4000000000000000000000000,
        Greater,
    );
    test::<i128>(
        3000000000000000000000001,
        2000000000000000000000000,
        Nearest,
        4000000000000000000000000,
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

    test::<i32>(1, -1, Down, 1, Equal);
    test::<i32>(1, -1, Floor, 1, Equal);
    test::<i32>(1, -1, Up, 1, Equal);
    test::<i32>(1, -1, Ceiling, 1, Equal);
    test::<i32>(1, -1, Nearest, 1, Equal);
    test::<i32>(1, -1, Exact, 1, Equal);

    test::<i64>(123, -1, Down, 123, Equal);
    test::<i64>(123, -1, Floor, 123, Equal);
    test::<i64>(123, -1, Up, 123, Equal);
    test::<i64>(123, -1, Ceiling, 123, Equal);
    test::<i64>(123, -1, Nearest, 123, Equal);
    test::<i64>(123, -1, Exact, 123, Equal);

    test::<i128>(123, -2, Down, 122, Less);
    test::<i128>(123, -2, Floor, 122, Less);
    test::<i128>(123, -2, Up, 124, Greater);
    test::<i128>(123, -2, Ceiling, 124, Greater);
    test::<i128>(123, -2, Nearest, 124, Greater);

    test::<isize>(125, -2, Down, 124, Less);
    test::<isize>(125, -2, Floor, 124, Less);
    test::<isize>(125, -2, Up, 126, Greater);
    test::<isize>(125, -2, Ceiling, 126, Greater);
    test::<isize>(125, -2, Nearest, 124, Less);

    test::<i8>(123, -123, Down, 123, Equal);
    test::<i8>(123, -123, Floor, 123, Equal);
    test::<i8>(123, -123, Up, 123, Equal);
    test::<i8>(123, -123, Ceiling, 123, Equal);
    test::<i8>(123, -123, Nearest, 123, Equal);
    test::<i8>(123, -123, Exact, 123, Equal);

    test::<i16>(123, -456, Down, 0, Less);
    test::<i16>(123, -456, Floor, 0, Less);
    test::<i16>(123, -456, Up, 456, Greater);
    test::<i16>(123, -456, Ceiling, 456, Greater);
    test::<i16>(123, -456, Nearest, 0, Less);

    test::<i64>(1000000000000, -1, Down, 1000000000000, Equal);
    test::<i64>(1000000000000, -1, Floor, 1000000000000, Equal);
    test::<i64>(1000000000000, -1, Up, 1000000000000, Equal);
    test::<i64>(1000000000000, -1, Ceiling, 1000000000000, Equal);
    test::<i64>(1000000000000, -1, Nearest, 1000000000000, Equal);
    test::<i64>(1000000000000, -1, Exact, 1000000000000, Equal);

    test::<i64>(1000000000000, -3, Down, 999999999999, Less);
    test::<i64>(1000000000000, -3, Floor, 999999999999, Less);
    test::<i64>(1000000000000, -3, Up, 1000000000002, Greater);
    test::<i64>(1000000000000, -3, Ceiling, 1000000000002, Greater);
    test::<i64>(1000000000000, -3, Nearest, 999999999999, Less);

    test::<i64>(999999999999, -2, Down, 999999999998, Less);
    test::<i64>(999999999999, -2, Floor, 999999999998, Less);
    test::<i64>(999999999999, -2, Up, 1000000000000, Greater);
    test::<i64>(999999999999, -2, Ceiling, 1000000000000, Greater);
    test::<i64>(999999999999, -2, Nearest, 1000000000000, Greater);

    test::<i64>(1000000000001, -2, Down, 1000000000000, Less);
    test::<i64>(1000000000001, -2, Floor, 1000000000000, Less);
    test::<i64>(1000000000001, -2, Up, 1000000000002, Greater);
    test::<i64>(1000000000001, -2, Ceiling, 1000000000002, Greater);
    test::<i64>(1000000000001, -2, Nearest, 1000000000000, Less);

    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        Down,
        999999999999996832276305,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        Floor,
        999999999999996832276305,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        Up,
        1000000000000001127243600,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        Ceiling,
        1000000000000001127243600,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        Nearest,
        1000000000000001127243600,
        Greater,
    );

    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Down,
        1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        Floor,
        1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        Up,
        1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        Ceiling,
        1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Nearest,
        1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        Exact,
        1000000000000000000000000,
        Equal,
    );

    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        Down,
        999999999999999999999999,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        Floor,
        999999999999999999999999,
        Less,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        Up,
        1000000000001000000000000,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        Ceiling,
        1000000000001000000000000,
        Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        Nearest,
        999999999999999999999999,
        Less,
    );

    test::<i128>(
        2999999999999999999999999,
        -2000000000000000000000000,
        Nearest,
        2000000000000000000000000,
        Less,
    );
    test::<i128>(
        3000000000000000000000000,
        -2000000000000000000000000,
        Nearest,
        4000000000000000000000000,
        Greater,
    );
    test::<i128>(
        3000000000000000000000001,
        -2000000000000000000000000,
        Nearest,
        4000000000000000000000000,
        Greater,
    );

    test::<i32>(-1, 1, Down, -1, Equal);
    test::<i32>(-1, 1, Floor, -1, Equal);
    test::<i32>(-1, 1, Up, -1, Equal);
    test::<i32>(-1, 1, Ceiling, -1, Equal);
    test::<i32>(-1, 1, Nearest, -1, Equal);
    test::<i32>(-1, 1, Exact, -1, Equal);

    test::<i64>(-123, 1, Down, -123, Equal);
    test::<i64>(-123, 1, Floor, -123, Equal);
    test::<i64>(-123, 1, Up, -123, Equal);
    test::<i64>(-123, 1, Ceiling, -123, Equal);
    test::<i64>(-123, 1, Nearest, -123, Equal);
    test::<i64>(-123, 1, Exact, -123, Equal);

    test::<i128>(-123, 2, Down, -122, Greater);
    test::<i128>(-123, 2, Floor, -124, Less);
    test::<i128>(-123, 2, Up, -124, Less);
    test::<i128>(-123, 2, Ceiling, -122, Greater);
    test::<i128>(-123, 2, Nearest, -124, Less);

    test::<isize>(-125, 2, Down, -124, Greater);
    test::<isize>(-125, 2, Floor, -126, Less);
    test::<isize>(-125, 2, Up, -126, Less);
    test::<isize>(-125, 2, Ceiling, -124, Greater);
    test::<isize>(-125, 2, Nearest, -124, Greater);

    test::<i8>(-123, 123, Down, -123, Equal);
    test::<i8>(-123, 123, Floor, -123, Equal);
    test::<i8>(-123, 123, Up, -123, Equal);
    test::<i8>(-123, 123, Ceiling, -123, Equal);
    test::<i8>(-123, 123, Nearest, -123, Equal);
    test::<i8>(-123, 123, Exact, -123, Equal);

    test::<i16>(-123, 456, Down, 0, Greater);
    test::<i16>(-123, 456, Floor, -456, Less);
    test::<i16>(-123, 456, Up, -456, Less);
    test::<i16>(-123, 456, Ceiling, 0, Greater);
    test::<i16>(-123, 456, Nearest, 0, Greater);

    test::<i64>(-1000000000000, 1, Down, -1000000000000, Equal);
    test::<i64>(-1000000000000, 1, Floor, -1000000000000, Equal);
    test::<i64>(-1000000000000, 1, Up, -1000000000000, Equal);
    test::<i64>(-1000000000000, 1, Ceiling, -1000000000000, Equal);
    test::<i64>(-1000000000000, 1, Nearest, -1000000000000, Equal);
    test::<i64>(-1000000000000, 1, Exact, -1000000000000, Equal);

    test::<i64>(-1000000000000, 3, Down, -999999999999, Greater);
    test::<i64>(-1000000000000, 3, Floor, -1000000000002, Less);
    test::<i64>(-1000000000000, 3, Up, -1000000000002, Less);
    test::<i64>(-1000000000000, 3, Ceiling, -999999999999, Greater);
    test::<i64>(-1000000000000, 3, Nearest, -999999999999, Greater);

    test::<i64>(-999999999999, 2, Down, -999999999998, Greater);
    test::<i64>(-999999999999, 2, Floor, -1000000000000, Less);
    test::<i64>(-999999999999, 2, Up, -1000000000000, Less);
    test::<i64>(-999999999999, 2, Ceiling, -999999999998, Greater);
    test::<i64>(-999999999999, 2, Nearest, -1000000000000, Less);

    test::<i64>(-1000000000001, 2, Down, -1000000000000, Greater);
    test::<i64>(-1000000000001, 2, Floor, -1000000000002, Less);
    test::<i64>(-1000000000001, 2, Up, -1000000000002, Less);
    test::<i64>(-1000000000001, 2, Ceiling, -1000000000000, Greater);
    test::<i64>(-1000000000001, 2, Nearest, -1000000000000, Greater);

    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        Down,
        -999999999999996832276305,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        Floor,
        -1000000000000001127243600,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        Up,
        -1000000000000001127243600,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        Ceiling,
        -999999999999996832276305,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        Nearest,
        -1000000000000001127243600,
        Less,
    );

    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        Down,
        -1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        Floor,
        -1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        Up,
        -1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        Ceiling,
        -1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        Nearest,
        -1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        Exact,
        -1000000000000000000000000,
        Equal,
    );

    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        Down,
        -999999999999999999999999,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        Floor,
        -1000000000001000000000000,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        Up,
        -1000000000001000000000000,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        Ceiling,
        -999999999999999999999999,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        Nearest,
        -999999999999999999999999,
        Greater,
    );

    test::<i128>(
        -2999999999999999999999999,
        2000000000000000000000000,
        Nearest,
        -2000000000000000000000000,
        Greater,
    );
    test::<i128>(
        -3000000000000000000000000,
        2000000000000000000000000,
        Nearest,
        -4000000000000000000000000,
        Less,
    );
    test::<i128>(
        -3000000000000000000000001,
        2000000000000000000000000,
        Nearest,
        -4000000000000000000000000,
        Less,
    );

    test::<i32>(-1, -1, Down, -1, Equal);
    test::<i32>(-1, -1, Floor, -1, Equal);
    test::<i32>(-1, -1, Up, -1, Equal);
    test::<i32>(-1, -1, Ceiling, -1, Equal);
    test::<i32>(-1, -1, Nearest, -1, Equal);
    test::<i32>(-1, -1, Exact, -1, Equal);

    test::<i64>(-123, -1, Down, -123, Equal);
    test::<i64>(-123, -1, Floor, -123, Equal);
    test::<i64>(-123, -1, Up, -123, Equal);
    test::<i64>(-123, -1, Ceiling, -123, Equal);
    test::<i64>(-123, -1, Nearest, -123, Equal);
    test::<i64>(-123, -1, Exact, -123, Equal);

    test::<i128>(-123, -2, Down, -122, Greater);
    test::<i128>(-123, -2, Floor, -124, Less);
    test::<i128>(-123, -2, Up, -124, Less);
    test::<i128>(-123, -2, Ceiling, -122, Greater);
    test::<i128>(-123, -2, Nearest, -124, Less);

    test::<isize>(-125, -2, Down, -124, Greater);
    test::<isize>(-125, -2, Floor, -126, Less);
    test::<isize>(-125, -2, Up, -126, Less);
    test::<isize>(-125, -2, Ceiling, -124, Greater);
    test::<isize>(-125, -2, Nearest, -124, Greater);

    test::<i8>(-123, -123, Down, -123, Equal);
    test::<i8>(-123, -123, Floor, -123, Equal);
    test::<i8>(-123, -123, Up, -123, Equal);
    test::<i8>(-123, -123, Ceiling, -123, Equal);
    test::<i8>(-123, -123, Nearest, -123, Equal);
    test::<i8>(-123, -123, Exact, -123, Equal);

    test::<i16>(-123, -456, Down, 0, Greater);
    test::<i16>(-123, -456, Floor, -456, Less);
    test::<i16>(-123, -456, Up, -456, Less);
    test::<i16>(-123, -456, Ceiling, 0, Greater);
    test::<i16>(-123, -456, Nearest, 0, Greater);

    test::<i64>(-1000000000000, -1, Down, -1000000000000, Equal);
    test::<i64>(-1000000000000, -1, Floor, -1000000000000, Equal);
    test::<i64>(-1000000000000, -1, Up, -1000000000000, Equal);
    test::<i64>(-1000000000000, -1, Ceiling, -1000000000000, Equal);
    test::<i64>(-1000000000000, -1, Nearest, -1000000000000, Equal);
    test::<i64>(-1000000000000, -1, Exact, -1000000000000, Equal);

    test::<i64>(-1000000000000, -3, Down, -999999999999, Greater);
    test::<i64>(-1000000000000, -3, Floor, -1000000000002, Less);
    test::<i64>(-1000000000000, -3, Up, -1000000000002, Less);
    test::<i64>(-1000000000000, -3, Ceiling, -999999999999, Greater);
    test::<i64>(-1000000000000, -3, Nearest, -999999999999, Greater);

    test::<i64>(-999999999999, -2, Down, -999999999998, Greater);
    test::<i64>(-999999999999, -2, Floor, -1000000000000, Less);
    test::<i64>(-999999999999, -2, Up, -1000000000000, Less);
    test::<i64>(-999999999999, -2, Ceiling, -999999999998, Greater);
    test::<i64>(-999999999999, -2, Nearest, -1000000000000, Less);

    test::<i64>(-1000000000001, -2, Down, -1000000000000, Greater);
    test::<i64>(-1000000000001, -2, Floor, -1000000000002, Less);
    test::<i64>(-1000000000001, -2, Up, -1000000000002, Less);
    test::<i64>(-1000000000001, -2, Ceiling, -1000000000000, Greater);
    test::<i64>(-1000000000001, -2, Nearest, -1000000000000, Greater);

    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        Down,
        -999999999999996832276305,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        Floor,
        -1000000000000001127243600,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        Up,
        -1000000000000001127243600,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        Ceiling,
        -999999999999996832276305,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        Nearest,
        -1000000000000001127243600,
        Less,
    );

    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        Down,
        -1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        Floor,
        -1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        Up,
        -1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        Ceiling,
        -1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        Nearest,
        -1000000000000000000000000,
        Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        Exact,
        -1000000000000000000000000,
        Equal,
    );

    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        Down,
        -999999999999999999999999,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        Floor,
        -1000000000001000000000000,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        Up,
        -1000000000001000000000000,
        Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        Ceiling,
        -999999999999999999999999,
        Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        Nearest,
        -999999999999999999999999,
        Greater,
    );

    test::<i128>(
        -2999999999999999999999999,
        -2000000000000000000000000,
        Nearest,
        -2000000000000000000000000,
        Greater,
    );
    test::<i128>(
        -3000000000000000000000000,
        -2000000000000000000000000,
        Nearest,
        -4000000000000000000000000,
        Less,
    );
    test::<i128>(
        -3000000000000000000000001,
        -2000000000000000000000000,
        Nearest,
        -4000000000000000000000000,
        Less,
    );

    test::<i8>(-128, 1, Down, -128, Equal);
    test::<i8>(-128, 1, Up, -128, Equal);
    test::<i8>(-128, 1, Floor, -128, Equal);
    test::<i8>(-128, 1, Ceiling, -128, Equal);
    test::<i8>(-128, 1, Nearest, -128, Equal);
    test::<i8>(-128, 1, Exact, -128, Equal);

    test::<i8>(-128, -1, Down, -128, Equal);
    test::<i8>(-128, -1, Up, -128, Equal);
    test::<i8>(-128, -1, Floor, -128, Equal);
    test::<i8>(-128, -1, Ceiling, -128, Equal);
    test::<i8>(-128, -1, Nearest, -128, Equal);
    test::<i8>(-128, -1, Exact, -128, Equal);

    test::<i8>(-128, -128, Down, -128, Equal);
    test::<i8>(-128, -128, Up, -128, Equal);
    test::<i8>(-128, -128, Floor, -128, Equal);
    test::<i8>(-128, -128, Ceiling, -128, Equal);
    test::<i8>(-128, -128, Nearest, -128, Equal);
    test::<i8>(-128, -128, Exact, -128, Equal);

    test::<i8>(0, 0, Floor, 0, Equal);
    test::<i16>(0, 0, Ceiling, 0, Equal);
    test::<i32>(0, 0, Down, 0, Equal);
    test::<i64>(0, 0, Up, 0, Equal);
    test::<i128>(0, 0, Nearest, 0, Equal);
    test::<isize>(0, 0, Exact, 0, Equal);

    test::<i8>(2, 0, Floor, 0, Less);
    test::<i16>(2, 0, Down, 0, Less);
    test::<i32>(2, 0, Nearest, 0, Less);
    test::<i64>(-2, 0, Ceiling, 0, Greater);
    test::<i128>(-2, 0, Down, 0, Greater);
    test::<isize>(-2, 0, Nearest, 0, Greater);
}

fn round_to_multiple_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::exact_from(10).round_to_multiple(T::ZERO, Up));
    assert_panic!(T::exact_from(10).round_to_multiple(T::exact_from(3), Exact));
    assert_panic!(T::MAX.round_to_multiple(T::TWO, Ceiling));
    assert_panic!(T::ONE.round_to_multiple(T::ZERO, Up));
    assert_panic!(T::ONE.round_to_multiple(T::ZERO, Ceiling));
    assert_panic!(T::ONE.round_to_multiple(T::ZERO, Exact));

    assert_panic!(T::exact_from(10).round_to_multiple_assign(T::ZERO, Up));
    assert_panic!({
        T::exact_from(10).round_to_multiple_assign(T::exact_from(3), Exact);
    });
    assert_panic!({
        let mut n = T::MAX;
        n.round_to_multiple_assign(T::TWO, Ceiling);
    });
    assert_panic!({
        let mut n = T::ONE;
        n.round_to_multiple_assign(T::ZERO, Up);
    });
    assert_panic!({
        let mut n = T::ONE;
        n.round_to_multiple_assign(T::ZERO, Ceiling);
    });
    assert_panic!({
        let mut n = T::ONE;
        n.round_to_multiple_assign(T::ZERO, Exact);
    });
}

fn round_to_multiple_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::MIN.round_to_multiple(T::exact_from(3), Floor));
    assert_panic!(T::NEGATIVE_ONE.round_to_multiple(T::ZERO, Up));
    assert_panic!(T::NEGATIVE_ONE.round_to_multiple(T::ZERO, Floor));
    assert_panic!(T::NEGATIVE_ONE.round_to_multiple(T::ZERO, Exact));

    assert_panic!({
        let mut n = T::MIN;
        n.round_to_multiple_assign(T::exact_from(3), Floor);
    });
    assert_panic!({
        let mut n = T::NEGATIVE_ONE;
        n.round_to_multiple_assign(T::ZERO, Up);
    });
    assert_panic!({
        let mut n = T::NEGATIVE_ONE;
        n.round_to_multiple_assign(T::ZERO, Floor);
    });
    assert_panic!({
        let mut n = T::NEGATIVE_ONE;
        n.round_to_multiple_assign(T::ZERO, Exact);
    });
}

#[test]
fn round_to_multiple_fail() {
    apply_fn_to_primitive_ints!(round_to_multiple_fail_helper);
    apply_fn_to_signeds!(round_to_multiple_signed_fail_helper);
}

fn round_to_multiple_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_unsigned_rounding_mode_triple_gen_var_2::<T>().test_properties(|(x, y, rm)| {
        let (rounded, o) = x.round_to_multiple(y, rm);

        let mut mut_x = x;
        assert_eq!(mut_x.round_to_multiple_assign(y, rm), o);
        assert_eq!(mut_x, rounded);

        assert!(rounded.divisible_by(y));
        assert_eq!(rounded.cmp(&x), o);
        match rm {
            Floor | Down => assert_ne!(o, Greater),
            Ceiling | Up => assert_ne!(o, Less),
            Exact => assert_eq!(o, Equal),
            _ => {}
        }
        match rm {
            Floor | Down => {
                assert!(rounded <= x);
            }
            Ceiling | Up => {
                assert!(rounded >= x);
            }
            Exact => assert_eq!(rounded, x),
            Nearest => {
                if y == T::ZERO {
                    assert_eq!(rounded, T::ZERO);
                } else {
                    let mut closest = None;
                    let mut second_closest = None;
                    if rounded <= x {
                        if let Some(above) = rounded.checked_add(y) {
                            closest = Some(x - rounded);
                            second_closest = Some(above - x);
                        }
                    } else if let Some(below) = rounded.checked_sub(y) {
                        closest = Some(rounded - x);
                        second_closest = Some(x - below);
                    }
                    if let (Some(closest), Some(second_closest)) = (closest, second_closest) {
                        assert!(closest <= second_closest);
                        if closest == second_closest {
                            assert!(rounded.div_exact(y).even());
                        }
                    }
                }
            }
        }
    });

    unsigned_pair_gen_var_27::<T>().test_properties(|(x, y)| {
        if let Some(product) = x.checked_mul(y) {
            let xo = (product, Equal);
            assert_eq!(product.round_to_multiple(y, Down), xo);
            assert_eq!(product.round_to_multiple(y, Up), xo);
            assert_eq!(product.round_to_multiple(y, Floor), xo);
            assert_eq!(product.round_to_multiple(y, Ceiling), xo);
            assert_eq!(product.round_to_multiple(y, Nearest), xo);
            assert_eq!(product.round_to_multiple(y, Exact), xo);
        }
    });

    unsigned_pair_gen_var_13::<T>().test_properties(|(x, y)| {
        let down = x.round_to_multiple(y, Down);
        assert_eq!(down.1, Less);
        if let Some(up) = down.0.checked_add(y) {
            let up = (up, Greater);
            assert_eq!(x.round_to_multiple(y, Up), up);
            assert_eq!(x.round_to_multiple(y, Floor), down);
            assert_eq!(x.round_to_multiple(y, Ceiling), up);
            let nearest = x.round_to_multiple(y, Nearest);
            assert!(nearest == down || nearest == up);
        }
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(n, rm)| {
        if rm == Floor || rm == Down || rm == Nearest {
            assert_eq!(
                n.round_to_multiple(T::ZERO, rm),
                (T::ZERO, if n == T::ZERO { Equal } else { Less })
            );
        }
        assert_eq!(T::ZERO.round_to_multiple(n, rm), (T::ZERO, Equal));
        assert_eq!(n.round_to_multiple(T::ONE, rm), (n, Equal));
        assert_eq!(n.round_to_multiple(n, rm), (n, Equal));
    });
}

fn round_to_multiple_properties_helper_signed<
    U: PrimitiveUnsigned,
    S: TryFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>() {
    signed_signed_rounding_mode_triple_gen_var_2::<U, S>().test_properties(|(x, y, rm)| {
        let (rounded, o) = x.round_to_multiple(y, rm);

        let mut mut_x = x;
        assert_eq!(mut_x.round_to_multiple_assign(y, rm), o);
        assert_eq!(mut_x, rounded);

        assert!(rounded.divisible_by(y));
        assert_eq!(rounded.cmp(&x), o);
        match (x >= S::ZERO, rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }
        match rm {
            Floor => assert!(rounded <= x),
            Ceiling => assert!(rounded >= x),
            Down => assert!(rounded.le_abs(&x)),
            Up => assert!(rounded.ge_abs(&x)),
            Exact => assert_eq!(rounded, x),
            Nearest => {
                if y == S::ZERO {
                    assert_eq!(rounded, S::ZERO);
                } else {
                    let mut closest = None;
                    let mut second_closest = None;
                    let (o_above, o_below) = if y >= S::ZERO {
                        (rounded.checked_add(y), rounded.checked_sub(y))
                    } else {
                        (rounded.checked_sub(y), rounded.checked_add(y))
                    };
                    if rounded <= x {
                        if let Some(above) = o_above {
                            closest = x.checked_sub(rounded);
                            second_closest = above.checked_sub(x);
                        }
                    } else if let Some(below) = o_below {
                        closest = rounded.checked_sub(x);
                        second_closest = x.checked_sub(below);
                    }
                    if let (Some(closest), Some(second_closest)) = (closest, second_closest) {
                        assert!(closest <= second_closest);
                        if closest == second_closest {
                            assert!(rounded.div_exact(y).even());
                        }
                    }
                }
            }
        }
    });

    signed_pair_gen::<S>().test_properties(|(x, y)| {
        if let Some(product) = x.checked_mul(y) {
            let xo = (product, Equal);
            assert_eq!(product.round_to_multiple(y, Down), xo);
            assert_eq!(product.round_to_multiple(y, Up), xo);
            assert_eq!(product.round_to_multiple(y, Floor), xo);
            assert_eq!(product.round_to_multiple(y, Ceiling), xo);
            assert_eq!(product.round_to_multiple(y, Nearest), xo);
            assert_eq!(product.round_to_multiple(y, Exact), xo);
        }
    });

    signed_pair_gen_var_5::<S>().test_properties(|(x, y)| {
        let down = x.round_to_multiple(y, Down);
        assert_eq!(down.1, if x >= S::ZERO { Less } else { Greater });
        if let Some(up) = if (x >= S::ZERO) == (y >= S::ZERO) {
            down.0.checked_add(y)
        } else {
            down.0.checked_sub(y)
        } {
            let up = (up, if x >= S::ZERO { Greater } else { Less });
            assert_eq!(x.round_to_multiple(y, Up), up);
            if x >= S::ZERO {
                assert_eq!(x.round_to_multiple(y, Floor), down);
                assert_eq!(x.round_to_multiple(y, Ceiling), up);
            } else {
                assert_eq!(x.round_to_multiple(y, Floor), up);
                assert_eq!(x.round_to_multiple(y, Ceiling), down);
            }
            let nearest = x.round_to_multiple(y, Nearest);
            assert!(nearest == down || nearest == up);
        }
    });

    signed_rounding_mode_pair_gen::<S>().test_properties(|(n, rm)| {
        if rm == Down || rm == Nearest || rm == if n >= S::ZERO { Floor } else { Ceiling } {
            assert_eq!(
                n.round_to_multiple(S::ZERO, rm),
                (S::ZERO, n.cmp(&S::ZERO).reverse())
            );
        }
        assert_eq!(S::ZERO.round_to_multiple(n, rm), (S::ZERO, Equal));
        assert_eq!(n.round_to_multiple(S::ONE, rm), (n, Equal));
        assert_eq!(n.round_to_multiple(n, rm), (n, Equal));
    });
}

#[test]
fn round_to_multiple_properties() {
    apply_fn_to_unsigneds!(round_to_multiple_properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(round_to_multiple_properties_helper_signed);
}
