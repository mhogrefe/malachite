// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    IsPowerOf2, PowerOf2, ShlRound, ShlRoundAssign, ShrRound, UnsignedAbs,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    signed_rounding_mode_pair_gen, signed_rounding_mode_pair_gen_var_5,
    signed_unsigned_pair_gen_var_19, signed_unsigned_pair_gen_var_20,
    signed_unsigned_rounding_mode_triple_gen_var_7, signed_unsigned_rounding_mode_triple_gen_var_8,
    unsigned_pair_gen_var_18, unsigned_rounding_mode_pair_gen,
    unsigned_rounding_mode_pair_gen_var_5, unsigned_unsigned_rounding_mode_triple_gen_var_7,
};
use malachite_float::test_util::arithmetic::shl_round::{
    rug_shl_prec_round_signed, rug_shl_prec_round_unsigned, rug_shl_prec_signed,
    rug_shl_prec_unsigned, rug_shl_round_signed, rug_shl_round_unsigned, shl_prec_naive,
    shl_prec_round_naive, shl_round_naive,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_rounding_mode_pair_gen, float_signed_rounding_mode_triple_gen_var_1,
    float_signed_rounding_mode_triple_gen_var_2,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_1,
    float_signed_unsigned_rounding_mode_quadruple_gen_var_2,
    float_signed_unsigned_triple_gen_var_1, float_signed_unsigned_triple_gen_var_2,
    float_unsigned_pair_gen_var_1, float_unsigned_rounding_mode_triple_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_5, float_unsigned_rounding_mode_triple_gen_var_6,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2,
    float_unsigned_unsigned_triple_gen_var_1, float_unsigned_unsigned_triple_gen_var_2,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_unsigned_rounding_mode_triple_gen_var_5;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::ops::Shl;
use std::panic::catch_unwind;

fn test_shl_prec_round_unsigned_helper<
    T: PrimitiveUnsigned,
    F: Fn(Float, T, u64, RoundingMode, Float, Ordering),
>(
    f: F,
) where
    Float: ShlRoundAssign<T> + ShlRound<T, Output = Float>,
    Rational: Shl<T, Output = Rational>,
    for<'a> &'a Float: ShlRound<T, Output = Float>,
{
    let test_r = |s,
                  s_hex,
                  v: u64,
                  prec: u64,
                  rm: RoundingMode,
                  out: &str,
                  out_hex: &str,
                  out_o: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let v = T::exact_from(v);

        let mut n = x.clone();
        let o = n.shl_prec_round_assign(v, prec, rm);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());
        assert_eq!(o, out_o);

        let (n, o) = x.clone().shl_prec_round(v, prec, rm);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());
        assert_eq!(o, out_o);

        let (n, o) = x.shl_prec_round_ref(v, prec, rm);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());
        assert_eq!(o, out_o);

        let (n, o) = shl_prec_round_naive(x.clone(), v, prec, rm);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert_eq!(o, out_o);

        f(x, v, prec, rm, n, out_o);
    };

    let test = |s, s_hex, v: u64, prec: u64, out: &str, out_hex: &str| {
        for rm in exhaustive_rounding_modes() {
            test_r(s, s_hex, v, prec, rm, out, out_hex, Equal);
        }
    };
    test("NaN", "NaN", 0, 1, "NaN", "NaN");
    test("NaN", "NaN", 0, 10, "NaN", "NaN");
    test("NaN", "NaN", 10, 1, "NaN", "NaN");
    test("NaN", "NaN", 10, 10, "NaN", "NaN");
    test("Infinity", "Infinity", 0, 1, "Infinity", "Infinity");
    test("Infinity", "Infinity", 0, 10, "Infinity", "Infinity");
    test("Infinity", "Infinity", 10, 1, "Infinity", "Infinity");
    test("Infinity", "Infinity", 10, 10, "Infinity", "Infinity");
    test("-Infinity", "-Infinity", 0, 1, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 0, 10, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 10, 1, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 10, 10, "-Infinity", "-Infinity");
    test("0.0", "0x0.0", 10, 1, "0.0", "0x0.0");
    test("0.0", "0x0.0", 10, 10, "0.0", "0x0.0");
    test("-0.0", "-0x0.0", 10, 1, "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", 10, 10, "-0.0", "-0x0.0");

    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        1,
        Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Less,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        1,
        Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );
    test_r("123.0", "0x7b.0#7", 0, 1, Down, "6.0e1", "0x4.0E+1#1", Less);
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        1,
        Up,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        1,
        Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );

    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        10,
        Floor,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        10,
        Ceiling,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        10,
        Down,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test_r("123.0", "0x7b.0#7", 0, 10, Up, "123.0", "0x7b.0#10", Equal);
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        10,
        Nearest,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        10,
        Exact,
        "123.0",
        "0x7b.0#10",
        Equal,
    );

    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        1,
        Floor,
        "7.0e4",
        "0x1.0E+4#1",
        Less,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        1,
        Ceiling,
        "1.0e5",
        "0x2.0E+4#1",
        Greater,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        1,
        Down,
        "7.0e4",
        "0x1.0E+4#1",
        Less,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        1,
        Up,
        "1.0e5",
        "0x2.0E+4#1",
        Greater,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        1,
        Nearest,
        "1.0e5",
        "0x2.0E+4#1",
        Greater,
    );

    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        Floor,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        Ceiling,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        Down,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        Up,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        Nearest,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        Exact,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );

    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        1,
        Floor,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        1,
        Ceiling,
        "-6.0e1",
        "-0x4.0E+1#1",
        Greater,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        1,
        Down,
        "-6.0e1",
        "-0x4.0E+1#1",
        Greater,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        1,
        Up,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        1,
        Nearest,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
    );

    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        10,
        Floor,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        10,
        Ceiling,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        10,
        Down,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        10,
        Up,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        10,
        Nearest,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        10,
        Exact,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );

    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        1,
        Floor,
        "-1.0e5",
        "-0x2.0E+4#1",
        Less,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        1,
        Ceiling,
        "-7.0e4",
        "-0x1.0E+4#1",
        Greater,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        1,
        Down,
        "-7.0e4",
        "-0x1.0E+4#1",
        Greater,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        1,
        Up,
        "-1.0e5",
        "-0x2.0E+4#1",
        Less,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        1,
        Nearest,
        "-1.0e5",
        "-0x2.0E+4#1",
        Less,
    );

    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        Floor,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        Ceiling,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        Down,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        Up,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        Nearest,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        Exact,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );

    // - overflow in shl_prec_round
    // - overflow && sign && rm == Floor | Down in shl_prec_round
    test_r(
        "too_big",
        "0x4.0E+268435455#1",
        1,
        1,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    // - overflow && sign && rm == Up | Ceiling | Nearest in shl_prec_round
    test_r(
        "too_big",
        "0x4.0E+268435455#1",
        1,
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_r(
        "too_big",
        "0x4.0E+268435455#1",
        1,
        1,
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_r(
        "too_big",
        "0x4.0E+268435455#1",
        1,
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_r(
        "too_big",
        "0x4.0E+268435455#1",
        1,
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );

    // - overflow && !sign && rm == Up | Floor | Nearest in shl_prec_round
    test_r(
        "-too_big",
        "-0x4.0E+268435455#1",
        1,
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    // - overflow && !sign && rm == Ceiling | Down in shl_prec_round
    test_r(
        "-too_big",
        "-0x4.0E+268435455#1",
        1,
        1,
        Ceiling,
        "-too_big",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_r(
        "-too_big",
        "-0x4.0E+268435455#1",
        1,
        1,
        Down,
        "-too_big",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_r(
        "-too_big",
        "-0x4.0E+268435455#1",
        1,
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_r(
        "-too_big",
        "-0x4.0E+268435455#1",
        1,
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
}

#[test]
fn test_shl_prec_round_unsigned() {
    test_shl_prec_round_unsigned_helper::<u8, _>(|_, _, _, _, _, _| {});
    test_shl_prec_round_unsigned_helper::<u16, _>(|_, _, _, _, _, _| {});
    test_shl_prec_round_unsigned_helper::<u32, _>(|x, v, prec, rm, shifted, o_out| {
        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (n, o) = rug_shl_prec_round_unsigned(&rug::Float::exact_from(&x), v, prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&n)),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o, o_out);
        }
    });
    test_shl_prec_round_unsigned_helper::<u64, _>(|_, _, _, _, _, _| {});
    test_shl_prec_round_unsigned_helper::<u128, _>(|_, _, _, _, _, _| {});
    test_shl_prec_round_unsigned_helper::<usize, _>(|_, _, _, _, _, _| {});
}

fn test_shl_prec_round_signed_helper<
    T: PrimitiveSigned,
    F: Fn(Float, T, u64, RoundingMode, Float, Ordering),
>(
    f: F,
) where
    Float: ShlRoundAssign<T> + ShlRound<T, Output = Float>,
    Rational: Shl<T, Output = Rational>,
    for<'a> &'a Float: ShlRound<T, Output = Float>,
{
    let test_r = |s,
                  s_hex,
                  v: i64,
                  prec: u64,
                  rm: RoundingMode,
                  out: &str,
                  out_hex: &str,
                  out_o: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let v = T::exact_from(v);

        let mut n = x.clone();
        let o = n.shl_prec_round_assign(v, prec, rm);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());
        assert_eq!(o, out_o);

        let (n, o) = x.clone().shl_prec_round(v, prec, rm);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());
        assert_eq!(o, out_o);

        let (n, o) = x.shl_prec_round_ref(v, prec, rm);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());
        assert_eq!(o, out_o);

        let (n, o) = shl_prec_round_naive(x.clone(), v, prec, rm);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert_eq!(o, out_o);

        f(x, v, prec, rm, n, out_o);
    };

    let test = |s, s_hex, v: i64, prec: u64, out: &str, out_hex: &str| {
        for rm in exhaustive_rounding_modes() {
            test_r(s, s_hex, v, prec, rm, out, out_hex, Equal);
        }
    };
    test("NaN", "NaN", 0, 1, "NaN", "NaN");
    test("NaN", "NaN", 0, 10, "NaN", "NaN");
    test("NaN", "NaN", 10, 1, "NaN", "NaN");
    test("NaN", "NaN", 10, 10, "NaN", "NaN");
    test("NaN", "NaN", -10, 1, "NaN", "NaN");
    test("NaN", "NaN", -10, 10, "NaN", "NaN");
    test("Infinity", "Infinity", 0, 1, "Infinity", "Infinity");
    test("Infinity", "Infinity", 0, 10, "Infinity", "Infinity");
    test("Infinity", "Infinity", 10, 1, "Infinity", "Infinity");
    test("Infinity", "Infinity", 10, 10, "Infinity", "Infinity");
    test("Infinity", "Infinity", -10, 1, "Infinity", "Infinity");
    test("Infinity", "Infinity", -10, 10, "Infinity", "Infinity");
    test("-Infinity", "-Infinity", 0, 1, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 0, 10, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 10, 1, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 10, 10, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", -10, 1, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", -10, 10, "-Infinity", "-Infinity");
    test("0.0", "0x0.0", 10, 1, "0.0", "0x0.0");
    test("0.0", "0x0.0", 10, 10, "0.0", "0x0.0");
    test("0.0", "0x0.0", -10, 1, "0.0", "0x0.0");
    test("0.0", "0x0.0", -10, 10, "0.0", "0x0.0");
    test("-0.0", "-0x0.0", 10, 1, "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", 10, 10, "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", -10, 1, "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", -10, 10, "-0.0", "-0x0.0");

    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        1,
        Floor,
        "6.0e1",
        "0x4.0E+1#1",
        Less,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        1,
        Ceiling,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );
    test_r("123.0", "0x7b.0#7", 0, 1, Down, "6.0e1", "0x4.0E+1#1", Less);
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        1,
        Up,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        1,
        Nearest,
        "1.0e2",
        "0x8.0E+1#1",
        Greater,
    );

    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        10,
        Floor,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        10,
        Ceiling,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        10,
        Down,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test_r("123.0", "0x7b.0#7", 0, 10, Up, "123.0", "0x7b.0#10", Equal);
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        10,
        Nearest,
        "123.0",
        "0x7b.0#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        0,
        10,
        Exact,
        "123.0",
        "0x7b.0#10",
        Equal,
    );

    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        1,
        Floor,
        "7.0e4",
        "0x1.0E+4#1",
        Less,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        1,
        Ceiling,
        "1.0e5",
        "0x2.0E+4#1",
        Greater,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        1,
        Down,
        "7.0e4",
        "0x1.0E+4#1",
        Less,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        1,
        Up,
        "1.0e5",
        "0x2.0E+4#1",
        Greater,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        1,
        Nearest,
        "1.0e5",
        "0x2.0E+4#1",
        Greater,
    );

    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        Floor,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        Ceiling,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        Down,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        Up,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        Nearest,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        Exact,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );

    test_r("123.0", "0x7b.0#7", -10, 1, Floor, "0.06", "0x0.1#1", Less);
    test_r(
        "123.0", "0x7b.0#7", -10, 1, Ceiling, "0.1", "0x0.2#1", Greater,
    );
    test_r("123.0", "0x7b.0#7", -10, 1, Down, "0.06", "0x0.1#1", Less);
    test_r("123.0", "0x7b.0#7", -10, 1, Up, "0.1", "0x0.2#1", Greater);
    test_r(
        "123.0", "0x7b.0#7", -10, 1, Nearest, "0.1", "0x0.2#1", Greater,
    );

    test_r(
        "123.0",
        "0x7b.0#7",
        -10,
        10,
        Floor,
        "0.1201",
        "0x0.1ec0#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        -10,
        10,
        Ceiling,
        "0.1201",
        "0x0.1ec0#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        -10,
        10,
        Down,
        "0.1201",
        "0x0.1ec0#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        -10,
        10,
        Up,
        "0.1201",
        "0x0.1ec0#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        -10,
        10,
        Nearest,
        "0.1201",
        "0x0.1ec0#10",
        Equal,
    );
    test_r(
        "123.0",
        "0x7b.0#7",
        -10,
        10,
        Exact,
        "0.1201",
        "0x0.1ec0#10",
        Equal,
    );

    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        1,
        Floor,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        1,
        Ceiling,
        "-6.0e1",
        "-0x4.0E+1#1",
        Greater,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        1,
        Down,
        "-6.0e1",
        "-0x4.0E+1#1",
        Greater,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        1,
        Up,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        1,
        Nearest,
        "-1.0e2",
        "-0x8.0E+1#1",
        Less,
    );

    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        10,
        Floor,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        10,
        Ceiling,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        10,
        Down,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        10,
        Up,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        10,
        Nearest,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        0,
        10,
        Exact,
        "-123.0",
        "-0x7b.0#10",
        Equal,
    );

    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        1,
        Floor,
        "-1.0e5",
        "-0x2.0E+4#1",
        Less,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        1,
        Ceiling,
        "-7.0e4",
        "-0x1.0E+4#1",
        Greater,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        1,
        Down,
        "-7.0e4",
        "-0x1.0E+4#1",
        Greater,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        1,
        Up,
        "-1.0e5",
        "-0x2.0E+4#1",
        Less,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        1,
        Nearest,
        "-1.0e5",
        "-0x2.0E+4#1",
        Less,
    );

    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        Floor,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        Ceiling,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        Down,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        Up,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        Nearest,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        Exact,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );

    test_r(
        "-123.0",
        "-0x7b.0#7",
        -10,
        1,
        Floor,
        "-0.1",
        "-0x0.2#1",
        Less,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        -10,
        1,
        Ceiling,
        "-0.06",
        "-0x0.1#1",
        Greater,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        -10,
        1,
        Down,
        "-0.06",
        "-0x0.1#1",
        Greater,
    );
    test_r("-123.0", "-0x7b.0#7", -10, 1, Up, "-0.1", "-0x0.2#1", Less);
    test_r(
        "-123.0",
        "-0x7b.0#7",
        -10,
        1,
        Nearest,
        "-0.1",
        "-0x0.2#1",
        Less,
    );

    test_r(
        "-123.0",
        "-0x7b.0#7",
        -10,
        10,
        Floor,
        "-0.1201",
        "-0x0.1ec0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        -10,
        10,
        Ceiling,
        "-0.1201",
        "-0x0.1ec0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        -10,
        10,
        Down,
        "-0.1201",
        "-0x0.1ec0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        -10,
        10,
        Up,
        "-0.1201",
        "-0x0.1ec0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        -10,
        10,
        Nearest,
        "-0.1201",
        "-0x0.1ec0#10",
        Equal,
    );
    test_r(
        "-123.0",
        "-0x7b.0#7",
        -10,
        10,
        Exact,
        "-0.1201",
        "-0x0.1ec0#10",
        Equal,
    );

    // - overflow in shl_prec_round
    // - overflow && sign && rm == Floor | Down in shl_prec_round
    test_r(
        "too_big",
        "0x4.0E+268435455#1",
        1,
        1,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    // - overflow && sign && rm == Up | Ceiling | Nearest in shl_prec_round
    test_r(
        "too_big",
        "0x4.0E+268435455#1",
        1,
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_r(
        "too_big",
        "0x4.0E+268435455#1",
        1,
        1,
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_r(
        "too_big",
        "0x4.0E+268435455#1",
        1,
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_r(
        "too_big",
        "0x4.0E+268435455#1",
        1,
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );

    // - overflow && !sign && rm == Up | Floor | Nearest in shl_prec_round
    test_r(
        "-too_big",
        "-0x4.0E+268435455#1",
        1,
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    // - overflow && !sign && rm == Ceiling | Down in shl_prec_round
    test_r(
        "-too_big",
        "-0x4.0E+268435455#1",
        1,
        1,
        Ceiling,
        "-too_big",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_r(
        "-too_big",
        "-0x4.0E+268435455#1",
        1,
        1,
        Down,
        "-too_big",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_r(
        "-too_big",
        "-0x4.0E+268435455#1",
        1,
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_r(
        "-too_big",
        "-0x4.0E+268435455#1",
        1,
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );

    // - underflow in shl_prec_round
    // - underflow && sign && rm == Floor | Down | Nearest in shl_prec_round
    test_r(
        "too_small",
        "0x1.0E-268435456#1",
        -1,
        1,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    // - underflow && sign && rm == Up | Ceiling in shl_prec_round
    test_r(
        "too_small",
        "0x1.0E-268435456#1",
        -1,
        1,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_r(
        "too_small",
        "0x1.0E-268435456#1",
        -1,
        1,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_r(
        "too_small",
        "0x1.0E-268435456#1",
        -1,
        1,
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_r(
        "too_small",
        "0x1.0E-268435456#1",
        -1,
        1,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );

    // - underflow && !sign && rm == Up | Floor in shl_prec_round
    test_r(
        "-too_small",
        "-0x1.0E-268435456#1",
        -1,
        1,
        Floor,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    // - underflow && !sign && rm == Ceiling | Down | Nearest in shl_prec_round
    test_r(
        "-too_small",
        "-0x1.0E-268435456#1",
        -1,
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_r(
        "-too_small",
        "-0x1.0E-268435456#1",
        -1,
        1,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_r(
        "-too_small",
        "-0x1.0E-268435456#1",
        -1,
        1,
        Up,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_r(
        "-too_small",
        "-0x1.0E-268435456#1",
        -1,
        1,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
    );

    // - underflow half-up in shl_prec_round
    // - underflow half-up && sign in shl_prec_round
    test_r(
        "too_small",
        "0x4.df81e47e11c9aa6dE-268435455#67",
        -7,
        66,
        Nearest,
        "too_small",
        "0x1.00000000000000000E-268435456#66",
        Greater,
    );
    // - underflow half-up && !sign in shl_prec_round
    test_r(
        "-too_small",
        "-0x4.df81e47e11c9aa6dE-268435455#67",
        -7,
        66,
        Nearest,
        "-too_small",
        "-0x1.00000000000000000E-268435456#66",
        Less,
    );
}

#[test]
fn test_shl_prec_round_signed() {
    test_shl_prec_round_signed_helper::<i8, _>(|_, _, _, _, _, _| {});
    test_shl_prec_round_signed_helper::<i16, _>(|_, _, _, _, _, _| {});
    test_shl_prec_round_signed_helper::<i32, _>(|x, v, prec, rm, shifted, o_out| {
        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (n, o) = rug_shl_prec_round_signed(&rug::Float::exact_from(&x), v, prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&n)),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o, o_out);
        }
    });
    test_shl_prec_round_signed_helper::<i64, _>(|_, _, _, _, _, _| {});
    test_shl_prec_round_signed_helper::<i128, _>(|_, _, _, _, _, _| {});
    test_shl_prec_round_signed_helper::<isize, _>(|_, _, _, _, _, _| {});
}

#[test]
fn shl_prec_round_fail() {
    assert_panic!(Float::ONE.shl_prec_round(Float::MAX_EXPONENT, 1, Exact));
    assert_panic!(Float::ONE.shl_prec_round(Float::MIN_EXPONENT - 2, 1, Exact));
    assert_panic!(Float::NEGATIVE_ONE.shl_prec_round(Float::MAX_EXPONENT, 1, Exact));
    assert_panic!(Float::NEGATIVE_ONE.shl_prec_round(Float::MIN_EXPONENT - 2, 1, Exact));
}

fn test_shl_prec_unsigned_helper<T: PrimitiveUnsigned, F: Fn(Float, T, u64, Float, Ordering)>(f: F)
where
    Rational: Shl<T, Output = Rational>,
{
    let test = |s, s_hex, v: u64, prec: u64, out: &str, out_hex: &str, out_o: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let v = T::exact_from(v);

        let mut n = x.clone();
        let o = n.shl_prec_assign(v, prec);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());
        assert_eq!(o, out_o);

        let (n, o) = x.clone().shl_prec(v, prec);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());
        assert_eq!(o, out_o);

        let (n, o) = x.shl_prec_ref(v, prec);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());
        assert_eq!(o, out_o);

        let (n, o) = shl_prec_naive(x.clone(), v, prec);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert_eq!(o, out_o);

        f(x, v, prec, n, out_o);
    };

    test("NaN", "NaN", 0, 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", 0, 10, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, 10, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 0, 1, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", 0, 10, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", 10, 1, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", 10, 10, "Infinity", "Infinity", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        0,
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        0,
        10,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        10,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("0.0", "0x0.0", 10, 1, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, 10, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, 1, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, 10, "-0.0", "-0x0.0", Equal);

    test("123.0", "0x7b.0#7", 0, 1, "1.0e2", "0x8.0E+1#1", Greater);
    test("123.0", "0x7b.0#7", 0, 10, "123.0", "0x7b.0#10", Equal);
    test("123.0", "0x7b.0#7", 10, 1, "1.0e5", "0x2.0E+4#1", Greater);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );
    test("-123.0", "-0x7b.0#7", 0, 1, "-1.0e2", "-0x8.0E+1#1", Less);
    test("-123.0", "-0x7b.0#7", 0, 10, "-123.0", "-0x7b.0#10", Equal);
    test("-123.0", "-0x7b.0#7", 10, 1, "-1.0e5", "-0x2.0E+4#1", Less);
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );
    test(
        "too_big",
        "0x4.0E+268435455#1",
        1,
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-too_big",
        "-0x4.0E+268435455#1",
        1,
        1,
        "-Infinity",
        "-Infinity",
        Less,
    );
}

#[test]
fn test_shl_prec_unsigned() {
    test_shl_prec_unsigned_helper::<u8, _>(|_, _, _, _, _| {});
    test_shl_prec_unsigned_helper::<u16, _>(|_, _, _, _, _| {});
    test_shl_prec_unsigned_helper::<u32, _>(|x, v, prec, shifted, o_out| {
        let (n, o) = rug_shl_prec_unsigned(&rug::Float::exact_from(&x), v, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&n)),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o, o_out);
    });
    test_shl_prec_unsigned_helper::<u64, _>(|_, _, _, _, _| {});
    test_shl_prec_unsigned_helper::<u128, _>(|_, _, _, _, _| {});
    test_shl_prec_unsigned_helper::<usize, _>(|_, _, _, _, _| {});
}

fn test_shl_prec_signed_helper<T: PrimitiveSigned, F: Fn(Float, T, u64, Float, Ordering)>(f: F)
where
    Rational: Shl<T, Output = Rational>,
{
    let test = |s, s_hex, v: i64, prec: u64, out: &str, out_hex: &str, out_o: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let v = T::exact_from(v);

        let mut n = x.clone();
        let o = n.shl_prec_assign(v, prec);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());
        assert_eq!(o, out_o);

        let (n, o) = x.clone().shl_prec(v, prec);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());
        assert_eq!(o, out_o);

        let (n, o) = x.shl_prec_ref(v, prec);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert!(n.is_valid());
        assert_eq!(o, out_o);

        let (n, o) = shl_prec_naive(x.clone(), v, prec);
        assert_eq!(n.to_string(), out);
        assert_eq!(to_hex_string(&n), out_hex);
        assert_eq!(o, out_o);

        f(x, v, prec, n, out_o);
    };
    test("NaN", "NaN", 0, 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", 0, 10, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, 10, "NaN", "NaN", Equal);
    test("NaN", "NaN", -10, 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", -10, 10, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 0, 1, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", 0, 10, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", 10, 1, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", 10, 10, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", -10, 1, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", -10, 10, "Infinity", "Infinity", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        0,
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        0,
        10,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        10,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        -10,
        1,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        -10,
        10,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("0.0", "0x0.0", 10, 1, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, 10, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", -10, 1, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", -10, 10, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, 1, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, 10, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", -10, 1, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", -10, 10, "-0.0", "-0x0.0", Equal);

    test("123.0", "0x7b.0#7", 0, 1, "1.0e2", "0x8.0E+1#1", Greater);
    test("123.0", "0x7b.0#7", 0, 10, "123.0", "0x7b.0#10", Equal);
    test("123.0", "0x7b.0#7", 10, 1, "1.0e5", "0x2.0E+4#1", Greater);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        10,
        "1.26e5",
        "0x1.ec0E+4#10",
        Equal,
    );
    test("123.0", "0x7b.0#7", -10, 1, "0.1", "0x0.2#1", Greater);
    test("123.0", "0x7b.0#7", -10, 10, "0.1201", "0x0.1ec0#10", Equal);
    test("-123.0", "-0x7b.0#7", 0, 1, "-1.0e2", "-0x8.0E+1#1", Less);
    test("-123.0", "-0x7b.0#7", 0, 10, "-123.0", "-0x7b.0#10", Equal);
    test("-123.0", "-0x7b.0#7", 10, 1, "-1.0e5", "-0x2.0E+4#1", Less);
    test(
        "-123.0",
        "-0x7b.0#7",
        10,
        10,
        "-1.26e5",
        "-0x1.ec0E+4#10",
        Equal,
    );
    test("-123.0", "-0x7b.0#7", -10, 1, "-0.1", "-0x0.2#1", Less);
    test(
        "-123.0",
        "-0x7b.0#7",
        -10,
        10,
        "-0.1201",
        "-0x0.1ec0#10",
        Equal,
    );
    test(
        "too_big",
        "0x4.0E+268435455#1",
        1,
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-too_big",
        "-0x4.0E+268435455#1",
        1,
        1,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "too_small",
        "0x1.0E-268435456#1",
        -1,
        1,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "-too_small",
        "-0x1.0E-268435456#1",
        -1,
        1,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    // - underflow half-up in shl_prec_round
    // - underflow half-up && sign in shl_prec_round
    test(
        "too_small",
        "0x4.df81e47e11c9aa6dE-268435455#67",
        -7,
        66,
        "too_small",
        "0x1.00000000000000000E-268435456#66",
        Greater,
    );
    // - underflow half-up && !sign in shl_prec_round
    test(
        "-too_small",
        "-0x4.df81e47e11c9aa6dE-268435455#67",
        -7,
        66,
        "-too_small",
        "-0x1.00000000000000000E-268435456#66",
        Less,
    );
}

#[test]
fn test_shl_prec_signed() {
    test_shl_prec_signed_helper::<i8, _>(|_, _, _, _, _| {});
    test_shl_prec_signed_helper::<i16, _>(|_, _, _, _, _| {});
    test_shl_prec_signed_helper::<i32, _>(|x, v, prec, shifted, o_out| {
        let (n, o) = rug_shl_prec_signed(&rug::Float::exact_from(&x), v, prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&n)),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o, o_out);
    });
    test_shl_prec_signed_helper::<i64, _>(|_, _, _, _, _| {});
    test_shl_prec_signed_helper::<i128, _>(|_, _, _, _, _| {});
    test_shl_prec_signed_helper::<isize, _>(|_, _, _, _, _| {});
}

fn test_shl_round_unsigned_helper<
    T: PrimitiveUnsigned,
    F: Fn(Float, T, RoundingMode, Float, Ordering),
>(
    f: F,
) where
    Float: ShlRoundAssign<T> + ShlRound<T, Output = Float>,
    Rational: Shl<T, Output = Rational>,
    for<'a> &'a Float: ShlRound<T, Output = Float>,
{
    let test = |s, s_hex, v: u64, out: &str, out_hex: &str| {
        for rm in exhaustive_rounding_modes() {
            let x = parse_hex_string(s_hex);
            assert_eq!(x.to_string(), s);
            let v = T::exact_from(v);

            let mut n = x.clone();
            let o = n.shl_round_assign(v, rm);
            assert_eq!(n.to_string(), out);
            assert_eq!(to_hex_string(&n), out_hex);
            assert!(n.is_valid());
            assert_eq!(o, Equal);

            let (n, o) = x.clone().shl_round(v, rm);
            assert_eq!(n.to_string(), out);
            assert_eq!(to_hex_string(&n), out_hex);
            assert!(n.is_valid());
            assert_eq!(o, Equal);

            let (n, o) = (&x).shl_round(v, rm);
            assert_eq!(n.to_string(), out);
            assert_eq!(to_hex_string(&n), out_hex);
            assert!(n.is_valid());
            assert_eq!(o, Equal);

            let (n, o) = shl_round_naive(x.clone(), v, rm);
            assert_eq!(n.to_string(), out);
            assert_eq!(to_hex_string(&n), out_hex);
            assert_eq!(o, Equal);

            f(x, v, rm, n, Equal);
        }
    };
    test("NaN", "NaN", 0, "NaN", "NaN");
    test("NaN", "NaN", 10, "NaN", "NaN");
    test("Infinity", "Infinity", 0, "Infinity", "Infinity");
    test("Infinity", "Infinity", 10, "Infinity", "Infinity");
    test("-Infinity", "-Infinity", 0, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 10, "-Infinity", "-Infinity");
    test("0.0", "0x0.0", 10, "0.0", "0x0.0");
    test("-0.0", "-0x0.0", 10, "-0.0", "-0x0.0");

    test("123.0", "0x7b.0#7", 0, "123.0", "0x7b.0#7");
    test("123.0", "0x7b.0#7", 1, "246.0", "0xf6.0#7");
    test("123.0", "0x7b.0#7", 10, "1.26e5", "0x1.ecE+4#7");
    test("123.0", "0x7b.0#7", 100, "1.56e32", "0x7.bE+26#7");

    test("-123.0", "-0x7b.0#7", 0, "-123.0", "-0x7b.0#7");
    test("-123.0", "-0x7b.0#7", 1, "-246.0", "-0xf6.0#7");
    test("-123.0", "-0x7b.0#7", 10, "-1.26e5", "-0x1.ecE+4#7");
    test("-123.0", "-0x7b.0#7", 100, "-1.56e32", "-0x7.bE+26#7");

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        0,
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "6.283185307179586",
        "0x6.487ed5110b460#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "3216.9908772759482",
        "0xc90.fdaa22168c0#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        "3.9824418129956972e30",
        "0x3.243f6a8885a30E+25#53",
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        0,
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "-6.283185307179586",
        "-0x6.487ed5110b460#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-3216.9908772759482",
        "-0xc90.fdaa22168c0#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        "-3.9824418129956972e30",
        "-0x3.243f6a8885a30E+25#53",
    );

    let test_extreme =
        |s, s_hex, v: i32, rm: RoundingMode, out: &str, out_hex: &str, out_o: Ordering| {
            let v = u64::exact_from(v);
            if T::convertible_from(v) {
                let x = parse_hex_string(s_hex);
                assert_eq!(x.to_string(), s);
                let v = T::exact_from(v);

                let mut n = x.clone();
                let o = n.shl_round_assign(v, rm);
                assert_eq!(n.to_string(), out);
                assert_eq!(to_hex_string(&n), out_hex);
                assert!(n.is_valid());
                assert_eq!(o, out_o);

                let (n, o) = x.clone().shl_round(v, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());
                assert_eq!(o, out_o);

                let (n, o) = (&x).shl_round(v, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());
                assert_eq!(o, out_o);

                let (n, o) = shl_round_naive(x.clone(), v, rm);
                assert_eq!(n.to_string(), out);
                assert_eq!(to_hex_string(&n), out_hex);
                assert_eq!(o, out_o);

                f(x, v, rm, n, out_o);
            }
        };
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Ceiling,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Up,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Nearest,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Exact,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );

    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT,
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );

    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Floor,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Ceiling,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Down,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Up,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Nearest,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Exact,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );

    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT,
        Ceiling,
        "-too_big",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT,
        Down,
        "-too_big",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
}

#[test]
fn test_shl_round_unsigned() {
    test_shl_round_unsigned_helper::<u8, _>(|_, _, _, _, _| {});
    test_shl_round_unsigned_helper::<u16, _>(|_, _, _, _, _| {});
    test_shl_round_unsigned_helper::<u32, _>(|x, v, rm, shifted, o_out| {
        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (n, o) = rug_shl_round_unsigned(&rug::Float::exact_from(&x), v, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&n)),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o, o_out);
        }
    });
    test_shl_round_unsigned_helper::<u64, _>(|_, _, _, _, _| {});
    test_shl_round_unsigned_helper::<u128, _>(|_, _, _, _, _| {});
    test_shl_round_unsigned_helper::<usize, _>(|_, _, _, _, _| {});
}

fn test_shl_round_signed_helper<
    T: PrimitiveSigned,
    F: Fn(Float, T, RoundingMode, Float, Ordering),
>(
    f: F,
) where
    Float: ShlRoundAssign<T> + ShlRound<T, Output = Float>,
    Rational: Shl<T, Output = Rational>,
    for<'a> &'a Float: ShlRound<T, Output = Float>,
{
    let test = |s, s_hex, v: i64, out: &str, out_hex: &str| {
        for rm in exhaustive_rounding_modes() {
            let x = parse_hex_string(s_hex);
            assert_eq!(x.to_string(), s);
            let v = T::exact_from(v);

            let mut n = x.clone();
            let o = n.shl_round_assign(v, rm);
            assert_eq!(n.to_string(), out);
            assert_eq!(to_hex_string(&n), out_hex);
            assert!(n.is_valid());
            assert_eq!(o, Equal);

            let (n, o) = x.clone().shl_round(v, rm);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());
            assert_eq!(o, Equal);

            let (n, o) = (&x).shl_round(v, rm);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());
            assert_eq!(o, Equal);

            let (n, o) = shl_round_naive(x.clone(), v, rm);
            assert_eq!(n.to_string(), out);
            assert_eq!(to_hex_string(&n), out_hex);
            assert_eq!(o, Equal);

            f(x, v, rm, n, Equal);
        }
    };
    test("NaN", "NaN", 0, "NaN", "NaN");
    test("NaN", "NaN", 10, "NaN", "NaN");
    test("NaN", "NaN", -10, "NaN", "NaN");
    test("Infinity", "Infinity", 0, "Infinity", "Infinity");
    test("Infinity", "Infinity", 10, "Infinity", "Infinity");
    test("Infinity", "Infinity", -10, "Infinity", "Infinity");
    test("-Infinity", "-Infinity", 0, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", 10, "-Infinity", "-Infinity");
    test("-Infinity", "-Infinity", -10, "-Infinity", "-Infinity");
    test("0.0", "0x0.0", 10, "0.0", "0x0.0");
    test("0.0", "0x0.0", -10, "0.0", "0x0.0");
    test("-0.0", "-0x0.0", 10, "-0.0", "-0x0.0");
    test("-0.0", "-0x0.0", -10, "-0.0", "-0x0.0");

    test("123.0", "0x7b.0#7", 0, "123.0", "0x7b.0#7");
    test("123.0", "0x7b.0#7", 1, "246.0", "0xf6.0#7");
    test("123.0", "0x7b.0#7", 10, "1.26e5", "0x1.ecE+4#7");
    test("123.0", "0x7b.0#7", 100, "1.56e32", "0x7.bE+26#7");
    test("123.0", "0x7b.0#7", -1, "61.5", "0x3d.8#7");
    test("123.0", "0x7b.0#7", -10, "0.12", "0x0.1ec#7");
    test("123.0", "0x7b.0#7", -100, "9.7e-29", "0x7.bE-24#7");

    test("-123.0", "-0x7b.0#7", 0, "-123.0", "-0x7b.0#7");
    test("-123.0", "-0x7b.0#7", 1, "-246.0", "-0xf6.0#7");
    test("-123.0", "-0x7b.0#7", 10, "-1.26e5", "-0x1.ecE+4#7");
    test("-123.0", "-0x7b.0#7", 100, "-1.56e32", "-0x7.bE+26#7");
    test("-123.0", "-0x7b.0#7", -1, "-61.5", "-0x3d.8#7");
    test("-123.0", "-0x7b.0#7", -10, "-0.12", "-0x0.1ec#7");
    test("-123.0", "-0x7b.0#7", -100, "-9.7e-29", "-0x7.bE-24#7");

    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        0,
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "6.283185307179586",
        "0x6.487ed5110b460#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "3216.9908772759482",
        "0xc90.fdaa22168c0#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        "3.9824418129956972e30",
        "0x3.243f6a8885a30E+25#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -1,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -10,
        "0.0030679615757712823",
        "0x0.00c90fdaa22168c0#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -100,
        "2.4782796245465248e-30",
        "0x3.243f6a8885a30E-25#53",
    );

    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        0,
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "-6.283185307179586",
        "-0x6.487ed5110b460#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-3216.9908772759482",
        "-0xc90.fdaa22168c0#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        "-3.9824418129956972e30",
        "-0x3.243f6a8885a30E+25#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -1,
        "-1.5707963267948966",
        "-0x1.921fb54442d18#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -10,
        "-0.0030679615757712823",
        "-0x0.00c90fdaa22168c0#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -100,
        "-2.4782796245465248e-30",
        "-0x3.243f6a8885a30E-25#53",
    );

    let test_extreme =
        |s, s_hex, v: i32, rm: RoundingMode, out: &str, out_hex: &str, out_o: Ordering| {
            let v = i64::from(v);
            if T::convertible_from(v) {
                let x = parse_hex_string(s_hex);
                assert_eq!(x.to_string(), s);
                let v = T::exact_from(v);

                let mut n = x.clone();
                let o = n.shl_round_assign(v, rm);
                assert_eq!(n.to_string(), out);
                assert_eq!(to_hex_string(&n), out_hex);
                assert!(n.is_valid());
                assert_eq!(o, out_o);

                let (n, o) = x.clone().shl_round(v, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());
                assert_eq!(o, out_o);

                let (n, o) = (&x).shl_round(v, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());
                assert_eq!(o, out_o);

                let (n, o) = shl_round_naive(x.clone(), v, rm);
                assert_eq!(n.to_string(), out);
                assert_eq!(to_hex_string(&n), out_hex);
                assert_eq!(o, out_o);

                f(x, v, rm, n, out_o);
            }
        };
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Ceiling,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Up,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Nearest,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Exact,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );

    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT,
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );

    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Floor,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Ceiling,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Down,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Up,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Nearest,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT - 1,
        Exact,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );

    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT,
        Ceiling,
        "-too_big",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT,
        Down,
        "-too_big",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );

    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT - 1,
        Floor,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT - 1,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT - 1,
        Down,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT - 1,
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT - 1,
        Nearest,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT - 1,
        Exact,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );

    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT - 2,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT - 2,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT - 2,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT - 2,
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT - 2,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );

    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MIN_EXPONENT - 2,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MIN_EXPONENT - 2,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#2",
        Greater,
    );
    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MIN_EXPONENT - 2,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MIN_EXPONENT - 2,
        Up,
        "too_small",
        "0x1.0E-268435456#2",
        Greater,
    );
    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MIN_EXPONENT - 2,
        Nearest,
        "too_small",
        "0x1.0E-268435456#2",
        Greater,
    );

    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT - 1,
        Floor,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT - 1,
        Ceiling,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT - 1,
        Down,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT - 1,
        Up,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT - 1,
        Nearest,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT - 1,
        Exact,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );

    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT - 2,
        Floor,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT - 2,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT - 2,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT - 2,
        Up,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT - 2,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
    );

    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MIN_EXPONENT - 2,
        Floor,
        "-too_small",
        "-0x1.0E-268435456#2",
        Less,
    );
    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MIN_EXPONENT - 2,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MIN_EXPONENT - 2,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MIN_EXPONENT - 2,
        Up,
        "-too_small",
        "-0x1.0E-268435456#2",
        Less,
    );
    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MIN_EXPONENT - 2,
        Nearest,
        "-too_small",
        "-0x1.0E-268435456#2",
        Less,
    );
}

#[test]
fn shl_round_fail() {
    assert_panic!(Float::ONE.shl_round(Float::MAX_EXPONENT, Exact));
    assert_panic!(Float::ONE.shl_round(Float::MIN_EXPONENT - 2, Exact));
    assert_panic!(Float::NEGATIVE_ONE.shl_round(Float::MAX_EXPONENT, Exact));
    assert_panic!(Float::NEGATIVE_ONE.shl_round(Float::MIN_EXPONENT - 2, Exact));
}

#[test]
fn test_shl_round_signed() {
    test_shl_round_signed_helper::<i8, _>(|_, _, _, _, _| {});
    test_shl_round_signed_helper::<i16, _>(|_, _, _, _, _| {});
    test_shl_round_signed_helper::<i32, _>(|x, v, rm, shifted, o_out| {
        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (n, o) = rug_shl_round_signed(&rug::Float::exact_from(&x), v, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&n)),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o, o_out);
        }
    });
    test_shl_round_signed_helper::<i64, _>(|_, _, _, _, _| {});
    test_shl_round_signed_helper::<i128, _>(|_, _, _, _, _| {});
    test_shl_round_signed_helper::<isize, _>(|_, _, _, _, _| {});
}

#[allow(clippy::needless_pass_by_value)]
fn shl_prec_properties_helper_unsigned_helper<T: PrimitiveUnsigned>(n: Float, u: T, prec: u64)
where
    i128: TryFrom<T>,
    u64: TryFrom<T>,
    Rational: Shl<T, Output = Rational>,
{
    let mut mut_n = n.clone();
    let o = mut_n.shl_prec_assign(u, prec);
    assert!(mut_n.is_valid());
    let shifted = mut_n;
    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (shifted_alt, o_alt) = n.shl_prec_round_ref(u, prec, rm);
            assert_eq!(
                ComparableFloatRef(&shifted_alt),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o_alt, Equal);
        }
    }

    let (shifted_alt, o_alt) = n.shl_prec_ref(u, prec);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    let (shifted_alt, o_alt) = n.clone().shl_prec(u, prec);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    if i128::from(n.get_exponent().unwrap_or(1))
        .wrapping_add(i128::exact_from(u))
        .lt_abs(&1_000_000)
    {
        let (shifted_alt, o_alt) = shl_prec_naive(n.clone(), u, prec);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted),
        );
        assert_eq!(o_alt, o);
    }

    if shifted.is_normal() {
        assert_eq!(shifted.get_prec(), Some(prec));
    }

    let (shifted_2, o_2) = (-&n).shl_prec(u, prec);
    let (shifted_2_alt, o_2_alt) = n.shl_prec_ref(u, prec);
    assert_eq!(ComparableFloat(-shifted_2_alt), ComparableFloat(shifted_2));
    assert_eq!(o_2_alt.reverse(), o_2);

    if shifted.is_normal() {
        let (shifted_alt, o_alt) = n.mul_prec(Float::power_of_2(u64::exact_from(u)), prec);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    }
}

fn shl_prec_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    i128: TryFrom<T>,
    u64: TryFrom<T>,
    Rational: Shl<T, Output = Rational>,
{
    float_unsigned_unsigned_triple_gen_var_1::<T, _>().test_properties(|(n, u, prec)| {
        shl_prec_properties_helper_unsigned_helper(n, u, prec);
    });

    float_unsigned_unsigned_triple_gen_var_2::<T, _>().test_properties(|(n, u, prec)| {
        shl_prec_properties_helper_unsigned_helper(n, u, prec);
    });

    float_unsigned_pair_gen_var_1().test_properties(|(n, prec)| {
        let (shifted, o) = n.shl_prec_ref(T::ZERO, prec);
        let (shifted_alt, o_alt) = Float::from_float_prec(n, prec);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(shifted_alt));
        assert_eq!(o, o_alt);
    });

    unsigned_pair_gen_var_18::<T, _>().test_properties(|(u, prec)| {
        let (shifted, o) = Float::NAN.shl_prec(u, prec);
        assert!(shifted.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(Float::INFINITY.shl_prec(u, prec), (Float::INFINITY, Equal));
        assert_eq!(
            Float::NEGATIVE_INFINITY.shl_prec(u, prec),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        let (shifted, o) = Float::ZERO.shl_prec(u, prec);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (shifted, o) = Float::NEGATIVE_ZERO.shl_prec(u, prec);
        assert_eq!(
            ComparableFloat(shifted),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn shl_prec_properties_helper_signed_helper<T: PrimitiveSigned>(n: Float, i: T, prec: u64)
where
    i128: TryFrom<T>,
    i64: TryFrom<T>,
    Rational: Shl<T, Output = Rational>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    let mut mut_n = n.clone();
    let o = mut_n.shl_prec_assign(i, prec);
    assert!(mut_n.is_valid());
    let shifted = mut_n;

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (shifted_alt, o_alt) = n.shl_prec_round_ref(i, prec, rm);
            assert_eq!(
                ComparableFloatRef(&shifted_alt),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o_alt, Equal);
        }
    }

    let (shifted_alt, o_alt) = n.shl_prec_ref(i, prec);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    let (shifted_alt, o_alt) = n.clone().shl_prec(i, prec);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    if i128::from(n.get_exponent().unwrap_or(1))
        .wrapping_add(i128::exact_from(i))
        .lt_abs(&1_000_000)
    {
        let (shifted_alt, o_alt) = shl_prec_naive(n.clone(), i, prec);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    }

    if shifted.is_normal() {
        assert_eq!(shifted.get_prec(), Some(prec));
    }

    if i >= T::ZERO {
        let (shifted_alt, o_alt) = n.shl_prec_ref(i.unsigned_abs(), prec);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    } else if i != T::MIN {
        let (shifted_alt, o_alt) = n.shr_prec_ref(i.unsigned_abs(), prec);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    };

    let (shifted_2, o_2) = (-&n).shl_prec(i, prec);
    let (shifted_2_alt, o_2_alt) = n.shl_prec_ref(i, prec);
    assert_eq!(ComparableFloat(-shifted_2_alt), ComparableFloat(shifted_2));
    assert_eq!(o_2_alt.reverse(), o_2);

    if shifted.is_normal() {
        let (shifted_alt, o_alt) = n.mul_prec(Float::power_of_2(i64::exact_from(i)), prec);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    }
}

fn shl_prec_properties_helper_signed<T: PrimitiveSigned>()
where
    i128: TryFrom<T>,
    i64: TryFrom<T>,
    Rational: Shl<T, Output = Rational>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    float_signed_unsigned_triple_gen_var_1::<T, _>().test_properties(|(n, i, prec)| {
        shl_prec_properties_helper_signed_helper(n, i, prec);
    });

    float_signed_unsigned_triple_gen_var_2::<T, _>().test_properties(|(n, i, prec)| {
        shl_prec_properties_helper_signed_helper(n, i, prec);
    });

    float_unsigned_pair_gen_var_1().test_properties(|(n, prec)| {
        let (shifted, o) = n.shl_prec_ref(T::ZERO, prec);
        let (shifted_alt, o_alt) = Float::from_float_prec(n, prec);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(shifted_alt));
        assert_eq!(o, o_alt);
    });

    signed_unsigned_pair_gen_var_19::<T, _>().test_properties(|(i, prec)| {
        let (shifted, o) = Float::NAN.shl_prec(i, prec);
        assert!(shifted.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(Float::INFINITY.shl_prec(i, prec), (Float::INFINITY, Equal));
        assert_eq!(
            Float::NEGATIVE_INFINITY.shl_prec(i, prec),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        let (shifted, o) = Float::ZERO.shl_prec(i, prec);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (shifted, o) = Float::NEGATIVE_ZERO.shl_prec(i, prec);
        assert_eq!(
            ComparableFloat(shifted),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
        assert_eq!(o, Equal);
    });

    signed_unsigned_pair_gen_var_20::<T, _>().test_properties(|(i, prec)| {
        let (shifted, o) = Float::ONE.shl_prec(i, prec);
        if shifted.is_normal() {
            assert!(shifted.is_power_of_2());
            assert_eq!(o, Equal);
        }
    });
}

#[allow(clippy::needless_pass_by_value)]
fn shl_prec_rug_unsigned_helper(n: Float, u: u32, prec: u64) {
    let (shifted, o) = n.shl_prec_ref(u, prec);
    let (rug_shifted, rug_o) = rug_shl_prec_unsigned(&rug::Float::exact_from(&n), u, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_shifted)),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o, rug_o);
}

#[allow(clippy::needless_pass_by_value)]
fn shl_prec_rug_signed_helper(n: Float, i: i32, prec: u64) {
    let (shifted, o) = n.shl_prec_ref(i, prec);
    let (rug_shifted, rug_o) = rug_shl_prec_signed(&rug::Float::exact_from(&n), i, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_shifted)),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o, rug_o);
}

#[test]
fn shl_prec_properties() {
    apply_fn_to_unsigneds!(shl_prec_properties_helper_unsigned);
    apply_fn_to_signeds!(shl_prec_properties_helper_signed);

    float_unsigned_unsigned_triple_gen_var_1::<u32, _>()
        .test_properties(|(n, u, prec)| shl_prec_rug_unsigned_helper(n, u, prec));

    float_unsigned_unsigned_triple_gen_var_2::<u32, _>()
        .test_properties(|(n, u, prec)| shl_prec_rug_unsigned_helper(n, u, prec));

    float_signed_unsigned_triple_gen_var_1::<i32, _>()
        .test_properties(|(n, i, prec)| shl_prec_rug_signed_helper(n, i, prec));

    float_signed_unsigned_triple_gen_var_2::<i32, _>()
        .test_properties(|(n, i, prec)| shl_prec_rug_signed_helper(n, i, prec));
}

#[allow(clippy::needless_pass_by_value)]
fn shl_prec_round_properties_helper_unsigned_helper<T: PrimitiveUnsigned>(
    n: Float,
    u: T,
    prec: u64,
    rm: RoundingMode,
) where
    i128: TryFrom<T>,
    u64: TryFrom<T>,
    Rational: Shl<T, Output = Rational>,
{
    let mut mut_n = n.clone();
    let o = mut_n.shl_prec_round_assign(u, prec, rm);
    assert!(mut_n.is_valid());
    let shifted = mut_n;

    match (n >= 0, rm) {
        (_, Floor) | (true, Down) | (false, Up) => {
            assert_ne!(o, Greater);
        }
        (_, Ceiling) | (true, Up) | (false, Down) => {
            assert_ne!(o, Less);
        }
        (_, Exact) => assert_eq!(o, Equal),
        _ => {}
    }
    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (shifted_alt, o_alt) = n.shl_prec_round_ref(u, prec, rm);
            assert_eq!(
                ComparableFloatRef(&shifted_alt),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o_alt, Equal);
        }
    }

    let (shifted_alt, o_alt) = n.shl_prec_round_ref(u, prec, rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    let (shifted_alt, o_alt) = n.clone().shl_prec_round(u, prec, rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    if i128::from(n.get_exponent().unwrap_or(1))
        .wrapping_add(i128::exact_from(u))
        .lt_abs(&1_000_000)
    {
        let (shifted_alt, o_alt) = shl_prec_round_naive(n.clone(), u, prec, rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted),
        );
        assert_eq!(o_alt, o);
    }

    if shifted.is_normal() {
        assert_eq!(shifted.get_prec(), Some(prec));
    }

    let (shifted_2, o_2) = (-&n).shl_prec_round(u, prec, rm);
    let (shifted_2_alt, o_2_alt) = n.shl_prec_round_ref(u, prec, -rm);
    assert_eq!(ComparableFloat(-shifted_2_alt), ComparableFloat(shifted_2));
    assert_eq!(o_2_alt.reverse(), o_2);

    if shifted.is_normal() {
        let (shifted_alt, o_alt) =
            n.mul_prec_round(Float::power_of_2(u64::exact_from(u)), prec, rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    }
}

fn shl_prec_round_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    i128: TryFrom<T>,
    u64: TryFrom<T>,
    Rational: Shl<T, Output = Rational>,
{
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<T>().test_properties(
        |(n, u, prec, rm)| {
            shl_prec_round_properties_helper_unsigned_helper(n, u, prec, rm);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2::<T>().test_properties(
        |(n, u, prec, rm)| {
            shl_prec_round_properties_helper_unsigned_helper(n, u, prec, rm);
        },
    );

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(n, prec, rm)| {
        let (shifted, o) = n.shl_prec_round_ref(T::ZERO, prec, rm);
        let (shifted_alt, o_alt) = Float::from_float_prec_round(n, prec, rm);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(shifted_alt));
        assert_eq!(o, o_alt);
    });

    unsigned_unsigned_rounding_mode_triple_gen_var_7::<T, _>().test_properties(|(u, prec, rm)| {
        let (shifted, o) = Float::NAN.shl_prec_round(u, prec, rm);
        assert!(shifted.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(
            Float::INFINITY.shl_prec_round(u, prec, rm),
            (Float::INFINITY, Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.shl_prec_round(u, prec, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        let (shifted, o) = Float::ZERO.shl_prec_round(u, prec, rm);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (shifted, o) = Float::NEGATIVE_ZERO.shl_prec_round(u, prec, rm);
        assert_eq!(
            ComparableFloat(shifted),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn shl_prec_round_properties_helper_signed_helper<T: PrimitiveSigned>(
    n: Float,
    i: T,
    prec: u64,
    rm: RoundingMode,
) where
    i128: TryFrom<T>,
    i64: TryFrom<T>,
    Rational: Shl<T, Output = Rational>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    let mut mut_n = n.clone();
    let o = mut_n.shl_prec_round_assign(i, prec, rm);
    assert!(mut_n.is_valid());
    let shifted = mut_n;

    match (n >= 0, rm) {
        (_, Floor) | (true, Down) | (false, Up) => {
            assert_ne!(o, Greater);
        }
        (_, Ceiling) | (true, Up) | (false, Down) => {
            assert_ne!(o, Less);
        }
        (_, Exact) => assert_eq!(o, Equal),
        _ => {}
    }
    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (shifted_alt, o_alt) = n.shl_prec_round_ref(i, prec, rm);
            assert_eq!(
                ComparableFloatRef(&shifted_alt),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o_alt, Equal);
        }
    }

    let (shifted_alt, o_alt) = n.shl_prec_round_ref(i, prec, rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    let (shifted_alt, o_alt) = n.clone().shl_prec_round(i, prec, rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    if i128::from(n.get_exponent().unwrap_or(1))
        .wrapping_add(i128::exact_from(i))
        .lt_abs(&1_000_000)
    {
        let (shifted_alt, o_alt) = shl_prec_round_naive(n.clone(), i, prec, rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    }

    if shifted.is_normal() {
        assert_eq!(shifted.get_prec(), Some(prec));
    }

    if i >= T::ZERO {
        let (shifted_alt, o_alt) = n.shl_prec_round_ref(i.unsigned_abs(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    } else if i != T::MIN {
        let (shifted_alt, o_alt) = n.shr_prec_round_ref(i.unsigned_abs(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    };

    let (shifted_2, o_2) = (-&n).shl_prec_round(i, prec, rm);
    let (shifted_2_alt, o_2_alt) = n.shl_prec_round_ref(i, prec, -rm);
    assert_eq!(ComparableFloat(-shifted_2_alt), ComparableFloat(shifted_2));
    assert_eq!(o_2_alt.reverse(), o_2);

    if shifted.is_normal() {
        let (shifted_alt, o_alt) =
            n.mul_prec_round(Float::power_of_2(i64::exact_from(i)), prec, rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    }
}

fn shl_prec_round_properties_helper_signed<T: PrimitiveSigned>()
where
    i128: TryFrom<T>,
    i64: TryFrom<T>,
    Rational: Shl<T, Output = Rational>,
    <T as UnsignedAbs>::Output: PrimitiveUnsigned,
{
    float_signed_unsigned_rounding_mode_quadruple_gen_var_1::<T>().test_properties(
        |(n, i, prec, rm)| {
            shl_prec_round_properties_helper_signed_helper(n, i, prec, rm);
        },
    );

    float_signed_unsigned_rounding_mode_quadruple_gen_var_2::<T>().test_properties(
        |(n, i, prec, rm)| {
            shl_prec_round_properties_helper_signed_helper(n, i, prec, rm);
        },
    );

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(n, prec, rm)| {
        let (shifted, o) = n.shl_prec_round_ref(T::ZERO, prec, rm);
        let (shifted_alt, o_alt) = Float::from_float_prec_round(n, prec, rm);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(shifted_alt));
        assert_eq!(o, o_alt);
    });

    signed_unsigned_rounding_mode_triple_gen_var_7::<T, _>().test_properties(|(i, prec, rm)| {
        let (shifted, o) = Float::NAN.shl_prec_round(i, prec, rm);
        assert!(shifted.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(
            Float::INFINITY.shl_prec_round(i, prec, rm),
            (Float::INFINITY, Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.shl_prec_round(i, prec, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        let (shifted, o) = Float::ZERO.shl_prec_round(i, prec, rm);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (shifted, o) = Float::NEGATIVE_ZERO.shl_prec_round(i, prec, rm);
        assert_eq!(
            ComparableFloat(shifted),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
        assert_eq!(o, Equal);
    });

    signed_unsigned_rounding_mode_triple_gen_var_8::<T, _>().test_properties(|(i, prec, rm)| {
        let (shifted, o) = Float::ONE.shl_prec_round(i, prec, rm);
        assert!(shifted.is_power_of_2());
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn shl_prec_round_rug_unsigned_helper(n: Float, u: u32, prec: u64, rm: RoundingMode) {
    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (shifted, o) = n.shl_prec_round_ref(u, prec, rm);
        let (rug_shifted, rug_o) =
            rug_shl_prec_round_unsigned(&rug::Float::exact_from(&n), u, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_shifted)),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o, rug_o);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn shl_prec_round_rug_signed_helper(n: Float, i: i32, prec: u64, rm: RoundingMode) {
    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (shifted, o) = n.shl_prec_round_ref(i, prec, rm);
        let (rug_shifted, rug_o) =
            rug_shl_prec_round_signed(&rug::Float::exact_from(&n), i, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_shifted)),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o, rug_o);
    }
}

#[test]
fn shl_prec_round_properties() {
    apply_fn_to_unsigneds!(shl_prec_round_properties_helper_unsigned);
    apply_fn_to_signeds!(shl_prec_round_properties_helper_signed);

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_1::<u32>()
        .test_properties(|(n, u, prec, rm)| shl_prec_round_rug_unsigned_helper(n, u, prec, rm));

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_2::<u32>()
        .test_properties(|(n, u, prec, rm)| shl_prec_round_rug_unsigned_helper(n, u, prec, rm));

    float_signed_unsigned_rounding_mode_quadruple_gen_var_1::<i32>()
        .test_properties(|(n, i, prec, rm)| shl_prec_round_rug_signed_helper(n, i, prec, rm));

    float_signed_unsigned_rounding_mode_quadruple_gen_var_2::<i32>()
        .test_properties(|(n, i, prec, rm)| shl_prec_round_rug_signed_helper(n, i, prec, rm));
}

#[allow(clippy::needless_pass_by_value)]
fn shl_round_properties_helper_unsigned_helper<T: PrimitiveUnsigned>(
    n: Float,
    u: T,
    rm: RoundingMode,
) where
    for<'a> &'a Integer: Shl<T, Output = Integer>,
    Float: ShlRound<T, Output = Float> + ShlRoundAssign<T> + ShrRound<T, Output = Float>,
    Rational: Shl<T, Output = Rational>,
    for<'a> &'a Float: ShlRound<T, Output = Float>,
    u64: TryFrom<T>,
    i128: TryFrom<T>,
{
    let mut mut_n = n.clone();
    let o = mut_n.shl_round_assign(u, rm);
    assert!(mut_n.is_valid());
    let shifted = mut_n;

    match (n >= 0, rm) {
        (_, Floor) | (true, Down) | (false, Up) => {
            assert_ne!(o, Greater);
        }
        (_, Ceiling) | (true, Up) | (false, Down) => {
            assert_ne!(o, Less);
        }
        (_, Exact) => assert_eq!(o, Equal),
        _ => {}
    }
    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (shifted_alt, o_alt) = (&n).shl_round(u, rm);
            assert_eq!(
                ComparableFloatRef(&shifted_alt),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o_alt, Equal);
        }
    }

    let (shifted_alt, o_alt) = (&n).shl_round(u, rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    let (shifted_alt, o_alt) = n.clone().shl_round(u, rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    let (shifted_alt, o_alt) = n.shl_prec_round_ref(u, n.get_prec().unwrap_or(1), rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    if i128::from(n.get_exponent().unwrap_or(1))
        .wrapping_add(i128::exact_from(u))
        .lt_abs(&1_000_000)
    {
        let (shifted_alt, o_alt) = shl_round_naive(n.clone(), u, rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    }

    if shifted.is_normal() {
        assert_eq!(n.get_prec(), shifted.get_prec());
    }

    if !n.is_nan() {
        assert!((&n).shl_round(u, rm).0.ge_abs(&n));
    }

    let (shifted_2, o_2) = (-&n).shl_round(u, rm);
    let (shifted_2_alt, o_2_alt) = (&n).shl_round(u, -rm);
    assert_eq!(ComparableFloat(-shifted_2_alt), ComparableFloat(shifted_2));
    assert_eq!(o_2_alt.reverse(), o_2);

    if shifted.is_normal() {
        let (shifted_alt, o_alt) = n.mul_round(Float::power_of_2(u64::exact_from(u)), rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    }
}

fn shl_round_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    for<'a> &'a Integer: Shl<T, Output = Integer>,
    Float: ShlRound<T, Output = Float> + ShrRound<T, Output = Float> + ShlRoundAssign<T>,
    Rational: Shl<T, Output = Rational>,
    for<'a> &'a Float: ShlRound<T, Output = Float>,
    u64: TryFrom<T>,
    i128: TryFrom<T>,
{
    float_unsigned_rounding_mode_triple_gen_var_5::<T>().test_properties(|(n, u, rm)| {
        shl_round_properties_helper_unsigned_helper(n, u, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_6::<T>().test_properties(|(n, u, rm)| {
        shl_round_properties_helper_unsigned_helper(n, u, rm);
    });

    float_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        let (shifted, o) = (&n).shl_round(T::ZERO, rm);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(n));
        assert_eq!(o, Equal);
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(u, rm)| {
        let (shifted, o) = Float::NAN.shl_round(u, rm);
        assert!(shifted.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(Float::INFINITY.shl_round(u, rm), (Float::INFINITY, Equal));
        assert_eq!(
            Float::NEGATIVE_INFINITY.shl_round(u, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        let (shifted, o) = Float::ZERO.shl_round(u, rm);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (shifted, o) = Float::NEGATIVE_ZERO.shl_round(u, rm);
        assert_eq!(
            ComparableFloat(shifted),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
        assert_eq!(o, Equal);
    });

    unsigned_rounding_mode_pair_gen_var_5::<T>().test_properties(|(u, rm)| {
        let (shifted, o) = Float::ONE.shl_round(u, rm);
        assert!(shifted.is_power_of_2());
        assert_eq!(o, Equal);
    });

    integer_unsigned_rounding_mode_triple_gen_var_5::<T>().test_properties(|(n, u, rm)| {
        let (shifted, o) = Float::exact_from(&n).shl_round(u, rm);
        assert_eq!(&n << u, shifted);
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn shl_round_properties_helper_signed_helper<T: PrimitiveSigned>(n: Float, i: T, rm: RoundingMode)
where
    for<'a> &'a Integer: Shl<T, Output = Integer>,
    Float: ShlRound<T, Output = Float> + ShlRoundAssign<T> + ShrRound<T, Output = Float>,
    Rational: Shl<T, Output = Rational>,
    for<'a> &'a Float: ShlRound<T, Output = Float>
        + ShlRound<<T as UnsignedAbs>::Output, Output = Float>
        + ShrRound<<T as UnsignedAbs>::Output, Output = Float>
        + ShrRound<T, Output = Float>,
    i64: TryFrom<T>,
    i128: TryFrom<T>,
{
    let mut mut_n = n.clone();
    let o = mut_n.shl_round_assign(i, rm);
    assert!(mut_n.is_valid());
    let shifted = mut_n;

    match (n >= 0, rm) {
        (_, Floor) | (true, Down) | (false, Up) => {
            assert_ne!(o, Greater);
        }
        (_, Ceiling) | (true, Up) | (false, Down) => {
            assert_ne!(o, Less);
        }
        (_, Exact) => assert_eq!(o, Equal),
        _ => {}
    }
    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (shifted_alt, o_alt) = (&n).shl_round(i, rm);
            assert_eq!(
                ComparableFloatRef(&shifted_alt),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o_alt, Equal);
        }
    }

    let (shifted_alt, o_alt) = (&n).shl_round(i, rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    let (shifted_alt, o_alt) = n.clone().shl_round(i, rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    let (shifted_alt, o_alt) = n.shl_prec_round_ref(i, n.get_prec().unwrap_or(1), rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    if i128::from(n.get_exponent().unwrap_or(1))
        .wrapping_add(i128::exact_from(i))
        .lt_abs(&1_000_000)
    {
        let (shifted_alt, o_alt) = shl_round_naive(n.clone(), i, rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    }

    if shifted.is_normal() {
        assert_eq!(n.get_prec(), shifted.get_prec());
    }

    if !n.is_nan() {
        if i >= T::ZERO {
            assert!((&n).shl_round(i, rm).0.ge_abs(&n));
        } else {
            assert!((&n).shl_round(i, rm).0.le_abs(&n));
        }
    }

    if i >= T::ZERO {
        let (shifted_alt, o_alt) = (&n).shl_round(i.unsigned_abs(), rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    } else if i != T::MIN {
        let (shifted_alt, o_alt) = (&n).shr_round(i.unsigned_abs(), rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    };

    let (shifted_2, o_2) = (-&n).shl_round(i, rm);
    let (shifted_2_alt, o_2_alt) = (&n).shl_round(i, -rm);
    assert_eq!(ComparableFloat(-shifted_2_alt), ComparableFloat(shifted_2));
    assert_eq!(o_2_alt.reverse(), o_2);

    if shifted.is_normal() {
        let (shifted_alt, o_alt) = n.mul_round(Float::power_of_2(i64::exact_from(i)), rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    }
}

fn shl_round_properties_helper_signed<T: PrimitiveSigned>()
where
    for<'a> &'a Integer: Shl<T, Output = Integer>,
    Float: ShlRound<T, Output = Float> + ShlRoundAssign<T> + ShrRound<T, Output = Float>,
    Rational: Shl<T, Output = Rational>,
    for<'a> &'a Float: ShlRound<T, Output = Float>
        + ShlRound<<T as UnsignedAbs>::Output, Output = Float>
        + ShrRound<<T as UnsignedAbs>::Output, Output = Float>
        + ShrRound<T, Output = Float>,
    i64: TryFrom<T>,
    i128: TryFrom<T>,
{
    float_signed_rounding_mode_triple_gen_var_1::<T>().test_properties(|(n, i, rm)| {
        shl_round_properties_helper_signed_helper(n, i, rm);
    });

    float_signed_rounding_mode_triple_gen_var_2::<T>().test_properties(|(n, i, rm)| {
        shl_round_properties_helper_signed_helper(n, i, rm);
    });

    float_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        let (shifted, o) = (&n).shl_round(T::ZERO, rm);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(n));
        assert_eq!(o, Equal);
    });

    signed_rounding_mode_pair_gen::<T>().test_properties(|(i, rm)| {
        let (shifted, o) = Float::NAN.shl_round(i, rm);
        assert!(shifted.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(Float::INFINITY.shl_round(i, rm), (Float::INFINITY, Equal));
        assert_eq!(
            Float::NEGATIVE_INFINITY.shl_round(i, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        let (shifted, o) = Float::ZERO.shl_round(i, rm);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (shifted, o) = Float::NEGATIVE_ZERO.shl_round(i, rm);
        assert_eq!(
            ComparableFloat(shifted),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
        assert_eq!(o, Equal);
    });

    signed_rounding_mode_pair_gen_var_5::<T>().test_properties(|(i, rm)| {
        let (shifted, o) = Float::ONE.shl_round(i, rm);
        assert!(shifted.is_power_of_2());
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn shl_round_rug_unsigned_helper(n: Float, u: u32, rm: RoundingMode) {
    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (shifted, o) = (&n).shl_round(u, rm);
        let (rug_shifted, rug_o) = rug_shl_round_unsigned(&rug::Float::exact_from(&n), u, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_shifted)),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o, rug_o);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn shl_round_rug_signed_helper(n: Float, i: i32, rm: RoundingMode) {
    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (shifted, o) = (&n).shl_round(i, rm);
        let (rug_shifted, rug_o) = rug_shl_round_signed(&rug::Float::exact_from(&n), i, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_shifted)),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o, rug_o);
    }
}

#[test]
fn shl_round_properties() {
    apply_fn_to_unsigneds!(shl_round_properties_helper_unsigned);
    apply_fn_to_signeds!(shl_round_properties_helper_signed);

    float_unsigned_rounding_mode_triple_gen_var_5::<u32>()
        .test_properties(|(n, u, rm)| shl_round_rug_unsigned_helper(n, u, rm));

    float_unsigned_rounding_mode_triple_gen_var_6::<u32>()
        .test_properties(|(n, u, rm)| shl_round_rug_unsigned_helper(n, u, rm));

    float_signed_rounding_mode_triple_gen_var_1::<i32>()
        .test_properties(|(n, i, rm)| shl_round_rug_signed_helper(n, i, rm));

    float_signed_rounding_mode_triple_gen_var_2::<i32>()
        .test_properties(|(n, i, rm)| shl_round_rug_signed_helper(n, i, rm));
}
