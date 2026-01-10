// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Agm, AgmAssign, Square};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::primitive_float_pair_gen;
use malachite_float::arithmetic::agm::primitive_float_agm;
use malachite_float::test_util::arithmetic::agm::{
    rug_agm, rug_agm_prec, rug_agm_prec_round, rug_agm_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_float_rounding_mode_triple_gen_var_33, float_float_rounding_mode_triple_gen_var_34,
    float_float_unsigned_rounding_mode_quadruple_gen_var_9,
    float_float_unsigned_rounding_mode_quadruple_gen_var_10, float_float_unsigned_triple_gen_var_1,
    float_float_unsigned_triple_gen_var_2, float_gen, float_gen_var_4, float_gen_var_15,
    float_pair_gen, float_pair_gen_var_10, float_rounding_mode_pair_gen,
    float_rounding_mode_pair_gen_var_32, float_rounding_mode_pair_gen_var_33,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_6, float_unsigned_pair_gen_var_7,
    float_unsigned_rounding_mode_triple_gen_var_1, float_unsigned_rounding_mode_triple_gen_var_17,
    float_unsigned_rounding_mode_triple_gen_var_18,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use std::cmp::{
    Ordering::{self, *},
    max,
};
use std::panic::catch_unwind;

#[test]
fn test_agm() {
    let test = |s, s_hex, t, t_hex, out: &str, out_hex: &str| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let agm = x.clone().agm(y.clone());
        assert!(agm.is_valid());

        assert_eq!(agm.to_string(), out);
        assert_eq!(to_hex_string(&agm), out_hex);

        let agm_alt = x.clone().agm(&y);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        let agm_alt = (&x).agm(y.clone());
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        let agm_alt = (&x).agm(&y);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));

        let mut agm_alt = x.clone();
        agm_alt.agm_assign(y.clone());
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        let mut agm_alt = x.clone();
        agm_alt.agm_assign(&y);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_agm(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y)
            ))),
            ComparableFloatRef(&agm),
        );
    };
    test("NaN", "NaN", "NaN", "NaN", "NaN", "NaN");
    test("NaN", "NaN", "Infinity", "Infinity", "NaN", "NaN");
    test("NaN", "NaN", "-Infinity", "-Infinity", "NaN", "NaN");
    test("NaN", "NaN", "0.0", "0x0.0", "NaN", "NaN");
    test("NaN", "NaN", "-0.0", "-0x0.0", "NaN", "NaN");

    test("Infinity", "Infinity", "NaN", "NaN", "NaN", "NaN");
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
    );
    test("Infinity", "Infinity", "0.0", "0x0.0", "NaN", "NaN");
    test("Infinity", "Infinity", "-0.0", "-0x0.0", "NaN", "NaN");

    test("-Infinity", "-Infinity", "NaN", "NaN", "NaN", "NaN");
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        "NaN",
        "NaN",
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
    );
    test("-Infinity", "-Infinity", "0.0", "0x0.0", "NaN", "NaN");
    test("-Infinity", "-Infinity", "-0.0", "-0x0.0", "NaN", "NaN");

    test("0.0", "0x0.0", "NaN", "NaN", "NaN", "NaN");
    test("0.0", "0x0.0", "Infinity", "Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "-Infinity", "-Infinity", "NaN", "NaN");
    test("0.0", "0x0.0", "0.0", "0x0.0", "0.0", "0x0.0");
    test("0.0", "0x0.0", "-0.0", "-0x0.0", "0.0", "0x0.0");

    test("-0.0", "-0x0.0", "NaN", "NaN", "NaN", "NaN");
    test("-0.0", "-0x0.0", "Infinity", "Infinity", "NaN", "NaN");
    test("-0.0", "-0x0.0", "-Infinity", "-Infinity", "NaN", "NaN");
    test("-0.0", "-0x0.0", "0.0", "0x0.0", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", "0.0", "0x0.0");

    test("123.0", "0x7b.0#7", "NaN", "NaN", "NaN", "NaN");
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", "Infinity", "Infinity",
    );
    test("123.0", "0x7b.0#7", "-Infinity", "-Infinity", "NaN", "NaN");
    test("123.0", "0x7b.0#7", "0.0", "0x0.0", "0.0", "0x0.0");
    test("123.0", "0x7b.0#7", "-0.0", "-0x0.0", "0.0", "0x0.0");

    test("-123.0", "-0x7b.0#7", "NaN", "NaN", "NaN", "NaN");
    test("-123.0", "-0x7b.0#7", "Infinity", "Infinity", "NaN", "NaN");
    test(
        "-123.0",
        "-0x7b.0#7",
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
    );
    test("-123.0", "-0x7b.0#7", "0.0", "0x0.0", "0.0", "0x0.0");
    test("-123.0", "-0x7b.0#7", "-0.0", "-0x0.0", "0.0", "0x0.0");

    test("NaN", "NaN", "123.0", "0x7b.0#7", "NaN", "NaN");
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", "Infinity", "Infinity",
    );
    test("-Infinity", "-Infinity", "123.0", "0x7b.0#7", "NaN", "NaN");
    test("0.0", "0x0.0", "123.0", "0x7b.0#7", "0.0", "0x0.0");
    test("-0.0", "-0x0.0", "123.0", "0x7b.0#7", "0.0", "0x0.0");

    test("1.0", "0x1.0#1", "2.0", "0x2.0#1", "1.0", "0x1.0#1");
    test("1.0", "0x1.0#1", "2.0", "0x2.0#2", "1.5", "0x1.8#2");
    test("1.0", "0x1.0#2", "2.0", "0x2.0#1", "1.5", "0x1.8#2");
    test("1.0", "0x1.0#2", "2.0", "0x2.0#2", "1.5", "0x1.8#2");
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        "1.457",
        "0x1.750#10",
    );
    test(
        "24.0",
        "0x18.000000000000000000000000#100",
        "6.0",
        "0x6.0000000000000000000000000#100",
        "13.45817148172561542076681315698",
        "0xd.754ab9e9f8ac5a0692360241#100",
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "2.1920339783176708",
        "0x2.31292388985d0#53",
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "NaN",
        "NaN",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "NaN",
        "NaN",
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "NaN",
        "NaN",
    );

    test(
        "too_big",
        "0x4.0E+268435455#1",
        "too_big",
        "0x4.0E+268435455#1",
        "too_big",
        "0x4.0E+268435455#1",
    );
    test(
        "too_big",
        "0x4.0E+268435455#1",
        "1.0",
        "0x1.0#1",
        "too_big",
        "0x2.0E+268435448#1",
    );
    test(
        "too_small",
        "0x1.0E-268435456#1",
        "too_small",
        "0x1.0E-268435456#1",
        "too_small",
        "0x1.0E-268435456#1",
    );
    test(
        "too_small",
        "0x1.0E-268435456#1",
        "1.0",
        "0x1.0#1",
        "2.0e-9",
        "0x8.0E-8#1",
    );
    test(
        "too_big",
        "0x4.0E+268435455#1",
        "too_small",
        "0x1.0E-268435456#1",
        "too_big",
        "0x1.0E+268435448#1",
    );
    test(
        "too_small",
        "0x7.184a0f216320E-268435451#48",
        "1.0e-7",
        "0x2.0E-6#2",
        "2.51596539536877e-16",
        "0x1.2212310c5964E-13#48",
    );
}

#[test]
fn test_agm_prec() {
    let test = |s, s_hex, t, t_hex, prec: u64, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (agm, o) = x.clone().agm_prec(y.clone(), prec);
        assert!(agm.is_valid());

        assert_eq!(agm.to_string(), out);
        assert_eq!(to_hex_string(&agm), out_hex);
        assert_eq!(o, o_out);

        let (agm_alt, o_alt) = x.clone().agm_prec_val_ref(&y, prec);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        let (agm_alt, o_alt) = x.agm_prec_ref_val(y.clone(), prec);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        let (agm_alt, o_alt) = x.agm_prec_ref_ref(&y, prec);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        let mut agm_alt = x.clone();
        let o_alt = agm_alt.agm_prec_assign(y.clone(), prec);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        let mut agm_alt = x.clone();
        let o_alt = agm_alt.agm_prec_assign_ref(&y, prec);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        let (rug_agm, rug_o) = rug_agm_prec(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_agm)),
            ComparableFloatRef(&agm)
        );
        assert_eq!(rug_o, o);
    };
    test("NaN", "NaN", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", "Infinity", "Infinity", 1, "NaN", "NaN", Equal);
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test("NaN", "NaN", "0.0", "0x0.0", 1, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", 1, "NaN", "NaN", Equal);

    test("Infinity", "Infinity", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, "NaN", "NaN", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        "NaN",
        "NaN",
        Equal,
    );

    test("0.0", "0x0.0", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, "NaN", "NaN", Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", 1, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "-0.0", "-0x0.0", 1, "0.0", "0x0.0", Equal);

    test("-0.0", "-0x0.0", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, "NaN", "NaN", Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test("-0.0", "-0x0.0", "0.0", "0x0.0", 1, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, "0.0", "0x0.0", Equal);

    test("123.0", "0x7b.0#7", "NaN", "NaN", 1, "NaN", "NaN", Equal);
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", 1, "0.0", "0x0.0", Equal,
    );

    test("NaN", "NaN", "123.0", "0x7b.0#7", 1, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, "Infinity", "Infinity", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", 1, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", 1, "0.0", "0x0.0", Equal,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 2, "1.5", "0x1.8#2", Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        "1.457",
        "0x1.750#10",
        Greater,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 2, "1.5", "0x1.8#2", Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        "1.457",
        "0x1.750#10",
        Greater,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 2, "1.5", "0x1.8#2", Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        "1.457",
        "0x1.750#10",
        Greater,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 2, "1.5", "0x1.8#2", Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        "1.457",
        "0x1.750#10",
        Greater,
    );

    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        2,
        "1.5",
        "0x1.8#2",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        "1.457",
        "0x1.750#10",
        Greater,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "2.191",
        "0x2.31#10",
        Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        1,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        1,
        "0.1",
        "0x0.2#1",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "0.0002",
        "0x0.001#1",
        20,
        "0.16187",
        "0x0.297050#20",
        Less,
    );
    // - in agm_prec_round_ref_ref_normal
    // - *a >= 0u32 && *b >= 0u32 in agm_prec_round_ref_ref_normal
    // - a == b in agm_prec_round_ref_ref_normal
    test(
        "1.0", "0x1.0#1", "1.0", "0x1.0#1", 1, "1.0", "0x1.0#1", Equal,
    );
    // - *a < 0u32 || *b < 0u32 in agm_prec_round_ref_ref_normal
    test("1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 1, "NaN", "NaN", Equal);
    // - a < b in agm_prec_round_ref_ref_normal
    // - !u_overflow && !u_underflow && !v_overflow && !v_underflow in agm_prec_round_ref_ref_normal
    // - cmp != Equal in agm_prec_round_ref_ref_normal
    // - eq <= p / 4 in agm_prec_round_ref_ref_normal
    // - !uf.is_infinite() in agm_prec_round_ref_ref_normal
    // - eq > p / 4 in agm_prec_round_ref_ref_normal
    // - !underflow in agm_prec_round_ref_ref_normal
    // - terminate Ziv in agm_prec_round_ref_ref_normal
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, "1.0", "0x1.0#1", Less,
    );
    // - a > b in agm_prec_round_ref_ref_normal
    test(
        "2.0", "0x2.0#1", "1.0", "0x1.0#1", 1, "1.0", "0x1.0#1", Less,
    );
    // - don't terminate Ziv in agm_prec_round_ref_ref_normal
    test(
        "3.0379e20",
        "0x1.0780E+17#14",
        "6.0e1",
        "0x4.0E+1#3",
        14,
        "1.075e19",
        "0x9.530E+15#14",
        Less,
    );
    // - uf.is_infinite() in agm_prec_round_ref_ref_normal
    test(
        "1.0",
        "0x1.0#1",
        "too_big",
        "0x4.0E+268435455#1",
        1,
        "too_big",
        "0x2.0E+268435448#1",
        Less,
    );
    // - u_overflow || u_underflow || v_overflow || v_underflow in agm_prec_round_ref_ref_normal
    // - u_overflow || v_overflow in agm_prec_round_ref_ref_normal
    // - e1 + e2 > Float::MAX_EXPONENT in agm_prec_round_ref_ref_normal
    test(
        "2.0",
        "0x2.0#1",
        "too_big",
        "0x4.0E+268435455#1",
        1,
        "too_big",
        "0x2.0E+268435448#1",
        Less,
    );
    // - !u_overflow && !v_overflow in agm_prec_round_ref_ref_normal
    test(
        "0.5",
        "0x0.8#1",
        "too_small",
        "0x1.0E-268435456#1",
        1,
        "9.0e-10",
        "0x4.0E-8#1",
        Less,
    );
    // - underflow in agm_prec_round_ref_ref_normal
    test(
        "too_small",
        "0x1.0E-268435456#1",
        "too_small",
        "0x1.8E-268435456#2",
        2,
        "too_small",
        "0x1.0E-268435456#2",
        Less,
    );
    // - e1 + e2 <= Float::MAX_EXPONENT in agm_prec_round_ref_ref_normal
    test(
        "too_big",
        "0x7.ffffffe3ffffffffeE+268435455#70",
        "too_small",
        "0x1.fffffffc1ffffffffffff1ffffe0000000000003ffffffffc00000000000000000000001fffffffffffff\
        ffffffffffff800000000E-268435442#424",
        7,
        "too_big",
        "0x2.48E+268435448#7",
        Greater,
    );
}

#[test]
fn agm_prec_fail() {
    assert_panic!(Float::NAN.agm_prec(Float::NAN, 0));
    assert_panic!(Float::NAN.agm_prec_val_ref(&Float::NAN, 0));
    assert_panic!(Float::NAN.agm_prec_ref_val(Float::NAN, 0));
    assert_panic!(Float::NAN.agm_prec_ref_ref(&Float::NAN, 0));
    assert_panic!({
        let mut x = Float::NAN;
        x.agm_prec_assign(Float::NAN, 0)
    });
    assert_panic!({
        let mut x = Float::NAN;
        x.agm_prec_assign_ref(&Float::NAN, 0)
    });
}

#[test]
fn test_agm_round() {
    let test = |s, s_hex, t, t_hex, rm, out: &str, out_hex: &str, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (agm, o) = x.clone().agm_round(y.clone(), rm);
        assert!(agm.is_valid());

        assert_eq!(agm.to_string(), out);
        assert_eq!(to_hex_string(&agm), out_hex);
        assert_eq!(o, o_out);

        let (agm_alt, o_alt) = x.clone().agm_round_val_ref(&y, rm);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        let (agm_alt, o_alt) = x.agm_round_ref_val(y.clone(), rm);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        let (agm_alt, o_alt) = x.agm_round_ref_ref(&y, rm);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        let mut agm_alt = x.clone();
        let o_alt = agm_alt.agm_round_assign(y.clone(), rm);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        let mut agm_alt = x.clone();
        let o_alt = agm_alt.agm_round_assign_ref(&y, rm);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);
        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_agm, rug_o) =
                rug_agm_round(&rug::Float::exact_from(&x), &rug::Float::exact_from(&y), rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_agm)),
                ComparableFloatRef(&agm),
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", "NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", Exact, "NaN", "NaN", Equal);

    test(
        "NaN", "NaN", "Infinity", "Infinity", Floor, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", Down, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", Up, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", Exact, "NaN", "NaN", Equal,
    );

    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test("NaN", "NaN", "0.0", "0x0.0", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0.0", "0x0.0", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0.0", "0x0.0", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0.0", "0x0.0", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0.0", "0x0.0", Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0.0", "0x0.0", Exact, "NaN", "NaN", Equal);

    test("NaN", "NaN", "-0.0", "-0x0.0", Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", Exact, "NaN", "NaN", Equal);

    test(
        "Infinity", "Infinity", "NaN", "NaN", Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", Exact, "NaN", "NaN", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test("0.0", "0x0.0", "NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", Exact, "NaN", "NaN", Equal);

    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Floor, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Down, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Up, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", Exact, "NaN", "NaN", Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test("0.0", "0x0.0", "0.0", "0x0.0", Floor, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", Ceiling, "0.0", "0x0.0", Equal,
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", Down, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", "0.0", "0x0.0", Up, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", Nearest, "0.0", "0x0.0", Equal,
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", Exact, "0.0", "0x0.0", Equal);

    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Down, "0.0", "0x0.0", Equal,
    );
    test("0.0", "0x0.0", "-0.0", "-0x0.0", Up, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", Exact, "0.0", "0x0.0", Equal,
    );

    test("-0.0", "-0x0.0", "NaN", "NaN", Floor, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Up, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Nearest, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", Exact, "NaN", "NaN", Equal);

    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Floor, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Down, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Up, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", Exact, "NaN", "NaN", Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Down, "0.0", "0x0.0", Equal,
    );
    test("-0.0", "-0x0.0", "0.0", "0x0.0", Up, "0.0", "0x0.0", Equal);
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Down, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Up, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", Floor, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", Ceiling, "NaN", "NaN", Equal,
    );
    test("123.0", "0x7b.0#7", "NaN", "NaN", Down, "NaN", "NaN", Equal);
    test("123.0", "0x7b.0#7", "NaN", "NaN", Up, "NaN", "NaN", Equal);
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", Exact, "NaN", "NaN", Equal,
    );

    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Down, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Up, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Down, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Up, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", Floor, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", Ceiling, "NaN", "NaN", Equal,
    );
    test("NaN", "NaN", "123.0", "0x7b.0#7", Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "123.0", "0x7b.0#7", Up, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Down, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Up, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Down, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Up, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Floor, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Ceiling, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Down, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Up, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", Nearest, "1.0", "0x1.0#1", Less,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", Floor, "1.0", "0x1.0#2", Less,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", Floor, "1.0", "0x1.0#2", Less,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Floor, "1.0", "0x1.0#2", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", Floor, "1.0", "0x1.0#2", Less,
    );

    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        Floor,
        "1.455",
        "0x1.748#10",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        Ceiling,
        "1.457",
        "0x1.750#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        Down,
        "1.455",
        "0x1.748#10",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        Up,
        "1.457",
        "0x1.750#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        Nearest,
        "1.457",
        "0x1.750#10",
        Greater,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "2.1920339783176708",
        "0x2.31292388985d0#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "2.1920339783176712",
        "0x2.31292388985d2#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "2.1920339783176708",
        "0x2.31292388985d0#53",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "2.1920339783176712",
        "0x2.31292388985d2#53",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "2.1920339783176708",
        "0x2.31292388985d0#53",
        Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        Floor,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", Floor, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", Down, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", Up, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", Nearest, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", Exact, "NaN", "NaN", Equal,
    );

    test(
        "24.0",
        "0x18.000000000000000000000000#100",
        "6.0",
        "0x6.0000000000000000000000000#100",
        Floor,
        "13.45817148172561542076681315696",
        "0xd.754ab9e9f8ac5a0692360240#100",
        Less,
    );
    test(
        "24.0",
        "0x18.000000000000000000000000#100",
        "6.0",
        "0x6.0000000000000000000000000#100",
        Ceiling,
        "13.45817148172561542076681315698",
        "0xd.754ab9e9f8ac5a0692360241#100",
        Greater,
    );
    test(
        "24.0",
        "0x18.000000000000000000000000#100",
        "6.0",
        "0x6.0000000000000000000000000#100",
        Down,
        "13.45817148172561542076681315696",
        "0xd.754ab9e9f8ac5a0692360240#100",
        Less,
    );
    test(
        "24.0",
        "0x18.000000000000000000000000#100",
        "6.0",
        "0x6.0000000000000000000000000#100",
        Up,
        "13.45817148172561542076681315698",
        "0xd.754ab9e9f8ac5a0692360241#100",
        Greater,
    );
    test(
        "24.0",
        "0x18.000000000000000000000000#100",
        "6.0",
        "0x6.0000000000000000000000000#100",
        Nearest,
        "13.45817148172561542076681315698",
        "0xd.754ab9e9f8ac5a0692360241#100",
        Greater,
    );
}

#[test]
fn agm_round_fail() {
    assert_panic!(Float::one_prec(1).agm_round(Float::two_prec(1), Exact));
    assert_panic!(Float::one_prec(1).agm_round_val_ref(&Float::two_prec(1), Exact));
    assert_panic!(Float::one_prec(1).agm_round_ref_val(Float::two_prec(1), Exact));
    assert_panic!(Float::one_prec(1).agm_round_ref_ref(&Float::two_prec(1), Exact));
}

#[test]
fn test_agm_prec_round() {
    let test = |s, s_hex, t, t_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);
        let y = parse_hex_string(t_hex);
        assert_eq!(y.to_string(), t);

        let (agm, o) = x.clone().agm_prec_round(y.clone(), prec, rm);
        assert!(agm.is_valid());

        assert_eq!(agm.to_string(), out);
        assert_eq!(to_hex_string(&agm), out_hex);
        assert_eq!(o, o_out);

        let (agm_alt, o_alt) = x.clone().agm_prec_round_val_ref(&y, prec, rm);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        let (agm_alt, o_alt) = x.agm_prec_round_ref_val(y.clone(), prec, rm);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        let (agm_alt, o_alt) = x.agm_prec_round_ref_ref(&y, prec, rm);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        let mut agm_alt = x.clone();
        let o_alt = agm_alt.agm_prec_round_assign(y.clone(), prec, rm);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        let mut agm_alt = x.clone();
        let o_alt = agm_alt.agm_prec_round_assign_ref(&y, prec, rm);
        assert!(agm_alt.is_valid());
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&agm_alt));
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_agm, rug_o) = rug_agm_prec_round(
                &rug::Float::exact_from(&x),
                &rug::Float::exact_from(&y),
                prec,
                rm,
            );
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_agm)),
                ComparableFloatRef(&agm)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", "NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", 1, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", 1, Up, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", "NaN", "NaN", 1, Exact, "NaN", "NaN", Equal);

    test(
        "NaN", "NaN", "Infinity", "Infinity", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "Infinity", "Infinity", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "NaN",
        "NaN",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test("NaN", "NaN", "0.0", "0x0.0", 1, Floor, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "0.0", "0x0.0", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test("NaN", "NaN", "0.0", "0x0.0", 1, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "0.0", "0x0.0", 1, Up, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "0.0", "0x0.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test("NaN", "NaN", "0.0", "0x0.0", 1, Exact, "NaN", "NaN", Equal);

    test(
        "NaN", "NaN", "-0.0", "-0x0.0", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "-0.0", "-0x0.0", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test("NaN", "NaN", "-0.0", "-0x0.0", 1, Down, "NaN", "NaN", Equal);
    test("NaN", "NaN", "-0.0", "-0x0.0", 1, Up, "NaN", "NaN", Equal);
    test(
        "NaN", "NaN", "-0.0", "-0x0.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "-0.0", "-0x0.0", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "NaN", "NaN", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "NaN", "NaN", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "Infinity", "Infinity", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "0.0", "0x0.0", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "Infinity", "Infinity", "-0.0", "-0x0.0", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "NaN",
        "NaN",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "Infinity",
        "Infinity",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "0.0",
        "0x0.0",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "-0.0",
        "-0x0.0",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test("0.0", "0x0.0", "NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0.0", "NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test("0.0", "0x0.0", "NaN", "NaN", 1, Down, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", "NaN", "NaN", 1, Up, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0.0", "NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal,
    );
    test("0.0", "0x0.0", "NaN", "NaN", 1, Exact, "NaN", "NaN", Equal);

    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "0.0", "0x0.0", "Infinity", "Infinity", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Down, "0.0", "0x0.0", Equal,
    );
    test("0.0", "0x0.0", "0.0", "0x0.0", 1, Up, "0.0", "0x0.0", Equal);
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "0.0", "0x0.0", 1, Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "-0.0", "-0x0.0", 1, Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "NaN", "NaN", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test("-0.0", "-0x0.0", "NaN", "NaN", 1, Down, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", "NaN", "NaN", 1, Up, "NaN", "NaN", Equal);
    test(
        "-0.0", "-0x0.0", "NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "NaN", "NaN", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-0.0", "-0x0.0", "Infinity", "Infinity", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "0.0", "0x0.0", 1, Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Nearest, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "-0.0", "-0x0.0", 1, Exact, "0.0", "0x0.0", Equal,
    );

    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "NaN", "NaN", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "Infinity", "Infinity", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "123.0",
        "0x7b.0#7",
        "-Infinity",
        "-Infinity",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal,
    );

    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "123.0", "0x7b.0#7", "-0.0", "-0x0.0", 1, Nearest, "0.0", "0x0.0", Equal,
    );

    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", 1, Floor, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", 1, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", 1, Down, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", 1, Up, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", 1, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "NaN", "NaN", "123.0", "0x7b.0#7", 1, Exact, "NaN", "NaN", Equal,
    );

    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, Down, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, Up, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", "123.0", "0x7b.0#7", 1, Exact, "Infinity", "Infinity", Equal,
    );

    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        "123.0",
        "0x7b.0#7",
        1,
        Exact,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "0.0", "0x0.0", "123.0", "0x7b.0#7", 1, Nearest, "0.0", "0x0.0", Equal,
    );

    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", 1, Floor, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", 1, Ceiling, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", 1, Down, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", 1, Up, "0.0", "0x0.0", Equal,
    );
    test(
        "-0.0", "-0x0.0", "123.0", "0x7b.0#7", 1, Nearest, "0.0", "0x0.0", Equal,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Floor, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Ceiling, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Down, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Up, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#1", 1, Nearest, "1.0", "0x1.0#1", Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Floor,
        "1.455",
        "0x1.748#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "1.457",
        "0x1.750#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Down,
        "1.455",
        "0x1.748#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Up,
        "1.457",
        "0x1.750#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "1.457",
        "0x1.750#10",
        Greater,
    );

    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Floor, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Ceiling, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Down, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Up, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#1", "2.0", "0x2.0#2", 1, Nearest, "1.0", "0x1.0#1", Less,
    );

    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Floor,
        "1.455",
        "0x1.748#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Ceiling,
        "1.457",
        "0x1.750#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Down,
        "1.455",
        "0x1.748#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Up,
        "1.457",
        "0x1.750#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        "2.0",
        "0x2.0#2",
        10,
        Nearest,
        "1.457",
        "0x1.750#10",
        Greater,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Floor, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Ceiling, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Down, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Up, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#1", 1, Nearest, "1.0", "0x1.0#1", Less,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Floor,
        "1.455",
        "0x1.748#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "1.457",
        "0x1.750#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Down,
        "1.455",
        "0x1.748#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Up,
        "1.457",
        "0x1.750#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "1.457",
        "0x1.750#10",
        Greater,
    );

    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Floor, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Ceiling, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Down, "1.0", "0x1.0#1", Less,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Up, "2.0", "0x2.0#1", Greater,
    );
    test(
        "1.0", "0x1.0#2", "2.0", "0x2.0#2", 1, Nearest, "1.0", "0x1.0#1", Less,
    );

    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Floor,
        "1.455",
        "0x1.748#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Ceiling,
        "1.457",
        "0x1.750#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Down,
        "1.455",
        "0x1.748#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Up,
        "1.457",
        "0x1.750#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#2",
        "2.0",
        "0x2.0#2",
        10,
        Nearest,
        "1.457",
        "0x1.750#10",
        Greater,
    );

    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );

    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        Floor,
        "1.455",
        "0x1.748#10",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        Ceiling,
        "1.457",
        "0x1.750#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        Down,
        "1.455",
        "0x1.748#10",
        Less,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        Up,
        "1.457",
        "0x1.750#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.000#10",
        "2.0",
        "0x2.00#10",
        10,
        Nearest,
        "1.457",
        "0x1.750#10",
        Greater,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "2.191",
        "0x2.31#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "2.195",
        "0x2.32#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "2.191",
        "0x2.31#10",
        Less,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "2.195",
        "0x2.32#10",
        Greater,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "2.191",
        "0x2.31#10",
        Less,
    );

    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Floor,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Ceiling,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Down,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Up,
        "NaN",
        "NaN",
        Equal,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );

    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 10, Floor, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 10, Ceiling, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 10, Down, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 10, Up, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "1.0", "0x1.0#1", "-1.0", "-0x1.0#1", 10, Exact, "NaN", "NaN", Equal,
    );
}

#[test]
fn agm_prec_round_fail() {
    assert_panic!(Float::one_prec(1).agm_prec_round(Float::two_prec(1), 0, Floor));
    assert_panic!(Float::one_prec(1).agm_prec_round_val_ref(&Float::two_prec(1), 0, Floor));
    assert_panic!(Float::one_prec(1).agm_prec_round_ref_val(Float::two_prec(1), 0, Floor));
    assert_panic!(Float::one_prec(1).agm_prec_round_ref_ref(&Float::two_prec(1), 0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.agm_prec_round_assign(Float::two_prec(1), 0, Floor)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.agm_prec_round_assign_ref(&Float::two_prec(1), 0, Floor)
    });

    assert_panic!(Float::one_prec(1).agm_prec_round(Float::two_prec(1), 1, Exact));
    assert_panic!(Float::one_prec(1).agm_prec_round_val_ref(&Float::two_prec(1), 1, Exact));
    assert_panic!(Float::one_prec(1).agm_prec_round_ref_val(Float::two_prec(1), 1, Exact));
    assert_panic!(Float::one_prec(1).agm_prec_round_ref_ref(&Float::two_prec(1), 1, Exact));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.agm_prec_round_assign(Float::two_prec(1), 1, Exact)
    });
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.agm_prec_round_assign_ref(&Float::two_prec(1), 1, Exact)
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_agm() {
    fn test<T: PrimitiveFloat>(x: T, y: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_agm(x, y)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN, f32::NAN);
    test::<f32>(f32::NAN, f32::INFINITY, f32::NAN);
    test::<f32>(f32::NAN, f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(f32::NAN, 0.0, f32::NAN);
    test::<f32>(f32::NAN, -0.0, f32::NAN);

    test::<f32>(f32::INFINITY, f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::INFINITY, f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(f32::INFINITY, 0.0, f32::NAN);
    test::<f32>(f32::INFINITY, -0.0, f32::NAN);

    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, 0.0, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, -0.0, f32::NAN);

    test::<f32>(0.0, f32::NAN, f32::NAN);
    test::<f32>(0.0, f32::INFINITY, f32::NAN);
    test::<f32>(0.0, f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, 0.0, 0.0);
    test::<f32>(0.0, -0.0, 0.0);

    test::<f32>(-0.0, f32::NAN, f32::NAN);
    test::<f32>(-0.0, f32::INFINITY, f32::NAN);
    test::<f32>(-0.0, f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(-0.0, 0.0, 0.0);
    test::<f32>(-0.0, -0.0, 0.0);

    test::<f32>(123.0, f32::NAN, f32::NAN);
    test::<f32>(123.0, f32::INFINITY, f32::INFINITY);
    test::<f32>(123.0, f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(123.0, 0.0, 0.0);
    test::<f32>(123.0, -0.0, 0.0);

    test::<f32>(-123.0, f32::NAN, f32::NAN);
    test::<f32>(-123.0, f32::INFINITY, f32::NAN);
    test::<f32>(-123.0, f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(-123.0, 0.0, 0.0);
    test::<f32>(-123.0, -0.0, 0.0);

    test::<f32>(f32::NAN, 123.0, f32::NAN);
    test::<f32>(f32::INFINITY, 123.0, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, 123.0, f32::NAN);
    test::<f32>(0.0, 123.0, 0.0);
    test::<f32>(-0.0, 123.0, 0.0);

    test::<f32>(1.0, 2.0, 1.456791);
    test::<f32>(24.0, 6.0, 13.458172);

    test::<f64>(f64::NAN, f64::NAN, f64::NAN);
    test::<f64>(f64::NAN, f64::INFINITY, f64::NAN);
    test::<f64>(f64::NAN, f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(f64::NAN, 0.0, f64::NAN);
    test::<f64>(f64::NAN, -0.0, f64::NAN);

    test::<f64>(f64::INFINITY, f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::INFINITY, f64::INFINITY);
    test::<f64>(f64::INFINITY, f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(f64::INFINITY, 0.0, f64::NAN);
    test::<f64>(f64::INFINITY, -0.0, f64::NAN);

    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, 0.0, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, -0.0, f64::NAN);

    test::<f64>(0.0, f64::NAN, f64::NAN);
    test::<f64>(0.0, f64::INFINITY, f64::NAN);
    test::<f64>(0.0, f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, 0.0, 0.0);
    test::<f64>(0.0, -0.0, 0.0);

    test::<f64>(-0.0, f64::NAN, f64::NAN);
    test::<f64>(-0.0, f64::INFINITY, f64::NAN);
    test::<f64>(-0.0, f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(-0.0, 0.0, 0.0);
    test::<f64>(-0.0, -0.0, 0.0);

    test::<f64>(123.0, f64::NAN, f64::NAN);
    test::<f64>(123.0, f64::INFINITY, f64::INFINITY);
    test::<f64>(123.0, f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(123.0, 0.0, 0.0);
    test::<f64>(123.0, -0.0, 0.0);

    test::<f64>(-123.0, f64::NAN, f64::NAN);
    test::<f64>(-123.0, f64::INFINITY, f64::NAN);
    test::<f64>(-123.0, f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(-123.0, 0.0, 0.0);
    test::<f64>(-123.0, -0.0, 0.0);

    test::<f64>(f64::NAN, 123.0, f64::NAN);
    test::<f64>(f64::INFINITY, 123.0, f64::INFINITY);
    test::<f64>(f64::NEGATIVE_INFINITY, 123.0, f64::NAN);
    test::<f64>(0.0, 123.0, 0.0);
    test::<f64>(-0.0, 123.0, 0.0);

    test::<f64>(1.0, 2.0, 1.4567910310469068);
    test::<f64>(24.0, 6.0, 13.458171481725616);
}

#[allow(clippy::needless_pass_by_value)]
fn agm_prec_round_properties_helper(
    x: Float,
    y: Float,
    prec: u64,
    rm: RoundingMode,
    extreme: bool,
) {
    let (agm, o) = x.clone().agm_prec_round(y.clone(), prec, rm);
    assert!(agm.is_valid());
    let (agm_alt, o_alt) = x.clone().agm_prec_round_val_ref(&y, prec, rm);
    assert!(agm_alt.is_valid());
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);
    let (agm_alt, o_alt) = x.agm_prec_round_ref_val(y.clone(), prec, rm);
    assert!(agm_alt.is_valid());
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);
    let (agm_alt, o_alt) = x.agm_prec_round_ref_ref(&y, prec, rm);
    assert!(agm_alt.is_valid());
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.agm_prec_round_assign(y.clone(), prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.agm_prec_round_assign_ref(&y, prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_agm, rug_o) = rug_agm_prec_round(
            &rug::Float::exact_from(&x),
            &rug::Float::exact_from(&y),
            prec,
            rm,
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_agm)),
            ComparableFloatRef(&agm)
        );
        assert_eq!(rug_o, o);
    }

    if x < 0u32 && y != 0u32 || y < 0u32 && x != 0u32 {
        assert!(agm.is_nan());
    }

    if agm.is_normal() && agm > 0u32 {
        let (min, max) = if x <= y { (&x, &y) } else { (&y, &x) };
        if o != Greater {
            assert!(agm <= *max);
        } else if o != Less {
            assert!(agm >= *min);
        }
        if !extreme {
            assert_eq!(agm.get_prec(), Some(prec));
            let rx = Rational::exact_from(&x);
            let ry = Rational::exact_from(&y);
            if o != Greater {
                let arith = (rx + ry) >> 1u32;
                assert!(agm <= arith);
            } else if o != Less {
                let geom_squared = rx * ry;
                assert!(Rational::exact_from(&agm).square() >= geom_squared);
            }
        }
    } else {
        assert_eq!(o, Equal);
    }

    let (agm_alt, o_alt) = y.agm_prec_round_ref_ref(&x, prec, rm);
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.agm_prec_round_ref_ref(&y, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(agm.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.agm_prec_round_ref_ref(&y, prec, Exact));
    }
}

#[test]
fn agm_prec_round_properties() {
    float_float_unsigned_rounding_mode_quadruple_gen_var_9().test_properties(|(x, y, prec, rm)| {
        agm_prec_round_properties_helper(x, y, prec, rm, false);
    });

    float_float_unsigned_rounding_mode_quadruple_gen_var_10().test_properties(
        |(x, y, prec, rm)| {
            agm_prec_round_properties_helper(x, y, prec, rm, true);
        },
    );

    float_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        if !x.is_sign_negative() {
            let (agm, o) = x.agm_prec_round_ref_ref(&x, prec, rm);
            let (agm_alt, o_alt) = Float::from_float_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
            assert_eq!(o_alt, o);
        }

        let (agm, o) = x.agm_prec_round_ref_val(Float::NAN, prec, rm);
        assert!(agm.is_nan());
        assert_eq!(o, Equal);

        let (agm, o) = Float::NAN.agm_prec_round_val_ref(&x, prec, rm);
        assert!(agm.is_nan());
        assert_eq!(o, Equal);

        let (agm, o) = x.agm_prec_round_ref_val(Float::NEGATIVE_INFINITY, prec, rm);
        assert!(agm.is_nan());
        assert_eq!(o, Equal);

        let (agm, o) = Float::NEGATIVE_INFINITY.agm_prec_round_val_ref(&x, prec, rm);
        assert!(agm.is_nan());
        assert_eq!(o, Equal);
    });

    float_unsigned_rounding_mode_triple_gen_var_17().test_properties(|(x, prec, rm)| {
        let (agm, o) = x.agm_prec_round_ref_val(Float::INFINITY, prec, rm);
        assert_eq!(agm, Float::INFINITY);
        assert_eq!(o, Equal);

        let (agm, o) = Float::INFINITY.agm_prec_round_val_ref(&x, prec, rm);
        assert_eq!(agm, Float::INFINITY);
        assert_eq!(o, Equal);
    });

    float_unsigned_rounding_mode_triple_gen_var_18().test_properties(|(x, prec, rm)| {
        let (agm, o) = x.agm_prec_round_ref_val(Float::ZERO, prec, rm);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));
        assert_eq!(o, Equal);

        let (agm, o) = Float::ZERO.agm_prec_round_val_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));
        assert_eq!(o, Equal);

        let (agm, o) = x.agm_prec_round_ref_val(Float::NEGATIVE_ZERO, prec, rm);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));
        assert_eq!(o, Equal);

        let (agm, o) = Float::NEGATIVE_ZERO.agm_prec_round_val_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn agm_prec_properties_helper(x: Float, y: Float, prec: u64, extreme: bool) {
    let (agm, o) = x.clone().agm_prec(y.clone(), prec);
    assert!(agm.is_valid());
    let (agm_alt, o_alt) = x.clone().agm_prec_val_ref(&y, prec);
    assert!(agm_alt.is_valid());
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);
    let (agm_alt, o_alt) = x.agm_prec_ref_val(y.clone(), prec);
    assert!(agm_alt.is_valid());
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);
    let (agm_alt, o_alt) = x.agm_prec_ref_ref(&y, prec);
    assert!(agm_alt.is_valid());
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.agm_prec_assign(y.clone(), prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.agm_prec_assign_ref(&y, prec);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);

    let (rug_agm, rug_o) = rug_agm_prec(
        &rug::Float::exact_from(&x),
        &rug::Float::exact_from(&y),
        prec,
    );
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_agm)),
        ComparableFloatRef(&agm)
    );
    assert_eq!(rug_o, o);

    if x < 0u32 && y != 0u32 || y < 0u32 && x != 0u32 {
        assert!(agm.is_nan());
    }

    if agm.is_normal() && agm > 0u32 {
        let (min, max) = if x <= y { (&x, &y) } else { (&y, &x) };
        if o != Greater {
            assert!(agm <= *max);
        } else if o != Less {
            assert!(agm >= *min);
        }
        assert_eq!(agm.get_prec(), Some(prec));
        if !extreme {
            let rx = Rational::exact_from(&x);
            let ry = Rational::exact_from(&y);
            if o != Greater {
                let arith = (rx + ry) >> 1u32;
                assert!(agm <= arith);
            } else if o != Less {
                let geom_squared = rx * ry;
                assert!(Rational::exact_from(&agm).square() >= geom_squared);
            }
        }
    } else {
        assert_eq!(o, Equal);
    }

    let (agm_alt, o_alt) = x.agm_prec_round_ref_ref(&y, prec, Nearest);
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);

    let (agm_alt, o_alt) = y.agm_prec_ref_ref(&x, prec);
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);
}

#[test]
fn agm_prec_properties() {
    float_float_unsigned_triple_gen_var_1().test_properties(|(x, y, prec)| {
        agm_prec_properties_helper(x, y, prec, false);
    });

    float_float_unsigned_triple_gen_var_2().test_properties(|(x, y, prec)| {
        agm_prec_properties_helper(x, y, prec, true);
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        if !x.is_sign_negative() {
            let (agm, o) = x.agm_prec_ref_ref(&x, prec);
            let (agm_alt, o_alt) = Float::from_float_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
            assert_eq!(o_alt, o);
        }

        let (agm, o) = x.agm_prec_ref_val(Float::NAN, prec);
        assert!(agm.is_nan());
        assert_eq!(o, Equal);

        let (agm, o) = Float::NAN.agm_prec_val_ref(&x, prec);
        assert!(agm.is_nan());
        assert_eq!(o, Equal);

        let (agm, o) = x.agm_prec_ref_val(Float::NEGATIVE_INFINITY, prec);
        assert!(agm.is_nan());
        assert_eq!(o, Equal);

        let (agm, o) = Float::NEGATIVE_INFINITY.agm_prec_val_ref(&x, prec);
        assert!(agm.is_nan());
        assert_eq!(o, Equal);
    });

    float_unsigned_pair_gen_var_6().test_properties(|(x, prec)| {
        let (agm, o) = x.agm_prec_ref_val(Float::INFINITY, prec);
        assert_eq!(agm, Float::INFINITY);
        assert_eq!(o, Equal);

        let (agm, o) = Float::INFINITY.agm_prec_val_ref(&x, prec);
        assert_eq!(agm, Float::INFINITY);
        assert_eq!(o, Equal);
    });

    float_unsigned_pair_gen_var_7().test_properties(|(x, prec)| {
        let (agm, o) = x.agm_prec_ref_val(Float::ZERO, prec);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));
        assert_eq!(o, Equal);

        let (agm, o) = Float::ZERO.agm_prec_val_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));
        assert_eq!(o, Equal);

        let (agm, o) = x.agm_prec_ref_val(Float::NEGATIVE_ZERO, prec);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));
        assert_eq!(o, Equal);

        let (agm, o) = Float::NEGATIVE_ZERO.agm_prec_val_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn agm_round_properties_helper(x: Float, y: Float, rm: RoundingMode, extreme: bool) {
    let (agm, o) = x.clone().agm_round(y.clone(), rm);
    assert!(agm.is_valid());
    let (agm_alt, o_alt) = x.clone().agm_round_val_ref(&y, rm);
    assert!(agm_alt.is_valid());
    assert_eq!(o_alt, o);
    let (agm_alt, o_alt) = x.agm_round_ref_val(y.clone(), rm);
    assert!(agm_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    let (agm_alt, o_alt) = x.agm_round_ref_ref(&y, rm);
    assert!(agm_alt.is_valid());
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));

    let mut x_alt = x.clone();
    let o_alt = x_alt.agm_round_assign(y.clone(), rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.agm_round_assign_ref(&y, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);

    if agm.is_normal() && agm > 0u32 {
        let (min_xy, max_xy) = if x <= y { (&x, &y) } else { (&y, &x) };
        if o != Greater {
            assert!(agm <= *max_xy);
        } else if o != Less {
            assert!(agm >= *min_xy);
        }
        if !extreme {
            assert_eq!(
                agm.get_prec(),
                Some(max(x.get_prec().unwrap(), y.get_prec().unwrap()))
            );
            let rx = Rational::exact_from(&x);
            let ry = Rational::exact_from(&y);
            if o != Greater {
                let arith = (rx + ry) >> 1u32;
                assert!(agm <= arith);
            } else if o != Less {
                let geom_squared = rx * ry;
                assert!(Rational::exact_from(&agm).square() >= geom_squared);
            }
        }
    } else {
        assert_eq!(o, Equal);
    }
    let (agm_alt, o_alt) =
        x.agm_prec_round_ref_ref(&y, max(x.significant_bits(), y.significant_bits()), rm);
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    assert_eq!(o_alt, o);

    if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_agm, rug_o) =
            rug_agm_round(&rug::Float::exact_from(&x), &rug::Float::exact_from(&y), rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_agm)),
            ComparableFloatRef(&agm)
        );
        assert_eq!(rug_o, o);
    }

    if x < 0u32 && y != 0u32 || y < 0u32 && x != 0u32 {
        assert!(agm.is_nan());
    }

    let (agm_alt, o_alt) = y.agm_round_ref_ref(&x, rm);
    assert_eq!(o_alt, o);
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = x.agm_round_ref_ref(&y, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(agm.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.agm_round_ref_ref(&y, Exact));
    }
}

#[test]
fn agm_round_properties() {
    float_float_rounding_mode_triple_gen_var_33().test_properties(|(x, y, rm)| {
        agm_round_properties_helper(x, y, rm, false);
    });

    float_float_rounding_mode_triple_gen_var_34().test_properties(|(x, y, rm)| {
        agm_round_properties_helper(x, y, rm, true);
    });

    float_rounding_mode_pair_gen().test_properties(|(x, rm)| {
        if !x.is_sign_negative() {
            let (agm, o) = x.agm_round_ref_ref(&x, rm);
            assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&x));
            assert_eq!(o, Equal);
        }

        let (agm, o) = x.agm_round_ref_val(Float::NAN, rm);
        assert!(agm.is_nan());
        assert_eq!(o, Equal);

        let (agm, o) = Float::NAN.agm_round_val_ref(&x, rm);
        assert!(agm.is_nan());
        assert_eq!(o, Equal);

        let (agm, o) = x.agm_round_ref_val(Float::NEGATIVE_INFINITY, rm);
        assert!(agm.is_nan());
        assert_eq!(o, Equal);

        let (agm, o) = Float::NEGATIVE_INFINITY.agm_round_val_ref(&x, rm);
        assert!(agm.is_nan());
        assert_eq!(o, Equal);
    });

    float_rounding_mode_pair_gen_var_32().test_properties(|(x, rm)| {
        let (agm, o) = x.agm_round_ref_val(Float::INFINITY, rm);
        assert_eq!(agm, Float::INFINITY);
        assert_eq!(o, Equal);

        let (agm, o) = Float::INFINITY.agm_round_val_ref(&x, rm);
        assert_eq!(agm, Float::INFINITY);
        assert_eq!(o, Equal);
    });

    float_rounding_mode_pair_gen_var_33().test_properties(|(x, rm)| {
        let (agm, o) = x.agm_round_ref_val(Float::ZERO, rm);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));
        assert_eq!(o, Equal);

        let (agm, o) = Float::ZERO.agm_round_val_ref(&x, rm);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));
        assert_eq!(o, Equal);

        let (agm, o) = x.agm_round_ref_val(Float::NEGATIVE_ZERO, rm);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));
        assert_eq!(o, Equal);

        let (agm, o) = Float::NEGATIVE_ZERO.agm_round_val_ref(&x, rm);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn agm_properties_helper(x: Float, y: Float) {
    let agm = x.clone().agm(y.clone());
    assert!(agm.is_valid());
    let agm_alt = x.clone().agm(&y);
    assert!(agm_alt.is_valid());
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    let agm_alt = (&x).agm(y.clone());
    assert!(agm_alt.is_valid());
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    let agm_alt = (&x).agm(&y);
    assert!(agm_alt.is_valid());
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));

    let mut x_alt = x.clone();
    x_alt.agm_assign(y.clone());
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&agm));

    let mut x_alt = x.clone();
    x_alt.agm_assign(&y);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&agm));

    let agm_alt = x
        .agm_prec_round_ref_ref(&y, max(x.significant_bits(), y.significant_bits()), Nearest)
        .0;
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    let agm_alt = x
        .agm_prec_ref_ref(&y, max(x.significant_bits(), y.significant_bits()))
        .0;
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
    let agm_alt = x.agm_round_ref_ref(&y, Nearest).0;
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));

    let rug_agm = rug_agm(&rug::Float::exact_from(&x), &rug::Float::exact_from(&y));
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_agm)),
        ComparableFloatRef(&agm),
    );

    if x < 0u32 && y != 0u32 || y < 0u32 && x != 0u32 {
        assert!(agm.is_nan());
    }

    let agm_alt = y.agm(x);
    assert_eq!(ComparableFloatRef(&agm_alt), ComparableFloatRef(&agm));
}

#[test]
fn agm_properties() {
    float_pair_gen().test_properties(|(x, y)| {
        agm_properties_helper(x, y);
    });

    float_pair_gen_var_10().test_properties(|(x, y)| {
        agm_properties_helper(x, y);
    });

    float_gen().test_properties(|x| {
        if !x.is_sign_negative() {
            assert_eq!(ComparableFloatRef(&(&x).agm(&x)), ComparableFloatRef(&x));
        }

        let agm = (&x).agm(Float::NAN);
        assert!(agm.is_nan());

        let agm = Float::NAN.agm(&x);
        assert!(agm.is_nan());

        let agm = (&x).agm(Float::NEGATIVE_INFINITY);
        assert!(agm.is_nan());

        let agm = Float::NEGATIVE_INFINITY.agm(&x);
        assert!(agm.is_nan());
    });

    float_gen_var_15().test_properties(|x| {
        let agm = (&x).agm(Float::INFINITY);
        assert_eq!(agm, Float::INFINITY);

        let agm = Float::INFINITY.agm(&x);
        assert_eq!(agm, Float::INFINITY);
    });

    float_gen_var_4().test_properties(|x| {
        let agm = (&x).agm(Float::ZERO);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));

        let agm = Float::ZERO.agm(&x);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));

        let agm = (&x).agm(Float::NEGATIVE_ZERO);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));

        let agm = Float::NEGATIVE_ZERO.agm(&x);
        assert_eq!(ComparableFloatRef(&agm), ComparableFloatRef(&Float::ZERO));
    });
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_agm_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_pair_gen::<T>().test_properties(|(x, y)| {
        primitive_float_agm(x, y);
    });
}

#[test]
fn primitive_float_agm_properties() {
    apply_fn_to_primitive_floats!(primitive_float_agm_properties_helper);
}
