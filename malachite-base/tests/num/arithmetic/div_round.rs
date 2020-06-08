use malachite_base::num::arithmetic::traits::{DivRound, DivRoundAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_mode::RoundingMode;

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

macro_rules! div_round_fail {
    ($t:ident, $div_round_fail_1:ident, $div_round_fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $div_round_fail_1() {
            $t::exact_from(10).div_round(0, RoundingMode::Floor);
        }

        #[test]
        #[should_panic]
        fn $div_round_fail_2() {
            $t::exact_from(10).div_round(3, RoundingMode::Exact);
        }
    };
}
div_round_fail!(u8, div_round_u8_fail_1, div_round_u8_fail_2);
div_round_fail!(u16, div_round_u16_fail_1, div_round_u16_fail_2);
div_round_fail!(u32, div_round_u32_fail_1, div_round_u32_fail_2);
div_round_fail!(u64, div_round_u64_fail_1, div_round_u64_fail_2);
div_round_fail!(u128, div_round_u128_fail_1, div_round_u128_fail_2);
div_round_fail!(usize, div_round_usize_fail_1, div_round_usize_fail_2);
div_round_fail!(i8, div_round_i8_fail_1, div_round_i8_fail_2);
div_round_fail!(i16, div_round_i16_fail_1, div_round_i16_fail_2);
div_round_fail!(i32, div_round_i32_fail_1, div_round_i32_fail_2);
div_round_fail!(i64, div_round_i64_fail_1, div_round_i64_fail_2);
div_round_fail!(i128, div_round_i128_fail_1, div_round_i128_fail_2);
div_round_fail!(isize, div_round_isize_fail_1, div_round_isize_fail_2);

macro_rules! div_round_signed_fail {
    ($t:ident, $div_round_signed_fail:ident) => {
        #[test]
        #[should_panic]
        fn $div_round_signed_fail() {
            $t::MIN.div_round(-1, RoundingMode::Floor);
        }
    };
}
div_round_signed_fail!(i8, div_round_i8_fail_3);
div_round_signed_fail!(i16, div_round_i16_fail_3);
div_round_signed_fail!(i32, div_round_i32_fail_3);
div_round_signed_fail!(i64, div_round_i64_fail_3);
div_round_signed_fail!(i128, div_round_signed_i128_fail_3);
div_round_signed_fail!(isize, div_round_isize_fail_3);

macro_rules! div_round_assign_fail {
    ($t:ident, $div_round_assign_fail_1:ident, $div_round_assign_fail_2:ident) => {
        #[test]
        #[should_panic]
        fn $div_round_assign_fail_1() {
            $t::exact_from(10).div_round_assign(0, RoundingMode::Floor);
        }

        #[test]
        #[should_panic]
        fn $div_round_assign_fail_2() {
            $t::exact_from(10).div_round_assign(3, RoundingMode::Exact);
        }
    };
}
div_round_assign_fail!(u8, div_round_assign_u8_fail_1, div_round_assign_u8_fail_2);
div_round_assign_fail!(
    u16,
    div_round_assign_u16_fail_1,
    div_round_assign_u16_fail_2
);
div_round_assign_fail!(
    u32,
    div_round_assign_u32_fail_1,
    div_round_assign_u32_fail_2
);
div_round_assign_fail!(
    u64,
    div_round_assign_u64_fail_1,
    div_round_assign_u64_fail_2
);
div_round_assign_fail!(
    u128,
    div_round_assign_u128_fail_1,
    div_round_assign_u128_fail_2
);
div_round_assign_fail!(
    usize,
    div_round_assign_usize_fail_1,
    div_round_assign_usize_fail_2
);
div_round_assign_fail!(i8, div_round_assign_i8_fail_1, div_round_assign_i8_fail_2);
div_round_assign_fail!(
    i16,
    div_round_assign_i16_fail_1,
    div_round_assign_i16_fail_2
);
div_round_assign_fail!(
    i32,
    div_round_assign_i32_fail_1,
    div_round_assign_i32_fail_2
);
div_round_assign_fail!(
    i64,
    div_round_assign_i64_fail_1,
    div_round_assign_i64_fail_2
);
div_round_assign_fail!(
    i128,
    div_round_assign_i128_fail_1,
    div_round_assign_i128_fail_2
);
div_round_assign_fail!(
    isize,
    div_round_assign_isize_fail_1,
    div_round_assign_isize_fail_2
);

macro_rules! div_round_assign_signed_fail {
    ($t:ident, $div_round_assign_signed_fail:ident) => {
        #[test]
        #[should_panic]
        fn $div_round_assign_signed_fail() {
            $t::MIN.div_round_assign(-1, RoundingMode::Floor);
        }
    };
}
div_round_assign_signed_fail!(i8, div_round_assign_i8_fail_3);
div_round_assign_signed_fail!(i16, div_round_assign_i16_fail_3);
div_round_assign_signed_fail!(i32, div_round_assign_i32_fail_3);
div_round_assign_signed_fail!(i64, div_round_assign_i64_fail_3);
div_round_assign_signed_fail!(i128, div_round_signed_assign_i128_fail_3);
div_round_assign_signed_fail!(isize, div_round_assign_isize_fail_3);
