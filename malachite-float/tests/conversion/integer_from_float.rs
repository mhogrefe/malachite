// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Ceiling, Floor, Parity};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, One, OneHalf};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::{ConvertibleFrom, ExactFrom, RoundingFrom};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::strings::ToDebugString;
use malachite_float::test_util::common::{parse_hex_string, rug_round_try_from_rounding_mode};
use malachite_float::test_util::generators::{
    float_gen, float_gen_var_4, float_rounding_mode_pair_gen_var_2,
};
use malachite_float::Float;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_gen;
use malachite_q::Rational;
use std::cmp::Ordering::*;
use std::panic::catch_unwind;

#[test]
fn test_try_from_float() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let on = Integer::try_from(x.clone());
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));

        let on = Integer::try_from(&x);
        assert_eq!(on.to_debug_string(), out);
        assert!(on.map_or(true, |n| n.is_valid()));
    };
    test("NaN", "NaN", "Err(FloatInfiniteOrNan)");
    test("Infinity", "Infinity", "Err(FloatInfiniteOrNan)");
    test("-Infinity", "-Infinity", "Err(FloatInfiniteOrNan)");
    test("0.0", "0x0.0", "Ok(0)");
    test("-0.0", "-0x0.0", "Ok(0)");

    test("1.0", "0x1.0#1", "Ok(1)");
    test("2.0", "0x2.0#1", "Ok(2)");
    test("0.5", "0x0.8#1", "Err(FloatNonIntegerOrOutOfRange)");
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "Err(FloatNonIntegerOrOutOfRange)",
    );
    test("123.0", "0x7b.0#7", "Ok(123)");
    test("1000000000000.0", "0xe8d4a51000.0#40", "Ok(1000000000000)");

    test("-1.0", "-0x1.0#1", "Ok(-1)");
    test("-2.0", "-0x2.0#1", "Ok(-2)");
    test("-0.5", "-0x0.8#1", "Err(FloatNonIntegerOrOutOfRange)");
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "Err(FloatNonIntegerOrOutOfRange)",
    );
    test("-123.0", "-0x7b.0#7", "Ok(-123)");
    test(
        "-1000000000000.0",
        "-0xe8d4a51000.0#40",
        "Ok(-1000000000000)",
    );
}

#[test]
fn test_convertible_from_float() {
    let test = |s, s_hex, out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        assert_eq!(Integer::convertible_from(&x), out);
    };
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
    test("0.0", "0x0.0", true);
    test("-0.0", "-0x0.0", true);

    test("1.0", "0x1.0#1", true);
    test("2.0", "0x2.0#1", true);
    test("0.5", "0x0.8#1", false);
    test("0.33333333333333331", "0x0.55555555555554#53", false);
    test("123.0", "0x7b.0#7", true);
    test("1000000000000.0", "0xe8d4a51000.0#40", true);

    test("-1.0", "-0x1.0#1", true);
    test("-2.0", "-0x2.0#1", true);
    test("-0.5", "-0x0.8#1", false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", false);
    test("-123.0", "-0x7b.0#7", true);
    test("-1000000000000.0", "-0xe8d4a51000.0#40", true);
}

#[test]
fn test_rounding_from_float() {
    let test = |s, s_hex, rm, out, o_out| {
        let x = parse_hex_string(s_hex);
        assert_eq!(x.to_string(), s);

        let (n, o) = Integer::rounding_from(x.clone(), rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o, o_out);

        let (n, o) = Integer::rounding_from(&x, rm);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert_eq!(o, o_out);

        if let Ok(rm) = rug_round_try_from_rounding_mode(rm) {
            let (rn, ro) = rug::Float::exact_from(&x).to_integer_round(rm).unwrap();
            assert_eq!(Integer::from(&rn), n);
            assert_eq!(ro, o);
        }
    };
    test("0.0", "0x0.0", Floor, "0", Equal);
    test("0.0", "0x0.0", Ceiling, "0", Equal);
    test("0.0", "0x0.0", Down, "0", Equal);
    test("0.0", "0x0.0", Up, "0", Equal);
    test("0.0", "0x0.0", Nearest, "0", Equal);
    test("0.0", "0x0.0", Exact, "0", Equal);

    test("-0.0", "-0x0.0", Floor, "0", Equal);
    test("-0.0", "-0x0.0", Ceiling, "0", Equal);
    test("-0.0", "-0x0.0", Down, "0", Equal);
    test("-0.0", "-0x0.0", Up, "0", Equal);
    test("-0.0", "-0x0.0", Nearest, "0", Equal);
    test("-0.0", "-0x0.0", Exact, "0", Equal);

    test("1.0", "0x1.0#1", Floor, "1", Equal);
    test("1.0", "0x1.0#1", Ceiling, "1", Equal);
    test("1.0", "0x1.0#1", Down, "1", Equal);
    test("1.0", "0x1.0#1", Up, "1", Equal);
    test("1.0", "0x1.0#1", Nearest, "1", Equal);
    test("1.0", "0x1.0#1", Exact, "1", Equal);

    test("2.0", "0x2.0#1", Floor, "2", Equal);
    test("2.0", "0x2.0#1", Ceiling, "2", Equal);
    test("2.0", "0x2.0#1", Down, "2", Equal);
    test("2.0", "0x2.0#1", Up, "2", Equal);
    test("2.0", "0x2.0#1", Nearest, "2", Equal);
    test("2.0", "0x2.0#1", Exact, "2", Equal);

    test("0.5", "0x0.8#1", Floor, "0", Less);
    test("0.5", "0x0.8#1", Ceiling, "1", Greater);
    test("0.5", "0x0.8#1", Down, "0", Less);
    test("0.5", "0x0.8#1", Up, "1", Greater);
    test("0.5", "0x0.8#1", Nearest, "0", Less);

    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Floor,
        "0",
        Less,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Ceiling,
        "1",
        Greater,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Down,
        "0",
        Less,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Up,
        "1",
        Greater,
    );
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        Nearest,
        "0",
        Less,
    );

    test(
        "0.6666666666666666",
        "0x0.aaaaaaaaaaaaa8#53",
        Floor,
        "0",
        Less,
    );
    test(
        "0.6666666666666666",
        "0x0.aaaaaaaaaaaaa8#53",
        Ceiling,
        "1",
        Greater,
    );
    test(
        "0.6666666666666666",
        "0x0.aaaaaaaaaaaaa8#53",
        Down,
        "0",
        Less,
    );
    test(
        "0.6666666666666666",
        "0x0.aaaaaaaaaaaaa8#53",
        Up,
        "1",
        Greater,
    );
    test(
        "0.6666666666666666",
        "0x0.aaaaaaaaaaaaa8#53",
        Nearest,
        "1",
        Greater,
    );

    test("1.5", "0x1.8#2", Floor, "1", Less);
    test("1.5", "0x1.8#2", Ceiling, "2", Greater);
    test("1.5", "0x1.8#2", Down, "1", Less);
    test("1.5", "0x1.8#2", Up, "2", Greater);
    test("1.5", "0x1.8#2", Nearest, "2", Greater);

    test("2.5", "0x2.8#3", Floor, "2", Less);
    test("2.5", "0x2.8#3", Ceiling, "3", Greater);
    test("2.5", "0x2.8#3", Down, "2", Less);
    test("2.5", "0x2.8#3", Up, "3", Greater);
    test("2.5", "0x2.8#3", Nearest, "2", Less);

    test("123.0", "0x7b.0#7", Floor, "123", Equal);
    test("123.0", "0x7b.0#7", Ceiling, "123", Equal);
    test("123.0", "0x7b.0#7", Down, "123", Equal);
    test("123.0", "0x7b.0#7", Up, "123", Equal);
    test("123.0", "0x7b.0#7", Nearest, "123", Equal);
    test("123.0", "0x7b.0#7", Exact, "123", Equal);

    test("-1.0", "-0x1.0#1", Floor, "-1", Equal);
    test("-1.0", "-0x1.0#1", Ceiling, "-1", Equal);
    test("-1.0", "-0x1.0#1", Down, "-1", Equal);
    test("-1.0", "-0x1.0#1", Up, "-1", Equal);
    test("-1.0", "-0x1.0#1", Nearest, "-1", Equal);
    test("-1.0", "-0x1.0#1", Exact, "-1", Equal);

    test("-2.0", "-0x2.0#1", Floor, "-2", Equal);
    test("-2.0", "-0x2.0#1", Ceiling, "-2", Equal);
    test("-2.0", "-0x2.0#1", Down, "-2", Equal);
    test("-2.0", "-0x2.0#1", Up, "-2", Equal);
    test("-2.0", "-0x2.0#1", Nearest, "-2", Equal);
    test("-2.0", "-0x2.0#1", Exact, "-2", Equal);

    test("-0.5", "-0x0.8#1", Floor, "-1", Less);
    test("-0.5", "-0x0.8#1", Ceiling, "0", Greater);
    test("-0.5", "-0x0.8#1", Down, "0", Greater);
    test("-0.5", "-0x0.8#1", Up, "-1", Less);
    test("-0.5", "-0x0.8#1", Nearest, "0", Greater);

    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Floor,
        "-1",
        Less,
    );
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Ceiling,
        "0",
        Greater,
    );
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Down,
        "0",
        Greater,
    );
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Up,
        "-1",
        Less,
    );
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        Nearest,
        "0",
        Greater,
    );

    test(
        "-0.6666666666666666",
        "-0x0.aaaaaaaaaaaaa8#53",
        Floor,
        "-1",
        Less,
    );
    test(
        "-0.6666666666666666",
        "-0x0.aaaaaaaaaaaaa8#53",
        Ceiling,
        "0",
        Greater,
    );
    test(
        "-0.6666666666666666",
        "-0x0.aaaaaaaaaaaaa8#53",
        Down,
        "0",
        Greater,
    );
    test(
        "-0.6666666666666666",
        "-0x0.aaaaaaaaaaaaa8#53",
        Up,
        "-1",
        Less,
    );
    test(
        "-0.6666666666666666",
        "-0x0.aaaaaaaaaaaaa8#53",
        Nearest,
        "-1",
        Less,
    );

    test("-1.5", "-0x1.8#2", Floor, "-2", Less);
    test("-1.5", "-0x1.8#2", Ceiling, "-1", Greater);
    test("-1.5", "-0x1.8#2", Down, "-1", Greater);
    test("-1.5", "-0x1.8#2", Up, "-2", Less);
    test("-1.5", "-0x1.8#2", Nearest, "-2", Less);

    test("-2.5", "-0x2.8#3", Floor, "-3", Less);
    test("-2.5", "-0x2.8#3", Ceiling, "-2", Greater);
    test("-2.5", "-0x2.8#3", Down, "-2", Greater);
    test("-2.5", "-0x2.8#3", Up, "-3", Less);
    test("-2.5", "-0x2.8#3", Nearest, "-2", Greater);

    test("-123.0", "-0x7b.0#7", Floor, "-123", Equal);
    test("-123.0", "-0x7b.0#7", Ceiling, "-123", Equal);
    test("-123.0", "-0x7b.0#7", Down, "-123", Equal);
    test("-123.0", "-0x7b.0#7", Up, "-123", Equal);
    test("-123.0", "-0x7b.0#7", Nearest, "-123", Equal);
    test("-123.0", "-0x7b.0#7", Exact, "-123", Equal);
}

#[test]
fn integer_rounding_from_float_fail() {
    assert_panic!(Integer::rounding_from(Float::NAN, Floor));
    assert_panic!(Integer::rounding_from(Float::NAN, Ceiling));
    assert_panic!(Integer::rounding_from(Float::NAN, Down));
    assert_panic!(Integer::rounding_from(Float::NAN, Up));
    assert_panic!(Integer::rounding_from(Float::NAN, Nearest));
    assert_panic!(Integer::rounding_from(Float::NAN, Exact));

    assert_panic!(Integer::rounding_from(Float::INFINITY, Floor));
    assert_panic!(Integer::rounding_from(Float::INFINITY, Ceiling));
    assert_panic!(Integer::rounding_from(Float::INFINITY, Down));
    assert_panic!(Integer::rounding_from(Float::INFINITY, Up));
    assert_panic!(Integer::rounding_from(Float::INFINITY, Nearest));
    assert_panic!(Integer::rounding_from(Float::INFINITY, Exact));

    assert_panic!(Integer::rounding_from(Float::NEGATIVE_INFINITY, Floor));
    assert_panic!(Integer::rounding_from(Float::NEGATIVE_INFINITY, Ceiling));
    assert_panic!(Integer::rounding_from(Float::NEGATIVE_INFINITY, Down));
    assert_panic!(Integer::rounding_from(Float::NEGATIVE_INFINITY, Up));
    assert_panic!(Integer::rounding_from(Float::NEGATIVE_INFINITY, Nearest));
    assert_panic!(Integer::rounding_from(Float::NEGATIVE_INFINITY, Exact));

    assert_panic!(Integer::rounding_from(Float::from(3u8) >> 1, Exact));
    assert_panic!(Integer::rounding_from(Float::from(-3i8) >> 1, Exact));
}

#[test]
fn integer_rounding_from_float_ref_fail() {
    assert_panic!(Integer::rounding_from(&Float::NAN, Floor));
    assert_panic!(Integer::rounding_from(&Float::NAN, Ceiling));
    assert_panic!(Integer::rounding_from(&Float::NAN, Down));
    assert_panic!(Integer::rounding_from(&Float::NAN, Up));
    assert_panic!(Integer::rounding_from(&Float::NAN, Nearest));
    assert_panic!(Integer::rounding_from(&Float::NAN, Exact));

    assert_panic!(Integer::rounding_from(&Float::INFINITY, Floor));
    assert_panic!(Integer::rounding_from(&Float::INFINITY, Ceiling));
    assert_panic!(Integer::rounding_from(&Float::INFINITY, Down));
    assert_panic!(Integer::rounding_from(&Float::INFINITY, Up));
    assert_panic!(Integer::rounding_from(&Float::INFINITY, Nearest));
    assert_panic!(Integer::rounding_from(&Float::INFINITY, Exact));

    assert_panic!(Integer::rounding_from(&Float::NEGATIVE_INFINITY, Floor));
    assert_panic!(Integer::rounding_from(&Float::NEGATIVE_INFINITY, Ceiling));
    assert_panic!(Integer::rounding_from(&Float::NEGATIVE_INFINITY, Down));
    assert_panic!(Integer::rounding_from(&Float::NEGATIVE_INFINITY, Up));
    assert_panic!(Integer::rounding_from(&Float::NEGATIVE_INFINITY, Nearest));
    assert_panic!(Integer::rounding_from(&Float::NEGATIVE_INFINITY, Exact));

    assert_panic!(Integer::rounding_from(&(Float::from(3u8) >> 1), Exact));
    assert_panic!(Integer::rounding_from(&(Float::from(-3i8) >> 1), Exact));
}

#[test]
fn try_from_float_properties() {
    float_gen().test_properties(|x| {
        let integer_x = Integer::try_from(x.clone());
        assert!(integer_x.as_ref().map_or(true, Integer::is_valid));

        let integer_x_alt = Integer::try_from(&x);
        assert!(integer_x_alt.as_ref().map_or(true, Integer::is_valid));
        assert_eq!(integer_x, integer_x_alt);

        assert_eq!(integer_x.is_ok(), Integer::convertible_from(&x));
        if let Ok(n) = integer_x {
            assert_eq!(Integer::exact_from(&x), n);
            assert_eq!(n, x);
            assert_eq!(Float::exact_from(&n), x);
            assert_eq!(Float::exact_from(n), x);
        }
    });
}

#[test]
fn convertible_from_float_properties() {
    float_gen().test_properties(|x| {
        Integer::convertible_from(&x);
    });
}

#[test]
fn rounding_from_float_properties() {
    float_rounding_mode_pair_gen_var_2().test_properties(|(x, rm)| {
        let no = Integer::rounding_from(&x, rm);
        assert_eq!(Integer::rounding_from(x.clone(), rm), no);
        let (n, o) = no;
        assert!((Rational::from(&n) - Rational::exact_from(&x)).lt_abs(&1));

        assert_eq!(n.partial_cmp(&x), Some(o));
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
            let (rn, ro) = rug::Float::exact_from(&x).to_integer_round(rm).unwrap();
            assert_eq!(Integer::from(&rn), n);
            assert_eq!(ro, o);
        }
    });

    float_gen_var_4().test_properties(|x| {
        let floor = Integer::rounding_from(&x, Floor);
        assert_eq!(floor.0, Rational::exact_from(&x).floor());
        assert!(floor.0 <= x);
        assert!(&floor.0 + Integer::ONE > x);
        assert_eq!(
            Integer::rounding_from(&x, if x >= 0 { Down } else { Up }),
            floor
        );

        let ceiling = Integer::rounding_from(&x, Ceiling);
        assert_eq!(ceiling.0, Rational::exact_from(&x).ceiling());
        assert!(ceiling.0 >= x);
        assert!(&ceiling.0 - Integer::ONE < x);
        assert_eq!(
            Integer::rounding_from(&x, if x >= 0 { Up } else { Down }),
            ceiling
        );

        let nearest = Integer::rounding_from(&x, Nearest);
        assert!(nearest == floor || nearest == ceiling);
        assert!((Rational::from(nearest.0) - Rational::exact_from(x)).le_abs(&Rational::ONE_HALF));
    });

    integer_gen().test_properties(|n| {
        let x = Float::exact_from(&n);
        let no = (n, Equal);
        assert_eq!(Integer::rounding_from(&x, Floor), no);
        assert_eq!(Integer::rounding_from(&x, Down), no);
        assert_eq!(Integer::rounding_from(&x, Ceiling), no);
        assert_eq!(Integer::rounding_from(&x, Up), no);
        assert_eq!(Integer::rounding_from(&x, Nearest), no);
        assert_eq!(Integer::rounding_from(&x, Exact), no);

        let x = Float::exact_from((no.0 << 1) | Integer::ONE) >> 1;
        assert!(Integer::rounding_from(x, Nearest).0.even());
    });
}
