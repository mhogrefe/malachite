// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Asec, AsecAssign, PowerOf2, Reciprocal};
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
use malachite_float::float::arithmetic::asec::{
    primitive_float_asec, primitive_float_asec_rational, primitive_float_asec_with_period,
};
use malachite_float::test_util::common::{
    assert_rounding_ordering_consistent, parse_hex_string, rug_round_try_from_rounding_mode,
    to_hex_string,
};
use malachite_float::test_util::float::arithmetic::asec::{
    rug_asec, rug_asec_prec_round, rug_asec_rational_prec_round, rug_asec_with_period_prec_round,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_50, float_unsigned_pair_gen_var_1,
    float_unsigned_pair_gen_var_2, float_unsigned_rounding_mode_triple_gen_var_45,
    float_unsigned_rounding_mode_triple_gen_var_46, float_unsigned_rounding_mode_triple_gen_var_47,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_25,
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_26,
    float_unsigned_unsigned_triple_gen_var_1, rational_unsigned_rounding_mode_triple_gen_var_12,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;
use std::str::FromStr;

// The arcsecant of x is the arccosine of its reciprocal, and a `Float`'s reciprocal is an exact
// `Rational`, so the `Rational` arccosine gives the same correctly rounded answer. That identity is
// the strongest check available here, MPFR having no arcsecant of its own.
//
// It is only applied to inputs of moderate magnitude: the `Rational` holds the `Float`'s exponent
// in full, so a `Float` with an extreme exponent would turn into a `Rational` of hundreds of
// megabytes.
fn acos_of_reciprocal(x: &Float, prec: u64, rm: RoundingMode) -> Option<(Float, Ordering)> {
    if !x.is_finite() || *x == 0u32 || x.get_exponent().unwrap().unsigned_abs() > 1000 {
        return None;
    }
    Some(Float::acos_rational_prec_round(
        Rational::exact_from(x).reciprocal(),
        prec,
        rm,
    ))
}

#[test]
fn test_asec_prec_round() {
    let test = |s: &str,
                s_hex: &str,
                prec: u64,
                rm: RoundingMode,
                out: &str,
                out_hex: &str,
                o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (c, o) = x.clone().asec_prec_round(prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.asec_prec_round_ref(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.asec_prec_round_assign(prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = x.asec_prec_ref(prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        // the arccosine of the exact reciprocal, which is the same real number
        if let Some((c_alt, o_alt)) = acos_of_reciprocal(&x, prec, rm) {
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_asec_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 10, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 1, Nearest, "2.0", "0x2.0#1", Greater,
    );
    test(
        "Infinity",
        "Infinity",
        10,
        Floor,
        "1.5703",
        "0x1.920#10",
        Less,
    );
    test(
        "Infinity",
        "Infinity",
        10,
        Ceiling,
        "1.5723",
        "0x1.928#10",
        Greater,
    );
    test(
        "-Infinity",
        "-Infinity",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test("0.0", "0x0.0", 10, Exact, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 10, Exact, "NaN", "NaN", Equal);
    test("-0.50", "-0x0.8#1", 10, Nearest, "NaN", "NaN", Equal);
    test("0.99902", "0x0.ffc#10", 10, Nearest, "NaN", "NaN", Equal);
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
    test(
        "-1.0",
        "-0x1.0#1",
        53,
        Nearest,
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        Less,
    );
    test("2.0", "0x2.0#1", 10, Floor, "1.0469", "0x1.0c0#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "1.0488",
        "0x1.0c8#10",
        Greater,
    );
    test("2.0", "0x2.0#1", 10, Nearest, "1.0469", "0x1.0c0#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        53,
        Nearest,
        "1.0471975511965979",
        "0x1.0c152382d7366#53",
        Greater,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        53,
        Nearest,
        "2.0943951023931957",
        "0x2.182a4705ae6cc#53",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        20,
        Nearest,
        "0.84106827",
        "0x0.d7504#20",
        Less,
    );
    test(
        "-1.5",
        "-0x1.8#2",
        20,
        Nearest,
        "2.3005257",
        "0x2.4cef4#20",
        Greater,
    );
    test(
        "1.0010",
        "0x1.004#11",
        20,
        Nearest,
        "0.044176221",
        "0x0.0b4f22#20",
        Greater,
    );
    test(
        "1.0010",
        "0x1.004#11",
        100,
        Nearest,
        "0.044176202487635610232666265452557",
        "0x0.0b4f21b0f2247500b4ff3d6475#100",
        Greater,
    );
    test(
        "-1.0010",
        "-0x1.004#11",
        20,
        Nearest,
        "3.0974159",
        "0x3.18f04#20",
        Less,
    );
    test(
        "1.60000000000000000002",
        "0x1.999999999999999a#64",
        64,
        Nearest,
        "0.895664793857864972024",
        "0x0.e54a49b5cc510cd7#64",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        20,
        Nearest,
        "1.5707970",
        "0x1.921fc#20",
        Greater,
    );
    test(
        "-1.3e30",
        "-0x1.0E+25#1",
        20,
        Nearest,
        "1.5707970",
        "0x1.921fc#20",
        Greater,
    );
    test(
        "2.6e323228495",
        "0x1.0E+268435455#1",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    // - the Ziv loop retries at least once
    test("-1.2", "-0x1.4#3", 2, Nearest, "2.0", "0x2.0#2", Less);
    test("-1.2", "-0x1.4#3", 3, Down, "2.0", "0x2.0#3", Less);
    test("-1.2", "-0x1.4#3", 3, Nearest, "2.5", "0x2.8#3", Greater);
    test(
        "-2.570",
        "-0x2.92#9",
        24,
        Nearest,
        "1.97040486",
        "0x1.f86c74#24",
        Greater,
    );
}

#[test]
#[should_panic]
fn asec_prec_round_fail_1() {
    Float::ONE.asec_prec_round(0, Floor);
}

#[test]
#[should_panic]
fn asec_prec_round_fail_2() {
    // asec(2) = pi/3 is not exactly representable
    Float::TWO.asec_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn asec_prec_round_fail_3() {
    // asec(-1) = pi is not exactly representable either
    Float::NEGATIVE_ONE.asec_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn asec_prec_round_fail_4() {
    // an infinite input gives pi/2, which is not exactly representable
    Float::INFINITY.asec_prec_round(10, Exact);
}

#[test]
#[should_panic]
fn asec_prec_round_ref_fail() {
    Float::ONE.asec_prec_round_ref(0, Floor);
}

#[test]
#[should_panic]
fn asec_prec_fail() {
    Float::ONE.asec_prec(0);
}

#[test]
#[should_panic]
fn asec_round_fail() {
    Float::TWO.asec_round(Exact);
}

// Whether asec(x) is exactly representable at `prec`: at NaN and at any $|x| < 1$, where the result
// is NaN, and at x = 1, where it is zero. An infinite input gives pi/2 and x = -1 gives pi, neither
// of which is ever representable.
fn asec_exact(x: &Float) -> bool {
    x.is_nan() || x.lt_abs(&1u32) || *x == 1u32
}

#[allow(clippy::needless_pass_by_value)]
fn asec_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    if rm == Exact && !asec_exact(&x) {
        assert_panic!(x.asec_prec_round_ref(prec, Exact));
        return;
    }
    let (c, o) = x.clone().asec_prec_round(prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.asec_prec_round_ref(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut c_alt = x.clone();
    let o_alt = c_alt.asec_prec_round_assign(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // the arccosine of the exact reciprocal, which is the same real number
    if let Some((c_alt, o_alt)) = acos_of_reciprocal(&x, prec, rm) {
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o, "x = {x} prec = {prec} rm = {rm:?}");
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_asec_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // NaN exactly for a NaN input or one inside (-1, 1)
    assert_eq!(c.is_nan(), x.is_nan() || x.lt_abs(&1u32));
    if !c.is_nan() {
        // 0 <= asec(x) <= pi, so the result never overflows, and it is zero only at x = 1
        assert!(c.is_finite());
        assert!(c >= 0u32);
        assert!(c <= Float::pi_prec_round(prec, Ceiling).0);
        assert_eq!(c == 0u32, x == 1u32);
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
        // asec(-x) + asec(x) = pi
        let w = prec + 64;
        let sum = (-&x).asec_prec_ref(w).0.add_prec(x.asec_prec_ref(w).0, w).0;
        let diff = sum.sub_prec(Float::pi_prec(w).0, w).0;
        assert!(
            diff == 0u32 || i64::from(diff.get_exponent().unwrap()) < 10 - i64::exact_from(w),
            "asec(-x) + asec(x) is not pi for {x}"
        );
    }

    if o == Equal {
        assert!(asec_exact(&x));
        for rm in exhaustive_rounding_modes() {
            let (c2, oo) = x.asec_prec_round_ref(prec, rm);
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.asec_prec_round_ref(prec, Exact));
    }
}

#[test]
fn asec_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_45().test_properties(|(x, prec, rm)| {
        asec_prec_round_properties_helper(x, prec, rm);
    });

    float_unsigned_rounding_mode_triple_gen_var_46().test_properties(|(x, prec, rm)| {
        asec_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // asec(NaN) = NaN, and so is asec(x) for |x| < 1, both zeros included
        for x in [Float::NAN, Float::ZERO, Float::NEGATIVE_ZERO, Float::ONE_HALF, -Float::ONE_HALF]
        {
            let (c, o) = x.asec_prec_round(prec, rm);
            assert!(c.is_nan());
            assert_eq!(o, Equal);
        }
        // asec(1) = +0, exactly
        let (c, o) = Float::ONE.asec_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        if rm == Exact {
            // pi and pi/2 are never exactly representable
            assert_panic!(Float::NEGATIVE_ONE.asec_prec_round(prec, Exact));
            assert_panic!(Float::INFINITY.asec_prec_round(prec, Exact));
            assert_panic!(Float::NEGATIVE_INFINITY.asec_prec_round(prec, Exact));
            return;
        }
        // asec(-1) = pi, and asec(±infinity) = pi/2
        let (pi, o_pi) = Float::pi_prec_round(prec, rm);
        let (c, o) = Float::NEGATIVE_ONE.asec_prec_round(prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(pi.clone()));
        assert_eq!(o, o_pi);
        for x in [Float::INFINITY, Float::NEGATIVE_INFINITY] {
            let (c, o) = x.asec_prec_round(prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(pi.clone() >> 1u32));
            assert_eq!(o, o_pi);
        }
    });
}

#[test]
fn asec_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (c, o) = x.clone().asec_prec(prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.asec_prec_ref(prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.asec_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.asec_prec_assign(prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn asec_round_properties() {
    float_rounding_mode_pair_gen_var_50().test_properties(|(x, rm)| {
        let (c, o) = x.clone().asec_round(rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.asec_round_ref(rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.asec_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.asec_round_assign(rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn asec_properties() {
    float_gen().test_properties(|x| {
        let c = x.clone().asec();
        assert!(c.is_valid());
        let c_alt = (&x).asec();
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let mut c_alt = x.clone();
        c_alt.asec_assign();
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let (c_alt, _) = x.asec_prec_round_ref(x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let rug_c = rug_asec(&rug::Float::exact_from(&x));
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
    });
}

#[test]
// The table pins the correctly rounded arcsecants of a few inputs, some of which are the pi
// constants; naming the constants instead would not test the rounding.
#[allow(clippy::approx_constant, clippy::type_repetition_in_bounds)]
fn test_primitive_float_asec() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_asec(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, 1.5707964);
    test::<f32>(f32::NEGATIVE_INFINITY, 1.5707964);
    test::<f32>(0.0, f32::NAN);
    test::<f32>(-0.0, f32::NAN);
    test::<f32>(0.5, f32::NAN);
    test::<f32>(-0.5, f32::NAN);
    test::<f32>(1.0, 0.0);
    test::<f32>(-1.0, 3.1415927);
    test::<f32>(1.5, 0.8410687);
    test::<f32>(-1.5, 2.300524);
    test::<f32>(2.0, 1.0471976);
    test::<f32>(-2.0, 2.0943952);
    test::<f32>(100.0, 1.5607961);
    test::<f32>(1.0009766, 0.044176202);
    test::<f32>(1.0e30, 1.5707964);
    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, 1.5707963267948966);
    test::<f64>(f64::NEGATIVE_INFINITY, 1.5707963267948966);
    test::<f64>(0.0, f64::NAN);
    test::<f64>(-0.0, f64::NAN);
    test::<f64>(0.5, f64::NAN);
    test::<f64>(-0.5, f64::NAN);
    test::<f64>(1.0, 0.0);
    test::<f64>(-1.0, 3.141592653589793);
    test::<f64>(1.5, 0.8410686705679302);
    test::<f64>(-1.5, 2.300523983021863);
    test::<f64>(2.0, 1.0471975511965979);
    test::<f64>(-2.0, 2.0943951023931957);
    test::<f64>(100.0, 1.5607961601207294);
    test::<f64>(1.0009765625, 0.04417620248763561);
    test::<f64>(1.0e300, 1.5707963267948966);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_asec_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let c = primitive_float_asec(x);
        // NaN exactly for a NaN input or one inside (-1, 1)
        assert_eq!(c.is_nan(), x.is_nan() || x.abs() < T::ONE);
        if !c.is_nan() {
            // the result lies in [0, pi], so it never overflows
            assert!(c.is_finite());
            assert!(c >= T::ZERO);
            // the same as the `Float` version taken with 64 bits to spare and rounded once
            let (c_float, _) = Float::asec_prec(Float::from(x), T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&c_float, Nearest).0),
                NiceFloat(c)
            );
        }
    });
}

#[test]
fn primitive_float_asec_properties() {
    apply_fn_to_primitive_floats!(primitive_float_asec_properties_helper);
}

#[test]
fn test_asec_rational_prec_round() {
    let test = |s: &str, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o_out: Ordering| {
        let x = Rational::from_str(s).unwrap();

        let (c, o) = Float::asec_rational_prec_round(x.clone(), prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = Float::asec_rational_prec_round_ref(&x, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = Float::asec_rational_prec(x.clone(), prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
            let (c_alt, o_alt) = Float::asec_rational_prec_ref(&x, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        // the arccosine of the exact reciprocal, which is the same real number
        if x != 0u32 {
            let (c_alt, o_alt) = Float::acos_rational_prec_round(x.clone().reciprocal(), prec, rm);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_c, rug_o) = rug_asec_rational_prec_round(&x, prec, rug_rm);
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
    test("999999/1000000", 10, Nearest, "NaN", "NaN", Equal);
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
    test("2", 10, Floor, "1.0469", "0x1.0c0#10", Less);
    test("2", 10, Ceiling, "1.0488", "0x1.0c8#10", Greater);
    test("2", 10, Nearest, "1.0469", "0x1.0c0#10", Less);
    test(
        "2",
        53,
        Nearest,
        "1.0471975511965979",
        "0x1.0c152382d7366#53",
        Greater,
    );
    test(
        "-2",
        53,
        Nearest,
        "2.0943951023931957",
        "0x2.182a4705ae6cc#53",
        Greater,
    );
    test("5/3", 10, Floor, "0.92676", "0x0.ed4#10", Less);
    test("5/3", 10, Ceiling, "0.92773", "0x0.ed8#10", Greater);
    test(
        "5/3",
        53,
        Nearest,
        "0.92729521800161219",
        "0x0.ed63382b0dda78#53",
        Less,
    );
    test(
        "-5/3",
        53,
        Nearest,
        "2.2142974355881808",
        "0x2.36dc325d77c88#53",
        Less,
    );
    test(
        "1000001/1000000",
        20,
        Nearest,
        "0.0014142133",
        "0x0.005cae90#20",
        Greater,
    );
    test(
        "1000001/1000000",
        100,
        Nearest,
        "0.0014142129731178241296276027330038",
        "0x0.005cae8e78183f43b1be300096a0#100",
        Less,
    );
    test(
        "-1000001/1000000",
        20,
        Nearest,
        "3.1401787",
        "0x3.23e2c#20",
        Greater,
    );
    test("1000000", 20, Nearest, "1.5707951", "0x1.921fa#20", Less);
    test(
        "-1000000",
        53,
        Nearest,
        "1.5707973267948967",
        "0x1.921fc60b3a724#53",
        Greater,
    );
    test(
        "1000000000000000000000000000001/1000000000000000000000000000000",
        53,
        Nearest,
        "1.4142135623730951e-15",
        "0x6.5e7a2ba008448E-13#53",
        Greater,
    );
    test(
        "340282366920938463463374607431768211456",
        53,
        Nearest,
        "1.5707963267948966",
        "0x1.921fb54442d18#53",
        Less,
    );
    test(
        "115792089237316195423570985008687907853269984665640564039457584007913129639936",
        20,
        Nearest,
        "1.5707970",
        "0x1.921fc#20",
        Greater,
    );
}

#[test]
#[should_panic]
fn asec_rational_prec_round_fail_1() {
    Float::asec_rational_prec_round(Rational::TWO, 0, Floor);
}

#[test]
#[should_panic]
fn asec_rational_prec_round_fail_2() {
    // asec(2) = pi/3 is not exactly representable
    Float::asec_rational_prec_round(Rational::TWO, 10, Exact);
}

#[test]
#[should_panic]
fn asec_rational_prec_round_fail_3() {
    // asec(-1) = pi is not exactly representable either
    Float::asec_rational_prec_round(Rational::NEGATIVE_ONE, 10, Exact);
}

#[test]
#[should_panic]
fn asec_rational_prec_round_ref_fail() {
    Float::asec_rational_prec_round_ref(&Rational::TWO, 0, Floor);
}

#[test]
#[should_panic]
fn asec_rational_prec_fail() {
    Float::asec_rational_prec(Rational::TWO, 0);
}

// A `Rational` within 2^(-2^31) of 1, where asec(x) is about sqrt(2(x - 1)) and falls below the
// smallest positive `Float` -- a regime the `Float` arcsecant cannot reach, since it would need an
// input of more than 2^31 bits. The `Rational` here has about that many, so each call costs a few
// seconds.
#[test]
fn test_asec_rational_underflow() {
    let min_positive = Float::min_positive_value_prec(53);
    let x = Rational::ONE + Rational::power_of_2(-((1i64 << 31) + 4));
    for (rm, expected, o_out) in
        [(Nearest, Float::ZERO, Less), (Ceiling, min_positive.clone(), Greater)]
    {
        let (c, o) = Float::asec_rational_prec_round_ref(&x, 53, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(expected), "rm = {rm:?}");
        assert_eq!(o, o_out, "rm = {rm:?}");
    }
    // two exponents up, the same path returns a representable value instead
    let x = Rational::ONE + Rational::power_of_2(-(1i64 << 31));
    let (c, o) = Float::asec_rational_prec_round_ref(&x, 53, Nearest);
    assert!(c > min_positive);
    assert_eq!(o, Greater);
}

// Whether asec(x) is exactly representable for a `Rational` x: inside (-1, 1), where the result is
// NaN, and at x = 1, where it is zero. The arcsecant of -1 is pi, which is never representable.
fn asec_rational_exact(x: &Rational) -> bool {
    x.lt_abs(&1u32) || *x == 1u32
}

#[allow(clippy::needless_pass_by_value)]
fn asec_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    if rm == Exact && !asec_rational_exact(&x) {
        assert_panic!(Float::asec_rational_prec_round_ref(&x, prec, Exact));
        return;
    }
    let (c, o) = Float::asec_rational_prec_round(x.clone(), prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = Float::asec_rational_prec_round_ref(&x, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // the arccosine of the exact reciprocal, which is the same real number
    if x != 0u32 {
        let (c_alt, o_alt) = Float::acos_rational_prec_round(x.clone().reciprocal(), prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o, "x = {x} prec = {prec} rm = {rm:?}");
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_c, rug_o) = rug_asec_rational_prec_round(&x, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // NaN exactly for an input inside (-1, 1)
    assert_eq!(c.is_nan(), x.lt_abs(&1u32));
    if !c.is_nan() {
        // 0 <= asec(x) <= pi, so the result never overflows, and it is zero only at x = 1
        assert!(c.is_finite());
        assert!(c >= 0u32);
        assert!(c <= Float::pi_prec_round(prec, Ceiling).0);
        assert_eq!(c == 0u32, x == 1u32);
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
    }
    // a `Float` input agrees with the `Float` version
    if let Ok(f) = Float::try_from(&x) {
        let (c_alt, o_alt) = f.asec_prec_round(prec, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    }

    if o == Equal {
        assert!(asec_rational_exact(&x));
        for rm in exhaustive_rounding_modes() {
            let (c2, oo) = Float::asec_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::asec_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn asec_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_12().test_properties(|(x, prec, rm)| {
        asec_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // asec(x) = NaN inside (-1, 1), zero included
        for x in [Rational::ZERO, Rational::ONE_HALF, -Rational::ONE_HALF] {
            let (c, o) = Float::asec_rational_prec_round(x, prec, rm);
            assert!(c.is_nan());
            assert_eq!(o, Equal);
        }
        // asec(1) = +0, exactly
        let (c, o) = Float::asec_rational_prec_round(Rational::ONE, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        if rm == Exact {
            // pi is never exactly representable
            assert_panic!(Float::asec_rational_prec_round(
                Rational::NEGATIVE_ONE,
                prec,
                Exact
            ));
            return;
        }
        // asec(-1) = pi
        let (pi, o_pi) = Float::pi_prec_round(prec, rm);
        let (c, o) = Float::asec_rational_prec_round(Rational::NEGATIVE_ONE, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(pi));
        assert_eq!(o, o_pi);
    });
}

#[test]
fn asec_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (c, o) = Float::asec_rational_prec(x.clone(), prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = Float::asec_rational_prec_ref(&x, prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = Float::asec_rational_prec_round_ref(&x, prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
// As in `test_primitive_float_asec`, some of the pinned values are the pi constants.
#[allow(clippy::approx_constant, clippy::type_repetition_in_bounds)]
fn test_primitive_float_asec_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_asec_rational::<T>(
                &Rational::from_str(s).unwrap()
            )),
            NiceFloat(out)
        );
    }
    test::<f32>("0", f32::NAN);
    test::<f32>("1/2", f32::NAN);
    test::<f32>("-1/2", f32::NAN);
    test::<f32>("1", 0.0);
    test::<f32>("-1", 3.1415927);
    test::<f32>("2", 1.0471976);
    test::<f32>("-2", 2.0943952);
    test::<f32>("5/3", 0.9272952);
    test::<f32>("-5/3", 2.2142975);
    test::<f32>("1000001/1000000", 0.001414213);
    test::<f32>("1000000", 1.5707953);
    test::<f32>("-1000000", 1.5707973);
    test::<f32>(
        "1000000000000000000000000000001/1000000000000000000000000000000",
        1.4142135e-15,
    );
    test::<f64>("0", f64::NAN);
    test::<f64>("1/2", f64::NAN);
    test::<f64>("-1/2", f64::NAN);
    test::<f64>("1", 0.0);
    test::<f64>("-1", 3.141592653589793);
    test::<f64>("2", 1.0471975511965979);
    test::<f64>("-2", 2.0943951023931957);
    test::<f64>("5/3", 0.9272952180016122);
    test::<f64>("-5/3", 2.214297435588181);
    test::<f64>("1000001/1000000", 0.0014142129731178241);
    test::<f64>("1000000", 1.5707953267948966);
    test::<f64>("-1000000", 1.5707973267948967);
    test::<f64>(
        "1000000000000000000000000000001/1000000000000000000000000000000",
        1.414213562373095e-15,
    );
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_asec_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let c = primitive_float_asec_rational::<T>(&x);
        // NaN exactly for an input inside (-1, 1)
        assert_eq!(c.is_nan(), x.lt_abs(&1u32));
        if !c.is_nan() {
            // the result lies in [0, pi], so it never overflows
            assert!(c.is_finite());
            assert!(c >= T::ZERO);
            // the same as the `Float` version taken with 64 bits to spare and rounded once
            let (c_float, _) = Float::asec_rational_prec_ref(&x, T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&c_float, Nearest).0),
                NiceFloat(c)
            );
        }
    });

    primitive_float_gen_var_1::<T>().test_properties(|x| {
        // The arcsecant of a finite primitive float, taken through the `Rational` path, matches the
        // direct primitive-float arcsecant. A zero is excluded, since `Rational` has no signed
        // zeros, though both give NaN there.
        if x != T::ZERO {
            assert_eq!(
                NiceFloat(primitive_float_asec_rational::<T>(&Rational::exact_from(x))),
                NiceFloat(primitive_float_asec(x))
            );
        }
    });
}

#[test]
fn primitive_float_asec_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_asec_rational_properties_helper);
}

// The arcsecant in u-ths of a turn is the arccosine of the reciprocal in the same units, and a
// `Float`'s reciprocal is an exact `Rational`. As for the plain arcsecant, the check is confined to
// inputs of moderate magnitude, since the `Rational` holds the exponent in full.
fn acosu_of_reciprocal(
    x: &Float,
    u: u64,
    prec: u64,
    rm: RoundingMode,
) -> Option<(Float, Ordering)> {
    if !x.is_finite() || *x == 0u32 || x.get_exponent().unwrap().unsigned_abs() > 1000 {
        return None;
    }
    Some(Float::acos_with_period_rational_prec_round(
        Rational::exact_from(x).reciprocal(),
        u,
        prec,
        rm,
    ))
}

#[test]
fn test_asec_with_period_prec_round() {
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

        let (c, o) = x.clone().asec_with_period_prec_round(u, prec, rm);
        assert!(c.is_valid());
        assert_eq!(c.to_string(), out);
        assert_eq!(to_hex_string(&c), out_hex);
        assert_eq!(o, o_out);

        let (c_alt, o_alt) = x.asec_with_period_prec_round_ref(u, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        let mut c_alt = x.clone();
        let o_alt = c_alt.asec_with_period_prec_round_assign(u, prec, rm);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);

        if rm == Nearest {
            let (c_alt, o_alt) = x.asec_with_period_prec_ref(u, prec);
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        // the arccosine of the exact reciprocal, in the same units
        if let Some((c_alt, o_alt)) = acosu_of_reciprocal(&x, u, prec, rm) {
            assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
            assert_eq!(o_alt, o);
        }

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
            && u <= u64::from(u32::MAX)
        {
            let (rug_c, rug_o) =
                rug_asec_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_c)),
                ComparableFloatRef(&c)
            );
            assert_eq!(rug_o, o);
        }
    };
    test("NaN", "NaN", 360, 10, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity",
        "Infinity",
        360,
        10,
        Exact,
        "90.000",
        "0x5a.0#10",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        360,
        10,
        Exact,
        "90.000",
        "0x5a.0#10",
        Equal,
    );
    test(
        "Infinity",
        "Infinity",
        7,
        10,
        Nearest,
        "1.7500",
        "0x1.c00#10",
        Equal,
    );
    test("Infinity", "Infinity", 0, 10, Exact, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 360, 10, Exact, "NaN", "NaN", Equal);
    test("-0.0", "-0x0.0", 360, 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 360, 10, Exact, "NaN", "NaN", Equal);
    test("0.50", "0x0.8#1", 0, 10, Exact, "NaN", "NaN", Equal);
    test("1.0", "0x1.0#1", 360, 10, Exact, "0.0", "0x0.0", Equal);
    test("1.0", "0x1.0#1", 0, 10, Exact, "0.0", "0x0.0", Equal);
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
    test(
        "2.0",
        "0x2.0#1",
        360,
        10,
        Exact,
        "60.000",
        "0x3c.0#10",
        Equal,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        360,
        10,
        Exact,
        "120.00",
        "0x78.0#10",
        Equal,
    );
    test(
        "2.0",
        "0x2.0#1",
        12,
        10,
        Exact,
        "2.0000",
        "0x2.00#10",
        Equal,
    );
    test(
        "-2.0",
        "-0x2.0#1",
        12,
        10,
        Exact,
        "4.0000",
        "0x4.00#10",
        Equal,
    );
    test("2.0", "0x2.0#1", 7, 10, Floor, "1.1660", "0x1.2a8#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        7,
        10,
        Ceiling,
        "1.1680",
        "0x1.2b0#10",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        360,
        10,
        Floor,
        "48.188",
        "0x30.3#10",
        Less,
    );
    test(
        "1.5",
        "0x1.8#2",
        360,
        10,
        Ceiling,
        "48.250",
        "0x30.4#10",
        Greater,
    );
    test(
        "1.5",
        "0x1.8#2",
        360,
        53,
        Nearest,
        "48.189685104221404",
        "0x30.308f33f72b56#53",
        Greater,
    );
    test(
        "-1.5",
        "-0x1.8#2",
        360,
        53,
        Nearest,
        "131.81031489577859",
        "0x83.cf70cc08d4a8#53",
        Less,
    );
    test(
        "1.0010",
        "0x1.004#11",
        360,
        20,
        Nearest,
        "2.5311089",
        "0x2.87f6c#20",
        Less,
    );
    test(
        "4.0",
        "0x4.0#1",
        360,
        20,
        Nearest,
        "75.522461",
        "0x4b.85c0#20",
        Less,
    );
    test(
        "-4.0",
        "-0x4.0#1",
        360,
        20,
        Nearest,
        "104.47754",
        "0x68.7a40#20",
        Greater,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        360,
        20,
        Nearest,
        "90.000000",
        "0x5a.0000#20",
        Greater,
    );
    test(
        "-1.3e30",
        "-0x1.0E+25#1",
        360,
        20,
        Nearest,
        "90.000000",
        "0x5a.0000#20",
        Less,
    );
    test(
        "1.3e30",
        "0x1.0E+25#1",
        1,
        20,
        Nearest,
        "0.25000000",
        "0x0.400000#20",
        Greater,
    );
    test(
        "2.6e323228495",
        "0x1.0E+268435455#1",
        360,
        53,
        Nearest,
        "90.000000000000000",
        "0x5a.000000000000#53",
        Greater,
    );
    test(
        "2.0",
        "0x2.0#1",
        18446744073709551615,
        64,
        Exact,
        "3074457345618258602.50",
        "0x2aaaaaaaaaaaaaaa.8#64",
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
        "Infinity",
        "Infinity",
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
fn asec_with_period_prec_round_fail_1() {
    Float::TWO.asec_with_period_prec_round(7, 0, Floor);
}

#[test]
#[should_panic]
fn asec_with_period_prec_round_fail_2() {
    // asec(3/2) is not an exact number of sevenths of a turn
    Float::from(1.5).asec_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn asec_with_period_prec_round_fail_3() {
    // asec(2) is a sixth of a turn, but 7 is not a multiple of 3
    Float::TWO.asec_with_period_prec_round(7, 10, Exact);
}

#[test]
#[should_panic]
fn asec_with_period_prec_round_fail_4() {
    // a quarter turn needs more than 2 bits when u = 7
    Float::INFINITY.asec_with_period_prec_round(7, 2, Exact);
}

#[test]
#[should_panic]
fn asec_with_period_prec_round_ref_fail() {
    Float::TWO.asec_with_period_prec_round_ref(7, 0, Floor);
}

#[test]
#[should_panic]
fn asec_with_period_prec_fail() {
    Float::TWO.asec_with_period_prec(7, 0);
}

#[test]
#[should_panic]
fn asec_with_period_round_fail() {
    Float::from(1.5).asec_with_period_round(7, Exact);
}

// Whether asecu(x, u) is exactly representable at `prec`: at NaN and at any |x| < 1, where the
// result is NaN, at u = 0, at x = 1, where the result is zero, and at the turn fractions -- a
// quarter at an infinite input, a half at -1, and a sixth or a third at |x| = 2 with u a multiple
// of 3 -- each of which needs a `prec` wide enough to hold it.
fn asec_with_period_exact(x: &Float, u: u64, prec: u64) -> bool {
    x.is_nan()
        || x.lt_abs(&1u32)
        || u == 0
        || *x == 1u32
        || ((!x.is_finite() || *x == -1i32) && Float::from_unsigned_prec(u, prec).1 == Equal)
        || (x.eq_abs(&Float::TWO)
            && u.is_multiple_of(3)
            && Float::from_unsigned_prec(u / 3, prec).1 == Equal)
}

#[allow(clippy::needless_pass_by_value)]
fn asec_with_period_prec_round_properties_helper(x: Float, u: u64, prec: u64, rm: RoundingMode) {
    if rm == Exact && !asec_with_period_exact(&x, u, prec) {
        assert_panic!(x.asec_with_period_prec_round_ref(u, prec, Exact));
        return;
    }
    let (c, o) = x.clone().asec_with_period_prec_round(u, prec, rm);
    assert!(c.is_valid());
    assert_rounding_ordering_consistent(&c, rm, o);

    let (c_alt, o_alt) = x.asec_with_period_prec_round_ref(u, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut c_alt = x.clone();
    let o_alt = c_alt.asec_with_period_prec_round_assign(u, prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    // the arccosine of the exact reciprocal, in the same units
    if let Some((c_alt, o_alt)) = acosu_of_reciprocal(&x, u, prec, rm) {
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o, "x = {x} u = {u} prec = {prec} rm = {rm:?}");
    }

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm)
        && u <= u64::from(u32::MAX)
    {
        let (rug_c, rug_o) =
            rug_asec_with_period_prec_round(&rug::Float::exact_from(&x), u, prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_c)),
            ComparableFloatRef(&c)
        );
        assert_eq!(rug_o, o);
    }

    // NaN exactly for a NaN input or one inside (-1, 1)
    assert_eq!(c.is_nan(), x.is_nan() || x.lt_abs(&1u32));
    if !c.is_nan() {
        // 0 <= asecu(x, u) <= u/2, a half turn, so the result never overflows
        assert!(c.is_finite());
        assert!(c >= 0u32);
        assert!(c <= Float::from_unsigned_prec_round(u, prec, Ceiling).0 >> 1u32);
        if c.is_normal() {
            assert_eq!(c.get_prec(), Some(prec));
        }
    }

    if o == Equal {
        assert!(asec_with_period_exact(&x, u, prec));
        for rm in exhaustive_rounding_modes() {
            let (c2, oo) = x.asec_with_period_prec_round_ref(u, prec, rm);
            assert_eq!(ComparableFloatRef(&c2), ComparableFloatRef(&c));
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(x.asec_with_period_prec_round_ref(u, prec, Exact));
    }
}

#[test]
fn asec_with_period_prec_round_properties() {
    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_25().test_properties(
        |(x, u, prec, rm)| {
            asec_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    float_unsigned_unsigned_rounding_mode_quadruple_gen_var_26().test_properties(
        |(x, u, prec, rm)| {
            asec_with_period_prec_round_properties_helper(x, u, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        // asecu(NaN, u) = NaN, and so is asecu(x, u) for |x| < 1, even when u is zero
        for x in [Float::NAN, Float::ZERO, Float::NEGATIVE_ZERO, Float::ONE_HALF] {
            for u in [0, 4] {
                let (c, o) = x.clone().asec_with_period_prec_round(u, prec, rm);
                assert!(c.is_nan());
                assert_eq!(o, Equal);
            }
        }
        // asecu(x, 0) = +0, exactly, since the arcsecant is never negative
        for x in [Float::ONE, Float::NEGATIVE_ONE, Float::TWO, Float::INFINITY] {
            let (c, o) = x.asec_with_period_prec_round(0, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
            assert_eq!(o, Equal);
        }
        // asecu(1, u) = +0, exactly
        let (c, o) = Float::ONE.asec_with_period_prec_round(8, prec, rm);
        assert_eq!(ComparableFloat(c), ComparableFloat(Float::ZERO));
        assert_eq!(o, Equal);
        // asecu(±infinity, u) = u/4, a quarter turn, and asecu(-1, u) = u/2, a half turn
        let (q, o_q) = Float::from_unsigned_prec_round(8u32, prec, rm);
        for (x, k) in
            [(Float::INFINITY, 2u32), (Float::NEGATIVE_INFINITY, 2), (Float::NEGATIVE_ONE, 1)]
        {
            let (c, o) = x.asec_with_period_prec_round(8, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(q.clone() >> k));
            assert_eq!(o, o_q);
        }
        // asecu(2, u) = u/6 and asecu(-2, u) = u/3 when u is a multiple of 3
        let (q, o_q) = Float::from_unsigned_prec_round(4u32, prec, rm);
        for (x, k) in [(Float::TWO, 1u32), (-Float::TWO, 0)] {
            let (c, o) = x.asec_with_period_prec_round(12, prec, rm);
            assert_eq!(ComparableFloat(c), ComparableFloat(q.clone() >> k));
            assert_eq!(o, o_q);
        }
    });
}

#[test]
fn asec_with_period_prec_properties() {
    float_unsigned_unsigned_triple_gen_var_1::<u64, u64>().test_properties(|(x, u, prec)| {
        let (c, o) = x.clone().asec_with_period_prec(u, prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.asec_with_period_prec_ref(u, prec);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.asec_with_period_prec_round_ref(u, prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.asec_with_period_prec_assign(u, prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn asec_with_period_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_47().test_properties(|(x, u, rm)| {
        if rm == Exact && !asec_with_period_exact(&x, u, x.significant_bits()) {
            assert_panic!(x.asec_with_period_round_ref(u, Exact));
            return;
        }
        let (c, o) = x.clone().asec_with_period_round(u, rm);
        assert!(c.is_valid());
        assert_rounding_ordering_consistent(&c, rm, o);
        let (c_alt, o_alt) = x.asec_with_period_round_ref(u, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (c_alt, o_alt) = x.asec_with_period_prec_round_ref(u, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut c_alt = x.clone();
        let o_alt = c_alt.asec_with_period_round_assign(u, rm);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
    });
}

#[test]
fn asec_with_period_properties() {
    float_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, u)| {
        let c = x.clone().asec_with_period(u);
        assert!(c.is_valid());
        let c_alt = x.asec_with_period_ref(u);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let mut c_alt = x.clone();
        c_alt.asec_with_period_assign(u);
        assert!(c_alt.is_valid());
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        let (c_alt, _) = x.asec_with_period_prec_round_ref(u, x.significant_bits(), Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    });
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_asec_with_period() {
    fn test<T: PrimitiveFloat>(x: T, u: u64, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(
            NiceFloat(primitive_float_asec_with_period(x, u)),
            NiceFloat(out)
        );
    }
    test::<f32>(f32::NAN, 360, f32::NAN);
    test::<f32>(f32::INFINITY, 360, 90.0);
    test::<f32>(f32::NEGATIVE_INFINITY, 360, 90.0);
    test::<f32>(0.0, 360, f32::NAN);
    test::<f32>(0.5, 360, f32::NAN);
    test::<f32>(0.5, 0, f32::NAN);
    test::<f32>(1.0, 360, 0.0);
    test::<f32>(1.0, 0, 0.0);
    test::<f32>(-1.0, 360, 180.0);
    test::<f32>(2.0, 360, 60.0);
    test::<f32>(-2.0, 360, 120.0);
    test::<f32>(2.0, 12, 2.0);
    test::<f32>(-2.0, 12, 4.0);
    test::<f32>(2.0, 7, 1.1666666);
    test::<f32>(1.5, 360, 48.189686);
    test::<f32>(-1.5, 360, 131.81032);
    test::<f32>(100.0, 360, 89.42703);
    test::<f32>(1.0e30, 360, 90.0);
    test::<f32>(1.0e30, 1, 0.25);
    test::<f32>(1.0, 18446744073709551615, 0.0);
    test::<f64>(f64::NAN, 360, f64::NAN);
    test::<f64>(f64::INFINITY, 360, 90.0);
    test::<f64>(f64::NEGATIVE_INFINITY, 360, 90.0);
    test::<f64>(0.0, 360, f64::NAN);
    test::<f64>(0.5, 360, f64::NAN);
    test::<f64>(0.5, 0, f64::NAN);
    test::<f64>(1.0, 360, 0.0);
    test::<f64>(1.0, 0, 0.0);
    test::<f64>(-1.0, 360, 180.0);
    test::<f64>(2.0, 360, 60.0);
    test::<f64>(-2.0, 360, 120.0);
    test::<f64>(2.0, 12, 2.0);
    test::<f64>(-2.0, 12, 4.0);
    test::<f64>(2.0, 7, 1.1666666666666667);
    test::<f64>(1.5, 360, 48.189685104221404);
    test::<f64>(-1.5, 360, 131.8103148957786);
    test::<f64>(100.0, 360, 89.42703265514285);
    test::<f64>(1.0e300, 360, 90.0);
    test::<f64>(1.0e300, 1, 0.25);
    test::<f64>(1.0, 18446744073709551615, 0.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_asec_with_period_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_unsigned_pair_gen_var_1::<T, u64>().test_properties(|(x, u)| {
        let c = primitive_float_asec_with_period(x, u);
        // NaN exactly for a NaN input or one inside (-1, 1)
        assert_eq!(c.is_nan(), x.is_nan() || x.abs() < T::ONE);
        if !c.is_nan() {
            // the result lies in [0, u/2], so it never overflows
            assert!(c.is_finite());
            assert!(c >= T::ZERO);
            // the same as the `Float` version taken with 64 bits to spare and rounded once
            let (c_float, _) =
                Float::asec_with_period_prec(Float::from(x), u, T::MANTISSA_WIDTH + 64);
            assert_eq!(
                NiceFloat(T::rounding_from(&c_float, Nearest).0),
                NiceFloat(c)
            );
        }
    });
}

#[test]
fn primitive_float_asec_with_period_properties() {
    apply_fn_to_primitive_floats!(primitive_float_asec_with_period_properties_helper);
}
