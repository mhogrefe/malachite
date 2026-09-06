// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Sin, SinAssign};
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeZero, One, Zero,
};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::unsigned_rounding_mode_pair_gen_var_3;
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::float::arithmetic::sin::{
    rug_sin, rug_sin_prec, rug_sin_prec_round, rug_sin_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::panic::catch_unwind;

// rug's exponent range is much wider than `Float`'s, so it cannot see underflow
fn rug_can_see(x: &Float) -> bool {
    x.get_exponent().is_none_or(|e| e > Float::MIN_EXPONENT + 2)
}

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

        if rug_can_see(&x)
            && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        {
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

    let (s_alt, o_alt) = x.sin_prec_round_ref(prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.sin_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    if rug_can_see(&x)
        && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
    {
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

        if rug_can_see(&x)
            && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        {
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

        if rug_can_see(&x) {
            let (rug_s, rug_o) = rug_sin_prec(&rug::Float::exact_from(&x), prec);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(rug_o, o);
        }
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

        if rug_can_see(&x) {
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_sin(&rug::Float::exact_from(&x)))),
                ComparableFloatRef(&s)
            );
        }

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
