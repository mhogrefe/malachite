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
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::sec::{primitive_float_sec, primitive_float_sec_rational};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::sec::{
    rug_sec, rug_sec_prec, rug_sec_prec_round, rug_sec_rational_prec, rug_sec_rational_prec_round,
    rug_sec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
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

        let (s_alt, _) = x.sec_prec_round_ref(x.significant_bits(), Nearest);
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
#[test]
fn test_sec_overflow() {
    let p = (1u64 << 30) + 64;
    // just below pi/2, where the cosine is positive and tiny
    let half_pi = Float::pi_prec_round(p, Floor).0 >> 1u32;
    let (s, o) = half_pi.sec_prec_round_ref(10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (s, o) = half_pi.sec_prec_round_ref(10, Down);
    assert_eq!(
        ComparableFloat(s),
        ComparableFloat(Float::max_finite_value_with_prec(10))
    );
    assert_eq!(o, Less);
    let (s, o) = half_pi.sec_prec_round_ref(10, Up);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    // just above pi/2, where the cosine is negative and tiny
    let half_pi = Float::pi_prec_round(p, Ceiling).0 >> 1u32;
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
    let (s, o) = half_pi.sec_prec_round_ref(10, Floor);
    assert_eq!(
        ComparableFloat(s),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
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
// 2^30 bits; slow even in release mode.
#[test]
fn test_sec_rational_huge() {
    let x = Rational::power_of_2(1i64 << 30);
    let (t, o) = Float::sec_rational_prec_round_ref(&x, 10, Nearest);
    assert_eq!(t.to_string(), "-1.2852");
    assert_eq!(to_hex_string(&t), "-0x1.490#10");
    assert_eq!(o, Less);
}

// An input within 2^(-2^30) of an odd multiple of pi/2, whose secant overflows. The call computes
// pi to about 2^30 bits, so this test is slow even in release mode.
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
