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
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Two, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::hyperbolic::traits::{InverseGudermannian, InverseGudermannianAssign};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::hyperbolic::inverse_gudermannian::{
    primitive_float_inverse_gudermannian, primitive_float_inverse_gudermannian_rational,
};
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, inverse_gudermannian_prec_round_naive,
    inverse_gudermannian_rational_prec_round_naive,
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
fn test_inverse_gudermannian() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let inverse_gudermannian = x.clone().inverse_gudermannian();
        assert!(inverse_gudermannian.is_valid());

        assert_eq!(inverse_gudermannian.to_string(), out);
        assert_eq!(to_hex_string(&inverse_gudermannian), out_hex);

        let inverse_gudermannian_alt = (&x).inverse_gudermannian();
        assert!(inverse_gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&inverse_gudermannian),
            ComparableFloatRef(&inverse_gudermannian_alt)
        );

        let mut inverse_gudermannian_alt = x.clone();
        inverse_gudermannian_alt.inverse_gudermannian_assign();
        assert!(inverse_gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&inverse_gudermannian),
            ComparableFloatRef(&inverse_gudermannian_alt)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "NaN", "NaN");
    test("-Infinity", "-Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0");
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("0.50", "0x0.8#1", "0.50", "0x0.8#1");
    test("2.0", "0x2.0#1", "NaN", "NaN");
    test("2.7182818284590451", "0x2.b7e151628aed2#53", "NaN", "NaN");
    test("10.0", "0xa.0#3", "NaN", "NaN");
    test("1.00000000e10", "0x2.540be4E+8#24", "NaN", "NaN");
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        "0.000010000000000166668",
        "0x0.0000a7c5ac47274a00#53",
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        "NaN",
        "NaN",
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
    );
    test("123.45600000000000", "0x7b.74bc6a7ef9dc#53", "NaN", "NaN");
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        "-0.10016708454748019",
        "-0x0.19a48cd044604c#52",
    );
    test("-0.25", "-0x0.4#1", "-0.25", "-0x0.4#1");
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        "-0.30460397440170411",
        "-0x0.4dfa86ac4974ec#53",
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        "-0.000010000000000166668",
        "-0x0.0000a7c5ac47274a00#53",
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
    test("1.0e323228496", "0x4.0E+268435455#1", "NaN", "NaN");
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        "NaN",
        "NaN",
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
fn test_inverse_gudermannian_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (inverse_gudermannian, o) = x.clone().inverse_gudermannian_prec(prec);
        assert!(inverse_gudermannian.is_valid());

        assert_eq!(inverse_gudermannian.to_string(), out);
        assert_eq!(to_hex_string(&inverse_gudermannian), out_hex);
        assert_eq!(o, o_out);

        let (inverse_gudermannian_alt, o_alt) = x.inverse_gudermannian_prec_ref(prec);
        assert!(inverse_gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&inverse_gudermannian),
            ComparableFloatRef(&inverse_gudermannian_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut inverse_gudermannian_alt = x.clone();
        let o_alt = inverse_gudermannian_alt.inverse_gudermannian_prec_assign(prec);
        assert!(inverse_gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&inverse_gudermannian),
            ComparableFloatRef(&inverse_gudermannian_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", 10, "1.2266", "0x1.3a0#10", Greater);
    test(
        "1.0",
        "0x1.0#1",
        100,
        "1.2261911708835170708130609674712",
        "0x1.39e7aa2196b1faa23d907321e#100",
        Less,
    );
    test("0.50", "0x0.8#1", 1, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", 10, "0.52246", "0x0.85c#10", Greater);
    test(
        "0.50",
        "0x0.8#1",
        100,
        "0.52223810327844033018988714493618",
        "0x0.85b165764e54279fbb823aabc#100",
        Less,
    );
    test("2.0", "0x2.0#1", 1, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 100, "NaN", "NaN", Equal);
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        100,
        "NaN",
        "NaN",
        Equal,
    );
    test("10.0", "0xa.0#3", 1, "NaN", "NaN", Equal);
    test("10.0", "0xa.0#3", 10, "NaN", "NaN", Equal);
    test("10.0", "0xa.0#3", 100, "NaN", "NaN", Equal);
    test("1.00000000e10", "0x2.540be4E+8#24", 1, "NaN", "NaN", Equal);
    test("1.00000000e10", "0x2.540be4E+8#24", 10, "NaN", "NaN", Equal);
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        100,
        "NaN",
        "NaN",
        Equal,
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
        "0.000010000000000166667484701372514549",
        "0x0.0000a7c5ac472749fd149b83829b6#100",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        100,
        "NaN",
        "NaN",
        Equal,
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
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        100,
        "NaN",
        "NaN",
        Equal,
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
        "-0.10022",
        "-0x0.19a8#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        100,
        "-0.10016708454748018807480331513681",
        "-0x0.19a48cd044604be32a1b2246ac#100",
        Less,
    );
    test("-0.25", "-0x0.4#1", 1, "-0.25", "-0x0.4#1", Greater);
    test("-0.25", "-0x0.4#1", 10, "-0.25244", "-0x0.40a#10", Greater);
    test(
        "-0.25",
        "-0x0.4#1",
        100,
        "-0.25264561035786753829411005567848",
        "-0x0.40ad61f9f70a3c520cfb1912a0#100",
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
        "-0.30469",
        "-0x0.4e0#10",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        100,
        "-0.30460397440170409217179017685850",
        "-0x0.4dfa86ac4974ea6da585798f80#100",
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
        "-0.000010000000000166667484701372514549",
        "-0x0.0000a7c5ac472749fd149b83829b6#100",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        100,
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        100,
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        100,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        100,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        "5.3e-323228249",
        "0x2.0E-268435250#1",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        "3.9981e-323228249",
        "0x1.800E-268435250#10",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        100,
        "3.9980887142939335100715454835713e-323228249",
        "0x1.8000000000000000000000000E-268435250#100",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        1,
        "-5.3e-323228249",
        "-0x2.0E-268435250#1",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        "-3.9981e-323228249",
        "-0x1.800E-268435250#10",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        100,
        "-3.9980887142939335100715454835713e-323228249",
        "-0x1.8000000000000000000000000E-268435250#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn inverse_gudermannian_prec_fail() {
    Float::NAN.inverse_gudermannian_prec(0);
}

#[test]
fn test_inverse_gudermannian_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (inverse_gudermannian, o) = x.clone().inverse_gudermannian_round(rm);
        assert!(inverse_gudermannian.is_valid());

        assert_eq!(inverse_gudermannian.to_string(), out);
        assert_eq!(to_hex_string(&inverse_gudermannian), out_hex);
        assert_eq!(o, o_out);

        let (inverse_gudermannian_alt, o_alt) = x.inverse_gudermannian_round_ref(rm);
        assert!(inverse_gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&inverse_gudermannian),
            ComparableFloatRef(&inverse_gudermannian_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut inverse_gudermannian_alt = x.clone();
        let o_alt = inverse_gudermannian_alt.inverse_gudermannian_round_assign(rm);
        assert!(inverse_gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&inverse_gudermannian),
            ComparableFloatRef(&inverse_gudermannian_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    test("NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", Ceiling, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", Down, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", Up, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Up, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Nearest, "NaN", "NaN", Equal);
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
    test("1.0", "0x1.0#1", Floor, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", Ceiling, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", Down, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", Up, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", Nearest, "1.0", "0x1.0#1", Less);
    test("0.50", "0x0.8#1", Floor, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", Ceiling, "1.0", "0x1.0#1", Greater);
    test("0.50", "0x0.8#1", Down, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", Up, "1.0", "0x1.0#1", Greater);
    test("0.50", "0x0.8#1", Nearest, "0.50", "0x0.8#1", Less);
    test("2.0", "0x2.0#1", Floor, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", Ceiling, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", Down, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", Up, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", Nearest, "NaN", "NaN", Equal);
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("10.0", "0xa.0#3", Floor, "NaN", "NaN", Equal);
    test("10.0", "0xa.0#3", Ceiling, "NaN", "NaN", Equal);
    test("10.0", "0xa.0#3", Down, "NaN", "NaN", Equal);
    test("10.0", "0xa.0#3", Up, "NaN", "NaN", Equal);
    test("10.0", "0xa.0#3", Nearest, "NaN", "NaN", Equal);
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test("1.00000000e10", "0x2.540be4E+8#24", Up, "NaN", "NaN", Equal);
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Floor,
        "0.000010000000000166666",
        "0x0.0000a7c5ac472749f8#53",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Ceiling,
        "0.000010000000000166668",
        "0x0.0000a7c5ac47274a00#53",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Down,
        "0.000010000000000166666",
        "0x0.0000a7c5ac472749f8#53",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Up,
        "0.000010000000000166668",
        "0x0.0000a7c5ac47274a00#53",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Nearest,
        "0.000010000000000166668",
        "0x0.0000a7c5ac47274a00#53",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Floor,
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Ceiling,
        "1.000000000000002e-100",
        "0xd.ff97724702a0E-84#49",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Down,
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Up,
        "1.000000000000002e-100",
        "0xd.ff97724702a0E-84#49",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Nearest,
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Floor,
        "-0.10016708454748019",
        "-0x0.19a48cd044604c#52",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Ceiling,
        "-0.10016708454748016",
        "-0x0.19a48cd044604a#52",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Down,
        "-0.10016708454748016",
        "-0x0.19a48cd044604a#52",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Up,
        "-0.10016708454748019",
        "-0x0.19a48cd044604c#52",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Nearest,
        "-0.10016708454748019",
        "-0x0.19a48cd044604c#52",
        Less,
    );
    test("-0.25", "-0x0.4#1", Floor, "-0.50", "-0x0.8#1", Less);
    test("-0.25", "-0x0.4#1", Ceiling, "-0.25", "-0x0.4#1", Greater);
    test("-0.25", "-0x0.4#1", Down, "-0.25", "-0x0.4#1", Greater);
    test("-0.25", "-0x0.4#1", Up, "-0.50", "-0x0.8#1", Less);
    test("-0.25", "-0x0.4#1", Nearest, "-0.25", "-0x0.4#1", Greater);
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Floor,
        "-0.30460397440170411",
        "-0x0.4dfa86ac4974ec#53",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Ceiling,
        "-0.30460397440170406",
        "-0x0.4dfa86ac4974e8#53",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Down,
        "-0.30460397440170406",
        "-0x0.4dfa86ac4974e8#53",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Up,
        "-0.30460397440170411",
        "-0x0.4dfa86ac4974ec#53",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Nearest,
        "-0.30460397440170411",
        "-0x0.4dfa86ac4974ec#53",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Floor,
        "-0.000010000000000166668",
        "-0x0.0000a7c5ac47274a00#53",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Ceiling,
        "-0.000010000000000166666",
        "-0x0.0000a7c5ac472749f8#53",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Down,
        "-0.000010000000000166666",
        "-0x0.0000a7c5ac472749f8#53",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Up,
        "-0.000010000000000166668",
        "-0x0.0000a7c5ac47274a00#53",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Nearest,
        "-0.000010000000000166668",
        "-0x0.0000a7c5ac47274a00#53",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Floor,
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Ceiling,
        "2.3825649048879510732161697817364e-323228497",
        "0x1.0000000000000000000000002E-268435456#100",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Down,
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Up,
        "2.3825649048879510732161697817364e-323228497",
        "0x1.0000000000000000000000002E-268435456#100",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Nearest,
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Floor,
        "-2.3825649048879510732161697817364e-323228497",
        "-0x1.0000000000000000000000002E-268435456#100",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Ceiling,
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Down,
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Up,
        "-2.3825649048879510732161697817364e-323228497",
        "-0x1.0000000000000000000000002E-268435456#100",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Nearest,
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Floor,
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Ceiling,
        "5.3e-323228249",
        "0x2.0E-268435250#2",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Down,
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Up,
        "5.3e-323228249",
        "0x2.0E-268435250#2",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Nearest,
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Floor,
        "-5.3e-323228249",
        "-0x2.0E-268435250#2",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Ceiling,
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Down,
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Up,
        "-5.3e-323228249",
        "-0x2.0E-268435250#2",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Nearest,
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Greater,
    );
}

#[test]
#[should_panic]
fn inverse_gudermannian_round_fail() {
    Float::from(0.5).inverse_gudermannian_round(Exact);
}

#[test]
fn test_inverse_gudermannian_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (inverse_gudermannian, o) = x.clone().inverse_gudermannian_prec_round(prec, rm);
        assert!(inverse_gudermannian.is_valid());

        assert_eq!(inverse_gudermannian.to_string(), out);
        assert_eq!(to_hex_string(&inverse_gudermannian), out_hex);
        assert_eq!(o, o_out);

        let (inverse_gudermannian_alt, o_alt) = x.inverse_gudermannian_prec_round_ref(prec, rm);
        assert!(inverse_gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&inverse_gudermannian),
            ComparableFloatRef(&inverse_gudermannian_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut inverse_gudermannian_alt = x.clone();
        let o_alt = inverse_gudermannian_alt.inverse_gudermannian_prec_round_assign(prec, rm);
        assert!(inverse_gudermannian_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&inverse_gudermannian),
            ComparableFloatRef(&inverse_gudermannian_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    // The expected values agree with Azurite's verified inverse Gudermannian function.
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Ceiling, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Down, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Up, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Up, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Nearest, "NaN", "NaN", Equal);
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
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", 10, Down, "1.2246", "0x1.398#10", Less);
    test("1.0", "0x1.0#1", 10, Up, "1.2266", "0x1.3a0#10", Greater);
    test("1.0", "0x1.0#1", 10, Floor, "1.2246", "0x1.398#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "1.2266",
        "0x1.3a0#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "1.2266",
        "0x1.3a0#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        64,
        Nearest,
        "1.22619117088351707080",
        "0x1.39e7aa2196b1faa2#64",
        Less,
    );
    test("0.50", "0x0.8#1", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", 10, Down, "0.52148", "0x0.858#10", Less);
    test("0.50", "0x0.8#1", 10, Up, "0.52246", "0x0.85c#10", Greater);
    test("0.50", "0x0.8#1", 10, Floor, "0.52148", "0x0.858#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "0.52246",
        "0x0.85c#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        10,
        Nearest,
        "0.52246",
        "0x0.85c#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        64,
        Nearest,
        "0.522238103278440330204",
        "0x0.85b165764e5427a0#64",
        Greater,
    );
    test("2.0", "0x2.0#1", 1, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Down, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Up, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Floor, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 64, Nearest, "NaN", "NaN", Equal);
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        64,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("10.0", "0xa.0#3", 1, Nearest, "NaN", "NaN", Equal);
    test("10.0", "0xa.0#3", 10, Down, "NaN", "NaN", Equal);
    test("10.0", "0xa.0#3", 10, Up, "NaN", "NaN", Equal);
    test("10.0", "0xa.0#3", 10, Floor, "NaN", "NaN", Equal);
    test("10.0", "0xa.0#3", 10, Ceiling, "NaN", "NaN", Equal);
    test("10.0", "0xa.0#3", 10, Nearest, "NaN", "NaN", Equal);
    test("10.0", "0xa.0#3", 64, Nearest, "NaN", "NaN", Equal);
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        64,
        Nearest,
        "NaN",
        "NaN",
        Equal,
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
        "0.0000100000000001666674850",
        "0x0.0000a7c5ac472749fd15#64",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        64,
        Nearest,
        "NaN",
        "NaN",
        Equal,
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
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        64,
        Nearest,
        "NaN",
        "NaN",
        Equal,
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
        "-0.10010",
        "-0x0.19a0#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Up,
        "-0.10022",
        "-0x0.19a8#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Floor,
        "-0.10022",
        "-0x0.19a8#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Ceiling,
        "-0.10010",
        "-0x0.19a0#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Nearest,
        "-0.10022",
        "-0x0.19a8#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        64,
        Nearest,
        "-0.100167084547480188073",
        "-0x0.19a48cd044604be32#64",
        Greater,
    );
    test(
        "-0.25", "-0x0.4#1", 1, Nearest, "-0.25", "-0x0.4#1", Greater,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        10,
        Down,
        "-0.25244",
        "-0x0.40a#10",
        Greater,
    );
    test("-0.25", "-0x0.4#1", 10, Up, "-0.25293", "-0x0.40c#10", Less);
    test(
        "-0.25",
        "-0x0.4#1",
        10,
        Floor,
        "-0.25293",
        "-0x0.40c#10",
        Less,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        10,
        Ceiling,
        "-0.25244",
        "-0x0.40a#10",
        Greater,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        10,
        Nearest,
        "-0.25244",
        "-0x0.40a#10",
        Greater,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        64,
        Nearest,
        "-0.252645610357867538291",
        "-0x0.40ad61f9f70a3c520#64",
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
        "-0.30420",
        "-0x0.4de#10",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Up,
        "-0.30469",
        "-0x0.4e0#10",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Floor,
        "-0.30469",
        "-0x0.4e0#10",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Ceiling,
        "-0.30420",
        "-0x0.4de#10",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Nearest,
        "-0.30469",
        "-0x0.4e0#10",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        64,
        Nearest,
        "-0.304603974401704092164",
        "-0x0.4dfa86ac4974ea6d8#64",
        Greater,
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
        "-0.0000100000000001666674850",
        "-0x0.0000a7c5ac472749fd15#64",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Down,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Up,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Floor,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Ceiling,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Down,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Up,
        "2.3872e-323228497",
        "0x1.008E-268435456#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Floor,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Ceiling,
        "2.3872e-323228497",
        "0x1.008E-268435456#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        64,
        Nearest,
        "2.38256490488795107322e-323228497",
        "0x1.0000000000000000E-268435456#64",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Down,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Up,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Floor,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Ceiling,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Down,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Up,
        "-2.3872e-323228497",
        "-0x1.008E-268435456#10",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Floor,
        "-2.3872e-323228497",
        "-0x1.008E-268435456#10",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Ceiling,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Nearest,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        64,
        Nearest,
        "-2.38256490488795107322e-323228497",
        "-0x1.0000000000000000E-268435456#64",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        64,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        64,
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
        "5.3e-323228249",
        "0x2.0E-268435250#1",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Down,
        "3.9981e-323228249",
        "0x1.800E-268435250#10",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Up,
        "4.0033e-323228249",
        "0x1.808E-268435250#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Floor,
        "3.9981e-323228249",
        "0x1.800E-268435250#10",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Ceiling,
        "4.0033e-323228249",
        "0x1.808E-268435250#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Nearest,
        "3.9981e-323228249",
        "0x1.800E-268435250#10",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        64,
        Nearest,
        "3.99808871429393351007e-323228249",
        "0x1.8000000000000000E-268435250#64",
        Less,
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
        "-5.3e-323228249",
        "-0x2.0E-268435250#1",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Down,
        "-3.9981e-323228249",
        "-0x1.800E-268435250#10",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Up,
        "-4.0033e-323228249",
        "-0x1.808E-268435250#10",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Floor,
        "-4.0033e-323228249",
        "-0x1.808E-268435250#10",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Ceiling,
        "-3.9981e-323228249",
        "-0x1.800E-268435250#10",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Nearest,
        "-3.9981e-323228249",
        "-0x1.800E-268435250#10",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        64,
        Nearest,
        "-3.99808871429393351007e-323228249",
        "-0x1.8000000000000000E-268435250#64",
        Greater,
    );
    test("1.5", "0x1.8#2", 10, Nearest, "3.3398", "0x3.57#10", Less);
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        53,
        Nearest,
        "38.025003373828866",
        "0x26.06669f00e278#53",
        Less,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        100,
        Floor,
        "38.025003373828868061802405161167",
        "0x26.06669f00e27876a3a88273cc#100",
        Less,
    );
    test(
        "1.5707963267948968",
        "0x1.921fb54442d19#53",
        53,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.5707963267948966",
        "-0x1.921fb54442d18#53",
        20,
        Up,
        "-38.025024",
        "-0x26.0668#20",
        Less,
    );
}

#[test]
#[should_panic]
fn inverse_gudermannian_prec_round_fail_1() {
    Float::ONE.inverse_gudermannian_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn inverse_gudermannian_prec_round_fail_2() {
    Float::from(0.5).inverse_gudermannian_prec_round(10, Exact);
}

// Whether the reference, which would take as many bits as a tiny argument has leading zeros, is run
// on `x`.
fn reference_is_cheap(x: &Float) -> bool {
    x.get_exponent().is_some_and(|e| e >= -1000)
}

// Checks a result that is finite and nonzero and did not overflow or underflow against the
// reference, which computes atanh(sin(x)).
fn check_rounding(
    x: &Float,
    inverse_gudermannian: &Float,
    o: Ordering,
    prec: u64,
    rm: RoundingMode,
) {
    if rm != Exact
        && inverse_gudermannian.is_normal()
        && result_in_range(inverse_gudermannian, o)
        && reference_is_cheap(x)
    {
        let (g, go) =
            inverse_gudermannian_prec_round_naive(&float_to_big_exponent_float(x), prec, rm);
        assert_eq!(float_to_big_exponent_float(inverse_gudermannian), g);
        assert_eq!(o, go);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn inverse_gudermannian_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (inverse_gudermannian, o) = x.clone().inverse_gudermannian_prec_round(prec, rm);
    assert!(inverse_gudermannian.is_valid());
    assert_rounding_ordering_consistent(&inverse_gudermannian, rm, o);

    let (inverse_gudermannian_alt, o_alt) = x.inverse_gudermannian_prec_round_ref(prec, rm);
    assert!(inverse_gudermannian_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&inverse_gudermannian_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.inverse_gudermannian_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &inverse_gudermannian, o, prec, rm);

    // The function is odd.
    let (neg, neg_o) = (-&x).inverse_gudermannian_prec_round(prec, -rm);
    assert_eq!(
        ComparableFloatRef(&-neg),
        ComparableFloatRef(&inverse_gudermannian)
    );
    assert_eq!(neg_o.reverse(), o);

    if inverse_gudermannian.is_normal() {
        assert_eq!(inverse_gudermannian.get_prec(), Some(prec));
    }
    if inverse_gudermannian.is_nan() {
        // Outside the domain: |x| > pi / 2.
        assert!(x.is_nan() || x.ge_abs(&1u32));
    } else {
        // The function is odd, and its domain is (-pi / 2, pi / 2).
        assert_eq!(
            inverse_gudermannian.is_sign_negative(),
            x.is_sign_negative()
        );
        assert!(x.lt_abs(&2u32));
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.inverse_gudermannian_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s),
                ComparableFloat(inverse_gudermannian.clone())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.inverse_gudermannian_prec_round_ref(prec, Exact));
    }

    // A `Float` that is not too large or too small gives the same result as the equal `Rational`.
    if x.is_normal() && exponent_in_gate(&x) {
        let (r, ro) =
            Float::inverse_gudermannian_rational_prec_round(Rational::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&r),
            ComparableFloatRef(&inverse_gudermannian)
        );
        assert_eq!(ro, o);
    }
}

#[test]
fn inverse_gudermannian_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        inverse_gudermannian_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            inverse_gudermannian_prec_round_properties_helper(x, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.inverse_gudermannian_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        // Outside the domain
        let (s, o) = Float::INFINITY.inverse_gudermannian_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_INFINITY.inverse_gudermannian_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::TWO.inverse_gudermannian_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.inverse_gudermannian_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.inverse_gudermannian_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn inverse_gudermannian_prec_properties_helper(x: Float, prec: u64) {
    let (inverse_gudermannian, o) = x.clone().inverse_gudermannian_prec(prec);
    assert!(inverse_gudermannian.is_valid());

    let (inverse_gudermannian_alt, o_alt) = x.inverse_gudermannian_prec_ref(prec);
    assert!(inverse_gudermannian_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&inverse_gudermannian_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.inverse_gudermannian_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    assert_eq!(o_alt, o);

    let (inverse_gudermannian_alt, o_alt) = x.inverse_gudermannian_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&inverse_gudermannian_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &inverse_gudermannian, o, prec, Nearest);

    if inverse_gudermannian.is_normal() {
        assert_eq!(inverse_gudermannian.get_prec(), Some(prec));
    }
    if inverse_gudermannian.is_nan() {
        // Outside the domain: |x| > pi / 2.
        assert!(x.is_nan() || x.ge_abs(&1u32));
    } else {
        // The function is odd, and its domain is (-pi / 2, pi / 2).
        assert_eq!(
            inverse_gudermannian.is_sign_negative(),
            x.is_sign_negative()
        );
        assert!(x.lt_abs(&2u32));
    }
}

#[test]
fn inverse_gudermannian_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        inverse_gudermannian_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        inverse_gudermannian_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        inverse_gudermannian_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (s, o) = Float::NAN.inverse_gudermannian_prec(prec);
        assert!(s.is_nan());
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn inverse_gudermannian_round_properties_helper(x: Float, rm: RoundingMode) {
    let (inverse_gudermannian, o) = x.clone().inverse_gudermannian_round(rm);
    assert!(inverse_gudermannian.is_valid());
    assert_rounding_ordering_consistent(&inverse_gudermannian, rm, o);

    let (inverse_gudermannian_alt, o_alt) = x.inverse_gudermannian_round_ref(rm);
    assert!(inverse_gudermannian_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&inverse_gudermannian_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.inverse_gudermannian_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    assert_eq!(o_alt, o);

    let (inverse_gudermannian_alt, o_alt) =
        x.inverse_gudermannian_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&inverse_gudermannian_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &inverse_gudermannian, o, x.significant_bits(), rm);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.inverse_gudermannian_round_ref(rm);
            assert_eq!(
                ComparableFloat(s),
                ComparableFloat(inverse_gudermannian.clone())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.inverse_gudermannian_round_ref(Exact));
    }
}

#[test]
fn inverse_gudermannian_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        inverse_gudermannian_round_properties_helper(x, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_rounding_mode_pair_gen_var_47().test_properties_with_config(&config, |(x, rm)| {
        inverse_gudermannian_round_properties_helper(x, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn inverse_gudermannian_properties_helper(x: Float) {
    let inverse_gudermannian = x.clone().inverse_gudermannian();
    assert!(inverse_gudermannian.is_valid());

    let inverse_gudermannian_alt = (&x).inverse_gudermannian();
    assert!(inverse_gudermannian_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&inverse_gudermannian_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );

    let mut x_alt = x.clone();
    x_alt.inverse_gudermannian_assign();
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );

    let inverse_gudermannian_alt = x
        .inverse_gudermannian_prec_round_ref(x.significant_bits(), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&inverse_gudermannian_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    let inverse_gudermannian_alt = x.inverse_gudermannian_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&inverse_gudermannian_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    let inverse_gudermannian_alt = x.inverse_gudermannian_round_ref(Nearest).0;
    assert_eq!(
        ComparableFloatRef(&inverse_gudermannian_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    if inverse_gudermannian.is_nan() {
        // Outside the domain: |x| > pi / 2.
        assert!(x.is_nan() || x.ge_abs(&1u32));
    } else {
        // The function is odd, and its domain is (-pi / 2, pi / 2).
        assert_eq!(
            inverse_gudermannian.is_sign_negative(),
            x.is_sign_negative()
        );
        assert!(x.lt_abs(&2u32));
    }
}

#[test]
fn inverse_gudermannian_properties() {
    float_gen().test_properties(|x| {
        inverse_gudermannian_properties_helper(x);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_gen().test_properties_with_config(&config, |x| {
        inverse_gudermannian_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        inverse_gudermannian_properties_helper(x);
    });
}

#[test]
fn test_primitive_float_inverse_gudermannian() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_inverse_gudermannian(x)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, 0.0);
    test::<f32>(-0.0, -0.0);
    test::<f32>(1.0, 1.2261912);
    test::<f32>(2.7182817, f32::NAN);
    test::<f32>(7.0, f32::NAN);
    test::<f32>(100.0, f32::NAN);
    test::<f32>(1.0e-10, 1.0e-10);
    test::<f32>(-0.1, -0.10016709);
    test::<f32>(-0.25, -0.2526456);
    test::<f32>(-0.36787942, -0.37646955);
    test::<f32>(-0.5, -0.5222381);
    test::<f32>(3.4028235e38, f32::NAN);
    test::<f32>(1.1754944e-38, 1.1754944e-38);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, 0.0);
    test::<f64>(-0.0, -0.0);
    test::<f64>(1.0, 1.2261911708835171);
    test::<f64>(core::f64::consts::E, f64::NAN);
    test::<f64>(7.0, f64::NAN);
    test::<f64>(100.0, f64::NAN);
    test::<f64>(1.0e-10, 1.0e-10);
    test::<f64>(-0.1, -0.10016708454748019);
    test::<f64>(-0.25, -0.25264561035786753);
    test::<f64>(-0.36787944117144233, -0.3764695635432142);
    test::<f64>(-0.5, -0.5222381032784403);
    test::<f64>(1.7976931348623157e308, f64::NAN);
    test::<f64>(2.2250738585072014e-308, 2.2250738585072014e-308);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_inverse_gudermannian_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        primitive_float_inverse_gudermannian(x);
    });
}

#[test]
fn primitive_float_inverse_gudermannian_properties() {
    apply_fn_to_primitive_floats!(primitive_float_inverse_gudermannian_properties_helper);
}

#[test]
fn test_inverse_gudermannian_rational_prec() {
    let test = |s, prec, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::inverse_gudermannian_rational_prec(x.clone(), prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::inverse_gudermannian_rational_prec_ref(&x, prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 1, "0.0", "0x0.0", Equal);
    test("0", 10, "0.0", "0x0.0", Equal);
    test("1", 1, "1.0", "0x1.0#1", Less);
    test("1", 10, "1.2266", "0x1.3a0#10", Greater);
    test(
        "1",
        100,
        "1.2261911708835170708130609674712",
        "0x1.39e7aa2196b1faa23d907321e#100",
        Less,
    );
    test("1/3", 1, "0.25", "0x0.4#1", Less);
    test("1/3", 10, "0.33984", "0x0.570#10", Greater);
    test(
        "1/3",
        100,
        "0.33968337608418651239388641458822",
        "0x0.56f57d5f46c566e0f513f155a8#100",
        Less,
    );
    test("-1/3", 1, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 10, "-0.33984", "-0x0.570#10", Less);
    test(
        "-1/3",
        100,
        "-0.33968337608418651239388641458822",
        "-0x0.56f57d5f46c566e0f513f155a8#100",
        Greater,
    );
    test("22/7", 1, "NaN", "NaN", Equal);
    test("22/7", 10, "NaN", "NaN", Equal);
    test("22/7", 100, "NaN", "NaN", Equal);
    test("-1/4", 1, "-0.25", "-0x0.4#1", Greater);
    test("-1/4", 10, "-0.25244", "-0x0.40a#10", Greater);
    test(
        "-1/4",
        100,
        "-0.25264561035786753829411005567848",
        "-0x0.40ad61f9f70a3c520cfb1912a0#100",
        Less,
    );
    test("-1/1000", 1, "-0.00098", "-0x0.004#1", Greater);
    test("-1/1000", 10, "-0.00099945", "-0x0.00418#10", Greater);
    test(
        "-1/1000",
        100,
        "-0.0010000001666667083333454365117536",
        "-0x0.00418938030729f1d94c277c31b8#100",
        Less,
    );
    test("1000000000000000000000000000000", 1, "NaN", "NaN", Equal);
    test("1000000000000000000000000000000", 10, "NaN", "NaN", Equal);
    test("1000000000000000000000000000000", 100, "NaN", "NaN", Equal);
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
    test("-11/30", 1, "-0.50", "-0x0.8#1", Less);
    test("-11/30", 10, "-0.37500", "-0x0.600#10", Greater);
    test(
        "-11/30",
        100,
        "-0.37517012967630634643319182241108",
        "-0x0.600b264d65541ecc62b159ec90#100",
        Greater,
    );
    test("-3/8", 10, "-0.38428", "-0x0.626#10", Less);
    test(
        "-4598493014643029/12500000000000000",
        1,
        "-0.50",
        "-0x0.8#1",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        "-0.37646",
        "-0x0.606#10",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        100,
        "-0.37646956354321421037157802608841",
        "-0x0.60604f2f5b850c89ba6a4b1f00#100",
        Less,
    );
    test(
        "-36787944117144233/100000000000000000",
        10,
        "-0.37646",
        "-0x0.606#10",
        Greater,
    );
}

#[test]
#[should_panic]
fn inverse_gudermannian_rational_prec_fail() {
    Float::inverse_gudermannian_rational_prec(Rational::ONE, 0);
}

#[test]
#[should_panic]
fn inverse_gudermannian_rational_prec_ref_fail() {
    Float::inverse_gudermannian_rational_prec_ref(&Rational::ONE, 0);
}

#[test]
fn test_inverse_gudermannian_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::inverse_gudermannian_rational_prec_round(x.clone(), prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::inverse_gudermannian_rational_prec_round_ref(&x, prec, rm);
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
        "1.2261911708835171",
        "0x1.39e7aa2196b20#53",
        Greater,
    );
    test("1", 10, Floor, "1.2246", "0x1.398#10", Less);
    test("1", 10, Ceiling, "1.2266", "0x1.3a0#10", Greater);
    test("1", 10, Down, "1.2246", "0x1.398#10", Less);
    test("1", 10, Up, "1.2266", "0x1.3a0#10", Greater);
    test("1", 10, Nearest, "1.2266", "0x1.3a0#10", Greater);
    test(
        "1/3",
        53,
        Nearest,
        "0.33968337608418653",
        "0x0.56f57d5f46c568#53",
        Greater,
    );
    test("1/3", 10, Floor, "0.33936", "0x0.56e#10", Less);
    test("1/3", 10, Ceiling, "0.33984", "0x0.570#10", Greater);
    test("1/3", 10, Down, "0.33936", "0x0.56e#10", Less);
    test("1/3", 10, Up, "0.33984", "0x0.570#10", Greater);
    test("1/3", 10, Nearest, "0.33984", "0x0.570#10", Greater);
    test(
        "-1/3",
        53,
        Nearest,
        "-0.33968337608418653",
        "-0x0.56f57d5f46c568#53",
        Less,
    );
    test("-1/3", 10, Floor, "-0.33984", "-0x0.570#10", Less);
    test("-1/3", 10, Ceiling, "-0.33936", "-0x0.56e#10", Greater);
    test("-1/3", 10, Down, "-0.33936", "-0x0.56e#10", Greater);
    test("-1/3", 10, Up, "-0.33984", "-0x0.570#10", Less);
    test("-1/3", 10, Nearest, "-0.33984", "-0x0.570#10", Less);
    test("22/7", 53, Nearest, "NaN", "NaN", Equal);
    test("22/7", 10, Floor, "NaN", "NaN", Equal);
    test("22/7", 10, Ceiling, "NaN", "NaN", Equal);
    test("22/7", 10, Down, "NaN", "NaN", Equal);
    test("22/7", 10, Up, "NaN", "NaN", Equal);
    test("22/7", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "-1/4",
        53,
        Nearest,
        "-0.25264561035786753",
        "-0x0.40ad61f9f70a3c#53",
        Greater,
    );
    test("-1/4", 10, Floor, "-0.25293", "-0x0.40c#10", Less);
    test("-1/4", 10, Ceiling, "-0.25244", "-0x0.40a#10", Greater);
    test("-1/4", 10, Down, "-0.25244", "-0x0.40a#10", Greater);
    test("-1/4", 10, Up, "-0.25293", "-0x0.40c#10", Less);
    test("-1/4", 10, Nearest, "-0.25244", "-0x0.40a#10", Greater);
    test(
        "-1/1000",
        53,
        Nearest,
        "-0.0010000001666667082",
        "-0x0.00418938030729f0#53",
        Greater,
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
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1000000000000000000000000000000",
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
        "-0.37517012967630636",
        "-0x0.600b264d655420#53",
        Less,
    );
    test("-11/30", 10, Floor, "-0.37549", "-0x0.602#10", Less);
    test("-11/30", 10, Ceiling, "-0.37500", "-0x0.600#10", Greater);
    test("-11/30", 10, Down, "-0.37500", "-0x0.600#10", Greater);
    test("-11/30", 10, Up, "-0.37549", "-0x0.602#10", Less);
    test("-11/30", 10, Nearest, "-0.37500", "-0x0.600#10", Greater);
    test("-3/8", 10, Nearest, "-0.38428", "-0x0.626#10", Less);
    test(
        "-4598493014643029/12500000000000000",
        53,
        Nearest,
        "-0.37646956354321420",
        "-0x0.60604f2f5b850c#53",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Floor,
        "-0.37695",
        "-0x0.608#10",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Ceiling,
        "-0.37646",
        "-0x0.606#10",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Down,
        "-0.37646",
        "-0x0.606#10",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Up,
        "-0.37695",
        "-0x0.608#10",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Nearest,
        "-0.37646",
        "-0x0.606#10",
        Greater,
    );
    test(
        "-36787944117144233/100000000000000000",
        10,
        Nearest,
        "-0.37646",
        "-0x0.606#10",
        Greater,
    );
    test("3/2", 10, Nearest, "3.3398", "0x3.57#10", Less);
    test(
        "157/100",
        30,
        Ceiling,
        "7.8286480382",
        "0x7.d422472#30",
        Greater,
    );
    test("11/7", 10, Nearest, "NaN", "NaN", Equal);
    test("-3/2", 10, Down, "-3.3398", "-0x3.57#10", Greater);
}

#[test]
#[should_panic]
fn inverse_gudermannian_rational_prec_round_fail_1() {
    Float::inverse_gudermannian_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn inverse_gudermannian_rational_prec_round_fail_2() {
    Float::inverse_gudermannian_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[test]
#[should_panic]
fn inverse_gudermannian_rational_prec_round_ref_fail() {
    Float::inverse_gudermannian_rational_prec_round_ref(
        &Rational::from_unsigneds(1u32, 3),
        10,
        Exact,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn inverse_gudermannian_rational_prec_round_properties_helper(
    x: Rational,
    prec: u64,
    rm: RoundingMode,
) {
    let (inverse_gudermannian, o) =
        Float::inverse_gudermannian_rational_prec_round(x.clone(), prec, rm);
    assert!(inverse_gudermannian.is_valid());
    assert_rounding_ordering_consistent(&inverse_gudermannian, rm, o);

    let (inverse_gudermannian_alt, o_alt) =
        Float::inverse_gudermannian_rational_prec_round_ref(&x, prec, rm);
    assert!(inverse_gudermannian_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&inverse_gudermannian_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    assert_eq!(o_alt, o);

    // The reference computes atanh(sin(x)).
    if rm != Exact && inverse_gudermannian.is_normal() && result_in_range(&inverse_gudermannian, o)
    {
        let (g, go) = inverse_gudermannian_rational_prec_round_naive(&x, prec, rm);
        assert_eq!(float_to_big_exponent_float(&inverse_gudermannian), g);
        assert_eq!(o, go);
    }

    if inverse_gudermannian.is_normal() {
        assert_eq!(inverse_gudermannian.get_prec(), Some(prec));
    }
    if inverse_gudermannian.is_nan() {
        assert!(x.ge_abs(&1u32));
    } else {
        assert_eq!(
            inverse_gudermannian.partial_cmp(&0u32),
            x.partial_cmp(&0u32)
        );
        assert!(x.lt_abs(&2u32));
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::inverse_gudermannian_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(
                ComparableFloat(s),
                ComparableFloat(inverse_gudermannian.clone())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::inverse_gudermannian_rational_prec_round_ref(
            &x, prec, Exact
        ));
    }
}

#[test]
fn inverse_gudermannian_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        inverse_gudermannian_rational_prec_round_properties_helper(x, prec, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn inverse_gudermannian_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (inverse_gudermannian, o) = Float::inverse_gudermannian_rational_prec(x.clone(), prec);
    assert!(inverse_gudermannian.is_valid());

    let (inverse_gudermannian_alt, o_alt) = Float::inverse_gudermannian_rational_prec_ref(&x, prec);
    assert!(inverse_gudermannian_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&inverse_gudermannian_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    assert_eq!(o_alt, o);

    let (inverse_gudermannian_alt, o_alt) =
        Float::inverse_gudermannian_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&inverse_gudermannian_alt),
        ComparableFloatRef(&inverse_gudermannian)
    );
    assert_eq!(o_alt, o);
}

#[test]
fn inverse_gudermannian_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        inverse_gudermannian_rational_prec_properties_helper(x, prec);
    });
}

#[test]
fn test_primitive_float_inverse_gudermannian_rational() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_inverse_gudermannian_rational::<T>(
                &Rational::from_str(s).unwrap()
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0.0);
    test::<f64>("0", 0.0);
    test::<f32>("1", 1.2261912);
    test::<f64>("1", 1.2261911708835171);
    test::<f32>("1/3", 0.33968338);
    test::<f64>("1/3", 0.33968337608418653);
    test::<f32>("-1/3", -0.33968338);
    test::<f64>("-1/3", -0.33968337608418653);
    test::<f32>("-1/4", -0.2526456);
    test::<f64>("-1/4", -0.25264561035786753);
    test::<f32>("-1/2", -0.5222381);
    test::<f64>("-1/2", -0.5222381032784403);
    test::<f32>("1000", f32::NAN);
    test::<f64>("1000", f64::NAN);
    test::<f32>("-1/1000", -0.0010000002);
    test::<f64>("-1/1000", -0.0010000001666667082);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_inverse_gudermannian_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        primitive_float_inverse_gudermannian_rational::<T>(&x);
    });
}

#[test]
fn primitive_float_inverse_gudermannian_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_inverse_gudermannian_rational_properties_helper);
}
