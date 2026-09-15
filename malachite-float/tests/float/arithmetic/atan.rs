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
use malachite_base::num::comparison::traits::{EqAbs, PartialOrdAbs};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_unsigned_pair_gen_var_1,
    unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::atan::{
    primitive_float_atan, primitive_float_atan_rational, primitive_float_atan_with_period,
    primitive_float_atan_with_period_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::atan::{
    rug_atan, rug_atan_prec, rug_atan_prec_round, rug_atan_rational_prec,
    rug_atan_rational_prec_round, rug_atan_round, rug_atan_with_period_prec_round,
    rug_atan_with_period_rational_prec, rug_atan_with_period_rational_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_2, float_unsigned_rounding_mode_triple_gen_var_36,
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

// Rows reuse the cosecant's `with_period` inputs. Branches of
// `atan_with_period_prec_round_normal_ref` covered: |x| = 1, which is an exact eighth of a turn;
// the shortcut for a huge x, whose result is one ulp below a quarter turn; and the general Ziv
// loop, at its first working precision and after a retry.
#[test]
fn test_atan_with_period_prec_round() {
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

        let (t, o) = x.clone().atan_with_period_prec_round(u, prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = x.atan_with_period_prec_round_ref(u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        let mut t_alt = x.clone();
        let o_alt = t_alt.atan_with_period_prec_round_assign(u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = x.atan_with_period_prec_ref(u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }
    };
    test("NaN", "NaN", 4, 10, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity",
        "Infinity",
        4,
        10,
        Nearest,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        4,
        10,
        Nearest,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test("0.0", "0x0.0", 4, 10, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 4, 10, Nearest, "-0.0", "-0x0.0", Equal);
    test("1.0", "0x1.0#1", 0, 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 0, 10, Nearest, "0.0", "0x0.0", Equal);
    test("NaN", "NaN", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 360, 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 360, 10, Floor, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "90.0",
        "0x5a.0#6",
        360,
        10,
        Nearest,
        "89.375",
        "0x59.6#10",
        Greater,
    );
    test(
        "90.0",
        "0x5a.0#6",
        360,
        10,
        Floor,
        "89.250",
        "0x59.4#10",
        Less,
    );
    test(
        "90.0",
        "0x5a.0#6",
        360,
        10,
        Nearest,
        "89.375",
        "0x59.6#10",
        Greater,
    );
    test(
        "180.0",
        "0xb4.0#6",
        360,
        10,
        Nearest,
        "89.625",
        "0x59.a#10",
        Less,
    );
    test(
        "180.0",
        "0xb4.0#6",
        360,
        10,
        Floor,
        "89.625",
        "0x59.a#10",
        Less,
    );
    test(
        "180.0",
        "0xb4.0#6",
        360,
        10,
        Nearest,
        "89.625",
        "0x59.a#10",
        Less,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Nearest,
        "89.750",
        "0x59.c#10",
        Less,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Floor,
        "89.750",
        "0x59.c#10",
        Less,
    );
    test(
        "270.0",
        "0x10e.0#8",
        360,
        10,
        Nearest,
        "89.750",
        "0x59.c#10",
        Less,
    );
    test(
        "360.0",
        "0x168.0#6",
        360,
        10,
        Nearest,
        "89.875",
        "0x59.e#10",
        Greater,
    );
    test(
        "360.0",
        "0x168.0#6",
        360,
        10,
        Floor,
        "89.750",
        "0x59.c#10",
        Less,
    );
    test(
        "360.0",
        "0x168.0#6",
        360,
        10,
        Nearest,
        "89.875",
        "0x59.e#10",
        Greater,
    );
    test(
        "60.0",
        "0x3c.0#4",
        360,
        10,
        Nearest,
        "89.000",
        "0x59.0#10",
        Less,
    );
    test(
        "60.0",
        "0x3c.0#4",
        360,
        10,
        Floor,
        "89.000",
        "0x59.0#10",
        Less,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Nearest,
        "89.500",
        "0x59.8#10",
        Less,
    );
    test(
        "120.0",
        "0x78.0#4",
        360,
        10,
        Floor,
        "89.500",
        "0x59.8#10",
        Less,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Nearest,
        "89.750",
        "0x59.c#10",
        Less,
    );
    test(
        "240.0",
        "0xf.0E+1#4",
        360,
        10,
        Floor,
        "89.750",
        "0x59.c#10",
        Less,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Nearest,
        "89.750",
        "0x59.c#10",
        Less,
    );
    test(
        "300.0",
        "0x12c.0#7",
        360,
        10,
        Floor,
        "89.750",
        "0x59.c#10",
        Less,
    );
    test(
        "450.0",
        "0x1c2.0#8",
        360,
        10,
        Nearest,
        "89.875",
        "0x59.e#10",
        Greater,
    );
    test(
        "450.0",
        "0x1c2.0#8",
        360,
        10,
        Floor,
        "89.750",
        "0x59.c#10",
        Less,
    );
    test(
        "450.0",
        "0x1c2.0#8",
        360,
        10,
        Nearest,
        "89.875",
        "0x59.e#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        1,
        10,
        Ceiling,
        "0.12500",
        "0x0.200#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        3,
        10,
        Nearest,
        "0.37500",
        "0x0.600#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        4,
        10,
        Ceiling,
        "0.50000",
        "0x0.800#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        7,
        10,
        Nearest,
        "0.87500",
        "0x0.e00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        360,
        10,
        Nearest,
        "45.000",
        "0x2d.0#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        1000000,
        10,
        Ceiling,
        "1.2506e5",
        "0x1.e88E+4#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        1,
        10,
        Floor,
        "0.073730",
        "0x0.12e0#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        2,
        53,
        Nearest,
        "0.14758361765043326",
        "0x0.25c80a3b3be610#53",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        4,
        10,
        Floor,
        "0.29492",
        "0x0.4b8#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        6,
        53,
        Nearest,
        "0.44275085295129984",
        "0x0.71581eb1b3b234#53",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        12,
        10,
        Ceiling,
        "0.88574",
        "0x0.e2c#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        1000000,
        10,
        Floor,
        "73728.0",
        "0x1.200E+4#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        1,
        10,
        Nearest,
        "0.039001",
        "0x0.09fc#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        2,
        10,
        Ceiling,
        "0.078003",
        "0x0.13f8#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        4,
        10,
        Nearest,
        "0.15601",
        "0x0.27f#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        6,
        10,
        Ceiling,
        "0.23413",
        "0x0.3bf#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        12,
        10,
        Floor,
        "0.46777",
        "0x0.77c#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        1000000,
        10,
        Nearest,
        "38976.0",
        "0x9.84E+3#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        1099511627776,
        53,
        Nearest,
        "42869480286.889137",
        "0x9fb385b5e.e39e8#53",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        2,
        10,
        Floor,
        "0.31250",
        "0x0.500#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        3,
        53,
        Nearest,
        "0.46924943728350177",
        "0x0.7820bb2acc8e18#53",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        6,
        10,
        Floor,
        "0.93848",
        "0x0.f04#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        12,
        10,
        Nearest,
        "1.8770",
        "0x1.e08#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        360,
        53,
        Nearest,
        "56.309932474020215",
        "0x38.4f57bc0fe29c#53",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        1099511627776,
        10,
        Ceiling,
        "1.7207e11",
        "0x2.81E+9#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        2,
        10,
        Nearest,
        "0.35254",
        "0x0.5a4#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        3,
        10,
        Ceiling,
        "0.52930",
        "0x0.878#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        6,
        10,
        Nearest,
        "1.0566",
        "0x1.0e8#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        7,
        10,
        Ceiling,
        "1.2344",
        "0x1.3c0#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Ceiling,
        "63.438",
        "0x3f.7#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        1099511627776,
        10,
        Floor,
        "1.9354e11",
        "0x2.d1E+9#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        1,
        53,
        Nearest,
        "0.19879180882521663",
        "0x0.32e4051d9df308#53",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        3,
        10,
        Floor,
        "0.59570",
        "0x0.988#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        4,
        53,
        Nearest,
        "0.79516723530086653",
        "0x0.cb90147677cc20#53",
        Less,
    );
    test("3.0", "0x3.0#2", 7, 10, Floor, "1.3906", "0x1.640#10", Less);
    test(
        "3.0",
        "0x3.0#2",
        360,
        10,
        Floor,
        "71.500",
        "0x47.8#10",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        1099511627776,
        10,
        Nearest,
        "2.1851e11",
        "0x3.2eE+9#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1,
        10,
        Ceiling,
        "-0.12500",
        "-0x0.200#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        3,
        10,
        Nearest,
        "-0.37500",
        "-0x0.600#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        4,
        10,
        Ceiling,
        "-0.50000",
        "-0x0.800#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        7,
        10,
        Nearest,
        "-0.87500",
        "-0x0.e00#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        360,
        10,
        Nearest,
        "-45.000",
        "-0x2d.0#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        1000000,
        10,
        Ceiling,
        "-1.2493e5",
        "-0x1.e80E+4#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        1,
        10,
        Floor,
        "0.24829",
        "0x0.3f9#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        2,
        53,
        Nearest,
        "0.49681700723509176",
        "0x0.7f2f663e2bdb44#53",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        4,
        10,
        Floor,
        "0.99316",
        "0x0.fe4#10",
        Less,
    );
    test(
        "100.0",
        "0x64.0#5",
        6,
        53,
        Nearest,
        "1.4904510217052753",
        "0x1.7d8e32ba8391d#53",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        12,
        10,
        Ceiling,
        "2.9844",
        "0x2.fc#10",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#5",
        1000000,
        10,
        Floor,
        "2.4832e5",
        "0x3.caE+4#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1,
        10,
        Nearest,
        "0.24878",
        "0x0.3fb#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        2,
        10,
        Ceiling,
        "0.49756",
        "0x0.7f6#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        4,
        10,
        Nearest,
        "0.99512",
        "0x0.fec#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        6,
        10,
        Ceiling,
        "1.4941",
        "0x1.7e8#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        12,
        10,
        Floor,
        "2.9844",
        "0x2.fc#10",
        Less,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1000000,
        10,
        Nearest,
        "2.4883e5",
        "0x3.ccE+4#10",
        Greater,
    );
    test(
        "123.45600000000000",
        "0x7b.74bc6a7ef9dc#53",
        1099511627776,
        53,
        Nearest,
        "273460487915.69312",
        "0x3fab83e6eb.b170#53",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        2,
        10,
        Floor,
        "0.49951",
        "0x0.7fe#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        3,
        53,
        Nearest,
        "0.74999999995225353",
        "0x0.bfffffffcb8090#53",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        6,
        10,
        Floor,
        "1.4980",
        "0x1.7f8#10",
        Less,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        12,
        10,
        Nearest,
        "3.0000",
        "0x3.00#10",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        360,
        53,
        Nearest,
        "89.999999994270425",
        "0x59.ffffffe76444#53",
        Greater,
    );
    test(
        "1.00000000e10",
        "0x2.540be4E+8#24",
        1099511627776,
        10,
        Ceiling,
        "2.7488e11",
        "0x4.00E+9#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        2,
        10,
        Nearest,
        "-0.20483",
        "-0x0.347#10",
        Less,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        3,
        10,
        Ceiling,
        "-0.30713",
        "-0x0.4ea#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        6,
        10,
        Nearest,
        "-0.61426",
        "-0x0.9d4#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        7,
        10,
        Ceiling,
        "-0.71680",
        "-0x0.b78#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        360,
        10,
        Ceiling,
        "-36.812",
        "-0x24.d#10",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        1099511627776,
        10,
        Floor,
        "-1.1261e11",
        "-0x1.a38E+9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1,
        53,
        Nearest,
        "1.5915494309189536e-11",
        "0x1.17fd03a5404edE-9#53",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        3,
        10,
        Floor,
        "4.7692e-11",
        "0x3.47E-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        4,
        53,
        Nearest,
        "6.3661977236758143e-11",
        "0x4.5ff40e95013b4E-9#53",
        Greater,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        7,
        10,
        Floor,
        "1.1130e-10",
        "0x7.a6E-9#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        360,
        10,
        Floor,
        "5.7262e-9",
        "0x1.898E-7#10",
        Less,
    );
    test(
        "1.0000000000000000e-10",
        "0x6.df37f675ef6ecE-9#53",
        1099511627776,
        10,
        Nearest,
        "17.500",
        "0x11.80#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        1,
        10,
        Ceiling,
        "0.019806",
        "0x0.0512#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        3,
        10,
        Nearest,
        "0.059387",
        "0x0.0f34#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        4,
        10,
        Ceiling,
        "0.079224",
        "0x0.1448#10",
        Greater,
    );
    test(
        "0.12",
        "0x0.2#1",
        7,
        10,
        Nearest,
        "0.13843",
        "0x0.237#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        360,
        10,
        Nearest,
        "7.1250",
        "0x7.20#10",
        Less,
    );
    test(
        "0.12",
        "0x0.2#1",
        1000000,
        10,
        Ceiling,
        "19808.0",
        "0x4.d6E+3#10",
        Greater,
    );
    test(
        "1.000000000000000e-30",
        "0x1.4484bfeebc2aE-25#48",
        1,
        53,
        Floor,
        "1.5915494309189534e-31",
        "0x3.3a6134be12ffeE-26#53",
        Less,
    );
    test(
        "-90.000000000000000000000000000808",
        "-0x5a.000000000000000000000040#100",
        360,
        53,
        Nearest,
        "-89.363406424036512",
        "-0x59.5d08341264a4#53",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        8,
        10,
        Ceiling,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        8,
        10,
        Nearest,
        "-1.5898",
        "-0x1.970#10",
        Greater,
    );
    test("5.0", "0x5.0#3", 8, 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("5.0", "0x5.0#3", 8, 10, Up, "1.7500", "0x1.c00#10", Greater);
    test(
        "7.0",
        "0x7.0#3",
        8,
        10,
        Ceiling,
        "1.8203",
        "0x1.d20#10",
        Greater,
    );
    test(
        "-9.00",
        "-0x9.0#4",
        8,
        10,
        Nearest,
        "-1.8594",
        "-0x1.dc0#10",
        Less,
    );
    test("1.0", "0x1.0#1", 12, 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", 12, 10, Up, "1.5000", "0x1.800#10", Equal);
    test(
        "5.0",
        "0x5.0#3",
        12,
        10,
        Ceiling,
        "2.6250",
        "0x2.a0#10",
        Greater,
    );
    test(
        "-7.0",
        "-0x7.0#3",
        12,
        10,
        Nearest,
        "-2.7305",
        "-0x2.bb#10",
        Less,
    );
    test("11.0", "0xb.0#4", 12, 1, Nearest, "2.0", "0x2.0#1", Less);
    test(
        "11.0",
        "0xb.0#4",
        12,
        10,
        Up,
        "2.8281",
        "0x2.d4#10",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        10,
        Ceiling,
        "0.62500",
        "0x0.a00#10",
        Equal,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        5,
        10,
        Nearest,
        "-0.88086",
        "-0x0.e18#10",
        Greater,
    );
    test("3.0", "0x3.0#2", 5, 1, Nearest, "1.0", "0x1.0#1", Greater);
    test(
        "3.0",
        "0x3.0#2",
        5,
        10,
        Up,
        "0.99414",
        "0x0.fe8#10",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        5,
        10,
        Ceiling,
        "1.0566",
        "0x1.0e8#10",
        Greater,
    );
    test(
        "-6.0",
        "-0x6.0#2",
        5,
        10,
        Nearest,
        "-1.1191",
        "-0x1.1e8#10",
        Less,
    );
    test("1.0", "0x1.0#1", 10, 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", 10, 10, Up, "1.2500", "0x1.400#10", Equal);
    test(
        "3.0",
        "0x3.0#2",
        10,
        10,
        Ceiling,
        "1.9883",
        "0x1.fd0#10",
        Greater,
    );
    test(
        "-7.0",
        "-0x7.0#3",
        10,
        10,
        Nearest,
        "-2.2734",
        "-0x2.46#10",
        Greater,
    );
    test("9.00", "0x9.0#4", 10, 1, Nearest, "2.0", "0x2.0#1", Less);
    test(
        "9.00",
        "0x9.0#4",
        10,
        10,
        Up,
        "2.3242",
        "0x2.53#10",
        Greater,
    );
    test(
        "45.0",
        "0x2d.0#6",
        360,
        10,
        Ceiling,
        "88.750",
        "0x58.c#10",
        Greater,
    );
    test(
        "-135.0",
        "-0x87.0#8",
        360,
        10,
        Nearest,
        "-89.625",
        "-0x59.a#10",
        Less,
    );
    test(
        "30.0",
        "0x1e.0#4",
        360,
        1,
        Nearest,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "30.0",
        "0x1e.0#4",
        360,
        10,
        Up,
        "88.125",
        "0x58.2#10",
        Greater,
    );
    test(
        "150.0",
        "0x96.0#7",
        360,
        10,
        Ceiling,
        "89.625",
        "0x59.a#10",
        Greater,
    );
    test(
        "-72.0",
        "-0x48.0#4",
        360,
        10,
        Nearest,
        "-89.250",
        "-0x59.4#10",
        Less,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        1,
        Nearest,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        10,
        Up,
        "89.625",
        "0x59.a#10",
        Greater,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        10,
        Ceiling,
        "88.500",
        "0x58.8#10",
        Greater,
    );
    test(
        "-108.0",
        "-0x6c.0#5",
        360,
        10,
        Nearest,
        "-89.500",
        "-0x59.8#10",
        Less,
    );
    test("2.0", "0x2.0#1", 16, 1, Nearest, "2.0", "0x2.0#1", Less);
    test("2.0", "0x2.0#1", 16, 10, Up, "2.8203", "0x2.d2#10", Greater);
    test(
        "3.0",
        "0x3.0#2",
        24,
        10,
        Ceiling,
        "4.7734",
        "0x4.c6#10",
        Greater,
    );
    test(
        "-4.0",
        "-0x4.0#1",
        20,
        10,
        Nearest,
        "-4.2188",
        "-0x4.38#10",
        Greater,
    );
    test(
        "6.0",
        "0x6.0#2",
        60,
        1,
        Nearest,
        "16.0",
        "0x1.0E+1#1",
        Greater,
    );
    test("6.0", "0x6.0#2", 60, 10, Up, "13.438", "0xd.70#10", Greater);
    test(
        "3.0",
        "0x3.0#2",
        20,
        30,
        Ceiling,
        "3.9758361802",
        "0x3.f9d0666#30",
        Greater,
    );
    test(
        "9.00",
        "0x9.0#4",
        20,
        30,
        Ceiling,
        "4.6477671266",
        "0x4.a5d4110#30",
        Greater,
    );
    test(
        "13.0",
        "0xd.0#4",
        20,
        30,
        Ceiling,
        "4.7556274831",
        "0x4.c170cd8#30",
        Greater,
    );
    test(
        "19.0",
        "0x13.0#5",
        20,
        30,
        Ceiling,
        "4.8326229230",
        "0x4.d526c6a#30",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        6,
        30,
        Floor,
        "0.75000000000",
        "0x0.c0000000#30",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        8,
        30,
        Nearest,
        "1.0000000000",
        "0x1.00000000#30",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#2",
        4,
        10,
        Nearest,
        "0.70508",
        "0x0.b48#10",
        Greater,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        100,
        10,
        Floor,
        "-3.7935e-323228496",
        "-0xf.ecE-268435456#10",
        Less,
    );
    test(
        "-2.4e-323228497",
        "-0x1.0E-268435456#1",
        100,
        10,
        Down,
        "-3.7898e-323228496",
        "-0xf.e8E-268435456#10",
        Greater,
    );
    test(
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        4,
        1,
        Up,
        "2.4e-323228497",
        "0x1.0E-268435456#1",
        Greater,
    );
    // fifths and tenths of a turn, where the cosine is a multiple of the golden ratio
    test(
        "1.0",
        "0x1.0#1",
        5,
        10,
        Nearest,
        "0.62500",
        "0x0.a00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        20,
        Nearest,
        "0.62500000",
        "0x0.a0000#20",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        20,
        Floor,
        "0.62500000",
        "0x0.a0000#20",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        5,
        20,
        Ceiling,
        "0.62500000",
        "0x0.a0000#20",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        5,
        20,
        Nearest,
        "0.88104057",
        "0x0.e18be#20",
        Less,
    );
    test(
        "3.0",
        "0x3.0#2",
        5,
        20,
        Nearest,
        "0.99395943",
        "0x0.fe742#20",
        Greater,
    );
    test(
        "4.0",
        "0x4.0#1",
        5,
        20,
        Nearest,
        "1.0550518",
        "0x1.0e17e#20",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        5,
        20,
        Nearest,
        "-0.62500000",
        "-0x0.a0000#20",
        Equal,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        5,
        20,
        Nearest,
        "-0.88104057",
        "-0x0.e18be#20",
        Greater,
    );
    test(
        "6.0",
        "0x6.0#2",
        5,
        20,
        Nearest,
        "1.1185780",
        "0x1.1e5b2#20",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        10,
        Nearest,
        "1.2500",
        "0x1.400#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        20,
        Nearest,
        "1.2500000",
        "0x1.40000#20",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        20,
        Floor,
        "1.2500000",
        "0x1.40000#20",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        10,
        20,
        Ceiling,
        "1.2500000",
        "0x1.40000#20",
        Equal,
    );
    test(
        "3.0",
        "0x3.0#2",
        10,
        20,
        Nearest,
        "1.9879189",
        "0x1.fce84#20",
        Greater,
    );
    test(
        "7.0",
        "0x7.0#3",
        10,
        20,
        Nearest,
        "2.2741623",
        "0x2.462f8#20",
        Less,
    );
    test(
        "9.00",
        "0x9.0#4",
        10,
        20,
        Nearest,
        "2.3238831",
        "0x2.52ea0#20",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        20,
        Nearest,
        "-1.2500000",
        "-0x1.40000#20",
        Equal,
    );
    test(
        "-3.0",
        "-0x3.0#2",
        10,
        20,
        Nearest,
        "-1.9879189",
        "-0x1.fce84#20",
        Less,
    );
    test(
        "11.0",
        "0xb.0#4",
        10,
        20,
        Nearest,
        "2.3557091",
        "0x2.5b0fc#20",
        Less,
    );
    test(
        "72.0",
        "0x48.0#4",
        360,
        20,
        Nearest,
        "89.204224",
        "0x59.3448#20",
        Less,
    );
    test(
        "36.0",
        "0x24.0#4",
        360,
        20,
        Nearest,
        "88.408813",
        "0x58.68a8#20",
        Less,
    );
    test(
        "108.0",
        "0x6c.0#5",
        360,
        20,
        Nearest,
        "89.469482",
        "0x59.7830#20",
        Less,
    );
    test(
        "144.0",
        "0x9.0E+1#4",
        360,
        20,
        Nearest,
        "89.602173",
        "0x59.9a28#20",
        Greater,
    );
}

#[test]
#[should_panic]
fn atan_with_period_prec_round_fail_1() {
    Float::ONE.atan_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn atan_with_period_prec_round_fail_2() {
    // atan(2) is not an exact number of sevenths of a turn
    Float::from(2u32).atan_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn atan_with_period_prec_round_fail_3() {
    // an eighth of a turn needs more than 3 bits when u = 7
    Float::ONE.atan_with_period_prec_round(7, 1, Exact);
}

#[test]
#[should_panic]
fn atan_with_period_prec_round_ref_fail() {
    Float::ONE.atan_with_period_prec_round_ref(7, 0, Floor);
}

#[test]
#[should_panic]
fn atan_with_period_prec_fail() {
    Float::ONE.atan_with_period_prec(7, 0);
}

#[test]
#[should_panic]
fn atan_with_period_round_fail() {
    Float::from(2u32).atan_with_period_round(7, Exact);
}

// Whether atanu(x, u) is exactly representable at `prec`: only at zero and NaN, at u = 0, and at
// |x| = 1 or infinity, where the result is an eighth or a quarter turn and `prec` must be wide
// enough to hold it.
fn atan_with_period_exact(x: &Float, u: u64, prec: u64) -> bool {
    x.is_nan()
        || *x == 0u32
        || u == 0
        || (x.eq_abs(&1u32) && Float::from_unsigned_prec(u, prec).1 == Equal)
        || (!x.is_finite() && Float::from_unsigned_prec(u, prec).1 == Equal)
}

#[allow(clippy::needless_pass_by_value)]
fn atan_with_period_prec_round_properties_helper(x: Float, u: u64, prec: u64, rm: RoundingMode) {
    if rm == Exact && !atan_with_period_exact(&x, u, prec) {
        assert_panic!(x.atan_with_period_prec_round_ref(u, prec, Exact));
        return;
    }
    let (t, o) = x.clone().atan_with_period_prec_round(u, prec, rm);
    assert!(t.is_valid());
    assert_rounding_ordering_consistent(&t, rm, o);

    let (t_alt, o_alt) = x.atan_with_period_prec_round_ref(u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    let mut t_alt = x.clone();
    let o_alt = t_alt.atan_with_period_prec_round_assign(u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && u <= u64::from(u32::MAX)
    {
        let (rug_t, rug_o) =
            rug_atan_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_t)),
            ComparableFloatRef(&t)
        );
        assert_eq!(rug_o, o);
    }

    // NaN exactly for a NaN input
    assert_eq!(t.is_nan(), x.is_nan());
    if !t.is_nan() {
        // |atanu(x, u)| <= u/4, a quarter turn, and the result never overflows
        assert!(t.is_finite());
        assert!(PartialOrdAbs::le_abs(
            &t,
            &Float::from_unsigned_prec(u, prec + 2).0
        ));
        if t.is_normal() {
            assert_eq!(t.get_prec(), Some(prec));
        }
        // atan_with_period is odd
        let (t_neg, o_neg) = (-&x).atan_with_period_prec_round(u, prec, -rm);
        assert_eq!(ComparableFloatRef(&t_neg), ComparableFloatRef(&-&t));
        assert_eq!(o_neg, o.reverse());
    }

    if o == Equal {
        assert!(atan_with_period_exact(&x, u, prec));
        for rm in exhaustive_rounding_modes() {
            let (t2, oo) = x.atan_with_period_prec_round_ref(u, prec, rm);
            assert_eq!(ComparableFloatRef(&t2), ComparableFloatRef(&t));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.atan_with_period_prec_round_ref(u, prec, Exact));
    }
}

#[test]
fn atan_with_period_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_17().test_properties(
        |(x, u, prec, rm)| {
            atan_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_18().test_properties(
        |(x, u, prec, rm)| {
            atan_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // atanu(NaN, u) = NaN
        let (t, o) = Float::NAN.atan_with_period_prec_round(4, prec, rm);
        assert!(t.is_nan());
        assert_eq!(o, Equal);
        // atanu(±0.0, u) = ±0.0, exactly
        let (t, o) = Float::ZERO.atan_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (t, o) = Float::NEGATIVE_ZERO.atan_with_period_prec_round(4, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
        // atanu(x, 0) = ±0.0 with the sign of x, exactly, which keeps the function odd
        let (t, o) = Float::ONE.atan_with_period_prec_round(0, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (t, o) = (-Float::ONE).atan_with_period_prec_round(0, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
        let (t, o) = Float::INFINITY.atan_with_period_prec_round(0, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        // atanu(±infinity, u) = ±u/4 and atanu(±1, u) = ±u/8, both exact when `prec` holds them
        for (x, k) in [(Float::INFINITY, 2u32), (Float::ONE, 3)] {
            let (q, o_q) = Float::from_unsigned_prec_round(8u32, prec, rm);
            let (t, o) = x.atan_with_period_prec_round(8, prec, rm);
            assert_eq!(ComparableFloat(t), ComparableFloat(q >> k));
            assert_eq!(o, o_q);
        }
    });
}

#[test]
fn atan_with_period_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, u, prec)| {
        let (t, o) = x.clone().atan_with_period_prec(u, prec);
        assert!(t.is_valid());
        let (t_alt, o_alt) = x.atan_with_period_prec_ref(u, prec);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.atan_with_period_prec_round_ref(u, prec, Nearest);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.atan_with_period_prec_assign(u, prec);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn atan_with_period_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_39().test_properties(|(x, u, rm)| {
        if rm == Exact && !atan_with_period_exact(&x, u, x.significant_bits()) {
            assert_panic!(x.atan_with_period_round_ref(u, Exact));
            return;
        }
        let (t, o) = x.clone().atan_with_period_round(u, rm);
        assert!(t.is_valid());
        assert_rounding_ordering_consistent(&t, rm, o);
        let (t_alt, o_alt) = x.atan_with_period_round_ref(u, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let (t_alt, o_alt) = x.atan_with_period_prec_round_ref(u, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
        let mut t_alt = x.clone();
        let o_alt = t_alt.atan_with_period_round_assign(u, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn atan_with_period_properties() {
    float_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, u)| {
        let t = x.clone().atan_with_period(u);
        assert!(t.is_valid());
        let t_alt = x.atan_with_period_ref(u);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let mut t_alt = x.clone();
        t_alt.atan_with_period_assign(u);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        let (t_alt, _) = x.atan_with_period_prec_round_ref(u, x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        // atan_with_period is odd
        assert_eq!(
            ComparableFloatRef(&(-&x).atan_with_period_ref(u)),
            ComparableFloatRef(&-&t)
        );
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_atan_with_period() {
    fn test<T: PrimitiveFloat>(x: T, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_atan_with_period(x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 360, f32::NAN);
    test::<f32>(f32::INFINITY, 360, 90.0);
    test::<f32>(f32::NEGATIVE_INFINITY, 360, -90.0);
    test::<f32>(1.0, 0, 0.0);
    test::<f32>(0.0, 360, 0.0);
    test::<f32>(-0.0, 360, -0.0);
    test::<f32>(90.0, 360, 89.3634);
    test::<f32>(-90.0, 360, -89.3634);
    test::<f32>(270.0, 360, 89.787796);
    test::<f32>(180.0, 360, 89.681694);
    test::<f32>(-180.0, 360, -89.681694);
    test::<f32>(360.0, 360, 89.84084);
    test::<f32>(45.0, 360, 88.72697);
    test::<f32>(135.0, 360, 89.57559);
    test::<f32>(30.0, 360, 88.09085);
    test::<f32>(60.0, 360, 89.04516);
    test::<f32>(120.0, 360, 89.522545);
    test::<f32>(1.0, 7, 0.875);
    test::<f32>(-1.0, 7, -0.875);
    test::<f32>(2.0, 7, 1.2334573);
    test::<f32>(1.0, 360, 45.0);
    test::<f32>(100.0, 360, 89.42706);
    test::<f32>(10000000000.0, 360, 90.0);
    test::<f32>(1.0e30, 7, 1.75);
    test::<f32>(1.0e-30, 7, 1.1140846e-30);
    test::<f32>(3.4028235e38, 360, 90.0);
    test::<f32>(0.5, 1, 0.07379181);
    test::<f32>(0.25, 1, 0.038989566);
    test::<f32>(0.1, 1, 0.01586276);
    test::<f32>(1.0e-45, 1, 0.0);
    test::<f32>(1.0e-45, 360, 8.0e-44);
    test::<f64>(f64::NAN, 360, f64::NAN);
    test::<f64>(f64::INFINITY, 360, 90.0);
    test::<f64>(f64::NEGATIVE_INFINITY, 360, -90.0);
    test::<f64>(1.0, 0, 0.0);
    test::<f64>(0.0, 360, 0.0);
    test::<f64>(-0.0, 360, -0.0);
    test::<f64>(90.0, 360, 89.36340642403651);
    test::<f64>(-90.0, 360, -89.36340642403651);
    test::<f64>(270.0, 360, 89.78779437951188);
    test::<f64>(180.0, 360, 89.68169338854864);
    test::<f64>(-180.0, 360, -89.68169338854864);
    test::<f64>(360.0, 360, 89.84084546625535);
    test::<f64>(45.0, 360, 88.72696997994329);
    test::<f64>(135.0, 360, 89.57559458063852);
    test::<f64>(30.0, 360, 88.09084756700362);
    test::<f64>(60.0, 360, 89.04515874612781);
    test::<f64>(120.0, 360, 89.52254622269042);
    test::<f64>(1.0, 7, 0.875);
    test::<f64>(-1.0, 7, -0.875);
    test::<f64>(2.0, 7, 1.2334573382234835);
    test::<f64>(1.0, 360, 45.0);
    test::<f64>(100.0, 360, 89.42706130231652);
    test::<f64>(10000000000.0, 360, 89.99999999427042);
    test::<f64>(1.0e100, 7, 1.75);
    test::<f64>(1.0e-100, 7, 1.1140846016432673e-100);
    test::<f64>(1.7976931348623157e308, 360, 90.0);
    test::<f64>(0.5, 1, 0.07379180882521663);
    test::<f64>(0.25, 1, 0.03898956518868466);
    test::<f64>(0.1, 1, 0.015862758715276787);
    test::<f64>(5.0e-324, 1, 0.0);
    test::<f64>(5.0e-324, 360, 2.8e-322);
    test::<f32>(72.0, 360, 89.20428);
    test::<f32>(36.0, 360, 88.40886);
    test::<f32>(108.0, 360, 89.4695);
    test::<f32>(144.0, 360, 89.60212);
    test::<f64>(72.0, 360, 89.20427644726072);
    test::<f64>(36.0, 360, 88.40885972880541);
    test::<f64>(108.0, 360, 89.4694986833262);
    test::<f64>(144.0, 360, 89.60211903816541);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_atan_with_period_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        let t = primitive_float_atan_with_period(x, u);
        // NaN exactly for a NaN input
        assert_eq!(t.is_nan(), x.is_nan());
        if !x.is_nan() {
            // odd
            assert_eq!(
                NiceFloat(primitive_float_atan_with_period(-x, u)),
                NiceFloat(-t)
            );
            // the same as the `Float` version taken with 64 bits to spare and rounded once
            let (t_float, _) =
                Float::atan_with_period_prec(Float::from(x), u, T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&t_float, Nearest).0),
                NiceFloat(t)
            );
            // the result never overflows, since it is at most a quarter turn
            assert!(t.is_finite());
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // u = 0 gives a zero with the sign of x, and an infinite input a quarter turn
        assert_eq!(primitive_float_atan_with_period(x, 0).is_nan(), x.is_nan());
    });
}

#[test]
fn primitive_float_atan_with_period_properties() {
    apply_fn_to_primitive_floats!(primitive_float_atan_with_period_properties_helper);
}

// A tiny input with a small period, where the result is about xu/(2 pi) and falls below the
// smallest positive `Float`. MPFR's wider exponent range never reaches this, so the quotient is
// formed with the numerator scaled up and the underflow decided by the rounding mode alone.
#[test]
fn test_atan_with_period_underflow() {
    let min_positive = Float::min_positive_value_prec(10);
    let tiny = Float::one_prec(10) >> (1u64 << 30);
    for (rm, expected, o_out) in [
        (Nearest, Float::ZERO, Less),
        (Floor, Float::ZERO, Less),
        (Down, Float::ZERO, Less),
        (Ceiling, min_positive.clone(), Greater),
        (Up, min_positive.clone(), Greater),
    ] {
        let (t, o) = tiny.atan_with_period_prec_round_ref(1, 10, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(expected));
        assert_eq!(o, o_out);
    }
    // the function is odd, so the negation mirrors
    let (t, o) = (-&tiny).atan_with_period_prec_round_ref(1, 10, Nearest);
    assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    // with u = 8 the result is representable after all
    let (t, o) = tiny.atan_with_period_prec_round_ref(8, 10, Nearest);
    assert!(t > 0u32);
    assert_eq!(o, Greater);
}

#[test]
fn test_atan_with_period_rational_prec_round() {
    let test = |s: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (t, o) = Float::atan_with_period_rational_prec_round(x.clone(), u, prec, rm);
        assert!(t.is_valid());
        assert_eq!(t.to_string(), out);
        assert_eq!(to_hex_string(&t), out_hex);
        assert_eq!(o, o_out);

        let (t_alt, o_alt) = Float::atan_with_period_rational_prec_round_ref(&x, u, prec, rm);
        assert!(t_alt.is_valid());
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (t_alt, o_alt) = Float::atan_with_period_rational_prec(x.clone(), u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) = Float::atan_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u <= u64::from(u32::MAX)
        {
            let (rug_t, rug_o) = rug_atan_with_period_rational_prec_round(&x, u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_t)),
                ComparableFloatRef(&t)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 4, 1, Down, "0.0", "0x0.0", Equal);
    test("0", 360, 1, Up, "0.0", "0x0.0", Equal);
    test("0", 1, 1, Floor, "0.0", "0x0.0", Equal);
    test("0", 2, 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 1000000, 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", 18446744073709551615, 1, Exact, "0.0", "0x0.0", Equal);
    test("0", 3, 5, Nearest, "0.0", "0x0.0", Equal);
    test("0", 65536, 10, Down, "0.0", "0x0.0", Equal);
    test("0", 100, 10, Up, "0.0", "0x0.0", Equal);
    test("0", 8, 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 16, 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 1000, 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 4, 10, Exact, "0.0", "0x0.0", Equal);
    test("0", 360, 20, Nearest, "0.0", "0x0.0", Equal);
    test("0", 1, 53, Down, "0.0", "0x0.0", Equal);
    test("0", 2, 53, Up, "0.0", "0x0.0", Equal);
    test("0", 1000000, 53, Floor, "0.0", "0x0.0", Equal);
    test(
        "0",
        18446744073709551615,
        53,
        Ceiling,
        "0.0",
        "0x0.0",
        Equal,
    );
    test("0", 3, 53, Nearest, "0.0", "0x0.0", Equal);
    test("0", 65536, 53, Exact, "0.0", "0x0.0", Equal);
    test("0", 100, 100, Nearest, "0.0", "0x0.0", Equal);
    test("1", 8, 1, Down, "1.0", "0x1.0#1", Equal);
    test("1", 16, 1, Up, "2.0", "0x2.0#1", Equal);
    test("1", 1000, 1, Floor, "64.0", "0x4.0E+1#1", Less);
    test("1", 4, 1, Ceiling, "0.50", "0x0.8#1", Equal);
    test("1", 360, 1, Nearest, "32.0", "0x2.0E+1#1", Less);
    test("1", 1, 5, Nearest, "0.125", "0x0.20#5", Equal);
    test("1", 2, 10, Down, "0.25000", "0x0.400#10", Equal);
    test("1", 1000000, 10, Up, "1.2506e5", "0x1.e88E+4#10", Greater);
    test(
        "1",
        18446744073709551615,
        10,
        Floor,
        "2.3036e18",
        "0x1.ff8E+15#10",
        Less,
    );
    test("1", 3, 10, Ceiling, "0.37500", "0x0.600#10", Equal);
    test("1", 65536, 10, Nearest, "8192.0", "0x2.00E+3#10", Equal);
    test("1", 100, 20, Nearest, "12.500000", "0xc.8000#20", Equal);
    test(
        "1",
        8,
        53,
        Down,
        "1.0000000000000000",
        "0x1.0000000000000#53",
        Equal,
    );
    test(
        "1",
        16,
        53,
        Up,
        "2.0000000000000000",
        "0x2.0000000000000#53",
        Equal,
    );
    test(
        "1",
        1000,
        53,
        Floor,
        "125.00000000000000",
        "0x7d.000000000000#53",
        Equal,
    );
    test(
        "1",
        4,
        53,
        Ceiling,
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Equal,
    );
    test(
        "1",
        360,
        53,
        Nearest,
        "45.000000000000000",
        "0x2d.000000000000#53",
        Equal,
    );
    test(
        "1",
        1,
        100,
        Nearest,
        "0.12500000000000000000000000000000",
        "0x0.20000000000000000000000000#100",
        Equal,
    );
    test("-1", 2, 1, Down, "-0.25", "-0x0.4#1", Equal);
    test("-1", 1000000, 1, Up, "-1.3e5", "-0x2.0E+4#1", Less);
    test(
        "-1",
        18446744073709551615,
        1,
        Floor,
        "-2.3e18",
        "-0x2.0E+15#1",
        Less,
    );
    test("-1", 3, 1, Ceiling, "-0.25", "-0x0.4#1", Greater);
    test("-1", 65536, 1, Nearest, "-8.2e3", "-0x2.0E+3#1", Equal);
    test("-1", 100, 5, Nearest, "-12.5", "-0xc.8#5", Equal);
    test("-1", 8, 10, Down, "-1.0000", "-0x1.000#10", Equal);
    test("-1", 16, 10, Up, "-2.0000", "-0x2.00#10", Equal);
    test("-1", 1000, 10, Floor, "-125.00", "-0x7d.0#10", Equal);
    test("-1", 4, 10, Ceiling, "-0.50000", "-0x0.800#10", Equal);
    test("-1", 360, 10, Nearest, "-45.000", "-0x2d.0#10", Equal);
    test("-1", 1, 20, Nearest, "-0.12500000", "-0x0.200000#20", Equal);
    test(
        "-1",
        2,
        53,
        Down,
        "-0.25000000000000000",
        "-0x0.40000000000000#53",
        Equal,
    );
    test(
        "-1",
        1000000,
        53,
        Up,
        "-125000.00000000000",
        "-0x1e848.000000000#53",
        Equal,
    );
    test(
        "-1",
        18446744073709551615,
        53,
        Floor,
        "-2.3058430092136940e18",
        "-0x2.0000000000000E+15#53",
        Less,
    );
    test(
        "-1",
        3,
        53,
        Ceiling,
        "-0.37500000000000000",
        "-0x0.60000000000000#53",
        Equal,
    );
    test(
        "-1",
        65536,
        53,
        Nearest,
        "-8192.0000000000000",
        "-0x2000.0000000000#53",
        Equal,
    );
    test(
        "-1",
        100,
        100,
        Nearest,
        "-12.500000000000000000000000000000",
        "-0xc.800000000000000000000000#100",
        Equal,
    );
    test("1/2", 8, 1, Down, "0.50", "0x0.8#1", Less);
    test("1/2", 16, 1, Up, "2.0", "0x2.0#1", Greater);
    test("1/2", 1000, 1, Floor, "64.0", "0x4.0E+1#1", Less);
    test("1/2", 4, 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("1/2", 360, 1, Nearest, "32.0", "0x2.0E+1#1", Greater);
    test("1/2", 1, 5, Nearest, "0.0742", "0x0.13#5", Greater);
    test("1/2", 2, 10, Down, "0.14746", "0x0.25c#10", Less);
    test("1/2", 1000000, 10, Up, "73856.0", "0x1.208E+4#10", Greater);
    test(
        "1/2",
        18446744073709551615,
        10,
        Floor,
        "1.3601e18",
        "0x1.2e0E+15#10",
        Less,
    );
    test("1/2", 3, 10, Ceiling, "0.22144", "0x0.38b#10", Greater);
    test("1/2", 65536, 10, Nearest, "4840.0", "0x12e8.0#10", Greater);
    test(
        "1/2",
        100,
        20,
        Nearest,
        "7.3791809",
        "0x7.61120#20",
        Greater,
    );
    test(
        "1/2",
        8,
        53,
        Down,
        "0.59033447060173305",
        "0x0.972028ecef9840#53",
        Less,
    );
    test(
        "1/2",
        16,
        53,
        Up,
        "1.1806689412034663",
        "0x1.2e4051d9df309#53",
        Greater,
    );
    test(
        "1/2",
        1000,
        53,
        Floor,
        "73.791808825216634",
        "0x49.cab3fbb0fd58#53",
        Less,
    );
    test(
        "1/2",
        4,
        53,
        Ceiling,
        "0.29516723530086658",
        "0x0.4b90147677cc24#53",
        Greater,
    );
    test(
        "1/2",
        360,
        53,
        Nearest,
        "26.565051177077990",
        "0x1a.90a731a61dc4#53",
        Greater,
    );
    test(
        "1/2",
        1,
        100,
        Nearest,
        "0.073791808825216637087700538112404",
        "0x0.12e4051d9df308665688f6dae4#100",
        Greater,
    );
    test("1/3", 2, 1, Down, "0.062", "0x0.1#1", Less);
    test("1/3", 1000000, 1, Up, "6.6e4", "0x1.0E+4#1", Greater);
    test(
        "1/3",
        18446744073709551615,
        1,
        Floor,
        "5.8e17",
        "0x8.0E+14#1",
        Less,
    );
    test("1/3", 3, 1, Ceiling, "0.25", "0x0.4#1", Greater);
    test("1/3", 65536, 1, Nearest, "4.1e3", "0x1.0E+3#1", Greater);
    test("1/3", 100, 5, Nearest, "5.00", "0x5.0#5", Less);
    test("1/3", 8, 10, Down, "0.40918", "0x0.68c#10", Less);
    test("1/3", 16, 10, Up, "0.81934", "0x0.d1c#10", Greater);
    test("1/3", 1000, 10, Floor, "51.188", "0x33.3#10", Less);
    test("1/3", 4, 10, Ceiling, "0.20483", "0x0.347#10", Greater);
    test("1/3", 360, 10, Nearest, "18.438", "0x12.70#10", Greater);
    test(
        "1/3",
        1,
        20,
        Nearest,
        "0.051208198",
        "0x0.0d1bfb#20",
        Greater,
    );
    test(
        "1/3",
        2,
        53,
        Down,
        "0.10241638234956672",
        "0x0.1a37f5c4c419ef#53",
        Less,
    );
    test(
        "1/3",
        1000000,
        53,
        Up,
        "51208.191174783366",
        "0xc808.30f0d4a260#53",
        Greater,
    );
    test(
        "1/3",
        18446744073709551615,
        53,
        Floor,
        "9.4462439707882074e17",
        "0xd.1bfae2620cf78E+14#53",
        Less,
    );
    test(
        "1/3",
        3,
        53,
        Ceiling,
        "0.15362457352435011",
        "0x0.2753f0a72626e8#53",
        Greater,
    );
    test(
        "1/3",
        65536,
        53,
        Nearest,
        "3355.9800168306024",
        "0xd1b.fae2620cf78#53",
        Less,
    );
    test(
        "1/3",
        100,
        100,
        Nearest,
        "5.1208191174783362912299461887604",
        "0x5.1eee006e4d10b806327f927f0#100",
        Less,
    );
    test("-1/3", 8, 1, Down, "-0.25", "-0x0.4#1", Greater);
    test("-1/3", 16, 1, Up, "-1.0", "-0x1.0#1", Less);
    test("-1/3", 1000, 1, Floor, "-64.0", "-0x4.0E+1#1", Less);
    test("-1/3", 4, 1, Ceiling, "-0.12", "-0x0.2#1", Greater);
    test("-1/3", 360, 1, Nearest, "-16.0", "-0x1.0E+1#1", Greater);
    test("-1/3", 1, 5, Nearest, "-0.0508", "-0x0.0d0#5", Greater);
    test("-1/3", 2, 10, Down, "-0.10229", "-0x0.1a30#10", Greater);
    test("-1/3", 1000000, 10, Up, "-51264.0", "-0xc.84E+3#10", Less);
    test(
        "-1/3",
        18446744073709551615,
        10,
        Floor,
        "-9.4463e17",
        "-0xd.1cE+14#10",
        Less,
    );
    test("-1/3", 3, 10, Ceiling, "-0.15356", "-0x0.275#10", Greater);
    test("-1/3", 65536, 10, Nearest, "-3356.0", "-0xd1c.0#10", Less);
    test(
        "-1/3",
        100,
        20,
        Nearest,
        "-5.1208191",
        "-0x5.1eee0#20",
        Greater,
    );
    test(
        "-1/3",
        8,
        53,
        Down,
        "-0.40966552939826689",
        "-0x0.68dfd7131067bc#53",
        Greater,
    );
    test(
        "-1/3",
        16,
        53,
        Up,
        "-0.81933105879653390",
        "-0x0.d1bfae2620cf80#53",
        Less,
    );
    test(
        "-1/3",
        1000,
        53,
        Floor,
        "-51.208191174783366",
        "-0x33.354c044f02a8#53",
        Less,
    );
    test(
        "-1/3",
        4,
        53,
        Ceiling,
        "-0.20483276469913345",
        "-0x0.346feb898833de#53",
        Greater,
    );
    test(
        "-1/3",
        360,
        53,
        Nearest,
        "-18.434948822922010",
        "-0x12.6f58ce59e23c#53",
        Greater,
    );
    test(
        "-1/3",
        1,
        100,
        Nearest,
        "-0.051208191174783362912299461887645",
        "-0x0.0d1bfae2620cf799a97709251d#100",
        Less,
    );
    test("3/5", 2, 1, Down, "0.12", "0x0.2#1", Less);
    test("3/5", 1000000, 1, Up, "1.3e5", "0x2.0E+4#1", Greater);
    test(
        "3/5",
        18446744073709551615,
        1,
        Floor,
        "1.2e18",
        "0x1.0E+15#1",
        Less,
    );
    test("3/5", 3, 1, Ceiling, "0.50", "0x0.8#1", Greater);
    test("3/5", 65536, 1, Nearest, "4.1e3", "0x1.0E+3#1", Less);
    test("3/5", 100, 10, Down, "8.5938", "0x8.98#10", Less);
    test("3/5", 8, 10, Up, "0.68848", "0x0.b04#10", Greater);
    test("3/5", 16, 10, Floor, "1.3750", "0x1.600#10", Less);
    test("3/5", 1000, 10, Ceiling, "86.125", "0x56.2#10", Greater);
    test("3/5", 4, 10, Nearest, "0.34424", "0x0.582#10", Greater);
    test(
        "3/5",
        360,
        53,
        Down,
        "30.963756532073521",
        "0x1e.f6b8bf828fe9#53",
        Less,
    );
    test(
        "3/5",
        1,
        53,
        Up,
        "0.086010434811315345",
        "0x0.1604c7a4a11c62#53",
        Greater,
    );
    test(
        "3/5",
        2,
        53,
        Floor,
        "0.17202086962263066",
        "0x0.2c098f494238c2#53",
        Less,
    );
    test(
        "3/5",
        1000000,
        53,
        Ceiling,
        "86010.434811315339",
        "0x14ffa.6f4fcb5b5#53",
        Greater,
    );
    test(
        "3/5",
        18446744073709551615,
        53,
        Nearest,
        "1.5866124786328128e18",
        "0x1.604c7a4a11c61E+15#53",
        Less,
    );
    test(
        "3/5",
        3,
        100,
        Nearest,
        "0.25803130443394601180923066534047",
        "0x0.420e56ede3552456623424a1e8#100",
        Greater,
    );
    test("22/7", 65536, 1, Down, "8.2e3", "0x2.0E+3#1", Less);
    test("22/7", 100, 1, Up, "32.0", "0x2.0E+1#1", Greater);
    test("22/7", 8, 1, Floor, "1.0", "0x1.0#1", Less);
    test("22/7", 16, 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("22/7", 1000, 1, Nearest, "2.6e2", "0x1.0E+2#1", Greater);
    test("22/7", 4, 5, Nearest, "0.812", "0x0.d0#5", Greater);
    test("22/7", 360, 10, Down, "72.250", "0x48.4#10", Less);
    test("22/7", 1, 10, Up, "0.20117", "0x0.338#10", Greater);
    test("22/7", 2, 10, Floor, "0.40186", "0x0.66e#10", Less);
    test(
        "22/7",
        1000000,
        10,
        Ceiling,
        "2.0122e5",
        "0x3.12E+4#10",
        Greater,
    );
    test(
        "22/7",
        18446744073709551615,
        10,
        Nearest,
        "3.7065e18",
        "0x3.37E+15#10",
        Less,
    );
    test(
        "22/7",
        3,
        20,
        Nearest,
        "0.60291576",
        "0x0.9a58b#20",
        Greater,
    );
    test(
        "22/7",
        65536,
        53,
        Down,
        "13170.892942007387",
        "0x3372.e497d8eef4#53",
        Less,
    );
    test(
        "22/7",
        100,
        53,
        Up,
        "20.097187716686079",
        "0x14.18e14b50bd58#53",
        Greater,
    );
    test(
        "22/7",
        8,
        53,
        Floor,
        "1.6077750173348861",
        "0x1.9b9724bec777a#53",
        Less,
    );
    test(
        "22/7",
        16,
        53,
        Ceiling,
        "3.2155500346697727",
        "0x3.372e497d8eef6#53",
        Greater,
    );
    test(
        "22/7",
        1000,
        53,
        Nearest,
        "200.97187716686076",
        "0xc8.f8ccf1276568#53",
        Less,
    );
    test(
        "22/7",
        4,
        100,
        Nearest,
        "0.80388750866744308352812618774285",
        "0x0.cdcb925f63bbd0b5fe97cf3b0#100",
        Greater,
    );
    test("-22/7", 360, 1, Down, "-64.0", "-0x4.0E+1#1", Greater);
    test("-22/7", 1, 1, Up, "-0.25", "-0x0.4#1", Less);
    test("-22/7", 2, 1, Floor, "-0.50", "-0x0.8#1", Less);
    test(
        "-22/7",
        1000000,
        1,
        Ceiling,
        "-1.3e5",
        "-0x2.0E+4#1",
        Greater,
    );
    test(
        "-22/7",
        18446744073709551615,
        1,
        Nearest,
        "-4.6e18",
        "-0x4.0E+15#1",
        Less,
    );
    test("-22/7", 3, 5, Nearest, "-0.594", "-0x0.98#5", Greater);
    test(
        "-22/7",
        65536,
        10,
        Down,
        "-13168.0",
        "-0x3.37E+3#10",
        Greater,
    );
    test("-22/7", 100, 10, Up, "-20.125", "-0x14.20#10", Less);
    test("-22/7", 8, 10, Floor, "-1.6094", "-0x1.9c0#10", Less);
    test("-22/7", 16, 10, Ceiling, "-3.2148", "-0x3.37#10", Greater);
    test("-22/7", 1000, 10, Nearest, "-201.00", "-0xc9.0#10", Less);
    test(
        "-22/7",
        4,
        20,
        Nearest,
        "-0.80388737",
        "-0x0.cdcb9#20",
        Greater,
    );
    test(
        "-22/7",
        360,
        53,
        Down,
        "-72.349875780069866",
        "-0x48.599175891004#53",
        Greater,
    );
    test(
        "-22/7",
        1,
        53,
        Up,
        "-0.20097187716686080",
        "-0x0.3372e497d8eef6#53",
        Less,
    );
    test(
        "-22/7",
        2,
        53,
        Floor,
        "-0.40194375433372159",
        "-0x0.66e5c92fb1ddec#53",
        Less,
    );
    test(
        "-22/7",
        1000000,
        53,
        Ceiling,
        "-200971.87716686077",
        "-0x3110b.e08e01e42#53",
        Greater,
    );
    test(
        "-22/7",
        18446744073709551615,
        53,
        Nearest,
        "-3.7072767841100728e18",
        "-0x3.372e497d8eef4E+15#53",
        Greater,
    );
    test(
        "-22/7",
        3,
        100,
        Nearest,
        "-0.60291563150058231264609464080714",
        "-0x0.9a58adc78accdc887ef1db6c4#100",
        Less,
    );
    test("355/113", 65536, 1, Down, "8.2e3", "0x2.0E+3#1", Less);
    test("355/113", 100, 1, Up, "32.0", "0x2.0E+1#1", Greater);
    test("355/113", 8, 1, Floor, "1.0", "0x1.0#1", Less);
    test("355/113", 16, 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("355/113", 1000, 1, Nearest, "2.6e2", "0x1.0E+2#1", Greater);
    test("355/113", 4, 5, Nearest, "0.812", "0x0.d0#5", Greater);
    test("355/113", 360, 10, Down, "72.250", "0x48.4#10", Less);
    test("355/113", 1, 10, Up, "0.20117", "0x0.338#10", Greater);
    test("355/113", 2, 10, Floor, "0.40186", "0x0.66e#10", Less);
    test(
        "355/113",
        1000000,
        10,
        Ceiling,
        "2.0096e5",
        "0x3.11E+4#10",
        Greater,
    );
    test(
        "355/113",
        18446744073709551615,
        10,
        Nearest,
        "3.7065e18",
        "0x3.37E+15#10",
        Less,
    );
    test(
        "355/113",
        3,
        20,
        Nearest,
        "0.60286045",
        "0x0.9a551#20",
        Greater,
    );
    test(
        "355/113",
        65536,
        53,
        Down,
        "13169.680248331830",
        "0x3371.ae24c1325e#53",
        Less,
    );
    test(
        "355/113",
        100,
        53,
        Up,
        "20.095337292986802",
        "0x14.1868065b77ad#53",
        Greater,
    );
    test(
        "355/113",
        8,
        53,
        Floor,
        "1.6076269834389441",
        "0x1.9b8d71260992f#53",
        Less,
    );
    test(
        "355/113",
        16,
        53,
        Ceiling,
        "3.2152539668778886",
        "0x3.371ae24c13260#53",
        Greater,
    );
    test(
        "355/113",
        1000,
        53,
        Nearest,
        "200.95337292986801",
        "0xc8.f4103f92acc0#53",
        Greater,
    );
    test(
        "355/113",
        4,
        100,
        Nearest,
        "0.80381349171947202888886810365167",
        "0x0.cdc6b89304c978059b6f42e7e#100",
        Less,
    );
    test("3", 360, 1, Down, "64.0", "0x4.0E+1#1", Less);
    test("3", 1, 1, Up, "0.25", "0x0.4#1", Greater);
    test("3", 2, 1, Floor, "0.25", "0x0.4#1", Less);
    test("3", 1000000, 1, Ceiling, "2.6e5", "0x4.0E+4#1", Greater);
    test(
        "3",
        18446744073709551615,
        1,
        Nearest,
        "4.6e18",
        "0x4.0E+15#1",
        Greater,
    );
    test("3", 3, 5, Nearest, "0.594", "0x0.98#5", Less);
    test("3", 65536, 10, Down, "13024.0", "0x3.2eE+3#10", Less);
    test("3", 100, 10, Up, "19.906", "0x13.e8#10", Greater);
    test("3", 8, 10, Floor, "1.5898", "0x1.970#10", Less);
    test("3", 16, 10, Ceiling, "3.1836", "0x3.2f#10", Greater);
    test("3", 1000, 10, Nearest, "198.75", "0xc6.c#10", Less);
    test("3", 4, 20, Nearest, "0.79516697", "0x0.cb901#20", Less);
    test(
        "3",
        360,
        53,
        Down,
        "71.565051177077976",
        "0x47.90a731a61dc0#53",
        Less,
    );
    test(
        "3",
        1,
        53,
        Up,
        "0.19879180882521666",
        "0x0.32e4051d9df30a#53",
        Greater,
    );
    test(
        "3",
        2,
        53,
        Floor,
        "0.39758361765043326",
        "0x0.65c80a3b3be610#53",
        Less,
    );
    test(
        "3",
        1000000,
        53,
        Ceiling,
        "198791.80882521666",
        "0x30887.cf0f2b5dc#53",
        Greater,
    );
    test(
        "3",
        18446744073709551615,
        53,
        Nearest,
        "3.6670616213485670e18",
        "0x3.2e4051d9df308E+15#53",
        Less,
    );
    test(
        "3",
        3,
        100,
        Nearest,
        "0.59637542647564991126310161433741",
        "0x0.98ac0f58d9d91933039ae490b#100",
        Greater,
    );
    test("100", 65536, 1, Down, "8.2e3", "0x2.0E+3#1", Less);
    test("100", 100, 1, Up, "32.0", "0x2.0E+1#1", Greater);
    test("100", 8, 1, Floor, "1.0", "0x1.0#1", Less);
    test("100", 16, 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("100", 1000, 1, Nearest, "2.6e2", "0x1.0E+2#1", Greater);
    test("100", 4, 5, Nearest, "1.00", "0x1.0#5", Greater);
    test("100", 360, 10, Down, "89.375", "0x59.6#10", Less);
    test("100", 1, 10, Up, "0.24854", "0x0.3fa#10", Greater);
    test("100", 2, 10, Floor, "0.49658", "0x0.7f2#10", Less);
    test(
        "100",
        1000000,
        10,
        Ceiling,
        "2.4858e5",
        "0x3.cbE+4#10",
        Greater,
    );
    test(
        "100",
        18446744073709551615,
        10,
        Nearest,
        "4.5802e18",
        "0x3.f9E+15#10",
        Less,
    );
    test("100", 3, 20, Nearest, "0.74522591", "0x0.bec72#20", Greater);
    test(
        "100",
        65536,
        53,
        Down,
        "16279.699693079485",
        "0x3f97.b31f15eda0#53",
        Less,
    );
    test(
        "100",
        100,
        53,
        Up,
        "24.840850361754590",
        "0x18.d741f82490d4#53",
        Greater,
    );
    test(
        "100",
        8,
        53,
        Floor,
        "1.9872680289403668",
        "0x1.fcbd98f8af6d0#53",
        Less,
    );
    test(
        "100",
        16,
        53,
        Ceiling,
        "3.9745360578807341",
        "0x3.f97b31f15eda2#53",
        Greater,
    );
    test(
        "100",
        1000,
        53,
        Nearest,
        "248.40850361754588",
        "0xf8.6893b16da840#53",
        Greater,
    );
    test(
        "100",
        4,
        100,
        Nearest,
        "0.99363401447018348970176197245465",
        "0x0.fe5ecc7c57b686508eee13f7c#100",
        Less,
    );
    test("1000000", 360, 1, Down, "64.0", "0x4.0E+1#1", Less);
    test("1000000", 1, 1, Up, "0.25", "0x0.4#1", Greater);
    test("1000000", 2, 1, Floor, "0.25", "0x0.4#1", Less);
    test(
        "1000000",
        1000000,
        1,
        Ceiling,
        "2.6e5",
        "0x4.0E+4#1",
        Greater,
    );
    test(
        "1000000",
        18446744073709551615,
        1,
        Nearest,
        "4.6e18",
        "0x4.0E+15#1",
        Greater,
    );
    test("1000000", 3, 5, Nearest, "0.750", "0x0.c0#5", Greater);
    test("1000000", 65536, 10, Down, "16368.0", "0x3.ffE+3#10", Less);
    test("1000000", 100, 10, Up, "25.000", "0x19.00#10", Greater);
    test("1000000", 8, 10, Floor, "1.9980", "0x1.ff8#10", Less);
    test("1000000", 16, 10, Ceiling, "4.0000", "0x4.00#10", Greater);
    test("1000000", 1000, 10, Nearest, "250.00", "0xfa.0#10", Greater);
    test(
        "1000000",
        4,
        20,
        Nearest,
        "0.99999905",
        "0x0.fffff#20",
        Less,
    );
    test(
        "1000000",
        360,
        53,
        Down,
        "89.999942704220473",
        "0x59.fffc3ebc8030#53",
        Less,
    );
    test(
        "1000000",
        1,
        53,
        Up,
        "0.24999984084505691",
        "0x0.3ffffd546f4a1a#53",
        Greater,
    );
    test(
        "1000000",
        2,
        53,
        Floor,
        "0.49999968169011377",
        "0x0.7ffffaa8de9430#53",
        Less,
    );
    test(
        "1000000",
        1000000,
        53,
        Ceiling,
        "249999.84084505693",
        "0x3d08f.d7419f248#53",
        Greater,
    );
    test(
        "1000000",
        18446744073709551615,
        53,
        Nearest,
        "4.6116830825368847e18",
        "0x3.ffffd546f4a1aE+15#53",
        Greater,
    );
    test(
        "1000000",
        3,
        100,
        Nearest,
        "0.74999952253517072447314763644025",
        "0x0.bffff7fd4dde4caceea5618c7#100",
        Less,
    );
    test("1/1000000", 65536, 1, Down, "0.0078", "0x0.02#1", Less);
    test("1/1000000", 100, 1, Up, "0.000031", "0x0.0002#1", Greater);
    test("1/1000000", 8, 1, Floor, "9.5e-7", "0x0.00001#1", Less);
    test(
        "1/1000000",
        16,
        1,
        Ceiling,
        "3.8e-6",
        "0x0.00004#1",
        Greater,
    );
    test("1/1000000", 1000, 1, Nearest, "0.00012", "0x0.0008#1", Less);
    test("1/1000000", 4, 5, Nearest, "6.26e-7", "0xa.8E-6#5", Less);
    test(
        "1/1000000",
        360,
        10,
        Down,
        "0.000057280",
        "0x0.0003c1#10",
        Less,
    );
    test("1/1000000", 1, 10, Up, "1.5926e-7", "0x2.acE-6#10", Greater);
    test("1/1000000", 2, 10, Floor, "3.1805e-7", "0x5.56E-6#10", Less);
    test(
        "1/1000000",
        1000000,
        10,
        Ceiling,
        "0.15918",
        "0x0.28c#10",
        Greater,
    );
    test(
        "1/1000000",
        18446744073709551615,
        10,
        Nearest,
        "2.9378e12",
        "0x2.acE+10#10",
        Greater,
    );
    test(
        "1/1000000",
        3,
        20,
        Nearest,
        "4.7746471e-7",
        "0x8.02b2E-6#20",
        Less,
    );
    test(
        "1/1000000",
        65536,
        53,
        Down,
        "0.010430378350466975",
        "0x0.02ab90b5e67105c#53",
        Less,
    );
    test(
        "1/1000000",
        100,
        53,
        Up,
        "0.000015915494309184231",
        "0x0.00010b04870e04265#53",
        Greater,
    );
    test(
        "1/1000000",
        8,
        53,
        Floor,
        "1.2732395447347382e-6",
        "0x0.0000155c85af33882e#53",
        Less,
    );
    test(
        "1/1000000",
        16,
        53,
        Ceiling,
        "2.5464790894694769e-6",
        "0x0.00002ab90b5e67105e#53",
        Greater,
    );
    test(
        "1/1000000",
        1000,
        53,
        Nearest,
        "0.00015915494309184228",
        "0x0.000a6e2d468c297e8#53",
        Less,
    );
    test(
        "1/1000000",
        4,
        100,
        Nearest,
        "6.3661977236736913648474598703324e-7",
        "0xa.ae42d799c417237def63b99eE-6#100",
        Greater,
    );
    test(
        "-1/1000000",
        360,
        1,
        Down,
        "-0.000031",
        "-0x0.0002#1",
        Greater,
    );
    test("-1/1000000", 1, 1, Up, "-2.4e-7", "-0x4.0E-6#1", Less);
    test("-1/1000000", 2, 1, Floor, "-4.8e-7", "-0x8.0E-6#1", Less);
    test(
        "-1/1000000",
        1000000,
        1,
        Ceiling,
        "-0.12",
        "-0x0.2#1",
        Greater,
    );
    test(
        "-1/1000000",
        18446744073709551615,
        1,
        Nearest,
        "-2.2e12",
        "-0x2.0E+10#1",
        Greater,
    );
    test(
        "-1/1000000",
        3,
        5,
        Nearest,
        "-4.77e-7",
        "-0x8.0E-6#5",
        Greater,
    );
    test(
        "-1/1000000",
        65536,
        10,
        Down,
        "-0.010422",
        "-0x0.02ab#10",
        Greater,
    );
    test(
        "-1/1000000",
        100,
        10,
        Up,
        "-0.000015944",
        "-0x0.00010b8#10",
        Less,
    );
    test(
        "-1/1000000",
        8,
        10,
        Floor,
        "-1.2740e-6",
        "-0x0.00001560#10",
        Less,
    );
    test(
        "-1/1000000",
        16,
        10,
        Ceiling,
        "-2.5444e-6",
        "-0x0.00002ab#10",
        Greater,
    );
    test(
        "-1/1000000",
        1000,
        10,
        Nearest,
        "-0.00015926",
        "-0x0.000a70#10",
        Less,
    );
    test(
        "-1/1000000",
        4,
        20,
        Nearest,
        "-6.3661992e-7",
        "-0xa.ae43E-6#20",
        Less,
    );
    test(
        "-1/1000000",
        360,
        53,
        Down,
        "-0.000057295779513063222",
        "-0x0.0003c1437fcc0ef02#53",
        Greater,
    );
    test(
        "-1/1000000",
        1,
        53,
        Up,
        "-1.5915494309184230e-7",
        "-0x2.ab90b5e67105eE-6#53",
        Less,
    );
    test(
        "-1/1000000",
        2,
        53,
        Floor,
        "-3.1830988618368461e-7",
        "-0x5.57216bcce20bcE-6#53",
        Less,
    );
    test(
        "-1/1000000",
        1000000,
        53,
        Ceiling,
        "-0.15915494309184228",
        "-0x0.28be60db938216#53",
        Greater,
    );
    test(
        "-1/1000000",
        18446744073709551615,
        53,
        Nearest,
        "-2935890503281.0225",
        "-0x2ab90b5e671.05c#53",
        Greater,
    );
    test(
        "-1/1000000",
        3,
        100,
        Nearest,
        "-4.7746482927552685236355949027456e-7",
        "-0x8.02b221b353115a9e738acb36E-6#100",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        65536,
        1,
        Down,
        "6.8e-21",
        "0x2.0E-17#1",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        100,
        1,
        Up,
        "2.6e-23",
        "0x2.0E-19#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        8,
        1,
        Floor,
        "8.3e-25",
        "0x1.0E-20#1",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        16,
        1,
        Ceiling,
        "3.3e-24",
        "0x4.0E-20#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1000,
        1,
        Nearest,
        "2.1e-22",
        "0x1.0E-18#1",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        4,
        5,
        Nearest,
        "6.46e-25",
        "0xc.8E-21#5",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        360,
        10,
        Down,
        "5.7282e-23",
        "0x4.54E-19#10",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1,
        10,
        Up,
        "1.5934e-25",
        "0x3.15E-21#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        2,
        10,
        Floor,
        "3.1827e-25",
        "0x6.28E-21#10",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        1000000,
        10,
        Ceiling,
        "1.5924e-19",
        "0x2.f0E-16#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        18446744073709551615,
        10,
        Nearest,
        "2.9355e-6",
        "0x0.0000314#10",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        3,
        20,
        Nearest,
        "4.7746516e-25",
        "0x9.3c4bE-21#20",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        65536,
        53,
        Down,
        "1.0430378350470452e-20",
        "0x3.1418dbf094d68E-17#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        100,
        53,
        Up,
        "1.5915494309189534e-23",
        "0x1.33d9b5e9fa23dE-19#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        8,
        53,
        Floor,
        "1.2732395447351626e-24",
        "0x1.8a0c6df84a6b4E-20#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        16,
        53,
        Ceiling,
        "2.5464790894703256e-24",
        "0x3.1418dbf094d6aE-20#53",
        Greater,
    );
    test(
        "1/1000000000000000000000000",
        1000,
        53,
        Nearest,
        "1.5915494309189533e-22",
        "0xc.06811b23c5660E-19#53",
        Less,
    );
    test(
        "1/1000000000000000000000000",
        4,
        100,
        Nearest,
        "6.3661977236758134307553505349007e-25",
        "0xc.50636fc2535a2d70b27527b2E-21#100",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        360,
        1,
        Down,
        "32.0",
        "0x2.0E+1#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        1,
        Up,
        "0.25",
        "0x0.4#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        2,
        1,
        Floor,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1000000,
        1,
        Ceiling,
        "2.6e5",
        "0x4.0E+4#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        18446744073709551615,
        1,
        Nearest,
        "2.3e18",
        "0x2.0E+15#1",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        3,
        5,
        Nearest,
        "0.484",
        "0x0.7c#5",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        65536,
        10,
        Down,
        "10464.0",
        "0x2.8eE+3#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        100,
        10,
        Up,
        "15.984",
        "0xf.fc#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        8,
        10,
        Floor,
        "1.2773",
        "0x1.470#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        16,
        10,
        Ceiling,
        "2.5586",
        "0x2.8f#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1000,
        10,
        Nearest,
        "159.75",
        "0x9f.c#10",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        4,
        20,
        Nearest,
        "0.63909340",
        "0x0.a39ba#20",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        360,
        53,
        Down,
        "57.518363409470240",
        "0x39.84b376e31e24#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1,
        53,
        Up,
        "0.15977323169297292",
        "0x0.28e6e604e5c5cc#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        2,
        53,
        Floor,
        "0.31954646338594578",
        "0x0.51cdcc09cb8b94#53",
        Less,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        1000000,
        53,
        Ceiling,
        "159773.23169297291",
        "0x2701d.3b503b0d6#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        18446744073709551615,
        53,
        Nearest,
        "2.9472959148697713e18",
        "0x2.8e6e604e5c5ccE+15#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/79228162514264337593543950336",
        3,
        100,
        Nearest,
        "0.47931969507891872002131069872708",
        "0x0.7ab4b20eb15161ce06967a02b8#100",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        65536,
        1,
        Down,
        "8.2e3",
        "0x2.0E+3#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        100,
        1,
        Up,
        "32.0",
        "0x2.0E+1#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        8,
        1,
        Floor,
        "1.0",
        "0x1.0#1",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        16,
        1,
        Ceiling,
        "4.0",
        "0x4.0#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1000,
        1,
        Nearest,
        "2.6e2",
        "0x1.0E+2#1",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        4,
        5,
        Nearest,
        "0.875",
        "0x0.e0#5",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        360,
        10,
        Down,
        "78.000",
        "0x4e.0#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1,
        10,
        Up,
        "0.21680",
        "0x0.378#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        2,
        10,
        Floor,
        "0.43311",
        "0x0.6ee#10",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1000000,
        10,
        Ceiling,
        "2.1683e5",
        "0x3.4fE+4#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        18446744073709551615,
        10,
        Nearest,
        "3.9992e18",
        "0x3.78E+15#10",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        3,
        20,
        Nearest,
        "0.65015984",
        "0x0.a670e#20",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        65536,
        53,
        Down,
        "14202.959450078099",
        "0x377a.f59e853394#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        100,
        53,
        Up,
        "21.671996231198275",
        "0x15.ac07f1ec0827#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        8,
        53,
        Floor,
        "1.7337596984958616",
        "0x1.bbd7acf4299ca#53",
        Less,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        16,
        53,
        Ceiling,
        "3.4675193969917237",
        "0x3.77af59e853396#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        1000,
        53,
        Nearest,
        "216.71996231198273",
        "0xd8.b84f73385180#53",
        Greater,
    );
    test(
        "373353919968346627845782916933/79228162514264337593543950336",
        4,
        100,
        Nearest,
        "0.86687984924793087252390012285258",
        "0x0.ddebd67a14ce539827da9226d#100",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        360,
        1,
        Down,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        1,
        Up,
        "0.25",
        "0x0.4#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        2,
        1,
        Floor,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1000000,
        1,
        Ceiling,
        "2.6e5",
        "0x4.0E+4#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        18446744073709551615,
        1,
        Nearest,
        "4.6e18",
        "0x4.0E+15#1",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        3,
        5,
        Nearest,
        "0.594",
        "0x0.98#5",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        65536,
        10,
        Down,
        "13168.0",
        "0x3.37E+3#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        100,
        10,
        Up,
        "20.125",
        "0x14.20#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        8,
        10,
        Floor,
        "1.6074",
        "0x1.9b8#10",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        16,
        10,
        Ceiling,
        "3.2188",
        "0x3.38#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1000,
        10,
        Nearest,
        "201.00",
        "0xc9.0#10",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        4,
        20,
        Nearest,
        "0.80381393",
        "0x0.cdc6c#20",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        360,
        53,
        Down,
        "72.343212848587129",
        "0x48.57dccc183fac#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1,
        53,
        Up,
        "0.20095336902385319",
        "0x0.3371ae13fa7cf0#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        2,
        53,
        Floor,
        "0.40190673804770632",
        "0x0.66e35c27f4f9dc#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        1000000,
        53,
        Ceiling,
        "200953.36902385319",
        "0x310f9.5e7858e4e#53",
        Greater,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        18446744073709551615,
        53,
        Nearest,
        "3.7069353691327319e18",
        "0x3.371ae13fa7ceeE+15#53",
        Less,
    );
    test(
        "124451306656115542615260972311/39614081257132168796771975168",
        3,
        100,
        Nearest,
        "0.60286010707155951267651888767553",
        "0x0.9a550a3bef76cbde86e2ffdeb#100",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        65536,
        1,
        Down,
        "-8.2e3",
        "-0x2.0E+3#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        100,
        1,
        Up,
        "-16.0",
        "-0x1.0E+1#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        8,
        1,
        Floor,
        "-2.0",
        "-0x2.0#1",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        16,
        1,
        Ceiling,
        "-2.0",
        "-0x2.0#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1000,
        1,
        Nearest,
        "-1.3e2",
        "-0x8.0E+1#1",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        4,
        5,
        Nearest,
        "-0.625",
        "-0x0.a0#5",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        360,
        10,
        Down,
        "-57.500",
        "-0x39.8#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1,
        10,
        Up,
        "-0.15991",
        "-0x0.28f#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        2,
        10,
        Floor,
        "-0.31982",
        "-0x0.51e#10",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1000000,
        10,
        Ceiling,
        "-1.5974e5",
        "-0x2.70E+4#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        18446744073709551615,
        10,
        Nearest,
        "-2.9454e18",
        "-0x2.8eE+15#10",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        3,
        20,
        Nearest,
        "-0.47931957",
        "-0x0.7ab4b0#20",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        65536,
        53,
        Down,
        "-10470.898512230671",
        "-0x28e6.e604e5c5ca#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        100,
        53,
        Up,
        "-15.977323169297291",
        "-0xf.fa31d9e9c1438#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        8,
        53,
        Floor,
        "-1.2781858535437833",
        "-0x1.473730272e2e6#53",
        Less,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        16,
        53,
        Ceiling,
        "-2.5563717070875662",
        "-0x2.8e6e604e5c5ca#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        1000,
        53,
        Nearest,
        "-159.77323169297290",
        "-0x9f.c5f283218ca0#53",
        Greater,
    );
    test(
        "-124451306656115542615260972311/79228162514264337593543950336",
        4,
        100,
        Nearest,
        "-0.63909292677189162669508093163663",
        "-0x0.a39b981397172d12b3734d590#100",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        360,
        1,
        Down,
        "64.0",
        "0x4.0E+1#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        1,
        Up,
        "0.25",
        "0x0.4#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        2,
        1,
        Floor,
        "0.25",
        "0x0.4#1",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1000000,
        1,
        Ceiling,
        "2.6e5",
        "0x4.0E+4#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        18446744073709551615,
        1,
        Nearest,
        "4.6e18",
        "0x4.0E+15#1",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        3,
        5,
        Nearest,
        "0.750",
        "0x0.c0#5",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        65536,
        10,
        Down,
        "16368.0",
        "0x3.ffE+3#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        100,
        10,
        Up,
        "25.000",
        "0x19.00#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        8,
        10,
        Floor,
        "1.9980",
        "0x1.ff8#10",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        16,
        10,
        Ceiling,
        "4.0000",
        "0x4.00#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        1000,
        10,
        Nearest,
        "250.00",
        "0xfa.0#10",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        4,
        20,
        Nearest,
        "1.0000000",
        "0x1.00000#20",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        360,
        53,
        Down,
        "89.999999999999986",
        "0x59.fffffffffffc#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1,
        53,
        Up,
        "0.25000000000000000",
        "0x0.40000000000000#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        2,
        53,
        Floor,
        "0.49999999999999994",
        "0x0.7ffffffffffffc#53",
        Less,
    );
    test(
        "1267650600228229401496703205376",
        1000000,
        53,
        Ceiling,
        "250000.00000000000",
        "0x3d090.000000000#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        18446744073709551615,
        53,
        Nearest,
        "4.6116860184273879e18",
        "0x4.0000000000000E+15#53",
        Greater,
    );
    test(
        "1267650600228229401496703205376",
        3,
        100,
        Nearest,
        "0.75000000000000000000000000000000",
        "0x0.c000000000000000000000000#100",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        65536,
        1,
        Down,
        "6.5e-27",
        "0x2.0E-22#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        100,
        1,
        Up,
        "1.3e-29",
        "0x1.0E-24#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        8,
        1,
        Floor,
        "7.9e-31",
        "0x1.0E-25#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        16,
        1,
        Ceiling,
        "3.2e-30",
        "0x4.0E-25#1",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1000,
        1,
        Nearest,
        "1.0e-28",
        "0x8.0E-24#1",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        4,
        5,
        Nearest,
        "4.93e-31",
        "0xa.0E-26#5",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        360,
        10,
        Down,
        "4.5162e-29",
        "0x3.94E-24#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1,
        10,
        Up,
        "1.2557e-31",
        "0x2.8cE-26#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        2,
        10,
        Floor,
        "2.5076e-31",
        "0x5.16E-26#10",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        1000000,
        10,
        Ceiling,
        "1.2561e-25",
        "0x2.6eE-21#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        18446744073709551615,
        10,
        Nearest,
        "2.3164e-12",
        "0x2.8cE-10#10",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        3,
        20,
        Nearest,
        "3.7665322e-31",
        "0x7.a3b20E-26#20",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        65536,
        53,
        Down,
        "8.2281177073497643e-27",
        "0x2.8be60db939104E-22#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        100,
        53,
        Up,
        "1.2555111247787119e-29",
        "0xf.ea5dd5c5a4a68E-25#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        8,
        53,
        Floor,
        "1.0044088998229693e-30",
        "0x1.45f306dc9c882E-25#53",
        Less,
    );
    test(
        "1/1267650600228229401496703205376",
        16,
        53,
        Ceiling,
        "2.0088177996459389e-30",
        "0x2.8be60db939106E-25#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        1000,
        53,
        Nearest,
        "1.2555111247787118e-28",
        "0x9.f27aa59b86e80E-24#53",
        Greater,
    );
    test(
        "1/1267650600228229401496703205376",
        4,
        100,
        Nearest,
        "5.0220444991148469621144488728019e-31",
        "0xa.2f9836e4e441529fc2757d1fE-26#100",
        Less,
    );
    test("1", 360, 10, Exact, "45.000", "0x2d.0#10", Equal);
    test("-1", 360, 10, Exact, "-45.000", "-0x2d.0#10", Equal);
    test("1", 8, 1, Exact, "1.0", "0x1.0#1", Equal);
    test("-1", 8, 1, Exact, "-1.0", "-0x1.0#1", Equal);
    test("1", 3, 2, Exact, "0.38", "0x0.6#2", Equal);
    test("1", 7, 3, Exact, "0.88", "0x0.e#3", Equal);
    test("1", 360, 1, Nearest, "32.0", "0x2.0E+1#1", Less);
    test("1", 360, 1, Floor, "32.0", "0x2.0E+1#1", Less);
    test("1", 360, 1, Ceiling, "64.0", "0x4.0E+1#1", Greater);
    test("1", 360, 5, Nearest, "44.0", "0x2c.0#5", Less);
    test("-1", 360, 5, Nearest, "-44.0", "-0x2c.0#5", Greater);
    test("0", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test("0", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("3/5", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-3/5", 0, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("3/5", 0, 1, Nearest, "0.0", "0x0.0", Equal);
    test("-3/5", 0, 1, Nearest, "-0.0", "-0x0.0", Equal);
    test("1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1", 0, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("123", 0, 10, Nearest, "0.0", "0x0.0", Equal);
    test(
        "1",
        18446744073709551615,
        64,
        Exact,
        "2305843009213693951.88",
        "0x1fffffffffffffff.e#64",
        Equal,
    );
    test(
        "1",
        18446744073709551615,
        62,
        Nearest,
        "2305843009213693952.0",
        "0x2000000000000000.0#62",
        Greater,
    );
    test(
        "-1",
        18446744073709551615,
        62,
        Nearest,
        "-2305843009213693952.0",
        "-0x2000000000000000.0#62",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        360,
        10,
        Nearest,
        "90.000",
        "0x5a.0#10",
        Greater,
    );
    test(
        "-1000000000000000000000000000000",
        360,
        10,
        Nearest,
        "-90.000",
        "-0x5a.0#10",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        360,
        10,
        Floor,
        "89.875",
        "0x59.e#10",
        Less,
    );
    test(
        "1000000000000000000000000000000",
        360,
        10,
        Ceiling,
        "90.000",
        "0x5a.0#10",
        Greater,
    );
    test(
        "1000000000000000000000000000000",
        1,
        10,
        Nearest,
        "0.25000",
        "0x0.400#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        360,
        10,
        Nearest,
        "5.7291e-29",
        "0x4.8aE-24#10",
        Less,
    );
    test(
        "-1/1000000000000000000000000000000",
        360,
        10,
        Nearest,
        "-5.7291e-29",
        "-0x4.8aE-24#10",
        Greater,
    );
    test(
        "1/1000000000000000000000000000000",
        1,
        10,
        Nearest,
        "1.5908e-31",
        "0x3.3aE-26#10",
        Less,
    );
    test(
        "1/1000000000000000000000000000000",
        18446744073709551615,
        10,
        Nearest,
        "2.9345e-12",
        "0x3.3aE-10#10",
        Less,
    );
    test(
        "22/7",
        360,
        53,
        Nearest,
        "72.349875780069880",
        "0x48.599175891008#53",
        Greater,
    );
    test(
        "-22/7",
        360,
        53,
        Nearest,
        "-72.349875780069880",
        "-0x48.599175891008#53",
        Less,
    );
    test(
        "355/113",
        1000000,
        100,
        Nearest,
        "200953.37292986800722221702591299",
        "0x310f9.5f7854f2cae8e3bf9da9c#100",
        Greater,
    );
    test("1/3", 4, 10, Nearest, "0.20483", "0x0.347#10", Greater);
    test("1/3", 4, 10, Floor, "0.20459", "0x0.346#10", Less);
    test("1/3", 4, 10, Ceiling, "0.20483", "0x0.347#10", Greater);
    test("1/3", 4, 10, Down, "0.20459", "0x0.346#10", Less);
    test("1/3", 4, 10, Up, "0.20483", "0x0.347#10", Greater);
}

#[test]
#[should_panic]
fn atan_with_period_rational_prec_round_fail_1() {
    Float::atan_with_period_rational_prec_round(Rational::ONE, 360, 0, Floor);
}

#[test]
#[should_panic]
fn atan_with_period_rational_prec_round_fail_2() {
    Float::atan_with_period_rational_prec_round_ref(&Rational::ONE, 360, 0, Floor);
}

#[test]
#[should_panic]
fn atan_with_period_rational_prec_round_fail_3() {
    // 3/5 is not an eighth of a turn, so no precision makes the result exact
    Float::atan_with_period_rational_prec_round(Rational::from_unsigneds(3u8, 5), 360, 10, Exact);
}

#[test]
#[should_panic]
fn atan_with_period_rational_prec_fail() {
    Float::atan_with_period_rational_prec(Rational::ONE, 360, 0);
}

#[test]
#[should_panic]
fn atan_with_period_rational_prec_ref_fail() {
    Float::atan_with_period_rational_prec_ref(&Rational::ONE, 360, 0);
}

// Whether atanu(x, u) is exactly representable at `prec`: only at zero, at u = 0, and at |x| = 1,
// where the result is an eighth of a turn and `prec` must be wide enough to hold it.
fn atan_with_period_rational_exact(x: &Rational, u: u64, prec: u64) -> bool {
    *x == 0u32 || u == 0 || (x.eq_abs(&1u32) && Float::from_unsigned_prec(u, prec).1 == Equal)
}

#[allow(clippy::needless_pass_by_value)]
fn atan_with_period_rational_prec_round_properties_helper(
    x: Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) {
    if rm == Exact && !atan_with_period_rational_exact(&x, u, prec) {
        assert_panic!(Float::atan_with_period_rational_prec_round_ref(
            &x, u, prec, Exact
        ));
        return;
    }
    let (t, o) = Float::atan_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(t.is_valid());
    assert_rounding_ordering_consistent(&t, rm, o);

    let (t_alt, o_alt) = Float::atan_with_period_rational_prec_round_ref(&x, u, prec, rm);
    assert!(t_alt.is_valid());
    assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && u <= u64::from(u32::MAX)
    {
        let (rug_t, rug_o) = rug_atan_with_period_rational_prec_round(&x, u, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_t)),
            ComparableFloatRef(&t)
        );
        assert_eq!(rug_o, o, "x = {x} u = {u} prec = {prec} rm = {rm:?}");
    }

    // |atanu(x, u)| <= u/4, a quarter turn, so the result never overflows
    assert!(t.is_finite());
    assert!(PartialOrdAbs::le_abs(
        &t,
        &Float::from_unsigned_prec(u, prec + 2).0
    ));
    if t.is_normal() {
        assert_eq!(t.get_prec(), Some(prec));
    }
    // atan_with_period is odd (a `Rational` has no negative zero, so x = 0 is excluded)
    if x != 0u32 {
        let (t_neg, o_neg) = Float::atan_with_period_rational_prec_round(-&x, u, prec, -rm);
        assert_eq!(ComparableFloatRef(&t_neg), ComparableFloatRef(&-&t));
        assert_eq!(o_neg, o.reverse());
    }
    // a `Float` input agrees with the `Float` version
    if let Ok(f) = Float::try_from(&x) {
        let (t_alt, o_alt) = f.atan_with_period_prec_round(u, prec, rm);
        assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
        assert_eq!(o_alt, o);
    }

    if o == Equal {
        assert!(atan_with_period_rational_exact(&x, u, prec));
        for rm in exhaustive_rounding_modes() {
            let (t2, oo) = Float::atan_with_period_rational_prec_round_ref(&x, u, prec, rm);
            assert_eq!(ComparableFloatRef(&t2), ComparableFloatRef(&t));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::atan_with_period_rational_prec_round_ref(
            &x, u, prec, Exact
        ));
    }
}

#[test]
fn atan_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, u, prec, rm)| {
            atan_with_period_rational_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // atanu(0, u) = 0, exactly (a `Rational` zero has no sign, so the result is positive)
        let (t, o) = Float::atan_with_period_rational_prec_round(Rational::ZERO, 360, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        // atanu(x, 0) = ±0.0 with the sign of x, exactly, which keeps the function odd
        let (t, o) = Float::atan_with_period_rational_prec_round(Rational::ONE, 0, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (t, o) = Float::atan_with_period_rational_prec_round(-Rational::ONE, 0, prec, rm);
        assert_eq!(ComparableFloat(t), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
        // atanu(±1, u) = ±u/8, an eighth of a turn, exact when `prec` holds it
        for (x, neg) in [(Rational::ONE, false), (-Rational::ONE, true)] {
            let (q, o_q) = Float::from_unsigned_prec_round(8u32, prec, if neg { -rm } else { rm });
            let (t, o) = Float::atan_with_period_rational_prec_round(x, 8, prec, rm);
            let q = q >> 3u32;
            assert_eq!(
                ComparableFloat(t),
                ComparableFloat(if neg { -q } else { q })
            );
            assert_eq!(o, if neg { o_q.reverse() } else { o_q });
        }
    });
}

#[test]
fn atan_with_period_rational_prec_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_5().test_properties(
        |(x, u, prec, _)| {
            let (t, o) = Float::atan_with_period_rational_prec(x.clone(), u, prec);
            assert!(t.is_valid());
            assert_rounding_ordering_consistent(&t, Nearest, o);
            let (t_alt, o_alt) = Float::atan_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            let (t_alt, o_alt) =
                Float::atan_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
            assert_eq!(ComparableFloatRef(&t_alt), ComparableFloatRef(&t));
            assert_eq!(o_alt, o);
            if u <= u64::from(u32::MAX) {
                let (rug_t, rug_o) = rug_atan_with_period_rational_prec(&x, u, prec);
                assert_eq!(
                    ComparableFloatRef(&Float::from(&rug_t)),
                    ComparableFloatRef(&t)
                );
                assert_eq!(rug_o, o);
            }
        },
    );
}

// A `Rational` below the bottom of the exponent range is not a `Float` at all, so the arctangent's
// own underflow shortcut cannot be used: a large enough u lifts the quotient back into the range,
// and the linear regime makes the answer agree exactly with one taken well inside it.
#[test]
fn test_atan_with_period_rational_underflow() {
    let k = u64::power_of_2(30) - 1000;
    for m in
        [Rational::ONE, Rational::from_unsigneds(3u8, 5), Rational::from_unsigneds(123456789u64, 7)]
    {
        let small = m >> 1000u32;
        let big = &small >> k;
        for (small, big) in [(small.clone(), big.clone()), (-small, -big)] {
            for u in [1u64 << 20, 1u64 << 40, u64::MAX] {
                for rm in exhaustive_rounding_modes() {
                    if rm == Exact {
                        continue;
                    }
                    let (t, o) = Float::atan_with_period_rational_prec_round_ref(&big, u, 53, rm);
                    let (t_alt, o_alt) =
                        Float::atan_with_period_rational_prec_round_ref(&small, u, 53, rm);
                    assert_eq!(
                        ComparableFloatRef(&t),
                        ComparableFloatRef(&(t_alt >> k)),
                        "u = {u} rm = {rm:?}"
                    );
                    assert_eq!(o, o_alt, "u = {u} rm = {rm:?}");
                }
            }
            // with a small u the quotient can fall below the bottom of the range, and the rounding
            // mode alone decides; the larger m keeps it inside, so round away first to see which
            // case this is
            let min_positive = Float::min_positive_value_prec(53);
            let positive = big > 0u32;
            let (away_t, _) = Float::atan_with_period_rational_prec_round_ref(&big, 1, 53, Up);
            if !away_t.eq_abs(&min_positive) {
                continue;
            }
            for rm in exhaustive_rounding_modes() {
                if rm == Exact {
                    continue;
                }
                let (t, o) = Float::atan_with_period_rational_prec_round_ref(&big, 1, 53, rm);
                let away = match rm {
                    Up => true,
                    Ceiling => positive,
                    Floor => !positive,
                    _ => false,
                };
                let expected = match (positive, away) {
                    (true, true) => min_positive.clone(),
                    (true, false) => Float::ZERO,
                    (false, true) => -min_positive.clone(),
                    (false, false) => Float::NEGATIVE_ZERO,
                };
                assert_eq!(
                    ComparableFloatRef(&t),
                    ComparableFloatRef(&expected),
                    "rm = {rm:?}"
                );
                assert_eq!(o, if away == positive { Greater } else { Less });
            }
        }
    }
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_atan_with_period_rational() {
    fn test<T: PrimitiveFloat>(s: &str, u: u64, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_atan_with_period_rational::<T>(&x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0, 0.0);
    test::<f64>("0", 0, 0.0);
    test::<f32>("0", 360, 0.0);
    test::<f64>("0", 360, 0.0);
    test::<f32>("1", 0, 0.0);
    test::<f64>("1", 0, 0.0);
    test::<f32>("-1", 0, -0.0);
    test::<f64>("-1", 0, -0.0);
    test::<f32>("1", 360, 45.0);
    test::<f64>("1", 360, 45.0);
    test::<f32>("-1", 360, -45.0);
    test::<f64>("-1", 360, -45.0);
    test::<f32>("1", 7, 0.875);
    test::<f64>("1", 7, 0.875);
    test::<f32>("-1", 7, -0.875);
    test::<f64>("-1", 7, -0.875);
    test::<f32>("2", 7, 1.2334573);
    test::<f64>("2", 7, 1.2334573382234835);
    test::<f32>("3/5", 360, 30.963757);
    test::<f64>("3/5", 360, 30.96375653207352);
    test::<f32>("-3/5", 360, -30.963757);
    test::<f64>("-3/5", 360, -30.96375653207352);
    test::<f32>("1/3", 360, 18.434948);
    test::<f64>("1/3", 360, 18.43494882292201);
    test::<f32>("90", 360, 89.3634);
    test::<f64>("90", 360, 89.36340642403651);
    test::<f32>("-90", 360, -89.3634);
    test::<f64>("-90", 360, -89.36340642403651);
    test::<f32>("100", 360, 89.42706);
    test::<f64>("100", 360, 89.42706130231652);
    test::<f32>("10000000000", 360, 90.0);
    test::<f64>("10000000000", 360, 89.99999999427042);
    test::<f32>("1000000000000000000000000000000", 7, 1.75);
    test::<f64>("1000000000000000000000000000000", 7, 1.75);
    test::<f32>("1/1000000000000000000000000000000", 7, 1.1140846e-30);
    test::<f64>(
        "1/1000000000000000000000000000000",
        7,
        1.1140846016432673e-30,
    );
    test::<f32>(
        "1/1000000000000000000000000000000",
        18446744073709551615,
        2.9358905e-12,
    );
    test::<f64>(
        "1/1000000000000000000000000000000",
        18446744073709551615,
        2.9358905032820013e-12,
    );
    test::<f32>("1/2", 1, 0.07379181);
    test::<f64>("1/2", 1, 0.07379180882521663);
    test::<f32>("1/4", 1, 0.038989566);
    test::<f64>("1/4", 1, 0.03898956518868466);
    test::<f32>("1/10", 1, 0.01586276);
    test::<f64>("1/10", 1, 0.015862758715276783);
    test::<f32>("22/7", 360, 72.34988);
    test::<f64>("22/7", 360, 72.34987578006988);
    test::<f32>("355/113", 1000000, 200953.38);
    test::<f64>("355/113", 1000000, 200953.372929868);
    test::<f32>("-22/7", 360, -72.34988);
    test::<f64>("-22/7", 360, -72.34987578006988);
    test::<f32>("123456789/7", 4, 0.99999994);
    test::<f64>("123456789/7", 4, 0.9999999639036585);
    test::<f32>("7/123456789", 4, 3.6096342e-8);
    test::<f64>("7/123456789", 4, 3.609634142171853e-8);
    test::<f32>("1", 18446744073709551615, 2.305843e18);
    test::<f64>("1", 18446744073709551615, 2.305843009213694e18);
    test::<f32>("-1", 18446744073709551615, -2.305843e18);
    test::<f64>("-1", 18446744073709551615, -2.305843009213694e18);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_atan_with_period_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, u)| {
        let t = primitive_float_atan_with_period_rational::<T>(&x, u);
        // never NaN, and never overflowing, since the result is at most a quarter turn
        assert!(!t.is_nan());
        assert!(t.is_finite());
        // odd (a `Rational` has no negative zero, so x = 0 is excluded)
        if x != 0u32 {
            assert_eq!(
                NiceFloat(primitive_float_atan_with_period_rational::<T>(&-&x, u)),
                NiceFloat(-t)
            );
        }
        // the same as the `Float` version taken with 64 bits to spare and rounded once
        let (t_float, _) = Float::atan_with_period_rational_prec_ref(&x, u, T::MANTISSA_WIDTH + 64);
        assert_eq!(
            NiceFloat(T::rounding_from(&t_float, Nearest).0),
            NiceFloat(t)
        );
    });

    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        // The arctangent of a finite primitive float, taken through the `Rational` path, matches
        // the direct primitive-float arctangent.
        assert_eq!(
            NiceFloat(primitive_float_atan_with_period_rational::<T>(
                &Rational::exact_from(x),
                u
            )),
            NiceFloat(primitive_float_atan_with_period(x, u))
        );
    });
}

#[test]
fn primitive_float_atan_with_period_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_atan_with_period_rational_properties_helper);
}
