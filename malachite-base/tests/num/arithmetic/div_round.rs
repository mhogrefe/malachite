use std::panic::catch_unwind;

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode;

#[test]
fn test_div_round_unsigned() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, rm: RoundingMode, q: T) {
        assert_eq!(n.div_round(d, rm), q);

        let mut mut_n = n;
        mut_n.div_round_assign(d, rm);
        assert_eq!(mut_n, q);
    };
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

    test::<u128>(123, 2, RoundingMode::Down, 61);
    test::<u128>(123, 2, RoundingMode::Floor, 61);
    test::<u128>(123, 2, RoundingMode::Up, 62);
    test::<u128>(123, 2, RoundingMode::Ceiling, 62);
    test::<u128>(123, 2, RoundingMode::Nearest, 62);

    test::<usize>(125, 2, RoundingMode::Down, 62);
    test::<usize>(125, 2, RoundingMode::Floor, 62);
    test::<usize>(125, 2, RoundingMode::Up, 63);
    test::<usize>(125, 2, RoundingMode::Ceiling, 63);
    test::<usize>(125, 2, RoundingMode::Nearest, 62);

    test::<u8>(123, 123, RoundingMode::Down, 1);
    test::<u8>(123, 123, RoundingMode::Floor, 1);
    test::<u8>(123, 123, RoundingMode::Up, 1);
    test::<u8>(123, 123, RoundingMode::Ceiling, 1);
    test::<u8>(123, 123, RoundingMode::Nearest, 1);
    test::<u8>(123, 123, RoundingMode::Exact, 1);

    test::<u16>(123, 456, RoundingMode::Down, 0);
    test::<u16>(123, 456, RoundingMode::Floor, 0);
    test::<u16>(123, 456, RoundingMode::Up, 1);
    test::<u16>(123, 456, RoundingMode::Ceiling, 1);
    test::<u16>(123, 456, RoundingMode::Nearest, 0);

    test::<u64>(1_000_000_000_000, 1, RoundingMode::Down, 1_000_000_000_000);
    test::<u64>(1_000_000_000_000, 1, RoundingMode::Floor, 1_000_000_000_000);
    test::<u64>(1_000_000_000_000, 1, RoundingMode::Up, 1_000_000_000_000);
    test::<u64>(
        1_000_000_000_000,
        1,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<u64>(
        1_000_000_000_000,
        1,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );
    test::<u64>(1_000_000_000_000, 1, RoundingMode::Exact, 1_000_000_000_000);

    test::<u64>(1_000_000_000_000, 3, RoundingMode::Down, 333_333_333_333);
    test::<u64>(1_000_000_000_000, 3, RoundingMode::Floor, 333_333_333_333);
    test::<u64>(1_000_000_000_000, 3, RoundingMode::Up, 333_333_333_334);
    test::<u64>(1_000_000_000_000, 3, RoundingMode::Ceiling, 333_333_333_334);
    test::<u64>(1_000_000_000_000, 3, RoundingMode::Nearest, 333_333_333_333);

    test::<u64>(999_999_999_999, 2, RoundingMode::Down, 499_999_999_999);
    test::<u64>(999_999_999_999, 2, RoundingMode::Floor, 499_999_999_999);
    test::<u64>(999_999_999_999, 2, RoundingMode::Up, 500_000_000_000);
    test::<u64>(999_999_999_999, 2, RoundingMode::Ceiling, 500_000_000_000);
    test::<u64>(999_999_999_999, 2, RoundingMode::Nearest, 500_000_000_000);

    test::<u64>(1_000_000_000_001, 2, RoundingMode::Down, 500_000_000_000);
    test::<u64>(1_000_000_000_001, 2, RoundingMode::Floor, 500_000_000_000);
    test::<u64>(1_000_000_000_001, 2, RoundingMode::Up, 500_000_000_001);
    test::<u64>(1_000_000_000_001, 2, RoundingMode::Ceiling, 500_000_000_001);
    test::<u64>(1_000_000_000_001, 2, RoundingMode::Nearest, 500_000_000_000);

    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Down,
        232_830_643_708_079,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Floor,
        232_830_643_708_079,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Up,
        232_830_643_708_080,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Ceiling,
        232_830_643_708_080,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Nearest,
        232_830_643_708_080,
    );

    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Down,
        1_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Floor,
        1_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Up,
        1_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Exact,
        1_000_000_000_000,
    );

    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Down,
        999_999_999_999,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Floor,
        999_999_999_999,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Up,
        1_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Nearest,
        999_999_999_999,
    );

    test::<u128>(
        2_999_999_999_999_999_999_999_999,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        1,
    );
    test::<u128>(
        3_000_000_000_000_000_000_000_000,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        2,
    );
    test::<u128>(
        3_000_000_000_000_000_000_000_001,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        2,
    );
}

#[test]
fn test_div_round_signed() {
    fn test<T: PrimitiveSigned>(n: T, d: T, rm: RoundingMode, q: T) {
        assert_eq!(n.div_round(d, rm), q);

        let mut mut_n = n;
        mut_n.div_round_assign(d, rm);
        assert_eq!(mut_n, q);
    };
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

    test::<i128>(123, 2, RoundingMode::Down, 61);
    test::<i128>(123, 2, RoundingMode::Floor, 61);
    test::<i128>(123, 2, RoundingMode::Up, 62);
    test::<i128>(123, 2, RoundingMode::Ceiling, 62);
    test::<i128>(123, 2, RoundingMode::Nearest, 62);

    test::<isize>(125, 2, RoundingMode::Down, 62);
    test::<isize>(125, 2, RoundingMode::Floor, 62);
    test::<isize>(125, 2, RoundingMode::Up, 63);
    test::<isize>(125, 2, RoundingMode::Ceiling, 63);
    test::<isize>(125, 2, RoundingMode::Nearest, 62);

    test::<i8>(123, 123, RoundingMode::Down, 1);
    test::<i8>(123, 123, RoundingMode::Floor, 1);
    test::<i8>(123, 123, RoundingMode::Up, 1);
    test::<i8>(123, 123, RoundingMode::Ceiling, 1);
    test::<i8>(123, 123, RoundingMode::Nearest, 1);
    test::<i8>(123, 123, RoundingMode::Exact, 1);

    test::<i16>(123, 456, RoundingMode::Down, 0);
    test::<i16>(123, 456, RoundingMode::Floor, 0);
    test::<i16>(123, 456, RoundingMode::Up, 1);
    test::<i16>(123, 456, RoundingMode::Ceiling, 1);
    test::<i16>(123, 456, RoundingMode::Nearest, 0);

    test::<i64>(1_000_000_000_000, 1, RoundingMode::Down, 1_000_000_000_000);
    test::<i64>(1_000_000_000_000, 1, RoundingMode::Floor, 1_000_000_000_000);
    test::<i64>(1_000_000_000_000, 1, RoundingMode::Up, 1_000_000_000_000);
    test::<i64>(
        1_000_000_000_000,
        1,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<i64>(
        1_000_000_000_000,
        1,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );
    test::<i64>(1_000_000_000_000, 1, RoundingMode::Exact, 1_000_000_000_000);

    test::<i64>(1_000_000_000_000, 3, RoundingMode::Down, 333_333_333_333);
    test::<i64>(1_000_000_000_000, 3, RoundingMode::Floor, 333_333_333_333);
    test::<i64>(1_000_000_000_000, 3, RoundingMode::Up, 333_333_333_334);
    test::<i64>(1_000_000_000_000, 3, RoundingMode::Ceiling, 333_333_333_334);
    test::<i64>(1_000_000_000_000, 3, RoundingMode::Nearest, 333_333_333_333);

    test::<i64>(999_999_999_999, 2, RoundingMode::Down, 499_999_999_999);
    test::<i64>(999_999_999_999, 2, RoundingMode::Floor, 499_999_999_999);
    test::<i64>(999_999_999_999, 2, RoundingMode::Up, 500_000_000_000);
    test::<i64>(999_999_999_999, 2, RoundingMode::Ceiling, 500_000_000_000);
    test::<i64>(999_999_999_999, 2, RoundingMode::Nearest, 500_000_000_000);

    test::<i64>(1_000_000_000_001, 2, RoundingMode::Down, 500_000_000_000);
    test::<i64>(1_000_000_000_001, 2, RoundingMode::Floor, 500_000_000_000);
    test::<i64>(1_000_000_000_001, 2, RoundingMode::Up, 500_000_000_001);
    test::<i64>(1_000_000_000_001, 2, RoundingMode::Ceiling, 500_000_000_001);
    test::<i64>(1_000_000_000_001, 2, RoundingMode::Nearest, 500_000_000_000);

    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Down,
        232_830_643_708_079,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Floor,
        232_830_643_708_079,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Up,
        232_830_643_708_080,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Ceiling,
        232_830_643_708_080,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Nearest,
        232_830_643_708_080,
    );

    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Down,
        1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Floor,
        1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Up,
        1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Exact,
        1_000_000_000_000,
    );

    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Down,
        999_999_999_999,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Floor,
        999_999_999_999,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Up,
        1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Nearest,
        999_999_999_999,
    );

    test::<i128>(
        2_999_999_999_999_999_999_999_999,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        1,
    );
    test::<i128>(
        3_000_000_000_000_000_000_000_000,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        2,
    );
    test::<i128>(
        3_000_000_000_000_000_000_000_001,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        2,
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

    test::<i32>(1, -1, RoundingMode::Down, -1);
    test::<i32>(1, -1, RoundingMode::Floor, -1);
    test::<i32>(1, -1, RoundingMode::Up, -1);
    test::<i32>(1, -1, RoundingMode::Ceiling, -1);
    test::<i32>(1, -1, RoundingMode::Nearest, -1);
    test::<i32>(1, -1, RoundingMode::Exact, -1);

    test::<i64>(123, -1, RoundingMode::Down, -123);
    test::<i64>(123, -1, RoundingMode::Floor, -123);
    test::<i64>(123, -1, RoundingMode::Up, -123);
    test::<i64>(123, -1, RoundingMode::Ceiling, -123);
    test::<i64>(123, -1, RoundingMode::Nearest, -123);
    test::<i64>(123, -1, RoundingMode::Exact, -123);

    test::<i128>(123, -2, RoundingMode::Down, -61);
    test::<i128>(123, -2, RoundingMode::Floor, -62);
    test::<i128>(123, -2, RoundingMode::Up, -62);
    test::<i128>(123, -2, RoundingMode::Ceiling, -61);
    test::<i128>(123, -2, RoundingMode::Nearest, -62);

    test::<isize>(125, -2, RoundingMode::Down, -62);
    test::<isize>(125, -2, RoundingMode::Floor, -63);
    test::<isize>(125, -2, RoundingMode::Up, -63);
    test::<isize>(125, -2, RoundingMode::Ceiling, -62);
    test::<isize>(125, -2, RoundingMode::Nearest, -62);

    test::<i8>(123, -123, RoundingMode::Down, -1);
    test::<i8>(123, -123, RoundingMode::Floor, -1);
    test::<i8>(123, -123, RoundingMode::Up, -1);
    test::<i8>(123, -123, RoundingMode::Ceiling, -1);
    test::<i8>(123, -123, RoundingMode::Nearest, -1);
    test::<i8>(123, -123, RoundingMode::Exact, -1);

    test::<i16>(123, -456, RoundingMode::Down, 0);
    test::<i16>(123, -456, RoundingMode::Floor, -1);
    test::<i16>(123, -456, RoundingMode::Up, -1);
    test::<i16>(123, -456, RoundingMode::Ceiling, 0);
    test::<i16>(123, -456, RoundingMode::Nearest, 0);

    test::<i64>(
        1_000_000_000_000,
        -1,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i64>(
        1_000_000_000_000,
        -1,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i64>(1_000_000_000_000, -1, RoundingMode::Up, -1_000_000_000_000);
    test::<i64>(
        1_000_000_000_000,
        -1,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i64>(
        1_000_000_000_000,
        -1,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i64>(
        1_000_000_000_000,
        -1,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i64>(1_000_000_000_000, -3, RoundingMode::Down, -333_333_333_333);
    test::<i64>(1_000_000_000_000, -3, RoundingMode::Floor, -333_333_333_334);
    test::<i64>(1_000_000_000_000, -3, RoundingMode::Up, -333_333_333_334);
    test::<i64>(
        1_000_000_000_000,
        -3,
        RoundingMode::Ceiling,
        -333_333_333_333,
    );
    test::<i64>(
        1_000_000_000_000,
        -3,
        RoundingMode::Nearest,
        -333_333_333_333,
    );

    test::<i64>(999_999_999_999, -2, RoundingMode::Down, -499_999_999_999);
    test::<i64>(999_999_999_999, -2, RoundingMode::Floor, -500_000_000_000);
    test::<i64>(999_999_999_999, -2, RoundingMode::Up, -500_000_000_000);
    test::<i64>(999_999_999_999, -2, RoundingMode::Ceiling, -499_999_999_999);
    test::<i64>(999_999_999_999, -2, RoundingMode::Nearest, -500_000_000_000);

    test::<i64>(1_000_000_000_001, -2, RoundingMode::Down, -500_000_000_000);
    test::<i64>(1_000_000_000_001, -2, RoundingMode::Floor, -500_000_000_001);
    test::<i64>(1_000_000_000_001, -2, RoundingMode::Up, -500_000_000_001);
    test::<i64>(
        1_000_000_000_001,
        -2,
        RoundingMode::Ceiling,
        -500_000_000_000,
    );
    test::<i64>(
        1_000_000_000_001,
        -2,
        RoundingMode::Nearest,
        -500_000_000_000,
    );

    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Down,
        -232_830_643_708_079,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Floor,
        -232_830_643_708_080,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Up,
        -232_830_643_708_080,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Ceiling,
        -232_830_643_708_079,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Nearest,
        -232_830_643_708_080,
    );

    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Down,
        -999_999_999_999,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Ceiling,
        -999_999_999_999,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Nearest,
        -999_999_999_999,
    );

    test::<i128>(
        2_999_999_999_999_999_999_999_999,
        -2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        -1,
    );
    test::<i128>(
        3_000_000_000_000_000_000_000_000,
        -2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        -2,
    );
    test::<i128>(
        3_000_000_000_000_000_000_000_001,
        -2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        -2,
    );

    test::<i8>(-1, 1, RoundingMode::Down, -1);
    test::<i8>(-1, 1, RoundingMode::Floor, -1);
    test::<i8>(-1, 1, RoundingMode::Up, -1);
    test::<i8>(-1, 1, RoundingMode::Ceiling, -1);
    test::<i8>(-1, 1, RoundingMode::Nearest, -1);
    test::<i8>(-1, 1, RoundingMode::Exact, -1);

    test::<i16>(-123, 1, RoundingMode::Down, -123);
    test::<i16>(-123, 1, RoundingMode::Floor, -123);
    test::<i16>(-123, 1, RoundingMode::Up, -123);
    test::<i16>(-123, 1, RoundingMode::Ceiling, -123);
    test::<i16>(-123, 1, RoundingMode::Nearest, -123);
    test::<i16>(-123, 1, RoundingMode::Exact, -123);

    test::<i32>(-123, 2, RoundingMode::Down, -61);
    test::<i32>(-123, 2, RoundingMode::Floor, -62);
    test::<i32>(-123, 2, RoundingMode::Up, -62);
    test::<i32>(-123, 2, RoundingMode::Ceiling, -61);
    test::<i32>(-123, 2, RoundingMode::Nearest, -62);

    test::<i64>(-125, 2, RoundingMode::Down, -62);
    test::<i64>(-125, 2, RoundingMode::Floor, -63);
    test::<i64>(-125, 2, RoundingMode::Up, -63);
    test::<i64>(-125, 2, RoundingMode::Ceiling, -62);
    test::<i64>(-125, 2, RoundingMode::Nearest, -62);

    test::<i128>(-123, 123, RoundingMode::Down, -1);
    test::<i128>(-123, 123, RoundingMode::Floor, -1);
    test::<i128>(-123, 123, RoundingMode::Up, -1);
    test::<i128>(-123, 123, RoundingMode::Ceiling, -1);
    test::<i128>(-123, 123, RoundingMode::Nearest, -1);
    test::<i128>(-123, 123, RoundingMode::Exact, -1);

    test::<isize>(-123, 456, RoundingMode::Down, 0);
    test::<isize>(-123, 456, RoundingMode::Floor, -1);
    test::<isize>(-123, 456, RoundingMode::Up, -1);
    test::<isize>(-123, 456, RoundingMode::Ceiling, 0);
    test::<isize>(-123, 456, RoundingMode::Nearest, 0);

    test::<i64>(
        -1_000_000_000_000,
        1,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_000,
        1,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i64>(-1_000_000_000_000, 1, RoundingMode::Up, -1_000_000_000_000);
    test::<i64>(
        -1_000_000_000_000,
        1,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_000,
        1,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_000,
        1,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i64>(-1_000_000_000_000, 3, RoundingMode::Down, -333_333_333_333);
    test::<i64>(-1_000_000_000_000, 3, RoundingMode::Floor, -333_333_333_334);
    test::<i64>(-1_000_000_000_000, 3, RoundingMode::Up, -333_333_333_334);
    test::<i64>(
        -1_000_000_000_000,
        3,
        RoundingMode::Ceiling,
        -333_333_333_333,
    );
    test::<i64>(
        -1_000_000_000_000,
        3,
        RoundingMode::Nearest,
        -333_333_333_333,
    );

    test::<i64>(-999_999_999_999, 2, RoundingMode::Down, -499_999_999_999);
    test::<i64>(-999_999_999_999, 2, RoundingMode::Floor, -500_000_000_000);
    test::<i64>(-999_999_999_999, 2, RoundingMode::Up, -500_000_000_000);
    test::<i64>(-999_999_999_999, 2, RoundingMode::Ceiling, -499_999_999_999);
    test::<i64>(-999_999_999_999, 2, RoundingMode::Nearest, -500_000_000_000);

    test::<i64>(-1_000_000_000_001, 2, RoundingMode::Down, -500_000_000_000);
    test::<i64>(-1_000_000_000_001, 2, RoundingMode::Floor, -500_000_000_001);
    test::<i64>(-1_000_000_000_001, 2, RoundingMode::Up, -500_000_000_001);
    test::<i64>(
        -1_000_000_000_001,
        2,
        RoundingMode::Ceiling,
        -500_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_001,
        2,
        RoundingMode::Nearest,
        -500_000_000_000,
    );

    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Down,
        -232_830_643_708_079,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Floor,
        -232_830_643_708_080,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Up,
        -232_830_643_708_080,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Ceiling,
        -232_830_643_708_079,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Nearest,
        -232_830_643_708_080,
    );

    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Down,
        -999_999_999_999,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Ceiling,
        -999_999_999_999,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Nearest,
        -999_999_999_999,
    );

    test::<i128>(
        -2_999_999_999_999_999_999_999_999,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        -1,
    );
    test::<i128>(
        -3_000_000_000_000_000_000_000_000,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        -2,
    );
    test::<i128>(
        -3_000_000_000_000_000_000_000_001,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        -2,
    );

    test::<i8>(-1, -1, RoundingMode::Down, 1);
    test::<i8>(-1, -1, RoundingMode::Floor, 1);
    test::<i8>(-1, -1, RoundingMode::Up, 1);
    test::<i8>(-1, -1, RoundingMode::Ceiling, 1);
    test::<i8>(-1, -1, RoundingMode::Nearest, 1);
    test::<i8>(-1, -1, RoundingMode::Exact, 1);

    test::<i16>(-123, -1, RoundingMode::Down, 123);
    test::<i16>(-123, -1, RoundingMode::Floor, 123);
    test::<i16>(-123, -1, RoundingMode::Up, 123);
    test::<i16>(-123, -1, RoundingMode::Ceiling, 123);
    test::<i16>(-123, -1, RoundingMode::Nearest, 123);
    test::<i16>(-123, -1, RoundingMode::Exact, 123);

    test::<i32>(-123, -2, RoundingMode::Down, 61);
    test::<i32>(-123, -2, RoundingMode::Floor, 61);
    test::<i32>(-123, -2, RoundingMode::Up, 62);
    test::<i32>(-123, -2, RoundingMode::Ceiling, 62);
    test::<i32>(-123, -2, RoundingMode::Nearest, 62);

    test::<i64>(-125, -2, RoundingMode::Down, 62);
    test::<i64>(-125, -2, RoundingMode::Floor, 62);
    test::<i64>(-125, -2, RoundingMode::Up, 63);
    test::<i64>(-125, -2, RoundingMode::Ceiling, 63);
    test::<i64>(-125, -2, RoundingMode::Nearest, 62);

    test::<i128>(-123, -123, RoundingMode::Down, 1);
    test::<i128>(-123, -123, RoundingMode::Floor, 1);
    test::<i128>(-123, -123, RoundingMode::Up, 1);
    test::<i128>(-123, -123, RoundingMode::Ceiling, 1);
    test::<i128>(-123, -123, RoundingMode::Nearest, 1);
    test::<i128>(-123, -123, RoundingMode::Exact, 1);

    test::<isize>(-123, -456, RoundingMode::Down, 0);
    test::<isize>(-123, -456, RoundingMode::Floor, 0);
    test::<isize>(-123, -456, RoundingMode::Up, 1);
    test::<isize>(-123, -456, RoundingMode::Ceiling, 1);
    test::<isize>(-123, -456, RoundingMode::Nearest, 0);

    test::<i64>(
        -1_000_000_000_000,
        -1,
        RoundingMode::Down,
        1_000_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_000,
        -1,
        RoundingMode::Floor,
        1_000_000_000_000,
    );
    test::<i64>(-1_000_000_000_000, -1, RoundingMode::Up, 1_000_000_000_000);
    test::<i64>(
        -1_000_000_000_000,
        -1,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_000,
        -1,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_000,
        -1,
        RoundingMode::Exact,
        1_000_000_000_000,
    );

    test::<i64>(-1_000_000_000_000, -3, RoundingMode::Down, 333_333_333_333);
    test::<i64>(-1_000_000_000_000, -3, RoundingMode::Floor, 333_333_333_333);
    test::<i64>(-1_000_000_000_000, -3, RoundingMode::Up, 333_333_333_334);
    test::<i64>(
        -1_000_000_000_000,
        -3,
        RoundingMode::Ceiling,
        333_333_333_334,
    );
    test::<i64>(
        -1_000_000_000_000,
        -3,
        RoundingMode::Nearest,
        333_333_333_333,
    );

    test::<i64>(-999_999_999_999, -2, RoundingMode::Down, 499_999_999_999);
    test::<i64>(-999_999_999_999, -2, RoundingMode::Floor, 499_999_999_999);
    test::<i64>(-999_999_999_999, -2, RoundingMode::Up, 500_000_000_000);
    test::<i64>(-999_999_999_999, -2, RoundingMode::Ceiling, 500_000_000_000);
    test::<i64>(-999_999_999_999, -2, RoundingMode::Nearest, 500_000_000_000);

    test::<i64>(-1_000_000_000_001, -2, RoundingMode::Down, 500_000_000_000);
    test::<i64>(-1_000_000_000_001, -2, RoundingMode::Floor, 500_000_000_000);
    test::<i64>(-1_000_000_000_001, -2, RoundingMode::Up, 500_000_000_001);
    test::<i64>(
        -1_000_000_000_001,
        -2,
        RoundingMode::Ceiling,
        500_000_000_001,
    );
    test::<i64>(
        -1_000_000_000_001,
        -2,
        RoundingMode::Nearest,
        500_000_000_000,
    );

    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Down,
        232_830_643_708_079,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Floor,
        232_830_643_708_079,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Up,
        232_830_643_708_080,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Ceiling,
        232_830_643_708_080,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Nearest,
        232_830_643_708_080,
    );

    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Down,
        1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Floor,
        1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Up,
        1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Exact,
        1_000_000_000_000,
    );

    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Down,
        999_999_999_999,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Floor,
        999_999_999_999,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Up,
        1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Nearest,
        999_999_999_999,
    );

    test::<i128>(
        -2_999_999_999_999_999_999_999_999,
        -2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        1,
    );
    test::<i128>(
        -3_000_000_000_000_000_000_000_000,
        -2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        2,
    );
    test::<i128>(
        -3_000_000_000_000_000_000_000_001,
        -2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        2,
    );

    test::<i8>(-128, 1, RoundingMode::Down, -128);
    test::<i8>(-128, 1, RoundingMode::Up, -128);
    test::<i8>(-128, 1, RoundingMode::Floor, -128);
    test::<i8>(-128, 1, RoundingMode::Ceiling, -128);
    test::<i8>(-128, 1, RoundingMode::Nearest, -128);
    test::<i8>(-128, 1, RoundingMode::Exact, -128);
}

fn div_round_fail_helper<T: PrimitiveInt>() {
    assert_panic!(T::exact_from(10).div_round(T::ZERO, RoundingMode::Floor));
    assert_panic!(T::exact_from(10).div_round(T::exact_from(3), RoundingMode::Exact));
    assert_panic!(T::exact_from(10).div_round_assign(T::ZERO, RoundingMode::Floor));
    assert_panic!(T::exact_from(10).div_round_assign(T::exact_from(3), RoundingMode::Exact));
}

fn div_round_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::MIN.div_round(T::NEGATIVE_ONE, RoundingMode::Floor));
    assert_panic!(T::MIN.div_round_assign(T::NEGATIVE_ONE, RoundingMode::Floor));
}

#[test]
fn div_round_fail() {
    apply_fn_to_primitive_ints!(div_round_fail_helper);
    apply_fn_to_signeds!(div_round_signed_fail_helper);
}
