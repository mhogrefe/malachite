// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Acos, AcosAssign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, Two, Zero,
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
use malachite_float::float::arithmetic::acos::primitive_float_acos;
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::acos::{rug_acos, rug_acos_prec_round};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_49, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_42, float_unsigned_rounding_mode_triple_gen_var_43,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use std::panic::catch_unwind;

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
