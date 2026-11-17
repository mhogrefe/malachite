// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::exponential::traits::{TreeTMinus1, TreeTMinus1Assign};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::exponential::lambert_w_minus_1::{
    primitive_float_lambert_w_minus_1, primitive_float_lambert_w_minus_1_rational,
};
use malachite_float::float::exponential::tree_t_minus_1::{
    primitive_float_tree_t_minus_1, primitive_float_tree_t_minus_1_rational,
};
use malachite_float::test_util::big_exponent_float::{
    assert_lambert_w_correctly_rounded, float_to_big_exponent_float,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, exponent_in_gate, parse_hex_string, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;

// Whether a result did not overflow or underflow, so that it is the correct rounding of the exact
// value with an unbounded exponent.
fn result_in_range(x: &Float, o: Ordering) -> bool {
    x.get_exponent().map_or_else(
        || !x.is_infinite() && (!x.is_zero() || o == Equal),
        |e| e > Float::MIN_EXPONENT + 1 && e < Float::MAX_EXPONENT - 1,
    )
}

#[test]
fn test_tree_t_minus_1() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let tree_t_minus_1 = x.clone().tree_t_minus_1();
        assert!(tree_t_minus_1.is_valid());

        assert_eq!(tree_t_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&tree_t_minus_1), out_hex);

        let tree_t_minus_1_alt = (&x).tree_t_minus_1();
        assert!(tree_t_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&tree_t_minus_1),
            ComparableFloatRef(&tree_t_minus_1_alt)
        );

        let mut tree_t_minus_1_alt = x.clone();
        tree_t_minus_1_alt.tree_t_minus_1_assign();
        assert!(tree_t_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&tree_t_minus_1),
            ComparableFloatRef(&tree_t_minus_1_alt)
        );
    };
    test("-1.0", "-0x1.0#1", "NaN", "NaN");
    test("-0.50", "-0x0.8#1", "NaN", "NaN");
    test("-2.0", "-0x2.0#1", "NaN", "NaN");
    test("-2.7182818284590451", "-0x2.b7e151628aed2#53", "NaN", "NaN");
    test("-10.0", "-0xa.0#3", "NaN", "NaN");
    test("-1.00000000e10", "-0x2.540be4E+8#24", "NaN", "NaN");
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        "NaN",
        "NaN",
    );
    test(
        "-1.0000000000000000e100",
        "-0x1.249ad2594c37dE+83#53",
        "NaN",
        "NaN",
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84#49",
        "NaN",
        "NaN",
    );
    test("-123.45600000000000", "-0x7b.74bc6a7ef9dc#53", "NaN", "NaN");
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        "3.5771520639572971",
        "0x3.93c03cd783f74#52",
    );
    test("0.25", "0x0.4#1", "2.0", "0x2.0#1");
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        "1.7813370234216277",
        "0x1.c805b402bff7f#53",
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        "14.163600815810183",
        "0xe.29e1be3980ef8#53",
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        "235.7211588756854",
        "0xeb.b89dde3a4a0#49",
    );
    test("0.36787944117144233", "0x0.5e2d58d8b3bce#50", "NaN", "NaN");
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        "1.0000000153042543",
        "0x1.00000041bb34a#53",
    );
    test("0.38", "0x0.6#2", "NaN", "NaN");
    test("1.0", "0x1.0#1", "NaN", "NaN");
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        "NaN",
        "NaN",
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        "1.5121345516578424738967396780705",
        "0x1.831b3ffe85a576660f8350f58#100",
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        "9.1180064704027401212583371820468142742704349737639473102927349",
        "0x9.1e35ac0b189dfe008de388a018f6274a806fed68ff921f63d#200",
    );
    test("-2.4e-323228497", "-0x1.0E-268435456#1", "NaN", "NaN");
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        "5.4e8",
        "0x2.0E+7#1",
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        "NaN",
        "NaN",
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        "744261138.38279554153782298947663",
        "0x2c5c8612.61fee37bf5a5ad6b8c#100",
    );
    test("-1.0e323228496", "-0x4.0E+268435455#1", "NaN", "NaN");
    test(
        "-2.0985787164673876924043581168822e323228496",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        "NaN",
        "NaN",
    );
    test("-4.0e-323228249", "-0x1.8E-268435250#2", "NaN", "NaN");
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        "8.1e8",
        "0x3.0E+7#2",
    );
}

#[test]
fn test_tree_t_minus_1_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (tree_t_minus_1, o) = x.clone().tree_t_minus_1_prec(prec);
        assert!(tree_t_minus_1.is_valid());

        assert_eq!(tree_t_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&tree_t_minus_1), out_hex);
        assert_eq!(o, o_out);

        let (tree_t_minus_1_alt, o_alt) = x.tree_t_minus_1_prec_ref(prec);
        assert!(tree_t_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&tree_t_minus_1),
            ComparableFloatRef(&tree_t_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut tree_t_minus_1_alt = x.clone();
        let o_alt = tree_t_minus_1_alt.tree_t_minus_1_prec_assign(prec);
        assert!(tree_t_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&tree_t_minus_1),
            ComparableFloatRef(&tree_t_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    test("-1.0", "-0x1.0#1", 10, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, "NaN", "NaN", Equal);
    test(
        "-2.7182818284590451",
        "-0x2.b7e151628aed2#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test("-10.0", "-0xa.0#3", 10, "NaN", "NaN", Equal);
    test(
        "-1.00000000e10",
        "-0x2.540be4E+8#24",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e100",
        "-0x1.249ad2594c37dE+83#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84#49",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-123.45600000000000",
        "-0x7b.74bc6a7ef9dc#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        1,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        "3.5781",
        "0x3.94#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        100,
        "3.5771520639572971413585139898516",
        "0x3.93c03cd783f7414b1d17f26dc#100",
        Less,
    );
    test("0.25", "0x0.4#1", 1, "2.0", "0x2.0#1", Less);
    test("0.25", "0x0.4#1", 10, "2.1523", "0x2.27#10", Less);
    test(
        "0.25",
        "0x0.4#1",
        100,
        "2.1532923641103496491690991500941",
        "0x2.273e2b1a949a11317152b2a04#100",
        Greater,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        1,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        10,
        "1.7812",
        "0x1.c80#10",
        Less,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        100,
        "1.7813370234216276963458442513146",
        "0x1.c805b402bff7effb309635158#100",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        1,
        "16.0",
        "0x1.0E+1#1",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        "14.156",
        "0xe.28#10",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        100,
        "14.163600815810182921092165214014",
        "0xe.29e1be3980ef8f43f0940b8d#100",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        1,
        "2.6e2",
        "0x1.0E+2#1",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        "235.75",
        "0xeb.c#10",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        100,
        "235.72115887568531364038353335125",
        "0xeb.b89dde3a49de85be86b8a0b#100",
        Greater,
    );
    test(
        "0.36787944117144233",
        "0x0.5e2d58d8b3bce#50",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        1,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        10,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        100,
        "1.0000000153042542846419691855759",
        "0x1.00000041bb349e4bc364e6606#100",
        Greater,
    );
    test("0.38", "0x0.6#2", 10, "NaN", "NaN", Equal);
    test("1.0", "0x1.0#1", 10, "NaN", "NaN", Equal);
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        1,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        "1.5117",
        "0x1.830#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        "1.5121345516578424738967396780705",
        "0x1.831b3ffe85a576660f8350f58#100",
        Less,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        1,
        "8.0",
        "0x8.0#1",
        Less,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        10,
        "9.1250",
        "0x9.20#10",
        Greater,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        100,
        "9.1180064704027401212583371820456",
        "0x9.1e35ac0b189dfe008de388a0#100",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        10,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        "744261138.38279554153782298947663",
        "0x2c5c8612.61fee37bf5a5ad6b8c#100",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        100,
        "744261138.38279554153782298947663",
        "0x2c5c8612.61fee37bf5a5ad6b8c#100",
        Less,
    );
    test(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0985787164673876924043581168822e323228496",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        100,
        "744260566.82405288407985855144556",
        "0x2c5c83d6.d2f5213b4c27182020#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn tree_t_minus_1_prec_fail() {
    Float::NAN.tree_t_minus_1_prec(0);
}

#[test]
fn test_tree_t_minus_1_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (tree_t_minus_1, o) = x.clone().tree_t_minus_1_round(rm);
        assert!(tree_t_minus_1.is_valid());

        assert_eq!(tree_t_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&tree_t_minus_1), out_hex);
        assert_eq!(o, o_out);

        let (tree_t_minus_1_alt, o_alt) = x.tree_t_minus_1_round_ref(rm);
        assert!(tree_t_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&tree_t_minus_1),
            ComparableFloatRef(&tree_t_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut tree_t_minus_1_alt = x.clone();
        let o_alt = tree_t_minus_1_alt.tree_t_minus_1_round_assign(rm);
        assert!(tree_t_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&tree_t_minus_1),
            ComparableFloatRef(&tree_t_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    test("-1.0", "-0x1.0#1", Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", Nearest, "NaN", "NaN", Equal);
    test(
        "-2.7182818284590451",
        "-0x2.b7e151628aed2#53",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("-10.0", "-0xa.0#3", Nearest, "NaN", "NaN", Equal);
    test(
        "-1.00000000e10",
        "-0x2.540be4E+8#24",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e100",
        "-0x1.249ad2594c37dE+83#53",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84#49",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-123.45600000000000",
        "-0x7b.74bc6a7ef9dc#53",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Ceiling,
        "3.5771520639572980",
        "0x3.93c03cd783f78#52",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Floor,
        "3.5771520639572971",
        "0x3.93c03cd783f74#52",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Down,
        "3.5771520639572971",
        "0x3.93c03cd783f74#52",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Up,
        "3.5771520639572980",
        "0x3.93c03cd783f78#52",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        Nearest,
        "3.5771520639572971",
        "0x3.93c03cd783f74#52",
        Less,
    );
    test("0.25", "0x0.4#1", Ceiling, "4.0", "0x4.0#1", Greater);
    test("0.25", "0x0.4#1", Floor, "2.0", "0x2.0#1", Less);
    test("0.25", "0x0.4#1", Down, "2.0", "0x2.0#1", Less);
    test("0.25", "0x0.4#1", Up, "4.0", "0x4.0#1", Greater);
    test("0.25", "0x0.4#1", Nearest, "2.0", "0x2.0#1", Less);
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        Ceiling,
        "1.7813370234216277",
        "0x1.c805b402bff7f#53",
        Greater,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        Floor,
        "1.7813370234216275",
        "0x1.c805b402bff7e#53",
        Less,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        Down,
        "1.7813370234216275",
        "0x1.c805b402bff7e#53",
        Less,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        Up,
        "1.7813370234216277",
        "0x1.c805b402bff7f#53",
        Greater,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        Nearest,
        "1.7813370234216277",
        "0x1.c805b402bff7f#53",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Ceiling,
        "14.163600815810184",
        "0xe.29e1be3980f00#53",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Floor,
        "14.163600815810183",
        "0xe.29e1be3980ef8#53",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Down,
        "14.163600815810183",
        "0xe.29e1be3980ef8#53",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Up,
        "14.163600815810184",
        "0xe.29e1be3980f00#53",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Nearest,
        "14.163600815810183",
        "0xe.29e1be3980ef8#53",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Ceiling,
        "235.7211588756854",
        "0xeb.b89dde3a4a0#49",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Floor,
        "235.7211588756850",
        "0xeb.b89dde3a498#49",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Down,
        "235.7211588756850",
        "0xeb.b89dde3a498#49",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Up,
        "235.7211588756854",
        "0xeb.b89dde3a4a0#49",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Nearest,
        "235.7211588756854",
        "0xeb.b89dde3a4a0#49",
        Greater,
    );
    test(
        "0.36787944117144233",
        "0x0.5e2d58d8b3bce#50",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        Ceiling,
        "1.0000000153042543",
        "0x1.00000041bb34a#53",
        Greater,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        Floor,
        "1.0000000153042541",
        "0x1.00000041bb349#53",
        Less,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        Down,
        "1.0000000153042541",
        "0x1.00000041bb349#53",
        Less,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        Up,
        "1.0000000153042543",
        "0x1.00000041bb34a#53",
        Greater,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        Nearest,
        "1.0000000153042543",
        "0x1.00000041bb34a#53",
        Greater,
    );
    test("0.38", "0x0.6#2", Nearest, "NaN", "NaN", Equal);
    test("1.0", "0x1.0#1", Nearest, "NaN", "NaN", Equal);
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        Ceiling,
        "1.5121345516578424738967396780721",
        "0x1.831b3ffe85a576660f8350f5a#100",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        Floor,
        "1.5121345516578424738967396780705",
        "0x1.831b3ffe85a576660f8350f58#100",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        Down,
        "1.5121345516578424738967396780705",
        "0x1.831b3ffe85a576660f8350f58#100",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        Up,
        "1.5121345516578424738967396780721",
        "0x1.831b3ffe85a576660f8350f5a#100",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        Nearest,
        "1.5121345516578424738967396780705",
        "0x1.831b3ffe85a576660f8350f58#100",
        Less,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        Ceiling,
        "9.1180064704027401212583371820468142742704349737639473102927449",
        "0x9.1e35ac0b189dfe008de388a018f6274a806fed68ff921f63e#200",
        Greater,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        Floor,
        "9.1180064704027401212583371820468142742704349737639473102927349",
        "0x9.1e35ac0b189dfe008de388a018f6274a806fed68ff921f63d#200",
        Less,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        Down,
        "9.1180064704027401212583371820468142742704349737639473102927349",
        "0x9.1e35ac0b189dfe008de388a018f6274a806fed68ff921f63d#200",
        Less,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        Up,
        "9.1180064704027401212583371820468142742704349737639473102927449",
        "0x9.1e35ac0b189dfe008de388a018f6274a806fed68ff921f63e#200",
        Greater,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        Nearest,
        "9.1180064704027401212583371820468142742704349737639473102927349",
        "0x9.1e35ac0b189dfe008de388a018f6274a806fed68ff921f63d#200",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Ceiling,
        "1.1e9",
        "0x4.0E+7#1",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Floor,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Down,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Up,
        "1.1e9",
        "0x4.0E+7#1",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Nearest,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Ceiling,
        "744261138.38279554153782298947747",
        "0x2c5c8612.61fee37bf5a5ad6b90#100",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Floor,
        "744261138.38279554153782298947663",
        "0x2c5c8612.61fee37bf5a5ad6b8c#100",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Down,
        "744261138.38279554153782298947663",
        "0x2c5c8612.61fee37bf5a5ad6b8c#100",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Up,
        "744261138.38279554153782298947747",
        "0x2c5c8612.61fee37bf5a5ad6b90#100",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Nearest,
        "744261138.38279554153782298947663",
        "0x2c5c8612.61fee37bf5a5ad6b8c#100",
        Less,
    );
    test(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0985787164673876924043581168822e323228496",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Ceiling,
        "8.1e8",
        "0x3.0E+7#2",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Floor,
        "5.4e8",
        "0x2.0E+7#2",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Down,
        "5.4e8",
        "0x2.0E+7#2",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Up,
        "8.1e8",
        "0x3.0E+7#2",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Nearest,
        "8.1e8",
        "0x3.0E+7#2",
        Greater,
    );
}

#[test]
#[should_panic]
fn tree_t_minus_1_round_fail() {
    Float::from(0.25).tree_t_minus_1_round(Exact);
}

#[test]
fn test_tree_t_minus_1_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (tree_t_minus_1, o) = x.clone().tree_t_minus_1_prec_round(prec, rm);
        assert!(tree_t_minus_1.is_valid());

        assert_eq!(tree_t_minus_1.to_string(), out);
        assert_eq!(to_hex_string(&tree_t_minus_1), out_hex);
        assert_eq!(o, o_out);

        let (tree_t_minus_1_alt, o_alt) = x.tree_t_minus_1_prec_round_ref(prec, rm);
        assert!(tree_t_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&tree_t_minus_1),
            ComparableFloatRef(&tree_t_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut tree_t_minus_1_alt = x.clone();
        let o_alt = tree_t_minus_1_alt.tree_t_minus_1_prec_round_assign(prec, rm);
        assert!(tree_t_minus_1_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&tree_t_minus_1),
            ComparableFloatRef(&tree_t_minus_1_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    // The expected values are those of the Lambert W tests, with the arguments and the results
    // negated: they agree with Azurite's verified Lambert W functions.
    test("-1.0", "-0x1.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "-2.7182818284590451",
        "-0x2.b7e151628aed2#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("-10.0", "-0xa.0#3", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "-1.00000000e10",
        "-0x2.540be4E+8#24",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0000000000000000e100",
        "-0x1.249ad2594c37dE+83#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.000000000000000e-100",
        "-0xd.ff9772470298E-84#49",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-123.45600000000000",
        "-0x7b.74bc6a7ef9dc#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        1,
        Nearest,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Down,
        "3.5742",
        "0x3.93#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Up,
        "3.5781",
        "0x3.94#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Ceiling,
        "3.5781",
        "0x3.94#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Floor,
        "3.5742",
        "0x3.93#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Nearest,
        "3.5781",
        "0x3.94#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        64,
        Nearest,
        "3.57715206395729714141",
        "0x3.93c03cd783f7414c#64",
        Greater,
    );
    test("0.25", "0x0.4#1", 1, Nearest, "2.0", "0x2.0#1", Less);
    test("0.25", "0x0.4#1", 10, Down, "2.1523", "0x2.27#10", Less);
    test("0.25", "0x0.4#1", 10, Up, "2.1562", "0x2.28#10", Greater);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Ceiling,
        "2.1562",
        "0x2.28#10",
        Greater,
    );
    test("0.25", "0x0.4#1", 10, Floor, "2.1523", "0x2.27#10", Less);
    test("0.25", "0x0.4#1", 10, Nearest, "2.1523", "0x2.27#10", Less);
    test(
        "0.25",
        "0x0.4#1",
        64,
        Nearest,
        "2.15329236411034964909",
        "0x2.273e2b1a949a1130#64",
        Less,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        10,
        Down,
        "1.7812",
        "0x1.c80#10",
        Less,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        10,
        Up,
        "1.7832",
        "0x1.c88#10",
        Greater,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        10,
        Ceiling,
        "1.7832",
        "0x1.c88#10",
        Greater,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        10,
        Floor,
        "1.7812",
        "0x1.c80#10",
        Less,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        10,
        Nearest,
        "1.7812",
        "0x1.c80#10",
        Less,
    );
    test(
        "0.29999999999999999",
        "0x0.4ccccccccccccc#53",
        64,
        Nearest,
        "1.78133702342162769639",
        "0x1.c805b402bff7effc#64",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        1,
        Nearest,
        "16.0",
        "0x1.0E+1#1",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Down,
        "14.156",
        "0xe.28#10",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Up,
        "14.172",
        "0xe.2c#10",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Ceiling,
        "14.172",
        "0xe.2c#10",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Floor,
        "14.156",
        "0xe.28#10",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Nearest,
        "14.156",
        "0xe.28#10",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        64,
        Nearest,
        "14.1636008158101829209",
        "0xe.29e1be3980ef8f4#64",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        1,
        Nearest,
        "2.6e2",
        "0x1.0E+2#1",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Down,
        "235.50",
        "0xeb.8#10",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Up,
        "235.75",
        "0xeb.c#10",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Ceiling,
        "235.75",
        "0xeb.c#10",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Floor,
        "235.50",
        "0xeb.8#10",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Nearest,
        "235.75",
        "0xeb.c#10",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        64,
        Nearest,
        "235.721158875685313644",
        "0xeb.b89dde3a49de86#64",
        Greater,
    );
    test(
        "0.36787944117144233",
        "0x0.5e2d58d8b3bce#50",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        10,
        Down,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        10,
        Up,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "0.36787944117144228",
        "0x0.5e2d58d8b3bcdc#53",
        64,
        Nearest,
        "1.00000001530425428465",
        "0x1.00000041bb349e4c#64",
        Greater,
    );
    test("0.38", "0x0.6#2", 10, Nearest, "NaN", "NaN", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Down,
        "1.5117",
        "0x1.830#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Up,
        "1.5137",
        "0x1.838#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Ceiling,
        "1.5137",
        "0x1.838#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Floor,
        "1.5117",
        "0x1.830#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Nearest,
        "1.5117",
        "0x1.830#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        64,
        Nearest,
        "1.51213455165784247389",
        "0x1.831b3ffe85a57666#64",
        Less,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        1,
        Nearest,
        "8.0",
        "0x8.0#1",
        Less,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        10,
        Down,
        "9.1094",
        "0x9.1c#10",
        Less,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        10,
        Up,
        "9.1250",
        "0x9.20#10",
        Greater,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        10,
        Ceiling,
        "9.1250",
        "0x9.20#10",
        Greater,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        10,
        Floor,
        "9.1094",
        "0x9.1c#10",
        Less,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        10,
        Nearest,
        "9.1250",
        "0x9.20#10",
        Greater,
    );
    test(
        "0.0010000000000000000000000000000000000000000000000000000000000006",
        "0x0.004189374bc6a7ef9db22d0e5604189374bc6a7ef9db22d0e5608#200",
        64,
        Nearest,
        "9.11800647040274012123",
        "0x9.1e35ac0b189dfe0#64",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Down,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Up,
        "1.1e9",
        "0x4.0E+7#1",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Ceiling,
        "1.1e9",
        "0x4.0E+7#1",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Floor,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Nearest,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        10,
        Down,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        10,
        Up,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        10,
        Ceiling,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        10,
        Floor,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        10,
        Nearest,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        64,
        Nearest,
        "744261138.382795541547",
        "0x2c5c8612.61fee37c0#64",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Down,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Up,
        "1.1e9",
        "0x4.0E+7#1",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Ceiling,
        "1.1e9",
        "0x4.0E+7#1",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Floor,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Nearest,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Down,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Up,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Ceiling,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Floor,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Nearest,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        64,
        Nearest,
        "744261138.382795541547",
        "0x2c5c8612.61fee37c0#64",
        Greater,
    );
    test(
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0985787164673876924043581168822e323228496",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Down,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Up,
        "1.1e9",
        "0x4.0E+7#1",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Ceiling,
        "1.1e9",
        "0x4.0E+7#1",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Floor,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Nearest,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Down,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Up,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Ceiling,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Floor,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Nearest,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        64,
        Nearest,
        "744260566.824052884069",
        "0x2c5c83d6.d2f5213b4#64",
        Less,
    );
}

#[test]
#[should_panic]
fn tree_t_minus_1_prec_round_fail_1() {
    Float::ONE.tree_t_minus_1_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn tree_t_minus_1_prec_round_fail_2() {
    Float::from(0.25).tree_t_minus_1_prec_round(10, Exact);
}

// Checks a result against the Lambert W function, since T(x) = -W(-x). A result that is finite and
// nonzero and did not overflow or underflow is also checked against the algorithm-independent
// rounding check, applied to W(-x) = -T(x).
fn check_rounding(x: &Float, tree_t_minus_1: &Float, o: Ordering, prec: u64, rm: RoundingMode) {
    let (w, wo) = (-x).lambert_w_minus_1_prec_round(prec, -rm);
    assert_eq!(ComparableFloatRef(&-w), ComparableFloatRef(tree_t_minus_1));
    assert_eq!(wo.reverse(), o);
    if rm != Exact && tree_t_minus_1.is_normal() && result_in_range(tree_t_minus_1, o) {
        let neg_x = float_to_big_exponent_float(&-x);
        assert_lambert_w_correctly_rounded(
            false,
            &|f| f.partial_cmp(&neg_x).unwrap(),
            Some(&neg_x),
            &float_to_big_exponent_float(&-tree_t_minus_1),
            o.reverse(),
            prec,
            -rm,
        );
    }
}

#[allow(clippy::needless_pass_by_value)]
fn tree_t_minus_1_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (tree_t_minus_1, o) = x.clone().tree_t_minus_1_prec_round(prec, rm);
    assert!(tree_t_minus_1.is_valid());
    assert_rounding_ordering_consistent(&tree_t_minus_1, rm, o);

    let (tree_t_minus_1_alt, o_alt) = x.tree_t_minus_1_prec_round_ref(prec, rm);
    assert!(tree_t_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&tree_t_minus_1_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.tree_t_minus_1_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &tree_t_minus_1, o, prec, rm);

    if tree_t_minus_1.is_normal() {
        assert_eq!(tree_t_minus_1.get_prec(), Some(prec));
    }
    if x < 0u32 {
        assert!(tree_t_minus_1.is_nan());
    } else if tree_t_minus_1.is_normal() {
        // T_{-1} maps (0, 1/e) to (1, ∞).
        assert!(tree_t_minus_1 >= 1u32);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.tree_t_minus_1_prec_round_ref(prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(tree_t_minus_1.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.tree_t_minus_1_prec_round_ref(prec, Exact));
    }

    // A `Float` that is not too large or too small gives the same result as the equal `Rational`.
    if x.is_normal() && exponent_in_gate(&x) {
        let (r, ro) = Float::tree_t_minus_1_rational_prec_round(Rational::exact_from(&x), prec, rm);
        assert_eq!(ComparableFloatRef(&r), ComparableFloatRef(&tree_t_minus_1));
        assert_eq!(ro, o);
    }
}

#[test]
fn tree_t_minus_1_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        tree_t_minus_1_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            tree_t_minus_1_prec_round_properties_helper(x, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.tree_t_minus_1_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::INFINITY.tree_t_minus_1_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_INFINITY.tree_t_minus_1_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::ZERO.tree_t_minus_1_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );
        assert_eq!(
            Float::NEGATIVE_ZERO.tree_t_minus_1_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );

        // Negative, and above 1/e
        let (s, o) = Float::ONE.tree_t_minus_1_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);
        let (s, o) = Float::NEGATIVE_ONE.tree_t_minus_1_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn tree_t_minus_1_prec_properties_helper(x: Float, prec: u64) {
    let (tree_t_minus_1, o) = x.clone().tree_t_minus_1_prec(prec);
    assert!(tree_t_minus_1.is_valid());

    let (tree_t_minus_1_alt, o_alt) = x.tree_t_minus_1_prec_ref(prec);
    assert!(tree_t_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&tree_t_minus_1_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.tree_t_minus_1_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    assert_eq!(o_alt, o);

    let (tree_t_minus_1_alt, o_alt) = x.tree_t_minus_1_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&tree_t_minus_1_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &tree_t_minus_1, o, prec, Nearest);

    if tree_t_minus_1.is_normal() {
        assert_eq!(tree_t_minus_1.get_prec(), Some(prec));
    }
    if x < 0u32 {
        assert!(tree_t_minus_1.is_nan());
    } else if tree_t_minus_1.is_normal() {
        // T_{-1} maps (0, 1/e) to (1, ∞).
        assert!(tree_t_minus_1 >= 1u32);
    }
}

#[test]
fn tree_t_minus_1_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        tree_t_minus_1_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        tree_t_minus_1_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        tree_t_minus_1_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (s, o) = Float::NAN.tree_t_minus_1_prec(prec);
        assert!(s.is_nan());
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn tree_t_minus_1_round_properties_helper(x: Float, rm: RoundingMode) {
    let (tree_t_minus_1, o) = x.clone().tree_t_minus_1_round(rm);
    assert!(tree_t_minus_1.is_valid());
    assert_rounding_ordering_consistent(&tree_t_minus_1, rm, o);

    let (tree_t_minus_1_alt, o_alt) = x.tree_t_minus_1_round_ref(rm);
    assert!(tree_t_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&tree_t_minus_1_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.tree_t_minus_1_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    assert_eq!(o_alt, o);

    let (tree_t_minus_1_alt, o_alt) = x.tree_t_minus_1_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&tree_t_minus_1_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &tree_t_minus_1, o, x.significant_bits(), rm);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.tree_t_minus_1_round_ref(rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(tree_t_minus_1.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.tree_t_minus_1_round_ref(Exact));
    }
}

#[test]
fn tree_t_minus_1_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        tree_t_minus_1_round_properties_helper(x, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_rounding_mode_pair_gen_var_47().test_properties_with_config(&config, |(x, rm)| {
        tree_t_minus_1_round_properties_helper(x, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn tree_t_minus_1_properties_helper(x: Float) {
    let tree_t_minus_1 = x.clone().tree_t_minus_1();
    assert!(tree_t_minus_1.is_valid());

    let tree_t_minus_1_alt = (&x).tree_t_minus_1();
    assert!(tree_t_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&tree_t_minus_1_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );

    let mut x_alt = x.clone();
    x_alt.tree_t_minus_1_assign();
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );

    let tree_t_minus_1_alt = x
        .tree_t_minus_1_prec_round_ref(x.significant_bits(), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&tree_t_minus_1_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    let tree_t_minus_1_alt = x.tree_t_minus_1_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&tree_t_minus_1_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    let tree_t_minus_1_alt = x.tree_t_minus_1_round_ref(Nearest).0;
    assert_eq!(
        ComparableFloatRef(&tree_t_minus_1_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    if x < 0u32 {
        assert!(tree_t_minus_1.is_nan());
    } else if tree_t_minus_1.is_normal() {
        // T_{-1} maps (0, 1/e) to (1, ∞).
        assert!(tree_t_minus_1 >= 1u32);
    }
}

#[test]
fn tree_t_minus_1_properties() {
    float_gen().test_properties(|x| {
        tree_t_minus_1_properties_helper(x);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_gen().test_properties_with_config(&config, |x| {
        tree_t_minus_1_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        tree_t_minus_1_properties_helper(x);
    });
}

#[test]
fn test_primitive_float_tree_t_minus_1() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_tree_t_minus_1(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN);
    test::<f32>(-0.0, f32::INFINITY);
    test::<f32>(0.0, f32::INFINITY);
    test::<f32>(-1.0, f32::NAN);
    test::<f32>(-2.7182817, f32::NAN);
    test::<f32>(-7.0, f32::NAN);
    test::<f32>(-100.0, f32::NAN);
    test::<f32>(-1.0e-10, f32::NAN);
    test::<f32>(0.1, 3.577152);
    test::<f32>(0.25, 2.1532924);
    test::<f32>(0.36787942, 1.0003351);
    test::<f32>(0.5, f32::NAN);
    test::<f32>(-3.4028235e38, f32::NAN);
    test::<f32>(-1.1754944e-38, f32::NAN);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN);
    test::<f64>(-0.0, f64::INFINITY);
    test::<f64>(0.0, f64::INFINITY);
    test::<f64>(-1.0, f64::NAN);
    test::<f64>(-core::f64::consts::E, f64::NAN);
    test::<f64>(-7.0, f64::NAN);
    test::<f64>(-100.0, f64::NAN);
    test::<f64>(-1.0e-10, f64::NAN);
    test::<f64>(0.1, 3.577152063957297);
    test::<f64>(0.25, 2.15329236411035);
    test::<f64>(0.36787944117144233, f64::NAN);
    test::<f64>(0.5, f64::NAN);
    test::<f64>(-1.7976931348623157e308, f64::NAN);
    test::<f64>(-2.2250738585072014e-308, f64::NAN);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_tree_t_minus_1_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        // T(x) = -W(-x)
        assert_eq!(
            NiceFloat(primitive_float_tree_t_minus_1(x)),
            NiceFloat(-primitive_float_lambert_w_minus_1(-x))
        );
    });
}

#[test]
fn primitive_float_tree_t_minus_1_properties() {
    apply_fn_to_primitive_floats!(primitive_float_tree_t_minus_1_properties_helper);
}

#[test]
fn test_tree_t_minus_1_rational_prec() {
    let test = |s, prec, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::tree_t_minus_1_rational_prec(x.clone(), prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::tree_t_minus_1_rational_prec_ref(&x, prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("-1", 10, "NaN", "NaN", Equal);
    test("-1/3", 10, "NaN", "NaN", Equal);
    test("1/3", 1, "2.0", "0x2.0#1", Greater);
    test("1/3", 10, "1.5117", "0x1.830#10", Less);
    test(
        "1/3",
        100,
        "1.5121345516578424738967396780721",
        "0x1.831b3ffe85a576660f8350f5a#100",
        Greater,
    );
    test("-22/7", 10, "NaN", "NaN", Equal);
    test("1/4", 1, "2.0", "0x2.0#1", Less);
    test("1/4", 10, "2.1523", "0x2.27#10", Less);
    test(
        "1/4",
        100,
        "2.1532923641103496491690991500941",
        "0x2.273e2b1a949a11317152b2a04#100",
        Greater,
    );
    test("1/1000", 1, "8.0", "0x8.0#1", Less);
    test("1/1000", 10, "9.1250", "0x9.20#10", Greater);
    test(
        "1/1000",
        100,
        "9.1180064704027401212583371820456",
        "0x9.1e35ac0b189dfe008de388a0#100",
        Less,
    );
    test("-1000000000000000000000000000000", 10, "NaN", "NaN", Equal);
    test(
        "-1/1000000000000000000000000000000",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1/1000000000000000000000000000000",
        1,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        "73.375",
        "0x49.6#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        100,
        "73.373110313822976797067478758072",
        "0x49.5f842853ab83f204a3f11da8#100",
        Less,
    );
    test("11/30", 1, "1.0", "0x1.0#1", Less);
    test("11/30", 10, "1.0840", "0x1.158#10", Greater);
    test(
        "11/30",
        100,
        "1.0834824917754593226793354377566",
        "0x1.155f1bcbf6d4f1a46f428ebb6#100",
        Greater,
    );
    test("3/8", 10, "NaN", "NaN", Equal);
    test(
        "4598493014643029/12500000000000000",
        1,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "4598493014643029/12500000000000000",
        10,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "4598493014643029/12500000000000000",
        100,
        "1.0000000029451938068045932316911",
        "0x1.0000000ca6465bb19944af358#100",
        Less,
    );
    test(
        "36787944117144233/100000000000000000",
        10,
        "NaN",
        "NaN",
        Equal,
    );
}

#[test]
#[should_panic]
fn tree_t_minus_1_rational_prec_fail() {
    Float::tree_t_minus_1_rational_prec(Rational::ONE, 0);
}

#[test]
#[should_panic]
fn tree_t_minus_1_rational_prec_ref_fail() {
    Float::tree_t_minus_1_rational_prec_ref(&Rational::ONE, 0);
}

#[test]
fn test_tree_t_minus_1_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::tree_t_minus_1_rational_prec_round(x.clone(), prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::tree_t_minus_1_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("-1", 10, Nearest, "NaN", "NaN", Equal);
    test("-1/3", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "1/3",
        53,
        Nearest,
        "1.5121345516578424",
        "0x1.831b3ffe85a57#53",
        Less,
    );
    test("1/3", 10, Ceiling, "1.5137", "0x1.838#10", Greater);
    test("1/3", 10, Floor, "1.5117", "0x1.830#10", Less);
    test("1/3", 10, Down, "1.5117", "0x1.830#10", Less);
    test("1/3", 10, Up, "1.5137", "0x1.838#10", Greater);
    test("1/3", 10, Nearest, "1.5117", "0x1.830#10", Less);
    test("-22/7", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "1/4",
        53,
        Nearest,
        "2.1532923641103499",
        "0x2.273e2b1a949a2#53",
        Greater,
    );
    test("1/4", 10, Ceiling, "2.1562", "0x2.28#10", Greater);
    test("1/4", 10, Floor, "2.1523", "0x2.27#10", Less);
    test("1/4", 10, Down, "2.1523", "0x2.27#10", Less);
    test("1/4", 10, Up, "2.1562", "0x2.28#10", Greater);
    test("1/4", 10, Nearest, "2.1523", "0x2.27#10", Less);
    test(
        "1/1000",
        53,
        Nearest,
        "9.1180064704027401",
        "0x9.1e35ac0b189e0#53",
        Greater,
    );
    test("1/1000", 10, Ceiling, "9.1250", "0x9.20#10", Greater);
    test("1/1000", 10, Floor, "9.1094", "0x9.1c#10", Less);
    test("1/1000", 10, Down, "9.1094", "0x9.1c#10", Less);
    test("1/1000", 10, Up, "9.1250", "0x9.20#10", Greater);
    test("1/1000", 10, Nearest, "9.1250", "0x9.20#10", Greater);
    test(
        "-1000000000000000000000000000000",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1/1000000000000000000000000000000",
        53,
        Nearest,
        "73.373110313822977",
        "0x49.5f842853ab84#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Ceiling,
        "73.375",
        "0x49.6#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Floor,
        "73.250",
        "0x49.4#10",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Down,
        "73.250",
        "0x49.4#10",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Up,
        "73.375",
        "0x49.6#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Nearest,
        "73.375",
        "0x49.6#10",
        Greater,
    );
    test(
        "11/30",
        53,
        Nearest,
        "1.0834824917754593",
        "0x1.155f1bcbf6d4f#53",
        Less,
    );
    test("11/30", 10, Ceiling, "1.0840", "0x1.158#10", Greater);
    test("11/30", 10, Floor, "1.0820", "0x1.150#10", Less);
    test("11/30", 10, Down, "1.0820", "0x1.150#10", Less);
    test("11/30", 10, Up, "1.0840", "0x1.158#10", Greater);
    test("11/30", 10, Nearest, "1.0840", "0x1.158#10", Greater);
    test("3/8", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "4598493014643029/12500000000000000",
        53,
        Nearest,
        "1.0000000029451939",
        "0x1.0000000ca6466#53",
        Greater,
    );
    test(
        "4598493014643029/12500000000000000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "4598493014643029/12500000000000000",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "4598493014643029/12500000000000000",
        10,
        Down,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "4598493014643029/12500000000000000",
        10,
        Up,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "4598493014643029/12500000000000000",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "36787944117144233/100000000000000000",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
}

#[test]
#[should_panic]
fn tree_t_minus_1_rational_prec_round_fail_1() {
    Float::tree_t_minus_1_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn tree_t_minus_1_rational_prec_round_fail_2() {
    Float::tree_t_minus_1_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[test]
#[should_panic]
fn tree_t_minus_1_rational_prec_round_ref_fail() {
    Float::tree_t_minus_1_rational_prec_round_ref(&Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn tree_t_minus_1_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (tree_t_minus_1, o) = Float::tree_t_minus_1_rational_prec_round(x.clone(), prec, rm);
    assert!(tree_t_minus_1.is_valid());
    assert_rounding_ordering_consistent(&tree_t_minus_1, rm, o);

    let (tree_t_minus_1_alt, o_alt) = Float::tree_t_minus_1_rational_prec_round_ref(&x, prec, rm);
    assert!(tree_t_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&tree_t_minus_1_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    assert_eq!(o_alt, o);

    // T(x) = -W(-x)
    let neg_x = -&x;
    let (w, wo) = Float::lambert_w_minus_1_rational_prec_round_ref(&neg_x, prec, -rm);
    let t = -w;
    assert_eq!(ComparableFloatRef(&t), ComparableFloatRef(&tree_t_minus_1));
    assert_eq!(wo.reverse(), o);

    if rm != Exact && tree_t_minus_1.is_normal() && result_in_range(&tree_t_minus_1, o) {
        assert_lambert_w_correctly_rounded(
            false,
            &|f| f.partial_cmp(&neg_x).unwrap(),
            None,
            &float_to_big_exponent_float(&-&tree_t_minus_1),
            o.reverse(),
            prec,
            -rm,
        );
    }

    if tree_t_minus_1.is_normal() {
        assert_eq!(tree_t_minus_1.get_prec(), Some(prec));
    }
    if x < 0u32 {
        assert!(tree_t_minus_1.is_nan());
    } else if tree_t_minus_1.is_normal() {
        assert!(tree_t_minus_1 >= 1u32);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::tree_t_minus_1_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(tree_t_minus_1.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::tree_t_minus_1_rational_prec_round_ref(
            &x, prec, Exact
        ));
    }
}

#[test]
fn tree_t_minus_1_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        tree_t_minus_1_rational_prec_round_properties_helper(x, prec, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn tree_t_minus_1_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (tree_t_minus_1, o) = Float::tree_t_minus_1_rational_prec(x.clone(), prec);
    assert!(tree_t_minus_1.is_valid());

    let (tree_t_minus_1_alt, o_alt) = Float::tree_t_minus_1_rational_prec_ref(&x, prec);
    assert!(tree_t_minus_1_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&tree_t_minus_1_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    assert_eq!(o_alt, o);

    let (tree_t_minus_1_alt, o_alt) =
        Float::tree_t_minus_1_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&tree_t_minus_1_alt),
        ComparableFloatRef(&tree_t_minus_1)
    );
    assert_eq!(o_alt, o);
}

#[test]
fn tree_t_minus_1_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        tree_t_minus_1_rational_prec_properties_helper(x, prec);
    });
}

#[test]
fn test_primitive_float_tree_t_minus_1_rational() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_tree_t_minus_1_rational::<T>(
                &Rational::from_str(s).unwrap()
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", f32::INFINITY);
    test::<f64>("0", f64::INFINITY);
    test::<f32>("-1", f32::NAN);
    test::<f64>("-1", f64::NAN);
    test::<f32>("-1/3", f32::NAN);
    test::<f64>("-1/3", f64::NAN);
    test::<f32>("1/3", 1.5121346);
    test::<f64>("1/3", 1.5121345516578424);
    test::<f32>("1/4", 2.1532924);
    test::<f64>("1/4", 2.15329236411035);
    test::<f32>("1/2", f32::NAN);
    test::<f64>("1/2", f64::NAN);
    test::<f32>("-1000", f32::NAN);
    test::<f64>("-1000", f64::NAN);
    test::<f32>("1/1000", 9.118007);
    test::<f64>("1/1000", 9.11800647040274);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_tree_t_minus_1_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        // T(x) = -W(-x)
        let t = primitive_float_tree_t_minus_1_rational::<T>(&x);
        let w = primitive_float_lambert_w_minus_1_rational::<T>(&-&x);
        assert_eq!(NiceFloat(t), NiceFloat(-w));
    });
}

#[test]
fn primitive_float_tree_t_minus_1_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_tree_t_minus_1_rational_properties_helper);
}
