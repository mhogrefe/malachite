// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{PowerOf2, PowerOf2Assign};
use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::{ExactFrom, RoundingFrom};
use malachite_base::num::float::NiceFloat;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    primitive_float_gen, unsigned_gen_var_11, unsigned_rounding_mode_pair_gen_var_3,
};
use malachite_float::arithmetic::power_of_2_of_float::{
    primitive_float_power_of_2, primitive_float_power_of_2_rational,
};
use malachite_float::test_util::arithmetic::power_of_2_of_float::{
    rug_power_of_2_of_float, rug_power_of_2_of_float_prec, rug_power_of_2_of_float_prec_round,
    rug_power_of_2_of_float_round, rug_power_of_2_rational_prec,
    rug_power_of_2_rational_prec_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_12, float_rounding_mode_pair_gen_var_47,
    float_unsigned_pair_gen_var_1, float_unsigned_pair_gen_var_4,
    float_unsigned_rounding_mode_triple_gen_var_36,
    rational_unsigned_rounding_mode_triple_gen_var_10,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::platform::Limb;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use std::panic::catch_unwind;

#[test]
fn test_power_of_2_of_float_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (p, o) = Float::power_of_2_of_float_prec_round(x.clone(), prec, rm);
        assert!(p.is_valid());
        assert_eq!(p.to_string(), out);
        assert_eq!(to_hex_string(&p), out_hex);
        assert_eq!(o, o_out);

        let (p_alt, o_alt) = Float::power_of_2_of_float_prec_round_ref(&x, prec, rm);
        assert!(p_alt.is_valid());
        assert_eq!(ComparableFloatRef(&p), ComparableFloatRef(&p_alt));
        assert_eq!(o_alt, o_out);

        let mut p_alt = x.clone();
        let o_alt = p_alt.power_of_2_of_float_prec_round_assign(prec, rm);
        assert!(p_alt.is_valid());
        assert_eq!(ComparableFloatRef(&p), ComparableFloatRef(&p_alt));
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_p, rug_o) =
                rug_power_of_2_of_float_prec_round(&rug::Float::exact_from(&x), prec, rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_p)),
                ComparableFloatRef(&p)
            );
            assert_eq!(rug_o, o);
        }
    };
    // specials (exact, rounding-mode-invariant): NaN, +inf, -inf, +0, -0
    test("NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 1, Ceiling, "Infinity", "Infinity", Equal,
    );
    test("-Infinity", "-Infinity", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("-0.0", "-0x0.0", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    // Integer exponents: 2^x is a power of 2, hence exact and rounding-mode-invariant.
    test("3.0", "0x3.0#2", 10, Nearest, "8.0", "0x8.00#10", Equal);
    test("3.0", "0x3.0#2", 10, Floor, "8.0", "0x8.00#10", Equal);
    test("3.0", "0x3.0#2", 10, Ceiling, "8.0", "0x8.00#10", Equal);
    test("-1.0", "-0x1.0#1", 10, Nearest, "0.5", "0x0.800#10", Equal);
    test("10.0", "0xa.0#3", 4, Nearest, "1.0e3", "0x4.0E+2#4", Equal);
    test(
        "-5.0",
        "-0x5.0#3",
        20,
        Nearest,
        "0.03125",
        "0x0.080000#20",
        Equal,
    );
    // 2^0.5 = sqrt(2); non-integer with integer part 0, so xfrac = x
    test("0.5", "0x0.8#1", 1, Floor, "1.0", "0x1.0#1", Less);
    test("0.5", "0x0.8#1", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("0.5", "0x0.8#1", 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "0.5",
        "0x0.8#1",
        53,
        Nearest,
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Greater,
    );
    // 2^(-0.5) = 1/sqrt(2)
    test(
        "-0.5",
        "-0x0.8#1",
        53,
        Nearest,
        "0.7071067811865476",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test("-0.5", "-0x0.8#1", 10, Floor, "0.707", "0x0.b50#10", Less);
    test(
        "-0.5",
        "-0x0.8#1",
        10,
        Ceiling,
        "0.708",
        "0x0.b54#10",
        Greater,
    );
    // 2^0.25
    test(
        "0.2",
        "0x0.4#1",
        53,
        Nearest,
        "1.189207115002721",
        "0x1.306fe0a31b715#53",
        Less,
    );
    // 2^1.5 = 2 * sqrt(2); non-integer with nonzero integer part (xint = 1), so xfrac = x - 1
    test("1.5", "0x1.8#2", 10, Nearest, "2.828", "0x2.d4#10", Less);
    test("1.5", "0x1.8#2", 10, Floor, "2.828", "0x2.d4#10", Less);
    // 2^(-1.5); nonzero integer part (xint = -2)
    test(
        "-1.5",
        "-0x1.8#2",
        20,
        Nearest,
        "0.3535533",
        "0x0.5a8278#20",
        Less,
    );
    // overflow: x = 2^30 = MAX_EXPONENT + 1, so 2^x exceeds the largest finite Float
    test(
        "1.0e9",
        "0x4.0E+7#1",
        20,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test(
        "1.0e9",
        "0x4.0E+7#1",
        20,
        Floor,
        "too_big",
        "0x7.ffff8E+268435455#20",
        Less,
    );
    // underflow: x = -2^31 < MIN_EXPONENT - 2, so 2^x rounds to 0 or the smallest positive Float
    test("-2.0e9", "-0x8.0E+7#1", 20, Nearest, "0.0", "0x0.0", Less);
    test(
        "-2.0e9",
        "-0x8.0E+7#1",
        20,
        Ceiling,
        "too_small",
        "0x1.00000E-268435456#20",
        Greater,
    );
    // Ziv loop iterates: x = 2^-100 makes 2^x so close to 1 that the initial working precision
    // can't round it
    test("8.0e-31", "0x1.0E-25#1", 1, Nearest, "1.0", "0x1.0#1", Less);
    test(
        "8.0e-31",
        "0x1.0E-25#1",
        53,
        Nearest,
        "1.0",
        "0x1.0000000000000#53",
        Less,
    );
    // Nearest double-rounding underflow: x = -(2^30 + 0.5), so xint = MIN_EXPONENT - 1 and 2^xfrac
    // rounds to 1/2; the unrounded result is the underflow midpoint, rounded up to the smallest
    // positive Float
    test(
        "-1073741824.5",
        "-0x40000000.800#40",
        1,
        Nearest,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    // The same boundary with other rounding modes: the exact result lies strictly between 0 and the
    // smallest positive Float, so rounding toward zero gives +0 and rounding away from zero gives
    // that smallest value. (`shl_prec_round` handles this; a plain `<<` would ignore the rounding
    // mode and always produce the smallest positive value.)
    test(
        "-1073741824.5",
        "-0x40000000.800#40",
        1,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "-1073741824.5",
        "-0x40000000.800#40",
        1,
        Down,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "-1073741824.5",
        "-0x40000000.800#40",
        1,
        Ceiling,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(
        "-1073741824.5",
        "-0x40000000.800#40",
        1,
        Up,
        "too_small",
        "0x1.0E-268435456#1",
        Greater,
    );
    test(
        "-1073741824.5",
        "-0x40000000.800#40",
        20,
        Floor,
        "0.0",
        "0x0.0",
        Less,
    );
    test(
        "-1073741824.5",
        "-0x40000000.800#40",
        20,
        Nearest,
        "too_small",
        "0x1.00000E-268435456#20",
        Greater,
    );
}

#[test]
fn power_of_2_of_float_prec_round_fail() {
    assert_panic!(Float::power_of_2_of_float_prec_round(
        Float::one_prec(1),
        0,
        Floor
    ));
    assert_panic!(Float::power_of_2_of_float_prec_round_ref(
        &Float::one_prec(1),
        0,
        Floor
    ));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.power_of_2_of_float_prec_round_assign(0, Floor)
    });

    // 2^x for a non-integer x is transcendental, so `Exact` panics.
    let half = parse_hex_string("0x0.8#1");
    assert_panic!(Float::power_of_2_of_float_prec_round(
        half.clone(),
        1,
        Exact
    ));
    assert_panic!(Float::power_of_2_of_float_prec_round_ref(&half, 1, Exact));
    assert_panic!({
        let mut x = parse_hex_string("0x0.8#1");
        x.power_of_2_of_float_prec_round_assign(1, Exact)
    });
}

#[test]
fn power_of_2_of_float_round_fail() {
    let half = parse_hex_string("0x0.8#1");
    assert_panic!(Float::power_of_2_of_float_round(half.clone(), Exact));
    assert_panic!(Float::power_of_2_of_float_round_ref(&half, Exact));
    assert_panic!({
        let mut x = parse_hex_string("0x0.8#1");
        x.power_of_2_of_float_round_assign(Exact)
    });
}

#[test]
fn power_of_2_of_float_prec_fail() {
    assert_panic!(Float::power_of_2_of_float_prec(Float::NAN, 0));
    assert_panic!(Float::power_of_2_of_float_prec_ref(&Float::NAN, 0));
    assert_panic!({
        let mut x = Float::NAN;
        x.power_of_2_of_float_prec_assign(0)
    });
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_2_of_float_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (p, o) = Float::power_of_2_of_float_prec_round(x.clone(), prec, rm);
    assert!(p.is_valid());

    let (p_alt, o_alt) = Float::power_of_2_of_float_prec_round_ref(&x, prec, rm);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.power_of_2_of_float_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_p, rug_o) =
            rug_power_of_2_of_float_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_p)),
            ComparableFloatRef(&p)
        );
        assert_eq!(rug_o, o);
    }

    // 2^x is never negative: the result is positive, +0, +inf, or NaN.
    assert!(p.is_nan() || p.is_sign_positive());

    if p.is_normal() {
        assert_eq!(p.get_prec(), Some(prec));
    }

    if o == Equal {
        // 2^x is exact only for special or integer x, so the result is rounding-mode-invariant.
        for rm2 in exhaustive_rounding_modes() {
            let (p2, o2) = Float::power_of_2_of_float_prec_round_ref(&x, prec, rm2);
            assert_eq!(
                ComparableFloat(p2.abs_negative_zero_ref()),
                ComparableFloat(p.abs_negative_zero_ref())
            );
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(Float::power_of_2_of_float_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn power_of_2_of_float_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        power_of_2_of_float_prec_round_properties_helper(x, prec, rm);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties_with_config(
        &config,
        |(x, prec, rm)| {
            power_of_2_of_float_prec_round_properties_helper(x, prec, rm);
        },
    );

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (p, o) = Float::power_of_2_of_float_prec_round(Float::NAN, prec, rm);
        assert!(p.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::power_of_2_of_float_prec_round(Float::INFINITY, prec, rm),
            (Float::INFINITY, Equal)
        );
        assert_eq!(
            Float::power_of_2_of_float_prec_round(Float::NEGATIVE_INFINITY, prec, rm),
            (Float::ZERO, Equal)
        );
        assert_eq!(
            Float::power_of_2_of_float_prec_round(Float::ZERO, prec, rm),
            (Float::one_prec(prec), Equal)
        );
        assert_eq!(
            Float::power_of_2_of_float_prec_round(Float::NEGATIVE_ZERO, prec, rm),
            (Float::one_prec(prec), Equal)
        );
    });
}

#[test]
fn power_of_2_of_float_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (p, o) = Float::power_of_2_of_float_round(x.clone(), rm);
        assert!(p.is_valid());

        let (p_alt, o_alt) = Float::power_of_2_of_float_round_ref(&x, rm);
        assert!(p_alt.is_valid());
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.power_of_2_of_float_round_assign(rm);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);

        // power_of_2_of_float_round is power_of_2_of_float_prec_round at the input's precision.
        let (p_alt, o_alt) =
            Float::power_of_2_of_float_prec_round_ref(&x, x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
        assert_eq!(o_alt, o);

        assert!(p.is_nan() || p.is_sign_positive());

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_p, rug_o) = rug_power_of_2_of_float_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(
                ComparableFloatRef(&Float::from(&rug_p)),
                ComparableFloatRef(&p)
            );
            assert_eq!(rug_o, o);
        }
    });
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_2_of_float_prec_properties_helper(x: Float, prec: u64) {
    let (p, o) = Float::power_of_2_of_float_prec(x.clone(), prec);
    assert!(p.is_valid());

    let (p_alt, o_alt) = Float::power_of_2_of_float_prec_ref(&x, prec);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.power_of_2_of_float_prec_assign(prec);
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    // power_of_2_of_float_prec is power_of_2_of_float_prec_round with Nearest.
    let (p_alt, o_alt) = Float::power_of_2_of_float_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));
    assert_eq!(o_alt, o);

    assert!(p.is_nan() || p.is_sign_positive());
    if p.is_normal() {
        assert_eq!(p.get_prec(), Some(prec));
    }

    let (rug_p, rug_o) = rug_power_of_2_of_float_prec(&rug::Float::exact_from(&x), prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_p)),
        ComparableFloatRef(&p)
    );
    assert_eq!(rug_o, o);
}

#[test]
fn power_of_2_of_float_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        power_of_2_of_float_prec_properties_helper(x, prec);
    });

    let mut config = GenConfig::new();
    config.insert("mean_precision_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    float_unsigned_pair_gen_var_1().test_properties_with_config(&config, |(x, prec)| {
        power_of_2_of_float_prec_properties_helper(x, prec);
    });

    float_unsigned_pair_gen_var_4().test_properties(|(x, prec)| {
        power_of_2_of_float_prec_properties_helper(x, prec);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_2_of_float_properties_helper(x: Float) {
    let p = Float::power_of_2(x.clone());
    assert!(p.is_valid());

    let p_alt = Float::power_of_2(&x);
    assert!(p_alt.is_valid());
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

    let mut x_alt = x.clone();
    x_alt.power_of_2_assign();
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&p));

    // power_of_2 is power_of_2_of_float_round at the input's precision with Nearest.
    let p_alt = Float::power_of_2_of_float_round_ref(&x, Nearest).0;
    assert_eq!(ComparableFloatRef(&p_alt), ComparableFloatRef(&p));

    assert!(p.is_nan() || p.is_sign_positive());

    let rug_p = Float::from(&rug_power_of_2_of_float(&rug::Float::exact_from(&x)));
    assert_eq!(ComparableFloatRef(&rug_p), ComparableFloatRef(&p));
}

#[test]
fn power_of_2_of_float_properties() {
    float_gen().test_properties(power_of_2_of_float_properties_helper);
    float_gen_var_12().test_properties(power_of_2_of_float_properties_helper);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_power_of_2() {
    fn test<T: PrimitiveFloat>(x: T, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        assert_eq!(NiceFloat(primitive_float_power_of_2(x)), NiceFloat(out));
    }
    test::<f32>(f32::NAN, f32::NAN);
    test::<f32>(f32::INFINITY, f32::INFINITY);
    test::<f32>(f32::NEGATIVE_INFINITY, 0.0);
    test::<f32>(0.0, 1.0);
    test::<f32>(-0.0, 1.0);
    test::<f32>(1.0, 2.0);
    test::<f32>(-1.0, 0.5);
    test::<f32>(0.5, 1.4142135);
    test::<f32>(-0.5, 0.70710677);
    test::<f32>(2.0, 4.0);
    test::<f32>(-2.0, 0.25);
    test::<f32>(core::f32::consts::PI, 8.824979);
    test::<f32>(-core::f32::consts::PI, 0.113314725);
    test::<f32>(core::f32::consts::E, 6.5808854);
    test::<f32>(-core::f32::consts::E, 0.15195523);

    test::<f64>(f64::NAN, f64::NAN);
    test::<f64>(f64::INFINITY, f64::INFINITY);
    test::<f64>(f64::NEGATIVE_INFINITY, 0.0);
    test::<f64>(0.0, 1.0);
    test::<f64>(-0.0, 1.0);
    test::<f64>(1.0, 2.0);
    test::<f64>(-1.0, 0.5);
    test::<f64>(0.5, 1.4142135623730951);
    test::<f64>(-0.5, 0.7071067811865476);
    test::<f64>(2.0, 4.0);
    test::<f64>(-2.0, 0.25);
    test::<f64>(core::f64::consts::PI, 8.824977827076287);
    test::<f64>(-core::f64::consts::PI, 0.11331473229676088);
    test::<f64>(core::f64::consts::E, 6.5808859910179205);
    test::<f64>(-core::f64::consts::E, 0.15195522325791297);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_power_of_2_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    primitive_float_gen::<T>().test_properties(|x| {
        let p = primitive_float_power_of_2(x);
        // 2^x is never negative, and is NaN only for a NaN input.
        assert_eq!(p.is_nan(), x.is_nan());
        assert!(p.is_nan() || p >= T::ZERO);
    });
}

#[test]
fn primitive_float_power_of_2_properties() {
    apply_fn_to_primitive_floats!(primitive_float_power_of_2_properties_helper);
}

#[test]
#[allow(clippy::type_repetition_in_bounds)]
fn test_primitive_float_power_of_2_rational() {
    fn test<T: PrimitiveFloat>(s: &str, out: T)
    where
        Float: From<T> + PartialOrd<T>,
        for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
    {
        let u = Rational::from_str(s).unwrap();
        assert_eq!(
            NiceFloat(primitive_float_power_of_2_rational(&u)),
            NiceFloat(out)
        );
    }
    test::<f32>("0", 1.0);
    test::<f32>("1", 2.0);
    test::<f32>("1/2", 1.4142135);
    test::<f32>("1/3", 1.2599211);
    test::<f32>("22/7", 8.832716);
    test::<f32>("1000000", f32::INFINITY);
    test::<f32>("1/1000000", 1.0000007);
    test::<f32>("-1", 0.5);
    test::<f32>("-1/2", 0.70710677);
    test::<f32>("-1/3", 0.7937005);
    test::<f32>("-22/7", 0.11321546);
    test::<f32>("-1000000", 0.0);

    test::<f64>("0", 1.0);
    test::<f64>("1", 2.0);
    test::<f64>("1/2", 1.4142135623730951);
    test::<f64>("1/3", 1.2599210498948732);
    test::<f64>("22/7", 8.832716109390498);
    test::<f64>("1000000", f64::INFINITY);
    test::<f64>("1/1000000", 1.0000006931474208);
    test::<f64>("-1", 0.5);
    test::<f64>("-1/2", 0.7071067811865476);
    test::<f64>("-1/3", 0.7937005259840998);
    test::<f64>("-22/7", 0.11321545803298834);
    test::<f64>("-1000000", 0.0);
}

#[allow(clippy::type_repetition_in_bounds)]
fn primitive_float_power_of_2_rational_properties_helper<T: PrimitiveFloat>()
where
    Float: From<T> + PartialOrd<T>,
    Rational: ExactFrom<T>,
    for<'a> T: ExactFrom<&'a Float> + RoundingFrom<&'a Float>,
{
    rational_gen().test_properties(|x| {
        let y = primitive_float_power_of_2_rational::<T>(&x);
        // 2^x is always positive (a positive value, +0, or +inf), never negative or NaN.
        assert!(y >= T::ZERO);
        if x > 0u32 {
            assert!(y >= T::ONE);
        } else if x < 0u32 {
            assert!(y <= T::ONE);
        }
    });

    primitive_float_gen::<T>().test_properties(|x| {
        // 2^x of a finite primitive float, taken through the `Rational` path, matches the direct
        // primitive-float `2^x`.
        if x.is_finite() {
            assert_eq!(
                NiceFloat(primitive_float_power_of_2_rational::<T>(
                    &Rational::exact_from(x)
                )),
                NiceFloat(primitive_float_power_of_2(x))
            );
        }
    });
}

#[test]
fn primitive_float_power_of_2_rational_properties() {
    apply_fn_to_primitive_floats!(primitive_float_power_of_2_rational_properties_helper);
}

#[test]
fn test_power_of_2_rational_prec() {
    let test = |s, prec, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::power_of_2_rational_prec(x.clone(), prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::power_of_2_rational_prec_ref(&x, prec);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    // integer x: 2^x is an exact power of 2
    test("0", 1, "1.0", "0x1.0#1", Equal);
    test("0", 10, "1.0", "0x1.000#10", Equal);
    test("0", 53, "1.0", "0x1.0000000000000#53", Equal);
    test("1", 1, "2.0", "0x2.0#1", Equal);
    test("1", 10, "2.0", "0x2.00#10", Equal);
    test("1", 53, "2.0", "0x2.0000000000000#53", Equal);
    test("-1", 1, "0.5", "0x0.8#1", Equal);
    test("-1", 10, "0.5", "0x0.800#10", Equal);
    test("-1", 53, "0.5", "0x0.80000000000000#53", Equal);
    // non-integer x: 2^x is transcendental
    test("1/2", 1, "1.0", "0x1.0#1", Less);
    test("1/2", 10, "1.414", "0x1.6a0#10", Less);
    test(
        "1/2",
        53,
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        Greater,
    );
    test("-1/2", 1, "0.5", "0x0.8#1", Less);
    test("-1/2", 10, "0.707", "0x0.b50#10", Less);
    test(
        "-1/2",
        53,
        "0.7071067811865476",
        "0x0.b504f333f9de68#53",
        Greater,
    );
    test("1/3", 1, "1.0", "0x1.0#1", Less);
    test("1/3", 10, "1.26", "0x1.428#10", Less);
    test(
        "1/3",
        53,
        "1.2599210498948732",
        "0x1.428a2f98d728b#53",
        Greater,
    );
    test("22/7", 1, "8.0", "0x8.0#1", Less);
    test("22/7", 10, "8.83", "0x8.d4#10", Less);
    test(
        "22/7",
        53,
        "8.832716109390498",
        "0x8.d52ce208af3e8#53",
        Less,
    );
    test("-22/7", 1, "0.1", "0x0.2#1", Greater);
    test("-22/7", 10, "0.1132", "0x0.1cf8#10", Less);
    test(
        "-22/7",
        53,
        "0.11321545803298834",
        "0x0.1cfbb031a741a5#53",
        Greater,
    );
    test("100", 1, "1.0e30", "0x1.0E+25#1", Equal);
    test("100", 10, "1.268e30", "0x1.000E+25#10", Equal);
    test(
        "100",
        53,
        "1.2676506002282294e30",
        "0x1.0000000000000E+25#53",
        Equal,
    );
    test("-100", 1, "8.0e-31", "0x1.0E-25#1", Equal);
    test("-100", 10, "7.89e-31", "0x1.000E-25#10", Equal);
    test(
        "-100",
        53,
        "7.888609052210118e-31",
        "0x1.0000000000000E-25#53",
        Equal,
    );
}

#[test]
#[should_panic]
fn power_of_2_rational_prec_fail() {
    Float::power_of_2_rational_prec(Rational::from(1), 0);
}

#[test]
#[should_panic]
fn power_of_2_rational_prec_ref_fail() {
    Float::power_of_2_rational_prec_ref(&Rational::from(1), 0);
}

#[test]
fn test_power_of_2_rational_prec_round() {
    let test = |s, prec, rm, out: &str, out_hex: &str, out_o| {
        let x = Rational::from_str(s).unwrap();

        let (f, o) = Float::power_of_2_rational_prec_round(x.clone(), prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::power_of_2_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    // Like `test`, but takes a constructed `Rational` directly (for inputs too large or small to
    // write as a literal).
    let test_big = |x: Rational, prec, rm, out: &str, out_hex: &str, out_o| {
        let (f, o) = Float::power_of_2_rational_prec_round(x.clone(), prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);

        let (f, o) = Float::power_of_2_rational_prec_round_ref(&x, prec, rm);
        assert!(f.is_valid());
        assert_eq!(f.to_string(), out);
        assert_eq!(to_hex_string(&f), out_hex);
        assert_eq!(o, out_o);
    };
    // integer x: 2^x is an exact power of 2, rounding-mode-invariant
    test("0", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("0", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("0", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("0", 10, Floor, "1.0", "0x1.000#10", Equal);
    test("0", 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test("0", 10, Nearest, "1.0", "0x1.000#10", Equal);
    test("1", 1, Floor, "2.0", "0x2.0#1", Equal);
    test("1", 1, Ceiling, "2.0", "0x2.0#1", Equal);
    test("1", 1, Nearest, "2.0", "0x2.0#1", Equal);
    test("1", 10, Floor, "2.0", "0x2.00#10", Equal);
    test("1", 10, Ceiling, "2.0", "0x2.00#10", Equal);
    test("1", 10, Nearest, "2.0", "0x2.00#10", Equal);
    test("-1", 1, Floor, "0.5", "0x0.8#1", Equal);
    test("-1", 1, Ceiling, "0.5", "0x0.8#1", Equal);
    test("-1", 1, Nearest, "0.5", "0x0.8#1", Equal);
    test("-1", 10, Floor, "0.5", "0x0.800#10", Equal);
    test("-1", 10, Ceiling, "0.5", "0x0.800#10", Equal);
    test("-1", 10, Nearest, "0.5", "0x0.800#10", Equal);
    // non-integer x: 2^x is transcendental
    test("1/2", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1/2", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1/2", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/2", 10, Floor, "1.414", "0x1.6a0#10", Less);
    test("1/2", 10, Ceiling, "1.416", "0x1.6a8#10", Greater);
    test("1/2", 10, Nearest, "1.414", "0x1.6a0#10", Less);
    test("-1/2", 1, Floor, "0.5", "0x0.8#1", Less);
    test("-1/2", 1, Ceiling, "1.0", "0x1.0#1", Greater);
    test("-1/2", 1, Nearest, "0.5", "0x0.8#1", Less);
    test("-1/2", 10, Floor, "0.707", "0x0.b50#10", Less);
    test("-1/2", 10, Ceiling, "0.708", "0x0.b54#10", Greater);
    test("-1/2", 10, Nearest, "0.707", "0x0.b50#10", Less);
    test("1/3", 1, Floor, "1.0", "0x1.0#1", Less);
    test("1/3", 1, Ceiling, "2.0", "0x2.0#1", Greater);
    test("1/3", 1, Nearest, "1.0", "0x1.0#1", Less);
    test("1/3", 10, Floor, "1.26", "0x1.428#10", Less);
    test("1/3", 10, Ceiling, "1.262", "0x1.430#10", Greater);
    test("1/3", 10, Nearest, "1.26", "0x1.428#10", Less);
    test("22/7", 1, Floor, "8.0", "0x8.0#1", Less);
    test("22/7", 1, Ceiling, "2.0e1", "0x1.0E+1#1", Greater);
    test("22/7", 1, Nearest, "8.0", "0x8.0#1", Less);
    test("22/7", 10, Floor, "8.83", "0x8.d4#10", Less);
    test("22/7", 10, Ceiling, "8.84", "0x8.d8#10", Greater);
    test("22/7", 10, Nearest, "8.83", "0x8.d4#10", Less);
    test("-22/7", 1, Floor, "0.06", "0x0.1#1", Less);
    test("-22/7", 1, Ceiling, "0.1", "0x0.2#1", Greater);
    test("-22/7", 1, Nearest, "0.1", "0x0.2#1", Greater);
    test("-22/7", 10, Floor, "0.1132", "0x0.1cf8#10", Less);
    test("-22/7", 10, Ceiling, "0.1133", "0x0.1d00#10", Greater);
    test("-22/7", 10, Nearest, "0.1132", "0x0.1cf8#10", Less);
    test("100", 1, Floor, "1.0e30", "0x1.0E+25#1", Equal);
    test("100", 1, Ceiling, "1.0e30", "0x1.0E+25#1", Equal);
    test("100", 1, Nearest, "1.0e30", "0x1.0E+25#1", Equal);
    test("100", 10, Floor, "1.268e30", "0x1.000E+25#10", Equal);
    test("100", 10, Ceiling, "1.268e30", "0x1.000E+25#10", Equal);
    test("100", 10, Nearest, "1.268e30", "0x1.000E+25#10", Equal);
    test("-100", 1, Floor, "8.0e-31", "0x1.0E-25#1", Equal);
    test("-100", 1, Ceiling, "8.0e-31", "0x1.0E-25#1", Equal);
    test("-100", 1, Nearest, "8.0e-31", "0x1.0E-25#1", Equal);
    test("-100", 10, Floor, "7.89e-31", "0x1.000E-25#10", Equal);
    test("-100", 10, Ceiling, "7.89e-31", "0x1.000E-25#10", Equal);
    test("-100", 10, Nearest, "7.89e-31", "0x1.000E-25#10", Equal);
    // Exact succeeds for integer x (2^x is an exact power of 2)
    test("0", 10, Exact, "1.0", "0x1.000#10", Equal);
    test("1", 10, Exact, "2.0", "0x2.00#10", Equal);
    test("-1", 10, Exact, "0.5", "0x0.800#10", Equal);
    test("100", 10, Exact, "1.268e30", "0x1.000E+25#10", Equal);
    test("-100", 10, Exact, "7.89e-31", "0x1.000E-25#10", Equal);
    // x near a rounding boundary: the bracket loop iterates before the bounds agree
    test("17/37", 3, Nearest, "1.5", "0x1.8#3", Greater);

    // The cases below cover the helper's branches for integer and extreme x. Integer x too large to
    // fit in an i64 (2^x is an exact power of 2 that overflows or underflows):
    test_big(
        Rational::power_of_2(100i64),
        20,
        Nearest,
        "Infinity",
        "Infinity",
        Greater,
    );
    test_big(
        Rational::power_of_2(100i64),
        20,
        Floor,
        "too_big",
        "0x7.ffff8E+268435455#20",
        Less,
    );
    test_big(
        -Rational::power_of_2(100i64),
        20,
        Nearest,
        "0.0",
        "0x0.0",
        Less,
    );
    test_big(
        -Rational::power_of_2(100i64),
        20,
        Ceiling,
        "too_small",
        "0x1.00000E-268435456#20",
        Greater,
    );
    // Tiny non-integer x = +/- 2^-100: 2^x is within half an ulp of 1, so it rounds to 1 or its
    // neighbor.
    test_big(
        Rational::power_of_2(-100i64),
        10,
        Nearest,
        "1.0",
        "0x1.000#10",
        Less,
    ); // 1
    test_big(
        Rational::power_of_2(-100i64),
        10,
        Ceiling,
        "1.002",
        "0x1.008#10",
        Greater,
    ); // 1 + ulp
    test_big(
        -Rational::power_of_2(-100i64),
        10,
        Nearest,
        "1.0",
        "0x1.000#10",
        Greater,
    ); // 1
    test_big(
        -Rational::power_of_2(-100i64),
        10,
        Floor,
        "0.999",
        "0x0.ffc#10",
        Less,
    ); // 1 - ulp
    // Sub-MIN x (|x| < 2^MIN_EXPONENT): too small for a normal Float, so 2^x is rounded directly
    // from 1 by `float_round_near_x`. 2^x is above 1 for x > 0 and below 1 for x < 0. (See
    // `power_of_2_rational_prec_round_near_one_huge` for the prec >= MAX_EXPONENT fallback.)
    let min1 = Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 1);
    test_big(min1.clone(), 1, Nearest, "1.0", "0x1.0#1", Less);
    test_big(-min1, 1, Nearest, "1.0", "0x1.0#1", Greater);
    // Non-integer x too large to be a finite Float (exp_x >= MAX_EXPONENT): 2^x overflows (x > 0)
    // or underflows (x < 0).
    let big =
        Rational::power_of_2(i64::from(Float::MAX_EXPONENT)) + Rational::from_unsigneds(1u32, 2u32);
    test_big(big.clone(), 1, Nearest, "Infinity", "Infinity", Greater);
    test_big(-big, 1, Nearest, "0.0", "0x0.0", Less);
}

// Sub-MIN x at a precision where `float_round_near_x` cannot resolve the rounding (prec >= -exp_x),
// so `power_of_2_rational_near_one` falls through to actually computing 2^x = exp(x * ln(2)). Here
// x = 3 * 2^(MIN_EXPONENT - 2) has exp_x = MIN_EXPONENT and 2^x - 1 ~ 0.52 ulp at this precision,
// so 2^x is genuinely more than half an ulp above 1 and the correct result is 1 + ulp (not 1). The
// result is a ~128 MB Float, checked by value. (Mirrors the `exp_x_minus_1_rational` test at prec =
// MAX_EXPONENT.)
#[test]
fn power_of_2_rational_near_one_compute_huge() {
    let prec = u64::exact_from(Float::MAX_EXPONENT) + 1;
    let one = Float::one_prec(prec);
    // 1 + ulp away from zero is 1 + 2^(1 - prec).
    let one_plus_ulp = one
        .clone()
        .add_prec_round(Float::power_of_2(1 - i64::exact_from(prec)), prec, Exact)
        .0;
    let x = Rational::from(3) * Rational::power_of_2(i64::from(Float::MIN_EXPONENT) - 2);
    // Nearest (and rounding away from 1) gives 1 + ulp.
    let (f, o) = Float::power_of_2_rational_prec_round_ref(&x, prec, Nearest);
    assert!(f.is_valid());
    assert_eq!(ComparableFloatRef(&f), ComparableFloatRef(&one_plus_ulp));
    assert_eq!(o, Greater);
    // Rounding toward 1 gives 1.
    let (f, o) = Float::power_of_2_rational_prec_round_ref(&x, prec, Floor);
    assert!(f.is_valid());
    assert_eq!(ComparableFloatRef(&f), ComparableFloatRef(&one));
    assert_eq!(o, Less);
}

#[test]
#[should_panic]
fn power_of_2_rational_prec_round_fail_1() {
    Float::power_of_2_rational_prec_round(Rational::from(1), 0, Floor);
}

#[test]
#[should_panic]
fn power_of_2_rational_prec_round_fail_2() {
    // 2^(1/2) is transcendental, so Exact panics.
    Float::power_of_2_rational_prec_round(Rational::from_unsigneds(1u32, 2u32), 10, Exact);
}

#[test]
#[should_panic]
fn power_of_2_rational_prec_round_ref_fail() {
    Float::power_of_2_rational_prec_round_ref(&Rational::from_unsigneds(1u32, 2u32), 10, Exact);
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_2_rational_prec_round_properties_helper(x: Rational, prec: u64, rm: RoundingMode) {
    let (f, o) = Float::power_of_2_rational_prec_round(x.clone(), prec, rm);
    assert!(f.is_valid());

    let (f_alt, o_alt) = Float::power_of_2_rational_prec_round_ref(&x, prec, rm);
    assert!(f_alt.is_valid());
    assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
    assert_eq!(o_alt, o);

    // 2^x is always positive (a positive finite value, +0, or +inf), never negative or NaN.
    assert!(f >= 0u32);

    if let Ok(rrm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_f, rug_o) = rug_power_of_2_rational_prec_round(&x, prec, rrm);
        assert_eq!(
            ComparableFloatRef(&Float::from(&rug_f)),
            ComparableFloatRef(&f)
        );
        assert_eq!(rug_o, o);
    }

    if f.is_normal() {
        assert_eq!(f.get_prec(), Some(prec));
        if x > 0u32 {
            assert!(f >= 1u32);
        } else if x < 0u32 {
            assert!(f <= 1u32);
        }
    }

    if o == Equal {
        // 2^x is exact only for integer x, so the result is rounding-mode-invariant.
        for rm in exhaustive_rounding_modes() {
            let (s, oo) = Float::power_of_2_rational_prec_round_ref(&x, prec, rm);
            assert_eq!(
                ComparableFloat(s.abs_negative_zero_ref()),
                ComparableFloat(f.abs_negative_zero_ref())
            );
            assert_eq!(oo, Equal);
        }
    } else {
        assert_panic!(Float::power_of_2_rational_prec_round_ref(&x, prec, Exact));
    }
}

#[test]
fn power_of_2_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_10().test_properties(|(x, prec, rm)| {
        power_of_2_rational_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (f, o) = Float::power_of_2_rational_prec_round(Rational::ZERO, prec, rm);
        assert_eq!(ComparableFloat(f), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o, Equal);
    });
}

#[allow(clippy::needless_pass_by_value)]
fn power_of_2_rational_prec_properties_helper(x: Rational, prec: u64) {
    let (f, o) = Float::power_of_2_rational_prec(x.clone(), prec);
    assert!(f.is_valid());

    let (f_alt, o_alt) = Float::power_of_2_rational_prec_ref(&x, prec);
    assert!(f_alt.is_valid());
    assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
    assert_eq!(o_alt, o);

    let (f_alt, o_alt) = Float::power_of_2_rational_prec_round_ref(&x, prec, Nearest);
    assert_eq!(ComparableFloatRef(&f_alt), ComparableFloatRef(&f));
    assert_eq!(o_alt, o);

    assert!(f >= 0u32);

    let (rug_f, rug_o) = rug_power_of_2_rational_prec(&x, prec);
    assert_eq!(
        ComparableFloatRef(&Float::from(&rug_f)),
        ComparableFloatRef(&f)
    );
    assert_eq!(rug_o, o);

    if f.is_normal() {
        assert_eq!(f.get_prec(), Some(prec));
        if x > 0u32 {
            assert!(f >= 1u32);
        } else if x < 0u32 {
            assert!(f <= 1u32);
        }
    }
}

#[test]
fn power_of_2_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        power_of_2_rational_prec_properties_helper(x, prec);
    });

    unsigned_gen_var_11().test_properties(|prec| {
        let (f, o) = Float::power_of_2_rational_prec(Rational::ZERO, prec);
        assert_eq!(ComparableFloat(f), ComparableFloat(Float::one_prec(prec)));
        assert_eq!(o, Equal);
    });
}
