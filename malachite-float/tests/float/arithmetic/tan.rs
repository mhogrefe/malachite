// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{PowerOf2, Tan, TanAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_unsigned_pair_gen_var_1,
    unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::tan::{
    primitive_float_tan, primitive_float_tan_rational, primitive_float_tan_with_period,
    primitive_float_tan_with_period_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::tan::{
    rug_tan, rug_tan_prec, rug_tan_prec_round, rug_tan_rational_prec, rug_tan_rational_prec_round,
    rug_tan_round, rug_tan_with_period_prec, rug_tan_with_period_prec_round,
    rug_tan_with_period_rational_prec, rug_tan_with_period_rational_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36, float_unsigned_rounding_mode_triple_gen_var_39,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_18,
    float_unsigned_unsigned_triple_gen_var_1, rational_unsigned_rounding_mode_triple_gen_var_10,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{
    rational_gen, rational_unsigned_pair_gen_var_1, rational_unsigned_pair_gen_var_3,
};
use std::panic::catch_unwind;
use std::str::FromStr;

// Rows reuse the sine test's inputs. Branches of `tan_prec_round_normal_ref` covered:
// - tiny x: the small-input shortcut rounds x directly
// - the general Ziv loop, at the first working precision and after a retry
// - |x| near an odd multiple of pi/2: a large result, from the cosine's near-zero path inside
//   `sin_cos`
// - |x| near a nonzero multiple of pi: a tiny result, from the sine's near-zero path
#[test]
fn test_tan_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (t, o) = x.clone().tan_prec_round(prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.tan_prec_round_ref(prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.tan_prec_round_assign(prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) = rug_tan_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Floor, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Floor, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Floor, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Floor, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", 1, Exact, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Exact, "-0.0", "-0x0.0", Equal);
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Nearest,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Floor,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Ceiling,
        "0.50",
        "0x0.8#1",
        Greater,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        2,
        Nearest,
        "0.25",
        "0x0.4#2",
        Less,
    );
    test("0.25", "0x0.4#1", 1, Floor, "0.25", "0x0.4#1", Less);
    test("0.25", "0x0.4#1", 1, Nearest, "0.25", "0x0.4#1", Less);
    test("0.25", "0x0.4#1", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", 10, Floor, "1.5566", "0x1.8e8#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "1.5586",
        "0x1.8f0#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 10, Nearest, "1.5566", "0x1.8e8#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        100,
        Floor,
        "1.5574077246549022305069748074575",
        "0x1.8eb245cbee3a5b8acc7d41322#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Ceiling,
        "1.5574077246549022305069748074591",
        "0x1.8eb245cbee3a5b8acc7d41324#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Nearest,
        "1.5574077246549022305069748074591",
        "0x1.8eb245cbee3a5b8acc7d41324#100",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-1.5566",
        "-0x1.8e8#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "-2.1836",
        "-0x2.2f#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "-0.14258",
        "-0x0.248#10",
        Less,
    );
    test(
        "4.0",
        "0x4.0#1",
        10,
        Nearest,
        "1.1582",
        "0x1.288#10",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        100,
        Nearest,
        "1.1578212823495775831373424182673",
        "0x1.2866f9be4de1370db90786070#100",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        10,
        Nearest,
        "-0.58691",
        "-0x0.964#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Floor,
        "-0.58721391515692907667780963564527",
        "-0x0.9653a6b15ae9bd7c866895de5#100",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Ceiling,
        "-0.58721391515692907667780963564448",
        "-0x0.9653a6b15ae9bd7c866895de4#100",
        Greater,
    );
    test(
        "1.00000e6",
        "0xf.424E+4#14",
        64,
        Nearest,
        "-0.373624453987599029173",
        "-0x0.5fa5da2adcd300420#64",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        50,
        Nearest,
        "0.54630248984379026",
        "0x0.8bda7adf9a3a4#50",
        Less,
    );
    test(
        "0.102",
        "0x0.1a#4",
        50,
        Nearest,
        "0.10191315059274786",
        "0x0.1a16faf0d40348#50",
        Greater,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        50,
        Nearest,
        "1.0004441719502211e-10",
        "0x6.e00000000000E-9#50",
        Less,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        10,
        Floor,
        "1.0004e-10",
        "0x6.e0E-9#10",
        Less,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        10,
        Ceiling,
        "1.0016e-10",
        "0x6.e2E-9#10",
        Greater,
    );
    test(
        "1.570796326794896600",
        "0x1.921fb54442d183#57",
        53,
        Nearest,
        "51023072468558680.0",
        "0xb54536cdd66358.0#53",
        Less,
    );
    test(
        "3.14159265358979289",
        "0x3.243f6a8885a2f#54",
        53,
        Nearest,
        "-3.4450928483976660e-16",
        "-0x1.8d313198a2e03E-13#53",
        Greater,
    );
    test(
        "6.28318530717958579",
        "0x6.487ed5110b45e#54",
        53,
        Nearest,
        "-6.8901856967953321e-16",
        "-0x3.1a62633145c06E-13#53",
        Greater,
    );
    test(
        "9.979e99",
        "0x1.24E+83#7",
        53,
        Nearest,
        "2.1007816758467186",
        "0x2.19ccd3f1cc380#53",
        Less,
    );
    test("3.0", "0x3.0#2", 1, Nearest, "-0.12", "-0x0.2#1", Greater);
    test("3.0", "0x3.0#2", 1, Floor, "-0.25", "-0x0.4#1", Less);
    test("3.0", "0x3.0#2", 1, Ceiling, "-0.12", "-0x0.2#1", Greater);
    test("3.0", "0x3.0#2", 2, Nearest, "-0.12", "-0x0.2#2", Greater);
    test("0.25", "0x0.4#1", 1, Down, "0.25", "0x0.4#1", Less);
    test("1.0", "0x1.0#1", 1, Down, "1.0", "0x1.0#1", Less);
    test("2.0", "0x2.0#1", 1, Down, "-2.0", "-0x2.0#1", Greater);
    test("4.0", "0x4.0#1", 1, Down, "1.0", "0x1.0#1", Less);
    test(
        "-3.495934488151859089160804055e56",
        "-0xe.41ed086a5791d9e5b2924E+46#87",
        2,
        Down,
        "0.12",
        "0x0.2#2",
        Less,
    );
    test(
        "6.28318536",
        "0x6.487ed6#26",
        10,
        Nearest,
        "5.5647e-8",
        "0xe.f0E-7#10",
        Greater,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        53,
        Nearest,
        "16331239353195370.0",
        "0x3a052cf8639b6a.0#53",
        Greater,
    );
    test(
        "1.5707963267948966192313216916397514420985846996875529104874722",
        "0x1.921fb54442d18469898cc51701b839a252049c1114cf98e804#200",
        100,
        Floor,
        "1.7513231474107421355380165345084e61",
        "0xa.e604cc23ab94bbb749668cfdE+50#100",
        Less,
    );
    test(
        "4.7123889803846898576939650749192543286",
        "0x4.b65f1fccc8748d3c9ca64f450528b0#120",
        120,
        Ceiling,
        "-429086628331787135927979170061126206.50",
        "-0x52a39aa81dcd0e808d7147ed108a3e.8#120",
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        64,
        Nearest,
        "5.01655761266833202345e-20",
        "0xe.ce675d1fc8f8cbbE-17#64",
        Less,
    );
    test(
        "2.5",
        "0x2.8#3",
        10,
        Nearest,
        "-0.74707",
        "-0x0.bf4#10",
        Less,
    );
    test("-2.5", "-0x2.8#3", 10, Floor, "0.74609", "0x0.bf0#10", Less);
    test(
        "2.99976",
        "0x2.fff#14",
        20,
        Nearest,
        "-0.14279556",
        "-0x0.248e40#20",
        Greater,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        10,
        Ceiling,
        "0.14258",
        "0x0.248#10",
        Greater,
    );
    test(
        "6.28318548",
        "0x6.487ed8#24",
        10,
        Nearest,
        "1.7486e-7",
        "0x2.efE-6#10",
        Greater,
    );
    test(
        "6.28318548",
        "0x6.487ed8#24",
        10,
        Floor,
        "1.7462e-7",
        "0x2.eeE-6#10",
        Less,
    );
    test(
        "6.283185307179586476925286766559005788",
        "0x6.487ed5110b4611a62633145c06e10#117",
        100,
        Nearest,
        "1.9156791836247964809656913737618e-35",
        "0x1.976b7ed8fbbacc19c5fefa20aE-29#100",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Floor,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Ceiling,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Down,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Up,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Floor,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Ceiling,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Down,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Up,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Greater,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Floor,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Less,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Ceiling,
        "9.5e-323228497",
        "0x4.0E-268435456#1",
        Greater,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Down,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Less,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Up,
        "9.5e-323228497",
        "0x4.0E-268435456#1",
        Greater,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Nearest,
        "9.5e-323228497",
        "0x4.0E-268435456#1",
        Greater,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Floor,
        "-9.5e-323228497",
        "-0x4.0E-268435456#1",
        Less,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Ceiling,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Greater,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Down,
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Greater,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Up,
        "-9.5e-323228497",
        "-0x4.0E-268435456#1",
        Less,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Nearest,
        "-9.5e-323228497",
        "-0x4.0E-268435456#1",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Floor,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Less,
    );
}

#[test]
#[should_panic]
fn tan_prec_round_fail() {
    Float::ONE.tan_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn tan_prec_round_exact_fail() {
    Float::ONE.tan_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn tan_prec_fail() {
    Float::ONE.tan_prec(0);
}

#[test]
#[should_panic]
fn tan_round_fail() {
    Float::ONE.tan_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn tan_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (s, o) = x.clone().tan_prec_round(prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = x.tan_prec_round_ref(prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.tan_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_o) = rug_tan_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    }

    // tan is odd
    let (s_neg, o_neg) = (-&x).tan_prec_round(prec, -rm);
    assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
    assert_eq!(o_neg, o.reverse());

    if s.is_normal() {
        assert_eq!(s.get_prec(), Some(prec));
    }

    if o == Equal {
        // tan is exact only for x = 0 (and NaN, and ±inf): the result is rounding-mode-invariant
        if x.is_finite() {
            assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&x));
        }
        for rm2 in exhaustive_rounding_modes() {
            let (s2, o2) = x.tan_prec_round_ref(prec, rm2);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.tan_prec_round_ref(prec, Exact));
    }
}

#[test]
fn tan_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        tan_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.tan_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::INFINITY.tan_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_INFINITY.tan_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.tan_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.tan_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
    });
}

#[test]
fn tan_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (s, o) = x.clone().tan_round(rm);
        assert!(s.is_valid());
        assert_rounding_ordering_consistent(&s, rm, o);
        let (s_alt, o_alt) = x.tan_round_ref(rm);
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.tan_round_assign(rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (s_alt, o_alt) = x.tan_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_s, rug_o) = rug_tan_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn tan_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (s, o) = x.clone().tan_prec(prec);
        assert!(s.is_valid());
        let (s_alt, o_alt) = x.tan_prec_ref(prec);
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.tan_prec_assign(prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (s_alt, o_alt) = x.tan_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (rug_s, rug_o) = rug_tan_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn tan_properties() {
    float_gen().test_properties(|x| {
        let s = x.clone().tan();
        assert!(s.is_valid());
        let s_alt = (&x).tan();
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));

        let mut x_alt = x.clone();
        x_alt.tan_assign();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));

        let (s_alt, _) = x.tan_prec_round_ref(x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_tan(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&s)
        );

        assert_eq!(ComparableFloat((-&x).tan()), ComparableFloat(-&s));
    });
}

// n * pi, with pi rounded to the nearest `prec` bits and the product exact
fn multiple_of_pi(n: i64, prec: u64) -> Float {
    Float::pi_prec(prec)
        .0
        .mul_prec_round(Float::from(n), prec + 64, Exact)
        .0
}

// Inputs close to a nonzero multiple of pi (n * pi with pi rounded to `prec_x` bits, `half` false),
// where the tangent is tiny and comes from the sine's near-zero path, and to an odd multiple of
// pi/2 (n * pi / 2, `half` true), where it is huge and comes from the cosine's.
#[test]
fn test_tan_near_zero() {
    let test = |half: bool,
                n: i64,
                prec_x: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = if half {
            multiple_of_pi(n, prec_x) >> 1u32
        } else {
            multiple_of_pi(n, prec_x)
        };
        let (t, o) = x.tan_prec_round_ref(prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (rug_t, rug_o) = rug_tan_prec_round(
            &rug::Float::exact_from(&x),
            prec,
            rug_round_try_from_rounding_mode(rm).unwrap(),
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_t)),
            ComparableFloatRef(&t)
        );
        assert_eq!(rug_o, o);
    };
    test(false, 1, 100, 1, Nearest, "-2.0e-31", "-0x4.0E-26#1", Less);
    test(false, 1, 100, 1, Floor, "-2.0e-31", "-0x4.0E-26#1", Less);
    test(
        false,
        1,
        100,
        10,
        Nearest,
        "-1.6948e-31",
        "-0x3.70E-26#10",
        Greater,
    );
    test(
        false,
        1,
        100,
        10,
        Floor,
        "-1.6967e-31",
        "-0x3.71E-26#10",
        Less,
    );
    test(
        false,
        1,
        100,
        64,
        Nearest,
        "-1.69568553207377992875e-31",
        "-0x3.707344a409382228E-26#64",
        Greater,
    );
    test(
        false,
        1,
        100,
        64,
        Floor,
        "-1.69568553207377992886e-31",
        "-0x3.707344a40938222cE-26#64",
        Less,
    );
    test(false, 1, 150, 1, Nearest, "1.4e-45", "0x8.0E-38#1", Greater);
    test(false, 1, 150, 1, Floor, "7.0e-46", "0x4.0E-38#1", Less);
    test(
        false,
        1,
        150,
        10,
        Nearest,
        "1.3780e-45",
        "0x7.deE-38#10",
        Greater,
    );
    test(
        false,
        1,
        150,
        10,
        Floor,
        "1.3767e-45",
        "0x7.dcE-38#10",
        Less,
    );
    test(
        false,
        1,
        150,
        64,
        Nearest,
        "1.37792347486608826351e-45",
        "0x7.ddd660ce2ff7d108E-38#64",
        Greater,
    );
    test(
        false,
        1,
        150,
        64,
        Floor,
        "1.37792347486608826343e-45",
        "0x7.ddd660ce2ff7d100E-38#64",
        Less,
    );
    test(
        false,
        1,
        400,
        1,
        Nearest,
        "-4.8e-122",
        "-0x2.0E-101#1",
        Greater,
    );
    test(false, 1, 400, 1, Floor, "-9.7e-122", "-0x4.0E-101#1", Less);
    test(
        false,
        1,
        400,
        10,
        Nearest,
        "-6.3062e-122",
        "-0x2.9bE-101#10",
        Greater,
    );
    test(
        false,
        1,
        400,
        10,
        Floor,
        "-6.3157e-122",
        "-0x2.9cE-101#10",
        Less,
    );
    test(
        false,
        1,
        400,
        64,
        Nearest,
        "-6.31079971227703730891e-122",
        "-0x2.9b7c97c50dd3f84cE-101#64",
        Greater,
    );
    test(
        false,
        1,
        400,
        64,
        Floor,
        "-6.31079971227703730944e-122",
        "-0x2.9b7c97c50dd3f850E-101#64",
        Less,
    );
    test(false, 2, 100, 1, Nearest, "-3.9e-31", "-0x8.0E-26#1", Less);
    test(false, 2, 100, 1, Floor, "-3.9e-31", "-0x8.0E-26#1", Less);
    test(
        false,
        2,
        100,
        10,
        Nearest,
        "-3.3896e-31",
        "-0x6.e0E-26#10",
        Greater,
    );
    test(
        false,
        2,
        100,
        10,
        Floor,
        "-3.3935e-31",
        "-0x6.e2E-26#10",
        Less,
    );
    test(
        false,
        2,
        100,
        64,
        Nearest,
        "-3.39137106414755985750e-31",
        "-0x6.e0e6894812704450E-26#64",
        Greater,
    );
    test(
        false,
        2,
        100,
        64,
        Floor,
        "-3.39137106414755985771e-31",
        "-0x6.e0e6894812704458E-26#64",
        Less,
    );
    test(false, 2, 150, 1, Nearest, "2.8e-45", "0x1.0E-37#1", Greater);
    test(false, 2, 150, 1, Floor, "1.4e-45", "0x8.0E-38#1", Less);
    test(
        false,
        2,
        150,
        10,
        Nearest,
        "2.7561e-45",
        "0xf.bcE-38#10",
        Greater,
    );
    test(
        false,
        2,
        150,
        10,
        Floor,
        "2.7533e-45",
        "0xf.b8E-38#10",
        Less,
    );
    test(
        false,
        2,
        150,
        64,
        Nearest,
        "2.75584694973217652702e-45",
        "0xf.bbacc19c5fefa21E-38#64",
        Greater,
    );
    test(
        false,
        2,
        150,
        64,
        Floor,
        "2.75584694973217652687e-45",
        "0xf.bbacc19c5fefa20E-38#64",
        Less,
    );
    test(
        false,
        2,
        400,
        1,
        Nearest,
        "-9.7e-122",
        "-0x4.0E-101#1",
        Greater,
    );
    test(false, 2, 400, 1, Floor, "-1.9e-121", "-0x8.0E-101#1", Less);
    test(
        false,
        2,
        400,
        10,
        Nearest,
        "-1.2612e-121",
        "-0x5.36E-101#10",
        Greater,
    );
    test(
        false,
        2,
        400,
        10,
        Floor,
        "-1.2631e-121",
        "-0x5.38E-101#10",
        Less,
    );
    test(
        false,
        2,
        400,
        64,
        Nearest,
        "-1.26215994245540746178e-121",
        "-0x5.36f92f8a1ba7f098E-101#64",
        Greater,
    );
    test(
        false,
        2,
        400,
        64,
        Floor,
        "-1.26215994245540746189e-121",
        "-0x5.36f92f8a1ba7f0a0E-101#64",
        Less,
    );
    test(
        false,
        3,
        100,
        1,
        Nearest,
        "-3.9e-31",
        "-0x8.0E-26#1",
        Greater,
    );
    test(false, 3, 100, 1, Floor, "-7.9e-31", "-0x1.0E-25#1", Less);
    test(
        false,
        3,
        100,
        10,
        Nearest,
        "-5.0845e-31",
        "-0xa.50E-26#10",
        Greater,
    );
    test(
        false,
        3,
        100,
        10,
        Floor,
        "-5.0922e-31",
        "-0xa.54E-26#10",
        Less,
    );
    test(
        false,
        3,
        100,
        64,
        Nearest,
        "-5.08705659622133978646e-31",
        "-0xa.5159cdec1ba8668E-26#64",
        Less,
    );
    test(
        false,
        3,
        100,
        64,
        Floor,
        "-5.08705659622133978646e-31",
        "-0xa.5159cdec1ba8668E-26#64",
        Less,
    );
    test(false, 3, 150, 1, Nearest, "2.8e-45", "0x1.0E-37#1", Less);
    test(false, 3, 150, 1, Floor, "2.8e-45", "0x1.0E-37#1", Less);
    test(
        false,
        3,
        150,
        10,
        Nearest,
        "4.1327e-45",
        "0x1.798E-37#10",
        Less,
    );
    test(
        false,
        3,
        150,
        10,
        Floor,
        "4.1327e-45",
        "0x1.798E-37#10",
        Less,
    );
    test(
        false,
        3,
        150,
        64,
        Nearest,
        "4.13377042459826479061e-45",
        "0x1.79983226a8fe7732E-37#64",
        Greater,
    );
    test(
        false,
        3,
        150,
        64,
        Floor,
        "4.13377042459826479030e-45",
        "0x1.79983226a8fe7730E-37#64",
        Less,
    );
    test(
        false,
        3,
        400,
        1,
        Nearest,
        "-1.9e-121",
        "-0x8.0E-101#1",
        Less,
    );
    test(false, 3, 400, 1, Floor, "-1.9e-121", "-0x8.0E-101#1", Less);
    test(
        false,
        3,
        400,
        10,
        Nearest,
        "-1.8928e-121",
        "-0x7.d2E-101#10",
        Greater,
    );
    test(
        false,
        3,
        400,
        10,
        Floor,
        "-1.8947e-121",
        "-0x7.d4E-101#10",
        Less,
    );
    test(
        false,
        3,
        400,
        64,
        Nearest,
        "-1.89323991368311119273e-121",
        "-0x7.d275c74f297be8e8E-101#64",
        Greater,
    );
    test(
        false,
        3,
        400,
        64,
        Floor,
        "-1.89323991368311119283e-121",
        "-0x7.d275c74f297be8f0E-101#64",
        Less,
    );
    test(
        false,
        5,
        100,
        1,
        Nearest,
        "-7.9e-31",
        "-0x1.0E-25#1",
        Greater,
    );
    test(false, 5, 100, 1, Floor, "-1.6e-30", "-0x2.0E-25#1", Less);
    test(
        false,
        5,
        100,
        10,
        Nearest,
        "-8.4741e-31",
        "-0x1.130E-25#10",
        Greater,
    );
    test(
        false,
        5,
        100,
        10,
        Floor,
        "-8.4895e-31",
        "-0x1.138E-25#10",
        Less,
    );
    test(
        false,
        5,
        100,
        64,
        Nearest,
        "-8.47842766036889964438e-31",
        "-0x1.1324057342e18aaeE-25#64",
        Less,
    );
    test(
        false,
        5,
        100,
        64,
        Floor,
        "-8.47842766036889964438e-31",
        "-0x1.1324057342e18aaeE-25#64",
        Less,
    );
    test(false, 5, 150, 1, Nearest, "5.6e-45", "0x2.0E-37#1", Less);
    test(false, 5, 150, 1, Floor, "5.6e-45", "0x2.0E-37#1", Less);
    test(
        false,
        5,
        150,
        10,
        Nearest,
        "6.8861e-45",
        "0x2.75E-37#10",
        Less,
    );
    test(
        false,
        5,
        150,
        10,
        Floor,
        "6.8861e-45",
        "0x2.75E-37#10",
        Less,
    );
    test(
        false,
        5,
        150,
        64,
        Nearest,
        "6.88961737433044131717e-45",
        "0x2.7552fe406efd7150E-37#64",
        Less,
    );
    test(
        false,
        5,
        150,
        64,
        Floor,
        "6.88961737433044131717e-45",
        "0x2.7552fe406efd7150E-37#64",
        Less,
    );
    test(
        false,
        5,
        400,
        1,
        Nearest,
        "-3.9e-121",
        "-0x1.0E-100#1",
        Less,
    );
    test(false, 5, 400, 1, Floor, "-3.9e-121", "-0x1.0E-100#1", Less);
    test(
        false,
        5,
        400,
        10,
        Nearest,
        "-3.1540e-121",
        "-0xd.08E-101#10",
        Greater,
    );
    test(
        false,
        5,
        400,
        10,
        Floor,
        "-3.1578e-121",
        "-0xd.0cE-101#10",
        Less,
    );
    test(
        false,
        5,
        400,
        64,
        Nearest,
        "-3.15539985613851865451e-121",
        "-0xd.096ef6d94523d98E-101#64",
        Greater,
    );
    test(
        false,
        5,
        400,
        64,
        Floor,
        "-3.15539985613851865472e-121",
        "-0xd.096ef6d94523d99E-101#64",
        Less,
    );
    test(
        false,
        -1,
        100,
        1,
        Nearest,
        "2.0e-31",
        "0x4.0E-26#1",
        Greater,
    );
    test(false, -1, 100, 1, Floor, "9.9e-32", "0x2.0E-26#1", Less);
    test(
        false,
        -1,
        100,
        10,
        Nearest,
        "1.6948e-31",
        "0x3.70E-26#10",
        Less,
    );
    test(
        false,
        -1,
        100,
        10,
        Floor,
        "1.6948e-31",
        "0x3.70E-26#10",
        Less,
    );
    test(
        false,
        -1,
        100,
        64,
        Nearest,
        "1.69568553207377992875e-31",
        "0x3.707344a409382228E-26#64",
        Less,
    );
    test(
        false,
        -1,
        100,
        64,
        Floor,
        "1.69568553207377992875e-31",
        "0x3.707344a409382228E-26#64",
        Less,
    );
    test(false, -1, 150, 1, Nearest, "-1.4e-45", "-0x8.0E-38#1", Less);
    test(false, -1, 150, 1, Floor, "-1.4e-45", "-0x8.0E-38#1", Less);
    test(
        false,
        -1,
        150,
        10,
        Nearest,
        "-1.3780e-45",
        "-0x7.deE-38#10",
        Less,
    );
    test(
        false,
        -1,
        150,
        10,
        Floor,
        "-1.3780e-45",
        "-0x7.deE-38#10",
        Less,
    );
    test(
        false,
        -1,
        150,
        64,
        Nearest,
        "-1.37792347486608826351e-45",
        "-0x7.ddd660ce2ff7d108E-38#64",
        Less,
    );
    test(
        false,
        -1,
        150,
        64,
        Floor,
        "-1.37792347486608826351e-45",
        "-0x7.ddd660ce2ff7d108E-38#64",
        Less,
    );
    test(false, -1, 400, 1, Nearest, "4.8e-122", "0x2.0E-101#1", Less);
    test(false, -1, 400, 1, Floor, "4.8e-122", "0x2.0E-101#1", Less);
    test(
        false,
        -1,
        400,
        10,
        Nearest,
        "6.3062e-122",
        "0x2.9bE-101#10",
        Less,
    );
    test(
        false,
        -1,
        400,
        10,
        Floor,
        "6.3062e-122",
        "0x2.9bE-101#10",
        Less,
    );
    test(
        false,
        -1,
        400,
        64,
        Nearest,
        "6.31079971227703730891e-122",
        "0x2.9b7c97c50dd3f84cE-101#64",
        Less,
    );
    test(
        false,
        -1,
        400,
        64,
        Floor,
        "6.31079971227703730891e-122",
        "0x2.9b7c97c50dd3f84cE-101#64",
        Less,
    );
    test(false, -3, 100, 1, Nearest, "3.9e-31", "0x8.0E-26#1", Less);
    test(false, -3, 100, 1, Floor, "3.9e-31", "0x8.0E-26#1", Less);
    test(
        false,
        -3,
        100,
        10,
        Nearest,
        "5.0845e-31",
        "0xa.50E-26#10",
        Less,
    );
    test(
        false,
        -3,
        100,
        10,
        Floor,
        "5.0845e-31",
        "0xa.50E-26#10",
        Less,
    );
    test(
        false,
        -3,
        100,
        64,
        Nearest,
        "5.08705659622133978646e-31",
        "0xa.5159cdec1ba8668E-26#64",
        Greater,
    );
    test(
        false,
        -3,
        100,
        64,
        Floor,
        "5.08705659622133978603e-31",
        "0xa.5159cdec1ba8667E-26#64",
        Less,
    );
    test(
        false,
        -3,
        150,
        1,
        Nearest,
        "-2.8e-45",
        "-0x1.0E-37#1",
        Greater,
    );
    test(false, -3, 150, 1, Floor, "-5.6e-45", "-0x2.0E-37#1", Less);
    test(
        false,
        -3,
        150,
        10,
        Nearest,
        "-4.1327e-45",
        "-0x1.798E-37#10",
        Greater,
    );
    test(
        false,
        -3,
        150,
        10,
        Floor,
        "-4.1382e-45",
        "-0x1.7a0E-37#10",
        Less,
    );
    test(
        false,
        -3,
        150,
        64,
        Nearest,
        "-4.13377042459826479061e-45",
        "-0x1.79983226a8fe7732E-37#64",
        Less,
    );
    test(
        false,
        -3,
        150,
        64,
        Floor,
        "-4.13377042459826479061e-45",
        "-0x1.79983226a8fe7732E-37#64",
        Less,
    );
    test(
        false,
        -3,
        400,
        1,
        Nearest,
        "1.9e-121",
        "0x8.0E-101#1",
        Greater,
    );
    test(false, -3, 400, 1, Floor, "9.7e-122", "0x4.0E-101#1", Less);
    test(
        false,
        -3,
        400,
        10,
        Nearest,
        "1.8928e-121",
        "0x7.d2E-101#10",
        Less,
    );
    test(
        false,
        -3,
        400,
        10,
        Floor,
        "1.8928e-121",
        "0x7.d2E-101#10",
        Less,
    );
    test(
        false,
        -3,
        400,
        64,
        Nearest,
        "1.89323991368311119273e-121",
        "0x7.d275c74f297be8e8E-101#64",
        Less,
    );
    test(
        false,
        -3,
        400,
        64,
        Floor,
        "1.89323991368311119273e-121",
        "0x7.d275c74f297be8e8E-101#64",
        Less,
    );
    test(true, 1, 100, 1, Nearest, "1.0e31", "0x8.0E+25#1", Less);
    test(true, 1, 100, 1, Floor, "1.0e31", "0x8.0E+25#1", Less);
    test(
        true,
        1,
        100,
        10,
        Nearest,
        "1.1785e31",
        "0x9.4cE+25#10",
        Less,
    );
    test(true, 1, 100, 10, Floor, "1.1785e31", "0x9.4cE+25#10", Less);
    test(
        true,
        1,
        100,
        64,
        Nearest,
        "1.17946397617372560790e31",
        "0x9.4de8957cbf8e6ceE+25#64",
        Greater,
    );
    test(
        true,
        1,
        100,
        64,
        Floor,
        "1.17946397617372560779e31",
        "0x9.4de8957cbf8e6cdE+25#64",
        Less,
    );
    test(true, 1, 150, 1, Nearest, "-1.4e45", "-0x4.0E+37#1", Greater);
    test(true, 1, 150, 1, Floor, "-2.9e45", "-0x8.0E+37#1", Less);
    test(
        true,
        1,
        150,
        10,
        Nearest,
        "-1.4523e45",
        "-0x4.12E+37#10",
        Less,
    );
    test(
        true,
        1,
        150,
        10,
        Floor,
        "-1.4523e45",
        "-0x4.12E+37#10",
        Less,
    );
    test(
        true,
        1,
        150,
        64,
        Nearest,
        "-1.45145941446012990721e45",
        "-0x4.115efdaf9271d4a8E+37#64",
        Less,
    );
    test(
        true,
        1,
        150,
        64,
        Floor,
        "-1.45145941446012990721e45",
        "-0x4.115efdaf9271d4a8E+37#64",
        Less,
    );
    test(true, 1, 400, 1, Nearest, "4.1e121", "0x1.0E+101#1", Greater);
    test(true, 1, 400, 1, Floor, "2.1e121", "0x8.0E+100#1", Less);
    test(
        true,
        1,
        400,
        10,
        Nearest,
        "3.1673e121",
        "0xc.44E+100#10",
        Less,
    );
    test(
        true,
        1,
        400,
        10,
        Floor,
        "3.1673e121",
        "0xc.44E+100#10",
        Less,
    );
    test(
        true,
        1,
        400,
        64,
        Nearest,
        "3.16917045570183063752e121",
        "0xc.45dd0766b6b9926E+100#64",
        Greater,
    );
    test(
        true,
        1,
        400,
        64,
        Floor,
        "3.16917045570183063729e121",
        "0xc.45dd0766b6b9925E+100#64",
        Less,
    );
    test(true, 3, 100, 1, Nearest, "5.1e30", "0x4.0E+25#1", Greater);
    test(true, 3, 100, 1, Floor, "2.5e30", "0x2.0E+25#1", Less);
    test(
        true,
        3,
        100,
        10,
        Nearest,
        "3.9317e30",
        "0x3.1aE+25#10",
        Greater,
    );
    test(true, 3, 100, 10, Floor, "3.9267e30", "0x3.19E+25#10", Less);
    test(
        true,
        3,
        100,
        64,
        Nearest,
        "3.93154658724575202615e30",
        "0x3.19f831d43fda2448E+25#64",
        Less,
    );
    test(
        true,
        3,
        100,
        64,
        Floor,
        "3.93154658724575202615e30",
        "0x3.19f831d43fda2448E+25#64",
        Less,
    );
    test(true, 3, 150, 1, Nearest, "-3.6e44", "-0x1.0E+37#1", Greater);
    test(true, 3, 150, 1, Floor, "-7.1e44", "-0x2.0E+37#1", Less);
    test(
        true,
        3,
        150,
        10,
        Nearest,
        "-4.8365e44",
        "-0x1.5b0E+37#10",
        Greater,
    );
    test(
        true,
        3,
        150,
        10,
        Floor,
        "-4.8434e44",
        "-0x1.5b8E+37#10",
        Less,
    );
    test(
        true,
        3,
        150,
        64,
        Nearest,
        "-4.83819804820043302402e44",
        "-0x1.5b1fa9e530d09c38E+37#64",
        Less,
    );
    test(
        true,
        3,
        150,
        64,
        Floor,
        "-4.83819804820043302402e44",
        "-0x1.5b1fa9e530d09c38E+37#64",
        Less,
    );
    test(true, 3, 400, 1, Nearest, "1.0e121", "0x4.0E+100#1", Less);
    test(true, 3, 400, 1, Floor, "1.0e121", "0x4.0E+100#1", Less);
    test(
        true,
        3,
        400,
        10,
        Nearest,
        "1.0571e121",
        "0x4.18E+100#10",
        Greater,
    );
    test(
        true,
        3,
        400,
        10,
        Floor,
        "1.0551e121",
        "0x4.16E+100#10",
        Less,
    );
    test(
        true,
        3,
        400,
        64,
        Nearest,
        "1.05639015190061021251e121",
        "0x4.1749ad223ce88620E+100#64",
        Greater,
    );
    test(
        true,
        3,
        400,
        64,
        Floor,
        "1.05639015190061021239e121",
        "0x4.1749ad223ce88618E+100#64",
        Less,
    );
    test(true, 5, 100, 1, Nearest, "2.5e30", "0x2.0E+25#1", Greater);
    test(true, 5, 100, 1, Floor, "1.3e30", "0x1.0E+25#1", Less);
    test(
        true,
        5,
        100,
        10,
        Nearest,
        "2.3595e30",
        "0x1.dc8E+25#10",
        Greater,
    );
    test(true, 5, 100, 10, Floor, "2.3570e30", "0x1.dc0E+25#10", Less);
    test(
        true,
        5,
        100,
        64,
        Nearest,
        "2.35892795234745121571e30",
        "0x1.dc61b77f5982e292E+25#64",
        Less,
    );
    test(
        true,
        5,
        100,
        64,
        Floor,
        "2.35892795234745121571e30",
        "0x1.dc61b77f5982e292E+25#64",
        Less,
    );
    test(true, 5, 150, 1, Nearest, "-3.6e44", "-0x1.0E+37#1", Less);
    test(true, 5, 150, 1, Floor, "-3.6e44", "-0x1.0E+37#1", Less);
    test(
        true,
        5,
        150,
        10,
        Nearest,
        "-2.9026e44",
        "-0xd.04E+36#10",
        Greater,
    );
    test(
        true,
        5,
        150,
        10,
        Floor,
        "-2.9061e44",
        "-0xd.08E+36#10",
        Less,
    );
    test(
        true,
        5,
        150,
        64,
        Nearest,
        "-2.90291882892025981434e44",
        "-0xd.04632bcb6e390eeE+36#64",
        Greater,
    );
    test(
        true,
        5,
        150,
        64,
        Floor,
        "-2.90291882892025981453e44",
        "-0xd.04632bcb6e390efE+36#64",
        Less,
    );
    test(true, 5, 400, 1, Nearest, "5.2e120", "0x2.0E+100#1", Less);
    test(true, 5, 400, 1, Floor, "5.2e120", "0x2.0E+100#1", Less);
    test(
        true,
        5,
        400,
        10,
        Nearest,
        "6.3346e120",
        "0x2.74E+100#10",
        Less,
    );
    test(
        true,
        5,
        400,
        10,
        Floor,
        "6.3346e120",
        "0x2.74E+100#10",
        Less,
    );
    test(
        true,
        5,
        400,
        64,
        Nearest,
        "6.33834091140366127503e120",
        "0x2.745f67e157beb6e0E+100#64",
        Greater,
    );
    test(
        true,
        5,
        400,
        64,
        Floor,
        "6.33834091140366127447e120",
        "0x2.745f67e157beb6dcE+100#64",
        Less,
    );
    test(
        true,
        -1,
        100,
        1,
        Nearest,
        "-1.0e31",
        "-0x8.0E+25#1",
        Greater,
    );
    test(true, -1, 100, 1, Floor, "-2.0e31", "-0x1.0E+26#1", Less);
    test(
        true,
        -1,
        100,
        10,
        Nearest,
        "-1.1785e31",
        "-0x9.4cE+25#10",
        Greater,
    );
    test(
        true,
        -1,
        100,
        10,
        Floor,
        "-1.1805e31",
        "-0x9.50E+25#10",
        Less,
    );
    test(
        true,
        -1,
        100,
        64,
        Nearest,
        "-1.17946397617372560790e31",
        "-0x9.4de8957cbf8e6ceE+25#64",
        Less,
    );
    test(
        true,
        -1,
        100,
        64,
        Floor,
        "-1.17946397617372560790e31",
        "-0x9.4de8957cbf8e6ceE+25#64",
        Less,
    );
    test(true, -1, 150, 1, Nearest, "1.4e45", "0x4.0E+37#1", Less);
    test(true, -1, 150, 1, Floor, "1.4e45", "0x4.0E+37#1", Less);
    test(
        true,
        -1,
        150,
        10,
        Nearest,
        "1.4523e45",
        "0x4.12E+37#10",
        Greater,
    );
    test(true, -1, 150, 10, Floor, "1.4495e45", "0x4.10E+37#10", Less);
    test(
        true,
        -1,
        150,
        64,
        Nearest,
        "1.45145941446012990721e45",
        "0x4.115efdaf9271d4a8E+37#64",
        Greater,
    );
    test(
        true,
        -1,
        150,
        64,
        Floor,
        "1.45145941446012990705e45",
        "0x4.115efdaf9271d4a0E+37#64",
        Less,
    );
    test(true, -1, 400, 1, Nearest, "-4.1e121", "-0x1.0E+101#1", Less);
    test(true, -1, 400, 1, Floor, "-4.1e121", "-0x1.0E+101#1", Less);
    test(
        true,
        -1,
        400,
        10,
        Nearest,
        "-3.1673e121",
        "-0xc.44E+100#10",
        Greater,
    );
    test(
        true,
        -1,
        400,
        10,
        Floor,
        "-3.1713e121",
        "-0xc.48E+100#10",
        Less,
    );
    test(
        true,
        -1,
        400,
        64,
        Nearest,
        "-3.16917045570183063752e121",
        "-0xc.45dd0766b6b9926E+100#64",
        Less,
    );
    test(
        true,
        -1,
        400,
        64,
        Floor,
        "-3.16917045570183063752e121",
        "-0xc.45dd0766b6b9926E+100#64",
        Less,
    );
    test(true, -3, 100, 1, Nearest, "-5.1e30", "-0x4.0E+25#1", Less);
    test(true, -3, 100, 1, Floor, "-5.1e30", "-0x4.0E+25#1", Less);
    test(
        true,
        -3,
        100,
        10,
        Nearest,
        "-3.9317e30",
        "-0x3.1aE+25#10",
        Less,
    );
    test(
        true,
        -3,
        100,
        10,
        Floor,
        "-3.9317e30",
        "-0x3.1aE+25#10",
        Less,
    );
    test(
        true,
        -3,
        100,
        64,
        Nearest,
        "-3.93154658724575202615e30",
        "-0x3.19f831d43fda2448E+25#64",
        Greater,
    );
    test(
        true,
        -3,
        100,
        64,
        Floor,
        "-3.93154658724575202642e30",
        "-0x3.19f831d43fda244cE+25#64",
        Less,
    );
    test(true, -3, 150, 1, Nearest, "3.6e44", "0x1.0E+37#1", Less);
    test(true, -3, 150, 1, Floor, "3.6e44", "0x1.0E+37#1", Less);
    test(
        true,
        -3,
        150,
        10,
        Nearest,
        "4.8365e44",
        "0x1.5b0E+37#10",
        Less,
    );
    test(
        true,
        -3,
        150,
        10,
        Floor,
        "4.8365e44",
        "0x1.5b0E+37#10",
        Less,
    );
    test(
        true,
        -3,
        150,
        64,
        Nearest,
        "4.83819804820043302402e44",
        "0x1.5b1fa9e530d09c38E+37#64",
        Greater,
    );
    test(
        true,
        -3,
        150,
        64,
        Floor,
        "4.83819804820043302364e44",
        "0x1.5b1fa9e530d09c36E+37#64",
        Less,
    );
    test(
        true,
        -3,
        400,
        1,
        Nearest,
        "-1.0e121",
        "-0x4.0E+100#1",
        Greater,
    );
    test(true, -3, 400, 1, Floor, "-2.1e121", "-0x8.0E+100#1", Less);
    test(
        true,
        -3,
        400,
        10,
        Nearest,
        "-1.0571e121",
        "-0x4.18E+100#10",
        Less,
    );
    test(
        true,
        -3,
        400,
        10,
        Floor,
        "-1.0571e121",
        "-0x4.18E+100#10",
        Less,
    );
    test(
        true,
        -3,
        400,
        64,
        Nearest,
        "-1.05639015190061021251e121",
        "-0x4.1749ad223ce88620E+100#64",
        Less,
    );
    test(
        true,
        -3,
        400,
        64,
        Floor,
        "-1.05639015190061021251e121",
        "-0x4.1749ad223ce88620E+100#64",
        Less,
    );
}

// Inputs within 2^(-2^30) of pi and of pi/2, where the tangent underflows or overflows. Each call
// computes pi to about 2^30 bits twice (once for the sine and cosine, once for the exact bracket of
// the underflowed one), about half an hour each, so this test is slow even in release mode and
// makes just one call of each kind: the underflow exercises the sine's exact bracket, and the
// overflow the shortcut for an underflowed cosine.
#[test]
fn test_tan_underflow_and_overflow() {
    let p = (1u64 << 30) + 64;
    // pi rounded down: x < pi, so the sine is positive and tiny while the cosine is just above -1,
    // and the tangent is negative and tiny
    let pi = Float::pi_prec_round(p, Floor).0;
    let (t, o) = pi.tan_prec_round_ref(10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    // pi/2 rounded down: the tangent is positive and huge, beyond the largest finite Float
    let half_pi = pi >> 1u32;
    let (t, o) = half_pi.tan_prec_round_ref(10, Down);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::max_finite_value_with_prec(10))
    );
    assert_eq!(o, Less);
}

// Rows reuse the sine test's inputs, including the non-dyadic ones whose tangents MPFR cannot see
// exactly, since it must round the input first.
#[test]
fn test_tan_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (t, o) = Float::tan_rational_prec_round(x.clone(), prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = Float::tan_rational_prec_round_ref(&x, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = Float::tan_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) = Float::tan_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) = rug_tan_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 1, Down, "0.0", "0x0.0", Equal);
    test("0", 1, Up, "0.0", "0x0.0", Equal);
    test("0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", 1, Exact, "0.0", "0x0.0", Equal);
    test("0", 5, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Down, "0.0", "0x0.0", Equal);
    test("0", 10, Up, "0.0", "0x0.0", Equal);
    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("0", 20, Nearest, "0.0", "0x0.0", Equal);
    test("0", 53, Down, "0.0", "0x0.0", Equal);
    test("0", 53, Up, "0.0", "0x0.0", Equal);
    test("0", 53, Floor, "0.0", "0x0.0", Equal);
    test("0", 53, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 53, Nearest, "0.0", "0x0.0", Equal);
    test("0", 53, Exact, "0.0", "0x0.0", Equal);
    test("0", 100, Nearest, "0.0", "0x0.0", Equal);
    test("1", 1, Down, "1.0", "0x1.0#1", Less);
    test("1", 1, Up, "2.0", "0x2.0#1", Greater);
    test("1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1", 5, Nearest, "1.56", "0x1.9#5", Greater);
    test("1", 10, Down, "1.5566", "0x1.8e8#10", Less);
    test("1", 10, Up, "1.5586", "0x1.8f0#10", Greater);
    test("1", 10, Floor, "1.5566", "0x1.8e8#10", Less);
    test("1", 10, Ceiling, "1.5586", "0x1.8f0#10", Greater);
    test("1", 10, Nearest, "1.5566", "0x1.8e8#10", Less);
    test("1", 20, Nearest, "1.5574074", "0x1.8eb24#20", Less);
    test(
        "1",
        53,
        Down,
        "1.5574077246549021",
        "0x1.8eb245cbee3a5#53",
        Less,
    );
    test(
        "1",
        53,
        Up,
        "1.5574077246549023",
        "0x1.8eb245cbee3a6#53",
        Greater,
    );
    test(
        "1",
        53,
        Floor,
        "1.5574077246549021",
        "0x1.8eb245cbee3a5#53",
        Less,
    );
    test(
        "1",
        53,
        Ceiling,
        "1.5574077246549023",
        "0x1.8eb245cbee3a6#53",
        Greater,
    );
    test(
        "1",
        53,
        Nearest,
        "1.5574077246549023",
        "0x1.8eb245cbee3a6#53",
        Greater,
    );
    test(
        "1",
        100,
        Nearest,
        "1.5574077246549022305069748074591",
        "0x1.8eb245cbee3a5b8acc7d41324#100",
        Greater,
    );
    test("-1", 1, Down, "-1.0", "-0x1.0#1", Greater);
    test("-1", 1, Up, "-2.0", "-0x2.0#1", Less);
    test("-1", 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("-1", 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("-1", 1, Nearest, "-2.0", "-0x2.0#1", Less);
    test("-1", 5, Nearest, "-1.56", "-0x1.9#5", Less);
    test("-1", 10, Down, "-1.5566", "-0x1.8e8#10", Greater);
    test("-1", 10, Up, "-1.5586", "-0x1.8f0#10", Less);
    test("-1", 10, Floor, "-1.5586", "-0x1.8f0#10", Less);
    test("-1", 10, Ceiling, "-1.5566", "-0x1.8e8#10", Greater);
    test("-1", 10, Nearest, "-1.5566", "-0x1.8e8#10", Greater);
    test("-1", 20, Nearest, "-1.5574074", "-0x1.8eb24#20", Greater);
    test(
        "-1",
        53,
        Down,
        "-1.5574077246549021",
        "-0x1.8eb245cbee3a5#53",
        Greater,
    );
    test(
        "-1",
        53,
        Up,
        "-1.5574077246549023",
        "-0x1.8eb245cbee3a6#53",
        Less,
    );
    test(
        "-1",
        53,
        Floor,
        "-1.5574077246549023",
        "-0x1.8eb245cbee3a6#53",
        Less,
    );
    test(
        "-1",
        53,
        Ceiling,
        "-1.5574077246549021",
        "-0x1.8eb245cbee3a5#53",
        Greater,
    );
    test(
        "-1",
        53,
        Nearest,
        "-1.5574077246549023",
        "-0x1.8eb245cbee3a6#53",
        Less,
    );
    test(
        "-1",
        100,
        Nearest,
        "-1.5574077246549022305069748074591",
        "-0x1.8eb245cbee3a5b8acc7d41324#100",
        Less,
    );
    test("1/2", 1, Down, "0.50", "0x0.8#1", Less);
    test("1/2", 1, Up, "1.0", "0x1.0#1", Greater);
    test("1/2", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1/2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1/2", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("1/2", 5, Nearest, "0.531", "0x0.88#5", Less);
    test("1/2", 10, Down, "0.54590", "0x0.8bc#10", Less);
    test("1/2", 10, Up, "0.54688", "0x0.8c0#10", Greater);
    test("1/2", 10, Floor, "0.54590", "0x0.8bc#10", Less);
    test("1/2", 10, Ceiling, "0.54688", "0x0.8c0#10", Greater);
    test("1/2", 10, Nearest, "0.54590", "0x0.8bc#10", Less);
    test("1/2", 20, Nearest, "0.54630280", "0x0.8bda8#20", Greater);
    test(
        "1/2",
        53,
        Down,
        "0.54630248984379048",
        "0x0.8bda7adf9a3a50#53",
        Less,
    );
    test(
        "1/2",
        53,
        Up,
        "0.54630248984379060",
        "0x0.8bda7adf9a3a58#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Floor,
        "0.54630248984379048",
        "0x0.8bda7adf9a3a50#53",
        Less,
    );
    test(
        "1/2",
        53,
        Ceiling,
        "0.54630248984379060",
        "0x0.8bda7adf9a3a58#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Nearest,
        "0.54630248984379048",
        "0x0.8bda7adf9a3a50#53",
        Less,
    );
    test(
        "1/2",
        100,
        Nearest,
        "0.54630248984379051325517946578023",
        "0x0.8bda7adf9a3a5218bcb2403c4#100",
        Less,
    );
    test("1/3", 1, Down, "0.25", "0x0.4#1", Less);
    test("1/3", 1, Up, "0.50", "0x0.8#1", Greater);
    test("1/3", 1, Floor, "0.25", "0x0.4#1", Less);
    test("1/3", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("1/3", 1, Nearest, "0.25", "0x0.4#1", Less);
    test("1/3", 5, Nearest, "0.344", "0x0.58#5", Less);
    test("1/3", 10, Down, "0.34619", "0x0.58a#10", Less);
    test("1/3", 10, Up, "0.34668", "0x0.58c#10", Greater);
    test("1/3", 10, Floor, "0.34619", "0x0.58a#10", Less);
    test("1/3", 10, Ceiling, "0.34668", "0x0.58c#10", Greater);
    test("1/3", 10, Nearest, "0.34619", "0x0.58a#10", Less);
    test("1/3", 20, Nearest, "0.34625340", "0x0.58a410#20", Less);
    test(
        "1/3",
        53,
        Down,
        "0.34625354951057546",
        "0x0.58a41297459734#53",
        Less,
    );
    test(
        "1/3",
        53,
        Up,
        "0.34625354951057552",
        "0x0.58a41297459738#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Floor,
        "0.34625354951057546",
        "0x0.58a41297459734#53",
        Less,
    );
    test(
        "1/3",
        53,
        Ceiling,
        "0.34625354951057552",
        "0x0.58a41297459738#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Nearest,
        "0.34625354951057546",
        "0x0.58a41297459734#53",
        Less,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.34625354951057549103854356560970",
        "0x0.58a41297459735e7bc3b8169f0#100",
        Less,
    );
    test("-1/3", 1, Down, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 1, Up, "-0.50", "-0x0.8#1", Less);
    test("-1/3", 1, Floor, "-0.50", "-0x0.8#1", Less);
    test("-1/3", 1, Ceiling, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 1, Nearest, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 5, Nearest, "-0.344", "-0x0.58#5", Greater);
    test("-1/3", 10, Down, "-0.34619", "-0x0.58a#10", Greater);
    test("-1/3", 10, Up, "-0.34668", "-0x0.58c#10", Less);
    test("-1/3", 10, Floor, "-0.34668", "-0x0.58c#10", Less);
    test("-1/3", 10, Ceiling, "-0.34619", "-0x0.58a#10", Greater);
    test("-1/3", 10, Nearest, "-0.34619", "-0x0.58a#10", Greater);
    test(
        "-1/3",
        20,
        Nearest,
        "-0.34625340",
        "-0x0.58a410#20",
        Greater,
    );
    test(
        "-1/3",
        53,
        Down,
        "-0.34625354951057546",
        "-0x0.58a41297459734#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Up,
        "-0.34625354951057552",
        "-0x0.58a41297459738#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Floor,
        "-0.34625354951057552",
        "-0x0.58a41297459738#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Ceiling,
        "-0.34625354951057546",
        "-0x0.58a41297459734#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Nearest,
        "-0.34625354951057546",
        "-0x0.58a41297459734#53",
        Greater,
    );
    test(
        "-1/3",
        100,
        Nearest,
        "-0.34625354951057549103854356560970",
        "-0x0.58a41297459735e7bc3b8169f0#100",
        Greater,
    );
    test("3/5", 1, Down, "0.50", "0x0.8#1", Less);
    test("3/5", 1, Up, "1.0", "0x1.0#1", Greater);
    test("3/5", 1, Floor, "0.50", "0x0.8#1", Less);
    test("3/5", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("3/5", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("3/5", 10, Down, "0.68359", "0x0.af0#10", Less);
    test("3/5", 10, Up, "0.68457", "0x0.af4#10", Greater);
    test("3/5", 10, Floor, "0.68359", "0x0.af0#10", Less);
    test("3/5", 10, Ceiling, "0.68457", "0x0.af4#10", Greater);
    test("3/5", 10, Nearest, "0.68457", "0x0.af4#10", Greater);
    test(
        "3/5",
        53,
        Down,
        "0.68413680834169222",
        "0x0.af239701d14058#53",
        Less,
    );
    test(
        "3/5",
        53,
        Up,
        "0.68413680834169233",
        "0x0.af239701d14060#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Floor,
        "0.68413680834169222",
        "0x0.af239701d14058#53",
        Less,
    );
    test(
        "3/5",
        53,
        Ceiling,
        "0.68413680834169233",
        "0x0.af239701d14060#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Nearest,
        "0.68413680834169233",
        "0x0.af239701d14060#53",
        Greater,
    );
    test(
        "3/5",
        100,
        Nearest,
        "0.68413680834169231707092541746328",
        "0x0.af239701d1405f56a443d8429#100",
        Less,
    );
    test("22/7", 1, Down, "0.00098", "0x0.004#1", Less);
    test("22/7", 1, Up, "0.0020", "0x0.008#1", Greater);
    test("22/7", 1, Floor, "0.00098", "0x0.004#1", Less);
    test("22/7", 1, Ceiling, "0.0020", "0x0.008#1", Greater);
    test("22/7", 1, Nearest, "0.00098", "0x0.004#1", Less);
    test("22/7", 5, Nearest, "0.00128", "0x0.0054#5", Greater);
    test("22/7", 10, Down, "0.0012627", "0x0.0052c#10", Less);
    test("22/7", 10, Up, "0.0012646", "0x0.0052e#10", Greater);
    test("22/7", 10, Floor, "0.0012627", "0x0.0052c#10", Less);
    test("22/7", 10, Ceiling, "0.0012646", "0x0.0052e#10", Greater);
    test("22/7", 10, Nearest, "0.0012646", "0x0.0052e#10", Greater);
    test(
        "22/7",
        20,
        Nearest,
        "0.0012644902",
        "0x0.0052dea0#20",
        Greater,
    );
    test(
        "22/7",
        53,
        Down,
        "0.0012644899412946340",
        "0x0.0052de9ef1a8c410#53",
        Less,
    );
    test(
        "22/7",
        53,
        Up,
        "0.0012644899412946342",
        "0x0.0052de9ef1a8c414#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Floor,
        "0.0012644899412946340",
        "0x0.0052de9ef1a8c410#53",
        Less,
    );
    test(
        "22/7",
        53,
        Ceiling,
        "0.0012644899412946342",
        "0x0.0052de9ef1a8c414#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Nearest,
        "0.0012644899412946342",
        "0x0.0052de9ef1a8c414#53",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "0.0012644899412946341567363163883632",
        "0x0.0052de9ef1a8c413757001f149c8#100",
        Less,
    );
    test("-22/7", 1, Down, "-0.00098", "-0x0.004#1", Greater);
    test("-22/7", 1, Up, "-0.0020", "-0x0.008#1", Less);
    test("-22/7", 1, Floor, "-0.0020", "-0x0.008#1", Less);
    test("-22/7", 1, Ceiling, "-0.00098", "-0x0.004#1", Greater);
    test("-22/7", 1, Nearest, "-0.00098", "-0x0.004#1", Greater);
    test("-22/7", 5, Nearest, "-0.00128", "-0x0.0054#5", Less);
    test("-22/7", 10, Down, "-0.0012627", "-0x0.0052c#10", Greater);
    test("-22/7", 10, Up, "-0.0012646", "-0x0.0052e#10", Less);
    test("-22/7", 10, Floor, "-0.0012646", "-0x0.0052e#10", Less);
    test("-22/7", 10, Ceiling, "-0.0012627", "-0x0.0052c#10", Greater);
    test("-22/7", 10, Nearest, "-0.0012646", "-0x0.0052e#10", Less);
    test(
        "-22/7",
        20,
        Nearest,
        "-0.0012644902",
        "-0x0.0052dea0#20",
        Less,
    );
    test(
        "-22/7",
        53,
        Down,
        "-0.0012644899412946340",
        "-0x0.0052de9ef1a8c410#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Up,
        "-0.0012644899412946342",
        "-0x0.0052de9ef1a8c414#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Floor,
        "-0.0012644899412946342",
        "-0x0.0052de9ef1a8c414#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Ceiling,
        "-0.0012644899412946340",
        "-0x0.0052de9ef1a8c410#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Nearest,
        "-0.0012644899412946342",
        "-0x0.0052de9ef1a8c414#53",
        Less,
    );
    test(
        "-22/7",
        100,
        Nearest,
        "-0.0012644899412946341567363163883632",
        "-0x0.0052de9ef1a8c413757001f149c8#100",
        Greater,
    );
    test("355/113", 1, Down, "2.4e-7", "0x4.0E-6#1", Less);
    test("355/113", 1, Up, "4.8e-7", "0x8.0E-6#1", Greater);
    test("355/113", 1, Floor, "2.4e-7", "0x4.0E-6#1", Less);
    test("355/113", 1, Ceiling, "4.8e-7", "0x8.0E-6#1", Greater);
    test("355/113", 1, Nearest, "2.4e-7", "0x4.0E-6#1", Less);
    test("355/113", 5, Nearest, "2.68e-7", "0x4.8E-6#5", Greater);
    test("355/113", 10, Down, "2.6636e-7", "0x4.78E-6#10", Less);
    test("355/113", 10, Up, "2.6682e-7", "0x4.7aE-6#10", Greater);
    test("355/113", 10, Floor, "2.6636e-7", "0x4.78E-6#10", Less);
    test("355/113", 10, Ceiling, "2.6682e-7", "0x4.7aE-6#10", Greater);
    test("355/113", 10, Nearest, "2.6682e-7", "0x4.7aE-6#10", Greater);
    test(
        "355/113",
        20,
        Nearest,
        "2.6676435e-7",
        "0x4.79be8E-6#20",
        Greater,
    );
    test(
        "355/113",
        53,
        Down,
        "2.6676418906242859e-7",
        "0x4.79be53e7514a0E-6#53",
        Less,
    );
    test(
        "355/113",
        53,
        Up,
        "2.6676418906242865e-7",
        "0x4.79be53e7514a4E-6#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Floor,
        "2.6676418906242859e-7",
        "0x4.79be53e7514a0E-6#53",
        Less,
    );
    test(
        "355/113",
        53,
        Ceiling,
        "2.6676418906242865e-7",
        "0x4.79be53e7514a4E-6#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Nearest,
        "2.6676418906242865e-7",
        "0x4.79be53e7514a4E-6#53",
        Greater,
    );
    test(
        "355/113",
        100,
        Nearest,
        "2.6676418906242864029404960194488e-7",
        "0x4.79be53e7514a37760d79f4f28E-6#100",
        Greater,
    );
    test("3", 1, Down, "-0.12", "-0x0.2#1", Greater);
    test("3", 1, Up, "-0.25", "-0x0.4#1", Less);
    test("3", 1, Floor, "-0.25", "-0x0.4#1", Less);
    test("3", 1, Ceiling, "-0.12", "-0x0.2#1", Greater);
    test("3", 1, Nearest, "-0.12", "-0x0.2#1", Greater);
    test("3", 5, Nearest, "-0.141", "-0x0.24#5", Greater);
    test("3", 10, Down, "-0.14233", "-0x0.247#10", Greater);
    test("3", 10, Up, "-0.14258", "-0x0.248#10", Less);
    test("3", 10, Floor, "-0.14258", "-0x0.248#10", Less);
    test("3", 10, Ceiling, "-0.14233", "-0x0.247#10", Greater);
    test("3", 10, Nearest, "-0.14258", "-0x0.248#10", Less);
    test("3", 20, Nearest, "-0.14254665", "-0x0.247df0#20", Less);
    test(
        "3",
        53,
        Down,
        "-0.14254654307427780",
        "-0x0.247dee24a970de#53",
        Greater,
    );
    test(
        "3",
        53,
        Up,
        "-0.14254654307427783",
        "-0x0.247dee24a970e0#53",
        Less,
    );
    test(
        "3",
        53,
        Floor,
        "-0.14254654307427783",
        "-0x0.247dee24a970e0#53",
        Less,
    );
    test(
        "3",
        53,
        Ceiling,
        "-0.14254654307427780",
        "-0x0.247dee24a970de#53",
        Greater,
    );
    test(
        "3",
        53,
        Nearest,
        "-0.14254654307427780",
        "-0x0.247dee24a970de#53",
        Greater,
    );
    test(
        "3",
        100,
        Nearest,
        "-0.14254654307427780529563541053388",
        "-0x0.247dee24a970de1996164fbff0#100",
        Greater,
    );
    test("100", 1, Down, "-0.50", "-0x0.8#1", Greater);
    test("100", 1, Up, "-1.0", "-0x1.0#1", Less);
    test("100", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("100", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("100", 1, Nearest, "-0.50", "-0x0.8#1", Greater);
    test("100", 5, Nearest, "-0.594", "-0x0.98#5", Less);
    test("100", 10, Down, "-0.58691", "-0x0.964#10", Greater);
    test("100", 10, Up, "-0.58789", "-0x0.968#10", Less);
    test("100", 10, Floor, "-0.58789", "-0x0.968#10", Less);
    test("100", 10, Ceiling, "-0.58691", "-0x0.964#10", Greater);
    test("100", 10, Nearest, "-0.58691", "-0x0.964#10", Greater);
    test("100", 20, Nearest, "-0.58721352", "-0x0.9653a#20", Greater);
    test(
        "100",
        53,
        Down,
        "-0.58721391515692900",
        "-0x0.9653a6b15ae9b8#53",
        Greater,
    );
    test(
        "100",
        53,
        Up,
        "-0.58721391515692911",
        "-0x0.9653a6b15ae9c0#53",
        Less,
    );
    test(
        "100",
        53,
        Floor,
        "-0.58721391515692911",
        "-0x0.9653a6b15ae9c0#53",
        Less,
    );
    test(
        "100",
        53,
        Ceiling,
        "-0.58721391515692900",
        "-0x0.9653a6b15ae9b8#53",
        Greater,
    );
    test(
        "100",
        53,
        Nearest,
        "-0.58721391515692911",
        "-0x0.9653a6b15ae9c0#53",
        Less,
    );
    test(
        "100",
        100,
        Nearest,
        "-0.58721391515692907667780963564448",
        "-0x0.9653a6b15ae9bd7c866895de4#100",
        Greater,
    );
    test("1000000", 1, Down, "-0.25", "-0x0.4#1", Greater);
    test("1000000", 1, Up, "-0.50", "-0x0.8#1", Less);
    test("1000000", 1, Floor, "-0.50", "-0x0.8#1", Less);
    test("1000000", 1, Ceiling, "-0.25", "-0x0.4#1", Greater);
    test("1000000", 1, Nearest, "-0.25", "-0x0.4#1", Greater);
    test("1000000", 5, Nearest, "-0.375", "-0x0.60#5", Less);
    test("1000000", 10, Down, "-0.37354", "-0x0.5fa#10", Greater);
    test("1000000", 10, Up, "-0.37402", "-0x0.5fc#10", Less);
    test("1000000", 10, Floor, "-0.37402", "-0x0.5fc#10", Less);
    test("1000000", 10, Ceiling, "-0.37354", "-0x0.5fa#10", Greater);
    test("1000000", 10, Nearest, "-0.37354", "-0x0.5fa#10", Greater);
    test(
        "1000000",
        20,
        Nearest,
        "-0.37362432",
        "-0x0.5fa5d8#20",
        Greater,
    );
    test(
        "1000000",
        53,
        Down,
        "-0.37362445398759903",
        "-0x0.5fa5da2adcd300#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Up,
        "-0.37362445398759908",
        "-0x0.5fa5da2adcd304#53",
        Less,
    );
    test(
        "1000000",
        53,
        Floor,
        "-0.37362445398759908",
        "-0x0.5fa5da2adcd304#53",
        Less,
    );
    test(
        "1000000",
        53,
        Ceiling,
        "-0.37362445398759903",
        "-0x0.5fa5da2adcd300#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Nearest,
        "-0.37362445398759903",
        "-0x0.5fa5da2adcd300#53",
        Greater,
    );
    test(
        "1000000",
        100,
        Nearest,
        "-0.37362445398759902917349708857555",
        "-0x0.5fa5da2adcd3004202c27b19e0#100",
        Less,
    );
    test("1/1000000", 1, Down, "9.5e-7", "0x0.00001#1", Less);
    test("1/1000000", 1, Up, "1.9e-6", "0x0.00002#1", Greater);
    test("1/1000000", 1, Floor, "9.5e-7", "0x0.00001#1", Less);
    test("1/1000000", 1, Ceiling, "1.9e-6", "0x0.00002#1", Greater);
    test("1/1000000", 1, Nearest, "9.5e-7", "0x0.00001#1", Less);
    test("1/1000000", 5, Nearest, "1.01e-6", "0x0.000011#5", Greater);
    test("1/1000000", 10, Down, "9.9838e-7", "0x0.000010c0#10", Less);
    test("1/1000000", 10, Up, "1.0002e-6", "0x0.000010c8#10", Greater);
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
        20,
        Nearest,
        "1.0000003e-6",
        "0x0.000010c6f8#20",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Down,
        "1.0000000000003333e-6",
        "0x0.000010c6f7a0b5f3b3#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Up,
        "1.0000000000003335e-6",
        "0x0.000010c6f7a0b5f3b4#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Floor,
        "1.0000000000003333e-6",
        "0x0.000010c6f7a0b5f3b3#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Ceiling,
        "1.0000000000003335e-6",
        "0x0.000010c6f7a0b5f3b4#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Nearest,
        "1.0000000000003333e-6",
        "0x0.000010c6f7a0b5f3b3#53",
        Less,
    );
    test(
        "1/1000000",
        100,
        Nearest,
        "1.0000000000003333333333334666664e-6",
        "0x0.000010c6f7a0b5f3b355fab8b736bc#100",
        Less,
    );
    test("-1/1000000", 1, Down, "-9.5e-7", "-0x0.00001#1", Greater);
    test("-1/1000000", 1, Up, "-1.9e-6", "-0x0.00002#1", Less);
    test("-1/1000000", 1, Floor, "-1.9e-6", "-0x0.00002#1", Less);
    test("-1/1000000", 1, Ceiling, "-9.5e-7", "-0x0.00001#1", Greater);
    test("-1/1000000", 1, Nearest, "-9.5e-7", "-0x0.00001#1", Greater);
    test("-1/1000000", 5, Nearest, "-1.01e-6", "-0x0.000011#5", Less);
    test(
        "-1/1000000",
        10,
        Down,
        "-9.9838e-7",
        "-0x0.000010c0#10",
        Greater,
    );
    test("-1/1000000", 10, Up, "-1.0002e-6", "-0x0.000010c8#10", Less);
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
        20,
        Nearest,
        "-1.0000003e-6",
        "-0x0.000010c6f8#20",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Down,
        "-1.0000000000003333e-6",
        "-0x0.000010c6f7a0b5f3b3#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Up,
        "-1.0000000000003335e-6",
        "-0x0.000010c6f7a0b5f3b4#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Floor,
        "-1.0000000000003335e-6",
        "-0x0.000010c6f7a0b5f3b4#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Ceiling,
        "-1.0000000000003333e-6",
        "-0x0.000010c6f7a0b5f3b3#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Nearest,
        "-1.0000000000003333e-6",
        "-0x0.000010c6f7a0b5f3b3#53",
        Greater,
    );
    test(
        "-1/1000000",
        100,
        Nearest,
        "-1.0000000000003333333333334666664e-6",
        "-0x0.000010c6f7a0b5f3b355fab8b736bc#100",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Down,
        "8.3e-25",
        "0x1.0E-20#1",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Up,
        "1.7e-24",
        "0x2.0E-20#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Floor,
        "8.3e-25",
        "0x1.0E-20#1",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Ceiling,
        "1.7e-24",
        "0x2.0E-20#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Nearest,
        "8.3e-25",
        "0x1.0E-20#1",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        5,
        Nearest,
        "9.82e-25",
        "0x1.3E-20#5",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Down,
        "9.9843e-25",
        "0x1.350E-20#10",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Up,
        "1.0000e-24",
        "0x1.358E-20#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Floor,
        "9.9843e-25",
        "0x1.350E-20#10",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Ceiling,
        "1.0000e-24",
        "0x1.358E-20#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Nearest,
        "1.0000e-24",
        "0x1.358E-20#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        20,
        Nearest,
        "9.9999953e-25",
        "0x1.357c2E-20#20",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Down,
        "9.9999999999999992e-25",
        "0x1.357c299a88ea7E-20#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Up,
        "1.0000000000000001e-24",
        "0x1.357c299a88ea8E-20#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Floor,
        "9.9999999999999992e-25",
        "0x1.357c299a88ea7E-20#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Ceiling,
        "1.0000000000000001e-24",
        "0x1.357c299a88ea8E-20#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Nearest,
        "9.9999999999999992e-25",
        "0x1.357c299a88ea7E-20#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        100,
        Nearest,
        "9.9999999999999999999999999999980e-25",
        "0x1.357c299a88ea76a58924d52ceE-20#100",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Down,
        "1.0e31",
        "0x8.0E+25#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Up,
        "2.0e31",
        "0x1.0E+26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Floor,
        "1.0e31",
        "0x8.0E+25#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Ceiling,
        "2.0e31",
        "0x1.0E+26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Nearest,
        "1.0e31",
        "0x8.0E+25#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        5,
        Nearest,
        "1.20e31",
        "0x9.8E+25#5",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Down,
        "1.1785e31",
        "0x9.4cE+25#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "1.1805e31",
        "0x9.50E+25#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "1.1785e31",
        "0x9.4cE+25#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "1.1805e31",
        "0x9.50E+25#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "1.1785e31",
        "0x9.4cE+25#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "1.1794648e31",
        "0x9.4de9E+25#20",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "1.1794639761737254e31",
        "0x9.4de8957cbf8e0E+25#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "1.1794639761737256e31",
        "0x9.4de8957cbf8e8E+25#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "1.1794639761737254e31",
        "0x9.4de8957cbf8e0E+25#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "1.1794639761737256e31",
        "0x9.4de8957cbf8e8E+25#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "1.1794639761737256e31",
        "0x9.4de8957cbf8e8E+25#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "11794639761737256078832190295792.0",
        "0x9.4de8957cbf8e6cddc32b70efE+25#100",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Down,
        "2.5e30",
        "0x2.0E+25#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Up,
        "5.1e30",
        "0x4.0E+25#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Floor,
        "2.5e30",
        "0x2.0E+25#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Ceiling,
        "5.1e30",
        "0x4.0E+25#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Nearest,
        "5.1e30",
        "0x4.0E+25#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        5,
        Nearest,
        "3.96e30",
        "0x3.2E+25#5",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Down,
        "3.9267e30",
        "0x3.19E+25#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Up,
        "3.9317e30",
        "0x3.1aE+25#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Floor,
        "3.9267e30",
        "0x3.19E+25#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Ceiling,
        "3.9317e30",
        "0x3.1aE+25#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Nearest,
        "3.9317e30",
        "0x3.1aE+25#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        20,
        Nearest,
        "3.9315477e30",
        "0x3.19f84E+25#20",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Down,
        "3.9315465872457520e30",
        "0x3.19f831d43fda2E+25#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Up,
        "3.9315465872457525e30",
        "0x3.19f831d43fda4E+25#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Floor,
        "3.9315465872457520e30",
        "0x3.19f831d43fda2E+25#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Ceiling,
        "3.9315465872457525e30",
        "0x3.19f831d43fda4E+25#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Nearest,
        "3.9315465872457520e30",
        "0x3.19f831d43fda2E+25#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        100,
        Nearest,
        "3931546587245752026277396765264.0",
        "0x319f831d43fda2449ebb925a50.0#100",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Down,
        "-9.9e-32",
        "-0x2.0E-26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Up,
        "-2.0e-31",
        "-0x4.0E-26#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Floor,
        "-2.0e-31",
        "-0x4.0E-26#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Ceiling,
        "-9.9e-32",
        "-0x2.0E-26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Nearest,
        "-2.0e-31",
        "-0x4.0E-26#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        5,
        Nearest,
        "-1.73e-31",
        "-0x3.8E-26#5",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Down,
        "-1.6948e-31",
        "-0x3.70E-26#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Up,
        "-1.6967e-31",
        "-0x3.71E-26#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Floor,
        "-1.6967e-31",
        "-0x3.71E-26#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Ceiling,
        "-1.6948e-31",
        "-0x3.70E-26#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Nearest,
        "-1.6948e-31",
        "-0x3.70E-26#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        20,
        Nearest,
        "-1.6956854e-31",
        "-0x3.70734E-26#20",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Down,
        "-1.6956855320737799e-31",
        "-0x3.707344a409382E-26#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Up,
        "-1.6956855320737801e-31",
        "-0x3.707344a409384E-26#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Floor,
        "-1.6956855320737801e-31",
        "-0x3.707344a409384E-26#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Ceiling,
        "-1.6956855320737799e-31",
        "-0x3.707344a409382E-26#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Nearest,
        "-1.6956855320737799e-31",
        "-0x3.707344a409382E-26#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        100,
        Nearest,
        "-1.6956855320737799287917402938778e-31",
        "-0x3.707344a4093822299f31d0084E-26#100",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Down,
        "-1.0e31",
        "-0x8.0E+25#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Up,
        "-2.0e31",
        "-0x1.0E+26#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Floor,
        "-2.0e31",
        "-0x1.0E+26#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Ceiling,
        "-1.0e31",
        "-0x8.0E+25#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Nearest,
        "-1.0e31",
        "-0x8.0E+25#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        5,
        Nearest,
        "-1.20e31",
        "-0x9.8E+25#5",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Down,
        "-1.1785e31",
        "-0x9.4cE+25#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "-1.1805e31",
        "-0x9.50E+25#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "-1.1805e31",
        "-0x9.50E+25#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "-1.1785e31",
        "-0x9.4cE+25#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "-1.1785e31",
        "-0x9.4cE+25#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "-1.1794648e31",
        "-0x9.4de9E+25#20",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "-1.1794639761737254e31",
        "-0x9.4de8957cbf8e0E+25#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "-1.1794639761737256e31",
        "-0x9.4de8957cbf8e8E+25#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "-1.1794639761737256e31",
        "-0x9.4de8957cbf8e8E+25#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "-1.1794639761737254e31",
        "-0x9.4de8957cbf8e0E+25#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "-1.1794639761737256e31",
        "-0x9.4de8957cbf8e8E+25#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "-11794639761737256078832190295792.0",
        "-0x9.4de8957cbf8e6cddc32b70efE+25#100",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Down,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Up,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Floor,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Ceiling,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Nearest,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        5,
        Nearest,
        "-1.81",
        "-0x1.d#5",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Down,
        "-1.7812",
        "-0x1.c80#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Up,
        "-1.7832",
        "-0x1.c88#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Floor,
        "-1.7832",
        "-0x1.c88#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Ceiling,
        "-1.7812",
        "-0x1.c80#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Nearest,
        "-1.7832",
        "-0x1.c88#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        20,
        Nearest,
        "-1.7829552",
        "-0x1.c86fc#20",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Down,
        "-1.7829551493767191",
        "-0x1.c86fbfa8cecc3#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Up,
        "-1.7829551493767193",
        "-0x1.c86fbfa8cecc4#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Floor,
        "-1.7829551493767193",
        "-0x1.c86fbfa8cecc4#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Ceiling,
        "-1.7829551493767191",
        "-0x1.c86fbfa8cecc3#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Nearest,
        "-1.7829551493767191",
        "-0x1.c86fbfa8cecc3#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        100,
        Nearest,
        "-1.7829551493767190886252628916712",
        "-0x1.c86fbfa8cecc31efc7a121160#100",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Down,
        "7.9e-31",
        "0x1.0E-25#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Up,
        "1.6e-30",
        "0x2.0E-25#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Floor,
        "7.9e-31",
        "0x1.0E-25#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Ceiling,
        "1.6e-30",
        "0x2.0E-25#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Nearest,
        "7.9e-31",
        "0x1.0E-25#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        5,
        Nearest,
        "7.89e-31",
        "0x1.0E-25#5",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Down,
        "7.8886e-31",
        "0x1.000E-25#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Up,
        "7.9040e-31",
        "0x1.008E-25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Floor,
        "7.8886e-31",
        "0x1.000E-25#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Ceiling,
        "7.9040e-31",
        "0x1.008E-25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Nearest,
        "7.8886e-31",
        "0x1.000E-25#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        20,
        Nearest,
        "7.8886091e-31",
        "0x1.00000E-25#20",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Down,
        "7.8886090522101181e-31",
        "0x1.0000000000000E-25#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Up,
        "7.8886090522101198e-31",
        "0x1.0000000000001E-25#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Floor,
        "7.8886090522101181e-31",
        "0x1.0000000000000E-25#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Ceiling,
        "7.8886090522101198e-31",
        "0x1.0000000000001E-25#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Nearest,
        "7.8886090522101181e-31",
        "0x1.0000000000000E-25#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        100,
        Nearest,
        "7.8886090522101180541172856528279e-31",
        "0x1.0000000000000000000000000E-25#100",
        Less,
    );
}

// Inputs of magnitude around 2^(-2^30) or less, whose tangents underflow: they take the tiny
// bracket, where everything is `Rational` arithmetic, so no 2^30-bit `Float` is ever formed. The
// cube and fifth power of such an input have denominators three and five times its own, which makes
// this test slow and memory-hungry despite its handful of rows.
#[test]
fn test_tan_rational_tiny_underflow() {
    let test = |x: &Rational, prec, rm, out: Float, out_o| {
        let (t, o) = Float::tan_rational_prec_round_ref(x, prec, rm);
        assert!(t.is_valid());
        assert_eq!(ComparableFloat(t), ComparableFloat(out));
        assert_eq!(o, out_o);
    };
    let min_exp = -(1i64 << 30);
    let min_positive = |prec| Float::one_prec(prec) >> (1u64 << 30);
    // x = 2^(-2^30), the smallest positive Float: tan(x) is just above it, so rounding up gives the
    // next `Float` of the output precision rather than twice the smallest one
    let x = Rational::power_of_2(min_exp);
    test(&x, 1, Floor, min_positive(1), Less);
    test(&x, 10, Nearest, min_positive(10), Less);
    let mut next_up = min_positive(10);
    next_up.increment();
    test(&x, 10, Ceiling, next_up, Greater);
    test(&-&x, 1, Down, -min_positive(1), Greater);
    // x = 2^(-2^30 - 1): tan(x) is just above half the smallest positive Float, so it rounds up
    let x = Rational::power_of_2(min_exp - 1);
    test(&x, 1, Nearest, min_positive(1), Greater);
    test(&x, 10, Down, Float::ZERO, Less);
    // inputs far below the smallest positive Float are decided by the rounding mode alone
    let x = Rational::power_of_2(min_exp - 5);
    test(&x, 10, Ceiling, min_positive(10), Greater);
    test(&x, 10, Nearest, Float::ZERO, Less);
    test(&-&x, 10, Nearest, Float::NEGATIVE_ZERO, Greater);
    test(&-&x, 10, Floor, -min_positive(10), Less);
    test(
        &Rational::power_of_2(min_exp << 1),
        1,
        Up,
        min_positive(1),
        Greater,
    );
}

#[test]
#[should_panic]
fn tan_rational_prec_fail() {
    Float::tan_rational_prec(Rational::ONE, 0);
}

#[test]
#[should_panic]
fn tan_rational_prec_ref_fail() {
    Float::tan_rational_prec_ref(&Rational::ONE, 0);
}

#[test]
#[should_panic]
fn tan_rational_prec_round_fail_1() {
    Float::tan_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn tan_rational_prec_round_fail_2() {
    Float::tan_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn tan_rational_prec_round_ref_fail() {
    Float::tan_rational_prec_round_ref(&Rational::ONE, 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn tan_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (s, o) = Float::tan_rational_prec_round(x.clone(), prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = Float::tan_rational_prec_round_ref(&x, prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    // tan is odd (a `Rational` has no negative zero, so x = 0 is excluded)
    if x != 0u32 {
        let (s_neg, o_neg) = Float::tan_rational_prec_round(-&x, prec, -rm);
        assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
        assert_eq!(o_neg, o.reverse());
    }

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_o) = rug_tan_rational_prec_round(&x, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o, "x = {x} prec = {prec} rm = {rm:?}");
    }

    if s.is_normal() {
        assert_eq!(s.get_prec(), Some(prec));
    }

    if o == Equal {
        // only tan(0) = 0 is exact
        assert_eq!(x, 0u32);
        for rm in exhaustive_rounding_modes() {
            let (s2, oo) = Float::tan_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::tan_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn tan_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        tan_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::tan_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn tan_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (s, o) = Float::tan_rational_prec(x.clone(), prec);
    assert!(s.is_valid());

    let (s_alt, o_alt) = Float::tan_rational_prec_ref(&x, prec);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let (s_alt, o_alt) = Float::tan_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let (rug_s, rug_o) = rug_tan_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_s)),
        ComparableFloatRef(&s)
    );
    assert_eq!(rug_o, o, "x = {x} prec = {prec}");

    // the tangent of an exactly representable rational is the Float tangent
    if let Ok(f) = Float::try_from(&x) {
        let (s_alt, o_alt) = f.tan_prec(prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
    }
}

#[test]
fn tan_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        tan_rational_prec_properties_helper(x, prec);
    });
}

// An input too large to be a `Float`, reduced modulo 2 pi in `Rational` arithmetic with pi to about
// 2^30 bits; slow even in release mode.
#[test]
fn test_tan_rational_huge() {
    let x = Rational::power_of_2(1i64 << 30);
    let (t, o) = Float::tan_rational_prec_round_ref(&x, 10, Nearest);
    assert_eq!(t.to_string(), "-0.80664");
    assert_eq!(to_hex_string(&t), "-0x0.ce8#10");
    assert_eq!(o, Less);
}

// Inputs within 2^(-2^30) of pi and of pi/2, whose tangents underflow and overflow. Each call
// computes pi to about 2^30 bits twice, so this test is slow even in release mode.
#[test]
fn test_tan_rational_underflow_and_overflow() {
    let p = (1u64 << 30) + 64;
    // pi rounded down: x < pi, so the tangent is negative and tiny
    let pi = Float::pi_prec_round(p, Floor).0;
    let x = Rational::exact_from(&pi);
    let (t, o) = Float::tan_rational_prec_round_ref(&x, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    // pi/2 rounded down: the tangent is positive and huge, beyond the largest finite Float
    let x = Rational::exact_from(&(pi >> 1u32));
    let (t, o) = Float::tan_rational_prec_round_ref(&x, 10, Down);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::max_finite_value_with_prec(10))
    );
    assert_eq!(o, Less);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_tan_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_tan_rational::<T>(&x)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0.0);
    test::<f64>("0", 0.0);
    test::<f32>("1", 1.5574077);
    test::<f64>("1", 1.5574077246549023);
    test::<f32>("-1", -1.5574077);
    test::<f64>("-1", -1.5574077246549023);
    test::<f32>("1/2", 0.5463025);
    test::<f64>("1/2", 0.5463024898437905);
    test::<f32>("1/3", 0.34625354);
    test::<f64>("1/3", 0.34625354951057546);
    test::<f32>("22/7", 0.00126449);
    test::<f64>("22/7", 0.0012644899412946342);
    test::<f32>("1/7", 0.14383696);
    test::<f64>("1/7", 0.14383695943619093);
    test::<f32>("100", -0.58721393);
    test::<f64>("100", -0.5872139151569291);
    test::<f32>("355/113", 2.6676418e-7);
    test::<f64>("355/113", 2.6676418906242865e-7);
    test::<f32>("1/1000000", 0.000001);
    test::<f64>("1/1000000", 1.0000000000003333e-6);
    test::<f32>("-2/3", -0.7868429);
    test::<f64>("-2/3", -0.7868428894729773);
    test::<f32>("1000000", -0.37362444);
    test::<f64>("1000000", -0.373624453987599);
    test::<f32>("1/100000000000000000000", 1.0e-20);
    test::<f64>("1/100000000000000000000", 1.0e-20);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_tan_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let s = primitive_float_tan_rational::<T>(&x);
        // the tangent of a rational is never NaN
        assert!(!s.is_nan());
        // tan is odd (a `Rational` has no negative zero, so x = 0 is excluded)
        if x != 0u32 {
            assert_eq!(
                NiceFloat(primitive_float_tan_rational::<T>(&-&x)),
                NiceFloat(-s)
            );
        }
        // the result is the correctly rounded tangent, as computed by MPFR with 64 bits to spare,
        // so that a subnormal result is rounded once by the conversion rather than twice
        let rug_s = rug_tan_rational_prec(&x, T::MANTISSA_WIDTH + 64).0;
        let rug_s: T = T::rounding_from(&<Float as From<&rug::Float>>::from(&rug_s), Nearest).0;
        assert_eq!(NiceFloat(rug_s), NiceFloat(s));
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // The tangent of a finite nonzero primitive float, taken through the `Rational` path,
        // matches the direct primitive-float tangent (a `Rational` cannot carry the sign of a
        // zero).
        if x.is_finite() && x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_tan_rational::<T>(&Rational::exact_from(x))),
                NiceFloat(primitive_float_tan(x))
            );
        }
    });
}

#[test]
fn primitive_float_tan_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_tan_rational_properties_helper);
}
// Rows reuse the sine's `with_period` inputs, which cover the exact cases (quarter, eighth, and
// twelfth turns), the general Ziv loop at its first working precision and after a retry, and the
// tiny x/u whose tangent is 2 pi x/u to far more bits than any precision needs, including the
// inputs whose tangents underflow.
#[test]
fn test_tan_with_period_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (t, o) = x.clone().tan_with_period_prec_round(u, prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.tan_with_period_prec_round_ref(u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.tan_with_period_prec_round_assign(u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u32::try_from(u).is_ok()
        {
            let (rug_t, rug_o) =
                rug_tan_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 4, 10, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 4, 10, Nearest, "NaN", "NaN", Equal);
    test(
        "-Infinity",
        "-Infinity",
        4,
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("0.0", "0x0.0", 4, 10, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 4, 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 360, 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 360, 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "90.0", "0x5a.0#6", 360, 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "90.0", "0x5a.0#6", 360, 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "90.0", "0x5a.0#6", 360, 10, Exact, "Infinity", "Infinity", Equal,
    );
    test(
        "180.0", "0xb4.0#6", 360, 10, Nearest, "-0.0", "-0x0.0", Equal,
    );
    test("180.0", "0xb4.0#6", 360, 10, Floor, "-0.0", "-0x0.0", Equal);
    test("180.0", "0xb4.0#6", 360, 10, Exact, "-0.0", "-0x0.0", Equal);
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "360.0",
        "0x168.0#6",
        360,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test("360.0", "0x168.0#6", 360, 10, Floor, "0.0", "0x0.0", Equal);
    test("360.0", "0x168.0#6", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "60.0",
        "0x3c.0#4",
        360,
        10,
        Nearest,
        "1.7324",
        "0x1.bb8#10",
        Greater,
    );
    test(
        "60.0",
        "0x3c.0#4",
        360,
        10,
        Floor,
        "1.7305",
        "0x1.bb0#10",
        Less,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Nearest,
        "-1.7324",
        "-0x1.bb8#10",
        Less,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Floor,
        "-1.7324",
        "-0x1.bb8#10",
        Less,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Nearest,
        "1.7324",
        "0x1.bb8#10",
        Greater,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Floor,
        "1.7305",
        "0x1.bb0#10",
        Less,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Nearest,
        "-1.7324",
        "-0x1.bb8#10",
        Less,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Floor,
        "-1.7324",
        "-0x1.bb8#10",
        Less,
    );
    test(
        "450.0",
        "0x1c2.0#8",
        360,
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "450.0",
        "0x1c2.0#8",
        360,
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "450.0",
        "0x1c2.0#8",
        360,
        10,
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );
    test("1.0", "0x1.0#1", 1, 10, Ceiling, "0.0", "0x0.0", Equal);
    test(
        "1.0",
        "0x1.0#1",
        3,
        10,
        Nearest,
        "-1.7324",
        "-0x1.bb8#10",
        Less,
    );
    test(
        "1.0", "0x1.0#1", 4, 10, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        7,
        10,
        Nearest,
        "1.2539",
        "0x1.410#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        360,
        10,
        Nearest,
        "0.017456",
        "0x0.0478#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        1000000,
        10,
        Ceiling,
        "6.2883e-6",
        "0x0.0000698#10",
        Greater,
    );
    test("0.50", "0x0.8#1", 1, 10, Floor, "-0.0", "-0x0.0", Equal);
    test(
        "0.50", "0x0.8#1", 2, 53, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        4,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        6,
        53,
        Nearest,
        "0.57735026918962573",
        "0x0.93cd3a2c8198e0#53",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        12,
        10,
        Ceiling,
        "0.26807",
        "0x0.44a#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        1000000,
        10,
        Floor,
        "3.1404e-6",
        "0x0.000034b#10",
        Less,
    );
    test(
        "0.25", "0x0.4#1", 1, 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        2,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        4,
        10,
        Nearest,
        "0.41406",
        "0x0.6a0#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        6,
        10,
        Ceiling,
        "0.26807",
        "0x0.44a#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        12,
        10,
        Floor,
        "0.13159",
        "0x0.21b#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        1000000,
        10,
        Nearest,
        "1.5702e-6",
        "0x0.00001a58#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        1099511627776,
        53,
        Nearest,
        "1.4286309367843356e-12",
        "0x1.921fb54442d18E-10#53",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        2,
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("1.5", "0x1.8#2", 3, 53, Nearest, "-0.0", "-0x0.0", Equal);
    test(
        "1.5", "0x1.8#2", 6, 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        12,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        360,
        53,
        Nearest,
        "0.026185921569186928",
        "0x0.06b41edcc159f48#53",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        1099511627776,
        10,
        Ceiling,
        "8.5834e-12",
        "0x9.70E-10#10",
        Greater,
    );
    test("2.0", "0x2.0#1", 2, 10, Nearest, "0.0", "0x0.0", Equal);
    test(
        "2.0",
        "0x2.0#1",
        3,
        10,
        Ceiling,
        "1.7324",
        "0x1.bb8#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        6,
        10,
        Nearest,
        "-1.7324",
        "-0x1.bb8#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        7,
        10,
        Ceiling,
        "-4.3750",
        "-0x4.60#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Ceiling,
        "0.034973",
        "0x0.08f4#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        1099511627776,
        10,
        Floor,
        "1.1426e-11",
        "0xc.90E-10#10",
        Less,
    );
    test("3.0", "0x3.0#2", 1, 53, Nearest, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 3, 10, Floor, "0.0", "0x0.0", Equal);
    test(
        "3.0",
        "0x3.0#2",
        4,
        53,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        7,
        10,
        Floor,
        "-0.48193",
        "-0x0.7b6#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        360,
        10,
        Floor,
        "0.052368",
        "0x0.0d68#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        1099511627776,
        10,
        Nearest,
        "1.7138e-11",
        "0x1.2d8E-9#10",
        Less,
    );
    test("-1.0", "-0x1.0#1", 1, 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        3,
        10,
        Nearest,
        "1.7324",
        "0x1.bb8#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        4,
        10,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        7,
        10,
        Nearest,
        "-1.2539",
        "-0x1.410#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        360,
        10,
        Nearest,
        "-0.017456",
        "-0x0.0478#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1000000,
        10,
        Ceiling,
        "-6.2808e-6",
        "-0x0.0000696#10",
        Greater,
    );
    test("100.0", "0x64.0#5", 1, 10, Floor, "0.0", "0x0.0", Equal);
    test("100.0", "0x64.0#5", 2, 53, Nearest, "0.0", "0x0.0", Equal);
    test("100.0", "0x64.0#5", 4, 10, Floor, "0.0", "0x0.0", Equal);
    test(
        "100.0",
        "0x64.0#5",
        6,
        53,
        Nearest,
        "1.7320508075688772",
        "0x1.bb67ae8584caa#53",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        12,
        10,
        Ceiling,
        "-1.7305",
        "-0x1.bb0#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        1000000,
        10,
        Floor,
        "0.00062752",
        "0x0.00292#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        10,
        Nearest,
        "-0.28369",
        "-0x0.48a#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        2,
        10,
        Ceiling,
        "7.1953",
        "0x7.32#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        4,
        10,
        Nearest,
        "-1.1484",
        "-0x1.260#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        6,
        10,
        Ceiling,
        "0.51758",
        "0x0.848#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        12,
        10,
        Floor,
        "-4.1094",
        "-0x4.1c#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1000000,
        10,
        Nearest,
        "0.00077534",
        "0x0.0032d#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1099511627776,
        53,
        Nearest,
        "7.0549224372658779e-10",
        "0x3.07b269b202f0eE-8#53",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        2,
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        3,
        53,
        Nearest,
        "-1.7320508075688772",
        "-0x1.bb67ae8584caa#53",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        6,
        10,
        Floor,
        "1.7305",
        "0x1.bb0#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        12,
        10,
        Nearest,
        "-1.7324",
        "-0x1.bb8#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        360,
        53,
        Nearest,
        "-5.6712818196177093",
        "-0x5.abd92015a84d8#53",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1099511627776,
        10,
        Ceiling,
        "0.057251",
        "0x0.0ea8#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        2,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        3,
        10,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        6,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        7,
        10,
        Ceiling,
        "-0.79688",
        "-0x0.cc0#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        360,
        10,
        Ceiling,
        "-0.013077",
        "-0x0.0359#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1099511627776,
        10,
        Floor,
        "-4.2917e-12",
        "-0x4.b8E-10#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1,
        53,
        Nearest,
        "6.2831853071795868e-10",
        "0x2.b2d7f19cec63cE-8#53",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        3,
        10,
        Floor,
        "2.0941e-10",
        "0xe.64E-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        4,
        53,
        Nearest,
        "1.5707963267948967e-10",
        "0xa.cb5fc673b18f0E-9#53",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        7,
        10,
        Floor,
        "8.9699e-11",
        "0x6.2aE-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        360,
        10,
        Floor,
        "1.7444e-12",
        "0x1.eb0E-10#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1099511627776,
        10,
        Nearest,
        "5.7158e-22",
        "0x2.b3E-18#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        1,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "0.12",
        "0x0.2#1",
        3,
        10,
        Nearest,
        "0.26807",
        "0x0.44a#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        4,
        10,
        Ceiling,
        "0.19897",
        "0x0.32f#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        7,
        10,
        Nearest,
        "0.11267",
        "0x0.1cd8#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        360,
        10,
        Nearest,
        "0.0021820",
        "0x0.008f0#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        1000000,
        10,
        Ceiling,
        "7.8604e-7",
        "0xd.30E-6#10",
        Greater,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        1,
        53,
        Floor,
        "6.2831853071795868e-30",
        "0x7.f7029d0214354E-25#53",
        Less,
    );
    test(
        "-90.000000000000000000000000000808",
        "-0x5a.000000000000000000000040#100",
        360,
        53,
        Nearest,
        "7.0928739541311617e28",
        "0xe.52ee0d31e0fc0E+23#53",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        8,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        8,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test("5.0", "0x5.0#3", 8, 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("5.0", "0x5.0#3", 8, 10, Up, "1.0000", "0x1.000#10", Equal);
    test(
        "7.0",
        "0x7.0#3",
        8,
        10,
        Ceiling,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-9.00",
        "-0x9.0#4",
        8,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test("1.0", "0x1.0#1", 12, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Up,
        "0.57812",
        "0x0.940#10",
        Greater,
    );
    test(
        "5.0",
        "0x5.0#3",
        12,
        10,
        Ceiling,
        "-0.57715",
        "-0x0.93c#10",
        Greater,
    );
    test(
        "-7.0",
        "-0x7.0#3",
        12,
        10,
        Nearest,
        "-0.57715",
        "-0x0.93c#10",
        Greater,
    );
    test(
        "11.0", "0xb.0#4", 12, 1, Nearest, "-0.50", "-0x0.8#1", Greater,
    );
    test(
        "11.0",
        "0xb.0#4",
        12,
        10,
        Up,
        "-0.57812",
        "-0x0.940#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        10,
        Ceiling,
        "3.0781",
        "0x3.14#10",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        5,
        10,
        Nearest,
        "0.72656",
        "0x0.ba0#10",
        Greater,
    );
    test("3.0", "0x3.0#2", 5, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "3.0",
        "0x3.0#2",
        5,
        10,
        Up,
        "0.72656",
        "0x0.ba0#10",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        5,
        10,
        Ceiling,
        "-3.0742",
        "-0x3.13#10",
        Greater,
    );
    test(
        "-6.0",
        "-0x6.0#2",
        5,
        10,
        Nearest,
        "-3.0781",
        "-0x3.14#10",
        Less,
    );
    test("1.0", "0x1.0#1", 10, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Up,
        "0.72656",
        "0x0.ba0#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        10,
        Ceiling,
        "-3.0742",
        "-0x3.13#10",
        Greater,
    );
    test(
        "-7.0",
        "-0x7.0#3",
        10,
        10,
        Nearest,
        "-3.0781",
        "-0x3.14#10",
        Less,
    );
    test(
        "9.00", "0x9.0#4", 10, 1, Nearest, "-0.50", "-0x0.8#1", Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        10,
        Up,
        "-0.72656",
        "-0x0.ba0#10",
        Less,
    );
    test(
        "45.0",
        "0x2d.0#6",
        360,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "-135.0",
        "-0x87.0#8",
        360,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test("30.0", "0x1e.0#4", 360, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "30.0",
        "0x1e.0#4",
        360,
        10,
        Up,
        "0.57812",
        "0x0.940#10",
        Greater,
    );
    test(
        "150.0",
        "0x96.0#7",
        360,
        10,
        Ceiling,
        "-0.57715",
        "-0x0.93c#10",
        Greater,
    );
    test(
        "-72.0",
        "-0x48.0#4",
        360,
        10,
        Nearest,
        "-3.0781",
        "-0x3.14#10",
        Less,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        1,
        Nearest,
        "-0.50",
        "-0x0.8#1",
        Greater,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        10,
        Up,
        "-0.72656",
        "-0x0.ba0#10",
        Less,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        10,
        Ceiling,
        "0.72656",
        "0x0.ba0#10",
        Greater,
    );
    test(
        "-108.0",
        "-0x6c.0#5",
        360,
        10,
        Nearest,
        "3.0781",
        "0x3.14#10",
        Greater,
    );
    test("2.0", "0x2.0#1", 16, 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", 16, 10, Up, "1.0000", "0x1.000#10", Equal);
    test(
        "3.0",
        "0x3.0#2",
        24,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "-4.0",
        "-0x4.0#1",
        20,
        10,
        Nearest,
        "-3.0781",
        "-0x3.14#10",
        Less,
    );
    test("6.0", "0x6.0#2", 60, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "6.0",
        "0x6.0#2",
        60,
        10,
        Up,
        "0.72656",
        "0x0.ba0#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        20,
        30,
        Ceiling,
        "1.3763819207",
        "0x1.605a90c8#30",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        20,
        30,
        Ceiling,
        "-0.32491969597",
        "-0x0.532defec#30",
        Greater,
    );
    test(
        "13.0",
        "0xd.0#4",
        20,
        30,
        Ceiling,
        "1.3763819207",
        "0x1.605a90c8#30",
        Greater,
    );
    test(
        "19.0",
        "0x13.0#5",
        20,
        30,
        Ceiling,
        "-0.32491969597",
        "-0x0.532defec#30",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        6,
        30,
        Floor,
        "1.7320508063",
        "0x1.bb67ae80#30",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        8,
        30,
        Nearest,
        "1.0000000000",
        "0x1.00000000#30",
        Equal,
    );
    test("2.0", "0x2.0#2", 4, 10, Exact, "-0.0", "-0x0.0", Equal);
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        100,
        10,
        Floor,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        100,
        10,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        4,
        1,
        Up,
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Greater,
    );
}

// x/u approaching a multiple of 1/4 turn, where the tangent is tiny (at a multiple of 1/2) or huge
// (at an odd multiple of 1/4): x = n +- 2^-k with u = 4, so that x/u = n/4 +- 2^(-k-2) exactly and
// the near-zero paths of the sine and cosine work with that exact distance.
#[test]
fn test_tan_with_period_near_quarter_turn() {
    let test = |n: i64,
                k: u64,
                above: bool,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let eps = Rational::power_of_2(-i64::exact_from(k));
        let q = if above {
            Rational::from(n) + eps
        } else {
            Rational::from(n) - eps
        };
        let x = Float::from_rational_prec_round(q, k + 4, Exact).0;
        let (t, o) = x.tan_with_period_prec_round_ref(4, prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (rug_t, rug_o) = rug_tan_with_period_prec_round(
            &rug::Float::exact_from(&x),
            4,
            prec,
            rug_round_try_from_rounding_mode(rm).unwrap(),
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_t)),
            ComparableFloatRef(&t)
        );
        assert_eq!(rug_o, o);
    };
    test(0, 20, true, 1, Nearest, "1.9e-6", "0x0.00002#1", Greater);
    test(0, 20, true, 1, Floor, "9.5e-7", "0x0.00001#1", Less);
    test(
        0,
        20,
        true,
        10,
        Nearest,
        "1.4976e-6",
        "0x0.00001920#10",
        Less,
    );
    test(0, 20, true, 10, Floor, "1.4976e-6", "0x0.00001920#10", Less);
    test(
        0,
        20,
        true,
        64,
        Nearest,
        "1.49802811317069208249e-6",
        "0x0.00001921fb544441c4038#64",
        Greater,
    );
    test(
        0,
        20,
        true,
        64,
        Floor,
        "1.49802811317069208238e-6",
        "0x0.00001921fb544441c4036#64",
        Less,
    );
    test(0, 20, false, 1, Nearest, "-1.9e-6", "-0x0.00002#1", Less);
    test(0, 20, false, 1, Floor, "-1.9e-6", "-0x0.00002#1", Less);
    test(
        0,
        20,
        false,
        10,
        Nearest,
        "-1.4976e-6",
        "-0x0.00001920#10",
        Greater,
    );
    test(
        0,
        20,
        false,
        10,
        Floor,
        "-1.4994e-6",
        "-0x0.00001928#10",
        Less,
    );
    test(
        0,
        20,
        false,
        64,
        Nearest,
        "-1.49802811317069208249e-6",
        "-0x0.00001921fb544441c4038#64",
        Less,
    );
    test(
        0,
        20,
        false,
        64,
        Floor,
        "-1.49802811317069208249e-6",
        "-0x0.00001921fb544441c4038#64",
        Less,
    );
    test(0, 70, true, 1, Nearest, "1.7e-21", "0x8.0E-18#1", Greater);
    test(0, 70, true, 1, Floor, "8.5e-22", "0x4.0E-18#1", Less);
    test(
        0,
        70,
        true,
        10,
        Nearest,
        "1.3301e-21",
        "0x6.48E-18#10",
        Less,
    );
    test(0, 70, true, 10, Floor, "1.3301e-21", "0x6.48E-18#10", Less);
    test(
        0,
        70,
        true,
        64,
        Nearest,
        "1.33051624222131038648e-21",
        "0x6.487ed5110b4611a8E-18#64",
        Greater,
    );
    test(
        0,
        70,
        true,
        64,
        Floor,
        "1.33051624222131038639e-21",
        "0x6.487ed5110b4611a0E-18#64",
        Less,
    );
    test(0, 70, false, 1, Nearest, "-1.7e-21", "-0x8.0E-18#1", Less);
    test(0, 70, false, 1, Floor, "-1.7e-21", "-0x8.0E-18#1", Less);
    test(
        0,
        70,
        false,
        10,
        Nearest,
        "-1.3301e-21",
        "-0x6.48E-18#10",
        Greater,
    );
    test(
        0,
        70,
        false,
        10,
        Floor,
        "-1.3318e-21",
        "-0x6.4aE-18#10",
        Less,
    );
    test(
        0,
        70,
        false,
        64,
        Nearest,
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        Less,
    );
    test(
        0,
        70,
        false,
        64,
        Floor,
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        Less,
    );
    test(
        0,
        1000,
        true,
        1,
        Nearest,
        "1.9e-301",
        "0x2.0E-250#1",
        Greater,
    );
    test(0, 1000, true, 1, Floor, "9.3e-302", "0x1.0E-250#1", Less);
    test(
        0,
        1000,
        true,
        10,
        Nearest,
        "1.4655e-301",
        "0x1.920E-250#10",
        Less,
    );
    test(
        0,
        1000,
        true,
        10,
        Floor,
        "1.4655e-301",
        "0x1.920E-250#10",
        Less,
    );
    test(
        0,
        1000,
        true,
        64,
        Nearest,
        "1.46596706387616992951e-301",
        "0x1.921fb54442d1846aE-250#64",
        Greater,
    );
    test(
        0,
        1000,
        true,
        64,
        Floor,
        "1.46596706387616992941e-301",
        "0x1.921fb54442d18468E-250#64",
        Less,
    );
    test(
        0,
        1000,
        false,
        1,
        Nearest,
        "-1.9e-301",
        "-0x2.0E-250#1",
        Less,
    );
    test(0, 1000, false, 1, Floor, "-1.9e-301", "-0x2.0E-250#1", Less);
    test(
        0,
        1000,
        false,
        10,
        Nearest,
        "-1.4655e-301",
        "-0x1.920E-250#10",
        Greater,
    );
    test(
        0,
        1000,
        false,
        10,
        Floor,
        "-1.4673e-301",
        "-0x1.928E-250#10",
        Less,
    );
    test(
        0,
        1000,
        false,
        64,
        Nearest,
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        Less,
    );
    test(
        0,
        1000,
        false,
        64,
        Floor,
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        Less,
    );
    test(1, 20, true, 1, Nearest, "-5.2e5", "-0x8.0E+4#1", Greater);
    test(1, 20, true, 1, Floor, "-1.0e6", "-0x1.0E+5#1", Less);
    test(1, 20, true, 10, Nearest, "-6.6765e5", "-0xa.30E+4#10", Less);
    test(1, 20, true, 10, Floor, "-6.6765e5", "-0xa.30E+4#10", Less);
    test(
        1,
        20,
        true,
        64,
        Nearest,
        "-667544.214429609631679",
        "-0xa2f98.36e4dbe0a98#64",
        Greater,
    );
    test(
        1,
        20,
        true,
        64,
        Floor,
        "-667544.214429609631736",
        "-0xa2f98.36e4dbe0a99#64",
        Less,
    );
    test(1, 20, false, 1, Nearest, "5.2e5", "0x8.0E+4#1", Less);
    test(1, 20, false, 1, Floor, "5.2e5", "0x8.0E+4#1", Less);
    test(
        1,
        20,
        false,
        10,
        Nearest,
        "6.6765e5",
        "0xa.30E+4#10",
        Greater,
    );
    test(1, 20, false, 10, Floor, "6.6662e5", "0xa.2cE+4#10", Less);
    test(
        1,
        20,
        false,
        64,
        Nearest,
        "667544.214429609631679",
        "0xa2f98.36e4dbe0a98#64",
        Less,
    );
    test(
        1,
        20,
        false,
        64,
        Floor,
        "667544.214429609631679",
        "0xa2f98.36e4dbe0a98#64",
        Less,
    );
    test(1, 70, true, 1, Nearest, "-5.9e20", "-0x2.0E+17#1", Greater);
    test(1, 70, true, 1, Floor, "-1.2e21", "-0x4.0E+17#1", Less);
    test(
        1,
        70,
        true,
        10,
        Nearest,
        "-7.5170e20",
        "-0x2.8cE+17#10",
        Less,
    );
    test(1, 70, true, 10, Floor, "-7.5170e20", "-0x2.8cE+17#10", Less);
    test(
        1,
        70,
        true,
        64,
        Nearest,
        "-751587968840192313984.0",
        "-0x2.8be60db9391054a8E+17#64",
        Less,
    );
    test(
        1,
        70,
        true,
        64,
        Floor,
        "-751587968840192313984.0",
        "-0x2.8be60db9391054a8E+17#64",
        Less,
    );
    test(1, 70, false, 1, Nearest, "5.9e20", "0x2.0E+17#1", Less);
    test(1, 70, false, 1, Floor, "5.9e20", "0x2.0E+17#1", Less);
    test(
        1,
        70,
        false,
        10,
        Nearest,
        "7.5170e20",
        "0x2.8cE+17#10",
        Greater,
    );
    test(1, 70, false, 10, Floor, "7.5055e20", "0x2.8bE+17#10", Less);
    test(
        1,
        70,
        false,
        64,
        Nearest,
        "751587968840192313984.0",
        "0x2.8be60db9391054a8E+17#64",
        Greater,
    );
    test(
        1,
        70,
        false,
        64,
        Floor,
        "751587968840192313920.0",
        "0x2.8be60db9391054a4E+17#64",
        Less,
    );
    test(
        1,
        1000,
        true,
        1,
        Nearest,
        "-5.4e300",
        "-0x8.0E+249#1",
        Greater,
    );
    test(1, 1000, true, 1, Floor, "-1.1e301", "-0x1.0E+250#1", Less);
    test(
        1,
        1000,
        true,
        10,
        Nearest,
        "-6.8225e300",
        "-0xa.30E+249#10",
        Less,
    );
    test(
        1,
        1000,
        true,
        10,
        Floor,
        "-6.8225e300",
        "-0xa.30E+249#10",
        Less,
    );
    test(
        1,
        1000,
        true,
        64,
        Nearest,
        "-6.82143565596825636341e300",
        "-0xa.2f9836e4e44152aE+249#64",
        Less,
    );
    test(
        1,
        1000,
        true,
        64,
        Floor,
        "-6.82143565596825636341e300",
        "-0xa.2f9836e4e44152aE+249#64",
        Less,
    );
    test(1, 1000, false, 1, Nearest, "5.4e300", "0x8.0E+249#1", Less);
    test(1, 1000, false, 1, Floor, "5.4e300", "0x8.0E+249#1", Less);
    test(
        1,
        1000,
        false,
        10,
        Nearest,
        "6.8225e300",
        "0xa.30E+249#10",
        Greater,
    );
    test(
        1,
        1000,
        false,
        10,
        Floor,
        "6.8120e300",
        "0xa.2cE+249#10",
        Less,
    );
    test(
        1,
        1000,
        false,
        64,
        Nearest,
        "6.82143565596825636341e300",
        "0xa.2f9836e4e44152aE+249#64",
        Greater,
    );
    test(
        1,
        1000,
        false,
        64,
        Floor,
        "6.82143565596825636283e300",
        "0xa.2f9836e4e441529E+249#64",
        Less,
    );
    test(2, 20, true, 1, Nearest, "1.9e-6", "0x0.00002#1", Greater);
    test(2, 20, true, 1, Floor, "9.5e-7", "0x0.00001#1", Less);
    test(
        2,
        20,
        true,
        10,
        Nearest,
        "1.4976e-6",
        "0x0.00001920#10",
        Less,
    );
    test(2, 20, true, 10, Floor, "1.4976e-6", "0x0.00001920#10", Less);
    test(
        2,
        20,
        true,
        64,
        Nearest,
        "1.49802811317069208249e-6",
        "0x0.00001921fb544441c4038#64",
        Greater,
    );
    test(
        2,
        20,
        true,
        64,
        Floor,
        "1.49802811317069208238e-6",
        "0x0.00001921fb544441c4036#64",
        Less,
    );
    test(2, 20, false, 1, Nearest, "-1.9e-6", "-0x0.00002#1", Less);
    test(2, 20, false, 1, Floor, "-1.9e-6", "-0x0.00002#1", Less);
    test(
        2,
        20,
        false,
        10,
        Nearest,
        "-1.4976e-6",
        "-0x0.00001920#10",
        Greater,
    );
    test(
        2,
        20,
        false,
        10,
        Floor,
        "-1.4994e-6",
        "-0x0.00001928#10",
        Less,
    );
    test(
        2,
        20,
        false,
        64,
        Nearest,
        "-1.49802811317069208249e-6",
        "-0x0.00001921fb544441c4038#64",
        Less,
    );
    test(
        2,
        20,
        false,
        64,
        Floor,
        "-1.49802811317069208249e-6",
        "-0x0.00001921fb544441c4038#64",
        Less,
    );
    test(2, 70, true, 1, Nearest, "1.7e-21", "0x8.0E-18#1", Greater);
    test(2, 70, true, 1, Floor, "8.5e-22", "0x4.0E-18#1", Less);
    test(
        2,
        70,
        true,
        10,
        Nearest,
        "1.3301e-21",
        "0x6.48E-18#10",
        Less,
    );
    test(2, 70, true, 10, Floor, "1.3301e-21", "0x6.48E-18#10", Less);
    test(
        2,
        70,
        true,
        64,
        Nearest,
        "1.33051624222131038648e-21",
        "0x6.487ed5110b4611a8E-18#64",
        Greater,
    );
    test(
        2,
        70,
        true,
        64,
        Floor,
        "1.33051624222131038639e-21",
        "0x6.487ed5110b4611a0E-18#64",
        Less,
    );
    test(2, 70, false, 1, Nearest, "-1.7e-21", "-0x8.0E-18#1", Less);
    test(2, 70, false, 1, Floor, "-1.7e-21", "-0x8.0E-18#1", Less);
    test(
        2,
        70,
        false,
        10,
        Nearest,
        "-1.3301e-21",
        "-0x6.48E-18#10",
        Greater,
    );
    test(
        2,
        70,
        false,
        10,
        Floor,
        "-1.3318e-21",
        "-0x6.4aE-18#10",
        Less,
    );
    test(
        2,
        70,
        false,
        64,
        Nearest,
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        Less,
    );
    test(
        2,
        70,
        false,
        64,
        Floor,
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        Less,
    );
    test(
        2,
        1000,
        true,
        1,
        Nearest,
        "1.9e-301",
        "0x2.0E-250#1",
        Greater,
    );
    test(2, 1000, true, 1, Floor, "9.3e-302", "0x1.0E-250#1", Less);
    test(
        2,
        1000,
        true,
        10,
        Nearest,
        "1.4655e-301",
        "0x1.920E-250#10",
        Less,
    );
    test(
        2,
        1000,
        true,
        10,
        Floor,
        "1.4655e-301",
        "0x1.920E-250#10",
        Less,
    );
    test(
        2,
        1000,
        true,
        64,
        Nearest,
        "1.46596706387616992951e-301",
        "0x1.921fb54442d1846aE-250#64",
        Greater,
    );
    test(
        2,
        1000,
        true,
        64,
        Floor,
        "1.46596706387616992941e-301",
        "0x1.921fb54442d18468E-250#64",
        Less,
    );
    test(
        2,
        1000,
        false,
        1,
        Nearest,
        "-1.9e-301",
        "-0x2.0E-250#1",
        Less,
    );
    test(2, 1000, false, 1, Floor, "-1.9e-301", "-0x2.0E-250#1", Less);
    test(
        2,
        1000,
        false,
        10,
        Nearest,
        "-1.4655e-301",
        "-0x1.920E-250#10",
        Greater,
    );
    test(
        2,
        1000,
        false,
        10,
        Floor,
        "-1.4673e-301",
        "-0x1.928E-250#10",
        Less,
    );
    test(
        2,
        1000,
        false,
        64,
        Nearest,
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        Less,
    );
    test(
        2,
        1000,
        false,
        64,
        Floor,
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        Less,
    );
    test(3, 20, true, 1, Nearest, "-5.2e5", "-0x8.0E+4#1", Greater);
    test(3, 20, true, 1, Floor, "-1.0e6", "-0x1.0E+5#1", Less);
    test(3, 20, true, 10, Nearest, "-6.6765e5", "-0xa.30E+4#10", Less);
    test(3, 20, true, 10, Floor, "-6.6765e5", "-0xa.30E+4#10", Less);
    test(
        3,
        20,
        true,
        64,
        Nearest,
        "-667544.214429609631679",
        "-0xa2f98.36e4dbe0a98#64",
        Greater,
    );
    test(
        3,
        20,
        true,
        64,
        Floor,
        "-667544.214429609631736",
        "-0xa2f98.36e4dbe0a99#64",
        Less,
    );
    test(3, 20, false, 1, Nearest, "5.2e5", "0x8.0E+4#1", Less);
    test(3, 20, false, 1, Floor, "5.2e5", "0x8.0E+4#1", Less);
    test(
        3,
        20,
        false,
        10,
        Nearest,
        "6.6765e5",
        "0xa.30E+4#10",
        Greater,
    );
    test(3, 20, false, 10, Floor, "6.6662e5", "0xa.2cE+4#10", Less);
    test(
        3,
        20,
        false,
        64,
        Nearest,
        "667544.214429609631679",
        "0xa2f98.36e4dbe0a98#64",
        Less,
    );
    test(
        3,
        20,
        false,
        64,
        Floor,
        "667544.214429609631679",
        "0xa2f98.36e4dbe0a98#64",
        Less,
    );
    test(3, 70, true, 1, Nearest, "-5.9e20", "-0x2.0E+17#1", Greater);
    test(3, 70, true, 1, Floor, "-1.2e21", "-0x4.0E+17#1", Less);
    test(
        3,
        70,
        true,
        10,
        Nearest,
        "-7.5170e20",
        "-0x2.8cE+17#10",
        Less,
    );
    test(3, 70, true, 10, Floor, "-7.5170e20", "-0x2.8cE+17#10", Less);
    test(
        3,
        70,
        true,
        64,
        Nearest,
        "-751587968840192313984.0",
        "-0x2.8be60db9391054a8E+17#64",
        Less,
    );
    test(
        3,
        70,
        true,
        64,
        Floor,
        "-751587968840192313984.0",
        "-0x2.8be60db9391054a8E+17#64",
        Less,
    );
    test(3, 70, false, 1, Nearest, "5.9e20", "0x2.0E+17#1", Less);
    test(3, 70, false, 1, Floor, "5.9e20", "0x2.0E+17#1", Less);
    test(
        3,
        70,
        false,
        10,
        Nearest,
        "7.5170e20",
        "0x2.8cE+17#10",
        Greater,
    );
    test(3, 70, false, 10, Floor, "7.5055e20", "0x2.8bE+17#10", Less);
    test(
        3,
        70,
        false,
        64,
        Nearest,
        "751587968840192313984.0",
        "0x2.8be60db9391054a8E+17#64",
        Greater,
    );
    test(
        3,
        70,
        false,
        64,
        Floor,
        "751587968840192313920.0",
        "0x2.8be60db9391054a4E+17#64",
        Less,
    );
    test(
        3,
        1000,
        true,
        1,
        Nearest,
        "-5.4e300",
        "-0x8.0E+249#1",
        Greater,
    );
    test(3, 1000, true, 1, Floor, "-1.1e301", "-0x1.0E+250#1", Less);
    test(
        3,
        1000,
        true,
        10,
        Nearest,
        "-6.8225e300",
        "-0xa.30E+249#10",
        Less,
    );
    test(
        3,
        1000,
        true,
        10,
        Floor,
        "-6.8225e300",
        "-0xa.30E+249#10",
        Less,
    );
    test(
        3,
        1000,
        true,
        64,
        Nearest,
        "-6.82143565596825636341e300",
        "-0xa.2f9836e4e44152aE+249#64",
        Less,
    );
    test(
        3,
        1000,
        true,
        64,
        Floor,
        "-6.82143565596825636341e300",
        "-0xa.2f9836e4e44152aE+249#64",
        Less,
    );
    test(3, 1000, false, 1, Nearest, "5.4e300", "0x8.0E+249#1", Less);
    test(3, 1000, false, 1, Floor, "5.4e300", "0x8.0E+249#1", Less);
    test(
        3,
        1000,
        false,
        10,
        Nearest,
        "6.8225e300",
        "0xa.30E+249#10",
        Greater,
    );
    test(
        3,
        1000,
        false,
        10,
        Floor,
        "6.8120e300",
        "0xa.2cE+249#10",
        Less,
    );
    test(
        3,
        1000,
        false,
        64,
        Nearest,
        "6.82143565596825636341e300",
        "0xa.2f9836e4e44152aE+249#64",
        Greater,
    );
    test(
        3,
        1000,
        false,
        64,
        Floor,
        "6.82143565596825636283e300",
        "0xa.2f9836e4e441529E+249#64",
        Less,
    );
    test(-1, 20, true, 1, Nearest, "-5.2e5", "-0x8.0E+4#1", Greater);
    test(-1, 20, true, 1, Floor, "-1.0e6", "-0x1.0E+5#1", Less);
    test(
        -1,
        20,
        true,
        10,
        Nearest,
        "-6.6765e5",
        "-0xa.30E+4#10",
        Less,
    );
    test(-1, 20, true, 10, Floor, "-6.6765e5", "-0xa.30E+4#10", Less);
    test(
        -1,
        20,
        true,
        64,
        Nearest,
        "-667544.214429609631679",
        "-0xa2f98.36e4dbe0a98#64",
        Greater,
    );
    test(
        -1,
        20,
        true,
        64,
        Floor,
        "-667544.214429609631736",
        "-0xa2f98.36e4dbe0a99#64",
        Less,
    );
    test(-1, 20, false, 1, Nearest, "5.2e5", "0x8.0E+4#1", Less);
    test(-1, 20, false, 1, Floor, "5.2e5", "0x8.0E+4#1", Less);
    test(
        -1,
        20,
        false,
        10,
        Nearest,
        "6.6765e5",
        "0xa.30E+4#10",
        Greater,
    );
    test(-1, 20, false, 10, Floor, "6.6662e5", "0xa.2cE+4#10", Less);
    test(
        -1,
        20,
        false,
        64,
        Nearest,
        "667544.214429609631679",
        "0xa2f98.36e4dbe0a98#64",
        Less,
    );
    test(
        -1,
        20,
        false,
        64,
        Floor,
        "667544.214429609631679",
        "0xa2f98.36e4dbe0a98#64",
        Less,
    );
    test(-1, 70, true, 1, Nearest, "-5.9e20", "-0x2.0E+17#1", Greater);
    test(-1, 70, true, 1, Floor, "-1.2e21", "-0x4.0E+17#1", Less);
    test(
        -1,
        70,
        true,
        10,
        Nearest,
        "-7.5170e20",
        "-0x2.8cE+17#10",
        Less,
    );
    test(
        -1,
        70,
        true,
        10,
        Floor,
        "-7.5170e20",
        "-0x2.8cE+17#10",
        Less,
    );
    test(
        -1,
        70,
        true,
        64,
        Nearest,
        "-751587968840192313984.0",
        "-0x2.8be60db9391054a8E+17#64",
        Less,
    );
    test(
        -1,
        70,
        true,
        64,
        Floor,
        "-751587968840192313984.0",
        "-0x2.8be60db9391054a8E+17#64",
        Less,
    );
    test(-1, 70, false, 1, Nearest, "5.9e20", "0x2.0E+17#1", Less);
    test(-1, 70, false, 1, Floor, "5.9e20", "0x2.0E+17#1", Less);
    test(
        -1,
        70,
        false,
        10,
        Nearest,
        "7.5170e20",
        "0x2.8cE+17#10",
        Greater,
    );
    test(-1, 70, false, 10, Floor, "7.5055e20", "0x2.8bE+17#10", Less);
    test(
        -1,
        70,
        false,
        64,
        Nearest,
        "751587968840192313984.0",
        "0x2.8be60db9391054a8E+17#64",
        Greater,
    );
    test(
        -1,
        70,
        false,
        64,
        Floor,
        "751587968840192313920.0",
        "0x2.8be60db9391054a4E+17#64",
        Less,
    );
    test(
        -1,
        1000,
        true,
        1,
        Nearest,
        "-5.4e300",
        "-0x8.0E+249#1",
        Greater,
    );
    test(-1, 1000, true, 1, Floor, "-1.1e301", "-0x1.0E+250#1", Less);
    test(
        -1,
        1000,
        true,
        10,
        Nearest,
        "-6.8225e300",
        "-0xa.30E+249#10",
        Less,
    );
    test(
        -1,
        1000,
        true,
        10,
        Floor,
        "-6.8225e300",
        "-0xa.30E+249#10",
        Less,
    );
    test(
        -1,
        1000,
        true,
        64,
        Nearest,
        "-6.82143565596825636341e300",
        "-0xa.2f9836e4e44152aE+249#64",
        Less,
    );
    test(
        -1,
        1000,
        true,
        64,
        Floor,
        "-6.82143565596825636341e300",
        "-0xa.2f9836e4e44152aE+249#64",
        Less,
    );
    test(-1, 1000, false, 1, Nearest, "5.4e300", "0x8.0E+249#1", Less);
    test(-1, 1000, false, 1, Floor, "5.4e300", "0x8.0E+249#1", Less);
    test(
        -1,
        1000,
        false,
        10,
        Nearest,
        "6.8225e300",
        "0xa.30E+249#10",
        Greater,
    );
    test(
        -1,
        1000,
        false,
        10,
        Floor,
        "6.8120e300",
        "0xa.2cE+249#10",
        Less,
    );
    test(
        -1,
        1000,
        false,
        64,
        Nearest,
        "6.82143565596825636341e300",
        "0xa.2f9836e4e44152aE+249#64",
        Greater,
    );
    test(
        -1,
        1000,
        false,
        64,
        Floor,
        "6.82143565596825636283e300",
        "0xa.2f9836e4e441529E+249#64",
        Less,
    );
    test(-2, 20, true, 1, Nearest, "1.9e-6", "0x0.00002#1", Greater);
    test(-2, 20, true, 1, Floor, "9.5e-7", "0x0.00001#1", Less);
    test(
        -2,
        20,
        true,
        10,
        Nearest,
        "1.4976e-6",
        "0x0.00001920#10",
        Less,
    );
    test(
        -2,
        20,
        true,
        10,
        Floor,
        "1.4976e-6",
        "0x0.00001920#10",
        Less,
    );
    test(
        -2,
        20,
        true,
        64,
        Nearest,
        "1.49802811317069208249e-6",
        "0x0.00001921fb544441c4038#64",
        Greater,
    );
    test(
        -2,
        20,
        true,
        64,
        Floor,
        "1.49802811317069208238e-6",
        "0x0.00001921fb544441c4036#64",
        Less,
    );
    test(-2, 20, false, 1, Nearest, "-1.9e-6", "-0x0.00002#1", Less);
    test(-2, 20, false, 1, Floor, "-1.9e-6", "-0x0.00002#1", Less);
    test(
        -2,
        20,
        false,
        10,
        Nearest,
        "-1.4976e-6",
        "-0x0.00001920#10",
        Greater,
    );
    test(
        -2,
        20,
        false,
        10,
        Floor,
        "-1.4994e-6",
        "-0x0.00001928#10",
        Less,
    );
    test(
        -2,
        20,
        false,
        64,
        Nearest,
        "-1.49802811317069208249e-6",
        "-0x0.00001921fb544441c4038#64",
        Less,
    );
    test(
        -2,
        20,
        false,
        64,
        Floor,
        "-1.49802811317069208249e-6",
        "-0x0.00001921fb544441c4038#64",
        Less,
    );
    test(-2, 70, true, 1, Nearest, "1.7e-21", "0x8.0E-18#1", Greater);
    test(-2, 70, true, 1, Floor, "8.5e-22", "0x4.0E-18#1", Less);
    test(
        -2,
        70,
        true,
        10,
        Nearest,
        "1.3301e-21",
        "0x6.48E-18#10",
        Less,
    );
    test(-2, 70, true, 10, Floor, "1.3301e-21", "0x6.48E-18#10", Less);
    test(
        -2,
        70,
        true,
        64,
        Nearest,
        "1.33051624222131038648e-21",
        "0x6.487ed5110b4611a8E-18#64",
        Greater,
    );
    test(
        -2,
        70,
        true,
        64,
        Floor,
        "1.33051624222131038639e-21",
        "0x6.487ed5110b4611a0E-18#64",
        Less,
    );
    test(-2, 70, false, 1, Nearest, "-1.7e-21", "-0x8.0E-18#1", Less);
    test(-2, 70, false, 1, Floor, "-1.7e-21", "-0x8.0E-18#1", Less);
    test(
        -2,
        70,
        false,
        10,
        Nearest,
        "-1.3301e-21",
        "-0x6.48E-18#10",
        Greater,
    );
    test(
        -2,
        70,
        false,
        10,
        Floor,
        "-1.3318e-21",
        "-0x6.4aE-18#10",
        Less,
    );
    test(
        -2,
        70,
        false,
        64,
        Nearest,
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        Less,
    );
    test(
        -2,
        70,
        false,
        64,
        Floor,
        "-1.33051624222131038648e-21",
        "-0x6.487ed5110b4611a8E-18#64",
        Less,
    );
    test(
        -2,
        1000,
        true,
        1,
        Nearest,
        "1.9e-301",
        "0x2.0E-250#1",
        Greater,
    );
    test(-2, 1000, true, 1, Floor, "9.3e-302", "0x1.0E-250#1", Less);
    test(
        -2,
        1000,
        true,
        10,
        Nearest,
        "1.4655e-301",
        "0x1.920E-250#10",
        Less,
    );
    test(
        -2,
        1000,
        true,
        10,
        Floor,
        "1.4655e-301",
        "0x1.920E-250#10",
        Less,
    );
    test(
        -2,
        1000,
        true,
        64,
        Nearest,
        "1.46596706387616992951e-301",
        "0x1.921fb54442d1846aE-250#64",
        Greater,
    );
    test(
        -2,
        1000,
        true,
        64,
        Floor,
        "1.46596706387616992941e-301",
        "0x1.921fb54442d18468E-250#64",
        Less,
    );
    test(
        -2,
        1000,
        false,
        1,
        Nearest,
        "-1.9e-301",
        "-0x2.0E-250#1",
        Less,
    );
    test(
        -2,
        1000,
        false,
        1,
        Floor,
        "-1.9e-301",
        "-0x2.0E-250#1",
        Less,
    );
    test(
        -2,
        1000,
        false,
        10,
        Nearest,
        "-1.4655e-301",
        "-0x1.920E-250#10",
        Greater,
    );
    test(
        -2,
        1000,
        false,
        10,
        Floor,
        "-1.4673e-301",
        "-0x1.928E-250#10",
        Less,
    );
    test(
        -2,
        1000,
        false,
        64,
        Nearest,
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        Less,
    );
    test(
        -2,
        1000,
        false,
        64,
        Floor,
        "-1.46596706387616992951e-301",
        "-0x1.921fb54442d1846aE-250#64",
        Less,
    );
}

#[test]
#[should_panic]
fn tan_with_period_prec_round_fail_1() {
    Float::ONE.tan_with_period_prec_round(7, 0, Nearest);
}

#[test]
#[should_panic]
fn tan_with_period_prec_round_fail_2() {
    Float::ONE.tan_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn tan_with_period_prec_round_fail_3() {
    // a twelfth of a turn is sqrt(3)/3, which is not exact
    Float::ONE.tan_with_period_prec_round(12, 10, Exact);
}

#[test]
#[should_panic]
fn tan_with_period_prec_round_ref_fail() {
    Float::ONE.tan_with_period_prec_round_ref(7, 10, Exact);
}

#[test]
#[should_panic]
fn tan_with_period_prec_fail() {
    Float::ONE.tan_with_period_prec(7, 0);
}

#[test]
#[should_panic]
fn tan_with_period_round_fail() {
    Float::ONE.tan_with_period_round(7, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn tan_with_period_prec_round_properties_helper(x: Float, u: u64, prec: u64, rm: RoundingMode) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (t, o) = x.tan_with_period_prec_round_ref(u, prec, Nearest);
        if o == Equal {
            let (te, oe) = x.tan_with_period_prec_round_ref(u, prec, Exact);
            assert_eq!(ComparableFloatRef(&te), ComparableFloatRef(&t));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(x.tan_with_period_prec_round_ref(u, prec, Exact));
        }
        return;
    }
    let (t, o) = x.clone().tan_with_period_prec_round(u, prec, rm);
    assert!(t.is_valid());
    assert_rounding_ordering_consistent(&t, rm, o);

    let (t_alt, o_alt) = x.tan_with_period_prec_round_ref(u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    let mut t_alt = x.clone();
    let o_alt = t_alt.tan_with_period_prec_round_assign(u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm)
        && u32::try_from(u).is_ok()
    {
        let (rug_t, rug_o) =
            rug_tan_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_t)),
            ComparableFloatRef(&t),
            "RUGPROBE x = {x:#x} u = {u} prec = {prec} rm = {rm:?}"
        );
        assert_eq!(rug_o, o);
    }

    // tan_with_period is NaN exactly for u = 0 and non-finite x
    assert_eq!(t.is_nan(), u == 0 || !x.is_finite());
    if !t.is_nan() {
        // an infinity is a pole, and exact
        if t.is_infinite() {
            assert_eq!(o, Equal);
        } else if t.is_normal() {
            assert_eq!(t.get_prec(), Some(prec));
        }
        // tan_with_period is odd
        let (t_neg, o_neg) = (-&x).tan_with_period_prec_round(u, prec, -rm);
        assert_eq!(ComparableFloat(t_neg), ComparableFloat(-&t));
        assert_eq!(o_neg, o.reverse());
        // tan_with_period has period u (indeed u/2), up to the sign of a zero, which follows the
        // sign of x and so flips when the shift crosses zero
        if x.is_finite() && u != 0 {
            let (shifted, os) =
                x.add_prec_round_ref_val(Float::from(u), x.significant_bits() + 64, Nearest);
            if os == Equal {
                let (t_shifted, o_shifted) = shifted.tan_with_period_prec_round(u, prec, rm);
                assert_eq!(
                    ComparableFloat(t_shifted.abs_negative_zero()),
                    ComparableFloat(t.abs_negative_zero_ref())
                );
                assert_eq!(o_shifted, o);
            }
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (t2, oo) = x.tan_with_period_prec_round_ref(u, prec, rm);
            assert_eq!(ComparableFloatRef(&t2), ComparableFloatRef(&t));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.tan_with_period_prec_round_ref(u, prec, Exact));
    }
}

#[test]
fn tan_with_period_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, u, prec, rm)| {
            tan_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_18().test_properties(
        |(x, u, prec, rm)| {
            tan_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // tan_with_period(±0) = ±0 and tan_with_period(x, 0) = NaN, exactly
        let (t, o) = Float::ZERO.tan_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (t, o) = Float::NEGATIVE_ZERO.tan_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
        let (t, o) = Float::ONE.tan_with_period_prec_round(0, prec, rm);
        assert!(t.is_nan());
        assert_eq!(o, Equal);
        // exact cases: quarter turns are the tangent's zeros and poles, with the zeros reached from
        // below
        for (k, expected) in [
            (0u32, Float::ZERO),
            (1, Float::INFINITY),
            (2, Float::NEGATIVE_ZERO),
            (3, Float::NEGATIVE_INFINITY),
            (4, Float::ZERO),
        ] {
            let (t, o) = Float::from(k).tan_with_period_prec_round(4, prec, rm);
            assert_eq!(ComparableFloat(t), ComparableFloat(expected));
            assert_eq!(o, Equal);
        }
        for (k, expected) in [
            (1i32, Float::NEGATIVE_INFINITY),
            (2, Float::ZERO),
            (3, Float::INFINITY),
            (4, Float::NEGATIVE_ZERO),
        ] {
            let (t, o) = Float::from(-k).tan_with_period_prec_round(4, prec, rm);
            assert_eq!(ComparableFloat(t), ComparableFloat(expected));
            assert_eq!(o, Equal);
        }
        // exact cases: eighths of a turn
        let one = Float::one_prec(prec);
        for (k, expected) in [(1u32, one.clone()), (3, -&one), (5, one.clone()), (7, -&one)] {
            let (t, o) = Float::from(k).tan_with_period_prec_round(8, prec, rm);
            assert_eq!(ComparableFloat(t), ComparableFloat(expected));
            assert_eq!(o, Equal);
        }
        if rm != Exact {
            // closed forms: twelfths of a turn are ±sqrt(3)/3 and ±sqrt(3)
            for (k, constant) in [
                (1u32, Float::sqrt_3_over_3_prec_round(prec, rm)),
                (2, Float::sqrt_3_prec_round(prec, rm)),
                (7, Float::sqrt_3_over_3_prec_round(prec, rm)),
                (8, Float::sqrt_3_prec_round(prec, rm)),
            ] {
                let (t, o) = Float::from(k).tan_with_period_prec_round(12, prec, rm);
                assert_eq!(ComparableFloat(t), ComparableFloat(constant.0));
                assert_eq!(o, constant.1);
            }
        }
    });
}

#[test]
fn tan_with_period_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, u, prec)| {
        let (t, o) = x.clone().tan_with_period_prec(u, prec);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, Nearest, o);
        let (t_alt, o_alt) = x.tan_with_period_prec_ref(u, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.tan_with_period_prec_round_ref(u, prec, Nearest);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.tan_with_period_prec_assign(u, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        if u32::try_from(u).is_ok() {
            let (rug_t, rug_o) = rug_tan_with_period_prec(&rug::Float::exact_from(&x), u, prec);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn tan_with_period_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_39().test_properties(|(x, u, rm)| {
        if rm == Exact {
            // The generator admits Exact for inputs whose sine is exact, which the tangent need not
            // be (a twelfth of a turn).
            let (t, o) = x.tan_with_period_round_ref(u, Nearest);
            if o == Equal {
                let (te, oe) = x.tan_with_period_round_ref(u, Exact);
                assert_eq!(ComparableFloatRef(&te), ComparableFloatRef(&t));
                assert_eq!(oe, Equal);
            } else {
                assert_panic!(x.tan_with_period_round_ref(u, Exact));
            }
            return;
        }
        let (t, o) = x.clone().tan_with_period_round(u, rm);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = x.tan_with_period_round_ref(u, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.tan_with_period_prec_round_ref(u, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.tan_with_period_round_assign(u, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });
}

// Inputs within 2^(-2^30) of a quarter turn, whose tangents underflow (just past a half turn) and
// overflow (just past a quarter turn). The near-zero paths work with the exact distance to the
// multiple of 1/4, so no 2^30-bit pi is ever formed, but the inputs themselves have 2^30 bits.
#[test]
fn test_tan_with_period_underflow_and_overflow() {
    let min_positive = Float::one_prec(10) >> (1u64 << 30);
    let eps = Rational::power_of_2(-((1i64 << 30) + 70));
    let p = (1u64 << 30) + 74;
    // just past a half turn: x/u = 1/2 + 2^(-2^30 - 72), so the tangent is positive and tiny
    let above = Float::from_rational_prec_round(Rational::from(2u32) + &eps, p, Exact).0;
    let (t, o) = above.tan_with_period_prec_round_ref(4, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    let (t, o) = above.tan_with_period_prec_round_ref(4, 10, Up);
    assert_eq!(ComparableFloatRef(&t), ComparableFloatRef(&min_positive));
    assert_eq!(o, Greater);
    // just past a quarter turn: the tangent is negative and huge, beyond the largest finite Float
    let above = Float::from_rational_prec_round(Rational::ONE + eps, p, Exact).0;
    let (t, o) = above.tan_with_period_prec_round_ref(4, 10, Nearest);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    let (t, o) = above.tan_with_period_prec_round_ref(4, 10, Down);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(-Float::max_finite_value_with_prec(10))
    );
    assert_eq!(o, Greater);
}

// The `Rational` inputs include the exact and closed-form cases hit directly (a twelfth, an eighth,
// and a quarter of a turn, the last a pole), which the `Float` version can only reach through a
// divisor, and non-dyadic inputs whose tangents MPFR cannot compute exactly, since it must round
// the input first.
#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_tan_with_period() {
    fn test<T: PrimitiveFloat>(x: T, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_tan_with_period(x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 360, f32::NAN);
    test::<f32>(f32::INFINITY, 360, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, 360, f32::NAN);
    test::<f32>(1.0, 0, f32::NAN);
    test::<f32>(0.0, 360, 0.0);
    test::<f32>(-0.0, 360, -0.0);
    test::<f32>(90.0, 360, f32::INFINITY);
    test::<f32>(-90.0, 360, f32::NEGATIVE_INFINITY);
    test::<f32>(270.0, 360, f32::NEGATIVE_INFINITY);
    test::<f32>(180.0, 360, -0.0);
    test::<f32>(-180.0, 360, 0.0);
    test::<f32>(360.0, 360, 0.0);
    test::<f32>(-360.0, 360, -0.0);
    test::<f32>(45.0, 360, 1.0);
    test::<f32>(135.0, 360, -1.0);
    test::<f32>(225.0, 360, 1.0);
    test::<f32>(30.0, 360, 0.57735026);
    test::<f32>(60.0, 360, 1.7320508);
    test::<f32>(1.0, 7, 1.2539604);
    test::<f32>(-1.0, 7, -1.2539604);
    test::<f32>(2.0, 7, -4.381286);
    test::<f32>(1.0, 360, 0.017455066);
    test::<f32>(100.0, 360, -5.671282);
    test::<f32>(10000000000.0, 360, -5.671282);
    test::<f32>(1.0e30, 7, 1.2539604);
    test::<f32>(1.0e-30, 7, 8.975979e-31);
    test::<f32>(3.4028235e38, 360, 0.0);
    test::<f32>(0.5, 1, -0.0);
    test::<f32>(0.25, 1, f32::INFINITY);
    test::<f32>(0.1, 1, 0.72654253);
    test::<f32>(1.0e-45, 1, 8.0e-45);
    test::<f32>(1.0e-45, 360, 0.0);
    test::<f32>(-1.0e-45, 1, -8.0e-45);
    test::<f64>(f64::NAN, 360, f64::NAN);
    test::<f64>(f64::INFINITY, 360, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, 360, f64::NAN);
    test::<f64>(1.0, 0, f64::NAN);
    test::<f64>(0.0, 360, 0.0);
    test::<f64>(-0.0, 360, -0.0);
    test::<f64>(90.0, 360, f64::INFINITY);
    test::<f64>(-90.0, 360, f64::NEGATIVE_INFINITY);
    test::<f64>(270.0, 360, f64::NEGATIVE_INFINITY);
    test::<f64>(180.0, 360, -0.0);
    test::<f64>(-180.0, 360, 0.0);
    test::<f64>(360.0, 360, 0.0);
    test::<f64>(-360.0, 360, -0.0);
    test::<f64>(45.0, 360, 1.0);
    test::<f64>(135.0, 360, -1.0);
    test::<f64>(225.0, 360, 1.0);
    test::<f64>(30.0, 360, 0.5773502691896257);
    test::<f64>(60.0, 360, 1.7320508075688772);
    test::<f64>(1.0, 7, 1.2539603376627038);
    test::<f64>(-1.0, 7, -1.2539603376627038);
    test::<f64>(2.0, 7, -4.381286267534823);
    test::<f64>(1.0, 360, 0.017455064928217585);
    test::<f64>(100.0, 360, -5.671281819617709);
    test::<f64>(10000000000.0, 360, -5.671281819617709);
    test::<f64>(1.0e100, 7, -4.381286267534823);
    test::<f64>(1.0e-100, 7, 8.975979010256552e-101);
    test::<f64>(1.7976931348623157e308, 360, -1.2799416321930788);
    test::<f64>(0.5, 1, -0.0);
    test::<f64>(0.25, 1, f64::INFINITY);
    test::<f64>(0.1, 1, 0.7265425280053609);
    test::<f64>(5.0e-324, 1, 3.0e-323);
    test::<f64>(5.0e-324, 360, 0.0);
    test::<f64>(-5.0e-324, 1, -3.0e-323);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_tan_with_period_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        let t = primitive_float_tan_with_period(x, u);
        // NaN exactly for u = 0 (the inputs are finite)
        assert_eq!(t.is_nan(), u == 0);
        if u != 0 {
            // odd
            assert_eq!(
                NiceFloat(primitive_float_tan_with_period(-x, u)),
                NiceFloat(-t)
            );
            // the result is the correctly rounded tangent, as computed by MPFR with 64 bits to
            // spare, so that a subnormal result is rounded once by the conversion
            let rug_t = rug_tan_with_period_prec(
                &rug::Float::exact_from(&Float::from(x)),
                u,
                T::MANTISSA_WIDTH + 64,
            )
            .0;
            let rug_t: T = T::rounding_from(&<Float as From<&rug::Float>>::from(&rug_t), Nearest).0;
            assert_eq!(NiceFloat(rug_t), NiceFloat(t));
            // the result is infinite only at a pole, and never through overflow: no f32 or f64 is
            // merely close enough to an odd quarter turn for its tangent to leave the range
            let (t_wide, _) =
                Float::tan_with_period_prec(Float::from(x), u, T::MANTISSA_WIDTH + 64);
            assert_eq!(t.is_infinite(), t_wide.is_infinite());
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // NaN exactly for NaN and infinite inputs
        assert_eq!(
            primitive_float_tan_with_period(x, 7).is_nan(),
            !x.is_finite()
        );
    });
}

#[test]
fn primitive_float_tan_with_period_properties() {
    apply_fn_to_primitive_floats!(primitive_float_tan_with_period_properties_helper);
}

#[test]
fn test_tan_with_period_rational_prec_round() {
    let test = |s: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (t, o) = Float::tan_with_period_rational_prec_round(x.clone(), u, prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = Float::tan_with_period_rational_prec_round_ref(&x, u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = Float::tan_with_period_rational_prec(x.clone(), u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) = Float::tan_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        // MPFR rounds the input first, so it cannot see the exact cases of non-dyadic inputs
        if o != Equal
            && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u32::try_from(u).is_ok()
        {
            let (rug_t, rug_o) = rug_tan_with_period_rational_prec_round(&x, u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 4, 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", 4, 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 4, 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 4, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 4, 10, Exact, "0.0", "0x0.0", Equal);
    test("0", 4, 53, Nearest, "0.0", "0x0.0", Equal);
    test("0", 0, 1, Nearest, "NaN", "NaN", Equal);
    test("0", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("0", 0, 10, Floor, "NaN", "NaN", Equal);
    test("0", 0, 10, Ceiling, "NaN", "NaN", Equal);
    test("0", 0, 10, Exact, "NaN", "NaN", Equal);
    test("0", 0, 53, Nearest, "NaN", "NaN", Equal);
    test("1", 0, 1, Nearest, "NaN", "NaN", Equal);
    test("1", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("1", 0, 10, Floor, "NaN", "NaN", Equal);
    test("1", 0, 10, Ceiling, "NaN", "NaN", Equal);
    test("1", 0, 10, Exact, "NaN", "NaN", Equal);
    test("1", 0, 53, Nearest, "NaN", "NaN", Equal);
    test("90", 360, 1, Nearest, "Infinity", "Infinity", Equal);
    test("90", 360, 10, Nearest, "Infinity", "Infinity", Equal);
    test("90", 360, 10, Floor, "Infinity", "Infinity", Equal);
    test("90", 360, 10, Ceiling, "Infinity", "Infinity", Equal);
    test("90", 360, 10, Exact, "Infinity", "Infinity", Equal);
    test("90", 360, 53, Nearest, "Infinity", "Infinity", Equal);
    test("180", 360, 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("180", 360, 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("180", 360, 10, Floor, "-0.0", "-0x0.0", Equal);
    test("180", 360, 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test("180", 360, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("180", 360, 53, Nearest, "-0.0", "-0x0.0", Equal);
    test("270", 360, 1, Nearest, "-Infinity", "-Infinity", Equal);
    test("270", 360, 10, Nearest, "-Infinity", "-Infinity", Equal);
    test("270", 360, 10, Floor, "-Infinity", "-Infinity", Equal);
    test("270", 360, 10, Ceiling, "-Infinity", "-Infinity", Equal);
    test("270", 360, 10, Exact, "-Infinity", "-Infinity", Equal);
    test("270", 360, 53, Nearest, "-Infinity", "-Infinity", Equal);
    test("360", 360, 1, Nearest, "0.0", "0x0.0", Equal);
    test("360", 360, 10, Nearest, "0.0", "0x0.0", Equal);
    test("360", 360, 10, Floor, "0.0", "0x0.0", Equal);
    test("360", 360, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("360", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test("450", 360, 10, Ceiling, "Infinity", "Infinity", Equal);
    test("60", 360, 10, Floor, "1.7305", "0x1.bb0#10", Less);
    test("120", 360, 10, Floor, "-1.7324", "-0x1.bb8#10", Less);
    test("240", 360, 10, Floor, "1.7305", "0x1.bb0#10", Less);
    test("300", 360, 10, Floor, "-1.7324", "-0x1.bb8#10", Less);
    test("45", 360, 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("135", 360, 10, Floor, "-1.0000", "-0x1.000#10", Equal);
    test("30", 360, 10, Floor, "0.57715", "0x0.93c#10", Less);
    test("150", 360, 10, Floor, "-0.57812", "-0x0.940#10", Less);
    test("72", 360, 10, Floor, "3.0742", "0x3.13#10", Less);
    test("144", 360, 10, Floor, "-0.72656", "-0x0.ba0#10", Less);
    test("36", 360, 10, Floor, "0.72559", "0x0.b9c#10", Less);
    test("108", 360, 10, Floor, "-3.0781", "-0x3.14#10", Less);
    test("1/3", 1, 10, Floor, "-1.7324", "-0x1.bb8#10", Less);
    test("1/8", 1, 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("1/5", 1, 10, Floor, "3.0742", "0x3.13#10", Less);
    test("-1/10", 1, 10, Floor, "-0.72656", "-0x0.ba0#10", Less);
    test("1/7", 1, 10, Floor, "1.2539", "0x1.410#10", Less);
    test("2/7", 1, 10, Floor, "-4.3828", "-0x4.62#10", Less);
    test("-3/7", 1, 10, Floor, "0.48145", "0x0.7b4#10", Less);
    test("22/7", 1, 10, Floor, "1.2539", "0x1.410#10", Less);
    test("1/7", 3, 10, Floor, "0.30811", "0x0.4ee#10", Less);
    test("355/113", 360, 10, Floor, "0.054871", "0x0.0e0c#10", Less);
    test("1", 7, 10, Floor, "1.2539", "0x1.410#10", Less);
    test("1000000", 7, 10, Floor, "1.2539", "0x1.410#10", Less);
    test(
        "1/1000000",
        1,
        10,
        Floor,
        "6.2808e-6",
        "0x0.0000696#10",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        1,
        10,
        Floor,
        "6.2801e-30",
        "0x7.f6E-25#10",
        Less,
    );
    test(
        "100000000000000000000000000000000000000001",
        4,
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "111414603535684224740921180161/1237940039285380274899124224",
        360,
        53,
        Floor,
        "-7.0928739541311617e28",
        "-0xe.52ee0d31e0fc0E+23#53",
        Less,
    );
    test(
        "3/20",
        1,
        30,
        Nearest,
        "1.3763819207",
        "0x1.605a90c8#30",
        Greater,
    );
    test(
        "11/20",
        1,
        30,
        Floor,
        "0.32491969597",
        "0x0.532defec#30",
        Less,
    );
    test(
        "17/20",
        1,
        30,
        Nearest,
        "-1.3763819207",
        "-0x1.605a90c8#30",
        Less,
    );
    test(
        "-1/3",
        1,
        30,
        Nearest,
        "1.7320508081",
        "0x1.bb67ae88#30",
        Greater,
    );
    test(
        "3/8",
        1,
        30,
        Nearest,
        "-1.0000000000",
        "-0x1.00000000#30",
        Equal,
    );
    test("1/2", 1, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("-5", 1, 10, Exact, "-0.0", "-0x0.0", Equal);
    test(
        "-1/1000000000000000000000000000000",
        7,
        10,
        Floor,
        "-8.9825e-31",
        "-0x1.238E-25#10",
        Less,
    );
}

// Fractions of a turn so small that 2 pi x/u is at or below the bottom of the exponent range, where
// the tangent equals 2 pi x/u to far more bits than any precision needs, and underflows.
#[test]
fn test_tan_with_period_rational_tiny() {
    let test = |x: &Rational, u: u64, prec: u64, rm: RoundingMode, out: &str, o_out: Ordering| {
        let (t, o) = Float::tan_with_period_rational_prec_round_ref(x, u, prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(o, o_out);
    };
    let min_exp = -(1i64 << 30);
    // 2 pi x is about 1.57 times the smallest positive Float
    let x = Rational::power_of_2(min_exp - 2);
    test(&x, 1, 1, Nearest, "4.8e-323228497", Greater);
    test(&x, 1, 1, Floor, "2.4e-323228497", Less);
    test(&x, 1, 10, Nearest, "3.7414e-323228497", Less);
    // 2 pi x is about 0.79 times it: below, but above half
    let x = Rational::power_of_2(min_exp - 3);
    test(&x, 1, 10, Nearest, "2.3826e-323228497", Greater);
    test(&x, 1, 10, Down, "0.0", Less);
    let x = -Rational::power_of_2(min_exp - 3);
    test(&x, 1, 10, Nearest, "-2.3826e-323228497", Less);
    test(&x, 1, 10, Ceiling, "-0.0", Greater);
    // 2 pi x is about 0.39 times it: below half
    let x = Rational::power_of_2(min_exp - 4);
    test(&x, 1, 10, Nearest, "0.0", Less);
    test(&x, 1, 10, Up, "2.3826e-323228497", Greater);
    // and with a divisor
    let x = Rational::power_of_2(min_exp);
    test(&x, 100, 10, Nearest, "0.0", Less);
    test(&x, 7, 10, Nearest, "2.3826e-323228497", Greater);
    // a non-dyadic input a third of the way down
    let x = Rational::from_unsigneds(1u32, 3u32) >> (1u64 << 30);
    test(&x, 1, 10, Nearest, "4.9885e-323228497", Less);
}

#[test]
#[should_panic]
fn tan_with_period_rational_prec_round_fail_1() {
    Float::tan_with_period_rational_prec_round(Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn tan_with_period_rational_prec_round_fail_2() {
    Float::tan_with_period_rational_prec_round(Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn tan_with_period_rational_prec_round_fail_3() {
    // a twelfth of a turn is sqrt(3)/3, which is not exact
    Float::tan_with_period_rational_prec_round(Rational::from_unsigneds(1u8, 12), 1, 10, Exact);
}

#[test]
#[should_panic]
fn tan_with_period_rational_prec_round_ref_fail() {
    Float::tan_with_period_rational_prec_round_ref(&Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn tan_with_period_rational_prec_fail() {
    Float::tan_with_period_rational_prec(Rational::ONE, 7, 0);
}

#[allow(clippy::needless_pass_by_value)]
fn tan_with_period_rational_prec_round_properties_helper(
    x: Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (t, o) = Float::tan_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
        if o == Equal {
            let (te, oe) = Float::tan_with_period_rational_prec_round_ref(&x, u, prec, Exact);
            assert_eq!(ComparableFloatRef(&te), ComparableFloatRef(&t));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(Float::tan_with_period_rational_prec_round_ref(
                &x, u, prec, Exact
            ));
        }
        return;
    }
    let (t, o) = Float::tan_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(t.is_valid());
    assert_rounding_ordering_consistent(&t, rm, o);

    let (t_alt, o_alt) = Float::tan_with_period_rational_prec_round_ref(&x, u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    // MPFR rounds the input to a `Float` first, so it cannot see the exact cases of non-dyadic
    // inputs (a twelfth of a turn, say); those are checked separately below.
    if o != Equal
        && let Ok(rrm) = rug_round_try_from_rounding_mode(rm)
        && u32::try_from(u).is_ok()
    {
        let (rug_t, rug_o) = rug_tan_with_period_rational_prec_round(&x, u, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_t)),
            ComparableFloatRef(&t),
            "RUGPROBE x = {x} u = {u} prec = {prec} rm = {rm:?}"
        );
        assert_eq!(rug_o, o);
    }

    // NaN exactly for u = 0
    assert_eq!(t.is_nan(), u == 0);
    if u != 0 {
        // an infinity is a pole, and exact
        if t.is_infinite() {
            assert_eq!(o, Equal);
        } else if t.is_normal() {
            assert_eq!(t.get_prec(), Some(prec));
        }
        // tan is odd (a `Rational` has no negative zero, so x = 0 is excluded), and has period u up
        // to the sign of a zero, which follows the sign of x
        if x != 0u32 {
            let (t_neg, o_neg) = Float::tan_with_period_rational_prec_round(-&x, u, prec, -rm);
            assert_eq!(ComparableFloat(t_neg), ComparableFloat(-&t));
            assert_eq!(o_neg, o.reverse());
        }
        let (t_shifted, o_shifted) =
            Float::tan_with_period_rational_prec_round(&x + Rational::from(u), u, prec, rm);
        assert_eq!(
            ComparableFloat(t_shifted.abs_negative_zero()),
            ComparableFloat(t.abs_negative_zero_ref())
        );
        assert_eq!(o_shifted, o);
        // a `Float` input agrees with the `Float` version
        if let Ok(f) = Float::try_from(&x) {
            let (t_alt, o_alt) = f.tan_with_period_prec_round(u, prec, rm);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (t2, oo) = Float::tan_with_period_rational_prec_round_ref(&x, u, prec, rm);
            assert_eq!(ComparableFloatRef(&t2), ComparableFloatRef(&t));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::tan_with_period_rational_prec_round_ref(
            &x, u, prec, Exact
        ));
    }
}

#[test]
fn tan_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, u, prec, rm)| {
            tan_with_period_rational_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (t, o) = Float::tan_with_period_rational_prec_round(Rational::ZERO, 4, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (t, o) = Float::tan_with_period_rational_prec_round(Rational::ONE, 0, prec, rm);
        assert!(t.is_nan());
        assert_eq!(o, Equal);
        // exact cases, straight from a fraction of a turn: the quarter turns are the zeros and
        // poles, and the odd eighths are ±1
        let one = Float::one_prec(prec);
        for (s, expected) in [
            ("1/4", Float::INFINITY),
            ("1/2", Float::NEGATIVE_ZERO),
            ("-1/2", Float::ZERO),
            ("3/4", Float::NEGATIVE_INFINITY),
            ("-1/4", Float::NEGATIVE_INFINITY),
            ("1", Float::ZERO),
            ("-1", Float::NEGATIVE_ZERO),
            ("1/8", one.clone()),
            ("3/8", -&one),
            ("5/8", one.clone()),
            ("-1/8", -&one),
        ] {
            let (t, o) = Float::tan_with_period_rational_prec_round(
                Rational::from_str(s).unwrap(),
                1,
                prec,
                rm,
            );
            assert_eq!(ComparableFloat(t), ComparableFloat(expected));
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn tan_with_period_rational_prec_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, u, prec, _)| {
            let (t, o) = Float::tan_with_period_rational_prec(x.clone(), u, prec);
            assert!(t.is_valid());
            assert_rounding_ordering_consistent(&t, Nearest, o);
            let (t_alt, o_alt) = Float::tan_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) =
                Float::tan_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            if o != Equal && u32::try_from(u).is_ok() {
                let (rug_t, rug_o) = rug_tan_with_period_rational_prec(&x, u, prec);
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_t)),
                    ComparableFloatRef(&t)
                );
                assert_eq!(rug_o, o);
            }
        },
    );
}

// Fractions of a turn within 2^(-2^30) of a quarter turn, whose tangents underflow (just past a
// half turn) and overflow (just past a quarter turn), including non-dyadic ones that no `Float`
// could express. The near-zero paths work with the exact distance to the multiple of 1/4, so no
// 2^30-bit pi is ever formed.
#[test]
fn test_tan_with_period_rational_underflow_and_overflow() {
    let min_positive = Float::one_prec(10) >> (1u64 << 30);
    let eps = Rational::power_of_2(-((1i64 << 30) + 70));
    // just past a half turn: the tangent is positive and tiny
    let above = Rational::from_unsigneds(1u32, 2u32) + &eps;
    let (t, o) = Float::tan_with_period_rational_prec_round_ref(&above, 1, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    let (t, o) = Float::tan_with_period_rational_prec_round_ref(&above, 1, 10, Up);
    assert_eq!(ComparableFloatRef(&t), ComparableFloatRef(&min_positive));
    assert_eq!(o, Greater);
    // just past a quarter turn: the tangent is negative and beyond the largest finite Float
    let above = Rational::from_unsigneds(1u32, 4u32) + &eps;
    let (t, o) = Float::tan_with_period_rational_prec_round_ref(&above, 1, 10, Nearest);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    let (t, o) = Float::tan_with_period_rational_prec_round_ref(&above, 1, 10, Down);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(-Float::max_finite_value_with_prec(10))
    );
    assert_eq!(o, Greater);
    // a non-dyadic version: 1/4 + 1/(3 * 2^(2^30 + 70)) of a turn
    let above = Rational::from_unsigneds(1u32, 4u32) + eps / Rational::from(3u32);
    let (t, o) = Float::tan_with_period_rational_prec_round_ref(&above, 1, 10, Nearest);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_tan_with_period_rational() {
    fn test<T: PrimitiveFloat>(s: &str, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_tan_with_period_rational::<T>(&x, u)),
            NiceFloat(out)
        );
    }
    fn test_q<T: PrimitiveFloat>(x: &Rational, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_tan_with_period_rational::<T>(x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 360, 0.0);
    test::<f32>("1", 0, f32::NAN);
    test::<f32>("90", 360, f32::INFINITY);
    test::<f32>("-90", 360, f32::NEGATIVE_INFINITY);
    test::<f32>("270", 360, f32::NEGATIVE_INFINITY);
    test::<f32>("180", 360, -0.0);
    test::<f32>("-180", 360, 0.0);
    test::<f32>("45", 360, 1.0);
    test::<f32>("135", 360, -1.0);
    test::<f32>("30", 360, 0.57735026);
    test::<f32>("60", 360, 1.7320508);
    test::<f32>("1/12", 1, 0.57735026);
    test::<f32>("1/3", 1, -1.7320508);
    test::<f32>("1/8", 1, 1.0);
    test::<f32>("1/4", 1, f32::INFINITY);
    test::<f32>("1/2", 1, -0.0);
    test::<f32>("1/20", 1, 0.3249197);
    test::<f32>("1/5", 1, 3.0776834);
    test::<f32>("1/7", 1, 1.2539604);
    test::<f32>("-2/7", 1, 4.381286);
    test::<f32>("22/7", 1, 1.2539604);
    test::<f32>("1", 7, 1.2539604);
    test::<f32>("1000000", 7, 1.2539604);
    test::<f32>("1/1000000", 1, 0.0000062831855);
    test::<f32>("355/113", 360, 0.054886155);
    test::<f64>("0", 360, 0.0);
    test::<f64>("1", 0, f64::NAN);
    test::<f64>("90", 360, f64::INFINITY);
    test::<f64>("-90", 360, f64::NEGATIVE_INFINITY);
    test::<f64>("270", 360, f64::NEGATIVE_INFINITY);
    test::<f64>("180", 360, -0.0);
    test::<f64>("-180", 360, 0.0);
    test::<f64>("45", 360, 1.0);
    test::<f64>("135", 360, -1.0);
    test::<f64>("30", 360, 0.5773502691896257);
    test::<f64>("60", 360, 1.7320508075688772);
    test::<f64>("1/12", 1, 0.5773502691896257);
    test::<f64>("1/3", 1, -1.7320508075688772);
    test::<f64>("1/8", 1, 1.0);
    test::<f64>("1/4", 1, f64::INFINITY);
    test::<f64>("1/2", 1, -0.0);
    test::<f64>("1/20", 1, 0.32491969623290634);
    test::<f64>("1/5", 1, 3.0776835371752536);
    test::<f64>("1/7", 1, 1.2539603376627038);
    test::<f64>("-2/7", 1, 4.381286267534823);
    test::<f64>("22/7", 1, 1.2539603376627038);
    test::<f64>("1", 7, 1.2539603376627038);
    test::<f64>("1000000", 7, 1.2539603376627038);
    test::<f64>("1/1000000", 1, 6.28318530726227e-6);
    test::<f64>("355/113", 360, 0.05488615547794264);
    // tiny inputs, whose tangents are subnormal or zero
    let tiny = |zeros: usize| format!("1/1{}", "0".repeat(zeros));
    test::<f32>(&tiny(40), 1, 6.28318e-40);
    test::<f32>(&tiny(40), 360, 1.746e-42);
    test::<f32>(&tiny(50), 1, 0.0);
    test::<f64>(&tiny(310), 1, 6.28318530717956e-310);
    test::<f64>(&format!("-{}", tiny(310)), 7, -8.9759790102563e-311);
    test::<f64>(&tiny(330), 1, 0.0);
    // just off a pole: the tangent overflows to an infinity of either sign, approaching the pole
    // from below or from above
    let quarter = Rational::from_unsigneds(1u8, 4);
    test_q::<f32>(
        &(&quarter - Rational::power_of_2(-140i64)),
        1,
        f32::INFINITY,
    );
    test_q::<f32>(
        &(&quarter + Rational::power_of_2(-140i64)),
        1,
        f32::NEGATIVE_INFINITY,
    );
    test_q::<f32>(&(&quarter + Rational::power_of_2(-120i64)), 1, -2.115532e35);
    test_q::<f64>(
        &(&quarter - Rational::power_of_2(-1030i64)),
        1,
        f64::INFINITY,
    );
    test_q::<f64>(
        &(&quarter + Rational::power_of_2(-1030i64)),
        1,
        f64::NEGATIVE_INFINITY,
    );
    test_q::<f64>(
        &(&quarter + Rational::power_of_2(-1000i64)),
        1,
        -1.7053589139920642e300,
    );
    // just off a zero: the tangent underflows to a subnormal or to zero
    let half = Rational::from_unsigneds(1u8, 2);
    test_q::<f32>(&(&half + Rational::power_of_2(-140i64)), 1, 4.508e-42);
    test_q::<f32>(&(&half + Rational::power_of_2(-160i64)), 1, 0.0);
    test_q::<f64>(
        &(&half + Rational::power_of_2(-1030i64)),
        1,
        5.46115288092257e-310,
    );
    test_q::<f64>(&(&half + Rational::power_of_2(-1080i64)), 1, 0.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_tan_with_period_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, u)| {
        let t = primitive_float_tan_with_period_rational::<T>(&x, u);
        assert_eq!(t.is_nan(), u == 0);
        if u != 0 {
            // odd (a `Rational` has no negative zero), and periodic with period u up to the sign of
            // a zero
            if x != 0u32 {
                assert_eq!(
                    NiceFloat(primitive_float_tan_with_period_rational::<T>(&-&x, u)),
                    NiceFloat(-t)
                );
            }
            assert_eq!(
                NiceFloat(
                    primitive_float_tan_with_period_rational::<T>(&(&x + Rational::from(u)), u)
                        .abs()
                ),
                NiceFloat(t.abs())
            );
            // MPFR agrees, except that it cannot see the exact cases of non-dyadic inputs
            let (t_float, o) =
                Float::tan_with_period_rational_prec_ref(&x, u, T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&t_float, Nearest).0),
                NiceFloat(t)
            );
            if o != Equal {
                let rug_t = rug_tan_with_period_rational_prec(&x, u, T::MANTISSA_WIDTH + 64).0;
                let rug_t: T =
                    T::rounding_from(&<Float as From<&rug::Float>>::from(&rug_t), Nearest).0;
                assert_eq!(NiceFloat(rug_t), NiceFloat(t));
            }
        }
    });

    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        // The tangent of a finite nonzero primitive float, taken through the `Rational` path,
        // matches the direct primitive-float tangent (a `Rational` cannot carry the sign of a
        // zero).
        if x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_tan_with_period_rational::<T>(
                    &Rational::exact_from(x),
                    u
                )),
                NiceFloat(primitive_float_tan_with_period(x, u))
            );
        }
    });
}

#[test]
fn primitive_float_tan_with_period_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_tan_with_period_rational_properties_helper);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_tan() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_tan(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, 0.0);
    test::<f32>(-0.0, -0.0);
    test::<f32>(1.0, 1.5574077);
    test::<f32>(-1.0, -1.5574077);
    test::<f32>(0.5, 0.5463025);
    test::<f32>(2.0, -2.1850398);
    test::<f32>(100.0, -0.58721393);
    test::<f32>(10000000000.0, -0.5583496);
    test::<f32>(1.0e-10, 1.0e-10);
    test::<f32>(1.5707964, -22877332.0);
    test::<f32>(core::f32::consts::PI, 8.742278e-8);
    test::<f32>(1.0e-45, 1.0e-45);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, 0.0);
    test::<f64>(-0.0, -0.0);
    test::<f64>(1.0, 1.5574077246549023);
    test::<f64>(-1.0, -1.5574077246549023);
    test::<f64>(0.5, 0.5463024898437905);
    test::<f64>(2.0, -2.185039863261519);
    test::<f64>(100.0, -0.5872139151569291);
    test::<f64>(10000000000.0, -0.5583496378112418);
    test::<f64>(1.0e-10, 1.0e-10);
    test::<f64>(core::f64::consts::FRAC_PI_2, 1.633123935319537e16);
    test::<f64>(core::f64::consts::PI, -1.2246467991473532e-16);
    test::<f64>(1.0e300, 1.4214488238747245);
    test::<f64>(5.0e-324, 5.0e-324);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_tan_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let t = primitive_float_tan(x);
        // NaN exactly for NaN and infinite inputs
        assert_eq!(t.is_nan(), !x.is_finite());
        if x.is_finite() {
            // odd
            assert_eq!(NiceFloat(primitive_float_tan(-x)), NiceFloat(-t));
            // the result is the correctly rounded tangent, as computed by MPFR with 64 bits to
            // spare, so that a subnormal result is rounded once by the conversion
            let rug_t = rug_tan_prec(
                &rug::Float::exact_from(&Float::from(x)),
                T::MANTISSA_WIDTH + 64,
            )
            .0;
            let rug_t: T = T::rounding_from(&<Float as From<&rug::Float>>::from(&rug_t), Nearest).0;
            assert_eq!(NiceFloat(rug_t), NiceFloat(t));
        }
    });
}

#[test]
fn primitive_float_tan_properties() {
    apply_fn_to_primitive_floats!(primitive_float_tan_properties_helper);
}
