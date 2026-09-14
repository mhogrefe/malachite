// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Atan, AtanAssign, PowerOf2};
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
    primitive_float_gen, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::atan::{
    primitive_float_atan, primitive_float_atan_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::atan::{
    rug_atan, rug_atan_prec, rug_atan_prec_round, rug_atan_rational_prec,
    rug_atan_rational_prec_round, rug_atan_round,
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

// Rows reuse the sine test's inputs. Branches of `atan_prec_round_normal_ref` covered:
// - tiny x: the small-input shortcut rounds x directly
// - |x| = 1: pi/4 from the constant
// - |x| < 1 and |x| > 1 (inverted, and taken from pi/2), at low precision (no argument reduction,
//   and the table for the first chunks) and above 100 bits (reduced, and summed by binary
//   splitting), at the first working precision and after a retry

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
fn test_atan_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().atan_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.atan_prec_round_ref(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.atan_prec_round_assign(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_atan_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Floor, "1.0", "0x1.0#1", Less);
    test("-Infinity", "-Infinity", 1, Floor, "-2.0", "-0x2.0#1", Less);
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
    test(
        "Infinity", "Infinity", 1, Nearest, "2.0", "0x2.0#1", Greater,
    );
    test(
        "-Infinity",
        "-Infinity",
        1,
        Nearest,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test("0.0", "0x0.0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Floor, "0.50", "0x0.8#1", Less);
    test("1.0", "0x1.0#1", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test("1.0", "0x1.0#1", 10, Floor, "0.78516", "0x0.c90#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "0.78613",
        "0x0.c94#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 10, Nearest, "0.78516", "0x0.c90#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        100,
        Floor,
        "0.78539816339744830961566084581983",
        "0x0.c90fdaa22168c234c4c6628b8#100",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Ceiling,
        "0.78539816339744830961566084582062",
        "0x0.c90fdaa22168c234c4c6628b9#100",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        100,
        Nearest,
        "0.78539816339744830961566084581983",
        "0x0.c90fdaa22168c234c4c6628b8#100",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-0.78516",
        "-0x0.c90#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "1.1074",
        "0x1.1b8#10",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "1.2500",
        "0x1.400#10",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        10,
        Nearest,
        "1.3262",
        "0x1.538#10",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        100,
        Nearest,
        "1.3258176636680324650592392104283",
        "0x1.5368c951e9cfc9a42e1add598#100",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        10,
        Nearest,
        "1.5605",
        "0x1.8f8#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Floor,
        "1.5607966601082313810249815754304",
        "0x1.8f905eb2def218bedcc50e9e8#100",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        100,
        Ceiling,
        "1.5607966601082313810249815754320",
        "0x1.8f905eb2def218bedcc50e9ea#100",
        Greater,
    );
    test(
        "1.00000e6",
        "0xf.424E+4#14",
        64,
        Nearest,
        "1.57079532679489661956",
        "0x1.921fa47d4b30ce82#64",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        50,
        Nearest,
        "0.46364760900080615",
        "0x0.76b19c1586ed4#50",
        Greater,
    );
    test(
        "0.102",
        "0x0.1a#4",
        50,
        Nearest,
        "0.10121544166746665",
        "0x0.19e94153cfdcf0#50",
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
        "1.0038848218538872",
        "0x1.00fe987ed02ff#53",
        Less,
    );
    test(
        "3.14159265358979289",
        "0x3.243f6a8885a2f#54",
        53,
        Nearest,
        "1.2626272556789115",
        "0x1.433b8a322ddd2#53",
        Less,
    );
    test(
        "6.28318530717958579",
        "0x6.487ed5110b45e#54",
        53,
        Nearest,
        "1.4129651365067377",
        "0x1.69b8154baf42e#53",
        Less,
    );
    test(
        "9.979e99",
        "0x1.24E+83#7",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test("3.0", "0x3.0#2", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("3.0", "0x3.0#2", 1, Floor, "1.0", "0x1.0#1", Less);
    test("3.0", "0x3.0#2", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("3.0", "0x3.0#2", 2, Nearest, "1.0", "0x1.0#2", Less);
    test("0.25", "0x0.4#1", 1, Down, "0.12", "0x0.2#1", Less);
    test("1.0", "0x1.0#1", 1, Down, "0.50", "0x0.8#1", Less);
    test("2.0", "0x2.0#1", 1, Down, "1.0", "0x1.0#1", Less);
    test("4.0", "0x4.0#1", 1, Down, "1.0", "0x1.0#1", Less);
    test(
        "-3.495934488151859089160804055e56",
        "-0xe.41ed086a5791d9e5b2924E+46#87",
        2,
        Down,
        "-1.5",
        "-0x1.8#2",
        Greater,
    );
    test(
        "6.28318536",
        "0x6.487ed6#26",
        10,
        Nearest,
        "1.4121",
        "0x1.698#10",
        Less,
    );
    test(
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        53,
        Nearest,
        "1.0038848218538872",
        "0x1.00fe987ed02ff#53",
        Less,
    );
    test(
        "1.5707963267948966192313216916397514420985846996875529104874722",
        "0x1.921fb54442d18469898cc51701b839a252049c1114cf98e804#200",
        100,
        Floor,
        "1.0038848218538872141484239449168",
        "0x1.00fe987ed02ff3793a2b052ee#100",
        Less,
    );
    test(
        "4.7123889803846898576939650749192543286",
        "0x4.b65f1fccc8748d3c9ca64f450528b0#120",
        120,
        Ceiling,
        "1.3616916829711635391094547166587465464",
        "0x1.5c97d37d98aa3b0320822bf82e5dd0#120",
        Greater,
    );
    test(
        "3.14159265358979323851",
        "0x3.243f6a8885a308d4#64",
        64,
        Nearest,
        "1.26262725567891168348",
        "0x1.433b8a322ddd2a32#64",
        Greater,
    );
    // - 2 <= |x| < 3: the sign of atan(x) is the sign of x, so no argument reduction is needed
    //   (MPFR reduces every |x| >= 2); |x| = 3 is reduced
    test("2.5", "0x2.8#3", 10, Nearest, "1.1895", "0x1.308#10", Less);
    test(
        "-2.5",
        "-0x2.8#3",
        10,
        Floor,
        "-1.1914",
        "-0x1.310#10",
        Less,
    );
    test(
        "2.99976",
        "0x2.fff#14",
        20,
        Nearest,
        "1.2490215",
        "0x1.3fbfe#20",
        Greater,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        Nearest,
        "1.2500",
        "0x1.400#10",
        Greater,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        10,
        Ceiling,
        "-1.2480",
        "-0x1.3f8#10",
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
        "1.4121",
        "0x1.698#10",
        Less,
    );
    test(
        "6.28318548",
        "0x6.487ed8#24",
        10,
        Floor,
        "1.4121",
        "0x1.698#10",
        Less,
    );
    test(
        "6.283185307179586476925286766559005788",
        "0x6.487ed5110b4611a62633145c06e10#117",
        100,
        Nearest,
        "1.4129651365067377590637129498565",
        "0x1.69b8154baf42e2f988ab64c3c#100",
        Less,
    );
    // - |x| = 2^(-2^30), the smallest positive Float: atan(x) is just below x, so rounding toward
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
    // - x = 3 * 2^(-2^30) at precision 2, rounded to precision 1: atan(x) is just below the
    //   midpoint 3 * 2^(-2^30), so Nearest rounds down
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
fn atan_prec_round_fail() {
    Float::ONE.atan_prec_round(0, Nearest);
}

#[test]
#[should_panic]
fn atan_prec_round_exact_fail() {
    Float::ONE.atan_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn atan_prec_fail() {
    Float::ONE.atan_prec(0);
}

#[test]
#[should_panic]
fn atan_round_fail() {
    Float::ONE.atan_round(Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn atan_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    // the generator admits `Exact` for the inputs the sine takes exactly, but atan(±infinity) is
    // ±pi/2, never exact
    if rm == Exact && x.is_infinite() {
        assert_panic!(x.atan_prec_round_ref(prec, Exact));
        return;
    }
    let (s, o) = x.clone().atan_prec_round(prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = x.atan_prec_round_ref(prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.atan_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_o) = rug_atan_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    }

    // atan is odd
    let (s_neg, o_neg) = (-&x).atan_prec_round(prec, -rm);
    assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
    assert_eq!(o_neg, o.reverse());

    // |atan x| < pi/2, which at a precision of 1 rounds to 2
    if !s.is_nan() {
        assert!(s.le_abs(&2u32));
    }
    if s.is_normal() {
        assert_eq!(s.get_prec(), Some(prec));
    }

    if o == Equal {
        // atan is exact only for x = 0 (and NaN): the result is rounding-mode-invariant
        if !x.is_nan() {
            assert_eq!(x, 0u32);
            assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&x));
        }
        for rm2 in exhaustive_rounding_modes() {
            let (s2, o2) = x.atan_prec_round_ref(prec, rm2);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.atan_prec_round_ref(prec, Exact));
    }
}

#[test]
fn atan_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        atan_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (s, o) = Float::NAN.atan_prec_round(prec, rm);
        assert!(s.is_nan());
        assert_eq!(o, Equal);

        // atan(±infinity) = ±pi/2, rounded, so `Exact` panics
        if rm == Exact {
            assert_panic!(Float::INFINITY.atan_prec_round(prec, Exact));
            assert_panic!(Float::NEGATIVE_INFINITY.atan_prec_round(prec, Exact));
            return;
        }
        let (s, o) = Float::INFINITY.atan_prec_round(prec, rm);
        let (pi, o_pi) = Float::pi_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(pi >> 1u32));
        assert_eq!(o, o_pi);

        let (s, o) = Float::NEGATIVE_INFINITY.atan_prec_round(prec, rm);
        let (pi, o_pi) = Float::pi_prec_round(prec, -rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(-(pi >> 1u32)));
        assert_eq!(o, o_pi.reverse());

        let (s, o) = Float::ZERO.atan_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);

        let (s, o) = Float::NEGATIVE_ZERO.atan_prec_round(prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
    });
}

#[test]
fn atan_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        // atan(±infinity) is ±pi/2, never exact
        if rm == Exact && x.is_infinite() {
            assert_panic!(x.atan_round_ref(Exact));
            return;
        }
        let (s, o) = x.clone().atan_round(rm);
        assert!(s.is_valid());
        assert_rounding_ordering_consistent(&s, rm, o);
        let (s_alt, o_alt) = x.atan_round_ref(rm);
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.atan_round_assign(rm);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (s_alt, o_alt) = x.atan_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_s, rug_o) = rug_atan_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_s)),
                ComparableFloatRef(&s)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn atan_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (s, o) = x.clone().atan_prec(prec);
        assert!(s.is_valid());
        let (s_alt, o_alt) = x.atan_prec_ref(prec);
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.atan_prec_assign(prec);
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (s_alt, o_alt) = x.atan_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);

        let (rug_s, rug_o) = rug_atan_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_s)),
            ComparableFloatRef(&s)
        );
        assert_eq!(rug_o, o);
    });
}

#[test]
fn atan_properties() {
    float_gen().test_properties(|x| {
        let s = x.clone().atan();
        assert!(s.is_valid());
        let s_alt = (&x).atan();
        assert!(s_alt.is_valid());
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));

        let mut x_alt = x.clone();
        x_alt.atan_assign();
        assert!(x_alt.is_valid());
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&s));

        let (s_alt, _) = x.atan_prec_round_ref(x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));

        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_atan(&rug::Float::exact_from(&x)))),
            ComparableFloatRef(&s)
        );

        assert_eq!(ComparableFloat((-&x).atan()), ComparableFloat(-&s));
        // |atan x| < pi/2, which at a precision of 1 rounds to 2
        if !s.is_nan() {
            assert!(s.le_abs(&2u32));
        }
    });
}

// n * pi, with pi rounded to the nearest `prec` bits and the product exact

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_atan() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_atan(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, core::f32::consts::FRAC_PI_2);
    test::<f32>(f32::NEGATIVE_INFINITY, -core::f32::consts::FRAC_PI_2);
    test::<f32>(0.0, 0.0);
    test::<f32>(-0.0, -0.0);
    test::<f32>(1.0, core::f32::consts::FRAC_PI_4);
    test::<f32>(-1.0, -core::f32::consts::FRAC_PI_4);
    test::<f32>(0.5, 0.4636476);
    test::<f32>(2.0, 1.1071488);
    test::<f32>(100.0, 1.5607966);
    test::<f32>(10000000000.0, core::f32::consts::FRAC_PI_2);
    test::<f32>(1.0e-10, 1.0e-10);
    test::<f32>(core::f32::consts::FRAC_PI_2, 1.0038848);
    test::<f32>(core::f32::consts::PI, 1.2626272);
    test::<f32>(1.0e-45, 1.0e-45);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, core::f64::consts::FRAC_PI_2);
    test::<f64>(f64::NEGATIVE_INFINITY, -core::f64::consts::FRAC_PI_2);
    test::<f64>(0.0, 0.0);
    test::<f64>(-0.0, -0.0);
    test::<f64>(1.0, core::f64::consts::FRAC_PI_4);
    test::<f64>(-1.0, -core::f64::consts::FRAC_PI_4);
    test::<f64>(0.5, 0.4636476090008061);
    test::<f64>(2.0, 1.1071487177940904);
    test::<f64>(100.0, 1.5607966601082315);
    test::<f64>(10000000000.0, 1.5707963266948965);
    test::<f64>(1.0e-10, 1.0e-10);
    test::<f64>(core::f64::consts::FRAC_PI_2, 1.0038848218538872);
    test::<f64>(core::f64::consts::PI, 1.2626272556789118);
    test::<f64>(1.0e300, core::f64::consts::FRAC_PI_2);
    test::<f64>(5.0e-324, 5.0e-324);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_atan_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let t = primitive_float_atan(x);
        // NaN exactly for NaN and infinite inputs NaN exactly for a NaN input
        assert_eq!(t.is_nan(), x.is_nan());
        if !x.is_nan() {
            // odd
            assert_eq!(NiceFloat(primitive_float_atan(-x)), NiceFloat(-t));
            // the result is the correctly rounded arctangent, as computed by MPFR with 64 bits to
            // spare, so that a subnormal result is rounded once by the conversion
            let rug_t = rug_atan_prec(
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
fn primitive_float_atan_properties() {
    apply_fn_to_primitive_floats!(primitive_float_atan_properties_helper);
}

// The smallest positive `Float`, whose arctangent lies just below it: rounded toward zero the
// result underflows, and otherwise it is the input itself. MPFR's small-input shortcut declines the
// underflowing case, which the series bracket then decides.
#[test]
fn test_atan_underflow() {
    let min_positive = Float::one_prec(10) >> (1u64 << 30);
    assert_eq!(min_positive.get_exponent(), Some(Float::MIN_EXPONENT));
    let (a, o) = min_positive.atan_prec_round_ref(10, Floor);
    assert_eq!(ComparableFloat(a), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    let (a, o) = min_positive.atan_prec_round_ref(10, Nearest);
    assert_eq!(ComparableFloatRef(&a), ComparableFloatRef(&min_positive));
    assert_eq!(o, Greater);
    let (a, o) = min_positive.atan_prec_round_ref(10, Ceiling);
    assert_eq!(ComparableFloatRef(&a), ComparableFloatRef(&min_positive));
    assert_eq!(o, Greater);
    let min_negative = -min_positive;
    let (a, o) = min_negative.atan_prec_round_ref(10, Ceiling);
    assert_eq!(ComparableFloat(a), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    let (a, o) = min_negative.atan_prec_round_ref(10, Nearest);
    assert_eq!(ComparableFloatRef(&a), ComparableFloatRef(&min_negative));
    assert_eq!(o, Less);
}

// Rows reuse the sine test's inputs, including the non-dyadic ones whose arctangents MPFR cannot
// see exactly, since it must round the input first. Branches of `atan_rational_helper` covered: the
// shortcut for an input below the exponent range, the series bracket for a tiny input, the
// exactly-representable shortcut, and the general Ziv loop.
#[test]
fn test_atan_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (t, o) = Float::atan_rational_prec_round(x.clone(), prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = Float::atan_rational_prec_round_ref(&x, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = Float::atan_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) = Float::atan_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_t, rug_o) = rug_atan_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
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
    test("1", 5, Nearest, "0.781", "0x0.c8#5", Less);
    test("1", 10, Down, "0.78516", "0x0.c90#10", Less);
    test("1", 10, Up, "0.78613", "0x0.c94#10", Greater);
    test("1", 10, Floor, "0.78516", "0x0.c90#10", Less);
    test("1", 10, Ceiling, "0.78613", "0x0.c94#10", Greater);
    test("1", 10, Nearest, "0.78516", "0x0.c90#10", Less);
    test("1", 20, Nearest, "0.78539848", "0x0.c90fe#20", Greater);
    test(
        "1",
        53,
        Down,
        "0.78539816339744828",
        "0x0.c90fdaa22168c0#53",
        Less,
    );
    test(
        "1",
        53,
        Up,
        "0.78539816339744839",
        "0x0.c90fdaa22168c8#53",
        Greater,
    );
    test(
        "1",
        53,
        Floor,
        "0.78539816339744828",
        "0x0.c90fdaa22168c0#53",
        Less,
    );
    test(
        "1",
        53,
        Ceiling,
        "0.78539816339744839",
        "0x0.c90fdaa22168c8#53",
        Greater,
    );
    test(
        "1",
        53,
        Nearest,
        "0.78539816339744828",
        "0x0.c90fdaa22168c0#53",
        Less,
    );
    test(
        "1",
        100,
        Nearest,
        "0.78539816339744830961566084581983",
        "0x0.c90fdaa22168c234c4c6628b8#100",
        Less,
    );
    test("-1", 1, Down, "-0.50", "-0x0.8#1", Greater);
    test("-1", 1, Up, "-1.0", "-0x1.0#1", Less);
    test("-1", 1, Floor, "-1.0", "-0x1.0#1", Less);
    test("-1", 1, Ceiling, "-0.50", "-0x0.8#1", Greater);
    test("-1", 1, Nearest, "-1.0", "-0x1.0#1", Less);
    test("-1", 5, Nearest, "-0.781", "-0x0.c8#5", Greater);
    test("-1", 10, Down, "-0.78516", "-0x0.c90#10", Greater);
    test("-1", 10, Up, "-0.78613", "-0x0.c94#10", Less);
    test("-1", 10, Floor, "-0.78613", "-0x0.c94#10", Less);
    test("-1", 10, Ceiling, "-0.78516", "-0x0.c90#10", Greater);
    test("-1", 10, Nearest, "-0.78516", "-0x0.c90#10", Greater);
    test("-1", 20, Nearest, "-0.78539848", "-0x0.c90fe#20", Less);
    test(
        "-1",
        53,
        Down,
        "-0.78539816339744828",
        "-0x0.c90fdaa22168c0#53",
        Greater,
    );
    test(
        "-1",
        53,
        Up,
        "-0.78539816339744839",
        "-0x0.c90fdaa22168c8#53",
        Less,
    );
    test(
        "-1",
        53,
        Floor,
        "-0.78539816339744839",
        "-0x0.c90fdaa22168c8#53",
        Less,
    );
    test(
        "-1",
        53,
        Ceiling,
        "-0.78539816339744828",
        "-0x0.c90fdaa22168c0#53",
        Greater,
    );
    test(
        "-1",
        53,
        Nearest,
        "-0.78539816339744828",
        "-0x0.c90fdaa22168c0#53",
        Greater,
    );
    test(
        "-1",
        100,
        Nearest,
        "-0.78539816339744830961566084581983",
        "-0x0.c90fdaa22168c234c4c6628b8#100",
        Greater,
    );
    test("1/2", 1, Down, "0.25", "0x0.4#1", Less);
    test("1/2", 1, Up, "0.50", "0x0.8#1", Greater);
    test("1/2", 1, Floor, "0.25", "0x0.4#1", Less);
    test("1/2", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("1/2", 1, Nearest, "0.50", "0x0.8#1", Greater);
    test("1/2", 5, Nearest, "0.469", "0x0.78#5", Greater);
    test("1/2", 10, Down, "0.46338", "0x0.76a#10", Less);
    test("1/2", 10, Up, "0.46387", "0x0.76c#10", Greater);
    test("1/2", 10, Floor, "0.46338", "0x0.76a#10", Less);
    test("1/2", 10, Ceiling, "0.46387", "0x0.76c#10", Greater);
    test("1/2", 10, Nearest, "0.46387", "0x0.76c#10", Greater);
    test("1/2", 20, Nearest, "0.46364784", "0x0.76b1a0#20", Greater);
    test(
        "1/2",
        53,
        Down,
        "0.46364760900080609",
        "0x0.76b19c1586ed3c#53",
        Less,
    );
    test(
        "1/2",
        53,
        Up,
        "0.46364760900080615",
        "0x0.76b19c1586ed40#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Floor,
        "0.46364760900080609",
        "0x0.76b19c1586ed3c#53",
        Less,
    );
    test(
        "1/2",
        53,
        Ceiling,
        "0.46364760900080615",
        "0x0.76b19c1586ed40#53",
        Greater,
    );
    test(
        "1/2",
        53,
        Nearest,
        "0.46364760900080609",
        "0x0.76b19c1586ed3c#53",
        Less,
    );
    test(
        "1/2",
        100,
        Nearest,
        "0.46364760900080611621425623146131",
        "0x0.76b19c1586ed3da2b7f222f660#100",
        Greater,
    );
    test("1/3", 1, Down, "0.25", "0x0.4#1", Less);
    test("1/3", 1, Up, "0.50", "0x0.8#1", Greater);
    test("1/3", 1, Floor, "0.25", "0x0.4#1", Less);
    test("1/3", 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("1/3", 1, Nearest, "0.25", "0x0.4#1", Less);
    test("1/3", 5, Nearest, "0.328", "0x0.54#5", Greater);
    test("1/3", 10, Down, "0.32129", "0x0.524#10", Less);
    test("1/3", 10, Up, "0.32178", "0x0.526#10", Greater);
    test("1/3", 10, Floor, "0.32129", "0x0.524#10", Less);
    test("1/3", 10, Ceiling, "0.32178", "0x0.526#10", Greater);
    test("1/3", 10, Nearest, "0.32178", "0x0.526#10", Greater);
    test("1/3", 20, Nearest, "0.32175064", "0x0.525e40#20", Greater);
    test(
        "1/3",
        53,
        Down,
        "0.32175055439664219",
        "0x0.525e3e8c9a7b84#53",
        Less,
    );
    test(
        "1/3",
        53,
        Up,
        "0.32175055439664224",
        "0x0.525e3e8c9a7b88#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Floor,
        "0.32175055439664219",
        "0x0.525e3e8c9a7b84#53",
        Less,
    );
    test(
        "1/3",
        53,
        Ceiling,
        "0.32175055439664224",
        "0x0.525e3e8c9a7b88#53",
        Greater,
    );
    test(
        "1/3",
        53,
        Nearest,
        "0.32175055439664219",
        "0x0.525e3e8c9a7b84#53",
        Less,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.32175055439664219340140461435853",
        "0x0.525e3e8c9a7b84920cd43f9520#100",
        Less,
    );
    test("-1/3", 1, Down, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 1, Up, "-0.50", "-0x0.8#1", Less);
    test("-1/3", 1, Floor, "-0.50", "-0x0.8#1", Less);
    test("-1/3", 1, Ceiling, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 1, Nearest, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 5, Nearest, "-0.328", "-0x0.54#5", Less);
    test("-1/3", 10, Down, "-0.32129", "-0x0.524#10", Greater);
    test("-1/3", 10, Up, "-0.32178", "-0x0.526#10", Less);
    test("-1/3", 10, Floor, "-0.32178", "-0x0.526#10", Less);
    test("-1/3", 10, Ceiling, "-0.32129", "-0x0.524#10", Greater);
    test("-1/3", 10, Nearest, "-0.32178", "-0x0.526#10", Less);
    test("-1/3", 20, Nearest, "-0.32175064", "-0x0.525e40#20", Less);
    test(
        "-1/3",
        53,
        Down,
        "-0.32175055439664219",
        "-0x0.525e3e8c9a7b84#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Up,
        "-0.32175055439664224",
        "-0x0.525e3e8c9a7b88#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Floor,
        "-0.32175055439664224",
        "-0x0.525e3e8c9a7b88#53",
        Less,
    );
    test(
        "-1/3",
        53,
        Ceiling,
        "-0.32175055439664219",
        "-0x0.525e3e8c9a7b84#53",
        Greater,
    );
    test(
        "-1/3",
        53,
        Nearest,
        "-0.32175055439664219",
        "-0x0.525e3e8c9a7b84#53",
        Greater,
    );
    test(
        "-1/3",
        100,
        Nearest,
        "-0.32175055439664219340140461435853",
        "-0x0.525e3e8c9a7b84920cd43f9520#100",
        Greater,
    );
    test("3/5", 1, Down, "0.50", "0x0.8#1", Less);
    test("3/5", 1, Up, "1.0", "0x1.0#1", Greater);
    test("3/5", 1, Floor, "0.50", "0x0.8#1", Less);
    test("3/5", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("3/5", 1, Nearest, "0.50", "0x0.8#1", Less);
    test("3/5", 10, Down, "0.54004", "0x0.8a4#10", Less);
    test("3/5", 10, Up, "0.54102", "0x0.8a8#10", Greater);
    test("3/5", 10, Floor, "0.54004", "0x0.8a4#10", Less);
    test("3/5", 10, Ceiling, "0.54102", "0x0.8a8#10", Greater);
    test("3/5", 10, Nearest, "0.54004", "0x0.8a4#10", Less);
    test(
        "3/5",
        53,
        Down,
        "0.54041950027058405",
        "0x0.8a58eeafc86700#53",
        Less,
    );
    test(
        "3/5",
        53,
        Up,
        "0.54041950027058416",
        "0x0.8a58eeafc86708#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Floor,
        "0.54041950027058405",
        "0x0.8a58eeafc86700#53",
        Less,
    );
    test(
        "3/5",
        53,
        Ceiling,
        "0.54041950027058416",
        "0x0.8a58eeafc86708#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Nearest,
        "0.54041950027058416",
        "0x0.8a58eeafc86708#53",
        Greater,
    );
    test(
        "3/5",
        100,
        Nearest,
        "0.54041950027058415544357836460845",
        "0x0.8a58eeafc867076f69547ace0#100",
        Less,
    );
    test("22/7", 1, Down, "1.0", "0x1.0#1", Less);
    test("22/7", 1, Up, "2.0", "0x2.0#1", Greater);
    test("22/7", 1, Floor, "1.0", "0x1.0#1", Less);
    test("22/7", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("22/7", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("22/7", 5, Nearest, "1.25", "0x1.4#5", Less);
    test("22/7", 10, Down, "1.2617", "0x1.430#10", Less);
    test("22/7", 10, Up, "1.2637", "0x1.438#10", Greater);
    test("22/7", 10, Floor, "1.2617", "0x1.430#10", Less);
    test("22/7", 10, Ceiling, "1.2637", "0x1.438#10", Greater);
    test("22/7", 10, Nearest, "1.2637", "0x1.438#10", Greater);
    test("22/7", 20, Nearest, "1.2627430", "0x1.43432#20", Less);
    test(
        "22/7",
        53,
        Down,
        "1.2627435457711200",
        "0x1.4343293852713#53",
        Less,
    );
    test(
        "22/7",
        53,
        Up,
        "1.2627435457711202",
        "0x1.4343293852714#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Floor,
        "1.2627435457711200",
        "0x1.4343293852713#53",
        Less,
    );
    test(
        "22/7",
        53,
        Ceiling,
        "1.2627435457711202",
        "0x1.4343293852714#53",
        Greater,
    );
    test(
        "22/7",
        53,
        Nearest,
        "1.2627435457711202",
        "0x1.4343293852714#53",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "1.2627435457711202143021320542352",
        "0x1.4343293852713db73e6a80f00#100",
        Greater,
    );
    test("-22/7", 1, Down, "-1.0", "-0x1.0#1", Greater);
    test("-22/7", 1, Up, "-2.0", "-0x2.0#1", Less);
    test("-22/7", 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("-22/7", 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("-22/7", 1, Nearest, "-1.0", "-0x1.0#1", Greater);
    test("-22/7", 5, Nearest, "-1.25", "-0x1.4#5", Greater);
    test("-22/7", 10, Down, "-1.2617", "-0x1.430#10", Greater);
    test("-22/7", 10, Up, "-1.2637", "-0x1.438#10", Less);
    test("-22/7", 10, Floor, "-1.2637", "-0x1.438#10", Less);
    test("-22/7", 10, Ceiling, "-1.2617", "-0x1.430#10", Greater);
    test("-22/7", 10, Nearest, "-1.2637", "-0x1.438#10", Less);
    test("-22/7", 20, Nearest, "-1.2627430", "-0x1.43432#20", Greater);
    test(
        "-22/7",
        53,
        Down,
        "-1.2627435457711200",
        "-0x1.4343293852713#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Up,
        "-1.2627435457711202",
        "-0x1.4343293852714#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Floor,
        "-1.2627435457711202",
        "-0x1.4343293852714#53",
        Less,
    );
    test(
        "-22/7",
        53,
        Ceiling,
        "-1.2627435457711200",
        "-0x1.4343293852713#53",
        Greater,
    );
    test(
        "-22/7",
        53,
        Nearest,
        "-1.2627435457711202",
        "-0x1.4343293852714#53",
        Less,
    );
    test(
        "-22/7",
        100,
        Nearest,
        "-1.2627435457711202143021320542352",
        "-0x1.4343293852713db73e6a80f00#100",
        Less,
    );
    test("355/113", 1, Down, "1.0", "0x1.0#1", Less);
    test("355/113", 1, Up, "2.0", "0x2.0#1", Greater);
    test("355/113", 1, Floor, "1.0", "0x1.0#1", Less);
    test("355/113", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("355/113", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("355/113", 5, Nearest, "1.25", "0x1.4#5", Less);
    test("355/113", 10, Down, "1.2617", "0x1.430#10", Less);
    test("355/113", 10, Up, "1.2637", "0x1.438#10", Greater);
    test("355/113", 10, Floor, "1.2617", "0x1.430#10", Less);
    test("355/113", 10, Ceiling, "1.2637", "0x1.438#10", Greater);
    test("355/113", 10, Nearest, "1.2617", "0x1.430#10", Less);
    test("355/113", 20, Nearest, "1.2626266", "0x1.433b8#20", Less);
    test(
        "355/113",
        53,
        Down,
        "1.2626272802211267",
        "0x1.433b8a9b96509#53",
        Less,
    );
    test(
        "355/113",
        53,
        Up,
        "1.2626272802211269",
        "0x1.433b8a9b9650a#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Floor,
        "1.2626272802211267",
        "0x1.433b8a9b96509#53",
        Less,
    );
    test(
        "355/113",
        53,
        Ceiling,
        "1.2626272802211269",
        "0x1.433b8a9b9650a#53",
        Greater,
    );
    test(
        "355/113",
        53,
        Nearest,
        "1.2626272802211267",
        "0x1.433b8a9b96509#53",
        Less,
    );
    test(
        "355/113",
        100,
        Nearest,
        "1.2626272802211267126987321884111",
        "0x1.433b8a9b9650918ff1a33dfd8#100",
        Greater,
    );
    test("3", 1, Down, "1.0", "0x1.0#1", Less);
    test("3", 1, Up, "2.0", "0x2.0#1", Greater);
    test("3", 1, Floor, "1.0", "0x1.0#1", Less);
    test("3", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("3", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("3", 5, Nearest, "1.25", "0x1.4#5", Greater);
    test("3", 10, Down, "1.2480", "0x1.3f8#10", Less);
    test("3", 10, Up, "1.2500", "0x1.400#10", Greater);
    test("3", 10, Floor, "1.2480", "0x1.3f8#10", Less);
    test("3", 10, Ceiling, "1.2500", "0x1.400#10", Greater);
    test("3", 10, Nearest, "1.2500", "0x1.400#10", Greater);
    test("3", 20, Nearest, "1.2490463", "0x1.3fc18#20", Greater);
    test(
        "3",
        53,
        Down,
        "1.2490457723982542",
        "0x1.3fc176b7a855f#53",
        Less,
    );
    test(
        "3",
        53,
        Up,
        "1.2490457723982544",
        "0x1.3fc176b7a8560#53",
        Greater,
    );
    test(
        "3",
        53,
        Floor,
        "1.2490457723982542",
        "0x1.3fc176b7a855f#53",
        Less,
    );
    test(
        "3",
        53,
        Ceiling,
        "1.2490457723982544",
        "0x1.3fc176b7a8560#53",
        Greater,
    );
    test(
        "3",
        53,
        Nearest,
        "1.2490457723982544",
        "0x1.3fc176b7a8560#53",
        Greater,
    );
    test(
        "3",
        100,
        Nearest,
        "1.2490457723982544258299170772811",
        "0x1.3fc176b7a855ffd77cb88581e#100",
        Greater,
    );
    test("100", 1, Down, "1.0", "0x1.0#1", Less);
    test("100", 1, Up, "2.0", "0x2.0#1", Greater);
    test("100", 1, Floor, "1.0", "0x1.0#1", Less);
    test("100", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("100", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("100", 5, Nearest, "1.56", "0x1.9#5", Greater);
    test("100", 10, Down, "1.5605", "0x1.8f8#10", Less);
    test("100", 10, Up, "1.5625", "0x1.900#10", Greater);
    test("100", 10, Floor, "1.5605", "0x1.8f8#10", Less);
    test("100", 10, Ceiling, "1.5625", "0x1.900#10", Greater);
    test("100", 10, Nearest, "1.5605", "0x1.8f8#10", Less);
    test("100", 20, Nearest, "1.5607967", "0x1.8f906#20", Greater);
    test(
        "100",
        53,
        Down,
        "1.5607966601082313",
        "0x1.8f905eb2def21#53",
        Less,
    );
    test(
        "100",
        53,
        Up,
        "1.5607966601082315",
        "0x1.8f905eb2def22#53",
        Greater,
    );
    test(
        "100",
        53,
        Floor,
        "1.5607966601082313",
        "0x1.8f905eb2def21#53",
        Less,
    );
    test(
        "100",
        53,
        Ceiling,
        "1.5607966601082315",
        "0x1.8f905eb2def22#53",
        Greater,
    );
    test(
        "100",
        53,
        Nearest,
        "1.5607966601082315",
        "0x1.8f905eb2def22#53",
        Greater,
    );
    test(
        "100",
        100,
        Nearest,
        "1.5607966601082313810249815754304",
        "0x1.8f905eb2def218bedcc50e9e8#100",
        Less,
    );
    test("1000000", 1, Down, "1.0", "0x1.0#1", Less);
    test("1000000", 1, Up, "2.0", "0x2.0#1", Greater);
    test("1000000", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1000000", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1000000", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1000000", 5, Nearest, "1.56", "0x1.9#5", Less);
    test("1000000", 10, Down, "1.5703", "0x1.920#10", Less);
    test("1000000", 10, Up, "1.5723", "0x1.928#10", Greater);
    test("1000000", 10, Floor, "1.5703", "0x1.920#10", Less);
    test("1000000", 10, Ceiling, "1.5723", "0x1.928#10", Greater);
    test("1000000", 10, Nearest, "1.5703", "0x1.920#10", Less);
    test("1000000", 20, Nearest, "1.5707951", "0x1.921fa#20", Less);
    test(
        "1000000",
        53,
        Down,
        "1.5707953267948964",
        "0x1.921fa47d4b30c#53",
        Less,
    );
    test(
        "1000000",
        53,
        Up,
        "1.5707953267948966",
        "0x1.921fa47d4b30d#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Floor,
        "1.5707953267948964",
        "0x1.921fa47d4b30c#53",
        Less,
    );
    test(
        "1000000",
        53,
        Ceiling,
        "1.5707953267948966",
        "0x1.921fa47d4b30d#53",
        Greater,
    );
    test(
        "1000000",
        53,
        Nearest,
        "1.5707953267948966",
        "0x1.921fa47d4b30d#53",
        Greater,
    );
    test(
        "1000000",
        100,
        Nearest,
        "1.5707953267948966195646550249723",
        "0x1.921fa47d4b30ce822275563fc#100",
        Less,
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
        "9.9999999999966665e-7",
        "0x0.000010c6f7a0b5e767#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Up,
        "9.9999999999966686e-7",
        "0x0.000010c6f7a0b5e768#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Floor,
        "9.9999999999966665e-7",
        "0x0.000010c6f7a0b5e767#53",
        Less,
    );
    test(
        "1/1000000",
        53,
        Ceiling,
        "9.9999999999966686e-7",
        "0x0.000010c6f7a0b5e768#53",
        Greater,
    );
    test(
        "1/1000000",
        53,
        Nearest,
        "9.9999999999966665e-7",
        "0x0.000010c6f7a0b5e767#53",
        Less,
    );
    test(
        "1/1000000",
        100,
        Nearest,
        "9.9999999999966666666666686666643e-7",
        "0x0.000010c6f7a0b5e767176ed7361e78#100",
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
        "-9.9999999999966665e-7",
        "-0x0.000010c6f7a0b5e767#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Up,
        "-9.9999999999966686e-7",
        "-0x0.000010c6f7a0b5e768#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Floor,
        "-9.9999999999966686e-7",
        "-0x0.000010c6f7a0b5e768#53",
        Less,
    );
    test(
        "-1/1000000",
        53,
        Ceiling,
        "-9.9999999999966665e-7",
        "-0x0.000010c6f7a0b5e767#53",
        Greater,
    );
    test(
        "-1/1000000",
        53,
        Nearest,
        "-9.9999999999966665e-7",
        "-0x0.000010c6f7a0b5e767#53",
        Greater,
    );
    test(
        "-1/1000000",
        100,
        Nearest,
        "-9.9999999999966666666666686666643e-7",
        "-0x0.000010c6f7a0b5e767176ed7361e78#100",
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
        "1.0020",
        "0x1.008#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "1.0039",
        "0x1.010#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "1.0020",
        "0x1.008#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "1.0039",
        "0x1.010#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "1.0039",
        "0x1.010#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "1.0038853",
        "0x1.00fea#20",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "1.0038848218538872",
        "0x1.00fe987ed02ff#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "1.0038848218538874",
        "0x1.00fe987ed0300#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "1.0038848218538872",
        "0x1.00fe987ed02ff#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "1.0038848218538874",
        "0x1.00fe987ed0300#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "1.0038848218538872",
        "0x1.00fe987ed02ff#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "1.0038848218538872141484239449168",
        "0x1.00fe987ed02ff3793a2b052ee#100",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        5,
        Nearest,
        "1.38",
        "0x1.6#5",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Down,
        "1.3613",
        "0x1.5c8#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Up,
        "1.3633",
        "0x1.5d0#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Floor,
        "1.3613",
        "0x1.5c8#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Ceiling,
        "1.3633",
        "0x1.5d0#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        10,
        Nearest,
        "1.3613",
        "0x1.5c8#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        20,
        Nearest,
        "1.3616924",
        "0x1.5c97e#20",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Down,
        "1.3616916829711634",
        "0x1.5c97d37d98aa3#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Up,
        "1.3616916829711636",
        "0x1.5c97d37d98aa4#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Floor,
        "1.3616916829711634",
        "0x1.5c97d37d98aa3#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Ceiling,
        "1.3616916829711636",
        "0x1.5c97d37d98aa4#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        53,
        Nearest,
        "1.3616916829711636",
        "0x1.5c97d37d98aa4#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        100,
        Nearest,
        "1.3616916829711635391094547166580",
        "0x1.5c97d37d98aa3b0320822bf82#100",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        Nearest,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        5,
        Nearest,
        "1.25",
        "0x1.4#5",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Down,
        "1.2617",
        "0x1.430#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Up,
        "1.2637",
        "0x1.438#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Floor,
        "1.2617",
        "0x1.430#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Ceiling,
        "1.2637",
        "0x1.438#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        10,
        Nearest,
        "1.2617",
        "0x1.430#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        20,
        Nearest,
        "1.2626266",
        "0x1.433b8#20",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Down,
        "1.2626272556789115",
        "0x1.433b8a322ddd2#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Up,
        "1.2626272556789118",
        "0x1.433b8a322ddd3#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Floor,
        "1.2626272556789115",
        "0x1.433b8a322ddd2#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Ceiling,
        "1.2626272556789118",
        "0x1.433b8a322ddd3#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        53,
        Nearest,
        "1.2626272556789118",
        "0x1.433b8a322ddd3#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        100,
        Nearest,
        "1.2626272556789116834443220836063",
        "0x1.433b8a322ddd2a3156f12d8dc#100",
        Greater,
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
        "-1.0020",
        "-0x1.008#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Up,
        "-1.0039",
        "-0x1.010#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Floor,
        "-1.0039",
        "-0x1.010#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Ceiling,
        "-1.0020",
        "-0x1.008#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        10,
        Nearest,
        "-1.0039",
        "-0x1.010#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        20,
        Nearest,
        "-1.0038853",
        "-0x1.00fea#20",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Down,
        "-1.0038848218538872",
        "-0x1.00fe987ed02ff#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Up,
        "-1.0038848218538874",
        "-0x1.00fe987ed0300#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Floor,
        "-1.0038848218538874",
        "-0x1.00fe987ed0300#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Ceiling,
        "-1.0038848218538872",
        "-0x1.00fe987ed02ff#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        53,
        Nearest,
        "-1.0038848218538872",
        "-0x1.00fe987ed02ff#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        100,
        Nearest,
        "-1.0038848218538872141484239449168",
        "-0x1.00fe987ed02ff3793a2b052ee#100",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Down,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Up,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Ceiling,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1,
        Nearest,
        "2.0",
        "0x2.0#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        5,
        Nearest,
        "1.56",
        "0x1.9#5",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Down,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Up,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Floor,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Ceiling,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        10,
        Nearest,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        20,
        Nearest,
        "1.5707970",
        "0x1.921fc#20",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Down,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Up,
        "1.5707963267948968",
        "0x1.921fb54442d19#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Floor,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Ceiling,
        "1.5707963267948968",
        "0x1.921fb54442d19#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        100,
        Nearest,
        "1.5707963267948966192313216916397",
        "0x1.921fb54442d18469898cc5170#100",
        Greater,
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

#[allow(clippy::needless_pass_by_value)]
fn atan_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (s, o) = Float::atan_rational_prec_round(x.clone(), prec, rm);
    assert!(s.is_valid());
    assert_rounding_ordering_consistent(&s, rm, o);

    let (s_alt, o_alt) = Float::atan_rational_prec_round_ref(&x, prec, rm);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    // atan is odd (a `Rational` has no negative zero, so x = 0 is excluded)
    if x != 0u32 {
        let (s_neg, o_neg) = Float::atan_rational_prec_round(-&x, prec, -rm);
        assert_eq!(ComparableFloat(s_neg), ComparableFloat(-&s));
        assert_eq!(o_neg, o.reverse());
    }

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_s, rug_o) = rug_atan_rational_prec_round(&x, prec, rrm);
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
        // only atan(0) = 0 is exact
        assert_eq!(x, 0u32);
        assert_eq!(ComparableFloatRef(&s), ComparableFloatRef(&Float::ZERO));
        for rm in exhaustive_rounding_modes() {
            let (s2, oo) = Float::atan_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&s2), ComparableFloatRef(&s));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::atan_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn atan_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        atan_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // atan(0) = 0, exactly (a `Rational` zero has no sign, so the result is positive)
        let (s, o) = Float::atan_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(s), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn atan_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (s, o) = Float::atan_rational_prec(x.clone(), prec);
    assert!(s.is_valid());

    let (s_alt, o_alt) = Float::atan_rational_prec_ref(&x, prec);
    assert!(s_alt.is_valid());
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let (s_alt, o_alt) = Float::atan_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
    assert_eq!(o_alt, o);

    let (rug_s, rug_o) = rug_atan_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_s)),
        ComparableFloatRef(&s)
    );
    assert_eq!(rug_o, o, "x = {x} prec = {prec}");

    // the arctangent of an exactly representable rational is the Float arctangent
    if let Ok(f) = Float::try_from(&x) {
        let (s_alt, o_alt) = f.atan_prec(prec);
        assert_eq!(ComparableFloatRef(&s_alt), ComparableFloatRef(&s));
        assert_eq!(o_alt, o);
    }
}

#[test]
fn atan_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        atan_rational_prec_properties_helper(x, prec);
    });
}
#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_atan_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_atan_rational::<T>(&x)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0.0);
    test::<f32>("1", core::f32::consts::FRAC_PI_4);
    test::<f32>("-1", -core::f32::consts::FRAC_PI_4);
    test::<f32>("1/2", 0.4636476);
    test::<f32>("1/3", 0.32175055);
    test::<f32>("22/7", 1.2627436);
    test::<f32>("1/7", 0.14189705);
    test::<f32>("100", 1.5607966);
    test::<f32>("355/113", 1.2626272);
    test::<f32>("1/1000000", 0.000001);
    test::<f32>("-2/3", -0.5880026);
    test::<f32>("1000000", 1.5707953);
    test::<f32>("1/100000000000000000000", 1.0e-20);
    test::<f64>("0", 0.0);
    test::<f64>("1", core::f64::consts::FRAC_PI_4);
    test::<f64>("-1", -core::f64::consts::FRAC_PI_4);
    test::<f64>("1/2", 0.4636476090008061);
    test::<f64>("1/3", 0.3217505543966422);
    test::<f64>("22/7", 1.2627435457711202);
    test::<f64>("1/7", 0.14189705460416394);
    test::<f64>("100", 1.5607966601082315);
    test::<f64>("355/113", 1.2626272802211267);
    test::<f64>("1/1000000", 9.999999999996666e-7);
    test::<f64>("-2/3", -0.5880026035475675);
    test::<f64>("1000000", 1.5707953267948966);
    test::<f64>("1/100000000000000000000", 1.0e-20);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_atan_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let s = primitive_float_atan_rational::<T>(&x);
        // the arctangent of a rational is never NaN
        assert!(!s.is_nan());
        // atan is odd (a `Rational` has no negative zero, so x = 0 is excluded)
        if x != 0u32 {
            assert_eq!(
                NiceFloat(primitive_float_atan_rational::<T>(&-&x)),
                NiceFloat(-s)
            );
        }
        // The result is the correctly rounded arctangent, as computed by MPFR with 64 bits to
        // spare. The comparison is skipped when rounding that wider value to `T` is a tie:
        // atan(1/n) is just above n, so for an n that is a midpoint of the `T` grid the wider value
        // rounds to the midpoint itself and the tie breaks the wrong way, though the arctangent is
        // strictly above it.
        let wide = <Float as From<&rug::Float>>::from(
            &rug_atan_rational_prec(&x, T::MANTISSA_WIDTH + 64).0,
        );
        if !ties::<T>(&wide) {
            assert_eq!(NiceFloat(T::rounding_from(&wide, Nearest).0), NiceFloat(s));
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // The arctangent of a finite nonzero primitive float, taken through the `Rational` path,
        // matches the direct primitive-float arctangent (a `Rational` cannot carry the sign of a
        // zero).
        if x.is_finite() && x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_atan_rational::<T>(&Rational::exact_from(x))),
                NiceFloat(primitive_float_atan(x))
            );
        }
    });
}

#[test]
fn primitive_float_atan_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_atan_rational_properties_helper);
}

// A `Rational` beyond the top of the exponent range, where neither it nor its reciprocal is a
// `Float`: the arctangent is within 2^(-2^30) of pi/2, so it rounds exactly as pi/2 does, and the
// bracket confirms that rather than assuming it.
#[test]
fn test_atan_rational_huge() {
    let huge = Rational::power_of_2((1i64 << 30) + 1);
    for rm in [Nearest, Floor, Down, Ceiling, Up] {
        let (a, o) = Float::atan_rational_prec_round_ref(&huge, 10, rm);
        let (pi, o_pi) = Float::pi_prec_round(10, rm);
        assert_eq!(ComparableFloat(a), ComparableFloat(pi >> 1u32));
        assert_eq!(o, o_pi);
    }
    // the arctangent is odd, so the negation mirrors
    let (a, o) = Float::atan_rational_prec_round_ref(&-huge, 10, Nearest);
    let (pi, o_pi) = Float::pi_prec_round(10, Nearest);
    assert_eq!(ComparableFloat(a), ComparableFloat(-(pi >> 1u32)));
    assert_eq!(o, o_pi.reverse());
}

// A `Rational` below the bottom of the exponent range. Since the arctangent is smaller than its
// input in magnitude, the result is below half the smallest positive `Float` and the rounding mode
// alone decides it, without any 2^30-bit arithmetic.
#[test]
fn test_atan_rational_underflow() {
    let min_positive = Float::min_positive_value_prec(10);
    let tiny = Rational::power_of_2(-((1i64 << 30) + 70));
    for (rm, expected, o_out) in [
        (Nearest, Float::ZERO, Less),
        (Floor, Float::ZERO, Less),
        (Down, Float::ZERO, Less),
        (Ceiling, min_positive.clone(), Greater),
        (Up, min_positive.clone(), Greater),
    ] {
        let (a, o) = Float::atan_rational_prec_round_ref(&tiny, 10, rm);
        assert_eq!(ComparableFloat(a), ComparableFloat(expected));
        assert_eq!(o, o_out);
    }
    for (rm, expected, o_out) in [
        (Nearest, Float::NEGATIVE_ZERO, Greater),
        (Ceiling, Float::NEGATIVE_ZERO, Greater),
        (Down, Float::NEGATIVE_ZERO, Greater),
        (Floor, -min_positive.clone(), Less),
        (Up, -min_positive, Less),
    ] {
        let (a, o) = Float::atan_rational_prec_round_ref(&-&tiny, 10, rm);
        assert_eq!(ComparableFloat(a), ComparableFloat(expected));
        assert_eq!(o, o_out);
    }
}
