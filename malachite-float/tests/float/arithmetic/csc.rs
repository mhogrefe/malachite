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
use malachite_float::float::arithmetic::csc::{
    primitive_float_csc, primitive_float_csc_rational, primitive_float_csc_with_period,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::csc::{
    csc_with_period_naive, rug_csc, rug_csc_prec, rug_csc_prec_round, rug_csc_rational_prec,
    rug_csc_rational_prec_round, rug_csc_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_2, float_unsigned_rounding_mode_triple_gen_var_36,
    float_unsigned_rounding_mode_triple_gen_var_39,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_18,
    float_unsigned_unsigned_triple_gen_var_1, rational_unsigned_rounding_mode_triple_gen_var_10,
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

// Rows reuse the tangent's `with_period` inputs, which cover the exact cases (the poles at
// multiples of a half turn, the ±1 at odd quarter turns, and the ±2 at odd twelfths), the closed
// forms at eighths, sixths, and twentieths of a turn, the general Ziv loop at its first working
// precision and after a retry, and the tiny x/u whose cosecant overflows.
#[test]
fn test_csc_with_period_prec_round() {
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

        let (t, o) = x.clone().csc_with_period_prec_round(u, prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.csc_with_period_prec_round_ref(u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.csc_with_period_prec_round_assign(u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = x.csc_with_period_prec_ref(u, prec);
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
    test(
        "90.0",
        "0x5a.0#6",
        360,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "90.0",
        "0x5a.0#6",
        360,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "90.0",
        "0x5a.0#6",
        360,
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "180.0", "0xb4.0#6", 360, 10, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "180.0", "0xb4.0#6", 360, 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "180.0", "0xb4.0#6", 360, 10, Exact, "Infinity", "Infinity", Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Exact,
        "-1.0000",
        "-0x1.000#10",
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
        "1.1543",
        "0x1.278#10",
        Less,
    );
    test(
        "60.0",
        "0x3c.0#4",
        360,
        10,
        Floor,
        "1.1543",
        "0x1.278#10",
        Less,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Nearest,
        "1.1543",
        "0x1.278#10",
        Less,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Floor,
        "1.1543",
        "0x1.278#10",
        Less,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Nearest,
        "-1.1543",
        "-0x1.278#10",
        Greater,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Floor,
        "-1.1562",
        "-0x1.280#10",
        Less,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Nearest,
        "-1.1543",
        "-0x1.278#10",
        Greater,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Floor,
        "-1.1562",
        "-0x1.280#10",
        Less,
    );
    test(
        "450.0",
        "0x1c2.0#8",
        360,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "450.0",
        "0x1c2.0#8",
        360,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "450.0",
        "0x1c2.0#8",
        360,
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0", "0x1.0#1", 1, 10, Ceiling, "Infinity", "Infinity", Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        3,
        10,
        Nearest,
        "1.1543",
        "0x1.278#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        4,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        7,
        10,
        Nearest,
        "1.2793",
        "0x1.478#10",
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
        "0.50", "0x0.8#1", 1, 10, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        2,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
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
        "2.0000000000000000",
        "0x2.0000000000000#53",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        12,
        10,
        Ceiling,
        "3.8672",
        "0x3.de#10",
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
    test(
        "0.25",
        "0x0.4#1",
        1,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
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
        "2.6133",
        "0x2.9d#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        6,
        10,
        Ceiling,
        "3.8672",
        "0x3.de#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        12,
        10,
        Floor,
        "7.6562",
        "0x7.a8#10",
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
    test(
        "1.5",
        "0x1.8#2",
        2,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "1.5", "0x1.8#2", 3, 53, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        6,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
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
        "38.201550014110445",
        "0x26.3398c81f1cda#53",
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
        "-1.1543",
        "-0x1.278#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        6,
        10,
        Nearest,
        "1.1543",
        "0x1.278#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        7,
        10,
        Ceiling,
        "1.0273",
        "0x1.070#10",
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
    test(
        "3.0",
        "0x3.0#2",
        4,
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
    );
    test("3.0", "0x3.0#2", 7, 10, Floor, "2.3047", "0x2.4e#10", Less);
    test(
        "3.0",
        "0x3.0#2",
        360,
        10,
        Floor,
        "19.094",
        "0x13.18#10",
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
        "-1.1543",
        "-0x1.278#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        4,
        10,
        Ceiling,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        7,
        10,
        Nearest,
        "-1.2793",
        "-0x1.478#10",
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
        "-1.1547005383792515",
        "-0x1.279a74590331c#53",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        12,
        10,
        Ceiling,
        "1.1562",
        "0x1.280#10",
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
        "3.6641",
        "0x3.aa#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        2,
        10,
        Ceiling,
        "-1.0078",
        "-0x1.020#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        4,
        10,
        Nearest,
        "-1.3262",
        "-0x1.538#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        6,
        10,
        Ceiling,
        "-2.1758",
        "-0x2.2d#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        12,
        10,
        Floor,
        "1.0273",
        "0x1.070#10",
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
        "1.1547005383792515",
        "0x1.279a74590331c#53",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        6,
        10,
        Floor,
        "-1.1562",
        "-0x1.280#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        12,
        10,
        Nearest,
        "1.1543",
        "0x1.278#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        360,
        53,
        Nearest,
        "-1.0154266118857449",
        "-0x1.03f2ff9989906#53",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1099511627776,
        10,
        Ceiling,
        "17.531",
        "0x11.88#10",
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
        "-0.75",
        "-0x0.c#2",
        3,
        10,
        Ceiling,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        6,
        10,
        Nearest,
        "-1.4141",
        "-0x1.6a0#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        7,
        10,
        Ceiling,
        "-1.6035",
        "-0x1.9a8#10",
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
        "1591549430.9189534",
        "0x5edd1df6.eb4088#53",
        Greater,
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
        "3.8633",
        "0x3.dd#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        4,
        10,
        Ceiling,
        "5.1328",
        "0x5.22#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        7,
        10,
        Nearest,
        "8.9375",
        "0x8.f0#10",
        Greater,
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
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Greater,
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
        "-1.4141",
        "-0x1.6a0#10",
        Greater,
    );
    test(
        "-9.00",
        "-0x9.0#4",
        8,
        10,
        Nearest,
        "-1.4141",
        "-0x1.6a0#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 12, 1, Nearest, "2.0", "0x2.0#1", Equal);
    test("1.0", "0x1.0#1", 12, 10, Up, "2.0000", "0x2.00#10", Equal);
    test(
        "5.0",
        "0x5.0#3",
        12,
        10,
        Ceiling,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    test(
        "-7.0",
        "-0x7.0#3",
        12,
        10,
        Nearest,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    test("11.0", "0xb.0#4", 12, 1, Nearest, "-2.0", "-0x2.0#1", Equal);
    test(
        "11.0",
        "0xb.0#4",
        12,
        10,
        Up,
        "-2.0000",
        "-0x2.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        10,
        Ceiling,
        "1.0527",
        "0x1.0d8#10",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        5,
        10,
        Nearest,
        "-1.7012",
        "-0x1.b38#10",
        Greater,
    );
    test("3.0", "0x3.0#2", 5, 1, Nearest, "-2.0", "-0x2.0#1", Less);
    test("3.0", "0x3.0#2", 5, 10, Up, "-1.7031", "-0x1.b40#10", Less);
    test(
        "4.0",
        "0x4.0#1",
        5,
        10,
        Ceiling,
        "-1.0508",
        "-0x1.0d0#10",
        Greater,
    );
    test(
        "-6.0",
        "-0x6.0#2",
        5,
        10,
        Nearest,
        "-1.0508",
        "-0x1.0d0#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 10, 1, Nearest, "2.0", "0x2.0#1", Greater);
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Up,
        "1.7031",
        "0x1.b40#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        10,
        Ceiling,
        "1.0527",
        "0x1.0d8#10",
        Greater,
    );
    test(
        "-7.0",
        "-0x7.0#3",
        10,
        10,
        Nearest,
        "1.0508",
        "0x1.0d0#10",
        Less,
    );
    test("9.00", "0x9.0#4", 10, 1, Nearest, "-2.0", "-0x2.0#1", Less);
    test(
        "9.00",
        "0x9.0#4",
        10,
        10,
        Up,
        "-1.7031",
        "-0x1.b40#10",
        Less,
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
    test("30.0", "0x1e.0#4", 360, 1, Nearest, "2.0", "0x2.0#1", Equal);
    test(
        "30.0",
        "0x1e.0#4",
        360,
        10,
        Up,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    test(
        "150.0",
        "0x96.0#7",
        360,
        10,
        Ceiling,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    test(
        "-72.0",
        "-0x48.0#4",
        360,
        10,
        Nearest,
        "-1.0508",
        "-0x1.0d0#10",
        Greater,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        10,
        Up,
        "1.7031",
        "0x1.b40#10",
        Greater,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        10,
        Ceiling,
        "1.7031",
        "0x1.b40#10",
        Greater,
    );
    test(
        "-108.0",
        "-0x6c.0#5",
        360,
        10,
        Nearest,
        "-1.0508",
        "-0x1.0d0#10",
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
        "-1.0508",
        "-0x1.0d0#10",
        Greater,
    );
    test("6.0", "0x6.0#2", 60, 1, Nearest, "2.0", "0x2.0#1", Greater);
    test(
        "6.0",
        "0x6.0#2",
        60,
        10,
        Up,
        "1.7031",
        "0x1.b40#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        20,
        30,
        Ceiling,
        "1.2360679787",
        "0x1.3c6ef378#30",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        20,
        30,
        Ceiling,
        "3.2360679805",
        "0x3.3c6ef38#30",
        Greater,
    );
    test(
        "13.0",
        "0xd.0#4",
        20,
        30,
        Ceiling,
        "-1.2360679768",
        "-0x1.3c6ef370#30",
        Greater,
    );
    test(
        "19.0",
        "0x13.0#5",
        20,
        30,
        Ceiling,
        "-3.2360679768",
        "-0x3.3c6ef37#30",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        6,
        30,
        Floor,
        "1.1547005381",
        "0x1.279a7458#30",
        Less,
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
        "2.0", "0x2.0#2", 4, 10, Exact, "Infinity", "Infinity", Equal,
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
        "1.0508",
        "0x1.0d0#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        20,
        Nearest,
        "1.0514622",
        "0x1.0d2ca#20",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        20,
        Floor,
        "1.0514622",
        "0x1.0d2ca#20",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        20,
        Ceiling,
        "1.0514641",
        "0x1.0d2cc#20",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        5,
        20,
        Nearest,
        "1.7013016",
        "0x1.b3888#20",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        5,
        20,
        Nearest,
        "-1.7013016",
        "-0x1.b3888#20",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        5,
        20,
        Nearest,
        "-1.0514622",
        "-0x1.0d2ca#20",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        5,
        20,
        Nearest,
        "-1.0514622",
        "-0x1.0d2ca#20",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        5,
        20,
        Nearest,
        "-1.7013016",
        "-0x1.b3888#20",
        Greater,
    );
    test(
        "6.0",
        "0x6.0#2",
        5,
        20,
        Nearest,
        "1.0514622",
        "0x1.0d2ca#20",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Nearest,
        "1.7012",
        "0x1.b38#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        20,
        Nearest,
        "1.7013016",
        "0x1.b3888#20",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        20,
        Floor,
        "1.7013016",
        "0x1.b3888#20",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        20,
        Ceiling,
        "1.7013035",
        "0x1.b388a#20",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        20,
        Nearest,
        "1.0514622",
        "0x1.0d2ca#20",
        Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        10,
        20,
        Nearest,
        "-1.0514622",
        "-0x1.0d2ca#20",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        20,
        Nearest,
        "-1.7013016",
        "-0x1.b3888#20",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        20,
        Nearest,
        "-1.7013016",
        "-0x1.b3888#20",
        Greater,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        10,
        20,
        Nearest,
        "-1.0514622",
        "-0x1.0d2ca#20",
        Greater,
    );
    test(
        "11.0",
        "0xb.0#4",
        10,
        20,
        Nearest,
        "1.7013016",
        "0x1.b3888#20",
        Less,
    );
    test(
        "72.0",
        "0x48.0#4",
        360,
        20,
        Nearest,
        "1.0514622",
        "0x1.0d2ca#20",
        Less,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        20,
        Nearest,
        "1.7013016",
        "0x1.b3888#20",
        Less,
    );
    test(
        "108.0",
        "0x6c.0#5",
        360,
        20,
        Nearest,
        "1.0514622",
        "0x1.0d2ca#20",
        Less,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        20,
        Nearest,
        "1.7013016",
        "0x1.b3888#20",
        Less,
    );
}

#[test]
#[should_panic]
fn csc_with_period_prec_round_fail_1() {
    Float::ONE.csc_with_period_prec_round(4, 0, Floor);
}

#[test]
#[should_panic]
fn csc_with_period_prec_round_fail_2() {
    Float::from_unsigned_prec(1u32, 10)
        .0
        .csc_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn csc_with_period_prec_round_ref_fail() {
    Float::ONE.csc_with_period_prec_round_ref(4, 0, Floor);
}

#[test]
#[should_panic]
fn csc_with_period_prec_fail() {
    Float::ONE.csc_with_period_prec(4, 0);
}

#[test]
#[should_panic]
fn csc_with_period_round_fail() {
    Float::from_unsigned_prec(1u32, 10)
        .0
        .csc_with_period_round(7, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn csc_with_period_prec_round_properties_helper(x: Float, u: u64, prec: u64, rm: RoundingMode) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (t, o) = x.csc_with_period_prec_round_ref(u, prec, Nearest);
        if o == Equal {
            let (te, oe) = x.csc_with_period_prec_round_ref(u, prec, Exact);
            assert_eq!(ComparableFloatRef(&te), ComparableFloatRef(&t));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(x.csc_with_period_prec_round_ref(u, prec, Exact));
        }
        return;
    }
    let (t, o) = x.clone().csc_with_period_prec_round(u, prec, rm);
    assert!(t.is_valid());
    assert_rounding_ordering_consistent(&t, rm, o);

    let (t_alt, o_alt) = x.csc_with_period_prec_round_ref(u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    let mut t_alt = x.clone();
    let o_alt = t_alt.csc_with_period_prec_round_assign(u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    // csc_with_period is NaN exactly for u = 0 and non-finite x, and otherwise at least 1 in
    // magnitude
    assert_eq!(t.is_nan(), u == 0 || !x.is_finite());
    if !t.is_nan() {
        assert!(PartialOrdAbs::ge_abs(&t, &1u32));
        // an infinity is either a pole, which is exact, or an overflow, which is not; a finite
        // result carries the requested precision
        if t.is_normal() {
            assert_eq!(t.get_prec(), Some(prec));
        }
        // csc_with_period is odd
        let (t_neg, o_neg) = (-&x).csc_with_period_prec_round(u, prec, -rm);
        assert_eq!(ComparableFloatRef(&t_neg), ComparableFloatRef(&-&t));
        assert_eq!(o_neg, o.reverse());
        // csc_with_period has period u, except at a pole, where the infinity's sign follows the
        // sign of x rather than the angle
        if x.is_finite() && u != 0 && !t.is_infinite() {
            let (shifted, os) =
                x.add_prec_round_ref_val(Float::from(u), x.significant_bits() + 64, Nearest);
            if os == Equal {
                let (t_shifted, o_shifted) = shifted.csc_with_period_prec_round(u, prec, rm);
                assert_eq!(ComparableFloatRef(&t_shifted), ComparableFloatRef(&t));
                assert_eq!(o_shifted, o);
            }
        }
        // the cosecant is the reciprocal of the sine in the same units
        if x.is_finite()
            && u != 0
            && let Some((t_alt, o_alt)) = csc_with_period_naive(&x, u, prec, rm)
        {
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (t2, oo) = x.csc_with_period_prec_round_ref(u, prec, rm);
            assert_eq!(ComparableFloatRef(&t2), ComparableFloatRef(&t));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.csc_with_period_prec_round_ref(u, prec, Exact));
    }
}

#[test]
fn csc_with_period_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, u, prec, rm)| {
            csc_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_18().test_properties(
        |(x, u, prec, rm)| {
            csc_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // csc_with_period(±0) = ±infinity and csc_with_period(x, 0) = NaN, exactly
        let (t, o) = Float::ZERO.csc_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::INFINITY));
        assert_eq!(o, Equal);
        let (t, o) = Float::NEGATIVE_ZERO.csc_with_period_prec_round(4, prec, rm);
        assert_eq!(
            ComparableFloat(t),
            ComparableFloat(Float::NEGATIVE_INFINITY)
        );
        assert_eq!(o, Equal);
        let (t, o) = Float::ONE.csc_with_period_prec_round(0, prec, rm);
        assert!(t.is_nan());
        assert_eq!(o, Equal);
        // exact cases: quarter turns are the cosecant's ±1 values and its poles
        for (k, expected) in [
            (0u32, Float::INFINITY),
            (1, Float::one_prec(prec)),
            (2, Float::INFINITY),
            (3, -Float::one_prec(prec)),
            (4, Float::INFINITY),
        ] {
            let (t, o) = Float::from(k).csc_with_period_prec_round(4, prec, rm);
            assert_eq!(ComparableFloat(t), ComparableFloat(expected));
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn csc_with_period_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, u, prec)| {
        let (t, o) = x.clone().csc_with_period_prec(u, prec);
        assert!(t.is_valid());
        let (t_alt, o_alt) = x.csc_with_period_prec_ref(u, prec);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.csc_with_period_prec_round_ref(u, prec, Nearest);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.csc_with_period_prec_assign(u, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn csc_with_period_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_39().test_properties(|(x, u, rm)| {
        if rm == Exact && x.csc_with_period_round_ref(u, Nearest).1 != Equal {
            assert_panic!(x.csc_with_period_round_ref(u, Exact));
            return;
        }
        let (t, o) = x.clone().csc_with_period_round(u, rm);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = x.csc_with_period_round_ref(u, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.csc_with_period_prec_round_ref(u, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.csc_with_period_round_assign(u, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn csc_with_period_properties() {
    float_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, u)| {
        let t = x.clone().csc_with_period(u);
        assert!(t.is_valid());
        let t_alt = x.csc_with_period_ref(u);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let mut t_alt = x.clone();
        t_alt.csc_with_period_assign(u);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let (t_alt, _) = x.csc_with_period_prec_round_ref(u, x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        // csc_with_period is odd
        assert_eq!(
            ComparableFloatRef(&(-&x).csc_with_period_ref(u)),
            ComparableFloatRef(&-&t)
        );
    });
}

// Inputs within 2^(-2^30) of a half turn, whose cosecants overflow. The near-zero path works with
// the exact distance to the multiple of 1/2, so no 2^30-bit pi is ever formed, but the inputs
// themselves have 2^30 bits.
#[test]
fn test_csc_with_period_overflow() {
    let max = Float::max_finite_value_with_prec(10);
    let eps = Rational::power_of_2(-((1i64 << 30) + 70));
    let p = (1u64 << 30) + 74;
    // just past a half turn: x/u = 1/2 + 2^(-2^30 - 72), so the sine is negative and tiny and the
    // cosecant is negative and huge, beyond the largest finite `Float`
    let above = Float::from_rational_prec_round(Rational::from(2u32) + &eps, p, Exact).0;
    let (s, o) = above.csc_with_period_prec_round_ref(4, 10, Nearest);
    assert_eq!(
        ComparableFloat(s),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    let (s, o) = above.csc_with_period_prec_round_ref(4, 10, Up);
    assert_eq!(
        ComparableFloat(s),
        ComparableFloat(Float::NEGATIVE_INFINITY)
    );
    assert_eq!(o, Less);
    let (s, o) = above.csc_with_period_prec_round_ref(4, 10, Ceiling);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&-max.clone()));
    assert_eq!(o, Greater);
    // just below a half turn: the sine is positive and tiny, so the cosecant is positive and huge
    let below = Float::from_rational_prec_round(Rational::from(2u32) - eps, p, Exact).0;
    let (s, o) = below.csc_with_period_prec_round_ref(4, 10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (s, o) = below.csc_with_period_prec_round_ref(4, 10, Up);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::INFINITY));
    assert_eq!(o, Greater);
    let (s, o) = below.csc_with_period_prec_round_ref(4, 10, Down);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&max));
    assert_eq!(o, Less);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_csc_with_period() {
    fn test<T: PrimitiveFloat>(x: T, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_csc_with_period(x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 360, f32::NAN);
    test::<f32>(f32::INFINITY, 360, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, 360, f32::NAN);
    test::<f32>(1.0, 0, f32::NAN);
    test::<f32>(0.0, 360, f32::INFINITY);
    test::<f32>(-0.0, 360, f32::NEGATIVE_INFINITY);
    test::<f32>(90.0, 360, 1.0);
    test::<f32>(-90.0, 360, -1.0);
    test::<f32>(270.0, 360, -1.0);
    test::<f32>(180.0, 360, f32::INFINITY);
    test::<f32>(-180.0, 360, f32::NEGATIVE_INFINITY);
    test::<f32>(360.0, 360, f32::INFINITY);
    test::<f32>(45.0, 360, core::f32::consts::SQRT_2);
    test::<f32>(135.0, 360, core::f32::consts::SQRT_2);
    test::<f32>(30.0, 360, 2.0);
    test::<f32>(60.0, 360, 1.1547005);
    test::<f32>(120.0, 360, 1.1547005);
    test::<f32>(1.0, 7, 1.279048);
    test::<f32>(-1.0, 7, -1.279048);
    test::<f32>(2.0, 7, 1.0257169);
    test::<f32>(1.0, 360, 57.298687);
    test::<f32>(100.0, 360, 1.0154266);
    test::<f32>(10000000000.0, 360, -1.0154266);
    test::<f32>(1.0e30, 7, 1.279048);
    test::<f32>(1.0e-30, 7, 1.1140846e30);
    test::<f32>(3.4028235e38, 360, f32::INFINITY);
    test::<f32>(0.5, 1, f32::INFINITY);
    test::<f32>(0.25, 1, 1.0);
    test::<f32>(0.1, 1, 1.7013016);
    test::<f32>(1.0e-45, 1, f32::INFINITY);
    test::<f32>(1.0e-45, 360, f32::INFINITY);
    test::<f64>(f64::NAN, 360, f64::NAN);
    test::<f64>(f64::INFINITY, 360, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, 360, f64::NAN);
    test::<f64>(1.0, 0, f64::NAN);
    test::<f64>(0.0, 360, f64::INFINITY);
    test::<f64>(-0.0, 360, f64::NEGATIVE_INFINITY);
    test::<f64>(90.0, 360, 1.0);
    test::<f64>(-90.0, 360, -1.0);
    test::<f64>(270.0, 360, -1.0);
    test::<f64>(180.0, 360, f64::INFINITY);
    test::<f64>(-180.0, 360, f64::NEGATIVE_INFINITY);
    test::<f64>(360.0, 360, f64::INFINITY);
    test::<f64>(45.0, 360, core::f64::consts::SQRT_2);
    test::<f64>(135.0, 360, core::f64::consts::SQRT_2);
    test::<f64>(30.0, 360, 2.0);
    test::<f64>(60.0, 360, 1.1547005383792515);
    test::<f64>(120.0, 360, 1.1547005383792515);
    test::<f64>(1.0, 7, 1.2790480076899327);
    test::<f64>(-1.0, 7, -1.2790480076899327);
    test::<f64>(2.0, 7, 1.025716863272554);
    test::<f64>(1.0, 360, 57.298688498550185);
    test::<f64>(100.0, 360, 1.015426611885745);
    test::<f64>(10000000000.0, 360, -1.015426611885745);
    test::<f64>(1.0e100, 7, 1.025716863272554);
    test::<f64>(1.0e-100, 7, 1.1140846016432672e100);
    test::<f64>(1.7976931348623157e308, 360, 1.2690182150725788);
    test::<f64>(0.5, 1, f64::INFINITY);
    test::<f64>(0.25, 1, 1.0);
    test::<f64>(0.1, 1, 1.7013016167040798);
    test::<f64>(5.0e-324, 1, f64::INFINITY);
    test::<f64>(5.0e-324, 360, f64::INFINITY);
    test::<f32>(72.0, 360, 1.0514622);
    test::<f32>(36.0, 360, 1.7013016);
    test::<f32>(108.0, 360, 1.0514622);
    test::<f32>(144.0, 360, 1.7013016);
    test::<f64>(72.0, 360, 1.0514622242382672);
    test::<f64>(36.0, 360, 1.7013016167040798);
    test::<f64>(108.0, 360, 1.0514622242382672);
    test::<f64>(144.0, 360, 1.7013016167040798);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_csc_with_period_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        let s = primitive_float_csc_with_period(x, u);
        // NaN exactly for u = 0 (the inputs are finite)
        assert_eq!(s.is_nan(), u == 0);
        if u != 0 {
            // the cosecant is at least 1 in magnitude, so it never underflows
            assert!(s.abs() >= T::ONE);
            // odd
            assert_eq!(
                NiceFloat(primitive_float_csc_with_period(-x, u)),
                NiceFloat(-s)
            );
            // the same as the `Float` cosecant taken with 64 bits to spare and rounded once
            let (s_float, _) =
                Float::csc_with_period_prec(Float::from(x), u, T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&s_float, Nearest).0),
                NiceFloat(s)
            );
            // a pole is an infinity in both, but the cosecant of a tiny angle can exceed the
            // largest finite `T` while the `Float` cosecant, with its far wider exponent range,
            // stays finite
            if s_float.is_infinite() {
                assert!(s.is_infinite());
            }
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // NaN exactly for NaN and infinite inputs
        assert_eq!(
            primitive_float_csc_with_period(x, 7).is_nan(),
            !x.is_finite()
        );
    });
}

#[test]
fn primitive_float_csc_with_period_properties() {
    apply_fn_to_primitive_floats!(primitive_float_csc_with_period_properties_helper);
}
