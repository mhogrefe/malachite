use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::round::RoundingMode;

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

    test::<u32, i8>(4127195135, 25, RoundingMode::Down, 122);
    test::<u32, i8>(4127195135, 25, RoundingMode::Up, 123);
    test::<u32, i8>(4127195135, 25, RoundingMode::Floor, 122);
    test::<u32, i8>(4127195135, 25, RoundingMode::Ceiling, 123);
    test::<u32, i8>(4127195135, 25, RoundingMode::Nearest, 123);

    test::<u32, u16>(4127195136, 25, RoundingMode::Down, 123);
    test::<u32, u16>(4127195136, 25, RoundingMode::Up, 123);
    test::<u32, u16>(4127195136, 25, RoundingMode::Floor, 123);
    test::<u32, u16>(4127195136, 25, RoundingMode::Ceiling, 123);
    test::<u32, u16>(4127195136, 25, RoundingMode::Nearest, 123);
    test::<u32, u16>(4127195136, 25, RoundingMode::Exact, 123);

    test::<u32, i32>(4127195137, 25, RoundingMode::Down, 123);
    test::<u32, i32>(4127195137, 25, RoundingMode::Up, 124);
    test::<u32, i32>(4127195137, 25, RoundingMode::Floor, 123);
    test::<u32, i32>(4127195137, 25, RoundingMode::Ceiling, 124);
    test::<u32, i32>(4127195137, 25, RoundingMode::Nearest, 123);

    test::<i64, u8>(8254390271, 26, RoundingMode::Down, 122);
    test::<i64, u8>(8254390271, 26, RoundingMode::Up, 123);
    test::<i64, u8>(8254390271, 26, RoundingMode::Floor, 122);
    test::<i64, u8>(8254390271, 26, RoundingMode::Ceiling, 123);
    test::<i64, u8>(8254390271, 26, RoundingMode::Nearest, 123);

    test::<u64, i16>(8254390272, 26, RoundingMode::Down, 123);
    test::<u64, i16>(8254390272, 26, RoundingMode::Up, 123);
    test::<u64, i16>(8254390272, 26, RoundingMode::Floor, 123);
    test::<u64, i16>(8254390272, 26, RoundingMode::Ceiling, 123);
    test::<u64, i16>(8254390272, 26, RoundingMode::Nearest, 123);
    test::<u64, i16>(8254390272, 26, RoundingMode::Exact, 123);

    test::<i64, u32>(8254390273, 26, RoundingMode::Down, 123);
    test::<i64, u32>(8254390273, 26, RoundingMode::Up, 124);
    test::<i64, u32>(8254390273, 26, RoundingMode::Floor, 123);
    test::<i64, u32>(8254390273, 26, RoundingMode::Ceiling, 124);
    test::<i64, u32>(8254390273, 26, RoundingMode::Nearest, 123);

    test::<i64, i64>(4294967295, 1, RoundingMode::Down, 2147483647);
    test::<i64, i64>(4294967295, 1, RoundingMode::Up, 2147483648);
    test::<i64, i64>(4294967295, 1, RoundingMode::Floor, 2147483647);
    test::<i64, i64>(4294967295, 1, RoundingMode::Ceiling, 2147483648);
    test::<i64, i64>(4294967295, 1, RoundingMode::Nearest, 2147483648);

    test::<u64, u64>(4294967296, 1, RoundingMode::Down, 2147483648);
    test::<u64, u64>(4294967296, 1, RoundingMode::Up, 2147483648);
    test::<u64, u64>(4294967296, 1, RoundingMode::Floor, 2147483648);
    test::<u64, u64>(4294967296, 1, RoundingMode::Ceiling, 2147483648);
    test::<u64, u64>(4294967296, 1, RoundingMode::Nearest, 2147483648);
    test::<u64, u64>(4294967296, 1, RoundingMode::Exact, 2147483648);

    test::<u64, i128>(4294967297, 1, RoundingMode::Down, 2147483648);
    test::<u64, i128>(4294967297, 1, RoundingMode::Up, 2147483649);
    test::<u64, i128>(4294967297, 1, RoundingMode::Floor, 2147483648);
    test::<u64, i128>(4294967297, 1, RoundingMode::Ceiling, 2147483649);
    test::<u64, i128>(4294967297, 1, RoundingMode::Nearest, 2147483648);

    test::<i64, usize>(1000000000000, 0, RoundingMode::Down, 1000000000000);
    test::<i64, usize>(1000000000000, 0, RoundingMode::Up, 1000000000000);
    test::<i64, usize>(1000000000000, 0, RoundingMode::Floor, 1000000000000);
    test::<i64, usize>(1000000000000, 0, RoundingMode::Ceiling, 1000000000000);
    test::<i64, usize>(1000000000000, 0, RoundingMode::Nearest, 1000000000000);
    test::<i64, usize>(1000000000000, 0, RoundingMode::Exact, 1000000000000);

    test::<i128, i8>(7999999999999, 3, RoundingMode::Down, 999999999999);
    test::<i128, i8>(7999999999999, 3, RoundingMode::Up, 1000000000000);
    test::<i128, i8>(7999999999999, 3, RoundingMode::Floor, 999999999999);
    test::<i128, i8>(7999999999999, 3, RoundingMode::Ceiling, 1000000000000);
    test::<i128, i8>(7999999999999, 3, RoundingMode::Nearest, 1000000000000);

    test::<u128, u16>(8000000000000, 3, RoundingMode::Down, 1000000000000);
    test::<u128, u16>(8000000000000, 3, RoundingMode::Up, 1000000000000);
    test::<u128, u16>(8000000000000, 3, RoundingMode::Floor, 1000000000000);
    test::<u128, u16>(8000000000000, 3, RoundingMode::Ceiling, 1000000000000);
    test::<u128, u16>(8000000000000, 3, RoundingMode::Nearest, 1000000000000);
    test::<u128, u16>(8000000000000, 3, RoundingMode::Exact, 1000000000000);

    test::<u128, i32>(8000000000001, 3, RoundingMode::Down, 1000000000000);
    test::<u128, i32>(8000000000001, 3, RoundingMode::Up, 1000000000001);
    test::<u128, i32>(8000000000001, 3, RoundingMode::Floor, 1000000000000);
    test::<u128, i32>(8000000000001, 3, RoundingMode::Ceiling, 1000000000001);
    test::<u128, i32>(8000000000001, 3, RoundingMode::Nearest, 1000000000000);

    test::<i128, u64>(1000000000000, 10, RoundingMode::Down, 976562500);
    test::<i128, u64>(1000000000000, 10, RoundingMode::Up, 976562500);
    test::<i128, u64>(1000000000000, 10, RoundingMode::Floor, 976562500);
    test::<i128, u64>(1000000000000, 10, RoundingMode::Ceiling, 976562500);
    test::<i128, u64>(1000000000000, 10, RoundingMode::Nearest, 976562500);
    test::<i128, u64>(1000000000000, 10, RoundingMode::Exact, 976562500);

    test::<u128, i128>(980657949, 72, RoundingMode::Down, 0);
    test::<u128, i128>(980657949, 72, RoundingMode::Up, 1);
    test::<u128, i128>(980657949, 72, RoundingMode::Floor, 0);
    test::<u128, i128>(980657949, 72, RoundingMode::Ceiling, 1);
    test::<u128, i128>(980657949, 72, RoundingMode::Nearest, 0);

    test::<i128, isize>(4294967295, 31, RoundingMode::Down, 1);
    test::<i128, isize>(4294967295, 31, RoundingMode::Up, 2);
    test::<i128, isize>(4294967295, 31, RoundingMode::Floor, 1);
    test::<i128, isize>(4294967295, 31, RoundingMode::Ceiling, 2);
    test::<i128, isize>(4294967295, 31, RoundingMode::Nearest, 2);

    test::<u32, u128>(4294967295, 32, RoundingMode::Down, 0);
    test::<u32, u128>(4294967295, 32, RoundingMode::Up, 1);
    test::<u32, u128>(4294967295, 32, RoundingMode::Floor, 0);
    test::<u32, u128>(4294967295, 32, RoundingMode::Ceiling, 1);
    test::<u32, u128>(4294967295, 32, RoundingMode::Nearest, 1);

    test::<u64, i8>(4294967296, 32, RoundingMode::Down, 1);
    test::<u64, i8>(4294967296, 32, RoundingMode::Up, 1);
    test::<u64, i8>(4294967296, 32, RoundingMode::Floor, 1);
    test::<u64, i8>(4294967296, 32, RoundingMode::Ceiling, 1);
    test::<u64, i8>(4294967296, 32, RoundingMode::Nearest, 1);
    test::<u64, i8>(4294967296, 32, RoundingMode::Exact, 1);

    test::<i64, u16>(4294967296, 33, RoundingMode::Down, 0);
    test::<i64, u16>(4294967296, 33, RoundingMode::Up, 1);
    test::<i64, u16>(4294967296, 33, RoundingMode::Floor, 0);
    test::<i64, u16>(4294967296, 33, RoundingMode::Ceiling, 1);
    test::<i64, u16>(4294967296, 33, RoundingMode::Nearest, 0);

    test::<u8, i8>(0, -10, RoundingMode::Exact, 0);
    test::<u8, i16>(123, -1, RoundingMode::Exact, 246);
    test::<u16, i32>(123, -2, RoundingMode::Exact, 492);
    test::<u64, i64>(123, -25, RoundingMode::Exact, 4127195136);
    test::<u128, i128>(123, -26, RoundingMode::Exact, 8254390272);
    test::<u8, isize>(123, -100, RoundingMode::Exact, 0);

    test::<u64, i8>(2147483648, -1, RoundingMode::Exact, 4294967296);
    test::<i64, i16>(1000000000000, -3, RoundingMode::Exact, 8000000000000);
    test::<u64, i8>(
        1000000000000,
        -24,
        RoundingMode::Exact,
        16777216000000000000,
    );
    test::<i128, i16>(
        1000000000000,
        -25,
        RoundingMode::Exact,
        33554432000000000000,
    );
    test::<u128, i32>(
        1000000000000,
        -31,
        RoundingMode::Exact,
        2147483648000000000000,
    );
    test::<i128, i64>(
        1000000000000,
        -32,
        RoundingMode::Exact,
        4294967296000000000000,
    );
    test::<u128, i128>(
        1000000000000,
        -33,
        RoundingMode::Exact,
        8589934592000000000000,
    );
    test::<i64, isize>(1000000000000, -100, RoundingMode::Exact, 0);

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

    test::<i64, i8>(-4127195135, 25, RoundingMode::Down, -122);
    test::<i64, i8>(-4127195135, 25, RoundingMode::Up, -123);
    test::<i64, i8>(-4127195135, 25, RoundingMode::Floor, -123);
    test::<i64, i8>(-4127195135, 25, RoundingMode::Ceiling, -122);
    test::<i64, i8>(-4127195135, 25, RoundingMode::Nearest, -123);

    test::<i64, u16>(-4127195136, 25, RoundingMode::Down, -123);
    test::<i64, u16>(-4127195136, 25, RoundingMode::Up, -123);
    test::<i64, u16>(-4127195136, 25, RoundingMode::Floor, -123);
    test::<i64, u16>(-4127195136, 25, RoundingMode::Ceiling, -123);
    test::<i64, u16>(-4127195136, 25, RoundingMode::Nearest, -123);
    test::<i64, u16>(-4127195136, 25, RoundingMode::Exact, -123);

    test::<i64, i32>(-4127195137, 25, RoundingMode::Down, -123);
    test::<i64, i32>(-4127195137, 25, RoundingMode::Up, -124);
    test::<i64, i32>(-4127195137, 25, RoundingMode::Floor, -124);
    test::<i64, i32>(-4127195137, 25, RoundingMode::Ceiling, -123);
    test::<i64, i32>(-4127195137, 25, RoundingMode::Nearest, -123);

    test::<i64, u64>(-8254390271, 26, RoundingMode::Down, -122);
    test::<i64, u64>(-8254390271, 26, RoundingMode::Up, -123);
    test::<i64, u64>(-8254390271, 26, RoundingMode::Floor, -123);
    test::<i64, u64>(-8254390271, 26, RoundingMode::Ceiling, -122);
    test::<i64, u64>(-8254390271, 26, RoundingMode::Nearest, -123);

    test::<i64, i128>(-8254390272, 26, RoundingMode::Down, -123);
    test::<i64, i128>(-8254390272, 26, RoundingMode::Up, -123);
    test::<i64, i128>(-8254390272, 26, RoundingMode::Floor, -123);
    test::<i64, i128>(-8254390272, 26, RoundingMode::Ceiling, -123);
    test::<i64, i128>(-8254390272, 26, RoundingMode::Nearest, -123);
    test::<i64, i128>(-8254390272, 26, RoundingMode::Exact, -123);

    test::<i64, usize>(-8254390273, 26, RoundingMode::Down, -123);
    test::<i64, usize>(-8254390273, 26, RoundingMode::Up, -124);
    test::<i64, usize>(-8254390273, 26, RoundingMode::Floor, -124);
    test::<i64, usize>(-8254390273, 26, RoundingMode::Ceiling, -123);
    test::<i64, usize>(-8254390273, 26, RoundingMode::Nearest, -123);

    test::<i128, i8>(-4294967295, 1, RoundingMode::Down, -2147483647);
    test::<i128, i8>(-4294967295, 1, RoundingMode::Up, -2147483648);
    test::<i128, i8>(-4294967295, 1, RoundingMode::Floor, -2147483648);
    test::<i128, i8>(-4294967295, 1, RoundingMode::Ceiling, -2147483647);
    test::<i128, i8>(-4294967295, 1, RoundingMode::Nearest, -2147483648);

    test::<i128, u16>(-4294967296, 1, RoundingMode::Down, -2147483648);
    test::<i128, u16>(-4294967296, 1, RoundingMode::Up, -2147483648);
    test::<i128, u16>(-4294967296, 1, RoundingMode::Floor, -2147483648);
    test::<i128, u16>(-4294967296, 1, RoundingMode::Ceiling, -2147483648);
    test::<i128, u16>(-4294967296, 1, RoundingMode::Nearest, -2147483648);
    test::<i128, u16>(-4294967296, 1, RoundingMode::Exact, -2147483648);

    test::<i128, i32>(-4294967297, 1, RoundingMode::Down, -2147483648);
    test::<i128, i32>(-4294967297, 1, RoundingMode::Up, -2147483649);
    test::<i128, i32>(-4294967297, 1, RoundingMode::Floor, -2147483649);
    test::<i128, i32>(-4294967297, 1, RoundingMode::Ceiling, -2147483648);
    test::<i128, i32>(-4294967297, 1, RoundingMode::Nearest, -2147483648);

    test::<i128, u64>(-1000000000000, 0, RoundingMode::Down, -1000000000000);
    test::<i128, u64>(-1000000000000, 0, RoundingMode::Up, -1000000000000);
    test::<i128, u64>(-1000000000000, 0, RoundingMode::Floor, -1000000000000);
    test::<i128, u64>(-1000000000000, 0, RoundingMode::Ceiling, -1000000000000);
    test::<i128, u64>(-1000000000000, 0, RoundingMode::Nearest, -1000000000000);
    test::<i128, u64>(-1000000000000, 0, RoundingMode::Exact, -1000000000000);

    test::<i128, i128>(-7999999999999, 3, RoundingMode::Down, -999999999999);
    test::<i128, i128>(-7999999999999, 3, RoundingMode::Up, -1000000000000);
    test::<i128, i128>(-7999999999999, 3, RoundingMode::Floor, -1000000000000);
    test::<i128, i128>(-7999999999999, 3, RoundingMode::Ceiling, -999999999999);
    test::<i128, i128>(-7999999999999, 3, RoundingMode::Nearest, -1000000000000);

    test::<i128, usize>(-8000000000000, 3, RoundingMode::Down, -1000000000000);
    test::<i128, usize>(-8000000000000, 3, RoundingMode::Up, -1000000000000);
    test::<i128, usize>(-8000000000000, 3, RoundingMode::Floor, -1000000000000);
    test::<i128, usize>(-8000000000000, 3, RoundingMode::Ceiling, -1000000000000);
    test::<i128, usize>(-8000000000000, 3, RoundingMode::Nearest, -1000000000000);
    test::<i128, usize>(-8000000000000, 3, RoundingMode::Exact, -1000000000000);

    test::<i64, i8>(-8000000000001, 3, RoundingMode::Down, -1000000000000);
    test::<i64, i8>(-8000000000001, 3, RoundingMode::Up, -1000000000001);
    test::<i64, i8>(-8000000000001, 3, RoundingMode::Floor, -1000000000001);
    test::<i64, i8>(-8000000000001, 3, RoundingMode::Ceiling, -1000000000000);
    test::<i64, i8>(-8000000000001, 3, RoundingMode::Nearest, -1000000000000);

    test::<i128, u16>(
        -16777216000000000000,
        24,
        RoundingMode::Down,
        -1000000000000,
    );
    test::<i128, u16>(-16777216000000000000, 24, RoundingMode::Up, -1000000000000);
    test::<i128, u16>(
        -16777216000000000000,
        24,
        RoundingMode::Floor,
        -1000000000000,
    );
    test::<i128, u16>(
        -16777216000000000000,
        24,
        RoundingMode::Ceiling,
        -1000000000000,
    );
    test::<i128, u16>(
        -16777216000000000000,
        24,
        RoundingMode::Nearest,
        -1000000000000,
    );
    test::<i128, u16>(
        -16777216000000000000,
        24,
        RoundingMode::Exact,
        -1000000000000,
    );

    test::<i128, i32>(
        -33554432000000000000,
        25,
        RoundingMode::Down,
        -1000000000000,
    );
    test::<i128, i32>(-33554432000000000000, 25, RoundingMode::Up, -1000000000000);
    test::<i128, i32>(
        -33554432000000000000,
        25,
        RoundingMode::Floor,
        -1000000000000,
    );
    test::<i128, i32>(
        -33554432000000000000,
        25,
        RoundingMode::Ceiling,
        -1000000000000,
    );
    test::<i128, i32>(
        -33554432000000000000,
        25,
        RoundingMode::Nearest,
        -1000000000000,
    );
    test::<i128, i32>(
        -33554432000000000000,
        25,
        RoundingMode::Exact,
        -1000000000000,
    );

    test::<i128, u64>(
        -2147483648000000000000,
        31,
        RoundingMode::Down,
        -1000000000000,
    );
    test::<i128, u64>(
        -2147483648000000000000,
        31,
        RoundingMode::Up,
        -1000000000000,
    );
    test::<i128, u64>(
        -2147483648000000000000,
        31,
        RoundingMode::Floor,
        -1000000000000,
    );
    test::<i128, u64>(
        -2147483648000000000000,
        31,
        RoundingMode::Ceiling,
        -1000000000000,
    );
    test::<i128, u64>(
        -2147483648000000000000,
        31,
        RoundingMode::Nearest,
        -1000000000000,
    );
    test::<i128, u64>(
        -2147483648000000000000,
        31,
        RoundingMode::Exact,
        -1000000000000,
    );

    test::<i128, i128>(
        -4294967296000000000000,
        32,
        RoundingMode::Down,
        -1000000000000,
    );
    test::<i128, i128>(
        -4294967296000000000000,
        32,
        RoundingMode::Up,
        -1000000000000,
    );
    test::<i128, i128>(
        -4294967296000000000000,
        32,
        RoundingMode::Floor,
        -1000000000000,
    );
    test::<i128, i128>(
        -4294967296000000000000,
        32,
        RoundingMode::Ceiling,
        -1000000000000,
    );
    test::<i128, i128>(
        -4294967296000000000000,
        32,
        RoundingMode::Nearest,
        -1000000000000,
    );
    test::<i128, i128>(
        -4294967296000000000000,
        32,
        RoundingMode::Exact,
        -1000000000000,
    );

    test::<i128, usize>(
        -8589934592000000000000,
        33,
        RoundingMode::Down,
        -1000000000000,
    );
    test::<i128, usize>(
        -8589934592000000000000,
        33,
        RoundingMode::Up,
        -1000000000000,
    );
    test::<i128, usize>(
        -8589934592000000000000,
        33,
        RoundingMode::Floor,
        -1000000000000,
    );
    test::<i128, usize>(
        -8589934592000000000000,
        33,
        RoundingMode::Ceiling,
        -1000000000000,
    );
    test::<i128, usize>(
        -8589934592000000000000,
        33,
        RoundingMode::Nearest,
        -1000000000000,
    );
    test::<i128, usize>(
        -8589934592000000000000,
        33,
        RoundingMode::Exact,
        -1000000000000,
    );

    test::<i64, i8>(-1000000000000, 10, RoundingMode::Down, -976562500);
    test::<i64, i8>(-1000000000000, 10, RoundingMode::Up, -976562500);
    test::<i64, i8>(-1000000000000, 10, RoundingMode::Floor, -976562500);
    test::<i64, i8>(-1000000000000, 10, RoundingMode::Ceiling, -976562500);
    test::<i64, i8>(-1000000000000, 10, RoundingMode::Nearest, -976562500);
    test::<i64, i8>(-1000000000000, 10, RoundingMode::Exact, -976562500);

    test::<i64, u16>(-980657949, 72, RoundingMode::Down, 0);
    test::<i64, u16>(-980657949, 72, RoundingMode::Up, -1);
    test::<i64, u16>(-980657949, 72, RoundingMode::Floor, -1);
    test::<i64, u16>(-980657949, 72, RoundingMode::Ceiling, 0);
    test::<i64, u16>(-980657949, 72, RoundingMode::Nearest, 0);

    test::<i64, i32>(-4294967295, 31, RoundingMode::Down, -1);
    test::<i64, i32>(-4294967295, 31, RoundingMode::Up, -2);
    test::<i64, i32>(-4294967295, 31, RoundingMode::Floor, -2);
    test::<i64, i32>(-4294967295, 31, RoundingMode::Ceiling, -1);
    test::<i64, i32>(-4294967295, 31, RoundingMode::Nearest, -2);

    test::<i64, u64>(-4294967295, 32, RoundingMode::Down, 0);
    test::<i64, u64>(-4294967295, 32, RoundingMode::Up, -1);
    test::<i64, u64>(-4294967295, 32, RoundingMode::Floor, -1);
    test::<i64, u64>(-4294967295, 32, RoundingMode::Ceiling, 0);
    test::<i64, u64>(-4294967295, 32, RoundingMode::Nearest, -1);

    test::<i64, i128>(-4294967296, 32, RoundingMode::Down, -1);
    test::<i64, i128>(-4294967296, 32, RoundingMode::Up, -1);
    test::<i64, i128>(-4294967296, 32, RoundingMode::Floor, -1);
    test::<i64, i128>(-4294967296, 32, RoundingMode::Ceiling, -1);
    test::<i64, i128>(-4294967296, 32, RoundingMode::Nearest, -1);
    test::<i64, i128>(-4294967296, 32, RoundingMode::Exact, -1);

    test::<i64, usize>(-4294967296, 33, RoundingMode::Down, 0);
    test::<i64, usize>(-4294967296, 33, RoundingMode::Up, -1);
    test::<i64, usize>(-4294967296, 33, RoundingMode::Floor, -1);
    test::<i64, usize>(-4294967296, 33, RoundingMode::Ceiling, 0);
    test::<i64, usize>(-4294967296, 33, RoundingMode::Nearest, 0);

    test::<i16, i8>(-123, -1, RoundingMode::Exact, -246);
    test::<i16, i16>(-123, -2, RoundingMode::Exact, -492);
    test::<i64, i8>(-123, -25, RoundingMode::Exact, -4127195136);
    test::<i64, i16>(-123, -26, RoundingMode::Exact, -8254390272);
    test::<i64, i32>(-2147483648, -1, RoundingMode::Exact, -4294967296);
    test::<i64, i64>(-1000000000000, -3, RoundingMode::Exact, -8000000000000);
    test::<i128, i128>(
        -1000000000000,
        -24,
        RoundingMode::Exact,
        -16777216000000000000,
    );
    test::<i128, isize>(
        -1000000000000,
        -25,
        RoundingMode::Exact,
        -33554432000000000000,
    );
    test::<i128, i8>(
        -1000000000000,
        -31,
        RoundingMode::Exact,
        -2147483648000000000000,
    );
    test::<i128, i16>(
        -1000000000000,
        -32,
        RoundingMode::Exact,
        -4294967296000000000000,
    );
    test::<i128, i32>(
        -1000000000000,
        -33,
        RoundingMode::Exact,
        -8589934592000000000000,
    );
}

macro_rules! test_shr_round_fail {
    (
        $t:ident,
        $u:ident,
        $shr_round_assign_fail_1:ident,
        $shr_round_assign_fail_2:ident,
        $shr_round_fail_1:ident,
        $shr_round_fail_2:ident
    ) => {
        #[test]
        #[should_panic]
        fn $shr_round_assign_fail_1() {
            $t::exact_from(123).shr_round_assign($u::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_assign_fail_2() {
            $t::exact_from(123).shr_round_assign($u::exact_from(100), RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_fail_1() {
            $t::exact_from(123).shr_round($u::ONE, RoundingMode::Exact);
        }

        #[test]
        #[should_panic]
        fn $shr_round_fail_2() {
            $t::exact_from(123).shr_round($u::exact_from(100), RoundingMode::Exact);
        }
    };
}

test_shr_round_fail!(
    u8,
    u8,
    shr_round_assign_u8_u8_fail_1,
    shr_round_assign_u8_u8_fail_2,
    shr_round_u8_u8_fail_1,
    shr_round_u8_u8_fail_2
);
test_shr_round_fail!(
    u8,
    u16,
    shr_round_assign_u8_u16_fail_1,
    shr_round_assign_u8_u16_fail_2,
    shr_round_u8_u16_fail_1,
    shr_round_u8_u16_fail_2
);
test_shr_round_fail!(
    u8,
    u32,
    shr_round_assign_u8_u32_fail_1,
    shr_round_assign_u8_u32_fail_2,
    shr_round_u8_u32_fail_1,
    shr_round_u8_u32_fail_2
);
test_shr_round_fail!(
    u8,
    u64,
    shr_round_assign_u8_u64_fail_1,
    shr_round_assign_u8_u64_fail_2,
    shr_round_u8_u64_fail_1,
    shr_round_u8_u64_fail_2
);
test_shr_round_fail!(
    u8,
    u128,
    shr_round_assign_u8_u128_fail_1,
    shr_round_assign_u8_u128_fail_2,
    shr_round_u8_u128_fail_1,
    shr_round_u8_u128_fail_2
);
test_shr_round_fail!(
    u8,
    usize,
    shr_round_assign_u8_usize_fail_1,
    shr_round_assign_u8_usize_fail_2,
    shr_round_u8_usize_fail_1,
    shr_round_u8_usize_fail_2
);
test_shr_round_fail!(
    u16,
    u8,
    shr_round_assign_u16_u8_fail_1,
    shr_round_assign_u16_u8_fail_2,
    shr_round_u16_u8_fail_1,
    shr_round_u16_u8_fail_2
);
test_shr_round_fail!(
    u16,
    u16,
    shr_round_assign_u16_u16_fail_1,
    shr_round_assign_u16_u16_fail_2,
    shr_round_u16_u16_fail_1,
    shr_round_u16_u16_fail_2
);
test_shr_round_fail!(
    u16,
    u32,
    shr_round_assign_u16_u32_fail_1,
    shr_round_assign_u16_u32_fail_2,
    shr_round_u16_u32_fail_1,
    shr_round_u16_u32_fail_2
);
test_shr_round_fail!(
    u16,
    u64,
    shr_round_assign_u16_u64_fail_1,
    shr_round_assign_u16_u64_fail_2,
    shr_round_u16_u64_fail_1,
    shr_round_u16_u64_fail_2
);
test_shr_round_fail!(
    u16,
    u128,
    shr_round_assign_u16_u128_fail_1,
    shr_round_assign_u16_u128_fail_2,
    shr_round_u16_u128_fail_1,
    shr_round_u16_u128_fail_2
);
test_shr_round_fail!(
    u16,
    usize,
    shr_round_assign_u16_usize_fail_1,
    shr_round_assign_u16_usize_fail_2,
    shr_round_u16_usize_fail_1,
    shr_round_u16_usize_fail_2
);
test_shr_round_fail!(
    u32,
    u8,
    shr_round_assign_u32_u8_fail_1,
    shr_round_assign_u32_u8_fail_2,
    shr_round_u32_u8_fail_1,
    shr_round_u32_u8_fail_2
);
test_shr_round_fail!(
    u32,
    u16,
    shr_round_assign_u32_u16_fail_1,
    shr_round_assign_u32_u16_fail_2,
    shr_round_u32_u16_fail_1,
    shr_round_u32_u16_fail_2
);
test_shr_round_fail!(
    u32,
    u32,
    shr_round_assign_u32_u32_fail_1,
    shr_round_assign_u32_u32_fail_2,
    shr_round_u32_u32_fail_1,
    shr_round_u32_u32_fail_2
);
test_shr_round_fail!(
    u32,
    u64,
    shr_round_assign_u32_u64_fail_1,
    shr_round_assign_u32_u64_fail_2,
    shr_round_u32_u64_fail_1,
    shr_round_u32_u64_fail_2
);
test_shr_round_fail!(
    u32,
    u128,
    shr_round_assign_u32_u128_fail_1,
    shr_round_assign_u32_u128_fail_2,
    shr_round_u32_u128_fail_1,
    shr_round_u32_u128_fail_2
);
test_shr_round_fail!(
    u32,
    usize,
    shr_round_assign_u32_usize_fail_1,
    shr_round_assign_u32_usize_fail_2,
    shr_round_u32_usize_fail_1,
    shr_round_u32_usize_fail_2
);
test_shr_round_fail!(
    u64,
    u8,
    shr_round_assign_u64_u8_fail_1,
    shr_round_assign_u64_u8_fail_2,
    shr_round_u64_u8_fail_1,
    shr_round_u64_u8_fail_2
);
test_shr_round_fail!(
    u64,
    u16,
    shr_round_assign_u64_u16_fail_1,
    shr_round_assign_u64_u16_fail_2,
    shr_round_u64_u16_fail_1,
    shr_round_u64_u16_fail_2
);
test_shr_round_fail!(
    u64,
    u32,
    shr_round_assign_u64_u32_fail_1,
    shr_round_assign_u64_u32_fail_2,
    shr_round_u64_u32_fail_1,
    shr_round_u64_u32_fail_2
);
test_shr_round_fail!(
    u64,
    u64,
    shr_round_assign_u64_u64_fail_1,
    shr_round_assign_u64_u64_fail_2,
    shr_round_u64_u64_fail_1,
    shr_round_u64_u64_fail_2
);
test_shr_round_fail!(
    u64,
    u128,
    shr_round_assign_u64_u128_fail_1,
    shr_round_assign_u64_u128_fail_2,
    shr_round_u64_u128_fail_1,
    shr_round_u64_u128_fail_2
);
test_shr_round_fail!(
    u64,
    usize,
    shr_round_assign_u64_usize_fail_1,
    shr_round_assign_u64_usize_fail_2,
    shr_round_u64_usize_fail_1,
    shr_round_u64_usize_fail_2
);
test_shr_round_fail!(
    u128,
    u8,
    shr_round_assign_u128_u8_fail_1,
    shr_round_assign_u128_u8_fail_2,
    shr_round_u128_u8_fail_1,
    shr_round_u128_u8_fail_2
);
test_shr_round_fail!(
    u128,
    u16,
    shr_round_assign_u128_u16_fail_1,
    shr_round_assign_u128_u16_fail_2,
    shr_round_u128_u16_fail_1,
    shr_round_u128_u16_fail_2
);
test_shr_round_fail!(
    u128,
    u32,
    shr_round_assign_u128_u32_fail_1,
    shr_round_assign_u128_u32_fail_2,
    shr_round_u128_u32_fail_1,
    shr_round_u128_u32_fail_2
);
test_shr_round_fail!(
    u128,
    u64,
    shr_round_assign_u128_u64_fail_1,
    shr_round_assign_u128_u64_fail_2,
    shr_round_u128_u64_fail_1,
    shr_round_u128_u64_fail_2
);
test_shr_round_fail!(
    u128,
    u128,
    shr_round_assign_u128_u128_fail_1,
    shr_round_assign_u128_u128_fail_2,
    shr_round_u128_u128_fail_1,
    shr_round_u128_u128_fail_2
);
test_shr_round_fail!(
    u128,
    usize,
    shr_round_assign_u128_usize_fail_1,
    shr_round_assign_u128_usize_fail_2,
    shr_round_u128_usize_fail_1,
    shr_round_u128_usize_fail_2
);
test_shr_round_fail!(
    usize,
    u8,
    shr_round_assign_usize_u8_fail_1,
    shr_round_assign_usize_u8_fail_2,
    shr_round_usize_u8_fail_1,
    shr_round_usize_u8_fail_2
);
test_shr_round_fail!(
    usize,
    u16,
    shr_round_assign_usize_u16_fail_1,
    shr_round_assign_usize_u16_fail_2,
    shr_round_usize_u16_fail_1,
    shr_round_usize_u16_fail_2
);
test_shr_round_fail!(
    usize,
    u32,
    shr_round_assign_usize_u32_fail_1,
    shr_round_assign_usize_u32_fail_2,
    shr_round_usize_u32_fail_1,
    shr_round_usize_u32_fail_2
);
test_shr_round_fail!(
    usize,
    u64,
    shr_round_assign_usize_u64_fail_1,
    shr_round_assign_usize_u64_fail_2,
    shr_round_usize_u64_fail_1,
    shr_round_usize_u64_fail_2
);
test_shr_round_fail!(
    usize,
    u128,
    shr_round_assign_usize_u128_fail_1,
    shr_round_assign_usize_u128_fail_2,
    shr_round_usize_u128_fail_1,
    shr_round_usize_u128_fail_2
);
test_shr_round_fail!(
    usize,
    usize,
    shr_round_assign_usize_usize_fail_1,
    shr_round_assign_usize_usize_fail_2,
    shr_round_usize_usize_fail_1,
    shr_round_usize_usize_fail_2
);

test_shr_round_fail!(
    u8,
    i8,
    shr_round_assign_u8_i8_fail_1,
    shr_round_assign_u8_i8_fail_2,
    shr_round_u8_i8_fail_1,
    shr_round_u8_i8_fail_2
);
test_shr_round_fail!(
    u8,
    i16,
    shr_round_assign_u8_i16_fail_1,
    shr_round_assign_u8_i16_fail_2,
    shr_round_u8_i16_fail_1,
    shr_round_u8_i16_fail_2
);
test_shr_round_fail!(
    u8,
    i32,
    shr_round_assign_u8_i32_fail_1,
    shr_round_assign_u8_i32_fail_2,
    shr_round_u8_i32_fail_1,
    shr_round_u8_i32_fail_2
);
test_shr_round_fail!(
    u8,
    i64,
    shr_round_assign_u8_i64_fail_1,
    shr_round_assign_u8_i64_fail_2,
    shr_round_u8_i64_fail_1,
    shr_round_u8_i64_fail_2
);
test_shr_round_fail!(
    u8,
    i128,
    shr_round_assign_u8_i128_fail_1,
    shr_round_assign_u8_i128_fail_2,
    shr_round_u8_i128_fail_1,
    shr_round_u8_i128_fail_2
);
test_shr_round_fail!(
    u8,
    isize,
    shr_round_assign_u8_isize_fail_1,
    shr_round_assign_u8_isize_fail_2,
    shr_round_u8_isize_fail_1,
    shr_round_u8_isize_fail_2
);
test_shr_round_fail!(
    u16,
    i8,
    shr_round_assign_u16_i8_fail_1,
    shr_round_assign_u16_i8_fail_2,
    shr_round_u16_i8_fail_1,
    shr_round_u16_i8_fail_2
);
test_shr_round_fail!(
    u16,
    i16,
    shr_round_assign_u16_i16_fail_1,
    shr_round_assign_u16_i16_fail_2,
    shr_round_u16_i16_fail_1,
    shr_round_u16_i16_fail_2
);
test_shr_round_fail!(
    u16,
    i32,
    shr_round_assign_u16_i32_fail_1,
    shr_round_assign_u16_i32_fail_2,
    shr_round_u16_i32_fail_1,
    shr_round_u16_i32_fail_2
);
test_shr_round_fail!(
    u16,
    i64,
    shr_round_assign_u16_i64_fail_1,
    shr_round_assign_u16_i64_fail_2,
    shr_round_u16_i64_fail_1,
    shr_round_u16_i64_fail_2
);
test_shr_round_fail!(
    u16,
    i128,
    shr_round_assign_u16_i128_fail_1,
    shr_round_assign_u16_i128_fail_2,
    shr_round_u16_i128_fail_1,
    shr_round_u16_i128_fail_2
);
test_shr_round_fail!(
    u16,
    isize,
    shr_round_assign_u16_isize_fail_1,
    shr_round_assign_u16_isize_fail_2,
    shr_round_u16_isize_fail_1,
    shr_round_u16_isize_fail_2
);
test_shr_round_fail!(
    u32,
    i8,
    shr_round_assign_u32_i8_fail_1,
    shr_round_assign_u32_i8_fail_2,
    shr_round_u32_i8_fail_1,
    shr_round_u32_i8_fail_2
);
test_shr_round_fail!(
    u32,
    i16,
    shr_round_assign_u32_i16_fail_1,
    shr_round_assign_u32_i16_fail_2,
    shr_round_u32_i16_fail_1,
    shr_round_u32_i16_fail_2
);
test_shr_round_fail!(
    u32,
    i32,
    shr_round_assign_u32_i32_fail_1,
    shr_round_assign_u32_i32_fail_2,
    shr_round_u32_i32_fail_1,
    shr_round_u32_i32_fail_2
);
test_shr_round_fail!(
    u32,
    i64,
    shr_round_assign_u32_i64_fail_1,
    shr_round_assign_u32_i64_fail_2,
    shr_round_u32_i64_fail_1,
    shr_round_u32_i64_fail_2
);
test_shr_round_fail!(
    u32,
    i128,
    shr_round_assign_u32_i128_fail_1,
    shr_round_assign_u32_i128_fail_2,
    shr_round_u32_i128_fail_1,
    shr_round_u32_i128_fail_2
);
test_shr_round_fail!(
    u32,
    isize,
    shr_round_assign_u32_isize_fail_1,
    shr_round_assign_u32_isize_fail_2,
    shr_round_u32_isize_fail_1,
    shr_round_u32_isize_fail_2
);
test_shr_round_fail!(
    u64,
    i8,
    shr_round_assign_u64_i8_fail_1,
    shr_round_assign_u64_i8_fail_2,
    shr_round_u64_i8_fail_1,
    shr_round_u64_i8_fail_2
);
test_shr_round_fail!(
    u64,
    i16,
    shr_round_assign_u64_i16_fail_1,
    shr_round_assign_u64_i16_fail_2,
    shr_round_u64_i16_fail_1,
    shr_round_u64_i16_fail_2
);
test_shr_round_fail!(
    u64,
    i32,
    shr_round_assign_u64_i32_fail_1,
    shr_round_assign_u64_i32_fail_2,
    shr_round_u64_i32_fail_1,
    shr_round_u64_i32_fail_2
);
test_shr_round_fail!(
    u64,
    i64,
    shr_round_assign_u64_i64_fail_1,
    shr_round_assign_u64_i64_fail_2,
    shr_round_u64_i64_fail_1,
    shr_round_u64_i64_fail_2
);
test_shr_round_fail!(
    u64,
    i128,
    shr_round_assign_u64_i128_fail_1,
    shr_round_assign_u64_i128_fail_2,
    shr_round_u64_i128_fail_1,
    shr_round_u64_i128_fail_2
);
test_shr_round_fail!(
    u64,
    isize,
    shr_round_assign_u64_isize_fail_1,
    shr_round_assign_u64_isize_fail_2,
    shr_round_u64_isize_fail_1,
    shr_round_u64_isize_fail_2
);
test_shr_round_fail!(
    u128,
    i8,
    shr_round_assign_u128_i8_fail_1,
    shr_round_assign_u128_i8_fail_2,
    shr_round_u128_i8_fail_1,
    shr_round_u128_i8_fail_2
);
test_shr_round_fail!(
    u128,
    i16,
    shr_round_assign_u128_i16_fail_1,
    shr_round_assign_u128_i16_fail_2,
    shr_round_u128_i16_fail_1,
    shr_round_u128_i16_fail_2
);
test_shr_round_fail!(
    u128,
    i32,
    shr_round_assign_u128_i32_fail_1,
    shr_round_assign_u128_i32_fail_2,
    shr_round_u128_i32_fail_1,
    shr_round_u128_i32_fail_2
);
test_shr_round_fail!(
    u128,
    i64,
    shr_round_assign_u128_i64_fail_1,
    shr_round_assign_u128_i64_fail_2,
    shr_round_u128_i64_fail_1,
    shr_round_u128_i64_fail_2
);
test_shr_round_fail!(
    u128,
    i128,
    shr_round_assign_u128_i128_fail_1,
    shr_round_assign_u128_i128_fail_2,
    shr_round_u128_i128_fail_1,
    shr_round_u128_i128_fail_2
);
test_shr_round_fail!(
    u128,
    isize,
    shr_round_assign_u128_isize_fail_1,
    shr_round_assign_u128_isize_fail_2,
    shr_round_u128_isize_fail_1,
    shr_round_u128_isize_fail_2
);
test_shr_round_fail!(
    usize,
    i8,
    shr_round_assign_usize_i8_fail_1,
    shr_round_assign_usize_i8_fail_2,
    shr_round_usize_i8_fail_1,
    shr_round_usize_i8_fail_2
);
test_shr_round_fail!(
    usize,
    i16,
    shr_round_assign_usize_i16_fail_1,
    shr_round_assign_usize_i16_fail_2,
    shr_round_usize_i16_fail_1,
    shr_round_usize_i16_fail_2
);
test_shr_round_fail!(
    usize,
    i32,
    shr_round_assign_usize_i32_fail_1,
    shr_round_assign_usize_i32_fail_2,
    shr_round_usize_i32_fail_1,
    shr_round_usize_i32_fail_2
);
test_shr_round_fail!(
    usize,
    i64,
    shr_round_assign_usize_i64_fail_1,
    shr_round_assign_usize_i64_fail_2,
    shr_round_usize_i64_fail_1,
    shr_round_usize_i64_fail_2
);
test_shr_round_fail!(
    usize,
    i128,
    shr_round_assign_usize_i128_fail_1,
    shr_round_assign_usize_i128_fail_2,
    shr_round_usize_i128_fail_1,
    shr_round_usize_i128_fail_2
);
test_shr_round_fail!(
    usize,
    isize,
    shr_round_assign_usize_isize_fail_1,
    shr_round_assign_usize_isize_fail_2,
    shr_round_usize_isize_fail_1,
    shr_round_usize_isize_fail_2
);

test_shr_round_fail!(
    i8,
    u8,
    shr_round_assign_i8_u8_fail_1,
    shr_round_assign_i8_u8_fail_2,
    shr_round_i8_u8_fail_1,
    shr_round_i8_u8_fail_2
);
test_shr_round_fail!(
    i8,
    u16,
    shr_round_assign_i8_u16_fail_1,
    shr_round_assign_i8_u16_fail_2,
    shr_round_i8_u16_fail_1,
    shr_round_i8_u16_fail_2
);
test_shr_round_fail!(
    i8,
    u32,
    shr_round_assign_i8_u32_fail_1,
    shr_round_assign_i8_u32_fail_2,
    shr_round_i8_u32_fail_1,
    shr_round_i8_u32_fail_2
);
test_shr_round_fail!(
    i8,
    u64,
    shr_round_assign_i8_u64_fail_1,
    shr_round_assign_i8_u64_fail_2,
    shr_round_i8_u64_fail_1,
    shr_round_i8_u64_fail_2
);
test_shr_round_fail!(
    i8,
    u128,
    shr_round_assign_i8_u128_fail_1,
    shr_round_assign_i8_u128_fail_2,
    shr_round_i8_u128_fail_1,
    shr_round_i8_u128_fail_2
);
test_shr_round_fail!(
    i8,
    usize,
    shr_round_assign_i8_usize_fail_1,
    shr_round_assign_i8_usize_fail_2,
    shr_round_i8_usize_fail_1,
    shr_round_i8_usize_fail_2
);
test_shr_round_fail!(
    i16,
    u8,
    shr_round_assign_i16_u8_fail_1,
    shr_round_assign_i16_u8_fail_2,
    shr_round_i16_u8_fail_1,
    shr_round_i16_u8_fail_2
);
test_shr_round_fail!(
    i16,
    u16,
    shr_round_assign_i16_u16_fail_1,
    shr_round_assign_i16_u16_fail_2,
    shr_round_i16_u16_fail_1,
    shr_round_i16_u16_fail_2
);
test_shr_round_fail!(
    i16,
    u32,
    shr_round_assign_i16_u32_fail_1,
    shr_round_assign_i16_u32_fail_2,
    shr_round_i16_u32_fail_1,
    shr_round_i16_u32_fail_2
);
test_shr_round_fail!(
    i16,
    u64,
    shr_round_assign_i16_u64_fail_1,
    shr_round_assign_i16_u64_fail_2,
    shr_round_i16_u64_fail_1,
    shr_round_i16_u64_fail_2
);
test_shr_round_fail!(
    i16,
    u128,
    shr_round_assign_i16_u128_fail_1,
    shr_round_assign_i16_u128_fail_2,
    shr_round_i16_u128_fail_1,
    shr_round_i16_u128_fail_2
);
test_shr_round_fail!(
    i16,
    usize,
    shr_round_assign_i16_usize_fail_1,
    shr_round_assign_i16_usize_fail_2,
    shr_round_i16_usize_fail_1,
    shr_round_i16_usize_fail_2
);
test_shr_round_fail!(
    i32,
    u8,
    shr_round_assign_i32_u8_fail_1,
    shr_round_assign_i32_u8_fail_2,
    shr_round_i32_u8_fail_1,
    shr_round_i32_u8_fail_2
);
test_shr_round_fail!(
    i32,
    u16,
    shr_round_assign_i32_u16_fail_1,
    shr_round_assign_i32_u16_fail_2,
    shr_round_i32_u16_fail_1,
    shr_round_i32_u16_fail_2
);
test_shr_round_fail!(
    i32,
    u32,
    shr_round_assign_i32_u32_fail_1,
    shr_round_assign_i32_u32_fail_2,
    shr_round_i32_u32_fail_1,
    shr_round_i32_u32_fail_2
);
test_shr_round_fail!(
    i32,
    u64,
    shr_round_assign_i32_u64_fail_1,
    shr_round_assign_i32_u64_fail_2,
    shr_round_i32_u64_fail_1,
    shr_round_i32_u64_fail_2
);
test_shr_round_fail!(
    i32,
    u128,
    shr_round_assign_i32_u128_fail_1,
    shr_round_assign_i32_u128_fail_2,
    shr_round_i32_u128_fail_1,
    shr_round_i32_u128_fail_2
);
test_shr_round_fail!(
    i32,
    usize,
    shr_round_assign_i32_usize_fail_1,
    shr_round_assign_i32_usize_fail_2,
    shr_round_i32_usize_fail_1,
    shr_round_i32_usize_fail_2
);
test_shr_round_fail!(
    i64,
    u8,
    shr_round_assign_i64_u8_fail_1,
    shr_round_assign_i64_u8_fail_2,
    shr_round_i64_u8_fail_1,
    shr_round_i64_u8_fail_2
);
test_shr_round_fail!(
    i64,
    u16,
    shr_round_assign_i64_u16_fail_1,
    shr_round_assign_i64_u16_fail_2,
    shr_round_i64_u16_fail_1,
    shr_round_i64_u16_fail_2
);
test_shr_round_fail!(
    i64,
    u32,
    shr_round_assign_i64_u32_fail_1,
    shr_round_assign_i64_u32_fail_2,
    shr_round_i64_u32_fail_1,
    shr_round_i64_u32_fail_2
);
test_shr_round_fail!(
    i64,
    u64,
    shr_round_assign_i64_u64_fail_1,
    shr_round_assign_i64_u64_fail_2,
    shr_round_i64_u64_fail_1,
    shr_round_i64_u64_fail_2
);
test_shr_round_fail!(
    i64,
    u128,
    shr_round_assign_i64_u128_fail_1,
    shr_round_assign_i64_u128_fail_2,
    shr_round_i64_u128_fail_1,
    shr_round_i64_u128_fail_2
);
test_shr_round_fail!(
    i64,
    usize,
    shr_round_assign_i64_usize_fail_1,
    shr_round_assign_i64_usize_fail_2,
    shr_round_i64_usize_fail_1,
    shr_round_i64_usize_fail_2
);
test_shr_round_fail!(
    i128,
    u8,
    shr_round_assign_i128_u8_fail_1,
    shr_round_assign_i128_u8_fail_2,
    shr_round_i128_u8_fail_1,
    shr_round_i128_u8_fail_2
);
test_shr_round_fail!(
    i128,
    u16,
    shr_round_assign_i128_u16_fail_1,
    shr_round_assign_i128_u16_fail_2,
    shr_round_i128_u16_fail_1,
    shr_round_i128_u16_fail_2
);
test_shr_round_fail!(
    i128,
    u32,
    shr_round_assign_i128_u32_fail_1,
    shr_round_assign_i128_u32_fail_2,
    shr_round_i128_u32_fail_1,
    shr_round_i128_u32_fail_2
);
test_shr_round_fail!(
    i128,
    u64,
    shr_round_assign_i128_u64_fail_1,
    shr_round_assign_i128_u64_fail_2,
    shr_round_i128_u64_fail_1,
    shr_round_i128_u64_fail_2
);
test_shr_round_fail!(
    i128,
    u128,
    shr_round_assign_i128_u128_fail_1,
    shr_round_assign_i128_u128_fail_2,
    shr_round_i128_u128_fail_1,
    shr_round_i128_u128_fail_2
);
test_shr_round_fail!(
    i128,
    usize,
    shr_round_assign_i128_usize_fail_1,
    shr_round_assign_i128_usize_fail_2,
    shr_round_i128_usize_fail_1,
    shr_round_i128_usize_fail_2
);
test_shr_round_fail!(
    isize,
    u8,
    shr_round_assign_isize_u8_fail_1,
    shr_round_assign_isize_u8_fail_2,
    shr_round_isize_u8_fail_1,
    shr_round_isize_u8_fail_2
);
test_shr_round_fail!(
    isize,
    u16,
    shr_round_assign_isize_u16_fail_1,
    shr_round_assign_isize_u16_fail_2,
    shr_round_isize_u16_fail_1,
    shr_round_isize_u16_fail_2
);
test_shr_round_fail!(
    isize,
    u32,
    shr_round_assign_isize_u32_fail_1,
    shr_round_assign_isize_u32_fail_2,
    shr_round_isize_u32_fail_1,
    shr_round_isize_u32_fail_2
);
test_shr_round_fail!(
    isize,
    u64,
    shr_round_assign_isize_u64_fail_1,
    shr_round_assign_isize_u64_fail_2,
    shr_round_isize_u64_fail_1,
    shr_round_isize_u64_fail_2
);
test_shr_round_fail!(
    isize,
    u128,
    shr_round_assign_isize_u128_fail_1,
    shr_round_assign_isize_u128_fail_2,
    shr_round_isize_u128_fail_1,
    shr_round_isize_u128_fail_2
);
test_shr_round_fail!(
    isize,
    usize,
    shr_round_assign_isize_usize_fail_1,
    shr_round_assign_isize_usize_fail_2,
    shr_round_isize_usize_fail_1,
    shr_round_isize_usize_fail_2
);

test_shr_round_fail!(
    i8,
    i8,
    shr_round_assign_i8_i8_fail_1,
    shr_round_assign_i8_i8_fail_2,
    shr_round_i8_i8_fail_1,
    shr_round_i8_i8_fail_2
);
test_shr_round_fail!(
    i8,
    i16,
    shr_round_assign_i8_i16_fail_1,
    shr_round_assign_i8_i16_fail_2,
    shr_round_i8_i16_fail_1,
    shr_round_i8_i16_fail_2
);
test_shr_round_fail!(
    i8,
    i32,
    shr_round_assign_i8_i32_fail_1,
    shr_round_assign_i8_i32_fail_2,
    shr_round_i8_i32_fail_1,
    shr_round_i8_i32_fail_2
);
test_shr_round_fail!(
    i8,
    i64,
    shr_round_assign_i8_i64_fail_1,
    shr_round_assign_i8_i64_fail_2,
    shr_round_i8_i64_fail_1,
    shr_round_i8_i64_fail_2
);
test_shr_round_fail!(
    i8,
    i128,
    shr_round_assign_i8_i128_fail_1,
    shr_round_assign_i8_i128_fail_2,
    shr_round_i8_i128_fail_1,
    shr_round_i8_i128_fail_2
);
test_shr_round_fail!(
    i8,
    isize,
    shr_round_assign_i8_isize_fail_1,
    shr_round_assign_i8_isize_fail_2,
    shr_round_i8_isize_fail_1,
    shr_round_i8_isize_fail_2
);
test_shr_round_fail!(
    i16,
    i8,
    shr_round_assign_i16_i8_fail_1,
    shr_round_assign_i16_i8_fail_2,
    shr_round_i16_i8_fail_1,
    shr_round_i16_i8_fail_2
);
test_shr_round_fail!(
    i16,
    i16,
    shr_round_assign_i16_i16_fail_1,
    shr_round_assign_i16_i16_fail_2,
    shr_round_i16_i16_fail_1,
    shr_round_i16_i16_fail_2
);
test_shr_round_fail!(
    i16,
    i32,
    shr_round_assign_i16_i32_fail_1,
    shr_round_assign_i16_i32_fail_2,
    shr_round_i16_i32_fail_1,
    shr_round_i16_i32_fail_2
);
test_shr_round_fail!(
    i16,
    i64,
    shr_round_assign_i16_i64_fail_1,
    shr_round_assign_i16_i64_fail_2,
    shr_round_i16_i64_fail_1,
    shr_round_i16_i64_fail_2
);
test_shr_round_fail!(
    i16,
    i128,
    shr_round_assign_i16_i128_fail_1,
    shr_round_assign_i16_i128_fail_2,
    shr_round_i16_i128_fail_1,
    shr_round_i16_i128_fail_2
);
test_shr_round_fail!(
    i16,
    isize,
    shr_round_assign_i16_isize_fail_1,
    shr_round_assign_i16_isize_fail_2,
    shr_round_i16_isize_fail_1,
    shr_round_i16_isize_fail_2
);
test_shr_round_fail!(
    i32,
    i8,
    shr_round_assign_i32_i8_fail_1,
    shr_round_assign_i32_i8_fail_2,
    shr_round_i32_i8_fail_1,
    shr_round_i32_i8_fail_2
);
test_shr_round_fail!(
    i32,
    i16,
    shr_round_assign_i32_i16_fail_1,
    shr_round_assign_i32_i16_fail_2,
    shr_round_i32_i16_fail_1,
    shr_round_i32_i16_fail_2
);
test_shr_round_fail!(
    i32,
    i32,
    shr_round_assign_i32_i32_fail_1,
    shr_round_assign_i32_i32_fail_2,
    shr_round_i32_i32_fail_1,
    shr_round_i32_i32_fail_2
);
test_shr_round_fail!(
    i32,
    i64,
    shr_round_assign_i32_i64_fail_1,
    shr_round_assign_i32_i64_fail_2,
    shr_round_i32_i64_fail_1,
    shr_round_i32_i64_fail_2
);
test_shr_round_fail!(
    i32,
    i128,
    shr_round_assign_i32_i128_fail_1,
    shr_round_assign_i32_i128_fail_2,
    shr_round_i32_i128_fail_1,
    shr_round_i32_i128_fail_2
);
test_shr_round_fail!(
    i32,
    isize,
    shr_round_assign_i32_isize_fail_1,
    shr_round_assign_i32_isize_fail_2,
    shr_round_i32_isize_fail_1,
    shr_round_i32_isize_fail_2
);
test_shr_round_fail!(
    i64,
    i8,
    shr_round_assign_i64_i8_fail_1,
    shr_round_assign_i64_i8_fail_2,
    shr_round_i64_i8_fail_1,
    shr_round_i64_i8_fail_2
);
test_shr_round_fail!(
    i64,
    i16,
    shr_round_assign_i64_i16_fail_1,
    shr_round_assign_i64_i16_fail_2,
    shr_round_i64_i16_fail_1,
    shr_round_i64_i16_fail_2
);
test_shr_round_fail!(
    i64,
    i32,
    shr_round_assign_i64_i32_fail_1,
    shr_round_assign_i64_i32_fail_2,
    shr_round_i64_i32_fail_1,
    shr_round_i64_i32_fail_2
);
test_shr_round_fail!(
    i64,
    i64,
    shr_round_assign_i64_i64_fail_1,
    shr_round_assign_i64_i64_fail_2,
    shr_round_i64_i64_fail_1,
    shr_round_i64_i64_fail_2
);
test_shr_round_fail!(
    i64,
    i128,
    shr_round_assign_i64_i128_fail_1,
    shr_round_assign_i64_i128_fail_2,
    shr_round_i64_i128_fail_1,
    shr_round_i64_i128_fail_2
);
test_shr_round_fail!(
    i64,
    isize,
    shr_round_assign_i64_isize_fail_1,
    shr_round_assign_i64_isize_fail_2,
    shr_round_i64_isize_fail_1,
    shr_round_i64_isize_fail_2
);
test_shr_round_fail!(
    i128,
    i8,
    shr_round_assign_i128_i8_fail_1,
    shr_round_assign_i128_i8_fail_2,
    shr_round_i128_i8_fail_1,
    shr_round_i128_i8_fail_2
);
test_shr_round_fail!(
    i128,
    i16,
    shr_round_assign_i128_i16_fail_1,
    shr_round_assign_i128_i16_fail_2,
    shr_round_i128_i16_fail_1,
    shr_round_i128_i16_fail_2
);
test_shr_round_fail!(
    i128,
    i32,
    shr_round_assign_i128_i32_fail_1,
    shr_round_assign_i128_i32_fail_2,
    shr_round_i128_i32_fail_1,
    shr_round_i128_i32_fail_2
);
test_shr_round_fail!(
    i128,
    i64,
    shr_round_assign_i128_i64_fail_1,
    shr_round_assign_i128_i64_fail_2,
    shr_round_i128_i64_fail_1,
    shr_round_i128_i64_fail_2
);
test_shr_round_fail!(
    i128,
    i128,
    shr_round_assign_i128_i128_fail_1,
    shr_round_assign_i128_i128_fail_2,
    shr_round_i128_i128_fail_1,
    shr_round_i128_i128_fail_2
);
test_shr_round_fail!(
    i128,
    isize,
    shr_round_assign_i128_isize_fail_1,
    shr_round_assign_i128_isize_fail_2,
    shr_round_i128_isize_fail_1,
    shr_round_i128_isize_fail_2
);
test_shr_round_fail!(
    isize,
    i8,
    shr_round_assign_isize_i8_fail_1,
    shr_round_assign_isize_i8_fail_2,
    shr_round_isize_i8_fail_1,
    shr_round_isize_i8_fail_2
);
test_shr_round_fail!(
    isize,
    i16,
    shr_round_assign_isize_i16_fail_1,
    shr_round_assign_isize_i16_fail_2,
    shr_round_isize_i16_fail_1,
    shr_round_isize_i16_fail_2
);
test_shr_round_fail!(
    isize,
    i32,
    shr_round_assign_isize_i32_fail_1,
    shr_round_assign_isize_i32_fail_2,
    shr_round_isize_i32_fail_1,
    shr_round_isize_i32_fail_2
);
test_shr_round_fail!(
    isize,
    i64,
    shr_round_assign_isize_i64_fail_1,
    shr_round_assign_isize_i64_fail_2,
    shr_round_isize_i64_fail_1,
    shr_round_isize_i64_fail_2
);
test_shr_round_fail!(
    isize,
    i128,
    shr_round_assign_isize_i128_fail_1,
    shr_round_assign_isize_i128_fail_2,
    shr_round_isize_i128_fail_1,
    shr_round_isize_i128_fail_2
);
test_shr_round_fail!(
    isize,
    isize,
    shr_round_assign_isize_isize_fail_1,
    shr_round_assign_isize_isize_fail_2,
    shr_round_isize_isize_fail_1,
    shr_round_isize_isize_fail_2
);
