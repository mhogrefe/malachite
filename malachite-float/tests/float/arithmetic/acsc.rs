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
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::acsc::{
    primitive_float_acsc, primitive_float_acsc_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::acsc::{
    rug_acsc, rug_acsc_prec_round, rug_acsc_rational_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_51, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_48, float_unsigned_rounding_mode_triple_gen_var_49,
    rational_unsigned_rounding_mode_triple_gen_var_13,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
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
