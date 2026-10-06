// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Acosh, AcoshAssign, PowerOf2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Two, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::acosh::{
    primitive_float_acosh, primitive_float_acosh_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, round_once_to_primitive,
    rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::acosh::{
    rug_acosh, rug_acosh_prec, rug_acosh_prec_round, rug_acosh_rational_prec,
    rug_acosh_rational_prec_round, rug_acosh_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_15,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_acosh_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().acosh_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.acosh_prec_round_ref(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.acosh_prec_round_assign(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acosh_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test("-Infinity", "-Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Nearest, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 10, Nearest, "NaN", "NaN", Equal);
    test("1.0", "0x1.0#1", 1, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 100, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 100, Nearest, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 10, Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 100, Floor, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 100, Ceiling, "NaN", "NaN", Equal);
    test("-1.0", "-0x1.0#1", 100, Nearest, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 10, Floor, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 10, Nearest, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 100, Floor, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 100, Ceiling, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 100, Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Floor, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 100, Floor, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 100, Ceiling, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 100, Nearest, "NaN", "NaN", Equal);
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("2.0", "0x2.0#2", 10, Floor, "1.3164", "0x1.510#10", Less);
    test(
        "2.0",
        "0x2.0#2",
        10,
        Ceiling,
        "1.3184",
        "0x1.518#10",
        Greater,
    );
    test("2.0", "0x2.0#2", 10, Nearest, "1.3164", "0x1.510#10", Less);
    test(
        "2.0",
        "0x2.0#2",
        100,
        Floor,
        "1.3169578969248167086250463473073",
        "0x1.51242719804349be684bd0188#100",
        Less,
    );
    test(
        "2.0",
        "0x2.0#2",
        100,
        Ceiling,
        "1.3169578969248167086250463473089",
        "0x1.51242719804349be684bd018a#100",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#2",
        100,
        Nearest,
        "1.3169578969248167086250463473073",
        "0x1.51242719804349be684bd0188#100",
        Less,
    );
    test("10.0", "0xa.0#4", 10, Floor, "2.9922", "0x2.fe#10", Less);
    test(
        "10.0",
        "0xa.0#4",
        10,
        Ceiling,
        "2.9961",
        "0x2.ff#10",
        Greater,
    );
    test("10.0", "0xa.0#4", 10, Nearest, "2.9922", "0x2.fe#10", Less);
    test(
        "10.0",
        "0xa.0#4",
        100,
        Floor,
        "2.9932228461263808979126677137722",
        "0x2.fe43da39c0ba87f9072bef0f4#100",
        Less,
    );
    test(
        "10.0",
        "0xa.0#4",
        100,
        Ceiling,
        "2.9932228461263808979126677137753",
        "0x2.fe43da39c0ba87f9072bef0f8#100",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#4",
        100,
        Nearest,
        "2.9932228461263808979126677137753",
        "0x2.fe43da39c0ba87f9072bef0f8#100",
        Greater,
    );
    test("-100.0", "-0x64.0#7", 10, Floor, "NaN", "NaN", Equal);
    test("-100.0", "-0x64.0#7", 10, Ceiling, "NaN", "NaN", Equal);
    test("-100.0", "-0x64.0#7", 10, Nearest, "NaN", "NaN", Equal);
    test("-100.0", "-0x64.0#7", 100, Floor, "NaN", "NaN", Equal);
    test("-100.0", "-0x64.0#7", 100, Ceiling, "NaN", "NaN", Equal);
    test("-100.0", "-0x64.0#7", 100, Nearest, "NaN", "NaN", Equal);
    test(
        "1000.0",
        "0x3e8.0#10",
        10,
        Floor,
        "7.5938",
        "0x7.98#10",
        Less,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        10,
        Ceiling,
        "7.6016",
        "0x7.9a#10",
        Greater,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        10,
        Nearest,
        "7.6016",
        "0x7.9a#10",
        Greater,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        100,
        Floor,
        "7.6009022095419886114191231179932",
        "0x7.99d4ba2a13b4f9dc2aee43d48#100",
        Less,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        100,
        Ceiling,
        "7.6009022095419886114191231179995",
        "0x7.99d4ba2a13b4f9dc2aee43d50#100",
        Greater,
    );
    test(
        "1000.0",
        "0x3e8.0#10",
        100,
        Nearest,
        "7.6009022095419886114191231179995",
        "0x7.99d4ba2a13b4f9dc2aee43d50#100",
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        10,
        Floor,
        "1.8105",
        "0x1.cf8#10",
        Less,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        10,
        Ceiling,
        "1.8125",
        "0x1.d00#10",
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        10,
        Nearest,
        "1.8125",
        "0x1.d00#10",
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        100,
        Floor,
        "1.8115262724608531070386963769034",
        "0x1.cfc02f90106c1a9ad73c26d1a#100",
        Less,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        100,
        Ceiling,
        "1.8115262724608531070386963769050",
        "0x1.cfc02f90106c1a9ad73c26d1c#100",
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        100,
        Nearest,
        "1.8115262724608531070386963769050",
        "0x1.cfc02f90106c1a9ad73c26d1c#100",
        Greater,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        10,
        Floor,
        "14.500",
        "0xe.80#10",
        Less,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        10,
        Ceiling,
        "14.516",
        "0xe.84#10",
        Greater,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        10,
        Nearest,
        "14.516",
        "0xe.84#10",
        Greater,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        100,
        Floor,
        "14.508657738523969413525180755811",
        "0xe.823764bfd1593d6bbd500264#100",
        Less,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        100,
        Ceiling,
        "14.508657738523969413525180755824",
        "0xe.823764bfd1593d6bbd500265#100",
        Greater,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        100,
        Nearest,
        "14.508657738523969413525180755811",
        "0xe.823764bfd1593d6bbd500264#100",
        Less,
    );
    test(
        "-100000000.0",
        "-0x5f5e100.0#27",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-100000000.0",
        "-0x5f5e100.0#27",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-100000000.0",
        "-0x5f5e100.0#27",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-100000000.0",
        "-0x5f5e100.0#27",
        100,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-100000000.0",
        "-0x5f5e100.0#27",
        100,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-100000000.0",
        "-0x5f5e100.0#27",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("0.00098", "0x0.004#1", 10, Floor, "NaN", "NaN", Equal);
    test("-0.00098", "-0x0.004#1", 10, Floor, "NaN", "NaN", Equal);
    test("0.00098", "0x0.004#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-0.00098", "-0x0.004#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("0.00098", "0x0.004#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-0.00098", "-0x0.004#1", 10, Nearest, "NaN", "NaN", Equal);
    test("0.00098", "0x0.004#1", 10, Down, "NaN", "NaN", Equal);
    test("-0.00098", "-0x0.004#1", 10, Down, "NaN", "NaN", Equal);
    test("0.00098", "0x0.004#1", 10, Up, "NaN", "NaN", Equal);
    test("-0.00098", "-0x0.004#1", 10, Up, "NaN", "NaN", Equal);
    test("0.00098", "0x0.004#1", 100, Floor, "NaN", "NaN", Equal);
    test("-0.00098", "-0x0.004#1", 100, Floor, "NaN", "NaN", Equal);
    test("0.00098", "0x0.004#1", 100, Ceiling, "NaN", "NaN", Equal);
    test("-0.00098", "-0x0.004#1", 100, Ceiling, "NaN", "NaN", Equal);
    test("0.00098", "0x0.004#1", 100, Nearest, "NaN", "NaN", Equal);
    test("-0.00098", "-0x0.004#1", 100, Nearest, "NaN", "NaN", Equal);
    test("0.00098", "0x0.004#1", 100, Down, "NaN", "NaN", Equal);
    test("-0.00098", "-0x0.004#1", 100, Down, "NaN", "NaN", Equal);
    test("0.00098", "0x0.004#1", 100, Up, "NaN", "NaN", Equal);
    test("-0.00098", "-0x0.004#1", 100, Up, "NaN", "NaN", Equal);
    test("8.9e-16", "0x4.0E-13#1", 100, Floor, "NaN", "NaN", Equal);
    test("-8.9e-16", "-0x4.0E-13#1", 100, Floor, "NaN", "NaN", Equal);
    test("8.9e-16", "0x4.0E-13#1", 100, Ceiling, "NaN", "NaN", Equal);
    test(
        "-8.9e-16",
        "-0x4.0E-13#1",
        100,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test("8.9e-16", "0x4.0E-13#1", 100, Nearest, "NaN", "NaN", Equal);
    test(
        "-8.9e-16",
        "-0x4.0E-13#1",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("8.9e-16", "0x4.0E-13#1", 100, Down, "NaN", "NaN", Equal);
    test("-8.9e-16", "-0x4.0E-13#1", 100, Down, "NaN", "NaN", Equal);
    test("8.9e-16", "0x4.0E-13#1", 100, Up, "NaN", "NaN", Equal);
    test("-8.9e-16", "-0x4.0E-13#1", 100, Up, "NaN", "NaN", Equal);
    test("7.9e-31", "0x1.0E-25#1", 10, Floor, "NaN", "NaN", Equal);
    test("-7.9e-31", "-0x1.0E-25#1", 10, Floor, "NaN", "NaN", Equal);
    test("7.9e-31", "0x1.0E-25#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-7.9e-31", "-0x1.0E-25#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("7.9e-31", "0x1.0E-25#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-7.9e-31", "-0x1.0E-25#1", 10, Nearest, "NaN", "NaN", Equal);
    test("7.9e-31", "0x1.0E-25#1", 10, Down, "NaN", "NaN", Equal);
    test("-7.9e-31", "-0x1.0E-25#1", 10, Down, "NaN", "NaN", Equal);
    test("7.9e-31", "0x1.0E-25#1", 10, Up, "NaN", "NaN", Equal);
    test("-7.9e-31", "-0x1.0E-25#1", 10, Up, "NaN", "NaN", Equal);
    test("7.9e-31", "0x1.0E-25#1", 300, Floor, "NaN", "NaN", Equal);
    test("-7.9e-31", "-0x1.0E-25#1", 300, Floor, "NaN", "NaN", Equal);
    test("7.9e-31", "0x1.0E-25#1", 300, Ceiling, "NaN", "NaN", Equal);
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        300,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test("7.9e-31", "0x1.0E-25#1", 300, Nearest, "NaN", "NaN", Equal);
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        300,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("7.9e-31", "0x1.0E-25#1", 300, Down, "NaN", "NaN", Equal);
    test("-7.9e-31", "-0x1.0E-25#1", 300, Down, "NaN", "NaN", Equal);
    test("7.9e-31", "0x1.0E-25#1", 300, Up, "NaN", "NaN", Equal);
    test("-7.9e-31", "-0x1.0E-25#1", 300, Up, "NaN", "NaN", Equal);
    test(
        "9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "-0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "-0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "-0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "-0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-9.0949470177292823791503906250000000000000000000000000000000000e-13",
        "-0x1.00000000000000000000000000000000000000000000000000E-10#200",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        64,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        64,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        64,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        64,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        64,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        64,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        64,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        64,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.0e-301030",
        "0x1.0E-250000#1",
        64,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "700000000.00",
        "0x29b92700.0#30",
        10,
        Floor,
        "21.031",
        "0x15.08#10",
        Less,
    );
    test(
        "700000000.00",
        "0x29b92700.0#30",
        10,
        Ceiling,
        "21.062",
        "0x15.10#10",
        Greater,
    );
    test(
        "700000000.00",
        "0x29b92700.0#30",
        10,
        Nearest,
        "21.062",
        "0x15.10#10",
        Greater,
    );
    test(
        "700000000.00",
        "0x29b92700.0#30",
        10,
        Down,
        "21.031",
        "0x15.08#10",
        Less,
    );
    test(
        "700000000.00",
        "0x29b92700.0#30",
        10,
        Up,
        "21.062",
        "0x15.10#10",
        Greater,
    );
    test(
        "700000000.00",
        "0x29b92700.0#30",
        100,
        Floor,
        "21.059738073567624086156312420738",
        "0x15.0f4afe904c8a71c6a1f42ca0#100",
        Less,
    );
    test(
        "700000000.00",
        "0x29b92700.0#30",
        100,
        Ceiling,
        "21.059738073567624086156312420763",
        "0x15.0f4afe904c8a71c6a1f42ca2#100",
        Greater,
    );
    test(
        "700000000.00",
        "0x29b92700.0#30",
        100,
        Nearest,
        "21.059738073567624086156312420738",
        "0x15.0f4afe904c8a71c6a1f42ca0#100",
        Less,
    );
    test(
        "700000000.00",
        "0x29b92700.0#30",
        100,
        Down,
        "21.059738073567624086156312420738",
        "0x15.0f4afe904c8a71c6a1f42ca0#100",
        Less,
    );
    test(
        "700000000.00",
        "0x29b92700.0#30",
        100,
        Up,
        "21.059738073567624086156312420763",
        "0x15.0f4afe904c8a71c6a1f42ca2#100",
        Greater,
    );
    test(
        "744261117.50",
        "0x2c5c85fd.8#31",
        10,
        Floor,
        "21.094",
        "0x15.18#10",
        Less,
    );
    test(
        "744261117.50",
        "0x2c5c85fd.8#31",
        10,
        Ceiling,
        "21.125",
        "0x15.20#10",
        Greater,
    );
    test(
        "744261117.50",
        "0x2c5c85fd.8#31",
        10,
        Nearest,
        "21.125",
        "0x15.20#10",
        Greater,
    );
    test(
        "744261117.50",
        "0x2c5c85fd.8#31",
        10,
        Down,
        "21.094",
        "0x15.18#10",
        Less,
    );
    test(
        "744261117.50",
        "0x2c5c85fd.8#31",
        10,
        Up,
        "21.125",
        "0x15.20#10",
        Greater,
    );
    test(
        "744261117.50",
        "0x2c5c85fd.8#31",
        100,
        Floor,
        "21.121049676165439427776329197550",
        "0x15.1efd1c90526b837d1cec05de#100",
        Less,
    );
    test(
        "744261117.50",
        "0x2c5c85fd.8#31",
        100,
        Ceiling,
        "21.121049676165439427776329197575",
        "0x15.1efd1c90526b837d1cec05e0#100",
        Greater,
    );
    test(
        "744261117.50",
        "0x2c5c85fd.8#31",
        100,
        Nearest,
        "21.121049676165439427776329197550",
        "0x15.1efd1c90526b837d1cec05de#100",
        Less,
    );
    test(
        "744261117.50",
        "0x2c5c85fd.8#31",
        100,
        Down,
        "21.121049676165439427776329197550",
        "0x15.1efd1c90526b837d1cec05de#100",
        Less,
    );
    test(
        "744261117.50",
        "0x2c5c85fd.8#31",
        100,
        Up,
        "21.121049676165439427776329197575",
        "0x15.1efd1c90526b837d1cec05e0#100",
        Greater,
    );
    test(
        "-744261117.50",
        "-0x2c5c85fd.8#31",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261117.50",
        "-0x2c5c85fd.8#31",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261117.50",
        "-0x2c5c85fd.8#31",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261117.50",
        "-0x2c5c85fd.8#31",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261117.50",
        "-0x2c5c85fd.8#31",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261117.50",
        "-0x2c5c85fd.8#31",
        100,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261117.50",
        "-0x2c5c85fd.8#31",
        100,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261117.50",
        "-0x2c5c85fd.8#31",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261117.50",
        "-0x2c5c85fd.8#31",
        100,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261117.50",
        "-0x2c5c85fd.8#31",
        100,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "744261117.90039",
        "0x2c5c85fd.e68#40",
        10,
        Floor,
        "21.094",
        "0x15.18#10",
        Less,
    );
    test(
        "744261117.90039",
        "0x2c5c85fd.e68#40",
        10,
        Ceiling,
        "21.125",
        "0x15.20#10",
        Greater,
    );
    test(
        "744261117.90039",
        "0x2c5c85fd.e68#40",
        10,
        Nearest,
        "21.125",
        "0x15.20#10",
        Greater,
    );
    test(
        "744261117.90039",
        "0x2c5c85fd.e68#40",
        10,
        Down,
        "21.094",
        "0x15.18#10",
        Less,
    );
    test(
        "744261117.90039",
        "0x2c5c85fd.e68#40",
        10,
        Up,
        "21.125",
        "0x15.20#10",
        Greater,
    );
    test(
        "744261117.90039",
        "0x2c5c85fd.e68#40",
        100,
        Floor,
        "21.121049676703410061305093114572",
        "0x15.1efd1c92a1ecc900fd4e3f74#100",
        Less,
    );
    test(
        "744261117.90039",
        "0x2c5c85fd.e68#40",
        100,
        Ceiling,
        "21.121049676703410061305093114597",
        "0x15.1efd1c92a1ecc900fd4e3f76#100",
        Greater,
    );
    test(
        "744261117.90039",
        "0x2c5c85fd.e68#40",
        100,
        Nearest,
        "21.121049676703410061305093114572",
        "0x15.1efd1c92a1ecc900fd4e3f74#100",
        Less,
    );
    test(
        "744261117.90039",
        "0x2c5c85fd.e68#40",
        100,
        Down,
        "21.121049676703410061305093114572",
        "0x15.1efd1c92a1ecc900fd4e3f74#100",
        Less,
    );
    test(
        "744261117.90039",
        "0x2c5c85fd.e68#40",
        100,
        Up,
        "21.121049676703410061305093114597",
        "0x15.1efd1c92a1ecc900fd4e3f76#100",
        Greater,
    );
    test(
        "744261118.00",
        "0x2c5c85fe.0#30",
        10,
        Floor,
        "21.094",
        "0x15.18#10",
        Less,
    );
    test(
        "744261118.00",
        "0x2c5c85fe.0#30",
        10,
        Ceiling,
        "21.125",
        "0x15.20#10",
        Greater,
    );
    test(
        "744261118.00",
        "0x2c5c85fe.0#30",
        10,
        Nearest,
        "21.125",
        "0x15.20#10",
        Greater,
    );
    test(
        "744261118.00",
        "0x2c5c85fe.0#30",
        10,
        Down,
        "21.094",
        "0x15.18#10",
        Less,
    );
    test(
        "744261118.00",
        "0x2c5c85fe.0#30",
        10,
        Up,
        "21.125",
        "0x15.20#10",
        Greater,
    );
    test(
        "744261118.00",
        "0x2c5c85fe.0#30",
        100,
        Floor,
        "21.121049676837246657894122087544",
        "0x15.1efd1c9335147025f6ae9514#100",
        Less,
    );
    test(
        "744261118.00",
        "0x2c5c85fe.0#30",
        100,
        Ceiling,
        "21.121049676837246657894122087570",
        "0x15.1efd1c9335147025f6ae9516#100",
        Greater,
    );
    test(
        "744261118.00",
        "0x2c5c85fe.0#30",
        100,
        Nearest,
        "21.121049676837246657894122087544",
        "0x15.1efd1c9335147025f6ae9514#100",
        Less,
    );
    test(
        "744261118.00",
        "0x2c5c85fe.0#30",
        100,
        Down,
        "21.121049676837246657894122087544",
        "0x15.1efd1c9335147025f6ae9514#100",
        Less,
    );
    test(
        "744261118.00",
        "0x2c5c85fe.0#30",
        100,
        Up,
        "21.121049676837246657894122087570",
        "0x15.1efd1c9335147025f6ae9516#100",
        Greater,
    );
    test(
        "-744261118.00",
        "-0x2c5c85fe.0#30",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261118.00",
        "-0x2c5c85fe.0#30",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261118.00",
        "-0x2c5c85fe.0#30",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261118.00",
        "-0x2c5c85fe.0#30",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261118.00",
        "-0x2c5c85fe.0#30",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261118.00",
        "-0x2c5c85fe.0#30",
        100,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261118.00",
        "-0x2c5c85fe.0#30",
        100,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261118.00",
        "-0x2c5c85fe.0#30",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261118.00",
        "-0x2c5c85fe.0#30",
        100,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-744261118.00",
        "-0x2c5c85fe.0#30",
        100,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1000000000.0",
        "0x3b9aca00.0#30",
        10,
        Floor,
        "21.406",
        "0x15.68#10",
        Less,
    );
    test(
        "1000000000.0",
        "0x3b9aca00.0#30",
        10,
        Ceiling,
        "21.438",
        "0x15.70#10",
        Greater,
    );
    test(
        "1000000000.0",
        "0x3b9aca00.0#30",
        10,
        Nearest,
        "21.406",
        "0x15.68#10",
        Less,
    );
    test(
        "1000000000.0",
        "0x3b9aca00.0#30",
        10,
        Down,
        "21.406",
        "0x15.68#10",
        Less,
    );
    test(
        "1000000000.0",
        "0x3b9aca00.0#30",
        10,
        Up,
        "21.438",
        "0x15.70#10",
        Greater,
    );
    test(
        "1000000000.0",
        "0x3b9aca00.0#30",
        100,
        Floor,
        "21.416413017506356465329155213594",
        "0x15.6a9a0b23d187ace021825d40#100",
        Less,
    );
    test(
        "1000000000.0",
        "0x3b9aca00.0#30",
        100,
        Ceiling,
        "21.416413017506356465329155213619",
        "0x15.6a9a0b23d187ace021825d42#100",
        Greater,
    );
    test(
        "1000000000.0",
        "0x3b9aca00.0#30",
        100,
        Nearest,
        "21.416413017506356465329155213619",
        "0x15.6a9a0b23d187ace021825d42#100",
        Greater,
    );
    test(
        "1000000000.0",
        "0x3b9aca00.0#30",
        100,
        Down,
        "21.416413017506356465329155213594",
        "0x15.6a9a0b23d187ace021825d40#100",
        Less,
    );
    test(
        "1000000000.0",
        "0x3b9aca00.0#30",
        100,
        Up,
        "21.416413017506356465329155213619",
        "0x15.6a9a0b23d187ace021825d42#100",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Floor,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Ceiling,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Nearest,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Down,
        "7.4344e8",
        "0x2.c5E+7#10",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Up,
        "7.4449e8",
        "0x2.c6E+7#10",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        100,
        Floor,
        "744261117.95391597822607658233916",
        "0x2c5c85fd.f433d6699ce38ac130#100",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        100,
        Ceiling,
        "744261117.95391597822607658234001",
        "0x2c5c85fd.f433d6699ce38ac134#100",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        100,
        Nearest,
        "744261117.95391597822607658234001",
        "0x2c5c85fd.f433d6699ce38ac134#100",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        100,
        Down,
        "744261117.95391597822607658233916",
        "0x2c5c85fd.f433d6699ce38ac130#100",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        100,
        Up,
        "744261117.95391597822607658234001",
        "0x2c5c85fd.f433d6699ce38ac134#100",
        Greater,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        100,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        100,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        100,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        100,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "744261117.29980",
        "0x2c5c85fd.4cc#40",
        105,
        Floor,
        "21.1210496758964541109034175880539",
        "0x15.1efd1c8f2aaae0b92c369cf92#105",
        Less,
    );
    test(
        "744261117.29980",
        "0x2c5c85fd.4cc#40",
        105,
        Ceiling,
        "21.1210496758964541109034175880547",
        "0x15.1efd1c8f2aaae0b92c369cf93#105",
        Greater,
    );
    test(
        "744261117.29980",
        "0x2c5c85fd.4cc#40",
        105,
        Nearest,
        "21.1210496758964541109034175880539",
        "0x15.1efd1c8f2aaae0b92c369cf92#105",
        Less,
    );
    // - the largest x for which x^2 cannot overflow, so the general path is taken
    test(
        "5.1e161614247",
        "0x4.0E+134217727#1",
        1,
        Floor,
        "2.7e8",
        "0x1.0E+7#1",
        Less,
    );
    test(
        "5.1e161614247",
        "0x4.0E+134217727#1",
        1,
        Ceiling,
        "5.4e8",
        "0x2.0E+7#1",
        Greater,
    );
    test(
        "5.1e161614247",
        "0x4.0E+134217727#1",
        1,
        Nearest,
        "2.7e8",
        "0x1.0E+7#1",
        Less,
    );
    test(
        "5.1e161614247",
        "0x4.0E+134217727#1",
        10,
        Floor,
        "3.7172e8",
        "0x1.628E+7#10",
        Less,
    );
    test(
        "5.1e161614247",
        "0x4.0E+134217727#1",
        10,
        Ceiling,
        "3.7224e8",
        "0x1.630E+7#10",
        Greater,
    );
    test(
        "5.1e161614247",
        "0x4.0E+134217727#1",
        10,
        Nearest,
        "3.7224e8",
        "0x1.630E+7#10",
        Greater,
    );
    test(
        "5.1e161614247",
        "0x4.0E+134217727#1",
        100,
        Floor,
        "372130558.28429932837700628814559",
        "0x162e42fe.48c7d73da76cfcc736#100",
        Less,
    );
    test(
        "5.1e161614247",
        "0x4.0E+134217727#1",
        100,
        Ceiling,
        "372130558.28429932837700628814601",
        "0x162e42fe.48c7d73da76cfcc738#100",
        Greater,
    );
    test(
        "5.1e161614247",
        "0x4.0E+134217727#1",
        100,
        Nearest,
        "372130558.28429932837700628814559",
        "0x162e42fe.48c7d73da76cfcc736#100",
        Less,
    );
    test(
        "1.016e161614248",
        "0x7.fE+134217727#7",
        20,
        Down,
        "3.7213030e8",
        "0x1.62e42E+7#20",
        Less,
    );
    test(
        "1.016e161614248",
        "0x7.fE+134217727#7",
        20,
        Up,
        "3.7213082e8",
        "0x1.62e44E+7#20",
        Greater,
    );
    test(
        "1.016e161614248",
        "0x7.fE+134217727#7",
        20,
        Nearest,
        "3.7213030e8",
        "0x1.62e42E+7#20",
        Less,
    );
    // - x^2 would overflow, so acosh(x) is computed as ln(x) + ln 2; at precision 1 with Nearest
    //   the first iteration cannot round
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        1,
        Floor,
        "2.7e8",
        "0x1.0E+7#1",
        Less,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        1,
        Ceiling,
        "5.4e8",
        "0x2.0E+7#1",
        Greater,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        1,
        Nearest,
        "2.7e8",
        "0x1.0E+7#1",
        Less,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        10,
        Floor,
        "3.7172e8",
        "0x1.628E+7#10",
        Less,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        10,
        Ceiling,
        "3.7224e8",
        "0x1.630E+7#10",
        Greater,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        10,
        Nearest,
        "3.7224e8",
        "0x1.630E+7#10",
        Greater,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        100,
        Floor,
        "372130558.97744650893695159756284",
        "0x162e42fe.fa39ef35793c767300#100",
        Less,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        100,
        Ceiling,
        "372130558.97744650893695159756327",
        "0x162e42fe.fa39ef35793c767302#100",
        Greater,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        100,
        Nearest,
        "372130558.97744650893695159756284",
        "0x162e42fe.fa39ef35793c767300#100",
        Less,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        200,
        Floor,
        "372130558.97744650893695159756294602395972384199011277483256122",
        "0x162e42fe.fa39ef35793c7673007e5ed5e81e6864ce5316c5b14#200",
        Less,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        200,
        Ceiling,
        "372130558.97744650893695159756294602395972384199011277483256155",
        "0x162e42fe.fa39ef35793c7673007e5ed5e81e6864ce5316c5b16#200",
        Greater,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        200,
        Nearest,
        "372130558.97744650893695159756294602395972384199011277483256122",
        "0x162e42fe.fa39ef35793c7673007e5ed5e81e6864ce5316c5b14#200",
        Less,
    );
    test(
        "-1.0e161614248",
        "-0x8.0E+134217727#1",
        64,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0e161614248",
        "-0x8.0E+134217727#1",
        64,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0e161614248",
        "-0x8.0E+134217727#1",
        64,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0e161614248",
        "-0x8.0E+134217727#1",
        64,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.0e161614248",
        "-0x8.0E+134217727#1",
        64,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        30,
        Floor,
        "372130559.50",
        "0x162e42ff.8#30",
        Less,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        30,
        Nearest,
        "372130559.50",
        "0x162e42ff.8#30",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
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
        "4.656612873077392580113e-10",
        "0x2.000000000000000fcE-8#68",
        25,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "4.656612873077392580113e-10",
        "0x2.000000000000000fcE-8#68",
        25,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "4.656612873077392580113e-10",
        "0x2.000000000000000fcE-8#68",
        25,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "4.6938657760620117187500306e-7",
        "0x7.e000000000000000003eE-6#82",
        7,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("1.0", "0x1.0#1", 64, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 64, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 64, Nearest, "0.0", "0x0.0", Equal);
    test(
        "1.0000000000000000000000000000063",
        "0x1.0000000000000000000000008#100",
        1,
        Floor,
        "1.8e-15",
        "0x8.0E-13#1",
        Less,
    );
    test(
        "1.0000000000000000000000000000063",
        "0x1.0000000000000000000000008#100",
        1,
        Ceiling,
        "3.6e-15",
        "0x1.0E-12#1",
        Greater,
    );
    test(
        "1.0000000000000000000000000000063",
        "0x1.0000000000000000000000008#100",
        1,
        Nearest,
        "3.6e-15",
        "0x1.0E-12#1",
        Greater,
    );
    test(
        "1.0000000000000000000000000000063",
        "0x1.0000000000000000000000008#100",
        10,
        Floor,
        "3.5492e-15",
        "0xf.fcE-13#10",
        Less,
    );
    test(
        "1.0000000000000000000000000000063",
        "0x1.0000000000000000000000008#100",
        10,
        Ceiling,
        "3.5527e-15",
        "0x1.000E-12#10",
        Greater,
    );
    test(
        "1.0000000000000000000000000000063",
        "0x1.0000000000000000000000008#100",
        10,
        Nearest,
        "3.5527e-15",
        "0x1.000E-12#10",
        Greater,
    );
    test(
        "1.0000000000000000000000000000063",
        "0x1.0000000000000000000000008#100",
        64,
        Floor,
        "3.55271367880050092916e-15",
        "0xf.fffffffffffffffE-13#64",
        Less,
    );
    test(
        "1.0000000000000000000000000000063",
        "0x1.0000000000000000000000008#100",
        64,
        Ceiling,
        "3.55271367880050092936e-15",
        "0x1.0000000000000000E-12#64",
        Greater,
    );
    test(
        "1.0000000000000000000000000000063",
        "0x1.0000000000000000000000008#100",
        64,
        Nearest,
        "3.55271367880050092936e-15",
        "0x1.0000000000000000E-12#64",
        Greater,
    );
    test(
        "1.0000000000000000000000000000063",
        "0x1.0000000000000000000000008#100",
        100,
        Floor,
        "3.5527136788005009293556213378878e-15",
        "0xf.ffffffffffffffffffffffffE-13#100",
        Less,
    );
    test(
        "1.0000000000000000000000000000063",
        "0x1.0000000000000000000000008#100",
        100,
        Ceiling,
        "3.5527136788005009293556213378906e-15",
        "0x1.0000000000000000000000000E-12#100",
        Greater,
    );
    test(
        "1.0000000000000000000000000000063",
        "0x1.0000000000000000000000008#100",
        100,
        Nearest,
        "3.5527136788005009293556213378878e-15",
        "0xf.ffffffffffffffffffffffffE-13#100",
        Less,
    );
    test(
        "1.00000012",
        "0x1.000002#24",
        1,
        Floor,
        "0.00024",
        "0x0.001#1",
        Less,
    );
    test(
        "1.00000012",
        "0x1.000002#24",
        1,
        Ceiling,
        "0.00049",
        "0x0.002#1",
        Greater,
    );
    test(
        "1.00000012",
        "0x1.000002#24",
        1,
        Nearest,
        "0.00049",
        "0x0.002#1",
        Greater,
    );
    test(
        "1.00000012",
        "0x1.000002#24",
        10,
        Floor,
        "0.00048780",
        "0x0.001ff8#10",
        Less,
    );
    test(
        "1.00000012",
        "0x1.000002#24",
        10,
        Ceiling,
        "0.00048828",
        "0x0.00200#10",
        Greater,
    );
    test(
        "1.00000012",
        "0x1.000002#24",
        10,
        Nearest,
        "0.00048828",
        "0x0.00200#10",
        Greater,
    );
    test(
        "1.00000012",
        "0x1.000002#24",
        64,
        Floor,
        "0.000488281245149361720635",
        "0x0.001ffffffaaaaaad110#64",
        Less,
    );
    test(
        "1.00000012",
        "0x1.000002#24",
        64,
        Ceiling,
        "0.000488281245149361720661",
        "0x0.001ffffffaaaaaad112#64",
        Greater,
    );
    test(
        "1.00000012",
        "0x1.000002#24",
        64,
        Nearest,
        "0.000488281245149361720661",
        "0x0.001ffffffaaaaaad112#64",
        Greater,
    );
    test(
        "1.00000012",
        "0x1.000002#24",
        100,
        Floor,
        "0.00048828124514936172064863881341715",
        "0x0.001ffffffaaaaaad11110fa35a36#100",
        Less,
    );
    test(
        "1.00000012",
        "0x1.000002#24",
        100,
        Ceiling,
        "0.00048828124514936172064863881341754",
        "0x0.001ffffffaaaaaad11110fa35a38#100",
        Greater,
    );
    test(
        "1.00000012",
        "0x1.000002#24",
        100,
        Nearest,
        "0.00048828124514936172064863881341715",
        "0x0.001ffffffaaaaaad11110fa35a36#100",
        Less,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        1,
        Floor,
        "1.5e-8",
        "0x4.0E-7#1",
        Less,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        1,
        Ceiling,
        "3.0e-8",
        "0x8.0E-7#1",
        Greater,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        1,
        Nearest,
        "1.5e-8",
        "0x4.0E-7#1",
        Less,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        10,
        Floor,
        "2.1071e-8",
        "0x5.a8E-7#10",
        Less,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        10,
        Ceiling,
        "2.1100e-8",
        "0x5.aaE-7#10",
        Greater,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        10,
        Nearest,
        "2.1071e-8",
        "0x5.a8E-7#10",
        Less,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        64,
        Floor,
        "2.10734242554470155019e-8",
        "0x5.a827999fcef31c90E-7#64",
        Less,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        64,
        Ceiling,
        "2.10734242554470155036e-8",
        "0x5.a827999fcef31c98E-7#64",
        Greater,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        64,
        Nearest,
        "2.10734242554470155036e-8",
        "0x5.a827999fcef31c98E-7#64",
        Greater,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        100,
        Floor,
        "2.1073424255447015503547803751825e-8",
        "0x5.a827999fcef31c97ec74cddd0E-7#100",
        Less,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        100,
        Ceiling,
        "2.1073424255447015503547803751848e-8",
        "0x5.a827999fcef31c97ec74cddd8E-7#100",
        Greater,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        100,
        Nearest,
        "2.1073424255447015503547803751825e-8",
        "0x5.a827999fcef31c97ec74cddd0E-7#100",
        Less,
    );
    test("2.0", "0x2.0#1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("2.0", "0x2.0#1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("2.0", "0x2.0#1", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("2.0", "0x2.0#1", 10, Floor, "1.3164", "0x1.510#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "1.3184",
        "0x1.518#10",
        Greater,
    );
    test("2.0", "0x2.0#1", 10, Nearest, "1.3164", "0x1.510#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        64,
        Floor,
        "1.31695789692481670860",
        "0x1.51242719804349be#64",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        64,
        Ceiling,
        "1.31695789692481670871",
        "0x1.51242719804349c0#64",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        64,
        Nearest,
        "1.31695789692481670860",
        "0x1.51242719804349be#64",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        100,
        Floor,
        "1.3169578969248167086250463473073",
        "0x1.51242719804349be684bd0188#100",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        100,
        Ceiling,
        "1.3169578969248167086250463473089",
        "0x1.51242719804349be684bd018a#100",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        100,
        Nearest,
        "1.3169578969248167086250463473073",
        "0x1.51242719804349be684bd0188#100",
        Less,
    );
    test("10.0", "0xa.0#4", 1, Floor, "2.0", "0x2.0#1", Less);
    test("10.0", "0xa.0#4", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("10.0", "0xa.0#4", 1, Nearest, "2.0", "0x2.0#1", Less);
    test(
        "10.0",
        "0xa.0#4",
        64,
        Floor,
        "2.99322284612638089786",
        "0x2.fe43da39c0ba87f8#64",
        Less,
    );
    test(
        "10.0",
        "0xa.0#4",
        64,
        Ceiling,
        "2.99322284612638089807",
        "0x2.fe43da39c0ba87fc#64",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#4",
        64,
        Nearest,
        "2.99322284612638089786",
        "0x2.fe43da39c0ba87f8#64",
        Less,
    );
    // - the first error estimate leaves no bits to test, so the loop retries
    test("1.5", "0x1.8#2", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1.5", "0x1.8#2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1.5", "0x1.8#2", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1.5", "0x1.8#2", 10, Floor, "0.96191", "0x0.f64#10", Less);
    test(
        "1.5",
        "0x1.8#2",
        10,
        Ceiling,
        "0.96289",
        "0x0.f68#10",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        10,
        Nearest,
        "0.96289",
        "0x0.f68#10",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        64,
        Floor,
        "0.962423650119206894945",
        "0x0.f661657628b04ca5#64",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        64,
        Ceiling,
        "0.962423650119206894999",
        "0x0.f661657628b04ca6#64",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        64,
        Nearest,
        "0.962423650119206894999",
        "0x0.f661657628b04ca6#64",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        100,
        Floor,
        "0.96242365011920689499551782684852",
        "0x0.f661657628b04ca5f0210254b#100",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        100,
        Ceiling,
        "0.96242365011920689499551782684931",
        "0x0.f661657628b04ca5f0210254c#100",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        100,
        Nearest,
        "0.96242365011920689499551782684852",
        "0x0.f661657628b04ca5f0210254b#100",
        Less,
    );
    test("-2.0", "-0x2.0#1", 1, Floor, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 1, Ceiling, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 1, Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Floor, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 64, Floor, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 64, Ceiling, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 64, Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 100, Floor, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 100, Ceiling, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 100, Nearest, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 1, Floor, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 1, Ceiling, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 1, Nearest, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 64, Floor, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 64, Ceiling, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 64, Nearest, "NaN", "NaN", Equal);
    test(
        "5.1e161614247",
        "0x4.0E+134217727#1",
        64,
        Floor,
        "372130558.284299328370",
        "0x162e42fe.48c7d73da#64",
        Less,
    );
    test(
        "5.1e161614247",
        "0x4.0E+134217727#1",
        64,
        Ceiling,
        "372130558.284299328399",
        "0x162e42fe.48c7d73dc#64",
        Greater,
    );
    test(
        "5.1e161614247",
        "0x4.0E+134217727#1",
        64,
        Nearest,
        "372130558.284299328370",
        "0x162e42fe.48c7d73da#64",
        Less,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        64,
        Floor,
        "372130558.977446508914",
        "0x162e42fe.fa39ef356#64",
        Less,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        64,
        Ceiling,
        "372130558.977446508943",
        "0x162e42fe.fa39ef358#64",
        Greater,
    );
    test(
        "1.0e161614248",
        "0x8.0E+134217727#1",
        64,
        Nearest,
        "372130558.977446508943",
        "0x162e42fe.fa39ef358#64",
        Greater,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        1,
        Floor,
        "2.7e8",
        "0x1.0E+7#1",
        Less,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        1,
        Ceiling,
        "5.4e8",
        "0x2.0E+7#1",
        Greater,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        1,
        Nearest,
        "2.7e8",
        "0x1.0E+7#1",
        Less,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        10,
        Floor,
        "3.7172e8",
        "0x1.628E+7#10",
        Less,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        10,
        Ceiling,
        "3.7224e8",
        "0x1.630E+7#10",
        Greater,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        10,
        Nearest,
        "3.7224e8",
        "0x1.630E+7#10",
        Greater,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        64,
        Floor,
        "372130559.666679790156",
        "0x162e42ff.aaab86d7a#64",
        Less,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        64,
        Ceiling,
        "372130559.666679790185",
        "0x162e42ff.aaab86d7c#64",
        Greater,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        64,
        Nearest,
        "372130559.666679790185",
        "0x162e42ff.aaab86d7c#64",
        Greater,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        100,
        Floor,
        "372130559.66667979017576057788769",
        "0x162e42ff.aaab86d7b5833cc6e4#100",
        Less,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        100,
        Ceiling,
        "372130559.66667979017576057788812",
        "0x162e42ff.aaab86d7b5833cc6e6#100",
        Greater,
    );
    test(
        "2.041e161614248",
        "0xf.fE+134217727#8",
        100,
        Nearest,
        "372130559.66667979017576057788769",
        "0x162e42ff.aaab86d7b5833cc6e4#100",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        1,
        Floor,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        1,
        Ceiling,
        "1.1e9",
        "0x4.0E+7#1",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        1,
        Nearest,
        "5.4e8",
        "0x2.0E+7#1",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        64,
        Floor,
        "744261117.953915978200",
        "0x2c5c85fd.f433d6698#64",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        64,
        Ceiling,
        "744261117.953915978258",
        "0x2c5c85fd.f433d669c#64",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        64,
        Nearest,
        "744261117.953915978200",
        "0x2c5c85fd.f433d6698#64",
        Less,
    );
    // - x is so close to 1 that x^2 - 1 rounds to zero, so acosh(x) is computed as sqrt(2(x - 1))
    test(
        "1.0000000000000000000000000000000000000000000000000000000013",
        "0x1.000000000000000000000000000000000000000000000008#190",
        1,
        Floor,
        "2.5e-29",
        "0x2.0E-24#1",
        Less,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000013",
        "0x1.000000000000000000000000000000000000000000000008#190",
        1,
        Ceiling,
        "5.0e-29",
        "0x4.0E-24#1",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000013",
        "0x1.000000000000000000000000000000000000000000000008#190",
        1,
        Nearest,
        "5.0e-29",
        "0x4.0E-24#1",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000013",
        "0x1.000000000000000000000000000000000000000000000008#190",
        10,
        Floor,
        "5.0438e-29",
        "0x3.ffE-24#10",
        Less,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000013",
        "0x1.000000000000000000000000000000000000000000000008#190",
        10,
        Ceiling,
        "5.0487e-29",
        "0x4.00E-24#10",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000013",
        "0x1.000000000000000000000000000000000000000000000008#190",
        10,
        Nearest,
        "5.0487e-29",
        "0x4.00E-24#10",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000013",
        "0x1.000000000000000000000000000000000000000000000008#190",
        64,
        Floor,
        "5.04870979341447555436e-29",
        "0x3.fffffffffffffffcE-24#64",
        Less,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000013",
        "0x1.000000000000000000000000000000000000000000000008#190",
        64,
        Ceiling,
        "5.04870979341447555464e-29",
        "0x4.0000000000000000E-24#64",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000013",
        "0x1.000000000000000000000000000000000000000000000008#190",
        64,
        Nearest,
        "5.04870979341447555464e-29",
        "0x4.0000000000000000E-24#64",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000013",
        "0x1.000000000000000000000000000000000000000000000008#190",
        100,
        Floor,
        "5.0487097934144755546350628178058e-29",
        "0x3.ffffffffffffffffffffffffcE-24#100",
        Less,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000013",
        "0x1.000000000000000000000000000000000000000000000008#190",
        100,
        Ceiling,
        "5.0487097934144755546350628178098e-29",
        "0x4.0000000000000000000000000E-24#100",
        Greater,
    );
    test(
        "1.0000000000000000000000000000000000000000000000000000000013",
        "0x1.000000000000000000000000000000000000000000000008#190",
        100,
        Nearest,
        "5.0487097934144755546350628178098e-29",
        "0x4.0000000000000000000000000E-24#100",
        Greater,
    );
}

#[test]
#[should_panic]
fn acosh_prec_round_fail() {
    Float::ONE.acosh_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn acosh_prec_round_exact_fail() {
    Float::TWO.acosh_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acosh_prec_fail() {
    Float::ONE.acosh_prec(0);
}

#[test]
#[should_panic]
fn acosh_round_fail() {
    Float::TWO.acosh_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn acosh_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (c, o) = x.clone().acosh_prec_round(prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.acosh_prec_round_ref(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.acosh_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_acosh_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // acosh is NaN for NaN, -inf, and every x below 1, and otherwise nonnegative
    if x.is_nan() || x < 1u32 {
        assert!(c.is_nan());
    } else {
        assert!(c.is_sign_positive());
    }
    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o == Equal {
        // acosh is exact only for x = 1 (and for the inputs whose result is NaN or inf): the result
        // is rounding-mode-invariant
        for rm2 in exhaustive_rounding_modes() {
            let (c2, o2) = x.acosh_prec_round_ref(prec, rm2);
            assert_eq!(ComparableFloat(c2), ComparableFloat(c.clone()));
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.acosh_prec_round_ref(prec, Exact));
    }
}

#[test]
fn acosh_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        acosh_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (c, o) = Float::NAN.acosh_prec_round(prec, rm);
        assert!(c.is_nan());
        assert_eq!(o, Equal);

        let (c, o) = Float::INFINITY.acosh_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);

        for x in [Float::NEGATIVE_INFINITY, Float::ZERO, Float::NEGATIVE_ZERO] {
            let (c, o) = x.acosh_prec_round(prec, rm);
            assert!(c.is_nan());
            assert_eq!(o, Equal);
        }

        let (c, o) = Float::ONE.acosh_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[test]
fn acosh_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (c, o) = x.clone().acosh_round(rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.acosh_round_ref(rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.acosh_round_assign(rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.acosh_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acosh_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn acosh_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (c, o) = x.clone().acosh_prec(prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.acosh_prec_ref(prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.acosh_prec_assign(prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.acosh_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (rug_c, rug_o) = rug_acosh_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn acosh_properties() {
    float_gen().test_properties(|x| {
        let c = x.clone().acosh();
        assert!(c.is_valid());
        let c_alt = (&x).acosh();
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

        let mut x_alt = x.clone();
        x_alt.acosh_assign();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));

        let c_alt = x.acosh_prec_round_ref(x.significant_bits(), Nearest).0;
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

        assert_eq!(
            ComparableFloatRef(&<Float as From<&rug::Float>>::from(&rug_acosh(
                &rug::Float::exact_from(&x)
            ))),
            ComparableFloatRef(&c)
        );

        assert_eq!(c.is_nan(), x.is_nan() || x < 1u32);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_acosh() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_acosh(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, f32::NAN);
    test::<f32>(-0.0, f32::NAN);
    test::<f32>(0.5, f32::NAN);
    test::<f32>(1.0, 0.0);
    test::<f32>(1.0000001, 0.00048828125);
    test::<f32>(2.0, 1.316958);
    test::<f32>(10.0, 2.993223);
    test::<f32>(core::f32::consts::PI, 1.8115263);
    test::<f32>(88.0, 5.1704516);
    test::<f32>(10000000000.0, 23.718998);
    test::<f32>(1.0e30, 69.7707);
    test::<f32>(3.4028235e38, 89.415985);
    test::<f32>(-1.0, f32::NAN);

    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::INFINITY);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, f64::NAN);
    test::<f64>(-0.0, f64::NAN);
    test::<f64>(0.5, f64::NAN);
    test::<f64>(1.0, 0.0);
    test::<f64>(1.0000000000000002, 2.1073424255447014e-8);
    test::<f64>(2.0, 1.3169578969248168);
    test::<f64>(10.0, 2.993222846126381);
    test::<f64>(core::f64::consts::PI, 1.811526272460853);
    test::<f64>(709.0, 7.257002209758403);
    test::<f64>(1.0e100, 230.95165647996453);
    test::<f64>(1.0e300, 691.4686750787737);
    test::<f64>(1.7976931348623157e308, 710.475860073944);
    test::<f64>(-1.0, f64::NAN);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acosh_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let c = primitive_float_acosh(x);
        // acosh is NaN for NaN, -inf, and every x below 1, and otherwise nonnegative and below x
        assert_eq!(c.is_nan(), x.is_nan() || x < T::ONE);
        if !c.is_nan() {
            assert!(c >= T::ZERO);
            assert!(c < x || x == T::INFINITY);
        }
        if x.is_finite() && x >= T::ONE {
            // the result is the correctly rounded inverse hyperbolic cosine, as computed by MPFR at
            // the same precision; it never overflows or underflows
            let rug_x = rug::Float::with_val(
                u32::exact_from(T::MANTISSA_WIDTH + 1),
                &rug::Float::exact_from(&Float::from(x)),
            );
            let rug_c = <Float as From<&rug::Float>>::from(&rug_x.acosh());
            let expected = T::exact_from(&rug_c);
            assert_eq!(NiceFloat(expected), NiceFloat(c));
        }
    });
}

#[test]
fn primitive_float_acosh_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acosh_properties_helper);
}

#[test]
fn test_acosh_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::acosh_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        let (c, o) = Float::acosh_rational_prec_round_ref(&x, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acosh_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 1, Floor, "NaN", "NaN", Equal);
    test("0", 1, Ceiling, "NaN", "NaN", Equal);
    test("0", 1, Down, "NaN", "NaN", Equal);
    test("0", 1, Up, "NaN", "NaN", Equal);
    test("0", 1, Nearest, "NaN", "NaN", Equal);
    test("0", 1, Exact, "NaN", "NaN", Equal);
    test("0", 10, Floor, "NaN", "NaN", Equal);
    test("0", 10, Ceiling, "NaN", "NaN", Equal);
    test("0", 10, Down, "NaN", "NaN", Equal);
    test("0", 10, Up, "NaN", "NaN", Equal);
    test("0", 10, Nearest, "NaN", "NaN", Equal);
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1/2", 1, Floor, "NaN", "NaN", Equal);
    test("1/2", 1, Ceiling, "NaN", "NaN", Equal);
    test("1/2", 1, Down, "NaN", "NaN", Equal);
    test("1/2", 1, Up, "NaN", "NaN", Equal);
    test("1/2", 1, Nearest, "NaN", "NaN", Equal);
    test("1/2", 1, Exact, "NaN", "NaN", Equal);
    test("1/2", 10, Floor, "NaN", "NaN", Equal);
    test("1/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("1/2", 10, Down, "NaN", "NaN", Equal);
    test("1/2", 10, Up, "NaN", "NaN", Equal);
    test("1/2", 10, Nearest, "NaN", "NaN", Equal);
    test("1/2", 10, Exact, "NaN", "NaN", Equal);
    test("-1", 1, Floor, "NaN", "NaN", Equal);
    test("-1", 1, Ceiling, "NaN", "NaN", Equal);
    test("-1", 1, Down, "NaN", "NaN", Equal);
    test("-1", 1, Up, "NaN", "NaN", Equal);
    test("-1", 1, Nearest, "NaN", "NaN", Equal);
    test("-1", 1, Exact, "NaN", "NaN", Equal);
    test("-1", 10, Floor, "NaN", "NaN", Equal);
    test("-1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1", 10, Down, "NaN", "NaN", Equal);
    test("-1", 10, Up, "NaN", "NaN", Equal);
    test("-1", 10, Nearest, "NaN", "NaN", Equal);
    test("-1", 10, Exact, "NaN", "NaN", Equal);
    test("-22/7", 1, Floor, "NaN", "NaN", Equal);
    test("-22/7", 1, Ceiling, "NaN", "NaN", Equal);
    test("-22/7", 1, Down, "NaN", "NaN", Equal);
    test("-22/7", 1, Up, "NaN", "NaN", Equal);
    test("-22/7", 1, Nearest, "NaN", "NaN", Equal);
    test("-22/7", 1, Exact, "NaN", "NaN", Equal);
    test("-22/7", 10, Floor, "NaN", "NaN", Equal);
    test("-22/7", 10, Ceiling, "NaN", "NaN", Equal);
    test("-22/7", 10, Down, "NaN", "NaN", Equal);
    test("-22/7", 10, Up, "NaN", "NaN", Equal);
    test("-22/7", 10, Nearest, "NaN", "NaN", Equal);
    test("-22/7", 10, Exact, "NaN", "NaN", Equal);
    test("1", 1, Floor, "0.0", "0x0.0", Equal);
    test("1", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("1", 1, Down, "0.0", "0x0.0", Equal);
    test("1", 1, Up, "0.0", "0x0.0", Equal);
    test("1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1", 1, Exact, "0.0", "0x0.0", Equal);
    test("1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1", 10, Down, "0.0", "0x0.0", Equal);
    test("1", 10, Up, "0.0", "0x0.0", Equal);
    test("1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1", 10, Exact, "0.0", "0x0.0", Equal);
    test("3/2", 1, Floor, "0.50", "0x0.8#1", Less);
    test("3/2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("3/2", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("3/2", 10, Floor, "0.96191", "0x0.f64#10", Less);
    test("3/2", 10, Ceiling, "0.96289", "0x0.f68#10", Greater);
    test("3/2", 10, Nearest, "0.96289", "0x0.f68#10", Greater);
    test(
        "3/2",
        100,
        Floor,
        "0.96242365011920689499551782684852",
        "0x0.f661657628b04ca5f0210254b#100",
        Less,
    );
    test(
        "3/2",
        100,
        Ceiling,
        "0.96242365011920689499551782684931",
        "0x0.f661657628b04ca5f0210254c#100",
        Greater,
    );
    test(
        "3/2",
        100,
        Nearest,
        "0.96242365011920689499551782684852",
        "0x0.f661657628b04ca5f0210254b#100",
        Less,
    );
    test("2", 1, Floor, "1.0", "0x1.0#1", Less);
    test("2", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("2", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("2", 10, Floor, "1.3164", "0x1.510#10", Less);
    test("2", 10, Ceiling, "1.3184", "0x1.518#10", Greater);
    test("2", 10, Nearest, "1.3164", "0x1.510#10", Less);
    test(
        "2",
        100,
        Floor,
        "1.3169578969248167086250463473073",
        "0x1.51242719804349be684bd0188#100",
        Less,
    );
    test(
        "2",
        100,
        Ceiling,
        "1.3169578969248167086250463473089",
        "0x1.51242719804349be684bd018a#100",
        Greater,
    );
    test(
        "2",
        100,
        Nearest,
        "1.3169578969248167086250463473073",
        "0x1.51242719804349be684bd0188#100",
        Less,
    );
    test("22/7", 1, Floor, "1.0", "0x1.0#1", Less);
    test("22/7", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("22/7", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("22/7", 10, Floor, "1.8105", "0x1.cf8#10", Less);
    test("22/7", 10, Ceiling, "1.8125", "0x1.d00#10", Greater);
    test("22/7", 10, Nearest, "1.8125", "0x1.d00#10", Greater);
    test(
        "22/7",
        100,
        Floor,
        "1.8119507608214135562359728131092",
        "0x1.cfdc014bb0b6cd086e591b3d6#100",
        Less,
    );
    test(
        "22/7",
        100,
        Ceiling,
        "1.8119507608214135562359728131107",
        "0x1.cfdc014bb0b6cd086e591b3d8#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "1.8119507608214135562359728131092",
        "0x1.cfdc014bb0b6cd086e591b3d6#100",
        Less,
    );
    test("10", 1, Floor, "2.0", "0x2.0#1", Less);
    test("10", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("10", 1, Nearest, "2.0", "0x2.0#1", Less);
    test("10", 10, Floor, "2.9922", "0x2.fe#10", Less);
    test("10", 10, Ceiling, "2.9961", "0x2.ff#10", Greater);
    test("10", 10, Nearest, "2.9922", "0x2.fe#10", Less);
    test(
        "10",
        100,
        Floor,
        "2.9932228461263808979126677137722",
        "0x2.fe43da39c0ba87f9072bef0f4#100",
        Less,
    );
    test(
        "10",
        100,
        Ceiling,
        "2.9932228461263808979126677137753",
        "0x2.fe43da39c0ba87f9072bef0f8#100",
        Greater,
    );
    test(
        "10",
        100,
        Nearest,
        "2.9932228461263808979126677137753",
        "0x2.fe43da39c0ba87f9072bef0f8#100",
        Greater,
    );
    test("100", 1, Floor, "4.0", "0x4.0#1", Less);
    test("100", 1, Ceiling, "8.0", "0x8.0#1", Greater);
    test("100", 1, Nearest, "4.0", "0x4.0#1", Less);
    test("100", 10, Floor, "5.2969", "0x5.4c#10", Less);
    test("100", 10, Ceiling, "5.3047", "0x5.4e#10", Greater);
    test("100", 10, Nearest, "5.2969", "0x5.4c#10", Less);
    test(
        "100",
        100,
        Floor,
        "5.2982923656104845907016668349390",
        "0x5.4c5ce372f189a2883e416c220#100",
        Less,
    );
    test(
        "100",
        100,
        Ceiling,
        "5.2982923656104845907016668349453",
        "0x5.4c5ce372f189a2883e416c228#100",
        Greater,
    );
    test(
        "100",
        100,
        Nearest,
        "5.2982923656104845907016668349453",
        "0x5.4c5ce372f189a2883e416c228#100",
        Greater,
    );
    test("10000", 1, Floor, "8.0", "0x8.0#1", Less);
    test("10000", 1, Ceiling, "16.0", "0x1.0E+1#1", Greater);
    test("10000", 1, Nearest, "8.0", "0x8.0#1", Less);
    test("10000", 10, Floor, "9.8906", "0x9.e4#10", Less);
    test("10000", 10, Ceiling, "9.9062", "0x9.e8#10", Greater);
    test("10000", 10, Nearest, "9.9062", "0x9.e8#10", Greater);
    test(
        "10000",
        100,
        Floor,
        "9.9034875500361280361141978881043",
        "0x9.e74af5c7bf92c9553904874e#100",
        Less,
    );
    test(
        "10000",
        100,
        Ceiling,
        "9.9034875500361280361141978881169",
        "0x9.e74af5c7bf92c9553904874f#100",
        Greater,
    );
    test(
        "10000",
        100,
        Nearest,
        "9.9034875500361280361141978881169",
        "0x9.e74af5c7bf92c9553904874f#100",
        Greater,
    );
    test(
        "1000000000000000000000000",
        1,
        Floor,
        "32.0",
        "0x2.0E+1#1",
        Less,
    );
    test(
        "1000000000000000000000000",
        1,
        Ceiling,
        "64.0",
        "0x4.0E+1#1",
        Greater,
    );
    test(
        "1000000000000000000000000",
        1,
        Nearest,
        "64.0",
        "0x4.0E+1#1",
        Greater,
    );
    test(
        "1000000000000000000000000",
        10,
        Floor,
        "55.938",
        "0x37.f#10",
        Less,
    );
    test(
        "1000000000000000000000000",
        10,
        Ceiling,
        "56.000",
        "0x38.0#10",
        Greater,
    );
    test(
        "1000000000000000000000000",
        10,
        Nearest,
        "55.938",
        "0x37.f#10",
        Less,
    );
    test(
        "1000000000000000000000000",
        100,
        Floor,
        "55.955189412417041725849027033869",
        "0x37.f4874b17d1100243aa7403e8#100",
        Less,
    );
    test(
        "1000000000000000000000000",
        100,
        Ceiling,
        "55.955189412417041725849027033920",
        "0x37.f4874b17d1100243aa7403ec#100",
        Greater,
    );
    test(
        "1000000000000000000000000",
        100,
        Nearest,
        "55.955189412417041725849027033869",
        "0x37.f4874b17d1100243aa7403e8#100",
        Less,
    );
    test("1000001/1000000", 1, Floor, "0.00098", "0x0.004#1", Less);
    test(
        "1000001/1000000",
        1,
        Ceiling,
        "0.0020",
        "0x0.008#1",
        Greater,
    );
    test("1000001/1000000", 1, Nearest, "0.00098", "0x0.004#1", Less);
    test(
        "1000001/1000000",
        10,
        Floor,
        "0.0014133",
        "0x0.005ca#10",
        Less,
    );
    test(
        "1000001/1000000",
        10,
        Ceiling,
        "0.0014153",
        "0x0.005cc#10",
        Greater,
    );
    test(
        "1000001/1000000",
        10,
        Nearest,
        "0.0014133",
        "0x0.005ca#10",
        Less,
    );
    test(
        "1000001/1000000",
        100,
        Floor,
        "0.0014142134445219913675401706721988",
        "0x0.005cae907e68b96003235c5e8d60#100",
        Less,
    );
    test(
        "1000001/1000000",
        100,
        Ceiling,
        "0.0014142134445219913675401706722003",
        "0x0.005cae907e68b96003235c5e8d68#100",
        Greater,
    );
    test(
        "1000001/1000000",
        100,
        Nearest,
        "0.0014142134445219913675401706721988",
        "0x0.005cae907e68b96003235c5e8d60#100",
        Less,
    );
    test(
        "1073741825/1073741824",
        1,
        Floor,
        "0.000031",
        "0x0.0002#1",
        Less,
    );
    test(
        "1073741825/1073741824",
        1,
        Ceiling,
        "0.000061",
        "0x0.0004#1",
        Greater,
    );
    test(
        "1073741825/1073741824",
        1,
        Nearest,
        "0.000031",
        "0x0.0002#1",
        Less,
    );
    test(
        "1073741825/1073741824",
        10,
        Floor,
        "0.000043154",
        "0x0.0002d4#10",
        Less,
    );
    test(
        "1073741825/1073741824",
        10,
        Ceiling,
        "0.000043213",
        "0x0.0002d5#10",
        Greater,
    );
    test(
        "1073741825/1073741824",
        10,
        Nearest,
        "0.000043154",
        "0x0.0002d4#10",
        Less,
    );
    test(
        "1073741825/1073741824",
        100,
        Floor,
        "0.000043158372871805957972032722503902",
        "0x0.0002d413cccef61da322475c19f44#100",
        Less,
    );
    test(
        "1073741825/1073741824",
        100,
        Ceiling,
        "0.000043158372871805957972032722503950",
        "0x0.0002d413cccef61da322475c19f48#100",
        Greater,
    );
    test(
        "1073741825/1073741824",
        100,
        Nearest,
        "0.000043158372871805957972032722503902",
        "0x0.0002d413cccef61da322475c19f44#100",
        Less,
    );
    test(
        "1606938044258990275541962092341162602522202993782792835301377/160693804425899027554196209\
        2341162602522202993782792835301376",
        1,
        Floor,
        "7.9e-31",
        "0x1.0E-25#1",
        Less,
    );
    test(
        "1606938044258990275541962092341162602522202993782792835301377/160693804425899027554196209\
        2341162602522202993782792835301376",
        1,
        Ceiling,
        "1.6e-30",
        "0x2.0E-25#1",
        Greater,
    );
    test(
        "1606938044258990275541962092341162602522202993782792835301377/160693804425899027554196209\
        2341162602522202993782792835301376",
        1,
        Nearest,
        "7.9e-31",
        "0x1.0E-25#1",
        Less,
    );
    test(
        "1606938044258990275541962092341162602522202993782792835301377/160693804425899027554196209\
        2341162602522202993782792835301376",
        10,
        Floor,
        "1.1155e-30",
        "0x1.6a0E-25#10",
        Less,
    );
    test(
        "1606938044258990275541962092341162602522202993782792835301377/160693804425899027554196209\
        2341162602522202993782792835301376",
        10,
        Ceiling,
        "1.1170e-30",
        "0x1.6a8E-25#10",
        Greater,
    );
    test(
        "1606938044258990275541962092341162602522202993782792835301377/160693804425899027554196209\
        2341162602522202993782792835301376",
        10,
        Nearest,
        "1.1155e-30",
        "0x1.6a0E-25#10",
        Less,
    );
    test(
        "1606938044258990275541962092341162602522202993782792835301377/160693804425899027554196209\
        2341162602522202993782792835301376",
        100,
        Floor,
        "1.1156177909894716005065492737195e-30",
        "0x1.6a09e667f3bcc908b2fb1366eE-25#100",
        Less,
    );
    test(
        "1606938044258990275541962092341162602522202993782792835301377/160693804425899027554196209\
        2341162602522202993782792835301376",
        100,
        Ceiling,
        "1.1156177909894716005065492737207e-30",
        "0x1.6a09e667f3bcc908b2fb13670E-25#100",
        Greater,
    );
    test(
        "1606938044258990275541962092341162602522202993782792835301377/160693804425899027554196209\
        2341162602522202993782792835301376",
        100,
        Nearest,
        "1.1156177909894716005065492737195e-30",
        "0x1.6a09e667f3bcc908b2fb1366eE-25#100",
        Less,
    );
    test(
        "369988485035126972924700782451696644186473100389722973815184405301748250/3699884850351269\
        72924700782451696644186473100389722973815184405301748249",
        1,
        Floor,
        "1.5e-36",
        "0x2.0E-30#1",
        Less,
    );
    test(
        "369988485035126972924700782451696644186473100389722973815184405301748250/3699884850351269\
        72924700782451696644186473100389722973815184405301748249",
        1,
        Ceiling,
        "3.0e-36",
        "0x4.0E-30#1",
        Greater,
    );
    test(
        "369988485035126972924700782451696644186473100389722973815184405301748250/3699884850351269\
        72924700782451696644186473100389722973815184405301748249",
        1,
        Nearest,
        "3.0e-36",
        "0x4.0E-30#1",
        Greater,
    );
    test(
        "369988485035126972924700782451696644186473100389722973815184405301748250/3699884850351269\
        72924700782451696644186473100389722973815184405301748249",
        10,
        Floor,
        "2.3245e-36",
        "0x3.17E-30#10",
        Less,
    );
    test(
        "369988485035126972924700782451696644186473100389722973815184405301748250/3699884850351269\
        72924700782451696644186473100389722973815184405301748249",
        10,
        Ceiling,
        "2.3275e-36",
        "0x3.18E-30#10",
        Greater,
    );
    test(
        "369988485035126972924700782451696644186473100389722973815184405301748250/3699884850351269\
        72924700782451696644186473100389722973815184405301748249",
        10,
        Nearest,
        "2.3245e-36",
        "0x3.17E-30#10",
        Less,
    );
    test(
        "369988485035126972924700782451696644186473100389722973815184405301748250/3699884850351269\
        72924700782451696644186473100389722973815184405301748249",
        100,
        Floor,
        "2.3249889537607554019192985393201e-36",
        "0x3.17271a3f499b6175703ba4f94E-30#100",
        Less,
    );
    test(
        "369988485035126972924700782451696644186473100389722973815184405301748250/3699884850351269\
        72924700782451696644186473100389722973815184405301748249",
        100,
        Ceiling,
        "2.3249889537607554019192985393225e-36",
        "0x3.17271a3f499b6175703ba4f98E-30#100",
        Greater,
    );
    test(
        "369988485035126972924700782451696644186473100389722973815184405301748250/3699884850351269\
        72924700782451696644186473100389722973815184405301748249",
        100,
        Nearest,
        "2.3249889537607554019192985393201e-36",
        "0x3.17271a3f499b6175703ba4f94E-30#100",
        Less,
    );
    // - x is close enough to 1 to try the bracket sqrt(2t - t^2/3) < acosh(1 + t) < sqrt(2t), and
    //   sqrt(2t) = 1/16 is exactly representable, so the upper end is rounded as a number just
    //   below it
    test("513/512", 2, Floor, "0.047", "0x0.0c#2", Less);
    test("513/512", 2, Up, "0.062", "0x0.10#2", Greater);
    test("513/512", 2, Nearest, "0.062", "0x0.10#2", Greater);
    // - x is close enough to 1 to try the bracket, but its ends round differently with Up, so x is
    //   bracketed between Floats instead
    test("8255/8191", 2, Floor, "0.094", "0x0.18#2", Less);
    test("8255/8191", 2, Up, "0.12", "0x0.2#2", Greater);
    test("8255/8191", 2, Nearest, "0.12", "0x0.2#2", Greater);
}

#[test]
fn test_acosh_rational_extreme() {
    // x = 2^(2^30) is too large to be a `Float`; acosh(x) = ln(2x) + c with -2^(-2^31) < c < 0, far
    // below an ulp of the result, so the result is the rounding of ln(2^(2^30 + 1))
    let x = Rational::power_of_2(1i64 << 30);
    let two_x = Rational::power_of_2((1i64 << 30) + 1);
    for rm in [Floor, Ceiling, Down, Up, Nearest] {
        let (c, o) = Float::acosh_rational_prec_round_ref(&x, 100, rm);
        let (l, o_l) = Float::ln_rational_prec_round_ref(&two_x, 100, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(l));
        assert_eq!(o, o_l);
    }

    // x = 1 + t with t = 2^(-2^20); acosh(x) = sqrt(2t) (1 - c) with 0 < c < t/12, far below an ulp
    // of the result, and sqrt(2t) = sqrt(2) 2^(-2^19) is irrational, so the result is the rounding
    // of sqrt(2t)
    let t = Rational::power_of_2(-(1i64 << 20));
    let x = Rational::ONE + &t;
    let two_t = t << 1u32;
    for prec in [10, 100] {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (c, o) = Float::acosh_rational_prec_round_ref(&x, prec, rm);
            let (r, o_r) = Float::sqrt_rational_prec_round_ref(&two_t, prec, rm);
            assert_eq!(ComparableFloatRef(&c), ComparableFloatRef(&r));
            assert_eq!(o, o_r);
            if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
                let (rug_c, rug_o) = rug_acosh_rational_prec_round(&x, prec, rug_rm);
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_c)),
                    ComparableFloatRef(&c)
                );
                assert_eq!(rug_o, o);
            }
        }
    }
}

#[test]
#[should_panic]
fn acosh_rational_prec_fail() {
    Float::acosh_rational_prec(Rational::TWO, 0);
}

#[test]
#[should_panic]
fn acosh_rational_prec_ref_fail() {
    Float::acosh_rational_prec_ref(&Rational::TWO, 0);
}

#[test]
#[should_panic]
fn acosh_rational_prec_round_fail_1() {
    Float::acosh_rational_prec_round(Rational::TWO, 0, Floor);
}

#[test]
#[should_panic]
fn acosh_rational_prec_round_fail_2() {
    Float::acosh_rational_prec_round(Rational::TWO, 10, Exact);
}

#[test]
#[should_panic]
fn acosh_rational_prec_round_ref_fail() {
    Float::acosh_rational_prec_round_ref(&Rational::TWO, 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn acosh_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (c, o) = Float::acosh_rational_prec_round(x.clone(), prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = Float::acosh_rational_prec_round_ref(&x, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // acosh is NaN below 1 and non-negative from 1 on
    if x < 1u32 {
        assert!(c.is_nan());
    } else {
        assert!(c >= 0u32);
    }

    // acosh(x) < ln(2x) for x >= 1
    if x >= 1u32 && rm != Exact {
        let (l, _) = Float::ln_rational_prec_round(x.clone() << 1u32, prec, Ceiling);
        let (c_floor, _) = Float::acosh_rational_prec_round_ref(&x, prec, Floor);
        assert!(c_floor <= l);
    }

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_acosh_rational_prec_round(&x, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::acosh_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::acosh_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn acosh_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_15().test_properties(|(x, prec, rm)| {
        acosh_rational_prec_round_properties_helper(x, prec, rm);
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        if x < 1u32 {
            for rm in exhaustive_rounding_modes() {
                acosh_rational_prec_round_properties_helper(x.clone(), prec, rm);
            }
        }
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (c, o) = Float::acosh_rational_prec_round(Rational::ONE, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn acosh_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (c, o) = Float::acosh_rational_prec(x.clone(), prec);
    assert!(c.is_valid());

    let (c_alt, o_alt) = Float::acosh_rational_prec_ref(&x, prec);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let (c_alt, o_alt) = Float::acosh_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let (rug_c, rug_o) = rug_acosh_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_c)),
        ComparableFloatRef(&c)
    );
    assert_eq!(rug_o, o);

    // the inverse hyperbolic cosine of an exactly representable rational is the Float inverse
    // hyperbolic cosine
    if let Ok(f) = Float::try_from(&x) {
        let (c_alt, o_alt) = f.acosh_prec(prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }
}

#[test]
fn acosh_rational_prec_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_15().test_properties(|(x, prec, _)| {
        acosh_rational_prec_properties_helper(x, prec);
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        acosh_rational_prec_properties_helper(x, prec);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_acosh_rational() {
    fn test<T: PrimitiveFloat>(x: &Rational, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_acosh_rational::<T>(x)),
            NiceFloat(out)
        );
    }
    let test_s = |s: &str| Rational::from_str(s).unwrap();
    test::<f32>(&test_s("0"), f32::NAN);
    test::<f64>(&test_s("0"), f64::NAN);
    test::<f32>(&test_s("1/2"), f32::NAN);
    test::<f64>(&test_s("1/2"), f64::NAN);
    test::<f32>(&test_s("-1"), f32::NAN);
    test::<f64>(&test_s("-1"), f64::NAN);
    test::<f32>(&test_s("1"), 0.0);
    test::<f64>(&test_s("1"), 0.0);
    test::<f32>(&test_s("3/2"), 0.9624236);
    test::<f64>(&test_s("3/2"), 0.9624236501192069);
    test::<f32>(&test_s("2"), 1.316958);
    test::<f64>(&test_s("2"), 1.3169578969248168);
    test::<f32>(&test_s("22/7"), 1.8119508);
    test::<f64>(&test_s("22/7"), 1.8119507608214136);
    test::<f32>(&test_s("100"), 5.298292);
    test::<f64>(&test_s("100"), 5.298292365610485);
    test::<f32>(&test_s("10000"), 9.903487);
    test::<f64>(&test_s("10000"), 9.903487550036129);
    test::<f32>(
        &(Rational::ONE + Rational::power_of_2(-30i64)),
        0.000043158372,
    );
    test::<f64>(
        &(Rational::ONE + Rational::power_of_2(-30i64)),
        0.00004315837287180596,
    );
    test::<f32>(
        &(Rational::ONE + Rational::power_of_2(-100i64)),
        1.2560739e-15,
    );
    test::<f64>(
        &(Rational::ONE + Rational::power_of_2(-100i64)),
        1.2560739669470201e-15,
    );
    // results that are subnormal or underflow
    test::<f32>(&(Rational::ONE + Rational::power_of_2(-2000i64)), 0.0);
    test::<f64>(
        &(Rational::ONE + Rational::power_of_2(-2000i64)),
        1.3198340665566424e-301,
    );
    test::<f32>(&(Rational::ONE + Rational::power_of_2(-2100i64)), 0.0);
    test::<f64>(
        &(Rational::ONE + Rational::power_of_2(-2100i64)),
        1.1722481e-316,
    );
    test::<f32>(&(Rational::ONE + Rational::power_of_2(-2200i64)), 0.0);
    test::<f64>(&(Rational::ONE + Rational::power_of_2(-2200i64)), 0.0);
    test::<f32>(&(Rational::ONE + Rational::power_of_2(-260i64)), 1.039e-39);
    test::<f64>(
        &(Rational::ONE + Rational::power_of_2(-260i64)),
        1.0390000333911476e-39,
    );
    test::<f32>(&(Rational::ONE + Rational::power_of_2(-300i64)), 1.0e-45);
    test::<f64>(
        &(Rational::ONE + Rational::power_of_2(-300i64)),
        9.908676465903736e-46,
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acosh_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let c = primitive_float_acosh_rational::<T>(&x);
        // the inverse hyperbolic cosine of a rational is NaN exactly when the rational is below 1
        assert_eq!(c.is_nan(), x < 1u32);
        if x >= 1u32 {
            // the result is the correctly rounded inverse hyperbolic cosine, as computed by MPFR
            // with 64 bits to spare, so that a subnormal result is rounded once by the conversion
            // rather than twice
            let rug_c: T = round_once_to_primitive(|p| {
                <Float as From<&rug::Float>>::from(&rug_acosh_rational_prec(&x, p).0)
            });
            assert_eq!(NiceFloat(rug_c), NiceFloat(c));
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // The inverse hyperbolic cosine of a finite primitive float, taken through the `Rational`
        // path, matches the direct primitive-float inverse hyperbolic cosine (a `Rational` cannot
        // carry the sign of a zero, but the result is NaN either way).
        if x.is_finite() {
            assert_eq!(
                NiceFloat(primitive_float_acosh_rational::<T>(&Rational::exact_from(
                    x
                ))),
                NiceFloat(primitive_float_acosh(x))
            );
        }
    });
}

#[test]
fn primitive_float_acosh_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acosh_rational_properties_helper);
}
