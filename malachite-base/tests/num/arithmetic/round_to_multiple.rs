use malachite_base::num::arithmetic::traits::{RoundToMultiple, RoundToMultipleAssign};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{NegativeOne, One};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_mode::RoundingMode;

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

macro_rules! round_to_multiple_fail {
    (
        $t:ident,
        $round_to_multiple_fail_1:ident,
        $round_to_multiple_fail_2:ident,
        $round_to_multiple_fail_3:ident,
        $round_to_multiple_fail_4:ident,
        $round_to_multiple_fail_5:ident,
        $round_to_multiple_fail_6:ident
    ) => {
        #[test]
        #[should_panic]
        fn $round_to_multiple_fail_1() {
            $t::exact_from(10).round_to_multiple(0, RoundingMode::Up);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_fail_2() {
            $t::exact_from(10).round_to_multiple(3, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_fail_3() {
            $t::exact_from($t::MAX).round_to_multiple(2, RoundingMode::Ceiling);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_fail_4() {
            $t::ONE.round_to_multiple(0, RoundingMode::Up);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_fail_5() {
            $t::ONE.round_to_multiple(0, RoundingMode::Ceiling);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_fail_6() {
            $t::ONE.round_to_multiple(0, RoundingMode::Exact);
        }
    };
}
round_to_multiple_fail!(
    u8,
    round_to_multiple_u8_fail_1,
    round_to_multiple_u8_fail_2,
    round_to_multiple_u8_fail_3,
    round_to_multiple_u8_fail_4,
    round_to_multiple_u8_fail_5,
    round_to_multiple_u8_fail_6
);
round_to_multiple_fail!(
    u16,
    round_to_multiple_u16_fail_1,
    round_to_multiple_u16_fail_2,
    round_to_multiple_u16_fail_3,
    round_to_multiple_u16_fail_4,
    round_to_multiple_u16_fail_5,
    round_to_multiple_u16_fail_6
);
round_to_multiple_fail!(
    u32,
    round_to_multiple_u32_fail_1,
    round_to_multiple_u32_fail_2,
    round_to_multiple_u32_fail_3,
    round_to_multiple_u32_fail_4,
    round_to_multiple_u32_fail_5,
    round_to_multiple_u32_fail_6
);
round_to_multiple_fail!(
    u64,
    round_to_multiple_u64_fail_1,
    round_to_multiple_u64_fail_2,
    round_to_multiple_u64_fail_3,
    round_to_multiple_u64_fail_4,
    round_to_multiple_u64_fail_5,
    round_to_multiple_u64_fail_6
);
round_to_multiple_fail!(
    u128,
    round_to_multiple_u128_fail_1,
    round_to_multiple_u128_fail_2,
    round_to_multiple_u128_fail_3,
    round_to_multiple_u128_fail_4,
    round_to_multiple_u128_fail_5,
    round_to_multiple_u128_fail_6
);
round_to_multiple_fail!(
    usize,
    round_to_multiple_usize_fail_1,
    round_to_multiple_usize_fail_2,
    round_to_multiple_usize_fail_3,
    round_to_multiple_usize_fail_4,
    round_to_multiple_usize_fail_5,
    round_to_multiple_usize_fail_6
);
round_to_multiple_fail!(
    i8,
    round_to_multiple_i8_fail_1,
    round_to_multiple_i8_fail_2,
    round_to_multiple_i8_fail_3,
    round_to_multiple_i8_fail_4,
    round_to_multiple_i8_fail_5,
    round_to_multiple_i8_fail_6
);
round_to_multiple_fail!(
    i16,
    round_to_multiple_i16_fail_1,
    round_to_multiple_i16_fail_2,
    round_to_multiple_i16_fail_3,
    round_to_multiple_i16_fail_4,
    round_to_multiple_i16_fail_5,
    round_to_multiple_i16_fail_6
);
round_to_multiple_fail!(
    i32,
    round_to_multiple_i32_fail_1,
    round_to_multiple_i32_fail_2,
    round_to_multiple_i32_fail_3,
    round_to_multiple_i32_fail_4,
    round_to_multiple_i32_fail_5,
    round_to_multiple_i32_fail_6
);
round_to_multiple_fail!(
    i64,
    round_to_multiple_i64_fail_1,
    round_to_multiple_i64_fail_2,
    round_to_multiple_i64_fail_3,
    round_to_multiple_i64_fail_4,
    round_to_multiple_i64_fail_5,
    round_to_multiple_i64_fail_6
);
round_to_multiple_fail!(
    i128,
    round_to_multiple_i128_fail_1,
    round_to_multiple_i128_fail_2,
    round_to_multiple_i128_fail_3,
    round_to_multiple_i128_fail_4,
    round_to_multiple_i128_fail_5,
    round_to_multiple_i128_fail_6
);
round_to_multiple_fail!(
    isize,
    round_to_multiple_isize_fail_1,
    round_to_multiple_isize_fail_2,
    round_to_multiple_isize_fail_3,
    round_to_multiple_isize_fail_4,
    round_to_multiple_isize_fail_5,
    round_to_multiple_isize_fail_6
);

macro_rules! round_to_multiple_signed_fail {
    (
        $t:ident,
        $round_to_multiple_signed_fail_7:ident,
        $round_to_multiple_signed_fail_8:ident,
        $round_to_multiple_signed_fail_9:ident,
        $round_to_multiple_signed_fail_10:ident
    ) => {
        #[test]
        #[should_panic]
        fn $round_to_multiple_signed_fail_7() {
            $t::MIN.round_to_multiple(3, RoundingMode::Floor);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_signed_fail_8() {
            $t::NEGATIVE_ONE.round_to_multiple(0, RoundingMode::Up);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_signed_fail_9() {
            $t::NEGATIVE_ONE.round_to_multiple(0, RoundingMode::Floor);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_signed_fail_10() {
            $t::NEGATIVE_ONE.round_to_multiple(0, RoundingMode::Exact);
        }
    };
}
round_to_multiple_signed_fail!(
    i8,
    round_to_multiple_i8_fail_7,
    round_to_multiple_i8_fail_8,
    round_to_multiple_i8_fail_9,
    round_to_multiple_i8_fail_10
);
round_to_multiple_signed_fail!(
    i16,
    round_to_multiple_i16_fail_7,
    round_to_multiple_i16_fail_8,
    round_to_multiple_i16_fail_9,
    round_to_multiple_i16_fail_10
);
round_to_multiple_signed_fail!(
    i32,
    round_to_multiple_i32_fail_7,
    round_to_multiple_i32_fail_8,
    round_to_multiple_i32_fail_9,
    round_to_multiple_i32_fail_10
);
round_to_multiple_signed_fail!(
    i64,
    round_to_multiple_i64_fail_7,
    round_to_multiple_i64_fail_8,
    round_to_multiple_i64_fail_9,
    round_to_multiple_i64_fail_10
);
round_to_multiple_signed_fail!(
    i128,
    round_to_multiple_i128_fail_7,
    round_to_multiple_i128_fail_8,
    round_to_multiple_i128_fail_9,
    round_to_multiple_i128_fail_10
);
round_to_multiple_signed_fail!(
    isize,
    round_to_multiple_isize_fail_7,
    round_to_multiple_isize_fail_8,
    round_to_multiple_isize_fail_9,
    round_to_multiple_isize_fail_10
);

macro_rules! round_to_multiple_assign_fail {
    (
        $t:ident,
        $round_to_multiple_assign_fail_1:ident,
        $round_to_multiple_assign_fail_2:ident,
        $round_to_multiple_assign_fail_3:ident,
        $round_to_multiple_assign_fail_4:ident,
        $round_to_multiple_assign_fail_5:ident,
        $round_to_multiple_assign_fail_6:ident
    ) => {
        #[test]
        #[should_panic]
        fn $round_to_multiple_assign_fail_1() {
            $t::exact_from(10).round_to_multiple_assign(0, RoundingMode::Up);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_assign_fail_2() {
            $t::exact_from(10).round_to_multiple_assign(3, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_assign_fail_3() {
            $t::exact_from($t::MAX).round_to_multiple_assign(2, RoundingMode::Up);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_assign_fail_4() {
            $t::ONE.round_to_multiple_assign(0, RoundingMode::Up);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_assign_fail_5() {
            $t::ONE.round_to_multiple_assign(0, RoundingMode::Ceiling);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_assign_fail_6() {
            $t::ONE.round_to_multiple_assign(0, RoundingMode::Exact);
        }
    };
}
round_to_multiple_assign_fail!(
    u8,
    round_to_multiple_assign_u8_fail_1,
    round_to_multiple_assign_u8_fail_2,
    round_to_multiple_assign_u8_fail_3,
    round_to_multiple_assign_u8_fail_4,
    round_to_multiple_assign_u8_fail_5,
    round_to_multiple_assign_u8_fail_6
);
round_to_multiple_assign_fail!(
    u16,
    round_to_multiple_assign_u16_fail_1,
    round_to_multiple_assign_u16_fail_2,
    round_to_multiple_assign_u16_fail_3,
    round_to_multiple_assign_u16_fail_4,
    round_to_multiple_assign_u16_fail_5,
    round_to_multiple_assign_u16_fail_6
);
round_to_multiple_assign_fail!(
    u32,
    round_to_multiple_assign_u32_fail_1,
    round_to_multiple_assign_u32_fail_2,
    round_to_multiple_assign_u32_fail_3,
    round_to_multiple_assign_u32_fail_4,
    round_to_multiple_assign_u32_fail_5,
    round_to_multiple_assign_u32_fail_6
);
round_to_multiple_assign_fail!(
    u64,
    round_to_multiple_assign_u64_fail_1,
    round_to_multiple_assign_u64_fail_2,
    round_to_multiple_assign_u64_fail_3,
    round_to_multiple_assign_u64_fail_4,
    round_to_multiple_assign_u64_fail_5,
    round_to_multiple_assign_u64_fail_6
);
round_to_multiple_assign_fail!(
    u128,
    round_to_multiple_assign_u128_fail_1,
    round_to_multiple_assign_u128_fail_2,
    round_to_multiple_assign_u128_fail_3,
    round_to_multiple_assign_u128_fail_4,
    round_to_multiple_assign_u128_fail_5,
    round_to_multiple_assign_u128_fail_6
);
round_to_multiple_assign_fail!(
    usize,
    round_to_multiple_assign_usize_fail_1,
    round_to_multiple_assign_usize_fail_2,
    round_to_multiple_assign_usize_fail_3,
    round_to_multiple_assign_usize_fail_4,
    round_to_multiple_assign_usize_fail_5,
    round_to_multiple_assign_usize_fail_6
);
round_to_multiple_assign_fail!(
    i8,
    round_to_multiple_assign_i8_fail_1,
    round_to_multiple_assign_i8_fail_2,
    round_to_multiple_assign_i8_fail_3,
    round_to_multiple_assign_i8_fail_4,
    round_to_multiple_assign_i8_fail_5,
    round_to_multiple_assign_i8_fail_6
);
round_to_multiple_assign_fail!(
    i16,
    round_to_multiple_assign_i16_fail_1,
    round_to_multiple_assign_i16_fail_2,
    round_to_multiple_assign_i16_fail_3,
    round_to_multiple_assign_i16_fail_4,
    round_to_multiple_assign_i16_fail_5,
    round_to_multiple_assign_i16_fail_6
);
round_to_multiple_assign_fail!(
    i32,
    round_to_multiple_assign_i32_fail_1,
    round_to_multiple_assign_i32_fail_2,
    round_to_multiple_assign_i32_fail_3,
    round_to_multiple_assign_i32_fail_4,
    round_to_multiple_assign_i32_fail_5,
    round_to_multiple_assign_i32_fail_6
);
round_to_multiple_assign_fail!(
    i64,
    round_to_multiple_assign_i64_fail_1,
    round_to_multiple_assign_i64_fail_2,
    round_to_multiple_assign_i64_fail_3,
    round_to_multiple_assign_i64_fail_4,
    round_to_multiple_assign_i64_fail_5,
    round_to_multiple_assign_i64_fail_6
);
round_to_multiple_assign_fail!(
    i128,
    round_to_multiple_assign_i128_fail_1,
    round_to_multiple_assign_i128_fail_2,
    round_to_multiple_assign_i128_fail_3,
    round_to_multiple_assign_i128_fail_4,
    round_to_multiple_assign_i128_fail_5,
    round_to_multiple_assign_i128_fail_6
);
round_to_multiple_assign_fail!(
    isize,
    round_to_multiple_assign_isize_fail_1,
    round_to_multiple_assign_isize_fail_2,
    round_to_multiple_assign_isize_fail_3,
    round_to_multiple_assign_isize_fail_4,
    round_to_multiple_assign_isize_fail_5,
    round_to_multiple_assign_isize_fail_6
);

macro_rules! round_to_multiple_assign_signed_fail {
    (
        $t:ident,
        $round_to_multiple_assign_signed_fail_7:ident,
        $round_to_multiple_assign_signed_fail_8:ident,
        $round_to_multiple_assign_signed_fail_9:ident,
        $round_to_multiple_assign_signed_fail_10:ident
    ) => {
        #[test]
        #[should_panic]
        fn $round_to_multiple_assign_signed_fail_7() {
            $t::MIN.round_to_multiple_assign(3, RoundingMode::Floor);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_assign_signed_fail_8() {
            $t::NEGATIVE_ONE.round_to_multiple_assign(0, RoundingMode::Up);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_assign_signed_fail_9() {
            $t::NEGATIVE_ONE.round_to_multiple_assign(0, RoundingMode::Floor);
        }

        #[test]
        #[should_panic]
        fn $round_to_multiple_assign_signed_fail_10() {
            $t::NEGATIVE_ONE.round_to_multiple_assign(0, RoundingMode::Exact);
        }
    };
}
round_to_multiple_assign_signed_fail!(
    i8,
    round_to_multiple_assign_i8_fail_7,
    round_to_multiple_assign_i8_fail_8,
    round_to_multiple_assign_i8_fail_9,
    round_to_multiple_assign_i8_fail_10
);
round_to_multiple_assign_signed_fail!(
    i16,
    round_to_multiple_assign_i16_fail_7,
    round_to_multiple_assign_i16_fail_8,
    round_to_multiple_assign_i16_fail_9,
    round_to_multiple_assign_i16_fail_10
);
round_to_multiple_assign_signed_fail!(
    i32,
    round_to_multiple_assign_i32_fail_7,
    round_to_multiple_assign_i32_fail_8,
    round_to_multiple_assign_i32_fail_9,
    round_to_multiple_assign_i32_fail_10
);
round_to_multiple_assign_signed_fail!(
    i64,
    round_to_multiple_assign_i64_fail_7,
    round_to_multiple_assign_i64_fail_8,
    round_to_multiple_assign_i64_fail_9,
    round_to_multiple_assign_i64_fail_10
);
round_to_multiple_assign_signed_fail!(
    i128,
    round_to_multiple_assign_i128_fail_7,
    round_to_multiple_assign_i128_fail_8,
    round_to_multiple_assign_i128_fail_9,
    round_to_multiple_assign_i128_fail_10
);
round_to_multiple_assign_signed_fail!(
    isize,
    round_to_multiple_assign_isize_fail_7,
    round_to_multiple_assign_isize_fail_8,
    round_to_multiple_assign_isize_fail_9,
    round_to_multiple_assign_isize_fail_10
);
