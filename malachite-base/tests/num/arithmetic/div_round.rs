use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::rounding_modes::RoundingMode;
use malachite_base::test_util::generators::{
    signed_pair_gen_var_3, signed_pair_gen_var_5, signed_rounding_mode_pair_gen,
    signed_rounding_mode_pair_gen_var_1, signed_rounding_mode_pair_gen_var_2,
    signed_rounding_mode_pair_gen_var_3, signed_signed_rounding_mode_triple_gen_var_1,
    unsigned_pair_gen_var_11, unsigned_pair_gen_var_12, unsigned_pair_gen_var_13,
    unsigned_rounding_mode_pair_gen, unsigned_rounding_mode_pair_gen_var_1,
    unsigned_unsigned_rounding_mode_triple_gen_var_1,
};
use std::cmp::Ordering;
use std::panic::catch_unwind;

#[test]
fn test_div_round_unsigned() {
    fn test<T: PrimitiveUnsigned>(n: T, d: T, rm: RoundingMode, q: T, o: Ordering) {
        assert_eq!(n.div_round(d, rm), (q, o));

        let mut mut_n = n;
        assert_eq!(mut_n.div_round_assign(d, rm), o);
        assert_eq!(mut_n, q);
    }
    test::<u8>(0, 1, RoundingMode::Down, 0, Ordering::Equal);
    test::<u8>(0, 1, RoundingMode::Floor, 0, Ordering::Equal);
    test::<u8>(0, 1, RoundingMode::Up, 0, Ordering::Equal);
    test::<u8>(0, 1, RoundingMode::Ceiling, 0, Ordering::Equal);
    test::<u8>(0, 1, RoundingMode::Nearest, 0, Ordering::Equal);
    test::<u8>(0, 1, RoundingMode::Exact, 0, Ordering::Equal);

    test::<u16>(0, 123, RoundingMode::Down, 0, Ordering::Equal);
    test::<u16>(0, 123, RoundingMode::Floor, 0, Ordering::Equal);
    test::<u16>(0, 123, RoundingMode::Up, 0, Ordering::Equal);
    test::<u16>(0, 123, RoundingMode::Ceiling, 0, Ordering::Equal);
    test::<u16>(0, 123, RoundingMode::Nearest, 0, Ordering::Equal);
    test::<u16>(0, 123, RoundingMode::Exact, 0, Ordering::Equal);

    test::<u32>(1, 1, RoundingMode::Down, 1, Ordering::Equal);
    test::<u32>(1, 1, RoundingMode::Floor, 1, Ordering::Equal);
    test::<u32>(1, 1, RoundingMode::Up, 1, Ordering::Equal);
    test::<u32>(1, 1, RoundingMode::Ceiling, 1, Ordering::Equal);
    test::<u32>(1, 1, RoundingMode::Nearest, 1, Ordering::Equal);
    test::<u32>(1, 1, RoundingMode::Exact, 1, Ordering::Equal);

    test::<u64>(123, 1, RoundingMode::Down, 123, Ordering::Equal);
    test::<u64>(123, 1, RoundingMode::Floor, 123, Ordering::Equal);
    test::<u64>(123, 1, RoundingMode::Up, 123, Ordering::Equal);
    test::<u64>(123, 1, RoundingMode::Ceiling, 123, Ordering::Equal);
    test::<u64>(123, 1, RoundingMode::Nearest, 123, Ordering::Equal);
    test::<u64>(123, 1, RoundingMode::Exact, 123, Ordering::Equal);

    test::<u128>(123, 2, RoundingMode::Down, 61, Ordering::Less);
    test::<u128>(123, 2, RoundingMode::Floor, 61, Ordering::Less);
    test::<u128>(123, 2, RoundingMode::Up, 62, Ordering::Greater);
    test::<u128>(123, 2, RoundingMode::Ceiling, 62, Ordering::Greater);
    test::<u128>(123, 2, RoundingMode::Nearest, 62, Ordering::Greater);

    test::<usize>(125, 2, RoundingMode::Down, 62, Ordering::Less);
    test::<usize>(125, 2, RoundingMode::Floor, 62, Ordering::Less);
    test::<usize>(125, 2, RoundingMode::Up, 63, Ordering::Greater);
    test::<usize>(125, 2, RoundingMode::Ceiling, 63, Ordering::Greater);
    test::<usize>(125, 2, RoundingMode::Nearest, 62, Ordering::Less);

    test::<u8>(123, 123, RoundingMode::Down, 1, Ordering::Equal);
    test::<u8>(123, 123, RoundingMode::Floor, 1, Ordering::Equal);
    test::<u8>(123, 123, RoundingMode::Up, 1, Ordering::Equal);
    test::<u8>(123, 123, RoundingMode::Ceiling, 1, Ordering::Equal);
    test::<u8>(123, 123, RoundingMode::Nearest, 1, Ordering::Equal);
    test::<u8>(123, 123, RoundingMode::Exact, 1, Ordering::Equal);

    test::<u16>(123, 456, RoundingMode::Down, 0, Ordering::Less);
    test::<u16>(123, 456, RoundingMode::Floor, 0, Ordering::Less);
    test::<u16>(123, 456, RoundingMode::Up, 1, Ordering::Greater);
    test::<u16>(123, 456, RoundingMode::Ceiling, 1, Ordering::Greater);
    test::<u16>(123, 456, RoundingMode::Nearest, 0, Ordering::Less);

    test::<u64>(
        1000000000000,
        1,
        RoundingMode::Down,
        1000000000000,
        Ordering::Equal,
    );
    test::<u64>(
        1000000000000,
        1,
        RoundingMode::Floor,
        1000000000000,
        Ordering::Equal,
    );
    test::<u64>(
        1000000000000,
        1,
        RoundingMode::Up,
        1000000000000,
        Ordering::Equal,
    );
    test::<u64>(
        1000000000000,
        1,
        RoundingMode::Ceiling,
        1000000000000,
        Ordering::Equal,
    );
    test::<u64>(
        1000000000000,
        1,
        RoundingMode::Nearest,
        1000000000000,
        Ordering::Equal,
    );
    test::<u64>(
        1000000000000,
        1,
        RoundingMode::Exact,
        1000000000000,
        Ordering::Equal,
    );

    test::<u64>(
        1000000000000,
        3,
        RoundingMode::Down,
        333333333333,
        Ordering::Less,
    );
    test::<u64>(
        1000000000000,
        3,
        RoundingMode::Floor,
        333333333333,
        Ordering::Less,
    );
    test::<u64>(
        1000000000000,
        3,
        RoundingMode::Up,
        333333333334,
        Ordering::Greater,
    );
    test::<u64>(
        1000000000000,
        3,
        RoundingMode::Ceiling,
        333333333334,
        Ordering::Greater,
    );
    test::<u64>(
        1000000000000,
        3,
        RoundingMode::Nearest,
        333333333333,
        Ordering::Less,
    );

    test::<u64>(
        999999999999,
        2,
        RoundingMode::Down,
        499999999999,
        Ordering::Less,
    );
    test::<u64>(
        999999999999,
        2,
        RoundingMode::Floor,
        499999999999,
        Ordering::Less,
    );
    test::<u64>(
        999999999999,
        2,
        RoundingMode::Up,
        500000000000,
        Ordering::Greater,
    );
    test::<u64>(
        999999999999,
        2,
        RoundingMode::Ceiling,
        500000000000,
        Ordering::Greater,
    );
    test::<u64>(
        999999999999,
        2,
        RoundingMode::Nearest,
        500000000000,
        Ordering::Greater,
    );

    test::<u64>(
        1000000000001,
        2,
        RoundingMode::Down,
        500000000000,
        Ordering::Less,
    );
    test::<u64>(
        1000000000001,
        2,
        RoundingMode::Floor,
        500000000000,
        Ordering::Less,
    );
    test::<u64>(
        1000000000001,
        2,
        RoundingMode::Up,
        500000000001,
        Ordering::Greater,
    );
    test::<u64>(
        1000000000001,
        2,
        RoundingMode::Ceiling,
        500000000001,
        Ordering::Greater,
    );
    test::<u64>(
        1000000000001,
        2,
        RoundingMode::Nearest,
        500000000000,
        Ordering::Less,
    );

    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Down,
        232830643708079,
        Ordering::Less,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Floor,
        232830643708079,
        Ordering::Less,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Up,
        232830643708080,
        Ordering::Greater,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Ceiling,
        232830643708080,
        Ordering::Greater,
    );
    test::<u128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Nearest,
        232830643708080,
        Ordering::Greater,
    );

    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Down,
        1000000000000,
        Ordering::Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Floor,
        1000000000000,
        Ordering::Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Up,
        1000000000000,
        Ordering::Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Ceiling,
        1000000000000,
        Ordering::Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Nearest,
        1000000000000,
        Ordering::Equal,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Exact,
        1000000000000,
        Ordering::Equal,
    );

    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Down,
        999999999999,
        Ordering::Less,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Floor,
        999999999999,
        Ordering::Less,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Up,
        1000000000000,
        Ordering::Greater,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Ceiling,
        1000000000000,
        Ordering::Greater,
    );
    test::<u128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Nearest,
        999999999999,
        Ordering::Less,
    );

    test::<u128>(
        2999999999999999999999999,
        2000000000000000000000000,
        RoundingMode::Nearest,
        1,
        Ordering::Less,
    );
    test::<u128>(
        3000000000000000000000000,
        2000000000000000000000000,
        RoundingMode::Nearest,
        2,
        Ordering::Greater,
    );
    test::<u128>(
        3000000000000000000000001,
        2000000000000000000000000,
        RoundingMode::Nearest,
        2,
        Ordering::Greater,
    );
}

#[test]
fn test_div_round_signed() {
    fn test<T: PrimitiveSigned>(n: T, d: T, rm: RoundingMode, q: T, o: Ordering) {
        assert_eq!(n.div_round(d, rm), (q, o));

        let mut mut_n = n;
        assert_eq!(mut_n.div_round_assign(d, rm), o);
        assert_eq!(mut_n, q);
    }
    test::<i8>(0, 1, RoundingMode::Down, 0, Ordering::Equal);
    test::<i8>(0, 1, RoundingMode::Floor, 0, Ordering::Equal);
    test::<i8>(0, 1, RoundingMode::Up, 0, Ordering::Equal);
    test::<i8>(0, 1, RoundingMode::Ceiling, 0, Ordering::Equal);
    test::<i8>(0, 1, RoundingMode::Nearest, 0, Ordering::Equal);
    test::<i8>(0, 1, RoundingMode::Exact, 0, Ordering::Equal);

    test::<i16>(0, 123, RoundingMode::Down, 0, Ordering::Equal);
    test::<i16>(0, 123, RoundingMode::Floor, 0, Ordering::Equal);
    test::<i16>(0, 123, RoundingMode::Up, 0, Ordering::Equal);
    test::<i16>(0, 123, RoundingMode::Ceiling, 0, Ordering::Equal);
    test::<i16>(0, 123, RoundingMode::Nearest, 0, Ordering::Equal);
    test::<i16>(0, 123, RoundingMode::Exact, 0, Ordering::Equal);

    test::<i32>(1, 1, RoundingMode::Down, 1, Ordering::Equal);
    test::<i32>(1, 1, RoundingMode::Floor, 1, Ordering::Equal);
    test::<i32>(1, 1, RoundingMode::Up, 1, Ordering::Equal);
    test::<i32>(1, 1, RoundingMode::Ceiling, 1, Ordering::Equal);
    test::<i32>(1, 1, RoundingMode::Nearest, 1, Ordering::Equal);
    test::<i32>(1, 1, RoundingMode::Exact, 1, Ordering::Equal);

    test::<i64>(123, 1, RoundingMode::Down, 123, Ordering::Equal);
    test::<i64>(123, 1, RoundingMode::Floor, 123, Ordering::Equal);
    test::<i64>(123, 1, RoundingMode::Up, 123, Ordering::Equal);
    test::<i64>(123, 1, RoundingMode::Ceiling, 123, Ordering::Equal);
    test::<i64>(123, 1, RoundingMode::Nearest, 123, Ordering::Equal);
    test::<i64>(123, 1, RoundingMode::Exact, 123, Ordering::Equal);

    test::<i128>(123, 2, RoundingMode::Down, 61, Ordering::Less);
    test::<i128>(123, 2, RoundingMode::Floor, 61, Ordering::Less);
    test::<i128>(123, 2, RoundingMode::Up, 62, Ordering::Greater);
    test::<i128>(123, 2, RoundingMode::Ceiling, 62, Ordering::Greater);
    test::<i128>(123, 2, RoundingMode::Nearest, 62, Ordering::Greater);

    test::<isize>(125, 2, RoundingMode::Down, 62, Ordering::Less);
    test::<isize>(125, 2, RoundingMode::Floor, 62, Ordering::Less);
    test::<isize>(125, 2, RoundingMode::Up, 63, Ordering::Greater);
    test::<isize>(125, 2, RoundingMode::Ceiling, 63, Ordering::Greater);
    test::<isize>(125, 2, RoundingMode::Nearest, 62, Ordering::Less);

    test::<i8>(123, 123, RoundingMode::Down, 1, Ordering::Equal);
    test::<i8>(123, 123, RoundingMode::Floor, 1, Ordering::Equal);
    test::<i8>(123, 123, RoundingMode::Up, 1, Ordering::Equal);
    test::<i8>(123, 123, RoundingMode::Ceiling, 1, Ordering::Equal);
    test::<i8>(123, 123, RoundingMode::Nearest, 1, Ordering::Equal);
    test::<i8>(123, 123, RoundingMode::Exact, 1, Ordering::Equal);

    test::<i16>(123, 456, RoundingMode::Down, 0, Ordering::Less);
    test::<i16>(123, 456, RoundingMode::Floor, 0, Ordering::Less);
    test::<i16>(123, 456, RoundingMode::Up, 1, Ordering::Greater);
    test::<i16>(123, 456, RoundingMode::Ceiling, 1, Ordering::Greater);
    test::<i16>(123, 456, RoundingMode::Nearest, 0, Ordering::Less);

    test::<i64>(
        1000000000000,
        1,
        RoundingMode::Down,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        1000000000000,
        1,
        RoundingMode::Floor,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        1000000000000,
        1,
        RoundingMode::Up,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        1000000000000,
        1,
        RoundingMode::Ceiling,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        1000000000000,
        1,
        RoundingMode::Nearest,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        1000000000000,
        1,
        RoundingMode::Exact,
        1000000000000,
        Ordering::Equal,
    );

    test::<i64>(
        1000000000000,
        3,
        RoundingMode::Down,
        333333333333,
        Ordering::Less,
    );
    test::<i64>(
        1000000000000,
        3,
        RoundingMode::Floor,
        333333333333,
        Ordering::Less,
    );
    test::<i64>(
        1000000000000,
        3,
        RoundingMode::Up,
        333333333334,
        Ordering::Greater,
    );
    test::<i64>(
        1000000000000,
        3,
        RoundingMode::Ceiling,
        333333333334,
        Ordering::Greater,
    );
    test::<i64>(
        1000000000000,
        3,
        RoundingMode::Nearest,
        333333333333,
        Ordering::Less,
    );

    test::<i64>(
        999999999999,
        2,
        RoundingMode::Down,
        499999999999,
        Ordering::Less,
    );
    test::<i64>(
        999999999999,
        2,
        RoundingMode::Floor,
        499999999999,
        Ordering::Less,
    );
    test::<i64>(
        999999999999,
        2,
        RoundingMode::Up,
        500000000000,
        Ordering::Greater,
    );
    test::<i64>(
        999999999999,
        2,
        RoundingMode::Ceiling,
        500000000000,
        Ordering::Greater,
    );
    test::<i64>(
        999999999999,
        2,
        RoundingMode::Nearest,
        500000000000,
        Ordering::Greater,
    );

    test::<i64>(
        1000000000001,
        2,
        RoundingMode::Down,
        500000000000,
        Ordering::Less,
    );
    test::<i64>(
        1000000000001,
        2,
        RoundingMode::Floor,
        500000000000,
        Ordering::Less,
    );
    test::<i64>(
        1000000000001,
        2,
        RoundingMode::Up,
        500000000001,
        Ordering::Greater,
    );
    test::<i64>(
        1000000000001,
        2,
        RoundingMode::Ceiling,
        500000000001,
        Ordering::Greater,
    );
    test::<i64>(
        1000000000001,
        2,
        RoundingMode::Nearest,
        500000000000,
        Ordering::Less,
    );

    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Down,
        232830643708079,
        Ordering::Less,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Floor,
        232830643708079,
        Ordering::Less,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Up,
        232830643708080,
        Ordering::Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Ceiling,
        232830643708080,
        Ordering::Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        0xffffffff,
        RoundingMode::Nearest,
        232830643708080,
        Ordering::Greater,
    );

    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Down,
        1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Floor,
        1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Up,
        1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Ceiling,
        1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Nearest,
        1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000000,
        RoundingMode::Exact,
        1000000000000,
        Ordering::Equal,
    );

    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Down,
        999999999999,
        Ordering::Less,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Floor,
        999999999999,
        Ordering::Less,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Up,
        1000000000000,
        Ordering::Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Ceiling,
        1000000000000,
        Ordering::Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        1000000000001,
        RoundingMode::Nearest,
        999999999999,
        Ordering::Less,
    );

    test::<i128>(
        2999999999999999999999999,
        2000000000000000000000000,
        RoundingMode::Nearest,
        1,
        Ordering::Less,
    );
    test::<i128>(
        3000000000000000000000000,
        2000000000000000000000000,
        RoundingMode::Nearest,
        2,
        Ordering::Greater,
    );
    test::<i128>(
        3000000000000000000000001,
        2000000000000000000000000,
        RoundingMode::Nearest,
        2,
        Ordering::Greater,
    );

    test::<i8>(0, -1, RoundingMode::Down, 0, Ordering::Equal);
    test::<i8>(0, -1, RoundingMode::Floor, 0, Ordering::Equal);
    test::<i8>(0, -1, RoundingMode::Up, 0, Ordering::Equal);
    test::<i8>(0, -1, RoundingMode::Ceiling, 0, Ordering::Equal);
    test::<i8>(0, -1, RoundingMode::Nearest, 0, Ordering::Equal);
    test::<i8>(0, -1, RoundingMode::Exact, 0, Ordering::Equal);

    test::<i16>(0, -123, RoundingMode::Down, 0, Ordering::Equal);
    test::<i16>(0, -123, RoundingMode::Floor, 0, Ordering::Equal);
    test::<i16>(0, -123, RoundingMode::Up, 0, Ordering::Equal);
    test::<i16>(0, -123, RoundingMode::Ceiling, 0, Ordering::Equal);
    test::<i16>(0, -123, RoundingMode::Nearest, 0, Ordering::Equal);
    test::<i16>(0, -123, RoundingMode::Exact, 0, Ordering::Equal);

    test::<i32>(1, -1, RoundingMode::Down, -1, Ordering::Equal);
    test::<i32>(1, -1, RoundingMode::Floor, -1, Ordering::Equal);
    test::<i32>(1, -1, RoundingMode::Up, -1, Ordering::Equal);
    test::<i32>(1, -1, RoundingMode::Ceiling, -1, Ordering::Equal);
    test::<i32>(1, -1, RoundingMode::Nearest, -1, Ordering::Equal);
    test::<i32>(1, -1, RoundingMode::Exact, -1, Ordering::Equal);

    test::<i64>(123, -1, RoundingMode::Down, -123, Ordering::Equal);
    test::<i64>(123, -1, RoundingMode::Floor, -123, Ordering::Equal);
    test::<i64>(123, -1, RoundingMode::Up, -123, Ordering::Equal);
    test::<i64>(123, -1, RoundingMode::Ceiling, -123, Ordering::Equal);
    test::<i64>(123, -1, RoundingMode::Nearest, -123, Ordering::Equal);
    test::<i64>(123, -1, RoundingMode::Exact, -123, Ordering::Equal);

    test::<i128>(123, -2, RoundingMode::Down, -61, Ordering::Greater);
    test::<i128>(123, -2, RoundingMode::Floor, -62, Ordering::Less);
    test::<i128>(123, -2, RoundingMode::Up, -62, Ordering::Less);
    test::<i128>(123, -2, RoundingMode::Ceiling, -61, Ordering::Greater);
    test::<i128>(123, -2, RoundingMode::Nearest, -62, Ordering::Less);

    test::<isize>(125, -2, RoundingMode::Down, -62, Ordering::Greater);
    test::<isize>(125, -2, RoundingMode::Floor, -63, Ordering::Less);
    test::<isize>(125, -2, RoundingMode::Up, -63, Ordering::Less);
    test::<isize>(125, -2, RoundingMode::Ceiling, -62, Ordering::Greater);
    test::<isize>(125, -2, RoundingMode::Nearest, -62, Ordering::Greater);

    test::<i8>(123, -123, RoundingMode::Down, -1, Ordering::Equal);
    test::<i8>(123, -123, RoundingMode::Floor, -1, Ordering::Equal);
    test::<i8>(123, -123, RoundingMode::Up, -1, Ordering::Equal);
    test::<i8>(123, -123, RoundingMode::Ceiling, -1, Ordering::Equal);
    test::<i8>(123, -123, RoundingMode::Nearest, -1, Ordering::Equal);
    test::<i8>(123, -123, RoundingMode::Exact, -1, Ordering::Equal);

    test::<i16>(123, -456, RoundingMode::Down, 0, Ordering::Greater);
    test::<i16>(123, -456, RoundingMode::Floor, -1, Ordering::Less);
    test::<i16>(123, -456, RoundingMode::Up, -1, Ordering::Less);
    test::<i16>(123, -456, RoundingMode::Ceiling, 0, Ordering::Greater);
    test::<i16>(123, -456, RoundingMode::Nearest, 0, Ordering::Greater);

    test::<i64>(
        1000000000000,
        -1,
        RoundingMode::Down,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        1000000000000,
        -1,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        1000000000000,
        -1,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        1000000000000,
        -1,
        RoundingMode::Ceiling,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        1000000000000,
        -1,
        RoundingMode::Nearest,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        1000000000000,
        -1,
        RoundingMode::Exact,
        -1000000000000,
        Ordering::Equal,
    );

    test::<i64>(
        1000000000000,
        -3,
        RoundingMode::Down,
        -333333333333,
        Ordering::Greater,
    );
    test::<i64>(
        1000000000000,
        -3,
        RoundingMode::Floor,
        -333333333334,
        Ordering::Less,
    );
    test::<i64>(
        1000000000000,
        -3,
        RoundingMode::Up,
        -333333333334,
        Ordering::Less,
    );
    test::<i64>(
        1000000000000,
        -3,
        RoundingMode::Ceiling,
        -333333333333,
        Ordering::Greater,
    );
    test::<i64>(
        1000000000000,
        -3,
        RoundingMode::Nearest,
        -333333333333,
        Ordering::Greater,
    );

    test::<i64>(
        999999999999,
        -2,
        RoundingMode::Down,
        -499999999999,
        Ordering::Greater,
    );
    test::<i64>(
        999999999999,
        -2,
        RoundingMode::Floor,
        -500000000000,
        Ordering::Less,
    );
    test::<i64>(
        999999999999,
        -2,
        RoundingMode::Up,
        -500000000000,
        Ordering::Less,
    );
    test::<i64>(
        999999999999,
        -2,
        RoundingMode::Ceiling,
        -499999999999,
        Ordering::Greater,
    );
    test::<i64>(
        999999999999,
        -2,
        RoundingMode::Nearest,
        -500000000000,
        Ordering::Less,
    );

    test::<i64>(
        1000000000001,
        -2,
        RoundingMode::Down,
        -500000000000,
        Ordering::Greater,
    );
    test::<i64>(
        1000000000001,
        -2,
        RoundingMode::Floor,
        -500000000001,
        Ordering::Less,
    );
    test::<i64>(
        1000000000001,
        -2,
        RoundingMode::Up,
        -500000000001,
        Ordering::Less,
    );
    test::<i64>(
        1000000000001,
        -2,
        RoundingMode::Ceiling,
        -500000000000,
        Ordering::Greater,
    );
    test::<i64>(
        1000000000001,
        -2,
        RoundingMode::Nearest,
        -500000000000,
        Ordering::Greater,
    );

    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Down,
        -232830643708079,
        Ordering::Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Floor,
        -232830643708080,
        Ordering::Less,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Up,
        -232830643708080,
        Ordering::Less,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Ceiling,
        -232830643708079,
        Ordering::Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Nearest,
        -232830643708080,
        Ordering::Less,
    );

    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Down,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Ceiling,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Nearest,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000000,
        RoundingMode::Exact,
        -1000000000000,
        Ordering::Equal,
    );

    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Down,
        -999999999999,
        Ordering::Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Less,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Less,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Ceiling,
        -999999999999,
        Ordering::Greater,
    );
    test::<i128>(
        1000000000000000000000000,
        -1000000000001,
        RoundingMode::Nearest,
        -999999999999,
        Ordering::Greater,
    );

    test::<i128>(
        2999999999999999999999999,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        -1,
        Ordering::Greater,
    );
    test::<i128>(
        3000000000000000000000000,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        -2,
        Ordering::Less,
    );
    test::<i128>(
        3000000000000000000000001,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        -2,
        Ordering::Less,
    );

    test::<i8>(-1, 1, RoundingMode::Down, -1, Ordering::Equal);
    test::<i8>(-1, 1, RoundingMode::Floor, -1, Ordering::Equal);
    test::<i8>(-1, 1, RoundingMode::Up, -1, Ordering::Equal);
    test::<i8>(-1, 1, RoundingMode::Ceiling, -1, Ordering::Equal);
    test::<i8>(-1, 1, RoundingMode::Nearest, -1, Ordering::Equal);
    test::<i8>(-1, 1, RoundingMode::Exact, -1, Ordering::Equal);

    test::<i16>(-123, 1, RoundingMode::Down, -123, Ordering::Equal);
    test::<i16>(-123, 1, RoundingMode::Floor, -123, Ordering::Equal);
    test::<i16>(-123, 1, RoundingMode::Up, -123, Ordering::Equal);
    test::<i16>(-123, 1, RoundingMode::Ceiling, -123, Ordering::Equal);
    test::<i16>(-123, 1, RoundingMode::Nearest, -123, Ordering::Equal);
    test::<i16>(-123, 1, RoundingMode::Exact, -123, Ordering::Equal);

    test::<i32>(-123, 2, RoundingMode::Down, -61, Ordering::Greater);
    test::<i32>(-123, 2, RoundingMode::Floor, -62, Ordering::Less);
    test::<i32>(-123, 2, RoundingMode::Up, -62, Ordering::Less);
    test::<i32>(-123, 2, RoundingMode::Ceiling, -61, Ordering::Greater);
    test::<i32>(-123, 2, RoundingMode::Nearest, -62, Ordering::Less);

    test::<i64>(-125, 2, RoundingMode::Down, -62, Ordering::Greater);
    test::<i64>(-125, 2, RoundingMode::Floor, -63, Ordering::Less);
    test::<i64>(-125, 2, RoundingMode::Up, -63, Ordering::Less);
    test::<i64>(-125, 2, RoundingMode::Ceiling, -62, Ordering::Greater);
    test::<i64>(-125, 2, RoundingMode::Nearest, -62, Ordering::Greater);

    test::<i128>(-123, 123, RoundingMode::Down, -1, Ordering::Equal);
    test::<i128>(-123, 123, RoundingMode::Floor, -1, Ordering::Equal);
    test::<i128>(-123, 123, RoundingMode::Up, -1, Ordering::Equal);
    test::<i128>(-123, 123, RoundingMode::Ceiling, -1, Ordering::Equal);
    test::<i128>(-123, 123, RoundingMode::Nearest, -1, Ordering::Equal);
    test::<i128>(-123, 123, RoundingMode::Exact, -1, Ordering::Equal);

    test::<isize>(-123, 456, RoundingMode::Down, 0, Ordering::Greater);
    test::<isize>(-123, 456, RoundingMode::Floor, -1, Ordering::Less);
    test::<isize>(-123, 456, RoundingMode::Up, -1, Ordering::Less);
    test::<isize>(-123, 456, RoundingMode::Ceiling, 0, Ordering::Greater);
    test::<isize>(-123, 456, RoundingMode::Nearest, 0, Ordering::Greater);

    test::<i64>(
        -1000000000000,
        1,
        RoundingMode::Down,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        -1000000000000,
        1,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        -1000000000000,
        1,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        -1000000000000,
        1,
        RoundingMode::Ceiling,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        -1000000000000,
        1,
        RoundingMode::Nearest,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        -1000000000000,
        1,
        RoundingMode::Exact,
        -1000000000000,
        Ordering::Equal,
    );

    test::<i64>(
        -1000000000000,
        3,
        RoundingMode::Down,
        -333333333333,
        Ordering::Greater,
    );
    test::<i64>(
        -1000000000000,
        3,
        RoundingMode::Floor,
        -333333333334,
        Ordering::Less,
    );
    test::<i64>(
        -1000000000000,
        3,
        RoundingMode::Up,
        -333333333334,
        Ordering::Less,
    );
    test::<i64>(
        -1000000000000,
        3,
        RoundingMode::Ceiling,
        -333333333333,
        Ordering::Greater,
    );
    test::<i64>(
        -1000000000000,
        3,
        RoundingMode::Nearest,
        -333333333333,
        Ordering::Greater,
    );

    test::<i64>(
        -999999999999,
        2,
        RoundingMode::Down,
        -499999999999,
        Ordering::Greater,
    );
    test::<i64>(
        -999999999999,
        2,
        RoundingMode::Floor,
        -500000000000,
        Ordering::Less,
    );
    test::<i64>(
        -999999999999,
        2,
        RoundingMode::Up,
        -500000000000,
        Ordering::Less,
    );
    test::<i64>(
        -999999999999,
        2,
        RoundingMode::Ceiling,
        -499999999999,
        Ordering::Greater,
    );
    test::<i64>(
        -999999999999,
        2,
        RoundingMode::Nearest,
        -500000000000,
        Ordering::Less,
    );

    test::<i64>(
        -1000000000001,
        2,
        RoundingMode::Down,
        -500000000000,
        Ordering::Greater,
    );
    test::<i64>(
        -1000000000001,
        2,
        RoundingMode::Floor,
        -500000000001,
        Ordering::Less,
    );
    test::<i64>(
        -1000000000001,
        2,
        RoundingMode::Up,
        -500000000001,
        Ordering::Less,
    );
    test::<i64>(
        -1000000000001,
        2,
        RoundingMode::Ceiling,
        -500000000000,
        Ordering::Greater,
    );
    test::<i64>(
        -1000000000001,
        2,
        RoundingMode::Nearest,
        -500000000000,
        Ordering::Greater,
    );

    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Down,
        -232830643708079,
        Ordering::Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Floor,
        -232830643708080,
        Ordering::Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Up,
        -232830643708080,
        Ordering::Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Ceiling,
        -232830643708079,
        Ordering::Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        0xffffffff,
        RoundingMode::Nearest,
        -232830643708080,
        Ordering::Less,
    );

    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Down,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Ceiling,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Nearest,
        -1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000000,
        RoundingMode::Exact,
        -1000000000000,
        Ordering::Equal,
    );

    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Down,
        -999999999999,
        Ordering::Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Floor,
        -1000000000000,
        Ordering::Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Up,
        -1000000000000,
        Ordering::Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Ceiling,
        -999999999999,
        Ordering::Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        1000000000001,
        RoundingMode::Nearest,
        -999999999999,
        Ordering::Greater,
    );

    test::<i128>(
        -2999999999999999999999999,
        2000000000000000000000000,
        RoundingMode::Nearest,
        -1,
        Ordering::Greater,
    );
    test::<i128>(
        -3000000000000000000000000,
        2000000000000000000000000,
        RoundingMode::Nearest,
        -2,
        Ordering::Less,
    );
    test::<i128>(
        -3000000000000000000000001,
        2000000000000000000000000,
        RoundingMode::Nearest,
        -2,
        Ordering::Less,
    );

    test::<i8>(-1, -1, RoundingMode::Down, 1, Ordering::Equal);
    test::<i8>(-1, -1, RoundingMode::Floor, 1, Ordering::Equal);
    test::<i8>(-1, -1, RoundingMode::Up, 1, Ordering::Equal);
    test::<i8>(-1, -1, RoundingMode::Ceiling, 1, Ordering::Equal);
    test::<i8>(-1, -1, RoundingMode::Nearest, 1, Ordering::Equal);
    test::<i8>(-1, -1, RoundingMode::Exact, 1, Ordering::Equal);

    test::<i16>(-123, -1, RoundingMode::Down, 123, Ordering::Equal);
    test::<i16>(-123, -1, RoundingMode::Floor, 123, Ordering::Equal);
    test::<i16>(-123, -1, RoundingMode::Up, 123, Ordering::Equal);
    test::<i16>(-123, -1, RoundingMode::Ceiling, 123, Ordering::Equal);
    test::<i16>(-123, -1, RoundingMode::Nearest, 123, Ordering::Equal);
    test::<i16>(-123, -1, RoundingMode::Exact, 123, Ordering::Equal);

    test::<i32>(-123, -2, RoundingMode::Down, 61, Ordering::Less);
    test::<i32>(-123, -2, RoundingMode::Floor, 61, Ordering::Less);
    test::<i32>(-123, -2, RoundingMode::Up, 62, Ordering::Greater);
    test::<i32>(-123, -2, RoundingMode::Ceiling, 62, Ordering::Greater);
    test::<i32>(-123, -2, RoundingMode::Nearest, 62, Ordering::Greater);

    test::<i64>(-125, -2, RoundingMode::Down, 62, Ordering::Less);
    test::<i64>(-125, -2, RoundingMode::Floor, 62, Ordering::Less);
    test::<i64>(-125, -2, RoundingMode::Up, 63, Ordering::Greater);
    test::<i64>(-125, -2, RoundingMode::Ceiling, 63, Ordering::Greater);
    test::<i64>(-125, -2, RoundingMode::Nearest, 62, Ordering::Less);

    test::<i128>(-123, -123, RoundingMode::Down, 1, Ordering::Equal);
    test::<i128>(-123, -123, RoundingMode::Floor, 1, Ordering::Equal);
    test::<i128>(-123, -123, RoundingMode::Up, 1, Ordering::Equal);
    test::<i128>(-123, -123, RoundingMode::Ceiling, 1, Ordering::Equal);
    test::<i128>(-123, -123, RoundingMode::Nearest, 1, Ordering::Equal);
    test::<i128>(-123, -123, RoundingMode::Exact, 1, Ordering::Equal);

    test::<isize>(-123, -456, RoundingMode::Down, 0, Ordering::Less);
    test::<isize>(-123, -456, RoundingMode::Floor, 0, Ordering::Less);
    test::<isize>(-123, -456, RoundingMode::Up, 1, Ordering::Greater);
    test::<isize>(-123, -456, RoundingMode::Ceiling, 1, Ordering::Greater);
    test::<isize>(-123, -456, RoundingMode::Nearest, 0, Ordering::Less);

    test::<i64>(
        -1000000000000,
        -1,
        RoundingMode::Down,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        -1000000000000,
        -1,
        RoundingMode::Floor,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        -1000000000000,
        -1,
        RoundingMode::Up,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        -1000000000000,
        -1,
        RoundingMode::Ceiling,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        -1000000000000,
        -1,
        RoundingMode::Nearest,
        1000000000000,
        Ordering::Equal,
    );
    test::<i64>(
        -1000000000000,
        -1,
        RoundingMode::Exact,
        1000000000000,
        Ordering::Equal,
    );

    test::<i64>(
        -1000000000000,
        -3,
        RoundingMode::Down,
        333333333333,
        Ordering::Less,
    );
    test::<i64>(
        -1000000000000,
        -3,
        RoundingMode::Floor,
        333333333333,
        Ordering::Less,
    );
    test::<i64>(
        -1000000000000,
        -3,
        RoundingMode::Up,
        333333333334,
        Ordering::Greater,
    );
    test::<i64>(
        -1000000000000,
        -3,
        RoundingMode::Ceiling,
        333333333334,
        Ordering::Greater,
    );
    test::<i64>(
        -1000000000000,
        -3,
        RoundingMode::Nearest,
        333333333333,
        Ordering::Less,
    );

    test::<i64>(
        -999999999999,
        -2,
        RoundingMode::Down,
        499999999999,
        Ordering::Less,
    );
    test::<i64>(
        -999999999999,
        -2,
        RoundingMode::Floor,
        499999999999,
        Ordering::Less,
    );
    test::<i64>(
        -999999999999,
        -2,
        RoundingMode::Up,
        500000000000,
        Ordering::Greater,
    );
    test::<i64>(
        -999999999999,
        -2,
        RoundingMode::Ceiling,
        500000000000,
        Ordering::Greater,
    );
    test::<i64>(
        -999999999999,
        -2,
        RoundingMode::Nearest,
        500000000000,
        Ordering::Greater,
    );

    test::<i64>(
        -1000000000001,
        -2,
        RoundingMode::Down,
        500000000000,
        Ordering::Less,
    );
    test::<i64>(
        -1000000000001,
        -2,
        RoundingMode::Floor,
        500000000000,
        Ordering::Less,
    );
    test::<i64>(
        -1000000000001,
        -2,
        RoundingMode::Up,
        500000000001,
        Ordering::Greater,
    );
    test::<i64>(
        -1000000000001,
        -2,
        RoundingMode::Ceiling,
        500000000001,
        Ordering::Greater,
    );
    test::<i64>(
        -1000000000001,
        -2,
        RoundingMode::Nearest,
        500000000000,
        Ordering::Less,
    );

    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Down,
        232830643708079,
        Ordering::Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Floor,
        232830643708079,
        Ordering::Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Up,
        232830643708080,
        Ordering::Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Ceiling,
        232830643708080,
        Ordering::Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        -0xffffffff,
        RoundingMode::Nearest,
        232830643708080,
        Ordering::Greater,
    );

    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Down,
        1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Floor,
        1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Up,
        1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Ceiling,
        1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Nearest,
        1000000000000,
        Ordering::Equal,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000000,
        RoundingMode::Exact,
        1000000000000,
        Ordering::Equal,
    );

    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Down,
        999999999999,
        Ordering::Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Floor,
        999999999999,
        Ordering::Less,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Up,
        1000000000000,
        Ordering::Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Ceiling,
        1000000000000,
        Ordering::Greater,
    );
    test::<i128>(
        -1000000000000000000000000,
        -1000000000001,
        RoundingMode::Nearest,
        999999999999,
        Ordering::Less,
    );

    test::<i128>(
        -2999999999999999999999999,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        1,
        Ordering::Less,
    );
    test::<i128>(
        -3000000000000000000000000,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        2,
        Ordering::Greater,
    );
    test::<i128>(
        -3000000000000000000000001,
        -2000000000000000000000000,
        RoundingMode::Nearest,
        2,
        Ordering::Greater,
    );

    test::<i8>(-128, 1, RoundingMode::Down, -128, Ordering::Equal);
    test::<i8>(-128, 1, RoundingMode::Up, -128, Ordering::Equal);
    test::<i8>(-128, 1, RoundingMode::Floor, -128, Ordering::Equal);
    test::<i8>(-128, 1, RoundingMode::Ceiling, -128, Ordering::Equal);
    test::<i8>(-128, 1, RoundingMode::Nearest, -128, Ordering::Equal);
    test::<i8>(-128, 1, RoundingMode::Exact, -128, Ordering::Equal);
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
        let o = mut_x.div_round_assign(y, rm);
        let q = mut_x;

        assert_eq!(x.div_round(y, rm), (q, o));
        assert!(q <= x);
        assert_eq!(x.divisible_by(y), o == Ordering::Equal);

        match rm {
            RoundingMode::Floor | RoundingMode::Down => assert_ne!(o, Ordering::Greater),
            RoundingMode::Ceiling | RoundingMode::Up => assert_ne!(o, Ordering::Less),
            RoundingMode::Exact => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
        if let Some(product) = q.checked_mul(y) {
            assert_eq!(product.cmp(&x), o);
        }
    });

    unsigned_pair_gen_var_12::<T, T>().test_properties(|(x, y)| {
        assert_eq!(
            x.ceiling_div_neg_mod(y).0,
            x.div_round(y, RoundingMode::Ceiling).0
        );
    });

    unsigned_pair_gen_var_11::<T>().test_properties(|(x, y)| {
        let q = x.div_exact(y);
        let o = Ordering::Equal;
        assert_eq!(x.div_round(y, RoundingMode::Down), (q, o));
        assert_eq!(x.div_round(y, RoundingMode::Up), (q, o));
        assert_eq!(x.div_round(y, RoundingMode::Floor), (q, o));
        assert_eq!(x.div_round(y, RoundingMode::Ceiling), (q, o));
        assert_eq!(x.div_round(y, RoundingMode::Nearest), (q, o));
        assert_eq!(x.div_round(y, RoundingMode::Exact), (q, o));
    });

    unsigned_pair_gen_var_13::<T>().test_properties(|(x, y)| {
        let down = x.div_round(y, RoundingMode::Down);
        assert_eq!(down.1, Ordering::Less);
        let up = (down.0 + T::ONE, Ordering::Greater);
        assert_eq!(x.div_round(y, RoundingMode::Up), up);
        assert_eq!(x.div_round(y, RoundingMode::Floor), down);
        assert_eq!(x.div_round(y, RoundingMode::Ceiling), up);
        let nearest = x.div_round(y, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(x, rm)| {
        assert_eq!(x.div_round(T::ONE, rm), (x, Ordering::Equal));
    });

    unsigned_rounding_mode_pair_gen_var_1::<T>().test_properties(|(x, rm)| {
        assert_eq!(T::ZERO.div_round(x, rm), (T::ZERO, Ordering::Equal));
        assert_eq!(x.div_round(x, rm), (T::ONE, Ordering::Equal));
    });
}

fn div_round_properties_helper_signed<T: PrimitiveSigned>() {
    signed_signed_rounding_mode_triple_gen_var_1::<T>().test_properties(|(x, y, rm)| {
        let mut mut_x = x;
        let o = mut_x.div_round_assign(y, rm);
        let q = mut_x;

        assert_eq!(x.div_round(y, rm), (q, o));

        assert!(q.le_abs(&x));
        assert_eq!(x.divisible_by(y), o == Ordering::Equal);
        if x != T::MIN {
            let (q_alt, o_alt) = (-x).div_round(y, -rm);
            assert_eq!(-q_alt, q);
            assert_eq!(o_alt.reverse(), o);
        }
        if y != T::MIN && (x != T::MIN || (y != T::ONE && y != T::NEGATIVE_ONE)) {
            let (q_alt, o_alt) = x.div_round(-y, -rm);
            assert_eq!(-q_alt, q);
            assert_eq!(o_alt.reverse(), o);
        }

        match ((x >= T::ZERO) == (y >= T::ZERO), rm) {
            (_, RoundingMode::Floor) | (true, RoundingMode::Down) | (false, RoundingMode::Up) => {
                assert_ne!(o, Ordering::Greater)
            }
            (_, RoundingMode::Ceiling) | (true, RoundingMode::Up) | (false, RoundingMode::Down) => {
                assert_ne!(o, Ordering::Less)
            }
            (_, RoundingMode::Exact) => assert_eq!(o, Ordering::Equal),
            _ => {}
        }
        if let Some(product) = q.checked_mul(y) {
            assert_eq!(product.cmp(&x), if y >= T::ZERO { o } else { o.reverse() });
        }
    });

    signed_pair_gen_var_3::<T>().test_properties(|(x, y)| {
        let q = x.div_exact(y);
        let o = Ordering::Equal;
        assert_eq!(x.div_round(y, RoundingMode::Down), (q, o));
        assert_eq!(x.div_round(y, RoundingMode::Up), (q, o));
        assert_eq!(x.div_round(y, RoundingMode::Floor), (q, o));
        assert_eq!(x.div_round(y, RoundingMode::Ceiling), (q, o));
        assert_eq!(x.div_round(y, RoundingMode::Nearest), (q, o));
        assert_eq!(x.div_round(y, RoundingMode::Exact), (q, o));
    });

    signed_pair_gen_var_5::<T>().test_properties(|(x, y)| {
        let down = x.div_round(y, RoundingMode::Down);
        let up = if (x >= T::ZERO) == (y >= T::ZERO) {
            (down.0 + T::ONE, Ordering::Greater)
        } else {
            (down.0 - T::ONE, Ordering::Less)
        };
        let floor = x.div_round(y, RoundingMode::Floor);
        let ceiling = (floor.0 + T::ONE, Ordering::Greater);
        assert_eq!(x.div_round(y, RoundingMode::Up), up);
        assert_eq!(x.div_round(y, RoundingMode::Ceiling), ceiling);
        let nearest = x.div_round(y, RoundingMode::Nearest);
        assert!(nearest == down || nearest == up);
    });

    signed_rounding_mode_pair_gen::<T>().test_properties(|(x, rm)| {
        assert_eq!(x.div_round(T::ONE, rm), (x, Ordering::Equal));
    });

    signed_rounding_mode_pair_gen_var_2::<T>().test_properties(|(x, rm)| {
        assert_eq!(x.div_round(T::NEGATIVE_ONE, rm), (-x, Ordering::Equal));
    });

    signed_rounding_mode_pair_gen_var_1::<T>().test_properties(|(x, rm)| {
        assert_eq!(T::ZERO.div_round(x, rm), (T::ZERO, Ordering::Equal));
        assert_eq!(x.div_round(x, rm), (T::ONE, Ordering::Equal));
    });

    signed_rounding_mode_pair_gen_var_3::<T>().test_properties(|(x, rm)| {
        assert_eq!(x.div_round(-x, rm), (T::NEGATIVE_ONE, Ordering::Equal));
        assert_eq!((-x).div_round(x, rm), (T::NEGATIVE_ONE, Ordering::Equal));
    });
}

#[test]
fn div_round_properties() {
    apply_fn_to_unsigneds!(div_round_properties_helper_unsigned);
    apply_fn_to_signeds!(div_round_properties_helper_signed);
}
