// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Abs, Atanh, AtanhAssign, PowerOf2};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, OneHalf, Two, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::atanh::{
    primitive_float_atanh, primitive_float_atanh_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::atanh::{
    rug_atanh, rug_atanh_prec, rug_atanh_prec_round, rug_atanh_rational_prec,
    rug_atanh_rational_prec_round, rug_atanh_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_16,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_atanh_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().atanh_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.atanh_prec_round_ref(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.atanh_prec_round_assign(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_atanh_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
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
    test("Infinity", "Infinity", 1, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Ceiling, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Ceiling, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Ceiling, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Floor, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test("-0.0", "-0x0.0", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Floor, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", 1, Ceiling, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", 10, Floor, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "Infinity", "Infinity", Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("2.0", "0x2.0#1", 1, Floor, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 1, Ceiling, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 1, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Floor, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 1, Floor, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 1, Ceiling, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 1, Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Floor, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 1, Floor, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 1, Ceiling, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 1, Nearest, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 10, Floor, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 10, Ceiling, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 10, Nearest, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 1, Floor, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 1, Ceiling, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 1, Nearest, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Floor, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-1.5", "-0x1.8#2", 10, Nearest, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 1, Floor, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("0.50", "0x0.8#1", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("0.50", "0x0.8#1", 10, Floor, "0.54883", "0x0.8c8#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "0.54980",
        "0x0.8cc#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        10,
        Nearest,
        "0.54883",
        "0x0.8c8#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        100,
        Floor,
        "0.54930614433405484569762261846113",
        "0x0.8c9f53d5681854bb520cc6aa8#100",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        100,
        Ceiling,
        "0.54930614433405484569762261846192",
        "0x0.8c9f53d5681854bb520cc6aa9#100",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        100,
        Nearest,
        "0.54930614433405484569762261846113",
        "0x0.8c9f53d5681854bb520cc6aa8#100",
        Less,
    );
    test("-0.50", "-0x0.8#1", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test(
        "-0.50", "-0x0.8#1", 1, Ceiling, "-0.50", "-0x0.8#1", Greater,
    );
    test(
        "-0.50", "-0x0.8#1", 1, Nearest, "-0.50", "-0x0.8#1", Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Floor,
        "-0.54980",
        "-0x0.8cc#10",
        Less,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Ceiling,
        "-0.54883",
        "-0x0.8c8#10",
        Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Nearest,
        "-0.54883",
        "-0x0.8c8#10",
        Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        100,
        Floor,
        "-0.54930614433405484569762261846192",
        "-0x0.8c9f53d5681854bb520cc6aa9#100",
        Less,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        100,
        Ceiling,
        "-0.54930614433405484569762261846113",
        "-0x0.8c9f53d5681854bb520cc6aa8#100",
        Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        100,
        Nearest,
        "-0.54930614433405484569762261846113",
        "-0x0.8c9f53d5681854bb520cc6aa8#100",
        Greater,
    );
    test("0.75", "0x0.c#2", 1, Floor, "0.50", "0x0.8#1", Less);
    test("0.75", "0x0.c#2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("0.75", "0x0.c#2", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("0.75", "0x0.c#2", 10, Floor, "0.97266", "0x0.f90#10", Less);
    test(
        "0.75",
        "0x0.c#2",
        10,
        Ceiling,
        "0.97363",
        "0x0.f94#10",
        Greater,
    );
    test(
        "0.75",
        "0x0.c#2",
        10,
        Nearest,
        "0.97266",
        "0x0.f90#10",
        Less,
    );
    test(
        "0.75",
        "0x0.c#2",
        100,
        Floor,
        "0.97295507452765665255267637172144",
        "0x0.f913957192d2baa37b4a4b679#100",
        Less,
    );
    test(
        "0.75",
        "0x0.c#2",
        100,
        Ceiling,
        "0.97295507452765665255267637172223",
        "0x0.f913957192d2baa37b4a4b67a#100",
        Greater,
    );
    test(
        "0.75",
        "0x0.c#2",
        100,
        Nearest,
        "0.97295507452765665255267637172144",
        "0x0.f913957192d2baa37b4a4b679#100",
        Less,
    );
    test("-0.75", "-0x0.c#2", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test(
        "-0.75", "-0x0.c#2", 1, Ceiling, "-0.50", "-0x0.8#1", Greater,
    );
    test("-0.75", "-0x0.c#2", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test(
        "-0.75",
        "-0x0.c#2",
        10,
        Floor,
        "-0.97363",
        "-0x0.f94#10",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        10,
        Ceiling,
        "-0.97266",
        "-0x0.f90#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        10,
        Nearest,
        "-0.97266",
        "-0x0.f90#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        100,
        Floor,
        "-0.97295507452765665255267637172223",
        "-0x0.f913957192d2baa37b4a4b67a#100",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        100,
        Ceiling,
        "-0.97295507452765665255267637172144",
        "-0x0.f913957192d2baa37b4a4b679#100",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        100,
        Nearest,
        "-0.97295507452765665255267637172144",
        "-0x0.f913957192d2baa37b4a4b679#100",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        1,
        Floor,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        1,
        Ceiling,
        "0.50",
        "0x0.8#1",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        1,
        Nearest,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Floor,
        "0.34619",
        "0x0.58a#10",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Ceiling,
        "0.34668",
        "0x0.58c#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        10,
        Nearest,
        "0.34668",
        "0x0.58c#10",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Floor,
        "0.34657359027997265470861606072899",
        "0x0.58b90bfbe8e7bcd5e4f1d9cc00#100",
        Less,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Ceiling,
        "0.34657359027997265470861606072939",
        "0x0.58b90bfbe8e7bcd5e4f1d9cc08#100",
        Greater,
    );
    test(
        "0.33333333333333333333333333333346",
        "0x0.55555555555555555555555558#100",
        100,
        Nearest,
        "0.34657359027997265470861606072939",
        "0x0.58b90bfbe8e7bcd5e4f1d9cc08#100",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        1,
        Floor,
        "-0.50",
        "-0x0.8#1",
        Less,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        1,
        Ceiling,
        "-0.25",
        "-0x0.4#1",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        1,
        Nearest,
        "-0.25",
        "-0x0.4#1",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        10,
        Floor,
        "-0.34668",
        "-0x0.58c#10",
        Less,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        10,
        Ceiling,
        "-0.34619",
        "-0x0.58a#10",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        10,
        Nearest,
        "-0.34668",
        "-0x0.58c#10",
        Less,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        100,
        Floor,
        "-0.34657359027997265470861606072939",
        "-0x0.58b90bfbe8e7bcd5e4f1d9cc08#100",
        Less,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        100,
        Ceiling,
        "-0.34657359027997265470861606072899",
        "-0x0.58b90bfbe8e7bcd5e4f1d9cc00#100",
        Greater,
    );
    test(
        "-0.33333333333333333333333333333346",
        "-0x0.55555555555555555555555558#100",
        100,
        Nearest,
        "-0.34657359027997265470861606072939",
        "-0x0.58b90bfbe8e7bcd5e4f1d9cc08#100",
        Less,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        1,
        Floor,
        "0.062",
        "0x0.1#1",
        Less,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        1,
        Ceiling,
        "0.12",
        "0x0.2#1",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        1,
        Nearest,
        "0.12",
        "0x0.2#1",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        10,
        Floor,
        "0.10022",
        "0x0.19a8#10",
        Less,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        10,
        Ceiling,
        "0.10034",
        "0x0.19b0#10",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        10,
        Nearest,
        "0.10034",
        "0x0.19b0#10",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        100,
        Floor,
        "0.10033534773107558063572655205996",
        "0x0.19af93cd2344120521b26f7578#100",
        Less,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        100,
        Ceiling,
        "0.10033534773107558063572655206006",
        "0x0.19af93cd2344120521b26f757a#100",
        Greater,
    );
    test(
        "0.10000000000000000000000000000002",
        "0x0.1999999999999999999999999a#100",
        100,
        Nearest,
        "0.10033534773107558063572655206006",
        "0x0.19af93cd2344120521b26f757a#100",
        Greater,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        1,
        Floor,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        10,
        Floor,
        "2.6445",
        "0x2.a5#10",
        Less,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        10,
        Ceiling,
        "2.6484",
        "0x2.a6#10",
        Greater,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        10,
        Nearest,
        "2.6484",
        "0x2.a6#10",
        Greater,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        100,
        Floor,
        "2.6466524123622461977050606459247",
        "0x2.a58b0332f9b0c4c0aa1f8b888#100",
        Less,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        100,
        Ceiling,
        "2.6466524123622461977050606459278",
        "0x2.a58b0332f9b0c4c0aa1f8b88c#100",
        Greater,
    );
    test(
        "0.98999999999999999999999999999981",
        "0x0.fd70a3d70a3d70a3d70a3d70a#100",
        100,
        Nearest,
        "2.6466524123622461977050606459247",
        "0x2.a58b0332f9b0c4c0aa1f8b888#100",
        Less,
    );
    test(
        "-0.98999999999999999999999999999981",
        "-0x0.fd70a3d70a3d70a3d70a3d70a#100",
        1,
        Floor,
        "-4.0",
        "-0x4.0#1",
        Less,
    );
    test(
        "-0.98999999999999999999999999999981",
        "-0x0.fd70a3d70a3d70a3d70a3d70a#100",
        1,
        Ceiling,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );
    test(
        "-0.98999999999999999999999999999981",
        "-0x0.fd70a3d70a3d70a3d70a3d70a#100",
        1,
        Nearest,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );
    test(
        "-0.98999999999999999999999999999981",
        "-0x0.fd70a3d70a3d70a3d70a3d70a#100",
        10,
        Floor,
        "-2.6484",
        "-0x2.a6#10",
        Less,
    );
    test(
        "-0.98999999999999999999999999999981",
        "-0x0.fd70a3d70a3d70a3d70a3d70a#100",
        10,
        Ceiling,
        "-2.6445",
        "-0x2.a5#10",
        Greater,
    );
    test(
        "-0.98999999999999999999999999999981",
        "-0x0.fd70a3d70a3d70a3d70a3d70a#100",
        10,
        Nearest,
        "-2.6484",
        "-0x2.a6#10",
        Less,
    );
    test(
        "-0.98999999999999999999999999999981",
        "-0x0.fd70a3d70a3d70a3d70a3d70a#100",
        100,
        Floor,
        "-2.6466524123622461977050606459278",
        "-0x2.a58b0332f9b0c4c0aa1f8b88c#100",
        Less,
    );
    test(
        "-0.98999999999999999999999999999981",
        "-0x0.fd70a3d70a3d70a3d70a3d70a#100",
        100,
        Ceiling,
        "-2.6466524123622461977050606459247",
        "-0x2.a58b0332f9b0c4c0aa1f8b888#100",
        Greater,
    );
    test(
        "-0.98999999999999999999999999999981",
        "-0x0.fd70a3d70a3d70a3d70a3d70a#100",
        100,
        Nearest,
        "-2.6466524123622461977050606459247",
        "-0x2.a58b0332f9b0c4c0aa1f8b888#100",
        Greater,
    );
    test(
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        1,
        Floor,
        "-64.0",
        "-0x4.0E+1#1",
        Less,
    );
    test(
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        1,
        Ceiling,
        "-32.0",
        "-0x2.0E+1#1",
        Greater,
    );
    test(
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        1,
        Nearest,
        "-32.0",
        "-0x2.0E+1#1",
        Greater,
    );
    test(
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        10,
        Floor,
        "-35.062",
        "-0x23.1#10",
        Less,
    );
    test(
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        10,
        Ceiling,
        "-35.000",
        "-0x23.0#10",
        Greater,
    );
    test(
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        10,
        Nearest,
        "-35.000",
        "-0x23.0#10",
        Greater,
    );
    test(
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        100,
        Floor,
        "-35.003932618277238125570222133679",
        "-0x23.0101ba62e36d8063536aed80#100",
        Less,
    );
    test(
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        100,
        Ceiling,
        "-35.003932618277238125570222133628",
        "-0x23.0101ba62e36d8063536aed7c#100",
        Greater,
    );
    test(
        "-0.99999999999999999999999999999921",
        "-0x0.fffffffffffffffffffffffff#100",
        100,
        Nearest,
        "-35.003932618277238125570222133628",
        "-0x23.0101ba62e36d8063536aed7c#100",
        Greater,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        1,
        Floor,
        "32.0",
        "0x2.0E+1#1",
        Less,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        1,
        Ceiling,
        "64.0",
        "0x4.0E+1#1",
        Greater,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        1,
        Nearest,
        "32.0",
        "0x2.0E+1#1",
        Less,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        10,
        Floor,
        "35.000",
        "0x23.0#10",
        Less,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        10,
        Ceiling,
        "35.062",
        "0x23.1#10",
        Greater,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        10,
        Nearest,
        "35.000",
        "0x23.0#10",
        Less,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        100,
        Floor,
        "35.003932618277238125570222133628",
        "0x23.0101ba62e36d8063536aed7c#100",
        Less,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        100,
        Ceiling,
        "35.003932618277238125570222133679",
        "0x23.0101ba62e36d8063536aed80#100",
        Greater,
    );
    test(
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        100,
        Nearest,
        "35.003932618277238125570222133628",
        "0x23.0101ba62e36d8063536aed7c#100",
        Less,
    );
    test(
        "5.0862630208333333333333333333353e-6",
        "0x0.000055555555555555555555555558#100",
        1,
        Floor,
        "3.8e-6",
        "0x0.00004#1",
        Less,
    );
    test(
        "5.0862630208333333333333333333353e-6",
        "0x0.000055555555555555555555555558#100",
        1,
        Ceiling,
        "7.6e-6",
        "0x0.00008#1",
        Greater,
    );
    test(
        "5.0862630208333333333333333333353e-6",
        "0x0.000055555555555555555555555558#100",
        1,
        Nearest,
        "3.8e-6",
        "0x0.00004#1",
        Less,
    );
    test(
        "5.0862630208333333333333333333353e-6",
        "0x0.000055555555555555555555555558#100",
        10,
        Floor,
        "5.0813e-6",
        "0x0.0000554#10",
        Less,
    );
    test(
        "5.0862630208333333333333333333353e-6",
        "0x0.000055555555555555555555555558#100",
        10,
        Ceiling,
        "5.0887e-6",
        "0x0.0000556#10",
        Greater,
    );
    test(
        "5.0862630208333333333333333333353e-6",
        "0x0.000055555555555555555555555558#100",
        10,
        Nearest,
        "5.0887e-6",
        "0x0.0000556#10",
        Greater,
    );
    test(
        "5.0862630208333333333333333333353e-6",
        "0x0.000055555555555555555555555558#100",
        100,
        Floor,
        "5.0862630208771939960352548926335e-6",
        "0x0.000055555555587e6b74f06881ca78#100",
        Less,
    );
    test(
        "5.0862630208333333333333333333353e-6",
        "0x0.000055555555555555555555555558#100",
        100,
        Ceiling,
        "5.0862630208771939960352548926395e-6",
        "0x0.000055555555587e6b74f06881ca80#100",
        Greater,
    );
    test(
        "5.0862630208333333333333333333353e-6",
        "0x0.000055555555555555555555555558#100",
        100,
        Nearest,
        "5.0862630208771939960352548926395e-6",
        "0x0.000055555555587e6b74f06881ca80#100",
        Greater,
    );
    test(
        "1.5239831883763666429543780644077e-100",
        "0x1.5555555555555555555555556E-83#100",
        1,
        Floor,
        "1.1e-100",
        "0x1.0E-83#1",
        Less,
    );
    test(
        "1.5239831883763666429543780644077e-100",
        "0x1.5555555555555555555555556E-83#100",
        1,
        Ceiling,
        "2.3e-100",
        "0x2.0E-83#1",
        Greater,
    );
    test(
        "1.5239831883763666429543780644077e-100",
        "0x1.5555555555555555555555556E-83#100",
        1,
        Nearest,
        "1.1e-100",
        "0x1.0E-83#1",
        Less,
    );
    test(
        "1.5239831883763666429543780644077e-100",
        "0x1.5555555555555555555555556E-83#100",
        10,
        Floor,
        "1.5225e-100",
        "0x1.550E-83#10",
        Less,
    );
    test(
        "1.5239831883763666429543780644077e-100",
        "0x1.5555555555555555555555556E-83#100",
        10,
        Ceiling,
        "1.5247e-100",
        "0x1.558E-83#10",
        Greater,
    );
    test(
        "1.5239831883763666429543780644077e-100",
        "0x1.5555555555555555555555556E-83#100",
        10,
        Nearest,
        "1.5247e-100",
        "0x1.558E-83#10",
        Greater,
    );
    test(
        "1.5239831883763666429543780644077e-100",
        "0x1.5555555555555555555555556E-83#100",
        100,
        Floor,
        "1.5239831883763666429543780644077e-100",
        "0x1.5555555555555555555555556E-83#100",
        Less,
    );
    test(
        "1.5239831883763666429543780644077e-100",
        "0x1.5555555555555555555555556E-83#100",
        100,
        Ceiling,
        "1.5239831883763666429543780644095e-100",
        "0x1.5555555555555555555555558E-83#100",
        Greater,
    );
    test(
        "1.5239831883763666429543780644077e-100",
        "0x1.5555555555555555555555556E-83#100",
        100,
        Nearest,
        "1.5239831883763666429543780644077e-100",
        "0x1.5555555555555555555555556E-83#100",
        Less,
    );
    test(
        "0.00098",
        "0x0.004#1",
        1,
        Floor,
        "0.00098",
        "0x0.004#1",
        Less,
    );
    test(
        "0.00098",
        "0x0.004#1",
        1,
        Ceiling,
        "0.0020",
        "0x0.008#1",
        Greater,
    );
    test(
        "0.00098",
        "0x0.004#1",
        1,
        Nearest,
        "0.00098",
        "0x0.004#1",
        Less,
    );
    test(
        "0.00098",
        "0x0.004#1",
        10,
        Floor,
        "0.00097656",
        "0x0.00400#10",
        Less,
    );
    test(
        "0.00098",
        "0x0.004#1",
        10,
        Ceiling,
        "0.00097847",
        "0x0.00402#10",
        Greater,
    );
    test(
        "0.00098",
        "0x0.004#1",
        10,
        Nearest,
        "0.00097656",
        "0x0.00400#10",
        Less,
    );
    test(
        "0.00098",
        "0x0.004#1",
        100,
        Floor,
        "0.00097656281044103584096445002988428",
        "0x0.004000015555622222b46b4dd0d8#100",
        Less,
    );
    test(
        "0.00098",
        "0x0.004#1",
        100,
        Ceiling,
        "0.00097656281044103584096445002988582",
        "0x0.004000015555622222b46b4dd0e0#100",
        Greater,
    );
    test(
        "0.00098",
        "0x0.004#1",
        100,
        Nearest,
        "0.00097656281044103584096445002988582",
        "0x0.004000015555622222b46b4dd0e0#100",
        Greater,
    );
    // - EXP(x) <= -1 - prec / (1 + ceil(log2(prec))), so the series is used, and its first error
    //   estimate does not allow rounding, so the loop retries
    test("0.12", "0x0.2#1", 4, Up, "0.141", "0x0.24#4", Greater);
    test("0.12", "0x0.2#1", 8, Up, "0.1260", "0x0.204#8", Greater);
    // - the general path's first approximation does not allow rounding, so the loop retries
    test("0.25", "0x0.4#1", 2, Up, "0.38", "0x0.6#2", Greater);
    test("0.50", "0x0.8#1", 1, Up, "1.0", "0x1.0#1", Greater);
}

#[test]
#[should_panic]
fn atanh_prec_round_fail() {
    Float::ONE_HALF.atanh_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn atanh_prec_round_exact_fail() {
    Float::ONE_HALF.atanh_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn atanh_prec_fail() {
    Float::ONE_HALF.atanh_prec(0);
}

#[test]
#[should_panic]
fn atanh_round_fail() {
    Float::ONE_HALF.atanh_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn atanh_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (c, o) = x.clone().atanh_prec_round(prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.atanh_prec_round_ref(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.atanh_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_atanh_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // atanh is odd
    let (c_neg, o_neg) = (-&x).atanh_prec_round(prec, -rm);
    assert_eq!(ComparableFloat(c_neg), ComparableFloat(-&c));
    assert_eq!(o_neg, o.reverse());

    // atanh is NaN for NaN, infinities, and every x with |x| > 1, infinite at +/-1, and otherwise
    // has the sign of x and at least its magnitude
    let abs_x = (&x).abs();
    if x.is_nan() || abs_x > 1u32 {
        assert!(c.is_nan());
    } else if abs_x == 1u32 {
        assert!(c.is_infinite());
        assert_eq!(c.is_sign_positive(), x.is_sign_positive());
    } else {
        assert_eq!(c.is_sign_positive(), x.is_sign_positive());
        let rm_up = if x.is_sign_positive() { Ceiling } else { Floor };
        let (c_up, _) = x.atanh_prec_round_ref(prec, rm_up);
        let (x_down, _) = Float::from_float_prec_round_ref(&x, prec, -rm_up);
        assert!(c_up.abs() >= x_down.abs());
    }
    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o == Equal {
        // atanh is exact only for x = +/-0 and +/-1 (and for the inputs whose result is NaN): the
        // result is rounding-mode-invariant
        for rm2 in exhaustive_rounding_modes() {
            let (c2, o2) = x.atanh_prec_round_ref(prec, rm2);
            assert_eq!(ComparableFloat(c2), ComparableFloat(c.clone()));
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.atanh_prec_round_ref(prec, Exact));
    }
}

#[test]
fn atanh_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        atanh_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        for x in [Float::NAN, Float::INFINITY, Float::NEGATIVE_INFINITY, Float::TWO] {
            let (c, o) = x.atanh_prec_round(prec, rm);
            assert!(c.is_nan());
            assert_eq!(o, Equal);
        }
        for (x, out) in [
            (Float::ZERO, Float::ZERO),
            (Float::NEGATIVE_ZERO, Float::NEGATIVE_ZERO),
            (Float::ONE, Float::INFINITY),
            (-Float::ONE, Float::NEGATIVE_INFINITY),
        ] {
            let (c, o) = x.atanh_prec_round(prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(out));
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn atanh_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (c, o) = x.clone().atanh_round(rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.atanh_round_ref(rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.atanh_round_assign(rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.atanh_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_atanh_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn atanh_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (c, o) = x.clone().atanh_prec(prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.atanh_prec_ref(prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.atanh_prec_assign(prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.atanh_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (rug_c, rug_o) = rug_atanh_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn atanh_properties_helper(x: Float) {
    let c = x.clone().atanh();
    assert!(c.is_valid());
    let c_alt = (&x).atanh();
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

    let mut x_alt = x.clone();
    x_alt.atanh_assign();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));

    let c_alt = x.atanh_prec_round_ref(x.significant_bits(), Nearest).0;
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_atanh(&rug::Float::exact_from(&x)))),
        ComparableFloatRef(&c)
    );

    assert_eq!(
        c.is_nan(),
        x.is_nan() || x.is_infinite() || (&x).abs() > 1u32
    );
}

#[test]
fn atanh_properties() {
    float_gen().test_properties(|x| {
        atanh_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        atanh_properties_helper(x);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_atanh() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_atanh(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, 0.0);
    test::<f32>(-0.0, -0.0);
    test::<f32>(1.0, f32::INFINITY);
    test::<f32>(-1.0, f32::NEGATIVE_INFINITY);
    test::<f32>(2.0, f32::NAN);
    test::<f32>(0.5, 0.54930615);
    test::<f32>(-0.5, -0.54930615);
    test::<f32>(0.1, 0.10033535);
    test::<f32>(0.99999994, 8.66434);
    test::<f32>(1.0e-10, 1.0e-10);
    test::<f32>(1.0e-45, 1.0e-45);
    test::<f32>(1.0e-40, 1.0e-40);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, 0.0);
    test::<f64>(-0.0, -0.0);
    test::<f64>(1.0, f64::INFINITY);
    test::<f64>(-1.0, f64::NEGATIVE_INFINITY);
    test::<f64>(2.0, f64::NAN);
    test::<f64>(0.5, 0.5493061443340549);
    test::<f64>(-0.5, -0.5493061443340549);
    test::<f64>(0.1, 0.10033534773107558);
    test::<f64>(0.9999999999999999, 18.714973875118524);
    test::<f64>(1.0e-100, 1.0e-100);
    test::<f64>(5.0e-324, 5.0e-324);
    test::<f64>(1.0e-310, 1.0e-310);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_atanh_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let c = primitive_float_atanh(x);
        // atanh is NaN for NaN, infinities, and every x with |x| > 1, and odd
        assert_eq!(c.is_nan(), x.is_nan() || x.abs() > T::ONE);
        assert_eq!(NiceFloat(primitive_float_atanh(-x)), NiceFloat(-c));
        if x.is_finite() && x.abs() < T::ONE {
            // the result is the correctly rounded inverse hyperbolic tangent, as computed by MPFR
            // with 64 bits to spare, so that a subnormal result is rounded once by the conversion
            // rather than twice
            let rug_c = rug_atanh_prec(
                &rug::Float::exact_from(&Float::from(x)),
                T::MANTISSA_WIDTH + 64,
            )
            .0;
            let rug_c: T = T::rounding_from(&<Float as From<&rug::Float>>::from(&rug_c), Nearest).0;
            assert_eq!(NiceFloat(rug_c), NiceFloat(c));
        }
    });
}

#[test]
fn primitive_float_atanh_properties() {
    apply_fn_to_primitive_floats!(primitive_float_atanh_properties_helper);
}

#[test]
fn test_atanh_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::atanh_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        let (c, o) = Float::atanh_rational_prec_round_ref(&x, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_atanh_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 1, Down, "0.0", "0x0.0", Equal);
    test("0", 1, Up, "0.0", "0x0.0", Equal);
    test("0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", 1, Exact, "0.0", "0x0.0", Equal);
    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Down, "0.0", "0x0.0", Equal);
    test("0", 10, Up, "0.0", "0x0.0", Equal);
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 1, Floor, "Infinity", "Infinity", Equal);
    test("1", 1, Ceiling, "Infinity", "Infinity", Equal);
    test("1", 1, Down, "Infinity", "Infinity", Equal);
    test("1", 1, Up, "Infinity", "Infinity", Equal);
    test("1", 1, Nearest, "Infinity", "Infinity", Equal);
    test("1", 1, Exact, "Infinity", "Infinity", Equal);
    test("1", 10, Floor, "Infinity", "Infinity", Equal);
    test("1", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("1", 10, Down, "Infinity", "Infinity", Equal);
    test("1", 10, Up, "Infinity", "Infinity", Equal);
    test("1", 10, Nearest, "Infinity", "Infinity", Equal);
    test("1", 10, Exact, "Infinity", "Infinity", Equal);
    test("-1", 1, Floor, "-Infinity", "-Infinity", Equal);
    test("-1", 1, Ceiling, "-Infinity", "-Infinity", Equal);
    test("-1", 1, Down, "-Infinity", "-Infinity", Equal);
    test("-1", 1, Up, "-Infinity", "-Infinity", Equal);
    test("-1", 1, Nearest, "-Infinity", "-Infinity", Equal);
    test("-1", 1, Exact, "-Infinity", "-Infinity", Equal);
    test("-1", 10, Floor, "-Infinity", "-Infinity", Equal);
    test("-1", 10, Ceiling, "-Infinity", "-Infinity", Equal);
    test("-1", 10, Down, "-Infinity", "-Infinity", Equal);
    test("-1", 10, Up, "-Infinity", "-Infinity", Equal);
    test("-1", 10, Nearest, "-Infinity", "-Infinity", Equal);
    test("-1", 10, Exact, "-Infinity", "-Infinity", Equal);
    test("2", 1, Floor, "NaN", "NaN", Equal);
    test("2", 1, Ceiling, "NaN", "NaN", Equal);
    test("2", 1, Down, "NaN", "NaN", Equal);
    test("2", 1, Up, "NaN", "NaN", Equal);
    test("2", 1, Nearest, "NaN", "NaN", Equal);
    test("2", 1, Exact, "NaN", "NaN", Equal);
    test("2", 10, Floor, "NaN", "NaN", Equal);
    test("2", 10, Ceiling, "NaN", "NaN", Equal);
    test("2", 10, Down, "NaN", "NaN", Equal);
    test("2", 10, Up, "NaN", "NaN", Equal);
    test("2", 10, Nearest, "NaN", "NaN", Equal);
    test("2", 10, Exact, "NaN", "NaN", Equal);
    test("-3/2", 1, Floor, "NaN", "NaN", Equal);
    test("-3/2", 1, Ceiling, "NaN", "NaN", Equal);
    test("-3/2", 1, Down, "NaN", "NaN", Equal);
    test("-3/2", 1, Up, "NaN", "NaN", Equal);
    test("-3/2", 1, Nearest, "NaN", "NaN", Equal);
    test("-3/2", 1, Exact, "NaN", "NaN", Equal);
    test("-3/2", 10, Floor, "NaN", "NaN", Equal);
    test("-3/2", 10, Ceiling, "NaN", "NaN", Equal);
    test("-3/2", 10, Down, "NaN", "NaN", Equal);
    test("-3/2", 10, Up, "NaN", "NaN", Equal);
    test("-3/2", 10, Nearest, "NaN", "NaN", Equal);
    test("-3/2", 10, Exact, "NaN", "NaN", Equal);
    test("1/2", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1/2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1/2", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("1/2", 10, Floor, "0.54883", "0x0.8c8#10", Less);
    test("1/2", 10, Ceiling, "0.54980", "0x0.8cc#10", Greater);
    test("1/2", 10, Nearest, "0.54883", "0x0.8c8#10", Less);
    test(
        "1/2",
        100,
        Floor,
        "0.54930614433405484569762261846113",
        "0x0.8c9f53d5681854bb520cc6aa8#100",
        Less,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "0.54930614433405484569762261846192",
        "0x0.8c9f53d5681854bb520cc6aa9#100",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "0.54930614433405484569762261846113",
        "0x0.8c9f53d5681854bb520cc6aa8#100",
        Less,
    );
    test("-1/2", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-1/2", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-1/2", 1, Nearest, "-0.50", "-0x0.8#1", Greater);
    test("-1/2", 10, Floor, "-0.54980", "-0x0.8cc#10", Less);
    test("-1/2", 10, Ceiling, "-0.54883", "-0x0.8c8#10", Greater);
    test("-1/2", 10, Nearest, "-0.54883", "-0x0.8c8#10", Greater);
    test(
        "-1/2",
        100,
        Floor,
        "-0.54930614433405484569762261846192",
        "-0x0.8c9f53d5681854bb520cc6aa9#100",
        Less,
    );
    test(
        "-1/2",
        100,
        Ceiling,
        "-0.54930614433405484569762261846113",
        "-0x0.8c9f53d5681854bb520cc6aa8#100",
        Greater,
    );
    test(
        "-1/2",
        100,
        Nearest,
        "-0.54930614433405484569762261846113",
        "-0x0.8c9f53d5681854bb520cc6aa8#100",
        Greater,
    );
    test("1/3", 1, Floor, "0.25", "0x0.4#1", Less);
    test("1/3", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("1/3", 1, Nearest, "0.25", "0x0.4#1", Less);
    test("1/3", 10, Floor, "0.34619", "0x0.58a#10", Less);
    test("1/3", 10, Ceiling, "0.34668", "0x0.58c#10", Greater);
    test("1/3", 10, Nearest, "0.34668", "0x0.58c#10", Greater);
    test(
        "1/3",
        100,
        Floor,
        "0.34657359027997265470861606072899",
        "0x0.58b90bfbe8e7bcd5e4f1d9cc00#100",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "0.34657359027997265470861606072939",
        "0x0.58b90bfbe8e7bcd5e4f1d9cc08#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.34657359027997265470861606072899",
        "0x0.58b90bfbe8e7bcd5e4f1d9cc00#100",
        Less,
    );
    test("-1/3", 1, Floor, "-0.50", "-0x0.8#1", Less);
    test("-1/3", 1, Ceiling, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 1, Nearest, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 10, Floor, "-0.34668", "-0x0.58c#10", Less);
    test("-1/3", 10, Ceiling, "-0.34619", "-0x0.58a#10", Greater);
    test("-1/3", 10, Nearest, "-0.34668", "-0x0.58c#10", Less);
    test(
        "-1/3",
        100,
        Floor,
        "-0.34657359027997265470861606072939",
        "-0x0.58b90bfbe8e7bcd5e4f1d9cc08#100",
        Less,
    );
    test(
        "-1/3",
        100,
        Ceiling,
        "-0.34657359027997265470861606072899",
        "-0x0.58b90bfbe8e7bcd5e4f1d9cc00#100",
        Greater,
    );
    test(
        "-1/3",
        100,
        Nearest,
        "-0.34657359027997265470861606072899",
        "-0x0.58b90bfbe8e7bcd5e4f1d9cc00#100",
        Greater,
    );
    test("3/4", 1, Floor, "0.50", "0x0.8#1", Less);
    test("3/4", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("3/4", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("3/4", 10, Floor, "0.97266", "0x0.f90#10", Less);
    test("3/4", 10, Ceiling, "0.97363", "0x0.f94#10", Greater);
    test("3/4", 10, Nearest, "0.97266", "0x0.f90#10", Less);
    test(
        "3/4",
        100,
        Floor,
        "0.97295507452765665255267637172144",
        "0x0.f913957192d2baa37b4a4b679#100",
        Less,
    );
    test(
        "3/4",
        100,
        Ceiling,
        "0.97295507452765665255267637172223",
        "0x0.f913957192d2baa37b4a4b67a#100",
        Greater,
    );
    test(
        "3/4",
        100,
        Nearest,
        "0.97295507452765665255267637172144",
        "0x0.f913957192d2baa37b4a4b679#100",
        Less,
    );
    test("99/100", 1, Floor, "2.0", "0x2.0#1", Less);
    test("99/100", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("99/100", 1, Nearest, "2.0", "0x2.0#1", Less);
    test("99/100", 10, Floor, "2.6445", "0x2.a5#10", Less);
    test("99/100", 10, Ceiling, "2.6484", "0x2.a6#10", Greater);
    test("99/100", 10, Nearest, "2.6484", "0x2.a6#10", Greater);
    test(
        "99/100",
        100,
        Floor,
        "2.6466524123622461977050606459341",
        "0x2.a58b0332f9b0c4c0aa1f8b894#100",
        Less,
    );
    test(
        "99/100",
        100,
        Ceiling,
        "2.6466524123622461977050606459373",
        "0x2.a58b0332f9b0c4c0aa1f8b898#100",
        Greater,
    );
    test(
        "99/100",
        100,
        Nearest,
        "2.6466524123622461977050606459341",
        "0x2.a58b0332f9b0c4c0aa1f8b894#100",
        Less,
    );
    test("-99/100", 1, Floor, "-4.0", "-0x4.0#1", Less);
    test("-99/100", 1, Ceiling, "-2.0", "-0x2.0#1", Greater);
    test("-99/100", 1, Nearest, "-2.0", "-0x2.0#1", Greater);
    test("-99/100", 10, Floor, "-2.6484", "-0x2.a6#10", Less);
    test("-99/100", 10, Ceiling, "-2.6445", "-0x2.a5#10", Greater);
    test("-99/100", 10, Nearest, "-2.6484", "-0x2.a6#10", Less);
    test(
        "-99/100",
        100,
        Floor,
        "-2.6466524123622461977050606459373",
        "-0x2.a58b0332f9b0c4c0aa1f8b898#100",
        Less,
    );
    test(
        "-99/100",
        100,
        Ceiling,
        "-2.6466524123622461977050606459341",
        "-0x2.a58b0332f9b0c4c0aa1f8b894#100",
        Greater,
    );
    test(
        "-99/100",
        100,
        Nearest,
        "-2.6466524123622461977050606459341",
        "-0x2.a58b0332f9b0c4c0aa1f8b894#100",
        Greater,
    );
    test("1/10", 1, Floor, "0.062", "0x0.1#1", Less);
    test("1/10", 1, Ceiling, "0.12", "0x0.2#1", Greater);
    test("1/10", 1, Nearest, "0.12", "0x0.2#1", Greater);
    test("1/10", 10, Floor, "0.10022", "0x0.19a8#10", Less);
    test("1/10", 10, Ceiling, "0.10034", "0x0.19b0#10", Greater);
    test("1/10", 10, Nearest, "0.10034", "0x0.19b0#10", Greater);
    test(
        "1/10",
        100,
        Floor,
        "0.10033534773107558063572655205996",
        "0x0.19af93cd2344120521b26f7578#100",
        Less,
    );
    test(
        "1/10",
        100,
        Ceiling,
        "0.10033534773107558063572655206006",
        "0x0.19af93cd2344120521b26f757a#100",
        Greater,
    );
    test(
        "1/10",
        100,
        Nearest,
        "0.10033534773107558063572655206006",
        "0x0.19af93cd2344120521b26f757a#100",
        Greater,
    );
    test("1/1000000", 1, Floor, "9.5e-7", "0x0.00001#1", Less);
    test("1/1000000", 1, Ceiling, "1.9e-6", "0x0.00002#1", Greater);
    test("1/1000000", 1, Nearest, "9.5e-7", "0x0.00001#1", Less);
    test("1/1000000", 10, Floor, "9.9838e-7", "0x0.000010c0#10", Less);
    test(
        "1/1000000",
        10,
        Ceiling,
        "1.0002e-6",
        "0x0.000010c8#10",
        Greater,
    );
    test(
        "1/1000000",
        10,
        Nearest,
        "1.0002e-6",
        "0x0.000010c8#10",
        Greater,
    );
    test(
        "1/1000000",
        100,
        Floor,
        "1.0000000000003333333333335333321e-6",
        "0x0.000010c6f7a0b5f3b355fab8b890e2#100",
        Less,
    );
    test(
        "1/1000000",
        100,
        Ceiling,
        "1.0000000000003333333333335333337e-6",
        "0x0.000010c6f7a0b5f3b355fab8b890e4#100",
        Greater,
    );
    test(
        "1/1000000",
        100,
        Nearest,
        "1.0000000000003333333333335333337e-6",
        "0x0.000010c6f7a0b5f3b355fab8b890e4#100",
        Greater,
    );
    test("-1/1000000", 1, Floor, "-1.9e-6", "-0x0.00002#1", Less);
    test("-1/1000000", 1, Ceiling, "-9.5e-7", "-0x0.00001#1", Greater);
    test("-1/1000000", 1, Nearest, "-9.5e-7", "-0x0.00001#1", Greater);
    test(
        "-1/1000000",
        10,
        Floor,
        "-1.0002e-6",
        "-0x0.000010c8#10",
        Less,
    );
    test(
        "-1/1000000",
        10,
        Ceiling,
        "-9.9838e-7",
        "-0x0.000010c0#10",
        Greater,
    );
    test(
        "-1/1000000",
        10,
        Nearest,
        "-1.0002e-6",
        "-0x0.000010c8#10",
        Less,
    );
    test(
        "-1/1000000",
        100,
        Floor,
        "-1.0000000000003333333333335333337e-6",
        "-0x0.000010c6f7a0b5f3b355fab8b890e4#100",
        Less,
    );
    test(
        "-1/1000000",
        100,
        Ceiling,
        "-1.0000000000003333333333335333321e-6",
        "-0x0.000010c6f7a0b5f3b355fab8b890e2#100",
        Greater,
    );
    test(
        "-1/1000000",
        100,
        Nearest,
        "-1.0000000000003333333333335333337e-6",
        "-0x0.000010c6f7a0b5f3b355fab8b890e4#100",
        Less,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        1,
        Floor,
        "32.0",
        "0x2.0E+1#1",
        Less,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        1,
        Ceiling,
        "64.0",
        "0x4.0E+1#1",
        Greater,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        1,
        Nearest,
        "32.0",
        "0x2.0E+1#1",
        Less,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        10,
        Floor,
        "35.000",
        "0x23.0#10",
        Less,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        10,
        Ceiling,
        "35.062",
        "0x23.1#10",
        Greater,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        10,
        Nearest,
        "35.000",
        "0x23.0#10",
        Less,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        100,
        Floor,
        "35.003932618277238125570222133628",
        "0x23.0101ba62e36d8063536aed7c#100",
        Less,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        100,
        Ceiling,
        "35.003932618277238125570222133679",
        "0x23.0101ba62e36d8063536aed80#100",
        Greater,
    );
    test(
        "1267650600228229401496703205375/1267650600228229401496703205376",
        100,
        Nearest,
        "35.003932618277238125570222133628",
        "0x23.0101ba62e36d8063536aed7c#100",
        Less,
    );
    test(
        "53575430359313366047421252453000090528070240585276680372187519418517552556246806124659918\
        940784792906379733645877657341259357264284615702179922887873492874019672838874121154927105\
        373025311855709389770910765232374917909706336993837795827719730385314572855982388432710838\
        30214915826312193418602834034687/107150860718626732094842504906000181056140481170553360744\
        375038837035105112493612249319837881569585812759467291755314682518714528569231404359845775\
        746985748039345677748242309854210746050623711418779541821530464749835819412673987675591655\
        43946077062914571196477686542167660429831652624386837205668069376",
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "53575430359313366047421252453000090528070240585276680372187519418517552556246806124659918\
        940784792906379733645877657341259357264284615702179922887873492874019672838874121154927105\
        373025311855709389770910765232374917909706336993837795827719730385314572855982388432710838\
        30214915826312193418602834034687/107150860718626732094842504906000181056140481170553360744\
        375038837035105112493612249319837881569585812759467291755314682518714528569231404359845775\
        746985748039345677748242309854210746050623711418779541821530464749835819412673987675591655\
        43946077062914571196477686542167660429831652624386837205668069376",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "53575430359313366047421252453000090528070240585276680372187519418517552556246806124659918\
        940784792906379733645877657341259357264284615702179922887873492874019672838874121154927105\
        373025311855709389770910765232374917909706336993837795827719730385314572855982388432710838\
        30214915826312193418602834034687/107150860718626732094842504906000181056140481170553360744\
        375038837035105112493612249319837881569585812759467291755314682518714528569231404359845775\
        746985748039345677748242309854210746050623711418779541821530464749835819412673987675591655\
        43946077062914571196477686542167660429831652624386837205668069376",
        1,
        Nearest,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "53575430359313366047421252453000090528070240585276680372187519418517552556246806124659918\
        940784792906379733645877657341259357264284615702179922887873492874019672838874121154927105\
        373025311855709389770910765232374917909706336993837795827719730385314572855982388432710838\
        30214915826312193418602834034687/107150860718626732094842504906000181056140481170553360744\
        375038837035105112493612249319837881569585812759467291755314682518714528569231404359845775\
        746985748039345677748242309854210746050623711418779541821530464749835819412673987675591655\
        43946077062914571196477686542167660429831652624386837205668069376",
        10,
        Floor,
        "0.54883",
        "0x0.8c8#10",
        Less,
    );
    test(
        "53575430359313366047421252453000090528070240585276680372187519418517552556246806124659918\
        940784792906379733645877657341259357264284615702179922887873492874019672838874121154927105\
        373025311855709389770910765232374917909706336993837795827719730385314572855982388432710838\
        30214915826312193418602834034687/107150860718626732094842504906000181056140481170553360744\
        375038837035105112493612249319837881569585812759467291755314682518714528569231404359845775\
        746985748039345677748242309854210746050623711418779541821530464749835819412673987675591655\
        43946077062914571196477686542167660429831652624386837205668069376",
        10,
        Ceiling,
        "0.54980",
        "0x0.8cc#10",
        Greater,
    );
    test(
        "53575430359313366047421252453000090528070240585276680372187519418517552556246806124659918\
        940784792906379733645877657341259357264284615702179922887873492874019672838874121154927105\
        373025311855709389770910765232374917909706336993837795827719730385314572855982388432710838\
        30214915826312193418602834034687/107150860718626732094842504906000181056140481170553360744\
        375038837035105112493612249319837881569585812759467291755314682518714528569231404359845775\
        746985748039345677748242309854210746050623711418779541821530464749835819412673987675591655\
        43946077062914571196477686542167660429831652624386837205668069376",
        10,
        Nearest,
        "0.54883",
        "0x0.8c8#10",
        Less,
    );
    test(
        "53575430359313366047421252453000090528070240585276680372187519418517552556246806124659918\
        940784792906379733645877657341259357264284615702179922887873492874019672838874121154927105\
        373025311855709389770910765232374917909706336993837795827719730385314572855982388432710838\
        30214915826312193418602834034687/107150860718626732094842504906000181056140481170553360744\
        375038837035105112493612249319837881569585812759467291755314682518714528569231404359845775\
        746985748039345677748242309854210746050623711418779541821530464749835819412673987675591655\
        43946077062914571196477686542167660429831652624386837205668069376",
        100,
        Floor,
        "0.54930614433405484569762261846113",
        "0x0.8c9f53d5681854bb520cc6aa8#100",
        Less,
    );
    test(
        "53575430359313366047421252453000090528070240585276680372187519418517552556246806124659918\
        940784792906379733645877657341259357264284615702179922887873492874019672838874121154927105\
        373025311855709389770910765232374917909706336993837795827719730385314572855982388432710838\
        30214915826312193418602834034687/107150860718626732094842504906000181056140481170553360744\
        375038837035105112493612249319837881569585812759467291755314682518714528569231404359845775\
        746985748039345677748242309854210746050623711418779541821530464749835819412673987675591655\
        43946077062914571196477686542167660429831652624386837205668069376",
        100,
        Ceiling,
        "0.54930614433405484569762261846192",
        "0x0.8c9f53d5681854bb520cc6aa9#100",
        Greater,
    );
    test(
        "53575430359313366047421252453000090528070240585276680372187519418517552556246806124659918\
        940784792906379733645877657341259357264284615702179922887873492874019672838874121154927105\
        373025311855709389770910765232374917909706336993837795827719730385314572855982388432710838\
        30214915826312193418602834034687/107150860718626732094842504906000181056140481170553360744\
        375038837035105112493612249319837881569585812759467291755314682518714528569231404359845775\
        746985748039345677748242309854210746050623711418779541821530464749835819412673987675591655\
        43946077062914571196477686542167660429831652624386837205668069376",
        100,
        Nearest,
        "0.54930614433405484569762261846113",
        "0x0.8c9f53d5681854bb520cc6aa8#100",
        Less,
    );
    // - x is small enough for the series, and x = 1/64 alone is exactly representable, so the
    //   bracket needs a second term
    test("1/64", 1, Up, "0.031", "0x0.08#1", Greater);
    // - the series bracket needs a third term
    test(
        "3/512",
        24,
        Nearest,
        "0.00585944206",
        "0x0.01800120#24",
        Less,
    );
}

#[test]
fn test_atanh_rational_extreme() {
    // |x| = 2^(-2^30 - 10) is too small to be a `Float`, and atanh(x) underflows
    let x = Rational::power_of_2(-(1i64 << 30) - 10);
    for (rm, out, o_out) in [
        (Floor, Float::ZERO, Less),
        (Down, Float::ZERO, Less),
        (Nearest, Float::ZERO, Less),
        (Ceiling, Float::min_positive_value_prec(10), Greater),
        (Up, Float::min_positive_value_prec(10), Greater),
    ] {
        let (c, o) = Float::atanh_rational_prec_round_ref(&x, 10, rm);
        assert_eq!(ComparableFloat(c.clone()), ComparableFloat(out));
        assert_eq!(o, o_out);
        let (c_neg, o_neg) = Float::atanh_rational_prec_round(-&x, 10, -rm);
        assert_eq!(ComparableFloat(c_neg), ComparableFloat(-c));
        assert_eq!(o_neg, o.reverse());
    }

    // x = 1 - 2^(-2^20) is close to 1; atanh(x) = ln((2 - t)/t)/2 with t = 2^(-2^20)
    let x = Rational::ONE - Rational::power_of_2(-(1i64 << 20));
    for prec in [10, 100] {
        for rm in [Floor, Ceiling, Down, Up, Nearest] {
            let (c, o) = Float::atanh_rational_prec_round_ref(&x, prec, rm);
            let (rug_c, rug_o) = rug_atanh_rational_prec_round(
                &x,
                prec,
                rug_round_try_from_rounding_mode(rm).unwrap(),
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
            let (c_neg, o_neg) = Float::atanh_rational_prec_round(-&x, prec, -rm);
            assert_eq!(ComparableFloat(c_neg), ComparableFloat(-c));
            assert_eq!(o_neg, o.reverse());
        }
    }
}

#[test]
#[should_panic]
fn atanh_rational_prec_fail() {
    Float::atanh_rational_prec(Rational::ONE_HALF, 0);
}

#[test]
#[should_panic]
fn atanh_rational_prec_ref_fail() {
    Float::atanh_rational_prec_ref(&Rational::ONE_HALF, 0);
}

#[test]
#[should_panic]
fn atanh_rational_prec_round_fail_1() {
    Float::atanh_rational_prec_round(Rational::ONE_HALF, 0, Floor);
}

#[test]
#[should_panic]
fn atanh_rational_prec_round_fail_2() {
    Float::atanh_rational_prec_round(Rational::ONE_HALF, 10, Exact);
}

#[test]
#[should_panic]
fn atanh_rational_prec_round_ref_fail() {
    Float::atanh_rational_prec_round_ref(&Rational::ONE_HALF, 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn atanh_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (c, o) = Float::atanh_rational_prec_round(x.clone(), prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = Float::atanh_rational_prec_round_ref(&x, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // atanh is odd (a `Rational` has no negative zero, so x = 0 is excluded)
    if x != 0u32 {
        let (c_neg, o_neg) = Float::atanh_rational_prec_round(-&x, prec, -rm);
        assert_eq!(ComparableFloatRef(&c_neg), ComparableFloatRef(&-&c));
        assert_eq!(o_neg, o.reverse());
    }

    // atanh is NaN beyond +/-1, infinite at +/-1, and otherwise has the sign of x
    let x_abs = (&x).abs();
    if x_abs > 1u32 {
        assert!(c.is_nan());
    } else if x_abs == 1u32 {
        assert!(c.is_infinite());
    } else if x != 0u32 && c != 0u32 {
        assert_eq!(c > 0u32, x > 0u32);
    }

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_atanh_rational_prec_round(&x, prec, rrm);
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
            let (s, oo) = Float::atanh_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::atanh_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn atanh_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_16().test_properties(|(x, prec, rm)| {
        atanh_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (c, o) = Float::atanh_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (c, o) = Float::atanh_rational_prec_round(Rational::ONE, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn atanh_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (c, o) = Float::atanh_rational_prec(x.clone(), prec);
    assert!(c.is_valid());

    let (c_alt, o_alt) = Float::atanh_rational_prec_ref(&x, prec);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let (c_alt, o_alt) = Float::atanh_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let (rug_c, rug_o) = rug_atanh_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_c)),
        ComparableFloatRef(&c)
    );
    assert_eq!(rug_o, o);

    // the inverse hyperbolic tangent of an exactly representable rational is the Float inverse
    // hyperbolic tangent
    if let Ok(f) = Float::try_from(&x) {
        let (c_alt, o_alt) = f.atanh_prec(prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }
}

#[test]
fn atanh_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        atanh_rational_prec_properties_helper(x, prec);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_atanh_rational() {
    fn test<T: PrimitiveFloat>(x: &Rational, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_atanh_rational::<T>(x)),
            NiceFloat(out)
        );
    }
    let test_s = |s: &str| Rational::from_str(s).unwrap();
    test::<f32>(&test_s("0"), 0.0);
    test::<f64>(&test_s("0"), 0.0);
    test::<f32>(&test_s("1"), f32::INFINITY);
    test::<f64>(&test_s("1"), f64::INFINITY);
    test::<f32>(&test_s("-1"), f32::NEGATIVE_INFINITY);
    test::<f64>(&test_s("-1"), f64::NEGATIVE_INFINITY);
    test::<f32>(&test_s("2"), f32::NAN);
    test::<f64>(&test_s("2"), f64::NAN);
    test::<f32>(&test_s("1/2"), 0.54930615);
    test::<f64>(&test_s("1/2"), 0.5493061443340549);
    test::<f32>(&test_s("-1/2"), -0.54930615);
    test::<f64>(&test_s("-1/2"), -0.5493061443340549);
    test::<f32>(&test_s("1/3"), 0.3465736);
    test::<f64>(&test_s("1/3"), 0.34657359027997264);
    test::<f32>(&test_s("99/100"), 2.6466525);
    test::<f64>(&test_s("99/100"), 2.646652412362246);
    test::<f32>(&test_s("1/100000000000000000000"), 1.0e-20);
    test::<f64>(&test_s("1/100000000000000000000"), 1.0e-20);
    test::<f32>(
        &test_s("1/10000000000000000000000000000000000000000"),
        1.0e-40,
    );
    test::<f64>(
        &test_s("1/10000000000000000000000000000000000000000"),
        1.0e-40,
    );
    test::<f32>(
        &test_s("1/100000000000000000000000000000000000000000000000000"),
        0.0,
    );
    test::<f64>(
        &test_s("1/100000000000000000000000000000000000000000000000000"),
        1.0e-50,
    );

    // a subnormal f64 result, and results that underflow
    let x = Rational::power_of_2(-1070i64);
    test::<f32>(&x, 0.0);
    test::<f64>(&x, 8.0e-323);
    test::<f32>(&-&x, -0.0);
    test::<f64>(&-x, -8.0e-323);
    let x = Rational::from_unsigneds(1u32, 3) >> 1100u32;
    test::<f32>(&x, 0.0);
    test::<f64>(&x, 0.0);
    test::<f32>(&-&x, -0.0);
    test::<f64>(&-x, -0.0);
    // a subnormal f32 result
    let x = Rational::from_unsigneds(1u32, 3) >> 140u32;
    test::<f32>(&x, 2.4e-43);
    test::<f64>(&x, 2.3915493791143543e-43);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_atanh_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let c = primitive_float_atanh_rational::<T>(&x);
        // the inverse hyperbolic tangent of a rational is NaN exactly beyond +/-1
        assert_eq!(c.is_nan(), (&x).abs() > 1u32);
        // atanh is odd (a `Rational` has no negative zero, so x = 0 is excluded)
        if x != 0u32 {
            assert_eq!(
                NiceFloat(primitive_float_atanh_rational::<T>(&-&x)),
                NiceFloat(-c)
            );
        }
        if (&x).abs() < 1u32 {
            // the result is the correctly rounded inverse hyperbolic tangent, as computed by MPFR
            // with 64 bits to spare, so that a subnormal result is rounded once by the conversion
            // rather than twice
            let rug_c = rug_atanh_rational_prec(&x, T::MANTISSA_WIDTH + 64).0;
            let rug_c: T = T::rounding_from(&<Float as From<&rug::Float>>::from(&rug_c), Nearest).0;
            assert_eq!(NiceFloat(rug_c), NiceFloat(c));
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // The inverse hyperbolic tangent of a finite nonzero primitive float, taken through the
        // `Rational` path, matches the direct primitive-float inverse hyperbolic tangent (a
        // `Rational` cannot carry the sign of a zero).
        if x.is_finite() && x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_atanh_rational::<T>(&Rational::exact_from(
                    x
                ))),
                NiceFloat(primitive_float_atanh(x))
            );
        }
    });
}

#[test]
fn primitive_float_atanh_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_atanh_rational_properties_helper);
}
