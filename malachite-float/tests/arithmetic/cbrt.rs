// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use malachite_base::assert_panic;
use malachite_base::num::arithmetic::traits::{Cbrt, Root};
use malachite_base::num::basic::traits::NaN as NanTrait;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_base::rounding_modes::exhaustive::exhaustive_rounding_modes;
use malachite_float::arithmetic::cbrt::{primitive_float_cbrt, primitive_float_cbrt_rational};
use malachite_float::test_util::common::{parse_hex_string, to_hex_string};
use malachite_float::test_util::generators::{float_gen, float_unsigned_pair_gen_var_1};
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_q::Rational;
use malachite_q::test_util::generators::rational_unsigned_pair_gen_var_3;
use std::panic::catch_unwind;

#[test]
fn test_cbrt_prec_round() {
    let test = |s, s_hex, prec: u64, rm, out: &str, out_hex: &str, o_out: Ordering| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (cbrt, o) = x.clone().cbrt_prec_round(prec, rm);
        assert!(cbrt.is_valid());
        assert_eq!(cbrt.to_string(), out);
        assert_eq!(to_hex_string(&cbrt), out_hex);
        assert_eq!(o, o_out);

        let (cbrt_alt, o_alt) = x.cbrt_prec_round_ref(prec, rm);
        assert!(cbrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&cbrt_alt), ComparableFloatRef(&cbrt));
        assert_eq!(o_alt, o_out);

        let mut cbrt_alt = x.clone();
        let o_alt = cbrt_alt.cbrt_prec_round_assign(prec, rm);
        assert!(cbrt_alt.is_valid());
        assert_eq!(ComparableFloatRef(&cbrt_alt), ComparableFloatRef(&cbrt));
        assert_eq!(o_alt, o_out);

        // cbrt(x) delegates to the cube root, root_u(x, 3).
        let (root, root_o) = x.root_u_prec_round_ref(3, prec, rm);
        assert_eq!(ComparableFloatRef(&root), ComparableFloatRef(&cbrt));
        assert_eq!(root_o, o_out);
    };
    // Specials
    test("NaN", "NaN", 5, Floor, "NaN", "NaN", Equal);
    test("NaN", "NaN", 5, Nearest, "NaN", "NaN", Equal);
    test(
        "Infinity", "Infinity", 5, Floor, "Infinity", "Infinity", Equal,
    );
    test(
        "Infinity", "Infinity", 5, Nearest, "Infinity", "Infinity", Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        5,
        Floor,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test(
        "-Infinity",
        "-Infinity",
        5,
        Nearest,
        "-Infinity",
        "-Infinity",
        Equal,
    );
    test("0.0", "0x0.0", 5, Floor, "0.0", "0x0.0", Equal);
    test("-0.0", "-0x0.0", 5, Nearest, "-0.0", "-0x0.0", Equal);
    // Perfect cubes are exact, even under Exact rounding. The cube root of a negative number is a
    // negative real (unlike the square root).
    test("27.0", "0x1b.0#5", 10, Floor, "3.0", "0x3.00#10", Equal);
    test("27.0", "0x1b.0#5", 10, Exact, "3.0", "0x3.00#10", Equal);
    test("8.0", "0x8.0#1", 10, Nearest, "2.0", "0x2.00#10", Equal);
    test("8.0", "0x8.0#1", 10, Exact, "2.0", "0x2.00#10", Equal);
    test("-8.0", "-0x8.0#1", 10, Nearest, "-2.0", "-0x2.00#10", Equal);
    test("-8.0", "-0x8.0#1", 10, Exact, "-2.0", "-0x2.00#10", Equal);
    test("-27.0", "-0x1b.0#5", 10, Exact, "-3.0", "-0x3.00#10", Equal);
    // Inexact roots
    test("2.0", "0x2.0#1", 10, Floor, "1.26", "0x1.428#10", Less);
    test(
        "2.0",
        "0x2.0#1",
        10,
        Ceiling,
        "1.262",
        "0x1.430#10",
        Greater,
    );
    test("2.0", "0x2.0#1", 10, Nearest, "1.26", "0x1.428#10", Less);
    test("0.5", "0x0.8#1", 20, Floor, "0.7937", "0x0.cb2ff#20", Less);
    test(
        "0.5",
        "0x0.8#1",
        20,
        Ceiling,
        "0.793701",
        "0x0.cb300#20",
        Greater,
    );
    test(
        "0.5",
        "0x0.8#1",
        20,
        Nearest,
        "0.7937",
        "0x0.cb2ff#20",
        Less,
    );
}

#[test]
fn test_cbrt() {
    let test = |s_hex: &str, out: &str| {
        let x = parse_hex_string(s_hex);
        let cbrt = x.clone().cbrt();
        assert!(cbrt.is_valid());
        assert_eq!(cbrt.to_string(), out);

        let cbrt_alt = (&x).cbrt();
        assert_eq!(ComparableFloatRef(&cbrt_alt), ComparableFloatRef(&cbrt));

        // At the precision of the input and rounding to nearest, cbrt agrees with cbrt_prec.
        let (cbrt_prec, _) = x.cbrt_prec_ref(x.significant_bits());
        assert_eq!(ComparableFloatRef(&cbrt_prec), ComparableFloatRef(&cbrt));
    };
    test("NaN", "NaN");
    test("Infinity", "Infinity");
    test("-Infinity", "-Infinity");
    test("0x0.0", "0.0");
    test("-0x0.0", "-0.0");
    test("0x1b.0#5", "3.0");
    test("-0x8.0#1", "-2.0");
    test("0x2.0#1", "1.0");
}

#[test]
fn test_cbrt_rational_prec() {
    let test = |x: Rational, prec: u64, out: &str, o_out: Ordering| {
        let (cbrt, o) = Float::cbrt_rational_prec(x.clone(), prec);
        assert!(cbrt.is_valid());
        assert_eq!(cbrt.to_string(), out);
        assert_eq!(o, o_out);

        let (cbrt_alt, o_alt) = Float::cbrt_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&cbrt_alt), ComparableFloatRef(&cbrt));
        assert_eq!(o_alt, o_out);

        // Delegates to root_u_rational(x, 3).
        let (root, root_o) = Float::root_u_rational_prec_ref(&x, 3, prec);
        assert_eq!(ComparableFloatRef(&root), ComparableFloatRef(&cbrt));
        assert_eq!(root_o, o_out);
    };
    test(Rational::from(27), 10, "3.0", Equal);
    test(Rational::from(-8), 10, "-2.0", Equal);
    test(Rational::from(2), 10, "1.26", Less);
}

#[test]
fn cbrt_prec_fail() {
    assert_panic!(Float::NAN.cbrt_prec(0));
    assert_panic!(Float::NAN.cbrt_prec_ref(0));
    assert_panic!({
        let mut x = Float::NAN;
        x.cbrt_prec_assign(0)
    });
}

#[test]
fn cbrt_prec_round_fail() {
    assert_panic!(Float::NAN.cbrt_prec_round(0, Floor));
    // Exact rounding of an inexact cube root panics.
    assert_panic!(Float::from(2.0).cbrt_prec_round(10, Exact));
    assert_panic!(Float::from(2.0).cbrt_round(Exact));
}

fn cbrt_prec_round_properties_helper(x: &Float, prec: u64, rm: RoundingMode) {
    if rm == Exact {
        // Exact is allowed only when the cube root is exactly representable; otherwise panic.
        let (c, o) = x.cbrt_prec_round_ref(prec, Nearest);
        if o == Equal {
            let (ce, oe) = x.cbrt_prec_round_ref(prec, Exact);
            assert_eq!(ComparableFloatRef(&ce), ComparableFloatRef(&c));
            assert_eq!(oe, Equal);
        } else {
            assert_panic!(x.cbrt_prec_round_ref(prec, Exact));
        }
        return;
    }
    let (c, o) = x.clone().cbrt_prec_round(prec, rm);
    assert!(c.is_valid());

    // cbrt is exactly root_u(., 3).
    let (r, ro) = x.root_u_prec_round_ref(3, prec, rm);
    assert_eq!(ComparableFloatRef(&c), ComparableFloatRef(&r));
    assert_eq!(o, ro);

    let (c_alt, o_alt) = x.cbrt_prec_round_ref(prec, rm);
    assert!(c_alt.is_valid());
    assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    let mut x_alt = x.clone();
    let o_alt = x_alt.cbrt_prec_round_assign(prec, rm);
    assert!(x_alt.is_valid());
    assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
    assert_eq!(o_alt, o);

    if c.is_normal() {
        assert_eq!(c.get_prec(), Some(prec));
    }
}

#[test]
fn cbrt_prec_round_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        for rm in exhaustive_rounding_modes() {
            cbrt_prec_round_properties_helper(&x, prec, rm);
        }
    });
}

#[test]
fn cbrt_prec_properties() {
    float_unsigned_pair_gen_var_1().test_properties(|(x, prec)| {
        let (c, o) = x.clone().cbrt_prec(prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = x.cbrt_prec_ref(prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let mut x_alt = x.clone();
        let o_alt = x_alt.cbrt_prec_assign(prec);
        assert_eq!(ComparableFloatRef(&x_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        // cbrt_prec is cbrt_prec_round with Nearest, and equals root_u_prec(., 3).
        let (c_alt, o_alt) = x.cbrt_prec_round_ref(prec, Nearest);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        let (r, ro) = x.root_u_prec_ref(3, prec);
        assert_eq!(ComparableFloatRef(&r), ComparableFloatRef(&c));
        assert_eq!(ro, o);
    });
}

#[test]
fn cbrt_properties() {
    float_gen().test_properties(|x| {
        let c = x.clone().cbrt();
        assert!(c.is_valid());
        let c_alt = (&x).cbrt();
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));

        // The trait uses the precision of the input, rounding to nearest, and equals root_u(., 3).
        let (c_prec, _) = x.cbrt_prec_ref(x.significant_bits());
        assert_eq!(ComparableFloatRef(&c_prec), ComparableFloatRef(&c));
        let root = (&x).root(3u64);
        assert_eq!(ComparableFloatRef(&root), ComparableFloatRef(&c));

        // cbrt is an odd function: cbrt(-x) == -cbrt(x).
        if !x.is_nan() {
            let neg = (-&x).cbrt();
            assert_eq!(ComparableFloat(-c), ComparableFloat(neg));
        }
    });
}

#[test]
fn cbrt_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (c, o) = Float::cbrt_rational_prec(x.clone(), prec);
        assert!(c.is_valid());
        let (c_alt, o_alt) = Float::cbrt_rational_prec_ref(&x, prec);
        assert_eq!(ComparableFloatRef(&c_alt), ComparableFloatRef(&c));
        assert_eq!(o_alt, o);
        // Delegates to root_u_rational(x, 3).
        let (r, ro) = Float::root_u_rational_prec_ref(&x, 3, prec);
        assert_eq!(ComparableFloatRef(&r), ComparableFloatRef(&c));
        assert_eq!(ro, o);
    });
}

#[test]
fn primitive_float_cbrt_properties() {
    // Correctly-rounded primitive-float cube root matches the exact algebraic values.
    assert_eq!(primitive_float_cbrt::<f64>(27.0), 3.0);
    assert_eq!(primitive_float_cbrt::<f64>(-8.0), -2.0);
    assert_eq!(primitive_float_cbrt::<f32>(64.0), 4.0);
    assert!(primitive_float_cbrt::<f64>(f64::NAN).is_nan());
    assert_eq!(primitive_float_cbrt::<f64>(f64::INFINITY), f64::INFINITY);
    assert_eq!(
        primitive_float_cbrt::<f64>(f64::NEG_INFINITY),
        f64::NEG_INFINITY
    );
    assert_eq!(
        primitive_float_cbrt_rational::<f64>(&Rational::from(27)),
        3.0
    );
    assert_eq!(
        primitive_float_cbrt_rational::<f64>(&Rational::from(-8)),
        -2.0
    );
}
