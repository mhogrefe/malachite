// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Acsch, AcschAssign, PowerOf2, Reciprocal};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, Two, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::acsch::{
    primitive_float_acsch, primitive_float_acsch_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, round_once_to_primitive,
    rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::acsch::{
    rug_acsch, rug_acsch_prec, rug_acsch_prec_round, rug_acsch_rational_prec,
    rug_acsch_rational_prec_round, rug_acsch_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_acsch_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().acsch_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.acsch_prec_round_ref(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.acsch_prec_round_assign(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acsch_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
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
    test("Infinity", "Infinity", 1, Floor, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 1, Nearest, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 10, Floor, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("Infinity", "Infinity", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 1, Floor, "-0.0", "-0x0.0", Equal);
    test(
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test("-Infinity", "-Infinity", 10, Floor, "-0.0", "-0x0.0", Equal);
    test(
        "-Infinity",
        "-Infinity",
        10,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        10,
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test("0.0", "0x0.0", 1, Floor, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Ceiling, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 1, Nearest, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Floor, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Nearest, "Infinity", "Infinity", Equal);
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
    test("1.0", "0x1.0#1", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Greater);
    // - x is a power of 2, so 1/x is exact and the result is asinh(1/x)
    test("1.0", "0x1.0#1", 10, Floor, "0.88086", "0x0.e18#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "0.88184",
        "0x0.e1c#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "0.88184",
        "0x0.e1c#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Floor,
        "0.88137358701954302523260932497968",
        "0x0.e1a1b30bcea13660d8f99e8dd#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Ceiling,
        "0.88137358701954302523260932498047",
        "0x0.e1a1b30bcea13660d8f99e8de#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Nearest,
        "0.88137358701954302523260932497968",
        "0x0.e1a1b30bcea13660d8f99e8dd#100",
        Less,
    );
    test("-1.0", "-0x1.0#1", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-1.0", "-0x1.0#1", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-1.0", "-0x1.0#1", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Floor,
        "-0.88184",
        "-0x0.e1c#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Ceiling,
        "-0.88086",
        "-0x0.e18#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-0.88184",
        "-0x0.e1c#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Floor,
        "-0.88137358701954302523260932498047",
        "-0x0.e1a1b30bcea13660d8f99e8de#100",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Ceiling,
        "-0.88137358701954302523260932497968",
        "-0x0.e1a1b30bcea13660d8f99e8dd#100",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        100,
        Nearest,
        "-0.88137358701954302523260932497968",
        "-0x0.e1a1b30bcea13660d8f99e8dd#100",
        Greater,
    );
    test("2.0", "0x2.0#1", 1, Floor, "0.25", "0x0.4#1", Less);
    test("2.0", "0x2.0#1", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("2.0", "0x2.0#1", 1, Nearest, "0.50", "0x0.8#1", Greater);
    test("2.0", "0x2.0#1", 10, Floor, "0.48096", "0x0.7b2#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "0.48145",
        "0x0.7b4#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "0.48145",
        "0x0.7b4#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        100,
        Floor,
        "0.48121182505960344749775891342426",
        "0x0.7b30b2bb14582652f810812a58#100",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        100,
        Ceiling,
        "0.48121182505960344749775891342465",
        "0x0.7b30b2bb14582652f810812a60#100",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        100,
        Nearest,
        "0.48121182505960344749775891342426",
        "0x0.7b30b2bb14582652f810812a58#100",
        Less,
    );
    test("0.50", "0x0.8#1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("0.50", "0x0.8#1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("0.50", "0x0.8#1", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("0.50", "0x0.8#1", 10, Floor, "1.4434", "0x1.718#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "1.4453",
        "0x1.720#10",
        Greater,
    );
    test("0.50", "0x0.8#1", 10, Nearest, "1.4434", "0x1.718#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        100,
        Floor,
        "1.4436354751788103424932767402724",
        "0x1.719218313d0872f8e831837f0#100",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        100,
        Ceiling,
        "1.4436354751788103424932767402740",
        "0x1.719218313d0872f8e831837f2#100",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        100,
        Nearest,
        "1.4436354751788103424932767402724",
        "0x1.719218313d0872f8e831837f0#100",
        Less,
    );
    test("-0.50", "-0x0.8#1", 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("-0.50", "-0x0.8#1", 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("-0.50", "-0x0.8#1", 1, Nearest, "-1.0", "-0x1.0#1", Greater);
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Floor,
        "-1.4453",
        "-0x1.720#10",
        Less,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Ceiling,
        "-1.4434",
        "-0x1.718#10",
        Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Nearest,
        "-1.4434",
        "-0x1.718#10",
        Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        100,
        Floor,
        "-1.4436354751788103424932767402740",
        "-0x1.719218313d0872f8e831837f2#100",
        Less,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        100,
        Ceiling,
        "-1.4436354751788103424932767402724",
        "-0x1.719218313d0872f8e831837f0#100",
        Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        100,
        Nearest,
        "-1.4436354751788103424932767402724",
        "-0x1.719218313d0872f8e831837f0#100",
        Greater,
    );
    test("0.75", "0x0.c#2", 1, Floor, "1.0", "0x1.0#1", Less);
    test("0.75", "0x0.c#2", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("0.75", "0x0.c#2", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("0.75", "0x0.c#2", 10, Floor, "1.0977", "0x1.190#10", Less);
    test(
        "0.75",
        "0x0.c#2",
        10,
        Ceiling,
        "1.0996",
        "0x1.198#10",
        Greater,
    );
    test("0.75", "0x0.c#2", 10, Nearest, "1.0977", "0x1.190#10", Less);
    test(
        "0.75",
        "0x0.c#2",
        100,
        Floor,
        "1.0986122886681096913952452369223",
        "0x1.193ea7aad030a976a4198d550#100",
        Less,
    );
    test(
        "0.75",
        "0x0.c#2",
        100,
        Ceiling,
        "1.0986122886681096913952452369238",
        "0x1.193ea7aad030a976a4198d552#100",
        Greater,
    );
    test(
        "0.75",
        "0x0.c#2",
        100,
        Nearest,
        "1.0986122886681096913952452369223",
        "0x1.193ea7aad030a976a4198d550#100",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
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
        Floor,
        "1.8184",
        "0x1.d18#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Ceiling,
        "1.8203",
        "0x1.d20#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Nearest,
        "1.8184",
        "0x1.d18#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Floor,
        "1.8184464592320668234836989635592",
        "0x1.d185b507edc0dfdf653c5b01c#100",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Ceiling,
        "1.8184464592320668234836989635607",
        "0x1.d185b507edc0dfdf653c5b01e#100",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Nearest,
        "1.8184464592320668234836989635607",
        "0x1.d185b507edc0dfdf653c5b01e#100",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        1,
        Floor,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        1,
        Ceiling,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        1,
        Nearest,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        10,
        Floor,
        "-1.8203",
        "-0x1.d20#10",
        Less,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        10,
        Ceiling,
        "-1.8184",
        "-0x1.d18#10",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        10,
        Nearest,
        "-1.8184",
        "-0x1.d18#10",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        100,
        Floor,
        "-1.8184464592320668234836989635607",
        "-0x1.d185b507edc0dfdf653c5b01e#100",
        Less,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        100,
        Ceiling,
        "-1.8184464592320668234836989635592",
        "-0x1.d185b507edc0dfdf653c5b01c#100",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        100,
        Nearest,
        "-1.8184464592320668234836989635607",
        "-0x1.d185b507edc0dfdf653c5b01e#100",
        Less,
    );
    test("10.0", "0xa.0#4", 1, Floor, "0.062", "0x0.1#1", Less);
    test("10.0", "0xa.0#4", 1, Ceiling, "0.12", "0x0.2#1", Greater);
    test("10.0", "0xa.0#4", 1, Nearest, "0.12", "0x0.2#1", Greater);
    test(
        "10.0",
        "0xa.0#4",
        10,
        Floor,
        "0.099731",
        "0x0.1988#10",
        Less,
    );
    test(
        "10.0",
        "0xa.0#4",
        10,
        Ceiling,
        "0.099854",
        "0x0.1990#10",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#4",
        10,
        Nearest,
        "0.099854",
        "0x0.1990#10",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#4",
        100,
        Floor,
        "0.099834078899207563327303124704685",
        "0x0.198eb9e7e5fc3e1338317fdf1a#100",
        Less,
    );
    test(
        "10.0",
        "0xa.0#4",
        100,
        Ceiling,
        "0.099834078899207563327303124704783",
        "0x0.198eb9e7e5fc3e1338317fdf1c#100",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#4",
        100,
        Nearest,
        "0.099834078899207563327303124704783",
        "0x0.198eb9e7e5fc3e1338317fdf1c#100",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        1,
        Floor,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        10,
        Floor,
        "2.9961",
        "0x2.ff#10",
        Less,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        10,
        Ceiling,
        "3.0000",
        "0x3.00#10",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        100,
        Floor,
        "2.9982229502979697388465955375953",
        "0x2.ff8b8a0da57b5aa38395e9070#100",
        Less,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        100,
        Ceiling,
        "2.9982229502979697388465955375985",
        "0x2.ff8b8a0da57b5aa38395e9074#100",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        100,
        Nearest,
        "2.9982229502979697388465955375953",
        "0x2.ff8b8a0da57b5aa38395e9070#100",
        Less,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        1,
        Floor,
        "9.5e-7",
        "0x0.00001#1",
        Less,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        1,
        Ceiling,
        "1.9e-6",
        "0x0.00002#1",
        Greater,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        1,
        Nearest,
        "9.5e-7",
        "0x0.00001#1",
        Less,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        10,
        Floor,
        "9.9838e-7",
        "0x0.000010c0#10",
        Less,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        10,
        Ceiling,
        "1.0002e-6",
        "0x0.000010c8#10",
        Greater,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        10,
        Nearest,
        "1.0002e-6",
        "0x0.000010c8#10",
        Greater,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        100,
        Floor,
        "9.9999999999983333333333340833211e-7",
        "0x0.000010c6f7a0b5ea7a2711cf943208#100",
        Less,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        100,
        Ceiling,
        "9.9999999999983333333333340833361e-7",
        "0x0.000010c6f7a0b5ea7a2711cf94320a#100",
        Greater,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        100,
        Nearest,
        "9.9999999999983333333333340833361e-7",
        "0x0.000010c6f7a0b5ea7a2711cf94320a#100",
        Greater,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        1,
        Floor,
        "-0.50",
        "-0x0.8#1",
        Less,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        1,
        Ceiling,
        "-0.25",
        "-0x0.4#1",
        Greater,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        1,
        Nearest,
        "-0.25",
        "-0x0.4#1",
        Greater,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        10,
        Floor,
        "-0.31348",
        "-0x0.504#10",
        Less,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        10,
        Ceiling,
        "-0.31299",
        "-0x0.502#10",
        Greater,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        10,
        Nearest,
        "-0.31299",
        "-0x0.502#10",
        Greater,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        100,
        Floor,
        "-0.31304384340830898212066562005260",
        "-0x0.5023a42da72003b7db346297b8#100",
        Less,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        100,
        Ceiling,
        "-0.31304384340830898212066562005221",
        "-0x0.5023a42da72003b7db346297b0#100",
        Greater,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        100,
        Nearest,
        "-0.31304384340830898212066562005221",
        "-0x0.5023a42da72003b7db346297b0#100",
        Greater,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        1,
        Floor,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        1,
        Ceiling,
        "1.3e2",
        "0x8.0E+1#1",
        Greater,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        1,
        Nearest,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        10,
        Floor,
        "70.000",
        "0x46.0#10",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        10,
        Ceiling,
        "70.125",
        "0x46.2#10",
        Greater,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        10,
        Nearest,
        "70.000",
        "0x46.0#10",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        100,
        Floor,
        "70.007865236554476251140444267256",
        "0x46.020374c5c6db00c6a6d5daf8#100",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        100,
        Ceiling,
        "70.007865236554476251140444267357",
        "0x46.020374c5c6db00c6a6d5db00#100",
        Greater,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        100,
        Nearest,
        "70.007865236554476251140444267256",
        "0x46.020374c5c6db00c6a6d5daf8#100",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        1,
        Floor,
        "3.9e-31",
        "0x8.0E-26#1",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        1,
        Ceiling,
        "7.9e-31",
        "0x1.0E-25#1",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        1,
        Nearest,
        "7.9e-31",
        "0x1.0E-25#1",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        10,
        Floor,
        "7.8809e-31",
        "0xf.fcE-26#10",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        10,
        Ceiling,
        "7.8886e-31",
        "0x1.000E-25#10",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        10,
        Nearest,
        "7.8886e-31",
        "0x1.000E-25#10",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        100,
        Floor,
        "7.8886090522101180541172856528216e-31",
        "0xf.ffffffffffffffffffffffffE-26#100",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        100,
        Ceiling,
        "7.8886090522101180541172856528279e-31",
        "0x1.0000000000000000000000000E-25#100",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        100,
        Nearest,
        "7.8886090522101180541172856528279e-31",
        "0x1.0000000000000000000000000E-25#100",
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
        Ceiling,
        "1.1e9",
        "0x4.0E+7#1",
        Greater,
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
        Floor,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
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
        Nearest,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        Floor,
        "744261118.64804019843384850454251",
        "0x2c5c85fe.a5e5f662c4486691c8#100",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        Ceiling,
        "744261118.64804019843384850454336",
        "0x2c5c85fe.a5e5f662c4486691cc#100",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        Nearest,
        "744261118.64804019843384850454336",
        "0x2c5c85fe.a5e5f662c4486691cc#100",
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
        Ceiling,
        "1.1e9",
        "0x4.0E+7#1",
        Greater,
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
        Floor,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
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
        Nearest,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        100,
        Floor,
        "744261118.64804019843384850454251",
        "0x2c5c85fe.a5e5f662c4486691c8#100",
        Less,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        100,
        Ceiling,
        "744261118.64804019843384850454336",
        "0x2c5c85fe.a5e5f662c4486691cc#100",
        Greater,
    );
    test(
        "2.3825649048879510732161697817327e-323228497",
        "0x1.0000000000000000000000000E-268435456#100",
        100,
        Nearest,
        "744261118.64804019843384850454336",
        "0x2c5c85fe.a5e5f662c4486691cc#100",
        Greater,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        1,
        Floor,
        "-9.5e-323228497",
        "-0x4.0E-268435456#1",
        Less,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        1,
        Ceiling,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Greater,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        1,
        Nearest,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Greater,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        10,
        Floor,
        "-4.7744e-323228497",
        "-0x2.01E-268435456#10",
        Less,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        10,
        Ceiling,
        "-4.7651e-323228497",
        "-0x2.00E-268435456#10",
        Greater,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        10,
        Nearest,
        "-4.7744e-323228497",
        "-0x2.01E-268435456#10",
        Less,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        100,
        Floor,
        "-4.7697878056798864105050984486729e-323228497",
        "-0x2.0080200802008020080200804E-268435456#100",
        Less,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        100,
        Ceiling,
        "-4.7697878056798864105050984486654e-323228497",
        "-0x2.0080200802008020080200800E-268435456#100",
        Greater,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        100,
        Nearest,
        "-4.7697878056798864105050984486729e-323228497",
        "-0x2.0080200802008020080200804E-268435456#100",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Floor,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Ceiling,
        "9.5e-323228497",
        "0x4.0E-268435456#1",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Nearest,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Floor,
        "4.7651e-323228497",
        "0x2.00E-268435456#10",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Ceiling,
        "4.7744e-323228497",
        "0x2.01E-268435456#10",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Nearest,
        "4.7651e-323228497",
        "0x2.00E-268435456#10",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        100,
        Floor,
        "4.7651298097759021464323395634653e-323228497",
        "0x2.0000000000000000000000000E-268435456#100",
        Less,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        100,
        Ceiling,
        "4.7651298097759021464323395634729e-323228497",
        "0x2.0000000000000000000000004E-268435456#100",
        Greater,
    );
    test(
        "2.0985787164673876924043581168822e323228496",
        "0x7.ffffffffffffffffffffffff8E+268435455#100",
        100,
        Nearest,
        "4.7651298097759021464323395634729e-323228497",
        "0x2.0000000000000000000000004E-268435456#100",
        Greater,
    );
    // - the first approximation of asinh(1/|x|) does not allow rounding, so the loop retries
    test(
        "511.969",
        "0x1ff.f8#14",
        2,
        Up,
        "0.0029",
        "0x0.00c#2",
        Greater,
    );
    // - 1/x would overflow, so ln 2 - ln|x| is used, and its first approximation does not allow
    //   rounding, so the loop retries
    test(
        "3.1306e-323228497",
        "0x1.506E-268435456#13",
        33,
        Floor,
        "744261118.25",
        "0x2c5c85fe.4#33",
        Less,
    );
    // - y = 1/|x| is so small that y^2 < 2^-wp, and asinh(y) is approximated by y
    test(
        "3348.0",
        "0xd14.0#12",
        1,
        Up,
        "0.00049",
        "0x0.002#1",
        Greater,
    );
    // - y = 1/|x| is so large that y^2 would overflow, and asinh(y) is approximated by ln y + ln 2
    test(
        "4.45e-323228484",
        "0x1.1E-268435445#5",
        5,
        Nearest,
        "7.38e8",
        "0x2.cE+7#5",
        Less,
    );
}

#[test]
#[should_panic]
fn acsch_prec_round_fail() {
    Float::TWO.acsch_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn acsch_prec_round_exact_fail() {
    Float::TWO.acsch_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acsch_prec_fail() {
    Float::TWO.acsch_prec(0);
}

#[test]
#[should_panic]
fn acsch_round_fail() {
    Float::TWO.acsch_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn acsch_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (c, o) = x.clone().acsch_prec_round(prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.acsch_prec_round_ref(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.acsch_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_acsch_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // acsch is odd
    let (c_neg, o_neg) = (-&x).acsch_prec_round(prec, -rm);
    assert_eq!(ComparableFloat(c_neg), ComparableFloat(-&c));
    assert_eq!(o_neg, o.reverse());

    // acsch is NaN only for NaN, and otherwise has the sign of x
    assert_eq!(c.is_nan(), x.is_nan());
    if !x.is_nan() {
        assert_eq!(c.is_sign_positive(), x.is_sign_positive());
    }
    if x.is_finite() && x != 0u32 && rm != Exact {
        // acsch(x) = asinh(1/x), and the reciprocal of a finite nonzero Float is an exact Rational,
        // so the independent Rational inverse hyperbolic sine must agree
        let (c_alt, o_alt) =
            Float::asinh_rational_prec_round(Rational::exact_from(&x).reciprocal(), prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }
    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o == Equal {
        // acsch is exact only for x = ±0 and ±inf (and NaN): the result is
        // rounding-mode-invariant
        for rm2 in exhaustive_rounding_modes() {
            let (c2, o2) = x.acsch_prec_round_ref(prec, rm2);
            assert_eq!(ComparableFloat(c2), ComparableFloat(c.clone()));
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.acsch_prec_round_ref(prec, Exact));
    }
}

#[test]
fn acsch_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        acsch_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (c, o) = Float::NAN.acsch_prec_round(prec, rm);
        assert!(c.is_nan());
        assert_eq!(o, Equal);
        for (x, out) in [
            (Float::INFINITY, Float::ZERO),
            (Float::NEGATIVE_INFINITY, Float::NEGATIVE_ZERO),
            (Float::ZERO, Float::INFINITY),
            (Float::NEGATIVE_ZERO, Float::NEGATIVE_INFINITY),
        ] {
            let (c, o) = x.acsch_prec_round(prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(out));
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn acsch_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (c, o) = x.clone().acsch_round(rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.acsch_round_ref(rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.acsch_round_assign(rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.acsch_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acsch_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn acsch_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (c, o) = x.clone().acsch_prec(prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.acsch_prec_ref(prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.acsch_prec_assign(prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.acsch_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (rug_c, rug_o) = rug_acsch_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn acsch_properties_helper(x: Float) {
    let c = x.clone().acsch();
    assert!(c.is_valid());
    let c_alt = (&x).acsch();
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

    let mut x_alt = x.clone();
    x_alt.acsch_assign();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));

    let c_alt = x.acsch_prec_round_ref(x.significant_bits(), Nearest).0;
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

    assert_eq!(
        ComparableFloatRef(&<Float as From<&rug::Float>>::from(&rug_acsch(
            &rug::Float::exact_from(&x)
        ))),
        ComparableFloatRef(&c)
    );

    assert_eq!(c.is_nan(), x.is_nan());
}

#[test]
fn acsch_properties() {
    float_gen().test_properties(|x| {
        acsch_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        acsch_properties_helper(x);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_acsch() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_acsch(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, 0.0);
    test::<f32>(f32::NEGATIVE_INFINITY, -0.0);
    test::<f32>(0.0, f32::INFINITY);
    test::<f32>(-0.0, f32::NEGATIVE_INFINITY);
    test::<f32>(1.0, 0.8813736);
    test::<f32>(-1.0, -0.8813736);
    test::<f32>(2.0, 0.4812118);
    test::<f32>(0.5, 1.4436355);
    test::<f32>(0.1, 2.9982228);
    test::<f32>(-10.0, -0.09983408);
    test::<f32>(1.0e-10, 23.718998);
    test::<f32>(1.0e-45, 103.97208);
    test::<f32>(3.4028235e38, 2.938736e-39);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, 0.0);
    test::<f64>(f64::NEGATIVE_INFINITY, -0.0);
    test::<f64>(0.0, f64::INFINITY);
    test::<f64>(-0.0, f64::NEGATIVE_INFINITY);
    test::<f64>(1.0, 0.881373587019543);
    test::<f64>(-1.0, -0.881373587019543);
    test::<f64>(2.0, 0.48121182505960347);
    test::<f64>(0.5, 1.4436354751788103);
    test::<f64>(0.1, 2.99822295029797);
    test::<f64>(-10.0, -0.09983407889920756);
    test::<f64>(1.0e-100, 230.95165647996453);
    test::<f64>(5.0e-324, 745.1332191019412);
    test::<f64>(1.7976931348623157e308, 5.562684646268003e-309);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acsch_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let c = primitive_float_acsch(x);
        // acsch is NaN only for NaN, and odd
        assert_eq!(c.is_nan(), x.is_nan());
        assert_eq!(NiceFloat(primitive_float_acsch(-x)), NiceFloat(-c));
        if x.is_finite() && x != T::ZERO {
            // the result is the correctly rounded inverse hyperbolic cosecant, as given by the
            // oracle
            let rug_x = rug::Float::exact_from(&Float::from(x));
            let rug_c: T = round_once_to_primitive(|p| {
                <Float as From<&rug::Float>>::from(&rug_acsch_prec(&rug_x, p).0)
            });
            assert_eq!(NiceFloat(rug_c), NiceFloat(c));
        }
    });
}

#[test]
fn primitive_float_acsch_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acsch_properties_helper);
}

#[test]
fn test_acsch_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::acsch_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        let (c, o) = Float::acsch_rational_prec_round_ref(&x, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acsch_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 1, Floor, "Infinity", "Infinity", Equal);
    test("0", 1, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 1, Down, "Infinity", "Infinity", Equal);
    test("0", 1, Up, "Infinity", "Infinity", Equal);
    test("0", 1, Nearest, "Infinity", "Infinity", Equal);
    test("0", 1, Exact, "Infinity", "Infinity", Equal);
    test("0", 10, Floor, "Infinity", "Infinity", Equal);
    test("0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 10, Down, "Infinity", "Infinity", Equal);
    test("0", 10, Up, "Infinity", "Infinity", Equal);
    test("0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("0", 10, Exact, "Infinity", "Infinity", Equal);
    test("1", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1", 10, Floor, "0.88086", "0x0.e18#10", Less);
    test("1", 10, Ceiling, "0.88184", "0x0.e1c#10", Greater);
    test("1", 10, Nearest, "0.88184", "0x0.e1c#10", Greater);
    test(
        "1",
        100,
        Floor,
        "0.88137358701954302523260932497968",
        "0x0.e1a1b30bcea13660d8f99e8dd#100",
        Less,
    );
    test(
        "1",
        100,
        Ceiling,
        "0.88137358701954302523260932498047",
        "0x0.e1a1b30bcea13660d8f99e8de#100",
        Greater,
    );
    test(
        "1",
        100,
        Nearest,
        "0.88137358701954302523260932497968",
        "0x0.e1a1b30bcea13660d8f99e8dd#100",
        Less,
    );
    test("-1", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-1", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-1", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-1", 10, Floor, "-0.88184", "-0x0.e1c#10", Less);
    test("-1", 10, Ceiling, "-0.88086", "-0x0.e18#10", Greater);
    test("-1", 10, Nearest, "-0.88184", "-0x0.e1c#10", Less);
    test(
        "-1",
        100,
        Floor,
        "-0.88137358701954302523260932498047",
        "-0x0.e1a1b30bcea13660d8f99e8de#100",
        Less,
    );
    test(
        "-1",
        100,
        Ceiling,
        "-0.88137358701954302523260932497968",
        "-0x0.e1a1b30bcea13660d8f99e8dd#100",
        Greater,
    );
    test(
        "-1",
        100,
        Nearest,
        "-0.88137358701954302523260932497968",
        "-0x0.e1a1b30bcea13660d8f99e8dd#100",
        Greater,
    );
    test("2", 1, Floor, "0.25", "0x0.4#1", Less);
    test("2", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("2", 1, Nearest, "0.50", "0x0.8#1", Greater);
    test("2", 10, Floor, "0.48096", "0x0.7b2#10", Less);
    test("2", 10, Ceiling, "0.48145", "0x0.7b4#10", Greater);
    test("2", 10, Nearest, "0.48145", "0x0.7b4#10", Greater);
    test(
        "2",
        100,
        Floor,
        "0.48121182505960344749775891342426",
        "0x0.7b30b2bb14582652f810812a58#100",
        Less,
    );
    test(
        "2",
        100,
        Ceiling,
        "0.48121182505960344749775891342465",
        "0x0.7b30b2bb14582652f810812a60#100",
        Greater,
    );
    test(
        "2",
        100,
        Nearest,
        "0.48121182505960344749775891342426",
        "0x0.7b30b2bb14582652f810812a58#100",
        Less,
    );
    test("1/2", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1/2", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1/2", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/2", 10, Floor, "1.4434", "0x1.718#10", Less);
    test("1/2", 10, Ceiling, "1.4453", "0x1.720#10", Greater);
    test("1/2", 10, Nearest, "1.4434", "0x1.718#10", Less);
    test(
        "1/2",
        100,
        Floor,
        "1.4436354751788103424932767402724",
        "0x1.719218313d0872f8e831837f0#100",
        Less,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "1.4436354751788103424932767402740",
        "0x1.719218313d0872f8e831837f2#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "1.4436354751788103424932767402724",
        "0x1.719218313d0872f8e831837f0#100",
        Less,
    );
    test("-1/3", 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("-1/3", 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("-1/3", 1, Nearest, "-2.0", "-0x2.0#1", Less);
    test("-1/3", 10, Floor, "-1.8203", "-0x1.d20#10", Less);
    test("-1/3", 10, Ceiling, "-1.8184", "-0x1.d18#10", Greater);
    test("-1/3", 10, Nearest, "-1.8184", "-0x1.d18#10", Greater);
    test(
        "-1/3",
        100,
        Floor,
        "-1.8184464592320668234836989635607",
        "-0x1.d185b507edc0dfdf653c5b01e#100",
        Less,
    );
    test(
        "-1/3",
        100,
        Ceiling,
        "-1.8184464592320668234836989635592",
        "-0x1.d185b507edc0dfdf653c5b01c#100",
        Greater,
    );
    test(
        "-1/3",
        100,
        Nearest,
        "-1.8184464592320668234836989635607",
        "-0x1.d185b507edc0dfdf653c5b01e#100",
        Less,
    );
    test("22/7", 1, Floor, "0.25", "0x0.4#1", Less);
    test("22/7", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("22/7", 1, Nearest, "0.25", "0x0.4#1", Less);
    test("22/7", 10, Floor, "0.31299", "0x0.502#10", Less);
    test("22/7", 10, Ceiling, "0.31348", "0x0.504#10", Greater);
    test("22/7", 10, Nearest, "0.31299", "0x0.502#10", Less);
    test(
        "22/7",
        100,
        Floor,
        "0.31304384340830898212066562005221",
        "0x0.5023a42da72003b7db346297b0#100",
        Less,
    );
    test(
        "22/7",
        100,
        Ceiling,
        "0.31304384340830898212066562005260",
        "0x0.5023a42da72003b7db346297b8#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "0.31304384340830898212066562005260",
        "0x0.5023a42da72003b7db346297b8#100",
        Greater,
    );
    test("-1000000", 1, Floor, "-1.9e-6", "-0x0.00002#1", Less);
    test("-1000000", 1, Ceiling, "-9.5e-7", "-0x0.00001#1", Greater);
    test("-1000000", 1, Nearest, "-9.5e-7", "-0x0.00001#1", Greater);
    test(
        "-1000000",
        10,
        Floor,
        "-1.0002e-6",
        "-0x0.000010c8#10",
        Less,
    );
    test(
        "-1000000",
        10,
        Ceiling,
        "-9.9838e-7",
        "-0x0.000010c0#10",
        Greater,
    );
    test(
        "-1000000",
        10,
        Nearest,
        "-1.0002e-6",
        "-0x0.000010c8#10",
        Less,
    );
    test(
        "-1000000",
        100,
        Floor,
        "-9.9999999999983333333333340833361e-7",
        "-0x0.000010c6f7a0b5ea7a2711cf94320a#100",
        Less,
    );
    test(
        "-1000000",
        100,
        Ceiling,
        "-9.9999999999983333333333340833211e-7",
        "-0x0.000010c6f7a0b5ea7a2711cf943208#100",
        Greater,
    );
    test(
        "-1000000",
        100,
        Nearest,
        "-9.9999999999983333333333340833361e-7",
        "-0x0.000010c6f7a0b5ea7a2711cf94320a#100",
        Less,
    );
    test("1/1000000", 1, Floor, "8.0", "0x8.0#1", Less);
    test("1/1000000", 1, Ceiling, "16.0", "0x1.0E+1#1", Greater);
    test("1/1000000", 1, Nearest, "16.0", "0x1.0E+1#1", Greater);
    test("1/1000000", 10, Floor, "14.500", "0xe.80#10", Less);
    test("1/1000000", 10, Ceiling, "14.516", "0xe.84#10", Greater);
    test("1/1000000", 10, Nearest, "14.516", "0xe.84#10", Greater);
    test(
        "1/1000000",
        100,
        Floor,
        "14.508657738524469413525180755814",
        "0xe.823764bfd1e5fa37c6bf52ed#100",
        Less,
    );
    test(
        "1/1000000",
        100,
        Ceiling,
        "14.508657738524469413525180755826",
        "0xe.823764bfd1e5fa37c6bf52ee#100",
        Greater,
    );
    test(
        "1/1000000",
        100,
        Nearest,
        "14.508657738524469413525180755814",
        "0xe.823764bfd1e5fa37c6bf52ed#100",
        Less,
    );
}

#[test]
fn test_acsch_rational_extreme() {
    // x = 2^(-2^30) is below the smallest positive `Float`; acsch(x) = asinh(2^(2^30)) lies within
    // 2^(-2^31) of ln 2^(2^30 + 1), far below an ulp of the result
    let x = Rational::power_of_2(-(1i64 << 30));
    let two_over_x = Rational::power_of_2((1i64 << 30) + 1);
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        let (c, o) = Float::acsch_rational_prec_round_ref(&x, 100, rm);
        let (l, o_l) = Float::ln_rational_prec_round_ref(&two_over_x, 100, rm);
        assert_eq!(ComparableFloat(c.clone()), ComparableFloat(l));
        assert_eq!(o, o_l);
        let (c_neg, o_neg) = Float::acsch_rational_prec_round(-&x, 100, -rm);
        assert_eq!(ComparableFloat(c_neg), ComparableFloat(-c));
        assert_eq!(o_neg, o.reverse());
    }
}

#[test]
#[should_panic]
fn acsch_rational_prec_fail() {
    Float::acsch_rational_prec(Rational::TWO, 0);
}

#[test]
#[should_panic]
fn acsch_rational_prec_ref_fail() {
    Float::acsch_rational_prec_ref(&Rational::TWO, 0);
}

#[test]
#[should_panic]
fn acsch_rational_prec_round_fail_1() {
    Float::acsch_rational_prec_round(Rational::TWO, 0, Floor);
}

#[test]
#[should_panic]
fn acsch_rational_prec_round_fail_2() {
    Float::acsch_rational_prec_round(Rational::TWO, 10, Exact);
}

#[test]
#[should_panic]
fn acsch_rational_prec_round_ref_fail() {
    Float::acsch_rational_prec_round_ref(&Rational::TWO, 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn acsch_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (c, o) = Float::acsch_rational_prec_round(x.clone(), prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = Float::acsch_rational_prec_round_ref(&x, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // acsch is odd (a `Rational` has no negative zero, so x = 0 is excluded), and has the sign of x
    if x != 0u32 {
        let (c_neg, o_neg) = Float::acsch_rational_prec_round(-&x, prec, -rm);
        assert_eq!(ComparableFloatRef(&c_neg), ComparableFloatRef(&-&c));
        assert_eq!(o_neg, o.reverse());
        assert_eq!(c > 0u32, x > 0u32);
    } else {
        assert_eq!(ComparableFloat(c.clone()), ComparableFloat(Float::INFINITY));
    }

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_acsch_rational_prec_round(&x, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // the inverse hyperbolic cosecant of an exactly representable rational is the Float inverse
    // hyperbolic cosecant, computed by an independent algorithm
    if let Ok(f) = Float::try_from(&x)
        && (rm != Exact || o == Equal)
    {
        let (c_alt, o_alt) = f.acsch_prec_round(prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }

    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::acsch_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::acsch_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn acsch_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        acsch_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (c, o) = Float::acsch_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn acsch_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (c, o) = Float::acsch_rational_prec(x.clone(), prec);
    assert!(c.is_valid());

    let (c_alt, o_alt) = Float::acsch_rational_prec_ref(&x, prec);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let (c_alt, o_alt) = Float::acsch_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let (rug_c, rug_o) = rug_acsch_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_c)),
        ComparableFloatRef(&c)
    );
    assert_eq!(rug_o, o);
}

#[test]
fn acsch_rational_prec_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, _)| {
        acsch_rational_prec_properties_helper(x, prec);
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        acsch_rational_prec_properties_helper(x, prec);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_acsch_rational() {
    fn test<T: PrimitiveFloat>(x: &Rational, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_acsch_rational::<T>(x)),
            NiceFloat(out)
        );
    }
    let test_s = |s: &str| Rational::from_str(s).unwrap();
    test::<f32>(&test_s("0"), f32::INFINITY);
    test::<f64>(&test_s("0"), f64::INFINITY);
    test::<f32>(&test_s("1"), 0.8813736);
    test::<f64>(&test_s("1"), 0.881373587019543);
    test::<f32>(&test_s("-1"), -0.8813736);
    test::<f64>(&test_s("-1"), -0.881373587019543);
    test::<f32>(&test_s("2"), 0.4812118);
    test::<f64>(&test_s("2"), 0.48121182505960347);
    test::<f32>(&test_s("1/2"), 1.4436355);
    test::<f64>(&test_s("1/2"), 1.4436354751788103);
    test::<f32>(&test_s("-1/3"), -1.8184465);
    test::<f64>(&test_s("-1/3"), -1.8184464592320668);
    test::<f32>(&test_s("100000000000000000000"), 1.0e-20);
    test::<f64>(&test_s("100000000000000000000"), 1.0e-20);
    test::<f32>(&test_s("1/100000000000000000000"), 46.74485);
    test::<f64>(&test_s("1/100000000000000000000"), 46.74484904044086);

    // 1/x lies exactly halfway between two adjacent subnormals, or at half the smallest one, and
    // acsch(x) lies just short of it, so the result rounds down
    let x = Rational::power_of_2(150i64) / Rational::from(32767u32);
    test::<f32>(&x, 2.2957e-41);
    test::<f64>(&x, 2.295817339026564e-41);
    let x = Rational::power_of_2(150i64);
    test::<f32>(&x, 0.0);
    test::<f64>(&x, 7.006492321624085e-46);
    let x = Rational::power_of_2(1075i64) / Rational::from(32767u32);
    test::<f32>(&x, 0.0);
    test::<f64>(&x, 8.0943e-320);
    let x = Rational::power_of_2(1075i64);
    test::<f32>(&x, 0.0);
    test::<f64>(&x, 0.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acsch_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let c = primitive_float_acsch_rational::<T>(&x);
        // the inverse hyperbolic cosecant of a rational is never NaN
        assert!(!c.is_nan());
        if x != 0u32 {
            let rug_c: T = round_once_to_primitive(|p| {
                <Float as From<&rug::Float>>::from(&rug_acsch_rational_prec(&x, p).0)
            });
            assert_eq!(NiceFloat(rug_c), NiceFloat(c));
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // The inverse hyperbolic cosecant of a finite nonzero primitive float, taken through the
        // `Rational` path, matches the direct primitive-float inverse hyperbolic cosecant (a
        // `Rational` cannot carry the sign of a zero).
        if x.is_finite() && x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_acsch_rational::<T>(&Rational::exact_from(
                    x
                ))),
                NiceFloat(primitive_float_acsch(x))
            );
        }
    });
}

#[test]
fn primitive_float_acsch_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acsch_rational_properties_helper);
}
