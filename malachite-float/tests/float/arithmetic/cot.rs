// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Cot, CotAssign};
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
    primitive_float_gen, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::cot::primitive_float_cot;
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::cot::{
    rug_cot, rug_cot_prec, rug_cot_prec_round, rug_cot_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::panic::catch_unwind;

// Rows reuse the tangent test's inputs. Branches of `cot_prec_round_normal_ref` covered:
// - tiny x: the reciprocal shortcut, including the power-of-two case that needs the correction
// - the general Ziv loop, at the first working precision and after a retry
// - |x| near a nonzero multiple of pi: a large result, from the sine's near-zero path inside
//   `sin_cos`
// - |x| near an odd multiple of pi/2: a tiny result, from the cosine's near-zero path

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
