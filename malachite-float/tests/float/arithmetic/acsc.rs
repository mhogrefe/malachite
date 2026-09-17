// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Acsc, AcscAssign, PowerOf2, Reciprocal};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, Two, Zero,
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
use malachite_float::float::arithmetic::acsc::{
    primitive_float_acsc, primitive_float_acsc_rational, primitive_float_acsc_with_period,
    primitive_float_acsc_with_period_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::acsc::{
    rug_acsc, rug_acsc_prec_round, rug_acsc_rational_prec_round, rug_acsc_with_period_prec_round,
    rug_acsc_with_period_rational_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_51, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_2, float_unsigned_rounding_mode_triple_gen_var_48,
    float_unsigned_rounding_mode_triple_gen_var_49, float_unsigned_rounding_mode_triple_gen_var_50,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_27,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_28,
    float_unsigned_unsigned_triple_gen_var_1, rational_unsigned_rounding_mode_triple_gen_var_13,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{
    rational_gen, rational_unsigned_pair_gen_var_1, rational_unsigned_pair_gen_var_3,
};
use std::panic::catch_unwind;
use std::str::FromStr;

// The arccosecant of x is the arcsine of its reciprocal, and a `Float`'s reciprocal is an exact
// `Rational`, so the `Rational` arcsine gives the same correctly rounded answer. That identity is
// the strongest check available here, MPFR having no arccosecant.
//
// It is only applied to inputs of moderate magnitude: the `Rational` holds the `Float`'s exponent
// in full, so a `Float` with an extreme exponent would turn into a `Rational` of hundreds of
// megabytes.
fn asin_of_reciprocal(x: &Float, prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
    if !x.is_finite() || *x == 0u32 || x.get_exponent().unwrap().unsigned_abs() > 1000 {
        return None;
    }
    Some(Float::asin_rational_prec_round(
        Rational::exact_from(x).reciprocal(),
        prec,
        rm,
    ))
}

#[test]
fn test_acsc_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().acsc_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.acsc_prec_round_ref(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.acsc_prec_round_assign(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = x.acsc_prec_ref(prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        // the arcsine of the exact reciprocal, which is the same real number
        if let Some((c_alt, o_alt)) = asin_of_reciprocal(&x, prec, rm) {
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acsc_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
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
    test("0.0", "0x0.0", 10, Exact, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 10, Nearest, "NaN", "NaN", Equal);
    test("1.0", "0x1.0#1", 10, Floor, "1.5703", "0x1.920#10", Less);
    test(
        "1.0",
        "0x1.0#1",
        10,
        Ceiling,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test("1.0", "0x1.0#1", 10, Nearest, "1.5703", "0x1.920#10", Less);
    test("1.0", "0x1.0#1", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Floor,
        "-1.5723",
        "-0x1.928#10",
        Less,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Ceiling,
        "-1.5703",
        "-0x1.920#10",
        Greater,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Nearest,
        "-1.5703",
        "-0x1.920#10",
        Greater,
    );
    test("2.0", "0x2.0#1", 10, Floor, "0.52344", "0x0.860#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "0.52441",
        "0x0.864#10",
        Greater,
    );
    test("2.0", "0x2.0#1", 10, Down, "0.52344", "0x0.860#10", Less);
    test("2.0", "0x2.0#1", 10, Up, "0.52441", "0x0.864#10", Greater);
    test("2.0", "0x2.0#1", 10, Nearest, "0.52344", "0x0.860#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        53,
        Nearest,
        "0.52359877559829893",
        "0x0.860a91c16b9b30#53",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        53,
        Nearest,
        "-0.52359877559829893",
        "-0x0.860a91c16b9b30#53",
        Less,
    );
    test(
        "2.0",
        "0x2.0#1",
        100,
        Nearest,
        "0.52359877559829887307710723054682",
        "0x0.860a91c16b9b2c232dd99707b#100",
        Greater,
    );
    test(
        "2.5",
        "0x2.8#3",
        10,
        Nearest,
        "0.41162",
        "0x0.696#10",
        Greater,
    );
    test(
        "2.5",
        "0x2.8#3",
        53,
        Nearest,
        "0.41151684606748801",
        "0x0.69592b039ce8cc#53",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        20,
        Nearest,
        "0.72972775",
        "0x0.bacf7#20",
        Greater,
    );
    test(
        "-1.5",
        "-0x1.8#2",
        20,
        Nearest,
        "-0.72972775",
        "-0x0.bacf7#20",
        Less,
    );
    test(
        "100.0",
        "0x64.0#7",
        20,
        Nearest,
        "0.010000169",
        "0x0.028f5f0#20",
        Greater,
    );
    test(
        "-100.0",
        "-0x64.0#7",
        20,
        Nearest,
        "-0.010000169",
        "-0x0.028f5f0#20",
        Less,
    );
    test(
        "1.0000000000000002",
        "0x1.0000000000001#53",
        53,
        Nearest,
        "1.5707963057214724",
        "0x1.921fb4e9c057f#53",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        20,
        Nearest,
        "7.8886091e-31",
        "0x1.00000E-25#20",
        Less,
    );
    test(
        "-1.3e30",
        "-0x1.0E+25#1",
        20,
        Nearest,
        "-7.8886091e-31",
        "-0x1.00000E-25#20",
        Greater,
    );
}

#[test]
#[should_panic]
fn acsc_prec_round_fail_1() {
    Float::TWO.acsc_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn acsc_prec_round_fail_2() {
    // acsc(2) = pi/6 is not exactly representable
    Float::TWO.acsc_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acsc_prec_round_fail_3() {
    // acsc(1) = pi/2 is not exactly representable either
    Float::ONE.acsc_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acsc_prec_round_ref_fail() {
    Float::TWO.acsc_prec_round_ref(0, Floor);
}

#[test]
#[should_panic]
fn acsc_prec_fail() {
    Float::TWO.acsc_prec(0);
}

#[test]
#[should_panic]
fn acsc_round_fail() {
    Float::TWO.acsc_round(Exact);
}

// Whether acsc(x) is exactly representable at `prec`: at a NaN, at an input inside (-1, 1) (where
// the result is NaN), and at either infinity, where it is a zero. Unlike the arcsecant, |x| = 1 is
// not an exact case, pi/2 never being representable.
fn acsc_exact(x: &Float) -> bool {
    x.is_nan() || x.lt_abs(&1u32) || !x.is_finite()
}

#[allow(clippy::needless_pass_by_value)]
fn acsc_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    if rm == Exact && !acsc_exact(&x) {
        assert_panic!(x.acsc_prec_round_ref(prec, Exact));
        return;
    }
    let (c, o) = x.clone().acsc_prec_round(prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.acsc_prec_round_ref(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut c_alt = x.clone();
    let o_alt = c_alt.acsc_prec_round_assign(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // the arcsine of the exact reciprocal, which is the same real number
    if let Some((c_alt, o_alt)) = asin_of_reciprocal(&x, prec, rm) {
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
        let (rug_c, rug_o) = rug_acsc_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o, "x = {x} prec = {prec} rm = {rm:?}");
    }

    // NaN exactly for a NaN input or one inside (-1, 1)
    assert_eq!(c.is_nan(), x.is_nan() || x.lt_abs(&1u32));
    if !c.is_nan() {
        // |acsc(x)| <= pi/2, so the result never overflows, and it never underflows either
        assert!(c.is_finite());
        // |acsc(x)| <= pi/2 < 2, and rounding at any precision keeps it at most 2
        assert!(c <= 2u32);
        assert!(c >= -2i32);
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
    }
    // the arccosecant is odd
    let (c_neg, o_neg) = (-&x).acsc_prec_round(prec, -rm);
    assert_eq!(ComparableFloat(-c_neg), ComparableFloat(c.clone()));
    assert_eq!(o_neg.reverse(), o);

    if o == Equal {
        assert!(acsc_exact(&x));
        for rm in exhaustive_rounding_modes() {
            let (c2, oo) = x.acsc_prec_round_ref(prec, rm);
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.acsc_prec_round_ref(prec, Exact));
    }
}

#[test]
fn acsc_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_48().test_properties(|(x, prec, rm)| {
        acsc_prec_round_properties_helper(x, prec, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_49().test_properties(|(x, prec, rm)| {
        acsc_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // acsc(NaN) = NaN, and so is acsc(x) for |x| < 1
        for x in [Float::NAN, Float::ZERO, Float::NEGATIVE_ZERO, Float::ONE_HALF] {
            let (c, o) = x.acsc_prec_round(prec, rm);
            assert!(c.is_nan());
            assert_eq!(o, Equal);
        }
        // acsc(infinity) = +0 and acsc(-infinity) = -0, exactly
        let (c, o) = Float::INFINITY.acsc_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        let (c, o) = Float::NEGATIVE_INFINITY.acsc_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
        assert_eq!(o, Equal);
        // acsc(1) = pi/2 and acsc(-1) = -pi/2, neither of them representable, so `Exact` panics
        // rather than returning anything -- and `pi_prec_round` itself panics under `Exact`, so it
        // cannot be used to state the expectation either
        if rm == Exact {
            assert_panic!(Float::ONE.acsc_prec_round(prec, Exact));
            assert_panic!(Float::NEGATIVE_ONE.acsc_prec_round(prec, Exact));
        } else {
            let (p, o_p) = Float::pi_prec_round(prec, rm);
            let (c, o) = Float::ONE.acsc_prec_round(prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(p >> 1u32));
            assert_eq!(o, o_p);
            let (p, o_p) = Float::pi_prec_round(prec, -rm);
            let (c, o) = Float::NEGATIVE_ONE.acsc_prec_round(prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(-(p >> 1u32)));
            assert_eq!(o, o_p.reverse());
        }
    });
}

#[test]
fn acsc_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (c, o) = x.clone().acsc_prec(prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.acsc_prec_ref(prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acsc_prec_assign(prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acsc_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn acsc_round_properties() {
    float_rounding_mode_pair_gen_var_51().test_properties(|(x, rm)| {
        let (c, o) = x.clone().acsc_round(rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.acsc_round_ref(rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acsc_round_assign(rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acsc_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn acsc_properties() {
    float_gen().test_properties(|x| {
        let c = x.clone().acsc();
        assert!(c.is_valid());
        let c_alt = (&x).acsc();
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let mut c_alt = x.clone();
        c_alt.acsc_assign();
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        // the same as rounding to the input's precision, to nearest
        let (c_alt, _) = x.acsc_prec_ref(x.significant_bits());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        if x.is_finite() && x != 0u32 && x.get_exponent().unwrap().unsigned_abs() <= 1000 {
            let c_alt = Float::from(&rug_acsc(&rug::Float::exact_from(&x)));
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        }
    });
}

#[test]
fn test_acsc_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::acsc_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = Float::acsc_rational_prec_round_ref(&x, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = Float::acsc_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
            let (c_alt, o_alt) = Float::acsc_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        // the arcsine of the exact reciprocal, which is the same real number
        if x != 0u32 {
            let (c_alt, o_alt) = Float::asin_rational_prec_round(x.clone().reciprocal(), prec, rm);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acsc_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 10, Exact, "NaN", "NaN", Equal);
    test("1/2", 10, Exact, "NaN", "NaN", Equal);
    test("-1/2", 10, Nearest, "NaN", "NaN", Equal);
    test("1", 10, Floor, "1.5703", "0x1.920#10", Less);
    test("1", 10, Ceiling, "1.5723", "0x1.928#10", Greater);
    test("1", 10, Nearest, "1.5703", "0x1.920#10", Less);
    test("-1", 10, Nearest, "-1.5703", "-0x1.920#10", Greater);
    test("2", 10, Floor, "0.52344", "0x0.860#10", Less);
    test("2", 10, Ceiling, "0.52441", "0x0.864#10", Greater);
    test(
        "2",
        53,
        Nearest,
        "0.52359877559829893",
        "0x0.860a91c16b9b30#53",
        Greater,
    );
    test(
        "-2",
        53,
        Nearest,
        "-0.52359877559829893",
        "-0x0.860a91c16b9b30#53",
        Less,
    );
    test("5/3", 10, Floor, "0.64258", "0x0.a48#10", Less);
    test("5/3", 10, Ceiling, "0.64355", "0x0.a4c#10", Greater);
    test(
        "5/3",
        53,
        Nearest,
        "0.64350110879328437",
        "0x0.a4bc7d1934f708#53",
        Less,
    );
    test(
        "-5/3",
        53,
        Nearest,
        "-0.64350110879328437",
        "-0x0.a4bc7d1934f708#53",
        Greater,
    );
    test("100/99", 20, Nearest, "1.4292564", "0x1.6de3c#20", Less);
    test("100", 20, Nearest, "0.010000169", "0x0.028f5f0#20", Greater);
    test("-100", 20, Nearest, "-0.010000169", "-0x0.028f5f0#20", Less);
    test(
        "100000000000000000000000000000000000000000",
        20,
        Nearest,
        "1.0000005e-41",
        "0xd.f01fE-35#20",
        Greater,
    );
}

#[test]
#[should_panic]
fn acsc_rational_prec_round_fail_1() {
    Float::acsc_rational_prec_round(Rational::TWO, 0, Floor);
}

#[test]
#[should_panic]
fn acsc_rational_prec_round_fail_2() {
    // acsc(2) = pi/6 is not exactly representable
    Float::acsc_rational_prec_round(Rational::TWO, 10, Exact);
}

#[test]
#[should_panic]
fn acsc_rational_prec_round_fail_3() {
    // acsc(1) = pi/2 is not exactly representable either
    Float::acsc_rational_prec_round(Rational::ONE, 10, Exact);
}

#[test]
#[should_panic]
fn acsc_rational_prec_fail() {
    Float::acsc_rational_prec(Rational::TWO, 0);
}

// A `Rational` large enough that acsc(x), about 1/x, falls below the smallest positive `Float`.
// This is the case the scaled path exists for, and one the `Float` arccosecant cannot reach, a
// `Float`'s exponent being bounded. The `Rational` has about 2^30 bits, so each call costs a few
// seconds.
#[test]
fn test_acsc_rational_underflow() {
    let x = Rational::power_of_2((1i64 << 30) + 2);
    let (c, o) = Float::acsc_rational_prec_round_ref(&x, 53, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    let (c, o) = Float::acsc_rational_prec_round_ref(&x, 53, Ceiling);
    assert_eq!(
        ComparableFloat(c),
        ComparableFloat(Float::min_positive_value_prec(53))
    );
    assert_eq!(o, Greater);
    let (c, o) = Float::acsc_rational_prec_round_ref(&x, 53, Floor);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    // the arccosecant is odd, so a negative input underflows to the other side
    let x = -x;
    let (c, o) = Float::acsc_rational_prec_round_ref(&x, 53, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    let (c, o) = Float::acsc_rational_prec_round_ref(&x, 53, Floor);
    assert_eq!(
        ComparableFloat(c),
        ComparableFloat(-Float::min_positive_value_prec(53))
    );
    assert_eq!(o, Less);
}

#[allow(clippy::needless_pass_by_value)]
fn acsc_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    if rm == Exact && !x.lt_abs(&1u32) {
        assert_panic!(Float::acsc_rational_prec_round_ref(&x, prec, Exact));
        return;
    }
    let (c, o) = Float::acsc_rational_prec_round(x.clone(), prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = Float::acsc_rational_prec_round_ref(&x, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // the arcsine of the exact reciprocal, which is the same real number
    if x != 0u32 {
        let (c_alt, o_alt) = Float::asin_rational_prec_round(x.clone().reciprocal(), prec, rm);
        assert_eq!(
            ComparableFloatRef(&c_alt),
            ComparableFloatRef(&c),
            "x = {x} exp = {} prec = {prec} rm = {rm:?}",
            x.floor_log_base_2_abs() + 1
        );
        assert_eq!(o_alt, o, "x = {x} prec = {prec} rm = {rm:?}");
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_acsc_rational_prec_round(&x, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o, "x = {x} prec = {prec} rm = {rm:?}");
    }

    // NaN exactly for an input inside (-1, 1)
    assert_eq!(c.is_nan(), x.lt_abs(&1u32));
    if !c.is_nan() {
        // |acsc(x)| <= pi/2, so the result never overflows
        assert!(c.is_finite());
        // |acsc(x)| <= pi/2 < 2, and rounding at any precision keeps it at most 2
        assert!(c <= 2u32);
        assert!(c >= -2i32);
    }
    // a `Float` input agrees with the `Float` version
    if let Ok(f) = Float::try_from(&x) {
        let (c_alt, o_alt) = f.acsc_prec_round(prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }

    if o == Equal {
        assert!(x.lt_abs(&1u32));
    } else {
        assert_panic!(Float::acsc_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn acsc_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_13().test_properties(|(x, prec, rm)| {
        acsc_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // acsc(x) = NaN for |x| < 1, in every rounding mode
        for x in [Rational::ZERO, Rational::ONE_HALF, -Rational::ONE_HALF] {
            let (c, o) = Float::acsc_rational_prec_round(x, prec, rm);
            assert!(c.is_nan());
            assert_eq!(o, Equal);
        }
        // acsc(1) = pi/2 and acsc(-1) = -pi/2, neither of them representable, so `Exact` panics
        if rm == Exact {
            assert_panic!(Float::acsc_rational_prec_round(Rational::ONE, prec, Exact));
            assert_panic!(Float::acsc_rational_prec_round(
                Rational::NEGATIVE_ONE,
                prec,
                Exact
            ));
        } else {
            let (p, o_p) = Float::pi_prec_round(prec, rm);
            let (c, o) = Float::acsc_rational_prec_round(Rational::ONE, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(p >> 1u32));
            assert_eq!(o, o_p);
            let (p, o_p) = Float::pi_prec_round(prec, -rm);
            let (c, o) = Float::acsc_rational_prec_round(Rational::NEGATIVE_ONE, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(-(p >> 1u32)));
            assert_eq!(o, o_p.reverse());
        }
    });
}

#[test]
fn acsc_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (c, o) = Float::acsc_rational_prec(x.clone(), prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = Float::acsc_rational_prec_ref(&x, prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = Float::acsc_rational_prec_round_ref(&x, prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
#[allow(clippy::approx_constant, clippy::type_repetition_in_bounds)]
fn test_primitive_float_acsc() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_acsc(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, 0.0);
    test::<f32>(f32::NEGATIVE_INFINITY, -0.0);
    test::<f32>(0.0, f32::NAN);
    test::<f32>(-0.0, f32::NAN);
    test::<f32>(0.5, f32::NAN);
    test::<f32>(1.0, 1.5707964);
    test::<f32>(-1.0, -1.5707964);
    test::<f32>(2.0, 0.5235988);
    test::<f32>(-2.0, -0.5235988);
    test::<f32>(2.5, 0.41151685);
    test::<f32>(1.5, 0.7297277);
    test::<f32>(100.0, 0.0100001665);
    test::<f32>(1.0e30, 1.0e-30);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, 0.0);
    test::<f64>(f64::NEGATIVE_INFINITY, -0.0);
    test::<f64>(0.0, f64::NAN);
    test::<f64>(-0.0, f64::NAN);
    test::<f64>(0.5, f64::NAN);
    test::<f64>(1.0, 1.5707963267948966);
    test::<f64>(-1.0, -1.5707963267948966);
    test::<f64>(2.0, 0.5235987755982989);
    test::<f64>(-2.0, -0.5235987755982989);
    test::<f64>(2.5, 0.411516846067488);
    test::<f64>(1.5, 0.7297276562269663);
    test::<f64>(100.0, 0.010000166674167112);
    test::<f64>(1.0e300, 1.0e-300);
}

#[test]
#[allow(clippy::approx_constant, clippy::type_repetition_in_bounds)]
fn test_primitive_float_acsc_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_acsc_rational::<T>(
                &Rational::from_str(s).unwrap()
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", f32::NAN);
    test::<f32>("1/2", f32::NAN);
    test::<f32>("1", 1.5707964);
    test::<f32>("-1", -1.5707964);
    test::<f32>("2", 0.5235988);
    test::<f32>("-2", -0.5235988);
    test::<f32>("5/3", 0.6435011);
    test::<f32>("-5/3", -0.6435011);
    test::<f32>("100/99", 1.4292568);
    test::<f32>("100", 0.0100001665);
    test::<f64>("0", f64::NAN);
    test::<f64>("1/2", f64::NAN);
    test::<f64>("1", 1.5707963267948966);
    test::<f64>("-1", -1.5707963267948966);
    test::<f64>("2", 0.5235987755982989);
    test::<f64>("-2", -0.5235987755982989);
    test::<f64>("5/3", 0.6435011087932844);
    test::<f64>("-5/3", -0.6435011087932844);
    test::<f64>("100/99", 1.4292568534704695);
    test::<f64>("100", 0.010000166674167112);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acsc_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let c = primitive_float_acsc(x);
        // NaN exactly for a NaN input or one inside (-1, 1)
        assert_eq!(c.is_nan(), x.is_nan() || x.abs() < T::ONE);
        if !c.is_nan() {
            // the result lies in [-pi/2, pi/2], so it never overflows
            assert!(c.is_finite());
            // the same as the `Float` version taken with 64 bits to spare and rounded once -- but
            // only where the result is normal, a subnormal one being rounded twice here
            if c.is_normal() {
                let (c_float, _) = Float::acsc_prec(Float::from(x), T::MANTISSA_WIDTH + 64);
                assert_eq!(
                    NiceFloat(T::rounding_from(&c_float, Nearest).0),
                    NiceFloat(c)
                );
            }
        }
    });
}

#[test]
fn primitive_float_acsc_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acsc_properties_helper);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acsc_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let c = primitive_float_acsc_rational::<T>(&x);
        // NaN exactly for an input inside (-1, 1)
        assert_eq!(c.is_nan(), x.lt_abs(&1u32));
        if !c.is_nan() {
            assert!(c.is_finite());
            // the same as the `Float` version taken with 64 bits to spare and rounded once -- but
            // only where the result is normal, a subnormal one being rounded twice here and once by
            // `emulate_rational_to_float_fn`
            if c.is_normal() {
                let (c_float, _) = Float::acsc_rational_prec_ref(&x, T::MANTISSA_WIDTH + 64);
                assert_eq!(
                    NiceFloat(T::rounding_from(&c_float, Nearest).0),
                    NiceFloat(c)
                );
            }
        }
    });
}

#[test]
fn primitive_float_acsc_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acsc_rational_properties_helper);
}

// The arccosecant in u-ths of a turn is the arcsine of the reciprocal in the same units, and a
// `Float`'s reciprocal is an exact `Rational`. As for the plain arccosecant, the check is confined
// to inputs of moderate magnitude, since the `Rational` holds the exponent in full.
fn asinu_of_reciprocal(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    if !x.is_finite() || *x == 0u32 || x.get_exponent().unwrap().unsigned_abs() > 1000 {
        return None;
    }
    Some(Float::asin_with_period_rational_prec_round(
        Rational::exact_from(x).reciprocal(),
        u,
        prec,
        rm,
    ))
}

// Whether MPFR and Malachite are expected to disagree on acscu(x, u): for a zero period MPFR's
// `asinu` returns +0 even for a negative argument, so its arcsine of the reciprocal of a negative x
// is +0, while Malachite keeps the sign, the function being odd. An input of extreme exponent is
// also kept away from the oracle: its quotient underflows here and not in MPFR's wider range.
fn mpfr_divergence(x: &Float, u: u64) -> bool {
    (u == 0 && *x < 0u32 && !x.lt_abs(&1u32))
        || (x.is_finite() && x != &0u32 && x.get_exponent().unwrap().unsigned_abs() > 1000)
}

#[test]
fn test_acsc_with_period_prec_round() {
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

        let (c, o) = x.clone().acsc_with_period_prec_round(u, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.acsc_with_period_prec_round_ref(u, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.acsc_with_period_prec_round_assign(u, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = x.acsc_with_period_prec_ref(u, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        // the arcsine of the exact reciprocal, in the same units
        if let Some((c_alt, o_alt)) = asinu_of_reciprocal(&x, u, prec, rm) {
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u <= u64::from(u32::MAX)
            && !mpfr_divergence(&x, u)
        {
            let (rug_c, rug_o) =
                rug_acsc_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
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
    test("0.0", "0x0.0", 360, 10, Exact, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", 360, 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 360, 10, Nearest, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 0, 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1.0#1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 0, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("2.5", "0x2.8#3", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test(
        "1.0",
        "0x1.0#1",
        360,
        10,
        Exact,
        "90.000",
        "0x5a.0#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        360,
        10,
        Exact,
        "-90.000",
        "-0x5a.0#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        12,
        10,
        Exact,
        "3.0000",
        "0x3.00#10",
        Equal,
    );
    test(
        "1.0",
        "0x1.0#1",
        7,
        10,
        Exact,
        "1.7500",
        "0x1.c00#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        7,
        10,
        Exact,
        "-1.7500",
        "-0x1.c00#10",
        Equal,
    );
    test("1.0", "0x1.0#1", 7, 1, Floor, "1.0", "0x1.0#1", Less);
    test("1.0", "0x1.0#1", 7, 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1.0", "0x1.0#1", 7, 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("-1.0", "-0x1.0#1", 7, 1, Floor, "-2.0", "-0x2.0#1", Less);
    test(
        "-1.0", "-0x1.0#1", 7, 1, Ceiling, "-1.0", "-0x1.0#1", Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Exact,
        "30.000",
        "0x1e.00#10",
        Equal,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        360,
        10,
        Exact,
        "-30.000",
        "-0x1e.00#10",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        12,
        10,
        Exact,
        "1.0000",
        "0x1.000#10",
        Equal,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        12,
        10,
        Exact,
        "-1.0000",
        "-0x1.000#10",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        3,
        10,
        Exact,
        "0.25000",
        "0x0.400#10",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        7,
        10,
        Nearest,
        "0.58301",
        "0x0.954#10",
        Less,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        7,
        10,
        Nearest,
        "-0.58301",
        "-0x0.954#10",
        Greater,
    );
    test(
        "2.5",
        "0x2.8#3",
        360,
        10,
        Floor,
        "23.562",
        "0x17.90#10",
        Less,
    );
    test(
        "2.5",
        "0x2.8#3",
        360,
        10,
        Ceiling,
        "23.594",
        "0x17.98#10",
        Greater,
    );
    test(
        "2.5",
        "0x2.8#3",
        360,
        10,
        Down,
        "23.562",
        "0x17.90#10",
        Less,
    );
    test(
        "2.5",
        "0x2.8#3",
        360,
        10,
        Up,
        "23.594",
        "0x17.98#10",
        Greater,
    );
    test(
        "2.5",
        "0x2.8#3",
        360,
        10,
        Nearest,
        "23.594",
        "0x17.98#10",
        Greater,
    );
    test(
        "-2.5",
        "-0x2.8#3",
        360,
        10,
        Nearest,
        "-23.594",
        "-0x17.98#10",
        Less,
    );
    test(
        "2.5",
        "0x2.8#3",
        360,
        100,
        Nearest,
        "23.578178478201831104022499419824",
        "0x17.9403813720bef0116b2a86fe#100",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        360,
        20,
        Nearest,
        "41.810303",
        "0x29.cf70#20",
        Less,
    );
    test(
        "-1.5",
        "-0x1.8#2",
        360,
        20,
        Nearest,
        "-41.810303",
        "-0x29.cf70#20",
        Greater,
    );
    test(
        "100.0",
        "0x64.0#7",
        360,
        20,
        Nearest,
        "0.57296753",
        "0x0.92ae0#20",
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
        "4611686018427387903.75",
        "0x3fffffffffffffff.c#64",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        18446744073709551615,
        64,
        Exact,
        "1537228672809129301.25",
        "0x1555555555555555.4#64",
        Equal,
    );
    test(
        "2.5",
        "0x2.8#3",
        18446744073709551615,
        20,
        Nearest,
        "1.2081676e18",
        "0x1.0c446E+15#20",
        Less,
    );
}

#[test]
#[should_panic]
fn acsc_with_period_prec_round_fail_1() {
    Float::TWO.acsc_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn acsc_with_period_prec_round_fail_2() {
    // acsc(5/2) is not an exact number of sevenths of a turn
    Float::from(2.5).acsc_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn acsc_with_period_prec_round_fail_3() {
    // acsc(2) is a twelfth of a turn, but 7 is not a multiple of 3
    Float::TWO.acsc_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn acsc_with_period_prec_round_fail_4() {
    // a quarter turn needs more than 2 bits when u = 7
    Float::ONE.acsc_with_period_prec_round(7, 2, Exact);
}

#[test]
#[should_panic]
fn acsc_with_period_prec_round_ref_fail() {
    Float::TWO.acsc_with_period_prec_round_ref(7, 0, Floor);
}

#[test]
#[should_panic]
fn acsc_with_period_prec_fail() {
    Float::TWO.acsc_with_period_prec(7, 0);
}

#[test]
#[should_panic]
fn acsc_with_period_round_fail() {
    Float::from(2.5).acsc_with_period_round(7, Exact);
}

// The largest `Float`s put acsc(x), about 1/x, only twice above the smallest positive `Float`, so
// dividing by 2 pi with a period of 1 falls below it -- to about 0.64 of it, which `Nearest` still
// rounds up to it, and `Floor` down to zero. This is the underflow the plain arccosecant cannot
// reach, and a larger period lifts the quotient back into the range.
#[test]
fn test_acsc_with_period_underflow() {
    let x = parse_hex_string("0x4.0E+268435455#1");
    let min = Float::min_positive_value_prec(20);
    let (c, o) = x.acsc_with_period_prec_round_ref(1, 20, Nearest);
    assert_eq!(ComparableFloatRef(&c), ComparableFloatRef(&min));
    assert_eq!(o, Greater);
    let (c, o) = x.acsc_with_period_prec_round_ref(1, 20, Floor);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    let (c, o) = x.acsc_with_period_prec_round_ref(1, 20, Ceiling);
    assert_eq!(ComparableFloatRef(&c), ComparableFloatRef(&min));
    assert_eq!(o, Greater);
    // a period of 8 lifts the quotient back into the range, and doubling it doubles the result
    let (c, o) = x.acsc_with_period_prec_round_ref(8, 20, Nearest);
    assert!(c > min);
    assert_ne!(o, Equal);
    let (c_alt, o_alt) = x.acsc_with_period_prec_round_ref(16, 20, Nearest);
    assert_eq!(ComparableFloat(c_alt), ComparableFloat(c << 1u32));
    assert_eq!(o_alt, o);
    // the arccosecant is odd, so a negative input underflows to the other side
    let x = -x;
    let (c, o) = x.acsc_with_period_prec_round_ref(1, 20, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(-&min));
    assert_eq!(o, Less);
    let (c, o) = x.acsc_with_period_prec_round_ref(1, 20, Ceiling);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    let (c, o) = x.acsc_with_period_prec_round_ref(1, 20, Floor);
    assert_eq!(ComparableFloat(c), ComparableFloat(-min));
    assert_eq!(o, Less);
}

// Whether acscu(x, u) is exactly representable at `prec`: at a NaN, at an input inside (-1, 1)
// (where the result is NaN), at either infinity, at u = 0, and at the turn fractions -- a quarter
// at |x| = 1 and a twelfth at |x| = 2 with u a multiple of 3 -- each of which needs a `prec` wide
// enough to hold it.
fn acsc_with_period_exact(x: &Float, u: u64, prec: u64) -> bool {
    x.is_nan()
        || x.lt_abs(&1u32)
        || !x.is_finite()
        || u == 0
        || (x.eq_abs(&Float::ONE) && Float::from_unsigned_prec(u, prec).1 == Equal)
        || (x.eq_abs(&Float::TWO)
            && u.is_multiple_of(3)
            && Float::from_unsigned_prec(u / 3, prec).1 == Equal)
}

#[allow(clippy::needless_pass_by_value)]
fn acsc_with_period_prec_round_properties_helper(x: Float, u: u64, prec: u64, rm: RoundingMode) {
    if rm == Exact && !acsc_with_period_exact(&x, u, prec) {
        assert_panic!(x.acsc_with_period_prec_round_ref(u, prec, Exact));
        return;
    }
    let (c, o) = x.clone().acsc_with_period_prec_round(u, prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.acsc_with_period_prec_round_ref(u, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut c_alt = x.clone();
    let o_alt = c_alt.acsc_with_period_prec_round_assign(u, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // the arcsine of the exact reciprocal, in the same units
    if let Some((c_alt, o_alt)) = asinu_of_reciprocal(&x, u, prec, rm) {
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o, "x = {x} u = {u} prec = {prec} rm = {rm:?}");
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && u <= u64::from(u32::MAX)
        && !mpfr_divergence(&x, u)
    {
        let (rug_c, rug_o) =
            rug_acsc_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o, "x = {x} u = {u} prec = {prec} rm = {rm:?}");
    }

    // NaN exactly for a NaN input or one inside (-1, 1)
    assert_eq!(c.is_nan(), x.is_nan() || x.lt_abs(&1u32));
    if !c.is_nan() {
        // |acscu(x, u)| <= u/4, a quarter turn, so the result never overflows
        assert!(c.is_finite());
        let bound = Float::from_unsigned_prec_round(u, prec, Ceiling).0 >> 2u32;
        assert!(c <= bound);
        assert!(c >= -bound);
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
    }
    // the arccosecant is odd
    let (c_neg, o_neg) = (-&x).acsc_with_period_prec_round(u, prec, -rm);
    assert_eq!(ComparableFloat(-c_neg), ComparableFloat(c.clone()));
    assert_eq!(o_neg.reverse(), o);

    if o == Equal {
        assert!(acsc_with_period_exact(&x, u, prec));
        for rm in exhaustive_rounding_modes() {
            let (c2, oo) = x.acsc_with_period_prec_round_ref(u, prec, rm);
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.acsc_with_period_prec_round_ref(u, prec, Exact));
    }
}

#[test]
fn acsc_with_period_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_27().test_properties(
        |(x, u, prec, rm)| {
            acsc_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_28().test_properties(
        |(x, u, prec, rm)| {
            acsc_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // acscu(NaN, u) = NaN, and so is acscu(x, u) for |x| < 1, even when u is zero
        for x in [Float::NAN, Float::ZERO, Float::NEGATIVE_ZERO, Float::ONE_HALF] {
            for u in [0, 4] {
                let (c, o) = x.clone().acsc_with_period_prec_round(u, prec, rm);
                assert!(c.is_nan());
                assert_eq!(o, Equal);
            }
        }
        // acscu(±infinity, u) = ±0, exactly, for every period
        for u in [0, 4] {
            let (c, o) = Float::INFINITY.acsc_with_period_prec_round(u, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
            assert_eq!(o, Equal);
            let (c, o) = Float::NEGATIVE_INFINITY.acsc_with_period_prec_round(u, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
            assert_eq!(o, Equal);
        }
        // acscu(x, 0) = 0 with the sign of x, exactly
        for x in [Float::ONE, Float::TWO, Float::from(2.5)] {
            let (c, o) = x.acsc_with_period_prec_round_ref(0, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
            assert_eq!(o, Equal);
            let (c, o) = (-x).acsc_with_period_prec_round(0, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
            assert_eq!(o, Equal);
        }
        // acscu(±1, u) = ±u/4, a quarter turn, exact when `prec` holds it
        let (q, o_q) = Float::from_unsigned_prec_round(8u32, prec, rm);
        let (c, o) = Float::ONE.acsc_with_period_prec_round(8, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(q >> 2u32));
        assert_eq!(o, o_q);
        let (q, o_q) = Float::from_unsigned_prec_round(8u32, prec, -rm);
        let (c, o) = Float::NEGATIVE_ONE.acsc_with_period_prec_round(8, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(-(q >> 2u32)));
        assert_eq!(o, o_q.reverse());
        // acscu(±2, u) = ±u/12 when u is a multiple of 3
        let (q, o_q) = Float::from_unsigned_prec_round(4u32, prec, rm);
        let (c, o) = Float::TWO.acsc_with_period_prec_round(12, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(q >> 2u32));
        assert_eq!(o, o_q);
        let (q, o_q) = Float::from_unsigned_prec_round(4u32, prec, -rm);
        let (c, o) = (-Float::TWO).acsc_with_period_prec_round(12, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(-(q >> 2u32)));
        assert_eq!(o, o_q.reverse());
    });
}

#[test]
fn acsc_with_period_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, u, prec)| {
        let (c, o) = x.clone().acsc_with_period_prec(u, prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.acsc_with_period_prec_ref(u, prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acsc_with_period_prec_assign(u, prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acsc_with_period_prec_round_ref(u, prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn acsc_with_period_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_50().test_properties(|(x, u, rm)| {
        let (c, o) = x.clone().acsc_with_period_round(u, rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.acsc_with_period_round_ref(u, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acsc_with_period_round_assign(u, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acsc_with_period_prec_round_ref(u, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn acsc_with_period_properties() {
    float_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, u)| {
        let c = x.clone().acsc_with_period(u);
        assert!(c.is_valid());
        let c_alt = x.acsc_with_period_ref(u);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let mut c_alt = x.clone();
        c_alt.acsc_with_period_assign(u);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        // the same as rounding to the input's precision, to nearest
        let (c_alt, _) = x.acsc_with_period_prec_ref(u, x.significant_bits());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    });
}

#[test]
#[allow(clippy::approx_constant, clippy::type_repetition_in_bounds)]
fn test_primitive_float_acsc_with_period() {
    fn test<T: PrimitiveFloat>(x: T, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_acsc_with_period(x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 360, f32::NAN);
    test::<f32>(f32::INFINITY, 360, 0.0);
    test::<f32>(f32::NEGATIVE_INFINITY, 360, -0.0);
    test::<f32>(0.0, 360, f32::NAN);
    test::<f32>(0.5, 360, f32::NAN);
    test::<f32>(0.5, 0, f32::NAN);
    test::<f32>(1.0, 360, 90.0);
    test::<f32>(1.0, 0, 0.0);
    test::<f32>(-1.0, 0, -0.0);
    test::<f32>(-1.0, 360, -90.0);
    test::<f32>(2.0, 360, 30.0);
    test::<f32>(-2.0, 360, -30.0);
    test::<f32>(2.0, 12, 1.0);
    test::<f32>(-2.0, 12, -1.0);
    test::<f32>(2.0, 7, 0.5833333);
    test::<f32>(2.5, 360, 23.578178);
    test::<f32>(-2.5, 360, -23.578178);
    test::<f32>(1.5, 360, 41.810314);
    test::<f32>(100.0, 360, 0.57296735);
    test::<f32>(1.0e30, 360, 5.729578e-29);
    test::<f32>(1.0e30, 1, 1.5915494e-31);
    test::<f32>(1.0, 18446744073709551615, 4.611686e18);
    test::<f64>(f64::NAN, 360, f64::NAN);
    test::<f64>(f64::INFINITY, 360, 0.0);
    test::<f64>(f64::NEGATIVE_INFINITY, 360, -0.0);
    test::<f64>(0.0, 360, f64::NAN);
    test::<f64>(0.5, 360, f64::NAN);
    test::<f64>(0.5, 0, f64::NAN);
    test::<f64>(1.0, 360, 90.0);
    test::<f64>(1.0, 0, 0.0);
    test::<f64>(-1.0, 0, -0.0);
    test::<f64>(-1.0, 360, -90.0);
    test::<f64>(2.0, 360, 30.0);
    test::<f64>(-2.0, 360, -30.0);
    test::<f64>(2.0, 12, 1.0);
    test::<f64>(-2.0, 12, -1.0);
    test::<f64>(2.0, 7, 0.5833333333333334);
    test::<f64>(2.5, 360, 23.57817847820183);
    test::<f64>(-2.5, 360, -23.57817847820183);
    test::<f64>(1.5, 360, 41.810314895778596);
    test::<f64>(100.0, 360, 0.5729673448571526);
    test::<f64>(1.0e300, 360, 5.729577951308232e-299);
    test::<f64>(1.0e300, 1, 1.5915494309189532e-301);
    test::<f64>(1.0, 18446744073709551615, 4.611686018427388e18);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acsc_with_period_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        let c = primitive_float_acsc_with_period(x, u);
        // NaN exactly for a NaN input or one inside (-1, 1)
        assert_eq!(c.is_nan(), x.is_nan() || x.abs() < T::ONE);
        if !c.is_nan() {
            // the result lies in [-u/4, u/4], so it never overflows
            assert!(c.is_finite());
            // the same as the `Float` version taken with 64 bits to spare and rounded once -- but
            // only where the result is normal, a subnormal one being rounded twice here
            if c.is_normal() {
                let (c_float, _) =
                    Float::acsc_with_period_prec(Float::from(x), u, T::MANTISSA_WIDTH + 64);
                assert_eq!(
                    NiceFloat(T::rounding_from(&c_float, Nearest).0),
                    NiceFloat(c)
                );
            }
        }
    });
}

#[test]
fn primitive_float_acsc_with_period_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acsc_with_period_properties_helper);
}

// As `mpfr_divergence`, for a `Rational` input: only the signed zero at a zero period, a `Rational`
// having no exponent to be extreme in.
fn mpfr_rational_divergence(x: &Rational, u: u64) -> bool {
    u == 0 && *x < 0u32 && !x.lt_abs(&1u32)
}

#[test]
fn test_acsc_with_period_rational_prec_round() {
    let test = |s: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::acsc_with_period_rational_prec_round(x.clone(), u, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = Float::acsc_with_period_rational_prec_round_ref(&x, u, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = Float::acsc_with_period_rational_prec(x.clone(), u, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
            let (c_alt, o_alt) = Float::acsc_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        // the arcsine of the exact reciprocal, in the same units: the same real number
        if x != 0u32 {
            let (c_alt, o_alt) =
                Float::asin_with_period_rational_prec_round(x.clone().reciprocal(), u, prec, rm);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if !c.is_nan()
            && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u <= u64::from(u32::MAX)
            && !mpfr_rational_divergence(&x, u)
        {
            let (rug_c, rug_o) = rug_acsc_with_period_rational_prec_round(&x, u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 360, 10, Nearest, "NaN", "NaN", Equal);
    test("1/2", 360, 10, Exact, "NaN", "NaN", Equal);
    test("-1/2", 360, 10, Nearest, "NaN", "NaN", Equal);
    test("0", 0, 10, Exact, "NaN", "NaN", Equal);
    test("1/2", 0, 10, Exact, "NaN", "NaN", Equal);
    test("1", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("-1", 0, 10, Exact, "-0.0", "-0x0.0", Equal);
    test("5/3", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 360, 10, Exact, "90.000", "0x5a.0#10", Equal);
    test("-1", 360, 10, Exact, "-90.000", "-0x5a.0#10", Equal);
    test("1", 12, 10, Exact, "3.0000", "0x3.00#10", Equal);
    test("1", 7, 10, Exact, "1.7500", "0x1.c00#10", Equal);
    test("-1", 7, 10, Exact, "-1.7500", "-0x1.c00#10", Equal);
    test("1", 7, 1, Floor, "1.0", "0x1.0#1", Less);
    test("1", 7, 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1", 7, 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("-1", 7, 1, Floor, "-2.0", "-0x2.0#1", Less);
    test("-1", 7, 1, Ceiling, "-1.0", "-0x1.0#1", Greater);
    test("2", 360, 10, Exact, "30.000", "0x1e.00#10", Equal);
    test("-2", 360, 10, Exact, "-30.000", "-0x1e.00#10", Equal);
    test("2", 12, 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("-2", 12, 10, Exact, "-1.0000", "-0x1.000#10", Equal);
    test("2", 3, 10, Exact, "0.25000", "0x0.400#10", Equal);
    test("2", 7, 10, Nearest, "0.58301", "0x0.954#10", Less);
    test("-2", 7, 10, Nearest, "-0.58301", "-0x0.954#10", Greater);
    test("5/3", 360, 10, Floor, "36.812", "0x24.d#10", Less);
    test("5/3", 360, 10, Ceiling, "36.875", "0x24.e#10", Greater);
    test("5/3", 360, 10, Down, "36.812", "0x24.d#10", Less);
    test("5/3", 360, 10, Up, "36.875", "0x24.e#10", Greater);
    test("5/3", 360, 10, Nearest, "36.875", "0x24.e#10", Greater);
    test("-5/3", 360, 10, Nearest, "-36.875", "-0x24.e#10", Less);
    test(
        "5/3",
        360,
        100,
        Nearest,
        "36.869897645844021296855612559085",
        "0x24.deb19cb3c478602c9ec9b860#100",
        Less,
    );
    test("3/2", 360, 20, Nearest, "41.810303", "0x29.cf70#20", Less);
    test(
        "-3/2",
        360,
        20,
        Nearest,
        "-41.810303",
        "-0x29.cf70#20",
        Greater,
    );
    test(
        "100/99",
        360,
        20,
        Nearest,
        "81.890381",
        "0x51.e3f0#20",
        Less,
    );
    test(
        "-100/99",
        360,
        20,
        Nearest,
        "-81.890381",
        "-0x51.e3f0#20",
        Greater,
    );
    test(
        "100",
        360,
        20,
        Nearest,
        "0.57296753",
        "0x0.92ae0#20",
        Greater,
    );
    test(
        "-100",
        360,
        20,
        Nearest,
        "-0.57296753",
        "-0x0.92ae0#20",
        Less,
    );
    test(
        "100000000000000000000000000000000000000000",
        360,
        20,
        Nearest,
        "5.7295801e-40",
        "0x3.1e964E-33#20",
        Greater,
    );
    test(
        "-100000000000000000000000000000000000000000",
        360,
        20,
        Nearest,
        "-5.7295801e-40",
        "-0x3.1e964E-33#20",
        Less,
    );
    test(
        "100000000000000000000000000000000000000000",
        1,
        20,
        Nearest,
        "1.5915494e-42",
        "0x2.37e24E-35#20",
        Less,
    );
    test(
        "3/2",
        1,
        20,
        Nearest,
        "0.11613977",
        "0x0.1dbb56#20",
        Greater,
    );
    test(
        "1",
        18446744073709551615,
        64,
        Exact,
        "4611686018427387903.75",
        "0x3fffffffffffffff.c#64",
        Equal,
    );
    test(
        "2",
        18446744073709551615,
        64,
        Exact,
        "1537228672809129301.25",
        "0x1555555555555555.4#64",
        Equal,
    );
    test(
        "5/3",
        18446744073709551615,
        20,
        Nearest,
        "1.8892490e18",
        "0x1.a37f6E+15#20",
        Greater,
    );
}

#[test]
#[should_panic]
fn acsc_with_period_rational_prec_round_fail_1() {
    Float::acsc_with_period_rational_prec_round(Rational::TWO, 7, 0, Floor);
}

#[test]
#[should_panic]
fn acsc_with_period_rational_prec_round_fail_2() {
    // acsc(3/2) is not an exact number of sevenths of a turn
    Float::acsc_with_period_rational_prec_round(Rational::from_unsigneds(3u8, 2), 7, 10, Exact);
}

#[test]
#[should_panic]
fn acsc_with_period_rational_prec_round_fail_3() {
    // acsc(2) is a twelfth of a turn, but 7 is not a multiple of 3
    Float::acsc_with_period_rational_prec_round(Rational::TWO, 7, 10, Exact);
}

#[test]
#[should_panic]
fn acsc_with_period_rational_prec_round_fail_4() {
    // a quarter turn needs more than 2 bits when u = 7
    Float::acsc_with_period_rational_prec_round(Rational::ONE, 7, 2, Exact);
}

#[test]
#[should_panic]
fn acsc_with_period_rational_prec_round_ref_fail() {
    Float::acsc_with_period_rational_prec_round_ref(&Rational::TWO, 7, 0, Floor);
}

#[test]
#[should_panic]
fn acsc_with_period_rational_prec_fail() {
    Float::acsc_with_period_rational_prec(Rational::TWO, 7, 0);
}

// A `Rational` large enough that acsc(x), about 1/x, falls far below the smallest positive `Float`.
// This is the case the scaled path exists for: the arccosecant alone underflows, but a large enough
// period lifts the quotient back into the range, so the reciprocal has to be taken here rather than
// read off the underflowing arccosecant. The `Rational` has about 2^30 bits, so each call costs a
// few seconds.
#[test]
fn test_acsc_with_period_rational_underflow() {
    let x = Rational::power_of_2((1i64 << 30) + 2);
    let min = Float::min_positive_value_prec(53);
    // with u = 1 the quotient stays below the bottom of the range
    let (c, o) = Float::acsc_with_period_rational_prec_round_ref(&x, 1, 53, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    let (c, o) = Float::acsc_with_period_rational_prec_round_ref(&x, 1, 53, Ceiling);
    assert_eq!(ComparableFloatRef(&c), ComparableFloatRef(&min));
    assert_eq!(o, Greater);
    // a large period lifts the quotient back into the range -- the case this path exists for -- and
    // doubling the period there doubles the result exactly
    let (c, o) = Float::acsc_with_period_rational_prec_round_ref(&x, 1 << 10, 53, Nearest);
    assert!(c > min);
    assert_ne!(o, Equal);
    let (c_alt, o_alt) = Float::acsc_with_period_rational_prec_round_ref(&x, 1 << 11, 53, Nearest);
    assert_eq!(ComparableFloat(c_alt), ComparableFloat(c << 1u32));
    assert_eq!(o_alt, o);
    // the arccosecant is odd, so a negative input underflows to the other side
    let x = -x;
    let (c, o) = Float::acsc_with_period_rational_prec_round_ref(&x, 1, 53, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
    assert_eq!(o, Greater);
    let (c, o) = Float::acsc_with_period_rational_prec_round_ref(&x, 1, 53, Floor);
    assert_eq!(ComparableFloat(c), ComparableFloat(-min));
    assert_eq!(o, Less);
}

// Whether acscu(x, u) is exactly representable at `prec` for a `Rational` x: at an input inside
// (-1, 1) (where the result is NaN), at u = 0, and at the turn fractions -- a quarter at |x| = 1
// and a twelfth at |x| = 2 with u a multiple of 3 -- each of which needs a `prec` wide enough to
// hold it.
fn acsc_with_period_rational_exact(x: &Rational, u: u64, prec: u64) -> bool {
    x.lt_abs(&1u32)
        || u == 0
        || (x.eq_abs(&Rational::ONE) && Float::from_unsigned_prec(u, prec).1 == Equal)
        || (x.eq_abs(&Rational::TWO)
            && u.is_multiple_of(3)
            && Float::from_unsigned_prec(u / 3, prec).1 == Equal)
}

#[allow(clippy::needless_pass_by_value)]
fn acsc_with_period_rational_prec_round_properties_helper(
    x: Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) {
    if rm == Exact && !acsc_with_period_rational_exact(&x, u, prec) {
        assert_panic!(Float::acsc_with_period_rational_prec_round_ref(
            &x, u, prec, Exact
        ));
        return;
    }
    let (c, o) = Float::acsc_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = Float::acsc_with_period_rational_prec_round_ref(&x, u, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // the arcsine of the exact reciprocal, in the same units: the same real number
    if x != 0u32 {
        let (c_alt, o_alt) =
            Float::asin_with_period_rational_prec_round(x.clone().reciprocal(), u, prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o, "x = {x} u = {u} prec = {prec} rm = {rm:?}");
    }

    if !c.is_nan()
        && let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && u <= u64::from(u32::MAX)
        && !mpfr_rational_divergence(&x, u)
    {
        let (rug_c, rug_o) = rug_acsc_with_period_rational_prec_round(&x, u, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o, "x = {x} u = {u} prec = {prec} rm = {rm:?}");
    }

    // NaN exactly for an input inside (-1, 1)
    assert_eq!(c.is_nan(), x.lt_abs(&1u32));
    if !c.is_nan() {
        // |acscu(x, u)| <= u/4, a quarter turn, so the result never overflows
        assert!(c.is_finite());
        let bound = Float::from_unsigned_prec_round(u, prec, Ceiling).0 >> 2u32;
        assert!(c <= bound);
        assert!(c >= -bound);
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
    }
    // the arccosecant is odd
    let (c_neg, o_neg) = Float::acsc_with_period_rational_prec_round(-&x, u, prec, -rm);
    assert_eq!(ComparableFloat(-c_neg), ComparableFloat(c.clone()));
    assert_eq!(o_neg.reverse(), o);
    // a `Float` input agrees with the `Float` version
    if let Ok(f) = Float::try_from(&x) {
        let (c_alt, o_alt) = f.acsc_with_period_prec_round(u, prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }

    if o == Equal {
        assert!(acsc_with_period_rational_exact(&x, u, prec));
        for rm in exhaustive_rounding_modes() {
            let (c2, oo) = Float::acsc_with_period_rational_prec_round_ref(&x, u, prec, rm);
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::acsc_with_period_rational_prec_round_ref(
            &x, u, prec, Exact
        ));
    }
}

#[test]
fn acsc_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_10().test_properties(
        |(x, u, prec, rm)| {
            acsc_with_period_rational_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // acscu(x, u) = NaN for |x| < 1, in every rounding mode and even when u is zero
        for x in [Rational::ZERO, Rational::ONE_HALF, -Rational::ONE_HALF] {
            for u in [0, 4] {
                let (c, o) = Float::acsc_with_period_rational_prec_round(x.clone(), u, prec, rm);
                assert!(c.is_nan());
                assert_eq!(o, Equal);
            }
        }
        // acscu(x, 0) = 0 with the sign of x, exactly
        for x in [Rational::ONE, Rational::TWO, Rational::from_unsigneds(5u8, 3)] {
            let (c, o) = Float::acsc_with_period_rational_prec_round_ref(&x, 0, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
            assert_eq!(o, Equal);
            let (c, o) = Float::acsc_with_period_rational_prec_round(-x, 0, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(Float::NEGATIVE_ZERO));
            assert_eq!(o, Equal);
        }
        // acscu(±1, u) = ±u/4, a quarter turn, exact when `prec` holds it
        let (q, o_q) = Float::from_unsigned_prec_round(8u32, prec, rm);
        let (c, o) = Float::acsc_with_period_rational_prec_round(Rational::ONE, 8, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(q >> 2u32));
        assert_eq!(o, o_q);
        let (q, o_q) = Float::from_unsigned_prec_round(8u32, prec, -rm);
        let (c, o) =
            Float::acsc_with_period_rational_prec_round(Rational::NEGATIVE_ONE, 8, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(-(q >> 2u32)));
        assert_eq!(o, o_q.reverse());
        // acscu(±2, u) = ±u/12 when u is a multiple of 3
        let (q, o_q) = Float::from_unsigned_prec_round(4u32, prec, rm);
        let (c, o) = Float::acsc_with_period_rational_prec_round(Rational::TWO, 12, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(q >> 2u32));
        assert_eq!(o, o_q);
        let (q, o_q) = Float::from_unsigned_prec_round(4u32, prec, -rm);
        let (c, o) = Float::acsc_with_period_rational_prec_round(-Rational::TWO, 12, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(-(q >> 2u32)));
        assert_eq!(o, o_q.reverse());
    });
}

#[test]
fn acsc_with_period_rational_prec_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_10().test_properties(
        |(x, u, prec, _)| {
            let (c, o) = Float::acsc_with_period_rational_prec(x.clone(), u, prec);
            assert!(c.is_valid());
            let (c_alt, o_alt) = Float::acsc_with_period_rational_prec_ref(&x, u, prec);
            assert!(c_alt.is_valid());
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
            let (c_alt, o_alt) =
                Float::acsc_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        },
    );
}

#[test]
#[allow(clippy::approx_constant, clippy::type_repetition_in_bounds)]
fn test_primitive_float_acsc_with_period_rational() {
    fn test<T: PrimitiveFloat>(s: &str, u: u64, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_acsc_with_period_rational::<T>(
                &Rational::from_str(s).unwrap(),
                u
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 360, f32::NAN);
    test::<f32>("1/2", 360, f32::NAN);
    test::<f32>("1/2", 0, f32::NAN);
    test::<f32>("1", 360, 90.0);
    test::<f32>("1", 0, 0.0);
    test::<f32>("-1", 0, -0.0);
    test::<f32>("-1", 360, -90.0);
    test::<f32>("2", 360, 30.0);
    test::<f32>("-2", 360, -30.0);
    test::<f32>("2", 12, 1.0);
    test::<f32>("-2", 12, -1.0);
    test::<f32>("2", 7, 0.5833333);
    test::<f32>("5/3", 360, 36.869896);
    test::<f32>("-5/3", 360, -36.869896);
    test::<f32>("3/2", 360, 41.810314);
    test::<f32>("100/99", 360, 81.89039);
    test::<f32>("100", 360, 0.57296735);
    test::<f32>("1", 18446744073709551615, 4.611686e18);
    test::<f32>("-1", 18446744073709551615, -4.611686e18);
    test::<f64>("0", 360, f64::NAN);
    test::<f64>("1/2", 360, f64::NAN);
    test::<f64>("1/2", 0, f64::NAN);
    test::<f64>("1", 360, 90.0);
    test::<f64>("1", 0, 0.0);
    test::<f64>("-1", 0, -0.0);
    test::<f64>("-1", 360, -90.0);
    test::<f64>("2", 360, 30.0);
    test::<f64>("-2", 360, -30.0);
    test::<f64>("2", 12, 1.0);
    test::<f64>("-2", 12, -1.0);
    test::<f64>("2", 7, 0.5833333333333334);
    test::<f64>("5/3", 360, 36.86989764584402);
    test::<f64>("-5/3", 360, -36.86989764584402);
    test::<f64>("3/2", 360, 41.810314895778596);
    test::<f64>("100/99", 360, 81.89038554400582);
    test::<f64>("100", 360, 0.5729673448571526);
    test::<f64>("1", 18446744073709551615, 4.611686018427388e18);
    test::<f64>("-1", 18446744073709551615, -4.611686018427388e18);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acsc_with_period_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, u)| {
        let c = primitive_float_acsc_with_period_rational::<T>(&x, u);
        // NaN exactly for an input inside (-1, 1)
        assert_eq!(c.is_nan(), x.lt_abs(&1u32));
        if !c.is_nan() {
            // the result lies in [-u/4, u/4], so it never overflows
            assert!(c.is_finite());
            // the same as the `Float` version taken with 64 bits to spare and rounded once -- but
            // only where the result is normal, a subnormal one being rounded twice here
            if c.is_normal() {
                let (c_float, _) =
                    Float::acsc_with_period_rational_prec_ref(&x, u, T::MANTISSA_WIDTH + 64);
                assert_eq!(
                    NiceFloat(T::rounding_from(&c_float, Nearest).0),
                    NiceFloat(c)
                );
            }
        }
    });
}

#[test]
fn primitive_float_acsc_with_period_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acsc_with_period_rational_properties_helper);
}
