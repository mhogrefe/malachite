// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Acot, AcotAssign, IsPowerOf2, PowerOf2, Reciprocal};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Two, Zero,
};
use malachite_base::num::comparison::traits::EqAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, primitive_float_unsigned_pair_gen_var_1,
    unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::acot::{
    primitive_float_acot, primitive_float_acot_rational, primitive_float_acot_with_period,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::acot::{
    rug_acot, rug_acot_prec_round, rug_acot_rational_prec_round, rug_acot_with_period_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_52, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_2, float_unsigned_rounding_mode_triple_gen_var_51,
    float_unsigned_rounding_mode_triple_gen_var_52, float_unsigned_rounding_mode_triple_gen_var_53,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_29,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_30,
    float_unsigned_unsigned_triple_gen_var_1, rational_unsigned_rounding_mode_triple_gen_var_14,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;
use std::str::FromStr;

// The arccotangent of x is the arctangent of its reciprocal, and a `Float`'s reciprocal is an exact
// `Rational`, so the `Rational` arcsine gives the same correctly rounded answer. That identity is
// the strongest check available here, MPFR having no arccotangent.
//
// It is only applied to inputs of moderate magnitude: the `Rational` holds the `Float`'s exponent
// in full, so a `Float` with an extreme exponent would turn into a `Rational` of hundreds of
// megabytes; the huge regime is covered by `test_acot_huge` instead, a few inputs at a time.
fn atan_of_reciprocal(x: &Float, prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
    if !x.is_finite() || *x == 0u32 || x.get_exponent().unwrap().unsigned_abs() > 1000 {
        return None;
    }
    Some(Float::atan_rational_prec_round(
        Rational::exact_from(x).reciprocal(),
        prec,
        rm,
    ))
}

#[test]
fn test_acot_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().acot_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.acot_prec_round_ref(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.acot_prec_round_assign(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = x.acot_prec_ref(prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        // the arctangent of the exact reciprocal, which is the same real number
        if let Some((c_alt, o_alt)) = atan_of_reciprocal(&x, prec, rm) {
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acot_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Exact, "0.0", "0x0.0", Equal);
    test("-Infinity", "-Infinity", 10, Exact, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", 10, Floor, "1.5703", "0x1.920#10", Less);
    test("0.0", "0x0.0", 10, Ceiling, "1.5723", "0x1.928#10", Greater);
    test("0.0", "0x0.0", 10, Nearest, "1.5703", "0x1.920#10", Less);
    test("0.0", "0x0.0", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("-0.0", "-0x0.0", 10, Floor, "-1.5723", "-0x1.928#10", Less);
    test(
        "-0.0",
        "-0x0.0",
        10,
        Ceiling,
        "-1.5703",
        "-0x1.920#10",
        Greater,
    );
    test(
        "-0.0",
        "-0x0.0",
        10,
        Nearest,
        "-1.5703",
        "-0x1.920#10",
        Greater,
    );
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
    test("1.0", "0x1.0#1", 1, Nearest, "1.0", "0x1.0#1", Greater);
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Floor,
        "-0.78613",
        "-0x0.c94#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Ceiling,
        "-0.78516",
        "-0x0.c90#10",
        Greater,
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
    test("0.50", "0x0.8#1", 10, Floor, "1.1055", "0x1.1b0#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "1.1074",
        "0x1.1b8#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        10,
        Nearest,
        "1.1074",
        "0x1.1b8#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        53,
        Nearest,
        "1.1071487177940904",
        "0x1.1b6e192ebbe44#53",
        Less,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        53,
        Nearest,
        "-1.1071487177940904",
        "-0x1.1b6e192ebbe44#53",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        20,
        Nearest,
        "1.3258171",
        "0x1.5368c#20",
        Less,
    );
    test("2.0", "0x2.0#1", 10, Floor, "0.46338", "0x0.76a#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "0.46387",
        "0x0.76c#10",
        Greater,
    );
    test("2.0", "0x2.0#1", 10, Down, "0.46338", "0x0.76a#10", Less);
    test("2.0", "0x2.0#1", 10, Up, "0.46387", "0x0.76c#10", Greater);
    test(
        "2.0",
        "0x2.0#1",
        10,
        Nearest,
        "0.46387",
        "0x0.76c#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        53,
        Nearest,
        "0.46364760900080609",
        "0x0.76b19c1586ed3c#53",
        Less,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        53,
        Nearest,
        "-0.46364760900080609",
        "-0x0.76b19c1586ed3c#53",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        100,
        Nearest,
        "0.46364760900080611621425623146131",
        "0x0.76b19c1586ed3da2b7f222f660#100",
        Greater,
    );
    test("2.5", "0x2.8#3", 10, Nearest, "0.38037", "0x0.616#10", Less);
    test(
        "2.5",
        "0x2.8#3",
        53,
        Nearest,
        "0.38050637711236490",
        "0x0.6168ddad9df700#53",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        20,
        Nearest,
        "0.58800220",
        "0x0.96875#20",
        Less,
    );
    test(
        "-1.5",
        "-0x1.8#2",
        20,
        Nearest,
        "-0.58800220",
        "-0x0.96875#20",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#7",
        20,
        Nearest,
        "0.0099996626",
        "0x0.028f568#20",
        Less,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        53,
        Nearest,
        "0.78539816339744817",
        "0x0.c90fdaa22168b8#53",
        Less,
    );
    test(
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        53,
        Nearest,
        "0.78539816339744839",
        "0x0.c90fdaa22168c8#53",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        20,
        Nearest,
        "7.8886091e-31",
        "0x1.00000E-25#20",
        Greater,
    );
    test(
        "-1.3e30",
        "-0x1.0E+25#1",
        20,
        Nearest,
        "-7.8886091e-31",
        "-0x1.00000E-25#20",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        20,
        Nearest,
        "1.5707970",
        "0x1.921fc#20",
        Greater,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        20,
        Nearest,
        "-1.5707970",
        "-0x1.921fc#20",
        Less,
    );
}

#[test]
#[should_panic]
fn acot_prec_round_fail_1() {
    Float::TWO.acot_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn acot_prec_round_fail_2() {
    // acot(2) = atan(1/2) is not exactly representable
    Float::TWO.acot_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acot_prec_round_fail_3() {
    // acot(1) = pi/4 is not exactly representable either
    Float::ONE.acot_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acot_prec_round_fail_4() {
    // nor is acot(0) = pi/2
    Float::ZERO.acot_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acot_prec_round_ref_fail() {
    Float::TWO.acot_prec_round_ref(0, Floor);
}

#[test]
#[should_panic]
fn acot_prec_fail() {
    Float::TWO.acot_prec(0);
}

#[test]
#[should_panic]
fn acot_round_fail() {
    Float::TWO.acot_round(Exact);
}

// Whether acot(x) is exactly representable at `prec`: at a NaN, and at either infinity, where it is
// a zero. Neither zero (pi/2) nor |x| = 1 (pi/4) is an exact case.
const fn acot_exact(x: &Float) -> bool {
    x.is_nan() || !x.is_finite()
}

#[allow(clippy::needless_pass_by_value)]
fn acot_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    if rm == Exact && !acot_exact(&x) {
        assert_panic!(x.acot_prec_round_ref(prec, Exact));
        return;
    }
    let (c, o) = x.clone().acot_prec_round(prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.acot_prec_round_ref(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut c_alt = x.clone();
    let o_alt = c_alt.acot_prec_round_assign(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // the arctangent of the exact reciprocal, which is the same real number
    if let Some((c_alt, o_alt)) = atan_of_reciprocal(&x, prec, rm) {
        assert_eq!(
            ComparableFloatRef(&c_alt),
            ComparableFloatRef(&c),
            "x = {:#x} exp = {:?} prec_x = {:?} prec = {prec} rm = {rm:?}",
            ComparableFloatRef(&x),
            x.get_exponent(),
            x.get_prec()
        );
        assert_eq!(o_alt, o, "x = {x} prec = {prec} rm = {rm:?}");
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_acot_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o, "x = {x} prec = {prec} rm = {rm:?}");
    }

    // NaN exactly for a NaN input
    assert_eq!(c.is_nan(), x.is_nan());
    if !c.is_nan() {
        // |acot(x)| <= pi/2, so the result never overflows, and it never underflows either
        assert!(c.is_finite());
        // |acot(x)| <= pi/2 < 2, and rounding at any precision keeps it at most 2
        assert!(c <= 2u32);
        assert!(c >= -2i32);
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
    }
    // the arccotangent is odd
    let (c_neg, o_neg) = (-&x).acot_prec_round(prec, -rm);
    assert_eq!(ComparableFloat(-c_neg), ComparableFloat(c.clone()));
    assert_eq!(o_neg.reverse(), o);

    if o == Equal {
        assert!(acot_exact(&x));
        for rm in exhaustive_rounding_modes() {
            let (c2, oo) = x.acot_prec_round_ref(prec, rm);
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.acot_prec_round_ref(prec, Exact));
    }
}

#[test]
fn acot_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_51().test_properties(|(x, prec, rm)| {
        acot_prec_round_properties_helper(x, prec, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_52().test_properties(|(x, prec, rm)| {
        acot_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // acot(NaN) = NaN
        let (c, o) = Float::NAN.acot_prec_round(prec, rm);
        assert!(c.is_nan());
        assert_eq!(o, Equal);
        // acot(infinity) = +0 and acot(-infinity) = -0, exactly
        let (c, o) = Float::INFINITY.acot_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (c, o) = Float::NEGATIVE_INFINITY.acot_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
        // acot(+-0) = +-pi/2 and acot(+-1) = +-pi/4, none of them representable, so `Exact` panics
        // rather than returning anything -- and `pi_prec_round` itself panics under `Exact`, so it
        // cannot be used to state the expectation either
        if rm == Exact {
            for x in [Float::ZERO, Float::NEGATIVE_ZERO, Float::ONE, Float::NEGATIVE_ONE] {
                assert_panic!(x.acot_prec_round(prec, Exact));
            }
        } else {
            for (x, k) in [(Float::ZERO, 1u32), (Float::ONE, 2)] {
                let (p, o_p) = Float::pi_prec_round(prec, rm);
                let (c, o) = x.acot_prec_round_ref(prec, rm);
                assert_eq!(ComparableFloat(c), ComparableFloat(p >> k));
                assert_eq!(o, o_p);
                let (p, o_p) = Float::pi_prec_round(prec, -rm);
                let (c, o) = (-x).acot_prec_round(prec, rm);
                assert_eq!(ComparableFloat(c), ComparableFloat(-(p >> k)));
                assert_eq!(o, o_p.reverse());
            }
        }
    });
}

#[test]
fn acot_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (c, o) = x.clone().acot_prec(prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.acot_prec_ref(prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acot_prec_assign(prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acot_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn acot_round_properties() {
    float_rounding_mode_pair_gen_var_52().test_properties(|(x, rm)| {
        let (c, o) = x.clone().acot_round(rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.acot_round_ref(rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acot_round_assign(rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acot_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn acot_properties() {
    float_gen().test_properties(|x| {
        let c = x.clone().acot();
        assert!(c.is_valid());
        let c_alt = (&x).acot();
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let mut c_alt = x.clone();
        c_alt.acot_assign();
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        // the same as rounding to the input's precision, to nearest
        let (c_alt, _) = x.acot_prec_ref(x.significant_bits());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        if x.is_finite() && x != 0u32 && x.get_exponent().unwrap().unsigned_abs() <= 1000 {
            let c_alt = Float::from(&rug_acot(&rug::Float::exact_from(&x)));
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        }
    });
}

#[test]
fn test_acot_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::acot_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = Float::acot_rational_prec_round_ref(&x, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = Float::acot_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
            let (c_alt, o_alt) = Float::acot_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        // the arctangent of the exact reciprocal, which is the same real number
        if x != 0u32 {
            let (c_alt, o_alt) = Float::atan_rational_prec_round(x.clone().reciprocal(), prec, rm);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acot_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 10, Floor, "1.5703", "0x1.920#10", Less);
    test("0", 10, Ceiling, "1.5723", "0x1.928#10", Greater);
    test("0", 10, Nearest, "1.5703", "0x1.920#10", Less);
    test("0", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1", 10, Floor, "0.78516", "0x0.c90#10", Less);
    test("1", 10, Ceiling, "0.78613", "0x0.c94#10", Greater);
    test("1", 10, Nearest, "0.78516", "0x0.c90#10", Less);
    test("-1", 10, Nearest, "-0.78516", "-0x0.c90#10", Greater);
    test("1/2", 10, Floor, "1.1055", "0x1.1b0#10", Less);
    test("1/2", 10, Ceiling, "1.1074", "0x1.1b8#10", Greater);
    test(
        "1/2",
        53,
        Nearest,
        "1.1071487177940904",
        "0x1.1b6e192ebbe44#53",
        Less,
    );
    test(
        "-1/2",
        53,
        Nearest,
        "-1.1071487177940904",
        "-0x1.1b6e192ebbe44#53",
        Greater,
    );
    test("2", 10, Floor, "0.46338", "0x0.76a#10", Less);
    test("2", 10, Ceiling, "0.46387", "0x0.76c#10", Greater);
    test(
        "2",
        53,
        Nearest,
        "0.46364760900080609",
        "0x0.76b19c1586ed3c#53",
        Less,
    );
    test(
        "-2",
        53,
        Nearest,
        "-0.46364760900080609",
        "-0x0.76b19c1586ed3c#53",
        Greater,
    );
    test("5/3", 10, Floor, "0.54004", "0x0.8a4#10", Less);
    test("5/3", 10, Ceiling, "0.54102", "0x0.8a8#10", Greater);
    test(
        "5/3",
        53,
        Nearest,
        "0.54041950027058416",
        "0x0.8a58eeafc86708#53",
        Greater,
    );
    test(
        "3/5",
        53,
        Nearest,
        "1.0303768265243125",
        "0x1.07c6c6947a6a8#53",
        Greater,
    );
    test("100/99", 20, Nearest, "0.78037262", "0x0.c7c68#20", Less);
    test("99/100", 20, Nearest, "0.79042339", "0x0.ca593#20", Greater);
    test("100", 20, Nearest, "0.0099996626", "0x0.028f568#20", Less);
    test(
        "-100",
        20,
        Nearest,
        "-0.0099996626",
        "-0x0.028f568#20",
        Greater,
    );
    test("1/100", 20, Nearest, "1.5607967", "0x1.8f906#20", Greater);
    test(
        "100000000000000000000000000000000000000000",
        20,
        Nearest,
        "1.0000005e-41",
        "0xd.f01fE-35#20",
        Greater,
    );
    test(
        "1/100000000000000000000000000000000000000000",
        20,
        Nearest,
        "1.5707970",
        "0x1.921fc#20",
        Greater,
    );
}

// A `Rational` far below the target precision but above the bottom of the exponent range, where the
// arccotangent is the arctangent of the reciprocal, exact for a `Rational`, so the huge inputs lean
// on `atan_rational_helper`'s cheap tiny-input branch and the tiny ones on its use inside pi/2 -
// atan(x): 7 seconds a call at either end before that branch existed. Where x is dyadic it is
// exactly a `Float`, so the `Float` arccotangent, reached through independent code, must agree.
#[test]
fn test_acot_rational_tiny() {
    let test =
        |x: Rational, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
            let (c, o) = Float::acot_rational_prec_round_ref(&x, prec, rm);
            assert!(c.is_valid());
            assert_eq!(c.to_string(), out);
            assert_eq!(to_hex_string(&c), out_hex);
            assert_eq!(o, o_out);
            if let Ok(f) = Float::try_from(&x) {
                let (c_alt, o_alt) = f.acot_prec_round(prec, rm);
                assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
                assert_eq!(o_alt, o);
            }
        };
    test(
        Rational::power_of_2(-536870908i64),
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        Rational::power_of_2(-536870908i64),
        53,
        Ceiling,
        "1.5707963267948968",
        "0x1.921fb54442d19#53",
        Greater,
    );
    test(
        -Rational::power_of_2(-536870908i64),
        53,
        Nearest,
        "-1.5707963267948966",
        "-0x1.921fb54442d18#53",
        Greater,
    );
    test(
        Rational::power_of_2(-536870912i64) / Rational::from(3u32),
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        Rational::power_of_2(536870908i64),
        53,
        Nearest,
        "7.8098438886530598e-161614248",
        "0x1.0000000000000E-134217727#53",
        Greater,
    );
    test(
        Rational::power_of_2(536870908i64),
        53,
        Floor,
        "7.8098438886530590e-161614248",
        "0xf.ffffffffffff8E-134217728#53",
        Less,
    );
    test(
        Rational::power_of_2(536870912i64) * Rational::from(3u32),
        53,
        Nearest,
        "1.6270508101360540e-161614249",
        "0x5.5555555555554E-134217729#53",
        Less,
    );
}

#[test]
#[should_panic]
fn acot_rational_prec_round_fail_1() {
    Float::acot_rational_prec_round(Rational::TWO, 0, Floor);
}

#[test]
#[should_panic]
fn acot_rational_prec_round_fail_2() {
    // acot(2) = atan(1/2) is not exactly representable
    Float::acot_rational_prec_round(Rational::TWO, 10, Exact);
}

#[test]
#[should_panic]
fn acot_rational_prec_round_fail_3() {
    // acot(1) = pi/4 is not exactly representable either
    Float::acot_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn acot_rational_prec_round_fail_4() {
    // nor is acot(0) = pi/2
    Float::acot_rational_prec_round(Rational::ZERO, 10, Exact);
}

#[test]
#[should_panic]
fn acot_rational_prec_fail() {
    Float::acot_rational_prec(Rational::TWO, 0);
}

// A `Rational` large enough that acot(x), about 1/x, falls below the smallest positive `Float` --
// underflow the `Float` arccotangent cannot reach, a `Float`'s exponent being bounded. It is the
// arctangent's own handling of an exact tiny reciprocal that decides it. The `Rational` has about
// 2^30 bits, so each call costs a few seconds.
#[test]
fn test_acot_rational_underflow() {
    let x = Rational::power_of_2((1i64 << 30) + 2);
    let (c, o) = Float::acot_rational_prec_round_ref(&x, 53, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    let (c, o) = Float::acot_rational_prec_round_ref(&x, 53, Ceiling);
    assert_eq!(
        ComparableFloat(c),
        ComparableFloat(Float::min_positive_value_prec(53))
    );
    assert_eq!(o, Greater);
    let (c, o) = Float::acot_rational_prec_round_ref(&x, 53, Floor);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    // the arccotangent is odd, so a negative input underflows to the other side
    let x = -x;
    let (c, o) = Float::acot_rational_prec_round_ref(&x, 53, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    let (c, o) = Float::acot_rational_prec_round_ref(&x, 53, Floor);
    assert_eq!(
        ComparableFloat(c),
        ComparableFloat(-Float::min_positive_value_prec(53))
    );
    assert_eq!(o, Less);
}

#[allow(clippy::needless_pass_by_value)]
fn acot_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    if rm == Exact {
        // no `Rational` has a representable arccotangent
        assert_panic!(Float::acot_rational_prec_round_ref(&x, prec, Exact));
        return;
    }
    let (c, o) = Float::acot_rational_prec_round(x.clone(), prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = Float::acot_rational_prec_round_ref(&x, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // the arctangent of the exact reciprocal, which is the same real number
    if x != 0u32 {
        let (c_alt, o_alt) = Float::atan_rational_prec_round(x.clone().reciprocal(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&c_alt),
            ComparableFloatRef(&c),
            "x = {x} exp = {} prec = {prec} rm = {rm:?}",
            x.floor_log_base_2_abs() + 1
        );
        assert_eq!(o_alt, o, "x = {x} prec = {prec} rm = {rm:?}");
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_acot_rational_prec_round(&x, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o, "x = {x} prec = {prec} rm = {rm:?}");
    }

    // never NaN
    assert!(!c.is_nan());
    if !c.is_nan() {
        // |acot(x)| <= pi/2, so the result never overflows
        assert!(c.is_finite());
        // |acot(x)| <= pi/2 < 2, and rounding at any precision keeps it at most 2
        assert!(c <= 2u32);
        assert!(c >= -2i32);
    }
    // a `Float` input agrees with the `Float` version
    if let Ok(f) = Float::try_from(&x) {
        let (c_alt, o_alt) = f.acot_prec_round(prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }

    // and never exact
    assert_ne!(o, Equal);
    assert_panic!(Float::acot_rational_prec_round_ref(&x, prec, Exact));
}

#[test]
fn acot_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_14().test_properties(|(x, prec, rm)| {
        acot_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // acot(0) = pi/2 (the positive side, a `Rational` zero having no sign) and acot(+-1) =
        // +-pi/4, none of them representable, so `Exact` panics
        if rm == Exact {
            for x in [Rational::ZERO, Rational::ONE, Rational::NEGATIVE_ONE] {
                assert_panic!(Float::acot_rational_prec_round(x, prec, Exact));
            }
        } else {
            let (p, o_p) = Float::pi_prec_round(prec, rm);
            let (c, o) = Float::acot_rational_prec_round(Rational::ZERO, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(p >> 1u32));
            assert_eq!(o, o_p);
            let (p, o_p) = Float::pi_prec_round(prec, rm);
            let (c, o) = Float::acot_rational_prec_round(Rational::ONE, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(p >> 2u32));
            assert_eq!(o, o_p);
            let (p, o_p) = Float::pi_prec_round(prec, -rm);
            let (c, o) = Float::acot_rational_prec_round(Rational::NEGATIVE_ONE, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(-(p >> 2u32)));
            assert_eq!(o, o_p.reverse());
        }
    });
}

#[test]
fn acot_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (c, o) = Float::acot_rational_prec(x.clone(), prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = Float::acot_rational_prec_ref(&x, prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = Float::acot_rational_prec_round_ref(&x, prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
#[allow(clippy::approx_constant, clippy::type_repetition_in_bounds)]
fn test_primitive_float_acot() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_acot(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, 0.0);
    test::<f32>(f32::NEGATIVE_INFINITY, -0.0);
    test::<f32>(0.0, 1.5707964);
    test::<f32>(-0.0, -1.5707964);
    test::<f32>(1.0, 0.7853982);
    test::<f32>(-1.0, -0.7853982);
    test::<f32>(0.5, 1.1071488);
    test::<f32>(-0.5, -1.1071488);
    test::<f32>(2.0, 0.4636476);
    test::<f32>(-2.0, -0.4636476);
    test::<f32>(2.5, 0.38050637);
    test::<f32>(1.5, 0.5880026);
    test::<f32>(100.0, 0.009999666);
    test::<f32>(0.01, 1.5607966);
    test::<f32>(1.0e30, 1.0e-30);
    test::<f32>(1.0e-30, 1.5707964);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, 0.0);
    test::<f64>(f64::NEGATIVE_INFINITY, -0.0);
    test::<f64>(0.0, 1.5707963267948966);
    test::<f64>(-0.0, -1.5707963267948966);
    test::<f64>(1.0, 0.7853981633974483);
    test::<f64>(-1.0, -0.7853981633974483);
    test::<f64>(0.5, 1.1071487177940904);
    test::<f64>(-0.5, -1.1071487177940904);
    test::<f64>(2.0, 0.4636476090008061);
    test::<f64>(-2.0, -0.4636476090008061);
    test::<f64>(2.5, 0.3805063771123649);
    test::<f64>(1.5, 0.5880026035475675);
    test::<f64>(100.0, 0.009999666686665238);
    test::<f64>(0.01, 1.5607966601082315);
    test::<f64>(1.0e300, 1.0e-300);
    test::<f64>(1.0e-300, 1.5707963267948966);
}

#[test]
#[allow(clippy::approx_constant, clippy::type_repetition_in_bounds)]
fn test_primitive_float_acot_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_acot_rational::<T>(
                &Rational::from_str(s).unwrap()
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 1.5707964);
    test::<f32>("1", 0.7853982);
    test::<f32>("-1", -0.7853982);
    test::<f32>("1/2", 1.1071488);
    test::<f32>("-1/2", -1.1071488);
    test::<f32>("2", 0.4636476);
    test::<f32>("-2", -0.4636476);
    test::<f32>("5/3", 0.5404195);
    test::<f32>("3/5", 1.0303768);
    test::<f32>("100/99", 0.7803731);
    test::<f32>("100", 0.009999666);
    test::<f32>("1/100", 1.5607966);
    test::<f64>("0", 1.5707963267948966);
    test::<f64>("1", 0.7853981633974483);
    test::<f64>("-1", -0.7853981633974483);
    test::<f64>("1/2", 1.1071487177940904);
    test::<f64>("-1/2", -1.1071487177940904);
    test::<f64>("2", 0.4636476090008061);
    test::<f64>("-2", -0.4636476090008061);
    test::<f64>("5/3", 0.5404195002705842);
    test::<f64>("3/5", 1.0303768265243125);
    test::<f64>("100/99", 0.7803730800666359);
    test::<f64>("100", 0.009999666686665238);
    test::<f64>("1/100", 1.5607966601082315);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acot_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let c = primitive_float_acot(x);
        // NaN exactly for a NaN input
        assert_eq!(c.is_nan(), x.is_nan());
        if !c.is_nan() {
            // the result lies in [-pi/2, pi/2], so it never overflows
            assert!(c.is_finite());
            // the same as the `Float` version taken with 64 bits to spare and rounded once -- but
            // only where the result is normal, a subnormal one being rounded twice here
            if c.is_normal() {
                let (c_float, _) = Float::acot_prec(Float::from(x), T::MANTISSA_WIDTH + 64);
                assert_eq!(
                    NiceFloat(T::rounding_from(&c_float, Nearest).0),
                    NiceFloat(c)
                );
            }
        }
    });
}

#[test]
fn primitive_float_acot_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acot_properties_helper);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acot_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let c = primitive_float_acot_rational::<T>(&x);
        // never NaN
        assert!(!c.is_nan());
        if !c.is_nan() {
            assert!(c.is_finite());
            // the same as the `Float` version taken with 64 bits to spare and rounded once -- but
            // only where the result is normal, a subnormal one being rounded twice here and once by
            // `emulate_rational_to_float_fn`
            if c.is_normal() {
                let (c_float, _) = Float::acot_rational_prec_ref(&x, T::MANTISSA_WIDTH + 64);
                assert_eq!(
                    NiceFloat(T::rounding_from(&c_float, Nearest).0),
                    NiceFloat(c)
                );
            }
        }
    });
}

#[test]
fn primitive_float_acot_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acot_rational_properties_helper);
}

// The arccotangent in u-ths of a turn is the arctangent of the reciprocal in the same units, and a
// `Float`'s reciprocal is an exact `Rational`. As for the plain arccotangent, the check is confined
// to inputs of moderate magnitude, since the `Rational` holds the exponent in full;
// `test_acot_with_period_huge` covers the huge regime instead.
fn atanu_of_reciprocal(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    if !x.is_finite() || *x == 0u32 || x.get_exponent().unwrap().unsigned_abs() > 1000 {
        return None;
    }
    Some(Float::atan_with_period_rational_prec_round(
        Rational::exact_from(x).reciprocal(),
        u,
        prec,
        rm,
    ))
}

// The exact identity behind the arccotangent -- the arctangent of the reciprocal -- applied to the
// huge inputs that `atan_of_reciprocal` above cannot afford: a `Float` near the top of the exponent
// range becomes a `Rational` of hundreds of megabytes, so the property tests stop at an exponent of
// 1000. The tiny-|x| rows reach the other end, where the arccotangent approaches a quarter turn.
//
// Where 1/x is exactly representable -- x a power of two, the case that reaches the exact-value
// nudge -- the identity is taken through the `Float` arctangent, which is instant. Otherwise it
// goes through the `Rational` one, at about five seconds a call.
fn atan_of_reciprocal_huge(x: &Float, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    if x.significand_ref().unwrap().is_power_of_2() {
        let (r, o_r) = x.reciprocal_prec_round_ref(64, Exact);
        assert_eq!(o_r, Equal);
        r.atan_prec_round(prec, rm)
    } else {
        Float::atan_rational_prec_round(Rational::exact_from(x).reciprocal(), prec, rm)
    }
}

// As `atan_of_reciprocal_huge`, in u-ths of a turn.
fn atanu_of_reciprocal_huge(x: &Float, u: u64, prec: u64, rm: RoundingMode) -> (Float, Ordering) {
    if x.significand_ref().unwrap().is_power_of_2() {
        let (r, o_r) = x.reciprocal_prec_round_ref(64, Exact);
        assert_eq!(o_r, Equal);
        r.atan_with_period_prec_round(u, prec, rm)
    } else {
        Float::atan_with_period_rational_prec_round(
            Rational::exact_from(x).reciprocal(),
            u,
            prec,
            rm,
        )
    }
}

#[test]
fn test_acot_huge() {
    let test =
        |s_hex: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
            let x = parse_hex_string(s_hex);
            let (c, o) = x.acot_prec_round_ref(prec, rm);
            assert!(c.is_valid());
            assert_eq!(c.to_string(), out);
            assert_eq!(to_hex_string(&c), out_hex);
            assert_eq!(o, o_out);
            let (c_alt, o_alt) = atan_of_reciprocal_huge(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        };
    test(
        "0x1.0E+134217727#1",
        53,
        Nearest,
        "7.8098438886530598e-161614248",
        "0x1.0000000000000E-134217727#53",
        Greater,
    );
    test(
        "0x1.0E+134217728#1",
        53,
        Nearest,
        "4.8811524304081624e-161614249",
        "0x1.0000000000000E-134217728#53",
        Greater,
    );
    test(
        "0x1.0E+134217728#1",
        53,
        Floor,
        "4.8811524304081619e-161614249",
        "0xf.ffffffffffff8E-134217729#53",
        Less,
    );
    test(
        "0x1.0E+134217728#1",
        53,
        Ceiling,
        "4.8811524304081624e-161614249",
        "0x1.0000000000000E-134217728#53",
        Greater,
    );
    test(
        "0x4.0E+268435455#1",
        20,
        Nearest,
        "9.5302596e-323228497",
        "0x4.00000E-268435456#20",
        Greater,
    );
    test(
        "0x3.0E+134217728#2",
        53,
        Nearest,
        "1.6270508101360540e-161614249",
        "0x5.5555555555554E-134217729#53",
        Less,
    );
    test(
        "-0x4.0E+268435455#1",
        20,
        Nearest,
        "-9.5302596e-323228497",
        "-0x4.00000E-268435456#20",
        Less,
    );
    test(
        "-0x1.0E+134217728#1",
        53,
        Nearest,
        "-4.8811524304081624e-161614249",
        "-0x1.0000000000000E-134217728#53",
        Less,
    );
    test(
        "0x1.0E-134217728#1",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        "-0x1.0E-134217728#1",
        53,
        Nearest,
        "-1.5707963267948966",
        "-0x1.921fb54442d18#53",
        Greater,
    );
}

#[test]
fn test_acot_with_period_huge() {
    let test = |s_hex: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        let (c, o) = x.acot_with_period_prec_round_ref(u, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);
        let (c_alt, o_alt) = atanu_of_reciprocal_huge(&x, u, prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    };
    test(
        "0x1.0E+134217728#1",
        360,
        53,
        Nearest,
        "2.7966943342241198e-161614247",
        "0x3.94bb834c783f0E-134217727#53",
        Greater,
    );
    test(
        "0x1.0E+134217728#1",
        1,
        53,
        Nearest,
        "7.7685953728447774e-161614250",
        "0x2.8be60db939106E-134217729#53",
        Greater,
    );
    test(
        "0x1.0E+134217728#1",
        1,
        53,
        Ceiling,
        "7.7685953728447774e-161614250",
        "0x2.8be60db939106E-134217729#53",
        Greater,
    );
    test(
        "0x1.0E+134217728#1",
        1,
        53,
        Floor,
        "7.7685953728447761e-161614250",
        "0x2.8be60db939104E-134217729#53",
        Less,
    );
    test(
        "0x4.0E+268435455#1",
        1,
        20,
        Nearest,
        "2.3825649e-323228497",
        "0x1.00000E-268435456#20",
        Greater,
    );
    test(
        "0x3.0E+134217728#2",
        360,
        53,
        Nearest,
        "9.3223144474137315e-161614248",
        "0x1.3193d66ed2bfaE-134217727#53",
        Less,
    );
    test(
        "-0x4.0E+268435455#1",
        1,
        20,
        Nearest,
        "-2.3825649e-323228497",
        "-0x1.00000E-268435456#20",
        Less,
    );
    test(
        "-0x1.0E+134217728#1",
        360,
        53,
        Nearest,
        "-2.7966943342241198e-161614247",
        "-0x3.94bb834c783f0E-134217727#53",
        Less,
    );
    test(
        "0x1.0E-134217728#1",
        360,
        53,
        Nearest,
        "90.000000000000000",
        "0x5a.000000000000#53",
        Greater,
    );
    test(
        "-0x1.0E-134217728#1",
        360,
        53,
        Nearest,
        "-90.000000000000000",
        "-0x5a.000000000000#53",
        Less,
    );
}

#[test]
fn test_acot_with_period_prec_round() {
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

        let (c, o) = x.clone().acot_with_period_prec_round(u, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.acot_with_period_prec_round_ref(u, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.acot_with_period_prec_round_assign(u, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = x.acot_with_period_prec_ref(u, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        // the arctangent of the exact reciprocal, in the same units
        if let Some((c_alt, o_alt)) = atanu_of_reciprocal(&x, u, prec, rm) {
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u <= u64::from(u32::MAX)
        {
            let (rug_c, rug_o) =
                rug_acot_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 360, 10, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 360, 10, Exact, "0.0", "0x0.0", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        360,
        10,
        Exact,
        "-0.0",
        "-0x0.0",
        Equal,
    );
    test("Infinity", "Infinity", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 360, 10, Exact, "90.000", "0x5a.0#10", Equal);
    test(
        "-0.0",
        "-0x0.0",
        360,
        10,
        Exact,
        "-90.000",
        "-0x5a.0#10",
        Equal,
    );
    test("0.0", "0x0.0", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 0, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("0.0", "0x0.0", 7, 10, Exact, "1.7500", "0x1.c00#10", Equal);
    test("0.0", "0x0.0", 7, 1, Floor, "1.0", "0x1.0#1", Less);
    test("0.0", "0x0.0", 7, 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("0.0", "0x0.0", 7, 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 0, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("2.5", "0x2.8#3", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "1.0",
        "0x1.0#1",
        360,
        10,
        Exact,
        "45.000",
        "0x2d.0#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        360,
        10,
        Exact,
        "-45.000",
        "-0x2d.0#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        8,
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        7,
        10,
        Exact,
        "0.87500",
        "0x0.e00#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        7,
        10,
        Exact,
        "-0.87500",
        "-0x0.e00#10",
        Equal,
    );
    test("1.0", "0x1.0#1", 7, 2, Floor, "0.75", "0x0.c#2", Less);
    test("1.0", "0x1.0#1", 7, 2, Ceiling, "1.0", "0x1.0#2", Greater);
    test("1.0", "0x1.0#1", 7, 2, Nearest, "1.0", "0x1.0#2", Greater);
    test(
        "0.50",
        "0x0.8#1",
        360,
        10,
        Nearest,
        "63.438",
        "0x3f.7#10",
        Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        360,
        10,
        Nearest,
        "-63.438",
        "-0x3f.7#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        360,
        53,
        Nearest,
        "63.434948822922010",
        "0x3f.6f58ce59e23c#53",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Floor,
        "26.562",
        "0x1a.90#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Ceiling,
        "26.594",
        "0x1a.98#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Down,
        "26.562",
        "0x1a.90#10",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Up,
        "26.594",
        "0x1a.98#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Nearest,
        "26.562",
        "0x1a.90#10",
        Less,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        360,
        10,
        Nearest,
        "-26.562",
        "-0x1a.90#10",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        100,
        Nearest,
        "26.565051177077989351572193720457",
        "0x1a.90a731a61dc3cfe9b09b23d0#100",
        Greater,
    );
    test(
        "2.5",
        "0x2.8#3",
        360,
        10,
        Nearest,
        "21.812",
        "0x15.d0#10",
        Greater,
    );
    test(
        "2.5",
        "0x2.8#3",
        360,
        53,
        Nearest,
        "21.801409486351812",
        "0x15.cd292c0e95cf#53",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        360,
        20,
        Nearest,
        "33.690063",
        "0x21.b0a8#20",
        Less,
    );
    test(
        "-1.5",
        "-0x1.8#2",
        360,
        20,
        Nearest,
        "-33.690063",
        "-0x21.b0a8#20",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#7",
        360,
        20,
        Nearest,
        "0.57293892",
        "0x0.92ac2#20",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        360,
        20,
        Nearest,
        "4.5198398e-29",
        "0x3.94bb8E-24#20",
        Less,
    );
    test(
        "-1.3e30",
        "-0x1.0E+25#1",
        360,
        20,
        Nearest,
        "-4.5198398e-29",
        "-0x3.94bb8E-24#20",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        1,
        20,
        Nearest,
        "1.2555107e-31",
        "0x2.8be60E-26#20",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        360,
        20,
        Nearest,
        "90.000000",
        "0x5a.0000#20",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        18446744073709551615,
        20,
        Nearest,
        "2.3160085e-12",
        "0x2.8be60E-10#20",
        Less,
    );
    test(
        "1.0",
        "0x1.0#1",
        18446744073709551615,
        64,
        Exact,
        "2305843009213693951.88",
        "0x1fffffffffffffff.e#64",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        18446744073709551615,
        64,
        Exact,
        "4611686018427387903.75",
        "0x3fffffffffffffff.c#64",
        Equal,
    );
    test(
        "2.5",
        "0x2.8#3",
        18446744073709551615,
        20,
        Nearest,
        "1.1171247e18",
        "0xf.80d3E+14#20",
        Less,
    );
}

#[test]
#[should_panic]
fn acot_with_period_prec_round_fail_1() {
    Float::TWO.acot_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn acot_with_period_prec_round_fail_2() {
    // acot(5/2) is not an exact number of sevenths of a turn
    Float::from(2.5).acot_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn acot_with_period_prec_round_fail_3() {
    // an eighth of a turn needs more than 2 bits when u = 7
    Float::ONE.acot_with_period_prec_round(7, 2, Exact);
}

#[test]
#[should_panic]
fn acot_with_period_prec_round_fail_4() {
    // and so does a quarter turn
    Float::ZERO.acot_with_period_prec_round(7, 2, Exact);
}

#[test]
#[should_panic]
fn acot_with_period_prec_round_ref_fail() {
    Float::TWO.acot_with_period_prec_round_ref(7, 0, Floor);
}

#[test]
#[should_panic]
fn acot_with_period_prec_fail() {
    Float::TWO.acot_with_period_prec(7, 0);
}

#[test]
#[should_panic]
fn acot_with_period_round_fail() {
    Float::from(2.5).acot_with_period_round(7, Exact);
}

// The largest `Float`s put acot(x), about 1/x, only twice above the smallest positive `Float`, so
// dividing by 2 pi with a period of 1 falls below it -- to about 0.64 of it, which `Nearest` still
// rounds up to it, and `Floor` down to zero. This is the underflow the plain arccotangent cannot
// reach, and a larger period lifts the quotient back into the range.
#[test]
fn test_acot_with_period_underflow() {
    let x = parse_hex_string("0x4.0E+268435455#1");
    let min = Float::min_positive_value_prec(20);
    let (c, o) = x.acot_with_period_prec_round_ref(1, 20, Nearest);
    assert_eq!(ComparableFloatRef(&c), ComparableFloatRef(&min));
    assert_eq!(o, Greater);
    let (c, o) = x.acot_with_period_prec_round_ref(1, 20, Floor);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    let (c, o) = x.acot_with_period_prec_round_ref(1, 20, Ceiling);
    assert_eq!(ComparableFloatRef(&c), ComparableFloatRef(&min));
    assert_eq!(o, Greater);
    // a period of 8 lifts the quotient back into the range, and doubling it doubles the result
    let (c, o) = x.acot_with_period_prec_round_ref(8, 20, Nearest);
    assert!(c > min);
    assert_ne!(o, Equal);
    let (c_alt, o_alt) = x.acot_with_period_prec_round_ref(16, 20, Nearest);
    assert_eq!(ComparableFloat(c_alt), ComparableFloat(c << 1u32));
    assert_eq!(o_alt, o);
    // the arccotangent is odd, so a negative input underflows to the other side
    let x = -x;
    let (c, o) = x.acot_with_period_prec_round_ref(1, 20, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(-&min));
    assert_eq!(o, Less);
    let (c, o) = x.acot_with_period_prec_round_ref(1, 20, Ceiling);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    let (c, o) = x.acot_with_period_prec_round_ref(1, 20, Floor);
    assert_eq!(ComparableFloat(c), ComparableFloat(-min));
    assert_eq!(o, Less);
}

// Whether acotu(x, u) is exactly representable at `prec`: at a NaN, at either infinity, at u = 0,
// and at the turn fractions -- a quarter at a zero and an eighth at |x| = 1 -- each of which needs
// a `prec` wide enough to hold it.
fn acot_with_period_exact(x: &Float, u: u64, prec: u64) -> bool {
    x.is_nan()
        || !x.is_finite()
        || u == 0
        || ((*x == 0u32 || x.eq_abs(&Float::ONE)) && Float::from_unsigned_prec(u, prec).1 == Equal)
}

#[allow(clippy::needless_pass_by_value)]
fn acot_with_period_prec_round_properties_helper(x: Float, u: u64, prec: u64, rm: RoundingMode) {
    if rm == Exact && !acot_with_period_exact(&x, u, prec) {
        assert_panic!(x.acot_with_period_prec_round_ref(u, prec, Exact));
        return;
    }
    let (c, o) = x.clone().acot_with_period_prec_round(u, prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.acot_with_period_prec_round_ref(u, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut c_alt = x.clone();
    let o_alt = c_alt.acot_with_period_prec_round_assign(u, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // the arctangent of the exact reciprocal, in the same units
    if let Some((c_alt, o_alt)) = atanu_of_reciprocal(&x, u, prec, rm) {
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o, "x = {x} u = {u} prec = {prec} rm = {rm:?}");
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && u <= u64::from(u32::MAX)
    {
        let (rug_c, rug_o) =
            rug_acot_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o, "x = {x} u = {u} prec = {prec} rm = {rm:?}");
    }

    // NaN exactly for a NaN input
    assert_eq!(c.is_nan(), x.is_nan());
    if !c.is_nan() {
        // |acotu(x, u)| <= u/4, a quarter turn, so the result never overflows
        assert!(c.is_finite());
        let bound = Float::from_unsigned_prec_round(u, prec, Ceiling).0 >> 2u32;
        assert!(c <= bound);
        assert!(c >= -bound);
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
    }
    // the arccotangent is odd
    let (c_neg, o_neg) = (-&x).acot_with_period_prec_round(u, prec, -rm);
    assert_eq!(ComparableFloat(-c_neg), ComparableFloat(c.clone()));
    assert_eq!(o_neg.reverse(), o);

    if o == Equal {
        assert!(acot_with_period_exact(&x, u, prec));
        for rm in exhaustive_rounding_modes() {
            let (c2, oo) = x.acot_with_period_prec_round_ref(u, prec, rm);
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.acot_with_period_prec_round_ref(u, prec, Exact));
    }
}

#[test]
fn acot_with_period_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_29().test_properties(
        |(x, u, prec, rm)| {
            acot_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_30().test_properties(
        |(x, u, prec, rm)| {
            acot_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // acotu(NaN, u) = NaN, even when u is zero
        for u in [0, 4] {
            let (c, o) = Float::NAN.acot_with_period_prec_round(u, prec, rm);
            assert!(c.is_nan());
            assert_eq!(o, Equal);
        }
        // acotu(±infinity, u) = ±0, exactly, for every period
        for u in [0, 4] {
            let (c, o) = Float::INFINITY.acot_with_period_prec_round(u, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
            assert_eq!(o, Equal);
            let (c, o) = Float::NEGATIVE_INFINITY.acot_with_period_prec_round(u, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
            assert_eq!(o, Equal);
        }
        // acotu(x, 0) = 0 with the sign of x, exactly, the zeros included
        for x in [Float::ZERO, Float::ONE, Float::TWO, Float::from(2.5)] {
            let (c, o) = x.acot_with_period_prec_round_ref(0, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
            assert_eq!(o, Equal);
            let (c, o) = (-x).acot_with_period_prec_round(0, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
            assert_eq!(o, Equal);
        }
        // acotu(±0, u) = ±u/4, a quarter turn, and acotu(±1, u) = ±u/8, an eighth, exact when
        // `prec` holds them
        for (x, k) in [(Float::ZERO, 2u32), (Float::ONE, 3)] {
            let (q, o_q) = Float::from_unsigned_prec_round(8u32, prec, rm);
            let (c, o) = x.acot_with_period_prec_round_ref(8, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(q >> k));
            assert_eq!(o, o_q);
            let (q, o_q) = Float::from_unsigned_prec_round(8u32, prec, -rm);
            let (c, o) = (-x).acot_with_period_prec_round(8, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(-(q >> k)));
            assert_eq!(o, o_q.reverse());
        }
    });
}

#[test]
fn acot_with_period_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, u, prec)| {
        let (c, o) = x.clone().acot_with_period_prec(u, prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.acot_with_period_prec_ref(u, prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acot_with_period_prec_assign(u, prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acot_with_period_prec_round_ref(u, prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn acot_with_period_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_53().test_properties(|(x, u, rm)| {
        let (c, o) = x.clone().acot_with_period_round(u, rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.acot_with_period_round_ref(u, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acot_with_period_round_assign(u, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acot_with_period_prec_round_ref(u, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn acot_with_period_properties() {
    float_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, u)| {
        let c = x.clone().acot_with_period(u);
        assert!(c.is_valid());
        let c_alt = x.acot_with_period_ref(u);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let mut c_alt = x.clone();
        c_alt.acot_with_period_assign(u);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        // the same as rounding to the input's precision, to nearest
        let (c_alt, _) = x.acot_with_period_prec_ref(u, x.significant_bits());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    });
}

#[test]
#[allow(clippy::approx_constant, clippy::type_repetition_in_bounds)]
fn test_primitive_float_acot_with_period() {
    fn test<T: PrimitiveFloat>(x: T, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_acot_with_period(x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 360, f32::NAN);
    test::<f32>(f32::INFINITY, 360, 0.0);
    test::<f32>(f32::NEGATIVE_INFINITY, 360, -0.0);
    test::<f32>(0.0, 360, 90.0);
    test::<f32>(-0.0, 360, -90.0);
    test::<f32>(0.0, 0, 0.0);
    test::<f32>(-0.0, 0, -0.0);
    test::<f32>(1.0, 360, 45.0);
    test::<f32>(1.0, 0, 0.0);
    test::<f32>(-1.0, 0, -0.0);
    test::<f32>(-1.0, 360, -45.0);
    test::<f32>(1.0, 8, 1.0);
    test::<f32>(0.5, 360, 63.434948);
    test::<f32>(-0.5, 360, -63.434948);
    test::<f32>(2.0, 360, 26.565052);
    test::<f32>(-2.0, 360, -26.565052);
    test::<f32>(2.0, 7, 0.5165427);
    test::<f32>(2.5, 360, 21.801409);
    test::<f32>(100.0, 360, 0.5729387);
    test::<f32>(1.0e30, 360, 5.729578e-29);
    test::<f32>(1.0e30, 1, 1.5915494e-31);
    test::<f32>(1.0, 18446744073709551615, 2.305843e18);
    test::<f64>(f64::NAN, 360, f64::NAN);
    test::<f64>(f64::INFINITY, 360, 0.0);
    test::<f64>(f64::NEGATIVE_INFINITY, 360, -0.0);
    test::<f64>(0.0, 360, 90.0);
    test::<f64>(-0.0, 360, -90.0);
    test::<f64>(0.0, 0, 0.0);
    test::<f64>(-0.0, 0, -0.0);
    test::<f64>(1.0, 360, 45.0);
    test::<f64>(1.0, 0, 0.0);
    test::<f64>(-1.0, 0, -0.0);
    test::<f64>(-1.0, 360, -45.0);
    test::<f64>(1.0, 8, 1.0);
    test::<f64>(0.5, 360, 63.43494882292201);
    test::<f64>(-0.5, 360, -63.43494882292201);
    test::<f64>(2.0, 360, 26.56505117707799);
    test::<f64>(-2.0, 360, -26.56505117707799);
    test::<f64>(2.0, 7, 0.5165426617765164);
    test::<f64>(2.5, 360, 21.80140948635181);
    test::<f64>(100.0, 360, 0.5729386976834859);
    test::<f64>(1.0e300, 360, 5.729577951308232e-299);
    test::<f64>(1.0e300, 1, 1.5915494309189532e-301);
    test::<f64>(1.0, 18446744073709551615, 2.305843009213694e18);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acot_with_period_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        let c = primitive_float_acot_with_period(x, u);
        // NaN exactly for a NaN input
        assert_eq!(c.is_nan(), x.is_nan());
        if !c.is_nan() {
            // the result lies in [-u/4, u/4], so it never overflows
            assert!(c.is_finite());
            // the same as the `Float` version taken with 64 bits to spare and rounded once -- but
            // only where the result is normal, a subnormal one being rounded twice here
            if c.is_normal() {
                let (c_float, _) =
                    Float::acot_with_period_prec(Float::from(x), u, T::MANTISSA_WIDTH + 64);
                assert_eq!(
                    NiceFloat(T::rounding_from(&c_float, Nearest).0),
                    NiceFloat(c)
                );
            }
        }
    });
}

#[test]
fn primitive_float_acot_with_period_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acot_with_period_properties_helper);
}
