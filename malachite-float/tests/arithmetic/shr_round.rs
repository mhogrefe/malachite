// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    IsPowerOf2, ShlRound, ShrRound, ShrRoundAssign, UnsignedAbs,
};
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::test_util::generators::{
    signed_rounding_mode_pair_gen, signed_rounding_mode_pair_gen_var_5,
    unsigned_rounding_mode_pair_gen, unsigned_rounding_mode_pair_gen_var_5,
};
use malachite_float::test_util::arithmetic::shr_round::shr_round_naive;
use malachite_float::test_util::arithmetic::shr_round::{
    rug_shr_round_signed, rug_shr_round_unsigned,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_rounding_mode_pair_gen, float_signed_rounding_mode_triple_gen_var_4,
    float_signed_rounding_mode_triple_gen_var_5, float_unsigned_rounding_mode_triple_gen_var_8,
    float_unsigned_rounding_mode_triple_gen_var_9,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::integer::Integer;
use malachite_q::Rational;
use std::cmp::Ordering::{self, *};
use std::ops::Shr;
use std::panic::catch_unwind;

fn test_shr_round_unsigned_helper<
    T: PrimitiveUnsigned,
    F: Fn(Float, T, RoundingMode, Float, Ordering),
>(
    f: F,
) where
    Float: ShrRoundAssign<T> + ShrRound<T, Output = Float>,
    Rational: Shr<T, Output = Rational>,
    for<'a> &'a Float: ShrRound<T, Output = Float>,
{
    let test = |s, s_hex, v: u64, out: &str, out_hex: &str| {
        for rm in exhaustive_rounding_modes() {
            let x = parse_hex_string(s_hex);
            assert_eq!(x.to_string(), s);
            let v = T::exact_from(v);

            let mut n = x.clone();
            let o = n.shr_round_assign(v, rm);
            assert_eq!(n.to_string(), out);
            assert_eq!(to_hex_string(&n), out_hex);
            assert!(n.is_valid());
            assert_eq!(o, Equal);

            let (n, o) = x.clone().shr_round(v, rm);
            assert_eq!(n.to_string(), out);
            assert_eq!(to_hex_string(&n), out_hex);
            assert!(n.is_valid());
            assert_eq!(o, Equal);

            let (n, o) = (&x).shr_round(v, rm);
            assert_eq!(n.to_string(), out);
            assert_eq!(to_hex_string(&n), out_hex);
            assert!(n.is_valid());
            assert_eq!(o, Equal);

            let (n, o) = shr_round_naive(x.clone(), v, rm);
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
    test("123.0", "0x7b.0#7", 1, "61.5", "0x3d.8#7");
    test("123.0", "0x7b.0#7", 10, "0.12", "0x0.1ec#7");
    test("123.0", "0x7b.0#7", 100, "9.7e-29", "0x7.bE-24#7");

    test("-123.0", "-0x7b.0#7", 0, "-123.0", "-0x7b.0#7");
    test("-123.0", "-0x7b.0#7", 1, "-61.5", "-0x3d.8#7");
    test("-123.0", "-0x7b.0#7", 10, "-0.12", "-0x0.1ec#7");
    test("-123.0", "-0x7b.0#7", 100, "-9.7e-29", "-0x7.bE-24#7");

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
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "0.0030679615757712823",
        "0x0.00c90fdaa22168c0#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
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
        "-1.5707963267948966",
        "-0x1.921fb54442d18#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-0.0030679615757712823",
        "-0x0.00c90fdaa22168c0#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        "-2.4782796245465248e-30",
        "-0x3.243f6a8885a30E-25#53",
    );

    let test_extreme =
        |s, s_hex, v: i32, rm: RoundingMode, out: &str, out_hex: &str, out_o: Ordering| {
            let v = u64::exact_from(v);
            if T::convertible_from(v) {
                let x = parse_hex_string(s_hex);
                assert_eq!(x.to_string(), s);
                let v = T::exact_from(v);

                let mut n = x.clone();
                let o = n.shr_round_assign(v, rm);
                assert_eq!(n.to_string(), out);
                assert_eq!(to_hex_string(&n), out_hex);
                assert!(n.is_valid());
                assert_eq!(o, out_o);

                let (n, o) = x.clone().shr_round(v, rm);
                assert_eq!(n.to_string(), out);
                assert_eq!(to_hex_string(&n), out_hex);
                assert!(n.is_valid());
                assert_eq!(o, out_o);

                let (n, o) = (&x).shr_round(v, rm);
                assert_eq!(n.to_string(), out);
                assert_eq!(to_hex_string(&n), out_hex);
                assert!(n.is_valid());
                assert_eq!(o, out_o);

                let (n, o) = shr_round_naive(x.clone(), v, rm);
                assert_eq!(n.to_string(), out);
                assert_eq!(to_hex_string(&n), out_hex);
                assert_eq!(o, out_o);

                f(x, v, rm, n, out_o);
            }
        };
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Floor,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Down,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Nearest,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Exact,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );

    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );

    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#2",
        Greater,
    );
    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Up,
        "too_small",
        "0x1.0E-268435456#2",
        Greater,
    );
    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Nearest,
        "too_small",
        "0x1.0E-268435456#2",
        Greater,
    );

    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Floor,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Ceiling,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Down,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Up,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Nearest,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Exact,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );

    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Floor,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Up,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
    );

    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Floor,
        "-too_small",
        "-0x1.0E-268435456#2",
        Less,
    );
    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Up,
        "-too_small",
        "-0x1.0E-268435456#2",
        Less,
    );
    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Nearest,
        "-too_small",
        "-0x1.0E-268435456#2",
        Less,
    );
}

#[test]
fn test_shr_round_unsigned() {
    test_shr_round_unsigned_helper::<u8, _>(|_, _, _, _, _| {});
    test_shr_round_unsigned_helper::<u16, _>(|_, _, _, _, _| {});
    test_shr_round_unsigned_helper::<u32, _>(|x, v, rm, shifted, o_out| {
        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (n, o) = rug_shr_round_unsigned(&rug::Float::exact_from(&x), v, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&n)),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o, o_out);
        }
    });
    test_shr_round_unsigned_helper::<u64, _>(|_, _, _, _, _| {});
    test_shr_round_unsigned_helper::<u128, _>(|_, _, _, _, _| {});
    test_shr_round_unsigned_helper::<usize, _>(|_, _, _, _, _| {});
}

fn test_shr_round_signed_helper<
    T: PrimitiveSigned,
    F: Fn(Float, T, RoundingMode, Float, Ordering),
>(
    f: F,
) where
    Float: ShrRoundAssign<T> + ShrRound<T, Output = Float>,
    Rational: Shr<T, Output = Rational>,
    for<'a> &'a Float: ShrRound<T, Output = Float>,
{
    let test = |s, s_hex, v: i64, out: &str, out_hex: &str| {
        for rm in exhaustive_rounding_modes() {
            let x = parse_hex_string(s_hex);
            assert_eq!(x.to_string(), s);
            let v = T::exact_from(v);

            let mut n = x.clone();
            let o = n.shr_round_assign(v, rm);
            assert_eq!(n.to_string(), out);
            assert_eq!(to_hex_string(&n), out_hex);
            assert!(n.is_valid());
            assert_eq!(o, Equal);

            let (n, o) = x.clone().shr_round(v, rm);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());
            assert_eq!(o, Equal);

            let (n, o) = (&x).shr_round(v, rm);
            assert_eq!(n.to_string(), out);
            assert!(n.is_valid());
            assert_eq!(o, Equal);

            let (n, o) = shr_round_naive(x.clone(), v, rm);
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
    test("123.0", "0x7b.0#7", 1, "61.5", "0x3d.8#7");
    test("123.0", "0x7b.0#7", 10, "0.12", "0x0.1ec#7");
    test("123.0", "0x7b.0#7", 100, "9.7e-29", "0x7.bE-24#7");
    test("123.0", "0x7b.0#7", -1, "246.0", "0xf6.0#7");
    test("123.0", "0x7b.0#7", -10, "1.26e5", "0x1.ecE+4#7");
    test("123.0", "0x7b.0#7", -100, "1.56e32", "0x7.bE+26#7");

    test("-123.0", "-0x7b.0#7", 0, "-123.0", "-0x7b.0#7");
    test("-123.0", "-0x7b.0#7", 1, "-61.5", "-0x3d.8#7");
    test("-123.0", "-0x7b.0#7", 10, "-0.12", "-0x0.1ec#7");
    test("-123.0", "-0x7b.0#7", 100, "-9.7e-29", "-0x7.bE-24#7");
    test("-123.0", "-0x7b.0#7", -1, "-246.0", "-0xf6.0#7");
    test("-123.0", "-0x7b.0#7", -10, "-1.26e5", "-0x1.ecE+4#7");
    test("-123.0", "-0x7b.0#7", -100, "-1.56e32", "-0x7.bE+26#7");

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
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "0.0030679615757712823",
        "0x0.00c90fdaa22168c0#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        "2.4782796245465248e-30",
        "0x3.243f6a8885a30E-25#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -1,
        "6.283185307179586",
        "0x6.487ed5110b460#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -10,
        "3216.9908772759482",
        "0xc90.fdaa22168c0#53",
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        -100,
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
        "-1.5707963267948966",
        "-0x1.921fb54442d18#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "-0.0030679615757712823",
        "-0x0.00c90fdaa22168c0#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        100,
        "-2.4782796245465248e-30",
        "-0x3.243f6a8885a30E-25#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -1,
        "-6.283185307179586",
        "-0x6.487ed5110b460#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -10,
        "-3216.9908772759482",
        "-0xc90.fdaa22168c0#53",
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        -100,
        "-3.9824418129956972e30",
        "-0x3.243f6a8885a30E+25#53",
    );

    let test_extreme =
        |s, s_hex, v: i32, rm: RoundingMode, out: &str, out_hex: &str, out_o: Ordering| {
            let v = i64::from(v);
            if T::convertible_from(v) {
                let x = parse_hex_string(s_hex);
                assert_eq!(x.to_string(), s);
                let v = T::exact_from(v);

                let mut n = x.clone();
                let o = n.shr_round_assign(v, rm);
                assert_eq!(n.to_string(), out);
                assert_eq!(to_hex_string(&n), out_hex);
                assert!(n.is_valid());
                assert_eq!(o, out_o);

                let (n, o) = x.clone().shr_round(v, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());
                assert_eq!(o, out_o);

                let (n, o) = (&x).shr_round(v, rm);
                assert_eq!(n.to_string(), out);
                assert!(n.is_valid());
                assert_eq!(o, out_o);

                let (n, o) = shr_round_naive(x.clone(), v, rm);
                assert_eq!(n.to_string(), out);
                assert_eq!(to_hex_string(&n), out_hex);
                assert_eq!(o, out_o);

                f(x, v, rm, n, out_o);
            }
        };
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT + 1,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT + 1,
        Ceiling,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT + 1,
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT + 1,
        Up,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT + 1,
        Nearest,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT + 1,
        Exact,
        "too_big",
        "0x4.0E+268435455#1",
        Equal,
    );

    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT,
        Floor,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT,
        Down,
        "too_big",
        "0x4.0E+268435455#1",
        Less,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MIN_EXPONENT,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );

    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#2",
        Greater,
    );
    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Up,
        "too_small",
        "0x1.0E-268435456#2",
        Greater,
    );
    test_extreme(
        "1.5",
        "0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Nearest,
        "too_small",
        "0x1.0E-268435456#2",
        Greater,
    );

    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT + 1,
        Floor,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT + 1,
        Ceiling,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT + 1,
        Down,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT + 1,
        Up,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT + 1,
        Nearest,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT + 1,
        Exact,
        "-too_big",
        "-0x4.0E+268435455#1",
        Equal,
    );

    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT,
        Ceiling,
        "-too_big",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT,
        Down,
        "-too_big",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MIN_EXPONENT,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );

    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Floor,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Down,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Nearest,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Exact,
        "too_small",
        "0x1.0E-268435456#1",
        Equal,
    );

    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test_extreme(
        "1.0",
        "0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );

    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Floor,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Ceiling,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Down,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Up,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Nearest,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 1,
        Exact,
        "-too_small",
        "-0x1.0E-268435456#1",
        Equal,
    );

    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Floor,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Up,
        "-too_small",
        "-0x1.0E-268435456#1",
        Less,
    );
    test_extreme(
        "-1.0",
        "-0x1.0#1",
        Float::MAX_EXPONENT + 2,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
    );

    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Floor,
        "-too_small",
        "-0x1.0E-268435456#2",
        Less,
    );
    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Up,
        "-too_small",
        "-0x1.0E-268435456#2",
        Less,
    );
    test_extreme(
        "-1.5",
        "-0x1.8#2",
        Float::MAX_EXPONENT + 2,
        Nearest,
        "-too_small",
        "-0x1.0E-268435456#2",
        Less,
    );
}

#[test]
fn shr_round_fail() {
    assert_panic!(Float::ONE.shr_round(Float::MIN_EXPONENT, Exact));
    assert_panic!(Float::ONE.shr_round(Float::MAX_EXPONENT + 2, Exact));
    assert_panic!(Float::NEGATIVE_ONE.shr_round(Float::MIN_EXPONENT, Exact));
    assert_panic!(Float::NEGATIVE_ONE.shr_round(Float::MAX_EXPONENT + 2, Exact));
}

#[test]
fn test_shr_round_signed() {
    test_shr_round_signed_helper::<i8, _>(|_, _, _, _, _| {});
    test_shr_round_signed_helper::<i16, _>(|_, _, _, _, _| {});
    test_shr_round_signed_helper::<i32, _>(|x, v, rm, shifted, o_out| {
        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (n, o) = rug_shr_round_signed(&rug::Float::exact_from(&x), v, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&n)),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o, o_out);
        }
    });
    test_shr_round_signed_helper::<i64, _>(|_, _, _, _, _| {});
    test_shr_round_signed_helper::<i128, _>(|_, _, _, _, _| {});
    test_shr_round_signed_helper::<isize, _>(|_, _, _, _, _| {});
}

#[allow(clippy::needless_pass_by_value)]
fn shr_round_properties_helper_unsigned_helper<T: PrimitiveUnsigned>(
    n: Float,
    u: T,
    rm: RoundingMode,
) where
    for<'a> &'a Integer: Shr<T, Output = Integer>,
    Float: ShrRound<T, Output = Float> + ShrRoundAssign<T> + ShlRound<T, Output = Float>,
    Rational: Shr<T, Output = Rational>,
    for<'a> &'a Float: ShrRound<T, Output = Float>,
    u64: TryFrom<T>,
    i128: TryFrom<T>,
{
    let mut mut_n = n.clone();
    let o = mut_n.shr_round_assign(u, rm);
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
            let (shifted_alt, o_alt) = (&n).shr_round(u, rm);
            assert_eq!(
                ComparableFloatRef(&shifted_alt),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o_alt, Equal);
        }
    }

    let (shifted_alt, o_alt) = (&n).shr_round(u, rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    let (shifted_alt, o_alt) = n.clone().shr_round(u, rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    if i128::from(n.get_exponent().unwrap_or(1))
        .wrapping_sub(i128::exact_from(u))
        .lt_abs(&1_000_000)
    {
        let (shifted_alt, o_alt) = shr_round_naive(n.clone(), u, rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    }

    if i128::from(n.get_exponent().unwrap_or(1))
        .wrapping_sub(i128::exact_from(u))
        .lt_abs(&1_000_000)
    {
        let (shifted_alt, o_alt) = shr_round_naive(n.clone(), u, rm);
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
        assert!((&n).shr_round(u, rm).0.le_abs(&n));
    }

    let (shifted_2, o_2) = (-&n).shr_round(u, rm);
    let (shifted_2_alt, o_2_alt) = (&n).shr_round(u, -rm);
    assert_eq!(ComparableFloat(-shifted_2_alt), ComparableFloat(shifted_2));
    assert_eq!(o_2_alt.reverse(), o_2);

    // TODO add
    //
    // if shifted.is_normal() { assert_eq!( ComparableFloat(&n >> u), ComparableFloat(n /
    // Float::power_of_2(u64::exact_from(u))) ); }
}

fn shr_round_properties_helper_unsigned<T: PrimitiveUnsigned>()
where
    for<'a> &'a Integer: Shr<T, Output = Integer>,
    Float: ShrRound<T, Output = Float> + ShlRound<T, Output = Float> + ShrRoundAssign<T>,
    Rational: Shr<T, Output = Rational>,
    for<'a> &'a Float: ShrRound<T, Output = Float>,
    u64: TryFrom<T>,
    i128: TryFrom<T>,
{
    float_unsigned_rounding_mode_triple_gen_var_8::<T>().test_properties(|(n, u, rm)| {
        shr_round_properties_helper_unsigned_helper(n, u, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_9::<T>().test_properties(|(n, u, rm)| {
        shr_round_properties_helper_unsigned_helper(n, u, rm);
    });

    float_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        let (shifted, o) = (&n).shr_round(T::ZERO, rm);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(n));
        assert_eq!(o, Equal);
    });

    unsigned_rounding_mode_pair_gen::<T>().test_properties(|(u, rm)| {
        let (shifted, o) = Float::NAN.shr_round(u, rm);
        assert!(shifted.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(Float::INFINITY.shr_round(u, rm), (Float::INFINITY, Equal));
        assert_eq!(
            Float::NEGATIVE_INFINITY.shr_round(u, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        let (shifted, o) = Float::ZERO.shr_round(u, rm);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (shifted, o) = Float::NEGATIVE_ZERO.shr_round(u, rm);
        assert_eq!(
            ComparableFloat(shifted),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
        assert_eq!(o, Equal);
    });

    unsigned_rounding_mode_pair_gen_var_5::<T>().test_properties(|(u, rm)| {
        let (shifted, o) = Float::ONE.shr_round(u, rm);
        assert!(shifted.is_power_of_2());
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn shr_round_properties_helper_signed_helper<T: PrimitiveSigned>(n: Float, i: T, rm: RoundingMode)
where
    for<'a> &'a Integer: Shr<T, Output = Integer>,
    Float: ShrRound<T, Output = Float> + ShrRoundAssign<T> + ShlRound<T, Output = Float>,
    for<'a> &'a Float: ShrRound<T, Output = Float>
        + ShrRound<<T as UnsignedAbs>::Output, Output = Float>
        + ShlRound<<T as UnsignedAbs>::Output, Output = Float>
        + ShlRound<T, Output = Float>,
    i64: TryFrom<T>,
    i128: TryFrom<T>,
{
    let mut mut_n = n.clone();
    let o = mut_n.shr_round_assign(i, rm);
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
            let (shifted_alt, o_alt) = (&n).shr_round(i, rm);
            assert_eq!(
                ComparableFloatRef(&shifted_alt),
                ComparableFloatRef(&shifted)
            );
            assert_eq!(o_alt, Equal);
        }
    }

    let (shifted_alt, o_alt) = (&n).shr_round(i, rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    let (shifted_alt, o_alt) = n.clone().shr_round(i, rm);
    assert!(shifted_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&shifted_alt),
        ComparableFloatRef(&shifted)
    );
    assert_eq!(o_alt, o);

    if shifted.is_normal() {
        assert_eq!(n.get_prec(), shifted.get_prec());
    }

    if !n.is_nan() {
        if i >= T::ZERO {
            assert!((&n).shr_round(i, rm).0.le_abs(&n));
        } else {
            assert!((&n).shr_round(i, rm).0.ge_abs(&n));
        }
    }

    if i >= T::ZERO {
        let (shifted_alt, o_alt) = (&n).shr_round(i.unsigned_abs(), rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    } else if i != T::MIN {
        let (shifted_alt, o_alt) = (&n).shl_round(i.unsigned_abs(), rm);
        assert_eq!(
            ComparableFloatRef(&shifted_alt),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o_alt, o);
    };

    let (shifted_2, o_2) = (-&n).shr_round(i, rm);
    let (shifted_2_alt, o_2_alt) = (&n).shr_round(i, -rm);
    assert_eq!(ComparableFloat(-shifted_2_alt), ComparableFloat(shifted_2));
    assert_eq!(o_2_alt.reverse(), o_2);

    // TODO add
    //
    // if shifted.is_normal() { assert_eq!( ComparableFloat(&n >> i), ComparableFloat(n /
    // Float::power_of_2(u64::exact_from(i))) ); }
}

fn shr_round_properties_helper_signed<T: PrimitiveSigned>()
where
    for<'a> &'a Integer: Shr<T, Output = Integer>,
    Float: ShrRound<T, Output = Float> + ShrRoundAssign<T> + ShlRound<T, Output = Float>,
    for<'a> &'a Float: ShrRound<T, Output = Float>
        + ShrRound<<T as UnsignedAbs>::Output, Output = Float>
        + ShlRound<<T as UnsignedAbs>::Output, Output = Float>
        + ShlRound<T, Output = Float>,
    i64: TryFrom<T>,
    i128: TryFrom<T>,
{
    float_signed_rounding_mode_triple_gen_var_4::<T>().test_properties(|(n, i, rm)| {
        shr_round_properties_helper_signed_helper(n, i, rm);
    });

    float_signed_rounding_mode_triple_gen_var_5::<T>().test_properties(|(n, i, rm)| {
        shr_round_properties_helper_signed_helper(n, i, rm);
    });

    float_rounding_mode_pair_gen().test_properties(|(n, rm)| {
        let (shifted, o) = (&n).shr_round(T::ZERO, rm);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(n));
        assert_eq!(o, Equal);
    });

    signed_rounding_mode_pair_gen::<T>().test_properties(|(i, rm)| {
        let (shifted, o) = Float::NAN.shr_round(i, rm);
        assert!(shifted.is_nan());
        assert_eq!(o, Equal);
        assert_eq!(Float::INFINITY.shr_round(i, rm), (Float::INFINITY, Equal));
        assert_eq!(
            Float::NEGATIVE_INFINITY.shr_round(i, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        let (shifted, o) = Float::ZERO.shr_round(i, rm);
        assert_eq!(ComparableFloat(shifted), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (shifted, o) = Float::NEGATIVE_ZERO.shr_round(i, rm);
        assert_eq!(
            ComparableFloat(shifted),
            ComparableFloat(Float::NEGATIVE_ZERO)
        );
        assert_eq!(o, Equal);
    });

    signed_rounding_mode_pair_gen_var_5::<T>().test_properties(|(i, rm)| {
        let (shifted, o) = Float::ONE.shr_round(i, rm);
        assert!(shifted.is_power_of_2());
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn shr_round_rug_unsigned_helper(n: Float, u: u32, rm: RoundingMode) {
    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (shifted, o) = (&n).shr_round(u, rm);
        let (rug_shifted, rug_o) = rug_shr_round_unsigned(&rug::Float::exact_from(&n), u, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_shifted)),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o, rug_o);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn shr_round_rug_signed_helper(n: Float, i: i32, rm: RoundingMode) {
    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (shifted, o) = (&n).shr_round(i, rm);
        let (rug_shifted, rug_o) = rug_shr_round_signed(&rug::Float::exact_from(&n), i, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_shifted)),
            ComparableFloatRef(&shifted)
        );
        assert_eq!(o, rug_o);
    }
}

#[test]
fn shr_round_properties() {
    apply_fn_to_unsigneds!(shr_round_properties_helper_unsigned);
    apply_fn_to_signeds!(shr_round_properties_helper_signed);

    float_unsigned_rounding_mode_triple_gen_var_8::<u32>()
        .test_properties(|(n, u, rm)| shr_round_rug_unsigned_helper(n, u, rm));

    float_unsigned_rounding_mode_triple_gen_var_9::<u32>()
        .test_properties(|(n, u, rm)| shr_round_rug_unsigned_helper(n, u, rm));

    float_signed_rounding_mode_triple_gen_var_4::<i32>()
        .test_properties(|(n, i, rm)| shr_round_rug_signed_helper(n, i, rm));

    float_signed_rounding_mode_triple_gen_var_5::<i32>()
        .test_properties(|(n, i, rm)| shr_round_rug_signed_helper(n, i, rm));
}
