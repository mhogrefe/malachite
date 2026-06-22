// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::num::arithmetic::traits::{Exp, ExpAssign};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeZero, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_base::test_util::generators::unsigned_rounding_mode_pair_gen_var_3;
use malachite_float::test_util::arithmetic::exp::{
    rug_exp, rug_exp_prec, rug_exp_prec_round, rug_exp_round,
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
fn test_exp_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (e, o) = x.clone().exp_prec_round(prec, rm);
        assert!(e.is_valid());
        assert_eq!(e.to_string(), out);
        assert_eq!(to_hex_string(&e), out_hex);
        assert_eq!(o, o_out);

        let (e_alt, o_alt) = x.exp_prec_round_ref(prec, rm);
        assert!(e_alt.is_valid());
        assert_eq!(ComparableFloatRef(&e), ComparableFloatRef(&e_alt));
        assert_eq!(o_alt, o_out);

        let mut e_alt = x.clone();
        let o_alt = e_alt.exp_prec_round_assign(prec, rm);
        assert!(e_alt.is_valid());
        assert_eq!(ComparableFloatRef(&e), ComparableFloatRef(&e_alt));
        assert_eq!(o_alt, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_e, rug_o) = rug_exp_prec_round(&rug::Float::exact_from(&x), prec, rm);
            assert_eq!(ComparableFloatRef(&Float::from(&rug_e)), ComparableFloatRef(&e));
            assert_eq!(rug_o, o);
        }
    };
    // specials (exact, rounding-mode-invariant)
    test("NaN", "NaN", 1, Floor, "NaN", "NaN", Equal);
    test("Infinity", "Infinity", 1, Ceiling, "Infinity", "Infinity", Equal);
    test("-Infinity", "-Infinity", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", "0x0.0", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("-0.0", "-0x0.0", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    // e = exp(1)
    test(
        "1.0", "0x1.0#1", 53, Nearest, "2.7182818284590451", "0x2.b7e151628aed2#53", Less,
    );
    test(
        "1.0", "0x1.0#1", 53, Floor, "2.7182818284590451", "0x2.b7e151628aed2#53", Less,
    );
    test(
        "1.0", "0x1.0#1", 53, Ceiling, "2.7182818284590455", "0x2.b7e151628aed4#53", Greater,
    );
    // e^2, 1/e, sqrt(e)
    test("2.0", "0x2.0#2", 10, Nearest, "7.39", "0x7.64#10", Greater);
    test("-1.0", "-0x1.0#1", 20, Nearest, "0.3678794", "0x0.5e2d58#20", Less);
    test("0.5", "0x0.8#1", 30, Nearest, "1.64872127", "0x1.a61298e0#30", Less);
    test(
        "1.0",
        "0x1.0#1",
        100,
        Nearest,
        "2.718281828459045235360287471351",
        "0x2.b7e151628aed2a6abf7158808#100",
        Less,
    );
    // overflow: x = 2^30 > log(2^emax), so exp(x) is above the largest finite Float
    test("1.0e9", "0x4.0E+7#1", 20, Nearest, "Infinity", "Infinity", Greater);
    test("1.0e9", "0x4.0E+7#1", 20, Floor, "too_big", "0x7.ffff8E+268435455#20", Less);
    test("1.0e9", "0x4.0E+7#1", 20, Up, "Infinity", "Infinity", Greater);
    // underflow: x = -2^30 < log(2^(emin - 2)), so exp(x) is below the smallest positive Float
    test("-1.0e9", "-0x4.0E+7#1", 20, Nearest, "0.0", "0x0.0", Less);
    test("-1.0e9", "-0x4.0E+7#1", 20, Up, "too_small", "0x1.00000E-268435456#20", Greater);
    test("-1.0e9", "-0x4.0E+7#1", 20, Floor, "0.0", "0x0.0", Less);
    // tiny x: |x| = 2^-100 < ulp(1) at precision 50, so exp(x) = 1 +/- ulp(1)
    test("8.0e-31", "0x1.0E-25#1", 50, Nearest, "1.0", "0x1.0000000000000#50", Less);
    test("8.0e-31", "0x1.0E-25#1", 50, Up, "1.000000000000002", "0x1.0000000000008#50", Greater);
    test("8.0e-31", "0x1.0E-25#1", 50, Floor, "1.0", "0x1.0000000000000#50", Less);
    test("-8.0e-31", "-0x1.0E-25#1", 50, Nearest, "1.0", "0x1.0000000000000#50", Greater);
    test(
        "-8.0e-31", "-0x1.0E-25#1", 50, Floor, "0.999999999999999", "0x0.ffffffffffffc#50", Less,
    );
    test("-8.0e-31", "-0x1.0E-25#1", 50, Ceiling, "1.0", "0x1.0000000000000#50", Greater);
    // |x| <= 0.25 takes the n = 0 argument-reduction shortcut
    test("0.1", "0x0.2#1", 2, Down, "1.0", "0x1.0#2", Less);
    // 0.25 < |x| < ~0.5: argument reduction runs but n still rounds to 0
    test("0.2", "0x0.4#1", 1, Down, "1.0", "0x1.0#1", Less);
    // negative x: n < 0, and the initial n is reduced once when the reduced r comes out negative
    test("-1.0", "-0x1.0#1", 1, Down, "0.2", "0x0.4#1", Less);
    // low-precision input, high target precision: the first Ziv iteration cannot round
    test(
        "-8.0e-31",
        "-0x1.0E-25#1",
        108,
        Floor,
        "0.999999999999999999999999999999211",
        "0x0.fffffffffffffffffffffffff00#108",
        Less,
    );
}

#[test]
fn exp_prec_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(Float::one_prec(1).exp_prec_round(0, Floor));
    assert_panic!(Float::one_prec(1).exp_prec_round_ref(0, Floor));
    assert_panic!({
        let mut x = Float::one_prec(1);
        x.exp_prec_round_assign(0, Floor)
    });

    assert_panic!(THREE.exp_prec_round(1, Exact));
    assert_panic!(THREE.exp_prec_round_ref(1, Exact));
    assert_panic!({
        let mut x = THREE;
        x.exp_prec_round_assign(1, Exact)
    });
}

#[test]
fn exp_round_fail() {
    const THREE: Float = Float::const_from_unsigned(3);
    assert_panic!(THREE.exp_round(Exact));
    assert_panic!(THREE.exp_round_ref(Exact));
    assert_panic!({
        let mut x = THREE;
        x.exp_round_assign(Exact);
    });
}

#[test]
fn exp_prec_fail() {
    assert_panic!(Float::NAN.exp_prec(0));
    assert_panic!(Float::NAN.exp_prec_ref(0));
    assert_panic!({
        let mut x = Float::NAN;
        x.exp_prec_assign(0)
    });
}

fn exp_prec_round_properties_helper(x: Float, prec: u64, rm: RoundingMode) {
    let (e, o) = x.clone().exp_prec_round(prec, rm);
    assert!(e.is_valid());

    let (e_alt, o_alt) = x.exp_prec_round_ref(prec, rm);
    assert!(e_alt.is_valid());
    assert_eq!(ComparableFloatRef(&e_alt), ComparableFloatRef(&e));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.exp_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&e));
    assert_eq!(o_alt, o);

    if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
        let (rug_e, rug_o) = rug_exp_prec_round(&rug::Float::exact_from(&x), prec, rug_rm);
        assert_eq!(ComparableFloatRef(&Float::from(&rug_e)), ComparableFloatRef(&e));
        assert_eq!(rug_o, o);
    }

    // exp is never negative: the result is positive, +0, +inf, or NaN.
    assert!(e.is_nan() || e.is_sign_positive());

    if e.is_normal() {
        assert_eq!(e.get_prec(), Some(prec));
    }

    if o == Equal {
        // exp is exact only for special inputs, so the result is rounding-mode-invariant.
        for rm2 in exhaustive_rounding_modes() {
            let (e2, o2) = x.exp_prec_round_ref(prec, rm2);
            assert_eq!(
                ComparableFloat(e2.abs_negative_zero_ref()),
                ComparableFloat(e.abs_negative_zero_ref())
            );
            assert_eq!(o2, Equal);
        }
    } else {
        assert_panic!(x.exp_prec_round_ref(prec, Exact));
    }
}

#[test]
fn exp_prec_round_properties() {
    float_unsigned_rounding_mode_triple_gen_var_36().test_properties(|(x, prec, rm)| {
        exp_prec_round_properties_helper(x, prec, rm);
    });

    unsigned_rounding_mode_pair_gen_var_3().test_properties(|(prec, rm)| {
        let (e, o) = Float::NAN.exp_prec_round(prec, rm);
        assert!(e.is_nan());
        assert_eq!(o, Equal);

        assert_eq!(
            Float::INFINITY.exp_prec_round(prec, rm),
            (Float::INFINITY, Equal)
        );
        assert_eq!(
            Float::NEGATIVE_INFINITY.exp_prec_round(prec, rm),
            (Float::ZERO, Equal)
        );
        assert_eq!(
            Float::ZERO.exp_prec_round(prec, rm),
            (Float::one_prec(prec), Equal)
        );
        assert_eq!(
            Float::NEGATIVE_ZERO.exp_prec_round(prec, rm),
            (Float::one_prec(prec), Equal)
        );
    });
}

#[test]
fn exp_round_properties() {
    float_rounding_mode_pair_gen_var_47().test_properties(|(x, rm)| {
        let (e, o) = x.clone().exp_round(rm);
        assert!(e.is_valid());

        let (e_alt, o_alt) = x.exp_round_ref(rm);
        assert!(e_alt.is_valid());
        assert_eq!(ComparableFloatRef(&e_alt), ComparableFloatRef(&e));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.exp_round_assign(rm);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&e));
        assert_eq!(o_alt, o);

        // exp_round is exp_prec_round at the input's precision.
        let (e_alt, o_alt) = x.exp_prec_round_ref(x.significant_bits(), rm);
        assert_eq!(ComparableFloatRef(&e_alt), ComparableFloatRef(&e));
        assert_eq!(o_alt, o);

        assert!(e.is_nan() || e.is_sign_positive());

        if let Ok(rug_rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_e, rug_o) = rug_exp_round(&rug::Float::exact_from(&x), rug_rm);
            assert_eq!(ComparableFloatRef(&Float::from(&rug_e)), ComparableFloatRef(&e));
            assert_eq!(rug_o, o);
        }
    });
}

#[test]
fn exp_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (e, o) = x.clone().exp_prec(prec);
        assert!(e.is_valid());

        let (e_alt, o_alt) = x.exp_prec_ref(prec);
        assert!(e_alt.is_valid());
        assert_eq!(ComparableFloatRef(&e_alt), ComparableFloatRef(&e));
        assert_eq!(o_alt, o);

        let mut x_alt = x.clone();
        let o_alt = x_alt.exp_prec_assign(prec);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&e));
        assert_eq!(o_alt, o);

        // exp_prec is exp_prec_round with Nearest.
        let (e_alt, o_alt) = x.exp_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&e_alt), ComparableFloatRef(&e));
        assert_eq!(o_alt, o);

        assert!(e.is_nan() || e.is_sign_positive());
        if e.is_normal() {
            assert_eq!(e.get_prec(), Some(prec));
        }

        let (rug_e, rug_o) = rug_exp_prec(&rug::Float::exact_from(&x), prec);
        assert_eq!(ComparableFloatRef(&Float::from(&rug_e)), ComparableFloatRef(&e));
        assert_eq!(rug_o, o);
    });
}

#[test]
fn exp_properties() {
    float_gen().test_properties(|x| {
        let e = x.clone().exp();
        assert!(e.is_valid());

        let e_alt = (&x).exp();
        assert!(e_alt.is_valid());
        assert_eq!(ComparableFloatRef(&e_alt), ComparableFloatRef(&e));

        let mut x_alt = x.clone();
        x_alt.exp_assign();
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&e));

        // exp is exp_round at the input's precision with Nearest.
        let e_alt = x.exp_round_ref(Nearest).0;
        assert_eq!(ComparableFloatRef(&e_alt), ComparableFloatRef(&e));

        assert!(e.is_nan() || e.is_sign_positive());

        let rug_e = Float::from(&rug_exp(&rug::Float::exact_from(&x)));
        assert_eq!(ComparableFloatRef(&rug_e), ComparableFloatRef(&e));
    });
}
