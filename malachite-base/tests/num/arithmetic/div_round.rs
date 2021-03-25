use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base_test_util::generators::{
    signed_pair_gen_var_3, signed_pair_gen_var_5, signed_rounding_mode_pair_gen,
    signed_rounding_mode_pair_gen_var_1, signed_rounding_mode_pair_gen_var_2,
    signed_rounding_mode_pair_gen_var_3, signed_signed_rounding_mode_triple_gen_var_1,
    unsigned_pair_gen_var_11, unsigned_pair_gen_var_12, unsigned_pair_gen_var_13,
    unsigned_rounding_mode_pair_gen, unsigned_rounding_mode_pair_gen_var_1,
    unsigned_unsigned_rounding_mode_triple_gen_var_1,
};
use std::panic::catch_unwind;

#[test]
fn test_div_round_unsigned() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, rm: RoundingMode, q: T) {
        assert_eq!(n.div_round(d, rm), q);

        let mut mut_n = n;
        mut_n.div_round_assign(d, rm);
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

    test::<u64>(1000000000000, 1, RoundingMode::Down, 1000000000000);
    test::<u64>(1000000000000, 1, RoundingMode::Floor, 1000000000000);
    test::<u64>(1000000000000, 1, RoundingMode::Up, 1000000000000);
    test::<u64>(1000000000000, 1, RoundingMode::Ceiling, 1000000000000);
    test::<u64>(1000000000000, 1, RoundingMode::Nearest, 1000000000000);
    test::<u64>(1000000000000, 1, RoundingMode::Exact, 1000000000000);

    test::<u64>(1000000000000, 3, RoundingMode::Down, 333333333333);
    test::<u64>(1000000000000, 3, RoundingMode::Floor, 333333333333);
    test::<u64>(1000000000000, 3, RoundingMode::Up, 333333333334);
    test::<u64>(1000000000000, 3, RoundingMode::Ceiling, 333333333334);
    test::<u64>(1000000000000, 3, RoundingMode::Nearest, 333333333333);

    test::<u64>(999999999999, 2, RoundingMode::Down, 499999999999);
    test::<u64>(999999999999, 2, RoundingMode::Floor, 499999999999);
    test::<u64>(999999999999, 2, RoundingMode::Up, 500000000000);
    test::<u64>(999999999999, 2, RoundingMode::Ceiling, 500000000000);
    test::<u64>(999999999999, 2, RoundingMode::Nearest, 500000000000);

    test::<u64>(1000000000001, 2, RoundingMode::Down, 500000000000);
    test::<u64>(1000000000001, 2, RoundingMode::Floor, 500000000000);
    test::<u64>(1000000000001, 2, RoundingMode::Up, 500000000001);
    test::<u64>(1000000000001, 2, RoundingMode::Ceiling, 500000000001);
    test::<u64>(1000000000001, 2, RoundingMode::Nearest, 500000000000);

    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Down,
        232830643708079,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Floor,
        232830643708079,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Up,
        232830643708080,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Ceiling,
        232830643708080,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Nearest,
        232830643708080,
    );

    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Down,
        1000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Floor,
        1000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Up,
        1000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Ceiling,
        1000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Nearest,
        1000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Exact,
        1000000000000,
    );

    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Down,
        999999999999,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Floor,
        999999999999,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Up,
        1000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Ceiling,
        1000000000000,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Nearest,
        999999999999,
    );

    test::<u128>(
        2999999999999999999999999,
        2000000000000000000000000,
        RoundingMode::Nearest,
        1,
    );
    test::<u128>(
        3000000000000000000000000,
        2000000000000000000000000,
        RoundingMode::Nearest,
        2,
    );
    test::<u128>(
        3000000000000000000000001,
        2000000000000000000000000,
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

    test::<i64>(1000000000000, 1, RoundingMode::Down, 1000000000000);
    test::<i64>(1000000000000, 1, RoundingMode::Floor, 1000000000000);
    test::<i64>(1000000000000, 1, RoundingMode::Up, 1000000000000);
    test::<i64>(1000000000000, 1, RoundingMode::Ceiling, 1000000000000);
    test::<i64>(1000000000000, 1, RoundingMode::Nearest, 1000000000000);
    test::<i64>(1000000000000, 1, RoundingMode::Exact, 1000000000000);

    test::<i64>(1000000000000, 3, RoundingMode::Down, 333333333333);
    test::<i64>(1000000000000, 3, RoundingMode::Floor, 333333333333);
    test::<i64>(1000000000000, 3, RoundingMode::Up, 333333333334);
    test::<i64>(1000000000000, 3, RoundingMode::Ceiling, 333333333334);
    test::<i64>(1000000000000, 3, RoundingMode::Nearest, 333333333333);

    test::<i64>(999999999999, 2, RoundingMode::Down, 499999999999);
    test::<i64>(999999999999, 2, RoundingMode::Floor, 499999999999);
    test::<i64>(999999999999, 2, RoundingMode::Up, 500000000000);
    test::<i64>(999999999999, 2, RoundingMode::Ceiling, 500000000000);
    test::<i64>(999999999999, 2, RoundingMode::Nearest, 500000000000);

    test::<i64>(1000000000001, 2, RoundingMode::Down, 500000000000);
    test::<i64>(1000000000001, 2, RoundingMode::Floor, 500000000000);
    test::<i64>(1000000000001, 2, RoundingMode::Up, 500000000001);
    test::<i64>(1000000000001, 2, RoundingMode::Ceiling, 500000000001);
    test::<i64>(1000000000001, 2, RoundingMode::Nearest, 500000000000);

    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Down,
        232830643708079,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Floor,
        232830643708079,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Up,
        232830643708080,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Ceiling,
        232830643708080,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Nearest,
        232830643708080,
    );

    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Down,
        1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Floor,
        1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Up,
        1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Ceiling,
        1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Nearest,
        1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Exact,
        1000000000000,
    );

    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Down,
        999999999999,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Floor,
        999999999999,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Up,
        1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Ceiling,
        1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Nearest,
        999999999999,
    );

    test::<i128>(
        2999999999999999999999999,
        2000000000000000000000000,
        RoundingMode::Nearest,
        1,
    );
    test::<i128>(
        3000000000000000000000000,
        2000000000000000000000000,
        RoundingMode::Nearest,
        2,
    );
    test::<i128>(
        3000000000000000000000001,
        2000000000000000000000000,
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

    test::<i64>(1000000000000, -1, RoundingMode::Down, -1000000000000);
    test::<i64>(1000000000000, -1, RoundingMode::Floor, -1000000000000);
    test::<i64>(1000000000000, -1, RoundingMode::Up, -1000000000000);
    test::<i64>(1000000000000, -1, RoundingMode::Ceiling, -1000000000000);
    test::<i64>(1000000000000, -1, RoundingMode::Nearest, -1000000000000);
    test::<i64>(1000000000000, -1, RoundingMode::Exact, -1000000000000);

    test::<i64>(1000000000000, -3, RoundingMode::Down, -333333333333);
    test::<i64>(1000000000000, -3, RoundingMode::Floor, -333333333334);
    test::<i64>(1000000000000, -3, RoundingMode::Up, -333333333334);
    test::<i64>(1000000000000, -3, RoundingMode::Ceiling, -333333333333);
    test::<i64>(1000000000000, -3, RoundingMode::Nearest, -333333333333);

    test::<i64>(999999999999, -2, RoundingMode::Down, -499999999999);
    test::<i64>(999999999999, -2, RoundingMode::Floor, -500000000000);
    test::<i64>(999999999999, -2, RoundingMode::Up, -500000000000);
    test::<i64>(999999999999, -2, RoundingMode::Ceiling, -499999999999);
    test::<i64>(999999999999, -2, RoundingMode::Nearest, -500000000000);

    test::<i64>(1000000000001, -2, RoundingMode::Down, -500000000000);
    test::<i64>(1000000000001, -2, RoundingMode::Floor, -500000000001);
    test::<i64>(1000000000001, -2, RoundingMode::Up, -500000000001);
    test::<i64>(1000000000001, -2, RoundingMode::Ceiling, -500000000000);
    test::<i64>(1000000000001, -2, RoundingMode::Nearest, -500000000000);

    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Down,
        -232830643708079,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Floor,
        -232830643708080,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Up,
        -232830643708080,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Ceiling,
        -232830643708079,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Nearest,
        -232830643708080,
    );

    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Down,
        -1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Floor,
        -1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Up,
        -1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Ceiling,
        -1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Nearest,
        -1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Exact,
        -1000000000000,
    );

    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Down,
        -999999999999,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Floor,
        -1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Up,
        -1000000000000,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Ceiling,
        -999999999999,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Nearest,
        -999999999999,
    );

    test::<i128>(
        2999999999999999999999999,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        -1,
    );
    test::<i128>(
        3000000000000000000000000,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        -2,
    );
    test::<i128>(
        3000000000000000000000001,
        -2000000000000000000000000,
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

    test::<i64>(-1000000000000, 1, RoundingMode::Down, -1000000000000);
    test::<i64>(-1000000000000, 1, RoundingMode::Floor, -1000000000000);
    test::<i64>(-1000000000000, 1, RoundingMode::Up, -1000000000000);
    test::<i64>(-1000000000000, 1, RoundingMode::Ceiling, -1000000000000);
    test::<i64>(-1000000000000, 1, RoundingMode::Nearest, -1000000000000);
    test::<i64>(-1000000000000, 1, RoundingMode::Exact, -1000000000000);

    test::<i64>(-1000000000000, 3, RoundingMode::Down, -333333333333);
    test::<i64>(-1000000000000, 3, RoundingMode::Floor, -333333333334);
    test::<i64>(-1000000000000, 3, RoundingMode::Up, -333333333334);
    test::<i64>(-1000000000000, 3, RoundingMode::Ceiling, -333333333333);
    test::<i64>(-1000000000000, 3, RoundingMode::Nearest, -333333333333);

    test::<i64>(-999999999999, 2, RoundingMode::Down, -499999999999);
    test::<i64>(-999999999999, 2, RoundingMode::Floor, -500000000000);
    test::<i64>(-999999999999, 2, RoundingMode::Up, -500000000000);
    test::<i64>(-999999999999, 2, RoundingMode::Ceiling, -499999999999);
    test::<i64>(-999999999999, 2, RoundingMode::Nearest, -500000000000);

    test::<i64>(-1000000000001, 2, RoundingMode::Down, -500000000000);
    test::<i64>(-1000000000001, 2, RoundingMode::Floor, -500000000001);
    test::<i64>(-1000000000001, 2, RoundingMode::Up, -500000000001);
    test::<i64>(-1000000000001, 2, RoundingMode::Ceiling, -500000000000);
    test::<i64>(-1000000000001, 2, RoundingMode::Nearest, -500000000000);

    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Down,
        -232830643708079,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Floor,
        -232830643708080,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Up,
        -232830643708080,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Ceiling,
        -232830643708079,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Nearest,
        -232830643708080,
    );

    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Down,
        -1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Floor,
        -1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Up,
        -1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Ceiling,
        -1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Nearest,
        -1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Exact,
        -1000000000000,
    );

    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Down,
        -999999999999,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Floor,
        -1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Up,
        -1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Ceiling,
        -999999999999,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Nearest,
        -999999999999,
    );

    test::<i128>(
        -2999999999999999999999999,
        2000000000000000000000000,
        RoundingMode::Nearest,
        -1,
    );
    test::<i128>(
        -3000000000000000000000000,
        2000000000000000000000000,
        RoundingMode::Nearest,
        -2,
    );
    test::<i128>(
        -3000000000000000000000001,
        2000000000000000000000000,
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

    test::<i64>(-1000000000000, -1, RoundingMode::Down, 1000000000000);
    test::<i64>(-1000000000000, -1, RoundingMode::Floor, 1000000000000);
    test::<i64>(-1000000000000, -1, RoundingMode::Up, 1000000000000);
    test::<i64>(-1000000000000, -1, RoundingMode::Ceiling, 1000000000000);
    test::<i64>(-1000000000000, -1, RoundingMode::Nearest, 1000000000000);
    test::<i64>(-1000000000000, -1, RoundingMode::Exact, 1000000000000);

    test::<i64>(-1000000000000, -3, RoundingMode::Down, 333333333333);
    test::<i64>(-1000000000000, -3, RoundingMode::Floor, 333333333333);
    test::<i64>(-1000000000000, -3, RoundingMode::Up, 333333333334);
    test::<i64>(-1000000000000, -3, RoundingMode::Ceiling, 333333333334);
    test::<i64>(-1000000000000, -3, RoundingMode::Nearest, 333333333333);

    test::<i64>(-999999999999, -2, RoundingMode::Down, 499999999999);
    test::<i64>(-999999999999, -2, RoundingMode::Floor, 499999999999);
    test::<i64>(-999999999999, -2, RoundingMode::Up, 500000000000);
    test::<i64>(-999999999999, -2, RoundingMode::Ceiling, 500000000000);
    test::<i64>(-999999999999, -2, RoundingMode::Nearest, 500000000000);

    test::<i64>(-1000000000001, -2, RoundingMode::Down, 500000000000);
    test::<i64>(-1000000000001, -2, RoundingMode::Floor, 500000000000);
    test::<i64>(-1000000000001, -2, RoundingMode::Up, 500000000001);
    test::<i64>(-1000000000001, -2, RoundingMode::Ceiling, 500000000001);
    test::<i64>(-1000000000001, -2, RoundingMode::Nearest, 500000000000);

    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Down,
        232830643708079,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Floor,
        232830643708079,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Up,
        232830643708080,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Ceiling,
        232830643708080,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Nearest,
        232830643708080,
    );

    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Down,
        1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Floor,
        1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Up,
        1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Ceiling,
        1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Nearest,
        1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Exact,
        1000000000000,
    );

    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Down,
        999999999999,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Floor,
        999999999999,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Up,
        1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Ceiling,
        1000000000000,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Nearest,
        999999999999,
    );

    test::<i128>(
        -2999999999999999999999999,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        1,
    );
    test::<i128>(
        -3000000000000000000000000,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        2,
    );
    test::<i128>(
        -3000000000000000000000001,
        -2000000000000000000000000,
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
    assert_panic!({
        let mut n = T::MIN;
        n.div_round_assign(T::NEGATIVE_ONE, RoundingMode::Floor);
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
        mut_x.div_round_assign(y, rm);
        let q = mut_x;

        assert_eq!(x.div_round(y, rm), q);
        assert!(q <= x);
    });

    unsigned_pair_gen_var_12::<T>().test_properties(|(x, y)| {
        assert_eq!(
            x.ceiling_div_neg_mod(y).0,
            x.div_round(y, RoundingMode::Ceiling)
        );
    });

    unsigned_pair_gen_var_11::<T>().test_properties(|(x, y)| {
        let q = x.div_exact(y);
        assert_eq!(x.div_round(y, RoundingMode::Down), q);
        assert_eq!(x.div_round(y, RoundingMode::Up), q);
        assert_eq!(x.div_round(y, RoundingMode::Floor), q);
        assert_eq!(x.div_round(y, RoundingMode::Ceiling), q);
        assert_eq!(x.div_round(y, RoundingMode::Nearest), q);
        assert_eq!(x.div_round(y, RoundingMode::Exact), q);
    });

    //TODO test using Rationals
    unsigned_pair_gen_var_13::<T>().test_properties(|(x, y)| {
        let down = x.div_round(y, RoundingMode::Down);
        let up = down + T::ONE;
        assert_eq!(x.div_round(y, RoundingMode::Up), up);
        assert_eq!(x.div_round(y, RoundingMode::Floor), down);
        assert_eq!(x.div_round(y, RoundingMode::Ceiling), up);
        let nearest = x.div_round(y, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(x, rm)| {
        assert_eq!(x.div_round(T::ONE, rm), x);
    });

    unsigned_rounding_mode_pair_gen_var_1::<T>().test_properties(|(x, rm)| {
        assert_eq!(T::ZERO.div_round(x, rm), T::ZERO);
        assert_eq!(x.div_round(x, rm), T::ONE);
    });
}

fn div_round_properties_helper_signed<T: PrimitiveSigned>() {
    signed_signed_rounding_mode_triple_gen_var_1::<T>().test_properties(|(x, y, rm)| {
        let mut mut_x = x;
        mut_x.div_round_assign(y, rm);
        let q = mut_x;

        assert_eq!(x.div_round(y, rm), q);

        assert!(q.le_abs(&x));
        if x != T::MIN {
            assert_eq!(-(-x).div_round(y, -rm), q);
        }
        if y != T::MIN && (x != T::MIN || (y != T::ONE && y != T::NEGATIVE_ONE)) {
            assert_eq!(-x.div_round(-y, -rm), q);
        }
    });

    signed_pair_gen_var_3::<T>().test_properties(|(x, y)| {
        let q = x.div_exact(y);
        assert_eq!(x.div_round(y, RoundingMode::Down), q);
        assert_eq!(x.div_round(y, RoundingMode::Up), q);
        assert_eq!(x.div_round(y, RoundingMode::Floor), q);
        assert_eq!(x.div_round(y, RoundingMode::Ceiling), q);
        assert_eq!(x.div_round(y, RoundingMode::Nearest), q);
        assert_eq!(x.div_round(y, RoundingMode::Exact), q);
    });

    //TODO test using Rationals
    signed_pair_gen_var_5::<T>().test_properties(|(x, y)| {
        let down = x.div_round(y, RoundingMode::Down);
        let up = if (x >= T::ZERO) == (y >= T::ZERO) {
            down + T::ONE
        } else {
            down - T::ONE
        };
        let floor = x.div_round(y, RoundingMode::Floor);
        let ceiling = floor + T::ONE;
        assert_eq!(x.div_round(y, RoundingMode::Up), up);
        assert_eq!(x.div_round(y, RoundingMode::Ceiling), ceiling);
        let nearest = x.div_round(y, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    signed_rounding_mode_pair_gen::<T>().test_properties(|(x, rm)| {
        assert_eq!(x.div_round(T::ONE, rm), x);
    });

    signed_rounding_mode_pair_gen_var_2::<T>().test_properties(|(x, rm)| {
        assert_eq!(x.div_round(T::NEGATIVE_ONE, rm), -x);
    });

    signed_rounding_mode_pair_gen_var_1::<T>().test_properties(|(x, rm)| {
        assert_eq!(T::ZERO.div_round(x, rm), T::ZERO);
        assert_eq!(x.div_round(x, rm), T::ONE);
    });

    signed_rounding_mode_pair_gen_var_3::<T>().test_properties(|(x, rm)| {
        assert_eq!(x.div_round(-x, rm), T::NEGATIVE_ONE);
        assert_eq!((-x).div_round(x, rm), T::NEGATIVE_ONE);
    });
}

#[test]
fn div_round_properties() {
    apply_fn_to_unsigneds!(div_round_properties_helper_unsigned);
    apply_fn_to_signeds!(div_round_properties_helper_signed);
}
