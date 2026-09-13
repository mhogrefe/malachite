// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Csc, CscAssign};
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
use malachite_float::float::arithmetic::csc::primitive_float_csc;
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::csc::{
    rug_csc, rug_csc_prec, rug_csc_prec_round, rug_csc_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::panic::catch_unwind;

// Rows reuse the sine test's inputs. Branches of `csc_prec_round_normal_ref` covered:
// - tiny x: the small-input shortcut rounds x directly
// - the general Ziv loop, at the first working precision and after a retry
// - |x| near an odd multiple of pi/2: a large result, from the cosine's near-zero path inside
//   `sin_cos`
// - |x| near a nonzero multiple of pi: a tiny result, from the sine's near-zero path
#[test]
fn test_csc_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (t, o) = x.clone().csc_prec_round(prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.csc_prec_round_ref(prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.csc_prec_round_assign(prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) = rug_csc_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
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
        Less,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Floor,
        "4.0",
        "0x4.0#1",
        Less,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Ceiling,
        "8.0",
        "0x8.0#1",
        Greater,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        2,
        Nearest,
        "4.0",
        "0x4.0#2",
        Less,
    );
    test("0.25", "0x0.4#1", 1, Floor, "4.0", "0x4.0#1", Less);
    test("0.25", "0x0.4#1", 1, Nearest, "4.0", "0x4.0#1", Less);
    test("0.25", "0x0.4#1", 1, Ceiling, "8.0", "0x8.0#1", Greater);
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
    test("1.0", "0x1.0#1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", 10, Floor, "1.1875", "0x1.300#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "1.1895",
        "0x1.308#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 10, Nearest, "1.1875", "0x1.300#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        100,
        Floor,
        "1.1883951057781212162615994523744",
        "0x1.303aa9620b223e32e68b9bddc#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Ceiling,
        "1.1883951057781212162615994523760",
        "0x1.303aa9620b223e32e68b9bdde#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Nearest,
        "1.1883951057781212162615994523744",
        "0x1.303aa9620b223e32e68b9bddc#100",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-1.1875",
        "-0x1.300#10",
        Greater,
    );
    test("2.0", "0x2.0#1", 10, Nearest, "1.0996", "0x1.198#10", Less);
    test("3.0", "0x3.0#2", 10, Nearest, "7.0859", "0x7.16#10", Less);
    test(
        "4.0",
        "0x4.0#1",
        10,
        Nearest,
        "-1.3223",
        "-0x1.528#10",
        Less,
    );
    test(
        "4.0",
        "0x4.0#1",
        100,
        Nearest,
        "-1.3213487088109023776967917563728",
        "-0x1.5243e8b2f4641f913239810f2#100",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        10,
        Nearest,
        "-1.9746",
        "-0x1.f98#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Floor,
        "-1.9748575314240999612122645488032",
        "-0x1.f990435fb9ce22e1a4c3bf5a4#100",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Ceiling,
        "-1.9748575314240999612122645488016",
        "-0x1.f990435fb9ce22e1a4c3bf5a2#100",
        Greater,
    );
    test(
        "1.00000e6",
        "0xf.424E+4#14",
        64,
        Nearest,
        "-2.85719590162728929528",
        "-0x2.db7130cbc12306a8#64",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        50,
        Nearest,
        "2.0858296429334899",
        "0x2.15f8ee756d3c#50",
        Greater,
    );
    test(
        "0.102",
        "0x0.1a#4",
        50,
        Nearest,
        "9.8631013218843577",
        "0x9.dcf4354ea0b0#50",
        Greater,
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
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "3.14159265358979289",
        "0x3.243f6a8885a2f#54",
        53,
        Nearest,
        "2902679387770655.0",
        "0xa4ff8b5ce0f1f.0#53",
        Less,
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
        "-1.1075146290567388",
        "-0x1.1b861427a3e86#53",
        Less,
    );
    test("3.0", "0x3.0#2", 1, Nearest, "8.0", "0x8.0#1", Greater);
    test("3.0", "0x3.0#2", 1, Floor, "4.0", "0x4.0#1", Less);
    test("3.0", "0x3.0#2", 1, Ceiling, "8.0", "0x8.0#1", Greater);
    test("3.0", "0x3.0#2", 2, Nearest, "8.0", "0x8.0#2", Greater);
    test("0.25", "0x0.4#1", 1, Down, "4.0", "0x4.0#1", Less);
    test("1.0", "0x1.0#1", 1, Down, "1.0", "0x1.0#1", Less);
    test("2.0", "0x2.0#1", 1, Down, "1.0", "0x1.0#1", Less);
    test("4.0", "0x4.0#1", 1, Down, "-1.0", "-0x1.0#1", Greater);
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
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "1.5707963267948966192313216916397514420985846996875529104874722",
        "0x1.921fb54442d18469898cc51701b839a252049c1114cf98e804#200",
        100,
        Floor,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "4.7123889803846898576939650749192543286",
        "0x4.b65f1fccc8748d3c9ca64f450528b0#120",
        120,
        Ceiling,
        "-1.0000000000000000000000000000000000000",
        "-0x1.000000000000000000000000000000#120",
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        64,
        Nearest,
        "-19933988149058553534.0",
        "-0x114a3c09b557b46be.0#64",
        Less,
    );
    test(
        "2.5",
        "0x2.8#3",
        10,
        Nearest,
        "1.6719",
        "0x1.ac0#10",
        Greater,
    );
    test(
        "-2.5",
        "-0x2.8#3",
        10,
        Floor,
        "-1.6719",
        "-0x1.ac0#10",
        Less,
    );
    test(
        "2.99976",
        "0x2.fff#14",
        20,
        Nearest,
        "7.0740509",
        "0x7.12f50#20",
        Less,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        10,
        Ceiling,
        "-7.0859",
        "-0x7.16#10",
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
fn csc_prec_round_fail() {
    Float::ONE.csc_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn csc_round_fail() {
    Float::ONE.csc_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn csc_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (s, o) = x.clone().csc_prec_round(prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = x.csc_prec_round_ref(prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.csc_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_o) = rug_csc_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    }

    // csc is odd
    let (s_neg, o_neg) = (-&x).csc_prec_round(prec, -rm);
    assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
    assert_eq!(o_neg, o.reverse());

    if s.is_normal() {
        assert_eq!(s.get_prec(), Some(prec));
    }

    if o == Equal {
        // csc is exact only at ±0, where it is ±infinity, and for NaN and ±infinity, where it is
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
            let (s2, o2) = x.csc_prec_round_ref(prec, rm2);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.csc_prec_round_ref(prec, Exact));
    }
}

#[test]
fn csc_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        csc_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.csc_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::INFINITY.csc_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_INFINITY.csc_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        // csc(+0) = +infinity and csc(-0) = -infinity, exactly
        let (s, o) = Float::ZERO.csc_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.csc_prec_round(prec, rm);
        assert_eq!(
            ComparableFloat(s),
            ComparableFloat(Float::NEGATIVE_INFINITY)
        );
        assert_eq!(o, Equal);
    });
}

#[test]
fn csc_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (s, o) = x.clone().csc_round(rm);
        assert!(s.is_valid());
        assert_rounding_ordering_consistent(&s, rm, o);
        let (s_alt, o_alt) = x.csc_round_ref(rm);
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.csc_round_assign(rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (s_alt, o_alt) = x.csc_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_s, rug_o) = rug_csc_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn csc_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (s, o) = x.clone().csc_prec(prec);
        assert!(s.is_valid());
        let (s_alt, o_alt) = x.csc_prec_ref(prec);
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.csc_prec_assign(prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (s_alt, o_alt) = x.csc_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (rug_s, rug_o) = rug_csc_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn csc_properties() {
    float_gen().test_properties(|x| {
        let s = x.clone().csc();
        assert!(s.is_valid());
        let s_alt = (&x).csc();
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));

        let mut x_alt = x.clone();
        x_alt.csc_assign();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));

        let (s_alt, _) = x.csc_prec_round_ref(x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_csc(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&s)
        );

        // csc is odd
        assert_eq!(ComparableFloat((-&x).csc()), ComparableFloat(-&s));
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_csc() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_csc(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, f32::INFINITY);
    test::<f32>(-0.0, f32::NEGATIVE_INFINITY);
    test::<f32>(1.0, 1.1883951);
    test::<f32>(-1.0, -1.1883951);
    test::<f32>(0.5, 2.0858297);
    test::<f32>(2.0, 1.0997502);
    test::<f32>(100.0, -1.9748576);
    test::<f32>(10000000000.0, -2.0512567);
    test::<f32>(1.0e-10, 10000000000.0);
    test::<f32>(core::f32::consts::FRAC_PI_2, 1.0);
    test::<f32>(core::f32::consts::PI, -11438666.0);
    test::<f32>(1.0e-45, f32::INFINITY);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, f64::INFINITY);
    test::<f64>(-0.0, f64::NEGATIVE_INFINITY);
    test::<f64>(1.0, 1.1883951057781212);
    test::<f64>(-1.0, -1.1883951057781212);
    test::<f64>(0.5, 2.085829642933488);
    test::<f64>(2.0, 1.0997501702946164);
    test::<f64>(100.0, -1.9748575314241);
    test::<f64>(10000000000.0, -2.0512566994848793);
    test::<f64>(1.0e-10, 10000000000.0);
    test::<f64>(core::f64::consts::FRAC_PI_2, 1.0);
    test::<f64>(core::f64::consts::PI, 8165619676597685.0);
    test::<f64>(1.0e300, -1.2226703943273929);
    test::<f64>(5.0e-324, f64::INFINITY);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_csc_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let t = primitive_float_csc(x);
        // NaN exactly for NaN and infinite inputs
        assert_eq!(t.is_nan(), !x.is_finite());
        if x.is_finite() {
            // odd
            assert_eq!(NiceFloat(primitive_float_csc(-x)), NiceFloat(-t));
            // the result is the correctly rounded cosecant, as computed by MPFR with 64 bits to
            // spare, so that a subnormal result is rounded once by the conversion
            let rug_t = rug_csc_prec(
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
fn primitive_float_csc_properties() {
    apply_fn_to_primitive_floats!(primitive_float_csc_properties_helper);
}

// Inputs whose cosecants overflow: one within 2^(-2^30) of pi, where the sine is tiny, and the
// smallest positive `Float`, whose reciprocal alone is beyond the largest finite one.
#[test]
fn test_csc_overflow() {
    let max = Float::max_finite_value_with_prec(10);
    let p = (1u64 << 30) + 64;
    // just below pi, where the sine is positive and tiny
    let pi = Float::pi_prec_round(p, Floor).0;
    let (s, o) = pi.csc_prec_round_ref(10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (s, o) = pi.csc_prec_round_ref(10, Down);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&max));
    assert_eq!(o, Less);
    // just above pi, where the sine is negative and tiny
    let pi = Float::pi_prec_round(p, Ceiling).0;
    let (s, o) = pi.csc_prec_round_ref(10, Nearest);
    assert_eq!(
        ComparableFloat(s),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    let (s, o) = pi.csc_prec_round_ref(10, Ceiling);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&-max.clone()));
    assert_eq!(o, Greater);
    // the smallest positive `Float`, whose cosecant is about its reciprocal: the tiny path returns
    // the overflow directly
    let tiny = Float::one_prec(10) >> (1u64 << 30);
    let (s, o) = tiny.csc_prec_round_ref(10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (s, o) = tiny.csc_prec_round_ref(10, Down);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&max));
    assert_eq!(o, Less);
    let (s, o) = (-tiny).csc_prec_round_ref(10, Nearest);
    assert_eq!(
        ComparableFloat(s),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
}
