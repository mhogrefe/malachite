use std::panic::catch_unwind;

use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::rounding_modes::RoundingMode;

#[test]
fn test_shr_round() {
    fn test<T: PrimitiveInteger, U: PrimitiveInteger>(t: T, u: U, rm: RoundingMode, out: T)
    where
        T: ShrRound<U, Output = T> + ShrRoundAssign<U>,
    {
        assert_eq!(t.shr_round(u, rm), out);

        let mut t = t;
        t.shr_round_assign(u, rm);
        assert_eq!(t, out);
    };
    test::<u8, u8>(0, 0, RoundingMode::Down, 0);
    test::<u8, u8>(0, 0, RoundingMode::Up, 0);
    test::<u8, u8>(0, 0, RoundingMode::Floor, 0);
    test::<u8, u8>(0, 0, RoundingMode::Ceiling, 0);
    test::<u8, u8>(0, 0, RoundingMode::Nearest, 0);
    test::<u8, u8>(0, 0, RoundingMode::Exact, 0);

    test::<u8, i16>(0, 10, RoundingMode::Down, 0);
    test::<u8, i16>(0, 10, RoundingMode::Up, 0);
    test::<u8, i16>(0, 10, RoundingMode::Floor, 0);
    test::<u8, i16>(0, 10, RoundingMode::Ceiling, 0);
    test::<u8, i16>(0, 10, RoundingMode::Nearest, 0);
    test::<u8, i16>(0, 10, RoundingMode::Exact, 0);

    test::<i8, u32>(123, 0, RoundingMode::Down, 123);
    test::<i8, u32>(123, 0, RoundingMode::Up, 123);
    test::<i8, u32>(123, 0, RoundingMode::Floor, 123);
    test::<i8, u32>(123, 0, RoundingMode::Ceiling, 123);
    test::<i8, u32>(123, 0, RoundingMode::Nearest, 123);
    test::<i8, u32>(123, 0, RoundingMode::Exact, 123);

    test::<u8, u64>(245, 1, RoundingMode::Down, 122);
    test::<u8, u64>(245, 1, RoundingMode::Up, 123);
    test::<u8, u64>(245, 1, RoundingMode::Floor, 122);
    test::<u8, u64>(245, 1, RoundingMode::Ceiling, 123);
    test::<u8, u64>(245, 1, RoundingMode::Nearest, 122);

    test::<u8, u128>(246, 1, RoundingMode::Down, 123);
    test::<u8, u128>(246, 1, RoundingMode::Up, 123);
    test::<u8, u128>(246, 1, RoundingMode::Floor, 123);
    test::<u8, u128>(246, 1, RoundingMode::Ceiling, 123);
    test::<u8, u128>(246, 1, RoundingMode::Nearest, 123);
    test::<u8, u128>(246, 1, RoundingMode::Exact, 123);

    test::<u8, usize>(247, 1, RoundingMode::Down, 123);
    test::<u8, usize>(247, 1, RoundingMode::Up, 124);
    test::<u8, usize>(247, 1, RoundingMode::Floor, 123);
    test::<u8, usize>(247, 1, RoundingMode::Ceiling, 124);
    test::<u8, usize>(247, 1, RoundingMode::Nearest, 124);

    test::<i16, i8>(491, 2, RoundingMode::Down, 122);
    test::<i16, i8>(491, 2, RoundingMode::Up, 123);
    test::<i16, i8>(491, 2, RoundingMode::Floor, 122);
    test::<i16, i8>(491, 2, RoundingMode::Ceiling, 123);
    test::<i16, i8>(491, 2, RoundingMode::Nearest, 123);

    test::<u16, i16>(492, 2, RoundingMode::Down, 123);
    test::<u16, i16>(492, 2, RoundingMode::Up, 123);
    test::<u16, i16>(492, 2, RoundingMode::Floor, 123);
    test::<u16, i16>(492, 2, RoundingMode::Ceiling, 123);
    test::<u16, i16>(492, 2, RoundingMode::Nearest, 123);
    test::<u16, i16>(492, 2, RoundingMode::Exact, 123);

    test::<i16, u32>(493, 2, RoundingMode::Down, 123);
    test::<i16, u32>(493, 2, RoundingMode::Up, 124);
    test::<i16, u32>(493, 2, RoundingMode::Floor, 123);
    test::<i16, u32>(493, 2, RoundingMode::Ceiling, 124);
    test::<i16, u32>(493, 2, RoundingMode::Nearest, 123);

    test::<u32, i8>(4_127_195_135, 25, RoundingMode::Down, 122);
    test::<u32, i8>(4_127_195_135, 25, RoundingMode::Up, 123);
    test::<u32, i8>(4_127_195_135, 25, RoundingMode::Floor, 122);
    test::<u32, i8>(4_127_195_135, 25, RoundingMode::Ceiling, 123);
    test::<u32, i8>(4_127_195_135, 25, RoundingMode::Nearest, 123);

    test::<u32, u16>(4_127_195_136, 25, RoundingMode::Down, 123);
    test::<u32, u16>(4_127_195_136, 25, RoundingMode::Up, 123);
    test::<u32, u16>(4_127_195_136, 25, RoundingMode::Floor, 123);
    test::<u32, u16>(4_127_195_136, 25, RoundingMode::Ceiling, 123);
    test::<u32, u16>(4_127_195_136, 25, RoundingMode::Nearest, 123);
    test::<u32, u16>(4_127_195_136, 25, RoundingMode::Exact, 123);

    test::<u32, i32>(4_127_195_137, 25, RoundingMode::Down, 123);
    test::<u32, i32>(4_127_195_137, 25, RoundingMode::Up, 124);
    test::<u32, i32>(4_127_195_137, 25, RoundingMode::Floor, 123);
    test::<u32, i32>(4_127_195_137, 25, RoundingMode::Ceiling, 124);
    test::<u32, i32>(4_127_195_137, 25, RoundingMode::Nearest, 123);

    test::<i64, u8>(8_254_390_271, 26, RoundingMode::Down, 122);
    test::<i64, u8>(8_254_390_271, 26, RoundingMode::Up, 123);
    test::<i64, u8>(8_254_390_271, 26, RoundingMode::Floor, 122);
    test::<i64, u8>(8_254_390_271, 26, RoundingMode::Ceiling, 123);
    test::<i64, u8>(8_254_390_271, 26, RoundingMode::Nearest, 123);

    test::<u64, i16>(8_254_390_272, 26, RoundingMode::Down, 123);
    test::<u64, i16>(8_254_390_272, 26, RoundingMode::Up, 123);
    test::<u64, i16>(8_254_390_272, 26, RoundingMode::Floor, 123);
    test::<u64, i16>(8_254_390_272, 26, RoundingMode::Ceiling, 123);
    test::<u64, i16>(8_254_390_272, 26, RoundingMode::Nearest, 123);
    test::<u64, i16>(8_254_390_272, 26, RoundingMode::Exact, 123);

    test::<i64, u32>(8_254_390_273, 26, RoundingMode::Down, 123);
    test::<i64, u32>(8_254_390_273, 26, RoundingMode::Up, 124);
    test::<i64, u32>(8_254_390_273, 26, RoundingMode::Floor, 123);
    test::<i64, u32>(8_254_390_273, 26, RoundingMode::Ceiling, 124);
    test::<i64, u32>(8_254_390_273, 26, RoundingMode::Nearest, 123);

    test::<i64, i64>(0xffff_ffff, 1, RoundingMode::Down, 0x7fff_ffff);
    test::<i64, i64>(0xffff_ffff, 1, RoundingMode::Up, 0x8000_0000);
    test::<i64, i64>(0xffff_ffff, 1, RoundingMode::Floor, 0x7fff_ffff);
    test::<i64, i64>(0xffff_ffff, 1, RoundingMode::Ceiling, 0x8000_0000);
    test::<i64, i64>(0xffff_ffff, 1, RoundingMode::Nearest, 0x8000_0000);

    test::<u64, u64>(0x1_0000_0000, 1, RoundingMode::Down, 0x8000_0000);
    test::<u64, u64>(0x1_0000_0000, 1, RoundingMode::Up, 0x8000_0000);
    test::<u64, u64>(0x1_0000_0000, 1, RoundingMode::Floor, 0x8000_0000);
    test::<u64, u64>(0x1_0000_0000, 1, RoundingMode::Ceiling, 0x8000_0000);
    test::<u64, u64>(0x1_0000_0000, 1, RoundingMode::Nearest, 0x8000_0000);
    test::<u64, u64>(0x1_0000_0000, 1, RoundingMode::Exact, 0x8000_0000);

    test::<u64, i128>(0x1_0000_0001, 1, RoundingMode::Down, 0x8000_0000);
    test::<u64, i128>(0x1_0000_0001, 1, RoundingMode::Up, 0x8000_0001);
    test::<u64, i128>(0x1_0000_0001, 1, RoundingMode::Floor, 0x8000_0000);
    test::<u64, i128>(0x1_0000_0001, 1, RoundingMode::Ceiling, 0x8000_0001);
    test::<u64, i128>(0x1_0000_0001, 1, RoundingMode::Nearest, 0x8000_0000);

    test::<i64, usize>(1_000_000_000_000, 0, RoundingMode::Down, 1_000_000_000_000);
    test::<i64, usize>(1_000_000_000_000, 0, RoundingMode::Up, 1_000_000_000_000);
    test::<i64, usize>(1_000_000_000_000, 0, RoundingMode::Floor, 1_000_000_000_000);
    test::<i64, usize>(
        1_000_000_000_000,
        0,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<i64, usize>(
        1_000_000_000_000,
        0,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );
    test::<i64, usize>(1_000_000_000_000, 0, RoundingMode::Exact, 1_000_000_000_000);

    test::<i128, i8>(7_999_999_999_999, 3, RoundingMode::Down, 999_999_999_999);
    test::<i128, i8>(7_999_999_999_999, 3, RoundingMode::Up, 1_000_000_000_000);
    test::<i128, i8>(7_999_999_999_999, 3, RoundingMode::Floor, 999_999_999_999);
    test::<i128, i8>(
        7_999_999_999_999,
        3,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<i128, i8>(
        7_999_999_999_999,
        3,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );

    test::<u128, u16>(8_000_000_000_000, 3, RoundingMode::Down, 1_000_000_000_000);
    test::<u128, u16>(8_000_000_000_000, 3, RoundingMode::Up, 1_000_000_000_000);
    test::<u128, u16>(8_000_000_000_000, 3, RoundingMode::Floor, 1_000_000_000_000);
    test::<u128, u16>(
        8_000_000_000_000,
        3,
        RoundingMode::Ceiling,
        1_000_000_000_000,
    );
    test::<u128, u16>(
        8_000_000_000_000,
        3,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );
    test::<u128, u16>(8_000_000_000_000, 3, RoundingMode::Exact, 1_000_000_000_000);

    test::<u128, i32>(8_000_000_000_001, 3, RoundingMode::Down, 1_000_000_000_000);
    test::<u128, i32>(8_000_000_000_001, 3, RoundingMode::Up, 1_000_000_000_001);
    test::<u128, i32>(8_000_000_000_001, 3, RoundingMode::Floor, 1_000_000_000_000);
    test::<u128, i32>(
        8_000_000_000_001,
        3,
        RoundingMode::Ceiling,
        1_000_000_000_001,
    );
    test::<u128, i32>(
        8_000_000_000_001,
        3,
        RoundingMode::Nearest,
        1_000_000_000_000,
    );

    test::<i128, u64>(1_000_000_000_000, 10, RoundingMode::Down, 976_562_500);
    test::<i128, u64>(1_000_000_000_000, 10, RoundingMode::Up, 976_562_500);
    test::<i128, u64>(1_000_000_000_000, 10, RoundingMode::Floor, 976_562_500);
    test::<i128, u64>(1_000_000_000_000, 10, RoundingMode::Ceiling, 976_562_500);
    test::<i128, u64>(1_000_000_000_000, 10, RoundingMode::Nearest, 976_562_500);
    test::<i128, u64>(1_000_000_000_000, 10, RoundingMode::Exact, 976_562_500);

    test::<u128, i128>(980_657_949, 72, RoundingMode::Down, 0);
    test::<u128, i128>(980_657_949, 72, RoundingMode::Up, 1);
    test::<u128, i128>(980_657_949, 72, RoundingMode::Floor, 0);
    test::<u128, i128>(980_657_949, 72, RoundingMode::Ceiling, 1);
    test::<u128, i128>(980_657_949, 72, RoundingMode::Nearest, 0);

    test::<i128, isize>(0xffff_ffff, 31, RoundingMode::Down, 1);
    test::<i128, isize>(0xffff_ffff, 31, RoundingMode::Up, 2);
    test::<i128, isize>(0xffff_ffff, 31, RoundingMode::Floor, 1);
    test::<i128, isize>(0xffff_ffff, 31, RoundingMode::Ceiling, 2);
    test::<i128, isize>(0xffff_ffff, 31, RoundingMode::Nearest, 2);

    test::<u32, u128>(0xffff_ffff, 32, RoundingMode::Down, 0);
    test::<u32, u128>(0xffff_ffff, 32, RoundingMode::Up, 1);
    test::<u32, u128>(0xffff_ffff, 32, RoundingMode::Floor, 0);
    test::<u32, u128>(0xffff_ffff, 32, RoundingMode::Ceiling, 1);
    test::<u32, u128>(0xffff_ffff, 32, RoundingMode::Nearest, 1);

    test::<u64, i8>(0x1_0000_0000, 32, RoundingMode::Down, 1);
    test::<u64, i8>(0x1_0000_0000, 32, RoundingMode::Up, 1);
    test::<u64, i8>(0x1_0000_0000, 32, RoundingMode::Floor, 1);
    test::<u64, i8>(0x1_0000_0000, 32, RoundingMode::Ceiling, 1);
    test::<u64, i8>(0x1_0000_0000, 32, RoundingMode::Nearest, 1);
    test::<u64, i8>(0x1_0000_0000, 32, RoundingMode::Exact, 1);

    test::<i64, u16>(0x1_0000_0000, 33, RoundingMode::Down, 0);
    test::<i64, u16>(0x1_0000_0000, 33, RoundingMode::Up, 1);
    test::<i64, u16>(0x1_0000_0000, 33, RoundingMode::Floor, 0);
    test::<i64, u16>(0x1_0000_0000, 33, RoundingMode::Ceiling, 1);
    test::<i64, u16>(0x1_0000_0000, 33, RoundingMode::Nearest, 0);

    test::<u8, i8>(0, -10, RoundingMode::Exact, 0);
    test::<u8, i16>(123, -1, RoundingMode::Exact, 246);
    test::<u16, i32>(123, -2, RoundingMode::Exact, 492);
    test::<u64, i64>(123, -25, RoundingMode::Exact, 4_127_195_136);
    test::<u128, i128>(123, -26, RoundingMode::Exact, 8_254_390_272);
    test::<u8, isize>(123, -100, RoundingMode::Exact, 0);

    test::<u64, i8>(0x8000_0000, -1, RoundingMode::Exact, 0x1_0000_0000);
    test::<i64, i16>(
        1_000_000_000_000,
        -3,
        RoundingMode::Exact,
        8_000_000_000_000,
    );
    test::<u64, i8>(
        1_000_000_000_000,
        -24,
        RoundingMode::Exact,
        16_777_216_000_000_000_000,
    );
    test::<i128, i16>(
        1_000_000_000_000,
        -25,
        RoundingMode::Exact,
        33_554_432_000_000_000_000,
    );
    test::<u128, i32>(
        1_000_000_000_000,
        -31,
        RoundingMode::Exact,
        2_147_483_648_000_000_000_000,
    );
    test::<i128, i64>(
        1_000_000_000_000,
        -32,
        RoundingMode::Exact,
        4_294_967_296_000_000_000_000,
    );
    test::<u128, i128>(
        1_000_000_000_000,
        -33,
        RoundingMode::Exact,
        8_589_934_592_000_000_000_000,
    );
    test::<i64, isize>(1_000_000_000_000, -100, RoundingMode::Exact, 0);

    test::<i8, u8>(-123, 0, RoundingMode::Down, -123);
    test::<i8, u8>(-123, 0, RoundingMode::Up, -123);
    test::<i8, u8>(-123, 0, RoundingMode::Floor, -123);
    test::<i8, u8>(-123, 0, RoundingMode::Ceiling, -123);
    test::<i8, u8>(-123, 0, RoundingMode::Nearest, -123);
    test::<i8, u8>(-123, 0, RoundingMode::Exact, -123);

    test::<i16, i8>(-245, 1, RoundingMode::Down, -122);
    test::<i16, i8>(-245, 1, RoundingMode::Up, -123);
    test::<i16, i8>(-245, 1, RoundingMode::Floor, -123);
    test::<i16, i8>(-245, 1, RoundingMode::Ceiling, -122);
    test::<i16, i8>(-245, 1, RoundingMode::Nearest, -122);

    test::<i16, u16>(-246, 1, RoundingMode::Down, -123);
    test::<i16, u16>(-246, 1, RoundingMode::Up, -123);
    test::<i16, u16>(-246, 1, RoundingMode::Floor, -123);
    test::<i16, u16>(-246, 1, RoundingMode::Ceiling, -123);
    test::<i16, u16>(-246, 1, RoundingMode::Nearest, -123);
    test::<i16, u16>(-246, 1, RoundingMode::Exact, -123);

    test::<i16, i32>(-247, 1, RoundingMode::Down, -123);
    test::<i16, i32>(-247, 1, RoundingMode::Up, -124);
    test::<i16, i32>(-247, 1, RoundingMode::Floor, -124);
    test::<i16, i32>(-247, 1, RoundingMode::Ceiling, -123);
    test::<i16, i32>(-247, 1, RoundingMode::Nearest, -124);

    test::<i16, u64>(-491, 2, RoundingMode::Down, -122);
    test::<i16, u64>(-491, 2, RoundingMode::Up, -123);
    test::<i16, u64>(-491, 2, RoundingMode::Floor, -123);
    test::<i16, u64>(-491, 2, RoundingMode::Ceiling, -122);
    test::<i16, u64>(-491, 2, RoundingMode::Nearest, -123);

    test::<i16, i128>(-492, 2, RoundingMode::Down, -123);
    test::<i16, i128>(-492, 2, RoundingMode::Up, -123);
    test::<i16, i128>(-492, 2, RoundingMode::Floor, -123);
    test::<i16, i128>(-492, 2, RoundingMode::Ceiling, -123);
    test::<i16, i128>(-492, 2, RoundingMode::Nearest, -123);
    test::<i16, i128>(-492, 2, RoundingMode::Exact, -123);

    test::<i16, usize>(-493, 2, RoundingMode::Down, -123);
    test::<i16, usize>(-493, 2, RoundingMode::Up, -124);
    test::<i16, usize>(-493, 2, RoundingMode::Floor, -124);
    test::<i16, usize>(-493, 2, RoundingMode::Ceiling, -123);
    test::<i16, usize>(-493, 2, RoundingMode::Nearest, -123);

    test::<i64, i8>(-4_127_195_135, 25, RoundingMode::Down, -122);
    test::<i64, i8>(-4_127_195_135, 25, RoundingMode::Up, -123);
    test::<i64, i8>(-4_127_195_135, 25, RoundingMode::Floor, -123);
    test::<i64, i8>(-4_127_195_135, 25, RoundingMode::Ceiling, -122);
    test::<i64, i8>(-4_127_195_135, 25, RoundingMode::Nearest, -123);

    test::<i64, u16>(-4_127_195_136, 25, RoundingMode::Down, -123);
    test::<i64, u16>(-4_127_195_136, 25, RoundingMode::Up, -123);
    test::<i64, u16>(-4_127_195_136, 25, RoundingMode::Floor, -123);
    test::<i64, u16>(-4_127_195_136, 25, RoundingMode::Ceiling, -123);
    test::<i64, u16>(-4_127_195_136, 25, RoundingMode::Nearest, -123);
    test::<i64, u16>(-4_127_195_136, 25, RoundingMode::Exact, -123);

    test::<i64, i32>(-4_127_195_137, 25, RoundingMode::Down, -123);
    test::<i64, i32>(-4_127_195_137, 25, RoundingMode::Up, -124);
    test::<i64, i32>(-4_127_195_137, 25, RoundingMode::Floor, -124);
    test::<i64, i32>(-4_127_195_137, 25, RoundingMode::Ceiling, -123);
    test::<i64, i32>(-4_127_195_137, 25, RoundingMode::Nearest, -123);

    test::<i64, u64>(-8_254_390_271, 26, RoundingMode::Down, -122);
    test::<i64, u64>(-8_254_390_271, 26, RoundingMode::Up, -123);
    test::<i64, u64>(-8_254_390_271, 26, RoundingMode::Floor, -123);
    test::<i64, u64>(-8_254_390_271, 26, RoundingMode::Ceiling, -122);
    test::<i64, u64>(-8_254_390_271, 26, RoundingMode::Nearest, -123);

    test::<i64, i128>(-8_254_390_272, 26, RoundingMode::Down, -123);
    test::<i64, i128>(-8_254_390_272, 26, RoundingMode::Up, -123);
    test::<i64, i128>(-8_254_390_272, 26, RoundingMode::Floor, -123);
    test::<i64, i128>(-8_254_390_272, 26, RoundingMode::Ceiling, -123);
    test::<i64, i128>(-8_254_390_272, 26, RoundingMode::Nearest, -123);
    test::<i64, i128>(-8_254_390_272, 26, RoundingMode::Exact, -123);

    test::<i64, usize>(-8_254_390_273, 26, RoundingMode::Down, -123);
    test::<i64, usize>(-8_254_390_273, 26, RoundingMode::Up, -124);
    test::<i64, usize>(-8_254_390_273, 26, RoundingMode::Floor, -124);
    test::<i64, usize>(-8_254_390_273, 26, RoundingMode::Ceiling, -123);
    test::<i64, usize>(-8_254_390_273, 26, RoundingMode::Nearest, -123);

    test::<i128, i8>(-0xffff_ffff, 1, RoundingMode::Down, -0x7fff_ffff);
    test::<i128, i8>(-0xffff_ffff, 1, RoundingMode::Up, -0x8000_0000);
    test::<i128, i8>(-0xffff_ffff, 1, RoundingMode::Floor, -0x8000_0000);
    test::<i128, i8>(-0xffff_ffff, 1, RoundingMode::Ceiling, -0x7fff_ffff);
    test::<i128, i8>(-0xffff_ffff, 1, RoundingMode::Nearest, -0x8000_0000);

    test::<i128, u16>(-0x1_0000_0000, 1, RoundingMode::Down, -0x8000_0000);
    test::<i128, u16>(-0x1_0000_0000, 1, RoundingMode::Up, -0x8000_0000);
    test::<i128, u16>(-0x1_0000_0000, 1, RoundingMode::Floor, -0x8000_0000);
    test::<i128, u16>(-0x1_0000_0000, 1, RoundingMode::Ceiling, -0x8000_0000);
    test::<i128, u16>(-0x1_0000_0000, 1, RoundingMode::Nearest, -0x8000_0000);
    test::<i128, u16>(-0x1_0000_0000, 1, RoundingMode::Exact, -0x8000_0000);

    test::<i128, i32>(-0x1_0000_0001, 1, RoundingMode::Down, -0x8000_0000);
    test::<i128, i32>(-0x1_0000_0001, 1, RoundingMode::Up, -0x8000_0001);
    test::<i128, i32>(-0x1_0000_0001, 1, RoundingMode::Floor, -0x8000_0001);
    test::<i128, i32>(-0x1_0000_0001, 1, RoundingMode::Ceiling, -0x8000_0000);
    test::<i128, i32>(-0x1_0000_0001, 1, RoundingMode::Nearest, -0x8000_0000);

    test::<i128, u64>(
        -1_000_000_000_000,
        0,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, u64>(-1_000_000_000_000, 0, RoundingMode::Up, -1_000_000_000_000);
    test::<i128, u64>(
        -1_000_000_000_000,
        0,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, u64>(
        -1_000_000_000_000,
        0,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, u64>(
        -1_000_000_000_000,
        0,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, u64>(
        -1_000_000_000_000,
        0,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i128, i128>(-7_999_999_999_999, 3, RoundingMode::Down, -999_999_999_999);
    test::<i128, i128>(-7_999_999_999_999, 3, RoundingMode::Up, -1_000_000_000_000);
    test::<i128, i128>(
        -7_999_999_999_999,
        3,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, i128>(
        -7_999_999_999_999,
        3,
        RoundingMode::Ceiling,
        -999_999_999_999,
    );
    test::<i128, i128>(
        -7_999_999_999_999,
        3,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );

    test::<i128, usize>(
        -8_000_000_000_000,
        3,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, usize>(-8_000_000_000_000, 3, RoundingMode::Up, -1_000_000_000_000);
    test::<i128, usize>(
        -8_000_000_000_000,
        3,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, usize>(
        -8_000_000_000_000,
        3,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, usize>(
        -8_000_000_000_000,
        3,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, usize>(
        -8_000_000_000_000,
        3,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i64, i8>(
        -8_000_000_000_001,
        3,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i64, i8>(-8_000_000_000_001, 3, RoundingMode::Up, -1_000_000_000_001);
    test::<i64, i8>(
        -8_000_000_000_001,
        3,
        RoundingMode::Floor,
        -1_000_000_000_001,
    );
    test::<i64, i8>(
        -8_000_000_000_001,
        3,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i64, i8>(
        -8_000_000_000_001,
        3,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );

    test::<i128, u16>(
        -16_777_216_000_000_000_000,
        24,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, u16>(
        -16_777_216_000_000_000_000,
        24,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128, u16>(
        -16_777_216_000_000_000_000,
        24,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, u16>(
        -16_777_216_000_000_000_000,
        24,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, u16>(
        -16_777_216_000_000_000_000,
        24,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, u16>(
        -16_777_216_000_000_000_000,
        24,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i128, i32>(
        -33_554_432_000_000_000_000,
        25,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, i32>(
        -33_554_432_000_000_000_000,
        25,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128, i32>(
        -33_554_432_000_000_000_000,
        25,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, i32>(
        -33_554_432_000_000_000_000,
        25,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, i32>(
        -33_554_432_000_000_000_000,
        25,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, i32>(
        -33_554_432_000_000_000_000,
        25,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i128, u64>(
        -2_147_483_648_000_000_000_000,
        31,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, u64>(
        -2_147_483_648_000_000_000_000,
        31,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128, u64>(
        -2_147_483_648_000_000_000_000,
        31,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, u64>(
        -2_147_483_648_000_000_000_000,
        31,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, u64>(
        -2_147_483_648_000_000_000_000,
        31,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, u64>(
        -2_147_483_648_000_000_000_000,
        31,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i128, i128>(
        -4_294_967_296_000_000_000_000,
        32,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, i128>(
        -4_294_967_296_000_000_000_000,
        32,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128, i128>(
        -4_294_967_296_000_000_000_000,
        32,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, i128>(
        -4_294_967_296_000_000_000_000,
        32,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, i128>(
        -4_294_967_296_000_000_000_000,
        32,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, i128>(
        -4_294_967_296_000_000_000_000,
        32,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i128, usize>(
        -8_589_934_592_000_000_000_000,
        33,
        RoundingMode::Down,
        -1_000_000_000_000,
    );
    test::<i128, usize>(
        -8_589_934_592_000_000_000_000,
        33,
        RoundingMode::Up,
        -1_000_000_000_000,
    );
    test::<i128, usize>(
        -8_589_934_592_000_000_000_000,
        33,
        RoundingMode::Floor,
        -1_000_000_000_000,
    );
    test::<i128, usize>(
        -8_589_934_592_000_000_000_000,
        33,
        RoundingMode::Ceiling,
        -1_000_000_000_000,
    );
    test::<i128, usize>(
        -8_589_934_592_000_000_000_000,
        33,
        RoundingMode::Nearest,
        -1_000_000_000_000,
    );
    test::<i128, usize>(
        -8_589_934_592_000_000_000_000,
        33,
        RoundingMode::Exact,
        -1_000_000_000_000,
    );

    test::<i64, i8>(-1_000_000_000_000, 10, RoundingMode::Down, -976_562_500);
    test::<i64, i8>(-1_000_000_000_000, 10, RoundingMode::Up, -976_562_500);
    test::<i64, i8>(-1_000_000_000_000, 10, RoundingMode::Floor, -976_562_500);
    test::<i64, i8>(-1_000_000_000_000, 10, RoundingMode::Ceiling, -976_562_500);
    test::<i64, i8>(-1_000_000_000_000, 10, RoundingMode::Nearest, -976_562_500);
    test::<i64, i8>(-1_000_000_000_000, 10, RoundingMode::Exact, -976_562_500);

    test::<i64, u16>(-980_657_949, 72, RoundingMode::Down, 0);
    test::<i64, u16>(-980_657_949, 72, RoundingMode::Up, -1);
    test::<i64, u16>(-980_657_949, 72, RoundingMode::Floor, -1);
    test::<i64, u16>(-980_657_949, 72, RoundingMode::Ceiling, 0);
    test::<i64, u16>(-980_657_949, 72, RoundingMode::Nearest, 0);

    test::<i64, i32>(-0xffff_ffff, 31, RoundingMode::Down, -1);
    test::<i64, i32>(-0xffff_ffff, 31, RoundingMode::Up, -2);
    test::<i64, i32>(-0xffff_ffff, 31, RoundingMode::Floor, -2);
    test::<i64, i32>(-0xffff_ffff, 31, RoundingMode::Ceiling, -1);
    test::<i64, i32>(-0xffff_ffff, 31, RoundingMode::Nearest, -2);

    test::<i64, u64>(-0xffff_ffff, 32, RoundingMode::Down, 0);
    test::<i64, u64>(-0xffff_ffff, 32, RoundingMode::Up, -1);
    test::<i64, u64>(-0xffff_ffff, 32, RoundingMode::Floor, -1);
    test::<i64, u64>(-0xffff_ffff, 32, RoundingMode::Ceiling, 0);
    test::<i64, u64>(-0xffff_ffff, 32, RoundingMode::Nearest, -1);

    test::<i64, i128>(-0x1_0000_0000, 32, RoundingMode::Down, -1);
    test::<i64, i128>(-0x1_0000_0000, 32, RoundingMode::Up, -1);
    test::<i64, i128>(-0x1_0000_0000, 32, RoundingMode::Floor, -1);
    test::<i64, i128>(-0x1_0000_0000, 32, RoundingMode::Ceiling, -1);
    test::<i64, i128>(-0x1_0000_0000, 32, RoundingMode::Nearest, -1);
    test::<i64, i128>(-0x1_0000_0000, 32, RoundingMode::Exact, -1);

    test::<i64, usize>(-0x1_0000_0000, 33, RoundingMode::Down, 0);
    test::<i64, usize>(-0x1_0000_0000, 33, RoundingMode::Up, -1);
    test::<i64, usize>(-0x1_0000_0000, 33, RoundingMode::Floor, -1);
    test::<i64, usize>(-0x1_0000_0000, 33, RoundingMode::Ceiling, 0);
    test::<i64, usize>(-0x1_0000_0000, 33, RoundingMode::Nearest, 0);

    test::<i16, i8>(-123, -1, RoundingMode::Exact, -246);
    test::<i16, i16>(-123, -2, RoundingMode::Exact, -492);
    test::<i64, i8>(-123, -25, RoundingMode::Exact, -4_127_195_136);
    test::<i64, i16>(-123, -26, RoundingMode::Exact, -8_254_390_272);
    test::<i64, i32>(-0x8000_0000, -1, RoundingMode::Exact, -0x1_0000_0000);
    test::<i64, i64>(
        -1_000_000_000_000,
        -3,
        RoundingMode::Exact,
        -8_000_000_000_000,
    );
    test::<i128, i128>(
        -1_000_000_000_000,
        -24,
        RoundingMode::Exact,
        -16_777_216_000_000_000_000,
    );
    test::<i128, isize>(
        -1_000_000_000_000,
        -25,
        RoundingMode::Exact,
        -33_554_432_000_000_000_000,
    );
    test::<i128, i8>(
        -1_000_000_000_000,
        -31,
        RoundingMode::Exact,
        -2_147_483_648_000_000_000_000,
    );
    test::<i128, i16>(
        -1_000_000_000_000,
        -32,
        RoundingMode::Exact,
        -4_294_967_296_000_000_000_000,
    );
    test::<i128, i32>(
        -1_000_000_000_000,
        -33,
        RoundingMode::Exact,
        -8_589_934_592_000_000_000_000,
    );
}

fn shr_round_fail_helper<T: PrimitiveInteger, U: PrimitiveInteger>()
where
    T: ShrRound<U, Output = T> + ShrRoundAssign<U>,
{
    assert_panic!(T::exact_from(123).shr_round(U::ONE, RoundingMode::Exact));
    assert_panic!(T::exact_from(123).shr_round(U::exact_from(100), RoundingMode::Exact));
    assert_panic!(T::exact_from(123).shr_round_assign(U::ONE, RoundingMode::Exact));
    assert_panic!(T::exact_from(123).shr_round_assign(U::exact_from(100), RoundingMode::Exact));
}

#[test]
fn shr_round_fail() {
    apply_fn_to_primitive_ints_and_primitive_ints!(shr_round_fail_helper);
}
