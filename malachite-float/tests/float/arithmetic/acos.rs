// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Acos, AcosAssign, PowerOf2};
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
    primitive_float_gen, primitive_float_gen_var_1, primitive_float_unsigned_pair_gen_var_1,
    unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::float::arithmetic::acos::{
    primitive_float_acos, primitive_float_acos_pi, primitive_float_acos_pi_rational,
    primitive_float_acos_rational, primitive_float_acos_with_period,
    primitive_float_acos_with_period_rational,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::acos::{
    rug_acos, rug_acos_prec_round, rug_acos_rational_prec_round, rug_acos_with_period_prec_round,
    rug_acos_with_period_rational_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_49, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_2, float_unsigned_rounding_mode_triple_gen_var_42,
    float_unsigned_rounding_mode_triple_gen_var_43, float_unsigned_rounding_mode_triple_gen_var_44,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_23,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_24,
    float_unsigned_unsigned_triple_gen_var_1, rational_unsigned_rounding_mode_triple_gen_var_11,
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_8,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{
    rational_gen, rational_unsigned_pair_gen_var_1, rational_unsigned_pair_gen_var_3,
};
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_acos_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().acos_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.acos_prec_round_ref(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.acos_prec_round_assign(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = x.acos_prec_ref(prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acos_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test("0.0", "0x0.0", 10, Floor, "1.5703", "0x1.920#10", Less);
    test("0.0", "0x0.0", 10, Ceiling, "1.5723", "0x1.928#10", Greater);
    test("0.0", "0x0.0", 10, Nearest, "1.5703", "0x1.920#10", Less);
    test("-0.0", "-0x0.0", 10, Nearest, "1.5703", "0x1.920#10", Less);
    test(
        "0.0",
        "0x0.0",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test("2.0", "0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 10, Exact, "NaN", "NaN", Equal);
    test("1.5", "0x1.8#2", 10, Nearest, "NaN", "NaN", Equal);
    test("1.0", "0x1.0#1", 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 10, Floor, "3.1406", "0x3.24#10", Less);
    test(
        "-1.0",
        "-0x1.0#1",
        10,
        Ceiling,
        "3.1445",
        "0x3.25#10",
        Greater,
    );
    test("-1.0", "-0x1.0#1", 10, Nearest, "3.1406", "0x3.24#10", Less);
    test(
        "-1.0",
        "-0x1.0#1",
        53,
        Nearest,
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Less,
    );
    test("0.50", "0x0.8#1", 10, Floor, "1.0469", "0x1.0c0#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "1.0488",
        "0x1.0c8#10",
        Greater,
    );
    test("0.50", "0x0.8#1", 10, Down, "1.0469", "0x1.0c0#10", Less);
    test("0.50", "0x0.8#1", 10, Up, "1.0488", "0x1.0c8#10", Greater);
    test("0.50", "0x0.8#1", 10, Nearest, "1.0469", "0x1.0c0#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        53,
        Nearest,
        "1.0471975511965979",
        "0x1.0c152382d7366#53",
        Greater,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        10,
        Nearest,
        "2.0938",
        "0x2.18#10",
        Less,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        53,
        Nearest,
        "2.0943951023931957",
        "0x2.182a4705ae6cc#53",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        20,
        Nearest,
        "1.3181152",
        "0x1.51700#20",
        Less,
    );
    test(
        "0.75",
        "0x0.c#2",
        20,
        Nearest,
        "0.72273445",
        "0x0.b9052#20",
        Greater,
    );
    test(
        "-0.75",
        "-0x0.c#2",
        20,
        Nearest,
        "2.4188576",
        "0x2.6b3a4#20",
        Less,
    );
    test(
        "0.99902",
        "0x0.ffc#10",
        20,
        Floor,
        "0.044197738",
        "0x0.0b508b#20",
        Less,
    );
    test(
        "0.99902",
        "0x0.ffc#10",
        20,
        Ceiling,
        "0.044197798",
        "0x0.0b508c#20",
        Greater,
    );
    test(
        "0.99902",
        "0x0.ffc#10",
        20,
        Nearest,
        "0.044197798",
        "0x0.0b508c#20",
        Greater,
    );
    test(
        "0.99902",
        "0x0.ffc#10",
        100,
        Nearest,
        "0.044197771145715317926566687962741",
        "0x0.0b508b8da07f1f743f4b75b7d7#100",
        Less,
    );
    test(
        "-0.99902",
        "-0x0.ffc#10",
        20,
        Nearest,
        "3.0973930",
        "0x3.18eec#20",
        Less,
    );
    test(
        "0.99999999999999989",
        "0x0.fffffffffffff8#53",
        53,
        Nearest,
        "1.4901161193847656e-8",
        "0x4.0000000000000E-7#53",
        Less,
    );
    test(
        "0.600000000000000000022",
        "0x0.999999999999999a#64",
        64,
        Nearest,
        "0.927295218001612232405",
        "0x0.ed63382b0dda7b45#64",
        Greater,
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
        "7.9e-31",
        "0x1.0E-25#1",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    // - the Ziv loop retries at least once
    test("0.00830", "0x0.022#5", 5, Down, "1.50", "0x1.8#5", Less);
    test(
        "0.00830",
        "0x0.022#5",
        5,
        Nearest,
        "1.56",
        "0x1.9#5",
        Greater,
    );
    test("0.0703", "0x0.12#4", 1, Nearest, "2.0", "0x2.0#1", Greater);
    test(
        "-0.00134632",
        "-0x0.00583b8#16",
        53,
        Nearest,
        "1.5721426471154791",
        "0x1.9277f0c602029#53",
        Less,
    );
    test(
        "2.84457986519800883242389e-35",
        "0x2.5cf9a8bb4de1714d8e8E-29#75",
        180,
        Nearest,
        "1.5707963267948966192313216916397514136527860477074645862",
        "0x1.921fb54442d18469898cc51701b813d2b778e732fdbac#180",
        Less,
    );
}

#[test]
#[should_panic]
fn acos_prec_round_fail_1() {
    Float::ONE.acos_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn acos_prec_round_fail_2() {
    // acos(1/2) = pi/3 is not exactly representable
    Float::from(0.5).acos_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acos_prec_round_fail_3() {
    // acos(0) = pi/2 is not exactly representable either, although asin(0) is
    Float::ZERO.acos_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acos_prec_round_ref_fail() {
    Float::ONE.acos_prec_round_ref(0, Floor);
}

#[test]
#[should_panic]
fn acos_prec_fail() {
    Float::ONE.acos_prec(0);
}

#[test]
#[should_panic]
fn acos_round_fail() {
    Float::from(0.5).acos_round(Exact);
}

// Whether acos(x) is exactly representable at `prec`: at NaN, at an input outside [-1, 1] or
// infinite (where the result is NaN), and at x = 1, where the result is +0. Unlike the arcsine, a
// zero input is not exact: acos(0) is pi/2.
fn acos_exact(x: &Float) -> bool {
    x.is_nan() || !x.is_finite() || x.gt_abs(&1u32) || *x == 1u32
}

#[allow(clippy::needless_pass_by_value)]
fn acos_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    if rm == Exact && !acos_exact(&x) {
        assert_panic!(x.acos_prec_round_ref(prec, Exact));
        return;
    }
    let (c, o) = x.clone().acos_prec_round(prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.acos_prec_round_ref(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut c_alt = x.clone();
    let o_alt = c_alt.acos_prec_round_assign(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_acos_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // NaN exactly for a NaN input, an infinite input, or an input outside [-1, 1]
    assert_eq!(c.is_nan(), x.is_nan() || !x.is_finite() || x.gt_abs(&1u32));
    if !c.is_nan() {
        // 0 <= acos(x) <= pi, so the result never overflows, and it is zero only at x = 1
        assert!(c.is_finite());
        assert!(c >= 0u32);
        // rounding is monotone, so the result cannot exceed pi rounded up at the same precision
        assert!(c <= Float::pi_prec_round(prec, Ceiling).0);
        assert_eq!(c == 0u32, x == 1u32);
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
        // acos(x) + asin(x) = pi/2, and acos(-x) + acos(x) = pi
        let w = prec + 64;
        let sum = x.asin_prec_ref(w).0.add_prec(x.acos_prec_ref(w).0, w).0;
        let half_pi = Float::pi_prec(w).0 >> 1u32;
        let diff = sum.sub_prec(half_pi, w).0;
        assert!(
            diff == 0u32 || i64::from(diff.get_exponent().unwrap()) < 10 - i64::exact_from(w),
            "acos + asin is not pi/2 for {x}"
        );
        let sum = (-&x).acos_prec_ref(w).0.add_prec(x.acos_prec_ref(w).0, w).0;
        let diff = sum.sub_prec(Float::pi_prec(w).0, w).0;
        assert!(
            diff == 0u32 || i64::from(diff.get_exponent().unwrap()) < 10 - i64::exact_from(w),
            "acos(-x) + acos(x) is not pi for {x}"
        );
    }

    if o == Equal {
        assert!(acos_exact(&x));
        for rm in exhaustive_rounding_modes() {
            let (c2, oo) = x.acos_prec_round_ref(prec, rm);
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.acos_prec_round_ref(prec, Exact));
    }
}

#[test]
fn acos_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_42().test_properties(|(x, prec, rm)| {
        acos_prec_round_properties_helper(x, prec, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_43().test_properties(|(x, prec, rm)| {
        acos_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // acos(NaN) = acos(±infinity) = NaN, as is acos(x) for |x| > 1, in every rounding mode
        for x in [Float::NAN, Float::INFINITY, Float::NEGATIVE_INFINITY, Float::TWO] {
            let (c, o) = x.acos_prec_round(prec, rm);
            assert!(c.is_nan());
            assert_eq!(o, Equal);
        }
        // acos(1) = +0, exactly
        let (c, o) = Float::ONE.acos_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        if rm == Exact {
            // pi/2 and pi are never exactly representable, so unlike the arcsine, a zero input is
            // not an exact case
            assert_panic!(Float::ZERO.acos_prec_round(prec, Exact));
            assert_panic!(Float::NEGATIVE_ZERO.acos_prec_round(prec, Exact));
            assert_panic!(Float::NEGATIVE_ONE.acos_prec_round(prec, Exact));
            return;
        }
        // acos(±0.0) = pi/2, which is pi rounded and halved
        let (pi, o_pi) = Float::pi_prec_round(prec, rm);
        for x in [Float::ZERO, Float::NEGATIVE_ZERO] {
            let (c, o) = x.acos_prec_round(prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(pi.clone() >> 1u32));
            assert_eq!(o, o_pi);
        }
        // acos(-1) = pi
        let (c, o) = Float::NEGATIVE_ONE.acos_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(pi));
        assert_eq!(o, o_pi);
    });
}

#[test]
fn acos_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (c, o) = x.clone().acos_prec(prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.acos_prec_ref(prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acos_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acos_prec_assign(prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn acos_round_properties() {
    float_rounding_mode_pair_gen_var_49().test_properties(|(x, rm)| {
        let (c, o) = x.clone().acos_round(rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.acos_round_ref(rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acos_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acos_round_assign(rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn acos_properties() {
    float_gen().test_properties(|x| {
        let c = x.clone().acos();
        assert!(c.is_valid());
        let c_alt = (&x).acos();
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let mut c_alt = x.clone();
        c_alt.acos_assign();
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let (c_alt, _) = x.acos_prec_round_ref(x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let rug_c = rug_acos(&rug::Float::exact_from(&x));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
    });
}

#[test]
// The table pins the correctly rounded arccosines of a few inputs, some of which are the pi
// constants; naming the constants instead would not test the rounding.
#[allow(clippy::approx_constant, clippy::type_repetition_in_bounds)]
fn test_primitive_float_acos() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_acos(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(2.0, f32::NAN);
    test::<f32>(-2.0, f32::NAN);
    test::<f32>(0.0, 1.5707964);
    test::<f32>(-0.0, 1.5707964);
    test::<f32>(1.0, 0.0);
    test::<f32>(-1.0, 3.1415927);
    test::<f32>(0.5, 1.0471976);
    test::<f32>(-0.5, 2.0943952);
    test::<f32>(0.25, 1.3181161);
    test::<f32>(0.75, 0.7227343);
    test::<f32>(-0.75, 2.4188583);
    test::<f32>(0.1, 1.4706289);
    test::<f32>(0.9999, 0.014143427);
    test::<f32>(-0.9999, 3.1274493);
    test::<f32>(0.99999994, 0.00034526698);
    test::<f32>(1.0e-30, 1.5707964);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(2.0, f64::NAN);
    test::<f64>(-2.0, f64::NAN);
    test::<f64>(0.0, 1.5707963267948966);
    test::<f64>(-0.0, 1.5707963267948966);
    test::<f64>(1.0, 0.0);
    test::<f64>(-1.0, 3.141592653589793);
    test::<f64>(0.5, 1.0471975511965979);
    test::<f64>(-0.5, 2.0943951023931957);
    test::<f64>(0.25, 1.318116071652818);
    test::<f64>(0.75, 0.7227342478134157);
    test::<f64>(-0.75, 2.4188584057763776);
    test::<f64>(0.1, 1.4706289056333368);
    test::<f64>(0.9999, 0.014142253477512098);
    test::<f64>(-0.9999, 3.127450400112281);
    test::<f64>(0.9999999999999999, 1.4901161193847656e-8);
    test::<f64>(1.0e-300, 1.5707963267948966);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acos_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let c = primitive_float_acos(x);
        // NaN exactly for a NaN input, an infinite input, or an input outside [-1, 1]
        assert_eq!(c.is_nan(), x.is_nan() || !x.is_finite() || x.abs() > T::ONE);
        if !c.is_nan() {
            // the result lies in [0, pi], so it never overflows
            assert!(c.is_finite());
            assert!(c >= T::ZERO);
            // the same as the `Float` version taken with 64 bits to spare and rounded once
            let (c_float, _) = Float::acos_prec(Float::from(x), T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&c_float, Nearest).0),
                NiceFloat(c)
            );
        }
    });
}

#[test]
fn primitive_float_acos_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acos_properties_helper);
}

#[test]
fn test_acos_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::acos_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = Float::acos_rational_prec_round_ref(&x, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = Float::acos_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
            let (c_alt, o_alt) = Float::acos_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acos_rational_prec_round(&x, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 10, Nearest, "1.5703", "0x1.920#10", Less);
    test("0", 10, Floor, "1.5703", "0x1.920#10", Less);
    test(
        "0",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test("2", 10, Nearest, "NaN", "NaN", Equal);
    test("-2", 10, Nearest, "NaN", "NaN", Equal);
    test("2", 10, Exact, "NaN", "NaN", Equal);
    test("100/99", 10, Nearest, "NaN", "NaN", Equal);
    test("1", 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 1, Exact, "0.0", "0x0.0", Equal);
    test("-1", 10, Floor, "3.1406", "0x3.24#10", Less);
    test("-1", 10, Ceiling, "3.1445", "0x3.25#10", Greater);
    test(
        "-1",
        53,
        Nearest,
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Less,
    );
    test("1/2", 10, Floor, "1.0469", "0x1.0c0#10", Less);
    test("1/2", 10, Ceiling, "1.0488", "0x1.0c8#10", Greater);
    test("1/2", 10, Nearest, "1.0469", "0x1.0c0#10", Less);
    test(
        "1/2",
        53,
        Nearest,
        "1.0471975511965979",
        "0x1.0c152382d7366#53",
        Greater,
    );
    test(
        "-1/2",
        53,
        Nearest,
        "2.0943951023931957",
        "0x2.182a4705ae6cc#53",
        Greater,
    );
    test("3/5", 10, Floor, "0.92676", "0x0.ed4#10", Less);
    test("3/5", 10, Ceiling, "0.92773", "0x0.ed8#10", Greater);
    test(
        "3/5",
        53,
        Nearest,
        "0.92729521800161219",
        "0x0.ed63382b0dda78#53",
        Less,
    );
    test(
        "-3/5",
        53,
        Nearest,
        "2.2142974355881808",
        "0x2.36dc325d77c88#53",
        Less,
    );
    test("1/3", 20, Nearest, "1.2309589", "0x1.3b202#20", Less);
    test("2/3", 20, Nearest, "0.84106827", "0x0.d7504#20", Less);
    test(
        "999999/1000000",
        20,
        Nearest,
        "0.0014142133",
        "0x0.005cae90#20",
        Less,
    );
    test(
        "999999/1000000",
        100,
        Nearest,
        "0.0014142136802242517630717957726549",
        "0x0.005cae918191032b1f9c2e2fbe58#100",
        Less,
    );
    test(
        "-999999/1000000",
        20,
        Nearest,
        "3.1401787",
        "0x3.23e2c#20",
        Greater,
    );
    test("1/1000000", 20, Nearest, "1.5707951", "0x1.921fa#20", Less);
    test(
        "-1/1000000",
        53,
        Nearest,
        "1.5707973267948967",
        "0x1.921fc60b3a724#53",
        Greater,
    );
    test(
        "340282366920938463463374607431768211455/340282366920938463463374607431768211456",
        53,
        Nearest,
        "7.6664670834168709e-20",
        "0x1.6a09e667f3bcdE-16#53",
        Greater,
    );
}

// A `Rational` far below the target precision but above the bottom of the exponent range, where
// acos(x) = pi/2 - x - ..., where no leading term is rational: `acos_rational_helper` takes the
// `Float` arccosine of x rounded to the working precision, which the arccosine's unit slope passes
// on unamplified, instead of forming 1 - x^2 exactly -- for these inputs a dense `Rational` of
// hundreds of millions of bits, 5 seconds a call before that branch existed. Where x is dyadic it
// is exactly a `Float`, so the `Float` arccosine, reached through independent code, must agree.
#[test]
fn test_acos_rational_tiny() {
    let test =
        |x: Rational, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
            let (c, o) = Float::acos_rational_prec_round_ref(&x, prec, rm);
            assert!(c.is_valid());
            assert_eq!(c.to_string(), out);
            assert_eq!(to_hex_string(&c), out_hex);
            assert_eq!(o, o_out);
            if let Ok(f) = Float::try_from(&x) {
                let (c_alt, o_alt) = f.acos_prec_round(prec, rm);
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
        Floor,
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
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
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
        Rational::power_of_2(-1000i64),
        10,
        Nearest,
        "1.5703",
        "0x1.920#10",
        Less,
    );
}

#[test]
#[should_panic]
fn acos_rational_prec_round_fail_1() {
    Float::acos_rational_prec_round(Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn acos_rational_prec_round_fail_2() {
    // acos(1/2) = pi/3 is not exactly representable
    Float::acos_rational_prec_round(Rational::ONE_HALF, 10, Exact);
}

#[test]
#[should_panic]
fn acos_rational_prec_round_fail_3() {
    // acos(0) = pi/2 is not exactly representable either, although asin(0) is
    Float::acos_rational_prec_round(Rational::ZERO, 10, Exact);
}

#[test]
#[should_panic]
fn acos_rational_prec_round_ref_fail() {
    Float::acos_rational_prec_round_ref(&Rational::ONE, 0, Floor);
}

#[test]
#[should_panic]
fn acos_rational_prec_fail() {
    Float::acos_rational_prec(Rational::ONE, 0);
}

// Whether acos(x) is exactly representable for a `Rational` x: at an input outside [-1, 1] (where
// the result is NaN) and at x = 1, where the result is zero. Unlike the arcsine, a zero input is
// not exact: acos(0) is pi/2.
fn acos_rational_exact(x: &Rational) -> bool {
    x.gt_abs(&1u32) || *x == 1u32
}

#[allow(clippy::needless_pass_by_value)]
fn acos_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    if rm == Exact && !acos_rational_exact(&x) {
        assert_panic!(Float::acos_rational_prec_round_ref(&x, prec, Exact));
        return;
    }
    let (c, o) = Float::acos_rational_prec_round(x.clone(), prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = Float::acos_rational_prec_round_ref(&x, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_acos_rational_prec_round(&x, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o, "x = {x} prec = {prec} rm = {rm:?}");
    }

    // NaN exactly for an input outside [-1, 1]
    assert_eq!(c.is_nan(), x.gt_abs(&1u32));
    if !c.is_nan() {
        // 0 <= acos(x) <= pi, so the result never overflows, and it is zero only at x = 1
        assert!(c.is_finite());
        assert!(c >= 0u32);
        assert!(c <= Float::pi_prec_round(prec, Ceiling).0);
        assert_eq!(c == 0u32, x == 1u32);
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
        // acos(x) + asin(x) = pi/2, and acos(-x) + acos(x) = pi
        let w = prec + 64;
        let sum = Float::acos_rational_prec_ref(&x, w)
            .0
            .add_prec(Float::asin_rational_prec_ref(&x, w).0, w)
            .0;
        let half_pi = Float::pi_prec(w).0 >> 1u32;
        let diff = sum.sub_prec(half_pi, w).0;
        assert!(
            diff == 0u32 || i64::from(diff.get_exponent().unwrap()) < 10 - i64::exact_from(w),
            "acos + asin is not pi/2 for {x}"
        );
        let sum = Float::acos_rational_prec_ref(&-&x, w)
            .0
            .add_prec(Float::acos_rational_prec_ref(&x, w).0, w)
            .0;
        let diff = sum.sub_prec(Float::pi_prec(w).0, w).0;
        assert!(
            diff == 0u32 || i64::from(diff.get_exponent().unwrap()) < 10 - i64::exact_from(w),
            "acos(-x) + acos(x) is not pi for {x}"
        );
    }
    // a `Float` input agrees with the `Float` version
    if let Ok(f) = Float::try_from(&x) {
        let (c_alt, o_alt) = f.acos_prec_round(prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }

    if o == Equal {
        assert!(acos_rational_exact(&x));
        for rm in exhaustive_rounding_modes() {
            let (c2, oo) = Float::acos_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::acos_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn acos_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_11().test_properties(|(x, prec, rm)| {
        acos_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // acos(x) = NaN for |x| > 1, in every rounding mode
        let (c, o) = Float::acos_rational_prec_round(Rational::TWO, prec, rm);
        assert!(c.is_nan());
        assert_eq!(o, Equal);
        // acos(1) = +0, exactly
        let (c, o) = Float::acos_rational_prec_round(Rational::ONE, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        if rm == Exact {
            // pi/2 and pi are never exactly representable
            assert_panic!(Float::acos_rational_prec_round(Rational::ZERO, prec, Exact));
            assert_panic!(Float::acos_rational_prec_round(
                Rational::NEGATIVE_ONE,
                prec,
                Exact
            ));
            return;
        }
        // acos(0) = pi/2, which is pi rounded and halved
        let (pi, o_pi) = Float::pi_prec_round(prec, rm);
        let (c, o) = Float::acos_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(pi.clone() >> 1u32));
        assert_eq!(o, o_pi);
        // acos(-1) = pi
        let (c, o) = Float::acos_rational_prec_round(Rational::NEGATIVE_ONE, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(pi));
        assert_eq!(o, o_pi);
    });
}

#[test]
fn acos_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (c, o) = Float::acos_rational_prec(x.clone(), prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = Float::acos_rational_prec_ref(&x, prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = Float::acos_rational_prec_round_ref(&x, prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

// A `Rational` within 2^(-2^31) of 1, where acos(x) is about sqrt(2(1 - x)) and falls below the
// smallest positive `Float` -- a regime the `Float` arccosine cannot reach, since it would need an
// input of more than 2^31 bits. The `Rational` here has exactly that many, so each call costs a few
// seconds.
#[test]
fn test_acos_rational_underflow() {
    let min_positive = Float::min_positive_value_prec(53);
    // acos(1 - 2^-k) is about 2^(1 - k/2), which for this k is a bit below half the smallest
    // positive `Float`, so `Nearest` rounds it to zero too
    let x = Rational::ONE - Rational::power_of_2(-((1i64 << 31) + 4));
    for (rm, expected, o_out) in [
        (Nearest, Float::ZERO, Less),
        (Floor, Float::ZERO, Less),
        (Ceiling, min_positive.clone(), Greater),
    ] {
        let (c, o) = Float::acos_rational_prec_round_ref(&x, 53, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(expected), "rm = {rm:?}");
        assert_eq!(o, o_out, "rm = {rm:?}");
    }
    // two exponents up, the same path returns a representable value instead
    let x = Rational::ONE - Rational::power_of_2(-(1i64 << 31));
    let (c, o) = Float::acos_rational_prec_round_ref(&x, 53, Nearest);
    assert!(c > min_positive);
    assert_eq!(o, Greater);
}

#[test]
// As in `test_primitive_float_acos`, some of the pinned values are the pi constants.
#[allow(clippy::approx_constant, clippy::type_repetition_in_bounds)]
fn test_primitive_float_acos_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_acos_rational::<T>(
                &Rational::from_str(s).unwrap()
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 1.5707964);
    test::<f32>("1", 0.0);
    test::<f32>("-1", 3.1415927);
    test::<f32>("2", f32::NAN);
    test::<f32>("-2", f32::NAN);
    test::<f32>("100/99", f32::NAN);
    test::<f32>("1/2", 1.0471976);
    test::<f32>("-1/2", 2.0943952);
    test::<f32>("3/5", 0.9272952);
    test::<f32>("-3/5", 2.2142975);
    test::<f32>("1/3", 1.2309594);
    test::<f32>("2/3", 0.8410687);
    test::<f32>("99/100", 0.14153947);
    test::<f32>("-99/100", 3.0000532);
    test::<f32>("999999/1000000", 0.0014142137);
    test::<f32>("1/1000000", 1.5707953);
    test::<f32>("-1/1000000", 1.5707973);
    test::<f32>(
        "340282366920938463463374607431768211455/340282366920938463463374607431768211456",
        7.666467e-20,
    );
    test::<f64>("0", 1.5707963267948966);
    test::<f64>("1", 0.0);
    test::<f64>("-1", 3.141592653589793);
    test::<f64>("2", f64::NAN);
    test::<f64>("-2", f64::NAN);
    test::<f64>("100/99", f64::NAN);
    test::<f64>("1/2", 1.0471975511965979);
    test::<f64>("-1/2", 2.0943951023931957);
    test::<f64>("3/5", 0.9272952180016122);
    test::<f64>("-3/5", 2.214297435588181);
    test::<f64>("1/3", 1.2309594173407747);
    test::<f64>("2/3", 0.8410686705679302);
    test::<f64>("99/100", 0.1415394733244272);
    test::<f64>("-99/100", 3.000053180265366);
    test::<f64>("999999/1000000", 0.0014142136802242518);
    test::<f64>("1/1000000", 1.5707953267948966);
    test::<f64>("-1/1000000", 1.5707973267948967);
    test::<f64>(
        "340282366920938463463374607431768211455/340282366920938463463374607431768211456",
        7.666467083416871e-20,
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acos_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let c = primitive_float_acos_rational::<T>(&x);
        // NaN exactly for an input outside [-1, 1]
        assert_eq!(c.is_nan(), x.gt_abs(&1u32));
        if !c.is_nan() {
            // the result lies in [0, pi], so it never overflows
            assert!(c.is_finite());
            assert!(c >= T::ZERO);
            // the same as the `Float` version taken with 64 bits to spare and rounded once
            let (c_float, _) = Float::acos_rational_prec_ref(&x, T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&c_float, Nearest).0),
                NiceFloat(c)
            );
        }
    });

    primitive_float_gen_var_1::<T>().test_properties(|x| {
        // The arccosine of a finite primitive float, taken through the `Rational` path, matches the
        // direct primitive-float arccosine.
        assert_eq!(
            NiceFloat(primitive_float_acos_rational::<T>(&Rational::exact_from(x))),
            NiceFloat(primitive_float_acos(x))
        );
    });
}

#[test]
fn primitive_float_acos_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acos_rational_properties_helper);
}

#[test]
fn test_acos_with_period_prec_round() {
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

        let (c, o) = x.clone().acos_with_period_prec_round(u, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.acos_with_period_prec_round_ref(u, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.acos_with_period_prec_round_assign(u, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = x.acos_with_period_prec_ref(u, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u <= u64::from(u32::MAX)
        {
            let (rug_c, rug_o) =
                rug_acos_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 360, 10, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 360, 10, Nearest, "NaN", "NaN", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        360,
        10,
        Nearest,
        "NaN",
        "NaN",
        Equal,
    );
    test("0.0", "0x0.0", 360, 10, Exact, "90.000", "0x5a.0#10", Equal);
    test(
        "-0.0",
        "-0x0.0",
        360,
        10,
        Exact,
        "90.000",
        "0x5a.0#10",
        Equal,
    );
    test(
        "0.0",
        "0x0.0",
        7,
        10,
        Nearest,
        "1.7500",
        "0x1.c00#10",
        Equal,
    );
    test("0.0", "0x0.0", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("2.0", "0x2.0#1", 360, 10, Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 360, 10, Nearest, "NaN", "NaN", Equal);
    test("2.0", "0x2.0#1", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("1.0", "0x1.0#1", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 7, 1, Exact, "0.0", "0x0.0", Equal);
    test(
        "-1.0",
        "-0x1.0#1",
        360,
        10,
        Exact,
        "180.00",
        "0xb4.0#10",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        7,
        10,
        Exact,
        "3.5000",
        "0x3.80#10",
        Equal,
    );
    test("-1.0", "-0x1.0#1", 7, 2, Floor, "3.0", "0x3.0#2", Less);
    test("-1.0", "-0x1.0#1", 7, 2, Ceiling, "4.0", "0x4.0#2", Greater);
    test(
        "0.50",
        "0x0.8#1",
        360,
        10,
        Exact,
        "60.000",
        "0x3c.0#10",
        Equal,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        360,
        10,
        Exact,
        "120.00",
        "0x78.0#10",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        12,
        10,
        Exact,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        12,
        10,
        Exact,
        "4.0000",
        "0x4.00#10",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        7,
        10,
        Floor,
        "1.1660",
        "0x1.2a8#10",
        Less,
    );
    test(
        "0.50",
        "0x0.8#1",
        7,
        10,
        Ceiling,
        "1.1680",
        "0x1.2b0#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        10,
        Floor,
        "75.500",
        "0x4b.8#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        10,
        Ceiling,
        "75.625",
        "0x4b.a#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        10,
        Nearest,
        "75.500",
        "0x4b.8#10",
        Less,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        360,
        10,
        Nearest,
        "104.50",
        "0x68.8#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        360,
        53,
        Nearest,
        "75.522487814070075",
        "0x4b.85c1c2e9fd50#53",
        Less,
    );
    test(
        "0.75",
        "0x0.c#2",
        360,
        20,
        Nearest,
        "41.409607",
        "0x29.68dc#20",
        Less,
    );
    test(
        "0.99902",
        "0x0.ffc#10",
        360,
        20,
        Nearest,
        "2.5323448",
        "0x2.8847c#20",
        Less,
    );
    test(
        "-0.99902",
        "-0x0.ffc#10",
        360,
        20,
        Nearest,
        "177.46777",
        "0xb1.77c#20",
        Greater,
    );
    test(
        "0.600000000000000000022",
        "0x0.999999999999999a#64",
        360,
        64,
        Nearest,
        "53.1301023541559787021",
        "0x35.214e634c3b879fc#64",
        Greater,
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
        "7.9e-31",
        "0x1.0E-25#1",
        1,
        20,
        Nearest,
        "0.25000000",
        "0x0.400000#20",
        Greater,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        360,
        200,
        Nearest,
        "89.999999999999999999999999999954801599507966377340969960144766",
        "0x59.fffffffffffffffffffffffc6b447cb387c108f3d5a2b2030#200",
        Greater,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        1,
        20,
        Nearest,
        "0.25000000",
        "0x0.400000#20",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        18446744073709551615,
        64,
        Nearest,
        "4611686018427387903.75",
        "0x3fffffffffffffff.c#64",
        Greater,
    );
    test(
        "1.0",
        "0x1.0#1",
        18446744073709551615,
        64,
        Exact,
        "0.0",
        "0x0.0",
        Equal,
    );
    test(
        "-1.0",
        "-0x1.0#1",
        18446744073709551615,
        64,
        Exact,
        "9223372036854775807.50",
        "0x7fffffffffffffff.8#64",
        Equal,
    );
    test(
        "0.50",
        "0x0.8#1",
        18446744073709551615,
        64,
        Nearest,
        "3074457345618258602.50",
        "0x2aaaaaaaaaaaaaaa.8#64",
        Equal,
    );
}

#[test]
#[should_panic]
fn acos_with_period_prec_round_fail_1() {
    Float::ONE.acos_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn acos_with_period_prec_round_fail_2() {
    // acos(1/4) is not an exact number of sevenths of a turn
    Float::from(0.25).acos_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn acos_with_period_prec_round_fail_3() {
    // acos(1/2) is a sixth of a turn, but 7 is not a multiple of 3
    Float::ONE_HALF.acos_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn acos_with_period_prec_round_fail_4() {
    // a quarter turn needs more than 2 bits when u = 7
    Float::ZERO.acos_with_period_prec_round(7, 2, Exact);
}

#[test]
#[should_panic]
fn acos_with_period_prec_round_ref_fail() {
    Float::ONE.acos_with_period_prec_round_ref(7, 0, Floor);
}

#[test]
#[should_panic]
fn acos_with_period_prec_fail() {
    Float::ONE.acos_with_period_prec(7, 0);
}

#[test]
#[should_panic]
fn acos_with_period_round_fail() {
    Float::from(0.25).acos_with_period_round(7, Exact);
}

// Whether acosu(x, u) is exactly representable at `prec`: at NaN, at an infinite input or one
// outside [-1, 1] (where the result is NaN), at u = 0, at x = 1, where the result is zero, and at
// the turn fractions -- a quarter at a zero input, a half at -1, and a sixth or a third at |x| =
// 1/2 with u a multiple of 3 -- each of which needs a `prec` wide enough to hold it.
fn acos_with_period_exact(x: &Float, u: u64, prec: u64) -> bool {
    x.is_nan()
        || !x.is_finite()
        || x.gt_abs(&1u32)
        || u == 0
        || *x == 1u32
        || ((*x == 0u32 || *x == -1i32) && Float::from_unsigned_prec(u, prec).1 == Equal)
        || (x.eq_abs(&Float::ONE_HALF)
            && u.is_multiple_of(3)
            && Float::from_unsigned_prec(u / 3, prec).1 == Equal)
}

#[allow(clippy::needless_pass_by_value)]
fn acos_with_period_prec_round_properties_helper(x: Float, u: u64, prec: u64, rm: RoundingMode) {
    if rm == Exact && !acos_with_period_exact(&x, u, prec) {
        assert_panic!(x.acos_with_period_prec_round_ref(u, prec, Exact));
        return;
    }
    let (c, o) = x.clone().acos_with_period_prec_round(u, prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.acos_with_period_prec_round_ref(u, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut c_alt = x.clone();
    let o_alt = c_alt.acos_with_period_prec_round_assign(u, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && u <= u64::from(u32::MAX)
    {
        let (rug_c, rug_o) =
            rug_acos_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // NaN exactly for a NaN input, an infinite input, or an input outside [-1, 1]
    assert_eq!(c.is_nan(), x.is_nan() || !x.is_finite() || x.gt_abs(&1u32));
    if !c.is_nan() {
        // 0 <= acosu(x, u) <= u/2, a half turn, so the result never overflows
        assert!(c.is_finite());
        assert!(c >= 0u32);
        assert!(c <= Float::from_unsigned_prec_round(u, prec, Ceiling).0 >> 1u32);
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
        // acosu(x, u) + asinu(x, u) = u/4, since acos(x) + asin(x) is pi/2
        if u != 0 && !x.is_nan() {
            let w = prec + 64;
            let sum = x
                .acos_with_period_prec_ref(u, w)
                .0
                .add_prec(x.asin_with_period_prec_ref(u, w).0, w)
                .0;
            let quarter = Float::from_unsigned_prec(u, w).0 >> 2u32;
            let diff = sum.sub_prec(quarter.clone(), w).0;
            assert!(
                diff == 0u32
                    || i64::from(diff.get_exponent().unwrap())
                        < i64::from(quarter.get_exponent().unwrap()) + 10 - i64::exact_from(w),
                "acosu + asinu is not u/4 for {x} and {u}"
            );
        }
    }

    if o == Equal {
        assert!(acos_with_period_exact(&x, u, prec));
        for rm in exhaustive_rounding_modes() {
            let (c2, oo) = x.acos_with_period_prec_round_ref(u, prec, rm);
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.acos_with_period_prec_round_ref(u, prec, Exact));
    }
}

#[test]
fn acos_with_period_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_23().test_properties(
        |(x, u, prec, rm)| {
            acos_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_24().test_properties(
        |(x, u, prec, rm)| {
            acos_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // acosu(NaN, u) = acosu(±infinity, u) = NaN, as is acosu(x, u) for |x| > 1, in every
        // rounding mode and even when u is zero
        for x in [Float::NAN, Float::INFINITY, Float::NEGATIVE_INFINITY, Float::TWO] {
            for u in [0, 4] {
                let (c, o) = x.clone().acos_with_period_prec_round(u, prec, rm);
                assert!(c.is_nan());
                assert_eq!(o, Equal);
            }
        }
        // acosu(x, 0) = +0, exactly, since the arccosine is never negative
        for x in [Float::ZERO, Float::NEGATIVE_ZERO, Float::ONE, -Float::ONE] {
            let (c, o) = x.acos_with_period_prec_round(0, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
            assert_eq!(o, Equal);
        }
        // acosu(1, u) = +0, exactly
        let (c, o) = Float::ONE.acos_with_period_prec_round(8, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        // acosu(±0.0, u) = u/4, a quarter turn, and acosu(-1, u) = u/2, a half turn; both are
        // exact when `prec` holds them
        let (q, o_q) = Float::from_unsigned_prec_round(8u32, prec, rm);
        for (x, k) in [(Float::ZERO, 2u32), (Float::NEGATIVE_ZERO, 2), (-Float::ONE, 1)] {
            let (c, o) = x.acos_with_period_prec_round(8, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(q.clone() >> k));
            assert_eq!(o, o_q);
        }
        // acosu(1/2, u) = u/6 and acosu(-1/2, u) = u/3 when u is a multiple of 3
        let (q, o_q) = Float::from_unsigned_prec_round(4u32, prec, rm);
        for (x, k) in [(Float::ONE_HALF, 1u32), (-Float::ONE_HALF, 0)] {
            let (c, o) = x.acos_with_period_prec_round(12, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(q.clone() >> k));
            assert_eq!(o, o_q);
        }
    });
}

#[test]
fn acos_with_period_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, u, prec)| {
        let (c, o) = x.clone().acos_with_period_prec(u, prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.acos_with_period_prec_ref(u, prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acos_with_period_prec_round_ref(u, prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acos_with_period_prec_assign(u, prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn acos_with_period_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_44().test_properties(|(x, u, rm)| {
        if rm == Exact && !acos_with_period_exact(&x, u, x.significant_bits()) {
            assert_panic!(x.acos_with_period_round_ref(u, Exact));
            return;
        }
        let (c, o) = x.clone().acos_with_period_round(u, rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.acos_with_period_round_ref(u, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acos_with_period_prec_round_ref(u, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acos_with_period_round_assign(u, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn acos_with_period_properties() {
    float_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, u)| {
        let c = x.clone().acos_with_period(u);
        assert!(c.is_valid());
        let c_alt = x.acos_with_period_ref(u);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let mut c_alt = x.clone();
        c_alt.acos_with_period_assign(u);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let (c_alt, _) = x.acos_with_period_prec_round_ref(u, x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_acos_with_period() {
    fn test<T: PrimitiveFloat>(x: T, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_acos_with_period(x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 360, f32::NAN);
    test::<f32>(f32::INFINITY, 360, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, 360, f32::NAN);
    test::<f32>(2.0, 360, f32::NAN);
    test::<f32>(2.0, 0, f32::NAN);
    test::<f32>(0.25, 0, 0.0);
    test::<f32>(0.0, 360, 90.0);
    test::<f32>(-0.0, 360, 90.0);
    test::<f32>(1.0, 360, 0.0);
    test::<f32>(-1.0, 360, 180.0);
    test::<f32>(0.5, 360, 60.0);
    test::<f32>(-0.5, 360, 120.0);
    test::<f32>(0.5, 12, 2.0);
    test::<f32>(-0.5, 12, 4.0);
    test::<f32>(0.5, 7, 1.1666666);
    test::<f32>(0.25, 360, 75.52249);
    test::<f32>(-0.25, 360, 104.47751);
    test::<f32>(0.99, 360, 8.109611);
    test::<f32>(1.0e-30, 360, 90.0);
    test::<f32>(-1.0e-30, 1, 0.25);
    test::<f32>(1.0, 18446744073709551615, 0.0);
    test::<f32>(-1.0, 18446744073709551615, 9.223372e18);
    test::<f64>(f64::NAN, 360, f64::NAN);
    test::<f64>(f64::INFINITY, 360, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, 360, f64::NAN);
    test::<f64>(2.0, 360, f64::NAN);
    test::<f64>(2.0, 0, f64::NAN);
    test::<f64>(0.25, 0, 0.0);
    test::<f64>(0.0, 360, 90.0);
    test::<f64>(-0.0, 360, 90.0);
    test::<f64>(1.0, 360, 0.0);
    test::<f64>(-1.0, 360, 180.0);
    test::<f64>(0.5, 360, 60.0);
    test::<f64>(-0.5, 360, 120.0);
    test::<f64>(0.5, 12, 2.0);
    test::<f64>(-0.5, 12, 4.0);
    test::<f64>(0.5, 7, 1.1666666666666667);
    test::<f64>(0.25, 360, 75.52248781407008);
    test::<f64>(-0.25, 360, 104.47751218592992);
    test::<f64>(0.99, 360, 8.109614455994182);
    test::<f64>(1.0e-300, 360, 90.0);
    test::<f64>(-1.0e-300, 1, 0.25);
    test::<f64>(1.0, 18446744073709551615, 0.0);
    test::<f64>(-1.0, 18446744073709551615, 9.223372036854776e18);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acos_with_period_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        let c = primitive_float_acos_with_period(x, u);
        // NaN exactly for a NaN input, an infinite input, or an input outside [-1, 1]
        assert_eq!(c.is_nan(), x.is_nan() || !x.is_finite() || x.abs() > T::ONE);
        if !c.is_nan() {
            // the result lies in [0, u/2], so it never overflows
            assert!(c.is_finite());
            assert!(c >= T::ZERO);
            // the same as the `Float` version taken with 64 bits to spare and rounded once
            let (c_float, _) =
                Float::acos_with_period_prec(Float::from(x), u, T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&c_float, Nearest).0),
                NiceFloat(c)
            );
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // u = 0 gives a zero, except that an input outside [-1, 1] is still NaN
        assert_eq!(
            primitive_float_acos_with_period(x, 0).is_nan(),
            x.is_nan() || !x.is_finite() || x.abs() > T::ONE
        );
    });
}

#[test]
fn primitive_float_acos_with_period_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acos_with_period_properties_helper);
}

#[test]
fn test_acos_with_period_rational_prec_round() {
    let test = |s: &str,
                u: u64,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::acos_with_period_rational_prec_round(x.clone(), u, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = Float::acos_with_period_rational_prec_round_ref(&x, u, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = Float::acos_with_period_rational_prec(x.clone(), u, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
            let (c_alt, o_alt) = Float::acos_with_period_rational_prec_ref(&x, u, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u <= u64::from(u32::MAX)
        {
            let (rug_c, rug_o) = rug_acos_with_period_rational_prec_round(&x, u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 360, 10, Exact, "90.000", "0x5a.0#10", Equal);
    test("0", 7, 10, Nearest, "1.7500", "0x1.c00#10", Equal);
    test("0", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("2", 360, 10, Nearest, "NaN", "NaN", Equal);
    test("-2", 360, 10, Nearest, "NaN", "NaN", Equal);
    test("2", 0, 10, Nearest, "NaN", "NaN", Equal);
    test("100/99", 360, 10, Nearest, "NaN", "NaN", Equal);
    test("1", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test("1", 7, 1, Exact, "0.0", "0x0.0", Equal);
    test("-1", 360, 10, Exact, "180.00", "0xb4.0#10", Equal);
    test("-1", 7, 10, Exact, "3.5000", "0x3.80#10", Equal);
    test("-1", 7, 2, Floor, "3.0", "0x3.0#2", Less);
    test("-1", 7, 2, Ceiling, "4.0", "0x4.0#2", Greater);
    test("3/5", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("1/2", 360, 10, Exact, "60.000", "0x3c.0#10", Equal);
    test("-1/2", 360, 10, Exact, "120.00", "0x78.0#10", Equal);
    test("1/2", 12, 10, Exact, "2.0000", "0x2.00#10", Equal);
    test("-1/2", 12, 10, Exact, "4.0000", "0x4.00#10", Equal);
    test("1/2", 7, 10, Floor, "1.1660", "0x1.2a8#10", Less);
    test("1/2", 7, 10, Ceiling, "1.1680", "0x1.2b0#10", Greater);
    test("3/5", 360, 10, Floor, "53.125", "0x35.2#10", Less);
    test("3/5", 360, 10, Ceiling, "53.188", "0x35.3#10", Greater);
    test("3/5", 360, 10, Nearest, "53.125", "0x35.2#10", Less);
    test("-3/5", 360, 10, Nearest, "126.88", "0x7e.e#10", Greater);
    test(
        "3/5",
        360,
        53,
        Nearest,
        "53.130102354155980",
        "0x35.214e634c3b88#53",
        Greater,
    );
    test(
        "1/3",
        360,
        20,
        Nearest,
        "70.528809",
        "0x46.8760#20",
        Greater,
    );
    test(
        "999999/1000000",
        360,
        20,
        Nearest,
        "0.081028461",
        "0x0.14be48#20",
        Less,
    );
    test(
        "-999999/1000000",
        360,
        20,
        Nearest,
        "179.91895",
        "0xb3.eb4#20",
        Less,
    );
    test(
        "1/1000000",
        360,
        20,
        Nearest,
        "90.000000",
        "0x5a.0000#20",
        Greater,
    );
    test(
        "1/1000000",
        1,
        20,
        Nearest,
        "0.24999976",
        "0x0.3ffffc#20",
        Less,
    );
    test(
        "-1/1000000",
        1,
        20,
        Nearest,
        "0.25000000",
        "0x0.400000#20",
        Less,
    );
    test(
        "1/1000000",
        360,
        200,
        Nearest,
        "89.999942704220486908129826616327168565233158060263612159084030",
        "0x59.fffc3ebc8033eeff67b01f2d89426d6641c1778b1df3a4048#200",
        Greater,
    );
    test(
        "340282366920938463463374607431768211455/340282366920938463463374607431768211456",
        1,
        53,
        Nearest,
        "1.2201561323771008e-20",
        "0x3.99ec8537ccc42E-17#53",
        Less,
    );
    test("1", 18446744073709551615, 64, Exact, "0.0", "0x0.0", Equal);
    test(
        "-1",
        18446744073709551615,
        64,
        Exact,
        "9223372036854775807.50",
        "0x7fffffffffffffff.8#64",
        Equal,
    );
    test(
        "1/2",
        18446744073709551615,
        64,
        Exact,
        "3074457345618258602.50",
        "0x2aaaaaaaaaaaaaaa.8#64",
        Equal,
    );
    test(
        "0",
        18446744073709551615,
        64,
        Exact,
        "4611686018427387903.75",
        "0x3fffffffffffffff.c#64",
        Equal,
    );
}

#[test]
#[should_panic]
fn acos_with_period_rational_prec_round_fail_1() {
    Float::acos_with_period_rational_prec_round(Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn acos_with_period_rational_prec_round_fail_2() {
    // acos(1/4) is not an exact number of sevenths of a turn
    Float::acos_with_period_rational_prec_round(Rational::from_unsigneds(1u8, 4), 7, 10, Exact);
}

#[test]
#[should_panic]
fn acos_with_period_rational_prec_round_fail_3() {
    // acos(1/2) is a sixth of a turn, but 7 is not a multiple of 3
    Float::acos_with_period_rational_prec_round(Rational::ONE_HALF, 7, 10, Exact);
}

#[test]
#[should_panic]
fn acos_with_period_rational_prec_round_fail_4() {
    // a quarter turn needs more than 2 bits when u = 7
    Float::acos_with_period_rational_prec_round(Rational::ZERO, 7, 2, Exact);
}

#[test]
#[should_panic]
fn acos_with_period_rational_prec_round_ref_fail() {
    Float::acos_with_period_rational_prec_round_ref(&Rational::ONE, 7, 0, Floor);
}

#[test]
#[should_panic]
fn acos_with_period_rational_prec_fail() {
    Float::acos_with_period_rational_prec(Rational::ONE, 7, 0);
}

// A `Rational` within 2^(-2^31) of 1, where acos(x) is about sqrt(2(1 - x)) and falls below the
// smallest positive `Float`. This is the case the scaled path exists for: the arccosine alone
// underflows, but a large enough period lifts the quotient back into the range, so the square root
// has to be taken here rather than read off the underflowing arccosine. The `Rational` has about
// 2^31 bits, so each call costs a few seconds.
#[test]
fn test_acos_with_period_rational_underflow() {
    let x = Rational::ONE - Rational::power_of_2(-((1i64 << 31) + 4));
    // with u = 1 the quotient stays below the bottom of the range
    let (c, o) = Float::acos_with_period_rational_prec_round_ref(&x, 1, 53, Nearest);
    assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
    assert_eq!(o, Less);
    let (c, o) = Float::acos_with_period_rational_prec_round_ref(&x, 1, 53, Ceiling);
    assert_eq!(
        ComparableFloat(c),
        ComparableFloat(Float::min_positive_value_prec(53))
    );
    assert_eq!(o, Greater);
    // a large period lifts the quotient back into the range -- the case this path exists for -- and
    // doubling the period there doubles the result exactly
    let (c, o) = Float::acos_with_period_rational_prec_round_ref(&x, 1 << 40, 53, Nearest);
    assert!(c > Float::min_positive_value_prec(53));
    assert_ne!(o, Equal);
    let (c_alt, o_alt) = Float::acos_with_period_rational_prec_round_ref(&x, 1 << 41, 53, Nearest);
    assert_eq!(ComparableFloat(c_alt), ComparableFloat(c << 1u32));
    assert_eq!(o_alt, o);
}

// Whether acosu(x, u) is exactly representable at `prec` for a `Rational` x: at an input outside
// [-1, 1] (where the result is NaN), at u = 0, at x = 1, where the result is zero, and at the turn
// fractions -- a quarter at zero, a half at -1, and a sixth or a third at |x| = 1/2 with u a
// multiple of 3 -- each of which needs a `prec` wide enough to hold it.
fn acos_with_period_rational_exact(x: &Rational, u: u64, prec: u64) -> bool {
    x.gt_abs(&1u32)
        || u == 0
        || *x == 1u32
        || ((*x == 0u32 || *x == -1i32) && Float::from_unsigned_prec(u, prec).1 == Equal)
        || (x.eq_abs(&Rational::ONE_HALF)
            && u.is_multiple_of(3)
            && Float::from_unsigned_prec(u / 3, prec).1 == Equal)
}

#[allow(clippy::needless_pass_by_value)]
fn acos_with_period_rational_prec_round_properties_helper(
    x: Rational,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) {
    if rm == Exact && !acos_with_period_rational_exact(&x, u, prec) {
        assert_panic!(Float::acos_with_period_rational_prec_round_ref(
            &x, u, prec, Exact
        ));
        return;
    }
    let (c, o) = Float::acos_with_period_rational_prec_round(x.clone(), u, prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = Float::acos_with_period_rational_prec_round_ref(&x, u, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && u <= u64::from(u32::MAX)
    {
        let (rug_c, rug_o) = rug_acos_with_period_rational_prec_round(&x, u, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o, "x = {x} u = {u} prec = {prec} rm = {rm:?}");
    }

    // NaN exactly for an input outside [-1, 1]
    assert_eq!(c.is_nan(), x.gt_abs(&1u32));
    if !c.is_nan() {
        // 0 <= acosu(x, u) <= u/2, a half turn, so the result never overflows
        assert!(c.is_finite());
        assert!(c >= 0u32);
        assert!(c <= Float::from_unsigned_prec_round(u, prec, Ceiling).0 >> 1u32);
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
    }
    // a `Float` input agrees with the `Float` version
    if let Ok(f) = Float::try_from(&x) {
        let (c_alt, o_alt) = f.acos_with_period_prec_round(u, prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }

    if o == Equal {
        assert!(acos_with_period_rational_exact(&x, u, prec));
        for rm in exhaustive_rounding_modes() {
            let (c2, oo) = Float::acos_with_period_rational_prec_round_ref(&x, u, prec, rm);
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::acos_with_period_rational_prec_round_ref(
            &x, u, prec, Exact
        ));
    }
}

#[test]
fn acos_with_period_rational_prec_round_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(x, u, prec, rm)| {
            acos_with_period_rational_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // acosu(x, u) = NaN for |x| > 1, in every rounding mode and even when u is zero
        for u in [0, 4] {
            let (c, o) = Float::acos_with_period_rational_prec_round(Rational::TWO, u, prec, rm);
            assert!(c.is_nan());
            assert_eq!(o, Equal);
        }
        // acosu(x, 0) = +0, exactly, since the arccosine is never negative
        for x in [Rational::ZERO, Rational::ONE, Rational::NEGATIVE_ONE] {
            let (c, o) = Float::acos_with_period_rational_prec_round(x, 0, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
            assert_eq!(o, Equal);
        }
        // acosu(1, u) = +0, exactly
        let (c, o) = Float::acos_with_period_rational_prec_round(Rational::ONE, 8, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        // acosu(0, u) = u/4 and acosu(-1, u) = u/2, both exact when `prec` holds them
        let (q, o_q) = Float::from_unsigned_prec_round(8u32, prec, rm);
        for (x, k) in [(Rational::ZERO, 2u32), (Rational::NEGATIVE_ONE, 1)] {
            let (c, o) = Float::acos_with_period_rational_prec_round(x, 8, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(q.clone() >> k));
            assert_eq!(o, o_q);
        }
        // acosu(1/2, u) = u/6 and acosu(-1/2, u) = u/3 when u is a multiple of 3
        let (q, o_q) = Float::from_unsigned_prec_round(4u32, prec, rm);
        for (x, k) in [(Rational::ONE_HALF, 1u32), (-Rational::ONE_HALF, 0)] {
            let (c, o) = Float::acos_with_period_rational_prec_round(x, 12, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(q.clone() >> k));
            assert_eq!(o, o_q);
        }
    });
}

#[test]
fn acos_with_period_rational_prec_properties() {
    rational_unsigned_unsigned_rounding_mode_quadruple_gen_var_8().test_properties(
        |(x, u, prec, _)| {
            let (c, o) = Float::acos_with_period_rational_prec(x.clone(), u, prec);
            assert!(c.is_valid());
            let (c_alt, o_alt) = Float::acos_with_period_rational_prec_ref(&x, u, prec);
            assert!(c_alt.is_valid());
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
            let (c_alt, o_alt) =
                Float::acos_with_period_rational_prec_round_ref(&x, u, prec, Nearest);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        },
    );
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_acos_with_period_rational() {
    fn test<T: PrimitiveFloat>(s: &str, u: u64, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_acos_with_period_rational::<T>(
                &Rational::from_str(s).unwrap(),
                u
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 360, 90.0);
    test::<f32>("1", 360, 0.0);
    test::<f32>("-1", 360, 180.0);
    test::<f32>("2", 360, f32::NAN);
    test::<f32>("2", 0, f32::NAN);
    test::<f32>("3/5", 0, 0.0);
    test::<f32>("1/2", 360, 60.0);
    test::<f32>("-1/2", 360, 120.0);
    test::<f32>("1/2", 12, 2.0);
    test::<f32>("-1/2", 12, 4.0);
    test::<f32>("1/2", 7, 1.1666666);
    test::<f32>("3/5", 360, 53.130104);
    test::<f32>("-3/5", 360, 126.869896);
    test::<f32>("1/3", 360, 70.52878);
    test::<f32>("99/100", 360, 8.109614);
    test::<f32>("999999/1000000", 360, 0.08102848);
    test::<f32>("1/1000000", 360, 89.99994);
    test::<f32>("-1/1000000", 1, 0.25000015);
    test::<f32>("1", 18446744073709551615, 0.0);
    test::<f32>("-1", 18446744073709551615, 9.223372e18);
    test::<f64>("0", 360, 90.0);
    test::<f64>("1", 360, 0.0);
    test::<f64>("-1", 360, 180.0);
    test::<f64>("2", 360, f64::NAN);
    test::<f64>("2", 0, f64::NAN);
    test::<f64>("3/5", 0, 0.0);
    test::<f64>("1/2", 360, 60.0);
    test::<f64>("-1/2", 360, 120.0);
    test::<f64>("1/2", 12, 2.0);
    test::<f64>("-1/2", 12, 4.0);
    test::<f64>("1/2", 7, 1.1666666666666667);
    test::<f64>("3/5", 360, 53.13010235415598);
    test::<f64>("-3/5", 360, 126.86989764584402);
    test::<f64>("1/3", 360, 70.52877936550931);
    test::<f64>("99/100", 360, 8.109614455994178);
    test::<f64>("999999/1000000", 360, 0.08102847520651343);
    test::<f64>("1/1000000", 360, 89.99994270422049);
    test::<f64>("-1/1000000", 1, 0.2500001591549431);
    test::<f64>("1", 18446744073709551615, 0.0);
    test::<f64>("-1", 18446744073709551615, 9.223372036854776e18);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acos_with_period_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_unsigned_pair_gen_var_1::<u64>().test_properties(|(x, u)| {
        let c = primitive_float_acos_with_period_rational::<T>(&x, u);
        // NaN exactly for an input outside [-1, 1]
        assert_eq!(c.is_nan(), x.gt_abs(&1u32));
        if !c.is_nan() {
            // the result lies in [0, u/2], so it never overflows
            assert!(c.is_finite());
            assert!(c >= T::ZERO);
        }
        // the same as the `Float` version taken with 64 bits to spare and rounded once
        let (c_float, _) = Float::acos_with_period_rational_prec_ref(&x, u, T::MANTISSA_WIDTH + 64);
        assert_eq!(
            NiceFloat(T::rounding_from(&c_float, Nearest).0),
            NiceFloat(c)
        );
    });

    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        // The arccosine of a finite primitive float, taken through the `Rational` path, matches the
        // direct primitive-float arccosine. A zero is excluded, since `Rational` has no signed
        // zeros, though here both give the same quarter turn.
        if x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_acos_with_period_rational::<T>(
                    &Rational::exact_from(x),
                    u
                )),
                NiceFloat(primitive_float_acos_with_period(x, u))
            );
        }
    });
}

#[test]
fn primitive_float_acos_with_period_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acos_with_period_rational_properties_helper);
}

#[test]
fn test_acos_pi_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().acos_pi_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.acos_pi_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.acos_pi_prec_round_assign(prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = x.acos_pi_prec_ref(prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        // acosu with u = 2, which MPFR's acos_u gives directly
        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) =
                rug_acos_with_period_prec_round(&rug::Float::exact_from(&x), 2, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("-Infinity", "-Infinity", 10, Nearest, "NaN", "NaN", Equal);
    test("0.0", "0x0.0", 1, Exact, "0.50", "0x0.8#1", Equal);
    test("0.0", "0x0.0", 10, Exact, "0.50000", "0x0.800#10", Equal);
    test("-0.0", "-0x0.0", 10, Exact, "0.50000", "0x0.800#10", Equal);
    test("2.0", "0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("-2.0", "-0x2.0#1", 10, Nearest, "NaN", "NaN", Equal);
    test("1.0", "0x1.0#1", 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 1, Exact, "0.0", "0x0.0", Equal);
    test("-1.0", "-0x1.0#1", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("-1.0", "-0x1.0#1", 1, Exact, "1.0", "0x1.0#1", Equal);
    test("0.50", "0x0.8#1", 10, Floor, "0.33301", "0x0.554#10", Less);
    test(
        "0.50",
        "0x0.8#1",
        10,
        Ceiling,
        "0.33350",
        "0x0.556#10",
        Greater,
    );
    test(
        "0.50",
        "0x0.8#1",
        53,
        Nearest,
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Less,
    );
    test(
        "-0.50",
        "-0x0.8#1",
        53,
        Nearest,
        "0.66666666666666663",
        "0x0.aaaaaaaaaaaaa8#53",
        Less,
    );
    test("0.25", "0x0.4#1", 10, Floor, "0.41943", "0x0.6b6#10", Less);
    test(
        "0.25",
        "0x0.4#1",
        10,
        Ceiling,
        "0.41992",
        "0x0.6b8#10",
        Greater,
    );
    test(
        "0.25",
        "0x0.4#1",
        10,
        Nearest,
        "0.41943",
        "0x0.6b6#10",
        Less,
    );
    test(
        "0.25",
        "0x0.4#1",
        53,
        Nearest,
        "0.41956937674483374",
        "0x0.6b68e60f85ac88#53",
        Less,
    );
    test(
        "-0.25",
        "-0x0.4#1",
        53,
        Nearest,
        "0.58043062325516626",
        "0x0.949719f07a5378#53",
        Greater,
    );
    test(
        "0.99902",
        "0x0.ffc#10",
        20,
        Nearest,
        "0.014068589",
        "0x0.0399ffc#20",
        Greater,
    );
    test(
        "-0.99902",
        "-0x0.ffc#10",
        20,
        Nearest,
        "0.98593140",
        "0x0.fc660#20",
        Less,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        20,
        Nearest,
        "0.50000000",
        "0x0.80000#20",
        Greater,
    );
    test(
        "7.9e-31",
        "0x1.0E-25#1",
        53,
        Nearest,
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Greater,
    );
    test(
        "-7.9e-31",
        "-0x1.0E-25#1",
        53,
        Nearest,
        "0.50000000000000000",
        "0x0.80000000000000#53",
        Less,
    );
}

#[test]
fn test_acos_pi_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::acos_pi_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = Float::acos_pi_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = Float::acos_pi_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
            let (c_alt, o_alt) = Float::acos_pi_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acos_with_period_rational_prec_round(&x, 2, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("0", 10, Exact, "0.50000", "0x0.800#10", Equal);
    test("0", 1, Exact, "0.50", "0x0.8#1", Equal);
    test("2", 10, Nearest, "NaN", "NaN", Equal);
    test("-2", 10, Nearest, "NaN", "NaN", Equal);
    test("1", 10, Exact, "0.0", "0x0.0", Equal);
    test("-1", 10, Exact, "1.0000", "0x1.000#10", Equal);
    test("-1", 1, Exact, "1.0", "0x1.0#1", Equal);
    test("1/2", 10, Floor, "0.33301", "0x0.554#10", Less);
    test("1/2", 10, Ceiling, "0.33350", "0x0.556#10", Greater);
    test(
        "1/2",
        53,
        Nearest,
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Less,
    );
    test(
        "-1/2",
        53,
        Nearest,
        "0.66666666666666663",
        "0x0.aaaaaaaaaaaaa8#53",
        Less,
    );
    test("3/5", 10, Floor, "0.29492", "0x0.4b8#10", Less);
    test("3/5", 10, Ceiling, "0.29541", "0x0.4ba#10", Greater);
    test(
        "3/5",
        53,
        Nearest,
        "0.29516723530086653",
        "0x0.4b90147677cc20#53",
        Less,
    );
    test(
        "-3/5",
        53,
        Nearest,
        "0.70483276469913347",
        "0x0.b46feb898833e0#53",
        Greater,
    );
    test("1/3", 20, Nearest, "0.39182663", "0x0.644ec0#20", Greater);
    test(
        "999999/1000000",
        20,
        Nearest,
        "0.00045015803",
        "0x0.001d8066#20",
        Less,
    );
    test(
        "1/1000000",
        20,
        Nearest,
        "0.49999952",
        "0x0.7ffff8#20",
        Less,
    );
}

#[test]
#[should_panic]
fn acos_pi_prec_round_fail_1() {
    Float::ONE.acos_pi_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn acos_pi_prec_round_fail_2() {
    // acos(1/4)/pi is not exactly representable
    Float::from(0.25).acos_pi_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn acos_pi_rational_prec_round_fail() {
    Float::acos_pi_rational_prec_round(Rational::from_unsigneds(1u8, 4), 10, Exact);
}

#[test]
fn acos_pi_properties() {
    // The borrowed generators admit `Exact` for inputs whose arccosine is not exact, so `Exact` is
    // checked against the exactness of the result.
    let exact_ok = |x: &Float, prec: u64, rm: RoundingMode| {
        rm != Exact || x.acos_with_period_prec_round_ref(2, prec, Nearest).1 == Equal
    };
    float_unsigned_rounding_mode_triple_gen_var_42().test_properties(|(x, prec, rm)| {
        if !exact_ok(&x, prec, rm) {
            assert_panic!(x.acos_pi_prec_round_ref(prec, Exact));
            return;
        }
        let (c, o) = x.clone().acos_pi_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.acos_with_period_prec_round_ref(2, prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acos_pi_prec_round_ref(prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acos_pi_prec_round_assign(prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        // 0 <= acos(x)/pi <= 1, so the result never overflows
        if !c.is_nan() {
            assert!(c.is_finite());
            assert!(c >= 0u32);
            assert!(c <= 1u32);
        }
        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) =
                rug_acos_with_period_prec_round(&rug::Float::exact_from(&x), 2, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    });

    float_unsigned_rounding_mode_triple_gen_var_43().test_properties(|(x, prec, rm)| {
        if !exact_ok(&x, prec, rm) {
            return;
        }
        let (c, o) = x.acos_pi_prec_round_ref(prec, rm);
        let (c_alt, o_alt) = x.acos_with_period_prec_round_ref(2, prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });

    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (c, o) = x.clone().acos_pi_prec(prec);
        let (c_alt, o_alt) = x.acos_with_period_prec_ref(2, prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acos_pi_prec_ref(prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acos_pi_prec_assign(prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });

    float_rounding_mode_pair_gen_var_49().test_properties(|(x, rm)| {
        if !exact_ok(&x, x.significant_bits(), rm) {
            return;
        }
        let (c, o) = x.clone().acos_pi_round(rm);
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.acos_with_period_round_ref(2, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.acos_pi_round_ref(rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.acos_pi_round_assign(rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });

    rational_unsigned_rounding_mode_triple_gen_var_11().test_properties(|(x, prec, rm)| {
        if rm == Exact && Float::acos_with_period_rational_prec_ref(&x, 2, prec).1 != Equal {
            assert_panic!(Float::acos_pi_rational_prec_round_ref(&x, prec, Exact));
            return;
        }
        let (c, o) = Float::acos_pi_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = Float::acos_with_period_rational_prec_round_ref(&x, 2, prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = Float::acos_pi_rational_prec_round_ref(&x, prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_acos_with_period_rational_prec_round(&x, 2, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (c, o) = Float::acos_pi_rational_prec(x.clone(), prec);
        let (c_alt, o_alt) = Float::acos_with_period_rational_prec_ref(&x, 2, prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = Float::acos_pi_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });

    float_gen().test_properties(|x| {
        let c = x.clone().acos_pi();
        assert!(c.is_valid());
        let c_alt = x.acos_pi_ref();
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let mut c_alt = x.clone();
        c_alt.acos_pi_assign();
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        // the same as acos_with_period with a period of 2, and as rounding to the input's
        // precision, to nearest
        let c_alt = x.acos_with_period_ref(2);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let (c_alt, _) = x.acos_pi_prec_round_ref(x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_acos_pi() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_acos_pi(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::NAN);
    test::<f32>(f32::NEGATIVE_INFINITY, f32::NAN);
    test::<f32>(2.0, f32::NAN);
    test::<f32>(-2.0, f32::NAN);
    test::<f32>(0.0, 0.5);
    test::<f32>(-0.0, 0.5);
    test::<f32>(1.0, 0.0);
    test::<f32>(-1.0, 1.0);
    test::<f32>(0.5, 0.33333334);
    test::<f32>(-0.5, 0.6666667);
    test::<f32>(0.25, 0.41956937);
    test::<f32>(0.99, 0.045053393);
    test::<f32>(1.0e-30, 0.5);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::NAN);
    test::<f64>(f64::NEGATIVE_INFINITY, f64::NAN);
    test::<f64>(2.0, f64::NAN);
    test::<f64>(-2.0, f64::NAN);
    test::<f64>(0.0, 0.5);
    test::<f64>(-0.0, 0.5);
    test::<f64>(1.0, 0.0);
    test::<f64>(-1.0, 1.0);
    test::<f64>(0.5, 0.3333333333333333);
    test::<f64>(-0.5, 0.6666666666666666);
    test::<f64>(0.25, 0.41956937674483374);
    test::<f64>(0.99, 0.04505341364441212);
    test::<f64>(1.0e-300, 0.5);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_acos_pi_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_acos_pi_rational::<T>(
                &Rational::from_str(s).unwrap()
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 0.5);
    test::<f32>("1", 0.0);
    test::<f32>("-1", 1.0);
    test::<f32>("2", f32::NAN);
    test::<f32>("1/2", 0.33333334);
    test::<f32>("-1/2", 0.6666667);
    test::<f32>("3/5", 0.29516724);
    test::<f32>("1/3", 0.39182654);
    test::<f32>("999999/1000000", 0.0004501582);
    test::<f32>("1/1000000", 0.49999967);
    test::<f64>("0", 0.5);
    test::<f64>("1", 0.0);
    test::<f64>("-1", 1.0);
    test::<f64>("2", f64::NAN);
    test::<f64>("1/2", 0.3333333333333333);
    test::<f64>("-1/2", 0.6666666666666666);
    test::<f64>("3/5", 0.2951672353008665);
    test::<f64>("1/3", 0.3918265520306073);
    test::<f64>("999999/1000000", 0.0004501581955917413);
    test::<f64>("1/1000000", 0.49999968169011383);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_acos_pi_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        // the same as the period-2 version
        assert_eq!(
            NiceFloat(primitive_float_acos_pi(x)),
            NiceFloat(primitive_float_acos_with_period(x, 2))
        );
    });

    rational_gen().test_properties(|x| {
        assert_eq!(
            NiceFloat(primitive_float_acos_pi_rational::<T>(&x)),
            NiceFloat(primitive_float_acos_with_period_rational::<T>(&x, 2))
        );
    });
}

#[test]
fn primitive_float_acos_pi_properties() {
    apply_fn_to_primitive_floats!(primitive_float_acos_pi_properties_helper);
}
