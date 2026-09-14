// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Cot, CotAssign, PowerOf2};
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
use malachite_float::float::arithmetic::cot::{
    primitive_float_cot, primitive_float_cot_pi, primitive_float_cot_pi_rational,
    primitive_float_cot_rational, primitive_float_cot_with_period,
    primitive_float_cot_with_period_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::cot::{
    cot_with_period_naive, cot_with_period_rational_naive, rug_cot, rug_cot_prec,
    rug_cot_prec_round, rug_cot_rational_prec, rug_cot_rational_prec_round, rug_cot_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_2, float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_37, float_unsigned_rounding_mode_triple_gen_var_39,
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

// Rows reuse the tangent test's inputs. Branches of `cot_prec_round_normal_ref` covered:
// - tiny x: the reciprocal shortcut, including the power-of-two case that needs the correction
// - the general Ziv loop, at the first working precision and after a retry
// - |x| near a nonzero multiple of pi: a large result, from the sine's near-zero path inside
//   `sin_cos`
// - |x| near an odd multiple of pi/2: a tiny result, from the cosine's near-zero path

// Whether rounding `x` to `T` is a tie: it is exactly representable one bit past `T`'s precision
// but not at it. MPFR's wider results are compared against the primitive-float functions only when
// this is false, since a tie is broken by the value's own last bit rather than by the true result.
#[allow(clippy::type_repetition_in_bounds)]
fn ties<T: PrimitiveFloat>(x: &Float) -> bool {
    x.is_normal()
        && Float::from_float_prec_round_ref(x, T::MANTISSA_WIDTH + 2, Down).1 == Equal
        && Float::from_float_prec_round_ref(x, T::MANTISSA_WIDTH + 1, Down).1 != Equal
}

#[test]
fn test_cot_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (t, o) = x.clone().cot_prec_round(prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.cot_prec_round_ref(prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.cot_prec_round_assign(prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) = rug_cot_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
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
    test("0.0", "0x0.0", 1, Floor, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Floor, "-Infinity", "-Infinity", Equal);
    test("0.0", "0x0.0", 1, Exact, "Infinity", "Infinity", Equal);
    test("-0.0", "-0x0.0", 1, Exact, "-Infinity", "-Infinity", Equal);
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Nearest,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Floor,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        2,
        Nearest,
        "4.0",
        "0x4.0#2",
        Greater,
    );
    test("0.25", "0x0.4#1", 1, Floor, "2.0", "0x2.0#1", Less);
    test("0.25", "0x0.4#1", 1, Nearest, "4.0", "0x4.0#1", Greater);
    test("0.25", "0x0.4#1", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Nearest, "Infinity", "Infinity", Equal);
    test(
        "-0.0",
        "-0x0.0",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("0.0", "0x0.0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", 10, Floor, "0.64160", "0x0.a44#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "0.64258",
        "0x0.a48#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "0.64258",
        "0x0.a48#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Floor,
        "0.64209261593433070300641998659417",
        "0x0.a4602e8270e7ec1a29d95ba18#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Ceiling,
        "0.64209261593433070300641998659496",
        "0x0.a4602e8270e7ec1a29d95ba19#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Nearest,
        "0.64209261593433070300641998659417",
        "0x0.a4602e8270e7ec1a29d95ba18#100",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-0.64258",
        "-0x0.a48#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "-0.45752",
        "-0x0.752#10",
        Greater,
    );
    test("3.0", "0x3.0#2", 10, Nearest, "-7.0156", "-0x7.04#10", Less);
    test("4.0", "0x4.0#1", 10, Nearest, "0.86328", "0x0.dd0#10", Less);
    test(
        "4.0",
        "0x4.0#1",
        100,
        Nearest,
        "0.86369115445061661394651434594108",
        "0x0.dd1add0e35bae7d8e0e78e46d#100",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        10,
        Nearest,
        "-1.7031",
        "-0x1.b40#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Floor,
        "-1.7029569194264692160987314595587",
        "-0x1.b3f4fc136effe8ad8b2d8c54c#100",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Ceiling,
        "-1.7029569194264692160987314595571",
        "-0x1.b3f4fc136effe8ad8b2d8c54a#100",
        Greater,
    );
    test(
        "1.00000e6",
        "0xf.424E+4#14",
        64,
        Nearest,
        "-2.67648433962834510886",
        "-0x2.ad2e13e2f5bbf17c#64",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        50,
        Nearest,
        "1.8304877217124513",
        "0x1.d49ad7e47c0a0#50",
        Less,
    );
    test(
        "0.102",
        "0x0.1a#4",
        50,
        Nearest,
        "9.8122763763439167",
        "0x9.cff15837b5e4#50",
        Less,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        50,
        Nearest,
        "9995560252.5090942",
        "0x253c8253c.8254#50",
        Greater,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        10,
        Floor,
        "9.9824e9",
        "0x2.53E+8#10",
        Less,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        10,
        Ceiling,
        "9.9992e9",
        "0x2.54E+8#10",
        Greater,
    );
    test(
        "1.570796326794896600",
        "0x1.921fb54442d183#57",
        53,
        Nearest,
        "1.9598976533924290e-17",
        "0x1.69898cc51701cE-14#53",
        Greater,
    );
    test(
        "3.14159265358979289",
        "0x3.243f6a8885a2f#54",
        53,
        Nearest,
        "-2902679387770655.0",
        "-0xa4ff8b5ce0f1f.0#53",
        Greater,
    );
    test(
        "6.28318530717958579",
        "0x6.487ed5110b45e#54",
        53,
        Nearest,
        "-1451339693885327.5",
        "-0x527fc5ae7078f.8#53",
        Greater,
    );
    test(
        "9.979e99",
        "0x1.24E+83#7",
        53,
        Nearest,
        "0.47601329138447984",
        "0x0.79dc01cef5292c#53",
        Greater,
    );
    test("3.0", "0x3.0#2", 1, Nearest, "-8.0", "-0x8.0#1", Less);
    test("3.0", "0x3.0#2", 1, Floor, "-8.0", "-0x8.0#1", Less);
    test("3.0", "0x3.0#2", 1, Ceiling, "-4.0", "-0x4.0#1", Greater);
    test("3.0", "0x3.0#2", 2, Nearest, "-8.0", "-0x8.0#2", Less);
    test("0.25", "0x0.4#1", 1, Down, "2.0", "0x2.0#1", Less);
    test("1.0", "0x1.0#1", 1, Down, "0.50", "0x0.8#1", Less);
    test("2.0", "0x2.0#1", 1, Down, "-0.25", "-0x0.4#1", Greater);
    test("4.0", "0x4.0#1", 1, Down, "0.50", "0x0.8#1", Less);
    test(
        "-3.495934488151859089160804055e56",
        "-0xe.41ed086a5791d9e5b2924E+46#87",
        2,
        Down,
        "6.0",
        "0x6.0#2",
        Less,
    );
    test(
        "6.28318536",
        "0x6.487ed6#26",
        10,
        Nearest,
        "1.7990e7",
        "0x1.128E+6#10",
        Greater,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        53,
        Nearest,
        "6.1232339957367660e-17",
        "0x4.69898cc51701cE-14#53",
        Greater,
    );
    test(
        "1.5707963267948966192313216916397514420985846996875529104874722",
        "0x1.921fb54442d18469898cc51701b839a252049c1114cf98e804#200",
        100,
        Floor,
        "5.7099684971243490026437400060191e-62",
        "0x1.77d4c76273644a29410f31c68E-51#100",
        Less,
    );
    test(
        "4.7123889803846898576939650749192543286",
        "0x4.b65f1fccc8748d3c9ca64f450528b0#120",
        120,
        Ceiling,
        "-2.3305317247657495256428231620634736167e-36",
        "-0x3.1909f22bccc1913547f3b9881a9d88E-30#120",
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        64,
        Nearest,
        "19933988149058553534.0",
        "0x114a3c09b557b46be.0#64",
        Greater,
    );
    test(
        "2.5",
        "0x2.8#3",
        10,
        Nearest,
        "-1.3379",
        "-0x1.568#10",
        Greater,
    );
    test("-2.5", "-0x2.8#3", 10, Floor, "1.3379", "0x1.568#10", Less);
    test(
        "2.99976",
        "0x2.fff#14",
        20,
        Nearest,
        "-7.0030136",
        "-0x7.00c58#20",
        Greater,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        10,
        Ceiling,
        "7.0156",
        "0x7.04#10",
        Greater,
    );
    test(
        "6.28318548",
        "0x6.487ed8#24",
        10,
        Nearest,
        "5.7180e6",
        "0x5.74E+5#10",
        Less,
    );
    test(
        "6.28318548",
        "0x6.487ed8#24",
        10,
        Floor,
        "5.7180e6",
        "0x5.74E+5#10",
        Less,
    );
    test(
        "6.283185307179586476925286766559005788",
        "0x6.487ed5110b4611a62633145c06e10#117",
        100,
        Nearest,
        "5.2200807345404618966944832399344e34",
        "0xa.0db2d173bc0046c15a707178E+28#100",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Floor,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Down,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Ceiling,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Down,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Floor,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Ceiling,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Down,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Nearest,
        "1.0e323228496",
        "0x4.0E+268435455#1",
        Less,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Ceiling,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Down,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Up,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Nearest,
        "-1.0e323228496",
        "-0x4.0E+268435455#1",
        Greater,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Floor,
        "2.0965e323228496",
        "0x7.feE+268435455#10",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
}

#[test]
#[should_panic]
fn cot_prec_round_fail() {
    Float::ONE.cot_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn cot_round_fail() {
    Float::ONE.cot_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn cot_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (s, o) = x.clone().cot_prec_round(prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = x.cot_prec_round_ref(prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.cot_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_o) = rug_cot_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    }

    // cot is odd
    let (s_neg, o_neg) = (-&x).cot_prec_round(prec, -rm);
    assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
    assert_eq!(o_neg, o.reverse());

    if s.is_normal() {
        assert_eq!(s.get_prec(), Some(prec));
    }

    if o == Equal {
        // cot is exact only at ±0, where it is ±infinity, and for NaN and ±infinity, where it is
        // NaN: the result is rounding-mode-invariant
        if x.is_finite() {
            assert_eq!(x, 0u32);
            assert_eq!(
                ComparableFloatRef(&s),
                ComparableFloatRef(&if x.is_sign_negative() {
                    Float::NEGATIVE_INFINITY
                } else {
                    Float::INFINITY
                })
            );
        }
        for rm2 in exhaustive_rounding_modes() {
            let (s2, o2) = x.cot_prec_round_ref(prec, rm2);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.cot_prec_round_ref(prec, Exact));
    }
}

#[test]
fn cot_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        cot_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.cot_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::INFINITY.cot_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_INFINITY.cot_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        // cot(+0) = +infinity and cot(-0) = -infinity, exactly
        let (s, o) = Float::ZERO.cot_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.cot_prec_round(prec, rm);
        assert_eq!(
            ComparableFloat(s),
            ComparableFloat(Float::NEGATIVE_INFINITY)
        );
        assert_eq!(o, Equal);
    });
}

#[test]
fn cot_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (s, o) = x.clone().cot_round(rm);
        assert!(s.is_valid());
        assert_rounding_ordering_consistent(&s, rm, o);
        let (s_alt, o_alt) = x.cot_round_ref(rm);
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.cot_round_assign(rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (s_alt, o_alt) = x.cot_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_s, rug_o) = rug_cot_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn cot_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (s, o) = x.clone().cot_prec(prec);
        assert!(s.is_valid());
        let (s_alt, o_alt) = x.cot_prec_ref(prec);
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.cot_prec_assign(prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (s_alt, o_alt) = x.cot_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (rug_s, rug_o) = rug_cot_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn cot_properties() {
    float_gen().test_properties(|x| {
        let s = x.clone().cot();
        assert!(s.is_valid());
        let s_alt = (&x).cot();
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));

        let mut x_alt = x.clone();
        x_alt.cot_assign();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));

        let (s_alt, _) = x.cot_prec_round_ref(x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_cot(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&s)
        );

        // cot is odd
        assert_eq!(ComparableFloat((-&x).cot()), ComparableFloat(-&s));
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_cot() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_cot(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, f32::INFINITY);
    test::<f32>(-0.0, f32::NEGATIVE_INFINITY);
    test::<f32>(1.0, 0.64209265);
    test::<f32>(-1.0, -0.64209265);
    test::<f32>(0.5, 1.8304877);
    test::<f32>(2.0, -0.45765755);
    test::<f32>(100.0, -1.7029569);
    test::<f32>(10000000000.0, -1.7909925);
    test::<f32>(1.0e-10, 10000000000.0);
    test::<f32>(core::f32::consts::FRAC_PI_2, -4.371139e-8);
    test::<f32>(core::f32::consts::PI, 11438666.0);
    test::<f32>(1.0e-45, f32::INFINITY);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, f64::INFINITY);
    test::<f64>(-0.0, f64::NEGATIVE_INFINITY);
    test::<f64>(1.0, 0.6420926159343308);
    test::<f64>(-1.0, -0.6420926159343308);
    test::<f64>(0.5, 1.830487721712452);
    test::<f64>(2.0, -0.45765755436028577);
    test::<f64>(100.0, -1.7029569194264693);
    test::<f64>(10000000000.0, -1.790992475467611);
    test::<f64>(1.0e-10, 10000000000.0);
    test::<f64>(core::f64::consts::FRAC_PI_2, 6.123233995736766e-17);
    test::<f64>(core::f64::consts::PI, -8165619676597685.0);
    test::<f64>(1.0e300, 0.7035075643976418);
    test::<f64>(5.0e-324, f64::INFINITY);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_cot_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let t = primitive_float_cot(x);
        // NaN exactly for NaN and infinite inputs
        assert_eq!(t.is_nan(), !x.is_finite());
        if x.is_finite() {
            // odd
            assert_eq!(NiceFloat(primitive_float_cot(-x)), NiceFloat(-t));
            // the result is the correctly rounded cotangent, as computed by MPFR with 64 bits to
            // spare, so that a subnormal result is rounded once by the conversion
            let rug_t = rug_cot_prec(
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
fn primitive_float_cot_properties() {
    apply_fn_to_primitive_floats!(primitive_float_cot_properties_helper);
}

// Rows reuse the sine test's inputs, including the non-dyadic ones whose cotangents MPFR cannot see
// exactly, since it must round the input first.
#[test]
fn test_cot_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (t, o) = Float::cot_rational_prec_round(x.clone(), prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = Float::cot_rational_prec_round_ref(&x, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = Float::cot_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) = Float::cot_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) = rug_cot_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 1, Down, "Infinity", "Infinity", Equal);
    test("0", 1, Up, "Infinity", "Infinity", Equal);
    test("0", 1, Floor, "Infinity", "Infinity", Equal);
    test("0", 1, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 1, Nearest, "Infinity", "Infinity", Equal);
    test("0", 1, Exact, "Infinity", "Infinity", Equal);
    test("0", 5, Nearest, "Infinity", "Infinity", Equal);
    test("0", 10, Down, "Infinity", "Infinity", Equal);
    test("0", 10, Up, "Infinity", "Infinity", Equal);
    test("0", 10, Floor, "Infinity", "Infinity", Equal);
    test("0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("0", 10, Exact, "Infinity", "Infinity", Equal);
    test("0", 20, Nearest, "Infinity", "Infinity", Equal);
    test("0", 53, Down, "Infinity", "Infinity", Equal);
    test("0", 53, Up, "Infinity", "Infinity", Equal);
    test("0", 53, Floor, "Infinity", "Infinity", Equal);
    test("0", 53, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 53, Nearest, "Infinity", "Infinity", Equal);
    test("0", 53, Exact, "Infinity", "Infinity", Equal);
    test("0", 100, Nearest, "Infinity", "Infinity", Equal);
    test("1", 1, Down, "0.50", "0x0.8#1", Less);
    test("1", 1, Up, "1.0", "0x1.0#1", Greater);
    test("1", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("1", 5, Nearest, "0.656", "0x0.a8#5", Greater);
    test("1", 10, Down, "0.64160", "0x0.a44#10", Less);
    test("1", 10, Up, "0.64258", "0x0.a48#10", Greater);
    test("1", 10, Floor, "0.64160", "0x0.a44#10", Less);
    test("1", 10, Ceiling, "0.64258", "0x0.a48#10", Greater);
    test("1", 10, Nearest, "0.64258", "0x0.a48#10", Greater);
    test("1", 20, Nearest, "0.64209270", "0x0.a4603#20", Greater);
    test(
        "1",
        53,
        Down,
        "0.64209261593433065",
        "0x0.a4602e8270e7e8#53",
        Less,
    );
    test(
        "1",
        53,
        Up,
        "0.64209261593433076",
        "0x0.a4602e8270e7f0#53",
        Greater,
    );
    test(
        "1",
        53,
        Floor,
        "0.64209261593433065",
        "0x0.a4602e8270e7e8#53",
        Less,
    );
    test(
        "1",
        53,
        Ceiling,
        "0.64209261593433076",
        "0x0.a4602e8270e7f0#53",
        Greater,
    );
    test(
        "1",
        53,
        Nearest,
        "0.64209261593433076",
        "0x0.a4602e8270e7f0#53",
        Greater,
    );
    test(
        "1",
        100,
        Nearest,
        "0.64209261593433070300641998659417",
        "0x0.a4602e8270e7ec1a29d95ba18#100",
        Less,
    );
    test("-1", 1, Down, "-0.50", "-0x0.8#1", Greater);
    test("-1", 1, Up, "-1.0", "-0x1.0#1", Less);
    test("-1", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-1", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-1", 1, Nearest, "-0.50", "-0x0.8#1", Greater);
    test("-1", 5, Nearest, "-0.656", "-0x0.a8#5", Less);
    test("-1", 10, Down, "-0.64160", "-0x0.a44#10", Greater);
    test("-1", 10, Up, "-0.64258", "-0x0.a48#10", Less);
    test("-1", 10, Floor, "-0.64258", "-0x0.a48#10", Less);
    test("-1", 10, Ceiling, "-0.64160", "-0x0.a44#10", Greater);
    test("-1", 10, Nearest, "-0.64258", "-0x0.a48#10", Less);
    test("-1", 20, Nearest, "-0.64209270", "-0x0.a4603#20", Less);
    test(
        "-1",
        53,
        Down,
        "-0.64209261593433065",
        "-0x0.a4602e8270e7e8#53",
        Greater,
    );
    test(
        "-1",
        53,
        Up,
        "-0.64209261593433076",
        "-0x0.a4602e8270e7f0#53",
        Less,
    );
    test(
        "-1",
        53,
        Floor,
        "-0.64209261593433076",
        "-0x0.a4602e8270e7f0#53",
        Less,
    );
    test(
        "-1",
        53,
        Ceiling,
        "-0.64209261593433065",
        "-0x0.a4602e8270e7e8#53",
        Greater,
    );
    test(
        "-1",
        53,
        Nearest,
        "-0.64209261593433076",
        "-0x0.a4602e8270e7f0#53",
        Less,
    );
    test(
        "-1",
        100,
        Nearest,
        "-0.64209261593433070300641998659417",
        "-0x0.a4602e8270e7ec1a29d95ba18#100",
        Greater,
    );
    test("1/2", 1, Down, "1.0", "0x1.0#1", Less);
    test("1/2", 1, Up, "2.0", "0x2.0#1", Greater);
    test("1/2", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1/2", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1/2", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1/2", 5, Nearest, "1.81", "0x1.d#5", Less);
    test("1/2", 10, Down, "1.8301", "0x1.d48#10", Less);
    test("1/2", 10, Up, "1.8320", "0x1.d50#10", Greater);
    test("1/2", 10, Floor, "1.8301", "0x1.d48#10", Less);
    test("1/2", 10, Ceiling, "1.8320", "0x1.d50#10", Greater);
    test("1/2", 10, Nearest, "1.8301", "0x1.d48#10", Less);
    test("1/2", 20, Nearest, "1.8304882", "0x1.d49ae#20", Greater);
    test(
        "1/2",
        53,
        Down,
        "1.8304877217124518",
        "0x1.d49ad7e47c0a2#53",
        Less,
    );
    test(
        "1/2",
        53,
        Up,
        "1.8304877217124520",
        "0x1.d49ad7e47c0a3#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Floor,
        "1.8304877217124518",
        "0x1.d49ad7e47c0a2#53",
        Less,
    );
    test(
        "1/2",
        53,
        Ceiling,
        "1.8304877217124520",
        "0x1.d49ad7e47c0a3#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Nearest,
        "1.8304877217124520",
        "0x1.d49ad7e47c0a3#53",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "1.8304877217124519192680194389686",
        "0x1.d49ad7e47c0a2a4d1064f77f4#100",
        Less,
    );
    test("1/3", 1, Down, "2.0", "0x2.0#1", Less);
    test("1/3", 1, Up, "4.0", "0x4.0#1", Greater);
    test("1/3", 1, Floor, "2.0", "0x2.0#1", Less);
    test("1/3", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("1/3", 1, Nearest, "2.0", "0x2.0#1", Less);
    test("1/3", 5, Nearest, "2.88", "0x2.e#5", Less);
    test("1/3", 10, Down, "2.8867", "0x2.e3#10", Less);
    test("1/3", 10, Up, "2.8906", "0x2.e4#10", Greater);
    test("1/3", 10, Floor, "2.8867", "0x2.e3#10", Less);
    test("1/3", 10, Ceiling, "2.8906", "0x2.e4#10", Greater);
    test("1/3", 10, Nearest, "2.8867", "0x2.e3#10", Less);
    test("1/3", 20, Nearest, "2.8880577", "0x2.e357c#20", Greater);
    test(
        "1/3",
        53,
        Down,
        "2.8880570362772766",
        "0x2.e357b4b7cb28a#53",
        Less,
    );
    test(
        "1/3",
        53,
        Up,
        "2.8880570362772771",
        "0x2.e357b4b7cb28c#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Floor,
        "2.8880570362772766",
        "0x2.e357b4b7cb28a#53",
        Less,
    );
    test(
        "1/3",
        53,
        Ceiling,
        "2.8880570362772771",
        "0x2.e357b4b7cb28c#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Nearest,
        "2.8880570362772771",
        "0x2.e357b4b7cb28c#53",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "2.8880570362772768592053002743538",
        "0x2.e357b4b7cb28b1c850ee99938#100",
        Less,
    );
    test("-1/3", 1, Down, "-2.0", "-0x2.0#1", Greater);
    test("-1/3", 1, Up, "-4.0", "-0x4.0#1", Less);
    test("-1/3", 1, Floor, "-4.0", "-0x4.0#1", Less);
    test("-1/3", 1, Ceiling, "-2.0", "-0x2.0#1", Greater);
    test("-1/3", 1, Nearest, "-2.0", "-0x2.0#1", Greater);
    test("-1/3", 5, Nearest, "-2.88", "-0x2.e#5", Greater);
    test("-1/3", 10, Down, "-2.8867", "-0x2.e3#10", Greater);
    test("-1/3", 10, Up, "-2.8906", "-0x2.e4#10", Less);
    test("-1/3", 10, Floor, "-2.8906", "-0x2.e4#10", Less);
    test("-1/3", 10, Ceiling, "-2.8867", "-0x2.e3#10", Greater);
    test("-1/3", 10, Nearest, "-2.8867", "-0x2.e3#10", Greater);
    test("-1/3", 20, Nearest, "-2.8880577", "-0x2.e357c#20", Less);
    test(
        "-1/3",
        53,
        Down,
        "-2.8880570362772766",
        "-0x2.e357b4b7cb28a#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Up,
        "-2.8880570362772771",
        "-0x2.e357b4b7cb28c#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Floor,
        "-2.8880570362772771",
        "-0x2.e357b4b7cb28c#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Ceiling,
        "-2.8880570362772766",
        "-0x2.e357b4b7cb28a#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Nearest,
        "-2.8880570362772771",
        "-0x2.e357b4b7cb28c#53",
        Less,
    );
    test(
        "-1/3",
        100,
        Nearest,
        "-2.8880570362772768592053002743538",
        "-0x2.e357b4b7cb28b1c850ee99938#100",
        Greater,
    );
    test("3/5", 1, Down, "1.0", "0x1.0#1", Less);
    test("3/5", 1, Up, "2.0", "0x2.0#1", Greater);
    test("3/5", 1, Floor, "1.0", "0x1.0#1", Less);
    test("3/5", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("3/5", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("3/5", 10, Down, "1.4609", "0x1.760#10", Less);
    test("3/5", 10, Up, "1.4629", "0x1.768#10", Greater);
    test("3/5", 10, Floor, "1.4609", "0x1.760#10", Less);
    test("3/5", 10, Ceiling, "1.4629", "0x1.768#10", Greater);
    test("3/5", 10, Nearest, "1.4609", "0x1.760#10", Less);
    test(
        "3/5",
        53,
        Down,
        "1.4616959470781021",
        "0x1.7631b4a1656d1#53",
        Less,
    );
    test(
        "3/5",
        53,
        Up,
        "1.4616959470781024",
        "0x1.7631b4a1656d2#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Floor,
        "1.4616959470781021",
        "0x1.7631b4a1656d1#53",
        Less,
    );
    test(
        "3/5",
        53,
        Ceiling,
        "1.4616959470781024",
        "0x1.7631b4a1656d2#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Nearest,
        "1.4616959470781021",
        "0x1.7631b4a1656d1#53",
        Less,
    );
    test(
        "3/5",
        100,
        Nearest,
        "1.4616959470781021403393084274064",
        "0x1.7631b4a1656d10a6be1f9b8d4#100",
        Less,
    );
    test("22/7", 1, Down, "5.1e2", "0x2.0E+2#1", Less);
    test("22/7", 1, Up, "1.0e3", "0x4.0E+2#1", Greater);
    test("22/7", 1, Floor, "5.1e2", "0x2.0E+2#1", Less);
    test("22/7", 1, Ceiling, "1.0e3", "0x4.0E+2#1", Greater);
    test("22/7", 1, Nearest, "1.0e3", "0x4.0E+2#1", Greater);
    test("22/7", 5, Nearest, "800.0", "0x3.2E+2#5", Greater);
    test("22/7", 10, Down, "790.00", "0x316.0#10", Less);
    test("22/7", 10, Up, "791.00", "0x317.0#10", Greater);
    test("22/7", 10, Floor, "790.00", "0x316.0#10", Less);
    test("22/7", 10, Ceiling, "791.00", "0x317.0#10", Greater);
    test("22/7", 10, Nearest, "791.00", "0x317.0#10", Greater);
    test("22/7", 20, Nearest, "790.83301", "0x316.d54#20", Greater);
    test(
        "22/7",
        53,
        Down,
        "790.83270443113281",
        "0x316.d52c1e1af32#53",
        Less,
    );
    test(
        "22/7",
        53,
        Up,
        "790.83270443113292",
        "0x316.d52c1e1af34#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Floor,
        "790.83270443113281",
        "0x316.d52c1e1af32#53",
        Less,
    );
    test(
        "22/7",
        53,
        Ceiling,
        "790.83270443113292",
        "0x316.d52c1e1af34#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Nearest,
        "790.83270443113292",
        "0x316.d52c1e1af34#53",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "790.83270443113289503853207687955",
        "0x316.d52c1e1af337af1a2226a54#100",
        Greater,
    );
    test("-22/7", 1, Down, "-5.1e2", "-0x2.0E+2#1", Greater);
    test("-22/7", 1, Up, "-1.0e3", "-0x4.0E+2#1", Less);
    test("-22/7", 1, Floor, "-1.0e3", "-0x4.0E+2#1", Less);
    test("-22/7", 1, Ceiling, "-5.1e2", "-0x2.0E+2#1", Greater);
    test("-22/7", 1, Nearest, "-1.0e3", "-0x4.0E+2#1", Less);
    test("-22/7", 5, Nearest, "-800.0", "-0x3.2E+2#5", Less);
    test("-22/7", 10, Down, "-790.00", "-0x316.0#10", Greater);
    test("-22/7", 10, Up, "-791.00", "-0x317.0#10", Less);
    test("-22/7", 10, Floor, "-791.00", "-0x317.0#10", Less);
    test("-22/7", 10, Ceiling, "-790.00", "-0x316.0#10", Greater);
    test("-22/7", 10, Nearest, "-791.00", "-0x317.0#10", Less);
    test("-22/7", 20, Nearest, "-790.83301", "-0x316.d54#20", Less);
    test(
        "-22/7",
        53,
        Down,
        "-790.83270443113281",
        "-0x316.d52c1e1af32#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Up,
        "-790.83270443113292",
        "-0x316.d52c1e1af34#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Floor,
        "-790.83270443113292",
        "-0x316.d52c1e1af34#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Ceiling,
        "-790.83270443113281",
        "-0x316.d52c1e1af32#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Nearest,
        "-790.83270443113292",
        "-0x316.d52c1e1af34#53",
        Less,
    );
    test(
        "-22/7",
        100,
        Nearest,
        "-790.83270443113289503853207687955",
        "-0x316.d52c1e1af337af1a2226a54#100",
        Less,
    );
    test("355/113", 1, Down, "2.1e6", "0x2.0E+5#1", Less);
    test("355/113", 1, Up, "4.2e6", "0x4.0E+5#1", Greater);
    test("355/113", 1, Floor, "2.1e6", "0x2.0E+5#1", Less);
    test("355/113", 1, Ceiling, "4.2e6", "0x4.0E+5#1", Greater);
    test("355/113", 1, Nearest, "4.2e6", "0x4.0E+5#1", Greater);
    test("355/113", 5, Nearest, "3.80e6", "0x3.aE+5#5", Greater);
    test("355/113", 10, Down, "3.7478e6", "0x3.93E+5#10", Less);
    test("355/113", 10, Up, "3.7519e6", "0x3.94E+5#10", Greater);
    test("355/113", 10, Floor, "3.7478e6", "0x3.93E+5#10", Less);
    test("355/113", 10, Ceiling, "3.7519e6", "0x3.94E+5#10", Greater);
    test("355/113", 10, Nearest, "3.7478e6", "0x3.93E+5#10", Less);
    test("355/113", 20, Nearest, "3748628.0", "0x393314.0#20", Less);
    test(
        "355/113",
        53,
        Down,
        "3748629.0926627265",
        "0x393315.17b8be94#53",
        Less,
    );
    test(
        "355/113",
        53,
        Up,
        "3748629.0926627270",
        "0x393315.17b8be96#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Floor,
        "3748629.0926627265",
        "0x393315.17b8be94#53",
        Less,
    );
    test(
        "355/113",
        53,
        Ceiling,
        "3748629.0926627270",
        "0x393315.17b8be96#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Nearest,
        "3748629.0926627270",
        "0x393315.17b8be96#53",
        Greater,
    );
    test(
        "355/113",
        100,
        Nearest,
        "3748629.0926627268654052703099727",
        "0x393315.17b8be9571f1d3966c7c#100",
        Greater,
    );
    test("3", 1, Down, "-4.0", "-0x4.0#1", Greater);
    test("3", 1, Up, "-8.0", "-0x8.0#1", Less);
    test("3", 1, Floor, "-8.0", "-0x8.0#1", Less);
    test("3", 1, Ceiling, "-4.0", "-0x4.0#1", Greater);
    test("3", 1, Nearest, "-8.0", "-0x8.0#1", Less);
    test("3", 5, Nearest, "-7.00", "-0x7.0#5", Greater);
    test("3", 10, Down, "-7.0078", "-0x7.02#10", Greater);
    test("3", 10, Up, "-7.0156", "-0x7.04#10", Less);
    test("3", 10, Floor, "-7.0156", "-0x7.04#10", Less);
    test("3", 10, Ceiling, "-7.0078", "-0x7.02#10", Greater);
    test("3", 10, Nearest, "-7.0156", "-0x7.04#10", Less);
    test("3", 20, Nearest, "-7.0152512", "-0x7.03e78#20", Greater);
    test(
        "3",
        53,
        Down,
        "-7.0152525514345330",
        "-0x7.03e7975997854#53",
        Greater,
    );
    test(
        "3",
        53,
        Up,
        "-7.0152525514345339",
        "-0x7.03e7975997858#53",
        Less,
    );
    test(
        "3",
        53,
        Floor,
        "-7.0152525514345339",
        "-0x7.03e7975997858#53",
        Less,
    );
    test(
        "3",
        53,
        Ceiling,
        "-7.0152525514345330",
        "-0x7.03e7975997854#53",
        Greater,
    );
    test(
        "3",
        53,
        Nearest,
        "-7.0152525514345339",
        "-0x7.03e7975997858#53",
        Less,
    );
    test(
        "3",
        100,
        Nearest,
        "-7.0152525514345334694285513795254",
        "-0x7.03e797599785641d3841bee18#100",
        Greater,
    );
    test("100", 1, Down, "-1.0", "-0x1.0#1", Greater);
    test("100", 1, Up, "-2.0", "-0x2.0#1", Less);
    test("100", 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("100", 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("100", 1, Nearest, "-2.0", "-0x2.0#1", Less);
    test("100", 5, Nearest, "-1.69", "-0x1.b#5", Greater);
    test("100", 10, Down, "-1.7012", "-0x1.b38#10", Greater);
    test("100", 10, Up, "-1.7031", "-0x1.b40#10", Less);
    test("100", 10, Floor, "-1.7031", "-0x1.b40#10", Less);
    test("100", 10, Ceiling, "-1.7012", "-0x1.b38#10", Greater);
    test("100", 10, Nearest, "-1.7031", "-0x1.b40#10", Less);
    test("100", 20, Nearest, "-1.7029572", "-0x1.b3f50#20", Less);
    test(
        "100",
        53,
        Down,
        "-1.7029569194264691",
        "-0x1.b3f4fc136effe#53",
        Greater,
    );
    test(
        "100",
        53,
        Up,
        "-1.7029569194264693",
        "-0x1.b3f4fc136efff#53",
        Less,
    );
    test(
        "100",
        53,
        Floor,
        "-1.7029569194264693",
        "-0x1.b3f4fc136efff#53",
        Less,
    );
    test(
        "100",
        53,
        Ceiling,
        "-1.7029569194264691",
        "-0x1.b3f4fc136effe#53",
        Greater,
    );
    test(
        "100",
        53,
        Nearest,
        "-1.7029569194264693",
        "-0x1.b3f4fc136efff#53",
        Less,
    );
    test(
        "100",
        100,
        Nearest,
        "-1.7029569194264692160987314595571",
        "-0x1.b3f4fc136effe8ad8b2d8c54a#100",
        Greater,
    );
    test("1000000", 1, Down, "-2.0", "-0x2.0#1", Greater);
    test("1000000", 1, Up, "-4.0", "-0x4.0#1", Less);
    test("1000000", 1, Floor, "-4.0", "-0x4.0#1", Less);
    test("1000000", 1, Ceiling, "-2.0", "-0x2.0#1", Greater);
    test("1000000", 1, Nearest, "-2.0", "-0x2.0#1", Greater);
    test("1000000", 5, Nearest, "-2.62", "-0x2.a#5", Greater);
    test("1000000", 10, Down, "-2.6758", "-0x2.ad#10", Greater);
    test("1000000", 10, Up, "-2.6797", "-0x2.ae#10", Less);
    test("1000000", 10, Floor, "-2.6797", "-0x2.ae#10", Less);
    test("1000000", 10, Ceiling, "-2.6758", "-0x2.ad#10", Greater);
    test("1000000", 10, Nearest, "-2.6758", "-0x2.ad#10", Greater);
    test(
        "1000000",
        20,
        Nearest,
        "-2.6764832",
        "-0x2.ad2e0#20",
        Greater,
    );
    test(
        "1000000",
        53,
        Down,
        "-2.6764843396283449",
        "-0x2.ad2e13e2f5bbe#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Up,
        "-2.6764843396283453",
        "-0x2.ad2e13e2f5bc0#53",
        Less,
    );
    test(
        "1000000",
        53,
        Floor,
        "-2.6764843396283453",
        "-0x2.ad2e13e2f5bc0#53",
        Less,
    );
    test(
        "1000000",
        53,
        Ceiling,
        "-2.6764843396283449",
        "-0x2.ad2e13e2f5bbe#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Nearest,
        "-2.6764843396283453",
        "-0x2.ad2e13e2f5bc0#53",
        Less,
    );
    test(
        "1000000",
        100,
        Nearest,
        "-2.6764843396283451088384140346689",
        "-0x2.ad2e13e2f5bbf17ba5420d414#100",
        Greater,
    );
    test("1/1000000", 1, Down, "5.2e5", "0x8.0E+4#1", Less);
    test("1/1000000", 1, Up, "1.0e6", "0x1.0E+5#1", Greater);
    test("1/1000000", 1, Floor, "5.2e5", "0x8.0E+4#1", Less);
    test("1/1000000", 1, Ceiling, "1.0e6", "0x1.0E+5#1", Greater);
    test("1/1000000", 1, Nearest, "1.0e6", "0x1.0E+5#1", Greater);
    test("1/1000000", 5, Nearest, "1.02e6", "0xf.8E+4#5", Greater);
    test("1/1000000", 10, Down, "9.9942e5", "0xf.40E+4#10", Less);
    test("1/1000000", 10, Up, "1.0004e6", "0xf.44E+4#10", Greater);
    test("1/1000000", 10, Floor, "9.9942e5", "0xf.40E+4#10", Less);
    test(
        "1/1000000",
        10,
        Ceiling,
        "1.0004e6",
        "0xf.44E+4#10",
        Greater,
    );
    test(
        "1/1000000",
        10,
        Nearest,
        "1.0004e6",
        "0xf.44E+4#10",
        Greater,
    );
    test(
        "1/1000000",
        20,
        Nearest,
        "1000000.0",
        "0xf4240.0#20",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Down,
        "999999.99999966659",
        "0xf423f.fffffa680#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Up,
        "999999.99999966670",
        "0xf423f.fffffa688#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Floor,
        "999999.99999966659",
        "0xf423f.fffffa680#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Ceiling,
        "999999.99999966670",
        "0xf423f.fffffa688#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Nearest,
        "999999.99999966670",
        "0xf423f.fffffa688#53",
        Greater,
    );
    test(
        "1/1000000",
        100,
        Nearest,
        "999999.99999966666666666664444410",
        "0xf423f.fffffa68581fc35b12a7#100",
        Less,
    );
    test("-1/1000000", 1, Down, "-5.2e5", "-0x8.0E+4#1", Greater);
    test("-1/1000000", 1, Up, "-1.0e6", "-0x1.0E+5#1", Less);
    test("-1/1000000", 1, Floor, "-1.0e6", "-0x1.0E+5#1", Less);
    test("-1/1000000", 1, Ceiling, "-5.2e5", "-0x8.0E+4#1", Greater);
    test("-1/1000000", 1, Nearest, "-1.0e6", "-0x1.0E+5#1", Less);
    test("-1/1000000", 5, Nearest, "-1.02e6", "-0xf.8E+4#5", Less);
    test(
        "-1/1000000",
        10,
        Down,
        "-9.9942e5",
        "-0xf.40E+4#10",
        Greater,
    );
    test("-1/1000000", 10, Up, "-1.0004e6", "-0xf.44E+4#10", Less);
    test("-1/1000000", 10, Floor, "-1.0004e6", "-0xf.44E+4#10", Less);
    test(
        "-1/1000000",
        10,
        Ceiling,
        "-9.9942e5",
        "-0xf.40E+4#10",
        Greater,
    );
    test(
        "-1/1000000",
        10,
        Nearest,
        "-1.0004e6",
        "-0xf.44E+4#10",
        Less,
    );
    test(
        "-1/1000000",
        20,
        Nearest,
        "-1000000.0",
        "-0xf4240.0#20",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Down,
        "-999999.99999966659",
        "-0xf423f.fffffa680#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Up,
        "-999999.99999966670",
        "-0xf423f.fffffa688#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Floor,
        "-999999.99999966670",
        "-0xf423f.fffffa688#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Ceiling,
        "-999999.99999966659",
        "-0xf423f.fffffa680#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Nearest,
        "-999999.99999966670",
        "-0xf423f.fffffa688#53",
        Less,
    );
    test(
        "-1/1000000",
        100,
        Nearest,
        "-999999.99999966666666666664444410",
        "-0xf423f.fffffa68581fc35b12a7#100",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Down,
        "6.0e23",
        "0x8.0E+19#1",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Up,
        "1.2e24",
        "0x1.0E+20#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Floor,
        "6.0e23",
        "0x8.0E+19#1",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Ceiling,
        "1.2e24",
        "0x1.0E+20#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Nearest,
        "1.2e24",
        "0x1.0E+20#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        5,
        Nearest,
        "9.82e23",
        "0xd.0E+19#5",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Down,
        "9.9996e23",
        "0xd.3cE+19#10",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Up,
        "1.0011e24",
        "0xd.40E+19#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Floor,
        "9.9996e23",
        "0xd.3cE+19#10",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Ceiling,
        "1.0011e24",
        "0xd.40E+19#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Nearest,
        "9.9996e23",
        "0xd.3cE+19#10",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        20,
        Nearest,
        "1.0000003e24",
        "0xd.3c22E+19#20",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Down,
        "9.9999999999999998e23",
        "0xd.3c21bcecceda0E+19#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Up,
        "1.0000000000000001e24",
        "0xd.3c21bcecceda8E+19#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Floor,
        "9.9999999999999998e23",
        "0xd.3c21bcecceda0E+19#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Ceiling,
        "1.0000000000000001e24",
        "0xd.3c21bcecceda8E+19#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Nearest,
        "9.9999999999999998e23",
        "0xd.3c21bcecceda0E+19#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        100,
        Nearest,
        "1000000000000000000000000.0000000",
        "0xd3c21bcecceda1000000.00000#100",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Down,
        "4.9e-32",
        "0x1.0E-26#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Up,
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Floor,
        "4.9e-32",
        "0x1.0E-26#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Ceiling,
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Nearest,
        "9.9e-32",
        "0x2.0E-26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        5,
        Nearest,
        "8.63e-32",
        "0x1.cE-26#5",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Down,
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "8.4837e-32",
        "0x1.b88E-26#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "8.4837e-32",
        "0x1.b88E-26#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "8.4741e-32",
        "0x1.b80E-26#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "8.4784270e-32",
        "0x1.b839aE-26#20",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "8.4784276603689007e-32",
        "0x1.b839a252049c2E-26#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "8.4784276603689007e-32",
        "0x1.b839a252049c2E-26#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "8.4784276603688996e-32",
        "0x1.b839a252049c1E-26#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "8.4784276603688996439587014693888e-32",
        "0x1.b839a252049c1114cf98e8042E-26#100",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Down,
        "2.0e-31",
        "0x4.0E-26#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Up,
        "3.9e-31",
        "0x8.0E-26#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Floor,
        "2.0e-31",
        "0x4.0E-26#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Ceiling,
        "3.9e-31",
        "0x8.0E-26#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Nearest,
        "2.0e-31",
        "0x4.0E-26#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        5,
        Nearest,
        "2.59e-31",
        "0x5.4E-26#5",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Down,
        "2.5422e-31",
        "0x5.28E-26#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Up,
        "2.5461e-31",
        "0x5.2aE-26#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Floor,
        "2.5422e-31",
        "0x5.28E-26#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Ceiling,
        "2.5461e-31",
        "0x5.2aE-26#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Nearest,
        "2.5422e-31",
        "0x5.28E-26#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        20,
        Nearest,
        "2.5435290e-31",
        "0x5.28ad0E-26#20",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Down,
        "2.5435282981106695e-31",
        "0x5.28ace6f60dd40E-26#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Up,
        "2.5435282981106700e-31",
        "0x5.28ace6f60dd44E-26#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Floor,
        "2.5435282981106695e-31",
        "0x5.28ace6f60dd40E-26#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Ceiling,
        "2.5435282981106700e-31",
        "0x5.28ace6f60dd44E-26#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Nearest,
        "2.5435282981106700e-31",
        "0x5.28ace6f60dd44E-26#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        100,
        Nearest,
        "2.5435282981106698931876104408174e-31",
        "0x5.28ace6f60dd4333e6ecab80c8E-26#100",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Down,
        "-5.1e30",
        "-0x4.0E+25#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Up,
        "-1.0e31",
        "-0x8.0E+25#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Floor,
        "-1.0e31",
        "-0x8.0E+25#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Ceiling,
        "-5.1e30",
        "-0x4.0E+25#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Nearest,
        "-5.1e30",
        "-0x4.0E+25#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        5,
        Nearest,
        "-6.02e30",
        "-0x4.cE+25#5",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Down,
        "-5.8926e30",
        "-0x4.a6E+25#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Up,
        "-5.9025e30",
        "-0x4.a8E+25#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Floor,
        "-5.9025e30",
        "-0x4.a8E+25#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Ceiling,
        "-5.8926e30",
        "-0x4.a6E+25#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Nearest,
        "-5.8926e30",
        "-0x4.a6E+25#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        20,
        Nearest,
        "-5.8973239e30",
        "-0x4.a6f48E+25#20",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Down,
        "-5.8973198808686271e30",
        "-0x4.a6f44abe5fc70E+25#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Up,
        "-5.8973198808686282e30",
        "-0x4.a6f44abe5fc74E+25#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Floor,
        "-5.8973198808686282e30",
        "-0x4.a6f44abe5fc74E+25#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Ceiling,
        "-5.8973198808686271e30",
        "-0x4.a6f44abe5fc70E+25#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Nearest,
        "-5.8973198808686282e30",
        "-0x4.a6f44abe5fc74E+25#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        100,
        Nearest,
        "-5897319880868628039416095147896.0",
        "-0x4a6f44abe5fc7366ee195b8778.0#100",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Down,
        "-4.9e-32",
        "-0x1.0E-26#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Up,
        "-9.9e-32",
        "-0x2.0E-26#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Floor,
        "-9.9e-32",
        "-0x2.0E-26#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Ceiling,
        "-4.9e-32",
        "-0x1.0E-26#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Nearest,
        "-9.9e-32",
        "-0x2.0E-26#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        5,
        Nearest,
        "-8.63e-32",
        "-0x1.cE-26#5",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Down,
        "-8.4741e-32",
        "-0x1.b80E-26#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "-8.4837e-32",
        "-0x1.b88E-26#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "-8.4837e-32",
        "-0x1.b88E-26#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "-8.4741e-32",
        "-0x1.b80E-26#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "-8.4741e-32",
        "-0x1.b80E-26#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "-8.4784270e-32",
        "-0x1.b839aE-26#20",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "-8.4784276603688996e-32",
        "-0x1.b839a252049c1E-26#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "-8.4784276603689007e-32",
        "-0x1.b839a252049c2E-26#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "-8.4784276603689007e-32",
        "-0x1.b839a252049c2E-26#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "-8.4784276603688996e-32",
        "-0x1.b839a252049c1E-26#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "-8.4784276603688996e-32",
        "-0x1.b839a252049c1E-26#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "-8.4784276603688996439587014693888e-32",
        "-0x1.b839a252049c1114cf98e8042E-26#100",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Down,
        "-0.50",
        "-0x0.8#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Up,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Ceiling,
        "-0.50",
        "-0x0.8#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Nearest,
        "-0.50",
        "-0x0.8#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        5,
        Nearest,
        "-0.562",
        "-0x0.90#5",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Down,
        "-0.56055",
        "-0x0.8f8#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Up,
        "-0.56152",
        "-0x0.8fc#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Floor,
        "-0.56152",
        "-0x0.8fc#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Ceiling,
        "-0.56055",
        "-0x0.8f8#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Nearest,
        "-0.56055",
        "-0x0.8f8#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        20,
        Nearest,
        "-0.56086636",
        "-0x0.8f94f#20",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Down,
        "-0.56086660415971612",
        "-0x0.8f94f42a48d8d0#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Up,
        "-0.56086660415971623",
        "-0x0.8f94f42a48d8d8#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Floor,
        "-0.56086660415971623",
        "-0x0.8f94f42a48d8d8#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Ceiling,
        "-0.56086660415971612",
        "-0x0.8f94f42a48d8d0#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Nearest,
        "-0.56086660415971623",
        "-0x0.8f94f42a48d8d8#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        100,
        Nearest,
        "-0.56086660415971621507154671918252",
        "-0x0.8f94f42a48d8d6d91333a3326#100",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Down,
        "6.3e29",
        "0x8.0E+24#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Up,
        "1.3e30",
        "0x1.0E+25#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Floor,
        "6.3e29",
        "0x8.0E+24#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Ceiling,
        "1.3e30",
        "0x1.0E+25#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Nearest,
        "1.3e30",
        "0x1.0E+25#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        5,
        Nearest,
        "1.27e30",
        "0x1.0E+25#5",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Down,
        "1.2664e30",
        "0xf.fcE+24#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Up,
        "1.2677e30",
        "0x1.000E+25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Floor,
        "1.2664e30",
        "0xf.fcE+24#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Ceiling,
        "1.2677e30",
        "0x1.000E+25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Nearest,
        "1.2677e30",
        "0x1.000E+25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        20,
        Nearest,
        "1.2676506e30",
        "0x1.00000E+25#20",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Down,
        "1.2676506002282293e30",
        "0xf.ffffffffffff8E+24#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Up,
        "1.2676506002282294e30",
        "0x1.0000000000000E+25#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Floor,
        "1.2676506002282293e30",
        "0xf.ffffffffffff8E+24#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Ceiling,
        "1.2676506002282294e30",
        "0x1.0000000000000E+25#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Nearest,
        "1.2676506002282294e30",
        "0x1.0000000000000E+25#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        100,
        Nearest,
        "1267650600228229401496703205376.0",
        "0x10000000000000000000000000.0#100",
        Greater,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn cot_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (s, o) = Float::cot_rational_prec_round(x.clone(), prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = Float::cot_rational_prec_round_ref(&x, prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    // cot is odd (a `Rational` has no negative zero, so x = 0 is excluded)
    if x != 0u32 {
        let (s_neg, o_neg) = Float::cot_rational_prec_round(-&x, prec, -rm);
        assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
        assert_eq!(o_neg, o.reverse());
    }

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_o) = rug_cot_rational_prec_round(&x, prec, rrm);
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
        // only cot(0) = infinity is exact
        assert_eq!(x, 0u32);
        assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&Float::INFINITY));
        for rm in exhaustive_rounding_modes() {
            let (s2, oo) = Float::cot_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::cot_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn cot_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        cot_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // cot(0) = infinity, exactly
        let (s, o) = Float::cot_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn cot_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (s, o) = Float::cot_rational_prec(x.clone(), prec);
    assert!(s.is_valid());

    let (s_alt, o_alt) = Float::cot_rational_prec_ref(&x, prec);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let (s_alt, o_alt) = Float::cot_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let (rug_s, rug_o) = rug_cot_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_s)),
        ComparableFloatRef(&s)
    );
    assert_eq!(rug_o, o, "x = {x} prec = {prec}");

    // the cotangent of an exactly representable rational is the Float cotangent
    if let Ok(f) = Float::try_from(&x) {
        let (s_alt, o_alt) = f.cot_prec(prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
    }
}

#[test]
fn cot_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        cot_rational_prec_properties_helper(x, prec);
    });
}

// Rows reuse the tangent's `with_period` inputs, which cover the exact cases (the poles at
// multiples of a half turn, the ±1 at odd quarter turns, and the ±2 at odd twelfths), the closed
// forms at eighths, sixths, and twentieths of a turn, the general Ziv loop at its first working
// precision and after a retry, and the tiny x/u whose cotangent overflows.
#[test]
fn test_cot_with_period_prec_round() {
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

        let (t, o) = x.clone().cot_with_period_prec_round(u, prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.cot_with_period_prec_round_ref(u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.cot_with_period_prec_round_assign(u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = x.cot_with_period_prec_ref(u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
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
    test(
        "0.0", "0x0.0", 4, 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        4,
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("1.0", "0x1.0#1", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 0, 10, Nearest, "NaN", "NaN", Equal);
    test(
        "0.0", "0x0.0", 360, 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0", "0x0.0", 360, 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "0.0", "0x0.0", 360, 10, Exact, "Infinity", "Infinity", Equal,
    );
    test("90.0", "0x5a.0#6", 360, 10, Nearest, "0.0", "0x0.0", Equal);
    test("90.0", "0x5a.0#6", 360, 10, Floor, "0.0", "0x0.0", Equal);
    test("90.0", "0x5a.0#6", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "180.0",
        "0xb4.0#6",
        360,
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "180.0",
        "0xb4.0#6",
        360,
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "180.0",
        "0xb4.0#6",
        360,
        10,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Nearest,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Floor,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Exact,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test(
        "360.0",
        "0x168.0#6",
        360,
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "360.0",
        "0x168.0#6",
        360,
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "360.0",
        "0x168.0#6",
        360,
        10,
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "60.0",
        "0x3c.0#4",
        360,
        10,
        Nearest,
        "0.57715",
        "0x0.93c#10",
        Less,
    );
    test(
        "60.0",
        "0x3c.0#4",
        360,
        10,
        Floor,
        "0.57715",
        "0x0.93c#10",
        Less,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Nearest,
        "-0.57715",
        "-0x0.93c#10",
        Greater,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Floor,
        "-0.57812",
        "-0x0.940#10",
        Less,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Nearest,
        "0.57715",
        "0x0.93c#10",
        Less,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Floor,
        "0.57715",
        "0x0.93c#10",
        Less,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Nearest,
        "-0.57715",
        "-0x0.93c#10",
        Greater,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Floor,
        "-0.57812",
        "-0x0.940#10",
        Less,
    );
    test(
        "450.0",
        "0x1c2.0#8",
        360,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test("450.0", "0x1c2.0#8", 360, 10, Floor, "0.0", "0x0.0", Equal);
    test("450.0", "0x1c2.0#8", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "1.0", "0x1.0#1", 1, 10, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        3,
        10,
        Nearest,
        "-0.57715",
        "-0x0.93c#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 4, 10, Ceiling, "0.0", "0x0.0", Equal);
    test(
        "1.0",
        "0x1.0#1",
        7,
        10,
        Nearest,
        "0.79785",
        "0x0.cc4#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        360,
        10,
        Nearest,
        "57.312",
        "0x39.5#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        1000000,
        10,
        Ceiling,
        "1.5923e5",
        "0x2.6eE+4#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        1,
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("0.50", "0x0.8#1", 2, 53, Nearest, "0.0", "0x0.0", Equal);
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
        "1.7320508075688772",
        "0x1.bb67ae8584caa#53",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        12,
        10,
        Ceiling,
        "3.7344",
        "0x3.bc#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        1000000,
        10,
        Floor,
        "3.1795e5",
        "0x4.daE+4#10",
        Less,
    );
    test("0.25", "0x0.4#1", 1, 10, Nearest, "0.0", "0x0.0", Equal);
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
        "2.4141",
        "0x2.6a#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        6,
        10,
        Ceiling,
        "3.7344",
        "0x3.bc#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        12,
        10,
        Floor,
        "7.5938",
        "0x7.98#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        1000000,
        10,
        Nearest,
        "6.3693e5",
        "0x9.b8E+4#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        1099511627776,
        53,
        Nearest,
        "699970842190.26599",
        "0xa2f9836e4e.4418#53",
        Greater,
    );
    test("1.5", "0x1.8#2", 2, 10, Floor, "-0.0", "-0x0.0", Equal);
    test(
        "1.5",
        "0x1.8#2",
        3,
        53,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("1.5", "0x1.8#2", 6, 10, Floor, "0.0", "0x0.0", Equal);
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
        "38.188459297025609",
        "0x26.303ede555a26#53",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        1099511627776,
        10,
        Ceiling,
        "1.1677e11",
        "0x1.b30E+9#10",
        Greater,
    );
    test(
        "2.0", "0x2.0#1", 2, 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        3,
        10,
        Ceiling,
        "0.57812",
        "0x0.940#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        6,
        10,
        Nearest,
        "-0.57715",
        "-0x0.93c#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        7,
        10,
        Ceiling,
        "-0.22803",
        "-0x0.3a6#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Ceiling,
        "28.656",
        "0x1c.a8#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        1099511627776,
        10,
        Floor,
        "8.7376e10",
        "0x1.458E+9#10",
        Less,
    );
    test(
        "3.0", "0x3.0#2", 1, 53, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "3.0", "0x3.0#2", 3, 10, Floor, "Infinity", "Infinity", Equal,
    );
    test("3.0", "0x3.0#2", 4, 53, Nearest, "-0.0", "-0x0.0", Equal);
    test(
        "3.0",
        "0x3.0#2",
        7,
        10,
        Floor,
        "-2.0781",
        "-0x2.14#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        360,
        10,
        Floor,
        "19.062",
        "0x13.10#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        1099511627776,
        10,
        Nearest,
        "5.8318e10",
        "0xd.94E+8#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1,
        10,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        3,
        10,
        Nearest,
        "0.57715",
        "0x0.93c#10",
        Less,
    );
    test("-1.0", "-0x1.0#1", 4, 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        7,
        10,
        Nearest,
        "-0.79785",
        "-0x0.cc4#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        360,
        10,
        Nearest,
        "-57.312",
        "-0x39.5#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1000000,
        10,
        Ceiling,
        "-1.5898e5",
        "-0x2.6dE+4#10",
        Greater,
    );
    test(
        "100.0", "0x64.0#5", 1, 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "100.0", "0x64.0#5", 2, 53, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "100.0", "0x64.0#5", 4, 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "100.0",
        "0x64.0#5",
        6,
        53,
        Nearest,
        "0.57735026918962573",
        "0x0.93cd3a2c8198e0#53",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        12,
        10,
        Ceiling,
        "-0.57715",
        "-0x0.93c#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        1000000,
        10,
        Floor,
        "1590.0",
        "0x636.0#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        10,
        Nearest,
        "-3.5234",
        "-0x3.86#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        2,
        10,
        Ceiling,
        "0.13916",
        "0x0.23a#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        4,
        10,
        Nearest,
        "-0.87012",
        "-0x0.dec#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        6,
        10,
        Ceiling,
        "1.9336",
        "0x1.ef0#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        12,
        10,
        Floor,
        "-0.24341",
        "-0x0.3e5#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1000000,
        10,
        Nearest,
        "1290.0",
        "0x50a.0#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1099511627776,
        53,
        Nearest,
        "1417450027.1154621",
        "0x547c922b.1d8eec#53",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        2,
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        3,
        53,
        Nearest,
        "-0.57735026918962573",
        "-0x0.93cd3a2c8198e0#53",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        6,
        10,
        Floor,
        "0.57715",
        "0x0.93c#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        12,
        10,
        Nearest,
        "-0.57715",
        "-0x0.93c#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        360,
        53,
        Nearest,
        "-0.17632698070846498",
        "-0x0.2d23c3d78b9778#53",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1099511627776,
        10,
        Ceiling,
        "17.500",
        "0x11.80#10",
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
    test("-0.75", "-0x0.c#2", 3, 10, Ceiling, "-0.0", "-0x0.0", Equal);
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
        "-1.2539",
        "-0x1.410#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        360,
        10,
        Ceiling,
        "-76.375",
        "-0x4c.6#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1099511627776,
        10,
        Floor,
        "-2.3354e11",
        "-0x3.66E+9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1,
        53,
        Nearest,
        "1591549430.9189532",
        "0x5edd1df6.eb4084#53",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        3,
        10,
        Floor,
        "4.7731e9",
        "0x1.1c8E+8#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        4,
        53,
        Nearest,
        "6366197723.6758137",
        "0x17b7477db.ad022#53",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        7,
        10,
        Floor,
        "1.1140e10",
        "0x2.98E+8#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        360,
        10,
        Floor,
        "5.7230e11",
        "0x8.54E+9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1099511627776,
        10,
        Nearest,
        "1.7501e21",
        "0x5.eeE+17#10",
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
        "3.7305",
        "0x3.bb#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        4,
        10,
        Ceiling,
        "5.0312",
        "0x5.08#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        7,
        10,
        Nearest,
        "8.8750",
        "0x8.e0#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        360,
        10,
        Nearest,
        "458.50",
        "0x1ca.8#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        1000000,
        10,
        Ceiling,
        "1.2739e6",
        "0x1.370E+5#10",
        Greater,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        1,
        53,
        Floor,
        "1.5915494309189531e29",
        "0x2.0241e2211a1bcE+24#53",
        Less,
    );
    test(
        "-90.000000000000000000000000000808",
        "-0x5a.000000000000000000000040#100",
        360,
        53,
        Nearest,
        "1.4098657419642452e-29",
        "0x1.1df46a2529d39E-24#53",
        Less,
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
    test("1.0", "0x1.0#1", 12, 1, Nearest, "2.0", "0x2.0#1", Greater);
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Up,
        "1.7324",
        "0x1.bb8#10",
        Greater,
    );
    test(
        "5.0",
        "0x5.0#3",
        12,
        10,
        Ceiling,
        "-1.7305",
        "-0x1.bb0#10",
        Greater,
    );
    test(
        "-7.0",
        "-0x7.0#3",
        12,
        10,
        Nearest,
        "-1.7324",
        "-0x1.bb8#10",
        Less,
    );
    test("11.0", "0xb.0#4", 12, 1, Nearest, "-2.0", "-0x2.0#1", Less);
    test(
        "11.0",
        "0xb.0#4",
        12,
        10,
        Up,
        "-1.7324",
        "-0x1.bb8#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        10,
        Ceiling,
        "0.32520",
        "0x0.534#10",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        5,
        10,
        Nearest,
        "1.3770",
        "0x1.608#10",
        Greater,
    );
    test("3.0", "0x3.0#2", 5, 1, Nearest, "1.0", "0x1.0#1", Less);
    test("3.0", "0x3.0#2", 5, 10, Up, "1.3770", "0x1.608#10", Greater);
    test(
        "4.0",
        "0x4.0#1",
        5,
        10,
        Ceiling,
        "-0.32471",
        "-0x0.532#10",
        Greater,
    );
    test(
        "-6.0",
        "-0x6.0#2",
        5,
        10,
        Nearest,
        "-0.32471",
        "-0x0.532#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 10, 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Up,
        "1.3770",
        "0x1.608#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        10,
        Ceiling,
        "-0.32471",
        "-0x0.532#10",
        Greater,
    );
    test(
        "-7.0",
        "-0x7.0#3",
        10,
        10,
        Nearest,
        "-0.32471",
        "-0x0.532#10",
        Greater,
    );
    test(
        "9.00", "0x9.0#4", 10, 1, Nearest, "-1.0", "-0x1.0#1", Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        10,
        Up,
        "-1.3770",
        "-0x1.608#10",
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
    test(
        "30.0", "0x1e.0#4", 360, 1, Nearest, "2.0", "0x2.0#1", Greater,
    );
    test(
        "30.0",
        "0x1e.0#4",
        360,
        10,
        Up,
        "1.7324",
        "0x1.bb8#10",
        Greater,
    );
    test(
        "150.0",
        "0x96.0#7",
        360,
        10,
        Ceiling,
        "-1.7305",
        "-0x1.bb0#10",
        Greater,
    );
    test(
        "-72.0",
        "-0x48.0#4",
        360,
        10,
        Nearest,
        "-0.32471",
        "-0x0.532#10",
        Greater,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        10,
        Up,
        "-1.3770",
        "-0x1.608#10",
        Less,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        10,
        Ceiling,
        "1.3770",
        "0x1.608#10",
        Greater,
    );
    test(
        "-108.0",
        "-0x6c.0#5",
        360,
        10,
        Nearest,
        "0.32471",
        "0x0.532#10",
        Less,
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
        "-0.32471",
        "-0x0.532#10",
        Greater,
    );
    test("6.0", "0x6.0#2", 60, 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "6.0",
        "0x6.0#2",
        60,
        10,
        Up,
        "1.3770",
        "0x1.608#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        20,
        30,
        Ceiling,
        "0.72654252872",
        "0x0.b9feb0f0#30",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        20,
        30,
        Ceiling,
        "-3.0776835345",
        "-0x3.13e3117#30",
        Greater,
    );
    test(
        "13.0",
        "0xd.0#4",
        20,
        30,
        Ceiling,
        "0.72654252872",
        "0x0.b9feb0f0#30",
        Greater,
    );
    test(
        "19.0",
        "0x13.0#5",
        20,
        30,
        Ceiling,
        "-3.0776835345",
        "-0x3.13e3117#30",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        6,
        30,
        Floor,
        "0.57735026907",
        "0x0.93cd3a2c#30",
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
    test(
        "2.0",
        "0x2.0#2",
        4,
        10,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        100,
        10,
        Floor,
        "-Infinity",
        "-Infinity",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        100,
        10,
        Down,
        "-2.0965e323228496",
        "-0x7.feE+268435455#10",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        4,
        1,
        Up,
        "Infinity",
        "Infinity",
        Greater,
    );
    // fifths and tenths of a turn, where the cosine is a multiple of the golden ratio
    test(
        "1.0",
        "0x1.0#1",
        5,
        10,
        Nearest,
        "0.32471",
        "0x0.532#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        20,
        Nearest,
        "0.32491970",
        "0x0.532df0#20",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        20,
        Floor,
        "0.32491922",
        "0x0.532de8#20",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        20,
        Ceiling,
        "0.32491970",
        "0x0.532df0#20",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        5,
        20,
        Nearest,
        "-1.3763828",
        "-0x1.605aa#20",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        5,
        20,
        Nearest,
        "1.3763828",
        "0x1.605aa#20",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        5,
        20,
        Nearest,
        "-0.32491970",
        "-0x0.532df0#20",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        5,
        20,
        Nearest,
        "-0.32491970",
        "-0x0.532df0#20",
        Less,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        5,
        20,
        Nearest,
        "1.3763828",
        "0x1.605aa#20",
        Greater,
    );
    test(
        "6.0",
        "0x6.0#2",
        5,
        20,
        Nearest,
        "0.32491970",
        "0x0.532df0#20",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Nearest,
        "1.3770",
        "0x1.608#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        20,
        Nearest,
        "1.3763828",
        "0x1.605aa#20",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        20,
        Floor,
        "1.3763809",
        "0x1.605a8#20",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        20,
        Ceiling,
        "1.3763828",
        "0x1.605aa#20",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        20,
        Nearest,
        "-0.32491970",
        "-0x0.532df0#20",
        Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        10,
        20,
        Nearest,
        "0.32491970",
        "0x0.532df0#20",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        20,
        Nearest,
        "-1.3763828",
        "-0x1.605aa#20",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        20,
        Nearest,
        "-1.3763828",
        "-0x1.605aa#20",
        Less,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        10,
        20,
        Nearest,
        "0.32491970",
        "0x0.532df0#20",
        Greater,
    );
    test(
        "11.0",
        "0xb.0#4",
        10,
        20,
        Nearest,
        "1.3763828",
        "0x1.605aa#20",
        Greater,
    );
    test(
        "72.0",
        "0x48.0#4",
        360,
        20,
        Nearest,
        "0.32491970",
        "0x0.532df0#20",
        Greater,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        20,
        Nearest,
        "1.3763828",
        "0x1.605aa#20",
        Greater,
    );
    test(
        "108.0",
        "0x6c.0#5",
        360,
        20,
        Nearest,
        "-0.32491970",
        "-0x0.532df0#20",
        Less,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        20,
        Nearest,
        "-1.3763828",
        "-0x1.605aa#20",
        Less,
    );
}

#[test]
#[should_panic]
fn cot_with_period_prec_round_fail_1() {
    Float::ONE.cot_with_period_prec_round(4, 0, Floor);
}

#[test]
#[should_panic]
fn cot_with_period_prec_round_fail_2() {
    Float::from_unsigned_prec(1u32, 10)
        .0
        .cot_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn cot_with_period_prec_round_ref_fail() {
    Float::ONE.cot_with_period_prec_round_ref(4, 0, Floor);
}

#[test]
#[should_panic]
fn cot_with_period_prec_fail() {
    Float::ONE.cot_with_period_prec(4, 0);
}

#[test]
#[should_panic]
fn cot_with_period_round_fail() {
    Float::from_unsigned_prec(1u32, 10)
        .0
        .cot_with_period_round(7, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn cot_with_period_prec_round_properties_helper(x: Float, u: u64, prec: u64, rm: RoundingMode) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (t, o) = x.cot_with_period_prec_round_ref(u, prec, Nearest);
        if o == Equal {
            let (te, oe) = x.cot_with_period_prec_round_ref(u, prec, Exact);
            assert_eq!(ComparableFloatRef(&te), ComparableFloatRef(&t));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(x.cot_with_period_prec_round_ref(u, prec, Exact));
        }
        return;
    }
    let (t, o) = x.clone().cot_with_period_prec_round(u, prec, rm);
    assert!(t.is_valid());
    assert_rounding_ordering_consistent(&t, rm, o);

    let (t_alt, o_alt) = x.cot_with_period_prec_round_ref(u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    let mut t_alt = x.clone();
    let o_alt = t_alt.cot_with_period_prec_round_assign(u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    // cot_with_period is NaN exactly for u = 0 and non-finite x
    assert_eq!(t.is_nan(), u == 0 || !x.is_finite());
    if !t.is_nan() {
        // an infinity is either a pole, which is exact, or an overflow, which is not; a finite
        // result carries the requested precision
        if t.is_normal() {
            assert_eq!(t.get_prec(), Some(prec));
        }
        // cot_with_period is odd
        let (t_neg, o_neg) = (-&x).cot_with_period_prec_round(u, prec, -rm);
        assert_eq!(ComparableFloatRef(&t_neg), ComparableFloatRef(&-&t));
        assert_eq!(o_neg, o.reverse());
        // cot_with_period has period u, except at a pole, where the infinity's sign follows the
        // sign of x rather than the angle
        if x.is_finite() && u != 0 && !t.is_infinite() {
            let (shifted, os) =
                x.add_prec_round_ref_val(Float::from(u), x.significant_bits() + 64, Nearest);
            if os == Equal {
                let (t_shifted, o_shifted) = shifted.cot_with_period_prec_round(u, prec, rm);
                assert_eq!(ComparableFloatRef(&t_shifted), ComparableFloatRef(&t));
                assert_eq!(o_shifted, o);
            }
        }
        // the cotangent is the cosine over the sine in the same units
        if x.is_finite()
            && u != 0
            && let Some((t_alt, o_alt)) = cot_with_period_naive(&x, u, prec, rm)
        {
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (t2, oo) = x.cot_with_period_prec_round_ref(u, prec, rm);
            assert_eq!(ComparableFloatRef(&t2), ComparableFloatRef(&t));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.cot_with_period_prec_round_ref(u, prec, Exact));
    }
}

#[test]
fn cot_with_period_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, u, prec, rm)| {
            cot_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_18().test_properties(
        |(x, u, prec, rm)| {
            cot_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // cot_with_period(±0) = ±infinity and cot_with_period(x, 0) = NaN, exactly
        let (t, o) = Float::ZERO.cot_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);
        let (t, o) = Float::NEGATIVE_ZERO.cot_with_period_prec_round(4, prec, rm);
        assert_eq!(
            ComparableFloat(t),
            ComparableFloat(Float::NEGATIVE_INFINITY)
        );
        assert_eq!(o, Equal);
        let (t, o) = Float::ONE.cot_with_period_prec_round(0, prec, rm);
        assert!(t.is_nan());
        assert_eq!(o, Equal);
        // exact cases: even quarter turns are the cotangent's poles and odd ones its zeros
        for (k, expected) in [
            (0u32, Float::INFINITY),
            (1, Float::ZERO),
            (2, Float::NEGATIVE_INFINITY),
            (3, Float::NEGATIVE_ZERO),
            (4, Float::INFINITY),
        ] {
            let (t, o) = Float::from(k).cot_with_period_prec_round(4, prec, rm);
            assert_eq!(ComparableFloat(t), ComparableFloat(expected));
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn cot_with_period_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, u, prec)| {
        let (t, o) = x.clone().cot_with_period_prec(u, prec);
        assert!(t.is_valid());
        let (t_alt, o_alt) = x.cot_with_period_prec_ref(u, prec);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.cot_with_period_prec_round_ref(u, prec, Nearest);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.cot_with_period_prec_assign(u, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn cot_with_period_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_39().test_properties(|(x, u, rm)| {
        if rm == Exact && x.cot_with_period_round_ref(u, Nearest).1 != Equal {
            assert_panic!(x.cot_with_period_round_ref(u, Exact));
            return;
        }
        let (t, o) = x.clone().cot_with_period_round(u, rm);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = x.cot_with_period_round_ref(u, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.cot_with_period_prec_round_ref(u, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.cot_with_period_round_assign(u, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn cot_with_period_properties() {
    float_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, u)| {
        let t = x.clone().cot_with_period(u);
        assert!(t.is_valid());
        let t_alt = x.cot_with_period_ref(u);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let mut t_alt = x.clone();
        t_alt.cot_with_period_assign(u);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let (t_alt, _) = x.cot_with_period_prec_round_ref(u, x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        // cot_with_period is odd
        assert_eq!(
            ComparableFloatRef(&(-&x).cot_with_period_ref(u)),
            ComparableFloatRef(&-&t)
        );
    });
}

// Inputs within 2^(-2^30) of a half turn, whose cotangents overflow. The near-zero path works with
// the exact distance to the multiple of 1/2, so no 2^30-bit pi is ever formed, but the inputs
// themselves have 2^30 bits.

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_cot_with_period() {
    fn test<T: PrimitiveFloat>(x: T, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_cot_with_period(x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 360, f32::NAN);
    test::<f32>(f32::INFINITY, 360, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, 360, f32::NAN);
    test::<f32>(1.0, 0, f32::NAN);
    test::<f32>(0.0, 360, f32::INFINITY);
    test::<f32>(-0.0, 360, f32::NEGATIVE_INFINITY);
    test::<f32>(90.0, 360, 0.0);
    test::<f32>(-90.0, 360, -0.0);
    test::<f32>(270.0, 360, -0.0);
    test::<f32>(180.0, 360, f32::NEGATIVE_INFINITY);
    test::<f32>(-180.0, 360, f32::INFINITY);
    test::<f32>(360.0, 360, f32::INFINITY);
    test::<f32>(45.0, 360, 1.0);
    test::<f32>(135.0, 360, -1.0);
    test::<f32>(30.0, 360, 1.7320508);
    test::<f32>(60.0, 360, 0.57735026);
    test::<f32>(120.0, 360, -0.57735026);
    test::<f32>(1.0, 7, 0.7974734);
    test::<f32>(-1.0, 7, -0.7974734);
    test::<f32>(2.0, 7, -0.22824347);
    test::<f32>(1.0, 360, 57.289963);
    test::<f32>(100.0, 360, -0.17632698);
    test::<f32>(10000000000.0, 360, -0.17632698);
    test::<f32>(1.0e30, 7, 0.7974734);
    test::<f32>(1.0e-30, 7, 1.1140846e30);
    test::<f32>(3.4028235e38, 360, f32::INFINITY);
    test::<f32>(0.5, 1, f32::NEGATIVE_INFINITY);
    test::<f32>(0.25, 1, 0.0);
    test::<f32>(0.1, 1, 1.3763819);
    test::<f32>(1.0e-45, 1, f32::INFINITY);
    test::<f32>(1.0e-45, 360, f32::INFINITY);
    test::<f64>(f64::NAN, 360, f64::NAN);
    test::<f64>(f64::INFINITY, 360, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, 360, f64::NAN);
    test::<f64>(1.0, 0, f64::NAN);
    test::<f64>(0.0, 360, f64::INFINITY);
    test::<f64>(-0.0, 360, f64::NEGATIVE_INFINITY);
    test::<f64>(90.0, 360, 0.0);
    test::<f64>(-90.0, 360, -0.0);
    test::<f64>(270.0, 360, -0.0);
    test::<f64>(180.0, 360, f64::NEGATIVE_INFINITY);
    test::<f64>(-180.0, 360, f64::INFINITY);
    test::<f64>(360.0, 360, f64::INFINITY);
    test::<f64>(45.0, 360, 1.0);
    test::<f64>(135.0, 360, -1.0);
    test::<f64>(30.0, 360, 1.7320508075688772);
    test::<f64>(60.0, 360, 0.5773502691896257);
    test::<f64>(120.0, 360, -0.5773502691896257);
    test::<f64>(1.0, 7, 0.7974733888824039);
    test::<f64>(-1.0, 7, -0.7974733888824039);
    test::<f64>(2.0, 7, -0.22824347439014994);
    test::<f64>(1.0, 360, 57.28996163075942);
    test::<f64>(100.0, 360, -0.17632698070846498);
    test::<f64>(10000000000.0, 360, -0.17632698070846498);
    test::<f64>(1.0e100, 7, -0.22824347439014994);
    test::<f64>(1.0e-100, 7, 1.1140846016432672e100);
    test::<f64>(1.7976931348623157e308, 360, -0.7812856265067174);
    test::<f64>(0.5, 1, f64::NEGATIVE_INFINITY);
    test::<f64>(0.25, 1, 0.0);
    test::<f64>(0.1, 1, 1.3763819204711734);
    test::<f64>(5.0e-324, 1, f64::INFINITY);
    test::<f64>(5.0e-324, 360, f64::INFINITY);
    test::<f32>(72.0, 360, 0.3249197);
    test::<f32>(36.0, 360, 1.3763819);
    test::<f32>(108.0, 360, -0.3249197);
    test::<f32>(144.0, 360, -1.3763819);
    test::<f64>(72.0, 360, 0.32491969623290634);
    test::<f64>(36.0, 360, 1.3763819204711736);
    test::<f64>(108.0, 360, -0.32491969623290634);
    test::<f64>(144.0, 360, -1.3763819204711736);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_cot_with_period_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        let s = primitive_float_cot_with_period(x, u);
        // NaN exactly for u = 0 (the inputs are finite)
        assert_eq!(s.is_nan(), u == 0);
        if u != 0 {
            // odd
            assert_eq!(
                NiceFloat(primitive_float_cot_with_period(-x, u)),
                NiceFloat(-s)
            );
            // the same as the `Float` cotangent taken with 64 bits to spare and rounded once
            let (s_float, _) =
                Float::cot_with_period_prec(Float::from(x), u, T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&s_float, Nearest).0),
                NiceFloat(s)
            );
            // a pole is an infinity in both, but the cotangent of a tiny angle can exceed the
            // largest finite `T` while the `Float` cotangent, with its far wider exponent range,
            // stays finite
            if s_float.is_infinite() {
                assert!(s.is_infinite());
            }
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // NaN exactly for NaN and infinite inputs
        assert_eq!(
            primitive_float_cot_with_period(x, 7).is_nan(),
            !x.is_finite()
        );
    });
}

#[test]
fn primitive_float_cot_with_period_properties() {
    apply_fn_to_primitive_floats!(primitive_float_cot_with_period_properties_helper);
}

// Inputs within 2^(-2^30) of a half turn, whose cotangents overflow, and within the same of a
// quarter turn, where they underflow instead. The near-zero paths work with the exact distance to
// the multiple of 1/4, so no 2^30-bit pi is ever formed, but the inputs themselves have 2^30 bits.
#[test]
fn test_cot_with_period_underflow_and_overflow() {
    let max = Float::max_finite_value_with_prec(10);
    let min = Float::min_positive_value_prec(10);
    let eps = Rational::power_of_2(-((1i64 << 30) + 70));
    let p = (1u64 << 30) + 74;
    // just past a half turn: x/u = 1/2 + 2^(-2^30 - 72), so the sine is negative and tiny and the
    // cosine is just below -1, and the cotangent is positive and beyond the largest finite `Float`
    let above = Float::from_rational_prec_round(Rational::from(2u32) + &eps, p, Exact).0;
    let (t, o) = above.cot_with_period_prec_round_ref(4, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (t, o) = above.cot_with_period_prec_round_ref(4, 10, Down);
    assert_eq!(ComparableFloatRef(&t), ComparableFloatRef(&max));
    assert_eq!(o, Less);
    // just below a half turn: the sine is positive and tiny, so the cotangent is negative and huge
    let below = Float::from_rational_prec_round(Rational::from(2u32) - &eps, p, Exact).0;
    let (t, o) = below.cot_with_period_prec_round_ref(4, 10, Nearest);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    let (t, o) = below.cot_with_period_prec_round_ref(4, 10, Down);
    assert_eq!(ComparableFloatRef(&t), ComparableFloatRef(&-max.clone()));
    assert_eq!(o, Greater);
    // just past a quarter turn: the cosine is negative and tiny while the sine is just below 1, so
    // the cotangent is negative and below the smallest positive `Float`
    let above = Float::from_rational_prec_round(Rational::ONE + &eps, p, Exact).0;
    let (t, o) = above.cot_with_period_prec_round_ref(4, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    let (t, o) = above.cot_with_period_prec_round_ref(4, 10, Up);
    assert_eq!(ComparableFloatRef(&t), ComparableFloatRef(&-min.clone()));
    assert_eq!(o, Less);
    // just below a quarter turn: the cosine is positive and tiny, so the cotangent is a positive
    // underflow
    let below = Float::from_rational_prec_round(Rational::ONE - eps, p, Exact).0;
    let (t, o) = below.cot_with_period_prec_round_ref(4, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    let (t, o) = below.cot_with_period_prec_round_ref(4, 10, Up);
    assert_eq!(ComparableFloatRef(&t), ComparableFloatRef(&min));
    assert_eq!(o, Greater);
}

#[test]
fn test_cot_with_period_rational_prec_round() {
    let test = |s: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (t, o) = Float::cot_with_period_rational_prec_round(x.clone(), u, prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = Float::cot_with_period_rational_prec_round_ref(&x, u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = Float::cot_with_period_rational_prec(x.clone(), u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) = Float::cot_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        // the cotangent is the cosine over the sine in the same units
        if let Some((t_alt, o_alt)) = cot_with_period_rational_naive(&x, u, prec, rm) {
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    };
    test("0", 4, 1, Nearest, "Infinity", "Infinity", Equal);
    test("0", 4, 10, Nearest, "Infinity", "Infinity", Equal);
    test("0", 4, 10, Floor, "Infinity", "Infinity", Equal);
    test("0", 4, 10, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 4, 10, Exact, "Infinity", "Infinity", Equal);
    test("0", 4, 53, Nearest, "Infinity", "Infinity", Equal);
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
    test("90", 360, 1, Nearest, "0.0", "0x0.0", Equal);
    test("90", 360, 10, Nearest, "0.0", "0x0.0", Equal);
    test("90", 360, 10, Floor, "0.0", "0x0.0", Equal);
    test("90", 360, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("90", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test("90", 360, 53, Nearest, "0.0", "0x0.0", Equal);
    test("180", 360, 1, Nearest, "-Infinity", "-Infinity", Equal);
    test("180", 360, 10, Nearest, "-Infinity", "-Infinity", Equal);
    test("180", 360, 10, Floor, "-Infinity", "-Infinity", Equal);
    test("180", 360, 10, Ceiling, "-Infinity", "-Infinity", Equal);
    test("180", 360, 10, Exact, "-Infinity", "-Infinity", Equal);
    test("180", 360, 53, Nearest, "-Infinity", "-Infinity", Equal);
    test("270", 360, 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("270", 360, 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("270", 360, 10, Floor, "-0.0", "-0x0.0", Equal);
    test("270", 360, 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test("270", 360, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("270", 360, 53, Nearest, "-0.0", "-0x0.0", Equal);
    test("360", 360, 1, Nearest, "Infinity", "Infinity", Equal);
    test("360", 360, 10, Nearest, "Infinity", "Infinity", Equal);
    test("360", 360, 10, Floor, "Infinity", "Infinity", Equal);
    test("360", 360, 10, Ceiling, "Infinity", "Infinity", Equal);
    test("360", 360, 10, Exact, "Infinity", "Infinity", Equal);
    test("450", 360, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("60", 360, 10, Floor, "0.57715", "0x0.93c#10", Less);
    test("120", 360, 10, Floor, "-0.57812", "-0x0.940#10", Less);
    test("240", 360, 10, Floor, "0.57715", "0x0.93c#10", Less);
    test("300", 360, 10, Floor, "-0.57812", "-0x0.940#10", Less);
    test("45", 360, 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("135", 360, 10, Floor, "-1.0000", "-0x1.000#10", Equal);
    test("30", 360, 10, Floor, "1.7305", "0x1.bb0#10", Less);
    test("150", 360, 10, Floor, "-1.7324", "-0x1.bb8#10", Less);
    test("72", 360, 10, Floor, "0.32471", "0x0.532#10", Less);
    test("144", 360, 10, Floor, "-1.3770", "-0x1.608#10", Less);
    test("36", 360, 10, Floor, "1.3750", "0x1.600#10", Less);
    test("108", 360, 10, Floor, "-0.32520", "-0x0.534#10", Less);
    test("1/3", 1, 10, Floor, "-0.57812", "-0x0.940#10", Less);
    test("1/8", 1, 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("1/5", 1, 10, Floor, "0.32471", "0x0.532#10", Less);
    test("-1/10", 1, 10, Floor, "-1.3770", "-0x1.608#10", Less);
    test("1/7", 1, 10, Floor, "0.79688", "0x0.cc0#10", Less);
    test("2/7", 1, 10, Floor, "-0.22827", "-0x0.3a7#10", Less);
    test("-3/7", 1, 10, Floor, "2.0742", "0x2.13#10", Less);
    test("22/7", 1, 10, Floor, "0.79688", "0x0.cc0#10", Less);
    test("1/7", 3, 10, Floor, "3.2383", "0x3.3d#10", Less);
    test("355/113", 360, 10, Floor, "18.219", "0x12.38#10", Less);
    test("1", 7, 10, Floor, "0.79688", "0x0.cc0#10", Less);
    test("1000000", 7, 10, Floor, "0.79688", "0x0.cc0#10", Less);
    test("1/1000000", 1, 10, Floor, "1.5898e5", "0x2.6dE+4#10", Less);
    test(
        "1/1000000000000000000000000000000",
        1,
        10,
        Floor,
        "1.5908e29",
        "0x2.02E+24#10",
        Less,
    );
    test(
        "100000000000000000000000000000000000000001",
        4,
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "111414603535684224740921180161/1237940039285380274899124224",
        360,
        53,
        Floor,
        "-1.4098657419642455e-29",
        "-0x1.1df46a2529d3aE-24#53",
        Less,
    );
    test(
        "3/20",
        1,
        30,
        Nearest,
        "0.72654252779",
        "0x0.b9feb0ec#30",
        Less,
    );
    test(
        "11/20",
        1,
        30,
        Floor,
        "3.0776835345",
        "0x3.13e3117#30",
        Less,
    );
    test(
        "17/20",
        1,
        30,
        Nearest,
        "-0.72654252779",
        "-0x0.b9feb0ec#30",
        Greater,
    );
    test(
        "-1/3",
        1,
        30,
        Nearest,
        "0.57735026907",
        "0x0.93cd3a2c#30",
        Less,
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
    test("1/2", 1, 10, Exact, "-Infinity", "-Infinity", Equal);
    test("-5", 1, 10, Exact, "-Infinity", "-Infinity", Equal);
    test(
        "-1/1000000000000000000000000000000",
        7,
        10,
        Floor,
        "-1.1141e30",
        "-0xe.10E+24#10",
        Less,
    );
    // fifths and tenths of a turn, the poles, and a half turn
    test("1", 5, 20, Nearest, "0.32491970", "0x0.532df0#20", Greater);
    test("2", 5, 20, Nearest, "-1.3763828", "-0x1.605aa#20", Less);
    test("1", 10, 20, Nearest, "1.3763828", "0x1.605aa#20", Greater);
    test("3", 10, 20, Nearest, "-0.32491970", "-0x0.532df0#20", Less);
    test("1", 4, 20, Nearest, "0.0", "0x0.0", Equal);
    test("3", 4, 20, Nearest, "-0.0", "-0x0.0", Equal);
    test("1", 2, 20, Nearest, "-Infinity", "-Infinity", Equal);
    test(
        "1/5",
        1,
        20,
        Nearest,
        "0.32491970",
        "0x0.532df0#20",
        Greater,
    );
    test("2/5", 1, 20, Nearest, "-1.3763828", "-0x1.605aa#20", Less);
    test("1/10", 1, 20, Nearest, "1.3763828", "0x1.605aa#20", Greater);
    test(
        "-1/5",
        1,
        20,
        Nearest,
        "-0.32491970",
        "-0x0.532df0#20",
        Less,
    );
    test("7/5", 1, 20, Nearest, "-1.3763828", "-0x1.605aa#20", Less);
}

#[test]
#[should_panic]
fn cot_with_period_rational_prec_round_fail_1() {
    Float::cot_with_period_rational_prec_round(Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn cot_with_period_rational_prec_round_fail_2() {
    Float::cot_with_period_rational_prec_round(Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn cot_with_period_rational_prec_round_fail_3() {
    // a twelfth of a turn is sqrt(3), which is not exact
    Float::cot_with_period_rational_prec_round(Rational::from_unsigneds(1u8, 12), 1, 10, Exact);
}

#[test]
#[should_panic]
fn cot_with_period_rational_prec_round_ref_fail() {
    Float::cot_with_period_rational_prec_round_ref(&Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn cot_with_period_rational_prec_fail() {
    Float::cot_with_period_rational_prec(Rational::ONE, 7, 0);
}

#[allow(clippy::needless_pass_by_value)]
fn cot_with_period_rational_prec_round_properties_helper(
    x: Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (t, o) = Float::cot_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
        if o == Equal {
            let (te, oe) = Float::cot_with_period_rational_prec_round_ref(&x, u, prec, Exact);
            assert_eq!(ComparableFloatRef(&te), ComparableFloatRef(&t));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(Float::cot_with_period_rational_prec_round_ref(
                &x, u, prec, Exact
            ));
        }
        return;
    }
    let (t, o) = Float::cot_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(t.is_valid());
    assert_rounding_ordering_consistent(&t, rm, o);

    let (t_alt, o_alt) = Float::cot_with_period_rational_prec_round_ref(&x, u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    // MPFR has no cotu, so the oracle is a wider cosine over a wider sine, bracketed
    if let Some((t_alt, o_alt)) = cot_with_period_rational_naive(&x, u, prec, rm) {
        assert_eq!(
            ComparableFloatRef(&t_alt),
            ComparableFloatRef(&t),
            "NAIVEPROBE x = {x} u = {u} prec = {prec} rm = {rm:?}"
        );
        assert_eq!(o_alt, o);
    }

    // NaN exactly for u = 0
    assert_eq!(t.is_nan(), u == 0);
    if u != 0 {
        // an infinity is either a pole, which is exact, or an overflow, which is not; a finite
        // result carries the requested precision
        if t.is_normal() {
            assert_eq!(t.get_prec(), Some(prec));
        }
        // cot is odd (a `Rational` has no negative zero, so x = 0 is excluded: its cotangent is the
        // pole's positive infinity either way)
        if x != 0u32 {
            let (t_neg, o_neg) = Float::cot_with_period_rational_prec_round(-&x, u, prec, -rm);
            assert_eq!(ComparableFloatRef(&t_neg), ComparableFloatRef(&-&t));
            assert_eq!(o_neg, o.reverse());
        }
        // cot has period u, except at a pole, where the infinity's sign follows the sign of x
        // rather than the angle
        if !t.is_infinite() {
            let (t_shifted, o_shifted) =
                Float::cot_with_period_rational_prec_round(&x + Rational::from(u), u, prec, rm);
            assert_eq!(ComparableFloatRef(&t_shifted), ComparableFloatRef(&t));
            assert_eq!(o_shifted, o);
        }
        // a `Float` input agrees with the `Float` version
        if let Ok(f) = Float::try_from(&x) {
            let (t_alt, o_alt) = f.cot_with_period_prec_round(u, prec, rm);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (t2, oo) = Float::cot_with_period_rational_prec_round_ref(&x, u, prec, rm);
            assert_eq!(ComparableFloatRef(&t2), ComparableFloatRef(&t));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::cot_with_period_rational_prec_round_ref(
            &x, u, prec, Exact
        ));
    }
}

#[test]
fn cot_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, u, prec, rm)| {
            cot_with_period_rational_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // cot(0) = infinity, exactly
        let (t, o) = Float::cot_with_period_rational_prec_round(Rational::ZERO, 4, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);
        let (t, o) = Float::cot_with_period_rational_prec_round(Rational::ONE, 0, prec, rm);
        assert!(t.is_nan());
        assert_eq!(o, Equal);
        // exact cases, straight from a fraction of a turn: the half turns are the poles, the odd
        // quarter turns are the zeros, and the odd eighths are ±1
        let one = Float::one_prec(prec);
        for (s, expected) in [
            ("1/4", Float::ZERO),
            ("3/4", Float::NEGATIVE_ZERO),
            ("-1/4", Float::NEGATIVE_ZERO),
            ("1/2", Float::NEGATIVE_INFINITY),
            ("-1/2", Float::INFINITY),
            ("1", Float::INFINITY),
            ("-1", Float::NEGATIVE_INFINITY),
            ("1/8", one.clone()),
            ("-1/8", -&one),
            ("3/8", -&one),
            ("5/8", one.clone()),
        ] {
            let (t, o) = Float::cot_with_period_rational_prec_round(
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
fn cot_with_period_rational_prec_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, u, prec, _)| {
            let (t, o) = Float::cot_with_period_rational_prec(x.clone(), u, prec);
            assert!(t.is_valid());
            assert_rounding_ordering_consistent(&t, Nearest, o);
            let (t_alt, o_alt) = Float::cot_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) =
                Float::cot_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            if let Some((t_alt, o_alt)) = cot_with_period_rational_naive(&x, u, prec, Nearest) {
                assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
                assert_eq!(o_alt, o);
            }
        },
    );
}

// Fractions of a turn within 2^(-2^30) of a half turn, whose cotangents overflow, and a fraction of

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_cot_with_period_rational() {
    fn test<T: PrimitiveFloat>(s: &str, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_cot_with_period_rational::<T>(&x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0, f32::NAN);
    test::<f32>("0", 360, f32::INFINITY);
    test::<f32>("1", 0, f32::NAN);
    test::<f32>("90", 360, 0.0);
    test::<f32>("-90", 360, -0.0);
    test::<f32>("270", 360, -0.0);
    test::<f32>("180", 360, f32::NEGATIVE_INFINITY);
    test::<f32>("-180", 360, f32::INFINITY);
    test::<f32>("360", 360, f32::INFINITY);
    test::<f32>("45", 360, 1.0);
    test::<f32>("135", 360, -1.0);
    test::<f32>("30", 360, 1.7320508);
    test::<f32>("60", 360, 0.57735026);
    test::<f32>("120", 360, -0.57735026);
    test::<f32>("72", 360, 0.3249197);
    test::<f32>("36", 360, 1.3763819);
    test::<f32>("1/4", 1, 0.0);
    test::<f32>("3/4", 1, -0.0);
    test::<f32>("1/2", 1, f32::NEGATIVE_INFINITY);
    test::<f32>("1/8", 1, 1.0);
    test::<f32>("1/12", 1, 1.7320508);
    test::<f32>("1/6", 1, 0.57735026);
    test::<f32>("1/3", 1, -0.57735026);
    test::<f32>("1/5", 1, 0.3249197);
    test::<f32>("2/5", 1, -1.3763819);
    test::<f32>("1/10", 1, 1.3763819);
    test::<f32>("-1/5", 1, -0.3249197);
    test::<f32>("1/7", 1, 0.7974734);
    test::<f32>("-2/7", 1, 0.22824347);
    test::<f32>("22/7", 1, 0.7974734);
    test::<f32>("1", 7, 0.7974734);
    test::<f32>("1000000", 7, 0.7974734);
    test::<f32>("1/1000000", 1, 159154.94);
    test::<f32>("355/113", 360, 18.21953);
    test::<f64>("0", 0, f64::NAN);
    test::<f64>("0", 360, f64::INFINITY);
    test::<f64>("1", 0, f64::NAN);
    test::<f64>("90", 360, 0.0);
    test::<f64>("-90", 360, -0.0);
    test::<f64>("270", 360, -0.0);
    test::<f64>("180", 360, f64::NEGATIVE_INFINITY);
    test::<f64>("-180", 360, f64::INFINITY);
    test::<f64>("360", 360, f64::INFINITY);
    test::<f64>("45", 360, 1.0);
    test::<f64>("135", 360, -1.0);
    test::<f64>("30", 360, 1.7320508075688772);
    test::<f64>("60", 360, 0.5773502691896257);
    test::<f64>("120", 360, -0.5773502691896257);
    test::<f64>("72", 360, 0.32491969623290634);
    test::<f64>("36", 360, 1.3763819204711736);
    test::<f64>("1/4", 1, 0.0);
    test::<f64>("3/4", 1, -0.0);
    test::<f64>("1/2", 1, f64::NEGATIVE_INFINITY);
    test::<f64>("1/8", 1, 1.0);
    test::<f64>("1/12", 1, 1.7320508075688772);
    test::<f64>("1/6", 1, 0.5773502691896257);
    test::<f64>("1/3", 1, -0.5773502691896257);
    test::<f64>("1/5", 1, 0.32491969623290634);
    test::<f64>("2/5", 1, -1.3763819204711736);
    test::<f64>("1/10", 1, 1.3763819204711736);
    test::<f64>("-1/5", 1, -0.32491969623290634);
    test::<f64>("1/7", 1, 0.7974733888824039);
    test::<f64>("-2/7", 1, 0.22824347439014994);
    test::<f64>("22/7", 1, 0.7974733888824039);
    test::<f64>("1", 7, 0.7974733888824039);
    test::<f64>("1000000", 7, 0.7974733888824039);
    test::<f64>("1/1000000", 1, 159154.94308980095);
    test::<f64>("355/113", 360, 18.219530795919468);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_cot_with_period_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, u)| {
        let s = primitive_float_cot_with_period_rational::<T>(&x, u);
        assert_eq!(s.is_nan(), u == 0);
        if u != 0 {
            // odd, and periodic with period u away from a pole, where the infinity's sign follows
            // the sign of x rather than the angle (a `Rational` has no negative zero, so x = 0 is
            // excluded from the oddness check)
            if x != 0u32 {
                assert_eq!(
                    NiceFloat(primitive_float_cot_with_period_rational::<T>(&-&x, u)),
                    NiceFloat(-s)
                );
            }
            if s.is_finite() {
                assert_eq!(
                    NiceFloat(primitive_float_cot_with_period_rational::<T>(
                        &(&x + Rational::from(u)),
                        u
                    )),
                    NiceFloat(s)
                );
            }
            // the same as the `Float` cotangent taken with 64 bits to spare and rounded once
            let (s_float, _) =
                Float::cot_with_period_rational_prec_ref(&x, u, T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&s_float, Nearest).0),
                NiceFloat(s)
            );
        }
    });

    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        // The cotangent of a finite primitive float, taken through the `Rational` path, matches the
        // direct primitive-float cotangent.
        assert_eq!(
            NiceFloat(primitive_float_cot_with_period_rational::<T>(
                &Rational::exact_from(x),
                u
            )),
            NiceFloat(primitive_float_cot_with_period(x, u))
        );
    });
}

#[test]
fn primitive_float_cot_with_period_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_cot_with_period_rational_properties_helper);
}

// Fractions of a turn within 2^(-2^30) of a half turn, whose cotangents overflow, of a quarter
// turn, where they underflow instead, and a fraction of a turn small enough that its cotangent
// overflows on its own. The near-zero paths work with the exact distance to the multiple of 1/4, so
// no 2^30-bit pi is ever formed.
#[test]
fn test_cot_with_period_rational_underflow_and_overflow() {
    let max = Float::max_finite_value_with_prec(10);
    let min = Float::min_positive_value_prec(10);
    let eps = Rational::power_of_2(-((1i64 << 30) + 70));
    // just past a half turn: the sine is negative and tiny while the cosine is just below -1, so
    // the cotangent is positive and beyond the largest finite `Float`
    let above = Rational::from_unsigneds(1u32, 2u32) + &eps;
    let (t, o) = Float::cot_with_period_rational_prec_round_ref(&above, 1, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (t, o) = Float::cot_with_period_rational_prec_round_ref(&above, 1, 10, Down);
    assert_eq!(ComparableFloatRef(&t), ComparableFloatRef(&max));
    assert_eq!(o, Less);
    // just below a half turn: the cotangent is negative and beyond the largest finite `Float`
    let below = Rational::from_unsigneds(1u32, 2u32) - &eps;
    let (t, o) = Float::cot_with_period_rational_prec_round_ref(&below, 1, 10, Nearest);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    let (t, o) = Float::cot_with_period_rational_prec_round_ref(&below, 1, 10, Down);
    assert_eq!(ComparableFloatRef(&t), ComparableFloatRef(&-max.clone()));
    assert_eq!(o, Greater);
    // a non-dyadic version: 1/2 + 1/(3 * 2^(2^30 + 70)) of a turn
    let above = Rational::from_unsigneds(1u32, 2u32) + &eps / Rational::from(3u32);
    let (t, o) = Float::cot_with_period_rational_prec_round_ref(&above, 1, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    // just past a quarter turn: the cosine is negative and tiny while the sine is just below 1, so
    // the cotangent is a negative underflow
    let above = Rational::from_unsigneds(1u32, 4u32) + &eps;
    let (t, o) = Float::cot_with_period_rational_prec_round_ref(&above, 1, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    let (t, o) = Float::cot_with_period_rational_prec_round_ref(&above, 1, 10, Up);
    assert_eq!(ComparableFloatRef(&t), ComparableFloatRef(&-min.clone()));
    assert_eq!(o, Less);
    // just below a quarter turn: a positive underflow
    let below = Rational::from_unsigneds(1u32, 4u32) - &eps;
    let (t, o) = Float::cot_with_period_rational_prec_round_ref(&below, 1, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    let (t, o) = Float::cot_with_period_rational_prec_round_ref(&below, 1, 10, Up);
    assert_eq!(ComparableFloatRef(&t), ComparableFloatRef(&min));
    assert_eq!(o, Greater);
    // a tiny fraction of a turn, whose cotangent is about u/(2 pi x) and overflows on its own; the
    // `Float` version cannot reach this, since no `Float` is this small
    let (t, o) = Float::cot_with_period_rational_prec_round_ref(&eps, 1, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (t, o) = Float::cot_with_period_rational_prec_round_ref(&-eps, 1, 10, Nearest);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
}

// An input too large to be a `Float`, reduced modulo 2 pi in `Rational` arithmetic with pi to about

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_cot_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_cot_rational::<T>(&x)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", f32::INFINITY);
    test::<f32>("1", 0.64209265);
    test::<f32>("-1", -0.64209265);
    test::<f32>("1/2", 1.8304877);
    test::<f32>("1/3", 2.888057);
    test::<f32>("22/7", 790.8327);
    test::<f32>("1/7", 6.952316);
    test::<f32>("100", -1.7029569);
    test::<f32>("355/113", 3748629.0);
    test::<f32>("1/1000000", 1000000.0);
    test::<f32>("-2/3", -1.2709018);
    test::<f32>("1000000", -2.6764843);
    test::<f32>("1/100000000000000000000", 1.0e20);
    test::<f64>("0", f64::INFINITY);
    test::<f64>("1", 0.6420926159343308);
    test::<f64>("-1", -0.6420926159343308);
    test::<f64>("1/2", 1.830487721712452);
    test::<f64>("1/3", 2.888057036277277);
    test::<f64>("22/7", 790.8327044311329);
    test::<f64>("1/7", 6.9523160383796965);
    test::<f64>("100", -1.7029569194264693);
    test::<f64>("355/113", 3748629.092662727);
    test::<f64>("1/1000000", 999999.9999996667);
    test::<f64>("-2/3", -1.2709017433833507);
    test::<f64>("1000000", -2.6764843396283453);
    test::<f64>("1/100000000000000000000", 1.0e20);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_cot_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let s = primitive_float_cot_rational::<T>(&x);
        // the cotangent of a rational is never NaN
        assert!(!s.is_nan());
        // cot is odd (a `Rational` has no negative zero, so x = 0 is excluded)
        if x != 0u32 {
            assert_eq!(
                NiceFloat(primitive_float_cot_rational::<T>(&-&x)),
                NiceFloat(-s)
            );
        }
        // The result is the correctly rounded cotangent, as computed by MPFR with 64 bits to spare.
        // The comparison is skipped when rounding that wider value to `T` is a tie: cot(1/n) is
        // just above n, so for an n that is a midpoint of the `T` grid the wider value rounds to
        // the midpoint itself and the tie breaks the wrong way, though the cotangent is strictly
        // above it.
        let wide = <Float as From<&rug::Float>>::from(
            &rug_cot_rational_prec(&x, T::MANTISSA_WIDTH + 64).0,
        );
        if !ties::<T>(&wide) {
            assert_eq!(NiceFloat(T::rounding_from(&wide, Nearest).0), NiceFloat(s));
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // The cotangent of a finite nonzero primitive float, taken through the `Rational` path,
        // matches the direct primitive-float cotangent (a `Rational` cannot carry the sign of a
        // zero).
        if x.is_finite() && x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_cot_rational::<T>(&Rational::exact_from(x))),
                NiceFloat(primitive_float_cot(x))
            );
        }
    });
}

#[test]
fn primitive_float_cot_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_cot_rational_properties_helper);
}

// An input too large to be a `Float`, reduced modulo 2 pi in `Rational` arithmetic with pi to about
// 2^30 bits; slow even in release mode.
#[test]
fn test_cot_rational_huge() {
    let x = Rational::power_of_2(1i64 << 30);
    let (t, o) = Float::cot_rational_prec_round_ref(&x, 10, Nearest);
    assert_eq!(t.to_string(), "-1.2402");
    assert_eq!(to_hex_string(&t), "-0x1.3d8#10");
    assert_eq!(o, Greater);
}

// `Rational` inputs at both ends: within 2^(-2^30) of pi, where the cotangent overflows, and of
// pi/2, where it underflows. Each call computes pi to about 2^30 bits twice, so this test is slow
// even in release mode and makes just one call of each kind, off a single pi. The tiny input, far
// below the `Float` exponent range, takes the cheap series bracket and needs no pi at all.
#[test]
fn test_cot_rational_underflow_and_overflow() {
    let p = (1u64 << 30) + 64;
    // a `Rational` far below the `Float` exponent range, where the cotangent is about its
    // reciprocal: the tiny path decides it from the tangent's series bracket
    let tiny = Rational::power_of_2(-((1i64 << 30) + 70));
    let (t, o) = Float::cot_rational_prec_round_ref(&tiny, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (t, o) = Float::cot_rational_prec_round_ref(&tiny, 10, Down);
    assert_eq!(
        ComparableFloatRef(&t),
        ComparableFloatRef(&Float::max_finite_value_with_prec(10))
    );
    assert_eq!(o, Less);
    let (t, o) = Float::cot_rational_prec_round_ref(&-tiny, 10, Nearest);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    // pi rounded down: x < pi, so the sine is positive and tiny while the cosine is just above -1,
    // and the cotangent is negative and beyond the largest finite `Float`
    let pi = Rational::exact_from(&Float::pi_prec_round(p, Floor).0);
    let (t, o) = Float::cot_rational_prec_round_ref(&pi, 10, Nearest);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    // pi/2 rounded down: the cosine is positive and tiny while the sine is just below 1, so the
    // cotangent is positive and below the smallest positive `Float`
    let half_pi = pi >> 1u32;
    let (t, o) = Float::cot_rational_prec_round_ref(&half_pi, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
}

// Inputs within 2^(-2^30) of pi and of pi/2, where the cotangent overflows or underflows. Each call
// computes pi to about 2^30 bits twice (once for the sine and cosine, once for the exact bracket of
// the underflowed one), so this test is slow even in release mode and makes just one call of each
// kind, off a single pi: the overflow exercises the shortcut for an underflowed sine, and the
// underflow the cosine's exact bracket. The tiny input, whose reciprocal alone leaves the range,
// takes the cheap reciprocal path and needs no pi at all.
#[test]
fn test_cot_underflow_and_overflow() {
    let p = (1u64 << 30) + 64;
    // pi rounded down: x < pi, so the sine is positive and tiny while the cosine is just above -1,
    // and the cotangent is negative and beyond the largest finite `Float`
    let pi = Float::pi_prec_round(p, Floor).0;
    let (t, o) = pi.cot_prec_round_ref(10, Nearest);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    // pi/2 rounded down: the cosine is positive and tiny while the sine is just below 1, so the
    // cotangent is positive and below the smallest positive `Float`
    let half_pi = pi >> 1u32;
    let (t, o) = half_pi.cot_prec_round_ref(10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    // the smallest positive `Float`, whose cotangent is about its reciprocal: the tiny path returns
    // the overflow directly, without pi
    let tiny = Float::one_prec(10) >> (1u64 << 30);
    let (t, o) = tiny.cot_prec_round_ref(10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (t, o) = tiny.cot_prec_round_ref(10, Down);
    assert_eq!(
        ComparableFloatRef(&t),
        ComparableFloatRef(&Float::max_finite_value_with_prec(10))
    );
    assert_eq!(o, Less);
    let (t, o) = (-tiny).cot_prec_round_ref(10, Nearest);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
}

#[test]
fn test_cot_pi_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (t, o) = x.clone().cot_pi_prec_round(prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.cot_pi_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.cot_pi_prec_round_assign(prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = x.cot_pi_prec_ref(prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        // MPFR has no cotpi, so the oracle is the reciprocal of a wider sine, bracketed
        if let Some((t_alt, o_alt)) = cot_with_period_naive(&x, 2, prec, rm) {
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    };
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Ceiling, "NaN", "NaN", Equal);
    test("NaN", "NaN", 10, Exact, "NaN", "NaN", Equal);
    test("NaN", "NaN", 53, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Ceiling, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Exact, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 53, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Nearest, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Floor, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 10, Exact, "Infinity", "Infinity", Equal);
    test("0.0", "0x0.0", 53, Nearest, "Infinity", "Infinity", Equal);
    test("0.50", "0x0.8#1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8#1", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8#1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8#1", 10, Exact, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8#1", 53, Nearest, "0.0", "0x0.0", Equal);
    test("1.5", "0x1.8#2", 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("1.5", "0x1.8#2", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("1.5", "0x1.8#2", 10, Floor, "-0.0", "-0x0.0", Equal);
    test("1.5", "0x1.8#2", 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test("1.5", "0x1.8#2", 10, Exact, "-0.0", "-0x0.0", Equal);
    test("1.5", "0x1.8#2", 53, Nearest, "-0.0", "-0x0.0", Equal);
    test(
        "1.0",
        "0x1.0#1",
        1,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("1.0", "0x1.0#1", 10, Floor, "-Infinity", "-Infinity", Equal);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("1.0", "0x1.0#1", 10, Exact, "-Infinity", "-Infinity", Equal);
    test(
        "1.0",
        "0x1.0#1",
        53,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("2.0", "0x2.0#1", 1, Nearest, "Infinity", "Infinity", Equal);
    test("2.0", "0x2.0#1", 10, Nearest, "Infinity", "Infinity", Equal);
    test("2.0", "0x2.0#1", 10, Floor, "Infinity", "Infinity", Equal);
    test("2.0", "0x2.0#1", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("2.0", "0x2.0#1", 10, Exact, "Infinity", "Infinity", Equal);
    test("2.0", "0x2.0#1", 53, Nearest, "Infinity", "Infinity", Equal);
    test("0.25", "0x0.4#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test("0.25", "0x0.4#1", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        1,
        Nearest,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Nearest,
        "3.0781",
        "0x3.14#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Floor,
        "3.0742",
        "0x3.13#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Ceiling,
        "3.0781",
        "0x3.14#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        53,
        Nearest,
        "3.0776835371752531",
        "0x3.13e3117b9af5c#53",
        Less,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        1,
        Nearest,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        10,
        Nearest,
        "0.32471",
        "0x0.532#10",
        Less,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        10,
        Floor,
        "0.32471",
        "0x0.532#10",
        Less,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        10,
        Ceiling,
        "0.32520",
        "0x0.534#10",
        Greater,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        53,
        Nearest,
        "0.32491969623290623",
        "0x0.532defed2586b4#53",
        Less,
    );
    test("100.2", "0x64.4#9", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test(
        "100.2",
        "0x64.4#9",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "100.2",
        "0x64.4#9",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "100.2",
        "0x64.4#9",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "100.2",
        "0x64.4#9",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Ceiling,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        53,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        1,
        Nearest,
        "3.2e29",
        "0x4.0E+24#1",
        Less,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        10,
        Nearest,
        "3.1815e29",
        "0x4.04E+24#10",
        Less,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        10,
        Floor,
        "3.1815e29",
        "0x4.04E+24#10",
        Less,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        10,
        Ceiling,
        "3.1877e29",
        "0x4.06E+24#10",
        Greater,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        53,
        Nearest,
        "3.1830988618379062e29",
        "0x4.0483c44234378E+24#53",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        1,
        Nearest,
        "-4.0",
        "-0x4.0#1",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Nearest,
        "-3.0781",
        "-0x3.14#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Floor,
        "-3.0781",
        "-0x3.14#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Ceiling,
        "-3.0742",
        "-0x3.13#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        53,
        Nearest,
        "-3.0776835371752531",
        "-0x3.13e3117b9af5c#53",
        Greater,
    );
    test("1.0", "0x1.0#1", 10, Exact, "-Infinity", "-Infinity", Equal);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("-1.0", "-0x1.0#1", 10, Exact, "Infinity", "Infinity", Equal);
    test(
        "-1.0", "-0x1.0#1", 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test("2.0", "0x2.0#2", 10, Exact, "Infinity", "Infinity", Equal);
    test("2.0", "0x2.0#2", 10, Nearest, "Infinity", "Infinity", Equal);
    test(
        "-2.0",
        "-0x2.0#2",
        10,
        Exact,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-2.0",
        "-0x2.0#2",
        10,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("0.50", "0x0.8#1", 10, Exact, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-0.50", "-0x0.8#1", 10, Exact, "-0.0", "-0x0.0", Equal);
    test("-0.50", "-0x0.8#1", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("1.5", "0x1.8#2", 10, Exact, "-0.0", "-0x0.0", Equal);
    test("1.5", "0x1.8#2", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("-1.5", "-0x1.8#2", 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.5", "-0x1.8#2", 10, Nearest, "0.0", "0x0.0", Equal);
    test(
        "0.25",
        "0x0.4#1",
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        Equal,
    );
    test(
        "0.12",
        "0x0.2#1",
        20,
        Nearest,
        "2.4142151",
        "0x2.6a0a0#20",
        Greater,
    );
    test(
        "0.38",
        "0x0.6#2",
        20,
        Nearest,
        "0.41421366",
        "0x0.6a09e8#20",
        Greater,
    );
    test(
        "0.62",
        "0x0.a#3",
        20,
        Nearest,
        "-0.41421366",
        "-0x0.6a09e8#20",
        Less,
    );
    test(
        "1.2",
        "0x1.4#3",
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        Equal,
    );
}

#[test]
fn test_cot_pi_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (t, o) = Float::cot_pi_rational_prec_round(x.clone(), prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = Float::cot_pi_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = Float::cot_pi_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) = Float::cot_pi_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    };
    test("0", 1, Nearest, "Infinity", "Infinity", Equal);
    test("0", 10, Nearest, "Infinity", "Infinity", Equal);
    test("0", 10, Floor, "Infinity", "Infinity", Equal);
    test("0", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("0", 10, Exact, "Infinity", "Infinity", Equal);
    test("0", 53, Nearest, "Infinity", "Infinity", Equal);
    test("1/2", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1/2", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1/2", 10, Floor, "0.0", "0x0.0", Equal);
    test("1/2", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1/2", 10, Exact, "0.0", "0x0.0", Equal);
    test("1/2", 53, Nearest, "0.0", "0x0.0", Equal);
    test("1", 1, Nearest, "-Infinity", "-Infinity", Equal);
    test("1", 10, Nearest, "-Infinity", "-Infinity", Equal);
    test("1", 10, Floor, "-Infinity", "-Infinity", Equal);
    test("1", 10, Ceiling, "-Infinity", "-Infinity", Equal);
    test("1", 10, Exact, "-Infinity", "-Infinity", Equal);
    test("1", 53, Nearest, "-Infinity", "-Infinity", Equal);
    test("1/3", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("1/3", 10, Nearest, "0.57715", "0x0.93c#10", Less);
    test("1/3", 10, Floor, "0.57715", "0x0.93c#10", Less);
    test("1/3", 10, Ceiling, "0.57812", "0x0.940#10", Greater);
    test(
        "1/3",
        53,
        Nearest,
        "0.57735026918962573",
        "0x0.93cd3a2c8198e0#53",
        Less,
    );
    test("2/3", 1, Nearest, "-0.50", "-0x0.8#1", Greater);
    test("2/3", 10, Nearest, "-0.57715", "-0x0.93c#10", Greater);
    test("2/3", 10, Floor, "-0.57812", "-0x0.940#10", Less);
    test("2/3", 10, Ceiling, "-0.57715", "-0x0.93c#10", Greater);
    test(
        "2/3",
        53,
        Nearest,
        "-0.57735026918962573",
        "-0x0.93cd3a2c8198e0#53",
        Greater,
    );
    test("1/4", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("1/4", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("1/4", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("1/4", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test(
        "1/4",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test("1/6", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1/6", 10, Nearest, "1.7324", "0x1.bb8#10", Greater);
    test("1/6", 10, Floor, "1.7305", "0x1.bb0#10", Less);
    test("1/6", 10, Ceiling, "1.7324", "0x1.bb8#10", Greater);
    test(
        "1/6",
        53,
        Nearest,
        "1.7320508075688772",
        "0x1.bb67ae8584caa#53",
        Less,
    );
    test("1/5", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/5", 10, Nearest, "1.3770", "0x1.608#10", Greater);
    test("1/5", 10, Floor, "1.3750", "0x1.600#10", Less);
    test("1/5", 10, Ceiling, "1.3770", "0x1.608#10", Greater);
    test(
        "1/5",
        53,
        Nearest,
        "1.3763819204711736",
        "0x1.605a90c73ab79#53",
        Greater,
    );
    test("2/5", 1, Nearest, "0.25", "0x0.4#1", Less);
    test("2/5", 10, Nearest, "0.32471", "0x0.532#10", Less);
    test("2/5", 10, Floor, "0.32471", "0x0.532#10", Less);
    test("2/5", 10, Ceiling, "0.32520", "0x0.534#10", Greater);
    test(
        "2/5",
        53,
        Nearest,
        "0.32491969623290634",
        "0x0.532defed2586bc#53",
        Greater,
    );
    test("1/7", 1, Nearest, "2.0", "0x2.0#1", Less);
    test("1/7", 10, Nearest, "2.0781", "0x2.14#10", Greater);
    test("1/7", 10, Floor, "2.0742", "0x2.13#10", Less);
    test("1/7", 10, Ceiling, "2.0781", "0x2.14#10", Greater);
    test(
        "1/7",
        53,
        Nearest,
        "2.0765213965723364",
        "0x2.1396e7ffb8f14#53",
        Less,
    );
    test("-3/7", 1, Nearest, "-0.25", "-0x0.4#1", Less);
    test("-3/7", 10, Nearest, "-0.22827", "-0x0.3a7#10", Less);
    test("-3/7", 10, Floor, "-0.22827", "-0x0.3a7#10", Less);
    test("-3/7", 10, Ceiling, "-0.22803", "-0x0.3a6#10", Greater);
    test(
        "-3/7",
        53,
        Nearest,
        "-0.22824347439014994",
        "-0x0.3a6e2a1207f6aa#53",
        Less,
    );
    test("22/7", 1, Nearest, "2.0", "0x2.0#1", Less);
    test("22/7", 10, Nearest, "2.0781", "0x2.14#10", Greater);
    test("22/7", 10, Floor, "2.0742", "0x2.13#10", Less);
    test("22/7", 10, Ceiling, "2.0781", "0x2.14#10", Greater);
    test(
        "22/7",
        53,
        Nearest,
        "2.0765213965723364",
        "0x2.1396e7ffb8f14#53",
        Less,
    );
    test("1/1000000", 1, Nearest, "2.6e5", "0x4.0E+4#1", Less);
    test(
        "1/1000000",
        10,
        Nearest,
        "3.1846e5",
        "0x4.dcE+4#10",
        Greater,
    );
    test("1/1000000", 10, Floor, "3.1795e5", "0x4.daE+4#10", Less);
    test(
        "1/1000000",
        10,
        Ceiling,
        "3.1846e5",
        "0x4.dcE+4#10",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Nearest,
        "318309.88618274347",
        "0x4db65.e2dcdf4d8#53",
        Less,
    );
    test("1", 10, Exact, "-Infinity", "-Infinity", Equal);
    test("1", 10, Nearest, "-Infinity", "-Infinity", Equal);
    test("-1", 10, Exact, "Infinity", "Infinity", Equal);
    test("-1", 10, Nearest, "Infinity", "Infinity", Equal);
    test("2", 10, Exact, "Infinity", "Infinity", Equal);
    test("2", 10, Nearest, "Infinity", "Infinity", Equal);
    test("-2", 10, Exact, "-Infinity", "-Infinity", Equal);
    test("-2", 10, Nearest, "-Infinity", "-Infinity", Equal);
    test("1/2", 10, Exact, "0.0", "0x0.0", Equal);
    test("1/2", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-1/2", 10, Exact, "-0.0", "-0x0.0", Equal);
    test("-1/2", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("3/2", 10, Exact, "-0.0", "-0x0.0", Equal);
    test("3/2", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("1/6", 10, Nearest, "1.7324", "0x1.bb8#10", Greater);
    test("5/6", 10, Nearest, "-1.7324", "-0x1.bb8#10", Less);
    test("-1/6", 10, Nearest, "-1.7324", "-0x1.bb8#10", Less);
    test("1/3", 20, Nearest, "0.57735062", "0x0.93cd4#20", Greater);
    test("1/4", 20, Nearest, "1.0000000", "0x1.00000#20", Equal);
    test("1/5", 20, Nearest, "1.3763828", "0x1.605aa#20", Greater);
    test("1/10", 20, Nearest, "3.0776825", "0x3.13e30#20", Less);
    test("3/10", 20, Nearest, "0.72654247", "0x0.b9feb#20", Less);
    test("1/7", 20, Nearest, "2.0765228", "0x2.13970#20", Greater);
}

// Every `cot_pi` variant is `cot_with_period` with a period of 2.
#[test]
fn cot_pi_properties() {
    // The borrowed generators admit `Exact` for inputs whose cotangent is not exact, so `Exact` is
    // checked against the exactness of the result.
    let exact_ok = |x: &Float, prec: u64, rm: RoundingMode| {
        rm != Exact || x.cot_with_period_prec_round_ref(2, prec, Nearest).1 == Equal
    };
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        if !exact_ok(&x, prec, rm) {
            assert_panic!(x.cot_pi_prec_round_ref(prec, Exact));
            return;
        }
        let (t, o) = x.clone().cot_pi_prec_round(prec, rm);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = x.cot_with_period_prec_round_ref(2, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.cot_pi_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.cot_pi_prec_round_assign(prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        if let Some((t_alt, o_alt)) = cot_with_period_naive(&x, 2, prec, rm) {
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    });

    float_unsigned_rounding_mode_triple_gen_var_37().test_properties(|(x, prec, rm)| {
        if !exact_ok(&x, prec, rm) {
            return;
        }
        let (t, o) = x.cot_pi_prec_round_ref(prec, rm);
        let (t_alt, o_alt) = x.cot_with_period_prec_round_ref(2, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (t, o) = x.clone().cot_pi_prec(prec);
        let (t_alt, o_alt) = x.cot_with_period_prec_ref(2, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.cot_pi_prec_ref(prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.cot_pi_prec_assign(prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });

    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        if !exact_ok(&x, x.significant_bits(), rm) {
            return;
        }
        let (t, o) = x.clone().cot_pi_round(rm);
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = x.cot_with_period_round_ref(2, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.cot_pi_round_ref(rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.cot_pi_round_assign(rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });

    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        if rm == Exact && Float::cot_with_period_rational_prec_ref(&x, 2, prec).1 != Equal {
            assert_panic!(Float::cot_pi_rational_prec_round_ref(&x, prec, Exact));
            return;
        }
        let (t, o) = Float::cot_pi_rational_prec_round(x.clone(), prec, rm);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = Float::cot_with_period_rational_prec_round_ref(&x, 2, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = Float::cot_pi_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        // the cotangent is the reciprocal of the cosine in the same units
        if let Some((t_alt, o_alt)) = cot_with_period_rational_naive(&x, 2, prec, rm) {
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (t, o) = Float::cot_pi_rational_prec(x.clone(), prec);
        let (t_alt, o_alt) = Float::cot_with_period_rational_prec_ref(&x, 2, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = Float::cot_pi_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });

    float_gen().test_properties(|x| {
        let t = x.clone().cot_pi();
        assert!(t.is_valid());
        let t_alt = x.cot_pi_ref();
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let mut t_alt = x.clone();
        t_alt.cot_pi_assign();
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        // the same as cot_with_period with a period of 2, and as rounding to the input't precision,
        // to nearest
        let t_alt = x.cot_with_period_ref(2);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let (t_alt, _) = x.cot_pi_prec_round_ref(x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_cot_pi() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_cot_pi(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(0.5, 0.0);
    test::<f32>(1.0, f32::NEGATIVE_INFINITY);
    test::<f32>(-1.0, f32::INFINITY);
    test::<f32>(0.25, 1.0);
    test::<f32>(0.1, 3.0776834);
    test::<f32>(-0.1, -3.0776834);
    test::<f32>(100.25, 1.0);
    test::<f32>(10000000000.0, f32::INFINITY);
    test::<f32>(1.0e-45, f32::INFINITY);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(0.5, 0.0);
    test::<f64>(1.0, f64::NEGATIVE_INFINITY);
    test::<f64>(-1.0, f64::INFINITY);
    test::<f64>(0.25, 1.0);
    test::<f64>(0.1, 3.077683537175253);
    test::<f64>(-0.1, -3.077683537175253);
    test::<f64>(100.25, 1.0);
    test::<f64>(10000000000.0, f64::INFINITY);
    test::<f64>(5.0e-324, f64::INFINITY);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_cot_pi_rational() {
    fn test<T: PrimitiveFloat>(t: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(t).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_cot_pi_rational::<T>(&x)),
            NiceFloat(out)
        );
    }
    test::<f32>("1/2", 0.0);
    test::<f32>("1", f32::NEGATIVE_INFINITY);
    test::<f32>("-1", f32::INFINITY);
    test::<f32>("1/6", 1.7320508);
    test::<f32>("1/3", 0.57735026);
    test::<f32>("1/4", 1.0);
    test::<f32>("1/7", 2.0765214);
    test::<f32>("-3/7", -0.22824347);
    test::<f32>("22/7", 2.0765214);
    test::<f64>("1/2", 0.0);
    test::<f64>("1", f64::NEGATIVE_INFINITY);
    test::<f64>("-1", f64::INFINITY);
    test::<f64>("1/6", 1.7320508075688772);
    test::<f64>("1/3", 0.5773502691896257);
    test::<f64>("1/4", 1.0);
    test::<f64>("1/7", 2.0765213965723364);
    test::<f64>("-3/7", -0.22824347439014994);
    test::<f64>("22/7", 2.0765213965723364);
}

#[test]
#[should_panic]
fn cot_pi_prec_round_fail_1() {
    Float::from(0.1f64).cot_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn cot_pi_prec_round_fail_2() {
    Float::from(0.1f64).cot_pi_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn cot_pi_rational_prec_round_fail() {
    Float::cot_pi_rational_prec_round(Rational::from_unsigneds(1u8, 7), 10, Exact);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_cot_pi_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        assert_eq!(
            NiceFloat(primitive_float_cot_pi(x)),
            NiceFloat(primitive_float_cot_with_period(x, 2))
        );
    });
    rational_gen().test_properties(|x| {
        assert_eq!(
            NiceFloat(primitive_float_cot_pi_rational::<T>(&x)),
            NiceFloat(primitive_float_cot_with_period_rational::<T>(&x, 2))
        );
    });
}

#[test]
fn primitive_float_cot_pi_properties() {
    apply_fn_to_primitive_floats!(primitive_float_cot_pi_properties_helper);
}
