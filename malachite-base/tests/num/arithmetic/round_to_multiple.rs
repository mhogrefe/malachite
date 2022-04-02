use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{CheckedFrom, ConvertibleFrom};
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::{
    signed_pair_gen, signed_pair_gen_var_5, signed_rounding_mode_pair_gen,
    signed_signed_rounding_mode_triple_gen_var_2, unsigned_pair_gen_var_13,
    unsigned_pair_gen_var_27, unsigned_rounding_mode_pair_gen,
    unsigned_unsigned_rounding_mode_triple_gen_var_2,
};
use std::panic::catch_unwind;

#[test]
fn test_round_to_multiple_unsigned() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, rm: RoundingMode, q: T) {
        assert_eq!(n.round_to_multiple(d, rm), q);

        let mut mut_n = n;
        mut_n.round_to_multiple_assign(d, rm);
        assert_eq!(mut_n, q);
    }
    test::<u8>(0, 1, RoundingMode::Down, 0);
    test::<u8>(0, 1, RoundingMode::Floor, 0);
    test::<u8>(0, 1, RoundingMode::Up, 0);
    test::<u8>(0, 1, RoundingMode::Ceiling, 0);
    test::<u8>(0, 1, RoundingMode::Nearest, 0);
    test::<u8>(0, 1, RoundingMode::Exact, 0);

    test::<u16>(0, 123, RoundingMode::Down, 0);
    test::<u16>(0, 123, RoundingMode::Floor, 0);
    test::<u16>(0, 123, RoundingMode::Up, 0);
    test::<u16>(0, 123, RoundingMode::Ceiling, 0);
    test::<u16>(0, 123, RoundingMode::Nearest, 0);
    test::<u16>(0, 123, RoundingMode::Exact, 0);

    test::<u32>(1, 1, RoundingMode::Down, 1);
    test::<u32>(1, 1, RoundingMode::Floor, 1);
    test::<u32>(1, 1, RoundingMode::Up, 1);
    test::<u32>(1, 1, RoundingMode::Ceiling, 1);
    test::<u32>(1, 1, RoundingMode::Nearest, 1);
    test::<u32>(1, 1, RoundingMode::Exact, 1);

    test::<u64>(123, 1, RoundingMode::Down, 123);
    test::<u64>(123, 1, RoundingMode::Floor, 123);
    test::<u64>(123, 1, RoundingMode::Up, 123);
    test::<u64>(123, 1, RoundingMode::Ceiling, 123);
    test::<u64>(123, 1, RoundingMode::Nearest, 123);
    test::<u64>(123, 1, RoundingMode::Exact, 123);

    test::<u128>(123, 2, RoundingMode::Down, 122);
    test::<u128>(123, 2, RoundingMode::Floor, 122);
    test::<u128>(123, 2, RoundingMode::Up, 124);
    test::<u128>(123, 2, RoundingMode::Ceiling, 124);
    test::<u128>(123, 2, RoundingMode::Nearest, 124);

    test::<usize>(125, 2, RoundingMode::Down, 124);
    test::<usize>(125, 2, RoundingMode::Floor, 124);
    test::<usize>(125, 2, RoundingMode::Up, 126);
    test::<usize>(125, 2, RoundingMode::Ceiling, 126);
    test::<usize>(125, 2, RoundingMode::Nearest, 124);

    test::<u8>(123, 123, RoundingMode::Down, 123);
    test::<u8>(123, 123, RoundingMode::Floor, 123);
    test::<u8>(123, 123, RoundingMode::Up, 123);
    test::<u8>(123, 123, RoundingMode::Ceiling, 123);
    test::<u8>(123, 123, RoundingMode::Nearest, 123);
    test::<u8>(123, 123, RoundingMode::Exact, 123);

    test::<u16>(123, 456, RoundingMode::Down, 0);
    test::<u16>(123, 456, RoundingMode::Floor, 0);
    test::<u16>(123, 456, RoundingMode::Up, 456);
    test::<u16>(123, 456, RoundingMode::Ceiling, 456);
    test::<u16>(123, 456, RoundingMode::Nearest, 0);

    test::<u64>(1000000000000, 1, RoundingMode::Down, 1000000000000);
    test::<u64>(1000000000000, 1, RoundingMode::Floor, 1000000000000);
    test::<u64>(1000000000000, 1, RoundingMode::Up, 1000000000000);
    test::<u64>(1000000000000, 1, RoundingMode::Ceiling, 1000000000000);
    test::<u64>(1000000000000, 1, RoundingMode::Nearest, 1000000000000);
    test::<u64>(1000000000000, 1, RoundingMode::Exact, 1000000000000);

    test::<u64>(1000000000000, 3, RoundingMode::Down, 999999999999);
    test::<u64>(1000000000000, 3, RoundingMode::Floor, 999999999999);
    test::<u64>(1000000000000, 3, RoundingMode::Up, 1000000000002);
    test::<u64>(1000000000000, 3, RoundingMode::Ceiling, 1000000000002);
    test::<u64>(1000000000000, 3, RoundingMode::Nearest, 999999999999);

    test::<u64>(999999999999, 2, RoundingMode::Down, 999999999998);
    test::<u64>(999999999999, 2, RoundingMode::Floor, 999999999998);
    test::<u64>(999999999999, 2, RoundingMode::Up, 1000000000000);
    test::<u64>(999999999999, 2, RoundingMode::Ceiling, 1000000000000);
    test::<u64>(999999999999, 2, RoundingMode::Nearest, 1000000000000);

    test::<u64>(1000000000001, 2, RoundingMode::Down, 1000000000000);
    test::<u64>(1000000000001, 2, RoundingMode::Floor, 1000000000000);
    test::<u64>(1000000000001, 2, RoundingMode::Up, 1000000000002);
    test::<u64>(1000000000001, 2, RoundingMode::Ceiling, 1000000000002);
    test::<u64>(1000000000001, 2, RoundingMode::Nearest, 1000000000000);

    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Down,
        999999999999996832276305,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Floor,
        999999999999996832276305,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Up,
        1000000000000001127243600,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Ceiling,
        1000000000000001127243600,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Nearest,
        1000000000000001127243600,
    );

    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Down,
        1000000000000000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Floor,
        1000000000000000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Up,
        1000000000000000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Ceiling,
        1000000000000000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Nearest,
        1000000000000000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Exact,
        1000000000000000000000000,
    );

    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Down,
        999999999999999999999999,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Floor,
        999999999999999999999999,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Up,
        1000000000001000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Ceiling,
        1000000000001000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Nearest,
        999999999999999999999999,
    );

    test::<u128>(
        2999999999999999999999999,
        2000000000000000000000000,
        RoundingMode::Nearest,
        2000000000000000000000000,
    );
    test::<u128>(
        3000000000000000000000000,
        2000000000000000000000000,
        RoundingMode::Nearest,
        4000000000000000000000000,
    );
    test::<u128>(
        3000000000000000000000001,
        2000000000000000000000000,
        RoundingMode::Nearest,
        4000000000000000000000000,
    );

    test::<u8>(0, 0, RoundingMode::Floor, 0);
    test::<u16>(0, 0, RoundingMode::Ceiling, 0);
    test::<u32>(0, 0, RoundingMode::Down, 0);
    test::<u64>(0, 0, RoundingMode::Up, 0);
    test::<u128>(0, 0, RoundingMode::Nearest, 0);
    test::<usize>(0, 0, RoundingMode::Exact, 0);

    test::<u8>(2, 0, RoundingMode::Floor, 0);
    test::<u16>(2, 0, RoundingMode::Down, 0);
    test::<u32>(2, 0, RoundingMode::Nearest, 0);
}

#[test]
fn test_round_to_multiple_signed() {
    fn test<T: PrimitiveSigned>(n: T, d: T, rm: RoundingMode, q: T) {
        assert_eq!(n.round_to_multiple(d, rm), q);

        let mut mut_n = n;
        mut_n.round_to_multiple_assign(d, rm);
        assert_eq!(mut_n, q);
    }
    test::<i8>(0, 1, RoundingMode::Down, 0);
    test::<i8>(0, 1, RoundingMode::Floor, 0);
    test::<i8>(0, 1, RoundingMode::Up, 0);
    test::<i8>(0, 1, RoundingMode::Ceiling, 0);
    test::<i8>(0, 1, RoundingMode::Nearest, 0);
    test::<i8>(0, 1, RoundingMode::Exact, 0);

    test::<i16>(0, 123, RoundingMode::Down, 0);
    test::<i16>(0, 123, RoundingMode::Floor, 0);
    test::<i16>(0, 123, RoundingMode::Up, 0);
    test::<i16>(0, 123, RoundingMode::Ceiling, 0);
    test::<i16>(0, 123, RoundingMode::Nearest, 0);
    test::<i16>(0, 123, RoundingMode::Exact, 0);

    test::<i32>(1, 1, RoundingMode::Down, 1);
    test::<i32>(1, 1, RoundingMode::Floor, 1);
    test::<i32>(1, 1, RoundingMode::Up, 1);
    test::<i32>(1, 1, RoundingMode::Ceiling, 1);
    test::<i32>(1, 1, RoundingMode::Nearest, 1);
    test::<i32>(1, 1, RoundingMode::Exact, 1);

    test::<i64>(123, 1, RoundingMode::Down, 123);
    test::<i64>(123, 1, RoundingMode::Floor, 123);
    test::<i64>(123, 1, RoundingMode::Up, 123);
    test::<i64>(123, 1, RoundingMode::Ceiling, 123);
    test::<i64>(123, 1, RoundingMode::Nearest, 123);
    test::<i64>(123, 1, RoundingMode::Exact, 123);

    test::<i128>(123, 2, RoundingMode::Down, 122);
    test::<i128>(123, 2, RoundingMode::Floor, 122);
    test::<i128>(123, 2, RoundingMode::Up, 124);
    test::<i128>(123, 2, RoundingMode::Ceiling, 124);
    test::<i128>(123, 2, RoundingMode::Nearest, 124);

    test::<isize>(125, 2, RoundingMode::Down, 124);
    test::<isize>(125, 2, RoundingMode::Floor, 124);
    test::<isize>(125, 2, RoundingMode::Up, 126);
    test::<isize>(125, 2, RoundingMode::Ceiling, 126);
    test::<isize>(125, 2, RoundingMode::Nearest, 124);

    test::<i8>(123, 123, RoundingMode::Down, 123);
    test::<i8>(123, 123, RoundingMode::Floor, 123);
    test::<i8>(123, 123, RoundingMode::Up, 123);
    test::<i8>(123, 123, RoundingMode::Ceiling, 123);
    test::<i8>(123, 123, RoundingMode::Nearest, 123);
    test::<i8>(123, 123, RoundingMode::Exact, 123);

    test::<i16>(123, 456, RoundingMode::Down, 0);
    test::<i16>(123, 456, RoundingMode::Floor, 0);
    test::<i16>(123, 456, RoundingMode::Up, 456);
    test::<i16>(123, 456, RoundingMode::Ceiling, 456);
    test::<i16>(123, 456, RoundingMode::Nearest, 0);

    test::<i64>(1000000000000, 1, RoundingMode::Down, 1000000000000);
    test::<i64>(1000000000000, 1, RoundingMode::Floor, 1000000000000);
    test::<i64>(1000000000000, 1, RoundingMode::Up, 1000000000000);
    test::<i64>(1000000000000, 1, RoundingMode::Ceiling, 1000000000000);
    test::<i64>(1000000000000, 1, RoundingMode::Nearest, 1000000000000);
    test::<i64>(1000000000000, 1, RoundingMode::Exact, 1000000000000);

    test::<i64>(1000000000000, 3, RoundingMode::Down, 999999999999);
    test::<i64>(1000000000000, 3, RoundingMode::Floor, 999999999999);
    test::<i64>(1000000000000, 3, RoundingMode::Up, 1000000000002);
    test::<i64>(1000000000000, 3, RoundingMode::Ceiling, 1000000000002);
    test::<i64>(1000000000000, 3, RoundingMode::Nearest, 999999999999);

    test::<i64>(999999999999, 2, RoundingMode::Down, 999999999998);
    test::<i64>(999999999999, 2, RoundingMode::Floor, 999999999998);
    test::<i64>(999999999999, 2, RoundingMode::Up, 1000000000000);
    test::<i64>(999999999999, 2, RoundingMode::Ceiling, 1000000000000);
    test::<i64>(999999999999, 2, RoundingMode::Nearest, 1000000000000);

    test::<i64>(1000000000001, 2, RoundingMode::Down, 1000000000000);
    test::<i64>(1000000000001, 2, RoundingMode::Floor, 1000000000000);
    test::<i64>(1000000000001, 2, RoundingMode::Up, 1000000000002);
    test::<i64>(1000000000001, 2, RoundingMode::Ceiling, 1000000000002);
    test::<i64>(1000000000001, 2, RoundingMode::Nearest, 1000000000000);

    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Down,
        999999999999996832276305,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Floor,
        999999999999996832276305,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Up,
        1000000000000001127243600,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Ceiling,
        1000000000000001127243600,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Nearest,
        1000000000000001127243600,
    );

    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Down,
        1000000000000000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Floor,
        1000000000000000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Up,
        1000000000000000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Ceiling,
        1000000000000000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Nearest,
        1000000000000000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Exact,
        1000000000000000000000000,
    );

    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Down,
        999999999999999999999999,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Floor,
        999999999999999999999999,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Up,
        1000000000001000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Ceiling,
        1000000000001000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Nearest,
        999999999999999999999999,
    );

    test::<i128>(
        2999999999999999999999999,
        2000000000000000000000000,
        RoundingMode::Nearest,
        2000000000000000000000000,
    );
    test::<i128>(
        3000000000000000000000000,
        2000000000000000000000000,
        RoundingMode::Nearest,
        4000000000000000000000000,
    );
    test::<i128>(
        3000000000000000000000001,
        2000000000000000000000000,
        RoundingMode::Nearest,
        4000000000000000000000000,
    );

    test::<i8>(0, -1, RoundingMode::Down, 0);
    test::<i8>(0, -1, RoundingMode::Floor, 0);
    test::<i8>(0, -1, RoundingMode::Up, 0);
    test::<i8>(0, -1, RoundingMode::Ceiling, 0);
    test::<i8>(0, -1, RoundingMode::Nearest, 0);
    test::<i8>(0, -1, RoundingMode::Exact, 0);

    test::<i16>(0, -123, RoundingMode::Down, 0);
    test::<i16>(0, -123, RoundingMode::Floor, 0);
    test::<i16>(0, -123, RoundingMode::Up, 0);
    test::<i16>(0, -123, RoundingMode::Ceiling, 0);
    test::<i16>(0, -123, RoundingMode::Nearest, 0);
    test::<i16>(0, -123, RoundingMode::Exact, 0);

    test::<i32>(1, -1, RoundingMode::Down, 1);
    test::<i32>(1, -1, RoundingMode::Floor, 1);
    test::<i32>(1, -1, RoundingMode::Up, 1);
    test::<i32>(1, -1, RoundingMode::Ceiling, 1);
    test::<i32>(1, -1, RoundingMode::Nearest, 1);
    test::<i32>(1, -1, RoundingMode::Exact, 1);

    test::<i64>(123, -1, RoundingMode::Down, 123);
    test::<i64>(123, -1, RoundingMode::Floor, 123);
    test::<i64>(123, -1, RoundingMode::Up, 123);
    test::<i64>(123, -1, RoundingMode::Ceiling, 123);
    test::<i64>(123, -1, RoundingMode::Nearest, 123);
    test::<i64>(123, -1, RoundingMode::Exact, 123);

    test::<i128>(123, -2, RoundingMode::Down, 122);
    test::<i128>(123, -2, RoundingMode::Floor, 122);
    test::<i128>(123, -2, RoundingMode::Up, 124);
    test::<i128>(123, -2, RoundingMode::Ceiling, 124);
    test::<i128>(123, -2, RoundingMode::Nearest, 124);

    test::<isize>(125, -2, RoundingMode::Down, 124);
    test::<isize>(125, -2, RoundingMode::Floor, 124);
    test::<isize>(125, -2, RoundingMode::Up, 126);
    test::<isize>(125, -2, RoundingMode::Ceiling, 126);
    test::<isize>(125, -2, RoundingMode::Nearest, 124);

    test::<i8>(123, -123, RoundingMode::Down, 123);
    test::<i8>(123, -123, RoundingMode::Floor, 123);
    test::<i8>(123, -123, RoundingMode::Up, 123);
    test::<i8>(123, -123, RoundingMode::Ceiling, 123);
    test::<i8>(123, -123, RoundingMode::Nearest, 123);
    test::<i8>(123, -123, RoundingMode::Exact, 123);

    test::<i16>(123, -456, RoundingMode::Down, 0);
    test::<i16>(123, -456, RoundingMode::Floor, 0);
    test::<i16>(123, -456, RoundingMode::Up, 456);
    test::<i16>(123, -456, RoundingMode::Ceiling, 456);
    test::<i16>(123, -456, RoundingMode::Nearest, 0);

    test::<i64>(1000000000000, -1, RoundingMode::Down, 1000000000000);
    test::<i64>(1000000000000, -1, RoundingMode::Floor, 1000000000000);
    test::<i64>(1000000000000, -1, RoundingMode::Up, 1000000000000);
    test::<i64>(1000000000000, -1, RoundingMode::Ceiling, 1000000000000);
    test::<i64>(1000000000000, -1, RoundingMode::Nearest, 1000000000000);
    test::<i64>(1000000000000, -1, RoundingMode::Exact, 1000000000000);

    test::<i64>(1000000000000, -3, RoundingMode::Down, 999999999999);
    test::<i64>(1000000000000, -3, RoundingMode::Floor, 999999999999);
    test::<i64>(1000000000000, -3, RoundingMode::Up, 1000000000002);
    test::<i64>(1000000000000, -3, RoundingMode::Ceiling, 1000000000002);
    test::<i64>(1000000000000, -3, RoundingMode::Nearest, 999999999999);

    test::<i64>(999999999999, -2, RoundingMode::Down, 999999999998);
    test::<i64>(999999999999, -2, RoundingMode::Floor, 999999999998);
    test::<i64>(999999999999, -2, RoundingMode::Up, 1000000000000);
    test::<i64>(999999999999, -2, RoundingMode::Ceiling, 1000000000000);
    test::<i64>(999999999999, -2, RoundingMode::Nearest, 1000000000000);

    test::<i64>(1000000000001, -2, RoundingMode::Down, 1000000000000);
    test::<i64>(1000000000001, -2, RoundingMode::Floor, 1000000000000);
    test::<i64>(1000000000001, -2, RoundingMode::Up, 1000000000002);
    test::<i64>(1000000000001, -2, RoundingMode::Ceiling, 1000000000002);
    test::<i64>(1000000000001, -2, RoundingMode::Nearest, 1000000000000);

    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Down,
        999999999999996832276305,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Floor,
        999999999999996832276305,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Up,
        1000000000000001127243600,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Ceiling,
        1000000000000001127243600,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Nearest,
        1000000000000001127243600,
    );

    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Down,
        1000000000000000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Floor,
        1000000000000000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Up,
        1000000000000000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Ceiling,
        1000000000000000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Nearest,
        1000000000000000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Exact,
        1000000000000000000000000,
    );

    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Down,
        999999999999999999999999,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Floor,
        999999999999999999999999,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Up,
        1000000000001000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Ceiling,
        1000000000001000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Nearest,
        999999999999999999999999,
    );

    test::<i128>(
        2999999999999999999999999,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        2000000000000000000000000,
    );
    test::<i128>(
        3000000000000000000000000,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        4000000000000000000000000,
    );
    test::<i128>(
        3000000000000000000000001,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        4000000000000000000000000,
    );

    test::<i32>(-1, 1, RoundingMode::Down, -1);
    test::<i32>(-1, 1, RoundingMode::Floor, -1);
    test::<i32>(-1, 1, RoundingMode::Up, -1);
    test::<i32>(-1, 1, RoundingMode::Ceiling, -1);
    test::<i32>(-1, 1, RoundingMode::Nearest, -1);
    test::<i32>(-1, 1, RoundingMode::Exact, -1);

    test::<i64>(-123, 1, RoundingMode::Down, -123);
    test::<i64>(-123, 1, RoundingMode::Floor, -123);
    test::<i64>(-123, 1, RoundingMode::Up, -123);
    test::<i64>(-123, 1, RoundingMode::Ceiling, -123);
    test::<i64>(-123, 1, RoundingMode::Nearest, -123);
    test::<i64>(-123, 1, RoundingMode::Exact, -123);

    test::<i128>(-123, 2, RoundingMode::Down, -122);
    test::<i128>(-123, 2, RoundingMode::Floor, -124);
    test::<i128>(-123, 2, RoundingMode::Up, -124);
    test::<i128>(-123, 2, RoundingMode::Ceiling, -122);
    test::<i128>(-123, 2, RoundingMode::Nearest, -124);

    test::<isize>(-125, 2, RoundingMode::Down, -124);
    test::<isize>(-125, 2, RoundingMode::Floor, -126);
    test::<isize>(-125, 2, RoundingMode::Up, -126);
    test::<isize>(-125, 2, RoundingMode::Ceiling, -124);
    test::<isize>(-125, 2, RoundingMode::Nearest, -124);

    test::<i8>(-123, 123, RoundingMode::Down, -123);
    test::<i8>(-123, 123, RoundingMode::Floor, -123);
    test::<i8>(-123, 123, RoundingMode::Up, -123);
    test::<i8>(-123, 123, RoundingMode::Ceiling, -123);
    test::<i8>(-123, 123, RoundingMode::Nearest, -123);
    test::<i8>(-123, 123, RoundingMode::Exact, -123);

    test::<i16>(-123, 456, RoundingMode::Down, 0);
    test::<i16>(-123, 456, RoundingMode::Floor, -456);
    test::<i16>(-123, 456, RoundingMode::Up, -456);
    test::<i16>(-123, 456, RoundingMode::Ceiling, 0);
    test::<i16>(-123, 456, RoundingMode::Nearest, 0);

    test::<i64>(-1000000000000, 1, RoundingMode::Down, -1000000000000);
    test::<i64>(-1000000000000, 1, RoundingMode::Floor, -1000000000000);
    test::<i64>(-1000000000000, 1, RoundingMode::Up, -1000000000000);
    test::<i64>(-1000000000000, 1, RoundingMode::Ceiling, -1000000000000);
    test::<i64>(-1000000000000, 1, RoundingMode::Nearest, -1000000000000);
    test::<i64>(-1000000000000, 1, RoundingMode::Exact, -1000000000000);

    test::<i64>(-1000000000000, 3, RoundingMode::Down, -999999999999);
    test::<i64>(-1000000000000, 3, RoundingMode::Floor, -1000000000002);
    test::<i64>(-1000000000000, 3, RoundingMode::Up, -1000000000002);
    test::<i64>(-1000000000000, 3, RoundingMode::Ceiling, -999999999999);
    test::<i64>(-1000000000000, 3, RoundingMode::Nearest, -999999999999);

    test::<i64>(-999999999999, 2, RoundingMode::Down, -999999999998);
    test::<i64>(-999999999999, 2, RoundingMode::Floor, -1000000000000);
    test::<i64>(-999999999999, 2, RoundingMode::Up, -1000000000000);
    test::<i64>(-999999999999, 2, RoundingMode::Ceiling, -999999999998);
    test::<i64>(-999999999999, 2, RoundingMode::Nearest, -1000000000000);

    test::<i64>(-1000000000001, 2, RoundingMode::Down, -1000000000000);
    test::<i64>(-1000000000001, 2, RoundingMode::Floor, -1000000000002);
    test::<i64>(-1000000000001, 2, RoundingMode::Up, -1000000000002);
    test::<i64>(-1000000000001, 2, RoundingMode::Ceiling, -1000000000000);
    test::<i64>(-1000000000001, 2, RoundingMode::Nearest, -1000000000000);

    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Down,
        -999999999999996832276305,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Floor,
        -1000000000000001127243600,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Up,
        -1000000000000001127243600,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Ceiling,
        -999999999999996832276305,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Nearest,
        -1000000000000001127243600,
    );

    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Down,
        -1000000000000000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Floor,
        -1000000000000000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Up,
        -1000000000000000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Ceiling,
        -1000000000000000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Nearest,
        -1000000000000000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Exact,
        -1000000000000000000000000,
    );

    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Down,
        -999999999999999999999999,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Floor,
        -1000000000001000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Up,
        -1000000000001000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Ceiling,
        -999999999999999999999999,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Nearest,
        -999999999999999999999999,
    );

    test::<i128>(
        -2999999999999999999999999,
        2000000000000000000000000,
        RoundingMode::Nearest,
        -2000000000000000000000000,
    );
    test::<i128>(
        -3000000000000000000000000,
        2000000000000000000000000,
        RoundingMode::Nearest,
        -4000000000000000000000000,
    );
    test::<i128>(
        -3000000000000000000000001,
        2000000000000000000000000,
        RoundingMode::Nearest,
        -4000000000000000000000000,
    );

    test::<i32>(-1, -1, RoundingMode::Down, -1);
    test::<i32>(-1, -1, RoundingMode::Floor, -1);
    test::<i32>(-1, -1, RoundingMode::Up, -1);
    test::<i32>(-1, -1, RoundingMode::Ceiling, -1);
    test::<i32>(-1, -1, RoundingMode::Nearest, -1);
    test::<i32>(-1, -1, RoundingMode::Exact, -1);

    test::<i64>(-123, -1, RoundingMode::Down, -123);
    test::<i64>(-123, -1, RoundingMode::Floor, -123);
    test::<i64>(-123, -1, RoundingMode::Up, -123);
    test::<i64>(-123, -1, RoundingMode::Ceiling, -123);
    test::<i64>(-123, -1, RoundingMode::Nearest, -123);
    test::<i64>(-123, -1, RoundingMode::Exact, -123);

    test::<i128>(-123, -2, RoundingMode::Down, -122);
    test::<i128>(-123, -2, RoundingMode::Floor, -124);
    test::<i128>(-123, -2, RoundingMode::Up, -124);
    test::<i128>(-123, -2, RoundingMode::Ceiling, -122);
    test::<i128>(-123, -2, RoundingMode::Nearest, -124);

    test::<isize>(-125, -2, RoundingMode::Down, -124);
    test::<isize>(-125, -2, RoundingMode::Floor, -126);
    test::<isize>(-125, -2, RoundingMode::Up, -126);
    test::<isize>(-125, -2, RoundingMode::Ceiling, -124);
    test::<isize>(-125, -2, RoundingMode::Nearest, -124);

    test::<i8>(-123, -123, RoundingMode::Down, -123);
    test::<i8>(-123, -123, RoundingMode::Floor, -123);
    test::<i8>(-123, -123, RoundingMode::Up, -123);
    test::<i8>(-123, -123, RoundingMode::Ceiling, -123);
    test::<i8>(-123, -123, RoundingMode::Nearest, -123);
    test::<i8>(-123, -123, RoundingMode::Exact, -123);

    test::<i16>(-123, -456, RoundingMode::Down, 0);
    test::<i16>(-123, -456, RoundingMode::Floor, -456);
    test::<i16>(-123, -456, RoundingMode::Up, -456);
    test::<i16>(-123, -456, RoundingMode::Ceiling, 0);
    test::<i16>(-123, -456, RoundingMode::Nearest, 0);

    test::<i64>(-1000000000000, -1, RoundingMode::Down, -1000000000000);
    test::<i64>(-1000000000000, -1, RoundingMode::Floor, -1000000000000);
    test::<i64>(-1000000000000, -1, RoundingMode::Up, -1000000000000);
    test::<i64>(-1000000000000, -1, RoundingMode::Ceiling, -1000000000000);
    test::<i64>(-1000000000000, -1, RoundingMode::Nearest, -1000000000000);
    test::<i64>(-1000000000000, -1, RoundingMode::Exact, -1000000000000);

    test::<i64>(-1000000000000, -3, RoundingMode::Down, -999999999999);
    test::<i64>(-1000000000000, -3, RoundingMode::Floor, -1000000000002);
    test::<i64>(-1000000000000, -3, RoundingMode::Up, -1000000000002);
    test::<i64>(-1000000000000, -3, RoundingMode::Ceiling, -999999999999);
    test::<i64>(-1000000000000, -3, RoundingMode::Nearest, -999999999999);

    test::<i64>(-999999999999, -2, RoundingMode::Down, -999999999998);
    test::<i64>(-999999999999, -2, RoundingMode::Floor, -1000000000000);
    test::<i64>(-999999999999, -2, RoundingMode::Up, -1000000000000);
    test::<i64>(-999999999999, -2, RoundingMode::Ceiling, -999999999998);
    test::<i64>(-999999999999, -2, RoundingMode::Nearest, -1000000000000);

    test::<i64>(-1000000000001, -2, RoundingMode::Down, -1000000000000);
    test::<i64>(-1000000000001, -2, RoundingMode::Floor, -1000000000002);
    test::<i64>(-1000000000001, -2, RoundingMode::Up, -1000000000002);
    test::<i64>(-1000000000001, -2, RoundingMode::Ceiling, -1000000000000);
    test::<i64>(-1000000000001, -2, RoundingMode::Nearest, -1000000000000);

    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Down,
        -999999999999996832276305,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Floor,
        -1000000000000001127243600,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Up,
        -1000000000000001127243600,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Ceiling,
        -999999999999996832276305,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Nearest,
        -1000000000000001127243600,
    );

    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Down,
        -1000000000000000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Floor,
        -1000000000000000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Up,
        -1000000000000000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Ceiling,
        -1000000000000000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Nearest,
        -1000000000000000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Exact,
        -1000000000000000000000000,
    );

    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Down,
        -999999999999999999999999,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Floor,
        -1000000000001000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Up,
        -1000000000001000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Ceiling,
        -999999999999999999999999,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Nearest,
        -999999999999999999999999,
    );

    test::<i128>(
        -2999999999999999999999999,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        -2000000000000000000000000,
    );
    test::<i128>(
        -3000000000000000000000000,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        -4000000000000000000000000,
    );
    test::<i128>(
        -3000000000000000000000001,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        -4000000000000000000000000,
    );

    test::<i8>(-128, 1, RoundingMode::Down, -128);
    test::<i8>(-128, 1, RoundingMode::Up, -128);
    test::<i8>(-128, 1, RoundingMode::Floor, -128);
    test::<i8>(-128, 1, RoundingMode::Ceiling, -128);
    test::<i8>(-128, 1, RoundingMode::Nearest, -128);
    test::<i8>(-128, 1, RoundingMode::Exact, -128);

    test::<i8>(-128, -1, RoundingMode::Down, -128);
    test::<i8>(-128, -1, RoundingMode::Up, -128);
    test::<i8>(-128, -1, RoundingMode::Floor, -128);
    test::<i8>(-128, -1, RoundingMode::Ceiling, -128);
    test::<i8>(-128, -1, RoundingMode::Nearest, -128);
    test::<i8>(-128, -1, RoundingMode::Exact, -128);

    test::<i8>(-128, -128, RoundingMode::Down, -128);
    test::<i8>(-128, -128, RoundingMode::Up, -128);
    test::<i8>(-128, -128, RoundingMode::Floor, -128);
    test::<i8>(-128, -128, RoundingMode::Ceiling, -128);
    test::<i8>(-128, -128, RoundingMode::Nearest, -128);
    test::<i8>(-128, -128, RoundingMode::Exact, -128);

    test::<i8>(0, 0, RoundingMode::Floor, 0);
    test::<i16>(0, 0, RoundingMode::Ceiling, 0);
    test::<i32>(0, 0, RoundingMode::Down, 0);
    test::<i64>(0, 0, RoundingMode::Up, 0);
    test::<i128>(0, 0, RoundingMode::Nearest, 0);
    test::<isize>(0, 0, RoundingMode::Exact, 0);

    test::<i8>(2, 0, RoundingMode::Floor, 0);
    test::<i16>(2, 0, RoundingMode::Down, 0);
    test::<i32>(2, 0, RoundingMode::Nearest, 0);
    test::<i64>(-2, 0, RoundingMode::Ceiling, 0);
    test::<i128>(-2, 0, RoundingMode::Down, 0);
    test::<isize>(-2, 0, RoundingMode::Nearest, 0);
}

fn round_to_multiple_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::exact_from(10).round_to_multiple(T::ZERO, RoundingMode::Up));
    assert_panic!(T::exact_from(10).round_to_multiple(T::exact_from(3), RoundingMode::Exact));
    assert_panic!(T::MAX.round_to_multiple(T::TWO, RoundingMode::Ceiling));
    assert_panic!(T::ONE.round_to_multiple(T::ZERO, RoundingMode::Up));
    assert_panic!(T::ONE.round_to_multiple(T::ZERO, RoundingMode::Ceiling));
    assert_panic!(T::ONE.round_to_multiple(T::ZERO, RoundingMode::Exact));

    assert_panic!(T::exact_from(10).round_to_multiple_assign(T::ZERO, RoundingMode::Up));
    assert_panic!({
        T::exact_from(10).round_to_multiple_assign(T::exact_from(3), RoundingMode::Exact);
    });
    assert_panic!({
        let mut n = T::MAX;
        n.round_to_multiple_assign(T::TWO, RoundingMode::Ceiling);
    });
    assert_panic!({
        let mut n = T::ONE;
        n.round_to_multiple_assign(T::ZERO, RoundingMode::Up);
    });
    assert_panic!({
        let mut n = T::ONE;
        n.round_to_multiple_assign(T::ZERO, RoundingMode::Ceiling);
    });
    assert_panic!({
        let mut n = T::ONE;
        n.round_to_multiple_assign(T::ZERO, RoundingMode::Exact);
    });
}

fn round_to_multiple_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::MIN.round_to_multiple(T::exact_from(3), RoundingMode::Floor));
    assert_panic!(T::NEGATIVE_ONE.round_to_multiple(T::ZERO, RoundingMode::Up));
    assert_panic!(T::NEGATIVE_ONE.round_to_multiple(T::ZERO, RoundingMode::Floor));
    assert_panic!(T::NEGATIVE_ONE.round_to_multiple(T::ZERO, RoundingMode::Exact));

    assert_panic!({
        let mut n = T::MIN;
        n.round_to_multiple_assign(T::exact_from(3), RoundingMode::Floor);
    });
    assert_panic!({
        let mut n = T::NEGATIVE_ONE;
        n.round_to_multiple_assign(T::ZERO, RoundingMode::Up);
    });
    assert_panic!({
        let mut n = T::NEGATIVE_ONE;
        n.round_to_multiple_assign(T::ZERO, RoundingMode::Floor);
    });
    assert_panic!({
        let mut n = T::NEGATIVE_ONE;
        n.round_to_multiple_assign(T::ZERO, RoundingMode::Exact);
    });
}

#[test]
fn round_to_multiple_fail() {
    apply_fn_to_primitive_ints!(round_to_multiple_fail_helper);
    apply_fn_to_signeds!(round_to_multiple_signed_fail_helper);
}

fn round_to_multiple_properties_helper_unsigned<T: PrimitiveUnsigned>() {
    unsigned_unsigned_rounding_mode_triple_gen_var_2::<T>().test_properties(|(x, y, rm)| {
        let rounded = x.round_to_multiple(y, rm);

        let mut mut_x = x;
        mut_x.round_to_multiple_assign(y, rm);
        assert_eq!(mut_x, rounded);

        assert!(rounded.divisible_by(y));
        match rm {
            RoundingMode::Floor | RoundingMode::Down => {
                assert!(rounded <= x)
            }
            RoundingMode::Ceiling | RoundingMode::Up => {
                assert!(rounded >= x)
            }
            RoundingMode::Exact => assert_eq!(rounded, x),
            RoundingMode::Nearest => {
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
            assert_eq!(product.round_to_multiple(y, RoundingMode::Down), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Up), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Floor), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Ceiling), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Nearest), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Exact), product);
        }
    });

    unsigned_pair_gen_var_13::<T>().test_properties(|(x, y)| {
        let down = x.round_to_multiple(y, RoundingMode::Down);
        if let Some(up) = down.checked_add(y) {
            assert_eq!(x.round_to_multiple(y, RoundingMode::Up), up);
            assert_eq!(x.round_to_multiple(y, RoundingMode::Floor), down);
            assert_eq!(x.round_to_multiple(y, RoundingMode::Ceiling), up);
            let nearest = x.round_to_multiple(y, RoundingMode::Nearest);
            assert!(nearest == down || nearest == up);
        }
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(n, rm)| {
        if rm == RoundingMode::Floor || rm == RoundingMode::Down || rm == RoundingMode::Nearest {
            assert_eq!(n.round_to_multiple(T::ZERO, rm), T::ZERO);
        }
        assert_eq!(T::ZERO.round_to_multiple(n, rm), T::ZERO);
        assert_eq!(n.round_to_multiple(T::ONE, rm), n);
        assert_eq!(n.round_to_multiple(n, rm), n);
    });
}

fn round_to_multiple_properties_helper_signed<
    U: PrimitiveUnsigned,
    S: CheckedFrom<U> + ConvertibleFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>() {
    signed_signed_rounding_mode_triple_gen_var_2::<U, S>().test_properties(|(x, y, rm)| {
        let rounded = x.round_to_multiple(y, rm);

        let mut mut_x = x;
        mut_x.round_to_multiple_assign(y, rm);
        assert_eq!(mut_x, rounded);

        assert!(rounded.divisible_by(y));
        match rm {
            RoundingMode::Floor => assert!(rounded <= x),
            RoundingMode::Ceiling => assert!(rounded >= x),
            RoundingMode::Down => assert!(rounded.le_abs(&x)),
            RoundingMode::Up => assert!(rounded.ge_abs(&x)),
            RoundingMode::Exact => assert_eq!(rounded, x),
            RoundingMode::Nearest => {
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
            assert_eq!(product.round_to_multiple(y, RoundingMode::Down), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Up), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Floor), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Ceiling), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Nearest), product);
            assert_eq!(product.round_to_multiple(y, RoundingMode::Exact), product);
        }
    });

    signed_pair_gen_var_5::<S>().test_properties(|(x, y)| {
        let down = x.round_to_multiple(y, RoundingMode::Down);
        if let Some(up) = if (x >= S::ZERO) == (y >= S::ZERO) {
            down.checked_add(y)
        } else {
            down.checked_sub(y)
        } {
            assert_eq!(x.round_to_multiple(y, RoundingMode::Up), up);
            if x >= S::ZERO {
                assert_eq!(x.round_to_multiple(y, RoundingMode::Floor), down);
                assert_eq!(x.round_to_multiple(y, RoundingMode::Ceiling), up);
            } else {
                assert_eq!(x.round_to_multiple(y, RoundingMode::Floor), up);
                assert_eq!(x.round_to_multiple(y, RoundingMode::Ceiling), down);
            }
            let nearest = x.round_to_multiple(y, RoundingMode::Nearest);
            assert!(nearest == down || nearest == up);
        }
    });

    signed_rounding_mode_pair_gen::<S>().test_properties(|(n, rm)| {
        if rm == RoundingMode::Down
            || rm == RoundingMode::Nearest
            || rm
                == if n >= S::ZERO {
                    RoundingMode::Floor
                } else {
                    RoundingMode::Ceiling
                }
        {
            assert_eq!(n.round_to_multiple(S::ZERO, rm), S::ZERO);
        }
        assert_eq!(S::ZERO.round_to_multiple(n, rm), S::ZERO);
        assert_eq!(n.round_to_multiple(S::ONE, rm), n);
        assert_eq!(n.round_to_multiple(n, rm), n);
    });
}

#[test]
fn round_to_multiple_properties() {
    apply_fn_to_unsigneds!(round_to_multiple_properties_helper_unsigned);
    apply_fn_to_unsigned_signed_pairs!(round_to_multiple_properties_helper_signed);
}
