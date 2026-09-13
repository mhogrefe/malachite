// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Csc, CscAssign, PowerOf2};
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
use malachite_float::float::arithmetic::csc::{primitive_float_csc, primitive_float_csc_rational};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::csc::{
    rug_csc, rug_csc_prec, rug_csc_prec_round, rug_csc_rational_prec, rug_csc_rational_prec_round,
    rug_csc_round,
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

// Rows reuse the sine test's inputs. Branches of `csc_prec_round_normal_ref` covered:
// - tiny x: the small-input shortcut rounds x directly
// - the general Ziv loop, at the first working precision and after a retry
// - |x| near an odd multiple of pi/2: a large result, from the cosine's near-zero path inside
//   `sin_cos`
// - |x| near a nonzero multiple of pi: a tiny result, from the sine's near-zero path

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

// Rows reuse the sine test's inputs, including the non-dyadic ones whose cosecants MPFR cannot see
// exactly, since it must round the input first.
#[test]
fn test_csc_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (t, o) = Float::csc_rational_prec_round(x.clone(), prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = Float::csc_rational_prec_round_ref(&x, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = Float::csc_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) = Float::csc_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) = rug_csc_rational_prec_round(&x, prec, rug_rm);
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
    test("1", 1, Down, "1.0", "0x1.0#1", Less);
    test("1", 1, Up, "2.0", "0x2.0#1", Greater);
    test("1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1", 5, Nearest, "1.19", "0x1.3#5", Less);
    test("1", 10, Down, "1.1875", "0x1.300#10", Less);
    test("1", 10, Up, "1.1895", "0x1.308#10", Greater);
    test("1", 10, Floor, "1.1875", "0x1.300#10", Less);
    test("1", 10, Ceiling, "1.1895", "0x1.308#10", Greater);
    test("1", 10, Nearest, "1.1875", "0x1.300#10", Less);
    test("1", 20, Nearest, "1.1883945", "0x1.303aa#20", Less);
    test(
        "1",
        53,
        Down,
        "1.1883951057781210",
        "0x1.303aa9620b223#53",
        Less,
    );
    test(
        "1",
        53,
        Up,
        "1.1883951057781212",
        "0x1.303aa9620b224#53",
        Greater,
    );
    test(
        "1",
        53,
        Floor,
        "1.1883951057781210",
        "0x1.303aa9620b223#53",
        Less,
    );
    test(
        "1",
        53,
        Ceiling,
        "1.1883951057781212",
        "0x1.303aa9620b224#53",
        Greater,
    );
    test(
        "1",
        53,
        Nearest,
        "1.1883951057781212",
        "0x1.303aa9620b224#53",
        Greater,
    );
    test(
        "1",
        100,
        Nearest,
        "1.1883951057781212162615994523744",
        "0x1.303aa9620b223e32e68b9bddc#100",
        Less,
    );
    test("-1", 1, Down, "-1.0", "-0x1.0#1", Greater);
    test("-1", 1, Up, "-2.0", "-0x2.0#1", Less);
    test("-1", 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("-1", 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("-1", 1, Nearest, "-1.0", "-0x1.0#1", Greater);
    test("-1", 5, Nearest, "-1.19", "-0x1.3#5", Greater);
    test("-1", 10, Down, "-1.1875", "-0x1.300#10", Greater);
    test("-1", 10, Up, "-1.1895", "-0x1.308#10", Less);
    test("-1", 10, Floor, "-1.1895", "-0x1.308#10", Less);
    test("-1", 10, Ceiling, "-1.1875", "-0x1.300#10", Greater);
    test("-1", 10, Nearest, "-1.1875", "-0x1.300#10", Greater);
    test("-1", 20, Nearest, "-1.1883945", "-0x1.303aa#20", Greater);
    test(
        "-1",
        53,
        Down,
        "-1.1883951057781210",
        "-0x1.303aa9620b223#53",
        Greater,
    );
    test(
        "-1",
        53,
        Up,
        "-1.1883951057781212",
        "-0x1.303aa9620b224#53",
        Less,
    );
    test(
        "-1",
        53,
        Floor,
        "-1.1883951057781212",
        "-0x1.303aa9620b224#53",
        Less,
    );
    test(
        "-1",
        53,
        Ceiling,
        "-1.1883951057781210",
        "-0x1.303aa9620b223#53",
        Greater,
    );
    test(
        "-1",
        53,
        Nearest,
        "-1.1883951057781212",
        "-0x1.303aa9620b224#53",
        Less,
    );
    test(
        "-1",
        100,
        Nearest,
        "-1.1883951057781212162615994523744",
        "-0x1.303aa9620b223e32e68b9bddc#100",
        Greater,
    );
    test("1/2", 1, Down, "2.0", "0x2.0#1", Less);
    test("1/2", 1, Up, "4.0", "0x4.0#1", Greater);
    test("1/2", 1, Floor, "2.0", "0x2.0#1", Less);
    test("1/2", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("1/2", 1, Nearest, "2.0", "0x2.0#1", Less);
    test("1/2", 5, Nearest, "2.12", "0x2.2#5", Greater);
    test("1/2", 10, Down, "2.0820", "0x2.15#10", Less);
    test("1/2", 10, Up, "2.0859", "0x2.16#10", Greater);
    test("1/2", 10, Floor, "2.0820", "0x2.15#10", Less);
    test("1/2", 10, Ceiling, "2.0859", "0x2.16#10", Greater);
    test("1/2", 10, Nearest, "2.0859", "0x2.16#10", Greater);
    test("1/2", 20, Nearest, "2.0858307", "0x2.15f90#20", Greater);
    test(
        "1/2",
        53,
        Down,
        "2.0858296429334882",
        "0x2.15f8ee756d3b8#53",
        Less,
    );
    test(
        "1/2",
        53,
        Up,
        "2.0858296429334886",
        "0x2.15f8ee756d3ba#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Floor,
        "2.0858296429334882",
        "0x2.15f8ee756d3b8#53",
        Less,
    );
    test(
        "1/2",
        53,
        Ceiling,
        "2.0858296429334886",
        "0x2.15f8ee756d3ba#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Nearest,
        "2.0858296429334882",
        "0x2.15f8ee756d3b8#53",
        Less,
    );
    test(
        "1/2",
        100,
        Nearest,
        "2.0858296429334881857725016754579",
        "0x2.15f8ee756d3b81e5f56559a4c#100",
        Less,
    );
    test("1/3", 1, Down, "2.0", "0x2.0#1", Less);
    test("1/3", 1, Up, "4.0", "0x4.0#1", Greater);
    test("1/3", 1, Floor, "2.0", "0x2.0#1", Less);
    test("1/3", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("1/3", 1, Nearest, "4.0", "0x4.0#1", Greater);
    test("1/3", 5, Nearest, "3.00", "0x3.0#5", Less);
    test("1/3", 10, Down, "3.0547", "0x3.0e#10", Less);
    test("1/3", 10, Up, "3.0586", "0x3.0f#10", Greater);
    test("1/3", 10, Floor, "3.0547", "0x3.0e#10", Less);
    test("1/3", 10, Ceiling, "3.0586", "0x3.0f#10", Greater);
    test("1/3", 10, Nearest, "3.0547", "0x3.0e#10", Less);
    test("1/3", 20, Nearest, "3.0562859", "0x3.0e68c#20", Greater);
    test(
        "1/3",
        53,
        Down,
        "3.0562842545795190",
        "0x3.0e68a518b2e2c#53",
        Less,
    );
    test(
        "1/3",
        53,
        Up,
        "3.0562842545795195",
        "0x3.0e68a518b2e2e#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Floor,
        "3.0562842545795190",
        "0x3.0e68a518b2e2c#53",
        Less,
    );
    test(
        "1/3",
        53,
        Ceiling,
        "3.0562842545795195",
        "0x3.0e68a518b2e2e#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Nearest,
        "3.0562842545795195",
        "0x3.0e68a518b2e2e#53",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "3.0562842545795193204625163549239",
        "0x3.0e68a518b2e2d583b9214f68c#100",
        Less,
    );
    test("-1/3", 1, Down, "-2.0", "-0x2.0#1", Greater);
    test("-1/3", 1, Up, "-4.0", "-0x4.0#1", Less);
    test("-1/3", 1, Floor, "-4.0", "-0x4.0#1", Less);
    test("-1/3", 1, Ceiling, "-2.0", "-0x2.0#1", Greater);
    test("-1/3", 1, Nearest, "-4.0", "-0x4.0#1", Less);
    test("-1/3", 5, Nearest, "-3.00", "-0x3.0#5", Greater);
    test("-1/3", 10, Down, "-3.0547", "-0x3.0e#10", Greater);
    test("-1/3", 10, Up, "-3.0586", "-0x3.0f#10", Less);
    test("-1/3", 10, Floor, "-3.0586", "-0x3.0f#10", Less);
    test("-1/3", 10, Ceiling, "-3.0547", "-0x3.0e#10", Greater);
    test("-1/3", 10, Nearest, "-3.0547", "-0x3.0e#10", Greater);
    test("-1/3", 20, Nearest, "-3.0562859", "-0x3.0e68c#20", Less);
    test(
        "-1/3",
        53,
        Down,
        "-3.0562842545795190",
        "-0x3.0e68a518b2e2c#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Up,
        "-3.0562842545795195",
        "-0x3.0e68a518b2e2e#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Floor,
        "-3.0562842545795195",
        "-0x3.0e68a518b2e2e#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Ceiling,
        "-3.0562842545795190",
        "-0x3.0e68a518b2e2c#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Nearest,
        "-3.0562842545795195",
        "-0x3.0e68a518b2e2e#53",
        Less,
    );
    test(
        "-1/3",
        100,
        Nearest,
        "-3.0562842545795193204625163549239",
        "-0x3.0e68a518b2e2d583b9214f68c#100",
        Greater,
    );
    test("3/5", 1, Down, "1.0", "0x1.0#1", Less);
    test("3/5", 1, Up, "2.0", "0x2.0#1", Greater);
    test("3/5", 1, Floor, "1.0", "0x1.0#1", Less);
    test("3/5", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("3/5", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("3/5", 10, Down, "1.7695", "0x1.c50#10", Less);
    test("3/5", 10, Up, "1.7715", "0x1.c58#10", Greater);
    test("3/5", 10, Floor, "1.7695", "0x1.c50#10", Less);
    test("3/5", 10, Ceiling, "1.7715", "0x1.c58#10", Greater);
    test("3/5", 10, Nearest, "1.7715", "0x1.c58#10", Greater);
    test(
        "3/5",
        53,
        Down,
        "1.7710321966877252",
        "0x1.c5625db4efd36#53",
        Less,
    );
    test(
        "3/5",
        53,
        Up,
        "1.7710321966877254",
        "0x1.c5625db4efd37#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Floor,
        "1.7710321966877252",
        "0x1.c5625db4efd36#53",
        Less,
    );
    test(
        "3/5",
        53,
        Ceiling,
        "1.7710321966877254",
        "0x1.c5625db4efd37#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Nearest,
        "1.7710321966877254",
        "0x1.c5625db4efd37#53",
        Greater,
    );
    test(
        "3/5",
        100,
        Nearest,
        "1.7710321966877253733746121071053",
        "0x1.c5625db4efd36f8c4c32599fc#100",
        Less,
    );
    test("22/7", 1, Down, "-5.1e2", "-0x2.0E+2#1", Greater);
    test("22/7", 1, Up, "-1.0e3", "-0x4.0E+2#1", Less);
    test("22/7", 1, Floor, "-1.0e3", "-0x4.0E+2#1", Less);
    test("22/7", 1, Ceiling, "-5.1e2", "-0x2.0E+2#1", Greater);
    test("22/7", 1, Nearest, "-1.0e3", "-0x4.0E+2#1", Less);
    test("22/7", 5, Nearest, "-800.0", "-0x3.2E+2#5", Less);
    test("22/7", 10, Down, "-790.00", "-0x316.0#10", Greater);
    test("22/7", 10, Up, "-791.00", "-0x317.0#10", Less);
    test("22/7", 10, Floor, "-791.00", "-0x317.0#10", Less);
    test("22/7", 10, Ceiling, "-790.00", "-0x316.0#10", Greater);
    test("22/7", 10, Nearest, "-791.00", "-0x317.0#10", Less);
    test("22/7", 20, Nearest, "-790.83301", "-0x316.d54#20", Greater);
    test(
        "22/7",
        53,
        Down,
        "-790.83333667585077",
        "-0x316.d5558d69562#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Up,
        "-790.83333667585089",
        "-0x316.d5558d69564#53",
        Less,
    );
    test(
        "22/7",
        53,
        Floor,
        "-790.83333667585089",
        "-0x316.d5558d69564#53",
        Less,
    );
    test(
        "22/7",
        53,
        Ceiling,
        "-790.83333667585077",
        "-0x316.d5558d69562#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Nearest,
        "-790.83333667585077",
        "-0x316.d5558d69562#53",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "-790.83333667585081293439700481863",
        "-0x316.d5558d69562b0f1ad779954#100",
        Greater,
    );
    test("-22/7", 1, Down, "5.1e2", "0x2.0E+2#1", Less);
    test("-22/7", 1, Up, "1.0e3", "0x4.0E+2#1", Greater);
    test("-22/7", 1, Floor, "5.1e2", "0x2.0E+2#1", Less);
    test("-22/7", 1, Ceiling, "1.0e3", "0x4.0E+2#1", Greater);
    test("-22/7", 1, Nearest, "1.0e3", "0x4.0E+2#1", Greater);
    test("-22/7", 5, Nearest, "800.0", "0x3.2E+2#5", Greater);
    test("-22/7", 10, Down, "790.00", "0x316.0#10", Less);
    test("-22/7", 10, Up, "791.00", "0x317.0#10", Greater);
    test("-22/7", 10, Floor, "790.00", "0x316.0#10", Less);
    test("-22/7", 10, Ceiling, "791.00", "0x317.0#10", Greater);
    test("-22/7", 10, Nearest, "791.00", "0x317.0#10", Greater);
    test("-22/7", 20, Nearest, "790.83301", "0x316.d54#20", Less);
    test(
        "-22/7",
        53,
        Down,
        "790.83333667585077",
        "0x316.d5558d69562#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Up,
        "790.83333667585089",
        "0x316.d5558d69564#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Floor,
        "790.83333667585077",
        "0x316.d5558d69562#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Ceiling,
        "790.83333667585089",
        "0x316.d5558d69564#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Nearest,
        "790.83333667585077",
        "0x316.d5558d69562#53",
        Less,
    );
    test(
        "-22/7",
        100,
        Nearest,
        "790.83333667585081293439700481863",
        "0x316.d5558d69562b0f1ad779954#100",
        Less,
    );
    test("355/113", 1, Down, "-2.1e6", "-0x2.0E+5#1", Greater);
    test("355/113", 1, Up, "-4.2e6", "-0x4.0E+5#1", Less);
    test("355/113", 1, Floor, "-4.2e6", "-0x4.0E+5#1", Less);
    test("355/113", 1, Ceiling, "-2.1e6", "-0x2.0E+5#1", Greater);
    test("355/113", 1, Nearest, "-4.2e6", "-0x4.0E+5#1", Less);
    test("355/113", 5, Nearest, "-3.80e6", "-0x3.aE+5#5", Less);
    test("355/113", 10, Down, "-3.7478e6", "-0x3.93E+5#10", Greater);
    test("355/113", 10, Up, "-3.7519e6", "-0x3.94E+5#10", Less);
    test("355/113", 10, Floor, "-3.7519e6", "-0x3.94E+5#10", Less);
    test(
        "355/113",
        10,
        Ceiling,
        "-3.7478e6",
        "-0x3.93E+5#10",
        Greater,
    );
    test(
        "355/113",
        10,
        Nearest,
        "-3.7478e6",
        "-0x3.93E+5#10",
        Greater,
    );
    test(
        "355/113",
        20,
        Nearest,
        "-3748628.0",
        "-0x393314.0#20",
        Greater,
    );
    test(
        "355/113",
        53,
        Down,
        "-3748629.0926628602",
        "-0x393315.17b8c0d2#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Up,
        "-3748629.0926628606",
        "-0x393315.17b8c0d4#53",
        Less,
    );
    test(
        "355/113",
        53,
        Floor,
        "-3748629.0926628606",
        "-0x393315.17b8c0d4#53",
        Less,
    );
    test(
        "355/113",
        53,
        Ceiling,
        "-3748629.0926628602",
        "-0x393315.17b8c0d2#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Nearest,
        "-3748629.0926628602",
        "-0x393315.17b8c0d2#53",
        Greater,
    );
    test(
        "355/113",
        100,
        Nearest,
        "-3748629.0926628602474998015219207",
        "-0x393315.17b8c0d2511bc73f0664#100",
        Less,
    );
    test("3", 1, Down, "4.0", "0x4.0#1", Less);
    test("3", 1, Up, "8.0", "0x8.0#1", Greater);
    test("3", 1, Floor, "4.0", "0x4.0#1", Less);
    test("3", 1, Ceiling, "8.0", "0x8.0#1", Greater);
    test("3", 1, Nearest, "8.0", "0x8.0#1", Greater);
    test("3", 5, Nearest, "7.00", "0x7.0#5", Less);
    test("3", 10, Down, "7.0859", "0x7.16#10", Less);
    test("3", 10, Up, "7.0938", "0x7.18#10", Greater);
    test("3", 10, Floor, "7.0859", "0x7.16#10", Less);
    test("3", 10, Ceiling, "7.0938", "0x7.18#10", Greater);
    test("3", 10, Nearest, "7.0859", "0x7.16#10", Less);
    test("3", 20, Nearest, "7.0861664", "0x7.160f0#20", Less);
    test(
        "3",
        53,
        Down,
        "7.0861673957371858",
        "0x7.160f1102ac364#53",
        Less,
    );
    test(
        "3",
        53,
        Up,
        "7.0861673957371867",
        "0x7.160f1102ac368#53",
        Greater,
    );
    test(
        "3",
        53,
        Floor,
        "7.0861673957371858",
        "0x7.160f1102ac364#53",
        Less,
    );
    test(
        "3",
        53,
        Ceiling,
        "7.0861673957371867",
        "0x7.160f1102ac368#53",
        Greater,
    );
    test(
        "3",
        53,
        Nearest,
        "7.0861673957371858",
        "0x7.160f1102ac364#53",
        Less,
    );
    test(
        "3",
        100,
        Nearest,
        "7.0861673957371859182175322724590",
        "0x7.160f1102ac364911785c45808#100",
        Less,
    );
    test("100", 1, Down, "-1.0", "-0x1.0#1", Greater);
    test("100", 1, Up, "-2.0", "-0x2.0#1", Less);
    test("100", 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("100", 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("100", 1, Nearest, "-2.0", "-0x2.0#1", Less);
    test("100", 5, Nearest, "-2.00", "-0x2.0#5", Less);
    test("100", 10, Down, "-1.9746", "-0x1.f98#10", Greater);
    test("100", 10, Up, "-1.9766", "-0x1.fa0#10", Less);
    test("100", 10, Floor, "-1.9766", "-0x1.fa0#10", Less);
    test("100", 10, Ceiling, "-1.9746", "-0x1.f98#10", Greater);
    test("100", 10, Nearest, "-1.9746", "-0x1.f98#10", Greater);
    test("100", 20, Nearest, "-1.9748573", "-0x1.f9904#20", Greater);
    test(
        "100",
        53,
        Down,
        "-1.9748575314240999",
        "-0x1.f990435fb9ce2#53",
        Greater,
    );
    test(
        "100",
        53,
        Up,
        "-1.9748575314241001",
        "-0x1.f990435fb9ce3#53",
        Less,
    );
    test(
        "100",
        53,
        Floor,
        "-1.9748575314241001",
        "-0x1.f990435fb9ce3#53",
        Less,
    );
    test(
        "100",
        53,
        Ceiling,
        "-1.9748575314240999",
        "-0x1.f990435fb9ce2#53",
        Greater,
    );
    test(
        "100",
        53,
        Nearest,
        "-1.9748575314240999",
        "-0x1.f990435fb9ce2#53",
        Greater,
    );
    test(
        "100",
        100,
        Nearest,
        "-1.9748575314240999612122645488016",
        "-0x1.f990435fb9ce22e1a4c3bf5a2#100",
        Greater,
    );
    test("1000000", 1, Down, "-2.0", "-0x2.0#1", Greater);
    test("1000000", 1, Up, "-4.0", "-0x4.0#1", Less);
    test("1000000", 1, Floor, "-4.0", "-0x4.0#1", Less);
    test("1000000", 1, Ceiling, "-2.0", "-0x2.0#1", Greater);
    test("1000000", 1, Nearest, "-2.0", "-0x2.0#1", Greater);
    test("1000000", 5, Nearest, "-2.88", "-0x2.e#5", Less);
    test("1000000", 10, Down, "-2.8555", "-0x2.db#10", Greater);
    test("1000000", 10, Up, "-2.8594", "-0x2.dc#10", Less);
    test("1000000", 10, Floor, "-2.8594", "-0x2.dc#10", Less);
    test("1000000", 10, Ceiling, "-2.8555", "-0x2.db#10", Greater);
    test("1000000", 10, Nearest, "-2.8555", "-0x2.db#10", Greater);
    test("1000000", 20, Nearest, "-2.8571968", "-0x2.db714#20", Less);
    test(
        "1000000",
        53,
        Down,
        "-2.8571959016272892",
        "-0x2.db7130cbc1230#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Up,
        "-2.8571959016272896",
        "-0x2.db7130cbc1232#53",
        Less,
    );
    test(
        "1000000",
        53,
        Floor,
        "-2.8571959016272896",
        "-0x2.db7130cbc1232#53",
        Less,
    );
    test(
        "1000000",
        53,
        Ceiling,
        "-2.8571959016272892",
        "-0x2.db7130cbc1230#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Nearest,
        "-2.8571959016272892",
        "-0x2.db7130cbc1230#53",
        Greater,
    );
    test(
        "1000000",
        100,
        Nearest,
        "-2.8571959016272892953075563380105",
        "-0x2.db7130cbc12306a87fcda6ba4#100",
        Less,
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
    test("1/1000000", 20, Nearest, "1000000.0", "0xf4240.0#20", Less);
    test(
        "1/1000000",
        53,
        Down,
        "1000000.0000001666",
        "0xf4240.000002cb8#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Up,
        "1000000.0000001667",
        "0xf4240.000002cc0#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Floor,
        "1000000.0000001666",
        "0xf4240.000002cb8#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Ceiling,
        "1000000.0000001667",
        "0xf4240.000002cc0#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Nearest,
        "1000000.0000001667",
        "0xf4240.000002cc0#53",
        Greater,
    );
    test(
        "1/1000000",
        100,
        Nearest,
        "1000000.0000001666666666666861114",
        "0xf4240.000002cbd3f01e529e07#100",
        Greater,
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
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Down,
        "-1000000.0000001666",
        "-0xf4240.000002cb8#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Up,
        "-1000000.0000001667",
        "-0xf4240.000002cc0#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Floor,
        "-1000000.0000001667",
        "-0xf4240.000002cc0#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Ceiling,
        "-1000000.0000001666",
        "-0xf4240.000002cb8#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Nearest,
        "-1000000.0000001667",
        "-0xf4240.000002cc0#53",
        Less,
    );
    test(
        "-1/1000000",
        100,
        Nearest,
        "-1000000.0000001666666666666861114",
        "-0xf4240.000002cbd3f01e529e07#100",
        Less,
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
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        5,
        Nearest,
        "1.00",
        "0x1.0#5",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Down,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "1.0020",
        "0x1.008#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "1.0000000000000002",
        "0x1.0000000000001#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "1.0000000000000002",
        "0x1.0000000000001#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Down,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Up,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Floor,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Ceiling,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        5,
        Nearest,
        "-1.00",
        "-0x1.0#5",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Down,
        "-1.0000",
        "-0x1.000#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Up,
        "-1.0020",
        "-0x1.008#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Floor,
        "-1.0020",
        "-0x1.008#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Ceiling,
        "-1.0000",
        "-0x1.000#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        20,
        Nearest,
        "-1.0000000",
        "-0x1.00000#20",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Down,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Up,
        "-1.0000000000000002",
        "-0x1.0000000000001#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Floor,
        "-1.0000000000000002",
        "-0x1.0000000000001#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Ceiling,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        100,
        Nearest,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Down,
        "5.1e30",
        "0x4.0E+25#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Up,
        "1.0e31",
        "0x8.0E+25#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Floor,
        "5.1e30",
        "0x4.0E+25#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Ceiling,
        "1.0e31",
        "0x8.0E+25#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Nearest,
        "5.1e30",
        "0x4.0E+25#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        5,
        Nearest,
        "6.02e30",
        "0x4.cE+25#5",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Down,
        "5.8926e30",
        "0x4.a6E+25#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Up,
        "5.9025e30",
        "0x4.a8E+25#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Floor,
        "5.8926e30",
        "0x4.a6E+25#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Ceiling,
        "5.9025e30",
        "0x4.a8E+25#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Nearest,
        "5.8926e30",
        "0x4.a6E+25#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        20,
        Nearest,
        "5.8973239e30",
        "0x4.a6f48E+25#20",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Down,
        "5.8973198808686271e30",
        "0x4.a6f44abe5fc70E+25#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Up,
        "5.8973198808686282e30",
        "0x4.a6f44abe5fc74E+25#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Floor,
        "5.8973198808686271e30",
        "0x4.a6f44abe5fc70E+25#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Ceiling,
        "5.8973198808686282e30",
        "0x4.a6f44abe5fc74E+25#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Nearest,
        "5.8973198808686282e30",
        "0x4.a6f44abe5fc74E+25#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        100,
        Nearest,
        "5897319880868628039416095147896.0",
        "0x4a6f44abe5fc7366ee195b8778.0#100",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Down,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Up,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Floor,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Ceiling,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        5,
        Nearest,
        "-1.00",
        "-0x1.0#5",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Down,
        "-1.0000",
        "-0x1.000#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "-1.0020",
        "-0x1.008#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "-1.0020",
        "-0x1.008#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "-1.0000",
        "-0x1.000#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "-1.0000000",
        "-0x1.00000#20",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "-1.0000000000000002",
        "-0x1.0000000000001#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "-1.0000000000000002",
        "-0x1.0000000000001#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
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
        "-1.0",
        "-0x1.0#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        5,
        Nearest,
        "-1.12",
        "-0x1.2#5",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Down,
        "-1.1465",
        "-0x1.258#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Up,
        "-1.1484",
        "-0x1.260#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Floor,
        "-1.1484",
        "-0x1.260#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Ceiling,
        "-1.1465",
        "-0x1.258#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Nearest,
        "-1.1465",
        "-0x1.258#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        20,
        Nearest,
        "-1.1465473",
        "-0x1.25842#20",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Down,
        "-1.1465475775830900",
        "-0x1.2584245d07035#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Up,
        "-1.1465475775830902",
        "-0x1.2584245d07036#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Floor,
        "-1.1465475775830902",
        "-0x1.2584245d07036#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Ceiling,
        "-1.1465475775830900",
        "-0x1.2584245d07035#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Nearest,
        "-1.1465475775830900",
        "-0x1.2584245d07035#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        100,
        Nearest,
        "-1.1465475775830900068211209364114",
        "-0x1.2584245d0703501ed9171fbba#100",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Down,
        "1.3e30",
        "0x1.0E+25#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Up,
        "2.5e30",
        "0x2.0E+25#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Floor,
        "1.3e30",
        "0x1.0E+25#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Ceiling,
        "2.5e30",
        "0x2.0E+25#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Nearest,
        "1.3e30",
        "0x1.0E+25#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        5,
        Nearest,
        "1.27e30",
        "0x1.0E+25#5",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Down,
        "1.2677e30",
        "0x1.000E+25#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Up,
        "1.2701e30",
        "0x1.008E+25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Floor,
        "1.2677e30",
        "0x1.000E+25#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Ceiling,
        "1.2701e30",
        "0x1.008E+25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Nearest,
        "1.2677e30",
        "0x1.000E+25#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        20,
        Nearest,
        "1.2676506e30",
        "0x1.00000E+25#20",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Down,
        "1.2676506002282294e30",
        "0x1.0000000000000E+25#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Up,
        "1.2676506002282297e30",
        "0x1.0000000000001E+25#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Floor,
        "1.2676506002282294e30",
        "0x1.0000000000000E+25#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Ceiling,
        "1.2676506002282297e30",
        "0x1.0000000000001E+25#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Nearest,
        "1.2676506002282294e30",
        "0x1.0000000000000E+25#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        100,
        Nearest,
        "1267650600228229401496703205376.0",
        "0x10000000000000000000000000.0#100",
        Less,
    );
}

#[allow(clippy::needless_pass_by_value)]
fn csc_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (s, o) = Float::csc_rational_prec_round(x.clone(), prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = Float::csc_rational_prec_round_ref(&x, prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    // csc is odd (a `Rational` has no negative zero, so x = 0 is excluded)
    if x != 0u32 {
        let (s_neg, o_neg) = Float::csc_rational_prec_round(-&x, prec, -rm);
        assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
        assert_eq!(o_neg, o.reverse());
    }

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_o) = rug_csc_rational_prec_round(&x, prec, rrm);
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
        // only csc(0) = infinity is exact
        assert_eq!(x, 0u32);
        assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&Float::INFINITY));
        for rm in exhaustive_rounding_modes() {
            let (s2, oo) = Float::csc_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::csc_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn csc_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        csc_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // csc(0) = infinity, exactly
        let (s, o) = Float::csc_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn csc_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (s, o) = Float::csc_rational_prec(x.clone(), prec);
    assert!(s.is_valid());

    let (s_alt, o_alt) = Float::csc_rational_prec_ref(&x, prec);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let (s_alt, o_alt) = Float::csc_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let (rug_s, rug_o) = rug_csc_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_s)),
        ComparableFloatRef(&s)
    );
    assert_eq!(rug_o, o, "x = {x} prec = {prec}");

    // the cosecant of an exactly representable rational is the Float cosecant
    if let Ok(f) = Float::try_from(&x) {
        let (s_alt, o_alt) = f.csc_prec(prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
    }
}

#[test]
fn csc_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        csc_rational_prec_properties_helper(x, prec);
    });
}

// An input too large to be a `Float`, reduced modulo 2 pi in `Rational` arithmetic with pi to about
// 2^30 bits; slow even in release mode.
#[test]
fn test_csc_rational_huge() {
    let x = Rational::power_of_2(1i64 << 30);
    let (t, o) = Float::csc_rational_prec_round_ref(&x, 10, Nearest);
    assert_eq!(t.to_string(), "1.5938");
    assert_eq!(to_hex_string(&t), "0x1.980#10");
    assert_eq!(o, Greater);
}

// An input within 2^(-2^30) of an odd multiple of pi/2, whose cosecant overflows. The call computes
// pi to about 2^30 bits, so this test is slow even in release mode.
#[test]
fn test_csc_rational_overflow() {
    let max = Float::max_finite_value_with_prec(10);
    let p = (1u64 << 30) + 64;
    // just below pi, where the sine is positive and tiny
    let pi = Rational::exact_from(&Float::pi_prec_round(p, Floor).0);
    let (s, o) = Float::csc_rational_prec_round_ref(&pi, 10, Down);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&max));
    assert_eq!(o, Less);
    let (s, o) = Float::csc_rational_prec_round_ref(&pi, 10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    // a `Rational` far below the `Float` exponent range, where the cosecant is about its
    // reciprocal: the tiny path decides it from a bracket on the sine
    let tiny = Rational::power_of_2(-((1i64 << 30) + 70));
    let (s, o) = Float::csc_rational_prec_round_ref(&tiny, 10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (s, o) = Float::csc_rational_prec_round_ref(&tiny, 10, Down);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&max));
    assert_eq!(o, Less);
    let (s, o) = Float::csc_rational_prec_round_ref(&-tiny, 10, Nearest);
    assert_eq!(
        ComparableFloat(s),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_csc_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_csc_rational::<T>(&x)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", f32::INFINITY);
    test::<f32>("1", 1.1883951);
    test::<f32>("-1", -1.1883951);
    test::<f32>("1/2", 2.0858297);
    test::<f32>("1/3", 3.0562842);
    test::<f32>("22/7", -790.8333);
    test::<f32>("1/7", 7.023866);
    test::<f32>("100", -1.9748576);
    test::<f32>("355/113", -3748629.0);
    test::<f32>("1/1000000", 1000000.0);
    test::<f32>("-2/3", -1.6171553);
    test::<f32>("1000000", -2.8571959);
    test::<f32>("1/100000000000000000000", 1.0e20);
    test::<f64>("0", f64::INFINITY);
    test::<f64>("1", 1.1883951057781212);
    test::<f64>("-1", -1.1883951057781212);
    test::<f64>("1/2", 2.085829642933488);
    test::<f64>("1/3", 3.0562842545795195);
    test::<f64>("22/7", -790.8333366758508);
    test::<f64>("1/7", 7.023866335396165);
    test::<f64>("100", -1.9748575314241);
    test::<f64>("355/113", -3748629.09266286);
    test::<f64>("1/1000000", 1000000.0000001667);
    test::<f64>("-2/3", -1.6171552928939261);
    test::<f64>("1000000", -2.857195901627289);
    test::<f64>("1/100000000000000000000", 1.0e20);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_csc_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let s = primitive_float_csc_rational::<T>(&x);
        // the cosecant of a rational is never NaN
        assert!(!s.is_nan());
        // csc is odd (a `Rational` has no negative zero, so x = 0 is excluded)
        if x != 0u32 {
            assert_eq!(
                NiceFloat(primitive_float_csc_rational::<T>(&-&x)),
                NiceFloat(-s)
            );
        }
        // The result is the correctly rounded cosecant, as computed by MPFR with 64 bits to spare.
        // The comparison is skipped when rounding that wider value to `T` is a tie: csc(1/n) is
        // just above n, so for an n that is a midpoint of the `T` grid the wider value rounds to
        // the midpoint itself and the tie breaks the wrong way, though the cosecant is strictly
        // above it.
        let wide = <Float as From<&rug::Float>>::from(
            &rug_csc_rational_prec(&x, T::MANTISSA_WIDTH + 64).0,
        );
        if !ties::<T>(&wide) {
            assert_eq!(NiceFloat(T::rounding_from(&wide, Nearest).0), NiceFloat(s));
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // The cosecant of a finite nonzero primitive float, taken through the `Rational` path,
        // matches the direct primitive-float cosecant (a `Rational` cannot carry the sign of a
        // zero).
        if x.is_finite() && x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_csc_rational::<T>(&Rational::exact_from(x))),
                NiceFloat(primitive_float_csc(x))
            );
        }
    });
}

#[test]
fn primitive_float_csc_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_csc_rational_properties_helper);
}
