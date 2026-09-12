// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Sec, SecAssign};
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
use malachite_float::float::arithmetic::sec::primitive_float_sec;
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::sec::{
    rug_sec, rug_sec_prec, rug_sec_prec_round, rug_sec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::panic::catch_unwind;

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
