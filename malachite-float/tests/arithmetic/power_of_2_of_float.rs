// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{PowerOf2, PowerOf2Assign};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::unsigned_rounding_mode_pair_gen_var_3;
use malachite_float::test_util::arithmetic::power_of_2_of_float::{
    rug_power_of_2_of_float, rug_power_of_2_of_float_prec, rug_power_of_2_of_float_prec_round,
    rug_power_of_2_of_float_round,
};
use malachite_float::test_util::common::{
    parse_hex_string, rug_round_try_from_rounding_mode, to_hex_string,
};
use malachite_float::test_util::generators::{
    float_gen, float_rounding_mode_pair_gen_var_47, float_unsigned_pair_gen_var_1,
    float_unsigned_rounding_mode_triple_gen_var_36,
};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
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

#[test]
fn power_of_2_of_float_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
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
}
