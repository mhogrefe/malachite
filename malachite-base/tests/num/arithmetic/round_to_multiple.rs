use std::panic::catch_unwind;

use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode;

#[test]
fn test_round_to_multiple_unsigned() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, rm: RoundingMode, q: T) {
        assert_eq!(n.round_to_multiple(d, rm), q);

        let mut mut_n = n;
        mut_n.round_to_multiple_assign(d, rm);
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

    test::<u64>(1_000_000_000_000, 3, RoundingMode::Down, 999_999_999_999);
    test::<u64>(1_000_000_000_000, 3, RoundingMode::Floor, 999_999_999_999);
    test::<u64>(1_000_000_000_000, 3, RoundingMode::Up, 1_000_000_000_002);
    test::<u64>(
        1_000_000_000_000,
        3,
        RoundingMode::Ceiling,
        1_000_000_000_002,
    );
    test::<u64>(1_000_000_000_000, 3, RoundingMode::Nearest, 999_999_999_999);

    test::<u64>(999_999_999_999, 2, RoundingMode::Down, 999_999_999_998);
    test::<u64>(999_999_999_999, 2, RoundingMode::Floor, 999_999_999_998);
    test::<u64>(999_999_999_999, 2, RoundingMode::Up, 1_000_000_000_000);
    test::<u64>(999_999_999_999, 2, RoundingMode::Ceiling, 1_000_000_000_000);
    test::<u64>(999_999_999_999, 2, RoundingMode::Nearest, 1_000_000_000_000);

    test::<u64>(1_000_000_000_001, 2, RoundingMode::Down, 1_000_000_000_000);
    test::<u64>(1_000_000_000_001, 2, RoundingMode::Floor, 1_000_000_000_000);
    test::<u64>(1_000_000_000_001, 2, RoundingMode::Up, 1_000_000_000_002);
    test::<u64>(
        1_000_000_000_001,
        2,
        RoundingMode::Ceiling,
        1_000_000_000_002,
    );
    test::<u64>(
        1_000_000_000_001,
        2,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );

    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Down,
        999_999_999_999_996_832_276_305,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Floor,
        999_999_999_999_996_832_276_305,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Up,
        1_000_000_000_000_001_127_243_600,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Ceiling,
        1_000_000_000_000_001_127_243_600,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Nearest,
        1_000_000_000_000_001_127_243_600,
    );

    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Down,
        1_000_000_000_000_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Floor,
        1_000_000_000_000_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Up,
        1_000_000_000_000_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Ceiling,
        1_000_000_000_000_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Nearest,
        1_000_000_000_000_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Exact,
        1_000_000_000_000_000_000_000_000,
    );

    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Down,
        999_999_999_999_999_999_999_999,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Floor,
        999_999_999_999_999_999_999_999,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Up,
        1_000_000_000_001_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Ceiling,
        1_000_000_000_001_000_000_000_000,
    );
    test::<u128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Nearest,
        999_999_999_999_999_999_999_999,
    );

    test::<u128>(
        2_999_999_999_999_999_999_999_999,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        2_000_000_000_000_000_000_000_000,
    );
    test::<u128>(
        3_000_000_000_000_000_000_000_000,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        4_000_000_000_000_000_000_000_000,
    );
    test::<u128>(
        3_000_000_000_000_000_000_000_001,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        4_000_000_000_000_000_000_000_000,
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

    test::<i64>(1_000_000_000_000, 3, RoundingMode::Down, 999_999_999_999);
    test::<i64>(1_000_000_000_000, 3, RoundingMode::Floor, 999_999_999_999);
    test::<i64>(1_000_000_000_000, 3, RoundingMode::Up, 1_000_000_000_002);
    test::<i64>(
        1_000_000_000_000,
        3,
        RoundingMode::Ceiling,
        1_000_000_000_002,
    );
    test::<i64>(1_000_000_000_000, 3, RoundingMode::Nearest, 999_999_999_999);

    test::<i64>(999_999_999_999, 2, RoundingMode::Down, 999_999_999_998);
    test::<i64>(999_999_999_999, 2, RoundingMode::Floor, 999_999_999_998);
    test::<i64>(999_999_999_999, 2, RoundingMode::Up, 1_000_000_000_000);
    test::<i64>(999_999_999_999, 2, RoundingMode::Ceiling, 1_000_000_000_000);
    test::<i64>(999_999_999_999, 2, RoundingMode::Nearest, 1_000_000_000_000);

    test::<i64>(1_000_000_000_001, 2, RoundingMode::Down, 1_000_000_000_000);
    test::<i64>(1_000_000_000_001, 2, RoundingMode::Floor, 1_000_000_000_000);
    test::<i64>(1_000_000_000_001, 2, RoundingMode::Up, 1_000_000_000_002);
    test::<i64>(
        1_000_000_000_001,
        2,
        RoundingMode::Ceiling,
        1_000_000_000_002,
    );
    test::<i64>(
        1_000_000_000_001,
        2,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );

    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Down,
        999_999_999_999_996_832_276_305,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Floor,
        999_999_999_999_996_832_276_305,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Up,
        1_000_000_000_000_001_127_243_600,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Ceiling,
        1_000_000_000_000_001_127_243_600,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Nearest,
        1_000_000_000_000_001_127_243_600,
    );

    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Down,
        1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Floor,
        1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Up,
        1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Ceiling,
        1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Nearest,
        1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Exact,
        1_000_000_000_000_000_000_000_000,
    );

    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Down,
        999_999_999_999_999_999_999_999,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Floor,
        999_999_999_999_999_999_999_999,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Up,
        1_000_000_000_001_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Ceiling,
        1_000_000_000_001_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Nearest,
        999_999_999_999_999_999_999_999,
    );

    test::<i128>(
        2_999_999_999_999_999_999_999_999,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        2_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        3_000_000_000_000_000_000_000_000,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        4_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        3_000_000_000_000_000_000_000_001,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        4_000_000_000_000_000_000_000_000,
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

    test::<i64>(1_000_000_000_000, -1, RoundingMode::Down, 1_000_000_000_000);
    test::<i64>(
        1_000_000_000_000,
        -1,
        RoundingMode::Floor,
        1_000_000_000_000,
    );
    test::<i64>(1_000_000_000_000, -1, RoundingMode::Up, 1_000_000_000_000);
    test::<i64>(
        1_000_000_000_000,
        -1,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<i64>(
        1_000_000_000_000,
        -1,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );
    test::<i64>(
        1_000_000_000_000,
        -1,
        RoundingMode::Exact,
        1_000_000_000_000,
    );

    test::<i64>(1_000_000_000_000, -3, RoundingMode::Down, 999_999_999_999);
    test::<i64>(1_000_000_000_000, -3, RoundingMode::Floor, 999_999_999_999);
    test::<i64>(1_000_000_000_000, -3, RoundingMode::Up, 1_000_000_000_002);
    test::<i64>(
        1_000_000_000_000,
        -3,
        RoundingMode::Ceiling,
        1_000_000_000_002,
    );
    test::<i64>(
        1_000_000_000_000,
        -3,
        RoundingMode::Nearest,
        999_999_999_999,
    );

    test::<i64>(999_999_999_999, -2, RoundingMode::Down, 999_999_999_998);
    test::<i64>(999_999_999_999, -2, RoundingMode::Floor, 999_999_999_998);
    test::<i64>(999_999_999_999, -2, RoundingMode::Up, 1_000_000_000_000);
    test::<i64>(
        999_999_999_999,
        -2,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<i64>(
        999_999_999_999,
        -2,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );

    test::<i64>(1_000_000_000_001, -2, RoundingMode::Down, 1_000_000_000_000);
    test::<i64>(
        1_000_000_000_001,
        -2,
        RoundingMode::Floor,
        1_000_000_000_000,
    );
    test::<i64>(1_000_000_000_001, -2, RoundingMode::Up, 1_000_000_000_002);
    test::<i64>(
        1_000_000_000_001,
        -2,
        RoundingMode::Ceiling,
        1_000_000_000_002,
    );
    test::<i64>(
        1_000_000_000_001,
        -2,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );

    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Down,
        999_999_999_999_996_832_276_305,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Floor,
        999_999_999_999_996_832_276_305,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Up,
        1_000_000_000_000_001_127_243_600,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Ceiling,
        1_000_000_000_000_001_127_243_600,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Nearest,
        1_000_000_000_000_001_127_243_600,
    );

    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Down,
        1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Floor,
        1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Up,
        1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Ceiling,
        1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Nearest,
        1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Exact,
        1_000_000_000_000_000_000_000_000,
    );

    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Down,
        999_999_999_999_999_999_999_999,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Floor,
        999_999_999_999_999_999_999_999,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Up,
        1_000_000_000_001_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Ceiling,
        1_000_000_000_001_000_000_000_000,
    );
    test::<i128>(
        1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Nearest,
        999_999_999_999_999_999_999_999,
    );

    test::<i128>(
        2_999_999_999_999_999_999_999_999,
        -2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        2_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        3_000_000_000_000_000_000_000_000,
        -2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        4_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        3_000_000_000_000_000_000_000_001,
        -2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        4_000_000_000_000_000_000_000_000,
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

    test::<i64>(-1_000_000_000_000, 3, RoundingMode::Down, -999_999_999_999);
    test::<i64>(
        -1_000_000_000_000,
        3,
        RoundingMode::Floor,
        -1_000_000_000_002,
    );
    test::<i64>(-1_000_000_000_000, 3, RoundingMode::Up, -1_000_000_000_002);
    test::<i64>(
        -1_000_000_000_000,
        3,
        RoundingMode::Ceiling,
        -999_999_999_999,
    );
    test::<i64>(
        -1_000_000_000_000,
        3,
        RoundingMode::Nearest,
        -999_999_999_999,
    );

    test::<i64>(-999_999_999_999, 2, RoundingMode::Down, -999_999_999_998);
    test::<i64>(-999_999_999_999, 2, RoundingMode::Floor, -1_000_000_000_000);
    test::<i64>(-999_999_999_999, 2, RoundingMode::Up, -1_000_000_000_000);
    test::<i64>(-999_999_999_999, 2, RoundingMode::Ceiling, -999_999_999_998);
    test::<i64>(
        -999_999_999_999,
        2,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );

    test::<i64>(
        -1_000_000_000_001,
        2,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_001,
        2,
        RoundingMode::Floor,
        -1_000_000_000_002,
    );
    test::<i64>(-1_000_000_000_001, 2, RoundingMode::Up, -1_000_000_000_002);
    test::<i64>(
        -1_000_000_000_001,
        2,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_001,
        2,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );

    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Down,
        -999_999_999_999_996_832_276_305,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Floor,
        -1_000_000_000_000_001_127_243_600,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Up,
        -1_000_000_000_000_001_127_243_600,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Ceiling,
        -999_999_999_999_996_832_276_305,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        0xffff_ffff,
        RoundingMode::Nearest,
        -1_000_000_000_000_001_127_243_600,
    );

    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Down,
        -1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Floor,
        -1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Up,
        -1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Ceiling,
        -1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Nearest,
        -1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_000,
        RoundingMode::Exact,
        -1_000_000_000_000_000_000_000_000,
    );

    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Down,
        -999_999_999_999_999_999_999_999,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Floor,
        -1_000_000_000_001_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Up,
        -1_000_000_000_001_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Ceiling,
        -999_999_999_999_999_999_999_999,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        1_000_000_000_001,
        RoundingMode::Nearest,
        -999_999_999_999_999_999_999_999,
    );

    test::<i128>(
        -2_999_999_999_999_999_999_999_999,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        -2_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -3_000_000_000_000_000_000_000_000,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        -4_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -3_000_000_000_000_000_000_000_001,
        2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        -4_000_000_000_000_000_000_000_000,
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

    test::<i64>(
        -1_000_000_000_000,
        -1,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_000,
        -1,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i64>(-1_000_000_000_000, -1, RoundingMode::Up, -1_000_000_000_000);
    test::<i64>(
        -1_000_000_000_000,
        -1,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_000,
        -1,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_000,
        -1,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i64>(-1_000_000_000_000, -3, RoundingMode::Down, -999_999_999_999);
    test::<i64>(
        -1_000_000_000_000,
        -3,
        RoundingMode::Floor,
        -1_000_000_000_002,
    );
    test::<i64>(-1_000_000_000_000, -3, RoundingMode::Up, -1_000_000_000_002);
    test::<i64>(
        -1_000_000_000_000,
        -3,
        RoundingMode::Ceiling,
        -999_999_999_999,
    );
    test::<i64>(
        -1_000_000_000_000,
        -3,
        RoundingMode::Nearest,
        -999_999_999_999,
    );

    test::<i64>(-999_999_999_999, -2, RoundingMode::Down, -999_999_999_998);
    test::<i64>(
        -999_999_999_999,
        -2,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i64>(-999_999_999_999, -2, RoundingMode::Up, -1_000_000_000_000);
    test::<i64>(
        -999_999_999_999,
        -2,
        RoundingMode::Ceiling,
        -999_999_999_998,
    );
    test::<i64>(
        -999_999_999_999,
        -2,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );

    test::<i64>(
        -1_000_000_000_001,
        -2,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_001,
        -2,
        RoundingMode::Floor,
        -1_000_000_000_002,
    );
    test::<i64>(-1_000_000_000_001, -2, RoundingMode::Up, -1_000_000_000_002);
    test::<i64>(
        -1_000_000_000_001,
        -2,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i64>(
        -1_000_000_000_001,
        -2,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );

    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Down,
        -999_999_999_999_996_832_276_305,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Floor,
        -1_000_000_000_000_001_127_243_600,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Up,
        -1_000_000_000_000_001_127_243_600,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Ceiling,
        -999_999_999_999_996_832_276_305,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -0xffff_ffff,
        RoundingMode::Nearest,
        -1_000_000_000_000_001_127_243_600,
    );

    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Down,
        -1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Floor,
        -1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Up,
        -1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Ceiling,
        -1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Nearest,
        -1_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_000,
        RoundingMode::Exact,
        -1_000_000_000_000_000_000_000_000,
    );

    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Down,
        -999_999_999_999_999_999_999_999,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Floor,
        -1_000_000_000_001_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Up,
        -1_000_000_000_001_000_000_000_000,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Ceiling,
        -999_999_999_999_999_999_999_999,
    );
    test::<i128>(
        -1_000_000_000_000_000_000_000_000,
        -1_000_000_000_001,
        RoundingMode::Nearest,
        -999_999_999_999_999_999_999_999,
    );

    test::<i128>(
        -2_999_999_999_999_999_999_999_999,
        -2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        -2_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -3_000_000_000_000_000_000_000_000,
        -2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        -4_000_000_000_000_000_000_000_000,
    );
    test::<i128>(
        -3_000_000_000_000_000_000_000_001,
        -2_000_000_000_000_000_000_000_000,
        RoundingMode::Nearest,
        -4_000_000_000_000_000_000_000_000,
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

fn round_to_multiple_fail_helper<T: PrimitiveInteger>() {
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
    assert_panic!(T::MAX.round_to_multiple_assign(T::TWO, RoundingMode::Ceiling));
    assert_panic!(T::ONE.round_to_multiple_assign(T::ZERO, RoundingMode::Up));
    assert_panic!(T::ONE.round_to_multiple_assign(T::ZERO, RoundingMode::Ceiling));
    assert_panic!(T::ONE.round_to_multiple_assign(T::ZERO, RoundingMode::Exact));
}

fn round_to_multiple_signed_fail_helper<T: PrimitiveSigned>() {
    assert_panic!(T::MIN.round_to_multiple(T::exact_from(3), RoundingMode::Floor));
    assert_panic!(T::NEGATIVE_ONE.round_to_multiple(T::ZERO, RoundingMode::Up));
    assert_panic!(T::NEGATIVE_ONE.round_to_multiple(T::ZERO, RoundingMode::Floor));
    assert_panic!(T::NEGATIVE_ONE.round_to_multiple(T::ZERO, RoundingMode::Exact));

    assert_panic!(T::MIN.round_to_multiple_assign(T::exact_from(3), RoundingMode::Floor));
    assert_panic!(T::NEGATIVE_ONE.round_to_multiple_assign(T::ZERO, RoundingMode::Up));
    assert_panic!(T::NEGATIVE_ONE.round_to_multiple_assign(T::ZERO, RoundingMode::Floor));
    assert_panic!(T::NEGATIVE_ONE.round_to_multiple_assign(T::ZERO, RoundingMode::Exact));
}

#[test]
fn round_to_multiple_fail() {
    apply_fn_to_primitive_ints!(round_to_multiple_fail_helper);
    apply_fn_to_signeds!(round_to_multiple_signed_fail_helper);
}
