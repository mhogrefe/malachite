// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Acoth, AcothAssign, PowerOf2, Reciprocal};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, OneHalf, Two, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::acoth::{
    primitive_float_acoth, primitive_float_acoth_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, round_once_to_primitive,
    rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::acoth::{
    rug_acoth, rug_acoth_prec, rug_acoth_prec_round, rug_acoth_rational_prec,
    rug_acoth_rational_prec_round, rug_acoth_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_55,
    rational_unsigned_rounding_mode_triple_gen_var_18,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_acoth_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().acoth_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.acoth_prec_round_ref(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.acoth_prec_round_assign(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acoth_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
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
    test("0.0", "0x0.0", 1, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Ceiling, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 10, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 10, Ceiling, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 10, Nearest, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", 1, Floor, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", 1, Ceiling, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", 10, Floor, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", 10, Ceiling, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", 10, Nearest, "NaN", "NaN", Equal);
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
    test("0.50", "0x0.8#1", 1, Floor, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 1, Ceiling, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 1, Nearest, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 10, Floor, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 1, Floor, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 1, Ceiling, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 1, Nearest, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Floor, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Ceiling, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Nearest, "NaN", "NaN", Equal);
    // - x is a power of 2, so 1/x is exact and the result is atanh(1/x)
    test("2.0", "0x2.0#1", 1, Floor, "0.50", "0x0.8#1", Less);
    test("2.0", "0x2.0#1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("2.0", "0x2.0#1", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("2.0", "0x2.0#1", 10, Floor, "0.54883", "0x0.8c8#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "0.54980",
        "0x0.8cc#10",
        Greater,
    );
    test("2.0", "0x2.0#1", 10, Nearest, "0.54883", "0x0.8c8#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        100,
        Floor,
        "0.54930614433405484569762261846113",
        "0x0.8c9f53d5681854bb520cc6aa8#100",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        100,
        Ceiling,
        "0.54930614433405484569762261846192",
        "0x0.8c9f53d5681854bb520cc6aa9#100",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        100,
        Nearest,
        "0.54930614433405484569762261846113",
        "0x0.8c9f53d5681854bb520cc6aa8#100",
        Less,
    );
    test("-2.0", "-0x2.0#1", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-2.0", "-0x2.0#1", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-2.0", "-0x2.0#1", 1, Nearest, "-0.50", "-0x0.8#1", Greater);
    test(
        "-2.0",
        "-0x2.0#1",
        10,
        Floor,
        "-0.54980",
        "-0x0.8cc#10",
        Less,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        10,
        Ceiling,
        "-0.54883",
        "-0x0.8c8#10",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        10,
        Nearest,
        "-0.54883",
        "-0x0.8c8#10",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        100,
        Floor,
        "-0.54930614433405484569762261846192",
        "-0x0.8c9f53d5681854bb520cc6aa9#100",
        Less,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        100,
        Ceiling,
        "-0.54930614433405484569762261846113",
        "-0x0.8c9f53d5681854bb520cc6aa8#100",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        100,
        Nearest,
        "-0.54930614433405484569762261846113",
        "-0x0.8c9f53d5681854bb520cc6aa8#100",
        Greater,
    );
    test("1.5", "0x1.8#2", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1.5", "0x1.8#2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1.5", "0x1.8#2", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1.5", "0x1.8#2", 10, Floor, "0.80469", "0x0.ce0#10", Less);
    test(
        "1.5",
        "0x1.8#2",
        10,
        Ceiling,
        "0.80566",
        "0x0.ce4#10",
        Greater,
    );
    test("1.5", "0x1.8#2", 10, Nearest, "0.80469", "0x0.ce0#10", Less);
    test(
        "1.5",
        "0x1.8#2",
        100,
        Floor,
        "0.80471895621705018730037966661291",
        "0x0.ce020fbf6c699b57efbbd28b0#100",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        100,
        Ceiling,
        "0.80471895621705018730037966661370",
        "0x0.ce020fbf6c699b57efbbd28b1#100",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        100,
        Nearest,
        "0.80471895621705018730037966661291",
        "0x0.ce020fbf6c699b57efbbd28b0#100",
        Less,
    );
    test("-1.5", "-0x1.8#2", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-1.5", "-0x1.8#2", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-1.5", "-0x1.8#2", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test(
        "-1.5",
        "-0x1.8#2",
        10,
        Floor,
        "-0.80566",
        "-0x0.ce4#10",
        Less,
    );
    test(
        "-1.5",
        "-0x1.8#2",
        10,
        Ceiling,
        "-0.80469",
        "-0x0.ce0#10",
        Greater,
    );
    test(
        "-1.5",
        "-0x1.8#2",
        10,
        Nearest,
        "-0.80469",
        "-0x0.ce0#10",
        Greater,
    );
    test(
        "-1.5",
        "-0x1.8#2",
        100,
        Floor,
        "-0.80471895621705018730037966661370",
        "-0x0.ce020fbf6c699b57efbbd28b1#100",
        Less,
    );
    test(
        "-1.5",
        "-0x1.8#2",
        100,
        Ceiling,
        "-0.80471895621705018730037966661291",
        "-0x0.ce020fbf6c699b57efbbd28b0#100",
        Greater,
    );
    test(
        "-1.5",
        "-0x1.8#2",
        100,
        Nearest,
        "-0.80471895621705018730037966661291",
        "-0x0.ce020fbf6c699b57efbbd28b0#100",
        Greater,
    );
    test("3.0", "0x3.0#2", 1, Floor, "0.25", "0x0.4#1", Less);
    test("3.0", "0x3.0#2", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("3.0", "0x3.0#2", 1, Nearest, "0.25", "0x0.4#1", Less);
    test("3.0", "0x3.0#2", 10, Floor, "0.34619", "0x0.58a#10", Less);
    test(
        "3.0",
        "0x3.0#2",
        10,
        Ceiling,
        "0.34668",
        "0x0.58c#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "0.34668",
        "0x0.58c#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        100,
        Floor,
        "0.34657359027997265470861606072899",
        "0x0.58b90bfbe8e7bcd5e4f1d9cc00#100",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        100,
        Ceiling,
        "0.34657359027997265470861606072939",
        "0x0.58b90bfbe8e7bcd5e4f1d9cc08#100",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        100,
        Nearest,
        "0.34657359027997265470861606072899",
        "0x0.58b90bfbe8e7bcd5e4f1d9cc00#100",
        Less,
    );
    test("10.0", "0xa.0#4", 1, Floor, "0.062", "0x0.1#1", Less);
    test("10.0", "0xa.0#4", 1, Ceiling, "0.12", "0x0.2#1", Greater);
    test("10.0", "0xa.0#4", 1, Nearest, "0.12", "0x0.2#1", Greater);
    test("10.0", "0xa.0#4", 10, Floor, "0.10022", "0x0.19a8#10", Less);
    test(
        "10.0",
        "0xa.0#4",
        10,
        Ceiling,
        "0.10034",
        "0x0.19b0#10",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#4",
        10,
        Nearest,
        "0.10034",
        "0x0.19b0#10",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#4",
        100,
        Floor,
        "0.10033534773107558063572655205996",
        "0x0.19af93cd2344120521b26f7578#100",
        Less,
    );
    test(
        "10.0",
        "0xa.0#4",
        100,
        Ceiling,
        "0.10033534773107558063572655206006",
        "0x0.19af93cd2344120521b26f757a#100",
        Greater,
    );
    test(
        "10.0",
        "0xa.0#4",
        100,
        Nearest,
        "0.10033534773107558063572655206006",
        "0x0.19af93cd2344120521b26f757a#100",
        Greater,
    );
    test(
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        1,
        Floor,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        1,
        Ceiling,
        "0.50",
        "0x0.8#1",
        Greater,
    );
    test(
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        1,
        Nearest,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        10,
        Floor,
        "0.32959",
        "0x0.546#10",
        Less,
    );
    test(
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        10,
        Ceiling,
        "0.33008",
        "0x0.548#10",
        Greater,
    );
    test(
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        10,
        Nearest,
        "0.32959",
        "0x0.546#10",
        Less,
    );
    test(
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        100,
        Floor,
        "0.32962281444213198059363373110635",
        "0x0.546229280b5ffbc3284155d018#100",
        Less,
    );
    test(
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        100,
        Ceiling,
        "0.32962281444213198059363373110675",
        "0x0.546229280b5ffbc3284155d020#100",
        Greater,
    );
    test(
        "3.1428571428571428571428571428585",
        "0x3.2492492492492492492492494#100",
        100,
        Nearest,
        "0.32962281444213198059363373110635",
        "0x0.546229280b5ffbc3284155d018#100",
        Less,
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
        "-0.33008",
        "-0x0.548#10",
        Less,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        10,
        Ceiling,
        "-0.32959",
        "-0x0.546#10",
        Greater,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        10,
        Nearest,
        "-0.32959",
        "-0x0.546#10",
        Greater,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        100,
        Floor,
        "-0.32962281444213198059363373110675",
        "-0x0.546229280b5ffbc3284155d020#100",
        Less,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        100,
        Ceiling,
        "-0.32962281444213198059363373110635",
        "-0x0.546229280b5ffbc3284155d018#100",
        Greater,
    );
    test(
        "-3.1428571428571428571428571428585",
        "-0x3.2492492492492492492492494#100",
        100,
        Nearest,
        "-0.32962281444213198059363373110635",
        "-0x0.546229280b5ffbc3284155d018#100",
        Greater,
    );
    test("1.0010", "0x1.004#11", 1, Floor, "2.0", "0x2.0#1", Less);
    test(
        "1.0010",
        "0x1.004#11",
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "1.0010",
        "0x1.004#11",
        1,
        Nearest,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "1.0010",
        "0x1.004#11",
        10,
        Floor,
        "3.8125",
        "0x3.d0#10",
        Less,
    );
    test(
        "1.0010",
        "0x1.004#11",
        10,
        Ceiling,
        "3.8164",
        "0x3.d1#10",
        Greater,
    );
    test(
        "1.0010",
        "0x1.004#11",
        10,
        Nearest,
        "3.8125",
        "0x3.d0#10",
        Less,
    );
    test(
        "1.0010",
        "0x1.004#11",
        100,
        Floor,
        "3.8125535741194498773890451928640",
        "0x3.d00382d3174872b9599821304#100",
        Less,
    );
    test(
        "1.0010",
        "0x1.004#11",
        100,
        Ceiling,
        "3.8125535741194498773890451928671",
        "0x3.d00382d3174872b9599821308#100",
        Greater,
    );
    test(
        "1.0010",
        "0x1.004#11",
        100,
        Nearest,
        "3.8125535741194498773890451928671",
        "0x3.d00382d3174872b9599821308#100",
        Greater,
    );
    test(
        "1.0000000000000000000000000000008",
        "0x1.0000000000000000000000001#101",
        1,
        Floor,
        "32.0",
        "0x2.0E+1#1",
        Less,
    );
    test(
        "1.0000000000000000000000000000008",
        "0x1.0000000000000000000000001#101",
        1,
        Ceiling,
        "64.0",
        "0x4.0E+1#1",
        Greater,
    );
    test(
        "1.0000000000000000000000000000008",
        "0x1.0000000000000000000000001#101",
        1,
        Nearest,
        "32.0",
        "0x2.0E+1#1",
        Less,
    );
    test(
        "1.0000000000000000000000000000008",
        "0x1.0000000000000000000000001#101",
        10,
        Floor,
        "35.000",
        "0x23.0#10",
        Less,
    );
    test(
        "1.0000000000000000000000000000008",
        "0x1.0000000000000000000000001#101",
        10,
        Ceiling,
        "35.062",
        "0x23.1#10",
        Greater,
    );
    test(
        "1.0000000000000000000000000000008",
        "0x1.0000000000000000000000001#101",
        10,
        Nearest,
        "35.000",
        "0x23.0#10",
        Less,
    );
    test(
        "1.0000000000000000000000000000008",
        "0x1.0000000000000000000000001#101",
        100,
        Floor,
        "35.003932618277238125570222133628",
        "0x23.0101ba62e36d8063536aed7c#100",
        Less,
    );
    test(
        "1.0000000000000000000000000000008",
        "0x1.0000000000000000000000001#101",
        100,
        Ceiling,
        "35.003932618277238125570222133679",
        "0x23.0101ba62e36d8063536aed80#100",
        Greater,
    );
    test(
        "1.0000000000000000000000000000008",
        "0x1.0000000000000000000000001#101",
        100,
        Nearest,
        "35.003932618277238125570222133628",
        "0x23.0101ba62e36d8063536aed7c#100",
        Less,
    );
    test(
        "-1.0000010002",
        "-0x1.000010c8#30",
        1,
        Floor,
        "-8.0",
        "-0x8.0#1",
        Less,
    );
    test(
        "-1.0000010002",
        "-0x1.000010c8#30",
        1,
        Ceiling,
        "-4.0",
        "-0x4.0#1",
        Greater,
    );
    test(
        "-1.0000010002",
        "-0x1.000010c8#30",
        1,
        Nearest,
        "-8.0",
        "-0x8.0#1",
        Less,
    );
    test(
        "-1.0000010002",
        "-0x1.000010c8#30",
        10,
        Floor,
        "-7.2578",
        "-0x7.42#10",
        Less,
    );
    test(
        "-1.0000010002",
        "-0x1.000010c8#30",
        10,
        Ceiling,
        "-7.2500",
        "-0x7.40#10",
        Greater,
    );
    test(
        "-1.0000010002",
        "-0x1.000010c8#30",
        10,
        Nearest,
        "-7.2578",
        "-0x7.42#10",
        Less,
    );
    test(
        "-1.0000010002",
        "-0x1.000010c8#30",
        100,
        Floor,
        "-7.2542089112047960516895312961767",
        "-0x7.4113d5cff9f3300f6cb454830#100",
        Less,
    );
    test(
        "-1.0000010002",
        "-0x1.000010c8#30",
        100,
        Ceiling,
        "-7.2542089112047960516895312961704",
        "-0x7.4113d5cff9f3300f6cb454828#100",
        Greater,
    );
    test(
        "-1.0000010002",
        "-0x1.000010c8#30",
        100,
        Nearest,
        "-7.2542089112047960516895312961767",
        "-0x7.4113d5cff9f3300f6cb454830#100",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        1,
        Floor,
        "7.9e-31",
        "0x1.0E-25#1",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        1,
        Ceiling,
        "1.6e-30",
        "0x2.0E-25#1",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        1,
        Nearest,
        "7.9e-31",
        "0x1.0E-25#1",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        10,
        Floor,
        "7.8886e-31",
        "0x1.000E-25#10",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        10,
        Ceiling,
        "7.9040e-31",
        "0x1.008E-25#10",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        10,
        Nearest,
        "7.8886e-31",
        "0x1.000E-25#10",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        100,
        Floor,
        "7.8886090522101180541172856528279e-31",
        "0x1.0000000000000000000000000E-25#100",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        100,
        Ceiling,
        "7.8886090522101180541172856528403e-31",
        "0x1.0000000000000000000000002E-25#100",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        100,
        Nearest,
        "7.8886090522101180541172856528279e-31",
        "0x1.0000000000000000000000000E-25#100",
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
        "1.0000000000003333333333335333321e-6",
        "0x0.000010c6f7a0b5f3b355fab8b890e2#100",
        Less,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        100,
        Ceiling,
        "1.0000000000003333333333335333337e-6",
        "0x0.000010c6f7a0b5f3b355fab8b890e4#100",
        Greater,
    );
    test(
        "1000000.0",
        "0xf4240.0#20",
        100,
        Nearest,
        "1.0000000000003333333333335333337e-6",
        "0x0.000010c6f7a0b5f3b355fab8b890e4#100",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        1,
        Floor,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        1,
        Ceiling,
        "9.5e-323228497",
        "0x4.0E-268435456#1",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        1,
        Nearest,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Floor,
        "4.7651e-323228497",
        "0x2.00E-268435456#10",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Ceiling,
        "4.7744e-323228497",
        "0x2.01E-268435456#10",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        10,
        Nearest,
        "4.7744e-323228497",
        "0x2.01E-268435456#10",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        100,
        Floor,
        "4.7697878056798864105050984486654e-323228497",
        "0x2.0080200802008020080200800E-268435456#100",
        Less,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        100,
        Ceiling,
        "4.7697878056798864105050984486729e-323228497",
        "0x2.0080200802008020080200804E-268435456#100",
        Greater,
    );
    test(
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        100,
        Nearest,
        "4.7697878056798864105050984486729e-323228497",
        "0x2.0080200802008020080200804E-268435456#100",
        Greater,
    );
    test(
        "-2.0985787164673876924043581168822e323228496",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Floor,
        "-9.5e-323228497",
        "-0x4.0E-268435456#1",
        Less,
    );
    test(
        "-2.0985787164673876924043581168822e323228496",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Ceiling,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Greater,
    );
    test(
        "-2.0985787164673876924043581168822e323228496",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        1,
        Nearest,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Greater,
    );
    test(
        "-2.0985787164673876924043581168822e323228496",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Floor,
        "-4.7744e-323228497",
        "-0x2.01E-268435456#10",
        Less,
    );
    test(
        "-2.0985787164673876924043581168822e323228496",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Ceiling,
        "-4.7651e-323228497",
        "-0x2.00E-268435456#10",
        Greater,
    );
    test(
        "-2.0985787164673876924043581168822e323228496",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        10,
        Nearest,
        "-4.7651e-323228497",
        "-0x2.00E-268435456#10",
        Greater,
    );
    test(
        "-2.0985787164673876924043581168822e323228496",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        100,
        Floor,
        "-4.7651298097759021464323395634729e-323228497",
        "-0x2.0000000000000000000000004E-268435456#100",
        Less,
    );
    test(
        "-2.0985787164673876924043581168822e323228496",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        100,
        Ceiling,
        "-4.7651298097759021464323395634653e-323228497",
        "-0x2.0000000000000000000000000E-268435456#100",
        Greater,
    );
    test(
        "-2.0985787164673876924043581168822e323228496",
        "-0x7.ffffffffffffffffffffffff8E+268435455#100",
        100,
        Nearest,
        "-4.7651298097759021464323395634729e-323228497",
        "-0x2.0000000000000000000000004E-268435456#100",
        Less,
    );
    // - 1 < |x| < 2, so d = |x| - 1 is exact, and the first approximation does not allow rounding,
    //   so the loop retries
    test("1.0623", "0x1.0ff#13", 3, Down, "1.8", "0x1.c#3", Less);
    // - |x| > 2, so d = |x| - 1 is rounded down, and the first approximation does not allow
    //   rounding, so the loop retries
    test(
        "513.00",
        "0x201.0#10",
        1,
        Up,
        "0.0020",
        "0x0.008#1",
        Greater,
    );
}

#[test]
#[should_panic]
fn acoth_prec_round_fail() {
    Float::TWO.acoth_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn acoth_prec_round_exact_fail() {
    Float::TWO.acoth_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acoth_prec_fail() {
    Float::TWO.acoth_prec(0);
}

#[test]
#[should_panic]
fn acoth_round_fail() {
    Float::TWO.acoth_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn acoth_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (c, o) = x.clone().acoth_prec_round(prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.acoth_prec_round_ref(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.acoth_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_acoth_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // acoth is odd
    let (c_neg, o_neg) = (-&x).acoth_prec_round(prec, -rm);
    assert_eq!(ComparableFloat(c_neg), ComparableFloat(-&c));
    assert_eq!(o_neg, o.reverse());

    // acoth is NaN exactly for NaN and every x with |x| < 1, ±inf at ±1, and otherwise has the
    // sign of x
    assert_eq!(c.is_nan(), x.is_nan() || (x.is_finite() && x.lt_abs(&1u32)));
    if !c.is_nan() {
        assert_eq!(c.is_sign_positive(), x.is_sign_positive());
    }
    if x.is_finite() && x != 0u32 && rm != Exact {
        // acoth(x) = atanh(1/x), and the reciprocal of a finite nonzero Float is an exact Rational,
        // so the independent Rational inverse hyperbolic tangent must agree
        let (c_alt, o_alt) =
            Float::atanh_rational_prec_round(Rational::exact_from(&x).reciprocal(), prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }
    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o == Equal {
        // acoth is exact only for x = ±1 and ±inf (and the inputs whose result is NaN): the
        // result is rounding-mode-invariant
        for rm2 in exhaustive_rounding_modes() {
            let (c2, o2) = x.acoth_prec_round_ref(prec, rm2);
            assert_eq!(ComparableFloat(c2), ComparableFloat(c.clone()));
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.acoth_prec_round_ref(prec, Exact));
    }
}

#[test]
fn acoth_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        acoth_prec_round_properties_helper(x, prec, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_55().test_properties(|(x, prec, rm)| {
        acoth_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        for x in [Float::NAN, Float::ZERO, Float::NEGATIVE_ZERO, Float::ONE_HALF] {
            let (c, o) = x.acoth_prec_round(prec, rm);
            assert!(c.is_nan());
            assert_eq!(o, Equal);
        }
        for (x, out) in [
            (Float::INFINITY, Float::ZERO),
            (Float::NEGATIVE_INFINITY, Float::NEGATIVE_ZERO),
            (Float::ONE, Float::INFINITY),
            (-Float::ONE, Float::NEGATIVE_INFINITY),
        ] {
            let (c, o) = x.acoth_prec_round(prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(out));
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn acoth_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (c, o) = x.clone().acoth_round(rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.acoth_round_ref(rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.acoth_round_assign(rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.acoth_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acoth_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn acoth_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (c, o) = x.clone().acoth_prec(prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.acoth_prec_ref(prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.acoth_prec_assign(prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (c_alt, o_alt) = x.acoth_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let (rug_c, rug_o) = rug_acoth_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn acoth_properties_helper(x: Float) {
    let c = x.clone().acoth();
    assert!(c.is_valid());
    let c_alt = (&x).acoth();
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

    let mut x_alt = x.clone();
    x_alt.acoth_assign();
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));

    let c_alt = x.acoth_prec_round_ref(x.significant_bits(), Nearest).0;
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

    assert_eq!(
        ComparableFloatRef(&<Float as From<&rug::Float>>::from(&rug_acoth(
            &rug::Float::exact_from(&x)
        ))),
        ComparableFloatRef(&c)
    );

    assert_eq!(c.is_nan(), x.is_nan() || (x.is_finite() && x.lt_abs(&1u32)));
}

#[test]
fn acoth_properties() {
    float_gen().test_properties(|x| {
        acoth_properties_helper(x);
    });

    float_gen_var_12().test_properties(|x| {
        acoth_properties_helper(x);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_acoth() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_acoth(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, 0.0);
    test::<f32>(f32::NEGATIVE_INFINITY, -0.0);
    test::<f32>(0.0, f32::NAN);
    test::<f32>(-0.0, f32::NAN);
    test::<f32>(1.0, f32::INFINITY);
    test::<f32>(-1.0, f32::NEGATIVE_INFINITY);
    test::<f32>(0.5, f32::NAN);
    test::<f32>(1.5, 0.804719);
    test::<f32>(2.0, 0.54930615);
    test::<f32>(-10.0, -0.100335345);
    test::<f32>(1.0000001, 8.317766);
    test::<f32>(1.0e30, 1.0e-30);
    test::<f32>(3.4028235e38, 2.938736e-39);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, 0.0);
    test::<f64>(f64::NEGATIVE_INFINITY, -0.0);
    test::<f64>(0.0, f64::NAN);
    test::<f64>(-0.0, f64::NAN);
    test::<f64>(1.0, f64::INFINITY);
    test::<f64>(-1.0, f64::NEGATIVE_INFINITY);
    test::<f64>(0.5, f64::NAN);
    test::<f64>(1.5, 0.8047189562170501);
    test::<f64>(2.0, 0.5493061443340549);
    test::<f64>(-10.0, -0.10033534773107558);
    test::<f64>(1.0000000000000002, 18.36840028483855);
    test::<f64>(1.0e300, 1.0e-300);
    test::<f64>(1.7976931348623157e308, 5.562684646268003e-309);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acoth_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let c = primitive_float_acoth(x);
        // acoth is NaN exactly for NaN and every x with |x| < 1, and odd
        assert_eq!(c.is_nan(), x.is_nan() || x.abs() < T::ONE);
        assert_eq!(NiceFloat(primitive_float_acoth(-x)), NiceFloat(-c));
        if x.is_finite() && x.abs() > T::ONE {
            // the result is the correctly rounded inverse hyperbolic cotangent as given by the
            // oracle and rounded once to the primitive type
            let rug_c: T = round_once_to_primitive(|p| {
                <Float as From<&rug::Float>>::from(
                    &rug_acoth_prec(&rug::Float::exact_from(&Float::from(x)), p).0,
                )
            });
            assert_eq!(NiceFloat(rug_c), NiceFloat(c));
        }
    });
}

#[test]
fn primitive_float_acoth_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acoth_properties_helper);
}

#[test]
fn test_acoth_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::acoth_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        let (c, o) = Float::acoth_rational_prec_round_ref(&x, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acoth_rational_prec_round(&x, prec, rug_rm);
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
    test("2", 1, Floor, "0.50", "0x0.8#1", Less);
    test("2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("2", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("2", 10, Floor, "0.54883", "0x0.8c8#10", Less);
    test("2", 10, Ceiling, "0.54980", "0x0.8cc#10", Greater);
    test("2", 10, Nearest, "0.54883", "0x0.8c8#10", Less);
    test(
        "2",
        100,
        Floor,
        "0.54930614433405484569762261846113",
        "0x0.8c9f53d5681854bb520cc6aa8#100",
        Less,
    );
    test(
        "2",
        100,
        Ceiling,
        "0.54930614433405484569762261846192",
        "0x0.8c9f53d5681854bb520cc6aa9#100",
        Greater,
    );
    test(
        "2",
        100,
        Nearest,
        "0.54930614433405484569762261846113",
        "0x0.8c9f53d5681854bb520cc6aa8#100",
        Less,
    );
    test("-3/2", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-3/2", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-3/2", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-3/2", 10, Floor, "-0.80566", "-0x0.ce4#10", Less);
    test("-3/2", 10, Ceiling, "-0.80469", "-0x0.ce0#10", Greater);
    test("-3/2", 10, Nearest, "-0.80469", "-0x0.ce0#10", Greater);
    test(
        "-3/2",
        100,
        Floor,
        "-0.80471895621705018730037966661370",
        "-0x0.ce020fbf6c699b57efbbd28b1#100",
        Less,
    );
    test(
        "-3/2",
        100,
        Ceiling,
        "-0.80471895621705018730037966661291",
        "-0x0.ce020fbf6c699b57efbbd28b0#100",
        Greater,
    );
    test(
        "-3/2",
        100,
        Nearest,
        "-0.80471895621705018730037966661291",
        "-0x0.ce020fbf6c699b57efbbd28b0#100",
        Greater,
    );
    test("22/7", 1, Floor, "0.25", "0x0.4#1", Less);
    test("22/7", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("22/7", 1, Nearest, "0.25", "0x0.4#1", Less);
    test("22/7", 10, Floor, "0.32959", "0x0.546#10", Less);
    test("22/7", 10, Ceiling, "0.33008", "0x0.548#10", Greater);
    test("22/7", 10, Nearest, "0.32959", "0x0.546#10", Less);
    test(
        "22/7",
        100,
        Floor,
        "0.32962281444213198059363373110635",
        "0x0.546229280b5ffbc3284155d018#100",
        Less,
    );
    test(
        "22/7",
        100,
        Ceiling,
        "0.32962281444213198059363373110675",
        "0x0.546229280b5ffbc3284155d020#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "0.32962281444213198059363373110675",
        "0x0.546229280b5ffbc3284155d020#100",
        Greater,
    );
    test("1000001/1000000", 1, Floor, "4.0", "0x4.0#1", Less);
    test("1000001/1000000", 1, Ceiling, "8.0", "0x8.0#1", Greater);
    test("1000001/1000000", 1, Nearest, "8.0", "0x8.0#1", Greater);
    test("1000001/1000000", 10, Floor, "7.2500", "0x7.40#10", Less);
    test(
        "1000001/1000000",
        10,
        Ceiling,
        "7.2578",
        "0x7.42#10",
        Greater,
    );
    test(
        "1000001/1000000",
        10,
        Nearest,
        "7.2578",
        "0x7.42#10",
        Greater,
    );
    test(
        "1000001/1000000",
        100,
        Floor,
        "7.2543291192620472067834237503017",
        "0x7.411bb691a6a663cb258598eb0#100",
        Less,
    );
    test(
        "1000001/1000000",
        100,
        Ceiling,
        "7.2543291192620472067834237503080",
        "0x7.411bb691a6a663cb258598eb8#100",
        Greater,
    );
    test(
        "1000001/1000000",
        100,
        Nearest,
        "7.2543291192620472067834237503017",
        "0x7.411bb691a6a663cb258598eb0#100",
        Less,
    );
    test(
        "-100000000000000000000",
        1,
        Floor,
        "-1.4e-20",
        "-0x4.0E-17#1",
        Less,
    );
    test(
        "-100000000000000000000",
        1,
        Ceiling,
        "-6.8e-21",
        "-0x2.0E-17#1",
        Greater,
    );
    test(
        "-100000000000000000000",
        1,
        Nearest,
        "-6.8e-21",
        "-0x2.0E-17#1",
        Greater,
    );
    test(
        "-100000000000000000000",
        10,
        Floor,
        "-1.0006e-20",
        "-0x2.f4E-17#10",
        Less,
    );
    test(
        "-100000000000000000000",
        10,
        Ceiling,
        "-9.9923e-21",
        "-0x2.f3E-17#10",
        Greater,
    );
    test(
        "-100000000000000000000",
        10,
        Nearest,
        "-1.0006e-20",
        "-0x2.f4E-17#10",
        Less,
    );
    test(
        "-100000000000000000000",
        100,
        Floor,
        "-1.0000000000000000000000000000006e-20",
        "-0x2.f394219248446baa23d2ec72cE-17#100",
        Less,
    );
    test(
        "-100000000000000000000",
        100,
        Ceiling,
        "-9.9999999999999999999999999999955e-21",
        "-0x2.f394219248446baa23d2ec728E-17#100",
        Greater,
    );
    test(
        "-100000000000000000000",
        100,
        Nearest,
        "-9.9999999999999999999999999999955e-21",
        "-0x2.f394219248446baa23d2ec728E-17#100",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        1,
        Floor,
        "32.0",
        "0x2.0E+1#1",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        1,
        Ceiling,
        "64.0",
        "0x4.0E+1#1",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        1,
        Nearest,
        "32.0",
        "0x2.0E+1#1",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Floor,
        "35.000",
        "0x23.0#10",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Ceiling,
        "35.062",
        "0x23.1#10",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        10,
        Nearest,
        "35.000",
        "0x23.0#10",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        100,
        Floor,
        "35.003932618277238125570222133628",
        "0x23.0101ba62e36d8063536aed7c#100",
        Less,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        100,
        Ceiling,
        "35.003932618277238125570222133679",
        "0x23.0101ba62e36d8063536aed80#100",
        Greater,
    );
    test(
        "1267650600228229401496703205377/1267650600228229401496703205376",
        100,
        Nearest,
        "35.003932618277238125570222133628",
        "0x23.0101ba62e36d8063536aed7c#100",
        Less,
    );
}

#[test]
fn test_acoth_rational_extreme() {
    // x = 2^(2^30 + 10) is too large to be a `Float`; acoth(x) is just above 1/x = 2^(-2^30 - 10),
    // below half the smallest positive `Float`, so it underflows
    let x = Rational::power_of_2((1i64 << 30) + 10);
    for (rm, out, o_out) in [
        (Floor, Float::ZERO, Less),
        (Down, Float::ZERO, Less),
        (Nearest, Float::ZERO, Less),
        (Ceiling, Float::min_positive_value_prec(10), Greater),
        (Up, Float::min_positive_value_prec(10), Greater),
    ] {
        let (c, o) = Float::acoth_rational_prec_round_ref(&x, 10, rm);
        assert_eq!(ComparableFloat(c.clone()), ComparableFloat(out));
        assert_eq!(o, o_out);
        let (c_neg, o_neg) = Float::acoth_rational_prec_round(-&x, 10, -rm);
        assert_eq!(ComparableFloat(c_neg), ComparableFloat(-c));
        assert_eq!(o_neg, o.reverse());
    }
}

#[test]
#[should_panic]
fn acoth_rational_prec_fail() {
    Float::acoth_rational_prec(Rational::TWO, 0);
}

#[test]
#[should_panic]
fn acoth_rational_prec_ref_fail() {
    Float::acoth_rational_prec_ref(&Rational::TWO, 0);
}

#[test]
#[should_panic]
fn acoth_rational_prec_round_fail_1() {
    Float::acoth_rational_prec_round(Rational::TWO, 0, Floor);
}

#[test]
#[should_panic]
fn acoth_rational_prec_round_fail_2() {
    Float::acoth_rational_prec_round(Rational::TWO, 10, Exact);
}

#[test]
#[should_panic]
fn acoth_rational_prec_round_ref_fail() {
    Float::acoth_rational_prec_round_ref(&Rational::TWO, 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn acoth_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (c, o) = Float::acoth_rational_prec_round(x.clone(), prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = Float::acoth_rational_prec_round_ref(&x, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // acoth is NaN exactly for |x| < 1, ±inf at ±1, odd, and otherwise has the sign of x
    if x.lt_abs(&1u32) {
        assert!(c.is_nan());
    } else {
        let (c_neg, o_neg) = Float::acoth_rational_prec_round(-&x, prec, -rm);
        assert_eq!(ComparableFloatRef(&c_neg), ComparableFloatRef(&-&c));
        assert_eq!(o_neg, o.reverse());
        assert_eq!(c.is_sign_positive(), x > 0u32);
        assert_eq!(c.is_infinite(), x == 1u32 || x == -1i32);
    }

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_acoth_rational_prec_round(&x, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // the inverse hyperbolic cotangent of an exactly representable rational is the Float inverse
    // hyperbolic cotangent, computed by an independent algorithm
    if let Ok(f) = Float::try_from(&x)
        && (rm != Exact || o == Equal)
    {
        let (c_alt, o_alt) = f.acoth_prec_round(prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }

    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::acoth_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::acoth_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn acoth_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_18().test_properties(|(x, prec, rm)| {
        acoth_rational_prec_round_properties_helper(x, prec, rm);
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        if x.le_abs(&1u32) {
            for rm in exhaustive_rounding_modes() {
                acoth_rational_prec_round_properties_helper(x.clone(), prec, rm);
            }
        }
    });
}

#[allow(clippy::needless_pass_by_value)]
fn acoth_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (c, o) = Float::acoth_rational_prec(x.clone(), prec);
    assert!(c.is_valid());

    let (c_alt, o_alt) = Float::acoth_rational_prec_ref(&x, prec);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let (c_alt, o_alt) = Float::acoth_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let (rug_c, rug_o) = rug_acoth_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_c)),
        ComparableFloatRef(&c)
    );
    assert_eq!(rug_o, o);
}

#[test]
fn acoth_rational_prec_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_18().test_properties(|(x, prec, _)| {
        acoth_rational_prec_properties_helper(x, prec);
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        acoth_rational_prec_properties_helper(x, prec);
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_acoth_rational() {
    fn test<T: PrimitiveFloat>(x: &Rational, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_acoth_rational::<T>(x)),
            NiceFloat(out)
        );
    }
    let test_s = |s: &str| Rational::from_str(s).unwrap();
    test::<f32>(&test_s("0"), f32::NAN);
    test::<f64>(&test_s("0"), f64::NAN);
    test::<f32>(&test_s("1"), f32::INFINITY);
    test::<f64>(&test_s("1"), f64::INFINITY);
    test::<f32>(&test_s("-1"), f32::NEGATIVE_INFINITY);
    test::<f64>(&test_s("-1"), f64::NEGATIVE_INFINITY);
    test::<f32>(&test_s("1/2"), f32::NAN);
    test::<f64>(&test_s("1/2"), f64::NAN);
    test::<f32>(&test_s("2"), 0.54930615);
    test::<f64>(&test_s("2"), 0.5493061443340549);
    test::<f32>(&test_s("-3/2"), -0.804719);
    test::<f64>(&test_s("-3/2"), -0.8047189562170501);
    test::<f32>(&test_s("100000000000000000000"), 1.0e-20);
    test::<f64>(&test_s("100000000000000000000"), 1.0e-20);
    test::<f32>(
        &test_s("100000000000000000001/100000000000000000000"),
        23.372425,
    );
    test::<f64>(
        &test_s("100000000000000000001/100000000000000000000"),
        23.37242452022043,
    );

    // 1/x lies exactly halfway between two adjacent subnormals, or at half the smallest one, and
    // acoth(x) lies just beyond it, so the result rounds up
    let x = Rational::power_of_2(150i64) / Rational::from(32767u32);
    test::<f32>(&x, 2.2959e-41);
    test::<f64>(&x, 2.295817339026564e-41);
    let x = Rational::power_of_2(150i64);
    test::<f32>(&x, 1.0e-45);
    test::<f64>(&x, 7.006492321624085e-46);
    let x = Rational::power_of_2(1075i64) / Rational::from(32767u32);
    test::<f32>(&x, 0.0);
    test::<f64>(&x, 8.095e-320);
    let x = Rational::power_of_2(1075i64);
    test::<f32>(&x, 0.0);
    test::<f64>(&x, 5.0e-324);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acoth_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let c = primitive_float_acoth_rational::<T>(&x);
        // the inverse hyperbolic cotangent of a rational is NaN exactly when |x| < 1
        assert_eq!(c.is_nan(), x.lt_abs(&1u32));
        if x.gt_abs(&1u32) {
            let rug_c: T = round_once_to_primitive(|p| {
                <Float as From<&rug::Float>>::from(&rug_acoth_rational_prec(&x, p).0)
            });
            assert_eq!(NiceFloat(rug_c), NiceFloat(c));
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // The inverse hyperbolic cocotangent of a finite nonzero primitive float, taken through the
        // `Rational` path, matches the direct primitive-float inverse hyperbolic cocotangent (a
        // `Rational` cannot carry the sign of a zero).
        if x.is_finite() && x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_acoth_rational::<T>(&Rational::exact_from(
                    x
                ))),
                NiceFloat(primitive_float_acoth(x))
            );
        }
    });
}

#[test]
fn primitive_float_acoth_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acoth_rational_properties_helper);
}
