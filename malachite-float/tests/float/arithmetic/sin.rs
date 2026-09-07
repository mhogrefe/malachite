// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{PowerOf2, Sin, SinAssign};
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
use malachite_float::float::arithmetic::sin::{
    primitive_float_sin, primitive_float_sin_pi, primitive_float_sin_pi_rational,
    primitive_float_sin_rational, primitive_float_sin_with_period,
    primitive_float_sin_with_period_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::sin::{
    rug_sin, rug_sin_pi_prec_round, rug_sin_pi_rational_prec_round, rug_sin_prec,
    rug_sin_prec_round, rug_sin_rational_prec, rug_sin_rational_prec_round, rug_sin_round,
    rug_sin_with_period_prec, rug_sin_with_period_prec_round, rug_sin_with_period_rational_prec,
    rug_sin_with_period_rational_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36, float_unsigned_rounding_mode_triple_gen_var_37,
    float_unsigned_rounding_mode_triple_gen_var_39,
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

// Rows reuse the cosine test's inputs. Branches of `sin_prec_round_normal_ref` covered:
// - tiny x: the small-input shortcut rounds x directly
// - x below 1: the working precision grows by -2 EXP(x) to absorb the cancellation in 1 - cos^2
// - |x| >= 2: argument reduction modulo 2 pi
// - the first working precision can round; it cannot, and the loop retries
// - cancellation: sin(x) is much smaller than the working precision resolves, so the precision
//   grows by the lost bits
#[test]
fn test_sin_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().sin_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.sin_prec_round_ref(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.sin_prec_round_assign(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_sin_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
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
    // - the small-input shortcut applies, but cannot round: x is a power of 2 stored at a precision
    //   above the error bound, so the general path takes over
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Nearest,
        "0.25",
        "0x0.4#1",
        Greater,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Floor,
        "0.12",
        "0x0.2#1",
        Less,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        1,
        Ceiling,
        "0.25",
        "0x0.4#1",
        Greater,
    );
    test(
        "0.25000000000000000000000000000000",
        "0x0.40000000000000000000000000#100",
        2,
        Nearest,
        "0.25",
        "0x0.4#2",
        Greater,
    );
    test("0.25", "0x0.4#1", 1, Floor, "0.12", "0x0.2#1", Less);
    test("0.25", "0x0.4#1", 1, Nearest, "0.25", "0x0.4#1", Greater);
    test("0.25", "0x0.4#1", 1, Ceiling, "0.25", "0x0.4#1", Greater);
    test("NaN", "NaN", 1, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 1, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", 10, Floor, "0.84082", "0x0.d74#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "0.84180",
        "0x0.d78#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        Nearest,
        "0.84180",
        "0x0.d78#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Floor,
        "0.84147098480789650665250232163005",
        "0x0.d76aa47848677020c6e9e909c#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Ceiling,
        "0.84147098480789650665250232163084",
        "0x0.d76aa47848677020c6e9e909d#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Nearest,
        "0.84147098480789650665250232163005",
        "0x0.d76aa47848677020c6e9e909c#100",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-0.84180",
        "-0x0.d78#10",
        Less,
    );
    test("2.0", "0x2.0#1", 10, Nearest, "0.90918", "0x0.e8c#10", Less);
    test("3.0", "0x3.0#2", 10, Nearest, "0.14111", "0x0.242#10", Less);
    test(
        "4.0",
        "0x4.0#1",
        10,
        Nearest,
        "-0.75684",
        "-0x0.c1c#10",
        Less,
    );
    test(
        "4.0",
        "0x4.0#1",
        100,
        Nearest,
        "-0.75680249530792825137263909451172",
        "-0x0.c1bdceeee0f5738674c02ad07#100",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        10,
        Nearest,
        "-0.50684",
        "-0x0.81c#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Floor,
        "-0.50636564110975879365655761046048",
        "-0x0.81a12dbc626dc03847b0aae85#100",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Ceiling,
        "-0.50636564110975879365655761045969",
        "-0x0.81a12dbc626dc03847b0aae84#100",
        Greater,
    );
    test(
        "1.00000e6",
        "0xf.424E+4#14",
        64,
        Nearest,
        "-0.349993502171292952130",
        "-0x0.59992c95a3619d268#64",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        50,
        Nearest,
        "0.47942553860420301",
        "0x0.7abba1d12c17c#50",
        Greater,
    );
    test(
        "0.102",
        "0x0.1a#4",
        50,
        Nearest,
        "0.10138798815552963",
        "0x0.19f4902d55d1f8#50",
        Less,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        50,
        Nearest,
        "1.0004441719502211e-10",
        "0x6.e00000000000E-9#50",
        Greater,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        10,
        Floor,
        "9.9931e-11",
        "0x6.deE-9#10",
        Less,
    );
    test(
        "1.000e-10",
        "0x6.eE-9#7",
        10,
        Ceiling,
        "1.0004e-10",
        "0x6.e0E-9#10",
        Greater,
    );
    test(
        "1.570796326794896600",
        "0x1.921fb54442d183#57",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "3.14159265358979289",
        "0x3.243f6a8885a2f#54",
        53,
        Nearest,
        "3.4450928483976660e-16",
        "0x1.8d313198a2e03E-13#53",
        Less,
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
        "-0.90292261046853339",
        "-0x0.e725efaac80328#53",
        Less,
    );
    test("3.0", "0x3.0#2", 1, Nearest, "0.12", "0x0.2#1", Less);
    test("3.0", "0x3.0#2", 1, Floor, "0.12", "0x0.2#1", Less);
    test("3.0", "0x3.0#2", 1, Ceiling, "0.25", "0x0.4#1", Greater);
    test("3.0", "0x3.0#2", 2, Nearest, "0.12", "0x0.2#2", Less);
    test("0.25", "0x0.4#1", 1, Down, "0.12", "0x0.2#1", Less);
    test("1.0", "0x1.0#1", 1, Down, "0.50", "0x0.8#1", Less);
    test("2.0", "0x2.0#1", 1, Down, "0.50", "0x0.8#1", Less);
    test("4.0", "0x4.0#1", 1, Down, "-0.50", "-0x0.8#1", Greater);
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
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "1.5707963267948966192313216916397514420985846996875529104874722",
        "0x1.921fb54442d18469898cc51701b839a252049c1114cf98e804#200",
        100,
        Floor,
        "0.99999999999999999999999999999921",
        "0x0.fffffffffffffffffffffffff#100",
        Less,
    );
    test(
        "4.7123889803846898576939650749192543286",
        "0x4.b65f1fccc8748d3c9ca64f450528b0#120",
        120,
        Ceiling,
        "-0.99999999999999999999999999999999999925",
        "-0x0.ffffffffffffffffffffffffffffff#120",
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        64,
        Nearest,
        "-5.01655761266833202345e-20",
        "-0xe.ce675d1fc8f8cbbE-17#64",
        Greater,
    );
    // - 2 <= |x| < 3: the sign of sin(x) is the sign of x, so no argument reduction is needed (MPFR
    //   reduces every |x| >= 2); |x| = 3 is reduced
    test(
        "2.5",
        "0x2.8#3",
        10,
        Nearest,
        "0.59863",
        "0x0.994#10",
        Greater,
    );
    test(
        "-2.5",
        "-0x2.8#3",
        10,
        Floor,
        "-0.59863",
        "-0x0.994#10",
        Less,
    );
    test(
        "2.99976",
        "0x2.fff#14",
        20,
        Nearest,
        "0.14136171",
        "0x0.243048#20",
        Greater,
    );
    test("3.0", "0x3.0#2", 10, Nearest, "0.14111", "0x0.242#10", Less);
    test(
        "-3.0",
        "-0x3.0#2",
        10,
        Ceiling,
        "-0.14111",
        "-0x0.242#10",
        Greater,
    );
    // - x equals 2 pi at the working precision of the reduction, so the reduced argument is exactly
    //   zero; at precision 10 the loop retries at a higher precision, and at precision 100 the
    //   near-zero path is taken directly
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
    // - |x| = 2^(-2^30), the smallest positive Float: sin(x) is just below x, so rounding toward
    //   zero underflows to zero while the other modes return x
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Ceiling,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        Nearest,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Floor,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Down,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Up,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        1,
        Nearest,
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        Less,
    );
    // - x = 3 * 2^(-2^30) at precision 2, rounded to precision 1: sin(x) is just below the midpoint
    //   3 * 2^(-2^30), so Nearest rounds down
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
        "4.8e-323228497",
        "0x2.0E-268435456#1",
        Less,
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
        "-4.8e-323228497",
        "-0x2.0E-268435456#1",
        Greater,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        10,
        Nearest,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
}

#[test]
#[should_panic]
fn sin_prec_round_fail() {
    Float::ONE.sin_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn sin_prec_round_exact_fail() {
    Float::ONE.sin_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn sin_prec_fail() {
    Float::ONE.sin_prec(0);
}

#[test]
#[should_panic]
fn sin_round_fail() {
    Float::ONE.sin_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn sin_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (s, o) = x.clone().sin_prec_round(prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = x.sin_prec_round_ref(prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.sin_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_o) = rug_sin_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    }

    // sin is odd
    let (s_neg, o_neg) = (-&x).sin_prec_round(prec, -rm);
    assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
    assert_eq!(o_neg, o.reverse());

    // |sin x| <= 1
    if s.is_finite() {
        assert!(s.le_abs(&1u32));
    }
    if s.is_normal() {
        assert_eq!(s.get_prec(), Some(prec));
    }

    if o == Equal {
        // sin is exact only for x = 0 (and NaN, and ±inf): the result is rounding-mode-invariant
        if x.is_finite() {
            assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&x));
        }
        for rm2 in exhaustive_rounding_modes() {
            let (s2, o2) = x.sin_prec_round_ref(prec, rm2);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.sin_prec_round_ref(prec, Exact));
    }
}

#[test]
fn sin_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        sin_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.sin_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::INFINITY.sin_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_INFINITY.sin_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        let (s, o) = Float::ZERO.sin_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.sin_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
    });
}

#[test]
fn sin_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (s, o) = x.clone().sin_round(rm);
        assert!(s.is_valid());
        assert_rounding_ordering_consistent(&s, rm, o);
        let (s_alt, o_alt) = x.sin_round_ref(rm);
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.sin_round_assign(rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (s_alt, o_alt) = x.sin_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_s, rug_o) = rug_sin_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn sin_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (s, o) = x.clone().sin_prec(prec);
        assert!(s.is_valid());
        let (s_alt, o_alt) = x.sin_prec_ref(prec);
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.sin_prec_assign(prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (s_alt, o_alt) = x.sin_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (rug_s, rug_o) = rug_sin_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn sin_properties() {
    float_gen().test_properties(|x| {
        let s = x.clone().sin();
        assert!(s.is_valid());
        let s_alt = (&x).sin();
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));

        let mut x_alt = x.clone();
        x_alt.sin_assign();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));

        let (s_alt, _) = x.sin_prec_round_ref(x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_sin(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&s)
        );

        assert_eq!(ComparableFloat((-&x).sin()), ComparableFloat(-&s));
        if s.is_finite() {
            assert!(s.le_abs(&1u32));
        }
    });
}

// n * pi, with pi rounded to the nearest `prec` bits and the product exact
fn multiple_of_pi(n: i64, prec: u64) -> Float {
    Float::pi_prec(prec)
        .0
        .mul_prec_round(Float::from(n), prec + 64, Exact)
        .0
}

// Inputs close to a nonzero multiple of pi, where the sine is tiny and takes the near-zero path: n
// * pi with pi rounded to `prec_x` bits. The rows with `prec_x` at most 100 and a large output
// precision need several terms of the sin(delta) / delta series.
#[test]
fn test_sin_near_zero() {
    let test = |n: i64, prec_x: u64, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = multiple_of_pi(n, prec_x);
        let (s, o) = x.sin_prec_round_ref(prec, rm);
        assert!(s.is_valid());
        assert_eq!(s.to_string(), out);
        assert_eq!(to_hex_string(&s), out_hex);
        assert_eq!(o, o_out);

        let (rug_s, rug_o) = rug_sin_prec_round(
            &rug::Float::exact_from(&x),
            prec,
            rug_round_try_from_rounding_mode(rm).unwrap(),
        );
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    };
    // - the reduction cannot resolve the argument against 0 or pi at the working precision, and the
    //   working precision already exceeds the target by enough that the near-zero path is taken
    //   directly (prec_x well above prec)
    test(1, 150, 1, Up, "-1.4e-45", "-0x8.0E-38#1", Less);
    test(1, 150, 1, Floor, "-1.4e-45", "-0x8.0E-38#1", Less);
    test(1, 150, 1, Ceiling, "-7.0e-46", "-0x4.0E-38#1", Greater);
    test(1, 150, 1, Nearest, "-1.4e-45", "-0x8.0E-38#1", Less);
    test(1, 150, 10, Down, "-1.3767e-45", "-0x7.dcE-38#10", Greater);
    test(1, 150, 10, Up, "-1.3780e-45", "-0x7.deE-38#10", Less);
    test(1, 150, 10, Floor, "-1.3780e-45", "-0x7.deE-38#10", Less);
    test(1, 150, 10, Nearest, "-1.3780e-45", "-0x7.deE-38#10", Less);
    test(1, 190, 1, Down, "5.0e-60", "0x8.0E-50#1", Less);
    test(1, 190, 1, Up, "1.0e-59", "0x1.0E-49#1", Greater);
    test(1, 190, 1, Floor, "5.0e-60", "0x8.0E-50#1", Less);
    test(1, 190, 1, Ceiling, "1.0e-59", "0x1.0E-49#1", Greater);
    test(1, 190, 1, Nearest, "5.0e-60", "0x8.0E-50#1", Less);
    test(1, 190, 10, Down, "5.0854e-60", "0x8.2cE-50#10", Less);
    test(1, 190, 10, Up, "5.0951e-60", "0x8.30E-50#10", Greater);
    test(1, 190, 10, Floor, "5.0854e-60", "0x8.2cE-50#10", Less);
    test(1, 190, 10, Ceiling, "5.0951e-60", "0x8.30E-50#10", Greater);
    test(1, 190, 10, Nearest, "5.0951e-60", "0x8.30E-50#10", Greater);
    test(1, 200, 1, Down, "7.8e-62", "0x2.0E-51#1", Less);
    test(1, 200, 1, Up, "1.6e-61", "0x4.0E-51#1", Greater);
    test(1, 200, 1, Floor, "7.8e-62", "0x2.0E-51#1", Less);
    test(1, 200, 1, Ceiling, "1.6e-61", "0x4.0E-51#1", Greater);
    test(1, 200, 1, Nearest, "7.8e-62", "0x2.0E-51#1", Less);
    test(1, 200, 10, Down, "1.1410e-61", "0x2.efE-51#10", Less);
    test(1, 200, 10, Up, "1.1425e-61", "0x2.f0E-51#10", Greater);
    test(1, 200, 10, Floor, "1.1410e-61", "0x2.efE-51#10", Less);
    test(1, 200, 10, Ceiling, "1.1425e-61", "0x2.f0E-51#10", Greater);
    test(1, 200, 10, Nearest, "1.1425e-61", "0x2.f0E-51#10", Greater);
    test(3, 200, 1, Down, "3.1e-61", "0x8.0E-51#1", Less);
    test(3, 200, 1, Up, "6.2e-61", "0x1.0E-50#1", Greater);
    test(3, 200, 1, Floor, "3.1e-61", "0x8.0E-51#1", Less);
    test(3, 200, 1, Ceiling, "6.2e-61", "0x1.0E-50#1", Greater);
    test(3, 200, 1, Nearest, "3.1e-61", "0x8.0E-51#1", Less);
    test(3, 200, 10, Down, "3.4214e-61", "0x8.ccE-51#10", Less);
    test(3, 200, 10, Up, "3.4275e-61", "0x8.d0E-51#10", Greater);
    test(3, 200, 10, Floor, "3.4214e-61", "0x8.ccE-51#10", Less);
    test(3, 200, 10, Nearest, "3.4275e-61", "0x8.d0E-51#10", Greater);
    test(-1, 200, 1, Down, "-7.8e-62", "-0x2.0E-51#1", Greater);
    test(-1, 200, 1, Up, "-1.6e-61", "-0x4.0E-51#1", Less);
    test(-1, 200, 1, Floor, "-1.6e-61", "-0x4.0E-51#1", Less);
    test(-1, 200, 1, Ceiling, "-7.8e-62", "-0x2.0E-51#1", Greater);
    test(-1, 200, 1, Nearest, "-7.8e-62", "-0x2.0E-51#1", Greater);
    test(-1, 200, 10, Down, "-1.1410e-61", "-0x2.efE-51#10", Greater);
    test(-1, 200, 10, Up, "-1.1425e-61", "-0x2.f0E-51#10", Less);
    test(-1, 200, 10, Floor, "-1.1425e-61", "-0x2.f0E-51#10", Less);
    test(5, 300, 1, Down, "2.0e-90", "0x4.0E-75#1", Less);
    test(5, 300, 1, Up, "3.9e-90", "0x8.0E-75#1", Greater);
    test(5, 300, 1, Floor, "2.0e-90", "0x4.0E-75#1", Less);
    test(5, 300, 1, Ceiling, "3.9e-90", "0x8.0E-75#1", Greater);
    test(5, 300, 1, Nearest, "2.0e-90", "0x4.0E-75#1", Less);
    test(5, 300, 10, Down, "2.4661e-90", "0x5.06E-75#10", Less);
    test(5, 300, 10, Up, "2.4699e-90", "0x5.08E-75#10", Greater);
    test(5, 300, 10, Floor, "2.4661e-90", "0x5.06E-75#10", Less);
    test(5, 300, 10, Ceiling, "2.4699e-90", "0x5.08E-75#10", Greater);
    test(5, 300, 10, Nearest, "2.4661e-90", "0x5.06E-75#10", Less);
    test(-3, 300, 1, Down, "-9.8e-91", "-0x2.0E-75#1", Greater);
    test(-3, 300, 1, Up, "-2.0e-90", "-0x4.0E-75#1", Less);
    test(-3, 300, 1, Floor, "-2.0e-90", "-0x4.0E-75#1", Less);
    test(-3, 300, 1, Ceiling, "-9.8e-91", "-0x2.0E-75#1", Greater);
    test(-3, 300, 1, Nearest, "-2.0e-90", "-0x4.0E-75#1", Less);
    test(-3, 300, 10, Down, "-1.4785e-90", "-0x3.03E-75#10", Greater);
    test(-3, 300, 10, Up, "-1.4804e-90", "-0x3.04E-75#10", Less);
    test(-3, 300, 10, Floor, "-1.4804e-90", "-0x3.04E-75#10", Less);
    test(-3, 300, 10, Nearest, "-1.4804e-90", "-0x3.04E-75#10", Less);
    test(7, 1000, 1, Down, "7.5e-301", "0x8.0E-250#1", Less);
    test(7, 1000, 1, Up, "1.5e-300", "0x1.0E-249#1", Greater);
    test(7, 1000, 1, Floor, "7.5e-301", "0x8.0E-250#1", Less);
    test(7, 1000, 1, Ceiling, "1.5e-300", "0x1.0E-249#1", Greater);
    test(7, 1000, 1, Nearest, "7.5e-301", "0x8.0E-250#1", Less);
    test(7, 1000, 10, Up, "8.7639e-301", "0x9.64E-250#10", Greater);
    test(7, 1000, 10, Floor, "8.7493e-301", "0x9.60E-250#10", Less);
    test(1, 5000, 1, Down, "-7.1e-1506", "-0x1.0E-1250#1", Greater);
    test(1, 5000, 1, Up, "-1.4e-1505", "-0x2.0E-1250#1", Less);
    test(1, 5000, 1, Floor, "-1.4e-1505", "-0x2.0E-1250#1", Less);
    test(1, 5000, 1, Ceiling, "-7.1e-1506", "-0x1.0E-1250#1", Greater);
    test(1, 5000, 1, Nearest, "-7.1e-1506", "-0x1.0E-1250#1", Greater);
    test(1, 5000, 10, Up, "-9.7071e-1506", "-0x1.5f0E-1250#10", Less);
    test(1048577, 1000, 1, Down, "9.8e-296", "0x1.0E-245#1", Less);
    test(1048577, 1000, 1, Up, "2.0e-295", "0x2.0E-245#1", Greater);
    test(1048577, 1000, 1, Floor, "9.8e-296", "0x1.0E-245#1", Less);
    test(1048577, 1000, 1, Nearest, "9.8e-296", "0x1.0E-245#1", Less);
    test(1048579, 2000, 1, Up, "9.1e-597", "0x1.0E-495#1", Greater);
    test(1048579, 2000, 1, Floor, "4.6e-597", "0x8.0E-496#1", Less);
    // - 1 - cos(x)^2 is nonzero but tiny at the working precision, and the error bound alone shows
    //   sin(x) is tiny: the near-zero path is taken from a nonzero c (prec_x = 80 with prec = 170)
    // - 1 - cos(x)^2 rounds to zero at the working precision: the near-zero path is taken from a
    //   zero c (prec_x = 150 with prec = 200)
    test(
        1,
        80,
        170,
        Nearest,
        "1.2736634327021900279235533515370047227152097329098486e-24",
        "0x1.8a2e03707344a4093822299f31d0082efa98ec4dd10E-20#170",
        Greater,
    );
    test(
        1,
        80,
        170,
        Floor,
        "1.2736634327021900279235533515370047227152097329098475e-24",
        "0x1.8a2e03707344a4093822299f31d0082efa98ec4dd08E-20#170",
        Less,
    );
    test(
        1,
        80,
        170,
        Ceiling,
        "1.2736634327021900279235533515370047227152097329098486e-24",
        "0x1.8a2e03707344a4093822299f31d0082efa98ec4dd10E-20#170",
        Greater,
    );
    test(
        -1,
        80,
        170,
        Nearest,
        "-1.2736634327021900279235533515370047227152097329098486e-24",
        "-0x1.8a2e03707344a4093822299f31d0082efa98ec4dd10E-20#170",
        Less,
    );
    test(
        2,
        80,
        170,
        Nearest,
        "-2.5473268654043800558471067030740094454304194658176300e-24",
        "-0x3.145c06e0e68948127044533e63a0105df531d897fbE-20#170",
        Greater,
    );
    test(
        1,
        150,
        200,
        Nearest,
        "-1.3779234748660882634858956217182809760248956358110909616012260e-45",
        "-0x7.ddd660ce2ff7d1056713b19376bad7de19c72fec8841ab9930E-38#200",
        Greater,
    );
    test(
        1,
        150,
        200,
        Floor,
        "-1.3779234748660882634858956217182809760248956358110909616012269e-45",
        "-0x7.ddd660ce2ff7d1056713b19376bad7de19c72fec8841ab9938E-38#200",
        Less,
    );
    test(
        -3,
        150,
        200,
        Nearest,
        "4.1337704245982647904576868651548429280746869074332728848036797e-45",
        "0x1.79983226a8fe77310353b14ba6430879a4d558fc598c502cbaE-37#200",
        Greater,
    );
    test(
        2,
        150,
        200,
        Ceiling,
        "2.7558469497321765269717912434365619520497912716221819232024537e-45",
        "0xf.bbacc19c5fefa20ace276326ed75afbc338e5fd9108357327E-38#200",
        Greater,
    );
}

// Inputs within 2^(-2^30) of a nonzero multiple of pi, whose sines underflow. Constructing an input
// and each call computes pi to about 2^30 bits (~6 minutes each), so this test is slow even in
// release mode; the two calls cover both signs, rounding to zero and rounding away from it, and
// `Nearest` and a directed mode.
#[test]
fn test_sin_underflow() {
    let p = (1u64 << 30) + 64;
    // 2^(-2^30), the smallest positive `Float`, at the output precision
    let min_positive = Float::one_prec(10) >> (1u64 << 30);
    assert_eq!(min_positive.get_exponent(), Some(Float::MIN_EXPONENT));
    // pi rounded down: x_lo < pi, so sin(x_lo) is positive, and at most half an ulp of x_lo,
    // 2^(-2^30 - 63), in magnitude
    let mut x_lo = Float::pi_prec_round(p, Floor).0;
    let (s, o) = x_lo.sin_prec_round_ref(10, Ceiling);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&min_positive));
    assert_eq!(o, Greater);
    // pi rounded up: x_hi > pi, so sin(x_hi) is negative
    x_lo.increment();
    let x_hi = x_lo;
    let (s, o) = x_hi.sin_prec_round_ref(10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
}

#[test]
fn test_sin_rational_prec() {
    let test = |s, prec, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::sin_rational_prec(x.clone(), prec);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        let (c, o) = Float::sin_rational_prec_ref(&x, prec);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);
    };
    test("0", 1, "0.0", "0x0.0", Equal);
    test("0", 5, "0.0", "0x0.0", Equal);
    test("0", 10, "0.0", "0x0.0", Equal);
    test("0", 20, "0.0", "0x0.0", Equal);
    test("0", 53, "0.0", "0x0.0", Equal);
    test("0", 100, "0.0", "0x0.0", Equal);
    test("1", 1, "1.0", "0x1.0#1", Greater);
    test("1", 5, "0.844", "0x0.d8#5", Greater);
    test("1", 10, "0.84180", "0x0.d78#10", Greater);
    test("1", 20, "0.84147072", "0x0.d76aa#20", Less);
    test(
        "1",
        53,
        "0.84147098480789650",
        "0x0.d76aa478486770#53",
        Less,
    );
    test(
        "1",
        100,
        "0.84147098480789650665250232163005",
        "0x0.d76aa47848677020c6e9e909c#100",
        Less,
    );
    test("-1", 1, "-1.0", "-0x1.0#1", Less);
    test("-1", 5, "-0.844", "-0x0.d8#5", Less);
    test("-1", 10, "-0.84180", "-0x0.d78#10", Less);
    test("-1", 20, "-0.84147072", "-0x0.d76aa#20", Greater);
    test(
        "-1",
        53,
        "-0.84147098480789650",
        "-0x0.d76aa478486770#53",
        Greater,
    );
    test(
        "-1",
        100,
        "-0.84147098480789650665250232163005",
        "-0x0.d76aa47848677020c6e9e909c#100",
        Greater,
    );
    test("1/2", 1, "0.50", "0x0.8#1", Greater);
    test("1/2", 5, "0.484", "0x0.7c#5", Greater);
    test("1/2", 10, "0.47949", "0x0.7ac#10", Greater);
    test("1/2", 20, "0.47942543", "0x0.7abba0#20", Less);
    test(
        "1/2",
        53,
        "0.47942553860420301",
        "0x0.7abba1d12c17c0#53",
        Greater,
    );
    test(
        "1/2",
        100,
        "0.47942553860420300027328793521567",
        "0x0.7abba1d12c17bfa1d92f0d93f8#100",
        Greater,
    );
    test("1/3", 1, "0.25", "0x0.4#1", Less);
    test("1/3", 5, "0.328", "0x0.54#5", Greater);
    test("1/3", 10, "0.32715", "0x0.53c#10", Less);
    test("1/3", 20, "0.32719469", "0x0.53c308#20", Less);
    test(
        "1/3",
        53,
        "0.32719469679615226",
        "0x0.53c3081a2a031c#53",
        Greater,
    );
    test(
        "1/3",
        100,
        "0.32719469679615224417334408526753",
        "0x0.53c3081a2a031ab144c484c790#100",
        Less,
    );
    test("-1/3", 1, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 5, "-0.328", "-0x0.54#5", Less);
    test("-1/3", 10, "-0.32715", "-0x0.53c#10", Greater);
    test("-1/3", 20, "-0.32719469", "-0x0.53c308#20", Greater);
    test(
        "-1/3",
        53,
        "-0.32719469679615226",
        "-0x0.53c3081a2a031c#53",
        Less,
    );
    test(
        "-1/3",
        100,
        "-0.32719469679615224417334408526753",
        "-0x0.53c3081a2a031ab144c484c790#100",
        Greater,
    );
    test("3/5", 1, "0.50", "0x0.8#1", Less);
    test("3/5", 10, "0.56445", "0x0.908#10", Less);
    test(
        "3/5",
        53,
        "0.56464247339503537",
        "0x0.908c68bd2a0ac8#53",
        Greater,
    );
    test(
        "3/5",
        100,
        "0.56464247339503535720094544565865",
        "0x0.908c68bd2a0ac6fa881b0a82b#100",
        Less,
    );
    test("22/7", 1, "-0.00098", "-0x0.004#1", Greater);
    test("22/7", 5, "-0.00128", "-0x0.0054#5", Less);
    test("22/7", 10, "-0.0012646", "-0x0.0052e#10", Less);
    test("22/7", 20, "-0.0012644883", "-0x0.0052de98#20", Greater);
    test(
        "22/7",
        53,
        "-0.0012644889303773533",
        "-0x0.0052de9a9a24d90c#53",
        Greater,
    );
    test(
        "22/7",
        100,
        "-0.0012644889303773534003603504756467",
        "-0x0.0052de9a9a24d90da7cdffe30130#100",
        Less,
    );
    test("-22/7", 1, "0.00098", "0x0.004#1", Less);
    test("-22/7", 5, "0.00128", "0x0.0054#5", Greater);
    test("-22/7", 10, "0.0012646", "0x0.0052e#10", Greater);
    test("-22/7", 20, "0.0012644883", "0x0.0052de98#20", Less);
    test(
        "-22/7",
        53,
        "0.0012644889303773533",
        "0x0.0052de9a9a24d90c#53",
        Less,
    );
    test(
        "-22/7",
        100,
        "0.0012644889303773534003603504756467",
        "0x0.0052de9a9a24d90da7cdffe30130#100",
        Greater,
    );
    test("355/113", 1, "-2.4e-7", "-0x4.0E-6#1", Greater);
    test("355/113", 5, "-2.68e-7", "-0x4.8E-6#5", Less);
    test("355/113", 10, "-2.6682e-7", "-0x4.7aE-6#10", Less);
    test("355/113", 20, "-2.6676435e-7", "-0x4.79be8E-6#20", Less);
    test(
        "355/113",
        53,
        "-2.6676418906241917e-7",
        "-0x4.79be53e7511d8E-6#53",
        Less,
    );
    test(
        "355/113",
        100,
        "-2.6676418906241914840637452887346e-7",
        "-0x4.79be53e7511d64791a3045970E-6#100",
        Greater,
    );
    test("3", 1, "0.12", "0x0.2#1", Less);
    test("3", 5, "0.141", "0x0.24#5", Less);
    test("3", 10, "0.14111", "0x0.242#10", Less);
    test("3", 20, "0.14111996", "0x0.242070#20", Less);
    test(
        "3",
        53,
        "0.14112000805986721",
        "0x0.242070db6daab6#53",
        Less,
    );
    test(
        "3",
        100,
        "0.14112000805986722210074480280816",
        "0x0.242070db6daab69e3902e84684#100",
        Greater,
    );
    test("100", 1, "-0.50", "-0x0.8#1", Greater);
    test("100", 5, "-0.500", "-0x0.80#5", Greater);
    test("100", 10, "-0.50684", "-0x0.81c#10", Less);
    test("100", 20, "-0.50636578", "-0x0.81a13#20", Less);
    test(
        "100",
        53,
        "-0.50636564110975879",
        "-0x0.81a12dbc626dc0#53",
        Greater,
    );
    test(
        "100",
        100,
        "-0.50636564110975879365655761045969",
        "-0x0.81a12dbc626dc03847b0aae84#100",
        Greater,
    );
    test("1000000", 1, "-0.25", "-0x0.4#1", Greater);
    test("1000000", 5, "-0.344", "-0x0.58#5", Greater);
    test("1000000", 10, "-0.35010", "-0x0.59a#10", Less);
    test("1000000", 20, "-0.34999371", "-0x0.599930#20", Less);
    test(
        "1000000",
        53,
        "-0.34999350217129294",
        "-0x0.59992c95a3619c#53",
        Greater,
    );
    test(
        "1000000",
        100,
        "-0.34999350217129295211765248678059",
        "-0x0.59992c95a3619d264732d26e98#100",
        Greater,
    );
    test("1/1000000", 1, "9.5e-7", "0x0.00001#1", Less);
    test("1/1000000", 5, "1.01e-6", "0x0.000011#5", Greater);
    test("1/1000000", 10, "1.0002e-6", "0x0.000010c8#10", Greater);
    test(
        "1/1000000",
        20,
        "1.0000003e-6",
        "0x0.000010c6f8#20",
        Greater,
    );
    test(
        "1/1000000",
        53,
        "9.9999999999983330e-7",
        "0x0.000010c6f7a0b5ea7a#53",
        Less,
    );
    test(
        "1/1000000",
        100,
        "9.9999999999983333333333334166635e-7",
        "0x0.000010c6f7a0b5ea7a2711cf92d7e2#100",
        Less,
    );
    test("-1/1000000", 1, "-9.5e-7", "-0x0.00001#1", Greater);
    test("-1/1000000", 5, "-1.01e-6", "-0x0.000011#5", Less);
    test("-1/1000000", 10, "-1.0002e-6", "-0x0.000010c8#10", Less);
    test(
        "-1/1000000",
        20,
        "-1.0000003e-6",
        "-0x0.000010c6f8#20",
        Less,
    );
    test(
        "-1/1000000",
        53,
        "-9.9999999999983330e-7",
        "-0x0.000010c6f7a0b5ea7a#53",
        Greater,
    );
    test(
        "-1/1000000",
        100,
        "-9.9999999999983333333333334166635e-7",
        "-0x0.000010c6f7a0b5ea7a2711cf92d7e2#100",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1,
        "8.3e-25",
        "0x1.0E-20#1",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        5,
        "9.82e-25",
        "0x1.3E-20#5",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        10,
        "1.0000e-24",
        "0x1.358E-20#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        20,
        "9.9999953e-25",
        "0x1.357c2E-20#20",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        53,
        "9.9999999999999992e-25",
        "0x1.357c299a88ea7E-20#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        100,
        "9.9999999999999999999999999999980e-25",
        "0x1.357c299a88ea76a58924d52ceE-20#100",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        5,
        "1.00",
        "0x1.0#5",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        20,
        "1.0000000",
        "0x1.00000#20",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        100,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        5,
        "-1.00",
        "-0x1.0#5",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        20,
        "-1.0000000",
        "-0x1.00000#20",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        100,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        "2.0e-31",
        "0x4.0E-26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        5,
        "1.73e-31",
        "0x3.8E-26#5",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        "1.6948e-31",
        "0x3.70E-26#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        20,
        "1.6956854e-31",
        "0x3.70734E-26#20",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        "1.6956855320737799e-31",
        "0x3.707344a409382E-26#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        100,
        "1.6956855320737799287917402938778e-31",
        "0x3.707344a4093822299f31d0084E-26#100",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        5,
        "-1.00",
        "-0x1.0#5",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        20,
        "-1.0000000",
        "-0x1.00000#20",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        100,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        5,
        "-0.875",
        "-0x0.e0#5",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        "-0.87207",
        "-0x0.df4#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        20,
        "-0.87218380",
        "-0x0.df477#20",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        "-0.87218360541826734",
        "-0x0.df476cbd60fac8#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        100,
        "-0.87218360541826730978071977821350",
        "-0x0.df476cbd60fac5f54ce60f245#100",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        "7.9e-31",
        "0x1.0E-25#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        5,
        "7.89e-31",
        "0x1.0E-25#5",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        "7.8886e-31",
        "0x1.000E-25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        20,
        "7.8886091e-31",
        "0x1.00000E-25#20",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        "7.8886090522101181e-31",
        "0x1.0000000000000E-25#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        100,
        "7.8886090522101180541172856528279e-31",
        "0x1.0000000000000000000000000E-25#100",
        Greater,
    );
}

#[test]
fn test_sin_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::sin_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);

        let (c, o) = Float::sin_rational_prec_round_ref(&x, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, out_o);
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
    test("1", 1, Down, "0.50", "0x0.8#1", Less);
    test("1", 1, Up, "1.0", "0x1.0#1", Greater);
    test("1", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1", 5, Nearest, "0.844", "0x0.d8#5", Greater);
    test("1", 10, Down, "0.84082", "0x0.d74#10", Less);
    test("1", 10, Up, "0.84180", "0x0.d78#10", Greater);
    test("1", 10, Floor, "0.84082", "0x0.d74#10", Less);
    test("1", 10, Ceiling, "0.84180", "0x0.d78#10", Greater);
    test("1", 10, Nearest, "0.84180", "0x0.d78#10", Greater);
    test("1", 20, Nearest, "0.84147072", "0x0.d76aa#20", Less);
    test(
        "1",
        53,
        Down,
        "0.84147098480789650",
        "0x0.d76aa478486770#53",
        Less,
    );
    test(
        "1",
        53,
        Up,
        "0.84147098480789662",
        "0x0.d76aa478486778#53",
        Greater,
    );
    test(
        "1",
        53,
        Floor,
        "0.84147098480789650",
        "0x0.d76aa478486770#53",
        Less,
    );
    test(
        "1",
        53,
        Ceiling,
        "0.84147098480789662",
        "0x0.d76aa478486778#53",
        Greater,
    );
    test(
        "1",
        53,
        Nearest,
        "0.84147098480789650",
        "0x0.d76aa478486770#53",
        Less,
    );
    test(
        "1",
        100,
        Nearest,
        "0.84147098480789650665250232163005",
        "0x0.d76aa47848677020c6e9e909c#100",
        Less,
    );
    test("-1", 1, Down, "-0.50", "-0x0.8#1", Greater);
    test("-1", 1, Up, "-1.0", "-0x1.0#1", Less);
    test("-1", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-1", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-1", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-1", 5, Nearest, "-0.844", "-0x0.d8#5", Less);
    test("-1", 10, Down, "-0.84082", "-0x0.d74#10", Greater);
    test("-1", 10, Up, "-0.84180", "-0x0.d78#10", Less);
    test("-1", 10, Floor, "-0.84180", "-0x0.d78#10", Less);
    test("-1", 10, Ceiling, "-0.84082", "-0x0.d74#10", Greater);
    test("-1", 10, Nearest, "-0.84180", "-0x0.d78#10", Less);
    test("-1", 20, Nearest, "-0.84147072", "-0x0.d76aa#20", Greater);
    test(
        "-1",
        53,
        Down,
        "-0.84147098480789650",
        "-0x0.d76aa478486770#53",
        Greater,
    );
    test(
        "-1",
        53,
        Up,
        "-0.84147098480789662",
        "-0x0.d76aa478486778#53",
        Less,
    );
    test(
        "-1",
        53,
        Floor,
        "-0.84147098480789662",
        "-0x0.d76aa478486778#53",
        Less,
    );
    test(
        "-1",
        53,
        Ceiling,
        "-0.84147098480789650",
        "-0x0.d76aa478486770#53",
        Greater,
    );
    test(
        "-1",
        53,
        Nearest,
        "-0.84147098480789650",
        "-0x0.d76aa478486770#53",
        Greater,
    );
    test(
        "-1",
        100,
        Nearest,
        "-0.84147098480789650665250232163005",
        "-0x0.d76aa47848677020c6e9e909c#100",
        Greater,
    );
    test("1/2", 1, Down, "0.25", "0x0.4#1", Less);
    test("1/2", 1, Up, "0.50", "0x0.8#1", Greater);
    test("1/2", 1, Floor, "0.25", "0x0.4#1", Less);
    test("1/2", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("1/2", 1, Nearest, "0.50", "0x0.8#1", Greater);
    test("1/2", 5, Nearest, "0.484", "0x0.7c#5", Greater);
    test("1/2", 10, Down, "0.47900", "0x0.7aa#10", Less);
    test("1/2", 10, Up, "0.47949", "0x0.7ac#10", Greater);
    test("1/2", 10, Floor, "0.47900", "0x0.7aa#10", Less);
    test("1/2", 10, Ceiling, "0.47949", "0x0.7ac#10", Greater);
    test("1/2", 10, Nearest, "0.47949", "0x0.7ac#10", Greater);
    test("1/2", 20, Nearest, "0.47942543", "0x0.7abba0#20", Less);
    test(
        "1/2",
        53,
        Down,
        "0.47942553860420295",
        "0x0.7abba1d12c17bc#53",
        Less,
    );
    test(
        "1/2",
        53,
        Up,
        "0.47942553860420301",
        "0x0.7abba1d12c17c0#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Floor,
        "0.47942553860420295",
        "0x0.7abba1d12c17bc#53",
        Less,
    );
    test(
        "1/2",
        53,
        Ceiling,
        "0.47942553860420301",
        "0x0.7abba1d12c17c0#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Nearest,
        "0.47942553860420301",
        "0x0.7abba1d12c17c0#53",
        Greater,
    );
    test(
        "1/2",
        100,
        Nearest,
        "0.47942553860420300027328793521567",
        "0x0.7abba1d12c17bfa1d92f0d93f8#100",
        Greater,
    );
    test("1/3", 1, Down, "0.25", "0x0.4#1", Less);
    test("1/3", 1, Up, "0.50", "0x0.8#1", Greater);
    test("1/3", 1, Floor, "0.25", "0x0.4#1", Less);
    test("1/3", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("1/3", 1, Nearest, "0.25", "0x0.4#1", Less);
    test("1/3", 5, Nearest, "0.328", "0x0.54#5", Greater);
    test("1/3", 10, Down, "0.32715", "0x0.53c#10", Less);
    test("1/3", 10, Up, "0.32764", "0x0.53e#10", Greater);
    test("1/3", 10, Floor, "0.32715", "0x0.53c#10", Less);
    test("1/3", 10, Ceiling, "0.32764", "0x0.53e#10", Greater);
    test("1/3", 10, Nearest, "0.32715", "0x0.53c#10", Less);
    test("1/3", 20, Nearest, "0.32719469", "0x0.53c308#20", Less);
    test(
        "1/3",
        53,
        Down,
        "0.32719469679615221",
        "0x0.53c3081a2a0318#53",
        Less,
    );
    test(
        "1/3",
        53,
        Up,
        "0.32719469679615226",
        "0x0.53c3081a2a031c#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Floor,
        "0.32719469679615221",
        "0x0.53c3081a2a0318#53",
        Less,
    );
    test(
        "1/3",
        53,
        Ceiling,
        "0.32719469679615226",
        "0x0.53c3081a2a031c#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Nearest,
        "0.32719469679615226",
        "0x0.53c3081a2a031c#53",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.32719469679615224417334408526753",
        "0x0.53c3081a2a031ab144c484c790#100",
        Less,
    );
    test("-1/3", 1, Down, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 1, Up, "-0.50", "-0x0.8#1", Less);
    test("-1/3", 1, Floor, "-0.50", "-0x0.8#1", Less);
    test("-1/3", 1, Ceiling, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 1, Nearest, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 5, Nearest, "-0.328", "-0x0.54#5", Less);
    test("-1/3", 10, Down, "-0.32715", "-0x0.53c#10", Greater);
    test("-1/3", 10, Up, "-0.32764", "-0x0.53e#10", Less);
    test("-1/3", 10, Floor, "-0.32764", "-0x0.53e#10", Less);
    test("-1/3", 10, Ceiling, "-0.32715", "-0x0.53c#10", Greater);
    test("-1/3", 10, Nearest, "-0.32715", "-0x0.53c#10", Greater);
    test(
        "-1/3",
        20,
        Nearest,
        "-0.32719469",
        "-0x0.53c308#20",
        Greater,
    );
    test(
        "-1/3",
        53,
        Down,
        "-0.32719469679615221",
        "-0x0.53c3081a2a0318#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Up,
        "-0.32719469679615226",
        "-0x0.53c3081a2a031c#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Floor,
        "-0.32719469679615226",
        "-0x0.53c3081a2a031c#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Ceiling,
        "-0.32719469679615221",
        "-0x0.53c3081a2a0318#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Nearest,
        "-0.32719469679615226",
        "-0x0.53c3081a2a031c#53",
        Less,
    );
    test(
        "-1/3",
        100,
        Nearest,
        "-0.32719469679615224417334408526753",
        "-0x0.53c3081a2a031ab144c484c790#100",
        Greater,
    );
    test("3/5", 1, Down, "0.50", "0x0.8#1", Less);
    test("3/5", 1, Up, "1.0", "0x1.0#1", Greater);
    test("3/5", 1, Floor, "0.50", "0x0.8#1", Less);
    test("3/5", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("3/5", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("3/5", 10, Down, "0.56445", "0x0.908#10", Less);
    test("3/5", 10, Up, "0.56543", "0x0.90c#10", Greater);
    test("3/5", 10, Floor, "0.56445", "0x0.908#10", Less);
    test("3/5", 10, Ceiling, "0.56543", "0x0.90c#10", Greater);
    test("3/5", 10, Nearest, "0.56445", "0x0.908#10", Less);
    test(
        "3/5",
        53,
        Down,
        "0.56464247339503526",
        "0x0.908c68bd2a0ac0#53",
        Less,
    );
    test(
        "3/5",
        53,
        Up,
        "0.56464247339503537",
        "0x0.908c68bd2a0ac8#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Floor,
        "0.56464247339503526",
        "0x0.908c68bd2a0ac0#53",
        Less,
    );
    test(
        "3/5",
        53,
        Ceiling,
        "0.56464247339503537",
        "0x0.908c68bd2a0ac8#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Nearest,
        "0.56464247339503537",
        "0x0.908c68bd2a0ac8#53",
        Greater,
    );
    test(
        "3/5",
        100,
        Nearest,
        "0.56464247339503535720094544565865",
        "0x0.908c68bd2a0ac6fa881b0a82b#100",
        Less,
    );
    test("22/7", 1, Down, "-0.00098", "-0x0.004#1", Greater);
    test("22/7", 1, Up, "-0.0020", "-0x0.008#1", Less);
    test("22/7", 1, Floor, "-0.0020", "-0x0.008#1", Less);
    test("22/7", 1, Ceiling, "-0.00098", "-0x0.004#1", Greater);
    test("22/7", 1, Nearest, "-0.00098", "-0x0.004#1", Greater);
    test("22/7", 5, Nearest, "-0.00128", "-0x0.0054#5", Less);
    test("22/7", 10, Down, "-0.0012627", "-0x0.0052c#10", Greater);
    test("22/7", 10, Up, "-0.0012646", "-0x0.0052e#10", Less);
    test("22/7", 10, Floor, "-0.0012646", "-0x0.0052e#10", Less);
    test("22/7", 10, Ceiling, "-0.0012627", "-0x0.0052c#10", Greater);
    test("22/7", 10, Nearest, "-0.0012646", "-0x0.0052e#10", Less);
    test(
        "22/7",
        20,
        Nearest,
        "-0.0012644883",
        "-0x0.0052de98#20",
        Greater,
    );
    test(
        "22/7",
        53,
        Down,
        "-0.0012644889303773533",
        "-0x0.0052de9a9a24d90c#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Up,
        "-0.0012644889303773535",
        "-0x0.0052de9a9a24d910#53",
        Less,
    );
    test(
        "22/7",
        53,
        Floor,
        "-0.0012644889303773535",
        "-0x0.0052de9a9a24d910#53",
        Less,
    );
    test(
        "22/7",
        53,
        Ceiling,
        "-0.0012644889303773533",
        "-0x0.0052de9a9a24d90c#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Nearest,
        "-0.0012644889303773533",
        "-0x0.0052de9a9a24d90c#53",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "-0.0012644889303773534003603504756467",
        "-0x0.0052de9a9a24d90da7cdffe30130#100",
        Less,
    );
    test("-22/7", 1, Down, "0.00098", "0x0.004#1", Less);
    test("-22/7", 1, Up, "0.0020", "0x0.008#1", Greater);
    test("-22/7", 1, Floor, "0.00098", "0x0.004#1", Less);
    test("-22/7", 1, Ceiling, "0.0020", "0x0.008#1", Greater);
    test("-22/7", 1, Nearest, "0.00098", "0x0.004#1", Less);
    test("-22/7", 5, Nearest, "0.00128", "0x0.0054#5", Greater);
    test("-22/7", 10, Down, "0.0012627", "0x0.0052c#10", Less);
    test("-22/7", 10, Up, "0.0012646", "0x0.0052e#10", Greater);
    test("-22/7", 10, Floor, "0.0012627", "0x0.0052c#10", Less);
    test("-22/7", 10, Ceiling, "0.0012646", "0x0.0052e#10", Greater);
    test("-22/7", 10, Nearest, "0.0012646", "0x0.0052e#10", Greater);
    test(
        "-22/7",
        20,
        Nearest,
        "0.0012644883",
        "0x0.0052de98#20",
        Less,
    );
    test(
        "-22/7",
        53,
        Down,
        "0.0012644889303773533",
        "0x0.0052de9a9a24d90c#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Up,
        "0.0012644889303773535",
        "0x0.0052de9a9a24d910#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Floor,
        "0.0012644889303773533",
        "0x0.0052de9a9a24d90c#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Ceiling,
        "0.0012644889303773535",
        "0x0.0052de9a9a24d910#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Nearest,
        "0.0012644889303773533",
        "0x0.0052de9a9a24d90c#53",
        Less,
    );
    test(
        "-22/7",
        100,
        Nearest,
        "0.0012644889303773534003603504756467",
        "0x0.0052de9a9a24d90da7cdffe30130#100",
        Greater,
    );
    test("355/113", 1, Down, "-2.4e-7", "-0x4.0E-6#1", Greater);
    test("355/113", 1, Up, "-4.8e-7", "-0x8.0E-6#1", Less);
    test("355/113", 1, Floor, "-4.8e-7", "-0x8.0E-6#1", Less);
    test("355/113", 1, Ceiling, "-2.4e-7", "-0x4.0E-6#1", Greater);
    test("355/113", 1, Nearest, "-2.4e-7", "-0x4.0E-6#1", Greater);
    test("355/113", 5, Nearest, "-2.68e-7", "-0x4.8E-6#5", Less);
    test("355/113", 10, Down, "-2.6636e-7", "-0x4.78E-6#10", Greater);
    test("355/113", 10, Up, "-2.6682e-7", "-0x4.7aE-6#10", Less);
    test("355/113", 10, Floor, "-2.6682e-7", "-0x4.7aE-6#10", Less);
    test(
        "355/113",
        10,
        Ceiling,
        "-2.6636e-7",
        "-0x4.78E-6#10",
        Greater,
    );
    test("355/113", 10, Nearest, "-2.6682e-7", "-0x4.7aE-6#10", Less);
    test(
        "355/113",
        20,
        Nearest,
        "-2.6676435e-7",
        "-0x4.79be8E-6#20",
        Less,
    );
    test(
        "355/113",
        53,
        Down,
        "-2.6676418906241912e-7",
        "-0x4.79be53e7511d4E-6#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Up,
        "-2.6676418906241917e-7",
        "-0x4.79be53e7511d8E-6#53",
        Less,
    );
    test(
        "355/113",
        53,
        Floor,
        "-2.6676418906241917e-7",
        "-0x4.79be53e7511d8E-6#53",
        Less,
    );
    test(
        "355/113",
        53,
        Ceiling,
        "-2.6676418906241912e-7",
        "-0x4.79be53e7511d4E-6#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Nearest,
        "-2.6676418906241917e-7",
        "-0x4.79be53e7511d8E-6#53",
        Less,
    );
    test(
        "355/113",
        100,
        Nearest,
        "-2.6676418906241914840637452887346e-7",
        "-0x4.79be53e7511d64791a3045970E-6#100",
        Greater,
    );
    test("3", 1, Down, "0.12", "0x0.2#1", Less);
    test("3", 1, Up, "0.25", "0x0.4#1", Greater);
    test("3", 1, Floor, "0.12", "0x0.2#1", Less);
    test("3", 1, Ceiling, "0.25", "0x0.4#1", Greater);
    test("3", 1, Nearest, "0.12", "0x0.2#1", Less);
    test("3", 5, Nearest, "0.141", "0x0.24#5", Less);
    test("3", 10, Down, "0.14111", "0x0.242#10", Less);
    test("3", 10, Up, "0.14136", "0x0.243#10", Greater);
    test("3", 10, Floor, "0.14111", "0x0.242#10", Less);
    test("3", 10, Ceiling, "0.14136", "0x0.243#10", Greater);
    test("3", 10, Nearest, "0.14111", "0x0.242#10", Less);
    test("3", 20, Nearest, "0.14111996", "0x0.242070#20", Less);
    test(
        "3",
        53,
        Down,
        "0.14112000805986721",
        "0x0.242070db6daab6#53",
        Less,
    );
    test(
        "3",
        53,
        Up,
        "0.14112000805986724",
        "0x0.242070db6daab8#53",
        Greater,
    );
    test(
        "3",
        53,
        Floor,
        "0.14112000805986721",
        "0x0.242070db6daab6#53",
        Less,
    );
    test(
        "3",
        53,
        Ceiling,
        "0.14112000805986724",
        "0x0.242070db6daab8#53",
        Greater,
    );
    test(
        "3",
        53,
        Nearest,
        "0.14112000805986721",
        "0x0.242070db6daab6#53",
        Less,
    );
    test(
        "3",
        100,
        Nearest,
        "0.14112000805986722210074480280816",
        "0x0.242070db6daab69e3902e84684#100",
        Greater,
    );
    test("100", 1, Down, "-0.50", "-0x0.8#1", Greater);
    test("100", 1, Up, "-1.0", "-0x1.0#1", Less);
    test("100", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("100", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("100", 1, Nearest, "-0.50", "-0x0.8#1", Greater);
    test("100", 5, Nearest, "-0.500", "-0x0.80#5", Greater);
    test("100", 10, Down, "-0.50586", "-0x0.818#10", Greater);
    test("100", 10, Up, "-0.50684", "-0x0.81c#10", Less);
    test("100", 10, Floor, "-0.50684", "-0x0.81c#10", Less);
    test("100", 10, Ceiling, "-0.50586", "-0x0.818#10", Greater);
    test("100", 10, Nearest, "-0.50684", "-0x0.81c#10", Less);
    test("100", 20, Nearest, "-0.50636578", "-0x0.81a13#20", Less);
    test(
        "100",
        53,
        Down,
        "-0.50636564110975879",
        "-0x0.81a12dbc626dc0#53",
        Greater,
    );
    test(
        "100",
        53,
        Up,
        "-0.50636564110975890",
        "-0x0.81a12dbc626dc8#53",
        Less,
    );
    test(
        "100",
        53,
        Floor,
        "-0.50636564110975890",
        "-0x0.81a12dbc626dc8#53",
        Less,
    );
    test(
        "100",
        53,
        Ceiling,
        "-0.50636564110975879",
        "-0x0.81a12dbc626dc0#53",
        Greater,
    );
    test(
        "100",
        53,
        Nearest,
        "-0.50636564110975879",
        "-0x0.81a12dbc626dc0#53",
        Greater,
    );
    test(
        "100",
        100,
        Nearest,
        "-0.50636564110975879365655761045969",
        "-0x0.81a12dbc626dc03847b0aae84#100",
        Greater,
    );
    test("1000000", 1, Down, "-0.25", "-0x0.4#1", Greater);
    test("1000000", 1, Up, "-0.50", "-0x0.8#1", Less);
    test("1000000", 1, Floor, "-0.50", "-0x0.8#1", Less);
    test("1000000", 1, Ceiling, "-0.25", "-0x0.4#1", Greater);
    test("1000000", 1, Nearest, "-0.25", "-0x0.4#1", Greater);
    test("1000000", 5, Nearest, "-0.344", "-0x0.58#5", Greater);
    test("1000000", 10, Down, "-0.34961", "-0x0.598#10", Greater);
    test("1000000", 10, Up, "-0.35010", "-0x0.59a#10", Less);
    test("1000000", 10, Floor, "-0.35010", "-0x0.59a#10", Less);
    test("1000000", 10, Ceiling, "-0.34961", "-0x0.598#10", Greater);
    test("1000000", 10, Nearest, "-0.35010", "-0x0.59a#10", Less);
    test(
        "1000000",
        20,
        Nearest,
        "-0.34999371",
        "-0x0.599930#20",
        Less,
    );
    test(
        "1000000",
        53,
        Down,
        "-0.34999350217129294",
        "-0x0.59992c95a3619c#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Up,
        "-0.34999350217129299",
        "-0x0.59992c95a361a0#53",
        Less,
    );
    test(
        "1000000",
        53,
        Floor,
        "-0.34999350217129299",
        "-0x0.59992c95a361a0#53",
        Less,
    );
    test(
        "1000000",
        53,
        Ceiling,
        "-0.34999350217129294",
        "-0x0.59992c95a3619c#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Nearest,
        "-0.34999350217129294",
        "-0x0.59992c95a3619c#53",
        Greater,
    );
    test(
        "1000000",
        100,
        Nearest,
        "-0.34999350217129295211765248678059",
        "-0x0.59992c95a3619d264732d26e98#100",
        Greater,
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
        "9.9999999999983330e-7",
        "0x0.000010c6f7a0b5ea7a#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Up,
        "9.9999999999983351e-7",
        "0x0.000010c6f7a0b5ea7b#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Floor,
        "9.9999999999983330e-7",
        "0x0.000010c6f7a0b5ea7a#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Ceiling,
        "9.9999999999983351e-7",
        "0x0.000010c6f7a0b5ea7b#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Nearest,
        "9.9999999999983330e-7",
        "0x0.000010c6f7a0b5ea7a#53",
        Less,
    );
    test(
        "1/1000000",
        100,
        Nearest,
        "9.9999999999983333333333334166635e-7",
        "0x0.000010c6f7a0b5ea7a2711cf92d7e2#100",
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
        "-9.9999999999983330e-7",
        "-0x0.000010c6f7a0b5ea7a#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Up,
        "-9.9999999999983351e-7",
        "-0x0.000010c6f7a0b5ea7b#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Floor,
        "-9.9999999999983351e-7",
        "-0x0.000010c6f7a0b5ea7b#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Ceiling,
        "-9.9999999999983330e-7",
        "-0x0.000010c6f7a0b5ea7a#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Nearest,
        "-9.9999999999983330e-7",
        "-0x0.000010c6f7a0b5ea7a#53",
        Greater,
    );
    test(
        "-1/1000000",
        100,
        Nearest,
        "-9.9999999999983333333333334166635e-7",
        "-0x0.000010c6f7a0b5ea7a2711cf92d7e2#100",
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
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Up,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Floor,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Ceiling,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        5,
        Nearest,
        "1.00",
        "0x1.0#5",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Down,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "0.99902",
        "0x0.ffc#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "1.0000000000000000000000000000000",
        "0x1.0000000000000000000000000#100",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Down,
        "-0.50",
        "-0x0.8#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Up,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Ceiling,
        "-0.50",
        "-0x0.8#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        5,
        Nearest,
        "-1.00",
        "-0x1.0#5",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Up,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        20,
        Nearest,
        "-1.0000000",
        "-0x1.00000#20",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Down,
        "-0.99999999999999989",
        "-0x0.fffffffffffff8#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Up,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Floor,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Ceiling,
        "-0.99999999999999989",
        "-0x0.fffffffffffff8#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        100,
        Nearest,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Down,
        "9.9e-32",
        "0x2.0E-26#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Up,
        "2.0e-31",
        "0x4.0E-26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Floor,
        "9.9e-32",
        "0x2.0E-26#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Ceiling,
        "2.0e-31",
        "0x4.0E-26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Nearest,
        "2.0e-31",
        "0x4.0E-26#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        5,
        Nearest,
        "1.73e-31",
        "0x3.8E-26#5",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Down,
        "1.6948e-31",
        "0x3.70E-26#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Up,
        "1.6967e-31",
        "0x3.71E-26#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Floor,
        "1.6948e-31",
        "0x3.70E-26#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Ceiling,
        "1.6967e-31",
        "0x3.71E-26#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Nearest,
        "1.6948e-31",
        "0x3.70E-26#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        20,
        Nearest,
        "1.6956854e-31",
        "0x3.70734E-26#20",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Down,
        "1.6956855320737799e-31",
        "0x3.707344a409382E-26#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Up,
        "1.6956855320737801e-31",
        "0x3.707344a409384E-26#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Floor,
        "1.6956855320737799e-31",
        "0x3.707344a409382E-26#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Ceiling,
        "1.6956855320737801e-31",
        "0x3.707344a409384E-26#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Nearest,
        "1.6956855320737799e-31",
        "0x3.707344a409382E-26#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        100,
        Nearest,
        "1.6956855320737799287917402938778e-31",
        "0x3.707344a4093822299f31d0084E-26#100",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Down,
        "-0.50",
        "-0x0.8#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Up,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Floor,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Ceiling,
        "-0.50",
        "-0x0.8#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        Nearest,
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        5,
        Nearest,
        "-1.00",
        "-0x1.0#5",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Down,
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "-0.99902",
        "-0x0.ffc#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "-1.0000000",
        "-0x1.00000#20",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "-0.99999999999999989",
        "-0x0.fffffffffffff8#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "-0.99999999999999989",
        "-0x0.fffffffffffff8#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "-1.0000000000000000000000000000000",
        "-0x1.0000000000000000000000000#100",
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
        "-1.0",
        "-0x1.0#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        5,
        Nearest,
        "-0.875",
        "-0x0.e0#5",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Down,
        "-0.87207",
        "-0x0.df4#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Up,
        "-0.87305",
        "-0x0.df8#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Floor,
        "-0.87305",
        "-0x0.df8#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Ceiling,
        "-0.87207",
        "-0x0.df4#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Nearest,
        "-0.87207",
        "-0x0.df4#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        20,
        Nearest,
        "-0.87218380",
        "-0x0.df477#20",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Down,
        "-0.87218360541826723",
        "-0x0.df476cbd60fac0#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Up,
        "-0.87218360541826734",
        "-0x0.df476cbd60fac8#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Floor,
        "-0.87218360541826734",
        "-0x0.df476cbd60fac8#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Ceiling,
        "-0.87218360541826723",
        "-0x0.df476cbd60fac0#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Nearest,
        "-0.87218360541826734",
        "-0x0.df476cbd60fac8#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        100,
        Nearest,
        "-0.87218360541826730978071977821350",
        "-0x0.df476cbd60fac5f54ce60f245#100",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Down,
        "3.9e-31",
        "0x8.0E-26#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Up,
        "7.9e-31",
        "0x1.0E-25#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Floor,
        "3.9e-31",
        "0x8.0E-26#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Ceiling,
        "7.9e-31",
        "0x1.0E-25#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        Nearest,
        "7.9e-31",
        "0x1.0E-25#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        5,
        Nearest,
        "7.89e-31",
        "0x1.0E-25#5",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Down,
        "7.8809e-31",
        "0xf.fcE-26#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Up,
        "7.8886e-31",
        "0x1.000E-25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Floor,
        "7.8809e-31",
        "0xf.fcE-26#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Ceiling,
        "7.8886e-31",
        "0x1.000E-25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        10,
        Nearest,
        "7.8886e-31",
        "0x1.000E-25#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        20,
        Nearest,
        "7.8886091e-31",
        "0x1.00000E-25#20",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Down,
        "7.8886090522101172e-31",
        "0xf.ffffffffffff8E-26#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Up,
        "7.8886090522101181e-31",
        "0x1.0000000000000E-25#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Floor,
        "7.8886090522101172e-31",
        "0xf.ffffffffffff8E-26#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Ceiling,
        "7.8886090522101181e-31",
        "0x1.0000000000000E-25#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        53,
        Nearest,
        "7.8886090522101181e-31",
        "0x1.0000000000000E-25#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        100,
        Nearest,
        "7.8886090522101180541172856528279e-31",
        "0x1.0000000000000000000000000E-25#100",
        Greater,
    );
}

// Inputs of magnitude around 2^(-2^30) or less, whose sines underflow: they take the series path,
// where everything is `Rational` arithmetic, so no 2^30-bit `Float` is ever formed and the test is
// cheap.
#[test]
fn test_sin_rational_tiny_underflow() {
    let test = |x: Rational, prec, rm, out: Float, out_o| {
        let (s, o) = Float::sin_rational_prec_round_ref(&x, prec, rm);
        assert!(s.is_valid());
        assert_eq!(ComparableFloat(s), ComparableFloat(out));
        assert_eq!(o, out_o);
    };
    let min_exp = -(1i64 << 30);
    let min_positive = |prec| Float::one_prec(prec) >> (1u64 << 30);
    // x = 2^(-2^30), the smallest positive Float: sin(x) is just below it. These rows go through
    // the series bracket with 2^30-bit denominators, and take about a minute each.
    let x = Rational::power_of_2(min_exp);
    test(x.clone(), 1, Floor, Float::ZERO, Less);
    test(x.clone(), 10, Nearest, min_positive(10), Greater);
    test(-x, 1, Down, Float::NEGATIVE_ZERO, Greater);
    // x = 3 * 2^(-2^30 - 1): sin(x) is just below the midpoint of 2^(-2^30) and 2^(1 - 2^30)
    let x = Rational::power_of_2(min_exp - 1) * Rational::from(3u32);
    test(x, 1, Nearest, min_positive(1), Less);
    // inputs far below the smallest positive Float are decided by the rounding mode alone
    let x = Rational::power_of_2(min_exp - 5);
    test(x.clone(), 10, Ceiling, min_positive(10), Greater);
    test(x.clone(), 10, Nearest, Float::ZERO, Less);
    test(-&x, 10, Nearest, Float::NEGATIVE_ZERO, Greater);
    test(-x, 10, Floor, -min_positive(10), Less);
    test(
        Rational::power_of_2(min_exp << 1),
        1,
        Up,
        min_positive(1),
        Greater,
    );
}

#[test]
#[should_panic]
fn sin_rational_prec_fail() {
    Float::sin_rational_prec(Rational::ONE, 0);
}

#[test]
#[should_panic]
fn sin_rational_prec_ref_fail() {
    Float::sin_rational_prec_ref(&Rational::ONE, 0);
}

#[test]
#[should_panic]
fn sin_rational_prec_round_fail_1() {
    Float::sin_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn sin_rational_prec_round_fail_2() {
    Float::sin_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn sin_rational_prec_round_ref_fail() {
    Float::sin_rational_prec_round_ref(&Rational::ONE, 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn sin_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (s, o) = Float::sin_rational_prec_round(x.clone(), prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = Float::sin_rational_prec_round_ref(&x, prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    // |sin x| <= 1, and sin is odd (a `Rational` has no negative zero, so x = 0 is excluded)
    assert!(s.le_abs(&1u32));
    if x != 0u32 {
        let (s_neg, o_neg) = Float::sin_rational_prec_round(-&x, prec, -rm);
        assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
        assert_eq!(o_neg, o.reverse());
    }

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_o) = rug_sin_rational_prec_round(&x, prec, rrm);
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
        // only sin(0) = 0 is exact
        assert_eq!(x, 0u32);
        for rm in exhaustive_rounding_modes() {
            let (s2, oo) = Float::sin_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::sin_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn sin_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        sin_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::sin_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn sin_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (s, o) = Float::sin_rational_prec(x.clone(), prec);
    assert!(s.is_valid());

    let (s_alt, o_alt) = Float::sin_rational_prec_ref(&x, prec);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let (s_alt, o_alt) = Float::sin_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    assert!(s.le_abs(&1u32));

    let (rug_s, rug_o) = rug_sin_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_s)),
        ComparableFloatRef(&s)
    );
    assert_eq!(rug_o, o, "x = {x} prec = {prec}");

    // the sine of an exactly representable rational is the Float sine
    if let Ok(f) = Float::try_from(&x) {
        let (s_alt, o_alt) = f.sin_prec(prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
    }
}

#[test]
fn sin_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        sin_rational_prec_properties_helper(x, prec);
    });
}

// An input too large to be a `Float`, reduced modulo 2 pi in `Rational` arithmetic with pi to about
// 2^30 bits; slow even in release mode.
#[test]
fn test_sin_rational_huge() {
    let x = Rational::power_of_2(1i64 << 30);
    let (s, o) = Float::sin_rational_prec_round_ref(&x, 10, Nearest);
    assert_eq!(s.to_string(), "0.62793");
    assert_eq!(to_hex_string(&s), "0x0.a0c#10");
    assert_eq!(o, Greater);
}

// Inputs within 2^(-2^30) of pi, whose sines underflow. Each call computes pi to about 2^30 bits,
// so this test is slow even in release mode.
#[test]
fn test_sin_rational_underflow() {
    let p = (1u64 << 30) + 64;
    // 2^(-2^30), the smallest positive `Float`, at the output precision
    let min_positive = Float::one_prec(10) >> (1u64 << 30);
    // pi rounded down: x_lo < pi, so sin(x_lo) is positive, and at most half an ulp of x_lo,
    // 2^(-2^30 - 63), in magnitude
    let mut pi = Float::pi_prec_round(p, Floor).0;
    let x_lo = Rational::exact_from(&pi);
    let (s, o) = Float::sin_rational_prec_round_ref(&x_lo, 10, Ceiling);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&min_positive));
    assert_eq!(o, Greater);
    // pi rounded up: x_hi > pi, so sin(x_hi) is negative
    pi.increment();
    let x_hi = Rational::exact_from(&pi);
    let (s, o) = Float::sin_rational_prec_round_ref(&x_hi, 10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sin() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_sin(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(0.0, 0.0);
    test::<f32>(-0.0, -0.0);
    test::<f32>(1.0, 0.84147096);
    test::<f32>(-1.0, -0.84147096);
    test::<f32>(0.5, 0.47942555);
    test::<f32>(-0.5, -0.47942555);
    test::<f32>(2.0, 0.9092974);
    test::<f32>(-2.0, -0.9092974);
    test::<f32>(core::f32::consts::PI, -8.742278e-8);
    test::<f32>(core::f32::consts::FRAC_PI_2, 1.0);
    test::<f32>(core::f32::consts::E, 0.41078135);
    test::<f32>(100.0, -0.50636566);
    test::<f32>(1.0e10, -0.48750603);
    test::<f32>(1.0e30, -0.79116344);
    test::<f32>(3.4028235e38, -0.5218765);
    test::<f32>(1.1754944e-38, 1.1754944e-38);
    test::<f32>(1.0e-45, 1.0e-45);
    test::<f32>(-1.0e-45, -1.0e-45);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(0.0, 0.0);
    test::<f64>(-0.0, -0.0);
    test::<f64>(1.0, 0.8414709848078965);
    test::<f64>(-1.0, -0.8414709848078965);
    test::<f64>(0.5, 0.479425538604203);
    test::<f64>(-0.5, -0.479425538604203);
    test::<f64>(2.0, 0.9092974268256817);
    test::<f64>(-2.0, -0.9092974268256817);
    test::<f64>(core::f64::consts::PI, 1.2246467991473532e-16);
    test::<f64>(core::f64::consts::FRAC_PI_2, 1.0);
    test::<f64>(core::f64::consts::E, 0.41078129050290885);
    test::<f64>(100.0, -0.5063656411097588);
    test::<f64>(1.0e10, -0.4875060250875107);
    test::<f64>(1.0e100, -0.3806377310050287);
    test::<f64>(1.0e300, -0.8178819121159085);
    test::<f64>(1.7976931348623157e308, 0.004961954789184062);
    test::<f64>(2.2250738585072014e-308, 2.2250738585072014e-308);
    test::<f64>(5.0e-324, 5.0e-324);
    test::<f64>(-5.0e-324, -5.0e-324);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sin_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let s = primitive_float_sin(x);
        // sin is NaN exactly for NaN and infinite inputs, and otherwise lies in [-1, 1]
        assert_eq!(s.is_nan(), !x.is_finite());
        if x.is_finite() {
            assert!(s >= T::NEGATIVE_ONE && s <= T::ONE);
            // sin is odd
            assert_eq!(NiceFloat(primitive_float_sin(-x)), NiceFloat(-s));
            // the result is the correctly rounded sine, as computed by MPFR with 64 bits to spare,
            // so that a subnormal result is rounded once by the conversion rather than twice
            let rug_x = rug::Float::with_val(
                u32::exact_from(T::MANTISSA_WIDTH + 64),
                &rug::Float::exact_from(&Float::from(x)),
            );
            let rug_s: T =
                T::rounding_from(&<Float as From<&rug::Float>>::from(&rug_x.sin()), Nearest).0;
            assert_eq!(NiceFloat(rug_s), NiceFloat(s));
        }
    });
}

#[test]
fn primitive_float_sin_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sin_properties_helper);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sin_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_sin_rational::<T>(&x)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0.0);
    test::<f32>("1", 0.84147096);
    test::<f32>("1/2", 0.47942555);
    test::<f32>("1/3", 0.3271947);
    test::<f32>("3/5", 0.5646425);
    test::<f32>("22/7", -0.0012644889);
    test::<f32>("355/113", -2.6676418e-7);
    test::<f32>("1000000", -0.3499935);
    test::<f32>("1/1000000", 0.000001);
    test::<f32>("1/1000000000000000000000000", 1.0e-24);
    test::<f32>("-1", -0.84147096);
    test::<f32>("-1/2", -0.47942555);
    test::<f32>("-1/3", -0.3271947);
    test::<f32>("-22/7", 0.0012644889);
    test::<f32>("-1000000", 0.3499935);
    test::<f32>("10000", -0.30561438);
    test::<f64>("0", 0.0);
    test::<f64>("1", 0.8414709848078965);
    test::<f64>("1/2", 0.479425538604203);
    test::<f64>("1/3", 0.32719469679615226);
    test::<f64>("3/5", 0.5646424733950354);
    test::<f64>("22/7", -0.0012644889303773533);
    test::<f64>("355/113", -2.6676418906241917e-7);
    test::<f64>("1000000", -0.34999350217129294);
    test::<f64>("1/1000000", 9.999999999998333e-7);
    test::<f64>("1/1000000000000000000000000", 1.0e-24);
    test::<f64>("-1", -0.8414709848078965);
    test::<f64>("-1/2", -0.479425538604203);
    test::<f64>("-1/3", -0.32719469679615226);
    test::<f64>("-22/7", 0.0012644889303773533);
    test::<f64>("-1000000", 0.34999350217129294);
    test::<f64>("10000", -0.30561438888825215);
    // tiny inputs, whose sines are subnormal or zero
    let tiny = |zeros: usize| format!("1/1{}", "0".repeat(zeros));
    test::<f32>(&tiny(40), 1.0e-40);
    test::<f32>(&format!("-{}", tiny(40)), -1.0e-40);
    test::<f32>(&tiny(50), 0.0);
    test::<f64>(&tiny(310), 1.0e-310);
    test::<f64>(&format!("-{}", tiny(310)), -1.0e-310);
    test::<f64>(&tiny(330), 0.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sin_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let s = primitive_float_sin_rational::<T>(&x);
        // the sine of a rational is never NaN, and lies in [-1, 1]
        assert!(s >= T::NEGATIVE_ONE && s <= T::ONE);
        // sin is odd (a `Rational` has no negative zero, so x = 0 is excluded)
        if x != 0u32 {
            assert_eq!(
                NiceFloat(primitive_float_sin_rational::<T>(&-&x)),
                NiceFloat(-s)
            );
        }
        // the result is the correctly rounded sine, as computed by MPFR with 64 bits to spare, so
        // that a subnormal result is rounded once by the conversion rather than twice
        let rug_s = rug_sin_rational_prec(&x, T::MANTISSA_WIDTH + 64).0;
        let rug_s: T = T::rounding_from(&<Float as From<&rug::Float>>::from(&rug_s), Nearest).0;
        assert_eq!(NiceFloat(rug_s), NiceFloat(s));
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // The sine of a finite nonzero primitive float, taken through the `Rational` path, matches
        // the direct primitive-float sine (a `Rational` cannot carry the sign of a zero).
        if x.is_finite() && x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_sin_rational::<T>(&Rational::exact_from(x))),
                NiceFloat(primitive_float_sin(x))
            );
        }
    });
}

#[test]
fn primitive_float_sin_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sin_rational_properties_helper);
}

// Rows reuse the cosine test's inputs, plus the sine's own closed-form and underflow cases.
// Branches of `sin_with_period_prec_round_normal_ref` covered:
// - exact case (a): quarter turns, ±1, and half turns, ±0 with the sign of x (also via xr = 0 for
//   |x| >= u)
// - exact case (b): twelfths of a turn, ±1/2
// - |x| >= u: exact reduction of x mod u
// - closed-form cases: x/u with denominator 3 or 6 (sqrt(3)/2), 8 (sqrt(2)/2), and 20 (phi/2 and
//   (phi - 1)/2), with all rounding modes and both signs
// - x/u near a multiple of 1/2: the near-zero path
// - the general Ziv loop, at the first working precision and after a retry
// - x = 2^(-2^30), the smallest positive Float: 2 pi x/u below the smallest positive Float for u =
//   7 and u = 100 (underflow decided by the rounding mode, to nearest included), and not for u = 1
//   or u = 4 (2 pi x/4 is just above the smallest positive Float)
#[test]
fn test_sin_with_period_prec_round() {
    let test = |s, s_hex, u: u64, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().sin_with_period_prec_round(u, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.sin_with_period_prec_round_ref(u, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.sin_with_period_prec_round_assign(u, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u32::try_from(u).is_ok()
        {
            let (rug_c, rug_o) =
                rug_sin_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
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
    test("NaN", "NaN", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 360, 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 360, 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 360, 10, Exact, "0.0", "0x0.0", Equal);
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
    test("180.0", "0xb4.0#6", 360, 10, Nearest, "0.0", "0x0.0", Equal);
    test("180.0", "0xb4.0#6", 360, 10, Floor, "0.0", "0x0.0", Equal);
    test("180.0", "0xb4.0#6", 360, 10, Exact, "0.0", "0x0.0", Equal);
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
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "60.0",
        "0x3c.0#4",
        360,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        Less,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        Less,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Nearest,
        "-0.86621",
        "-0x0.ddc#10",
        Less,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
        Less,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Nearest,
        "-0.86621",
        "-0x0.ddc#10",
        Less,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
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
        "720.0",
        "0x2.dE+2#6",
        360,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test("720.0", "0x2.dE+2#6", 360, 10, Floor, "0.0", "0x0.0", Equal);
    test("720.0", "0x2.dE+2#6", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, 53, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 2, 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 2, 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 2, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 2, 53, Nearest, "0.0", "0x0.0", Equal);
    test(
        "1.0",
        "0x1.0#1",
        3,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        3,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        3,
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        3,
        53,
        Nearest,
        "0.86602540378443860",
        "0x0.ddb3d742c26550#53",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        4,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        4,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
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
        4,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        6,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        6,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        6,
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        6,
        53,
        Nearest,
        "0.86602540378443860",
        "0x0.ddb3d742c26550#53",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        7,
        10,
        Nearest,
        "0.78223",
        "0x0.c84#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        7,
        10,
        Floor,
        "0.78125",
        "0x0.c80#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        7,
        10,
        Ceiling,
        "0.78223",
        "0x0.c84#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Nearest,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Floor,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Ceiling,
        "0.50000",
        "0x0.800#10",
        Equal,
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
        360,
        10,
        Floor,
        "0.017426",
        "0x0.0476#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        360,
        10,
        Ceiling,
        "0.017456",
        "0x0.0478#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        360,
        53,
        Nearest,
        "0.017452406437283512",
        "0x0.0477c2cae277478#53",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        1000000,
        10,
        Nearest,
        "6.2808e-6",
        "0x0.0000696#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        1000000,
        10,
        Floor,
        "6.2808e-6",
        "0x0.0000696#10",
        Less,
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
    test(
        "1.0",
        "0x1.0#1",
        1099511627776,
        10,
        Nearest,
        "5.7128e-12",
        "0x6.48E-10#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        1099511627776,
        10,
        Floor,
        "5.7128e-12",
        "0x6.48E-10#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        1099511627776,
        10,
        Ceiling,
        "5.7199e-12",
        "0x6.4aE-10#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        1099511627776,
        53,
        Nearest,
        "5.7145237471373423e-12",
        "0x6.487ed5110b460E-10#53",
        Less,
    );
    test("0.50", "0x0.8#1", 1, 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8#1", 1, 10, Floor, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8#1", 1, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8#1", 1, 53, Nearest, "0.0", "0x0.0", Equal);
    test(
        "0.50",
        "0x0.8#1",
        2,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        2,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        2,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
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
        3,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        3,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        3,
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        3,
        53,
        Nearest,
        "0.86602540378443860",
        "0x0.ddb3d742c26550#53",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        4,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        4,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        4,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        4,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        6,
        10,
        Nearest,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        6,
        10,
        Floor,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        6,
        10,
        Ceiling,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        6,
        53,
        Nearest,
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        7,
        10,
        Nearest,
        "0.43408",
        "0x0.6f2#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        7,
        10,
        Floor,
        "0.43359",
        "0x0.6f0#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        7,
        10,
        Ceiling,
        "0.43408",
        "0x0.6f2#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        12,
        10,
        Nearest,
        "0.25879",
        "0x0.424#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        12,
        10,
        Floor,
        "0.25879",
        "0x0.424#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        12,
        10,
        Ceiling,
        "0.25928",
        "0x0.426#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        360,
        10,
        Nearest,
        "0.0087280",
        "0x0.023c#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        360,
        10,
        Floor,
        "0.0087128",
        "0x0.023b#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        360,
        10,
        Ceiling,
        "0.0087280",
        "0x0.023c#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        360,
        53,
        Nearest,
        "0.0087265354983739347",
        "0x0.023be6f892a9842#53",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        1000000,
        10,
        Nearest,
        "3.1404e-6",
        "0x0.000034b#10",
        Less,
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
        "0.50",
        "0x0.8#1",
        1000000,
        10,
        Ceiling,
        "3.1441e-6",
        "0x0.000034c#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        1099511627776,
        10,
        Nearest,
        "2.8564e-12",
        "0x3.24E-10#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        1099511627776,
        10,
        Floor,
        "2.8564e-12",
        "0x3.24E-10#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        1099511627776,
        10,
        Ceiling,
        "2.8599e-12",
        "0x3.25E-10#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        1099511627776,
        53,
        Nearest,
        "2.8572618735686711e-12",
        "0x3.243f6a8885a30E-10#53",
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
        1,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        1,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        1,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        2,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        2,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        2,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        2,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        3,
        10,
        Nearest,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        3,
        10,
        Floor,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        3,
        10,
        Ceiling,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        3,
        53,
        Nearest,
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        4,
        10,
        Nearest,
        "0.38281",
        "0x0.620#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        4,
        10,
        Floor,
        "0.38232",
        "0x0.61e#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        4,
        10,
        Ceiling,
        "0.38281",
        "0x0.620#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        4,
        53,
        Nearest,
        "0.38268343236508978",
        "0x0.61f78a9abaa58c#53",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        6,
        10,
        Nearest,
        "0.25879",
        "0x0.424#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        6,
        10,
        Floor,
        "0.25879",
        "0x0.424#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        6,
        10,
        Ceiling,
        "0.25928",
        "0x0.426#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        6,
        53,
        Nearest,
        "0.25881904510252074",
        "0x0.4241f7064c1a40#53",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        7,
        10,
        Nearest,
        "0.22241",
        "0x0.38f#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        7,
        10,
        Floor,
        "0.22241",
        "0x0.38f#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        7,
        10,
        Ceiling,
        "0.22266",
        "0x0.390#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        12,
        10,
        Nearest,
        "0.13062",
        "0x0.217#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        12,
        10,
        Floor,
        "0.13037",
        "0x0.216#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        12,
        10,
        Ceiling,
        "0.13062",
        "0x0.217#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        10,
        Nearest,
        "0.0043640",
        "0x0.011e0#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        10,
        Floor,
        "0.0043564",
        "0x0.011d8#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        10,
        Ceiling,
        "0.0043640",
        "0x0.011e0#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        53,
        Nearest,
        "0.0043633092847465711",
        "0x0.011df42eae296e2#53",
        Greater,
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
        1000000,
        10,
        Floor,
        "1.5702e-6",
        "0x0.00001a58#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        1000000,
        10,
        Ceiling,
        "1.5721e-6",
        "0x0.00001a60#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        1099511627776,
        10,
        Nearest,
        "1.4282e-12",
        "0x1.920E-10#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        1099511627776,
        10,
        Floor,
        "1.4282e-12",
        "0x1.920E-10#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        1099511627776,
        10,
        Ceiling,
        "1.4300e-12",
        "0x1.928E-10#10",
        Greater,
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
    test("1.5", "0x1.8#2", 1, 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.5", "0x1.8#2", 1, 10, Floor, "0.0", "0x0.0", Equal);
    test("1.5", "0x1.8#2", 1, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.5", "0x1.8#2", 1, 53, Nearest, "0.0", "0x0.0", Equal);
    test(
        "1.5",
        "0x1.8#2",
        2,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
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
        "1.5",
        "0x1.8#2",
        2,
        10,
        Ceiling,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        2,
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
    );
    test("1.5", "0x1.8#2", 3, 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.5", "0x1.8#2", 3, 10, Floor, "0.0", "0x0.0", Equal);
    test("1.5", "0x1.8#2", 3, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.5", "0x1.8#2", 3, 53, Nearest, "0.0", "0x0.0", Equal);
    test(
        "1.5",
        "0x1.8#2",
        4,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        4,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        4,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        4,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        6,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
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
        6,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        6,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "1.5",
        "0x1.8#2",
        7,
        10,
        Nearest,
        "0.97461",
        "0x0.f98#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        7,
        10,
        Floor,
        "0.97461",
        "0x0.f98#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        7,
        10,
        Ceiling,
        "0.97559",
        "0x0.f9c#10",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        12,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        12,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        12,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        360,
        10,
        Nearest,
        "0.026184",
        "0x0.06b4#10",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        360,
        10,
        Floor,
        "0.026154",
        "0x0.06b2#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        360,
        10,
        Ceiling,
        "0.026184",
        "0x0.06b4#10",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        360,
        53,
        Nearest,
        "0.026176948307873153",
        "0x0.06b38850e432a44#53",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        1000000,
        10,
        Nearest,
        "9.4175e-6",
        "0x0.00009e0#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        1000000,
        10,
        Floor,
        "9.4175e-6",
        "0x0.00009e0#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        1000000,
        10,
        Ceiling,
        "9.4324e-6",
        "0x0.00009e4#10",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        1099511627776,
        10,
        Nearest,
        "8.5691e-12",
        "0x9.6cE-10#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        1099511627776,
        10,
        Floor,
        "8.5691e-12",
        "0x9.6cE-10#10",
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
    test(
        "1.5",
        "0x1.8#2",
        1099511627776,
        53,
        Nearest,
        "8.5717856207060134e-12",
        "0x9.6cbe3f9990e90E-10#53",
        Less,
    );
    test("2.0", "0x2.0#1", 1, 10, Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 1, 10, Floor, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 1, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 1, 53, Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 2, 10, Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 2, 10, Floor, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 2, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 2, 53, Nearest, "0.0", "0x0.0", Equal);
    test(
        "2.0",
        "0x2.0#1",
        3,
        10,
        Nearest,
        "-0.86621",
        "-0x0.ddc#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        3,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        3,
        10,
        Ceiling,
        "-0.86523",
        "-0x0.dd8#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        3,
        53,
        Nearest,
        "-0.86602540378443860",
        "-0x0.ddb3d742c26550#53",
        Greater,
    );
    test("2.0", "0x2.0#1", 4, 10, Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 4, 10, Floor, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 4, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 4, 53, Nearest, "0.0", "0x0.0", Equal);
    test(
        "2.0",
        "0x2.0#1",
        6,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        6,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        6,
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        6,
        53,
        Nearest,
        "0.86602540378443860",
        "0x0.ddb3d742c26550#53",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        7,
        10,
        Nearest,
        "0.97461",
        "0x0.f98#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        7,
        10,
        Floor,
        "0.97461",
        "0x0.f98#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        7,
        10,
        Ceiling,
        "0.97559",
        "0x0.f9c#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        12,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        12,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        12,
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Nearest,
        "0.034912",
        "0x0.08f0#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Floor,
        "0.034851",
        "0x0.08ec#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Ceiling,
        "0.034912",
        "0x0.08f0#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        53,
        Nearest,
        "0.034899496702500969",
        "0x0.08ef2c64fbee138#53",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        1000000,
        10,
        Nearest,
        "0.000012562",
        "0x0.0000d2c#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        1000000,
        10,
        Floor,
        "0.000012562",
        "0x0.0000d2c#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        1000000,
        10,
        Ceiling,
        "0.000012577",
        "0x0.0000d30#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        1099511627776,
        10,
        Nearest,
        "1.1426e-11",
        "0xc.90E-10#10",
        Less,
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
    test(
        "2.0",
        "0x2.0#1",
        1099511627776,
        10,
        Ceiling,
        "1.1440e-11",
        "0xc.94E-10#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        1099511627776,
        53,
        Nearest,
        "1.1429047494274685e-11",
        "0xc.90fdaa22168c0E-10#53",
        Less,
    );
    test("3.0", "0x3.0#2", 1, 10, Nearest, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 1, 10, Floor, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 1, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 1, 53, Nearest, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 2, 10, Nearest, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 2, 10, Floor, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 2, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 2, 53, Nearest, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 3, 10, Nearest, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 3, 10, Floor, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 3, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 3, 53, Nearest, "0.0", "0x0.0", Equal);
    test(
        "3.0",
        "0x3.0#2",
        4,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        4,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        4,
        10,
        Ceiling,
        "-1.0000",
        "-0x1.000#10",
        Equal,
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
    test("3.0", "0x3.0#2", 6, 10, Nearest, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 6, 10, Floor, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 6, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("3.0", "0x3.0#2", 6, 53, Nearest, "0.0", "0x0.0", Equal);
    test(
        "3.0",
        "0x3.0#2",
        7,
        10,
        Nearest,
        "0.43408",
        "0x0.6f2#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        7,
        10,
        Floor,
        "0.43359",
        "0x0.6f0#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        7,
        10,
        Ceiling,
        "0.43408",
        "0x0.6f2#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        12,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        12,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        12,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        360,
        10,
        Nearest,
        "0.052307",
        "0x0.0d64#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        360,
        10,
        Floor,
        "0.052307",
        "0x0.0d64#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        360,
        10,
        Ceiling,
        "0.052368",
        "0x0.0d68#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        360,
        53,
        Nearest,
        "0.052335956242943835",
        "0x0.0d65e3a477e4870#53",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        1000000,
        10,
        Nearest,
        "0.000018835",
        "0x0.00013c0#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        1000000,
        10,
        Floor,
        "0.000018835",
        "0x0.00013c0#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        1000000,
        10,
        Ceiling,
        "0.000018865",
        "0x0.00013c8#10",
        Greater,
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
    test(
        "3.0",
        "0x3.0#2",
        1099511627776,
        10,
        Floor,
        "1.7138e-11",
        "0x1.2d8E-9#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        1099511627776,
        10,
        Ceiling,
        "1.7167e-11",
        "0x1.2e0E-9#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        1099511627776,
        53,
        Nearest,
        "1.7143571241412027e-11",
        "0x1.2d97c7f3321d2E-9#53",
        Less,
    );
    test("-1.0", "-0x1.0#1", 1, 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 1, 10, Floor, "-0.0", "-0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 1, 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 1, 53, Nearest, "-0.0", "-0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 2, 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 2, 10, Floor, "-0.0", "-0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 2, 10, Ceiling, "-0.0", "-0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 2, 53, Nearest, "-0.0", "-0x0.0", Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        3,
        10,
        Nearest,
        "-0.86621",
        "-0x0.ddc#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        3,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        3,
        10,
        Ceiling,
        "-0.86523",
        "-0x0.dd8#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        3,
        53,
        Nearest,
        "-0.86602540378443860",
        "-0x0.ddb3d742c26550#53",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        4,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        4,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        Equal,
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
        4,
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        6,
        10,
        Nearest,
        "-0.86621",
        "-0x0.ddc#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        6,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        6,
        10,
        Ceiling,
        "-0.86523",
        "-0x0.dd8#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        6,
        53,
        Nearest,
        "-0.86602540378443860",
        "-0x0.ddb3d742c26550#53",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        7,
        10,
        Nearest,
        "-0.78223",
        "-0x0.c84#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        7,
        10,
        Floor,
        "-0.78223",
        "-0x0.c84#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        7,
        10,
        Ceiling,
        "-0.78125",
        "-0x0.c80#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        12,
        10,
        Nearest,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        12,
        10,
        Floor,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        12,
        10,
        Ceiling,
        "-0.50000",
        "-0x0.800#10",
        Equal,
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
        360,
        10,
        Floor,
        "-0.017456",
        "-0x0.0478#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        360,
        10,
        Ceiling,
        "-0.017426",
        "-0x0.0476#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        360,
        53,
        Nearest,
        "-0.017452406437283512",
        "-0x0.0477c2cae277478#53",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1000000,
        10,
        Nearest,
        "-6.2808e-6",
        "-0x0.0000696#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1000000,
        10,
        Floor,
        "-6.2883e-6",
        "-0x0.0000698#10",
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
    test(
        "-1.0",
        "-0x1.0#1",
        1099511627776,
        10,
        Nearest,
        "-5.7128e-12",
        "-0x6.48E-10#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1099511627776,
        10,
        Floor,
        "-5.7199e-12",
        "-0x6.4aE-10#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1099511627776,
        10,
        Ceiling,
        "-5.7128e-12",
        "-0x6.48E-10#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1099511627776,
        53,
        Nearest,
        "-5.7145237471373423e-12",
        "-0x6.487ed5110b460E-10#53",
        Greater,
    );
    test("100.0", "0x64.0#5", 1, 10, Nearest, "0.0", "0x0.0", Equal);
    test("100.0", "0x64.0#5", 1, 10, Floor, "0.0", "0x0.0", Equal);
    test("100.0", "0x64.0#5", 1, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("100.0", "0x64.0#5", 1, 53, Nearest, "0.0", "0x0.0", Equal);
    test("100.0", "0x64.0#5", 2, 10, Nearest, "0.0", "0x0.0", Equal);
    test("100.0", "0x64.0#5", 2, 10, Floor, "0.0", "0x0.0", Equal);
    test("100.0", "0x64.0#5", 2, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("100.0", "0x64.0#5", 2, 53, Nearest, "0.0", "0x0.0", Equal);
    test(
        "100.0",
        "0x64.0#5",
        3,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        3,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        3,
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        3,
        53,
        Nearest,
        "0.86602540378443860",
        "0x0.ddb3d742c26550#53",
        Less,
    );
    test("100.0", "0x64.0#5", 4, 10, Nearest, "0.0", "0x0.0", Equal);
    test("100.0", "0x64.0#5", 4, 10, Floor, "0.0", "0x0.0", Equal);
    test("100.0", "0x64.0#5", 4, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("100.0", "0x64.0#5", 4, 53, Nearest, "0.0", "0x0.0", Equal);
    test(
        "100.0",
        "0x64.0#5",
        6,
        10,
        Nearest,
        "-0.86621",
        "-0x0.ddc#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        6,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        6,
        10,
        Ceiling,
        "-0.86523",
        "-0x0.dd8#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        6,
        53,
        Nearest,
        "-0.86602540378443860",
        "-0x0.ddb3d742c26550#53",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        7,
        10,
        Nearest,
        "0.97461",
        "0x0.f98#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        7,
        10,
        Floor,
        "0.97461",
        "0x0.f98#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        7,
        10,
        Ceiling,
        "0.97559",
        "0x0.f9c#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        12,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        12,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        12,
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        360,
        10,
        Nearest,
        "0.98438",
        "0x0.fc0#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        360,
        10,
        Floor,
        "0.98438",
        "0x0.fc0#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        360,
        10,
        Ceiling,
        "0.98535",
        "0x0.fc4#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        360,
        53,
        Nearest,
        "0.98480775301220802",
        "0x0.fc1c5c6408e0b8#53",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        1000000,
        10,
        Nearest,
        "0.00062847",
        "0x0.00293#10",
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
        "100.0",
        "0x64.0#5",
        1000000,
        10,
        Ceiling,
        "0.00062847",
        "0x0.00293#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        1099511627776,
        10,
        Nearest,
        "5.7116e-10",
        "0x2.74E-8#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        1099511627776,
        10,
        Floor,
        "5.7116e-10",
        "0x2.74E-8#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        1099511627776,
        10,
        Ceiling,
        "5.7207e-10",
        "0x2.75E-8#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        1099511627776,
        53,
        Nearest,
        "5.7145237471373425e-10",
        "0x2.74518b3aa8676E-8#53",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        10,
        Nearest,
        "0.27295",
        "0x0.45e#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        10,
        Floor,
        "0.27295",
        "0x0.45e#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        10,
        Ceiling,
        "0.27344",
        "0x0.460#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        53,
        Nearest,
        "0.27295193551730668",
        "0x0.45e02d946d3e60#53",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        2,
        10,
        Nearest,
        "-0.99023",
        "-0x0.fd8#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        2,
        10,
        Floor,
        "-0.99121",
        "-0x0.fdc#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        2,
        10,
        Ceiling,
        "-0.99023",
        "-0x0.fd8#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        2,
        53,
        Nearest,
        "-0.99046142569665252",
        "-0x0.fd8ee147511068#53",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        3,
        10,
        Nearest,
        "0.81641",
        "0x0.d10#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        3,
        10,
        Floor,
        "0.81543",
        "0x0.d0c#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        3,
        10,
        Ceiling,
        "0.81641",
        "0x0.d10#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        3,
        53,
        Nearest,
        "0.81633925071718771",
        "0x0.d0fb9bf0457ec8#53",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        4,
        10,
        Nearest,
        "-0.75391",
        "-0x0.c10#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        4,
        10,
        Floor,
        "-0.75488",
        "-0x0.c14#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        4,
        10,
        Ceiling,
        "-0.75391",
        "-0x0.c10#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        4,
        53,
        Nearest,
        "-0.75425138073610065",
        "-0x0.c1169e55397210#53",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        6,
        10,
        Nearest,
        "-0.45947",
        "-0x0.75a#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        6,
        10,
        Floor,
        "-0.45996",
        "-0x0.75c#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        6,
        10,
        Ceiling,
        "-0.45947",
        "-0x0.75a#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        6,
        53,
        Nearest,
        "-0.45957986062149070",
        "-0x0.75a706974500a0#53",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        7,
        10,
        Nearest,
        "-0.75684",
        "-0x0.c1c#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        7,
        10,
        Floor,
        "-0.75684",
        "-0x0.c1c#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        7,
        10,
        Ceiling,
        "-0.75586",
        "-0x0.c18#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        12,
        10,
        Nearest,
        "0.97168",
        "0x0.f8c#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        12,
        10,
        Floor,
        "0.97070",
        "0x0.f88#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        12,
        10,
        Ceiling,
        "0.97168",
        "0x0.f8c#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        360,
        10,
        Nearest,
        "0.83398",
        "0x0.d58#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        360,
        10,
        Floor,
        "0.83398",
        "0x0.d58#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        360,
        10,
        Ceiling,
        "0.83496",
        "0x0.d5c#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        360,
        53,
        Nearest,
        "0.83430943331480656",
        "0x0.d5954d92d4d5c8#53",
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
        1000000,
        10,
        Floor,
        "0.00077534",
        "0x0.0032d#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1000000,
        10,
        Ceiling,
        "0.00077629",
        "0x0.0032e#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1099511627776,
        10,
        Nearest,
        "7.0577e-10",
        "0x3.08E-8#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1099511627776,
        10,
        Floor,
        "7.0486e-10",
        "0x3.07E-8#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1099511627776,
        10,
        Ceiling,
        "7.0577e-10",
        "0x3.08E-8#10",
        Greater,
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
        1,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1,
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1,
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        2,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
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
        2,
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        2,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        3,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        3,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        3,
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        3,
        53,
        Nearest,
        "0.86602540378443860",
        "0x0.ddb3d742c26550#53",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        4,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        4,
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        4,
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        4,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        6,
        10,
        Nearest,
        "-0.86621",
        "-0x0.ddc#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        6,
        10,
        Floor,
        "-0.86621",
        "-0x0.ddc#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        6,
        10,
        Ceiling,
        "-0.86523",
        "-0x0.dd8#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        6,
        53,
        Nearest,
        "-0.86602540378443860",
        "-0x0.ddb3d742c26550#53",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        7,
        10,
        Nearest,
        "-0.43408",
        "-0x0.6f2#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        7,
        10,
        Floor,
        "-0.43408",
        "-0x0.6f2#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        7,
        10,
        Ceiling,
        "-0.43359",
        "-0x0.6f0#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        12,
        10,
        Nearest,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        12,
        10,
        Floor,
        "0.86523",
        "0x0.dd8#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        12,
        10,
        Ceiling,
        "0.86621",
        "0x0.ddc#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        360,
        10,
        Nearest,
        "-0.98438",
        "-0x0.fc0#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        360,
        10,
        Floor,
        "-0.98535",
        "-0x0.fc4#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        360,
        10,
        Ceiling,
        "-0.98438",
        "-0x0.fc0#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        360,
        53,
        Nearest,
        "-0.98480775301220802",
        "-0x0.fc1c5c6408e0b8#53",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1000000,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1000000,
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1000000,
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1099511627776,
        10,
        Nearest,
        "0.057129",
        "0x0.0ea0#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1099511627776,
        10,
        Floor,
        "0.057068",
        "0x0.0e9c#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1099511627776,
        10,
        Ceiling,
        "0.057129",
        "0x0.0ea0#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1099511627776,
        53,
        Nearest,
        "0.057114140509326068",
        "0x0.0e9f0845a06fb08#53",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        2,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        2,
        10,
        Floor,
        "-0.70801",
        "-0x0.b54#10",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        2,
        10,
        Ceiling,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        2,
        53,
        Nearest,
        "-0.70710678118654757",
        "-0x0.b504f333f9de68#53",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        3,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        3,
        10,
        Floor,
        "-1.0000",
        "-0x1.000#10",
        Equal,
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
        3,
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        4,
        10,
        Nearest,
        "-0.92383",
        "-0x0.ec8#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        4,
        10,
        Floor,
        "-0.92480",
        "-0x0.ecc#10",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        4,
        10,
        Ceiling,
        "-0.92383",
        "-0x0.ec8#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        4,
        53,
        Nearest,
        "-0.92387953251128674",
        "-0x0.ec835e79946a30#53",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        6,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        6,
        10,
        Floor,
        "-0.70801",
        "-0x0.b54#10",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        6,
        10,
        Ceiling,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        6,
        53,
        Nearest,
        "-0.70710678118654757",
        "-0x0.b504f333f9de68#53",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        7,
        10,
        Nearest,
        "-0.62305",
        "-0x0.9f8#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        7,
        10,
        Floor,
        "-0.62402",
        "-0x0.9fc#10",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        7,
        10,
        Ceiling,
        "-0.62305",
        "-0x0.9f8#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        12,
        10,
        Nearest,
        "-0.38281",
        "-0x0.620#10",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        12,
        10,
        Floor,
        "-0.38281",
        "-0x0.620#10",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        12,
        10,
        Ceiling,
        "-0.38232",
        "-0x0.61e#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        360,
        10,
        Nearest,
        "-0.013092",
        "-0x0.035a#10",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        360,
        10,
        Floor,
        "-0.013092",
        "-0x0.035a#10",
        Less,
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
        360,
        53,
        Nearest,
        "-0.013089595571344440",
        "-0x0.0359d6f8e594150#53",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1000000,
        10,
        Nearest,
        "-4.7088e-6",
        "-0x0.00004f0#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1000000,
        10,
        Floor,
        "-4.7162e-6",
        "-0x0.00004f2#10",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1000000,
        10,
        Ceiling,
        "-4.7088e-6",
        "-0x0.00004f0#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1099511627776,
        10,
        Nearest,
        "-4.2846e-12",
        "-0x4.b6E-10#10",
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
        "-0.75",
        "-0x0.c#2",
        1099511627776,
        10,
        Ceiling,
        "-4.2846e-12",
        "-0x4.b6E-10#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1099511627776,
        53,
        Nearest,
        "-4.2858928103530067e-12",
        "-0x4.b65f1fccc8748E-10#53",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1,
        10,
        Nearest,
        "6.2846e-10",
        "0x2.b3E-8#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1,
        10,
        Floor,
        "6.2755e-10",
        "0x2.b2E-8#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1,
        10,
        Ceiling,
        "6.2846e-10",
        "0x2.b3E-8#10",
        Greater,
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
        2,
        10,
        Nearest,
        "3.1423e-10",
        "0x1.598E-8#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        2,
        10,
        Floor,
        "3.1378e-10",
        "0x1.590E-8#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        2,
        10,
        Ceiling,
        "3.1423e-10",
        "0x1.598E-8#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        2,
        53,
        Nearest,
        "3.1415926535897934e-10",
        "0x1.596bf8ce7631eE-8#53",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        3,
        10,
        Nearest,
        "2.0941e-10",
        "0xe.64E-9#10",
        Less,
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
        3,
        10,
        Ceiling,
        "2.0964e-10",
        "0xe.68E-9#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        3,
        53,
        Nearest,
        "2.0943951023931955e-10",
        "0xe.647fb344ecbe8E-9#53",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        4,
        10,
        Nearest,
        "1.5712e-10",
        "0xa.ccE-9#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        4,
        10,
        Floor,
        "1.5689e-10",
        "0xa.c8E-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        4,
        10,
        Ceiling,
        "1.5712e-10",
        "0xa.ccE-9#10",
        Greater,
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
        6,
        10,
        Nearest,
        "1.0471e-10",
        "0x7.32E-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        6,
        10,
        Floor,
        "1.0471e-10",
        "0x7.32E-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        6,
        10,
        Ceiling,
        "1.0482e-10",
        "0x7.34E-9#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        6,
        53,
        Nearest,
        "1.0471975511965977e-10",
        "0x7.323fd9a2765f4E-9#53",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        7,
        10,
        Nearest,
        "8.9813e-11",
        "0x6.2cE-9#10",
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
        7,
        10,
        Ceiling,
        "8.9813e-11",
        "0x6.2cE-9#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        12,
        10,
        Nearest,
        "5.2353e-11",
        "0x3.99E-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        12,
        10,
        Floor,
        "5.2353e-11",
        "0x3.99E-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        12,
        10,
        Ceiling,
        "5.2410e-11",
        "0x3.9aE-9#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        360,
        10,
        Nearest,
        "1.7462e-12",
        "0x1.eb8E-10#10",
        Greater,
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
        360,
        10,
        Ceiling,
        "1.7462e-12",
        "0x1.eb8E-10#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        360,
        53,
        Nearest,
        "1.7453292519943296e-12",
        "0x1.eb443a0930a1fE-10#53",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1000000,
        10,
        Nearest,
        "6.2797e-16",
        "0x2.d4E-13#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1000000,
        10,
        Floor,
        "6.2797e-16",
        "0x2.d4E-13#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1000000,
        10,
        Ceiling,
        "6.2884e-16",
        "0x2.d5E-13#10",
        Greater,
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
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1099511627776,
        10,
        Floor,
        "5.7075e-22",
        "0x2.b2E-18#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1099511627776,
        10,
        Ceiling,
        "5.7158e-22",
        "0x2.b3E-18#10",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1099511627776,
        53,
        Nearest,
        "5.7145237471373428e-22",
        "0x2.b2d7f19cec63cE-18#53",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        1,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        1,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        1,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        1,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        2,
        10,
        Nearest,
        "0.38281",
        "0x0.620#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        2,
        10,
        Floor,
        "0.38232",
        "0x0.61e#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        2,
        10,
        Ceiling,
        "0.38281",
        "0x0.620#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        2,
        53,
        Nearest,
        "0.38268343236508978",
        "0x0.61f78a9abaa58c#53",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        3,
        10,
        Nearest,
        "0.25879",
        "0x0.424#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        3,
        10,
        Floor,
        "0.25879",
        "0x0.424#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        3,
        10,
        Ceiling,
        "0.25928",
        "0x0.426#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        3,
        53,
        Nearest,
        "0.25881904510252074",
        "0x0.4241f7064c1a40#53",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        4,
        10,
        Nearest,
        "0.19507",
        "0x0.31f#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        4,
        10,
        Floor,
        "0.19507",
        "0x0.31f#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        4,
        10,
        Ceiling,
        "0.19531",
        "0x0.320#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        4,
        53,
        Nearest,
        "0.19509032201612828",
        "0x0.31f17078d34c16#53",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        6,
        10,
        Nearest,
        "0.13062",
        "0x0.217#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        6,
        10,
        Floor,
        "0.13037",
        "0x0.216#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        6,
        10,
        Ceiling,
        "0.13062",
        "0x0.217#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        6,
        53,
        Nearest,
        "0.13052619222005160",
        "0x0.216a2a1edb45a2#53",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        7,
        10,
        Nearest,
        "0.11194",
        "0x0.1ca8#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        7,
        10,
        Floor,
        "0.11194",
        "0x0.1ca8#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        7,
        10,
        Ceiling,
        "0.11206",
        "0x0.1cb0#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        12,
        10,
        Nearest,
        "0.065430",
        "0x0.10c0#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        12,
        10,
        Floor,
        "0.065308",
        "0x0.10b8#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        12,
        10,
        Ceiling,
        "0.065430",
        "0x0.10c0#10",
        Greater,
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
        360,
        10,
        Floor,
        "0.0021782",
        "0x0.008ec#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        360,
        10,
        Ceiling,
        "0.0021820",
        "0x0.008f0#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        360,
        53,
        Nearest,
        "0.0021816598343367697",
        "0x0.008efa2da3b48410#53",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        1000000,
        10,
        Nearest,
        "7.8510e-7",
        "0xd.2cE-6#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        1000000,
        10,
        Floor,
        "7.8510e-7",
        "0xd.2cE-6#10",
        Less,
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
        "0.12",
        "0x0.2#1",
        1099511627776,
        10,
        Nearest,
        "7.1410e-13",
        "0xc.90E-11#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        1099511627776,
        10,
        Floor,
        "7.1410e-13",
        "0xc.90E-11#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        1099511627776,
        10,
        Ceiling,
        "7.1498e-13",
        "0xc.94E-11#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        1099511627776,
        53,
        Nearest,
        "7.1431546839216779e-13",
        "0xc.90fdaa22168c0E-11#53",
        Less,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        1,
        53,
        Nearest,
        "6.2831853071795868e-30",
        "0x7.f7029d0214354E-25#53",
        Less,
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
        "3.8e30",
        "0x3.0E+25#2",
        7,
        53,
        Nearest,
        "-0.78183148246802980",
        "-0x0.c8261ba82ef258#53",
        Greater,
    );
    test(
        "3.8e30",
        "0x3.0E+25#2",
        3,
        53,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "4.0e-323228490",
        "0x1.0E-268435450#1",
        18446744073709551615,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "90.000000000000000000000000000808",
        "0x5a.000000000000000000000040#100",
        360,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "90.000000000000000000000000000808",
        "0x5a.000000000000000000000040#100",
        360,
        53,
        Floor,
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
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
        Less,
    );
    test(
        "269.99999999999999999999999999919",
        "0x10d.ffffffffffffffffffffffc#100",
        360,
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Less,
    );
    test("1.0", "0x1.0#1", 8, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "1.0",
        "0x1.0#1",
        8,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        8,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        8,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        8,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 8, 10, Down, "0.70703", "0x0.b50#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        8,
        10,
        Up,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        8,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test("3.0", "0x3.0#2", 8, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "3.0",
        "0x3.0#2",
        8,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        8,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        8,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        8,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test("3.0", "0x3.0#2", 8, 10, Down, "0.70703", "0x0.b50#10", Less);
    test(
        "3.0",
        "0x3.0#2",
        8,
        10,
        Up,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        8,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test(
        "5.0", "0x5.0#3", 8, 1, Nearest, "-0.50", "-0x0.8#1", Greater,
    );
    test(
        "5.0",
        "0x5.0#3",
        8,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "-5.0",
        "-0x5.0#3",
        8,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "5.0",
        "0x5.0#3",
        8,
        10,
        Floor,
        "-0.70801",
        "-0x0.b54#10",
        Less,
    );
    test(
        "5.0",
        "0x5.0#3",
        8,
        10,
        Ceiling,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "5.0",
        "0x5.0#3",
        8,
        10,
        Down,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test("5.0", "0x5.0#3", 8, 10, Up, "-0.70801", "-0x0.b54#10", Less);
    test(
        "5.0",
        "0x5.0#3",
        8,
        53,
        Nearest,
        "-0.70710678118654757",
        "-0x0.b504f333f9de68#53",
        Less,
    );
    test(
        "7.0", "0x7.0#3", 8, 1, Nearest, "-0.50", "-0x0.8#1", Greater,
    );
    test(
        "7.0",
        "0x7.0#3",
        8,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "-7.0",
        "-0x7.0#3",
        8,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        8,
        10,
        Floor,
        "-0.70801",
        "-0x0.b54#10",
        Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        8,
        10,
        Ceiling,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "7.0",
        "0x7.0#3",
        8,
        10,
        Down,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test("7.0", "0x7.0#3", 8, 10, Up, "-0.70801", "-0x0.b54#10", Less);
    test(
        "7.0",
        "0x7.0#3",
        8,
        53,
        Nearest,
        "-0.70710678118654757",
        "-0x0.b504f333f9de68#53",
        Less,
    );
    test("9.00", "0x9.0#4", 8, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "9.00",
        "0x9.0#4",
        8,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "-9.00",
        "-0x9.0#4",
        8,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        8,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "9.00",
        "0x9.0#4",
        8,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        8,
        10,
        Down,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "9.00",
        "0x9.0#4",
        8,
        10,
        Up,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        8,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test("1.0", "0x1.0#1", 12, 1, Nearest, "0.50", "0x0.8#1", Equal);
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Nearest,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        12,
        10,
        Nearest,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Floor,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Ceiling,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Down,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test("1.0", "0x1.0#1", 12, 10, Up, "0.50000", "0x0.800#10", Equal);
    test(
        "1.0",
        "0x1.0#1",
        12,
        53,
        Nearest,
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Equal,
    );
    test("5.0", "0x5.0#3", 12, 1, Nearest, "0.50", "0x0.8#1", Equal);
    test(
        "5.0",
        "0x5.0#3",
        12,
        10,
        Nearest,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "-5.0",
        "-0x5.0#3",
        12,
        10,
        Nearest,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "5.0",
        "0x5.0#3",
        12,
        10,
        Floor,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "5.0",
        "0x5.0#3",
        12,
        10,
        Ceiling,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "5.0",
        "0x5.0#3",
        12,
        10,
        Down,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test("5.0", "0x5.0#3", 12, 10, Up, "0.50000", "0x0.800#10", Equal);
    test(
        "5.0",
        "0x5.0#3",
        12,
        53,
        Nearest,
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Equal,
    );
    test("7.0", "0x7.0#3", 12, 1, Nearest, "-0.50", "-0x0.8#1", Equal);
    test(
        "7.0",
        "0x7.0#3",
        12,
        10,
        Nearest,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "-7.0",
        "-0x7.0#3",
        12,
        10,
        Nearest,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "7.0",
        "0x7.0#3",
        12,
        10,
        Floor,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "7.0",
        "0x7.0#3",
        12,
        10,
        Ceiling,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "7.0",
        "0x7.0#3",
        12,
        10,
        Down,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "7.0",
        "0x7.0#3",
        12,
        10,
        Up,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "7.0",
        "0x7.0#3",
        12,
        53,
        Nearest,
        "-0.50000000000000000",
        "-0x0.80000000000000#53",
        Equal,
    );
    test(
        "11.0", "0xb.0#4", 12, 1, Nearest, "-0.50", "-0x0.8#1", Equal,
    );
    test(
        "11.0",
        "0xb.0#4",
        12,
        10,
        Nearest,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "-11.0",
        "-0xb.0#4",
        12,
        10,
        Nearest,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "11.0",
        "0xb.0#4",
        12,
        10,
        Floor,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "11.0",
        "0xb.0#4",
        12,
        10,
        Ceiling,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "11.0",
        "0xb.0#4",
        12,
        10,
        Down,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "11.0",
        "0xb.0#4",
        12,
        10,
        Up,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "11.0",
        "0xb.0#4",
        12,
        53,
        Nearest,
        "-0.50000000000000000",
        "-0x0.80000000000000#53",
        Equal,
    );
    test("1.0", "0x1.0#1", 5, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test(
        "1.0",
        "0x1.0#1",
        5,
        10,
        Nearest,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        5,
        10,
        Nearest,
        "-0.95117",
        "-0x0.f38#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        10,
        Floor,
        "0.95020",
        "0x0.f34#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        10,
        Ceiling,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 5, 10, Down, "0.95020", "0x0.f34#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        5,
        10,
        Up,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        53,
        Nearest,
        "0.95105651629515353",
        "0x0.f378709a22a7f8#53",
        Less,
    );
    test("2.0", "0x2.0#1", 5, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "2.0",
        "0x2.0#1",
        5,
        10,
        Nearest,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        5,
        10,
        Nearest,
        "-0.58789",
        "-0x0.968#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        5,
        10,
        Floor,
        "0.58691",
        "0x0.964#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        5,
        10,
        Ceiling,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test("2.0", "0x2.0#1", 5, 10, Down, "0.58691", "0x0.964#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        5,
        10,
        Up,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        5,
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        Greater,
    );
    test(
        "3.0", "0x3.0#2", 5, 1, Nearest, "-0.50", "-0x0.8#1", Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        5,
        10,
        Nearest,
        "-0.58789",
        "-0x0.968#10",
        Less,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        5,
        10,
        Nearest,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        5,
        10,
        Floor,
        "-0.58789",
        "-0x0.968#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        5,
        10,
        Ceiling,
        "-0.58691",
        "-0x0.964#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        5,
        10,
        Down,
        "-0.58691",
        "-0x0.964#10",
        Greater,
    );
    test("3.0", "0x3.0#2", 5, 10, Up, "-0.58789", "-0x0.968#10", Less);
    test(
        "3.0",
        "0x3.0#2",
        5,
        53,
        Nearest,
        "-0.58778525229247314",
        "-0x0.96791823aad2f0#53",
        Less,
    );
    test("4.0", "0x4.0#1", 5, 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test(
        "4.0",
        "0x4.0#1",
        5,
        10,
        Nearest,
        "-0.95117",
        "-0x0.f38#10",
        Less,
    );
    test(
        "-4.0",
        "-0x4.0#1",
        5,
        10,
        Nearest,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        5,
        10,
        Floor,
        "-0.95117",
        "-0x0.f38#10",
        Less,
    );
    test(
        "4.0",
        "0x4.0#1",
        5,
        10,
        Ceiling,
        "-0.95020",
        "-0x0.f34#10",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        5,
        10,
        Down,
        "-0.95020",
        "-0x0.f34#10",
        Greater,
    );
    test("4.0", "0x4.0#1", 5, 10, Up, "-0.95117", "-0x0.f38#10", Less);
    test(
        "4.0",
        "0x4.0#1",
        5,
        53,
        Nearest,
        "-0.95105651629515353",
        "-0x0.f378709a22a7f8#53",
        Greater,
    );
    test("6.0", "0x6.0#2", 5, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test(
        "6.0",
        "0x6.0#2",
        5,
        10,
        Nearest,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "-6.0",
        "-0x6.0#2",
        5,
        10,
        Nearest,
        "-0.95117",
        "-0x0.f38#10",
        Less,
    );
    test(
        "6.0",
        "0x6.0#2",
        5,
        10,
        Floor,
        "0.95020",
        "0x0.f34#10",
        Less,
    );
    test(
        "6.0",
        "0x6.0#2",
        5,
        10,
        Ceiling,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test("6.0", "0x6.0#2", 5, 10, Down, "0.95020", "0x0.f34#10", Less);
    test(
        "6.0",
        "0x6.0#2",
        5,
        10,
        Up,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "6.0",
        "0x6.0#2",
        5,
        53,
        Nearest,
        "0.95105651629515353",
        "0x0.f378709a22a7f8#53",
        Less,
    );
    test("1.0", "0x1.0#1", 10, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Nearest,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        10,
        Nearest,
        "-0.58789",
        "-0x0.968#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Floor,
        "0.58691",
        "0x0.964#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Ceiling,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Down,
        "0.58691",
        "0x0.964#10",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Up,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        Greater,
    );
    test("3.0", "0x3.0#2", 10, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test(
        "3.0",
        "0x3.0#2",
        10,
        10,
        Nearest,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        10,
        10,
        Nearest,
        "-0.95117",
        "-0x0.f38#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        10,
        Floor,
        "0.95020",
        "0x0.f34#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        10,
        Ceiling,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        10,
        Down,
        "0.95020",
        "0x0.f34#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        10,
        Up,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        53,
        Nearest,
        "0.95105651629515353",
        "0x0.f378709a22a7f8#53",
        Less,
    );
    test("7.0", "0x7.0#3", 10, 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test(
        "7.0",
        "0x7.0#3",
        10,
        10,
        Nearest,
        "-0.95117",
        "-0x0.f38#10",
        Less,
    );
    test(
        "-7.0",
        "-0x7.0#3",
        10,
        10,
        Nearest,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "7.0",
        "0x7.0#3",
        10,
        10,
        Floor,
        "-0.95117",
        "-0x0.f38#10",
        Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        10,
        10,
        Ceiling,
        "-0.95020",
        "-0x0.f34#10",
        Greater,
    );
    test(
        "7.0",
        "0x7.0#3",
        10,
        10,
        Down,
        "-0.95020",
        "-0x0.f34#10",
        Greater,
    );
    test(
        "7.0",
        "0x7.0#3",
        10,
        10,
        Up,
        "-0.95117",
        "-0x0.f38#10",
        Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        10,
        53,
        Nearest,
        "-0.95105651629515353",
        "-0x0.f378709a22a7f8#53",
        Greater,
    );
    test(
        "9.00", "0x9.0#4", 10, 1, Nearest, "-0.50", "-0x0.8#1", Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        10,
        Nearest,
        "-0.58789",
        "-0x0.968#10",
        Less,
    );
    test(
        "-9.00",
        "-0x9.0#4",
        10,
        10,
        Nearest,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        10,
        Floor,
        "-0.58789",
        "-0x0.968#10",
        Less,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        10,
        Ceiling,
        "-0.58691",
        "-0x0.964#10",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        10,
        Down,
        "-0.58691",
        "-0x0.964#10",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        10,
        Up,
        "-0.58789",
        "-0x0.968#10",
        Less,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        53,
        Nearest,
        "-0.58778525229247314",
        "-0x0.96791823aad2f0#53",
        Less,
    );
    test("45.0", "0x2d.0#6", 360, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "45.0",
        "0x2d.0#6",
        360,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "-45.0",
        "-0x2d.0#6",
        360,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "45.0",
        "0x2d.0#6",
        360,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "45.0",
        "0x2d.0#6",
        360,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "45.0",
        "0x2d.0#6",
        360,
        10,
        Down,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "45.0",
        "0x2d.0#6",
        360,
        10,
        Up,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "45.0",
        "0x2d.0#6",
        360,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test(
        "135.0", "0x87.0#8", 360, 1, Nearest, "0.50", "0x0.8#1", Less,
    );
    test(
        "135.0",
        "0x87.0#8",
        360,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "-135.0",
        "-0x87.0#8",
        360,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "135.0",
        "0x87.0#8",
        360,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "135.0",
        "0x87.0#8",
        360,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "135.0",
        "0x87.0#8",
        360,
        10,
        Down,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "135.0",
        "0x87.0#8",
        360,
        10,
        Up,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "135.0",
        "0x87.0#8",
        360,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test(
        "30.0", "0x1e.0#4", 360, 1, Nearest, "0.50", "0x0.8#1", Equal,
    );
    test(
        "30.0",
        "0x1e.0#4",
        360,
        10,
        Nearest,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "-30.0",
        "-0x1e.0#4",
        360,
        10,
        Nearest,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "30.0",
        "0x1e.0#4",
        360,
        10,
        Floor,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "30.0",
        "0x1e.0#4",
        360,
        10,
        Ceiling,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "30.0",
        "0x1e.0#4",
        360,
        10,
        Down,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "30.0",
        "0x1e.0#4",
        360,
        10,
        Up,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "30.0",
        "0x1e.0#4",
        360,
        53,
        Nearest,
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Equal,
    );
    test(
        "150.0", "0x96.0#7", 360, 1, Nearest, "0.50", "0x0.8#1", Equal,
    );
    test(
        "150.0",
        "0x96.0#7",
        360,
        10,
        Nearest,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "-150.0",
        "-0x96.0#7",
        360,
        10,
        Nearest,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "150.0",
        "0x96.0#7",
        360,
        10,
        Floor,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "150.0",
        "0x96.0#7",
        360,
        10,
        Ceiling,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "150.0",
        "0x96.0#7",
        360,
        10,
        Down,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "150.0",
        "0x96.0#7",
        360,
        10,
        Up,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "150.0",
        "0x96.0#7",
        360,
        53,
        Nearest,
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Equal,
    );
    test(
        "72.0", "0x48.0#4", 360, 1, Nearest, "1.0", "0x1.0#1", Greater,
    );
    test(
        "72.0",
        "0x48.0#4",
        360,
        10,
        Nearest,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "-72.0",
        "-0x48.0#4",
        360,
        10,
        Nearest,
        "-0.95117",
        "-0x0.f38#10",
        Less,
    );
    test(
        "72.0",
        "0x48.0#4",
        360,
        10,
        Floor,
        "0.95020",
        "0x0.f34#10",
        Less,
    );
    test(
        "72.0",
        "0x48.0#4",
        360,
        10,
        Ceiling,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "72.0",
        "0x48.0#4",
        360,
        10,
        Down,
        "0.95020",
        "0x0.f34#10",
        Less,
    );
    test(
        "72.0",
        "0x48.0#4",
        360,
        10,
        Up,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "72.0",
        "0x48.0#4",
        360,
        53,
        Nearest,
        "0.95105651629515353",
        "0x0.f378709a22a7f8#53",
        Less,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        1,
        Nearest,
        "0.50",
        "0x0.8#1",
        Less,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        10,
        Nearest,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "-144.0",
        "-0x9.0E+1#4",
        360,
        10,
        Nearest,
        "-0.58789",
        "-0x0.968#10",
        Less,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        10,
        Floor,
        "0.58691",
        "0x0.964#10",
        Less,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        10,
        Ceiling,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        10,
        Down,
        "0.58691",
        "0x0.964#10",
        Less,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        10,
        Up,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        Greater,
    );
    test("36.0", "0x24.0#4", 360, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "36.0",
        "0x24.0#4",
        360,
        10,
        Nearest,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "-36.0",
        "-0x24.0#4",
        360,
        10,
        Nearest,
        "-0.58789",
        "-0x0.968#10",
        Less,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        10,
        Floor,
        "0.58691",
        "0x0.964#10",
        Less,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        10,
        Ceiling,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        10,
        Down,
        "0.58691",
        "0x0.964#10",
        Less,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        10,
        Up,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        Greater,
    );
    test(
        "108.0", "0x6c.0#5", 360, 1, Nearest, "1.0", "0x1.0#1", Greater,
    );
    test(
        "108.0",
        "0x6c.0#5",
        360,
        10,
        Nearest,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "-108.0",
        "-0x6c.0#5",
        360,
        10,
        Nearest,
        "-0.95117",
        "-0x0.f38#10",
        Less,
    );
    test(
        "108.0",
        "0x6c.0#5",
        360,
        10,
        Floor,
        "0.95020",
        "0x0.f34#10",
        Less,
    );
    test(
        "108.0",
        "0x6c.0#5",
        360,
        10,
        Ceiling,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "108.0",
        "0x6c.0#5",
        360,
        10,
        Down,
        "0.95020",
        "0x0.f34#10",
        Less,
    );
    test(
        "108.0",
        "0x6c.0#5",
        360,
        10,
        Up,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "108.0",
        "0x6c.0#5",
        360,
        53,
        Nearest,
        "0.95105651629515353",
        "0x0.f378709a22a7f8#53",
        Less,
    );
    test("2.0", "0x2.0#1", 16, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "2.0",
        "0x2.0#1",
        16,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        16,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        16,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        16,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        16,
        10,
        Down,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        16,
        10,
        Up,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        16,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test("3.0", "0x3.0#2", 24, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "3.0",
        "0x3.0#2",
        24,
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        24,
        10,
        Nearest,
        "-0.70703",
        "-0x0.b50#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        24,
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        24,
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        24,
        10,
        Down,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        24,
        10,
        Up,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        24,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test("4.0", "0x4.0#1", 20, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test(
        "4.0",
        "0x4.0#1",
        20,
        10,
        Nearest,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "-4.0",
        "-0x4.0#1",
        20,
        10,
        Nearest,
        "-0.95117",
        "-0x0.f38#10",
        Less,
    );
    test(
        "4.0",
        "0x4.0#1",
        20,
        10,
        Floor,
        "0.95020",
        "0x0.f34#10",
        Less,
    );
    test(
        "4.0",
        "0x4.0#1",
        20,
        10,
        Ceiling,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        20,
        10,
        Down,
        "0.95020",
        "0x0.f34#10",
        Less,
    );
    test(
        "4.0",
        "0x4.0#1",
        20,
        10,
        Up,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        20,
        53,
        Nearest,
        "0.95105651629515353",
        "0x0.f378709a22a7f8#53",
        Less,
    );
    test("6.0", "0x6.0#2", 60, 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "6.0",
        "0x6.0#2",
        60,
        10,
        Nearest,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "-6.0",
        "-0x6.0#2",
        60,
        10,
        Nearest,
        "-0.58789",
        "-0x0.968#10",
        Less,
    );
    test(
        "6.0",
        "0x6.0#2",
        60,
        10,
        Floor,
        "0.58691",
        "0x0.964#10",
        Less,
    );
    test(
        "6.0",
        "0x6.0#2",
        60,
        10,
        Ceiling,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "6.0",
        "0x6.0#2",
        60,
        10,
        Down,
        "0.58691",
        "0x0.964#10",
        Less,
    );
    test(
        "6.0",
        "0x6.0#2",
        60,
        10,
        Up,
        "0.58789",
        "0x0.968#10",
        Greater,
    );
    test(
        "6.0",
        "0x6.0#2",
        60,
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        20,
        30,
        Floor,
        "0.30901699420",
        "0x0.4f1bbcdc#30",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        20,
        30,
        Ceiling,
        "0.30901699467",
        "0x0.4f1bbcde#30",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        20,
        30,
        Nearest,
        "0.30901699420",
        "0x0.4f1bbcdc#30",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        20,
        30,
        Floor,
        "0.80901699420",
        "0x0.cf1bbcdc#30",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        20,
        30,
        Ceiling,
        "0.80901699513",
        "0x0.cf1bbce0#30",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        20,
        30,
        Nearest,
        "0.80901699420",
        "0x0.cf1bbcdc#30",
        Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        20,
        30,
        Floor,
        "0.80901699420",
        "0x0.cf1bbcdc#30",
        Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        20,
        30,
        Ceiling,
        "0.80901699513",
        "0x0.cf1bbce0#30",
        Greater,
    );
    test(
        "7.0",
        "0x7.0#3",
        20,
        30,
        Nearest,
        "0.80901699420",
        "0x0.cf1bbcdc#30",
        Less,
    );
    test(
        "9.00",
        "0x9.0#4",
        20,
        30,
        Floor,
        "0.30901699420",
        "0x0.4f1bbcdc#30",
        Less,
    );
    test(
        "9.00",
        "0x9.0#4",
        20,
        30,
        Ceiling,
        "0.30901699467",
        "0x0.4f1bbcde#30",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        20,
        30,
        Nearest,
        "0.30901699420",
        "0x0.4f1bbcdc#30",
        Less,
    );
    test(
        "11.0",
        "0xb.0#4",
        20,
        30,
        Floor,
        "-0.30901699467",
        "-0x0.4f1bbcde#30",
        Less,
    );
    test(
        "11.0",
        "0xb.0#4",
        20,
        30,
        Ceiling,
        "-0.30901699420",
        "-0x0.4f1bbcdc#30",
        Greater,
    );
    test(
        "11.0",
        "0xb.0#4",
        20,
        30,
        Nearest,
        "-0.30901699420",
        "-0x0.4f1bbcdc#30",
        Greater,
    );
    test(
        "13.0",
        "0xd.0#4",
        20,
        30,
        Floor,
        "-0.80901699513",
        "-0x0.cf1bbce0#30",
        Less,
    );
    test(
        "13.0",
        "0xd.0#4",
        20,
        30,
        Ceiling,
        "-0.80901699420",
        "-0x0.cf1bbcdc#30",
        Greater,
    );
    test(
        "13.0",
        "0xd.0#4",
        20,
        30,
        Nearest,
        "-0.80901699420",
        "-0x0.cf1bbcdc#30",
        Greater,
    );
    test(
        "17.0",
        "0x11.0#5",
        20,
        30,
        Floor,
        "-0.80901699513",
        "-0x0.cf1bbce0#30",
        Less,
    );
    test(
        "17.0",
        "0x11.0#5",
        20,
        30,
        Ceiling,
        "-0.80901699420",
        "-0x0.cf1bbcdc#30",
        Greater,
    );
    test(
        "17.0",
        "0x11.0#5",
        20,
        30,
        Nearest,
        "-0.80901699420",
        "-0x0.cf1bbcdc#30",
        Greater,
    );
    test(
        "19.0",
        "0x13.0#5",
        20,
        30,
        Floor,
        "-0.30901699467",
        "-0x0.4f1bbcde#30",
        Less,
    );
    test(
        "19.0",
        "0x13.0#5",
        20,
        30,
        Ceiling,
        "-0.30901699420",
        "-0x0.4f1bbcdc#30",
        Greater,
    );
    test(
        "19.0",
        "0x13.0#5",
        20,
        30,
        Nearest,
        "-0.30901699420",
        "-0x0.4f1bbcdc#30",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        3,
        30,
        Floor,
        "0.86602540314",
        "0x0.ddb3d740#30",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        3,
        30,
        Nearest,
        "0.86602540407",
        "0x0.ddb3d744#30",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#2",
        3,
        30,
        Floor,
        "-0.86602540407",
        "-0x0.ddb3d744#30",
        Less,
    );
    test(
        "2.0",
        "0x2.0#2",
        3,
        30,
        Nearest,
        "-0.86602540407",
        "-0x0.ddb3d744#30",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        6,
        30,
        Floor,
        "0.86602540314",
        "0x0.ddb3d740#30",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        6,
        30,
        Nearest,
        "0.86602540407",
        "0x0.ddb3d744#30",
        Greater,
    );
    test(
        "5.0",
        "0x5.0#3",
        6,
        30,
        Floor,
        "-0.86602540407",
        "-0x0.ddb3d744#30",
        Less,
    );
    test(
        "5.0",
        "0x5.0#3",
        6,
        30,
        Nearest,
        "-0.86602540407",
        "-0x0.ddb3d744#30",
        Less,
    );
    test(
        "4.0",
        "0x4.0#3",
        6,
        30,
        Floor,
        "-0.86602540407",
        "-0x0.ddb3d744#30",
        Less,
    );
    test(
        "4.0",
        "0x4.0#3",
        6,
        30,
        Nearest,
        "-0.86602540407",
        "-0x0.ddb3d744#30",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        8,
        30,
        Nearest,
        "0.70710678119",
        "0x0.b504f334#30",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        8,
        30,
        Nearest,
        "0.70710678119",
        "0x0.b504f334#30",
        Greater,
    );
    test(
        "5.0",
        "0x5.0#3",
        8,
        30,
        Nearest,
        "-0.70710678119",
        "-0x0.b504f334#30",
        Less,
    );
    test(
        "7.0",
        "0x7.0#3",
        8,
        30,
        Nearest,
        "-0.70710678119",
        "-0x0.b504f334#30",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Exact,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "5.0",
        "0x5.0#3",
        12,
        10,
        Exact,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "7.0",
        "0x7.0#3",
        12,
        10,
        Exact,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "11.0",
        "0xb.0#4",
        12,
        10,
        Exact,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#2",
        4,
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        4,
        10,
        Exact,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        4,
        10,
        Exact,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        4,
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test("2.0", "0x2.0#2", 4, 10, Exact, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#2", 4, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", 2, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 2, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("3.0", "0x3.0#2", 2, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
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
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        10,
        Ceiling,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        100,
        10,
        Ceiling,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        100,
        10,
        Nearest,
        "-0.0",
        "-0x0.0",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        100,
        10,
        Down,
        "0.0",
        "0x0.0",
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
        100,
        10,
        Up,
        "2.3826e-323228497",
        "0x1.000E-268435456#10",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        100,
        10,
        Up,
        "-2.3826e-323228497",
        "-0x1.000E-268435456#10",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        1,
        10,
        Nearest,
        "1.4965e-323228496",
        "0x6.48E-268435456#10",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        7,
        10,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        4,
        1,
        Down,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Less,
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

#[test]
#[should_panic]
fn sin_with_period_prec_round_fail_1() {
    Float::ONE.sin_with_period_prec_round(7, 0, Nearest);
}

#[test]
#[should_panic]
fn sin_with_period_prec_round_fail_2() {
    Float::ONE.sin_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn sin_with_period_prec_round_ref_fail() {
    Float::ONE.sin_with_period_prec_round_ref(7, 10, Exact);
}

#[test]
#[should_panic]
fn sin_with_period_prec_fail() {
    Float::ONE.sin_with_period_prec(7, 0);
}

#[test]
#[should_panic]
fn sin_with_period_round_fail() {
    Float::ONE.sin_with_period_round(7, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn sin_with_period_prec_round_properties_helper(x: Float, u: u64, prec: u64, rm: RoundingMode) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (s, o) = x.sin_with_period_prec_round_ref(u, prec, Nearest);
        if o == Equal {
            let (se, oe) = x.sin_with_period_prec_round_ref(u, prec, Exact);
            assert_eq!(ComparableFloatRef(&se), ComparableFloatRef(&s));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(x.sin_with_period_prec_round_ref(u, prec, Exact));
        }
        return;
    }
    let (s, o) = x.clone().sin_with_period_prec_round(u, prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = x.sin_with_period_prec_round_ref(u, prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let mut s_alt = x.clone();
    let o_alt = s_alt.sin_with_period_prec_round_assign(u, prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm)
        && u32::try_from(u).is_ok()
    {
        let (rug_s, rug_o) =
            rug_sin_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s),
            "RUGPROBE x = {x:#x} u = {u} prec = {prec} rm = {rm:?}"
        );
        assert_eq!(rug_o, o);
    }

    // sin_with_period is NaN exactly for u = 0 and non-finite x, and otherwise lies in [-1, 1]
    assert_eq!(s.is_nan(), u == 0 || !x.is_finite());
    if !s.is_nan() {
        assert!(s.le_abs(&1u32));
        if s.is_normal() {
            assert_eq!(s.get_prec(), Some(prec));
        }
        // sin_with_period is odd
        let (s_neg, o_neg) = (-&x).sin_with_period_prec_round(u, prec, -rm);
        assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
        assert_eq!(o_neg, o.reverse());
        // sin_with_period has period u
        if x.is_finite() && u != 0 {
            let (shifted, os) =
                x.add_prec_round_ref_val(Float::from(u), x.significant_bits() + 64, Nearest);
            if os == Equal {
                let (s_shifted, o_shifted) = shifted.sin_with_period_prec_round(u, prec, rm);
                assert_eq!(
                    ComparableFloat(s_shifted.abs_negative_zero()),
                    ComparableFloat(s.abs_negative_zero_ref())
                );
                assert_eq!(o_shifted, o);
            }
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s2, oo) = x.sin_with_period_prec_round_ref(u, prec, rm);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.sin_with_period_prec_round_ref(u, prec, Exact));
    }
}

#[test]
fn sin_with_period_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, u, prec, rm)| {
            sin_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_18().test_properties(
        |(x, u, prec, rm)| {
            sin_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // sin_with_period(±0) = ±0 and sin_with_period(x, 0) = NaN, exactly
        let (s, o) = Float::ZERO.sin_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (s, o) = Float::NEGATIVE_ZERO.sin_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
        let (s, o) = Float::ONE.sin_with_period_prec_round(0, prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);
        // exact cases: quarter turns, with zeros taking the sign of x
        for (k, expected) in [
            (0u32, Float::ZERO),
            (1, Float::one_prec(prec)),
            (2, Float::ZERO),
            (3, -Float::one_prec(prec)),
            (4, Float::ZERO),
        ] {
            let (s, o) = Float::from(k).sin_with_period_prec_round(4, prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(expected));
            assert_eq!(o, Equal);
        }
        for (k, expected) in [
            (1i32, -Float::one_prec(prec)),
            (2, Float::NEGATIVE_ZERO),
            (3, Float::one_prec(prec)),
            (4, Float::NEGATIVE_ZERO),
        ] {
            let (s, o) = Float::from(-k).sin_with_period_prec_round(4, prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(expected));
            assert_eq!(o, Equal);
        }
        // exact cases: twelfths of a turn
        let half = Float::one_prec(prec) >> 1u32;
        for (k, expected) in [(1u32, half.clone()), (5, half.clone()), (7, -&half), (11, -&half)] {
            let (s, o) = Float::from(k).sin_with_period_prec_round(12, prec, rm);
            assert_eq!(ComparableFloat(s), ComparableFloat(expected));
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn sin_with_period_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, u, prec)| {
        let (s, o) = x.clone().sin_with_period_prec(u, prec);
        assert!(s.is_valid());
        assert_rounding_ordering_consistent(&s, Nearest, o);
        let (s_alt, o_alt) = x.sin_with_period_prec_ref(u, prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        let (s_alt, o_alt) = x.sin_with_period_prec_round_ref(u, prec, Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        let mut s_alt = x.clone();
        let o_alt = s_alt.sin_with_period_prec_assign(u, prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        if u32::try_from(u).is_ok() {
            let (rug_s, rug_o) = rug_sin_with_period_prec(&rug::Float::exact_from(&x), u, prec);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn sin_with_period_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_39().test_properties(|(x, u, rm)| {
        let (s, o) = x.clone().sin_with_period_round(u, rm);
        assert!(s.is_valid());
        assert_rounding_ordering_consistent(&s, rm, o);
        let (s_alt, o_alt) = x.sin_with_period_round_ref(u, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        let (s_alt, o_alt) = x.sin_with_period_prec_round_ref(u, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        let mut s_alt = x.clone();
        let o_alt = s_alt.sin_with_period_round_assign(u, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
    });
}

// Inputs within 2^(-2^30) of a half turn, whose sines underflow: cheap, since the near-zero path
// works with the exact distance to the half turn rather than with pi to 2^30 bits.
#[test]
fn test_sin_with_period_underflow() {
    let min_positive = Float::one_prec(10) >> (1u64 << 30);
    // 180 +- 2^-(2^30 + 70), which need 2^30 + 78 bits (the offset alone is below the exponent
    // range, so the sum is built from a `Rational`)
    let eps = Rational::power_of_2(-((1i64 << 30) + 70));
    let above =
        Float::from_rational_prec_round(Rational::from(180u32) + &eps, (1u64 << 30) + 78, Exact).0;
    let below =
        Float::from_rational_prec_round(Rational::from(180u32) - eps, (1u64 << 30) + 78, Exact).0;
    // just past a half turn: sin is negative and tiny
    let (s, o) = above.sin_with_period_prec_round_ref(360, 10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    let (s, o) = above.sin_with_period_prec_round_ref(360, 10, Floor);
    assert_eq!(ComparableFloat(s), ComparableFloat(-&min_positive));
    assert_eq!(o, Less);
    // just short of it: positive and tiny
    let (s, o) = below.sin_with_period_prec_round_ref(360, 10, Ceiling);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&min_positive));
    assert_eq!(o, Greater);
    let (s, o) = below.sin_with_period_prec_round_ref(360, 10, Down);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
}

// The `Rational` inputs include exact and closed-form cases hit directly (1/12, 1/3, 1/8, 1/20 of a
// turn), which the `Float` version can only reach through a divisor, and non-dyadic inputs whose
// sines MPFR cannot compute exactly, since it must round the input first.
#[test]
fn test_sin_with_period_rational_prec_round() {
    let test = |s, u: u64, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::sin_with_period_rational_prec_round(x.clone(), u, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = Float::sin_with_period_rational_prec_round_ref(&x, u, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = Float::sin_with_period_rational_prec(x.clone(), u, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
            let (c_alt, o_alt) = Float::sin_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        // MPFR rounds the input first, so it cannot see the exact cases of non-dyadic inputs
        if o != Equal
            && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u32::try_from(u).is_ok()
        {
            let (rug_c, rug_o) = rug_sin_with_period_rational_prec_round(&x, u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
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
    test("90", 360, 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("90", 360, 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("90", 360, 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("90", 360, 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("90", 360, 10, Exact, "1.0000", "0x1.000#10", Equal);
    test(
        "90",
        360,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test("180", 360, 1, Nearest, "0.0", "0x0.0", Equal);
    test("180", 360, 10, Nearest, "0.0", "0x0.0", Equal);
    test("180", 360, 10, Floor, "0.0", "0x0.0", Equal);
    test("180", 360, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("180", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test("180", 360, 53, Nearest, "0.0", "0x0.0", Equal);
    test("270", 360, 1, Nearest, "-1.0", "-0x1.0#1", Equal);
    test("270", 360, 10, Nearest, "-1.0000", "-0x1.000#10", Equal);
    test("270", 360, 10, Floor, "-1.0000", "-0x1.000#10", Equal);
    test("270", 360, 10, Ceiling, "-1.0000", "-0x1.000#10", Equal);
    test("270", 360, 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test(
        "270",
        360,
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
    );
    test("360", 360, 1, Nearest, "0.0", "0x0.0", Equal);
    test("360", 360, 10, Nearest, "0.0", "0x0.0", Equal);
    test("360", 360, 10, Floor, "0.0", "0x0.0", Equal);
    test("360", 360, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("360", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test("360", 360, 53, Nearest, "0.0", "0x0.0", Equal);
    test("450", 360, 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("450", 360, 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("450", 360, 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("450", 360, 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("450", 360, 10, Exact, "1.0000", "0x1.000#10", Equal);
    test(
        "450",
        360,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test("60", 360, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("60", 360, 10, Nearest, "0.86621", "0x0.ddc#10", Greater);
    test("60", 360, 10, Floor, "0.86523", "0x0.dd8#10", Less);
    test("60", 360, 10, Ceiling, "0.86621", "0x0.ddc#10", Greater);
    test(
        "60",
        360,
        53,
        Nearest,
        "0.86602540378443860",
        "0x0.ddb3d742c26550#53",
        Less,
    );
    test("120", 360, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("120", 360, 10, Nearest, "0.86621", "0x0.ddc#10", Greater);
    test("120", 360, 10, Floor, "0.86523", "0x0.dd8#10", Less);
    test("120", 360, 10, Ceiling, "0.86621", "0x0.ddc#10", Greater);
    test(
        "120",
        360,
        53,
        Nearest,
        "0.86602540378443860",
        "0x0.ddb3d742c26550#53",
        Less,
    );
    test("240", 360, 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("240", 360, 10, Nearest, "-0.86621", "-0x0.ddc#10", Less);
    test("240", 360, 10, Floor, "-0.86621", "-0x0.ddc#10", Less);
    test("240", 360, 10, Ceiling, "-0.86523", "-0x0.dd8#10", Greater);
    test(
        "240",
        360,
        53,
        Nearest,
        "-0.86602540378443860",
        "-0x0.ddb3d742c26550#53",
        Greater,
    );
    test("300", 360, 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("300", 360, 10, Nearest, "-0.86621", "-0x0.ddc#10", Less);
    test("300", 360, 10, Floor, "-0.86621", "-0x0.ddc#10", Less);
    test("300", 360, 10, Ceiling, "-0.86523", "-0x0.dd8#10", Greater);
    test(
        "300",
        360,
        53,
        Nearest,
        "-0.86602540378443860",
        "-0x0.ddb3d742c26550#53",
        Greater,
    );
    test("45", 360, 1, Nearest, "0.50", "0x0.8#1", Less);
    test("45", 360, 10, Nearest, "0.70703", "0x0.b50#10", Less);
    test("45", 360, 10, Floor, "0.70703", "0x0.b50#10", Less);
    test("45", 360, 10, Ceiling, "0.70801", "0x0.b54#10", Greater);
    test(
        "45",
        360,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test("135", 360, 1, Nearest, "0.50", "0x0.8#1", Less);
    test("135", 360, 10, Nearest, "0.70703", "0x0.b50#10", Less);
    test("135", 360, 10, Floor, "0.70703", "0x0.b50#10", Less);
    test("135", 360, 10, Ceiling, "0.70801", "0x0.b54#10", Greater);
    test(
        "135",
        360,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test("30", 360, 1, Nearest, "0.50", "0x0.8#1", Equal);
    test("30", 360, 10, Nearest, "0.50000", "0x0.800#10", Equal);
    test("30", 360, 10, Floor, "0.50000", "0x0.800#10", Equal);
    test("30", 360, 10, Ceiling, "0.50000", "0x0.800#10", Equal);
    test(
        "30",
        360,
        53,
        Nearest,
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Equal,
    );
    test("150", 360, 1, Nearest, "0.50", "0x0.8#1", Equal);
    test("150", 360, 10, Nearest, "0.50000", "0x0.800#10", Equal);
    test("150", 360, 10, Floor, "0.50000", "0x0.800#10", Equal);
    test("150", 360, 10, Ceiling, "0.50000", "0x0.800#10", Equal);
    test(
        "150",
        360,
        53,
        Nearest,
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Equal,
    );
    test("72", 360, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("72", 360, 10, Nearest, "0.95117", "0x0.f38#10", Greater);
    test("72", 360, 10, Floor, "0.95020", "0x0.f34#10", Less);
    test("72", 360, 10, Ceiling, "0.95117", "0x0.f38#10", Greater);
    test(
        "72",
        360,
        53,
        Nearest,
        "0.95105651629515353",
        "0x0.f378709a22a7f8#53",
        Less,
    );
    test("144", 360, 1, Nearest, "0.50", "0x0.8#1", Less);
    test("144", 360, 10, Nearest, "0.58789", "0x0.968#10", Greater);
    test("144", 360, 10, Floor, "0.58691", "0x0.964#10", Less);
    test("144", 360, 10, Ceiling, "0.58789", "0x0.968#10", Greater);
    test(
        "144",
        360,
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        Greater,
    );
    test("36", 360, 1, Nearest, "0.50", "0x0.8#1", Less);
    test("36", 360, 10, Nearest, "0.58789", "0x0.968#10", Greater);
    test("36", 360, 10, Floor, "0.58691", "0x0.964#10", Less);
    test("36", 360, 10, Ceiling, "0.58789", "0x0.968#10", Greater);
    test(
        "36",
        360,
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        Greater,
    );
    test("108", 360, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("108", 360, 10, Nearest, "0.95117", "0x0.f38#10", Greater);
    test("108", 360, 10, Floor, "0.95020", "0x0.f34#10", Less);
    test("108", 360, 10, Ceiling, "0.95117", "0x0.f38#10", Greater);
    test(
        "108",
        360,
        53,
        Nearest,
        "0.95105651629515353",
        "0x0.f378709a22a7f8#53",
        Less,
    );
    test("1/3", 1, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1/3", 1, 10, Nearest, "0.86621", "0x0.ddc#10", Greater);
    test("1/3", 1, 10, Floor, "0.86523", "0x0.dd8#10", Less);
    test("1/3", 1, 10, Ceiling, "0.86621", "0x0.ddc#10", Greater);
    test(
        "1/3",
        1,
        53,
        Nearest,
        "0.86602540378443860",
        "0x0.ddb3d742c26550#53",
        Less,
    );
    test("1/8", 1, 1, Nearest, "0.50", "0x0.8#1", Less);
    test("1/8", 1, 10, Nearest, "0.70703", "0x0.b50#10", Less);
    test("1/8", 1, 10, Floor, "0.70703", "0x0.b50#10", Less);
    test("1/8", 1, 10, Ceiling, "0.70801", "0x0.b54#10", Greater);
    test(
        "1/8",
        1,
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test("1/5", 1, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1/5", 1, 10, Nearest, "0.95117", "0x0.f38#10", Greater);
    test("1/5", 1, 10, Floor, "0.95020", "0x0.f34#10", Less);
    test("1/5", 1, 10, Ceiling, "0.95117", "0x0.f38#10", Greater);
    test(
        "1/5",
        1,
        53,
        Nearest,
        "0.95105651629515353",
        "0x0.f378709a22a7f8#53",
        Less,
    );
    test("-1/10", 1, 1, Nearest, "-0.50", "-0x0.8#1", Greater);
    test("-1/10", 1, 10, Nearest, "-0.58789", "-0x0.968#10", Less);
    test("-1/10", 1, 10, Floor, "-0.58789", "-0x0.968#10", Less);
    test("-1/10", 1, 10, Ceiling, "-0.58691", "-0x0.964#10", Greater);
    test(
        "-1/10",
        1,
        53,
        Nearest,
        "-0.58778525229247314",
        "-0x0.96791823aad2f0#53",
        Less,
    );
    test("1/7", 1, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1/7", 1, 10, Nearest, "0.78223", "0x0.c84#10", Greater);
    test("1/7", 1, 10, Floor, "0.78125", "0x0.c80#10", Less);
    test("1/7", 1, 10, Ceiling, "0.78223", "0x0.c84#10", Greater);
    test(
        "1/7",
        1,
        53,
        Nearest,
        "0.78183148246802980",
        "0x0.c8261ba82ef258#53",
        Less,
    );
    test("2/7", 1, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("2/7", 1, 10, Nearest, "0.97461", "0x0.f98#10", Less);
    test("2/7", 1, 10, Floor, "0.97461", "0x0.f98#10", Less);
    test("2/7", 1, 10, Ceiling, "0.97559", "0x0.f9c#10", Greater);
    test(
        "2/7",
        1,
        53,
        Nearest,
        "0.97492791218182362",
        "0x0.f994e02ac74b48#53",
        Greater,
    );
    test("-3/7", 1, 1, Nearest, "-0.50", "-0x0.8#1", Less);
    test("-3/7", 1, 10, Nearest, "-0.43408", "-0x0.6f2#10", Less);
    test("-3/7", 1, 10, Floor, "-0.43408", "-0x0.6f2#10", Less);
    test("-3/7", 1, 10, Ceiling, "-0.43359", "-0x0.6f0#10", Greater);
    test(
        "-3/7",
        1,
        53,
        Nearest,
        "-0.43388373911755812",
        "-0x0.6f130135c6af04#53",
        Greater,
    );
    test("22/7", 1, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("22/7", 1, 10, Nearest, "0.78223", "0x0.c84#10", Greater);
    test("22/7", 1, 10, Floor, "0.78125", "0x0.c80#10", Less);
    test("22/7", 1, 10, Ceiling, "0.78223", "0x0.c84#10", Greater);
    test(
        "22/7",
        1,
        53,
        Nearest,
        "0.78183148246802980",
        "0x0.c8261ba82ef258#53",
        Less,
    );
    test("1/7", 3, 1, Nearest, "0.25", "0x0.4#1", Less);
    test("1/7", 3, 10, Nearest, "0.29492", "0x0.4b8#10", Greater);
    test("1/7", 3, 10, Floor, "0.29443", "0x0.4b6#10", Less);
    test("1/7", 3, 10, Ceiling, "0.29492", "0x0.4b8#10", Greater);
    test(
        "1/7",
        3,
        53,
        Nearest,
        "0.29475517441090421",
        "0x0.4b75133a6bee9c#53",
        Less,
    );
    test("355/113", 360, 1, Nearest, "0.062", "0x0.1#1", Greater);
    test(
        "355/113",
        360,
        10,
        Nearest,
        "0.054810",
        "0x0.0e08#10",
        Greater,
    );
    test("355/113", 360, 10, Floor, "0.054749", "0x0.0e04#10", Less);
    test(
        "355/113",
        360,
        10,
        Ceiling,
        "0.054810",
        "0x0.0e08#10",
        Greater,
    );
    test(
        "355/113",
        360,
        53,
        Nearest,
        "0.054803669797705817",
        "0x0.0e079d017b5fba0#53",
        Greater,
    );
    test("1", 7, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1", 7, 10, Nearest, "0.78223", "0x0.c84#10", Greater);
    test("1", 7, 10, Floor, "0.78125", "0x0.c80#10", Less);
    test("1", 7, 10, Ceiling, "0.78223", "0x0.c84#10", Greater);
    test(
        "1",
        7,
        53,
        Nearest,
        "0.78183148246802980",
        "0x0.c8261ba82ef258#53",
        Less,
    );
    test("1000000", 7, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1000000", 7, 10, Nearest, "0.78223", "0x0.c84#10", Greater);
    test("1000000", 7, 10, Floor, "0.78125", "0x0.c80#10", Less);
    test("1000000", 7, 10, Ceiling, "0.78223", "0x0.c84#10", Greater);
    test(
        "1000000",
        7,
        53,
        Nearest,
        "0.78183148246802980",
        "0x0.c8261ba82ef258#53",
        Less,
    );
    test("1/1000000", 1, 1, Nearest, "7.6e-6", "0x0.00008#1", Greater);
    test(
        "1/1000000",
        1,
        10,
        Nearest,
        "6.2808e-6",
        "0x0.0000696#10",
        Less,
    );
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
        "1/1000000",
        1,
        10,
        Ceiling,
        "6.2883e-6",
        "0x0.0000698#10",
        Greater,
    );
    test(
        "1/1000000",
        1,
        53,
        Nearest,
        "6.2831853071382447e-6",
        "0x0.0000696a134dfed758#53",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        1,
        1,
        Nearest,
        "6.3e-30",
        "0x8.0E-25#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        1,
        10,
        Nearest,
        "6.2862e-30",
        "0x7.f8E-25#10",
        Greater,
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
        "1/1000000000000000000000000000000",
        1,
        10,
        Ceiling,
        "6.2862e-30",
        "0x7.f8E-25#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        1,
        53,
        Nearest,
        "6.2831853071795868e-30",
        "0x7.f7029d0214354E-25#53",
        Greater,
    );
    test(
        "100000000000000000000000000000000000000001",
        4,
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Equal,
    );
    test(
        "100000000000000000000000000000000000000001",
        4,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "100000000000000000000000000000000000000001",
        4,
        10,
        Floor,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "100000000000000000000000000000000000000001",
        4,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "100000000000000000000000000000000000000001",
        4,
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "100000000000000000000000000000000000000001",
        4,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "111414603535684224740921180161/1237940039285380274899124224",
        360,
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Greater,
    );
    test(
        "111414603535684224740921180161/1237940039285380274899124224",
        360,
        53,
        Floor,
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        Less,
    );
    test(
        "342265662061621938404109865451519/1267650600228229401496703205376",
        360,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Less,
    );
    test(
        "1/20",
        1,
        30,
        Floor,
        "0.30901699420",
        "0x0.4f1bbcdc#30",
        Less,
    );
    test(
        "1/20",
        1,
        30,
        Nearest,
        "0.30901699420",
        "0x0.4f1bbcdc#30",
        Less,
    );
    test(
        "3/20",
        1,
        30,
        Floor,
        "0.80901699420",
        "0x0.cf1bbcdc#30",
        Less,
    );
    test(
        "3/20",
        1,
        30,
        Nearest,
        "0.80901699420",
        "0x0.cf1bbcdc#30",
        Less,
    );
    test(
        "7/20",
        1,
        30,
        Floor,
        "0.80901699420",
        "0x0.cf1bbcdc#30",
        Less,
    );
    test(
        "7/20",
        1,
        30,
        Nearest,
        "0.80901699420",
        "0x0.cf1bbcdc#30",
        Less,
    );
    test(
        "9/20",
        1,
        30,
        Floor,
        "0.30901699420",
        "0x0.4f1bbcdc#30",
        Less,
    );
    test(
        "9/20",
        1,
        30,
        Nearest,
        "0.30901699420",
        "0x0.4f1bbcdc#30",
        Less,
    );
    test(
        "11/20",
        1,
        30,
        Floor,
        "-0.30901699467",
        "-0x0.4f1bbcde#30",
        Less,
    );
    test(
        "11/20",
        1,
        30,
        Nearest,
        "-0.30901699420",
        "-0x0.4f1bbcdc#30",
        Greater,
    );
    test(
        "13/20",
        1,
        30,
        Floor,
        "-0.80901699513",
        "-0x0.cf1bbce0#30",
        Less,
    );
    test(
        "13/20",
        1,
        30,
        Nearest,
        "-0.80901699420",
        "-0x0.cf1bbcdc#30",
        Greater,
    );
    test(
        "17/20",
        1,
        30,
        Floor,
        "-0.80901699513",
        "-0x0.cf1bbce0#30",
        Less,
    );
    test(
        "17/20",
        1,
        30,
        Nearest,
        "-0.80901699420",
        "-0x0.cf1bbcdc#30",
        Greater,
    );
    test(
        "19/20",
        1,
        30,
        Floor,
        "-0.30901699467",
        "-0x0.4f1bbcde#30",
        Less,
    );
    test(
        "19/20",
        1,
        30,
        Nearest,
        "-0.30901699420",
        "-0x0.4f1bbcdc#30",
        Greater,
    );
    test(
        "1/3",
        1,
        30,
        Nearest,
        "0.86602540407",
        "0x0.ddb3d744#30",
        Greater,
    );
    test(
        "2/3",
        1,
        30,
        Nearest,
        "-0.86602540407",
        "-0x0.ddb3d744#30",
        Less,
    );
    test(
        "-1/3",
        1,
        30,
        Nearest,
        "-0.86602540407",
        "-0x0.ddb3d744#30",
        Less,
    );
    test(
        "1/6",
        1,
        30,
        Nearest,
        "0.86602540407",
        "0x0.ddb3d744#30",
        Greater,
    );
    test(
        "5/6",
        1,
        30,
        Nearest,
        "-0.86602540407",
        "-0x0.ddb3d744#30",
        Less,
    );
    test(
        "-5/6",
        1,
        30,
        Nearest,
        "0.86602540407",
        "0x0.ddb3d744#30",
        Greater,
    );
    test(
        "1/8",
        1,
        30,
        Nearest,
        "0.70710678119",
        "0x0.b504f334#30",
        Greater,
    );
    test(
        "3/8",
        1,
        30,
        Nearest,
        "0.70710678119",
        "0x0.b504f334#30",
        Greater,
    );
    test(
        "-7/8",
        1,
        30,
        Nearest,
        "0.70710678119",
        "0x0.b504f334#30",
        Greater,
    );
    test("1/12", 1, 10, Exact, "0.50000", "0x0.800#10", Equal);
    test("5/12", 1, 10, Exact, "0.50000", "0x0.800#10", Equal);
    test("7/12", 1, 10, Exact, "-0.50000", "-0x0.800#10", Equal);
    test("11/12", 1, 10, Exact, "-0.50000", "-0x0.800#10", Equal);
    test("-1/12", 1, 10, Exact, "-0.50000", "-0x0.800#10", Equal);
    test("1/4", 1, 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("3/4", 1, 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test("-1/4", 1, 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test("1/2", 1, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1/2", 1, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("3/2", 1, 10, Exact, "0.0", "0x0.0", Equal);
    test("-3/2", 1, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("5", 1, 10, Exact, "0.0", "0x0.0", Equal);
    test("-5", 1, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("1", 2, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1", 2, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("3", 2, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "1/1000000000000000000000000000000",
        1,
        10,
        Nearest,
        "6.2862e-30",
        "0x7.f8E-25#10",
        Greater,
    );
    test(
        "-1/1000000000000000000000000000000",
        7,
        10,
        Floor,
        "-8.9825e-31",
        "-0x1.238E-25#10",
        Less,
    );
    test(
        "99999999/100000000",
        1,
        10,
        Nearest,
        "-6.2864e-8",
        "-0x1.0e0E-6#10",
        Less,
    );
    test(
        "-99999999/100000000",
        3,
        10,
        Ceiling,
        "-0.86523",
        "-0x0.dd8#10",
        Greater,
    );
}

// Inputs so small that 2 pi x/u is around the smallest positive Float or below it, decided by the
// scaled computation; none of them is a `Float`.
#[test]
fn test_sin_with_period_rational_tiny() {
    let test = |x: Rational, u, prec, rm, out: Float, out_o| {
        let (s, o) = Float::sin_with_period_rational_prec_round_ref(&x, u, prec, rm);
        assert!(s.is_valid());
        assert_eq!(ComparableFloat(s), ComparableFloat(out));
        assert_eq!(o, out_o);
    };
    let min_exp = -(1i64 << 30);
    let min_positive = |prec| Float::one_prec(prec) >> (1u64 << 30);
    // 2 pi x = pi/2 * 2^(-2^30), about 1.57 times the smallest positive Float
    let x = Rational::power_of_2(min_exp - 2);
    test(x.clone(), 1, 1, Nearest, min_positive(1) << 1u32, Greater);
    test(x.clone(), 1, 1, Floor, min_positive(1), Less);
    test(
        x.clone(),
        1,
        10,
        Nearest,
        Float::from_rational_prec(Rational::from(1608u32) >> 10u32, 10).0 >> (1u64 << 30),
        Less,
    );
    // 2 pi x = pi/4 * 2^(-2^30), about 0.79 times it: below, but above half
    let x = Rational::power_of_2(min_exp - 3);
    test(x.clone(), 1, 10, Nearest, min_positive(10), Greater);
    test(x.clone(), 1, 10, Down, Float::ZERO, Less);
    test(-&x, 1, 10, Nearest, -min_positive(10), Less);
    test(-&x, 1, 10, Ceiling, Float::NEGATIVE_ZERO, Greater);
    // 2 pi x = pi/8 * 2^(-2^30), about 0.39 times it: below half
    let x = Rational::power_of_2(min_exp - 4);
    test(x.clone(), 1, 10, Nearest, Float::ZERO, Less);
    test(x, 1, 10, Up, min_positive(10), Greater);
    // and with a divisor
    test(
        Rational::power_of_2(min_exp),
        100,
        10,
        Nearest,
        Float::ZERO,
        Less,
    );
    test(
        Rational::power_of_2(min_exp),
        7,
        10,
        Nearest,
        min_positive(10),
        Greater,
    );
}

#[test]
#[should_panic]
fn sin_with_period_rational_prec_round_fail_1() {
    Float::sin_with_period_rational_prec_round(Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn sin_with_period_rational_prec_round_fail_2() {
    Float::sin_with_period_rational_prec_round(Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn sin_with_period_rational_prec_round_ref_fail() {
    Float::sin_with_period_rational_prec_round_ref(&Rational::ONE, 7, 10, Exact);
}

#[test]
#[should_panic]
fn sin_with_period_rational_prec_fail() {
    Float::sin_with_period_rational_prec(Rational::ONE, 7, 0);
}

#[allow(clippy::needless_pass_by_value)]
fn sin_with_period_rational_prec_round_properties_helper(
    x: Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) {
    if rm == Exact {
        // Exact is only allowed when the result is exactly representable; otherwise panic.
        let (s, o) = Float::sin_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
        if o == Equal {
            let (se, oe) = Float::sin_with_period_rational_prec_round_ref(&x, u, prec, Exact);
            assert_eq!(ComparableFloatRef(&se), ComparableFloatRef(&s));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(Float::sin_with_period_rational_prec_round_ref(
                &x, u, prec, Exact
            ));
        }
        return;
    }
    let (s, o) = Float::sin_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = Float::sin_with_period_rational_prec_round_ref(&x, u, prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    // MPFR rounds the input to a `Float` first, so it cannot see the exact cases of non-dyadic
    // inputs (1/12 of a turn, say); those are checked separately below.
    if o != Equal
        && let Ok(rrm) = rug_round_try_from_rounding_mode(rm)
        && u32::try_from(u).is_ok()
    {
        let (rug_s, rug_o) = rug_sin_with_period_rational_prec_round(&x, u, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    }

    // NaN exactly for u = 0, and otherwise in [-1, 1]
    assert_eq!(s.is_nan(), u == 0);
    if u != 0 {
        assert!(s.le_abs(&1u32));
        if s.is_normal() {
            assert_eq!(s.get_prec(), Some(prec));
        }
        // odd (a `Rational` has no negative zero, so x = 0 is excluded), and periodic with period u
        // up to the sign of a zero
        if x != 0u32 {
            let (s_neg, o_neg) = Float::sin_with_period_rational_prec_round(-&x, u, prec, -rm);
            assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
            assert_eq!(o_neg, o.reverse());
        }
        let (s_shifted, o_shifted) =
            Float::sin_with_period_rational_prec_round(&x + Rational::from(u), u, prec, rm);
        assert_eq!(
            ComparableFloat(s_shifted.abs_negative_zero()),
            ComparableFloat(s.abs_negative_zero_ref())
        );
        assert_eq!(o_shifted, o);
        // a Float input agrees with the Float version
        if let Ok(f) = Float::try_from(&x) {
            let (s_alt, o_alt) = f.sin_with_period_prec_round(u, prec, rm);
            assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
            assert_eq!(o_alt, o);
        }
    }

    if o == Equal {
        for rm in exhaustive_rounding_modes() {
            let (s2, oo) = Float::sin_with_period_rational_prec_round_ref(&x, u, prec, rm);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::sin_with_period_rational_prec_round_ref(
            &x, u, prec, Exact
        ));
    }
}

#[test]
fn sin_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, u, prec, rm)| {
            sin_with_period_rational_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::sin_with_period_rational_prec_round(Rational::ZERO, 4, prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (s, o) = Float::sin_with_period_rational_prec_round(Rational::ONE, 0, prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);
        // exact cases, straight from a fraction of a turn
        let half = Float::one_prec(prec) >> 1u32;
        for (s, expected) in [
            ("1/4", Float::one_prec(prec)),
            ("1/2", Float::ZERO),
            ("-1/2", Float::NEGATIVE_ZERO),
            ("3/4", -Float::one_prec(prec)),
            ("1", Float::ZERO),
            ("-1", Float::NEGATIVE_ZERO),
            ("1/12", half.clone()),
            ("5/12", half.clone()),
            ("7/12", -&half),
            ("-1/12", -&half),
        ] {
            let (c, o) = Float::sin_with_period_rational_prec_round(
                Rational::from_str(s).unwrap(),
                1,
                prec,
                rm,
            );
            assert_eq!(ComparableFloat(c), ComparableFloat(expected));
            assert_eq!(o, Equal);
        }
    });
}

#[test]
fn sin_with_period_rational_prec_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, u, prec, _)| {
            let (s, o) = Float::sin_with_period_rational_prec(x.clone(), u, prec);
            assert!(s.is_valid());
            assert_rounding_ordering_consistent(&s, Nearest, o);
            let (s_alt, o_alt) = Float::sin_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
            assert_eq!(o_alt, o);
            let (s_alt, o_alt) =
                Float::sin_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
            assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
            assert_eq!(o_alt, o);
            if o != Equal && u32::try_from(u).is_ok() {
                let (rug_s, rug_o) = rug_sin_with_period_rational_prec(&x, u, prec);
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_s)),
                    ComparableFloatRef(&s)
                );
                assert_eq!(rug_o, o);
            }
        },
    );
}

// Inputs within 2^(-2^30) of a half turn, whose sines underflow, including a non-dyadic one that no
// `Float` could express.
#[test]
fn test_sin_with_period_rational_underflow() {
    let min_positive = Float::one_prec(10) >> (1u64 << 30);
    let eps = Rational::power_of_2(-((1i64 << 30) + 70));
    let above = Rational::from(180u32) + &eps;
    let below = Rational::from(180u32) - eps;
    let (s, o) = Float::sin_with_period_rational_prec_round_ref(&above, 360, 10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    let (s, o) = Float::sin_with_period_rational_prec_round_ref(&above, 360, 10, Floor);
    assert_eq!(ComparableFloat(s), ComparableFloat(-&min_positive));
    assert_eq!(o, Less);
    let (s, o) = Float::sin_with_period_rational_prec_round_ref(&below, 360, 10, Ceiling);
    assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&min_positive));
    assert_eq!(o, Greater);
    let (s, o) = Float::sin_with_period_rational_prec_round_ref(&below, 360, 10, Down);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    // a non-dyadic version: 1/2 + 1/(3 * 2^(2^30 + 70)) of a turn
    let tiny = Rational::power_of_2(-((1i64 << 30) + 70)) / Rational::from(3u32);
    let x = Rational::from_unsigneds(1u32, 2u32) + tiny;
    let (s, o) = Float::sin_with_period_rational_prec_round_ref(&x, 1, 10, Nearest);
    assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sin_with_period() {
    fn test<T: PrimitiveFloat>(x: T, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_sin_with_period(x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 360, f32::NAN);
    test::<f32>(f32::INFINITY, 360, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, 360, f32::NAN);
    test::<f32>(1.0, 0, f32::NAN);
    test::<f32>(0.0, 360, 0.0);
    test::<f32>(-0.0, 360, -0.0);
    test::<f32>(90.0, 360, 1.0);
    test::<f32>(180.0, 360, 0.0);
    test::<f32>(-180.0, 360, -0.0);
    test::<f32>(270.0, 360, -1.0);
    test::<f32>(360.0, 360, 0.0);
    test::<f32>(-360.0, 360, -0.0);
    test::<f32>(30.0, 360, 0.5);
    test::<f32>(150.0, 360, 0.5);
    test::<f32>(210.0, 360, -0.5);
    test::<f32>(45.0, 360, core::f32::consts::FRAC_1_SQRT_2);
    test::<f32>(60.0, 360, 0.8660254);
    test::<f32>(18.0, 360, 0.309017);
    test::<f32>(54.0, 360, 0.809017);
    test::<f32>(1.0, 7, 0.7818315);
    test::<f32>(-1.0, 7, -0.7818315);
    test::<f32>(2.0, 7, 0.9749279);
    test::<f32>(1.0, 360, 0.017452406);
    test::<f32>(100.0, 360, 0.9848077);
    test::<f32>(1.0e10, 360, -0.9848077);
    test::<f32>(1.0e30, 7, 0.7818315);
    test::<f32>(1.0e-30, 7, 8.975979e-31);
    test::<f32>(3.4028235e38, 360, 0.0);
    test::<f32>(0.5, 1, 0.0);
    test::<f32>(0.25, 1, 1.0);
    test::<f32>(0.1, 1, 0.58778524);
    test::<f32>(1.0e-45, 1, 8.0e-45);
    test::<f32>(1.0e-45, 360, 0.0);
    test::<f32>(-1.0e-45, 1, -8.0e-45);
    test::<f64>(f64::NAN, 360, f64::NAN);
    test::<f64>(f64::INFINITY, 360, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, 360, f64::NAN);
    test::<f64>(1.0, 0, f64::NAN);
    test::<f64>(0.0, 360, 0.0);
    test::<f64>(-0.0, 360, -0.0);
    test::<f64>(90.0, 360, 1.0);
    test::<f64>(180.0, 360, 0.0);
    test::<f64>(-180.0, 360, -0.0);
    test::<f64>(270.0, 360, -1.0);
    test::<f64>(360.0, 360, 0.0);
    test::<f64>(-360.0, 360, -0.0);
    test::<f64>(30.0, 360, 0.5);
    test::<f64>(150.0, 360, 0.5);
    test::<f64>(210.0, 360, -0.5);
    test::<f64>(45.0, 360, core::f64::consts::FRAC_1_SQRT_2);
    test::<f64>(60.0, 360, 0.8660254037844386);
    test::<f64>(18.0, 360, 0.30901699437494745);
    test::<f64>(54.0, 360, 0.8090169943749475);
    test::<f64>(1.0, 7, 0.7818314824680298);
    test::<f64>(-1.0, 7, -0.7818314824680298);
    test::<f64>(2.0, 7, 0.9749279121818236);
    test::<f64>(1.0, 360, 0.01745240643728351);
    test::<f64>(100.0, 360, 0.984807753012208);
    test::<f64>(1.0e10, 360, -0.984807753012208);
    test::<f64>(1.0e100, 7, 0.9749279121818236);
    test::<f64>(1.0e-100, 7, 8.975979010256552e-101);
    test::<f64>(1.7976931348623157e308, 360, 0.7880107536067219);
    test::<f64>(0.5, 1, 0.0);
    test::<f64>(0.25, 1, 1.0);
    test::<f64>(0.1, 1, 0.5877852522924731);
    test::<f64>(5.0e-324, 1, 3.0e-323);
    test::<f64>(5.0e-324, 360, 0.0);
    test::<f64>(-5.0e-324, 1, -3.0e-323);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sin_with_period_rational() {
    fn test<T: PrimitiveFloat>(s: &str, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_sin_with_period_rational::<T>(&x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 360, 0.0);
    test::<f32>("1", 0, f32::NAN);
    test::<f32>("90", 360, 1.0);
    test::<f32>("180", 360, 0.0);
    test::<f32>("-180", 360, -0.0);
    test::<f32>("30", 360, 0.5);
    test::<f32>("45", 360, core::f32::consts::FRAC_1_SQRT_2);
    test::<f32>("60", 360, 0.8660254);
    test::<f32>("18", 360, 0.309017);
    test::<f32>("54", 360, 0.809017);
    test::<f32>("1/12", 1, 0.5);
    test::<f32>("1/3", 1, 0.8660254);
    test::<f32>("1/8", 1, core::f32::consts::FRAC_1_SQRT_2);
    test::<f32>("1/20", 1, 0.309017);
    test::<f32>("1/5", 1, 0.95105654);
    test::<f32>("1/7", 1, 0.7818315);
    test::<f32>("-2/7", 1, -0.9749279);
    test::<f32>("22/7", 1, 0.7818315);
    test::<f32>("1", 7, 0.7818315);
    test::<f32>("1000000", 7, 0.7818315);
    test::<f32>("1/1000000", 1, 0.0000062831855);
    test::<f32>("355/113", 360, 0.05480367);
    test::<f64>("0", 360, 0.0);
    test::<f64>("1", 0, f64::NAN);
    test::<f64>("90", 360, 1.0);
    test::<f64>("180", 360, 0.0);
    test::<f64>("-180", 360, -0.0);
    test::<f64>("30", 360, 0.5);
    test::<f64>("45", 360, core::f64::consts::FRAC_1_SQRT_2);
    test::<f64>("60", 360, 0.8660254037844386);
    test::<f64>("18", 360, 0.30901699437494745);
    test::<f64>("54", 360, 0.8090169943749475);
    test::<f64>("1/12", 1, 0.5);
    test::<f64>("1/3", 1, 0.8660254037844386);
    test::<f64>("1/8", 1, core::f64::consts::FRAC_1_SQRT_2);
    test::<f64>("1/20", 1, 0.30901699437494745);
    test::<f64>("1/5", 1, 0.9510565162951535);
    test::<f64>("1/7", 1, 0.7818314824680298);
    test::<f64>("-2/7", 1, -0.9749279121818236);
    test::<f64>("22/7", 1, 0.7818314824680298);
    test::<f64>("1", 7, 0.7818314824680298);
    test::<f64>("1000000", 7, 0.7818314824680298);
    test::<f64>("1/1000000", 1, 6.283185307138245e-6);
    test::<f64>("355/113", 360, 0.05480366979770582);
    // tiny inputs, whose sines are subnormal or zero
    let tiny = |zeros: usize| format!("1/1{}", "0".repeat(zeros));
    test::<f32>(&tiny(40), 1, 6.28318e-40);
    test::<f32>(&tiny(40), 360, 1.746e-42);
    test::<f32>(&tiny(50), 1, 0.0);
    test::<f64>(&tiny(310), 1, 6.28318530717956e-310);
    test::<f64>(&format!("-{}", tiny(310)), 7, -8.9759790102563e-311);
    test::<f64>(&tiny(330), 1, 0.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sin_with_period_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        let s = primitive_float_sin_with_period(x, u);
        // NaN exactly for u = 0 (the inputs are finite), and otherwise in [-1, 1]
        assert_eq!(s.is_nan(), u == 0);
        if u != 0 {
            assert!(s >= T::NEGATIVE_ONE && s <= T::ONE);
            // odd
            assert_eq!(
                NiceFloat(primitive_float_sin_with_period(-x, u)),
                NiceFloat(-s)
            );
            // the result is the correctly rounded sine, as computed by MPFR with 64 bits to spare,
            // so that a subnormal result is rounded once by the conversion
            let rug_s = rug_sin_with_period_prec(
                &rug::Float::exact_from(&Float::from(x)),
                u,
                T::MANTISSA_WIDTH + 64,
            )
            .0;
            let rug_s: T = T::rounding_from(&<Float as From<&rug::Float>>::from(&rug_s), Nearest).0;
            assert_eq!(NiceFloat(rug_s), NiceFloat(s));
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // NaN exactly for NaN and infinite inputs
        assert_eq!(
            primitive_float_sin_with_period(x, 7).is_nan(),
            !x.is_finite()
        );
    });
}

#[test]
fn primitive_float_sin_with_period_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sin_with_period_properties_helper);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sin_with_period_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, u)| {
        let s = primitive_float_sin_with_period_rational::<T>(&x, u);
        assert_eq!(s.is_nan(), u == 0);
        if u != 0 {
            assert!(s >= T::NEGATIVE_ONE && s <= T::ONE);
            // odd (a `Rational` has no negative zero), and periodic with period u up to the sign of
            // a zero
            if x != 0u32 {
                assert_eq!(
                    NiceFloat(primitive_float_sin_with_period_rational::<T>(&-&x, u)),
                    NiceFloat(-s)
                );
            }
            assert_eq!(
                NiceFloat(
                    primitive_float_sin_with_period_rational::<T>(&(&x + Rational::from(u)), u)
                        .abs()
                ),
                NiceFloat(s.abs())
            );
            // MPFR agrees, except that it cannot see the exact cases of non-dyadic inputs
            let (s_float, o) =
                Float::sin_with_period_rational_prec_ref(&x, u, T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&s_float, Nearest).0),
                NiceFloat(s)
            );
            if o != Equal {
                let rug_s = rug_sin_with_period_rational_prec(&x, u, T::MANTISSA_WIDTH + 64).0;
                let rug_s: T =
                    T::rounding_from(&<Float as From<&rug::Float>>::from(&rug_s), Nearest).0;
                assert_eq!(NiceFloat(rug_s), NiceFloat(s));
            }
        }
    });

    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        // The sine of a finite nonzero primitive float, taken through the `Rational` path, matches
        // the direct primitive-float sine (a `Rational` cannot carry the sign of a zero).
        if x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_sin_with_period_rational::<T>(
                    &Rational::exact_from(x),
                    u
                )),
                NiceFloat(primitive_float_sin_with_period(x, u))
            );
        }
    });
}

#[test]
fn primitive_float_sin_with_period_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sin_with_period_rational_properties_helper);
}

#[test]
fn test_sin_pi_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().sin_pi_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.sin_pi_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.sin_pi_prec_round_assign(prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = x.sin_pi_prec_ref(prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_sin_pi_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
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
    test("0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 10, Exact, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 53, Nearest, "0.0", "0x0.0", Equal);
    test("0.50", "0x0.8#1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test("0.50", "0x0.8#1", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test("0.50", "0x0.8#1", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test(
        "0.50",
        "0x0.8#1",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test("1.5", "0x1.8#2", 1, Nearest, "-1.0", "-0x1.0#1", Equal);
    test(
        "1.5",
        "0x1.8#2",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test("1.5", "0x1.8#2", 10, Floor, "-1.0000", "-0x1.000#10", Equal);
    test(
        "1.5",
        "0x1.8#2",
        10,
        Ceiling,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test("1.5", "0x1.8#2", 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test(
        "1.5",
        "0x1.8#2",
        53,
        Nearest,
        "-1.0000000000000000",
        "-0x1.0000000000000#53",
        Equal,
    );
    test("1.0", "0x1.0#1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 53, Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 10, Floor, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 10, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 53, Nearest, "0.0", "0x0.0", Equal);
    test("0.25", "0x0.4#1", 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test("0.25", "0x0.4#1", 10, Floor, "0.70703", "0x0.b50#10", Less);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        1,
        Nearest,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Nearest,
        "0.30908",
        "0x0.4f2#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Floor,
        "0.30859",
        "0x0.4f0#10",
        Less,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        10,
        Ceiling,
        "0.30908",
        "0x0.4f2#10",
        Greater,
    );
    test(
        "0.10000000000000001",
        "0x0.1999999999999a#52",
        53,
        Nearest,
        "0.30901699437494745",
        "0x0.4f1bbcdcbfa540#53",
        Greater,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Greater,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        10,
        Nearest,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        10,
        Floor,
        "0.95020",
        "0x0.f34#10",
        Less,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        10,
        Ceiling,
        "0.95117",
        "0x0.f38#10",
        Greater,
    );
    test(
        "0.40000000000000002",
        "0x0.66666666666668#52",
        53,
        Nearest,
        "0.95105651629515364",
        "0x0.f378709a22a800#53",
        Greater,
    );
    test("100.2", "0x64.4#9", 1, Nearest, "0.50", "0x0.8#1", Less);
    test(
        "100.2",
        "0x64.4#9",
        10,
        Nearest,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "100.2",
        "0x64.4#9",
        10,
        Floor,
        "0.70703",
        "0x0.b50#10",
        Less,
    );
    test(
        "100.2",
        "0x64.4#9",
        10,
        Ceiling,
        "0.70801",
        "0x0.b54#10",
        Greater,
    );
    test(
        "100.2",
        "0x64.4#9",
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Floor,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        10,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        53,
        Nearest,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        1,
        Nearest,
        "3.2e-30",
        "0x4.0E-25#1",
        Greater,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        10,
        Nearest,
        "3.1431e-30",
        "0x3.fcE-25#10",
        Greater,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        10,
        Floor,
        "3.1400e-30",
        "0x3.fbE-25#10",
        Less,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        10,
        Ceiling,
        "3.1431e-30",
        "0x3.fcE-25#10",
        Greater,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        53,
        Nearest,
        "3.1415926535897934e-30",
        "0x3.fb814e810a1aaE-25#53",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        1,
        Nearest,
        "-0.25",
        "-0x0.4#1",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Nearest,
        "-0.30908",
        "-0x0.4f2#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Floor,
        "-0.30908",
        "-0x0.4f2#10",
        Less,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        10,
        Ceiling,
        "-0.30859",
        "-0x0.4f0#10",
        Greater,
    );
    test(
        "-0.10000000000000001",
        "-0x0.1999999999999a#52",
        53,
        Nearest,
        "-0.30901699437494745",
        "-0x0.4f1bbcdcbfa540#53",
        Less,
    );
    test("1.0", "0x1.0#1", 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 10, Exact, "-0.0", "-0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("2.0", "0x2.0#2", 10, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#2", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-2.0", "-0x2.0#2", 10, Exact, "-0.0", "-0x0.0", Equal);
    test("-2.0", "-0x2.0#2", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("0.50", "0x0.8#1", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Exact,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test("1.5", "0x1.8#2", 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test(
        "1.5",
        "0x1.8#2",
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test("-1.5", "-0x1.8#2", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test(
        "-1.5",
        "-0x1.8#2",
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "0.25",
        "0x0.4#1",
        20,
        Nearest,
        "0.70710659",
        "0x0.b504f#20",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        20,
        Nearest,
        "0.38268328",
        "0x0.61f788#20",
        Less,
    );
    test(
        "0.38",
        "0x0.6#2",
        20,
        Nearest,
        "0.92387962",
        "0x0.ec836#20",
        Greater,
    );
    test(
        "0.62",
        "0x0.a#3",
        20,
        Nearest,
        "0.92387962",
        "0x0.ec836#20",
        Greater,
    );
    test(
        "1.2",
        "0x1.4#3",
        20,
        Nearest,
        "-0.70710659",
        "-0x0.b504f#20",
        Greater,
    );
}

#[test]
fn test_sin_pi_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::sin_pi_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = Float::sin_pi_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = Float::sin_pi_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
            let (c_alt, o_alt) = Float::sin_pi_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }
    };
    test("0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Exact, "0.0", "0x0.0", Equal);
    test("0", 53, Nearest, "0.0", "0x0.0", Equal);
    test("1/2", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("1/2", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("1/2", 10, Floor, "1.0000", "0x1.000#10", Equal);
    test("1/2", 10, Ceiling, "1.0000", "0x1.000#10", Equal);
    test("1/2", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test(
        "1/2",
        53,
        Nearest,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test("1", 1, Nearest, "0.0", "0x0.0", Equal);
    test("1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1", 10, Floor, "0.0", "0x0.0", Equal);
    test("1", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("1", 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 53, Nearest, "0.0", "0x0.0", Equal);
    test("1/3", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1/3", 10, Nearest, "0.86621", "0x0.ddc#10", Greater);
    test("1/3", 10, Floor, "0.86523", "0x0.dd8#10", Less);
    test("1/3", 10, Ceiling, "0.86621", "0x0.ddc#10", Greater);
    test(
        "1/3",
        53,
        Nearest,
        "0.86602540378443860",
        "0x0.ddb3d742c26550#53",
        Less,
    );
    test("2/3", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("2/3", 10, Nearest, "0.86621", "0x0.ddc#10", Greater);
    test("2/3", 10, Floor, "0.86523", "0x0.dd8#10", Less);
    test("2/3", 10, Ceiling, "0.86621", "0x0.ddc#10", Greater);
    test(
        "2/3",
        53,
        Nearest,
        "0.86602540378443860",
        "0x0.ddb3d742c26550#53",
        Less,
    );
    test("1/4", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("1/4", 10, Nearest, "0.70703", "0x0.b50#10", Less);
    test("1/4", 10, Floor, "0.70703", "0x0.b50#10", Less);
    test("1/4", 10, Ceiling, "0.70801", "0x0.b54#10", Greater);
    test(
        "1/4",
        53,
        Nearest,
        "0.70710678118654757",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test("1/6", 1, Nearest, "0.50", "0x0.8#1", Equal);
    test("1/6", 10, Nearest, "0.50000", "0x0.800#10", Equal);
    test("1/6", 10, Floor, "0.50000", "0x0.800#10", Equal);
    test("1/6", 10, Ceiling, "0.50000", "0x0.800#10", Equal);
    test(
        "1/6",
        53,
        Nearest,
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Equal,
    );
    test("1/5", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("1/5", 10, Nearest, "0.58789", "0x0.968#10", Greater);
    test("1/5", 10, Floor, "0.58691", "0x0.964#10", Less);
    test("1/5", 10, Ceiling, "0.58789", "0x0.968#10", Greater);
    test(
        "1/5",
        53,
        Nearest,
        "0.58778525229247314",
        "0x0.96791823aad2f0#53",
        Greater,
    );
    test("2/5", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("2/5", 10, Nearest, "0.95117", "0x0.f38#10", Greater);
    test("2/5", 10, Floor, "0.95020", "0x0.f34#10", Less);
    test("2/5", 10, Ceiling, "0.95117", "0x0.f38#10", Greater);
    test(
        "2/5",
        53,
        Nearest,
        "0.95105651629515353",
        "0x0.f378709a22a7f8#53",
        Less,
    );
    test("1/7", 1, Nearest, "0.50", "0x0.8#1", Greater);
    test("1/7", 10, Nearest, "0.43408", "0x0.6f2#10", Greater);
    test("1/7", 10, Floor, "0.43359", "0x0.6f0#10", Less);
    test("1/7", 10, Ceiling, "0.43408", "0x0.6f2#10", Greater);
    test(
        "1/7",
        53,
        Nearest,
        "0.43388373911755812",
        "0x0.6f130135c6af04#53",
        Less,
    );
    test("-3/7", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-3/7", 10, Nearest, "-0.97461", "-0x0.f98#10", Greater);
    test("-3/7", 10, Floor, "-0.97559", "-0x0.f9c#10", Less);
    test("-3/7", 10, Ceiling, "-0.97461", "-0x0.f98#10", Greater);
    test(
        "-3/7",
        53,
        Nearest,
        "-0.97492791218182362",
        "-0x0.f994e02ac74b48#53",
        Less,
    );
    test("22/7", 1, Nearest, "-0.50", "-0x0.8#1", Less);
    test("22/7", 10, Nearest, "-0.43408", "-0x0.6f2#10", Less);
    test("22/7", 10, Floor, "-0.43408", "-0x0.6f2#10", Less);
    test("22/7", 10, Ceiling, "-0.43359", "-0x0.6f0#10", Greater);
    test(
        "22/7",
        53,
        Nearest,
        "-0.43388373911755812",
        "-0x0.6f130135c6af04#53",
        Greater,
    );
    test("1/1000000", 1, Nearest, "3.8e-6", "0x0.00004#1", Greater);
    test(
        "1/1000000",
        10,
        Nearest,
        "3.1404e-6",
        "0x0.000034b#10",
        Less,
    );
    test("1/1000000", 10, Floor, "3.1404e-6", "0x0.000034b#10", Less);
    test(
        "1/1000000",
        10,
        Ceiling,
        "3.1441e-6",
        "0x0.000034c#10",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Nearest,
        "3.1415926535846256e-6",
        "0x0.000034b509a70089a8#53",
        Greater,
    );
    test("1", 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-1", 10, Exact, "-0.0", "-0x0.0", Equal);
    test("-1", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("2", 10, Exact, "0.0", "0x0.0", Equal);
    test("2", 10, Nearest, "0.0", "0x0.0", Equal);
    test("-2", 10, Exact, "-0.0", "-0x0.0", Equal);
    test("-2", 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("1/2", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("1/2", 10, Nearest, "1.0000", "0x1.000#10", Equal);
    test("-1/2", 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test("-1/2", 10, Nearest, "-1.0000", "-0x1.000#10", Equal);
    test("3/2", 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test("3/2", 10, Nearest, "-1.0000", "-0x1.000#10", Equal);
    test("1/6", 10, Exact, "0.50000", "0x0.800#10", Equal);
    test("1/6", 10, Nearest, "0.50000", "0x0.800#10", Equal);
    test("5/6", 10, Exact, "0.50000", "0x0.800#10", Equal);
    test("5/6", 10, Nearest, "0.50000", "0x0.800#10", Equal);
    test("-1/6", 10, Exact, "-0.50000", "-0x0.800#10", Equal);
    test("-1/6", 10, Nearest, "-0.50000", "-0x0.800#10", Equal);
    test("1/3", 20, Nearest, "0.86602497", "0x0.ddb3d#20", Less);
    test("1/4", 20, Nearest, "0.70710659", "0x0.b504f#20", Less);
    test("1/5", 20, Nearest, "0.58778572", "0x0.96792#20", Greater);
    test("1/10", 20, Nearest, "0.30901718", "0x0.4f1bc0#20", Greater);
    test("3/10", 20, Nearest, "0.80901718", "0x0.cf1bc#20", Greater);
    test("1/7", 20, Nearest, "0.43388367", "0x0.6f1300#20", Less);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sin_pi() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_sin_pi(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(0.5, 1.0);
    test::<f32>(1.0, 0.0);
    test::<f32>(-1.0, -0.0);
    test::<f32>(0.25, core::f32::consts::FRAC_1_SQRT_2);
    test::<f32>(0.1, 0.309017);
    test::<f32>(-0.1, -0.309017);
    test::<f32>(100.25, core::f32::consts::FRAC_1_SQRT_2);
    test::<f32>(10000000000.0, 0.0);
    test::<f32>(1.0e-45, 4.0e-45);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(0.5, 1.0);
    test::<f64>(1.0, 0.0);
    test::<f64>(-1.0, -0.0);
    test::<f64>(0.25, core::f64::consts::FRAC_1_SQRT_2);
    test::<f64>(0.1, 0.30901699437494745);
    test::<f64>(-0.1, -0.30901699437494745);
    test::<f64>(100.25, core::f64::consts::FRAC_1_SQRT_2);
    test::<f64>(10000000000.0, 0.0);
    test::<f64>(5.0e-324, 1.5e-323);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_sin_pi_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_sin_pi_rational::<T>(&x)),
            NiceFloat(out)
        );
    }
    test::<f32>("1/2", 1.0);
    test::<f32>("1", 0.0);
    test::<f32>("-1", -0.0);
    test::<f32>("1/6", 0.5);
    test::<f32>("1/3", 0.8660254);
    test::<f32>("1/4", core::f32::consts::FRAC_1_SQRT_2);
    test::<f32>("1/7", 0.43388373);
    test::<f32>("-3/7", -0.9749279);
    test::<f32>("22/7", -0.43388373);
    test::<f64>("1/2", 1.0);
    test::<f64>("1", 0.0);
    test::<f64>("-1", -0.0);
    test::<f64>("1/6", 0.5);
    test::<f64>("1/3", 0.8660254037844386);
    test::<f64>("1/4", core::f64::consts::FRAC_1_SQRT_2);
    test::<f64>("1/7", 0.4338837391175581);
    test::<f64>("-3/7", -0.9749279121818236);
    test::<f64>("22/7", -0.4338837391175581);
}

#[test]
#[should_panic]
fn sin_pi_prec_round_fail_1() {
    Float::from(0.1f64).sin_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn sin_pi_prec_round_fail_2() {
    Float::from(0.1f64).sin_pi_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn sin_pi_rational_prec_round_fail() {
    Float::sin_pi_rational_prec_round(Rational::from_unsigneds(1u8, 7), 10, Exact);
}

// Every `sin_pi` variant is `sin_with_period` with a period of 2.
#[test]
fn sin_pi_properties() {
    // The borrowed generators admit `Exact` for inputs whose sine is not exact, so `Exact` is
    // checked against the exactness of the result.
    let exact_ok = |x: &Float, prec: u64, rm: RoundingMode| {
        rm != Exact || x.sin_with_period_prec_round_ref(2, prec, Nearest).1 == Equal
    };
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        if !exact_ok(&x, prec, rm) {
            assert_panic!(x.sin_pi_prec_round_ref(prec, Exact));
            return;
        }
        let (s, o) = x.clone().sin_pi_prec_round(prec, rm);
        assert!(s.is_valid());
        assert_rounding_ordering_consistent(&s, rm, o);
        let (s_alt, o_alt) = x.sin_with_period_prec_round_ref(2, prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        let (s_alt, o_alt) = x.sin_pi_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        let mut s_alt = x.clone();
        let o_alt = s_alt.sin_pi_prec_round_assign(prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_s, rug_o) = rug_sin_pi_prec_round(&rug::Float::exact_from(&x), prec, rrm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(rug_o, o);
        }
    });

    float_unsigned_rounding_mode_triple_gen_var_37().test_properties(|(x, prec, rm)| {
        if !exact_ok(&x, prec, rm) {
            return;
        }
        let (s, o) = x.sin_pi_prec_round_ref(prec, rm);
        let (s_alt, o_alt) = x.sin_with_period_prec_round_ref(2, prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (s, o) = x.clone().sin_pi_prec(prec);
        let (s_alt, o_alt) = x.sin_with_period_prec_ref(2, prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        let (s_alt, o_alt) = x.sin_pi_prec_ref(prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        let mut s_alt = x.clone();
        let o_alt = s_alt.sin_pi_prec_assign(prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
    });

    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        if !exact_ok(&x, x.significant_bits(), rm) {
            return;
        }
        let (s, o) = x.clone().sin_pi_round(rm);
        assert_rounding_ordering_consistent(&s, rm, o);
        let (s_alt, o_alt) = x.sin_with_period_round_ref(2, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        let (s_alt, o_alt) = x.sin_pi_round_ref(rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        let mut s_alt = x.clone();
        let o_alt = s_alt.sin_pi_round_assign(rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
    });

    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        if rm == Exact && Float::sin_with_period_rational_prec_ref(&x, 2, prec).1 != Equal {
            assert_panic!(Float::sin_pi_rational_prec_round_ref(&x, prec, Exact));
            return;
        }
        let (s, o) = Float::sin_pi_rational_prec_round(x.clone(), prec, rm);
        assert!(s.is_valid());
        assert_rounding_ordering_consistent(&s, rm, o);
        let (s_alt, o_alt) = Float::sin_with_period_rational_prec_round_ref(&x, 2, prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        let (s_alt, o_alt) = Float::sin_pi_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        // MPFR agrees, except that it cannot see the exact cases of non-dyadic inputs
        if o != Equal
            && let Ok(rrm) = rug_round_try_from_rounding_mode(rm)
        {
            let (rug_s, rug_o) = rug_sin_pi_rational_prec_round(&x, prec, rrm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(rug_o, o);
        }
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (s, o) = Float::sin_pi_rational_prec(x.clone(), prec);
        let (s_alt, o_alt) = Float::sin_with_period_rational_prec_ref(&x, 2, prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
        let (s_alt, o_alt) = Float::sin_pi_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
    });
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_sin_pi_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        assert_eq!(
            NiceFloat(primitive_float_sin_pi(x)),
            NiceFloat(primitive_float_sin_with_period(x, 2))
        );
    });
    rational_gen().test_properties(|x| {
        assert_eq!(
            NiceFloat(primitive_float_sin_pi_rational::<T>(&x)),
            NiceFloat(primitive_float_sin_with_period_rational::<T>(&x, 2))
        );
    });
}

#[test]
fn primitive_float_sin_pi_properties() {
    apply_fn_to_primitive_floats!(primitive_float_sin_pi_properties_helper);
}
