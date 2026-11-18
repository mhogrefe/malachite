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
use malachite_base::num::exponential::traits::{WrightOmega, WrightOmegaAssign};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::exponential::wright_omega::{
    primitive_float_wright_omega, primitive_float_wright_omega_rational,
};
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, wright_omega_prec_round_naive,
    wright_omega_rational_prec_round_naive,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, exponent_in_gate, parse_hex_string, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_55,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_58,
    rational_unsigned_rounding_mode_triple_gen_var_25,
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
fn test_wright_omega() {
    let test = |s, s_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let wright_omega = x.clone().wright_omega();
        assert!(wright_omega.is_valid());

        assert_eq!(wright_omega.to_string(), out);
        assert_eq!(to_hex_string(&wright_omega), out_hex);

        let wright_omega_alt = (&x).wright_omega();
        assert!(wright_omega_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&wright_omega),
            ComparableFloatRef(&wright_omega_alt)
        );

        let mut wright_omega_alt = x.clone();
        wright_omega_alt.wright_omega_assign();
        assert!(wright_omega_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&wright_omega),
            ComparableFloatRef(&wright_omega_alt)
        );
    };
    test("NaN", "NaN", "NaN", "NaN");
    test("Infinity", "Infinity", "Infinity", "Infinity");
    test("-Infinity", "-Infinity", "0.0", "0x0.0");
    test("0.0", "0x0.0", "0.50", "0x0.8#1");
    test("-0.0", "-0x0.0", "0.50", "0x0.8#1");
    test("1.0", "0x1.0#1", "1.0", "0x1.0#1");
    test("0.50", "0x0.8#1", "1.0", "0x1.0#1");
    test("2.0", "0x2.0#1", "2.0", "0x2.0#1");
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        "2.0167797648922003",
        "0x2.044badbd72526#53",
    );
    test("10.0", "0xa.0#3", "8.0", "0x8.0#3");
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        "1.00000000e10",
        "0x2.540be4E+8#24",
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        "0.56714690937971801",
        "0x0.91308a33d038a8#53",
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        "0.5671432904097831",
        "0x0.91304d7c74b28#49",
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        "118.67957280414448",
        "0x76.adf87bb90d34#53",
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        "0.53169161977901758",
        "0x0.881cf1268215f#52",
    );
    test("-0.25", "-0x0.4#1", "0.50", "0x0.8#1");
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        "0.46522789553303523",
        "0x0.77192ce4805498#53",
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        "0.56713967145458533",
        "0x0.913010c5296080#53",
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        "1.0e323228496",
        "0x4.0E+268435455#1",
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
    );
    test("4.0e-323228249", "0x1.8E-268435250#2", "0.50", "0x0.8#2");
    test("-4.0e-323228249", "-0x1.8E-268435250#2", "0.50", "0x0.8#2");
}

#[test]
fn test_wright_omega_prec() {
    let test = |s, s_hex, prec: u64, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (wright_omega, o) = x.clone().wright_omega_prec(prec);
        assert!(wright_omega.is_valid());

        assert_eq!(wright_omega.to_string(), out);
        assert_eq!(to_hex_string(&wright_omega), out_hex);
        assert_eq!(o, o_out);

        let (wright_omega_alt, o_alt) = x.wright_omega_prec_ref(prec);
        assert!(wright_omega_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&wright_omega),
            ComparableFloatRef(&wright_omega_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut wright_omega_alt = x.clone();
        let o_alt = wright_omega_alt.wright_omega_prec_assign(prec);
        assert!(wright_omega_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&wright_omega),
            ComparableFloatRef(&wright_omega_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    test("NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, "Infinity", "Infinity", Equal);
    test("Infinity", "Infinity", 10, "Infinity", "Infinity", Equal);
    test("-Infinity", "-Infinity", 1, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 10, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, "0.50", "0x0.8#1", Less);
    test("0.0", "0x0.0", 10, "0.56738", "0x0.914#10", Greater);
    test("-0.0", "-0x0.0", 1, "0.50", "0x0.8#1", Less);
    test("-0.0", "-0x0.0", 10, "0.56738", "0x0.914#10", Greater);
    test("1.0", "0x1.0#1", 1, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 10, "1.0000", "0x1.000#10", Equal);
    test(
        "1.0",
        "0x1.0#1",
        100,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("0.50", "0x0.8#1", 1, "1.0", "0x1.0#1", Greater);
    test("0.50", "0x0.8#1", 10, "0.76660", "0x0.c44#10", Greater);
    test(
        "0.50",
        "0x0.8#1",
        100,
        "0.76624860816175025887746864453104",
        "0x0.c428de68a9053aa76ede661bd#100",
        Less,
    );
    test("2.0", "0x2.0#1", 1, "2.0", "0x2.0#1", Greater);
    test("2.0", "0x2.0#1", 10, "1.5566", "0x1.8e8#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        100,
        "1.5571455989976114168586720000009",
        "0x1.8ea1180ece19291135fc7040c#100",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        "2.0156",
        "0x2.04#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        100,
        "2.0167797648922005276333012100077",
        "0x2.044badbd72526d3c57a9ac6f8#100",
        Greater,
    );
    test("10.0", "0xa.0#3", 1, "8.0", "0x8.0#1", Greater);
    test("10.0", "0xa.0#3", 10, "7.9297", "0x7.ee#10", Greater);
    test(
        "10.0",
        "0xa.0#3",
        100,
        "7.9294200950196973485617282661700",
        "0x7.edee79b05ad4ff6d32f0138a8#100",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1,
        "8.6e9",
        "0x2.0E+8#1",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        "9.9992e9",
        "0x2.54E+8#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        100,
        "9999999976.9741490723621282552382",
        "0x2540be3e8.f961d567395d32768#100",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        1,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        100,
        "0.56714690937971800106724074169531",
        "0x0.91308a33d038a73ca88759d7f#100",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        1,
        "8.7e99",
        "0x1.0E+83#1",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        "9.9964e99",
        "0x1.248E+83#10",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        100,
        "1.0000000000000000159028911097599e100",
        "0x1.249ad2594c37d000000000000E+83#100",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        1,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        100,
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        "118.62",
        "0x76.a#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        100,
        "118.67957280414448995527768753067",
        "0x76.adf87bb90d35805b7f4169b0#100",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        1,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        "0.53125",
        "0x0.880#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        100,
        "0.53169161977901767910682001737564",
        "0x0.881cf1268215f75dda93234ed#100",
        Less,
    );
    test("-0.25", "-0x0.4#1", 1, "0.50", "0x0.8#1", Greater);
    test("-0.25", "-0x0.4#1", 10, "0.48145", "0x0.7b4#10", Greater);
    test(
        "-0.25",
        "-0x0.4#1",
        100,
        "0.48128846639758644396592790375854",
        "0x0.7b35b88f1e9ffcb5d404acc268#100",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        1,
        "0.50",
        "0x0.8#1",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        "0.46533",
        "0x0.772#10",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        100,
        "0.46522789553303524952088817836664",
        "0x0.77192ce480549971e21d9e3d30#100",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        1,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        100,
        "0.56713967145458530596793841539708",
        "0x0.913010c529607e93ea4f1b381#100",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        100,
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        100,
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        100,
        "1.0492893582336938462021790584419e323228496",
        "0x4.0000000000000000000000000E+268435455#100",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        100,
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        100,
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        1,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        100,
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
        Less,
    );
}

#[test]
#[should_panic]
fn wright_omega_prec_fail() {
    Float::NAN.wright_omega_prec(0);
}

#[test]
fn test_wright_omega_round() {
    let test = |s, s_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (wright_omega, o) = x.clone().wright_omega_round(rm);
        assert!(wright_omega.is_valid());

        assert_eq!(wright_omega.to_string(), out);
        assert_eq!(to_hex_string(&wright_omega), out_hex);
        assert_eq!(o, o_out);

        let (wright_omega_alt, o_alt) = x.wright_omega_round_ref(rm);
        assert!(wright_omega_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&wright_omega),
            ComparableFloatRef(&wright_omega_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut wright_omega_alt = x.clone();
        let o_alt = wright_omega_alt.wright_omega_round_assign(rm);
        assert!(wright_omega_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&wright_omega),
            ComparableFloatRef(&wright_omega_alt)
        );
        assert_eq!(o_alt, o_out);
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
    test("-Infinity", "-Infinity", Floor, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", Ceiling, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", Down, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", Up, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", Floor, "0.50", "0x0.8#1", Less);
    test("0.0", "0x0.0", Ceiling, "1.0", "0x1.0#1", Greater);
    test("0.0", "0x0.0", Down, "0.50", "0x0.8#1", Less);
    test("0.0", "0x0.0", Up, "1.0", "0x1.0#1", Greater);
    test("0.0", "0x0.0", Nearest, "0.50", "0x0.8#1", Less);
    test("-0.0", "-0x0.0", Floor, "0.50", "0x0.8#1", Less);
    test("-0.0", "-0x0.0", Ceiling, "1.0", "0x1.0#1", Greater);
    test("-0.0", "-0x0.0", Down, "0.50", "0x0.8#1", Less);
    test("-0.0", "-0x0.0", Up, "1.0", "0x1.0#1", Greater);
    test("-0.0", "-0x0.0", Nearest, "0.50", "0x0.8#1", Less);
    test("NaN", "NaN", Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1.0#1", Floor, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Ceiling, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Down, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Up, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", Nearest, "1.0", "0x1.0#1", Equal);
    test("0.50", "0x0.8#1", Floor, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", Ceiling, "1.0", "0x1.0#1", Greater);
    test("0.50", "0x0.8#1", Down, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", Up, "1.0", "0x1.0#1", Greater);
    test("0.50", "0x0.8#1", Nearest, "1.0", "0x1.0#1", Greater);
    test("2.0", "0x2.0#1", Floor, "1.0", "0x1.0#1", Less);
    test("2.0", "0x2.0#1", Ceiling, "2.0", "0x2.0#1", Greater);
    test("2.0", "0x2.0#1", Down, "1.0", "0x1.0#1", Less);
    test("2.0", "0x2.0#1", Up, "2.0", "0x2.0#1", Greater);
    test("2.0", "0x2.0#1", Nearest, "2.0", "0x2.0#1", Greater);
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Floor,
        "2.0167797648922003",
        "0x2.044badbd72526#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Ceiling,
        "2.0167797648922008",
        "0x2.044badbd72528#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Down,
        "2.0167797648922003",
        "0x2.044badbd72526#53",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Up,
        "2.0167797648922008",
        "0x2.044badbd72528#53",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        Nearest,
        "2.0167797648922003",
        "0x2.044badbd72526#53",
        Less,
    );
    test("10.0", "0xa.0#3", Floor, "7.0", "0x7.0#3", Less);
    test("10.0", "0xa.0#3", Ceiling, "8.0", "0x8.0#3", Greater);
    test("10.0", "0xa.0#3", Down, "7.0", "0x7.0#3", Less);
    test("10.0", "0xa.0#3", Up, "8.0", "0x8.0#3", Greater);
    test("10.0", "0xa.0#3", Nearest, "8.0", "0x8.0#3", Greater);
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Floor,
        "9.99999898e9",
        "0x2.540be0E+8#24",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Ceiling,
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Down,
        "9.99999898e9",
        "0x2.540be0E+8#24",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Up,
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Nearest,
        "1.00000000e10",
        "0x2.540be4E+8#24",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Floor,
        "0.56714690937971790",
        "0x0.91308a33d038a0#53",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Ceiling,
        "0.56714690937971801",
        "0x0.91308a33d038a8#53",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Down,
        "0.56714690937971790",
        "0x0.91308a33d038a0#53",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Up,
        "0.56714690937971801",
        "0x0.91308a33d038a8#53",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        Nearest,
        "0.56714690937971801",
        "0x0.91308a33d038a8#53",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Floor,
        "9.9999999999999982e99",
        "0x1.249ad2594c37cE+83#53",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Ceiling,
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Down,
        "9.9999999999999982e99",
        "0x1.249ad2594c37cE+83#53",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Up,
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Nearest,
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Floor,
        "0.5671432904097831",
        "0x0.91304d7c74b28#49",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Ceiling,
        "0.5671432904097848",
        "0x0.91304d7c74b30#49",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Down,
        "0.5671432904097831",
        "0x0.91304d7c74b28#49",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Up,
        "0.5671432904097848",
        "0x0.91304d7c74b30#49",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        Nearest,
        "0.5671432904097831",
        "0x0.91304d7c74b28#49",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Floor,
        "118.67957280414448",
        "0x76.adf87bb90d34#53",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Ceiling,
        "118.67957280414450",
        "0x76.adf87bb90d38#53",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Down,
        "118.67957280414448",
        "0x76.adf87bb90d34#53",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Up,
        "118.67957280414450",
        "0x76.adf87bb90d38#53",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        Nearest,
        "118.67957280414448",
        "0x76.adf87bb90d34#53",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Floor,
        "0.53169161977901758",
        "0x0.881cf1268215f#52",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Ceiling,
        "0.53169161977901780",
        "0x0.881cf12682160#52",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Down,
        "0.53169161977901758",
        "0x0.881cf1268215f#52",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Up,
        "0.53169161977901780",
        "0x0.881cf12682160#52",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        Nearest,
        "0.53169161977901758",
        "0x0.881cf1268215f#52",
        Less,
    );
    test("-0.25", "-0x0.4#1", Floor, "0.25", "0x0.4#1", Less);
    test("-0.25", "-0x0.4#1", Ceiling, "0.50", "0x0.8#1", Greater);
    test("-0.25", "-0x0.4#1", Down, "0.25", "0x0.4#1", Less);
    test("-0.25", "-0x0.4#1", Up, "0.50", "0x0.8#1", Greater);
    test("-0.25", "-0x0.4#1", Nearest, "0.50", "0x0.8#1", Greater);
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Floor,
        "0.46522789553303523",
        "0x0.77192ce4805498#53",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Ceiling,
        "0.46522789553303528",
        "0x0.77192ce480549c#53",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Down,
        "0.46522789553303523",
        "0x0.77192ce4805498#53",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Up,
        "0.46522789553303528",
        "0x0.77192ce480549c#53",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        Nearest,
        "0.46522789553303523",
        "0x0.77192ce4805498#53",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Floor,
        "0.56713967145458521",
        "0x0.913010c5296078#53",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Ceiling,
        "0.56713967145458533",
        "0x0.913010c5296080#53",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Down,
        "0.56713967145458521",
        "0x0.913010c5296078#53",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Up,
        "0.56713967145458533",
        "0x0.913010c5296080#53",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        Nearest,
        "0.56713967145458533",
        "0x0.913010c5296080#53",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Floor,
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Ceiling,
        "0.56714329040978387299996866221081",
        "0x0.91304d7c74b2ba5eafddaa629#100",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Down,
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Up,
        "0.56714329040978387299996866221081",
        "0x0.91304d7c74b2ba5eafddaa629#100",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        Nearest,
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Floor,
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Ceiling,
        "0.56714329040978387299996866221081",
        "0x0.91304d7c74b2ba5eafddaa629#100",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Down,
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Up,
        "0.56714329040978387299996866221081",
        "0x0.91304d7c74b2ba5eafddaa629#100",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        Nearest,
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Floor,
        "5.2e323228495",
        "0x2.0E+268435455#1",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Ceiling,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Down,
        "5.2e323228495",
        "0x2.0E+268435455#1",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Up,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Nearest,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Floor,
        "2.0985787164673876924043581168805e323228496",
        "0x7.ffffffffffffffffffffffff0E+268435455#100",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Ceiling,
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Down,
        "2.0985787164673876924043581168805e323228496",
        "0x7.ffffffffffffffffffffffff0E+268435455#100",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Up,
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Nearest,
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Floor,
        "0.50",
        "0x0.8#2",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Ceiling,
        "0.75",
        "0x0.c#2",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Down,
        "0.50",
        "0x0.8#2",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Up,
        "0.75",
        "0x0.c#2",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        Nearest,
        "0.50",
        "0x0.8#2",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Floor,
        "0.50",
        "0x0.8#2",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Ceiling,
        "0.75",
        "0x0.c#2",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Down,
        "0.50",
        "0x0.8#2",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Up,
        "0.75",
        "0x0.c#2",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        Nearest,
        "0.50",
        "0x0.8#2",
        Less,
    );
}

#[test]
#[should_panic]
fn wright_omega_round_fail() {
    Float::from(0.5).wright_omega_round(Exact);
}

#[test]
fn test_wright_omega_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (wright_omega, o) = x.clone().wright_omega_prec_round(prec, rm);
        assert!(wright_omega.is_valid());

        assert_eq!(wright_omega.to_string(), out);
        assert_eq!(to_hex_string(&wright_omega), out_hex);
        assert_eq!(o, o_out);

        let (wright_omega_alt, o_alt) = x.wright_omega_prec_round_ref(prec, rm);
        assert!(wright_omega_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&wright_omega),
            ComparableFloatRef(&wright_omega_alt)
        );
        assert_eq!(o_alt, o_out);

        let mut wright_omega_alt = x.clone();
        let o_alt = wright_omega_alt.wright_omega_prec_round_assign(prec, rm);
        assert!(wright_omega_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&wright_omega),
            ComparableFloatRef(&wright_omega_alt)
        );
        assert_eq!(o_alt, o_out);
    };
    // The expected values agree with Azurite's verified Wright omega function.
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test("-Infinity", "-Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 10, Down, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 10, Up, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Floor, "0.56641", "0x0.910#10", Less);
    test(
        "0.0",
        "0x0.0",
        10,
        Ceiling,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test("0.0", "0x0.0", 10, Down, "0.56641", "0x0.910#10", Less);
    test("0.0", "0x0.0", 10, Up, "0.56738", "0x0.914#10", Greater);
    test(
        "0.0",
        "0x0.0",
        10,
        Nearest,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test("-0.0", "-0x0.0", 10, Floor, "0.56641", "0x0.910#10", Less);
    test(
        "-0.0",
        "-0x0.0",
        10,
        Ceiling,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test("-0.0", "-0x0.0", 10, Down, "0.56641", "0x0.910#10", Less);
    test("-0.0", "-0x0.0", 10, Up, "0.56738", "0x0.914#10", Greater);
    test(
        "-0.0",
        "-0x0.0",
        10,
        Nearest,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test("NaN", "NaN", 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("1.0", "0x1.0#1", 10, Down, "1.0000", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Up, "1.0000", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test(
        "1.0",
        "0x1.0#1",
        64,
        Nearest,
        "1.00000000000000000000",
        "0x1.0000000000000000#64",
        Equal,
    );
    test("0.50", "0x0.8#1", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("0.50", "0x0.8#1", 10, Down, "0.76562", "0x0.c40#10", Less);
    test("0.50", "0x0.8#1", 10, Up, "0.76660", "0x0.c44#10", Greater);
    test("0.50", "0x0.8#1", 10, Floor, "0.76562", "0x0.c40#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "0.76660",
        "0x0.c44#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        10,
        Nearest,
        "0.76660",
        "0x0.c44#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        64,
        Nearest,
        "0.766248608161750258854",
        "0x0.c428de68a9053aa7#64",
        Less,
    );
    test("2.0", "0x2.0#1", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("2.0", "0x2.0#1", 10, Down, "1.5566", "0x1.8e8#10", Less);
    test("2.0", "0x2.0#1", 10, Up, "1.5586", "0x1.8f0#10", Greater);
    test("2.0", "0x2.0#1", 10, Floor, "1.5566", "0x1.8e8#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "1.5586",
        "0x1.8f0#10",
        Greater,
    );
    test("2.0", "0x2.0#1", 10, Nearest, "1.5566", "0x1.8e8#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        64,
        Nearest,
        "1.55714559899761141690",
        "0x1.8ea1180ece192912#64",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Down,
        "2.0156",
        "0x2.04#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Up,
        "2.0195",
        "0x2.05#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Floor,
        "2.0156",
        "0x2.04#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Ceiling,
        "2.0195",
        "0x2.05#10",
        Greater,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        10,
        Nearest,
        "2.0156",
        "0x2.04#10",
        Less,
    );
    test(
        "2.7182818284590451",
        "0x2.b7e151628aed2#53",
        64,
        Nearest,
        "2.01677976489220052761",
        "0x2.044badbd72526d3c#64",
        Less,
    );
    test("10.0", "0xa.0#3", 1, Nearest, "8.0", "0x8.0#1", Greater);
    test("10.0", "0xa.0#3", 10, Down, "7.9219", "0x7.ec#10", Less);
    test("10.0", "0xa.0#3", 10, Up, "7.9297", "0x7.ee#10", Greater);
    test("10.0", "0xa.0#3", 10, Floor, "7.9219", "0x7.ec#10", Less);
    test(
        "10.0",
        "0xa.0#3",
        10,
        Ceiling,
        "7.9297",
        "0x7.ee#10",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#3",
        10,
        Nearest,
        "7.9297",
        "0x7.ee#10",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#3",
        64,
        Nearest,
        "7.92942009501969734871",
        "0x7.edee79b05ad4ff70#64",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1,
        Nearest,
        "8.6e9",
        "0x2.0E+8#1",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Down,
        "9.9992e9",
        "0x2.54E+8#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Up,
        "1.0016e10",
        "0x2.55E+8#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Floor,
        "9.9992e9",
        "0x2.54E+8#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Ceiling,
        "1.0016e10",
        "0x2.55E+8#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Nearest,
        "9.9992e9",
        "0x2.54E+8#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        64,
        Nearest,
        "9999999976.97414907254",
        "0x2540be3e8.f961d568#64",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        1,
        Nearest,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Down,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Up,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Floor,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Ceiling,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        10,
        Nearest,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "0.000010000000000000001",
        "0x0.0000a7c5ac471b4788#53",
        64,
        Nearest,
        "0.567146909379718001086",
        "0x0.91308a33d038a73d#64",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        1,
        Nearest,
        "8.7e99",
        "0x1.0E+83#1",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Down,
        "9.9964e99",
        "0x1.248E+83#10",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Up,
        "1.0014e100",
        "0x1.250E+83#10",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Floor,
        "9.9964e99",
        "0x1.248E+83#10",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Ceiling,
        "1.0014e100",
        "0x1.250E+83#10",
        Greater,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        10,
        Nearest,
        "9.9964e99",
        "0x1.248E+83#10",
        Less,
    );
    test(
        "1.0000000000000000e100",
        "0x1.249ad2594c37dE+83#53",
        64,
        Nearest,
        "1.00000000000000001590e100",
        "0x1.249ad2594c37d000E+83#64",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        1,
        Nearest,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Down,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Up,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Floor,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Ceiling,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        10,
        Nearest,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "1.000000000000000e-100",
        "0xd.ff9772470298E-84#49",
        64,
        Nearest,
        "0.567143290409783873017",
        "0x0.91304d7c74b2ba5f#64",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        Nearest,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Down,
        "118.62",
        "0x76.a#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Up,
        "118.75",
        "0x76.c#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Floor,
        "118.62",
        "0x76.a#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Ceiling,
        "118.75",
        "0x76.c#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        10,
        Nearest,
        "118.62",
        "0x76.a#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        64,
        Nearest,
        "118.679572804144489957",
        "0x76.adf87bb90d35808#64",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        1,
        Nearest,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Down,
        "0.53125",
        "0x0.880#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Up,
        "0.53223",
        "0x0.884#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Floor,
        "0.53125",
        "0x0.880#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Ceiling,
        "0.53223",
        "0x0.884#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Nearest,
        "0.53125",
        "0x0.880#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        64,
        Nearest,
        "0.531691619779017679115",
        "0x0.881cf1268215f75e#64",
        Greater,
    );
    test("-0.25", "-0x0.4#1", 1, Nearest, "0.50", "0x0.8#1", Greater);
    test("-0.25", "-0x0.4#1", 10, Down, "0.48096", "0x0.7b2#10", Less);
    test(
        "-0.25",
        "-0x0.4#1",
        10,
        Up,
        "0.48145",
        "0x0.7b4#10",
        Greater,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        10,
        Floor,
        "0.48096",
        "0x0.7b2#10",
        Less,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        10,
        Ceiling,
        "0.48145",
        "0x0.7b4#10",
        Greater,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        10,
        Nearest,
        "0.48145",
        "0x0.7b4#10",
        Greater,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        64,
        Nearest,
        "0.481288466397586443975",
        "0x0.7b35b88f1e9ffcb60#64",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        1,
        Nearest,
        "0.50",
        "0x0.8#1",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Down,
        "0.46484",
        "0x0.770#10",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Up,
        "0.46533",
        "0x0.772#10",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Floor,
        "0.46484",
        "0x0.770#10",
        Less,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Ceiling,
        "0.46533",
        "0x0.772#10",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        10,
        Nearest,
        "0.46533",
        "0x0.772#10",
        Greater,
    );
    test(
        "-0.29999999999999999",
        "-0x0.4ccccccccccccc#53",
        64,
        Nearest,
        "0.465227895533035249527",
        "0x0.77192ce4805499720#64",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        1,
        Nearest,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        Down,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        Up,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        Floor,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        Ceiling,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        10,
        Nearest,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-0.000010000000000000001",
        "-0x0.0000a7c5ac471b4788#53",
        64,
        Nearest,
        "0.567139671454585305973",
        "0x0.913010c529607e94#64",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Down,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        1,
        Nearest,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Down,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Up,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Floor,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Ceiling,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        10,
        Nearest,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        64,
        Nearest,
        "0.567143290409783873017",
        "0x0.91304d7c74b2ba5f#64",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Down,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        1,
        Nearest,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Down,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Up,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Floor,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Ceiling,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        10,
        Nearest,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-2.3825649048879510732161697817327e-323228497",
        "-0x1.0000000000000000000000000E-268435456#100",
        64,
        Nearest,
        "0.567143290409783873017",
        "0x0.91304d7c74b2ba5f#64",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Down,
        "5.2e323228495",
        "0x2.0E+268435455#1",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Up,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Floor,
        "5.2e323228495",
        "0x2.0E+268435455#1",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Ceiling,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        1,
        Nearest,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Down,
        "1.0483e323228496",
        "0x3.ffE+268435455#10",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Up,
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Floor,
        "1.0483e323228496",
        "0x3.ffE+268435455#10",
        Less,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Ceiling,
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        10,
        Nearest,
        "1.0493e323228496",
        "0x4.00E+268435455#10",
        Greater,
    );
    test(
        "1.0e323228496",
        "0x4.0E+268435455#1",
        64,
        Nearest,
        "1.04928935823369384620e323228496",
        "0x4.0000000000000000E+268435455#64",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Down,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Floor,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Down,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        64,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Down,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        1,
        Nearest,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Down,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Up,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Floor,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Ceiling,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        10,
        Nearest,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "4.0e-323228249",
        "0x1.8E-268435250#2",
        64,
        Nearest,
        "0.567143290409783873017",
        "0x0.91304d7c74b2ba5f#64",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        1,
        Down,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        1,
        Nearest,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Down,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Up,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Floor,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Ceiling,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        10,
        Nearest,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-4.0e-323228249",
        "-0x1.8E-268435250#2",
        64,
        Nearest,
        "0.567143290409783873017",
        "0x0.91304d7c74b2ba5f#64",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "0.27832",
        "0x0.474#10",
        Less,
    );
    test(
        "-10.0",
        "-0xa.0#3",
        20,
        Floor,
        "0.000045397843",
        "0x0.0002f9a64#20",
        Less,
    );
    test(
        "-100.0",
        "-0x64.0#5",
        20,
        Ceiling,
        "3.7200779e-44",
        "0xd.4610E-37#20",
        Greater,
    );
    test("1.0", "0x1.0#1", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test(
        "100.0",
        "0x64.0#5",
        20,
        Up,
        "95.441528",
        "0x5f.7108#20",
        Greater,
    );
    test("-1.1e9", "-0x4.0E+7#1", 10, Floor, "0.0", "0x0.0", Less);
    test(
        "-1.1e9",
        "-0x4.0E+7#1",
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test("-1.1e9", "-0x4.0E+7#1", 10, Nearest, "0.0", "0x0.0", Less);
    test("-4.3e9", "-0x1.0E+8#1", 10, Floor, "0.0", "0x0.0", Less);
    test(
        "-4.3e9",
        "-0x1.0E+8#1",
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test("-4.3e9", "-0x1.0E+8#1", 10, Down, "0.0", "0x0.0", Less);
    test(
        "-4.3e9",
        "-0x1.0E+8#1",
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test("-4.3e9", "-0x1.0E+8#1", 10, Nearest, "0.0", "0x0.0", Less);
}

#[test]
#[should_panic]
fn wright_omega_prec_round_fail_1() {
    Float::ONE.wright_omega_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn wright_omega_prec_round_fail_2() {
    Float::from(0.5).wright_omega_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn wright_omega_prec_round_fail_3() {
    Float::ZERO.wright_omega_prec_round(10, Exact);
}

// Whether the reference, whose time grows with the argument, is run on `x`.
fn reference_is_cheap(x: &Float) -> bool {
    x.get_exponent().is_none_or(|e| e <= 8)
}

// Checks a result that is finite and nonzero and did not overflow or underflow against the
// reference, which computes W_0(e^x).
fn check_rounding(x: &Float, wright_omega: &Float, o: Ordering, prec: u64, rm: RoundingMode) {
    if rm != Exact
        && wright_omega.is_normal()
        && result_in_range(wright_omega, o)
        && reference_is_cheap(x)
        && *x != 1u32
    {
        let (g, go) = wright_omega_prec_round_naive(&float_to_big_exponent_float(x), prec, rm);
        assert_eq!(float_to_big_exponent_float(wright_omega), g);
        assert_eq!(o, go);
    }
}

#[allow(clippy::needless_pass_by_value)]
fn wright_omega_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (wright_omega, o) = x.clone().wright_omega_prec_round(prec, rm);
    assert!(wright_omega.is_valid());
    assert_rounding_ordering_consistent(&wright_omega, rm, o);

    let (wright_omega_alt, o_alt) = x.wright_omega_prec_round_ref(prec, rm);
    assert!(wright_omega_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&wright_omega_alt),
        ComparableFloatRef(&wright_omega)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.wright_omega_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&wright_omega)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &wright_omega, o, prec, rm);

    if wright_omega.is_normal() {
        assert_eq!(wright_omega.get_prec(), Some(prec));
    }
    if !x.is_nan() {
        // omega is positive and increasing, and omega(1) = 1.
        assert!(wright_omega >= 0u32);
        assert!(!wright_omega.is_sign_negative());
        if x < 1u32 {
            assert!(wright_omega <= 1u32);
        } else {
            assert!(wright_omega >= 1u32);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.wright_omega_prec_round_ref(prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(wright_omega.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.wright_omega_prec_round_ref(prec, Exact));
    }

    // A `Float` that is not too large or too small gives the same result as the equal `Rational`.
    if x.is_normal() && exponent_in_gate(&x) {
        let (r, ro) = Float::wright_omega_rational_prec_round(Rational::exact_from(&x), prec, rm);
        assert_eq!(ComparableFloatRef(&r), ComparableFloatRef(&wright_omega));
        assert_eq!(ro, o);
    }
}

#[test]
fn wright_omega_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_58().test_properties(|(x, prec, rm)| {
        wright_omega_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_rounding_mode_triple_gen_var_58().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            wright_omega_prec_round_properties_helper(x, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.wright_omega_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::INFINITY.wright_omega_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_INFINITY.wright_omega_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        // omega(1) = 1
        let (s, o) = Float::ONE.wright_omega_prec_round(prec, rm);
        assert_eq!(s, 1u32);
        assert_eq!(o, Equal);

        // omega(0) = W_0(1) is irrational, so `Exact` panics; the sign of the zero does not matter
        if rm == Exact {
            assert_panic!(Float::ZERO.wright_omega_prec_round(prec, Exact));
            assert_panic!(Float::NEGATIVE_ZERO.wright_omega_prec_round(prec, Exact));
        } else {
            let (s, o) = Float::ZERO.wright_omega_prec_round(prec, rm);
            let (t, to) = Float::NEGATIVE_ZERO.wright_omega_prec_round(prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(t));
            assert_eq!(o, to);
        }
    });
}

#[allow(clippy::needless_pass_by_value)]
fn wright_omega_prec_properties_helper(x: Float, prec: u64) {
    let (wright_omega, o) = x.clone().wright_omega_prec(prec);
    assert!(wright_omega.is_valid());

    let (wright_omega_alt, o_alt) = x.wright_omega_prec_ref(prec);
    assert!(wright_omega_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&wright_omega_alt),
        ComparableFloatRef(&wright_omega)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.wright_omega_prec_assign(prec);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&wright_omega)
    );
    assert_eq!(o_alt, o);

    let (wright_omega_alt, o_alt) = x.wright_omega_prec_round_ref(prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&wright_omega_alt),
        ComparableFloatRef(&wright_omega)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &wright_omega, o, prec, Nearest);

    if wright_omega.is_normal() {
        assert_eq!(wright_omega.get_prec(), Some(prec));
    }
    if !x.is_nan() {
        // omega is positive and increasing, and omega(1) = 1.
        assert!(wright_omega >= 0u32);
        assert!(!wright_omega.is_sign_negative());
        if x < 1u32 {
            assert!(wright_omega <= 1u32);
        } else {
            assert!(wright_omega >= 1u32);
        }
    }
}

#[test]
fn wright_omega_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        wright_omega_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    config.insert("mean_small_n", 512);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        wright_omega_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        wright_omega_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (s, o) = Float::NAN.wright_omega_prec(prec);
        assert!(s.is_nan());
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn wright_omega_round_properties_helper(x: Float, rm: RoundingMode) {
    let (wright_omega, o) = x.clone().wright_omega_round(rm);
    assert!(wright_omega.is_valid());
    assert_rounding_ordering_consistent(&wright_omega, rm, o);

    let (wright_omega_alt, o_alt) = x.wright_omega_round_ref(rm);
    assert!(wright_omega_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&wright_omega_alt),
        ComparableFloatRef(&wright_omega)
    );
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.wright_omega_round_assign(rm);
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&wright_omega)
    );
    assert_eq!(o_alt, o);

    let (wright_omega_alt, o_alt) = x.wright_omega_prec_round_ref(x.significant_bits(), rm);
    assert_eq!(
        ComparableFloatRef(&wright_omega_alt),
        ComparableFloatRef(&wright_omega)
    );
    assert_eq!(o_alt, o);

    check_rounding(&x, &wright_omega, o, x.significant_bits(), rm);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.wright_omega_round_ref(rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(wright_omega.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.wright_omega_round_ref(Exact));
    }
}

#[test]
fn wright_omega_round_properties() {
    float_rounding_mode_pair_gen_var_55().test_properties(|(x, rm)| {
        wright_omega_round_properties_helper(x, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_rounding_mode_pair_gen_var_55().test_properties_with_config(&config, |(x, rm)| {
        wright_omega_round_properties_helper(x, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn wright_omega_properties_helper(x: Float) {
    let wright_omega = x.clone().wright_omega();
    assert!(wright_omega.is_valid());

    let wright_omega_alt = (&x).wright_omega();
    assert!(wright_omega_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&wright_omega_alt),
        ComparableFloatRef(&wright_omega)
    );

    let mut x_alt = x.clone();
    x_alt.wright_omega_assign();
    assert!(x_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&x_alt),
        ComparableFloatRef(&wright_omega)
    );

    let wright_omega_alt = x
        .wright_omega_prec_round_ref(x.significant_bits(), Nearest)
        .0;
    assert_eq!(
        ComparableFloatRef(&wright_omega_alt),
        ComparableFloatRef(&wright_omega)
    );
    let wright_omega_alt = x.wright_omega_prec_ref(x.significant_bits()).0;
    assert_eq!(
        ComparableFloatRef(&wright_omega_alt),
        ComparableFloatRef(&wright_omega)
    );
    let wright_omega_alt = x.wright_omega_round_ref(Nearest).0;
    assert_eq!(
        ComparableFloatRef(&wright_omega_alt),
        ComparableFloatRef(&wright_omega)
    );
    if !x.is_nan() {
        // omega is positive and increasing, and omega(1) = 1.
        assert!(wright_omega >= 0u32);
        assert!(!wright_omega.is_sign_negative());
        if x < 1u32 {
            assert!(wright_omega <= 1u32);
        } else {
            assert!(wright_omega >= 1u32);
        }
    }
}

#[test]
fn wright_omega_properties() {
    float_gen().test_properties(|x| {
        wright_omega_properties_helper(x);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 512);
    config.insert("mean_stripe_n", 2 << Limb::LOG_WIDTH);
    float_gen().test_properties_with_config(&config, |x| {
        wright_omega_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        wright_omega_properties_helper(x);
    });
}

#[test]
fn test_primitive_float_wright_omega() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_wright_omega(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, 0.0);
    test::<f32>(0.0, 0.56714326);
    test::<f32>(-0.0, 0.56714326);
    test::<f32>(1.0, 1.0);
    test::<f32>(2.7182817, 2.0167797);
    test::<f32>(7.0, 5.3271785);
    test::<f32>(100.0, 95.44149);
    test::<f32>(1.0e-10, 0.56714326);
    test::<f32>(-0.1, 0.5316916);
    test::<f32>(-0.25, 0.48128846);
    test::<f32>(-0.36787942, 0.44401577);
    test::<f32>(-0.5, 0.40467384);
    test::<f32>(3.4028235e38, 3.4028235e38);
    test::<f32>(1.1754944e-38, 0.56714326);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::INFINITY);
    test::<f64>(f64::NEGATIVE_INFINITY, 0.0);
    test::<f64>(0.0, 0.5671432904097838);
    test::<f64>(-0.0, 0.5671432904097838);
    test::<f64>(1.0, 1.0);
    test::<f64>(core::f64::consts::E, 2.0167797648922003);
    test::<f64>(7.0, 5.327178301371093);
    test::<f64>(100.0, 95.44148664557584);
    test::<f64>(1.0e-10, 0.5671432904459734);
    test::<f64>(-0.1, 0.5316916197790177);
    test::<f64>(-0.25, 0.48128846639758643);
    test::<f64>(-0.36787944117144233, 0.44401576630745326);
    test::<f64>(-0.5, 0.4046738485459385);
    test::<f64>(1.7976931348623157e308, 1.7976931348623157e308);
    test::<f64>(2.2250738585072014e-308, 0.5671432904097838);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_wright_omega_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        primitive_float_wright_omega(x);
    });
}

#[test]
fn primitive_float_wright_omega_properties() {
    apply_fn_to_primitive_floats!(primitive_float_wright_omega_properties_helper);
}

#[test]
fn test_wright_omega_rational_prec() {
    let test = |s, prec, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::wright_omega_rational_prec(x.clone(), prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::wright_omega_rational_prec_ref(&x, prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 1, "0.50", "0x0.8#1", Less);
    test("0", 10, "0.56738", "0x0.914#10", Greater);
    test("1", 1, "1.0", "0x1.0#1", Equal);
    test("1", 10, "1.0000", "0x1.000#10", Equal);
    test(
        "1",
        100,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("1/3", 1, "0.50", "0x0.8#1", Less);
    test("1/3", 10, "0.69629", "0x0.b24#10", Greater);
    test(
        "1/3",
        100,
        "0.69589287666479101712602367747266",
        "0x0.b226091acb6d75b2274c6d22d#100",
        Less,
    );
    test("-1/3", 1, "0.50", "0x0.8#1", Greater);
    test("-1/3", 10, "0.45459", "0x0.746#10", Less);
    test(
        "-1/3",
        100,
        "0.45472628316922338628141243024808",
        "0x0.7468f112d7ec516df5c48bed48#100",
        Greater,
    );
    test("22/7", 1, "2.0", "0x2.0#1", Less);
    test("22/7", 10, "2.3086", "0x2.4f#10", Greater);
    test(
        "22/7",
        100,
        "2.3069366318254238292510662616118",
        "0x2.4e93662ba2745e181247148a0#100",
        Less,
    );
    test("-1/4", 1, "0.50", "0x0.8#1", Greater);
    test("-1/4", 10, "0.48145", "0x0.7b4#10", Greater);
    test(
        "-1/4",
        100,
        "0.48128846639758644396592790375854",
        "0x0.7b35b88f1e9ffcb5d404acc268#100",
        Greater,
    );
    test("-1/1000", 1, "0.50", "0x0.8#1", Less);
    test("-1/1000", 10, "0.56641", "0x0.910#10", Less);
    test(
        "-1/1000",
        100,
        "0.56678146783229538350898884623781",
        "0x0.9118971c518ec41226222fccc#100",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        1,
        "1.3e30",
        "0x1.0E+25#1",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        "1.0003e30",
        "0xc.a0E+24#10",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        100,
        "999999999999999999999999999931.00",
        "0xc9f2c9cd04674edea3fffffbb.0#100",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        1,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        100,
        "0.56714329040978387299996866221081",
        "0x0.91304d7c74b2ba5eafddaa629#100",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        1,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        100,
        "0.56714329040978387299996866221002",
        "0x0.91304d7c74b2ba5eafddaa628#100",
        Greater,
    );
    test("-11/30", 1, "0.50", "0x0.8#1", Greater);
    test("-11/30", 10, "0.44434", "0x0.71c#10", Less);
    test(
        "-11/30",
        100,
        "0.44438878688970166124816283616586",
        "0x0.71c376aa667f728e3cd7e04740#100",
        Greater,
    );
    test("-3/8", 10, "0.44189", "0x0.712#10", Greater);
    test(
        "-4598493014643029/12500000000000000",
        1,
        "0.50",
        "0x0.8#1",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        "0.44385",
        "0x0.71a#10",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        100,
        "0.44401576630745324021323638965913",
        "0x0.71ab046b32ea7a53241ed47290#100",
        Greater,
    );
    test(
        "-36787944117144233/100000000000000000",
        10,
        "0.44385",
        "0x0.71a#10",
        Less,
    );
}

#[test]
#[should_panic]
fn wright_omega_rational_prec_fail() {
    Float::wright_omega_rational_prec(Rational::ONE, 0);
}

#[test]
#[should_panic]
fn wright_omega_rational_prec_ref_fail() {
    Float::wright_omega_rational_prec_ref(&Rational::ONE, 0);
}

#[test]
fn test_wright_omega_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::wright_omega_rational_prec_round(x.clone(), prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::wright_omega_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 10, Floor, "0.56641", "0x0.910#10", Less);
    test("0", 10, Ceiling, "0.56738", "0x0.914#10", Greater);
    test("0", 10, Down, "0.56641", "0x0.910#10", Less);
    test("0", 10, Up, "0.56738", "0x0.914#10", Greater);
    test("0", 10, Nearest, "0.56738", "0x0.914#10", Greater);
    test(
        "1",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test("1", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Down, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Up, "1.0000", "0x1.000#10", Equal);
    test("1", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test(
        "1/3",
        53,
        Nearest,
        "0.69589287666479105",
        "0x0.b226091acb6d78#53",
        Greater,
    );
    test("1/3", 10, Floor, "0.69531", "0x0.b20#10", Less);
    test("1/3", 10, Ceiling, "0.69629", "0x0.b24#10", Greater);
    test("1/3", 10, Down, "0.69531", "0x0.b20#10", Less);
    test("1/3", 10, Up, "0.69629", "0x0.b24#10", Greater);
    test("1/3", 10, Nearest, "0.69629", "0x0.b24#10", Greater);
    test(
        "-1/3",
        53,
        Nearest,
        "0.45472628316922337",
        "0x0.7468f112d7ec50#53",
        Less,
    );
    test("-1/3", 10, Floor, "0.45459", "0x0.746#10", Less);
    test("-1/3", 10, Ceiling, "0.45508", "0x0.748#10", Greater);
    test("-1/3", 10, Down, "0.45459", "0x0.746#10", Less);
    test("-1/3", 10, Up, "0.45508", "0x0.748#10", Greater);
    test("-1/3", 10, Nearest, "0.45459", "0x0.746#10", Less);
    test(
        "22/7",
        53,
        Nearest,
        "2.3069366318254239",
        "0x2.4e93662ba2746#53",
        Greater,
    );
    test("22/7", 10, Floor, "2.3047", "0x2.4e#10", Less);
    test("22/7", 10, Ceiling, "2.3086", "0x2.4f#10", Greater);
    test("22/7", 10, Down, "2.3047", "0x2.4e#10", Less);
    test("22/7", 10, Up, "2.3086", "0x2.4f#10", Greater);
    test("22/7", 10, Nearest, "2.3086", "0x2.4f#10", Greater);
    test(
        "-1/4",
        53,
        Nearest,
        "0.48128846639758643",
        "0x0.7b35b88f1e9ffc#53",
        Less,
    );
    test("-1/4", 10, Floor, "0.48096", "0x0.7b2#10", Less);
    test("-1/4", 10, Ceiling, "0.48145", "0x0.7b4#10", Greater);
    test("-1/4", 10, Down, "0.48096", "0x0.7b2#10", Less);
    test("-1/4", 10, Up, "0.48145", "0x0.7b4#10", Greater);
    test("-1/4", 10, Nearest, "0.48145", "0x0.7b4#10", Greater);
    test(
        "-1/1000",
        53,
        Nearest,
        "0.56678146783229544",
        "0x0.9118971c518ec8#53",
        Greater,
    );
    test("-1/1000", 10, Floor, "0.56641", "0x0.910#10", Less);
    test("-1/1000", 10, Ceiling, "0.56738", "0x0.914#10", Greater);
    test("-1/1000", 10, Down, "0.56641", "0x0.910#10", Less);
    test("-1/1000", 10, Up, "0.56738", "0x0.914#10", Greater);
    test("-1/1000", 10, Nearest, "0.56641", "0x0.910#10", Less);
    test(
        "1000000000000000000000000000000",
        53,
        Nearest,
        "1.0000000000000000e30",
        "0xc.9f2c9cd046750E+24#53",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Floor,
        "9.9902e29",
        "0xc.9cE+24#10",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Ceiling,
        "1.0003e30",
        "0xc.a0E+24#10",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Down,
        "9.9902e29",
        "0xc.9cE+24#10",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Up,
        "1.0003e30",
        "0xc.a0E+24#10",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        10,
        Nearest,
        "1.0003e30",
        "0xc.a0E+24#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        53,
        Nearest,
        "0.56714329040978384",
        "0x0.91304d7c74b2b8#53",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Floor,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Ceiling,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Down,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Up,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        10,
        Nearest,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        53,
        Nearest,
        "0.56714329040978384",
        "0x0.91304d7c74b2b8#53",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Floor,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Ceiling,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Down,
        "0.56641",
        "0x0.910#10",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Up,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        10,
        Nearest,
        "0.56738",
        "0x0.914#10",
        Greater,
    );
    test(
        "-11/30",
        53,
        Nearest,
        "0.44438878688970168",
        "0x0.71c376aa667f74#53",
        Greater,
    );
    test("-11/30", 10, Floor, "0.44434", "0x0.71c#10", Less);
    test("-11/30", 10, Ceiling, "0.44482", "0x0.71e#10", Greater);
    test("-11/30", 10, Down, "0.44434", "0x0.71c#10", Less);
    test("-11/30", 10, Up, "0.44482", "0x0.71e#10", Greater);
    test("-11/30", 10, Nearest, "0.44434", "0x0.71c#10", Less);
    test("-3/8", 10, Nearest, "0.44189", "0x0.712#10", Greater);
    test(
        "-4598493014643029/12500000000000000",
        53,
        Nearest,
        "0.44401576630745326",
        "0x0.71ab046b32ea7c#53",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Floor,
        "0.44385",
        "0x0.71a#10",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Ceiling,
        "0.44434",
        "0x0.71c#10",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Down,
        "0.44385",
        "0x0.71a#10",
        Less,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Up,
        "0.44434",
        "0x0.71c#10",
        Greater,
    );
    test(
        "-4598493014643029/12500000000000000",
        10,
        Nearest,
        "0.44385",
        "0x0.71a#10",
        Less,
    );
    test(
        "-36787944117144233/100000000000000000",
        10,
        Nearest,
        "0.44385",
        "0x0.71a#10",
        Less,
    );
    test("1", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test(
        "-10",
        20,
        Nearest,
        "0.000045397843",
        "0x0.0002f9a64#20",
        Less,
    );
    test("-1/3", 20, Floor, "0.45472622", "0x0.7468f0#20", Less);
    test("1000", 20, Ceiling, "993.09961", "0x3e1.198#20", Greater);
    test("-4294967296", 10, Nearest, "0.0", "0x0.0", Less);
    test(
        "-4294967296",
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
}

#[test]
#[should_panic]
fn wright_omega_rational_prec_round_fail_1() {
    Float::wright_omega_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn wright_omega_rational_prec_round_fail_2() {
    Float::wright_omega_rational_prec_round(Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[test]
#[should_panic]
fn wright_omega_rational_prec_round_ref_fail() {
    Float::wright_omega_rational_prec_round_ref(&Rational::from_unsigneds(1u32, 3), 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn wright_omega_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (wright_omega, o) = Float::wright_omega_rational_prec_round(x.clone(), prec, rm);
    assert!(wright_omega.is_valid());
    assert_rounding_ordering_consistent(&wright_omega, rm, o);

    let (wright_omega_alt, o_alt) = Float::wright_omega_rational_prec_round_ref(&x, prec, rm);
    assert!(wright_omega_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&wright_omega_alt),
        ComparableFloatRef(&wright_omega)
    );
    assert_eq!(o_alt, o);

    // The reference computes W_0(e^x), in time that grows with x.
    if rm != Exact
        && wright_omega.is_normal()
        && result_in_range(&wright_omega, o)
        && x.le_abs(&256u32)
        && x != 1u32
    {
        let (g, go) = wright_omega_rational_prec_round_naive(&x, prec, rm);
        assert_eq!(float_to_big_exponent_float(&wright_omega), g);
        assert_eq!(o, go);
    }

    if wright_omega.is_normal() {
        assert_eq!(wright_omega.get_prec(), Some(prec));
    }
    assert!(wright_omega >= 0u32);
    if x < 1u32 {
        assert!(wright_omega <= 1u32);
    } else {
        assert!(wright_omega >= 1u32);
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::wright_omega_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(wright_omega.clone()));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::wright_omega_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn wright_omega_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_25().test_properties(|(x, prec, rm)| {
        wright_omega_rational_prec_round_properties_helper(x, prec, rm);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn wright_omega_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (wright_omega, o) = Float::wright_omega_rational_prec(x.clone(), prec);
    assert!(wright_omega.is_valid());

    let (wright_omega_alt, o_alt) = Float::wright_omega_rational_prec_ref(&x, prec);
    assert!(wright_omega_alt.is_valid());
    assert_eq!(
        ComparableFloatRef(&wright_omega_alt),
        ComparableFloatRef(&wright_omega)
    );
    assert_eq!(o_alt, o);

    let (wright_omega_alt, o_alt) = Float::wright_omega_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(
        ComparableFloatRef(&wright_omega_alt),
        ComparableFloatRef(&wright_omega)
    );
    assert_eq!(o_alt, o);
}

#[test]
fn wright_omega_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        wright_omega_rational_prec_properties_helper(x, prec);
    });
}

#[test]
fn test_primitive_float_wright_omega_rational() {
    #[allow(clippy::type_repetition_in_bounds)]
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_wright_omega_rational::<T>(
                &Rational::from_str(s).unwrap()
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0.56714326);
    test::<f64>("0", 0.5671432904097838);
    test::<f32>("1", 1.0);
    test::<f64>("1", 1.0);
    test::<f32>("1/3", 0.6958929);
    test::<f64>("1/3", 0.695892876664791);
    test::<f32>("-1/3", 0.45472628);
    test::<f64>("-1/3", 0.45472628316922337);
    test::<f32>("-1/4", 0.48128846);
    test::<f64>("-1/4", 0.48128846639758643);
    test::<f32>("-1/2", 0.40467384);
    test::<f64>("-1/2", 0.4046738485459385);
    test::<f32>("1000", 993.0992);
    test::<f64>("1000", 993.0991694723891);
    test::<f32>("-1/1000", 0.56678146);
    test::<f64>("-1/1000", 0.5667814678322954);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_wright_omega_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        primitive_float_wright_omega_rational::<T>(&x);
    });
}

#[test]
fn primitive_float_wright_omega_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_wright_omega_rational_properties_helper);
}
