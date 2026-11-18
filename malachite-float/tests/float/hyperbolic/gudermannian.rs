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
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::hyperbolic::traits::{Gudermannian, GudermannianAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::hyperbolic::gudermannian::{
    primitive_float_gudermannian, primitive_float_gudermannian_rational,
};
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, gudermannian_prec_round_naive,
    gudermannian_rational_prec_round_naive,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, exponent_in_gate, parse_hex_string, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_54,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_57,
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
fn test_gudermannian() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let gudermannian = x.clone().gudermannian();
        assert!(gudermannian.is_valid());

        assert_eq!(gudermannian.to_string(), out);
        assert_eq!(to_hex_string(&gudermannian), out_hex);

        let gudermannian_alt = (&x).gudermannian();
        assert!(gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&gudermannian),
            ComparableFloatRef(&gudermannian_alt)
        );

        let mut gudermannian_alt = x.clone();
        gudermannian_alt.gudermannian_assign();
        assert!(gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&gudermannian),
            ComparableFloatRef(&gudermannian_alt)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "2.0", "0x2.0#1");
    test("-Infinity", "-Infinity", "-2.0", "-0x2.0#1");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("0.50", "0x0.8#1", "0.50", "0x0.8#1");
    test("2.0", "0x2.0#1", "1.0", "0x1.0#1");
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "1.4390113159636750",
        "0x1.70630baca34de#53",
    );
    test("10.0", "0xa.0#3", "1.5", "0x1.8#3");
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        "1.57079637",
        "0x1.921fb6#24",
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        "9.9999999998333335e-6",
        "0x0.0000a7c5ac470f4510#53",
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        "-0.099833748793486610",
        "-0x0.198eb45e1ae820#52",
    );
    test("-0.25", "-0x0.4#1", "-0.25", "-0x0.4#1");
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        "-0.29559867598460415",
        "-0x0.4bac5ad6184334#53",
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        "-9.9999999998333335e-6",
        "-0x0.0000a7c5ac470f4510#53",
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
    );
    test("1.0e323228496", "0x4.0E+268435455#1", "2.0", "0x2.0#1");
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        "1.5707963267948966192313216916397",
        "0x1.921fb54442d18469898cc5170#100",
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        "4.0e-323228249",
        "0x1.8E-268435250#2",
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
    );
}

#[test]
fn test_gudermannian_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (gudermannian, o) = x.clone().gudermannian_prec(prec);
        assert!(gudermannian.is_valid());

        assert_eq!(gudermannian.to_string(), out);
        assert_eq!(to_hex_string(&gudermannian), out_hex);
        assert_eq!(o, o_out);

        let (gudermannian_alt, o_alt) = x.gudermannian_prec_ref(prec);
        assert!(gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&gudermannian),
            ComparableFloatRef(&gudermannian_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut gudermannian_alt = x.clone();
        let o_alt = gudermannian_alt.gudermannian_prec_assign(prec);
        assert!(gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&gudermannian),
            ComparableFloatRef(&gudermannian_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "2.0", "0x2.0#1", Greater);
    test("Infinity", "Infinity", 10, "1.5703", "0x1.920#10", Less);
    test("-Infinity", "-Infinity", 1, "-2.0", "-0x2.0#1", Less);
    test(
        "-Infinity",
        "-Infinity",
        10,
        "-1.5703",
        "-0x1.920#10",
        Greater,
    );
    test("0.0", "0x0.0", 1, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", 10, "0.86621", "0x0.ddc#10", Greater);
    test(
        "1.0",
        "0x1.0#1",
        100,
        "0.86576948323965862428960184619172",
        "0x0.dda311a0639f976612e00f5aa#100",
        Less,
    );
    test("0.50", "0x0.8#1", 1, "0.50", "0x0.8#1", Greater);
    test("0.50", "0x0.8#1", 10, "0.48047", "0x0.7b0#10", Greater);
    test(
        "0.50",
        "0x0.8#1",
        100,
        "0.48038107913372944860488628996321",
        "0x0.7afa41207f1e4b236ed2ebc4a8#100",
        Less,
    );
    test("2.0", "0x2.0#1", 1, "1.0", "0x1.0#1", Less);
    test("2.0", "0x2.0#1", 10, "1.3027", "0x1.4d8#10", Greater);
    test(
        "2.0",
        "0x2.0#1",
        100,
        "1.3017603360460150998761562633654",
        "0x1.4d402a568c2fd418fec19a688#100",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        "1.4395",
        "0x1.708#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        100,
        "1.4390113159636751082201863864813",
        "0x1.70630baca34de4ca8aec5c45e#100",
        Greater,
    );
    test("10.0", "0xa.0#3", 1, "2.0", "0x2.0#1", Greater);
    test("10.0", "0xa.0#3", 10, "1.5703", "0x1.920#10", Less);
    test(
        "10.0",
        "0xa.0#3",
        100,
        "1.5707055269354340336812989597790",
        "0x1.9219c1e5d589f48363e10c2e0#100",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        100,
        "1.5707963267948966192313216916397",
        "0x1.921fb54442d18469898cc5170#100",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        1,
        "7.6e-6",
        "0x0.00008#1",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        "9.9987e-6",
        "0x0.0000a7c#10",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        100,
        "9.9999999998333341513680390994133e-6",
        "0x0.0000a7c5ac470f4512eb6710b9792#100",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        1,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        100,
        "1.5707963267948966192313216916397",
        "0x1.921fb54442d18469898cc5170#100",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        1,
        "1.1e-100",
        "0x1.0E-83#1",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        "1.0001e-100",
        "0xe.00E-84#10",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        100,
        "1.0000000000000000199918998026029e-100",
        "0xd.ff9772470298000000000000E-84#100",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        100,
        "1.5707963267948966192313216916397",
        "0x1.921fb54442d18469898cc5170#100",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        1,
        "-0.12",
        "-0x0.2#1",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        "-0.099854",
        "-0x0.1990#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        100,
        "-0.099833748793486620594769270338936",
        "-0x0.198eb45e1ae820c93fc6153550#100",
        Greater,
    );
    test("-0.25", "-0x0.4#1", 1, "-0.25", "-0x0.4#1", Less);
    test("-0.25", "-0x0.4#1", 10, "-0.24731", "-0x0.3f5#10", Greater);
    test(
        "-0.25",
        "-0x0.4#1",
        100,
        "-0.24743579898243148276130299863681",
        "-0x0.3f57f3d87d3a7610c7a7bc13d4#100",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        1,
        "-0.25",
        "-0x0.4#1",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        "-0.29541",
        "-0x0.4ba#10",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        100,
        "-0.29559867598460412705826336578108",
        "-0x0.4bac5ad61843321a4bb2a981d8#100",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        1,
        "-7.6e-6",
        "-0x0.00008#1",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        "-9.9987e-6",
        "-0x0.0000a7c#10",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        100,
        "-9.9999999998333341513680390994133e-6",
        "-0x0.0000a7c5ac470f4512eb6710b9792#100",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        100,
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        100,
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        100,
        "1.5707963267948966192313216916397",
        "0x1.921fb54442d18469898cc5170#100",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        100,
        "1.5707963267948966192313216916397",
        "0x1.921fb54442d18469898cc5170#100",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        "2.7e-323228249",
        "0x1.0E-268435250#1",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        "3.9981e-323228249",
        "0x1.800E-268435250#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        100,
        "3.9980887142939335100715454835713e-323228249",
        "0x1.8000000000000000000000000E-268435250#100",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        1,
        "-2.7e-323228249",
        "-0x1.0E-268435250#1",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        "-3.9981e-323228249",
        "-0x1.800E-268435250#10",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        100,
        "-3.9980887142939335100715454835713e-323228249",
        "-0x1.8000000000000000000000000E-268435250#100",
        Less,
    );
}

#[test]
#[should_panic]
fn gudermannian_prec_fail() {
    Float::NAN.gudermannian_prec(0);
}

#[test]
fn test_gudermannian_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (gudermannian, o) = x.clone().gudermannian_round(rm);
        assert!(gudermannian.is_valid());

        assert_eq!(gudermannian.to_string(), out);
        assert_eq!(to_hex_string(&gudermannian), out_hex);
        assert_eq!(o, o_out);

        let (gudermannian_alt, o_alt) = x.gudermannian_round_ref(rm);
        assert!(gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&gudermannian),
            ComparableFloatRef(&gudermannian_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut gudermannian_alt = x.clone();
        let o_alt = gudermannian_alt.gudermannian_round_assign(rm);
        assert!(gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&gudermannian),
            ComparableFloatRef(&gudermannian_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    test("NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", Floor, "1.0", "0x1.0#1", Less);
    test("Infinity", "Infinity", Ceiling, "2.0", "0x2.0#1", Greater);
    test("Infinity", "Infinity", Down, "1.0", "0x1.0#1", Less);
    test("Infinity", "Infinity", Up, "2.0", "0x2.0#1", Greater);
    test("Infinity", "Infinity", Nearest, "2.0", "0x2.0#1", Greater);
    test("-Infinity", "-Infinity", Floor, "-2.0", "-0x2.0#1", Less);
    test(
        "-Infinity",
        "-Infinity",
        Ceiling,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test("-Infinity", "-Infinity", Down, "-1.0", "-0x1.0#1", Greater);
    test("-Infinity", "-Infinity", Up, "-2.0", "-0x2.0#1", Less);
    test("-Infinity", "-Infinity", Nearest, "-2.0", "-0x2.0#1", Less);
    test("0.0", "0x0.0", Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", Nearest, "-0.0", "-0x0.0", Equal);
    test("NaN", "NaN", Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", Exact, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", Exact, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", Floor, "0.50", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", Ceiling, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", Down, "0.50", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", Up, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", Nearest, "1.0", "0x1.0#1", Greater);
    test("0.50", "0x0.8#1", Floor, "0.25", "0x0.4#1", Less);
    test("0.50", "0x0.8#1", Ceiling, "0.50", "0x0.8#1", Greater);
    test("0.50", "0x0.8#1", Down, "0.25", "0x0.4#1", Less);
    test("0.50", "0x0.8#1", Up, "0.50", "0x0.8#1", Greater);
    test("0.50", "0x0.8#1", Nearest, "0.50", "0x0.8#1", Greater);
    test("2.0", "0x2.0#1", Floor, "1.0", "0x1.0#1", Less);
    test("2.0", "0x2.0#1", Ceiling, "2.0", "0x2.0#1", Greater);
    test("2.0", "0x2.0#1", Down, "1.0", "0x1.0#1", Less);
    test("2.0", "0x2.0#1", Up, "2.0", "0x2.0#1", Greater);
    test("2.0", "0x2.0#1", Nearest, "1.0", "0x1.0#1", Less);
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Floor,
        "1.4390113159636750",
        "0x1.70630baca34de#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Ceiling,
        "1.4390113159636753",
        "0x1.70630baca34df#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Down,
        "1.4390113159636750",
        "0x1.70630baca34de#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Up,
        "1.4390113159636753",
        "0x1.70630baca34df#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Nearest,
        "1.4390113159636750",
        "0x1.70630baca34de#53",
        Less,
    );
    test("10.0", "0xa.0#3", Floor, "1.5", "0x1.8#3", Less);
    test("10.0", "0xa.0#3", Ceiling, "1.8", "0x1.c#3", Greater);
    test("10.0", "0xa.0#3", Down, "1.5", "0x1.8#3", Less);
    test("10.0", "0xa.0#3", Up, "1.8", "0x1.c#3", Greater);
    test("10.0", "0xa.0#3", Nearest, "1.5", "0x1.8#3", Less);
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Floor,
        "1.57079625",
        "0x1.921fb4#24",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Ceiling,
        "1.57079637",
        "0x1.921fb6#24",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Down,
        "1.57079625",
        "0x1.921fb4#24",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Up,
        "1.57079637",
        "0x1.921fb6#24",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Nearest,
        "1.57079637",
        "0x1.921fb6#24",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Floor,
        "9.9999999998333335e-6",
        "0x0.0000a7c5ac470f4510#53",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Ceiling,
        "9.9999999998333352e-6",
        "0x0.0000a7c5ac470f4518#53",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Down,
        "9.9999999998333335e-6",
        "0x0.0000a7c5ac470f4510#53",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Up,
        "9.9999999998333352e-6",
        "0x0.0000a7c5ac470f4518#53",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Nearest,
        "9.9999999998333335e-6",
        "0x0.0000a7c5ac470f4510#53",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Floor,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Ceiling,
        "1.5707963267948968",
        "0x1.921fb54442d19#53",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Down,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Up,
        "1.5707963267948968",
        "0x1.921fb54442d19#53",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Floor,
        "9.999999999999980e-101",
        "0xd.ff9772470290E-84#49",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Ceiling,
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Down,
        "9.999999999999980e-101",
        "0xd.ff9772470290E-84#49",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Up,
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Nearest,
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Floor,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Ceiling,
        "1.5707963267948968",
        "0x1.921fb54442d19#53",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Down,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Up,
        "1.5707963267948968",
        "0x1.921fb54442d19#53",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Floor,
        "-0.099833748793486637",
        "-0x0.198eb45e1ae822#52",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Ceiling,
        "-0.099833748793486610",
        "-0x0.198eb45e1ae820#52",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Down,
        "-0.099833748793486610",
        "-0x0.198eb45e1ae820#52",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Up,
        "-0.099833748793486637",
        "-0x0.198eb45e1ae822#52",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Nearest,
        "-0.099833748793486610",
        "-0x0.198eb45e1ae820#52",
        Greater,
    );
    test("-0.25", "-0x0.4#1", Floor, "-0.25", "-0x0.4#1", Less);
    test("-0.25", "-0x0.4#1", Ceiling, "-0.12", "-0x0.2#1", Greater);
    test("-0.25", "-0x0.4#1", Down, "-0.12", "-0x0.2#1", Greater);
    test("-0.25", "-0x0.4#1", Up, "-0.25", "-0x0.4#1", Less);
    test("-0.25", "-0x0.4#1", Nearest, "-0.25", "-0x0.4#1", Less);
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Floor,
        "-0.29559867598460415",
        "-0x0.4bac5ad6184334#53",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Ceiling,
        "-0.29559867598460410",
        "-0x0.4bac5ad6184330#53",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Down,
        "-0.29559867598460410",
        "-0x0.4bac5ad6184330#53",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Up,
        "-0.29559867598460415",
        "-0x0.4bac5ad6184334#53",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Nearest,
        "-0.29559867598460415",
        "-0x0.4bac5ad6184334#53",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Floor,
        "-9.9999999998333352e-6",
        "-0x0.0000a7c5ac470f4518#53",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Ceiling,
        "-9.9999999998333335e-6",
        "-0x0.0000a7c5ac470f4510#53",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Down,
        "-9.9999999998333335e-6",
        "-0x0.0000a7c5ac470f4510#53",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Up,
        "-9.9999999998333352e-6",
        "-0x0.0000a7c5ac470f4518#53",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Nearest,
        "-9.9999999998333335e-6",
        "-0x0.0000a7c5ac470f4510#53",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Ceiling,
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Up,
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Nearest,
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Floor,
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Up,
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Nearest,
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Floor,
        "1.5707963267948966192313216916397",
        "0x1.921fb54442d18469898cc5170#100",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Ceiling,
        "1.5707963267948966192313216916412",
        "0x1.921fb54442d18469898cc5172#100",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Down,
        "1.5707963267948966192313216916397",
        "0x1.921fb54442d18469898cc5170#100",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Up,
        "1.5707963267948966192313216916412",
        "0x1.921fb54442d18469898cc5172#100",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Nearest,
        "1.5707963267948966192313216916397",
        "0x1.921fb54442d18469898cc5170#100",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Floor,
        "2.7e-323228249",
        "0x1.0E-268435250#2",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Ceiling,
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Down,
        "2.7e-323228249",
        "0x1.0E-268435250#2",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Up,
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Nearest,
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Floor,
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Ceiling,
        "-2.7e-323228249",
        "-0x1.0E-268435250#2",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Down,
        "-2.7e-323228249",
        "-0x1.0E-268435250#2",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Up,
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Nearest,
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Less,
    );
}

#[test]
#[should_panic]
fn gudermannian_round_fail() {
    Float::from(0.5).gudermannian_round(Exact);
}

#[test]
fn test_gudermannian_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (gudermannian, o) = x.clone().gudermannian_prec_round(prec, rm);
        assert!(gudermannian.is_valid());

        assert_eq!(gudermannian.to_string(), out);
        assert_eq!(to_hex_string(&gudermannian), out_hex);
        assert_eq!(o, o_out);

        let (gudermannian_alt, o_alt) = x.gudermannian_prec_round_ref(prec, rm);
        assert!(gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&gudermannian),
            ComparableFloatRef(&gudermannian_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut gudermannian_alt = x.clone();
        let o_alt = gudermannian_alt.gudermannian_prec_round_assign(prec, rm);
        assert!(gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&gudermannian),
            ComparableFloatRef(&gudermannian_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    // The expected values agree with Azurite's verified Gudermannian function.
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity",
        "Infinity",
        10,
        Floor,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "Infinity",
        "Infinity",
        10,
        Ceiling,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "Infinity",
        "Infinity",
        10,
        Down,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "Infinity",
        "Infinity",
        10,
        Up,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "Infinity",
        "Infinity",
        10,
        Nearest,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        Floor,
        "-1.5723",
        "-0x1.928#10",
        Less,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        Ceiling,
        "-1.5703",
        "-0x1.920#10",
        Greater,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        Down,
        "-1.5703",
        "-0x1.920#10",
        Greater,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        Up,
        "-1.5723",
        "-0x1.928#10",
        Less,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        Nearest,
        "-1.5703",
        "-0x1.920#10",
        Greater,
    );
    test("0.0", "0x0.0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Up, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Down, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Up, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("NaN", "NaN", 10, Exact, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 10, Exact, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Exact, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", 10, Down, "0.86523", "0x0.dd8#10", Less);
    test("1.0", "0x1.0#1", 10, Up, "0.86621", "0x0.ddc#10", Greater);
    test("1.0", "0x1.0#1", 10, Floor, "0.86523", "0x0.dd8#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        64,
        Nearest,
        "0.865769483239658624286",
        "0x0.dda311a0639f9766#64",
        Less,
    );
    test("0.50", "0x0.8#1", 1, Nearest, "0.50", "0x0.8#1", Greater);
    test("0.50", "0x0.8#1", 10, Down, "0.47998", "0x0.7ae#10", Less);
    test("0.50", "0x0.8#1", 10, Up, "0.48047", "0x0.7b0#10", Greater);
    test("0.50", "0x0.8#1", 10, Floor, "0.47998", "0x0.7ae#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "0.48047",
        "0x0.7b0#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        10,
        Nearest,
        "0.48047",
        "0x0.7b0#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        64,
        Nearest,
        "0.480381079133729448609",
        "0x0.7afa41207f1e4b238#64",
        Greater,
    );
    test("2.0", "0x2.0#1", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("2.0", "0x2.0#1", 10, Down, "1.3008", "0x1.4d0#10", Less);
    test("2.0", "0x2.0#1", 10, Up, "1.3027", "0x1.4d8#10", Greater);
    test("2.0", "0x2.0#1", 10, Floor, "1.3008", "0x1.4d0#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "1.3027",
        "0x1.4d8#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "1.3027",
        "0x1.4d8#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        64,
        Nearest,
        "1.30176033604601509982",
        "0x1.4d402a568c2fd418#64",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Down,
        "1.4375",
        "0x1.700#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Up,
        "1.4395",
        "0x1.708#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Floor,
        "1.4375",
        "0x1.700#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Ceiling,
        "1.4395",
        "0x1.708#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Nearest,
        "1.4395",
        "0x1.708#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        64,
        Nearest,
        "1.43901131596367510819",
        "0x1.70630baca34de4ca#64",
        Less,
    );
    test("10.0", "0xa.0#3", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("10.0", "0xa.0#3", 10, Down, "1.5703", "0x1.920#10", Less);
    test("10.0", "0xa.0#3", 10, Up, "1.5723", "0x1.928#10", Greater);
    test("10.0", "0xa.0#3", 10, Floor, "1.5703", "0x1.920#10", Less);
    test(
        "10.0",
        "0xa.0#3",
        10,
        Ceiling,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test("10.0", "0xa.0#3", 10, Nearest, "1.5703", "0x1.920#10", Less);
    test(
        "10.0",
        "0xa.0#3",
        64,
        Nearest,
        "1.57070552693543403371",
        "0x1.9219c1e5d589f484#64",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Down,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Up,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Floor,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Ceiling,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Nearest,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        64,
        Nearest,
        "1.57079632679489661926",
        "0x1.921fb54442d1846a#64",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        1,
        Nearest,
        "7.6e-6",
        "0x0.00008#1",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Down,
        "9.9987e-6",
        "0x0.0000a7c#10",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Up,
        "0.000010014",
        "0x0.0000a80#10",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Floor,
        "9.9987e-6",
        "0x0.0000a7c#10",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Ceiling,
        "0.000010014",
        "0x0.0000a80#10",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Nearest,
        "9.9987e-6",
        "0x0.0000a7c#10",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        64,
        Nearest,
        "9.99999999983333415104e-6",
        "0x0.0000a7c5ac470f4512eb#64",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Down,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Up,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Floor,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Ceiling,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Nearest,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        64,
        Nearest,
        "1.57079632679489661926",
        "0x1.921fb54442d1846a#64",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        1,
        Nearest,
        "1.1e-100",
        "0x1.0E-83#1",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Down,
        "9.9900e-101",
        "0xd.fcE-84#10",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Up,
        "1.0001e-100",
        "0xe.00E-84#10",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Floor,
        "9.9900e-101",
        "0xd.fcE-84#10",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Ceiling,
        "1.0001e-100",
        "0xe.00E-84#10",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Nearest,
        "1.0001e-100",
        "0xe.00E-84#10",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        64,
        Nearest,
        "1.00000000000000001999e-100",
        "0xd.ff9772470298000E-84#64",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Down,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Up,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Floor,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Ceiling,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Nearest,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        64,
        Nearest,
        "1.57079632679489661926",
        "0x1.921fb54442d1846a#64",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        1,
        Nearest,
        "-0.12",
        "-0x0.2#1",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Down,
        "-0.099731",
        "-0x0.1988#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Up,
        "-0.099854",
        "-0x0.1990#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Floor,
        "-0.099854",
        "-0x0.1990#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Ceiling,
        "-0.099731",
        "-0x0.1988#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Nearest,
        "-0.099854",
        "-0x0.1990#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        64,
        Nearest,
        "-0.0998337487934866205948",
        "-0x0.198eb45e1ae820c94#64",
        Less,
    );
    test("-0.25", "-0x0.4#1", 1, Nearest, "-0.25", "-0x0.4#1", Less);
    test(
        "-0.25",
        "-0x0.4#1",
        10,
        Down,
        "-0.24731",
        "-0x0.3f5#10",
        Greater,
    );
    test("-0.25", "-0x0.4#1", 10, Up, "-0.24756", "-0x0.3f6#10", Less);
    test(
        "-0.25",
        "-0x0.4#1",
        10,
        Floor,
        "-0.24756",
        "-0x0.3f6#10",
        Less,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        10,
        Ceiling,
        "-0.24731",
        "-0x0.3f5#10",
        Greater,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        10,
        Nearest,
        "-0.24731",
        "-0x0.3f5#10",
        Greater,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        64,
        Nearest,
        "-0.247435798982431482760",
        "-0x0.3f57f3d87d3a7610c#64",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        1,
        Nearest,
        "-0.25",
        "-0x0.4#1",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Down,
        "-0.29541",
        "-0x0.4ba#10",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Up,
        "-0.29590",
        "-0x0.4bc#10",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Floor,
        "-0.29590",
        "-0x0.4bc#10",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Ceiling,
        "-0.29541",
        "-0x0.4ba#10",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Nearest,
        "-0.29541",
        "-0x0.4ba#10",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        64,
        Nearest,
        "-0.295598675984604127069",
        "-0x0.4bac5ad61843321a8#64",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        1,
        Nearest,
        "-7.6e-6",
        "-0x0.00008#1",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        Down,
        "-9.9987e-6",
        "-0x0.0000a7c#10",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        Up,
        "-0.000010014",
        "-0x0.0000a80#10",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        Floor,
        "-0.000010014",
        "-0x0.0000a80#10",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        Ceiling,
        "-9.9987e-6",
        "-0x0.0000a7c#10",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        Nearest,
        "-9.9987e-6",
        "-0x0.0000a7c#10",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        64,
        Nearest,
        "-9.99999999983333415104e-6",
        "-0x0.0000a7c5ac470f4512eb#64",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Ceiling,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        64,
        Nearest,
        "2.38256490488795107322e-323228497",
        "0x1.0000000000000000E-268435456#64",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Up,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Floor,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Up,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Floor,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Nearest,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        64,
        Nearest,
        "-2.38256490488795107322e-323228497",
        "-0x1.0000000000000000E-268435456#64",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Down,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Up,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Floor,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Ceiling,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Nearest,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        64,
        Nearest,
        "1.57079632679489661926",
        "0x1.921fb54442d1846a#64",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Down,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Up,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Floor,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Ceiling,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Nearest,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        64,
        Nearest,
        "1.57079632679489661926",
        "0x1.921fb54442d1846a#64",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Down,
        "2.7e-323228249",
        "0x1.0E-268435250#1",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Up,
        "5.3e-323228249",
        "0x2.0E-268435250#1",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Floor,
        "2.7e-323228249",
        "0x1.0E-268435250#1",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Ceiling,
        "5.3e-323228249",
        "0x2.0E-268435250#1",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Nearest,
        "2.7e-323228249",
        "0x1.0E-268435250#1",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Down,
        "3.9929e-323228249",
        "0x1.7f8E-268435250#10",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Up,
        "3.9981e-323228249",
        "0x1.800E-268435250#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Floor,
        "3.9929e-323228249",
        "0x1.7f8E-268435250#10",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Ceiling,
        "3.9981e-323228249",
        "0x1.800E-268435250#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Nearest,
        "3.9981e-323228249",
        "0x1.800E-268435250#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        64,
        Nearest,
        "3.99808871429393351007e-323228249",
        "0x1.8000000000000000E-268435250#64",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        1,
        Down,
        "-2.7e-323228249",
        "-0x1.0E-268435250#1",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        1,
        Up,
        "-5.3e-323228249",
        "-0x2.0E-268435250#1",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        1,
        Floor,
        "-5.3e-323228249",
        "-0x2.0E-268435250#1",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        1,
        Ceiling,
        "-2.7e-323228249",
        "-0x1.0E-268435250#1",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        1,
        Nearest,
        "-2.7e-323228249",
        "-0x1.0E-268435250#1",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Down,
        "-3.9929e-323228249",
        "-0x1.7f8E-268435250#10",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Up,
        "-3.9981e-323228249",
        "-0x1.800E-268435250#10",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Floor,
        "-3.9981e-323228249",
        "-0x1.800E-268435250#10",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Ceiling,
        "-3.9929e-323228249",
        "-0x1.7f8E-268435250#10",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Nearest,
        "-3.9981e-323228249",
        "-0x1.800E-268435250#10",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        64,
        Nearest,
        "-3.99808871429393351007e-323228249",
        "-0x1.8000000000000000E-268435250#64",
        Less,
    );
}

#[test]
#[should_panic]
fn gudermannian_prec_round_fail_1() {
    Float::ONE.gudermannian_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn gudermannian_prec_round_fail_2() {
    Float::from(0.5).gudermannian_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn gudermannian_prec_round_fail_3() {
    Float::INFINITY.gudermannian_prec_round(10, Exact);
}

// Whether the reference, whose time grows with the argument, is run on `x`.
fn reference_is_cheap(x: &Float) -> bool {
    x.get_exponent().is_some_and(|e| (-1000..=8).contains(&e))
}

// Checks a result that is finite and nonzero and did not overflow or underflow against the
// reference, which computes arctan(sinh(x)).
fn check_rounding(x: &Float, gudermannian: &Float, o: Ordering, prec: u64, rm: RoundingMode) {
    if rm != Exact
        && gudermannian.is_normal()
        && result_in_range(gudermannian, o)
        && reference_is_cheap(x)
    {
        let (g, go) = gudermannian_prec_round_naive(&float_to_big_exponent_float(x), prec, rm);
        assert_eq!(float_to_big_exponent_float(gudermannian), g);
        assert_eq!(o, go);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn gudermannian_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (gudermannian, o) = x.clone().gudermannian_prec_round(prec, rm);
    assert!(gudermannian.is_valid());
    assert_rounding_ordering_consistent(&gudermannian, rm, o);

    let (gudermannian_alt, o_alt) = x.gudermannian_prec_round_ref(prec, rm);
    assert!(gudermannian_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&gudermannian_alt),
        ComparableFloatRef(&gudermannian)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.gudermannian_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&gudermannian)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &gudermannian, o, prec, rm);

    // gd(-x) = -gd(x)
    let (neg, neg_o) = (-&x).gudermannian_prec_round(prec, -rm);
    assert_eq!(ComparableFloatRef(&-neg), ComparableFloatRef(&gudermannian));
    assert_eq!(neg_o.reverse(), o);

    if gudermannian.is_normal() {
        assert_eq!(gudermannian.get_prec(), Some(prec));
    }
    if !x.is_nan() {
        // gd is odd, and |gd(x)| < pi / 2.
        assert_eq!(gudermannian.is_sign_negative(), x.is_sign_negative());
        assert!(gudermannian.le_abs(&2u32));
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.gudermannian_prec_round_ref(prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(gudermannian.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.gudermannian_prec_round_ref(prec, Exact));
    }

    // A `Float` that is not too large or too small gives the same result as the equal `Rational`.
    if x.is_normal() && exponent_in_gate(&x) {
        let (r, ro) = Float::gudermannian_rational_prec_round(Rational::exact_from(&x), prec, rm);
        assert_eq!(ComparableFloatRef(&r), ComparableFloatRef(&gudermannian));
        assert_eq!(ro, o);
    }
}

#[test]
fn gudermannian_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_57().test_properties(|(x, prec, rm)| {
        gudermannian_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_rounding_mode_triple_gen_var_57().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            gudermannian_prec_round_properties_helper(x, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.gudermannian_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        // gd(±infinity) = ±pi/2, rounded, so `Exact` panics
        if rm == Exact {
            assert_panic!(Float::INFINITY.gudermannian_prec_round(prec, Exact));
            assert_panic!(Float::NEGATIVE_INFINITY.gudermannian_prec_round(prec, Exact));
        } else {
            let (s, o) = Float::INFINITY.gudermannian_prec_round(prec, rm);
            let (p, po) = Float::pi_over_2_prec_round(prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(p));
            assert_eq!(o, po);

            let (s, o) = Float::NEGATIVE_INFINITY.gudermannian_prec_round(prec, rm);
            let (p, po) = Float::pi_over_2_prec_round(prec, -rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(-p));
            assert_eq!(o, po.reverse());
        }

        let (s, o) = Float::ZERO.gudermannian_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.gudermannian_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn gudermannian_prec_properties_helper(x: Float, prec: u64) {
    let (gudermannian, o) = x.clone().gudermannian_prec(prec);
    assert!(gudermannian.is_valid());

    let (gudermannian_alt, o_alt) = x.gudermannian_prec_ref(prec);
    assert!(gudermannian_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&gudermannian_alt),
        ComparableFloatRef(&gudermannian)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.gudermannian_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&gudermannian)
    );
    assert_eq!(o_alt, o);

    let (gudermannian_alt, o_alt) = x.gudermannian_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&gudermannian_alt),
        ComparableFloatRef(&gudermannian)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &gudermannian, o, prec, Nearest);

    if gudermannian.is_normal() {
        assert_eq!(gudermannian.get_prec(), Some(prec));
    }
    if !x.is_nan() {
        // gd is odd, and |gd(x)| < pi / 2.
        assert_eq!(gudermannian.is_sign_negative(), x.is_sign_negative());
        assert!(gudermannian.le_abs(&2u32));
    }
}

#[test]
fn gudermannian_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        gudermannian_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        gudermannian_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        gudermannian_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (s, o) = Float::NAN.gudermannian_prec(prec);
        assert!(s.is_nan());
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn gudermannian_round_properties_helper(x: Float, rm: RoundingMode) {
    let (gudermannian, o) = x.clone().gudermannian_round(rm);
    assert!(gudermannian.is_valid());
    assert_rounding_ordering_consistent(&gudermannian, rm, o);

    let (gudermannian_alt, o_alt) = x.gudermannian_round_ref(rm);
    assert!(gudermannian_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&gudermannian_alt),
        ComparableFloatRef(&gudermannian)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.gudermannian_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&gudermannian)
    );
    assert_eq!(o_alt, o);

    let (gudermannian_alt, o_alt) = x.gudermannian_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&gudermannian_alt),
        ComparableFloatRef(&gudermannian)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &gudermannian, o, x.significant_bits(), rm);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.gudermannian_round_ref(rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(gudermannian.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.gudermannian_round_ref(Exact));
    }
}

#[test]
fn gudermannian_round_properties() {
    float_rounding_mode_pair_gen_var_54().test_properties(|(x, rm)| {
        gudermannian_round_properties_helper(x, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_rounding_mode_pair_gen_var_54().test_properties_with_config(&config, |(x, rm)| {
        gudermannian_round_properties_helper(x, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn gudermannian_properties_helper(x: Float) {
    let gudermannian = x.clone().gudermannian();
    assert!(gudermannian.is_valid());

    let gudermannian_alt = (&x).gudermannian();
    assert!(gudermannian_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&gudermannian_alt),
        ComparableFloatRef(&gudermannian)
    );

    let mut x_alt = x.clone();
    x_alt.gudermannian_assign();
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&gudermannian)
    );

    let gudermannian_alt = x
        .gudermannian_prec_round_ref(x.significant_bits(), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&gudermannian_alt),
        ComparableFloatRef(&gudermannian)
    );
    let gudermannian_alt = x.gudermannian_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&gudermannian_alt),
        ComparableFloatRef(&gudermannian)
    );
    let gudermannian_alt = x.gudermannian_round_ref(Nearest).0;
    assert_eq!(
        ComparableFloatRef(&gudermannian_alt),
        ComparableFloatRef(&gudermannian)
    );
    if !x.is_nan() {
        // gd is odd, and |gd(x)| < pi / 2.
        assert_eq!(gudermannian.is_sign_negative(), x.is_sign_negative());
        assert!(gudermannian.le_abs(&2u32));
    }
}

#[test]
fn gudermannian_properties() {
    float_gen().test_properties(|x| {
        gudermannian_properties_helper(x);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_gen().test_properties_with_config(&config, |x| {
        gudermannian_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        gudermannian_properties_helper(x);
    });
}

#[test]
fn test_primitive_float_gudermannian() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_gudermannian(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, 1.5707964);
    test::<f32>(f32::NEGATIVE_INFINITY, -1.5707964);
    test::<f32>(0.0, 0.0);
    test::<f32>(-0.0, -0.0);
    test::<f32>(1.0, 0.8657695);
    test::<f32>(2.7182817, 1.4390113);
    test::<f32>(7.0, 1.5689726);
    test::<f32>(100.0, 1.5707964);
    test::<f32>(1.0e-10, 1.0e-10);
    test::<f32>(-0.1, -0.09983375);
    test::<f32>(-0.25, -0.2474358);
    test::<f32>(-0.36787942, -0.35985175);
    test::<f32>(-0.5, -0.48038107);
    test::<f32>(3.4028235e38, 1.5707964);
    test::<f32>(1.1754944e-38, 1.1754944e-38);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, core::f64::consts::FRAC_PI_2);
    test::<f64>(f64::NEGATIVE_INFINITY, -core::f64::consts::FRAC_PI_2);
    test::<f64>(0.0, 0.0);
    test::<f64>(-0.0, -0.0);
    test::<f64>(1.0, 0.8657694832396586);
    test::<f64>(core::f64::consts::E, 1.439011315963675);
    test::<f64>(7.0, 1.5689725633692915);
    test::<f64>(100.0, core::f64::consts::FRAC_PI_2);
    test::<f64>(1.0e-10, 1.0e-10);
    test::<f64>(-0.1, -0.09983374879348662);
    test::<f64>(-0.25, -0.24743579898243148);
    test::<f64>(-0.36787944117144233, -0.3598517583832039);
    test::<f64>(-0.5, -0.48038107913372946);
    test::<f64>(1.7976931348623157e308, core::f64::consts::FRAC_PI_2);
    test::<f64>(2.2250738585072014e-308, 2.2250738585072014e-308);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_gudermannian_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        primitive_float_gudermannian(x);
    });
}

#[test]
fn primitive_float_gudermannian_properties() {
    apply_fn_to_primitive_floats!(primitive_float_gudermannian_properties_helper);
}

#[test]
fn test_gudermannian_rational_prec() {
    let test = |s, prec, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::gudermannian_rational_prec(x.clone(), prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::gudermannian_rational_prec_ref(&x, prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 1, "0.0", "0x0.0", Equal);
    test("0", 10, "0.0", "0x0.0", Equal);
    test("1", 1, "1.0", "0x1.0#1", Greater);
    test("1", 10, "0.86621", "0x0.ddc#10", Greater);
    test(
        "1",
        100,
        "0.86576948323965862428960184619172",
        "0x0.dda311a0639f976612e00f5aa#100",
        Less,
    );
    test("1/3", 1, "0.25", "0x0.4#1", Less);
    test("1/3", 10, "0.32715", "0x0.53c#10", Less);
    test(
        "1/3",
        100,
        "0.32732661447167714131646430263315",
        "0x0.53cbad50442932d5350ee98d08#100",
        Greater,
    );
    test("-1/3", 1, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 10, "-0.32715", "-0x0.53c#10", Greater);
    test(
        "-1/3",
        100,
        "-0.32732661447167714131646430263315",
        "-0x0.53cbad50442932d5350ee98d08#100",
        Less,
    );
    test("22/7", 1, "1.0", "0x1.0#1", Less);
    test("22/7", 10, "1.4844", "0x1.7c0#10", Less);
    test(
        "22/7",
        100,
        "1.4845312444284462431815765770975",
        "0x1.7c0a3d58b5da13787edcfb56e#100",
        Less,
    );
    test("-1/4", 1, "-0.25", "-0x0.4#1", Less);
    test("-1/4", 10, "-0.24731", "-0x0.3f5#10", Greater);
    test(
        "-1/4",
        100,
        "-0.24743579898243148276130299863681",
        "-0x0.3f57f3d87d3a7610c7a7bc13d4#100",
        Less,
    );
    test("-1/1000", 1, "-0.00098", "-0x0.004#1", Greater);
    test("-1/1000", 10, "-0.00099945", "-0x0.00418#10", Greater);
    test(
        "-1/1000",
        100,
        "-0.00099999983333337499998789682921282",
        "-0x0.0041893694862bee9ca27fbdc700#100",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        1,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        100,
        "1.5707963267948966192313216916397",
        "0x1.921fb54442d18469898cc5170#100",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        1,
        "7.9e-31",
        "0x1.0E-25#1",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        "9.9994e-31",
        "0x1.448E-25#10",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        100,
        "1.0000000000000000000000000000005e-30",
        "0x1.4484bfeebc29f863424b06f36E-25#100",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        1,
        "-7.9e-31",
        "-0x1.0E-25#1",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        "-9.9994e-31",
        "-0x1.448E-25#10",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        100,
        "-1.0000000000000000000000000000005e-30",
        "-0x1.4484bfeebc29f863424b06f36E-25#100",
        Less,
    );
    test("-11/30", 1, "-0.25", "-0x0.4#1", Greater);
    test("-11/30", 10, "-0.35889", "-0x0.5be#10", Less);
    test(
        "-11/30",
        100,
        "-0.35871642108764440385899669712973",
        "-0x0.5bd4d6e11c0e9454afa054deb8#100",
        Less,
    );
    test("-3/8", 10, "-0.36670", "-0x0.5de#10", Less);
    test(
        "-4598493014643029/12500000000000000",
        1,
        "-0.25",
        "-0x0.4#1",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        "-0.35986",
        "-0x0.5c2#10",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        100,
        "-0.35985175838320386137675293755588",
        "-0x0.5c1f3eada9f8eab948985a4b88#100",
        Greater,
    );
    test(
        "-36787944117144233/100000000000000000",
        10,
        "-0.35986",
        "-0x0.5c2#10",
        Less,
    );
}

#[test]
#[should_panic]
fn gudermannian_rational_prec_fail() {
    Float::gudermannian_rational_prec(Rational::ONE, 0);
}

#[test]
#[should_panic]
fn gudermannian_rational_prec_ref_fail() {
    Float::gudermannian_rational_prec_ref(&Rational::ONE, 0);
}

#[test]
fn test_gudermannian_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::gudermannian_rational_prec_round(x.clone(), prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::gudermannian_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Down, "0.0", "0x0.0", Equal);
    test("0", 10, Up, "0.0", "0x0.0", Equal);
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "1",
        53,
        Nearest,
        "0.86576948323965863",
        "0x0.dda311a0639f98#53",
        Greater,
    );
    test("1", 10, Floor, "0.86523", "0x0.dd8#10", Less);
    test("1", 10, Ceiling, "0.86621", "0x0.ddc#10", Greater);
    test("1", 10, Down, "0.86523", "0x0.dd8#10", Less);
    test("1", 10, Up, "0.86621", "0x0.ddc#10", Greater);
    test("1", 10, Nearest, "0.86621", "0x0.ddc#10", Greater);
    test(
        "1/3",
        53,
        Nearest,
        "0.32732661447167716",
        "0x0.53cbad50442934#53",
        Greater,
    );
    test("1/3", 10, Floor, "0.32715", "0x0.53c#10", Less);
    test("1/3", 10, Ceiling, "0.32764", "0x0.53e#10", Greater);
    test("1/3", 10, Down, "0.32715", "0x0.53c#10", Less);
    test("1/3", 10, Up, "0.32764", "0x0.53e#10", Greater);
    test("1/3", 10, Nearest, "0.32715", "0x0.53c#10", Less);
    test(
        "-1/3",
        53,
        Nearest,
        "-0.32732661447167716",
        "-0x0.53cbad50442934#53",
        Less,
    );
    test("-1/3", 10, Floor, "-0.32764", "-0x0.53e#10", Less);
    test("-1/3", 10, Ceiling, "-0.32715", "-0x0.53c#10", Greater);
    test("-1/3", 10, Down, "-0.32715", "-0x0.53c#10", Greater);
    test("-1/3", 10, Up, "-0.32764", "-0x0.53e#10", Less);
    test("-1/3", 10, Nearest, "-0.32715", "-0x0.53c#10", Greater);
    test(
        "22/7",
        53,
        Nearest,
        "1.4845312444284462",
        "0x1.7c0a3d58b5da1#53",
        Less,
    );
    test("22/7", 10, Floor, "1.4844", "0x1.7c0#10", Less);
    test("22/7", 10, Ceiling, "1.4863", "0x1.7c8#10", Greater);
    test("22/7", 10, Down, "1.4844", "0x1.7c0#10", Less);
    test("22/7", 10, Up, "1.4863", "0x1.7c8#10", Greater);
    test("22/7", 10, Nearest, "1.4844", "0x1.7c0#10", Less);
    test(
        "-1/4",
        53,
        Nearest,
        "-0.24743579898243148",
        "-0x0.3f57f3d87d3a76#53",
        Greater,
    );
    test("-1/4", 10, Floor, "-0.24756", "-0x0.3f6#10", Less);
    test("-1/4", 10, Ceiling, "-0.24731", "-0x0.3f5#10", Greater);
    test("-1/4", 10, Down, "-0.24731", "-0x0.3f5#10", Greater);
    test("-1/4", 10, Up, "-0.24756", "-0x0.3f6#10", Less);
    test("-1/4", 10, Nearest, "-0.24731", "-0x0.3f5#10", Greater);
    test(
        "-1/1000",
        53,
        Nearest,
        "-0.00099999983333337508",
        "-0x0.0041893694862bf0#53",
        Less,
    );
    test("-1/1000", 10, Floor, "-0.0010014", "-0x0.0041a#10", Less);
    test(
        "-1/1000",
        10,
        Ceiling,
        "-0.00099945",
        "-0x0.00418#10",
        Greater,
    );
    test("-1/1000", 10, Down, "-0.00099945", "-0x0.00418#10", Greater);
    test("-1/1000", 10, Up, "-0.0010014", "-0x0.0041a#10", Less);
    test(
        "-1/1000",
        10,
        Nearest,
        "-0.00099945",
        "-0x0.00418#10",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Floor,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Ceiling,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Down,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Up,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Nearest,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        53,
        Nearest,
        "1.0000000000000001e-30",
        "0x1.4484bfeebc2a0E-25#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Floor,
        "9.9994e-31",
        "0x1.448E-25#10",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Ceiling,
        "1.0015e-30",
        "0x1.450E-25#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Down,
        "9.9994e-31",
        "0x1.448E-25#10",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Up,
        "1.0015e-30",
        "0x1.450E-25#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Nearest,
        "9.9994e-31",
        "0x1.448E-25#10",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        53,
        Nearest,
        "-1.0000000000000001e-30",
        "-0x1.4484bfeebc2a0E-25#53",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Floor,
        "-1.0015e-30",
        "-0x1.450E-25#10",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Ceiling,
        "-9.9994e-31",
        "-0x1.448E-25#10",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Down,
        "-9.9994e-31",
        "-0x1.448E-25#10",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Up,
        "-1.0015e-30",
        "-0x1.450E-25#10",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Nearest,
        "-9.9994e-31",
        "-0x1.448E-25#10",
        Greater,
    );
    test(
        "-11/30",
        53,
        Nearest,
        "-0.35871642108764440",
        "-0x0.5bd4d6e11c0e94#53",
        Greater,
    );
    test("-11/30", 10, Floor, "-0.35889", "-0x0.5be#10", Less);
    test("-11/30", 10, Ceiling, "-0.35840", "-0x0.5bc#10", Greater);
    test("-11/30", 10, Down, "-0.35840", "-0x0.5bc#10", Greater);
    test("-11/30", 10, Up, "-0.35889", "-0x0.5be#10", Less);
    test("-11/30", 10, Nearest, "-0.35889", "-0x0.5be#10", Less);
    test("-3/8", 10, Nearest, "-0.36670", "-0x0.5de#10", Less);
    test(
        "-4598493014643029/12500000000000000",
        53,
        Nearest,
        "-0.35985175838320388",
        "-0x0.5c1f3eada9f8ec#53",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Floor,
        "-0.35986",
        "-0x0.5c2#10",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Ceiling,
        "-0.35938",
        "-0x0.5c0#10",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Down,
        "-0.35938",
        "-0x0.5c0#10",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Up,
        "-0.35986",
        "-0x0.5c2#10",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Nearest,
        "-0.35986",
        "-0x0.5c2#10",
        Less,
    );
    test(
        "-36787944117144233/100000000000000000",
        10,
        Nearest,
        "-0.35986",
        "-0x0.5c2#10",
        Less,
    );
}

#[test]
#[should_panic]
fn gudermannian_rational_prec_round_fail_1() {
    Float::gudermannian_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn gudermannian_rational_prec_round_fail_2() {
    Float::gudermannian_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[test]
#[should_panic]
fn gudermannian_rational_prec_round_ref_fail() {
    Float::gudermannian_rational_prec_round_ref(&Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn gudermannian_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (gudermannian, o) = Float::gudermannian_rational_prec_round(x.clone(), prec, rm);
    assert!(gudermannian.is_valid());
    assert_rounding_ordering_consistent(&gudermannian, rm, o);

    let (gudermannian_alt, o_alt) = Float::gudermannian_rational_prec_round_ref(&x, prec, rm);
    assert!(gudermannian_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&gudermannian_alt),
        ComparableFloatRef(&gudermannian)
    );
    assert_eq!(o_alt, o);

    // The reference computes arctan(sinh(x)), in time that grows with x.
    if rm != Exact
        && gudermannian.is_normal()
        && result_in_range(&gudermannian, o)
        && x.le_abs(&256u32)
    {
        let (g, go) = gudermannian_rational_prec_round_naive(&x, prec, rm);
        assert_eq!(float_to_big_exponent_float(&gudermannian), g);
        assert_eq!(o, go);
    }

    if gudermannian.is_normal() {
        assert_eq!(gudermannian.get_prec(), Some(prec));
    }
    assert_eq!(gudermannian.partial_cmp(&0u32), x.partial_cmp(&0u32));
    assert!(gudermannian.le_abs(&2u32));

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::gudermannian_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(gudermannian.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::gudermannian_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn gudermannian_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        gudermannian_rational_prec_round_properties_helper(x, prec, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn gudermannian_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (gudermannian, o) = Float::gudermannian_rational_prec(x.clone(), prec);
    assert!(gudermannian.is_valid());

    let (gudermannian_alt, o_alt) = Float::gudermannian_rational_prec_ref(&x, prec);
    assert!(gudermannian_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&gudermannian_alt),
        ComparableFloatRef(&gudermannian)
    );
    assert_eq!(o_alt, o);

    let (gudermannian_alt, o_alt) = Float::gudermannian_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&gudermannian_alt),
        ComparableFloatRef(&gudermannian)
    );
    assert_eq!(o_alt, o);
}

#[test]
fn gudermannian_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        gudermannian_rational_prec_properties_helper(x, prec);
    });
}

#[test]
fn test_primitive_float_gudermannian_rational() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_gudermannian_rational::<T>(
                &Rational::from_str(s).unwrap()
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0.0);
    test::<f64>("0", 0.0);
    test::<f32>("1", 0.8657695);
    test::<f64>("1", 0.8657694832396586);
    test::<f32>("1/3", 0.32732663);
    test::<f64>("1/3", 0.32732661447167716);
    test::<f32>("-1/3", -0.32732663);
    test::<f64>("-1/3", -0.32732661447167716);
    test::<f32>("-1/4", -0.2474358);
    test::<f64>("-1/4", -0.24743579898243148);
    test::<f32>("-1/2", -0.48038107);
    test::<f64>("-1/2", -0.48038107913372946);
    test::<f32>("1000", 1.5707964);
    test::<f64>("1000", core::f64::consts::FRAC_PI_2);
    test::<f32>("-1/1000", -0.0009999998);
    test::<f64>("-1/1000", -0.000999999833333375);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_gudermannian_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        primitive_float_gudermannian_rational::<T>(&x);
    });
}

#[test]
fn primitive_float_gudermannian_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_gudermannian_rational_properties_helper);
}
