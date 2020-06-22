use malachite_base::num::arithmetic::traits::{ShlRound, ShlRoundAssign};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::NegativeOne;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode;

#[test]
fn test_shl_round() {
    fn test<T: PrimitiveInteger, U: PrimitiveInteger>(t: T, u: U, rm: RoundingMode, out: T)
    where
        T: ShlRound<U, Output = T> + ShlRoundAssign<U>,
    {
        assert_eq!(t.shl_round(u, rm), out);

        let mut t = t;
        t.shl_round_assign(u, rm);
        assert_eq!(t, out);
    };
    test::<u8, i8>(0, 0, RoundingMode::Down, 0);
    test::<u8, i8>(0, 0, RoundingMode::Up, 0);
    test::<u8, i8>(0, 0, RoundingMode::Floor, 0);
    test::<u8, i8>(0, 0, RoundingMode::Ceiling, 0);
    test::<u8, i8>(0, 0, RoundingMode::Nearest, 0);
    test::<u8, i8>(0, 0, RoundingMode::Exact, 0);

    test::<u8, i16>(0, -10, RoundingMode::Down, 0);
    test::<u8, i16>(0, -10, RoundingMode::Up, 0);
    test::<u8, i16>(0, -10, RoundingMode::Floor, 0);
    test::<u8, i16>(0, -10, RoundingMode::Ceiling, 0);
    test::<u8, i16>(0, -10, RoundingMode::Nearest, 0);
    test::<u8, i16>(0, -10, RoundingMode::Exact, 0);

    test::<i8, i32>(123, 0, RoundingMode::Down, 123);
    test::<i8, i32>(123, 0, RoundingMode::Up, 123);
    test::<i8, i32>(123, 0, RoundingMode::Floor, 123);
    test::<i8, i32>(123, 0, RoundingMode::Ceiling, 123);
    test::<i8, i32>(123, 0, RoundingMode::Nearest, 123);
    test::<i8, i32>(123, 0, RoundingMode::Exact, 123);

    test::<u8, i64>(245, -1, RoundingMode::Down, 122);
    test::<u8, i64>(245, -1, RoundingMode::Up, 123);
    test::<u8, i64>(245, -1, RoundingMode::Floor, 122);
    test::<u8, i64>(245, -1, RoundingMode::Ceiling, 123);
    test::<u8, i64>(245, -1, RoundingMode::Nearest, 122);

    test::<u8, i128>(246, -1, RoundingMode::Down, 123);
    test::<u8, i128>(246, -1, RoundingMode::Up, 123);
    test::<u8, i128>(246, -1, RoundingMode::Floor, 123);
    test::<u8, i128>(246, -1, RoundingMode::Ceiling, 123);
    test::<u8, i128>(246, -1, RoundingMode::Nearest, 123);
    test::<u8, i128>(246, -1, RoundingMode::Exact, 123);

    test::<u8, isize>(247, -1, RoundingMode::Down, 123);
    test::<u8, isize>(247, -1, RoundingMode::Up, 124);
    test::<u8, isize>(247, -1, RoundingMode::Floor, 123);
    test::<u8, isize>(247, -1, RoundingMode::Ceiling, 124);
    test::<u8, isize>(247, -1, RoundingMode::Nearest, 124);

    test::<i16, i8>(491, -2, RoundingMode::Down, 122);
    test::<i16, i8>(491, -2, RoundingMode::Up, 123);
    test::<i16, i8>(491, -2, RoundingMode::Floor, 122);
    test::<i16, i8>(491, -2, RoundingMode::Ceiling, 123);
    test::<i16, i8>(491, -2, RoundingMode::Nearest, 123);

    test::<u16, i16>(492, -2, RoundingMode::Down, 123);
    test::<u16, i16>(492, -2, RoundingMode::Up, 123);
    test::<u16, i16>(492, -2, RoundingMode::Floor, 123);
    test::<u16, i16>(492, -2, RoundingMode::Ceiling, 123);
    test::<u16, i16>(492, -2, RoundingMode::Nearest, 123);
    test::<u16, i16>(492, -2, RoundingMode::Exact, 123);

    test::<i16, i32>(493, -2, RoundingMode::Down, 123);
    test::<i16, i32>(493, -2, RoundingMode::Up, 124);
    test::<i16, i32>(493, -2, RoundingMode::Floor, 123);
    test::<i16, i32>(493, -2, RoundingMode::Ceiling, 124);
    test::<i16, i32>(493, -2, RoundingMode::Nearest, 123);

    test::<u32, i8>(4_127_195_135, -25, RoundingMode::Down, 122);
    test::<u32, i8>(4_127_195_135, -25, RoundingMode::Up, 123);
    test::<u32, i8>(4_127_195_135, -25, RoundingMode::Floor, 122);
    test::<u32, i8>(4_127_195_135, -25, RoundingMode::Ceiling, 123);
    test::<u32, i8>(4_127_195_135, -25, RoundingMode::Nearest, 123);

    test::<u32, i16>(4_127_195_136, -25, RoundingMode::Down, 123);
    test::<u32, i16>(4_127_195_136, -25, RoundingMode::Up, 123);
    test::<u32, i16>(4_127_195_136, -25, RoundingMode::Floor, 123);
    test::<u32, i16>(4_127_195_136, -25, RoundingMode::Ceiling, 123);
    test::<u32, i16>(4_127_195_136, -25, RoundingMode::Nearest, 123);
    test::<u32, i16>(4_127_195_136, -25, RoundingMode::Exact, 123);

    test::<u32, i32>(4_127_195_137, -25, RoundingMode::Down, 123);
    test::<u32, i32>(4_127_195_137, -25, RoundingMode::Up, 124);
    test::<u32, i32>(4_127_195_137, -25, RoundingMode::Floor, 123);
    test::<u32, i32>(4_127_195_137, -25, RoundingMode::Ceiling, 124);
    test::<u32, i32>(4_127_195_137, -25, RoundingMode::Nearest, 123);

    test::<i64, i8>(8_254_390_271, -26, RoundingMode::Down, 122);
    test::<i64, i8>(8_254_390_271, -26, RoundingMode::Up, 123);
    test::<i64, i8>(8_254_390_271, -26, RoundingMode::Floor, 122);
    test::<i64, i8>(8_254_390_271, -26, RoundingMode::Ceiling, 123);
    test::<i64, i8>(8_254_390_271, -26, RoundingMode::Nearest, 123);

    test::<u64, i16>(8_254_390_272, -26, RoundingMode::Down, 123);
    test::<u64, i16>(8_254_390_272, -26, RoundingMode::Up, 123);
    test::<u64, i16>(8_254_390_272, -26, RoundingMode::Floor, 123);
    test::<u64, i16>(8_254_390_272, -26, RoundingMode::Ceiling, 123);
    test::<u64, i16>(8_254_390_272, -26, RoundingMode::Nearest, 123);
    test::<u64, i16>(8_254_390_272, -26, RoundingMode::Exact, 123);

    test::<i64, i32>(8_254_390_273, -26, RoundingMode::Down, 123);
    test::<i64, i32>(8_254_390_273, -26, RoundingMode::Up, 124);
    test::<i64, i32>(8_254_390_273, -26, RoundingMode::Floor, 123);
    test::<i64, i32>(8_254_390_273, -26, RoundingMode::Ceiling, 124);
    test::<i64, i32>(8_254_390_273, -26, RoundingMode::Nearest, 123);

    test::<i64, i64>(0xffff_ffff, -1, RoundingMode::Down, 0x7fff_ffff);
    test::<i64, i64>(0xffff_ffff, -1, RoundingMode::Up, 0x8000_0000);
    test::<i64, i64>(0xffff_ffff, -1, RoundingMode::Floor, 0x7fff_ffff);
    test::<i64, i64>(0xffff_ffff, -1, RoundingMode::Ceiling, 0x8000_0000);
    test::<i64, i64>(0xffff_ffff, -1, RoundingMode::Nearest, 0x8000_0000);

    test::<u64, i64>(0x1_0000_0000, -1, RoundingMode::Down, 0x8000_0000);
    test::<u64, i64>(0x1_0000_0000, -1, RoundingMode::Up, 0x8000_0000);
    test::<u64, i64>(0x1_0000_0000, -1, RoundingMode::Floor, 0x8000_0000);
    test::<u64, i64>(0x1_0000_0000, -1, RoundingMode::Ceiling, 0x8000_0000);
    test::<u64, i64>(0x1_0000_0000, -1, RoundingMode::Nearest, 0x8000_0000);
    test::<u64, i64>(0x1_0000_0000, -1, RoundingMode::Exact, 0x8000_0000);

    test::<u64, i128>(0x1_0000_0001, -1, RoundingMode::Down, 0x8000_0000);
    test::<u64, i128>(0x1_0000_0001, -1, RoundingMode::Up, 0x8000_0001);
    test::<u64, i128>(0x1_0000_0001, -1, RoundingMode::Floor, 0x8000_0000);
    test::<u64, i128>(0x1_0000_0001, -1, RoundingMode::Ceiling, 0x8000_0001);
    test::<u64, i128>(0x1_0000_0001, -1, RoundingMode::Nearest, 0x8000_0000);

    test::<i64, isize>(1_000_000_000_000, 0, RoundingMode::Down, 1_000_000_000_000);
    test::<i64, isize>(1_000_000_000_000, 0, RoundingMode::Up, 1_000_000_000_000);
    test::<i64, isize>(1_000_000_000_000, 0, RoundingMode::Floor, 1_000_000_000_000);
    test::<i64, isize>(
        1_000_000_000_000,
        0,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<i64, isize>(
        1_000_000_000_000,
        0,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );
    test::<i64, isize>(1_000_000_000_000, 0, RoundingMode::Exact, 1_000_000_000_000);

    test::<i128, i8>(7_999_999_999_999, -3, RoundingMode::Down, 999_999_999_999);
    test::<i128, i8>(7_999_999_999_999, -3, RoundingMode::Up, 1_000_000_000_000);
    test::<i128, i8>(7_999_999_999_999, -3, RoundingMode::Floor, 999_999_999_999);
    test::<i128, i8>(
        7_999_999_999_999,
        -3,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<i128, i8>(
        7_999_999_999_999,
        -3,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );

    test::<u128, i16>(8_000_000_000_000, -3, RoundingMode::Down, 1_000_000_000_000);
    test::<u128, i16>(8_000_000_000_000, -3, RoundingMode::Up, 1_000_000_000_000);
    test::<u128, i16>(
        8_000_000_000_000,
        -3,
        RoundingMode::Floor,
        1_000_000_000_000,
    );
    test::<u128, i16>(
        8_000_000_000_000,
        -3,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<u128, i16>(
        8_000_000_000_000,
        -3,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );
    test::<u128, i16>(
        8_000_000_000_000,
        -3,
        RoundingMode::Exact,
        1_000_000_000_000,
    );

    test::<u128, i32>(8_000_000_000_001, -3, RoundingMode::Down, 1_000_000_000_000);
    test::<u128, i32>(8_000_000_000_001, -3, RoundingMode::Up, 1_000_000_000_001);
    test::<u128, i32>(
        8_000_000_000_001,
        -3,
        RoundingMode::Floor,
        1_000_000_000_000,
    );
    test::<u128, i32>(
        8_000_000_000_001,
        -3,
        RoundingMode::Ceiling,
        1_000_000_000_001,
    );
    test::<u128, i32>(
        8_000_000_000_001,
        -3,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );

    test::<i128, i64>(1_000_000_000_000, -10, RoundingMode::Down, 976_562_500);
    test::<i128, i64>(1_000_000_000_000, -10, RoundingMode::Up, 976_562_500);
    test::<i128, i64>(1_000_000_000_000, -10, RoundingMode::Floor, 976_562_500);
    test::<i128, i64>(1_000_000_000_000, -10, RoundingMode::Ceiling, 976_562_500);
    test::<i128, i64>(1_000_000_000_000, -10, RoundingMode::Nearest, 976_562_500);
    test::<i128, i64>(1_000_000_000_000, -10, RoundingMode::Exact, 976_562_500);

    test::<u128, i128>(980_657_949, -72, RoundingMode::Down, 0);
    test::<u128, i128>(980_657_949, -72, RoundingMode::Up, 1);
    test::<u128, i128>(980_657_949, -72, RoundingMode::Floor, 0);
    test::<u128, i128>(980_657_949, -72, RoundingMode::Ceiling, 1);
    test::<u128, i128>(980_657_949, -72, RoundingMode::Nearest, 0);

    test::<i128, isize>(0xffff_ffff, -31, RoundingMode::Down, 1);
    test::<i128, isize>(0xffff_ffff, -31, RoundingMode::Up, 2);
    test::<i128, isize>(0xffff_ffff, -31, RoundingMode::Floor, 1);
    test::<i128, isize>(0xffff_ffff, -31, RoundingMode::Ceiling, 2);
    test::<i128, isize>(0xffff_ffff, -31, RoundingMode::Nearest, 2);

    test::<u32, i128>(0xffff_ffff, -32, RoundingMode::Down, 0);
    test::<u32, i128>(0xffff_ffff, -32, RoundingMode::Up, 1);
    test::<u32, i128>(0xffff_ffff, -32, RoundingMode::Floor, 0);
    test::<u32, i128>(0xffff_ffff, -32, RoundingMode::Ceiling, 1);
    test::<u32, i128>(0xffff_ffff, -32, RoundingMode::Nearest, 1);

    test::<u64, i8>(0x1_0000_0000, -32, RoundingMode::Down, 1);
    test::<u64, i8>(0x1_0000_0000, -32, RoundingMode::Up, 1);
    test::<u64, i8>(0x1_0000_0000, -32, RoundingMode::Floor, 1);
    test::<u64, i8>(0x1_0000_0000, -32, RoundingMode::Ceiling, 1);
    test::<u64, i8>(0x1_0000_0000, -32, RoundingMode::Nearest, 1);
    test::<u64, i8>(0x1_0000_0000, -32, RoundingMode::Exact, 1);

    test::<i64, i16>(0x1_0000_0000, -33, RoundingMode::Down, 0);
    test::<i64, i16>(0x1_0000_0000, -33, RoundingMode::Up, 1);
    test::<i64, i16>(0x1_0000_0000, -33, RoundingMode::Floor, 0);
    test::<i64, i16>(0x1_0000_0000, -33, RoundingMode::Ceiling, 1);
    test::<i64, i16>(0x1_0000_0000, -33, RoundingMode::Nearest, 0);

    test::<u8, i8>(0, 10, RoundingMode::Exact, 0);
    test::<u8, i16>(123, 1, RoundingMode::Exact, 246);
    test::<u16, i32>(123, 2, RoundingMode::Exact, 492);
    test::<u64, i64>(123, 25, RoundingMode::Exact, 4_127_195_136);
    test::<u128, i128>(123, 26, RoundingMode::Exact, 8_254_390_272);
    test::<u8, isize>(123, 100, RoundingMode::Exact, 0);

    test::<u64, i8>(0x8000_0000, 1, RoundingMode::Exact, 0x1_0000_0000);
    test::<i64, i16>(1_000_000_000_000, 3, RoundingMode::Exact, 8_000_000_000_000);
    test::<u64, i8>(
        1_000_000_000_000,
        24,
        RoundingMode::Exact,
        16_777_216_000_000_000_000,
    );
    test::<i128, i16>(
        1_000_000_000_000,
        25,
        RoundingMode::Exact,
        33_554_432_000_000_000_000,
    );
    test::<u128, i32>(
        1_000_000_000_000,
        31,
        RoundingMode::Exact,
        2_147_483_648_000_000_000_000,
    );
    test::<i128, i64>(
        1_000_000_000_000,
        32,
        RoundingMode::Exact,
        4_294_967_296_000_000_000_000,
    );
    test::<u128, i128>(
        1_000_000_000_000,
        33,
        RoundingMode::Exact,
        8_589_934_592_000_000_000_000,
    );
    test::<i64, isize>(1_000_000_000_000, 100, RoundingMode::Exact, 0);

    test::<i8, i8>(-123, 0, RoundingMode::Down, -123);
    test::<i8, i8>(-123, 0, RoundingMode::Up, -123);
    test::<i8, i8>(-123, 0, RoundingMode::Floor, -123);
    test::<i8, i8>(-123, 0, RoundingMode::Ceiling, -123);
    test::<i8, i8>(-123, 0, RoundingMode::Nearest, -123);
    test::<i8, i8>(-123, 0, RoundingMode::Exact, -123);

    test::<i16, i8>(-245, -1, RoundingMode::Down, -122);
    test::<i16, i8>(-245, -1, RoundingMode::Up, -123);
    test::<i16, i8>(-245, -1, RoundingMode::Floor, -123);
    test::<i16, i8>(-245, -1, RoundingMode::Ceiling, -122);
    test::<i16, i8>(-245, -1, RoundingMode::Nearest, -122);

    test::<i16, i16>(-246, -1, RoundingMode::Down, -123);
    test::<i16, i16>(-246, -1, RoundingMode::Up, -123);
    test::<i16, i16>(-246, -1, RoundingMode::Floor, -123);
    test::<i16, i16>(-246, -1, RoundingMode::Ceiling, -123);
    test::<i16, i16>(-246, -1, RoundingMode::Nearest, -123);
    test::<i16, i16>(-246, -1, RoundingMode::Exact, -123);

    test::<i16, i32>(-247, -1, RoundingMode::Down, -123);
    test::<i16, i32>(-247, -1, RoundingMode::Up, -124);
    test::<i16, i32>(-247, -1, RoundingMode::Floor, -124);
    test::<i16, i32>(-247, -1, RoundingMode::Ceiling, -123);
    test::<i16, i32>(-247, -1, RoundingMode::Nearest, -124);

    test::<i16, i64>(-491, -2, RoundingMode::Down, -122);
    test::<i16, i64>(-491, -2, RoundingMode::Up, -123);
    test::<i16, i64>(-491, -2, RoundingMode::Floor, -123);
    test::<i16, i64>(-491, -2, RoundingMode::Ceiling, -122);
    test::<i16, i64>(-491, -2, RoundingMode::Nearest, -123);

    test::<i16, i128>(-492, -2, RoundingMode::Down, -123);
    test::<i16, i128>(-492, -2, RoundingMode::Up, -123);
    test::<i16, i128>(-492, -2, RoundingMode::Floor, -123);
    test::<i16, i128>(-492, -2, RoundingMode::Ceiling, -123);
    test::<i16, i128>(-492, -2, RoundingMode::Nearest, -123);
    test::<i16, i128>(-492, -2, RoundingMode::Exact, -123);

    test::<i16, isize>(-493, -2, RoundingMode::Down, -123);
    test::<i16, isize>(-493, -2, RoundingMode::Up, -124);
    test::<i16, isize>(-493, -2, RoundingMode::Floor, -124);
    test::<i16, isize>(-493, -2, RoundingMode::Ceiling, -123);
    test::<i16, isize>(-493, -2, RoundingMode::Nearest, -123);

    test::<i64, i8>(-4_127_195_135, -25, RoundingMode::Down, -122);
    test::<i64, i8>(-4_127_195_135, -25, RoundingMode::Up, -123);
    test::<i64, i8>(-4_127_195_135, -25, RoundingMode::Floor, -123);
    test::<i64, i8>(-4_127_195_135, -25, RoundingMode::Ceiling, -122);
    test::<i64, i8>(-4_127_195_135, -25, RoundingMode::Nearest, -123);

    test::<i64, i16>(-4_127_195_136, -25, RoundingMode::Down, -123);
    test::<i64, i16>(-4_127_195_136, -25, RoundingMode::Up, -123);
    test::<i64, i16>(-4_127_195_136, -25, RoundingMode::Floor, -123);
    test::<i64, i16>(-4_127_195_136, -25, RoundingMode::Ceiling, -123);
    test::<i64, i16>(-4_127_195_136, -25, RoundingMode::Nearest, -123);
    test::<i64, i16>(-4_127_195_136, -25, RoundingMode::Exact, -123);

    test::<i64, i32>(-4_127_195_137, -25, RoundingMode::Down, -123);
    test::<i64, i32>(-4_127_195_137, -25, RoundingMode::Up, -124);
    test::<i64, i32>(-4_127_195_137, -25, RoundingMode::Floor, -124);
    test::<i64, i32>(-4_127_195_137, -25, RoundingMode::Ceiling, -123);
    test::<i64, i32>(-4_127_195_137, -25, RoundingMode::Nearest, -123);

    test::<i64, i64>(-8_254_390_271, -26, RoundingMode::Down, -122);
    test::<i64, i64>(-8_254_390_271, -26, RoundingMode::Up, -123);
    test::<i64, i64>(-8_254_390_271, -26, RoundingMode::Floor, -123);
    test::<i64, i64>(-8_254_390_271, -26, RoundingMode::Ceiling, -122);
    test::<i64, i64>(-8_254_390_271, -26, RoundingMode::Nearest, -123);

    test::<i64, i128>(-8_254_390_272, -26, RoundingMode::Down, -123);
    test::<i64, i128>(-8_254_390_272, -26, RoundingMode::Up, -123);
    test::<i64, i128>(-8_254_390_272, -26, RoundingMode::Floor, -123);
    test::<i64, i128>(-8_254_390_272, -26, RoundingMode::Ceiling, -123);
    test::<i64, i128>(-8_254_390_272, -26, RoundingMode::Nearest, -123);
    test::<i64, i128>(-8_254_390_272, -26, RoundingMode::Exact, -123);

    test::<i64, isize>(-8_254_390_273, -26, RoundingMode::Down, -123);
    test::<i64, isize>(-8_254_390_273, -26, RoundingMode::Up, -124);
    test::<i64, isize>(-8_254_390_273, -26, RoundingMode::Floor, -124);
    test::<i64, isize>(-8_254_390_273, -26, RoundingMode::Ceiling, -123);
    test::<i64, isize>(-8_254_390_273, -26, RoundingMode::Nearest, -123);

    test::<i128, i8>(-0xffff_ffff, -1, RoundingMode::Down, -0x7fff_ffff);
    test::<i128, i8>(-0xffff_ffff, -1, RoundingMode::Up, -0x8000_0000);
    test::<i128, i8>(-0xffff_ffff, -1, RoundingMode::Floor, -0x8000_0000);
    test::<i128, i8>(-0xffff_ffff, -1, RoundingMode::Ceiling, -0x7fff_ffff);
    test::<i128, i8>(-0xffff_ffff, -1, RoundingMode::Nearest, -0x8000_0000);

    test::<i128, i16>(-0x1_0000_0000, -1, RoundingMode::Down, -0x8000_0000);
    test::<i128, i16>(-0x1_0000_0000, -1, RoundingMode::Up, -0x8000_0000);
    test::<i128, i16>(-0x1_0000_0000, -1, RoundingMode::Floor, -0x8000_0000);
    test::<i128, i16>(-0x1_0000_0000, -1, RoundingMode::Ceiling, -0x8000_0000);
    test::<i128, i16>(-0x1_0000_0000, -1, RoundingMode::Nearest, -0x8000_0000);
    test::<i128, i16>(-0x1_0000_0000, -1, RoundingMode::Exact, -0x8000_0000);

    test::<i128, i32>(-0x1_0000_0001, -1, RoundingMode::Down, -0x8000_0000);
    test::<i128, i32>(-0x1_0000_0001, -1, RoundingMode::Up, -0x8000_0001);
    test::<i128, i32>(-0x1_0000_0001, -1, RoundingMode::Floor, -0x8000_0001);
    test::<i128, i32>(-0x1_0000_0001, -1, RoundingMode::Ceiling, -0x8000_0000);
    test::<i128, i32>(-0x1_0000_0001, -1, RoundingMode::Nearest, -0x8000_0000);

    test::<i128, i64>(
        -1_000_000_000_000,
        0,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, i64>(-1_000_000_000_000, 0, RoundingMode::Up, -1_000_000_000_000);
    test::<i128, i64>(
        -1_000_000_000_000,
        0,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, i64>(
        -1_000_000_000_000,
        0,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, i64>(
        -1_000_000_000_000,
        0,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, i64>(
        -1_000_000_000_000,
        0,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i128, i128>(-7_999_999_999_999, -3, RoundingMode::Down, -999_999_999_999);
    test::<i128, i128>(-7_999_999_999_999, -3, RoundingMode::Up, -1_000_000_000_000);
    test::<i128, i128>(
        -7_999_999_999_999,
        -3,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, i128>(
        -7_999_999_999_999,
        -3,
        RoundingMode::Ceiling,
        -999_999_999_999,
    );
    test::<i128, i128>(
        -7_999_999_999_999,
        -3,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );

    test::<i128, isize>(
        -8_000_000_000_000,
        -3,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, isize>(-8_000_000_000_000, -3, RoundingMode::Up, -1_000_000_000_000);
    test::<i128, isize>(
        -8_000_000_000_000,
        -3,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, isize>(
        -8_000_000_000_000,
        -3,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, isize>(
        -8_000_000_000_000,
        -3,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, isize>(
        -8_000_000_000_000,
        -3,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i64, i8>(
        -8_000_000_000_001,
        -3,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i64, i8>(-8_000_000_000_001, -3, RoundingMode::Up, -1_000_000_000_001);
    test::<i64, i8>(
        -8_000_000_000_001,
        -3,
        RoundingMode::Floor,
        -1_000_000_000_001,
    );
    test::<i64, i8>(
        -8_000_000_000_001,
        -3,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i64, i8>(
        -8_000_000_000_001,
        -3,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );

    test::<i128, i16>(
        -16_777_216_000_000_000_000,
        -24,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, i16>(
        -16_777_216_000_000_000_000,
        -24,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128, i16>(
        -16_777_216_000_000_000_000,
        -24,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, i16>(
        -16_777_216_000_000_000_000,
        -24,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, i16>(
        -16_777_216_000_000_000_000,
        -24,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, i16>(
        -16_777_216_000_000_000_000,
        -24,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i128, i32>(
        -33_554_432_000_000_000_000,
        -25,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, i32>(
        -33_554_432_000_000_000_000,
        -25,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128, i32>(
        -33_554_432_000_000_000_000,
        -25,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, i32>(
        -33_554_432_000_000_000_000,
        -25,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, i32>(
        -33_554_432_000_000_000_000,
        -25,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, i32>(
        -33_554_432_000_000_000_000,
        -25,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i128, i64>(
        -2_147_483_648_000_000_000_000,
        -31,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, i64>(
        -2_147_483_648_000_000_000_000,
        -31,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128, i64>(
        -2_147_483_648_000_000_000_000,
        -31,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, i64>(
        -2_147_483_648_000_000_000_000,
        -31,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, i64>(
        -2_147_483_648_000_000_000_000,
        -31,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, i64>(
        -2_147_483_648_000_000_000_000,
        -31,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i128, i128>(
        -4_294_967_296_000_000_000_000,
        -32,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, i128>(
        -4_294_967_296_000_000_000_000,
        -32,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128, i128>(
        -4_294_967_296_000_000_000_000,
        -32,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, i128>(
        -4_294_967_296_000_000_000_000,
        -32,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, i128>(
        -4_294_967_296_000_000_000_000,
        -32,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, i128>(
        -4_294_967_296_000_000_000_000,
        -32,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i128, isize>(
        -8_589_934_592_000_000_000_000,
        -33,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, isize>(
        -8_589_934_592_000_000_000_000,
        -33,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128, isize>(
        -8_589_934_592_000_000_000_000,
        -33,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, isize>(
        -8_589_934_592_000_000_000_000,
        -33,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, isize>(
        -8_589_934_592_000_000_000_000,
        -33,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, isize>(
        -8_589_934_592_000_000_000_000,
        -33,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i64, i8>(-1_000_000_000_000, -10, RoundingMode::Down, -976_562_500);
    test::<i64, i8>(-1_000_000_000_000, -10, RoundingMode::Up, -976_562_500);
    test::<i64, i8>(-1_000_000_000_000, -10, RoundingMode::Floor, -976_562_500);
    test::<i64, i8>(-1_000_000_000_000, -10, RoundingMode::Ceiling, -976_562_500);
    test::<i64, i8>(-1_000_000_000_000, -10, RoundingMode::Nearest, -976_562_500);
    test::<i64, i8>(-1_000_000_000_000, -10, RoundingMode::Exact, -976_562_500);

    test::<i64, i16>(-980_657_949, -72, RoundingMode::Down, 0);
    test::<i64, i16>(-980_657_949, -72, RoundingMode::Up, -1);
    test::<i64, i16>(-980_657_949, -72, RoundingMode::Floor, -1);
    test::<i64, i16>(-980_657_949, -72, RoundingMode::Ceiling, 0);
    test::<i64, i16>(-980_657_949, -72, RoundingMode::Nearest, 0);

    test::<i64, i32>(-0xffff_ffff, -31, RoundingMode::Down, -1);
    test::<i64, i32>(-0xffff_ffff, -31, RoundingMode::Up, -2);
    test::<i64, i32>(-0xffff_ffff, -31, RoundingMode::Floor, -2);
    test::<i64, i32>(-0xffff_ffff, -31, RoundingMode::Ceiling, -1);
    test::<i64, i32>(-0xffff_ffff, -31, RoundingMode::Nearest, -2);

    test::<i64, i64>(-0xffff_ffff, -32, RoundingMode::Down, 0);
    test::<i64, i64>(-0xffff_ffff, -32, RoundingMode::Up, -1);
    test::<i64, i64>(-0xffff_ffff, -32, RoundingMode::Floor, -1);
    test::<i64, i64>(-0xffff_ffff, -32, RoundingMode::Ceiling, 0);
    test::<i64, i64>(-0xffff_ffff, -32, RoundingMode::Nearest, -1);

    test::<i64, i128>(-0x1_0000_0000, -32, RoundingMode::Down, -1);
    test::<i64, i128>(-0x1_0000_0000, -32, RoundingMode::Up, -1);
    test::<i64, i128>(-0x1_0000_0000, -32, RoundingMode::Floor, -1);
    test::<i64, i128>(-0x1_0000_0000, -32, RoundingMode::Ceiling, -1);
    test::<i64, i128>(-0x1_0000_0000, -32, RoundingMode::Nearest, -1);
    test::<i64, i128>(-0x1_0000_0000, -32, RoundingMode::Exact, -1);

    test::<i64, isize>(-0x1_0000_0000, -33, RoundingMode::Down, 0);
    test::<i64, isize>(-0x1_0000_0000, -33, RoundingMode::Up, -1);
    test::<i64, isize>(-0x1_0000_0000, -33, RoundingMode::Floor, -1);
    test::<i64, isize>(-0x1_0000_0000, -33, RoundingMode::Ceiling, 0);
    test::<i64, isize>(-0x1_0000_0000, -33, RoundingMode::Nearest, 0);

    test::<i16, i8>(-123, 1, RoundingMode::Exact, -246);
    test::<i16, i16>(-123, 2, RoundingMode::Exact, -492);
    test::<i64, i8>(-123, 25, RoundingMode::Exact, -4_127_195_136);
    test::<i64, i16>(-123, 26, RoundingMode::Exact, -8_254_390_272);
    test::<i64, i32>(-0x8000_0000, 1, RoundingMode::Exact, -0x1_0000_0000);
    test::<i64, i64>(
        -1_000_000_000_000,
        3,
        RoundingMode::Exact,
        -8_000_000_000_000,
    );
    test::<i128, i128>(
        -1_000_000_000_000,
        24,
        RoundingMode::Exact,
        -16_777_216_000_000_000_000,
    );
    test::<i128, isize>(
        -1_000_000_000_000,
        25,
        RoundingMode::Exact,
        -33_554_432_000_000_000_000,
    );
    test::<i128, i8>(
        -1_000_000_000_000,
        31,
        RoundingMode::Exact,
        -2_147_483_648_000_000_000_000,
    );
    test::<i128, i16>(
        -1_000_000_000_000,
        32,
        RoundingMode::Exact,
        -4_294_967_296_000_000_000_000,
    );
    test::<i128, i32>(
        -1_000_000_000_000,
        33,
        RoundingMode::Exact,
        -8_589_934_592_000_000_000_000,
    );
}

macro_rules! test_shl_round_fail {
    (
        $t:ident,
        $u:ident,
        $shl_round_assign_fail_1:ident,
        $shl_round_assign_fail_2:ident,
        $shl_round_fail_1:ident,
        $shl_round_fail_2:ident
    ) => {
        #[test]
        #[should_panic]
        fn $shl_round_assign_fail_1() {
            $t::exact_from(123).shl_round_assign($u::NEGATIVE_ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shl_round_assign_fail_2() {
            $t::exact_from(123).shl_round_assign($u::exact_from(-100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shl_round_fail_1() {
            $t::exact_from(123).shl_round($u::NEGATIVE_ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shl_round_fail_2() {
            $t::exact_from(123).shl_round($u::exact_from(-100), RoundingMode::Exact);
        }
    };
}

test_shl_round_fail!(
    u8,
    i8,
    shl_round_assign_u8_i8_fail_1,
    shl_round_assign_u8_i8_fail_2,
    shl_round_u8_i8_fail_1,
    shl_round_u8_i8_fail_2
);
test_shl_round_fail!(
    u8,
    i16,
    shl_round_assign_u8_i16_fail_1,
    shl_round_assign_u8_i16_fail_2,
    shl_round_u8_i16_fail_1,
    shl_round_u8_i16_fail_2
);
test_shl_round_fail!(
    u8,
    i32,
    shl_round_assign_u8_i32_fail_1,
    shl_round_assign_u8_i32_fail_2,
    shl_round_u8_i32_fail_1,
    shl_round_u8_i32_fail_2
);
test_shl_round_fail!(
    u8,
    i64,
    shl_round_assign_u8_i64_fail_1,
    shl_round_assign_u8_i64_fail_2,
    shl_round_u8_i64_fail_1,
    shl_round_u8_i64_fail_2
);
test_shl_round_fail!(
    u8,
    i128,
    shl_round_assign_u8_i128_fail_1,
    shl_round_assign_u8_i128_fail_2,
    shl_round_u8_i128_fail_1,
    shl_round_u8_i128_fail_2
);
test_shl_round_fail!(
    u8,
    isize,
    shl_round_assign_u8_isize_fail_1,
    shl_round_assign_u8_isize_fail_2,
    shl_round_u8_isize_fail_1,
    shl_round_u8_isize_fail_2
);
test_shl_round_fail!(
    u16,
    i8,
    shl_round_assign_u16_i8_fail_1,
    shl_round_assign_u16_i8_fail_2,
    shl_round_u16_i8_fail_1,
    shl_round_u16_i8_fail_2
);
test_shl_round_fail!(
    u16,
    i16,
    shl_round_assign_u16_i16_fail_1,
    shl_round_assign_u16_i16_fail_2,
    shl_round_u16_i16_fail_1,
    shl_round_u16_i16_fail_2
);
test_shl_round_fail!(
    u16,
    i32,
    shl_round_assign_u16_i32_fail_1,
    shl_round_assign_u16_i32_fail_2,
    shl_round_u16_i32_fail_1,
    shl_round_u16_i32_fail_2
);
test_shl_round_fail!(
    u16,
    i64,
    shl_round_assign_u16_i64_fail_1,
    shl_round_assign_u16_i64_fail_2,
    shl_round_u16_i64_fail_1,
    shl_round_u16_i64_fail_2
);
test_shl_round_fail!(
    u16,
    i128,
    shl_round_assign_u16_i128_fail_1,
    shl_round_assign_u16_i128_fail_2,
    shl_round_u16_i128_fail_1,
    shl_round_u16_i128_fail_2
);
test_shl_round_fail!(
    u16,
    isize,
    shl_round_assign_u16_isize_fail_1,
    shl_round_assign_u16_isize_fail_2,
    shl_round_u16_isize_fail_1,
    shl_round_u16_isize_fail_2
);
test_shl_round_fail!(
    u32,
    i8,
    shl_round_assign_u32_i8_fail_1,
    shl_round_assign_u32_i8_fail_2,
    shl_round_u32_i8_fail_1,
    shl_round_u32_i8_fail_2
);
test_shl_round_fail!(
    u32,
    i16,
    shl_round_assign_u32_i16_fail_1,
    shl_round_assign_u32_i16_fail_2,
    shl_round_u32_i16_fail_1,
    shl_round_u32_i16_fail_2
);
test_shl_round_fail!(
    u32,
    i32,
    shl_round_assign_u32_i32_fail_1,
    shl_round_assign_u32_i32_fail_2,
    shl_round_u32_i32_fail_1,
    shl_round_u32_i32_fail_2
);
test_shl_round_fail!(
    u32,
    i64,
    shl_round_assign_u32_i64_fail_1,
    shl_round_assign_u32_i64_fail_2,
    shl_round_u32_i64_fail_1,
    shl_round_u32_i64_fail_2
);
test_shl_round_fail!(
    u32,
    i128,
    shl_round_assign_u32_i128_fail_1,
    shl_round_assign_u32_i128_fail_2,
    shl_round_u32_i128_fail_1,
    shl_round_u32_i128_fail_2
);
test_shl_round_fail!(
    u32,
    isize,
    shl_round_assign_u32_isize_fail_1,
    shl_round_assign_u32_isize_fail_2,
    shl_round_u32_isize_fail_1,
    shl_round_u32_isize_fail_2
);
test_shl_round_fail!(
    u64,
    i8,
    shl_round_assign_u64_i8_fail_1,
    shl_round_assign_u64_i8_fail_2,
    shl_round_u64_i8_fail_1,
    shl_round_u64_i8_fail_2
);
test_shl_round_fail!(
    u64,
    i16,
    shl_round_assign_u64_i16_fail_1,
    shl_round_assign_u64_i16_fail_2,
    shl_round_u64_i16_fail_1,
    shl_round_u64_i16_fail_2
);
test_shl_round_fail!(
    u64,
    i32,
    shl_round_assign_u64_i32_fail_1,
    shl_round_assign_u64_i32_fail_2,
    shl_round_u64_i32_fail_1,
    shl_round_u64_i32_fail_2
);
test_shl_round_fail!(
    u64,
    i64,
    shl_round_assign_u64_i64_fail_1,
    shl_round_assign_u64_i64_fail_2,
    shl_round_u64_i64_fail_1,
    shl_round_u64_i64_fail_2
);
test_shl_round_fail!(
    u64,
    i128,
    shl_round_assign_u64_i128_fail_1,
    shl_round_assign_u64_i128_fail_2,
    shl_round_u64_i128_fail_1,
    shl_round_u64_i128_fail_2
);
test_shl_round_fail!(
    u64,
    isize,
    shl_round_assign_u64_isize_fail_1,
    shl_round_assign_u64_isize_fail_2,
    shl_round_u64_isize_fail_1,
    shl_round_u64_isize_fail_2
);
test_shl_round_fail!(
    u128,
    i8,
    shl_round_assign_u128_i8_fail_1,
    shl_round_assign_u128_i8_fail_2,
    shl_round_u128_i8_fail_1,
    shl_round_u128_i8_fail_2
);
test_shl_round_fail!(
    u128,
    i16,
    shl_round_assign_u128_i16_fail_1,
    shl_round_assign_u128_i16_fail_2,
    shl_round_u128_i16_fail_1,
    shl_round_u128_i16_fail_2
);
test_shl_round_fail!(
    u128,
    i32,
    shl_round_assign_u128_i32_fail_1,
    shl_round_assign_u128_i32_fail_2,
    shl_round_u128_i32_fail_1,
    shl_round_u128_i32_fail_2
);
test_shl_round_fail!(
    u128,
    i64,
    shl_round_assign_u128_i64_fail_1,
    shl_round_assign_u128_i64_fail_2,
    shl_round_u128_i64_fail_1,
    shl_round_u128_i64_fail_2
);
test_shl_round_fail!(
    u128,
    i128,
    shl_round_assign_u128_i128_fail_1,
    shl_round_assign_u128_i128_fail_2,
    shl_round_u128_i128_fail_1,
    shl_round_u128_i128_fail_2
);
test_shl_round_fail!(
    u128,
    isize,
    shl_round_assign_u128_isize_fail_1,
    shl_round_assign_u128_isize_fail_2,
    shl_round_u128_isize_fail_1,
    shl_round_u128_isize_fail_2
);
test_shl_round_fail!(
    usize,
    i8,
    shl_round_assign_usize_i8_fail_1,
    shl_round_assign_usize_i8_fail_2,
    shl_round_usize_i8_fail_1,
    shl_round_usize_i8_fail_2
);
test_shl_round_fail!(
    usize,
    i16,
    shl_round_assign_usize_i16_fail_1,
    shl_round_assign_usize_i16_fail_2,
    shl_round_usize_i16_fail_1,
    shl_round_usize_i16_fail_2
);
test_shl_round_fail!(
    usize,
    i32,
    shl_round_assign_usize_i32_fail_1,
    shl_round_assign_usize_i32_fail_2,
    shl_round_usize_i32_fail_1,
    shl_round_usize_i32_fail_2
);
test_shl_round_fail!(
    usize,
    i64,
    shl_round_assign_usize_i64_fail_1,
    shl_round_assign_usize_i64_fail_2,
    shl_round_usize_i64_fail_1,
    shl_round_usize_i64_fail_2
);
test_shl_round_fail!(
    usize,
    i128,
    shl_round_assign_usize_i128_fail_1,
    shl_round_assign_usize_i128_fail_2,
    shl_round_usize_i128_fail_1,
    shl_round_usize_i128_fail_2
);
test_shl_round_fail!(
    usize,
    isize,
    shl_round_assign_usize_isize_fail_1,
    shl_round_assign_usize_isize_fail_2,
    shl_round_usize_isize_fail_1,
    shl_round_usize_isize_fail_2
);

test_shl_round_fail!(
    i8,
    i8,
    shl_round_assign_i8_i8_fail_1,
    shl_round_assign_i8_i8_fail_2,
    shl_round_i8_i8_fail_1,
    shl_round_i8_i8_fail_2
);
test_shl_round_fail!(
    i8,
    i16,
    shl_round_assign_i8_i16_fail_1,
    shl_round_assign_i8_i16_fail_2,
    shl_round_i8_i16_fail_1,
    shl_round_i8_i16_fail_2
);
test_shl_round_fail!(
    i8,
    i32,
    shl_round_assign_i8_i32_fail_1,
    shl_round_assign_i8_i32_fail_2,
    shl_round_i8_i32_fail_1,
    shl_round_i8_i32_fail_2
);
test_shl_round_fail!(
    i8,
    i64,
    shl_round_assign_i8_i64_fail_1,
    shl_round_assign_i8_i64_fail_2,
    shl_round_i8_i64_fail_1,
    shl_round_i8_i64_fail_2
);
test_shl_round_fail!(
    i8,
    i128,
    shl_round_assign_i8_i128_fail_1,
    shl_round_assign_i8_i128_fail_2,
    shl_round_i8_i128_fail_1,
    shl_round_i8_i128_fail_2
);
test_shl_round_fail!(
    i8,
    isize,
    shl_round_assign_i8_isize_fail_1,
    shl_round_assign_i8_isize_fail_2,
    shl_round_i8_isize_fail_1,
    shl_round_i8_isize_fail_2
);
test_shl_round_fail!(
    i16,
    i8,
    shl_round_assign_i16_i8_fail_1,
    shl_round_assign_i16_i8_fail_2,
    shl_round_i16_i8_fail_1,
    shl_round_i16_i8_fail_2
);
test_shl_round_fail!(
    i16,
    i16,
    shl_round_assign_i16_i16_fail_1,
    shl_round_assign_i16_i16_fail_2,
    shl_round_i16_i16_fail_1,
    shl_round_i16_i16_fail_2
);
test_shl_round_fail!(
    i16,
    i32,
    shl_round_assign_i16_i32_fail_1,
    shl_round_assign_i16_i32_fail_2,
    shl_round_i16_i32_fail_1,
    shl_round_i16_i32_fail_2
);
test_shl_round_fail!(
    i16,
    i64,
    shl_round_assign_i16_i64_fail_1,
    shl_round_assign_i16_i64_fail_2,
    shl_round_i16_i64_fail_1,
    shl_round_i16_i64_fail_2
);
test_shl_round_fail!(
    i16,
    i128,
    shl_round_assign_i16_i128_fail_1,
    shl_round_assign_i16_i128_fail_2,
    shl_round_i16_i128_fail_1,
    shl_round_i16_i128_fail_2
);
test_shl_round_fail!(
    i16,
    isize,
    shl_round_assign_i16_isize_fail_1,
    shl_round_assign_i16_isize_fail_2,
    shl_round_i16_isize_fail_1,
    shl_round_i16_isize_fail_2
);
test_shl_round_fail!(
    i32,
    i8,
    shl_round_assign_i32_i8_fail_1,
    shl_round_assign_i32_i8_fail_2,
    shl_round_i32_i8_fail_1,
    shl_round_i32_i8_fail_2
);
test_shl_round_fail!(
    i32,
    i16,
    shl_round_assign_i32_i16_fail_1,
    shl_round_assign_i32_i16_fail_2,
    shl_round_i32_i16_fail_1,
    shl_round_i32_i16_fail_2
);
test_shl_round_fail!(
    i32,
    i32,
    shl_round_assign_i32_i32_fail_1,
    shl_round_assign_i32_i32_fail_2,
    shl_round_i32_i32_fail_1,
    shl_round_i32_i32_fail_2
);
test_shl_round_fail!(
    i32,
    i64,
    shl_round_assign_i32_i64_fail_1,
    shl_round_assign_i32_i64_fail_2,
    shl_round_i32_i64_fail_1,
    shl_round_i32_i64_fail_2
);
test_shl_round_fail!(
    i32,
    i128,
    shl_round_assign_i32_i128_fail_1,
    shl_round_assign_i32_i128_fail_2,
    shl_round_i32_i128_fail_1,
    shl_round_i32_i128_fail_2
);
test_shl_round_fail!(
    i32,
    isize,
    shl_round_assign_i32_isize_fail_1,
    shl_round_assign_i32_isize_fail_2,
    shl_round_i32_isize_fail_1,
    shl_round_i32_isize_fail_2
);
test_shl_round_fail!(
    i64,
    i8,
    shl_round_assign_i64_i8_fail_1,
    shl_round_assign_i64_i8_fail_2,
    shl_round_i64_i8_fail_1,
    shl_round_i64_i8_fail_2
);
test_shl_round_fail!(
    i64,
    i16,
    shl_round_assign_i64_i16_fail_1,
    shl_round_assign_i64_i16_fail_2,
    shl_round_i64_i16_fail_1,
    shl_round_i64_i16_fail_2
);
test_shl_round_fail!(
    i64,
    i32,
    shl_round_assign_i64_i32_fail_1,
    shl_round_assign_i64_i32_fail_2,
    shl_round_i64_i32_fail_1,
    shl_round_i64_i32_fail_2
);
test_shl_round_fail!(
    i64,
    i64,
    shl_round_assign_i64_i64_fail_1,
    shl_round_assign_i64_i64_fail_2,
    shl_round_i64_i64_fail_1,
    shl_round_i64_i64_fail_2
);
test_shl_round_fail!(
    i64,
    i128,
    shl_round_assign_i64_i128_fail_1,
    shl_round_assign_i64_i128_fail_2,
    shl_round_i64_i128_fail_1,
    shl_round_i64_i128_fail_2
);
test_shl_round_fail!(
    i64,
    isize,
    shl_round_assign_i64_isize_fail_1,
    shl_round_assign_i64_isize_fail_2,
    shl_round_i64_isize_fail_1,
    shl_round_i64_isize_fail_2
);
test_shl_round_fail!(
    i128,
    i8,
    shl_round_assign_i128_i8_fail_1,
    shl_round_assign_i128_i8_fail_2,
    shl_round_i128_i8_fail_1,
    shl_round_i128_i8_fail_2
);
test_shl_round_fail!(
    i128,
    i16,
    shl_round_assign_i128_i16_fail_1,
    shl_round_assign_i128_i16_fail_2,
    shl_round_i128_i16_fail_1,
    shl_round_i128_i16_fail_2
);
test_shl_round_fail!(
    i128,
    i32,
    shl_round_assign_i128_i32_fail_1,
    shl_round_assign_i128_i32_fail_2,
    shl_round_i128_i32_fail_1,
    shl_round_i128_i32_fail_2
);
test_shl_round_fail!(
    i128,
    i64,
    shl_round_assign_i128_i64_fail_1,
    shl_round_assign_i128_i64_fail_2,
    shl_round_i128_i64_fail_1,
    shl_round_i128_i64_fail_2
);
test_shl_round_fail!(
    i128,
    i128,
    shl_round_assign_i128_i128_fail_1,
    shl_round_assign_i128_i128_fail_2,
    shl_round_i128_i128_fail_1,
    shl_round_i128_i128_fail_2
);
test_shl_round_fail!(
    i128,
    isize,
    shl_round_assign_i128_isize_fail_1,
    shl_round_assign_i128_isize_fail_2,
    shl_round_i128_isize_fail_1,
    shl_round_i128_isize_fail_2
);
test_shl_round_fail!(
    isize,
    i8,
    shl_round_assign_isize_i8_fail_1,
    shl_round_assign_isize_i8_fail_2,
    shl_round_isize_i8_fail_1,
    shl_round_isize_i8_fail_2
);
test_shl_round_fail!(
    isize,
    i16,
    shl_round_assign_isize_i16_fail_1,
    shl_round_assign_isize_i16_fail_2,
    shl_round_isize_i16_fail_1,
    shl_round_isize_i16_fail_2
);
test_shl_round_fail!(
    isize,
    i32,
    shl_round_assign_isize_i32_fail_1,
    shl_round_assign_isize_i32_fail_2,
    shl_round_isize_i32_fail_1,
    shl_round_isize_i32_fail_2
);
test_shl_round_fail!(
    isize,
    i64,
    shl_round_assign_isize_i64_fail_1,
    shl_round_assign_isize_i64_fail_2,
    shl_round_isize_i64_fail_1,
    shl_round_isize_i64_fail_2
);
test_shl_round_fail!(
    isize,
    i128,
    shl_round_assign_isize_i128_fail_1,
    shl_round_assign_isize_i128_fail_2,
    shl_round_isize_i128_fail_1,
    shl_round_isize_i128_fail_2
);
test_shl_round_fail!(
    isize,
    isize,
    shl_round_assign_isize_isize_fail_1,
    shl_round_assign_isize_isize_fail_2,
    shl_round_isize_isize_fail_1,
    shl_round_isize_isize_fail_2
);
