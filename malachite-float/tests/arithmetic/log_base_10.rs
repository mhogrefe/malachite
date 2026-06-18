// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{LogBase10, LogBase10Assign, Reciprocal};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    primitive_float_gen, rounding_mode_gen, unsigned_gen_var_11,
    unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::arithmetic::log_base::{
    primitive_float_log_base, primitive_float_log_base_rational,
};
use malachite_float::arithmetic::log_base_10::{
    primitive_float_log_base_10, primitive_float_log_base_10_rational,
};
use malachite_float::test_util::arithmetic::log_base_10::{
    rug_log_base_10, rug_log_base_10_prec, rug_log_base_10_prec_round,
    rug_log_base_10_rational_prec_round, rug_log_base_10_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_42,
    float_rounding_mode_pair_gen_var_43, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_4, float_unsigned_rounding_mode_triple_gen_var_29,
    float_unsigned_rounding_mode_triple_gen_var_30,
    rational_unsigned_rounding_mode_triple_gen_var_9,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_gen;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_log_base_10() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let log_base_10 = x.clone().log_base_10();
        assert!(log_base_10.is_valid());

        assert_eq!(log_base_10.to_string(), out);
        assert_eq!(to_hex_string(&log_base_10), out_hex);

        let log_base_10_alt = (&x).log_base_10();
        assert!(log_base_10_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_10),
            ComparableFloatRef(&log_base_10_alt)
        );

        let mut log_base_10_alt = x.clone();
        log_base_10_alt.log_base_10_assign();
        assert!(log_base_10_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_10),
            ComparableFloatRef(&log_base_10_alt)
        );

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_10(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&log_base_10)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "-Infinity", "-Infinity");
    test("-0.0", "-0x0.0", "-Infinity", "-Infinity");
    test("1.0", "0x1.0#1", "0.0", "0x0.0");
    test("-1.0", "-0x1.0#1", "NaN", "NaN");
    test("2.0", "0x2.0#1", "0.2", "0x0.4#1");
    test("50.0", "0x32.0#5", "1.7", "0x1.b#5");
    test("123.0", "0x7b.0#7", "2.09", "0x2.18#7");
    test("7.0", "0x7.0#3", "0.9", "0x0.e#3");
    test("0.1", "0x0.2#1", "-1.0", "-0x1.0#1");
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "0.49714987269413385",
        "0x0.7f4536cc6e8eec#53",
    );
    test("-5.0", "-0x5.0#3", "NaN", "NaN");
    test("3.0e301", "0x3.0E+250#2", "3.0e2", "0x1.0E+2#2");
    test("10.0", "0xa.0#3", "1.0", "0x1.0#3");
    test("100.0", "0x64.0#5", "2.0", "0x2.0#5");
    test("1.0e3", "0x3e8.0#7", "3.0", "0x3.00#7");
    test("1.0e6", "0xf.424E+4#14", "6.0", "0x6.000#14");
}

#[test]
fn test_log_base_10_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log_base_10, o) = x.clone().log_base_10_prec(prec);
        assert!(log_base_10.is_valid());

        assert_eq!(log_base_10.to_string(), out);
        assert_eq!(to_hex_string(&log_base_10), out_hex);
        assert_eq!(o, o_out);

        let (log_base_10_alt, o_alt) = x.log_base_10_prec_ref(prec);
        assert!(log_base_10_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_10),
            ComparableFloatRef(&log_base_10_alt)
        );
        assert_eq!(o_alt, o);

        let mut log_base_10_alt = x.clone();
        let o_alt = log_base_10_alt.log_base_10_prec_assign(prec);
        assert!(log_base_10_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_10),
            ComparableFloatRef(&log_base_10_alt)
        );
        assert_eq!(o_alt, o);

        let (rug_log_base_10, rug_o) = rug_log_base_10_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_10)),
            ComparableFloatRef(&log_base_10),
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, "NaN", "NaN", Equal);
    test("NaN", "NaN", 53, "NaN", "NaN", Equal);
    test("NaN", "NaN", 100, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", 10, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", 53, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", 100, "Infinity", "Infinity", Equal);
    test("-Infinity", "-Infinity", 1, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 53, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 100, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 10, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 53, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 100, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", 1, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", 10, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", 53, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", 100, "-Infinity", "-Infinity", Equal);
    test("1.0", "0x1.0#1", 1, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 53, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 100, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 1, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 53, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 100, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 1, "0.2", "0x0.4#1", Less);
    test("2.0", "0x2.0#1", 10, "0.3013", "0x0.4d2#10", Greater);
    test(
        "2.0",
        "0x2.0#1",
        53,
        "0.3010299956639812",
        "0x0.4d104d427de7fc#53",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        100,
        "0.3010299956639811952137388947246",
        "0x0.4d104d427de7fbcc47c4acd608#100",
        Greater,
    );
    test("50.0", "0x32.0#5", 1, "2.0", "0x2.0#1", Greater);
    test("50.0", "0x32.0#5", 10, "1.699", "0x1.b30#10", Greater);
    test(
        "50.0",
        "0x32.0#5",
        53,
        "1.6989700043360187",
        "0x1.b2efb2bd82180#53",
        Less,
    );
    test(
        "50.0",
        "0x32.0#5",
        100,
        "1.698970004336018804786261105276",
        "0x1.b2efb2bd82180433b83b532a0#100",
        Greater,
    );
    test("123.0", "0x7b.0#7", 1, "2.0", "0x2.0#1", Less);
    test("123.0", "0x7b.0#7", 10, "2.09", "0x2.17#10", Less);
    test(
        "123.0",
        "0x7b.0#7",
        53,
        "2.0899051114393981",
        "0x2.17040579601d8#53",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        100,
        "2.089905111439397931804439753222",
        "0x2.17040579601d77164f7c690d4#100",
        Less,
    );
    test("7.0", "0x7.0#3", 1, "1.0", "0x1.0#1", Greater);
    test("7.0", "0x7.0#3", 10, "0.845", "0x0.d84#10", Less);
    test(
        "7.0",
        "0x7.0#3",
        53,
        "0.8450980400142568",
        "0x0.d858585bc661f8#53",
        Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        100,
        "0.8450980400142568307122162585923",
        "0x0.d858585bc661f94b692ff8a80#100",
        Less,
    );
    test("0.1", "0x0.2#1", 1, "-1.0", "-0x1.0#1", Less);
    test("0.1", "0x0.2#1", 10, "-0.903", "-0x0.e74#10", Less);
    test(
        "0.1",
        "0x0.2#1",
        53,
        "-0.9030899869919435",
        "-0x0.e730e7c779b7f0#53",
        Greater,
    );
    test(
        "0.1",
        "0x0.2#1",
        100,
        "-0.9030899869919435856412166841734",
        "-0x0.e730e7c779b7f364d74e06821#100",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "0.4971",
        "0x0.7f4#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        53,
        "0.49714987269413385",
        "0x0.7f4536cc6e8eec#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        100,
        "0.4971498726941338374217231245522",
        "0x0.7f4536cc6e8eeacae4696b9e08#100",
        Less,
    );
    test("-5.0", "-0x5.0#3", 1, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", 10, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", 53, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", 100, "NaN", "NaN", Equal);
    test("3.0e301", "0x3.0E+250#2", 1, "3.0e2", "0x1.0E+2#1", Less);
    test("3.0e301", "0x3.0E+250#2", 10, "301.5", "0x12d.8#10", Less);
    test(
        "3.0e301",
        "0x3.0E+250#2",
        53,
        "301.50711691870083",
        "0x12d.81d26a15118#53",
        Less,
    );
    test(
        "3.0e301",
        "0x3.0E+250#2",
        100,
        "301.5071169187008576510339226277",
        "0x12d.81d26a1511878a1b645032a#100",
        Less,
    );
    test("10.0", "0xa.0#3", 10, "1.0", "0x1.000#10", Equal);
    test("100.0", "0x64.0#5", 10, "2.0", "0x2.00#10", Equal);
    test("1.0e3", "0x3e8.0#7", 10, "3.0", "0x3.00#10", Equal);
    test("1.0e6", "0xf.424E+4#14", 10, "6.0", "0x6.00#10", Equal);
}

#[test]
fn log_base_10_prec_fail() {
    assert_panic!(Float::NAN.log_base_10_prec(0));
    assert_panic!(Float::NAN.log_base_10_prec_ref(0));
    assert_panic!({
        let mut x = Float::NAN;
        x.log_base_10_prec_assign(0)
    });
}

#[test]
fn test_log_base_10_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log_base_10, o) = x.clone().log_base_10_round(rm);
        assert!(log_base_10.is_valid());

        assert_eq!(log_base_10.to_string(), out);
        assert_eq!(to_hex_string(&log_base_10), out_hex);
        assert_eq!(o, o_out);

        let (log_base_10_alt, o_alt) = x.log_base_10_round_ref(rm);
        assert!(log_base_10_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_10),
            ComparableFloatRef(&log_base_10_alt)
        );
        assert_eq!(o_alt, o);

        let mut log_base_10_alt = x.clone();
        let o_alt = log_base_10_alt.log_base_10_round_assign(rm);
        assert!(log_base_10_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_10),
            ComparableFloatRef(&log_base_10_alt)
        );
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log_base_10, rug_o) =
                rug_log_base_10_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log_base_10)),
                ComparableFloatRef(&log_base_10),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", Floor, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", Ceiling, "Infinity", "Infinity", Equal,
    );
    test("Infinity", "Infinity", Down, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", Up, "Infinity", "Infinity", Equal);
    test(
        "Infinity", "Infinity", Nearest, "Infinity", "Infinity", Equal,
    );
    test("-Infinity", "-Infinity", Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Down, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Up, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", Floor, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", Ceiling, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", Down, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", Up, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", Nearest, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Floor, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Ceiling, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Down, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Up, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", Nearest, "-Infinity", "-Infinity", Equal);
    test("1.0", "0x1.0#1", Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Down, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Up, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", Nearest, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Down, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Up, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", Floor, "0.2", "0x0.4#1", Less);
    test("2.0", "0x2.0#1", Ceiling, "0.5", "0x0.8#1", Greater);
    test("2.0", "0x2.0#1", Down, "0.2", "0x0.4#1", Less);
    test("2.0", "0x2.0#1", Up, "0.5", "0x0.8#1", Greater);
    test("2.0", "0x2.0#1", Nearest, "0.2", "0x0.4#1", Less);
    test("50.0", "0x32.0#5", Floor, "1.7", "0x1.b#5", Less);
    test("50.0", "0x32.0#5", Ceiling, "1.75", "0x1.c#5", Greater);
    test("50.0", "0x32.0#5", Down, "1.7", "0x1.b#5", Less);
    test("50.0", "0x32.0#5", Up, "1.75", "0x1.c#5", Greater);
    test("50.0", "0x32.0#5", Nearest, "1.7", "0x1.b#5", Less);
    test("123.0", "0x7b.0#7", Floor, "2.06", "0x2.10#7", Less);
    test("123.0", "0x7b.0#7", Ceiling, "2.09", "0x2.18#7", Greater);
    test("123.0", "0x7b.0#7", Down, "2.06", "0x2.10#7", Less);
    test("123.0", "0x7b.0#7", Up, "2.09", "0x2.18#7", Greater);
    test("123.0", "0x7b.0#7", Nearest, "2.09", "0x2.18#7", Greater);
    test("7.0", "0x7.0#3", Floor, "0.8", "0x0.c#3", Less);
    test("7.0", "0x7.0#3", Ceiling, "0.9", "0x0.e#3", Greater);
    test("7.0", "0x7.0#3", Down, "0.8", "0x0.c#3", Less);
    test("7.0", "0x7.0#3", Up, "0.9", "0x0.e#3", Greater);
    test("7.0", "0x7.0#3", Nearest, "0.9", "0x0.e#3", Greater);
    test("0.1", "0x0.2#1", Floor, "-1.0", "-0x1.0#1", Less);
    test("0.1", "0x0.2#1", Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("0.1", "0x0.2#1", Down, "-0.5", "-0x0.8#1", Greater);
    test("0.1", "0x0.2#1", Up, "-1.0", "-0x1.0#1", Less);
    test("0.1", "0x0.2#1", Nearest, "-1.0", "-0x1.0#1", Less);
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "0.4971498726941338",
        "0x0.7f4536cc6e8ee8#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "0.49714987269413385",
        "0x0.7f4536cc6e8eec#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "0.4971498726941338",
        "0x0.7f4536cc6e8ee8#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "0.49714987269413385",
        "0x0.7f4536cc6e8eec#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "0.49714987269413385",
        "0x0.7f4536cc6e8eec#53",
        Greater,
    );
    test("-5.0", "-0x5.0#3", Floor, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", Ceiling, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", Down, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", Up, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", Nearest, "NaN", "NaN", Equal);
    test(
        "3.0e301",
        "0x3.0E+250#2",
        Floor,
        "3.0e2",
        "0x1.0E+2#2",
        Less,
    );
    test(
        "3.0e301",
        "0x3.0E+250#2",
        Ceiling,
        "4.0e2",
        "0x1.8E+2#2",
        Greater,
    );
    test("3.0e301", "0x3.0E+250#2", Down, "3.0e2", "0x1.0E+2#2", Less);
    test(
        "3.0e301",
        "0x3.0E+250#2",
        Up,
        "4.0e2",
        "0x1.8E+2#2",
        Greater,
    );
    test(
        "3.0e301",
        "0x3.0E+250#2",
        Nearest,
        "3.0e2",
        "0x1.0E+2#2",
        Less,
    );
    test("10.0", "0xa.0#3", Exact, "1.0", "0x1.0#3", Equal);
    test("100.0", "0x64.0#5", Exact, "2.0", "0x2.0#5", Equal);
    test("1.0e3", "0x3e8.0#7", Exact, "3.0", "0x3.00#7", Equal);
    test("1.0e6", "0xf.424E+4#14", Exact, "6.0", "0x6.000#14", Equal);
}

#[test]
fn log_base_10_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(THREE.log_base_10_round(Exact));
    assert_panic!(THREE.log_base_10_round_ref(Exact));
    assert_panic!({
        let mut x = THREE;
        x.log_base_10_round_assign(Exact);
    });
}

#[test]
fn test_log_base_10_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (log_base_10, o) = x.clone().log_base_10_prec_round(prec, rm);
        assert!(log_base_10.is_valid());

        assert_eq!(log_base_10.to_string(), out);
        assert_eq!(to_hex_string(&log_base_10), out_hex);
        assert_eq!(o, o_out);

        let (log_base_10_alt, o_alt) = x.log_base_10_prec_round_ref(prec, rm);
        assert!(log_base_10_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_10),
            ComparableFloatRef(&log_base_10_alt)
        );
        assert_eq!(o_alt, o);

        let mut log_base_10_alt = x.clone();
        let o_alt = log_base_10_alt.log_base_10_prec_round_assign(prec, rm);
        assert!(log_base_10_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&log_base_10),
            ComparableFloatRef(&log_base_10_alt)
        );
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_log_base_10, rug_o) =
                rug_log_base_10_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_log_base_10)),
                ComparableFloatRef(&log_base_10),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 53, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 53, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 53, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 53, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 53, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 53, Nearest, "Infinity", "Infinity", Equal,
    );
    test("-Infinity", "-Infinity", 1, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 53, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 53, Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 53, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Floor, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 1, Ceiling, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 1, Nearest, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 10, Floor, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 10, Ceiling, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 10, Nearest, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 53, Floor, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 53, Ceiling, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 53, Nearest, "-Infinity", "-Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Floor, "-Infinity", "-Infinity", Equal);
    test(
        "-0.0",
        "-0x0.0",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-0.0", "-0x0.0", 10, Floor, "-Infinity", "-Infinity", Equal);
    test(
        "-0.0",
        "-0x0.0",
        10,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-0.0", "-0x0.0", 53, Floor, "-Infinity", "-Infinity", Equal);
    test(
        "-0.0",
        "-0x0.0",
        53,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        53,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("1.0", "0x1.0#1", 1, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 53, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 53, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 53, Nearest, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 1, Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 1, Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 1, Nearest, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 53, Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 53, Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 53, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 1, Floor, "0.2", "0x0.4#1", Less);
    test("2.0", "0x2.0#1", 1, Ceiling, "0.5", "0x0.8#1", Greater);
    test("2.0", "0x2.0#1", 1, Nearest, "0.2", "0x0.4#1", Less);
    test("2.0", "0x2.0#1", 10, Floor, "0.3008", "0x0.4d0#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "0.3013",
        "0x0.4d2#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "0.3013",
        "0x0.4d2#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        53,
        Floor,
        "0.30102999566398114",
        "0x0.4d104d427de7f8#53",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        53,
        Ceiling,
        "0.3010299956639812",
        "0x0.4d104d427de7fc#53",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        53,
        Nearest,
        "0.3010299956639812",
        "0x0.4d104d427de7fc#53",
        Greater,
    );
    test("50.0", "0x32.0#5", 1, Floor, "1.0", "0x1.0#1", Less);
    test("50.0", "0x32.0#5", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("50.0", "0x32.0#5", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("50.0", "0x32.0#5", 10, Floor, "1.697", "0x1.b28#10", Less);
    test(
        "50.0",
        "0x32.0#5",
        10,
        Ceiling,
        "1.699",
        "0x1.b30#10",
        Greater,
    );
    test(
        "50.0",
        "0x32.0#5",
        10,
        Nearest,
        "1.699",
        "0x1.b30#10",
        Greater,
    );
    test(
        "50.0",
        "0x32.0#5",
        53,
        Floor,
        "1.6989700043360187",
        "0x1.b2efb2bd82180#53",
        Less,
    );
    test(
        "50.0",
        "0x32.0#5",
        53,
        Ceiling,
        "1.698970004336019",
        "0x1.b2efb2bd82181#53",
        Greater,
    );
    test(
        "50.0",
        "0x32.0#5",
        53,
        Nearest,
        "1.6989700043360187",
        "0x1.b2efb2bd82180#53",
        Less,
    );
    test("123.0", "0x7b.0#7", 1, Floor, "2.0", "0x2.0#1", Less);
    test("123.0", "0x7b.0#7", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("123.0", "0x7b.0#7", 1, Nearest, "2.0", "0x2.0#1", Less);
    test("123.0", "0x7b.0#7", 10, Floor, "2.09", "0x2.17#10", Less);
    test(
        "123.0",
        "0x7b.0#7",
        10,
        Ceiling,
        "2.094",
        "0x2.18#10",
        Greater,
    );
    test("123.0", "0x7b.0#7", 10, Nearest, "2.09", "0x2.17#10", Less);
    test(
        "123.0",
        "0x7b.0#7",
        53,
        Floor,
        "2.0899051114393976",
        "0x2.17040579601d6#53",
        Less,
    );
    test(
        "123.0",
        "0x7b.0#7",
        53,
        Ceiling,
        "2.0899051114393981",
        "0x2.17040579601d8#53",
        Greater,
    );
    test(
        "123.0",
        "0x7b.0#7",
        53,
        Nearest,
        "2.0899051114393981",
        "0x2.17040579601d8#53",
        Greater,
    );
    test("7.0", "0x7.0#3", 1, Floor, "0.5", "0x0.8#1", Less);
    test("7.0", "0x7.0#3", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("7.0", "0x7.0#3", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("7.0", "0x7.0#3", 10, Floor, "0.845", "0x0.d84#10", Less);
    test(
        "7.0",
        "0x7.0#3",
        10,
        Ceiling,
        "0.846",
        "0x0.d88#10",
        Greater,
    );
    test("7.0", "0x7.0#3", 10, Nearest, "0.845", "0x0.d84#10", Less);
    test(
        "7.0",
        "0x7.0#3",
        53,
        Floor,
        "0.8450980400142568",
        "0x0.d858585bc661f8#53",
        Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        53,
        Ceiling,
        "0.8450980400142569",
        "0x0.d858585bc66200#53",
        Greater,
    );
    test(
        "7.0",
        "0x7.0#3",
        53,
        Nearest,
        "0.8450980400142568",
        "0x0.d858585bc661f8#53",
        Less,
    );
    test("0.1", "0x0.2#1", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("0.1", "0x0.2#1", 1, Ceiling, "-0.5", "-0x0.8#1", Greater);
    test("0.1", "0x0.2#1", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("0.1", "0x0.2#1", 10, Floor, "-0.903", "-0x0.e74#10", Less);
    test(
        "0.1",
        "0x0.2#1",
        10,
        Ceiling,
        "-0.902",
        "-0x0.e70#10",
        Greater,
    );
    test("0.1", "0x0.2#1", 10, Nearest, "-0.903", "-0x0.e74#10", Less);
    test(
        "0.1",
        "0x0.2#1",
        53,
        Floor,
        "-0.9030899869919436",
        "-0x0.e730e7c779b7f8#53",
        Less,
    );
    test(
        "0.1",
        "0x0.2#1",
        53,
        Ceiling,
        "-0.9030899869919435",
        "-0x0.e730e7c779b7f0#53",
        Greater,
    );
    test(
        "0.1",
        "0x0.2#1",
        53,
        Nearest,
        "-0.9030899869919435",
        "-0x0.e730e7c779b7f0#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Floor,
        "0.2",
        "0x0.4#1",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Ceiling,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        Nearest,
        "0.5",
        "0x0.8#1",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "0.4971",
        "0x0.7f4#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "0.4976",
        "0x0.7f6#10",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "0.4971",
        "0x0.7f4#10",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        53,
        Floor,
        "0.4971498726941338",
        "0x0.7f4536cc6e8ee8#53",
        Less,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        53,
        Ceiling,
        "0.49714987269413385",
        "0x0.7f4536cc6e8eec#53",
        Greater,
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        53,
        Nearest,
        "0.49714987269413385",
        "0x0.7f4536cc6e8eec#53",
        Greater,
    );
    test("-5.0", "-0x5.0#3", 1, Floor, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", 1, Ceiling, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", 1, Nearest, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", 10, Floor, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", 10, Ceiling, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", 10, Nearest, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", 53, Floor, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", 53, Ceiling, "NaN", "NaN", Equal);
    test("-5.0", "-0x5.0#3", 53, Nearest, "NaN", "NaN", Equal);
    test(
        "3.0e301",
        "0x3.0E+250#2",
        1,
        Floor,
        "3.0e2",
        "0x1.0E+2#1",
        Less,
    );
    test(
        "3.0e301",
        "0x3.0E+250#2",
        1,
        Ceiling,
        "5.0e2",
        "0x2.0E+2#1",
        Greater,
    );
    test(
        "3.0e301",
        "0x3.0E+250#2",
        1,
        Nearest,
        "3.0e2",
        "0x1.0E+2#1",
        Less,
    );
    test(
        "3.0e301",
        "0x3.0E+250#2",
        10,
        Floor,
        "301.5",
        "0x12d.8#10",
        Less,
    );
    test(
        "3.0e301",
        "0x3.0E+250#2",
        10,
        Ceiling,
        "302.0",
        "0x12e.0#10",
        Greater,
    );
    test(
        "3.0e301",
        "0x3.0E+250#2",
        10,
        Nearest,
        "301.5",
        "0x12d.8#10",
        Less,
    );
    test(
        "3.0e301",
        "0x3.0E+250#2",
        53,
        Floor,
        "301.50711691870083",
        "0x12d.81d26a15118#53",
        Less,
    );
    test(
        "3.0e301",
        "0x3.0E+250#2",
        53,
        Ceiling,
        "301.50711691870089",
        "0x12d.81d26a15119#53",
        Greater,
    );
    test(
        "3.0e301",
        "0x3.0E+250#2",
        53,
        Nearest,
        "301.50711691870083",
        "0x12d.81d26a15118#53",
        Less,
    );
    test("10.0", "0xa.0#3", 10, Floor, "1.0", "0x1.000#10", Equal);
    test("10.0", "0xa.0#3", 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test("10.0", "0xa.0#3", 10, Nearest, "1.0", "0x1.000#10", Equal);
    test("10.0", "0xa.0#3", 10, Exact, "1.0", "0x1.000#10", Equal);
    test("100.0", "0x64.0#5", 10, Floor, "2.0", "0x2.00#10", Equal);
    test("100.0", "0x64.0#5", 10, Ceiling, "2.0", "0x2.00#10", Equal);
    test("100.0", "0x64.0#5", 10, Nearest, "2.0", "0x2.00#10", Equal);
    test("100.0", "0x64.0#5", 10, Exact, "2.0", "0x2.00#10", Equal);
    test("1.0e3", "0x3e8.0#7", 10, Floor, "3.0", "0x3.00#10", Equal);
    test("1.0e3", "0x3e8.0#7", 10, Ceiling, "3.0", "0x3.00#10", Equal);
    test("1.0e3", "0x3e8.0#7", 10, Nearest, "3.0", "0x3.00#10", Equal);
    test("1.0e3", "0x3e8.0#7", 10, Exact, "3.0", "0x3.00#10", Equal);
    test(
        "1.0e6",
        "0xf.424E+4#14",
        10,
        Floor,
        "6.0",
        "0x6.00#10",
        Equal,
    );
    test(
        "1.0e6",
        "0xf.424E+4#14",
        10,
        Ceiling,
        "6.0",
        "0x6.00#10",
        Equal,
    );
    test(
        "1.0e6",
        "0xf.424E+4#14",
        10,
        Nearest,
        "6.0",
        "0x6.00#10",
        Equal,
    );
    test(
        "1.0e6",
        "0xf.424E+4#14",
        10,
        Exact,
        "6.0",
        "0x6.00#10",
        Equal,
    );
}

#[test]
fn log_base_10_prec_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::one_prec(1).log_base_10_prec_round(0, Floor));
    assert_panic!(Float::one_prec(1).log_base_10_prec_round_ref(0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.log_base_10_prec_round_assign(0, Floor)
    });

    assert_panic!(THREE.log_base_10_prec_round(1, Exact));
    assert_panic!(THREE.log_base_10_prec_round_ref(1, Exact));
    assert_panic!({
        let mut x = THREE;
        x.log_base_10_prec_round_assign(1, Exact)
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_10_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (log_base_10, o) = x.clone().log_base_10_prec_round(prec, rm);
    assert!(log_base_10.is_valid());

    let (log_base_10_alt, o_alt) = x.log_base_10_prec_round_ref(prec, rm);
    assert!(log_base_10_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&log_base_10_alt),
        ComparableFloatRef(&log_base_10)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_10_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log_base_10));
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log_base_10, rug_o) =
            rug_log_base_10_prec_round(&rug::Float::exact_from(&x), prec, rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_10)),
            ComparableFloatRef(&log_base_10),
        );
        assert_eq!(rug_o, o);
    }

    if x < 0u32 {
        assert!(log_base_10.is_nan());
    }

    if log_base_10.is_normal() {
        assert_eq!(log_base_10.get_prec(), Some(prec));
        if x > 1u32 && o > Less {
            assert!(log_base_10 > 0u32);
        } else if x > 0u32 && x < 1u32 && o < Greater {
            assert!(log_base_10 < 0u32);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.log_base_10_prec_round_ref(prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(log_base_10.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.log_base_10_prec_round_ref(prec, Exact));
    }
}

#[test]
fn log_base_10_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_29().test_properties(|(x, prec, rm)| {
        log_base_10_prec_round_properties_helper(x, prec, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_30().test_properties(|(x, prec, rm)| {
        log_base_10_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (log_base_10, o) = Float::NAN.log_base_10_prec_round(prec, rm);
        assert!(log_base_10.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.log_base_10_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );

        let (log_base_10, o) = Float::NEGATIVE_INFINITY.log_base_10_prec_round(prec, rm);
        assert_eq!(ComparableFloat(log_base_10), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        assert_eq!(
            Float::ZERO.log_base_10_prec_round(prec, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        assert_eq!(
            Float::NEGATIVE_ZERO.log_base_10_prec_round(prec, rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        let (log_base_10, o) = Float::ONE.log_base_10_prec_round(prec, rm);
        assert_eq!(ComparableFloat(log_base_10), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (log_base_10, o) = Float::NEGATIVE_ONE.log_base_10_prec_round(prec, rm);
        assert_eq!(ComparableFloat(log_base_10), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_10_prec_properties_helper(x: Float, prec: u64) {
    let (log_base_10, o) = x.clone().log_base_10_prec(prec);
    assert!(log_base_10.is_valid());

    let (log_base_10_alt, o_alt) = x.log_base_10_prec_ref(prec);
    assert!(log_base_10_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&log_base_10_alt),
        ComparableFloatRef(&log_base_10)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_10_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log_base_10));
    assert_eq!(o_alt, o);

    let (rug_log_base_10, rug_o) = rug_log_base_10_prec(&rug::Float::exact_from(&x), prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_log_base_10)),
        ComparableFloatRef(&log_base_10),
    );
    assert_eq!(rug_o, o);

    let (log_base_10_alt, o_alt) = x.log_base_10_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&log_base_10_alt),
        ComparableFloatRef(&log_base_10)
    );
    assert_eq!(o_alt, o);

    if x < 0u32 {
        assert!(log_base_10.is_nan());
    }

    if log_base_10.is_normal() {
        assert_eq!(log_base_10.get_prec(), Some(prec));
        if x > 1u32 && o > Less {
            assert!(log_base_10 > 0u32);
        } else if x > 0u32 && x < 1u32 && o < Greater {
            assert!(log_base_10 < 0u32);
        }
    }
}

#[test]
fn log_base_10_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        log_base_10_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        log_base_10_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        log_base_10_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (log_base_10, o) = Float::NAN.log_base_10_prec(prec);
        assert!(log_base_10.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::ZERO.log_base_10_prec(prec),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        assert_eq!(
            Float::NEGATIVE_ZERO.log_base_10_prec(prec),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        assert_eq!(
            Float::INFINITY.log_base_10_prec(prec),
            (Float::INFINITY, Equal)
        );

        let (log_base_10, o) = Float::NEGATIVE_INFINITY.log_base_10_prec(prec);
        assert_eq!(ComparableFloat(log_base_10), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        let (log_base_10, o) = Float::ONE.log_base_10_prec(prec);
        assert_eq!(ComparableFloat(log_base_10), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (log_base_10, o) = Float::NEGATIVE_ONE.log_base_10_prec(prec);
        assert_eq!(ComparableFloat(log_base_10), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_10_round_properties_helper(x: Float, rm: RoundingMode) {
    let (log_base_10, o) = x.clone().log_base_10_round(rm);
    assert!(log_base_10.is_valid());

    let (log_base_10_alt, o_alt) = x.log_base_10_round_ref(rm);
    assert!(log_base_10_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(
        ComparableFloatRef(&log_base_10_alt),
        ComparableFloatRef(&log_base_10)
    );

    let mut x_alt = x.clone();
    let o_alt = x_alt.log_base_10_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log_base_10));
    assert_eq!(o_alt, o);

    let (log_base_10_alt, o_alt) = x.log_base_10_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&log_base_10_alt),
        ComparableFloatRef(&log_base_10)
    );
    assert_eq!(o_alt, o);

    if x < 0u32 {
        assert!(log_base_10.is_nan());
    }

    if log_base_10.is_normal() {
        assert_eq!(log_base_10.get_prec(), Some(x.get_prec().unwrap()));
        if x > 1u32 && o > Less {
            assert!(log_base_10 > 0u32);
        } else if x > 0u32 && x < 1u32 && o < Greater {
            assert!(log_base_10 < 0u32);
        }
    }

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log_base_10, rug_o) = rug_log_base_10_round(&rug::Float::exact_from(&x), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log_base_10)),
            ComparableFloatRef(&log_base_10),
        );
        assert_eq!(rug_o, o);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.log_base_10_round_ref(rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(log_base_10.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.log_base_10_round_ref(Exact));
    }
}

#[test]
fn log_base_10_round_properties() {
    float_rounding_mode_pair_gen_var_42().test_properties(|(x, rm)| {
        log_base_10_round_properties_helper(x, rm);
    });

    float_rounding_mode_pair_gen_var_43().test_properties(|(x, rm)| {
        log_base_10_round_properties_helper(x, rm);
    });

    rounding_mode_gen().test_properties(|rm| {
        let (log_base_10, o) = Float::NAN.log_base_10_round(rm);
        assert!(log_base_10.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::ZERO.log_base_10_round(rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        assert_eq!(
            Float::NEGATIVE_ZERO.log_base_10_round(rm),
            (Float::NEGATIVE_INFINITY, Equal)
        );

        assert_eq!(
            Float::INFINITY.log_base_10_round(rm),
            (Float::INFINITY, Equal)
        );

        let (log_base_10, o) = Float::NEGATIVE_INFINITY.log_base_10_round(rm);
        assert_eq!(ComparableFloat(log_base_10), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);

        let (log_base_10, o) = Float::ONE.log_base_10_round(rm);
        assert_eq!(ComparableFloat(log_base_10), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (log_base_10, o) = Float::NEGATIVE_ONE.log_base_10_round(rm);
        assert_eq!(ComparableFloat(log_base_10), ComparableFloat(Float::NAN));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_10_properties_helper(x: Float) {
    let log_base_10 = x.clone().log_base_10();
    assert!(log_base_10.is_valid());

    let log_base_10_alt = (&x).log_base_10();
    assert!(log_base_10_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&log_base_10_alt),
        ComparableFloatRef(&log_base_10)
    );

    let mut x_alt = x.clone();
    x_alt.log_base_10_assign();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&log_base_10));

    let log_base_10_alt = x
        .log_base_10_prec_round_ref(x.significant_bits(), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&log_base_10_alt),
        ComparableFloatRef(&log_base_10)
    );
    let log_base_10_alt = x.log_base_10_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&log_base_10_alt),
        ComparableFloatRef(&log_base_10)
    );

    let (log_base_10_alt, _) = x.log_base_10_round_ref(Nearest);
    assert_eq!(
        ComparableFloatRef(&log_base_10_alt),
        ComparableFloatRef(&log_base_10)
    );

    if x < 0u32 {
        assert!(log_base_10.is_nan());
    }

    let rug_log_base_10 = rug_log_base_10(&rug::Float::exact_from(&x));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_log_base_10)),
        ComparableFloatRef(&log_base_10),
    );
}

#[test]
fn log_base_10_properties() {
    float_gen().test_properties(|x| {
        log_base_10_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        log_base_10_properties_helper(x);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn log_base_10_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (log, o) = Float::log_base_10_rational_prec_round(x.clone(), prec, rm);
    assert!(log.is_valid());

    let (log_alt, o_alt) = Float::log_base_10_rational_prec_round_ref(&x, prec, rm);
    assert!(log_alt.is_valid());
    assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
    assert_eq!(o_alt, o);

    // Independent rug oracle (native log10; skips Exact, which rug can't represent).
    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_log, rug_o) = rug_log_base_10_rational_prec_round(&x, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_log)),
            ComparableFloatRef(&log),
        );
        assert_eq!(rug_o, o);
    }

    // log_10(1/x) = -log_10(x), with the rounding direction reversed.
    if x != 0u32 {
        let (log_recip, o_recip) =
            Float::log_base_10_rational_prec_round((&x).reciprocal(), prec, -rm);
        assert!(log_recip.is_valid());
        assert_eq!(
            ComparableFloatRef(&(-log_recip).abs_negative_zero()),
            ComparableFloatRef(&log.abs_negative_zero_ref())
        );
        assert_eq!(o_recip.reverse(), o);
    }

    if log.is_normal() {
        assert_eq!(log.get_prec(), Some(prec));
        if x > 1u32 && o > Less {
            assert!(log > 0u32);
        } else if x > 0u32 && x < 1u32 && o < Greater {
            assert!(log < 0u32);
        }
    }
}

#[test]
fn log_base_10_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_9().test_properties(|(x, prec, rm)| {
        log_base_10_rational_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    rational_unsigned_rounding_mode_triple_gen_var_9().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            log_base_10_rational_prec_round_properties_helper(x, prec, rm);
        },
    );
}

#[test]
fn log_base_10_rational_prec_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_9().test_properties(|(x, prec, _rm)| {
        let (log, o) = Float::log_base_10_rational_prec(x.clone(), prec);
        assert!(log.is_valid());
        let (log_ref, o_ref) = Float::log_base_10_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&log_ref), ComparableFloatRef(&log));
        assert_eq!(o_ref, o);
        let (expected, eo) = Float::log_base_10_rational_prec_round(x, prec, Nearest);
        assert_eq!(ComparableFloatRef(&log), ComparableFloatRef(&expected));
        assert_eq!(o, eo);
    });
}

#[test]
fn test_log_base_10_rational_prec_round() {
    let test = |n: i64, d: u64, prec: u64, rm: RoundingMode, out: &str, o_out: Ordering| {
        let x = Rational::from_signeds(n, i64::exact_from(d));
        let (log, o) = Float::log_base_10_rational_prec_round(x.clone(), prec, rm);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);
        let (log_alt, o_alt) = Float::log_base_10_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rl, ro) = rug_log_base_10_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rl)),
                ComparableFloatRef(&log)
            );
            assert_eq!(ro, o);
        }
    };
    test(3, 5, 20, Floor, "-0.221849", Less);
    test(3, 5, 20, Ceiling, "-0.2218487", Greater);
    test(3, 5, 20, Nearest, "-0.2218487", Greater);
    test(2, 1, 20, Floor, "0.3010297", Less);
    test(2, 1, 20, Ceiling, "0.3010302", Greater);
    test(2, 1, 20, Nearest, "0.3010302", Greater);
    test(1, 8, 20, Floor, "-0.9030905", Less);
    test(1, 8, 20, Ceiling, "-0.9030895", Greater);
    test(1, 8, 20, Nearest, "-0.9030895", Greater);
    test(7, 1, 30, Floor, "0.845098039", Less);
    test(7, 1, 30, Ceiling, "0.84509804", Greater);
    test(7, 1, 30, Nearest, "0.84509804", Greater);
    test(50, 1, 10, Floor, "1.697", Less);
    test(50, 1, 10, Ceiling, "1.699", Greater);
    test(50, 1, 10, Nearest, "1.699", Greater);
    test(1, 100, 10, Floor, "-2.0", Equal);
    test(1, 100, 10, Ceiling, "-2.0", Equal);
    test(1, 100, 10, Nearest, "-2.0", Equal);
    test(1, 100, 10, Exact, "-2.0", Equal);
    test(22, 7, 15, Floor, "0.49731", Less);
    test(22, 7, 15, Ceiling, "0.49733", Greater);
    test(22, 7, 15, Nearest, "0.49733", Greater);
    test(1000, 1, 10, Floor, "3.0", Equal);
    test(1000, 1, 10, Ceiling, "3.0", Equal);
    test(1000, 1, 10, Nearest, "3.0", Equal);
    test(1000, 1, 10, Exact, "3.0", Equal);
    test(10, 1, 10, Floor, "1.0", Equal);
    test(10, 1, 10, Ceiling, "1.0", Equal);
    test(10, 1, 10, Nearest, "1.0", Equal);
    test(10, 1, 10, Exact, "1.0", Equal);
    test(1, 10, 10, Floor, "-1.0", Equal);
    test(1, 10, 10, Ceiling, "-1.0", Equal);
    test(1, 10, 10, Nearest, "-1.0", Equal);
    test(1, 10, 10, Exact, "-1.0", Equal);
    // Special cases.
    test(0, 1, 10, Nearest, "-Infinity", Equal);
    test(0, 1, 10, Exact, "-Infinity", Equal);
    test(-3, 1, 10, Nearest, "NaN", Equal);
    test(1, 1, 10, Exact, "0.0", Equal);
}

#[test]
fn test_log_base_10_rational_prec() {
    let test = |n: i64, d: u64, prec: u64, out: &str, o_out: Ordering| {
        let x = Rational::from_signeds(n, i64::exact_from(d));
        let (log, o) = Float::log_base_10_rational_prec(x.clone(), prec);
        assert!(log.is_valid());
        assert_eq!(log.to_string(), out);
        assert_eq!(o, o_out);
        let (log_alt, o_alt) = Float::log_base_10_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&log_alt), ComparableFloatRef(&log));
        assert_eq!(o_alt, o);
    };
    test(3, 5, 20, "-0.2218487", Greater);
    test(2, 1, 20, "0.3010302", Greater);
    test(1, 8, 20, "-0.9030895", Greater);
    test(7, 1, 30, "0.84509804", Greater);
    test(50, 1, 10, "1.699", Greater);
    test(1, 100, 10, "-2.0", Equal);
    test(22, 7, 15, "0.49733", Greater);
    test(1000, 1, 10, "3.0", Equal);
    test(10, 1, 10, "1.0", Equal);
    test(1, 10, 10, "-1.0", Equal);
    test(0, 1, 10, "-Infinity", Equal);
    test(1, 1, 10, "0.0", Equal);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_log_base_10() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_log_base_10(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, f32::NEGATIVE_INFINITY);
    test::<f32>(-0.0, f32::NEGATIVE_INFINITY);
    test::<f32>(1.0, 0.0);
    test::<f32>(10.0, 1.0); // log_10(10)
    test::<f32>(100.0, 2.0); // log_10(100)
    test::<f32>(1000.0, 3.0); // log_10(1000)
    test::<f32>(2.0, std::f32::consts::LOG10_2); // log_10(2)
    test::<f32>(0.5, -std::f32::consts::LOG10_2); // log_10(1/2) = -log_10(2)
    test::<f32>(50.0, 1.6989699602127075); // log_10(50)
    test::<f32>(-1.0, f32::NAN);

    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::INFINITY);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, f64::NEGATIVE_INFINITY);
    test::<f64>(-0.0, f64::NEGATIVE_INFINITY);
    test::<f64>(1.0, 0.0);
    test::<f64>(10.0, 1.0); // log_10(10)
    test::<f64>(100.0, 2.0); // log_10(100)
    test::<f64>(1000.0, 3.0); // log_10(1000)
    test::<f64>(2.0, std::f64::consts::LOG10_2); // log_10(2)
    test::<f64>(0.5, -std::f64::consts::LOG10_2); // log_10(1/2) = -log_10(2)
    test::<f64>(50.0, 1.6989700043360187); // log_10(50)
    test::<f64>(-1.0, f64::NAN);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_log_base_10_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        // log_base_10 agrees with log_base with a base of 10.
        assert_eq!(
            NiceFloat(primitive_float_log_base_10(x)),
            NiceFloat(primitive_float_log_base(x, 10))
        );
    });
}

#[test]
fn primitive_float_log_base_10_properties() {
    apply_fn_to_primitive_floats!(primitive_float_log_base_10_properties_helper);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_log_base_10_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_log_base_10_rational(&u)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", f32::NEGATIVE_INFINITY);
    test::<f32>("1", 0.0);
    test::<f32>("10", 1.0); // log_10(10)
    test::<f32>("1000", 3.0); // log_10(1000)
    test::<f32>("1/10", -1.0); // log_10(1/10)
    test::<f32>("1/1000", -3.0); // log_10(1/1000)
    test::<f32>("2", std::f32::consts::LOG10_2); // log_10(2)
    test::<f32>("1/2", -std::f32::consts::LOG10_2); // log_10(1/2)
    test::<f32>("50", 1.6989699602127075); // log_10(50)
    test::<f32>("1/3", -0.4771212637424469); // log_10(1/3)
    test::<f32>("-1", f32::NAN);
    test::<f32>("-22/7", f32::NAN);

    test::<f64>("0", f64::NEGATIVE_INFINITY);
    test::<f64>("1", 0.0);
    test::<f64>("10", 1.0); // log_10(10)
    test::<f64>("1000", 3.0); // log_10(1000)
    test::<f64>("1/10", -1.0); // log_10(1/10)
    test::<f64>("1/1000", -3.0); // log_10(1/1000)
    test::<f64>("2", std::f64::consts::LOG10_2); // log_10(2)
    test::<f64>("1/2", -std::f64::consts::LOG10_2); // log_10(1/2)
    test::<f64>("50", 1.6989700043360187); // log_10(50)
    test::<f64>("1/3", -0.47712125471966244); // log_10(1/3)
    test::<f64>("-1", f64::NAN);
    test::<f64>("-22/7", f64::NAN);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_log_base_10_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        // log_base_10_rational agrees with log_base_rational with a base of 10.
        assert_eq!(
            NiceFloat(primitive_float_log_base_10_rational::<T>(&x)),
            NiceFloat(primitive_float_log_base_rational::<T>(&x, 10))
        );
    });
}

#[test]
fn primitive_float_log_base_10_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_log_base_10_rational_properties_helper);
}
