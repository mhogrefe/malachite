// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{PowerOf2, Sec, SecAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_unsigned_pair_gen_var_1,
    unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::sec::{
    primitive_float_sec, primitive_float_sec_pi, primitive_float_sec_pi_rational,
    primitive_float_sec_rational, primitive_float_sec_with_period,
    primitive_float_sec_with_period_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::sec::{
    rug_sec, rug_sec_prec, rug_sec_prec_round, rug_sec_rational_prec, rug_sec_rational_prec_round,
    rug_sec_round, sec_with_period_naive, sec_with_period_rational_naive,
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

// Rows reuse the sine test's inputs. Branches of `sec_prec_round_normal_ref` covered:
// - tiny x: the small-input shortcut rounds x directly
// - the general Ziv loop, at the first working precision and after a retry
// - |x| near an odd multiple of pi/2: a large result, from the cosine's near-zero path inside
//   `sin_cos`
// - |x| near a nonzero multiple of pi: a tiny result, from the sine's near-zero path
#[test]
fn test_sec_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (t, o) = x.clone().sec_prec_round(prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.sec_prec_round_ref(prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.sec_prec_round_assign(prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) = rug_sec_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
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
    test("0.0", "0x0.0", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("-0.0", "-0x0.0", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("0.0", "0x0.0", 1, Exact, "1.0", "0x1.0#1", Equal);
    test("-0.0", "-0x0.0", 1, Exact, "1.0", "0x1.0#1", Equal);
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        2,
        Nearest,
        "1.0",
        "0x1.0#2",
        Less,
    );
    test("0.25", "0x0.4#1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("0.25", "0x0.4#1", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("0.25", "0x0.4#1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("0.0", "0x0.0", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("1.0", "0x1.0#1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", 10, Floor, "1.8496", "0x1.d98#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "1.8516",
        "0x1.da0#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "1.8516",
        "0x1.da0#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Floor,
        "1.8508157176809256179117532413979",
        "0x1.d9cf0f125cc29bcec42581638#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Ceiling,
        "1.8508157176809256179117532413995",
        "0x1.d9cf0f125cc29bcec4258163a#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Nearest,
        "1.8508157176809256179117532413979",
        "0x1.d9cf0f125cc29bcec42581638#100",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "1.8516",
        "0x1.da0#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "-2.4023",
        "-0x2.67#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "-1.0098",
        "-0x1.028#10",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        10,
        Nearest,
        "-1.5293",
        "-0x1.878#10",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        100,
        Nearest,
        "-1.5298856564663975746295109229379",
        "-0x1.87a6961d2485ec574ca958f40#100",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        10,
        Nearest,
        "1.1602",
        "0x1.290#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Floor,
        "1.1596638229046938325514044465857",
        "0x1.28dfba71bae100230e587db34#100",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Ceiling,
        "1.1596638229046938325514044465873",
        "0x1.28dfba71bae100230e587db36#100",
        Greater,
    );
    test(
        "1.00000e6",
        "0xf.424E+4#14",
        64,
        Nearest,
        "1.06751825868110167142",
        "0x1.1148e068eb0f23e8#64",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        50,
        Nearest,
        "1.1394939273245495",
        "0x1.23b5dfbfd97b8#50",
        Greater,
    );
    test(
        "0.102",
        "0x0.1a#4",
        50,
        Nearest,
        "1.0051797303287309",
        "0x1.015375745d2f0#50",
        Greater,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        50,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#50",
        Less,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
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
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Greater,
    );
    test(
        "6.28318530717958579",
        "0x6.487ed5110b45e#54",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "9.979e99",
        "0x1.24E+83#7",
        53,
        Nearest,
        "-2.3266464384545729",
        "-0x2.539f19da846ba#53",
        Less,
    );
    test("3.0", "0x3.0#2", 1, Nearest, "-1.0", "-0x1.0#1", Greater);
    test("3.0", "0x3.0#2", 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("3.0", "0x3.0#2", 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("3.0", "0x3.0#2", 2, Nearest, "-1.0", "-0x1.0#2", Greater);
    test("0.25", "0x0.4#1", 1, Down, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", 1, Down, "1.0", "0x1.0#1", Less);
    test("2.0", "0x2.0#1", 1, Down, "-2.0", "-0x2.0#1", Greater);
    test("4.0", "0x4.0#1", 1, Down, "-1.0", "-0x1.0#1", Greater);
    test(
        "-3.495934488151859089160804055e56",
        "-0xe.41ed086a5791d9e5b2924E+46#87",
        2,
        Down,
        "1.0",
        "0x1.0#2",
        Less,
    );
    test(
        "6.28318536",
        "0x6.487ed6#26",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
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
        "429086628331787135927979170061126207.00",
        "0x52a39aa81dcd0e808d7147ed108a3f.0#120",
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        64,
        Nearest,
        "-1.00000000000000000000",
        "-0x1.0000000000000000#64",
        Greater,
    );
    test(
        "2.5",
        "0x2.8#3",
        10,
        Nearest,
        "-1.2480",
        "-0x1.3f8#10",
        Greater,
    );
    test(
        "-2.5",
        "-0x2.8#3",
        10,
        Floor,
        "-1.2500",
        "-0x1.400#10",
        Less,
    );
    test(
        "2.99976",
        "0x2.fff#14",
        20,
        Nearest,
        "-1.0101433",
        "-0x1.0298c#20",
        Greater,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        10,
        Ceiling,
        "-1.0098",
        "-0x1.028#10",
        Greater,
    );
    test(
        "6.28318548",
        "0x6.487ed8#24",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "6.28318548",
        "0x6.487ed8#24",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "6.283185307179586476925286766559005788",
        "0x6.487ed5110b4611a62633145c06e10#117",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "7.1e-323228497",
        "0x3.0E-268435456#2",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "-7.1e-323228497",
        "-0x3.0E-268435456#2",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
}

#[test]
#[should_panic]
fn sec_prec_round_fail() {
    Float::ONE.sec_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn sec_round_fail() {
    Float::ONE.sec_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn sec_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (s, o) = x.clone().sec_prec_round(prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = x.sec_prec_round_ref(prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.sec_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_o) = rug_sec_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    }

    // sec is even
    let (s_neg, o_neg) = (-&x).sec_prec_round(prec, rm);
    assert_eq!(ComparableFloatRef(&s_neg), ComparableFloatRef(&s));
    assert_eq!(o_neg, o);

    if s.is_normal() {
        assert_eq!(s.get_prec(), Some(prec));
    }

    if o == Equal {
        // sec is exact only for x = 0 (and NaN, and ±inf), where it is 1: the result is
        // rounding-mode-invariant
        if x.is_finite() {
            assert_eq!(
                ComparableFloatRef(&s),
                ComparableFloatRef(&Float::one_prec(prec))
            );
        }
        for rm2 in exhaustive_rounding_modes() {
            let (s2, o2) = x.sec_prec_round_ref(prec, rm2);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.sec_prec_round_ref(prec, Exact));
    }
}

#[test]
fn sec_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        sec_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.sec_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::INFINITY.sec_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_INFINITY.sec_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        // sec(+0) = sec(-0) = 1, exactly
        let (s, o) = Float::ZERO.sec_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.sec_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o, Equal);
    });
}

#[test]
fn sec_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (s, o) = x.clone().sec_round(rm);
        assert!(s.is_valid());
        assert_rounding_ordering_consistent(&s, rm, o);
        let (s_alt, o_alt) = x.sec_round_ref(rm);
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.sec_round_assign(rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (s_alt, o_alt) = x.sec_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_s, rug_o) = rug_sec_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn sec_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (s, o) = x.clone().sec_prec(prec);
        assert!(s.is_valid());
        let (s_alt, o_alt) = x.sec_prec_ref(prec);
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.sec_prec_assign(prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (s_alt, o_alt) = x.sec_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (rug_s, rug_o) = rug_sec_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn sec_properties() {
    float_gen().test_properties(|x| {
        let s = x.clone().sec();
        assert!(s.is_valid());
        let s_alt = (&x).sec();
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));

        let mut x_alt = x.clone();
        x_alt.sec_assign();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));

        let s_alt = x.sec_prec_round_ref(x.significant_bits(), Nearest).0;
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sec(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&s)
        );

        // sec is even
        assert_eq!(ComparableFloat((-&x).sec()), ComparableFloat(s));
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sec() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_sec(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, 1.0);
    test::<f32>(-0.0, 1.0);
    test::<f32>(1.0, 1.8508158);
    test::<f32>(-1.0, 1.8508158);
    test::<f32>(0.5, 1.139494);
    test::<f32>(2.0, -2.402998);
    test::<f32>(100.0, 1.1596638);
    test::<f32>(10000000000.0, 1.1453184);
    test::<f32>(1.0e-10, 1.0);
    test::<f32>(1.5707964, -22877332.0);
    test::<f32>(core::f32::consts::PI, -1.0);
    test::<f32>(1.0e-45, 1.0);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, 1.0);
    test::<f64>(-0.0, 1.0);
    test::<f64>(1.0, 1.8508157176809257);
    test::<f64>(-1.0, 1.8508157176809257);
    test::<f64>(0.5, 1.139493927324549);
    test::<f64>(2.0, -2.402997961722381);
    test::<f64>(100.0, 1.1596638229046938);
    test::<f64>(10000000000.0, 1.1453184352152659);
    test::<f64>(1.0e-10, 1.0);
    test::<f64>(core::f64::consts::FRAC_PI_2, 1.633123935319537e16);
    test::<f64>(core::f64::consts::PI, -1.0);
    test::<f64>(1.0e300, -1.737963394003118);
    test::<f64>(5.0e-324, 1.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sec_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let t = primitive_float_sec(x);
        // NaN exactly for NaN and infinite inputs
        assert_eq!(t.is_nan(), !x.is_finite());
        if x.is_finite() {
            // even
            assert_eq!(NiceFloat(primitive_float_sec(-x)), NiceFloat(t));
            // the result is the correctly rounded secant, as computed by MPFR with 64 bits to
            // spare, so that a subnormal result is rounded once by the conversion
            let rug_t = rug_sec_prec(
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
fn primitive_float_sec_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sec_properties_helper);
}

// An input within 2^(-2^30) of an odd multiple of pi/2, whose secant overflows: the cosine there is
// below the smallest positive `Float`, so the reciprocal is decided from the bracket rather than
// from a `Float` division.
//
// Computing pi to 2^30 bits dominates this test, so it is done once: pi is irrational, so rounding
// it up gives exactly the neighbour of rounding it down, and `increment` moves between the two
// sides of pi/2 for free. Each side keeps one `Nearest` call and one directed call toward zero;
// rounding away from zero overflows to an infinity just as `Nearest` does here, so it is not
// repeated. Slow enough to dominate a test run, and its result does not depend on the limb width,
// so it runs only in the 64-bit-limb configuration.
#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_sec_overflow() {
    let p = (1u64 << 30) + 64;
    // just below pi/2, where the cosine is positive and tiny
    let mut half_pi = Float::pi_prec_round(p, Floor).0 >> 1u32;
    let (s, o) = half_pi.sec_prec_round_ref(10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (s, o) = half_pi.sec_prec_round_ref(10, Down);
    assert_eq!(
        ComparableFloat(s),
        ComparableFloat(Float::max_finite_value_with_prec(10))
    );
    assert_eq!(o, Less);
    // just above pi/2, where the cosine is negative and tiny
    half_pi.increment();
    let (s, o) = half_pi.sec_prec_round_ref(10, Nearest);
    assert_eq!(
        ComparableFloat(s),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    let (s, o) = half_pi.sec_prec_round_ref(10, Ceiling);
    assert_eq!(
        ComparableFloat(s),
        ComparableFloat(-Float::max_finite_value_with_prec(10))
    );
    assert_eq!(o, Greater);
}

// Rows reuse the sine test's inputs, including the non-dyadic ones whose secants MPFR cannot see
// exactly, since it must round the input first.
#[test]
fn test_sec_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (t, o) = Float::sec_rational_prec_round(x.clone(), prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = Float::sec_rational_prec_round_ref(&x, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = Float::sec_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) = Float::sec_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) = rug_sec_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 1, Down, "1.0", "0x1.0#1", Equal);
    test("0", 1, Up, "1.0", "0x1.0#1", Equal);
    test("0", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("0", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("0", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("0", 1, Exact, "1.0", "0x1.0#1", Equal);
    test("0", 5, Nearest, "1.00", "0x1.0#5", Equal);
    test("0", 10, Down, "1.0000", "0x1.000#10", Equal);
    test("0", 10, Up, "1.0000", "0x1.000#10", Equal);
    test("0", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("0", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("0", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("0", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("0", 20, Nearest, "1.0000000", "0x1.00000#20", Equal);
    test(
        "0",
        53,
        Down,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0",
        53,
        Up,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0",
        53,
        Floor,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0",
        53,
        Ceiling,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0",
        53,
        Exact,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test("1", 1, Down, "1.0", "0x1.0#1", Less);
    test("1", 1, Up, "2.0", "0x2.0#1", Greater);
    test("1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1", 5, Nearest, "1.88", "0x1.e#5", Greater);
    test("1", 10, Down, "1.8496", "0x1.d98#10", Less);
    test("1", 10, Up, "1.8516", "0x1.da0#10", Greater);
    test("1", 10, Floor, "1.8496", "0x1.d98#10", Less);
    test("1", 10, Ceiling, "1.8516", "0x1.da0#10", Greater);
    test("1", 10, Nearest, "1.8516", "0x1.da0#10", Greater);
    test("1", 20, Nearest, "1.8508148", "0x1.d9cf0#20", Less);
    test(
        "1",
        53,
        Down,
        "1.8508157176809255",
        "0x1.d9cf0f125cc29#53",
        Less,
    );
    test(
        "1",
        53,
        Up,
        "1.8508157176809257",
        "0x1.d9cf0f125cc2a#53",
        Greater,
    );
    test(
        "1",
        53,
        Floor,
        "1.8508157176809255",
        "0x1.d9cf0f125cc29#53",
        Less,
    );
    test(
        "1",
        53,
        Ceiling,
        "1.8508157176809257",
        "0x1.d9cf0f125cc2a#53",
        Greater,
    );
    test(
        "1",
        53,
        Nearest,
        "1.8508157176809257",
        "0x1.d9cf0f125cc2a#53",
        Greater,
    );
    test(
        "1",
        100,
        Nearest,
        "1.8508157176809256179117532413979",
        "0x1.d9cf0f125cc29bcec42581638#100",
        Less,
    );
    test("-1", 1, Down, "1.0", "0x1.0#1", Less);
    test("-1", 1, Up, "2.0", "0x2.0#1", Greater);
    test("-1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("-1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("-1", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("-1", 5, Nearest, "1.88", "0x1.e#5", Greater);
    test("-1", 10, Down, "1.8496", "0x1.d98#10", Less);
    test("-1", 10, Up, "1.8516", "0x1.da0#10", Greater);
    test("-1", 10, Floor, "1.8496", "0x1.d98#10", Less);
    test("-1", 10, Ceiling, "1.8516", "0x1.da0#10", Greater);
    test("-1", 10, Nearest, "1.8516", "0x1.da0#10", Greater);
    test("-1", 20, Nearest, "1.8508148", "0x1.d9cf0#20", Less);
    test(
        "-1",
        53,
        Down,
        "1.8508157176809255",
        "0x1.d9cf0f125cc29#53",
        Less,
    );
    test(
        "-1",
        53,
        Up,
        "1.8508157176809257",
        "0x1.d9cf0f125cc2a#53",
        Greater,
    );
    test(
        "-1",
        53,
        Floor,
        "1.8508157176809255",
        "0x1.d9cf0f125cc29#53",
        Less,
    );
    test(
        "-1",
        53,
        Ceiling,
        "1.8508157176809257",
        "0x1.d9cf0f125cc2a#53",
        Greater,
    );
    test(
        "-1",
        53,
        Nearest,
        "1.8508157176809257",
        "0x1.d9cf0f125cc2a#53",
        Greater,
    );
    test(
        "-1",
        100,
        Nearest,
        "1.8508157176809256179117532413979",
        "0x1.d9cf0f125cc29bcec42581638#100",
        Less,
    );
    test("1/2", 1, Down, "1.0", "0x1.0#1", Less);
    test("1/2", 1, Up, "2.0", "0x2.0#1", Greater);
    test("1/2", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1/2", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1/2", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/2", 5, Nearest, "1.12", "0x1.2#5", Less);
    test("1/2", 10, Down, "1.1387", "0x1.238#10", Less);
    test("1/2", 10, Up, "1.1406", "0x1.240#10", Greater);
    test("1/2", 10, Floor, "1.1387", "0x1.238#10", Less);
    test("1/2", 10, Ceiling, "1.1406", "0x1.240#10", Greater);
    test("1/2", 10, Nearest, "1.1387", "0x1.238#10", Less);
    test("1/2", 20, Nearest, "1.1394939", "0x1.23b5e#20", Greater);
    test(
        "1/2",
        53,
        Down,
        "1.1394939273245490",
        "0x1.23b5dfbfd97b6#53",
        Less,
    );
    test(
        "1/2",
        53,
        Up,
        "1.1394939273245492",
        "0x1.23b5dfbfd97b7#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Floor,
        "1.1394939273245490",
        "0x1.23b5dfbfd97b6#53",
        Less,
    );
    test(
        "1/2",
        53,
        Ceiling,
        "1.1394939273245492",
        "0x1.23b5dfbfd97b7#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Nearest,
        "1.1394939273245490",
        "0x1.23b5dfbfd97b6#53",
        Less,
    );
    test(
        "1/2",
        100,
        Nearest,
        "1.1394939273245491223133277682042",
        "0x1.23b5dfbfd97b67a2fc6dde0ce#100",
        Less,
    );
    test("1/3", 1, Down, "1.0", "0x1.0#1", Less);
    test("1/3", 1, Up, "2.0", "0x2.0#1", Greater);
    test("1/3", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1/3", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1/3", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/3", 5, Nearest, "1.06", "0x1.1#5", Greater);
    test("1/3", 10, Down, "1.0566", "0x1.0e8#10", Less);
    test("1/3", 10, Up, "1.0586", "0x1.0f0#10", Greater);
    test("1/3", 10, Floor, "1.0566", "0x1.0e8#10", Less);
    test("1/3", 10, Ceiling, "1.0586", "0x1.0f0#10", Greater);
    test("1/3", 10, Nearest, "1.0586", "0x1.0f0#10", Greater);
    test("1/3", 20, Nearest, "1.0582485", "0x1.0ee96#20", Less);
    test(
        "1/3",
        53,
        Down,
        "1.0582492714614418",
        "0x1.0ee96c9bf1560#53",
        Less,
    );
    test(
        "1/3",
        53,
        Up,
        "1.0582492714614420",
        "0x1.0ee96c9bf1561#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Floor,
        "1.0582492714614418",
        "0x1.0ee96c9bf1560#53",
        Less,
    );
    test(
        "1/3",
        53,
        Ceiling,
        "1.0582492714614420",
        "0x1.0ee96c9bf1561#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Nearest,
        "1.0582492714614420",
        "0x1.0ee96c9bf1561#53",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "1.0582492714614419014595217776412",
        "0x1.0ee96c9bf15609d47a83b4e70#100",
        Greater,
    );
    test("-1/3", 1, Down, "1.0", "0x1.0#1", Less);
    test("-1/3", 1, Up, "2.0", "0x2.0#1", Greater);
    test("-1/3", 1, Floor, "1.0", "0x1.0#1", Less);
    test("-1/3", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("-1/3", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("-1/3", 5, Nearest, "1.06", "0x1.1#5", Greater);
    test("-1/3", 10, Down, "1.0566", "0x1.0e8#10", Less);
    test("-1/3", 10, Up, "1.0586", "0x1.0f0#10", Greater);
    test("-1/3", 10, Floor, "1.0566", "0x1.0e8#10", Less);
    test("-1/3", 10, Ceiling, "1.0586", "0x1.0f0#10", Greater);
    test("-1/3", 10, Nearest, "1.0586", "0x1.0f0#10", Greater);
    test("-1/3", 20, Nearest, "1.0582485", "0x1.0ee96#20", Less);
    test(
        "-1/3",
        53,
        Down,
        "1.0582492714614418",
        "0x1.0ee96c9bf1560#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Up,
        "1.0582492714614420",
        "0x1.0ee96c9bf1561#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Floor,
        "1.0582492714614418",
        "0x1.0ee96c9bf1560#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Ceiling,
        "1.0582492714614420",
        "0x1.0ee96c9bf1561#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Nearest,
        "1.0582492714614420",
        "0x1.0ee96c9bf1561#53",
        Greater,
    );
    test(
        "-1/3",
        100,
        Nearest,
        "1.0582492714614419014595217776412",
        "0x1.0ee96c9bf15609d47a83b4e70#100",
        Greater,
    );
    test("3/5", 1, Down, "1.0", "0x1.0#1", Less);
    test("3/5", 1, Up, "2.0", "0x2.0#1", Greater);
    test("3/5", 1, Floor, "1.0", "0x1.0#1", Less);
    test("3/5", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("3/5", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("3/5", 10, Down, "1.2109", "0x1.360#10", Less);
    test("3/5", 10, Up, "1.2129", "0x1.368#10", Greater);
    test("3/5", 10, Floor, "1.2109", "0x1.360#10", Less);
    test("3/5", 10, Ceiling, "1.2129", "0x1.368#10", Greater);
    test("3/5", 10, Nearest, "1.2109", "0x1.360#10", Less);
    test(
        "3/5",
        53,
        Down,
        "1.2116283145123166",
        "0x1.362d45f1bcedb#53",
        Less,
    );
    test(
        "3/5",
        53,
        Up,
        "1.2116283145123168",
        "0x1.362d45f1bcedc#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Floor,
        "1.2116283145123166",
        "0x1.362d45f1bcedb#53",
        Less,
    );
    test(
        "3/5",
        53,
        Ceiling,
        "1.2116283145123168",
        "0x1.362d45f1bcedc#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Nearest,
        "1.2116283145123168",
        "0x1.362d45f1bcedc#53",
        Greater,
    );
    test(
        "3/5",
        100,
        Nearest,
        "1.2116283145123167046145551213725",
        "0x1.362d45f1bcedba44e6a865214#100",
        Greater,
    );
    test("22/7", 1, Down, "-1.0", "-0x1.0#1", Greater);
    test("22/7", 1, Up, "-2.0", "-0x2.0#1", Less);
    test("22/7", 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("22/7", 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("22/7", 1, Nearest, "-1.0", "-0x1.0#1", Greater);
    test("22/7", 5, Nearest, "-1.00", "-0x1.0#5", Greater);
    test("22/7", 10, Down, "-1.0000", "-0x1.000#10", Greater);
    test("22/7", 10, Up, "-1.0020", "-0x1.008#10", Less);
    test("22/7", 10, Floor, "-1.0020", "-0x1.008#10", Less);
    test("22/7", 10, Ceiling, "-1.0000", "-0x1.000#10", Greater);
    test("22/7", 10, Nearest, "-1.0000", "-0x1.000#10", Greater);
    test("22/7", 20, Nearest, "-1.0000000", "-0x1.00000#20", Greater);
    test(
        "22/7",
        53,
        Down,
        "-1.0000007994670861",
        "-0x1.00000d69af5b7#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Up,
        "-1.0000007994670863",
        "-0x1.00000d69af5b8#53",
        Less,
    );
    test(
        "22/7",
        53,
        Floor,
        "-1.0000007994670863",
        "-0x1.00000d69af5b8#53",
        Less,
    );
    test(
        "22/7",
        53,
        Ceiling,
        "-1.0000007994670861",
        "-0x1.00000d69af5b7#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Nearest,
        "-1.0000007994670863",
        "-0x1.00000d69af5b8#53",
        Less,
    );
    test(
        "22/7",
        100,
        Nearest,
        "-1.0000007994670862438426744016045",
        "-0x1.00000d69af5b7b3e72a647ea0#100",
        Less,
    );
    test("-22/7", 1, Down, "-1.0", "-0x1.0#1", Greater);
    test("-22/7", 1, Up, "-2.0", "-0x2.0#1", Less);
    test("-22/7", 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("-22/7", 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("-22/7", 1, Nearest, "-1.0", "-0x1.0#1", Greater);
    test("-22/7", 5, Nearest, "-1.00", "-0x1.0#5", Greater);
    test("-22/7", 10, Down, "-1.0000", "-0x1.000#10", Greater);
    test("-22/7", 10, Up, "-1.0020", "-0x1.008#10", Less);
    test("-22/7", 10, Floor, "-1.0020", "-0x1.008#10", Less);
    test("-22/7", 10, Ceiling, "-1.0000", "-0x1.000#10", Greater);
    test("-22/7", 10, Nearest, "-1.0000", "-0x1.000#10", Greater);
    test("-22/7", 20, Nearest, "-1.0000000", "-0x1.00000#20", Greater);
    test(
        "-22/7",
        53,
        Down,
        "-1.0000007994670861",
        "-0x1.00000d69af5b7#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Up,
        "-1.0000007994670863",
        "-0x1.00000d69af5b8#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Floor,
        "-1.0000007994670863",
        "-0x1.00000d69af5b8#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Ceiling,
        "-1.0000007994670861",
        "-0x1.00000d69af5b7#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Nearest,
        "-1.0000007994670863",
        "-0x1.00000d69af5b8#53",
        Less,
    );
    test(
        "-22/7",
        100,
        Nearest,
        "-1.0000007994670862438426744016045",
        "-0x1.00000d69af5b7b3e72a647ea0#100",
        Less,
    );
    test("355/113", 1, Down, "-1.0", "-0x1.0#1", Greater);
    test("355/113", 1, Up, "-2.0", "-0x2.0#1", Less);
    test("355/113", 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("355/113", 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("355/113", 1, Nearest, "-1.0", "-0x1.0#1", Greater);
    test("355/113", 5, Nearest, "-1.00", "-0x1.0#5", Greater);
    test("355/113", 10, Down, "-1.0000", "-0x1.000#10", Greater);
    test("355/113", 10, Up, "-1.0020", "-0x1.008#10", Less);
    test("355/113", 10, Floor, "-1.0020", "-0x1.008#10", Less);
    test("355/113", 10, Ceiling, "-1.0000", "-0x1.000#10", Greater);
    test("355/113", 10, Nearest, "-1.0000", "-0x1.000#10", Greater);
    test(
        "355/113",
        20,
        Nearest,
        "-1.0000000",
        "-0x1.00000#20",
        Greater,
    );
    test(
        "355/113",
        53,
        Down,
        "-1.0000000000000355",
        "-0x1.00000000000a0#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Up,
        "-1.0000000000000357",
        "-0x1.00000000000a1#53",
        Less,
    );
    test(
        "355/113",
        53,
        Floor,
        "-1.0000000000000357",
        "-0x1.00000000000a1#53",
        Less,
    );
    test(
        "355/113",
        53,
        Ceiling,
        "-1.0000000000000355",
        "-0x1.00000000000a0#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Nearest,
        "-1.0000000000000355",
        "-0x1.00000000000a0#53",
        Greater,
    );
    test(
        "355/113",
        100,
        Nearest,
        "-1.0000000000000355815662830669527",
        "-0x1.00000000000a03ec0c05edd10#100",
        Greater,
    );
    test("3", 1, Down, "-1.0", "-0x1.0#1", Greater);
    test("3", 1, Up, "-2.0", "-0x2.0#1", Less);
    test("3", 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("3", 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("3", 1, Nearest, "-1.0", "-0x1.0#1", Greater);
    test("3", 5, Nearest, "-1.00", "-0x1.0#5", Greater);
    test("3", 10, Down, "-1.0098", "-0x1.028#10", Greater);
    test("3", 10, Up, "-1.0117", "-0x1.030#10", Less);
    test("3", 10, Floor, "-1.0117", "-0x1.030#10", Less);
    test("3", 10, Ceiling, "-1.0098", "-0x1.028#10", Greater);
    test("3", 10, Nearest, "-1.0098", "-0x1.028#10", Greater);
    test("3", 20, Nearest, "-1.0101089", "-0x1.02968#20", Less);
    test(
        "3",
        53,
        Down,
        "-1.0101086659079936",
        "-0x1.02967b457b245#53",
        Greater,
    );
    test(
        "3",
        53,
        Up,
        "-1.0101086659079939",
        "-0x1.02967b457b246#53",
        Less,
    );
    test(
        "3",
        53,
        Floor,
        "-1.0101086659079939",
        "-0x1.02967b457b246#53",
        Less,
    );
    test(
        "3",
        53,
        Ceiling,
        "-1.0101086659079936",
        "-0x1.02967b457b245#53",
        Greater,
    );
    test(
        "3",
        53,
        Nearest,
        "-1.0101086659079936",
        "-0x1.02967b457b245#53",
        Greater,
    );
    test(
        "3",
        100,
        Nearest,
        "-1.0101086659079937513030364814625",
        "-0x1.02967b457b2457eb66a45a26e#100",
        Greater,
    );
    test("100", 1, Down, "1.0", "0x1.0#1", Less);
    test("100", 1, Up, "2.0", "0x2.0#1", Greater);
    test("100", 1, Floor, "1.0", "0x1.0#1", Less);
    test("100", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("100", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("100", 5, Nearest, "1.19", "0x1.3#5", Greater);
    test("100", 10, Down, "1.1582", "0x1.288#10", Less);
    test("100", 10, Up, "1.1602", "0x1.290#10", Greater);
    test("100", 10, Floor, "1.1582", "0x1.288#10", Less);
    test("100", 10, Ceiling, "1.1602", "0x1.290#10", Greater);
    test("100", 10, Nearest, "1.1602", "0x1.290#10", Greater);
    test("100", 20, Nearest, "1.1596642", "0x1.28dfc#20", Greater);
    test(
        "100",
        53,
        Down,
        "1.1596638229046938",
        "0x1.28dfba71bae10#53",
        Less,
    );
    test(
        "100",
        53,
        Up,
        "1.1596638229046941",
        "0x1.28dfba71bae11#53",
        Greater,
    );
    test(
        "100",
        53,
        Floor,
        "1.1596638229046938",
        "0x1.28dfba71bae10#53",
        Less,
    );
    test(
        "100",
        53,
        Ceiling,
        "1.1596638229046941",
        "0x1.28dfba71bae11#53",
        Greater,
    );
    test(
        "100",
        53,
        Nearest,
        "1.1596638229046938",
        "0x1.28dfba71bae10#53",
        Less,
    );
    test(
        "100",
        100,
        Nearest,
        "1.1596638229046938325514044465873",
        "0x1.28dfba71bae100230e587db36#100",
        Greater,
    );
    test("1000000", 1, Down, "1.0", "0x1.0#1", Less);
    test("1000000", 1, Up, "2.0", "0x2.0#1", Greater);
    test("1000000", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1000000", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1000000", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1000000", 5, Nearest, "1.06", "0x1.1#5", Less);
    test("1000000", 10, Down, "1.0664", "0x1.110#10", Less);
    test("1000000", 10, Up, "1.0684", "0x1.118#10", Greater);
    test("1000000", 10, Floor, "1.0664", "0x1.110#10", Less);
    test("1000000", 10, Ceiling, "1.0684", "0x1.118#10", Greater);
    test("1000000", 10, Nearest, "1.0684", "0x1.118#10", Greater);
    test("1000000", 20, Nearest, "1.0675182", "0x1.1148e#20", Less);
    test(
        "1000000",
        53,
        Down,
        "1.0675182586811016",
        "0x1.1148e068eb0f2#53",
        Less,
    );
    test(
        "1000000",
        53,
        Up,
        "1.0675182586811018",
        "0x1.1148e068eb0f3#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Floor,
        "1.0675182586811016",
        "0x1.1148e068eb0f2#53",
        Less,
    );
    test(
        "1000000",
        53,
        Ceiling,
        "1.0675182586811018",
        "0x1.1148e068eb0f3#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Nearest,
        "1.0675182586811016",
        "0x1.1148e068eb0f2#53",
        Less,
    );
    test(
        "1000000",
        100,
        Nearest,
        "1.0675182586811016714376468019258",
        "0x1.1148e068eb0f23e8426db0e6a#100",
        Greater,
    );
    test("1/1000000", 1, Down, "1.0", "0x1.0#1", Less);
    test("1/1000000", 1, Up, "2.0", "0x2.0#1", Greater);
    test("1/1000000", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1/1000000", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1/1000000", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/1000000", 5, Nearest, "1.00", "0x1.0#5", Less);
    test("1/1000000", 10, Down, "1.0000", "0x1.000#10", Less);
    test("1/1000000", 10, Up, "1.0020", "0x1.008#10", Greater);
    test("1/1000000", 10, Floor, "1.0000", "0x1.000#10", Less);
    test("1/1000000", 10, Ceiling, "1.0020", "0x1.008#10", Greater);
    test("1/1000000", 10, Nearest, "1.0000", "0x1.000#10", Less);
    test("1/1000000", 20, Nearest, "1.0000000", "0x1.00000#20", Less);
    test(
        "1/1000000",
        53,
        Down,
        "1.0000000000004998",
        "0x1.00000000008cb#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Up,
        "1.0000000000005000",
        "0x1.00000000008cc#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Floor,
        "1.0000000000004998",
        "0x1.00000000008cb#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Ceiling,
        "1.0000000000005000",
        "0x1.00000000008cc#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Nearest,
        "1.0000000000005000",
        "0x1.00000000008cc#53",
        Greater,
    );
    test(
        "1/1000000",
        100,
        Nearest,
        "1.0000000000005000000000002083328",
        "0x1.00000000008cbccc096f9102a#100",
        Less,
    );
    test("-1/1000000", 1, Down, "1.0", "0x1.0#1", Less);
    test("-1/1000000", 1, Up, "2.0", "0x2.0#1", Greater);
    test("-1/1000000", 1, Floor, "1.0", "0x1.0#1", Less);
    test("-1/1000000", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("-1/1000000", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("-1/1000000", 5, Nearest, "1.00", "0x1.0#5", Less);
    test("-1/1000000", 10, Down, "1.0000", "0x1.000#10", Less);
    test("-1/1000000", 10, Up, "1.0020", "0x1.008#10", Greater);
    test("-1/1000000", 10, Floor, "1.0000", "0x1.000#10", Less);
    test("-1/1000000", 10, Ceiling, "1.0020", "0x1.008#10", Greater);
    test("-1/1000000", 10, Nearest, "1.0000", "0x1.000#10", Less);
    test("-1/1000000", 20, Nearest, "1.0000000", "0x1.00000#20", Less);
    test(
        "-1/1000000",
        53,
        Down,
        "1.0000000000004998",
        "0x1.00000000008cb#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Up,
        "1.0000000000005000",
        "0x1.00000000008cc#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Floor,
        "1.0000000000004998",
        "0x1.00000000008cb#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Ceiling,
        "1.0000000000005000",
        "0x1.00000000008cc#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Nearest,
        "1.0000000000005000",
        "0x1.00000000008cc#53",
        Greater,
    );
    test(
        "-1/1000000",
        100,
        Nearest,
        "1.0000000000005000000000002083328",
        "0x1.00000000008cbccc096f9102a#100",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        5,
        Nearest,
        "1.00",
        "0x1.0#5",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Down,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Up,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Down,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Up,
        "1.0000000000000002",
        "0x1.0000000000001#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Floor,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Ceiling,
        "1.0000000000000002",
        "0x1.0000000000001#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
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
        "-2.5e30",
        "-0x2.0E+25#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Up,
        "-5.1e30",
        "-0x4.0E+25#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Floor,
        "-5.1e30",
        "-0x4.0E+25#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Ceiling,
        "-2.5e30",
        "-0x2.0E+25#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Nearest,
        "-5.1e30",
        "-0x4.0E+25#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        5,
        Nearest,
        "-3.96e30",
        "-0x3.2E+25#5",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Down,
        "-3.9267e30",
        "-0x3.19E+25#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Up,
        "-3.9317e30",
        "-0x3.1aE+25#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Floor,
        "-3.9317e30",
        "-0x3.1aE+25#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Ceiling,
        "-3.9267e30",
        "-0x3.19E+25#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Nearest,
        "-3.9317e30",
        "-0x3.1aE+25#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        20,
        Nearest,
        "-3.9315477e30",
        "-0x3.19f84E+25#20",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Down,
        "-3.9315465872457520e30",
        "-0x3.19f831d43fda2E+25#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Up,
        "-3.9315465872457525e30",
        "-0x3.19f831d43fda4E+25#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Floor,
        "-3.9315465872457525e30",
        "-0x3.19f831d43fda4E+25#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Ceiling,
        "-3.9315465872457520e30",
        "-0x3.19f831d43fda2E+25#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Nearest,
        "-3.9315465872457520e30",
        "-0x3.19f831d43fda2E+25#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        100,
        Nearest,
        "-3931546587245752026277396765264.0",
        "-0x319f831d43fda2449ebb925a50.0#100",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Down,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Up,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Floor,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Ceiling,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        5,
        Nearest,
        "-1.00",
        "-0x1.0#5",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Down,
        "-1.0000",
        "-0x1.000#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Up,
        "-1.0020",
        "-0x1.008#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Floor,
        "-1.0020",
        "-0x1.008#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Ceiling,
        "-1.0000",
        "-0x1.000#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        20,
        Nearest,
        "-1.0000000",
        "-0x1.00000#20",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Down,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Up,
        "-1.0000000000000002",
        "-0x1.0000000000001#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Floor,
        "-1.0000000000000002",
        "-0x1.0000000000001#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Ceiling,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        100,
        Nearest,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Down,
        "1.0e31",
        "0x8.0E+25#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Up,
        "2.0e31",
        "0x1.0E+26#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Floor,
        "1.0e31",
        "0x8.0E+25#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Ceiling,
        "2.0e31",
        "0x1.0E+26#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Nearest,
        "1.0e31",
        "0x8.0E+25#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        5,
        Nearest,
        "1.20e31",
        "0x9.8E+25#5",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Down,
        "1.1785e31",
        "0x9.4cE+25#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "1.1805e31",
        "0x9.50E+25#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "1.1785e31",
        "0x9.4cE+25#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "1.1805e31",
        "0x9.50E+25#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "1.1785e31",
        "0x9.4cE+25#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "1.1794648e31",
        "0x9.4de9E+25#20",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "1.1794639761737254e31",
        "0x9.4de8957cbf8e0E+25#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "1.1794639761737256e31",
        "0x9.4de8957cbf8e8E+25#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "1.1794639761737254e31",
        "0x9.4de8957cbf8e0E+25#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "1.1794639761737256e31",
        "0x9.4de8957cbf8e8E+25#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "1.1794639761737256e31",
        "0x9.4de8957cbf8e8E+25#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "11794639761737256078832190295792.0",
        "0x9.4de8957cbf8e6cddc32b70efE+25#100",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Down,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Up,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Floor,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        5,
        Nearest,
        "2.00",
        "0x2.0#5",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Down,
        "2.0430",
        "0x2.0b#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Up,
        "2.0469",
        "0x2.0c#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Floor,
        "2.0430",
        "0x2.0b#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Ceiling,
        "2.0469",
        "0x2.0c#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Nearest,
        "2.0430",
        "0x2.0b#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        20,
        Nearest,
        "2.0442429",
        "0x2.0b538#20",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Down,
        "2.0442429074571735",
        "0x2.0b5380d09bc7a#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Up,
        "2.0442429074571740",
        "0x2.0b5380d09bc7c#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Floor,
        "2.0442429074571735",
        "0x2.0b5380d09bc7a#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Ceiling,
        "2.0442429074571740",
        "0x2.0b5380d09bc7c#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Nearest,
        "2.0442429074571735",
        "0x2.0b5380d09bc7a#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        100,
        Nearest,
        "2.0442429074571736614847672609166",
        "0x2.0b5380d09bc7aa3fcf0162600#100",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        5,
        Nearest,
        "1.00",
        "0x1.0#5",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Down,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Up,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Down,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Up,
        "1.0000000000000002",
        "0x1.0000000000001#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Floor,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Ceiling,
        "1.0000000000000002",
        "0x1.0000000000001#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Less,
    );
}

// A `Rational` far below the target precision but above the bottom of the exponent range, where
// sec(x) = 1 + x^2/2 + ..., a correction beneath the last bit of the result: `sec_rational_helper`
// answers 1 itself, nudged up, instead of forming the bracket [1 + x^2/2, 1 + x^2/2 + x^4] exactly
// -- for these inputs a dense `Rational` of hundreds of millions of bits, 14 seconds a call before
// that branch existed. Where x is dyadic it is exactly a `Float`, so the `Float` secant, reached
// through independent code, must agree.
#[test]
fn test_sec_rational_tiny() {
    let test =
        |x: Rational, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
            let (c, o) = Float::sec_rational_prec_round_ref(&x, prec, rm);
            assert!(c.is_valid());
            assert_eq!(c.to_string(), out);
            assert_eq!(to_hex_string(&c), out_hex);
            assert_eq!(o, o_out);
            if let Ok(f) = Float::try_from(&x) {
                let (c_alt, o_alt) = f.sec_prec_round(prec, rm);
                assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
                assert_eq!(o_alt, o);
            }
        };
    test(
        Rational::power_of_2(-536870908i64),
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        Rational::power_of_2(-536870908i64),
        53,
        Floor,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        Rational::power_of_2(-536870908i64),
        53,
        Ceiling,
        "1.0000000000000002",
        "0x1.0000000000001#53",
        Greater,
    );
    test(
        -Rational::power_of_2(-536870908i64),
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        Rational::power_of_2(-536870912i64) / Rational::from(3u32),
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        Rational::power_of_2(-1000i64),
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        Rational::power_of_2(-1000i64),
        10,
        Up,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn sec_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (s, o) = Float::sec_rational_prec_round(x.clone(), prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = Float::sec_rational_prec_round_ref(&x, prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    // sec is even
    let (s_neg, o_neg) = Float::sec_rational_prec_round(-&x, prec, rm);
    assert_eq!(ComparableFloatRef(&s_neg), ComparableFloatRef(&s));
    assert_eq!(o_neg, o);

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_o) = rug_sec_rational_prec_round(&x, prec, rrm);
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
        // only sec(0) = 1 is exact
        assert_eq!(x, 0u32);
        assert_eq!(
            ComparableFloatRef(&s),
            ComparableFloatRef(&Float::one_prec(prec))
        );
        for rm in exhaustive_rounding_modes() {
            let (s2, oo) = Float::sec_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::sec_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn sec_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        sec_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // sec(0) = 1, exactly
        let (s, o) = Float::sec_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn sec_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (s, o) = Float::sec_rational_prec(x.clone(), prec);
    assert!(s.is_valid());

    let (s_alt, o_alt) = Float::sec_rational_prec_ref(&x, prec);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let (s_alt, o_alt) = Float::sec_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let (rug_s, rug_o) = rug_sec_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_s)),
        ComparableFloatRef(&s)
    );
    assert_eq!(rug_o, o, "x = {x} prec = {prec}");

    // the secant of an exactly representable rational is the Float secant
    if let Ok(f) = Float::try_from(&x) {
        let (s_alt, o_alt) = f.sec_prec(prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
    }
}

#[test]
fn sec_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        sec_rational_prec_properties_helper(x, prec);
    });
}

// An input too large to be a `Float`, reduced modulo 2 pi in `Rational` arithmetic with pi to about
// 2^30 bits; slow even in release mode. Slow enough to dominate a test run, and its result does not
// depend on the limb width, so it runs only in the 64-bit-limb configuration.
#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_sec_rational_huge() {
    let x = Rational::power_of_2(1i64 << 30);
    let (t, o) = Float::sec_rational_prec_round_ref(&x, 10, Nearest);
    assert_eq!(t.to_string(), "-1.2852");
    assert_eq!(to_hex_string(&t), "-0x1.490#10");
    assert_eq!(o, Less);
}

// An input within 2^(-2^30) of an odd multiple of pi/2, whose secant overflows. The call computes
// pi to about 2^30 bits, so this test is slow even in release mode. Slow enough to dominate a test
// run, and its result does not depend on the limb width, so it runs only in the 64-bit-limb
// configuration.
#[cfg(not(feature = "32_bit_limbs"))]
#[test]
fn test_sec_rational_overflow() {
    let p = (1u64 << 30) + 64;
    // just below pi/2, where the cosine is positive and tiny
    let pi = Float::pi_prec_round(p, Floor).0;
    let x = Rational::exact_from(&(pi >> 1u32));
    let (s, o) = Float::sec_rational_prec_round_ref(&x, 10, Down);
    assert_eq!(
        ComparableFloat(s),
        ComparableFloat(Float::max_finite_value_with_prec(10))
    );
    assert_eq!(o, Less);
    let (s, o) = Float::sec_rational_prec_round_ref(&x, 10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sec_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_sec_rational::<T>(&x)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 1.0);
    test::<f32>("1", 1.8508158);
    test::<f32>("-1", 1.8508158);
    test::<f32>("1/2", 1.139494);
    test::<f32>("1/3", 1.0582492);
    test::<f32>("22/7", -1.0000008);
    test::<f32>("1/7", 1.0102916);
    test::<f32>("100", 1.1596638);
    test::<f32>("355/113", -1.0);
    test::<f32>("1/1000000", 1.0);
    test::<f32>("-2/3", 1.2724471);
    test::<f32>("1000000", 1.0675182);
    test::<f32>("1/100000000000000000000", 1.0);
    test::<f64>("0", 1.0);
    test::<f64>("1", 1.8508157176809257);
    test::<f64>("-1", 1.8508157176809257);
    test::<f64>("1/2", 1.139493927324549);
    test::<f64>("1/3", 1.058249271461442);
    test::<f64>("22/7", -1.0000007994670863);
    test::<f64>("1/7", 1.0102915771696053);
    test::<f64>("100", 1.1596638229046938);
    test::<f64>("355/113", -1.0000000000000355);
    test::<f64>("1/1000000", 1.0000000000005);
    test::<f64>("-2/3", 1.2724471433871758);
    test::<f64>("1000000", 1.0675182586811016);
    test::<f64>("1/100000000000000000000", 1.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sec_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let s = primitive_float_sec_rational::<T>(&x);
        // the secant of a rational is never NaN
        assert!(!s.is_nan());
        // sec is even
        assert_eq!(
            NiceFloat(primitive_float_sec_rational::<T>(&-&x)),
            NiceFloat(s)
        );
        // the result is the correctly rounded secant, as computed by MPFR with 64 bits to spare, so
        // that a subnormal result is rounded once by the conversion rather than twice
        let rug_s = rug_sec_rational_prec(&x, T::MANTISSA_WIDTH + 64).0;
        let rug_s: T = T::rounding_from(&<Float as From<&rug::Float>>::from(&rug_s), Nearest).0;
        assert_eq!(NiceFloat(rug_s), NiceFloat(s));
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // The secant of a finite nonzero primitive float, taken through the `Rational` path,
        // matches the direct primitive-float secant (a `Rational` cannot carry the sign of a zero).
        if x.is_finite() && x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_sec_rational::<T>(&Rational::exact_from(x))),
                NiceFloat(primitive_float_sec(x))
            );
        }
    });
}

#[test]
fn primitive_float_sec_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sec_rational_properties_helper);
}

// Rows reuse the tangent's `with_period` inputs, which cover the exact cases (half, quarter, and
// eighth turns), the closed forms at thirds, sixths, and twelfths of a turn, the general Ziv loop
// at its first working precision and after a retry, and the tiny x/u whose secant is 1 to far more
// bits than any precision needs.
#[test]
fn test_sec_with_period_prec_round() {
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

        let (t, o) = x.clone().sec_with_period_prec_round(u, prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.sec_with_period_prec_round_ref(u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.sec_with_period_prec_round_assign(u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = x.sec_with_period_prec_ref(u, prec);
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
        "0.0",
        "0x0.0",
        4,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "-0.0",
        "-0x0.0",
        4,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test("1.0", "0x1.0#1", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("NaN", "NaN", 0, 10, Nearest, "NaN", "NaN", Equal);
    test(
        "0.0",
        "0x0.0",
        360,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        360,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        360,
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
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
        "180.0",
        "0xb4.0#6",
        360,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "180.0",
        "0xb4.0#6",
        360,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "180.0",
        "0xb4.0#6",
        360,
        10,
        Exact,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Nearest,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Floor,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Exact,
        "Infinity",
        "Infinity",
        Equal,
    );
    test(
        "360.0",
        "0x168.0#6",
        360,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "360.0",
        "0x168.0#6",
        360,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "360.0",
        "0x168.0#6",
        360,
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "60.0",
        "0x3c.0#4",
        360,
        10,
        Nearest,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    test(
        "60.0",
        "0x3c.0#4",
        360,
        10,
        Floor,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Nearest,
        "-2.0000",
        "-0x2.00#10",
        Equal,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Floor,
        "-2.0000",
        "-0x2.00#10",
        Equal,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Nearest,
        "-2.0000",
        "-0x2.00#10",
        Equal,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Floor,
        "-2.0000",
        "-0x2.00#10",
        Equal,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Nearest,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Floor,
        "2.0000",
        "0x2.00#10",
        Equal,
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
    test(
        "1.0",
        "0x1.0#1",
        1,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        3,
        10,
        Nearest,
        "-2.0000",
        "-0x2.00#10",
        Equal,
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
        "1.6035",
        "0x1.9a8#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        360,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        1000000,
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        1,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "0.50", "0x0.8#1", 2, 53, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        4,
        10,
        Floor,
        "1.4141",
        "0x1.6a0#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        6,
        53,
        Nearest,
        "1.1547005383792515",
        "0x1.279a74590331c#53",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        12,
        10,
        Ceiling,
        "1.0371",
        "0x1.098#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        1000000,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
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
        "1.4160",
        "0x1.6a8#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        4,
        10,
        Nearest,
        "1.0820",
        "0x1.150#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        6,
        10,
        Ceiling,
        "1.0371",
        "0x1.098#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        12,
        10,
        Floor,
        "1.0078",
        "0x1.020#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        1000000,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        1099511627776,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "1.5", "0x1.8#2", 2, 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        3,
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
    );
    test(
        "1.5", "0x1.8#2", 6, 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        12,
        10,
        Nearest,
        "1.4141",
        "0x1.6a0#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        360,
        53,
        Nearest,
        "1.0003427924908679",
        "0x1.0016771a899fa#53",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        1099511627776,
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        2,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        3,
        10,
        Ceiling,
        "-2.0000",
        "-0x2.00#10",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        6,
        10,
        Nearest,
        "-2.0000",
        "-0x2.00#10",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        7,
        10,
        Ceiling,
        "-4.4922",
        "-0x4.7e#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        1099511627776,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        1,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        3,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0", "0x3.0#2", 4, 53, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        7,
        10,
        Floor,
        "-1.1113",
        "-0x1.1c8#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        360,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        1099511627776,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        3,
        10,
        Nearest,
        "-2.0000",
        "-0x2.00#10",
        Equal,
    );
    test(
        "-1.0", "-0x1.0#1", 4, 10, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        7,
        10,
        Nearest,
        "1.6035",
        "0x1.9a8#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        360,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1000000,
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        1,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "100.0",
        "0x64.0#5",
        2,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "100.0",
        "0x64.0#5",
        4,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "100.0",
        "0x64.0#5",
        6,
        53,
        Nearest,
        "-2.0000000000000000",
        "-0x2.0000000000000#53",
        Equal,
    );
    test(
        "100.0",
        "0x64.0#5",
        12,
        10,
        Ceiling,
        "-2.0000",
        "-0x2.00#10",
        Equal,
    );
    test(
        "100.0",
        "0x64.0#5",
        1000000,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        10,
        Nearest,
        "-1.0391",
        "-0x1.0a0#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        2,
        10,
        Ceiling,
        "-7.2500",
        "-0x7.40#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        4,
        10,
        Nearest,
        "1.5234",
        "0x1.860#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        6,
        10,
        Ceiling,
        "-1.1250",
        "-0x1.200#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        12,
        10,
        Floor,
        "-4.2344",
        "-0x4.3c#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1000000,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1099511627776,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        2,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        3,
        53,
        Nearest,
        "-2.0000000000000000",
        "-0x2.0000000000000#53",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        6,
        10,
        Floor,
        "-2.0000",
        "-0x2.00#10",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        12,
        10,
        Nearest,
        "-2.0000",
        "-0x2.00#10",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        360,
        53,
        Nearest,
        "5.7587704831436337",
        "0x5.c23ec84a45a38#53",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1099511627776,
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        2,
        10,
        Nearest,
        "-1.4141",
        "-0x1.6a0#10",
        Greater,
    );
    test(
        "-0.75", "-0x0.c#2", 3, 10, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        6,
        10,
        Nearest,
        "1.4141",
        "0x1.6a0#10",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        7,
        10,
        Ceiling,
        "1.2793",
        "0x1.478#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        360,
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1099511627776,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        3,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        4,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        7,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        360,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1099511627776,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        1,
        10,
        Ceiling,
        "1.4160",
        "0x1.6a8#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        3,
        10,
        Nearest,
        "1.0352",
        "0x1.090#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        4,
        10,
        Ceiling,
        "1.0215",
        "0x1.058#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        7,
        10,
        Nearest,
        "1.0059",
        "0x1.018#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        360,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        1000000,
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        1,
        53,
        Floor,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "-90.000000000000000000000000000808",
        "-0x5a.000000000000000000000040#100",
        360,
        53,
        Nearest,
        "-7.0928739541311617e28",
        "-0xe.52ee0d31e0fc0E+23#53",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        8,
        10,
        Ceiling,
        "1.4160",
        "0x1.6a8#10",
        Greater,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        8,
        10,
        Nearest,
        "-1.4141",
        "-0x1.6a0#10",
        Greater,
    );
    test("5.0", "0x5.0#3", 8, 1, Nearest, "-1.0", "-0x1.0#1", Greater);
    test("5.0", "0x5.0#3", 8, 10, Up, "-1.4160", "-0x1.6a8#10", Less);
    test(
        "7.0",
        "0x7.0#3",
        8,
        10,
        Ceiling,
        "1.4160",
        "0x1.6a8#10",
        Greater,
    );
    test(
        "-9.00",
        "-0x9.0#4",
        8,
        10,
        Nearest,
        "1.4141",
        "0x1.6a0#10",
        Less,
    );
    test("1.0", "0x1.0#1", 12, 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Up,
        "1.1562",
        "0x1.280#10",
        Greater,
    );
    test(
        "5.0",
        "0x5.0#3",
        12,
        10,
        Ceiling,
        "-1.1543",
        "-0x1.278#10",
        Greater,
    );
    test(
        "-7.0",
        "-0x7.0#3",
        12,
        10,
        Nearest,
        "-1.1543",
        "-0x1.278#10",
        Greater,
    );
    test("11.0", "0xb.0#4", 12, 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "11.0",
        "0xb.0#4",
        12,
        10,
        Up,
        "1.1562",
        "0x1.280#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        10,
        Ceiling,
        "3.2383",
        "0x3.3d#10",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        5,
        10,
        Nearest,
        "-1.2363",
        "-0x1.3c8#10",
        Less,
    );
    test("3.0", "0x3.0#2", 5, 1, Nearest, "-1.0", "-0x1.0#1", Greater);
    test("3.0", "0x3.0#2", 5, 10, Up, "-1.2363", "-0x1.3c8#10", Less);
    test(
        "4.0",
        "0x4.0#1",
        5,
        10,
        Ceiling,
        "3.2383",
        "0x3.3d#10",
        Greater,
    );
    test(
        "-6.0",
        "-0x6.0#2",
        5,
        10,
        Nearest,
        "3.2344",
        "0x3.3c#10",
        Less,
    );
    test("1.0", "0x1.0#1", 10, 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Up,
        "1.2363",
        "0x1.3c8#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        10,
        Ceiling,
        "-3.2344",
        "-0x3.3c#10",
        Greater,
    );
    test(
        "-7.0",
        "-0x7.0#3",
        10,
        10,
        Nearest,
        "-3.2344",
        "-0x3.3c#10",
        Greater,
    );
    test("9.00", "0x9.0#4", 10, 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "9.00",
        "0x9.0#4",
        10,
        10,
        Up,
        "1.2363",
        "0x1.3c8#10",
        Greater,
    );
    test(
        "45.0",
        "0x2d.0#6",
        360,
        10,
        Ceiling,
        "1.4160",
        "0x1.6a8#10",
        Greater,
    );
    test(
        "-135.0",
        "-0x87.0#8",
        360,
        10,
        Nearest,
        "-1.4141",
        "-0x1.6a0#10",
        Greater,
    );
    test("30.0", "0x1e.0#4", 360, 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "30.0",
        "0x1e.0#4",
        360,
        10,
        Up,
        "1.1562",
        "0x1.280#10",
        Greater,
    );
    test(
        "150.0",
        "0x96.0#7",
        360,
        10,
        Ceiling,
        "-1.1543",
        "-0x1.278#10",
        Greater,
    );
    test(
        "-72.0",
        "-0x48.0#4",
        360,
        10,
        Nearest,
        "3.2344",
        "0x3.3c#10",
        Less,
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
        "-1.2363",
        "-0x1.3c8#10",
        Less,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        10,
        Ceiling,
        "1.2363",
        "0x1.3c8#10",
        Greater,
    );
    test(
        "-108.0",
        "-0x6c.0#5",
        360,
        10,
        Nearest,
        "-3.2344",
        "-0x3.3c#10",
        Greater,
    );
    test("2.0", "0x2.0#1", 16, 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "2.0",
        "0x2.0#1",
        16,
        10,
        Up,
        "1.4160",
        "0x1.6a8#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        24,
        10,
        Ceiling,
        "1.4160",
        "0x1.6a8#10",
        Greater,
    );
    test(
        "-4.0",
        "-0x4.0#1",
        20,
        10,
        Nearest,
        "3.2344",
        "0x3.3c#10",
        Less,
    );
    test("6.0", "0x6.0#2", 60, 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "6.0",
        "0x6.0#2",
        60,
        10,
        Up,
        "1.2363",
        "0x1.3c8#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        20,
        30,
        Ceiling,
        "1.7013016175",
        "0x1.b38880b8#30",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        20,
        30,
        Ceiling,
        "-1.0514622238",
        "-0x1.0d2ca0d8#30",
        Greater,
    );
    test(
        "13.0",
        "0xd.0#4",
        20,
        30,
        Ceiling,
        "-1.7013016157",
        "-0x1.b38880b0#30",
        Greater,
    );
    test(
        "19.0",
        "0x13.0#5",
        20,
        30,
        Ceiling,
        "1.0514622256",
        "0x1.0d2ca0e0#30",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        6,
        30,
        Floor,
        "2.0000000000",
        "0x2.0000000#30",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        8,
        30,
        Nearest,
        "1.4142135624",
        "0x1.6a09e668#30",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#2",
        4,
        10,
        Exact,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        100,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        100,
        10,
        Down,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        4,
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    // fifths and tenths of a turn, where the cosine is a multiple of the golden ratio
    test(
        "1.0",
        "0x1.0#1",
        5,
        10,
        Nearest,
        "3.2344",
        "0x3.3c#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        20,
        Nearest,
        "3.2360687",
        "0x3.3c6f0#20",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        20,
        Floor,
        "3.2360649",
        "0x3.3c6ec#20",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        20,
        Ceiling,
        "3.2360687",
        "0x3.3c6f0#20",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        5,
        20,
        Nearest,
        "-1.2360687",
        "-0x1.3c6f0#20",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        5,
        20,
        Nearest,
        "-1.2360687",
        "-0x1.3c6f0#20",
        Less,
    );
    test(
        "4.0",
        "0x4.0#1",
        5,
        20,
        Nearest,
        "3.2360687",
        "0x3.3c6f0#20",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        5,
        20,
        Nearest,
        "3.2360687",
        "0x3.3c6f0#20",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        5,
        20,
        Nearest,
        "-1.2360687",
        "-0x1.3c6f0#20",
        Less,
    );
    test(
        "6.0",
        "0x6.0#2",
        5,
        20,
        Nearest,
        "3.2360687",
        "0x3.3c6f0#20",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Nearest,
        "1.2363",
        "0x1.3c8#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        20,
        Nearest,
        "1.2360687",
        "0x1.3c6f0#20",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        20,
        Floor,
        "1.2360668",
        "0x1.3c6ee#20",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        20,
        Ceiling,
        "1.2360687",
        "0x1.3c6f0#20",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        20,
        Nearest,
        "-3.2360687",
        "-0x3.3c6f0#20",
        Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        10,
        20,
        Nearest,
        "-3.2360687",
        "-0x3.3c6f0#20",
        Less,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        20,
        Nearest,
        "1.2360687",
        "0x1.3c6f0#20",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        20,
        Nearest,
        "1.2360687",
        "0x1.3c6f0#20",
        Greater,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        10,
        20,
        Nearest,
        "-3.2360687",
        "-0x3.3c6f0#20",
        Less,
    );
    test(
        "11.0",
        "0xb.0#4",
        10,
        20,
        Nearest,
        "1.2360687",
        "0x1.3c6f0#20",
        Greater,
    );
    test(
        "72.0",
        "0x48.0#4",
        360,
        20,
        Nearest,
        "3.2360687",
        "0x3.3c6f0#20",
        Greater,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        20,
        Nearest,
        "1.2360687",
        "0x1.3c6f0#20",
        Greater,
    );
    test(
        "108.0",
        "0x6c.0#5",
        360,
        20,
        Nearest,
        "-3.2360687",
        "-0x3.3c6f0#20",
        Less,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        20,
        Nearest,
        "-1.2360687",
        "-0x1.3c6f0#20",
        Less,
    );
}

#[test]
#[should_panic]
fn sec_with_period_prec_round_fail_1() {
    Float::ONE.sec_with_period_prec_round(4, 0, Floor);
}

#[test]
#[should_panic]
fn sec_with_period_prec_round_fail_2() {
    Float::from_unsigned_prec(1u32, 10)
        .0
        .sec_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn sec_with_period_prec_round_ref_fail() {
    Float::ONE.sec_with_period_prec_round_ref(4, 0, Floor);
}

#[test]
#[should_panic]
fn sec_with_period_prec_fail() {
    Float::ONE.sec_with_period_prec(4, 0);
}

#[test]
#[should_panic]
fn sec_with_period_round_fail() {
    Float::from_unsigned_prec(1u32, 10)
        .0
        .sec_with_period_round(7, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn sec_with_period_prec_round_properties_helper(x: Float, u: u64, prec: u64, rm: RoundingMode) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (t, o) = x.sec_with_period_prec_round_ref(u, prec, Nearest);
        if o == Equal {
            let (te, oe) = x.sec_with_period_prec_round_ref(u, prec, Exact);
            assert_eq!(ComparableFloatRef(&te), ComparableFloatRef(&t));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(x.sec_with_period_prec_round_ref(u, prec, Exact));
        }
        return;
    }
    let (t, o) = x.clone().sec_with_period_prec_round(u, prec, rm);
    assert!(t.is_valid());
    assert_rounding_ordering_consistent(&t, rm, o);

    let (t_alt, o_alt) = x.sec_with_period_prec_round_ref(u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    let mut t_alt = x.clone();
    let o_alt = t_alt.sec_with_period_prec_round_assign(u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    // sec_with_period is NaN exactly for u = 0 and non-finite x, and otherwise at least 1 in
    // magnitude
    assert_eq!(t.is_nan(), u == 0 || !x.is_finite());
    if !t.is_nan() {
        assert!(PartialOrdAbs::ge_abs(&t, &1u32));
        // an infinity is a pole, and exact
        if t.is_infinite() {
            assert_eq!(o, Equal);
        } else if t.is_normal() {
            assert_eq!(t.get_prec(), Some(prec));
        }
        // sec_with_period is even
        let (t_neg, o_neg) = (-&x).sec_with_period_prec_round(u, prec, rm);
        assert_eq!(ComparableFloatRef(&t_neg), ComparableFloatRef(&t));
        assert_eq!(o_neg, o);
        // sec_with_period has period u
        if x.is_finite() && u != 0 {
            let (shifted, os) =
                x.add_prec_round_ref_val(Float::from(u), x.significant_bits() + 64, Nearest);
            if os == Equal {
                let (t_shifted, o_shifted) = shifted.sec_with_period_prec_round(u, prec, rm);
                assert_eq!(ComparableFloatRef(&t_shifted), ComparableFloatRef(&t));
                assert_eq!(o_shifted, o);
            }
        }
        // the secant is the reciprocal of the cosine in the same units
        if x.is_finite()
            && u != 0
            && let Some((t_alt, o_alt)) = sec_with_period_naive(&x, u, prec, rm)
        {
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (t2, oo) = x.sec_with_period_prec_round_ref(u, prec, rm);
            assert_eq!(ComparableFloatRef(&t2), ComparableFloatRef(&t));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.sec_with_period_prec_round_ref(u, prec, Exact));
    }
}

#[test]
fn sec_with_period_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, u, prec, rm)| {
            sec_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_18().test_properties(
        |(x, u, prec, rm)| {
            sec_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // sec_with_period(±0) = 1 and sec_with_period(x, 0) = NaN, exactly
        let (t, o) = Float::ZERO.sec_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o, Equal);
        let (t, o) = Float::NEGATIVE_ZERO.sec_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o, Equal);
        let (t, o) = Float::ONE.sec_with_period_prec_round(0, prec, rm);
        assert!(t.is_nan());
        assert_eq!(o, Equal);
        // exact cases: quarter turns are the secant's ±1 values and its poles
        for (k, expected) in [
            (0u32, Float::one_prec(prec)),
            (1, Float::INFINITY),
            (2, -Float::one_prec(prec)),
            (3, Float::INFINITY),
            (4, Float::one_prec(prec)),
        ] {
            let (t, o) = Float::from(k).sec_with_period_prec_round(4, prec, rm);
            assert_eq!(ComparableFloat(t), ComparableFloat(expected));
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn sec_with_period_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, u, prec)| {
        let (t, o) = x.clone().sec_with_period_prec(u, prec);
        assert!(t.is_valid());
        let (t_alt, o_alt) = x.sec_with_period_prec_ref(u, prec);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.sec_with_period_prec_round_ref(u, prec, Nearest);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.sec_with_period_prec_assign(u, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn sec_with_period_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_39().test_properties(|(x, u, rm)| {
        if rm == Exact && x.sec_with_period_round_ref(u, Nearest).1 != Equal {
            assert_panic!(x.sec_with_period_round_ref(u, Exact));
            return;
        }
        let (t, o) = x.clone().sec_with_period_round(u, rm);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = x.sec_with_period_round_ref(u, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.sec_with_period_prec_round_ref(u, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.sec_with_period_round_assign(u, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn sec_with_period_properties() {
    float_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, u)| {
        let t = x.clone().sec_with_period(u);
        assert!(t.is_valid());
        let t_alt = x.sec_with_period_ref(u);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let mut t_alt = x.clone();
        t_alt.sec_with_period_assign(u);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let t_alt = x
            .sec_with_period_prec_round_ref(u, x.significant_bits(), Nearest)
            .0;
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        // sec_with_period is even
        assert_eq!(
            ComparableFloatRef(&(-&x).sec_with_period_ref(u)),
            ComparableFloatRef(&t)
        );
    });
}

// Inputs within 2^(-2^30) of a quarter turn, whose secants overflow. The near-zero path works with
// the exact distance to the multiple of 1/4, so no 2^30-bit pi is ever formed, but the inputs
// themselves have 2^30 bits.
#[test]
fn test_sec_with_period_overflow() {
    let max = Float::max_finite_value_with_prec(10);
    let eps = Rational::power_of_2(-((1i64 << 30) + 70));
    let p = (1u64 << 30) + 74;
    // just past a quarter turn: x/u = 1/4 + 2^(-2^30 - 72), so the cosine is negative and tiny and
    // the secant is negative and huge, beyond the largest finite `Float`
    let above = Float::from_rational_prec_round(Rational::ONE + &eps, p, Exact).0;
    let (s, o) = above.sec_with_period_prec_round_ref(4, 10, Nearest);
    assert_eq!(
        ComparableFloat(s),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    let (s, o) = above.sec_with_period_prec_round_ref(4, 10, Up);
    assert_eq!(
        ComparableFloat(s),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    let (s, o) = above.sec_with_period_prec_round_ref(4, 10, Ceiling);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&-max.clone()));
    assert_eq!(o, Greater);
    // just below a quarter turn: the cosine is positive and tiny, so the secant is positive and
    // huge
    let below = Float::from_rational_prec_round(Rational::ONE - eps, p, Exact).0;
    let (s, o) = below.sec_with_period_prec_round_ref(4, 10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (s, o) = below.sec_with_period_prec_round_ref(4, 10, Up);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (s, o) = below.sec_with_period_prec_round_ref(4, 10, Down);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&max));
    assert_eq!(o, Less);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sec_with_period() {
    fn test<T: PrimitiveFloat>(x: T, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_sec_with_period(x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 360, f32::NAN);
    test::<f32>(f32::INFINITY, 360, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, 360, f32::NAN);
    test::<f32>(1.0, 0, f32::NAN);
    test::<f32>(0.0, 360, 1.0);
    test::<f32>(-0.0, 360, 1.0);
    test::<f32>(90.0, 360, f32::INFINITY);
    test::<f32>(-90.0, 360, f32::INFINITY);
    test::<f32>(270.0, 360, f32::INFINITY);
    test::<f32>(180.0, 360, -1.0);
    test::<f32>(-180.0, 360, -1.0);
    test::<f32>(360.0, 360, 1.0);
    test::<f32>(45.0, 360, core::f32::consts::SQRT_2);
    test::<f32>(135.0, 360, -core::f32::consts::SQRT_2);
    test::<f32>(30.0, 360, 1.1547005);
    test::<f32>(60.0, 360, 2.0);
    test::<f32>(120.0, 360, -2.0);
    test::<f32>(1.0, 7, 1.6038755);
    test::<f32>(-1.0, 7, 1.6038755);
    test::<f32>(2.0, 7, -4.4939594);
    test::<f32>(1.0, 360, 1.0001523);
    test::<f32>(100.0, 360, -5.7587705);
    test::<f32>(10000000000.0, 360, 5.7587705);
    test::<f32>(1.0e30, 7, 1.6038755);
    test::<f32>(1.0e-30, 7, 1.0);
    test::<f32>(3.4028235e38, 360, 1.0);
    test::<f32>(0.5, 1, -1.0);
    test::<f32>(0.25, 1, f32::INFINITY);
    test::<f32>(0.1, 1, 1.236068);
    test::<f32>(1.0e-45, 1, 1.0);
    test::<f32>(1.0e-45, 360, 1.0);
    test::<f64>(f64::NAN, 360, f64::NAN);
    test::<f64>(f64::INFINITY, 360, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, 360, f64::NAN);
    test::<f64>(1.0, 0, f64::NAN);
    test::<f64>(0.0, 360, 1.0);
    test::<f64>(-0.0, 360, 1.0);
    test::<f64>(90.0, 360, f64::INFINITY);
    test::<f64>(-90.0, 360, f64::INFINITY);
    test::<f64>(270.0, 360, f64::INFINITY);
    test::<f64>(180.0, 360, -1.0);
    test::<f64>(-180.0, 360, -1.0);
    test::<f64>(360.0, 360, 1.0);
    test::<f64>(45.0, 360, core::f64::consts::SQRT_2);
    test::<f64>(135.0, 360, -core::f64::consts::SQRT_2);
    test::<f64>(30.0, 360, 1.1547005383792515);
    test::<f64>(60.0, 360, 2.0);
    test::<f64>(120.0, 360, -2.0);
    test::<f64>(1.0, 7, 1.6038754716096766);
    test::<f64>(-1.0, 7, 1.6038754716096766);
    test::<f64>(2.0, 7, -4.493959207434934);
    test::<f64>(1.0, 360, 1.0001523280439077);
    test::<f64>(100.0, 360, -5.758770483143634);
    test::<f64>(10000000000.0, 360, 5.758770483143634);
    test::<f64>(1.0e100, 7, -4.493959207434934);
    test::<f64>(1.0e-100, 7, 1.0);
    test::<f64>(1.7976931348623157e308, 360, -1.624269245482744);
    test::<f64>(0.5, 1, -1.0);
    test::<f64>(0.25, 1, f64::INFINITY);
    test::<f64>(0.1, 1, 1.2360679774997898);
    test::<f64>(5.0e-324, 1, 1.0);
    test::<f64>(5.0e-324, 360, 1.0);
    test::<f32>(72.0, 360, 3.236068);
    test::<f32>(36.0, 360, 1.236068);
    test::<f32>(108.0, 360, -3.236068);
    test::<f32>(144.0, 360, -1.236068);
    test::<f64>(72.0, 360, 3.23606797749979);
    test::<f64>(36.0, 360, 1.2360679774997898);
    test::<f64>(108.0, 360, -3.23606797749979);
    test::<f64>(144.0, 360, -1.2360679774997898);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sec_with_period_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        let s = primitive_float_sec_with_period(x, u);
        // NaN exactly for u = 0 (the inputs are finite)
        assert_eq!(s.is_nan(), u == 0);
        if u != 0 {
            // the secant is at least 1 in magnitude, so it never underflows
            assert!(s.abs() >= T::ONE);
            // even
            assert_eq!(
                NiceFloat(primitive_float_sec_with_period(-x, u)),
                NiceFloat(s)
            );
            // the same as the `Float` secant taken with 64 bits to spare and rounded once
            let s_float = Float::sec_with_period_prec(Float::from(x), u, T::MANTISSA_WIDTH + 64).0;
            assert_eq!(
                NiceFloat(T::rounding_from(&s_float, Nearest).0),
                NiceFloat(s)
            );
            // an infinity is a pole, and no f32 or f64 is merely close enough to one to overflow
            assert_eq!(s.is_infinite(), s_float.is_infinite());
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // NaN exactly for NaN and infinite inputs
        assert_eq!(
            primitive_float_sec_with_period(x, 7).is_nan(),
            !x.is_finite()
        );
    });
}

#[test]
fn primitive_float_sec_with_period_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sec_with_period_properties_helper);
}

#[test]
fn test_sec_with_period_rational_prec_round() {
    let test = |s: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (t, o) = Float::sec_with_period_rational_prec_round(x.clone(), u, prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = Float::sec_with_period_rational_prec_round_ref(&x, u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = Float::sec_with_period_rational_prec(x.clone(), u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) = Float::sec_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        // the secant is the reciprocal of the cosine in the same units
        if let Some((t_alt, o_alt)) = sec_with_period_rational_naive(&x, u, prec, rm) {
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    };
    test("0", 4, 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("0", 4, 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("0", 4, 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("0", 4, 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("0", 4, 10, Exact, "1.0000", "0x1.000#10", Equal);
    test(
        "0",
        4,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
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
    test("180", 360, 1, Nearest, "-1.0", "-0x1.0#1", Equal);
    test("180", 360, 10, Nearest, "-1.0000", "-0x1.000#10", Equal);
    test("180", 360, 10, Floor, "-1.0000", "-0x1.000#10", Equal);
    test("180", 360, 10, Ceiling, "-1.0000", "-0x1.000#10", Equal);
    test("180", 360, 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test(
        "180",
        360,
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
    );
    test("270", 360, 1, Nearest, "Infinity", "Infinity", Equal);
    test("270", 360, 10, Nearest, "Infinity", "Infinity", Equal);
    test("270", 360, 10, Floor, "Infinity", "Infinity", Equal);
    test("270", 360, 10, Ceiling, "Infinity", "Infinity", Equal);
    test("270", 360, 10, Exact, "Infinity", "Infinity", Equal);
    test("270", 360, 53, Nearest, "Infinity", "Infinity", Equal);
    test("360", 360, 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("360", 360, 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("360", 360, 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("360", 360, 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("360", 360, 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("450", 360, 10, Ceiling, "Infinity", "Infinity", Equal);
    test("60", 360, 10, Floor, "2.0000", "0x2.00#10", Equal);
    test("120", 360, 10, Floor, "-2.0000", "-0x2.00#10", Equal);
    test("240", 360, 10, Floor, "-2.0000", "-0x2.00#10", Equal);
    test("300", 360, 10, Floor, "2.0000", "0x2.00#10", Equal);
    test("45", 360, 10, Floor, "1.4141", "0x1.6a0#10", Less);
    test("135", 360, 10, Floor, "-1.4160", "-0x1.6a8#10", Less);
    test("30", 360, 10, Floor, "1.1543", "0x1.278#10", Less);
    test("150", 360, 10, Floor, "-1.1562", "-0x1.280#10", Less);
    test("72", 360, 10, Floor, "3.2344", "0x3.3c#10", Less);
    test("144", 360, 10, Floor, "-1.2363", "-0x1.3c8#10", Less);
    test("36", 360, 10, Floor, "1.2344", "0x1.3c0#10", Less);
    test("108", 360, 10, Floor, "-3.2383", "-0x3.3d#10", Less);
    test("1/3", 1, 10, Floor, "-2.0000", "-0x2.00#10", Equal);
    test("1/8", 1, 10, Floor, "1.4141", "0x1.6a0#10", Less);
    test("1/5", 1, 10, Floor, "3.2344", "0x3.3c#10", Less);
    test("-1/10", 1, 10, Floor, "1.2344", "0x1.3c0#10", Less);
    test("1/7", 1, 10, Floor, "1.6035", "0x1.9a8#10", Less);
    test("2/7", 1, 10, Floor, "-4.5000", "-0x4.80#10", Less);
    test("-3/7", 1, 10, Floor, "-1.1113", "-0x1.1c8#10", Less);
    test("22/7", 1, 10, Floor, "1.6035", "0x1.9a8#10", Less);
    test("1/7", 3, 10, Floor, "1.0449", "0x1.0b8#10", Less);
    test("355/113", 360, 10, Floor, "1.0000", "0x1.000#10", Less);
    test("1", 7, 10, Floor, "1.6035", "0x1.9a8#10", Less);
    test("1000000", 7, 10, Floor, "1.6035", "0x1.9a8#10", Less);
    test("1/1000000", 1, 10, Floor, "1.0000", "0x1.000#10", Less);
    test(
        "1/1000000000000000000000000000000",
        1,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
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
        "1.7013016175",
        "0x1.b38880b8#30",
        Greater,
    );
    test(
        "11/20",
        1,
        30,
        Floor,
        "-1.0514622256",
        "-0x1.0d2ca0e0#30",
        Less,
    );
    test(
        "17/20",
        1,
        30,
        Nearest,
        "1.7013016175",
        "0x1.b38880b8#30",
        Greater,
    );
    test(
        "-1/3",
        1,
        30,
        Nearest,
        "-2.0000000000",
        "-0x2.0000000#30",
        Equal,
    );
    test(
        "3/8",
        1,
        30,
        Nearest,
        "-1.4142135624",
        "-0x1.6a09e668#30",
        Less,
    );
    test("1/2", 1, 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test("-5", 1, 10, Exact, "1.0000", "0x1.000#10", Equal);
    test(
        "-1/1000000000000000000000000000000",
        7,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    // fifths and tenths of a turn, the poles, and a half turn
    test("1", 5, 20, Nearest, "3.2360687", "0x3.3c6f0#20", Greater);
    test("2", 5, 20, Nearest, "-1.2360687", "-0x1.3c6f0#20", Less);
    test("1", 10, 20, Nearest, "1.2360687", "0x1.3c6f0#20", Greater);
    test("3", 10, 20, Nearest, "-3.2360687", "-0x3.3c6f0#20", Less);
    test("1", 4, 20, Nearest, "Infinity", "Infinity", Equal);
    test("3", 4, 20, Nearest, "Infinity", "Infinity", Equal);
    test("1", 2, 20, Nearest, "-1.0000000", "-0x1.00000#20", Equal);
    test("1/5", 1, 20, Nearest, "3.2360687", "0x3.3c6f0#20", Greater);
    test("2/5", 1, 20, Nearest, "-1.2360687", "-0x1.3c6f0#20", Less);
    test("1/10", 1, 20, Nearest, "1.2360687", "0x1.3c6f0#20", Greater);
    test("-1/5", 1, 20, Nearest, "3.2360687", "0x3.3c6f0#20", Greater);
    test("7/5", 1, 20, Nearest, "-1.2360687", "-0x1.3c6f0#20", Less);
}

#[test]
#[should_panic]
fn sec_with_period_rational_prec_round_fail_1() {
    Float::sec_with_period_rational_prec_round(Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn sec_with_period_rational_prec_round_fail_2() {
    Float::sec_with_period_rational_prec_round(Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn sec_with_period_rational_prec_round_fail_3() {
    // a twelfth of a turn is sqrt(3)/3, which is not exact
    Float::sec_with_period_rational_prec_round(Rational::from_unsigneds(1u8, 12), 1, 10, Exact);
}

#[test]
#[should_panic]
fn sec_with_period_rational_prec_round_ref_fail() {
    Float::sec_with_period_rational_prec_round_ref(&Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn sec_with_period_rational_prec_fail() {
    Float::sec_with_period_rational_prec(Rational::ONE, 7, 0);
}

#[allow(clippy::needless_pass_by_value)]
fn sec_with_period_rational_prec_round_properties_helper(
    x: Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (t, o) = Float::sec_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
        if o == Equal {
            let (te, oe) = Float::sec_with_period_rational_prec_round_ref(&x, u, prec, Exact);
            assert_eq!(ComparableFloatRef(&te), ComparableFloatRef(&t));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(Float::sec_with_period_rational_prec_round_ref(
                &x, u, prec, Exact
            ));
        }
        return;
    }
    let (t, o) = Float::sec_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(t.is_valid());
    assert_rounding_ordering_consistent(&t, rm, o);

    let (t_alt, o_alt) = Float::sec_with_period_rational_prec_round_ref(&x, u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    // MPFR has no secu, so the oracle is the reciprocal of a wider cosine, bracketed
    if let Some((t_alt, o_alt)) = sec_with_period_rational_naive(&x, u, prec, rm) {
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
        // an infinity is a pole, and exact
        if t.is_infinite() {
            assert_eq!(o, Equal);
        } else if t.is_normal() {
            assert_eq!(t.get_prec(), Some(prec));
        }
        // sec is even, and has period u
        let (t_neg, o_neg) = Float::sec_with_period_rational_prec_round(-&x, u, prec, rm);
        assert_eq!(ComparableFloatRef(&t_neg), ComparableFloatRef(&t));
        assert_eq!(o_neg, o);
        let (t_shifted, o_shifted) =
            Float::sec_with_period_rational_prec_round(&x + Rational::from(u), u, prec, rm);
        assert_eq!(ComparableFloatRef(&t_shifted), ComparableFloatRef(&t));
        assert_eq!(o_shifted, o);
        // a `Float` input agrees with the `Float` version
        if let Ok(f) = Float::try_from(&x) {
            let (t_alt, o_alt) = f.sec_with_period_prec_round(u, prec, rm);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (t2, oo) = Float::sec_with_period_rational_prec_round_ref(&x, u, prec, rm);
            assert_eq!(ComparableFloatRef(&t2), ComparableFloatRef(&t));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::sec_with_period_rational_prec_round_ref(
            &x, u, prec, Exact
        ));
    }
}

#[test]
fn sec_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, u, prec, rm)| {
            sec_with_period_rational_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // sec(0) = 1, exactly
        let (t, o) = Float::sec_with_period_rational_prec_round(Rational::ZERO, 4, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o, Equal);
        let (t, o) = Float::sec_with_period_rational_prec_round(Rational::ONE, 0, prec, rm);
        assert!(t.is_nan());
        assert_eq!(o, Equal);
        // exact cases, straight from a fraction of a turn: the quarter turns are the zeros and the
        // poles, the half turns, and the thirds and sixths, which are exactly ±2
        let one = Float::one_prec(prec);
        let two = &one << 1u32;
        for (s, expected) in [
            ("1/4", Float::INFINITY),
            ("1/2", -&one),
            ("-1/2", -&one),
            ("3/4", Float::INFINITY),
            ("-1/4", Float::INFINITY),
            ("1", one.clone()),
            ("-1", one.clone()),
            ("1/6", two.clone()),
            ("5/6", two.clone()),
            ("1/3", -&two),
            ("-1/3", -&two),
        ] {
            let (t, o) = Float::sec_with_period_rational_prec_round(
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
fn sec_with_period_rational_prec_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, u, prec, _)| {
            let (t, o) = Float::sec_with_period_rational_prec(x.clone(), u, prec);
            assert!(t.is_valid());
            assert_rounding_ordering_consistent(&t, Nearest, o);
            let (t_alt, o_alt) = Float::sec_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) =
                Float::sec_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            if let Some((t_alt, o_alt)) = sec_with_period_rational_naive(&x, u, prec, Nearest) {
                assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
                assert_eq!(o_alt, o);
            }
        },
    );
}

// Fractions of a turn within 2^(-2^30) of a quarter turn, whose secants overflow. The near-zero
// path of the cosine works with the exact distance to the quarter turn, so no 2^30-bit pi is ever
// formed.
#[test]
fn test_sec_with_period_rational_overflow() {
    let max = Float::max_finite_value_with_prec(10);
    let eps = Rational::power_of_2(-((1i64 << 30) + 70));
    // just past a quarter turn: the cosine is negative and tiny, so the secant is negative and
    // beyond the largest finite `Float`
    let above = Rational::from_unsigneds(1u32, 4u32) + &eps;
    let (t, o) = Float::sec_with_period_rational_prec_round_ref(&above, 1, 10, Nearest);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    let (t, o) = Float::sec_with_period_rational_prec_round_ref(&above, 1, 10, Ceiling);
    assert_eq!(ComparableFloatRef(&t), ComparableFloatRef(&-max.clone()));
    assert_eq!(o, Greater);
    // just below a quarter turn: the secant is positive and beyond the largest finite `Float`
    let below = Rational::from_unsigneds(1u32, 4u32) - &eps;
    let (t, o) = Float::sec_with_period_rational_prec_round_ref(&below, 1, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (t, o) = Float::sec_with_period_rational_prec_round_ref(&below, 1, 10, Down);
    assert_eq!(ComparableFloatRef(&t), ComparableFloatRef(&max));
    assert_eq!(o, Less);
    // a non-dyadic version: 1/4 + 1/(3 * 2^(2^30 + 70)) of a turn
    let above = Rational::from_unsigneds(1u32, 4u32) + eps / Rational::from(3u32);
    let (t, o) = Float::sec_with_period_rational_prec_round_ref(&above, 1, 10, Nearest);
    assert_eq!(
        ComparableFloat(t),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sec_with_period_rational() {
    fn test<T: PrimitiveFloat>(s: &str, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_sec_with_period_rational::<T>(&x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0, f32::NAN);
    test::<f32>("0", 360, 1.0);
    test::<f32>("1", 0, f32::NAN);
    test::<f32>("90", 360, f32::INFINITY);
    test::<f32>("-90", 360, f32::INFINITY);
    test::<f32>("270", 360, f32::INFINITY);
    test::<f32>("180", 360, -1.0);
    test::<f32>("-180", 360, -1.0);
    test::<f32>("360", 360, 1.0);
    test::<f32>("45", 360, core::f32::consts::SQRT_2);
    test::<f32>("135", 360, -core::f32::consts::SQRT_2);
    test::<f32>("30", 360, 1.1547005);
    test::<f32>("60", 360, 2.0);
    test::<f32>("120", 360, -2.0);
    test::<f32>("72", 360, 3.236068);
    test::<f32>("36", 360, 1.236068);
    test::<f32>("1/4", 1, f32::INFINITY);
    test::<f32>("3/4", 1, f32::INFINITY);
    test::<f32>("1/2", 1, -1.0);
    test::<f32>("1/8", 1, core::f32::consts::SQRT_2);
    test::<f32>("1/12", 1, 1.1547005);
    test::<f32>("1/6", 1, 2.0);
    test::<f32>("1/3", 1, -2.0);
    test::<f32>("1/5", 1, 3.236068);
    test::<f32>("2/5", 1, -1.236068);
    test::<f32>("1/10", 1, 1.236068);
    test::<f32>("-1/5", 1, 3.236068);
    test::<f32>("1/7", 1, 1.6038755);
    test::<f32>("-2/7", 1, -4.4939594);
    test::<f32>("22/7", 1, 1.6038755);
    test::<f32>("1", 7, 1.6038755);
    test::<f32>("1000000", 7, 1.6038755);
    test::<f32>("1/1000000", 1, 1.0);
    test::<f32>("355/113", 360, 1.0015051);
    test::<f64>("0", 0, f64::NAN);
    test::<f64>("0", 360, 1.0);
    test::<f64>("1", 0, f64::NAN);
    test::<f64>("90", 360, f64::INFINITY);
    test::<f64>("-90", 360, f64::INFINITY);
    test::<f64>("270", 360, f64::INFINITY);
    test::<f64>("180", 360, -1.0);
    test::<f64>("-180", 360, -1.0);
    test::<f64>("360", 360, 1.0);
    test::<f64>("45", 360, core::f64::consts::SQRT_2);
    test::<f64>("135", 360, -core::f64::consts::SQRT_2);
    test::<f64>("30", 360, 1.1547005383792515);
    test::<f64>("60", 360, 2.0);
    test::<f64>("120", 360, -2.0);
    test::<f64>("72", 360, 3.23606797749979);
    test::<f64>("36", 360, 1.2360679774997898);
    test::<f64>("1/4", 1, f64::INFINITY);
    test::<f64>("3/4", 1, f64::INFINITY);
    test::<f64>("1/2", 1, -1.0);
    test::<f64>("1/8", 1, core::f64::consts::SQRT_2);
    test::<f64>("1/12", 1, 1.1547005383792515);
    test::<f64>("1/6", 1, 2.0);
    test::<f64>("1/3", 1, -2.0);
    test::<f64>("1/5", 1, 3.23606797749979);
    test::<f64>("2/5", 1, -1.2360679774997898);
    test::<f64>("1/10", 1, 1.2360679774997898);
    test::<f64>("-1/5", 1, 3.23606797749979);
    test::<f64>("1/7", 1, 1.6038754716096766);
    test::<f64>("-2/7", 1, -4.493959207434934);
    test::<f64>("22/7", 1, 1.6038754716096766);
    test::<f64>("1", 7, 1.6038754716096766);
    test::<f64>("1000000", 7, 1.6038754716096766);
    test::<f64>("1/1000000", 1, 1.000000000019739);
    test::<f64>("355/113", 360, 1.0015051123499814);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sec_with_period_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, u)| {
        let s = primitive_float_sec_with_period_rational::<T>(&x, u);
        assert_eq!(s.is_nan(), u == 0);
        if u != 0 {
            // the secant is at least 1 in magnitude, so it never underflows
            assert!(s.abs() >= T::ONE);
            // even, and periodic with period u
            assert_eq!(
                NiceFloat(primitive_float_sec_with_period_rational::<T>(&-&x, u)),
                NiceFloat(s)
            );
            assert_eq!(
                NiceFloat(primitive_float_sec_with_period_rational::<T>(
                    &(&x + Rational::from(u)),
                    u
                )),
                NiceFloat(s)
            );
            // the same as the `Float` secant taken with 64 bits to spare and rounded once
            let s_float = Float::sec_with_period_rational_prec_ref(&x, u, T::MANTISSA_WIDTH + 64).0;
            assert_eq!(
                NiceFloat(T::rounding_from(&s_float, Nearest).0),
                NiceFloat(s)
            );
        }
    });

    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        // The secant of a finite primitive float, taken through the `Rational` path, matches the
        // direct primitive-float secant.
        assert_eq!(
            NiceFloat(primitive_float_sec_with_period_rational::<T>(
                &Rational::exact_from(x),
                u
            )),
            NiceFloat(primitive_float_sec_with_period(x, u))
        );
    });
}

#[test]
fn primitive_float_sec_with_period_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sec_with_period_rational_properties_helper);
}

#[test]
fn test_sec_pi_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (t, o) = x.clone().sec_pi_prec_round(prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.sec_pi_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.sec_pi_prec_round_assign(prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = x.sec_pi_prec_ref(prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        // MPFR has no secpi, so the oracle is the reciprocal of a wider cosine, bracketed
        if let Some((t_alt, o_alt)) = sec_with_period_naive(&x, 2, prec, rm) {
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
    test("0.0", "0x0.0", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("0.0", "0x0.0", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("0.0", "0x0.0", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("0.0", "0x0.0", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("0.0", "0x0.0", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test(
        "0.0",
        "0x0.0",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test("0.50", "0x0.8#1", 1, Nearest, "Infinity", "Infinity", Equal);
    test(
        "0.50", "0x0.8#1", 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test("0.50", "0x0.8#1", 10, Floor, "Infinity", "Infinity", Equal);
    test(
        "0.50", "0x0.8#1", 10, Ceiling, "Infinity", "Infinity", Equal,
    );
    test("0.50", "0x0.8#1", 10, Exact, "Infinity", "Infinity", Equal);
    test(
        "0.50", "0x0.8#1", 53, Nearest, "Infinity", "Infinity", Equal,
    );
    test("1.5", "0x1.8#2", 1, Nearest, "Infinity", "Infinity", Equal);
    test("1.5", "0x1.8#2", 10, Nearest, "Infinity", "Infinity", Equal);
    test("1.5", "0x1.8#2", 10, Floor, "Infinity", "Infinity", Equal);
    test("1.5", "0x1.8#2", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("1.5", "0x1.8#2", 10, Exact, "Infinity", "Infinity", Equal);
    test("1.5", "0x1.8#2", 53, Nearest, "Infinity", "Infinity", Equal);
    test("1.0", "0x1.0#1", 1, Nearest, "-1.0", "-0x1.0#1", Equal);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test("1.0", "0x1.0#1", 10, Floor, "-1.0000", "-0x1.000#10", Equal);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test("1.0", "0x1.0#1", 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test(
        "1.0",
        "0x1.0#1",
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
    );
    test("2.0", "0x2.0#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("2.0", "0x2.0#1", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("2.0", "0x2.0#1", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("2.0", "0x2.0#1", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("2.0", "0x2.0#1", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test(
        "2.0",
        "0x2.0#1",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test("0.25", "0x0.4#1", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("0.25", "0x0.4#1", 10, Nearest, "1.4141", "0x1.6a0#10", Less);
    test("0.25", "0x0.4#1", 10, Floor, "1.4141", "0x1.6a0#10", Less);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Ceiling,
        "1.4160",
        "0x1.6a8#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        53,
        Nearest,
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Nearest,
        "1.0508",
        "0x1.0d0#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Floor,
        "1.0508",
        "0x1.0d0#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Ceiling,
        "1.0527",
        "0x1.0d8#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        53,
        Nearest,
        "1.0514622242382672",
        "0x1.0d2ca0da1530d#53",
        Less,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        1,
        Nearest,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        10,
        Nearest,
        "3.2344",
        "0x3.3c#10",
        Less,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        10,
        Floor,
        "3.2344",
        "0x3.3c#10",
        Less,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        10,
        Ceiling,
        "3.2383",
        "0x3.3d#10",
        Greater,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        53,
        Nearest,
        "3.2360679774997902",
        "0x3.3c6ef372fe952#53",
        Less,
    );
    test("100.2", "0x64.4#9", 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "100.2",
        "0x64.4#9",
        10,
        Nearest,
        "1.4141",
        "0x1.6a0#10",
        Less,
    );
    test("100.2", "0x64.4#9", 10, Floor, "1.4141", "0x1.6a0#10", Less);
    test(
        "100.2",
        "0x64.4#9",
        10,
        Ceiling,
        "1.4160",
        "0x1.6a8#10",
        Greater,
    );
    test(
        "100.2",
        "0x64.4#9",
        53,
        Nearest,
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Nearest,
        "1.0508",
        "0x1.0d0#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Floor,
        "1.0508",
        "0x1.0d0#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Ceiling,
        "1.0527",
        "0x1.0d8#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        53,
        Nearest,
        "1.0514622242382672",
        "0x1.0d2ca0da1530d#53",
        Less,
    );
    test("1.0", "0x1.0#1", 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Exact,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test("2.0", "0x2.0#2", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("2.0", "0x2.0#2", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("-2.0", "-0x2.0#2", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test(
        "-2.0",
        "-0x2.0#2",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test("0.50", "0x0.8#1", 10, Exact, "Infinity", "Infinity", Equal);
    test(
        "0.50", "0x0.8#1", 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.50", "-0x0.8#1", 10, Exact, "Infinity", "Infinity", Equal,
    );
    test(
        "-0.50", "-0x0.8#1", 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test("1.5", "0x1.8#2", 10, Exact, "Infinity", "Infinity", Equal);
    test("1.5", "0x1.8#2", 10, Nearest, "Infinity", "Infinity", Equal);
    test("-1.5", "-0x1.8#2", 10, Exact, "Infinity", "Infinity", Equal);
    test(
        "-1.5", "-0x1.8#2", 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        20,
        Nearest,
        "1.4142132",
        "0x1.6a09e#20",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        20,
        Nearest,
        "1.0823917",
        "0x1.1517a#20",
        Less,
    );
    test(
        "0.38",
        "0x0.6#2",
        20,
        Nearest,
        "2.6131248",
        "0x2.9cf5c#20",
        Less,
    );
    test(
        "0.62",
        "0x0.a#3",
        20,
        Nearest,
        "-2.6131248",
        "-0x2.9cf5c#20",
        Greater,
    );
    test(
        "1.2",
        "0x1.4#3",
        20,
        Nearest,
        "-1.4142132",
        "-0x1.6a09e#20",
        Greater,
    );
}

#[test]
fn test_sec_pi_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (t, o) = Float::sec_pi_rational_prec_round(x.clone(), prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = Float::sec_pi_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = Float::sec_pi_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) = Float::sec_pi_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    };
    test("0", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("0", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("0", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("0", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("0", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test(
        "0",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test("1/2", 1, Nearest, "Infinity", "Infinity", Equal);
    test("1/2", 10, Nearest, "Infinity", "Infinity", Equal);
    test("1/2", 10, Floor, "Infinity", "Infinity", Equal);
    test("1/2", 10, Ceiling, "Infinity", "Infinity", Equal);
    test("1/2", 10, Exact, "Infinity", "Infinity", Equal);
    test("1/2", 53, Nearest, "Infinity", "Infinity", Equal);
    test("1", 1, Nearest, "-1.0", "-0x1.0#1", Equal);
    test("1", 10, Nearest, "-1.0000", "-0x1.000#10", Equal);
    test("1", 10, Floor, "-1.0000", "-0x1.000#10", Equal);
    test("1", 10, Ceiling, "-1.0000", "-0x1.000#10", Equal);
    test("1", 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test(
        "1",
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
    );
    test("1/3", 1, Nearest, "2.0", "0x2.0#1", Equal);
    test("1/3", 10, Nearest, "2.0000", "0x2.00#10", Equal);
    test("1/3", 10, Floor, "2.0000", "0x2.00#10", Equal);
    test("1/3", 10, Ceiling, "2.0000", "0x2.00#10", Equal);
    test(
        "1/3",
        53,
        Nearest,
        "2.0000000000000000",
        "0x2.0000000000000#53",
        Equal,
    );
    test("2/3", 1, Nearest, "-2.0", "-0x2.0#1", Equal);
    test("2/3", 10, Nearest, "-2.0000", "-0x2.00#10", Equal);
    test("2/3", 10, Floor, "-2.0000", "-0x2.00#10", Equal);
    test("2/3", 10, Ceiling, "-2.0000", "-0x2.00#10", Equal);
    test(
        "2/3",
        53,
        Nearest,
        "-2.0000000000000000",
        "-0x2.0000000000000#53",
        Equal,
    );
    test("1/4", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/4", 10, Nearest, "1.4141", "0x1.6a0#10", Less);
    test("1/4", 10, Floor, "1.4141", "0x1.6a0#10", Less);
    test("1/4", 10, Ceiling, "1.4160", "0x1.6a8#10", Greater);
    test(
        "1/4",
        53,
        Nearest,
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Greater,
    );
    test("1/6", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/6", 10, Nearest, "1.1543", "0x1.278#10", Less);
    test("1/6", 10, Floor, "1.1543", "0x1.278#10", Less);
    test("1/6", 10, Ceiling, "1.1562", "0x1.280#10", Greater);
    test(
        "1/6",
        53,
        Nearest,
        "1.1547005383792515",
        "0x1.279a74590331c#53",
        Less,
    );
    test("1/5", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/5", 10, Nearest, "1.2363", "0x1.3c8#10", Greater);
    test("1/5", 10, Floor, "1.2344", "0x1.3c0#10", Less);
    test("1/5", 10, Ceiling, "1.2363", "0x1.3c8#10", Greater);
    test(
        "1/5",
        53,
        Nearest,
        "1.2360679774997898",
        "0x1.3c6ef372fe950#53",
        Greater,
    );
    test("2/5", 1, Nearest, "4.0", "0x4.0#1", Greater);
    test("2/5", 10, Nearest, "3.2344", "0x3.3c#10", Less);
    test("2/5", 10, Floor, "3.2344", "0x3.3c#10", Less);
    test("2/5", 10, Ceiling, "3.2383", "0x3.3d#10", Greater);
    test(
        "2/5",
        53,
        Nearest,
        "3.2360679774997898",
        "0x3.3c6ef372fe950#53",
        Greater,
    );
    test("1/7", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/7", 10, Nearest, "1.1094", "0x1.1c0#10", Less);
    test("1/7", 10, Floor, "1.1094", "0x1.1c0#10", Less);
    test("1/7", 10, Ceiling, "1.1113", "0x1.1c8#10", Greater);
    test(
        "1/7",
        53,
        Nearest,
        "1.1099162641747424",
        "0x1.1c2378e7edd3f#53",
        Greater,
    );
    test("-3/7", 1, Nearest, "4.0", "0x4.0#1", Less);
    test("-3/7", 10, Nearest, "4.4922", "0x4.7e#10", Less);
    test("-3/7", 10, Floor, "4.4922", "0x4.7e#10", Less);
    test("-3/7", 10, Ceiling, "4.5000", "0x4.80#10", Greater);
    test(
        "-3/7",
        53,
        Nearest,
        "4.4939592074349344",
        "0x4.7e741c517dba4#53",
        Greater,
    );
    test("22/7", 1, Nearest, "-1.0", "-0x1.0#1", Greater);
    test("22/7", 10, Nearest, "-1.1094", "-0x1.1c0#10", Greater);
    test("22/7", 10, Floor, "-1.1113", "-0x1.1c8#10", Less);
    test("22/7", 10, Ceiling, "-1.1094", "-0x1.1c0#10", Greater);
    test(
        "22/7",
        53,
        Nearest,
        "-1.1099162641747424",
        "-0x1.1c2378e7edd3f#53",
        Less,
    );
    test("1/1000000", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/1000000", 10, Nearest, "1.0000", "0x1.000#10", Less);
    test("1/1000000", 10, Floor, "1.0000", "0x1.000#10", Less);
    test("1/1000000", 10, Ceiling, "1.0020", "0x1.008#10", Greater);
    test(
        "1/1000000",
        53,
        Nearest,
        "1.0000000000049347",
        "0x1.00000000056d0#53",
        Less,
    );
    test("1", 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test("1", 10, Nearest, "-1.0000", "-0x1.000#10", Equal);
    test("-1", 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test("-1", 10, Nearest, "-1.0000", "-0x1.000#10", Equal);
    test("2", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("2", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("-2", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("-2", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("1/2", 10, Exact, "Infinity", "Infinity", Equal);
    test("1/2", 10, Nearest, "Infinity", "Infinity", Equal);
    test("-1/2", 10, Exact, "Infinity", "Infinity", Equal);
    test("-1/2", 10, Nearest, "Infinity", "Infinity", Equal);
    test("3/2", 10, Exact, "Infinity", "Infinity", Equal);
    test("3/2", 10, Nearest, "Infinity", "Infinity", Equal);
    test("1/6", 10, Nearest, "1.1543", "0x1.278#10", Less);
    test("5/6", 10, Nearest, "-1.1543", "-0x1.278#10", Greater);
    test("-1/6", 10, Nearest, "1.1543", "0x1.278#10", Less);
    test("1/3", 20, Nearest, "2.0000000", "0x2.00000#20", Equal);
    test("1/4", 20, Nearest, "1.4142132", "0x1.6a09e#20", Less);
    test("1/5", 20, Nearest, "1.2360687", "0x1.3c6f0#20", Greater);
    test("1/10", 20, Nearest, "1.0514622", "0x1.0d2ca#20", Less);
    test("3/10", 20, Nearest, "1.7013016", "0x1.b3888#20", Less);
    test("1/7", 20, Nearest, "1.1099167", "0x1.1c238#20", Greater);
}

// Every `sec_pi` variant is `sec_with_period` with a period of 2.
#[test]
fn sec_pi_properties() {
    // The borrowed generators admit `Exact` for inputs whose secant is not exact, so `Exact` is
    // checked against the exactness of the result.
    let exact_ok = |x: &Float, prec: u64, rm: RoundingMode| {
        rm != Exact || x.sec_with_period_prec_round_ref(2, prec, Nearest).1 == Equal
    };
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        if !exact_ok(&x, prec, rm) {
            assert_panic!(x.sec_pi_prec_round_ref(prec, Exact));
            return;
        }
        let (t, o) = x.clone().sec_pi_prec_round(prec, rm);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = x.sec_with_period_prec_round_ref(2, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.sec_pi_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.sec_pi_prec_round_assign(prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        if let Some((t_alt, o_alt)) = sec_with_period_naive(&x, 2, prec, rm) {
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    });

    float_unsigned_rounding_mode_triple_gen_var_37().test_properties(|(x, prec, rm)| {
        if !exact_ok(&x, prec, rm) {
            return;
        }
        let (t, o) = x.sec_pi_prec_round_ref(prec, rm);
        let (t_alt, o_alt) = x.sec_with_period_prec_round_ref(2, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (t, o) = x.clone().sec_pi_prec(prec);
        let (t_alt, o_alt) = x.sec_with_period_prec_ref(2, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.sec_pi_prec_ref(prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.sec_pi_prec_assign(prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });

    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        if !exact_ok(&x, x.significant_bits(), rm) {
            return;
        }
        let (t, o) = x.clone().sec_pi_round(rm);
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = x.sec_with_period_round_ref(2, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.sec_pi_round_ref(rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.sec_pi_round_assign(rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });

    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        if rm == Exact && Float::sec_with_period_rational_prec_ref(&x, 2, prec).1 != Equal {
            assert_panic!(Float::sec_pi_rational_prec_round_ref(&x, prec, Exact));
            return;
        }
        let (t, o) = Float::sec_pi_rational_prec_round(x.clone(), prec, rm);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = Float::sec_with_period_rational_prec_round_ref(&x, 2, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = Float::sec_pi_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        // the secant is the reciprocal of the cosine in the same units
        if let Some((t_alt, o_alt)) = sec_with_period_rational_naive(&x, 2, prec, rm) {
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (t, o) = Float::sec_pi_rational_prec(x.clone(), prec);
        let (t_alt, o_alt) = Float::sec_with_period_rational_prec_ref(&x, 2, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = Float::sec_pi_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });

    float_gen().test_properties(|x| {
        let t = x.clone().sec_pi();
        assert!(t.is_valid());
        let t_alt = x.sec_pi_ref();
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let mut t_alt = x.clone();
        t_alt.sec_pi_assign();
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        // the same as sec_with_period with a period of 2, and as rounding to the input't precision,
        // to nearest
        let t_alt = x.sec_with_period_ref(2);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let t_alt = x.sec_pi_prec_round_ref(x.significant_bits(), Nearest).0;
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sec_pi() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_sec_pi(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(0.5, f32::INFINITY);
    test::<f32>(1.0, -1.0);
    test::<f32>(-1.0, -1.0);
    test::<f32>(0.25, core::f32::consts::SQRT_2);
    test::<f32>(0.1, 1.0514622);
    test::<f32>(-0.1, 1.0514622);
    test::<f32>(100.25, core::f32::consts::SQRT_2);
    test::<f32>(10000000000.0, 1.0);
    test::<f32>(1.0e-45, 1.0);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(0.5, f64::INFINITY);
    test::<f64>(1.0, -1.0);
    test::<f64>(-1.0, -1.0);
    test::<f64>(0.25, core::f64::consts::SQRT_2);
    test::<f64>(0.1, 1.0514622242382672);
    test::<f64>(-0.1, 1.0514622242382672);
    test::<f64>(100.25, core::f64::consts::SQRT_2);
    test::<f64>(10000000000.0, 1.0);
    test::<f64>(5.0e-324, 1.0);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sec_pi_rational() {
    fn test<T: PrimitiveFloat>(t: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(t).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_sec_pi_rational::<T>(&x)),
            NiceFloat(out)
        );
    }
    test::<f32>("1/2", f32::INFINITY);
    test::<f32>("1", -1.0);
    test::<f32>("-1", -1.0);
    test::<f32>("1/6", 1.1547005);
    test::<f32>("1/3", 2.0);
    test::<f32>("1/4", core::f32::consts::SQRT_2);
    test::<f32>("1/7", 1.1099162);
    test::<f32>("-3/7", 4.4939594);
    test::<f32>("22/7", -1.1099162);
    test::<f64>("1/2", f64::INFINITY);
    test::<f64>("1", -1.0);
    test::<f64>("-1", -1.0);
    test::<f64>("1/6", 1.1547005383792515);
    test::<f64>("1/3", 2.0);
    test::<f64>("1/4", core::f64::consts::SQRT_2);
    test::<f64>("1/7", 1.1099162641747424);
    test::<f64>("-3/7", 4.493959207434934);
    test::<f64>("22/7", -1.1099162641747424);
}

#[test]
#[should_panic]
fn sec_pi_prec_round_fail_1() {
    Float::from(0.1f64).sec_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sec_pi_prec_round_fail_2() {
    Float::from(0.1f64).sec_pi_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn sec_pi_rational_prec_round_fail() {
    Float::sec_pi_rational_prec_round(Rational::from_unsigneds(1u8, 7), 10, Exact);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sec_pi_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        assert_eq!(
            NiceFloat(primitive_float_sec_pi(x)),
            NiceFloat(primitive_float_sec_with_period(x, 2))
        );
    });
    rational_gen().test_properties(|x| {
        assert_eq!(
            NiceFloat(primitive_float_sec_pi_rational::<T>(&x)),
            NiceFloat(primitive_float_sec_with_period_rational::<T>(&x, 2))
        );
    });
}

#[test]
fn primitive_float_sec_pi_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sec_pi_properties_helper);
}
