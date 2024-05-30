// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_float::test_util::common::rug_round_try_from_rounding_mode;
use malachite_float::test_util::common::to_hex_string;
use malachite_float::test_util::generators::rational_unsigned_rounding_mode_triple_gen_var_1;
use malachite_float::{ComparableFloat, ComparableFloatRef, Float};
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::conversion::primitive_float_from_rational::FloatFromRationalError;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_3};
use malachite_q::Rational;
use std::cmp::Ordering::*;
use std::panic::catch_unwind;
use std::str::FromStr;

#[test]
fn test_from_rational_prec() {
    let test = |s, prec, out, out_hex, out_o| {
        let u = Rational::from_str(s).unwrap();

        let (x, o) = Float::from_rational_prec(u.clone(), prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = Float::from_rational_prec_ref(&u, prec);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let rug_x = rug::Float::with_val(u32::exact_from(prec), rug::Rational::from(&u));
        let x = Float::exact_from(&rug_x);
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
    };
    test("0", 1, "0.0", "0x0.0", Equal);
    test("0", 10, "0.0", "0x0.0", Equal);
    test("0", 100, "0.0", "0x0.0", Equal);
    test("1", 1, "1.0", "0x1.0#1", Equal);
    test("1", 10, "1.0", "0x1.000#10", Equal);
    test("1", 100, "1.0", "0x1.0000000000000000000000000#100", Equal);
    test("1/2", 1, "0.5", "0x0.8#1", Equal);
    test("1/2", 10, "0.5", "0x0.800#10", Equal);
    test(
        "1/2",
        100,
        "0.5",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test("1/3", 1, "0.2", "0x0.4#1", Less);
    test("1/3", 10, "0.3335", "0x0.556#10", Greater);
    test(
        "1/3",
        100,
        "0.3333333333333333333333333333335",
        "0x0.55555555555555555555555558#100",
        Greater,
    );
    test("22/7", 1, "4.0", "0x4.0#1", Greater);
    test("22/7", 10, "3.145", "0x3.25#10", Greater);
    test(
        "22/7",
        100,
        "3.142857142857142857142857142858",
        "0x3.2492492492492492492492494#100",
        Greater,
    );
}

#[test]
fn from_rational_prec_fail() {
    assert_panic!(Float::from_rational_prec(Rational::ZERO, 0));
    assert_panic!(Float::from_rational_prec(Rational::ONE, 0));
    assert_panic!(Float::from_rational_prec(Rational::NEGATIVE_ONE, 0));
}

#[test]
fn from_rational_prec_ref_fail() {
    assert_panic!(Float::from_rational_prec_ref(&Rational::ZERO, 0));
    assert_panic!(Float::from_rational_prec_ref(&Rational::ONE, 0));
    assert_panic!(Float::from_rational_prec_ref(&Rational::NEGATIVE_ONE, 0));
}

#[test]
fn test_from_rational_prec_round() {
    let test = |s, prec, rm, out, out_hex, out_o| {
        let u = Rational::from_str(s).unwrap();

        let (x, o) = Float::from_rational_prec_round(u.clone(), prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        let (x, o) = Float::from_rational_prec_round_ref(&u, prec, rm);
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out);
        assert_eq!(to_hex_string(&x), out_hex);
        assert_eq!(o, out_o);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_x, o) =
                rug::Float::with_val_round(u32::exact_from(prec), rug::Rational::from(&u), rm);
            let x = Float::exact_from(&rug_x);
            assert_eq!(x.to_string(), out);
            assert_eq!(to_hex_string(&x), out_hex);
            assert_eq!(o, out_o);
        }
    };
    test("0", 1, Floor, "0.0", "0x0.0", Equal);
    test("0", 1, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 1, Down, "0.0", "0x0.0", Equal);
    test("0", 1, Up, "0.0", "0x0.0", Equal);
    test("0", 1, Nearest, "0.0", "0x0.0", Equal);
    test("0", 1, Exact, "0.0", "0x0.0", Equal);

    test("0", 10, Floor, "0.0", "0x0.0", Equal);
    test("0", 10, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 10, Down, "0.0", "0x0.0", Equal);
    test("0", 10, Up, "0.0", "0x0.0", Equal);
    test("0", 10, Nearest, "0.0", "0x0.0", Equal);
    test("0", 10, Exact, "0.0", "0x0.0", Equal);

    test("0", 100, Floor, "0.0", "0x0.0", Equal);
    test("0", 100, Ceiling, "0.0", "0x0.0", Equal);
    test("0", 100, Down, "0.0", "0x0.0", Equal);
    test("0", 100, Up, "0.0", "0x0.0", Equal);
    test("0", 100, Nearest, "0.0", "0x0.0", Equal);
    test("0", 100, Exact, "0.0", "0x0.0", Equal);

    test("1", 1, Floor, "1.0", "0x1.0#1", Equal);
    test("1", 1, Ceiling, "1.0", "0x1.0#1", Equal);
    test("1", 1, Down, "1.0", "0x1.0#1", Equal);
    test("1", 1, Up, "1.0", "0x1.0#1", Equal);
    test("1", 1, Nearest, "1.0", "0x1.0#1", Equal);
    test("1", 1, Exact, "1.0", "0x1.0#1", Equal);

    test("1", 10, Floor, "1.0", "0x1.000#10", Equal);
    test("1", 10, Ceiling, "1.0", "0x1.000#10", Equal);
    test("1", 10, Down, "1.0", "0x1.000#10", Equal);
    test("1", 10, Up, "1.0", "0x1.000#10", Equal);
    test("1", 10, Nearest, "1.0", "0x1.000#10", Equal);
    test("1", 10, Exact, "1.0", "0x1.000#10", Equal);

    test(
        "1",
        100,
        Floor,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Ceiling,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Down,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Up,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Nearest,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "1",
        100,
        Exact,
        "1.0",
        "0x1.0000000000000000000000000#100",
        Equal,
    );

    test("1/2", 1, Floor, "0.5", "0x0.8#1", Equal);
    test("1/2", 1, Ceiling, "0.5", "0x0.8#1", Equal);
    test("1/2", 1, Down, "0.5", "0x0.8#1", Equal);
    test("1/2", 1, Up, "0.5", "0x0.8#1", Equal);
    test("1/2", 1, Nearest, "0.5", "0x0.8#1", Equal);
    test("1/2", 1, Exact, "0.5", "0x0.8#1", Equal);

    test("1/2", 10, Floor, "0.5", "0x0.800#10", Equal);
    test("1/2", 10, Ceiling, "0.5", "0x0.800#10", Equal);
    test("1/2", 10, Down, "0.5", "0x0.800#10", Equal);
    test("1/2", 10, Up, "0.5", "0x0.800#10", Equal);
    test("1/2", 10, Nearest, "0.5", "0x0.800#10", Equal);
    test("1/2", 10, Exact, "0.5", "0x0.800#10", Equal);

    test(
        "1/2",
        100,
        Floor,
        "0.5",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Ceiling,
        "0.5",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Down,
        "0.5",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Up,
        "0.5",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Nearest,
        "0.5",
        "0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "1/2",
        100,
        Exact,
        "0.5",
        "0x0.8000000000000000000000000#100",
        Equal,
    );

    test("1/3", 1, Floor, "0.2", "0x0.4#1", Less);
    test("1/3", 1, Ceiling, "0.5", "0x0.8#1", Greater);
    test("1/3", 1, Down, "0.2", "0x0.4#1", Less);
    test("1/3", 1, Up, "0.5", "0x0.8#1", Greater);
    test("1/3", 1, Nearest, "0.2", "0x0.4#1", Less);

    test("1/3", 10, Floor, "0.333", "0x0.554#10", Less);
    test("1/3", 10, Ceiling, "0.3335", "0x0.556#10", Greater);
    test("1/3", 10, Down, "0.333", "0x0.554#10", Less);
    test("1/3", 10, Up, "0.3335", "0x0.556#10", Greater);
    test("1/3", 10, Nearest, "0.3335", "0x0.556#10", Greater);

    test(
        "1/3",
        100,
        Floor,
        "0.3333333333333333333333333333331",
        "0x0.55555555555555555555555550#100",
        Less,
    );
    test(
        "1/3",
        100,
        Ceiling,
        "0.3333333333333333333333333333335",
        "0x0.55555555555555555555555558#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Down,
        "0.3333333333333333333333333333331",
        "0x0.55555555555555555555555550#100",
        Less,
    );
    test(
        "1/3",
        100,
        Up,
        "0.3333333333333333333333333333335",
        "0x0.55555555555555555555555558#100",
        Greater,
    );
    test(
        "1/3",
        100,
        Nearest,
        "0.3333333333333333333333333333335",
        "0x0.55555555555555555555555558#100",
        Greater,
    );

    test("22/7", 1, Floor, "2.0", "0x2.0#1", Less);
    test("22/7", 1, Ceiling, "4.0", "0x4.0#1", Greater);
    test("22/7", 1, Down, "2.0", "0x2.0#1", Less);
    test("22/7", 1, Up, "4.0", "0x4.0#1", Greater);
    test("22/7", 1, Nearest, "4.0", "0x4.0#1", Greater);

    test("22/7", 10, Floor, "3.141", "0x3.24#10", Less);
    test("22/7", 10, Ceiling, "3.145", "0x3.25#10", Greater);
    test("22/7", 10, Down, "3.141", "0x3.24#10", Less);
    test("22/7", 10, Up, "3.145", "0x3.25#10", Greater);
    test("22/7", 10, Nearest, "3.145", "0x3.25#10", Greater);

    test(
        "22/7",
        100,
        Floor,
        "3.142857142857142857142857142855",
        "0x3.2492492492492492492492490#100",
        Less,
    );
    test(
        "22/7",
        100,
        Ceiling,
        "3.142857142857142857142857142858",
        "0x3.2492492492492492492492494#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Down,
        "3.142857142857142857142857142855",
        "0x3.2492492492492492492492490#100",
        Less,
    );
    test(
        "22/7",
        100,
        Up,
        "3.142857142857142857142857142858",
        "0x3.2492492492492492492492494#100",
        Greater,
    );
    test(
        "22/7",
        100,
        Nearest,
        "3.142857142857142857142857142858",
        "0x3.2492492492492492492492494#100",
        Greater,
    );

    test("-1", 1, Floor, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Ceiling, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Down, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Up, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Nearest, "-1.0", "-0x1.0#1", Equal);
    test("-1", 1, Exact, "-1.0", "-0x1.0#1", Equal);

    test("-1", 10, Floor, "-1.0", "-0x1.000#10", Equal);
    test("-1", 10, Ceiling, "-1.0", "-0x1.000#10", Equal);
    test("-1", 10, Down, "-1.0", "-0x1.000#10", Equal);
    test("-1", 10, Up, "-1.0", "-0x1.000#10", Equal);
    test("-1", 10, Nearest, "-1.0", "-0x1.000#10", Equal);
    test("-1", 10, Exact, "-1.0", "-0x1.000#10", Equal);

    test(
        "-1",
        100,
        Floor,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1",
        100,
        Ceiling,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1",
        100,
        Down,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1",
        100,
        Up,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1",
        100,
        Nearest,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );
    test(
        "-1",
        100,
        Exact,
        "-1.0",
        "-0x1.0000000000000000000000000#100",
        Equal,
    );

    test("-1/2", 1, Floor, "-0.5", "-0x0.8#1", Equal);
    test("-1/2", 1, Ceiling, "-0.5", "-0x0.8#1", Equal);
    test("-1/2", 1, Down, "-0.5", "-0x0.8#1", Equal);
    test("-1/2", 1, Up, "-0.5", "-0x0.8#1", Equal);
    test("-1/2", 1, Nearest, "-0.5", "-0x0.8#1", Equal);
    test("-1/2", 1, Exact, "-0.5", "-0x0.8#1", Equal);

    test("-1/2", 10, Floor, "-0.5", "-0x0.800#10", Equal);
    test("-1/2", 10, Ceiling, "-0.5", "-0x0.800#10", Equal);
    test("-1/2", 10, Down, "-0.5", "-0x0.800#10", Equal);
    test("-1/2", 10, Up, "-0.5", "-0x0.800#10", Equal);
    test("-1/2", 10, Nearest, "-0.5", "-0x0.800#10", Equal);
    test("-1/2", 10, Exact, "-0.5", "-0x0.800#10", Equal);

    test(
        "-1/2",
        100,
        Floor,
        "-0.5",
        "-0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "-1/2",
        100,
        Ceiling,
        "-0.5",
        "-0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "-1/2",
        100,
        Down,
        "-0.5",
        "-0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "-1/2",
        100,
        Up,
        "-0.5",
        "-0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "-1/2",
        100,
        Nearest,
        "-0.5",
        "-0x0.8000000000000000000000000#100",
        Equal,
    );
    test(
        "-1/2",
        100,
        Exact,
        "-0.5",
        "-0x0.8000000000000000000000000#100",
        Equal,
    );

    test("-1/3", 1, Floor, "-0.5", "-0x0.8#1", Less);
    test("-1/3", 1, Ceiling, "-0.2", "-0x0.4#1", Greater);
    test("-1/3", 1, Down, "-0.2", "-0x0.4#1", Greater);
    test("-1/3", 1, Up, "-0.5", "-0x0.8#1", Less);
    test("-1/3", 1, Nearest, "-0.2", "-0x0.4#1", Greater);

    test("-1/3", 10, Floor, "-0.3335", "-0x0.556#10", Less);
    test("-1/3", 10, Ceiling, "-0.333", "-0x0.554#10", Greater);
    test("-1/3", 10, Down, "-0.333", "-0x0.554#10", Greater);
    test("-1/3", 10, Up, "-0.3335", "-0x0.556#10", Less);
    test("-1/3", 10, Nearest, "-0.3335", "-0x0.556#10", Less);

    test(
        "-1/3",
        100,
        Floor,
        "-0.3333333333333333333333333333335",
        "-0x0.55555555555555555555555558#100",
        Less,
    );
    test(
        "-1/3",
        100,
        Ceiling,
        "-0.3333333333333333333333333333331",
        "-0x0.55555555555555555555555550#100",
        Greater,
    );
    test(
        "-1/3",
        100,
        Down,
        "-0.3333333333333333333333333333331",
        "-0x0.55555555555555555555555550#100",
        Greater,
    );
    test(
        "-1/3",
        100,
        Up,
        "-0.3333333333333333333333333333335",
        "-0x0.55555555555555555555555558#100",
        Less,
    );
    test(
        "-1/3",
        100,
        Nearest,
        "-0.3333333333333333333333333333335",
        "-0x0.55555555555555555555555558#100",
        Less,
    );

    test("-22/7", 1, Floor, "-4.0", "-0x4.0#1", Less);
    test("-22/7", 1, Ceiling, "-2.0", "-0x2.0#1", Greater);
    test("-22/7", 1, Down, "-2.0", "-0x2.0#1", Greater);
    test("-22/7", 1, Up, "-4.0", "-0x4.0#1", Less);
    test("-22/7", 1, Nearest, "-4.0", "-0x4.0#1", Less);

    test("-22/7", 10, Floor, "-3.145", "-0x3.25#10", Less);
    test("-22/7", 10, Ceiling, "-3.141", "-0x3.24#10", Greater);
    test("-22/7", 10, Down, "-3.141", "-0x3.24#10", Greater);
    test("-22/7", 10, Up, "-3.145", "-0x3.25#10", Less);
    test("-22/7", 10, Nearest, "-3.145", "-0x3.25#10", Less);

    test(
        "-22/7",
        100,
        Floor,
        "-3.142857142857142857142857142858",
        "-0x3.2492492492492492492492494#100",
        Less,
    );
    test(
        "-22/7",
        100,
        Ceiling,
        "-3.142857142857142857142857142855",
        "-0x3.2492492492492492492492490#100",
        Greater,
    );
    test(
        "-22/7",
        100,
        Down,
        "-3.142857142857142857142857142855",
        "-0x3.2492492492492492492492490#100",
        Greater,
    );
    test(
        "-22/7",
        100,
        Up,
        "-3.142857142857142857142857142858",
        "-0x3.2492492492492492492492494#100",
        Less,
    );
    test(
        "-22/7",
        100,
        Nearest,
        "-3.142857142857142857142857142858",
        "-0x3.2492492492492492492492494#100",
        Less,
    );
}

#[test]
fn from_rational_prec_round_fail() {
    assert_panic!(Float::from_rational_prec_round(Rational::ZERO, 0, Floor));
    assert_panic!(Float::from_rational_prec_round(Rational::ONE, 0, Floor));
    assert_panic!(Float::from_rational_prec_round(
        Rational::from(123u32),
        1,
        Exact
    ));
    assert_panic!(Float::from_rational_prec_round(
        Rational::from_unsigneds(1u8, 3),
        100,
        Exact
    ));
    assert_panic!(Float::from_rational_prec_round(
        Rational::NEGATIVE_ONE,
        0,
        Floor
    ));
    assert_panic!(Float::from_rational_prec_round(
        Rational::from(-123),
        1,
        Exact
    ));
    assert_panic!(Float::from_rational_prec_round(
        Rational::from_signeds(-1i8, 3),
        100,
        Exact
    ));
}

#[test]
fn from_rational_prec_round_ref_fail() {
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::ZERO,
        0,
        Floor
    ));
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::ONE,
        0,
        Floor
    ));
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::from(123u32),
        1,
        Exact
    ));
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::from_unsigneds(1u8, 3),
        100,
        Exact
    ));
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::NEGATIVE_ONE,
        0,
        Floor
    ));
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::from(-123),
        1,
        Exact
    ));
    assert_panic!(Float::from_rational_prec_round_ref(
        &Rational::from_signeds(-1i8, 3),
        100,
        Exact
    ));
}

#[allow(clippy::needless_borrow)]
#[test]
fn test_try_from_rational() {
    let test = |s, out, out_hex| {
        let x = Rational::from_str(s).unwrap();

        let of = Float::try_from(x.clone());
        assert!(of.as_ref().map_or(true, Float::is_valid));
        let ofs = of.as_ref().map(ToString::to_string);
        assert_eq!(ofs.as_ref().map(String::as_str), out);
        let ofs = of.map(|f| to_hex_string(&f));
        assert_eq!(ofs.as_ref().map(String::as_str), out_hex);

        let of = Float::try_from(&x);
        assert!(of.as_ref().map_or(true, Float::is_valid));
        let ofs = of.as_ref().map(ToString::to_string);
        assert_eq!(ofs.as_ref().map(String::as_str), out);
        let ofs = of.map(|f| to_hex_string(&f));
        assert_eq!(ofs.as_ref().map(String::as_str), out_hex);
    };
    test("0", Ok("0.0"), Ok("0x0.0"));
    test("1", Ok("1.0"), Ok("0x1.0#1"));
    test("1/2", Ok("0.5"), Ok("0x0.8#1"));
    test("117/256", Ok("0.457"), Ok("0x0.75#7"));
    test(
        "6369051672525773/4503599627370496",
        Ok("1.4142135623730951"),
        Ok("0x1.6a09e667f3bcd#53"),
    );
    test(
        "884279719003555/281474976710656",
        Ok("3.141592653589793"),
        Ok("0x3.243f6a8885a3#50"),
    );
    test(
        "6121026514868073/2251799813685248",
        Ok("2.7182818284590451"),
        Ok("0x2.b7e151628aed2#53"),
    );
    test("-1", Ok("-1.0"), Ok("-0x1.0#1"));
    test("-1/2", Ok("-0.5"), Ok("-0x0.8#1"));
    test("-117/256", Ok("-0.457"), Ok("-0x0.75#7"));
    test(
        "-6369051672525773/4503599627370496",
        Ok("-1.4142135623730951"),
        Ok("-0x1.6a09e667f3bcd#53"),
    );
    test(
        "-884279719003555/281474976710656",
        Ok("-3.141592653589793"),
        Ok("-0x3.243f6a8885a3#50"),
    );
    test(
        "-6121026514868073/2251799813685248",
        Ok("-2.7182818284590451"),
        Ok("-0x2.b7e151628aed2#53"),
    );

    test(
        "1/3",
        Err(&&FloatFromRationalError),
        Err(&&FloatFromRationalError),
    );
    test(
        "22/7",
        Err(&&FloatFromRationalError),
        Err(&&FloatFromRationalError),
    );
    test(
        "-1/3",
        Err(&&FloatFromRationalError),
        Err(&&FloatFromRationalError),
    );
    test(
        "-22/7",
        Err(&&FloatFromRationalError),
        Err(&&FloatFromRationalError),
    );
}

#[test]
fn test_convertible_from_rational() {
    let test = |s, out| {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(Float::convertible_from(&x), out);
    };
    test("0", true);
    test("1", true);
    test("1/2", true);
    test("117/256", true);
    test("6369051672525773/4503599627370496", true);
    test("884279719003555/281474976710656", true);
    test("6121026514868073/2251799813685248", true);
    test("-1", true);
    test("-1/2", true);
    test("-117/256", true);
    test("-6369051672525773/4503599627370496", true);
    test("-884279719003555/281474976710656", true);
    test("-6121026514868073/2251799813685248", true);

    test("1/3", false);
    test("22/7", false);
    test("-1/3", false);
    test("-22/7", false);
}

#[test]
fn from_rational_prec_properties() {
    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let (float_x, o) = Float::from_rational_prec(x.clone(), prec);
        assert!(float_x.is_valid());

        let (float_x_alt, o_alt) = Float::from_rational_prec_ref(&x, prec);
        assert!(float_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_x_alt),
            ComparableFloatRef(&float_x)
        );
        assert_eq!(o, o_alt);
        assert_eq!(float_x.partial_cmp(&x), Some(o));

        let rug_x = rug::Float::with_val(u32::exact_from(prec), rug::Rational::from(&x));
        assert_eq!(
            ComparableFloatRef(&float_x),
            ComparableFloatRef(&Float::from(&rug_x))
        );

        assert_eq!(x == 0u32, float_x == 0u32);
        assert_eq!(
            float_x.get_prec(),
            if x == 0u32 { None } else { Some(prec) }
        );
        if x != 0u32 {
            assert!((Rational::exact_from(&float_x) - &x)
                .le_abs(&(Rational::exact_from(float_x.ulp().unwrap()) >> 1)));
        }
    });
}

#[test]
fn from_rational_prec_round_properties() {
    rational_unsigned_rounding_mode_triple_gen_var_1().test_properties(|(x, prec, rm)| {
        let (float_x, o) = Float::from_rational_prec_round(x.clone(), prec, rm);
        assert!(float_x.is_valid());

        let (float_x_alt, o_alt) = Float::from_rational_prec_round_ref(&x, prec, rm);
        assert!(float_x_alt.is_valid());
        assert_eq!(
            ComparableFloatRef(&float_x_alt),
            ComparableFloatRef(&float_x)
        );
        assert_eq!(o, o_alt);
        assert_eq!(float_x.partial_cmp(&x), Some(o));
        match (x >= 0, rm) {
            (_, Floor) | (true, Down) | (false, Up) => {
                assert_ne!(o, Greater);
            }
            (_, Ceiling) | (true, Up) | (false, Down) => {
                assert_ne!(o, Less);
            }
            (_, Exact) => assert_eq!(o, Equal),
            _ => {}
        }

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rug_x, rug_o) =
                rug::Float::with_val_round(u32::exact_from(prec), rug::Rational::from(&x), rm);
            assert_eq!(
                ComparableFloatRef(&float_x),
                ComparableFloatRef(&Float::from(&rug_x))
            );
            assert_eq!(rug_o, o);
        }

        assert_eq!(x == 0u32, float_x == 0u32);
        assert_eq!(
            float_x.get_prec(),
            if x == 0u32 { None } else { Some(prec) }
        );
        if x != 0u32 {
            assert!((Rational::exact_from(&float_x) - &x)
                .le_abs(&Rational::exact_from(float_x.ulp().unwrap())));
        }
    });

    rational_unsigned_pair_gen_var_3().test_properties(|(x, prec)| {
        let floor = Float::from_rational_prec_round_ref(&x, prec, Floor);
        let r_floor = Rational::exact_from(&floor.0);
        assert!(r_floor <= x);
        if r_floor != 0u32 {
            assert!(r_floor + Rational::exact_from(floor.0.ulp().unwrap()) > x);
        }
        let (floor_x_alt, o_alt) =
            Float::from_rational_prec_round_ref(&x, prec, if x >= 0 { Down } else { Up });
        assert_eq!(
            ComparableFloatRef(&floor_x_alt),
            ComparableFloatRef(&floor.0)
        );
        assert_eq!(o_alt, floor.1);

        let ceiling = Float::from_rational_prec_round_ref(&x, prec, Ceiling);
        let r_ceiling = Rational::exact_from(&ceiling.0);
        assert!(r_ceiling >= x);
        if r_ceiling != 0u32 {
            assert!(r_ceiling - Rational::exact_from(ceiling.0.ulp().unwrap()) < x);
        }
        let (ceiling_x_alt, o_alt) =
            Float::from_rational_prec_round_ref(&x, prec, if x >= 0 { Up } else { Down });
        assert_eq!(
            ComparableFloatRef(&ceiling_x_alt),
            ComparableFloatRef(&ceiling.0)
        );
        assert_eq!(o_alt, ceiling.1);

        let nearest = Float::from_rational_prec_round_ref(&x, prec, Nearest);
        let r_nearest = Rational::exact_from(&nearest.0);
        assert!(
            ComparableFloatRef(&nearest.0) == ComparableFloatRef(&floor.0) && nearest.1 == floor.1
                || ComparableFloatRef(&nearest.0) == ComparableFloatRef(&ceiling.0)
                    && nearest.1 == ceiling.1
        );
        if r_nearest != 0u32 {
            assert!((r_nearest - x).le_abs(&(Rational::exact_from(nearest.0.ulp().unwrap()) >> 1)));
        }
    });
}

#[test]
fn float_try_from_rational_properties() {
    rational_gen().test_properties(|x| {
        let of = Float::try_from(&x);
        assert!(of.as_ref().map_or(true, Float::is_valid));
        assert_eq!(
            Float::try_from(x.clone()).map(ComparableFloat),
            of.clone().map(ComparableFloat)
        );
        if let Ok(f) = of {
            assert_eq!(-x, -f);
        }
    });
}

#[test]
fn float_convertible_from_rational_properties() {
    rational_gen().test_properties(|x| {
        assert_eq!(Float::convertible_from(&x), Float::try_from(&x).is_ok());
    });

    integer_gen().test_properties(|n| {
        assert!(Float::convertible_from(&Rational::from(n)));
    });
}
