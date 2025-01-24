// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{
    float_rational_pair_gen, float_rational_pair_gen_var_1, float_rational_pair_gen_var_2,
};
use malachite_float::Float;
use malachite_q::Rational;
use rug;
use std::str::FromStr;

#[test]
fn test_partial_eq_rational() {
    let test = |s, s_hex, t, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);
        let v = Rational::from_str(t).unwrap();

        assert_eq!(u == v, out);
        assert_eq!(v == u, out);
        assert_eq!(rug::Float::exact_from(&u) == rug::Rational::from(&v), out);
    };
    test("NaN", "NaN", "0", false);
    test("Infinity", "Infinity", "0", false);
    test("-Infinity", "-Infinity", "0", false);
    test("0.0", "0x0.0", "0", true);
    test("-0.0", "-0x0.0", "0", true);
    test("1.0", "0x1.0#1", "0", false);
    test("2.0", "0x2.0#1", "0", false);
    test("0.5", "0x0.8#1", "0", false);
    test("0.33333333333333331", "0x0.55555555555554#53", "0", false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", "0", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", "0", false);
    test("3.0e120", "0x1.0E+100#1", "0", false);
    test("4.0e-121", "0x1.0E-100#1", "0", false);
    test("-1.0", "-0x1.0#1", "0", false);
    test("-2.0", "-0x2.0#1", "0", false);
    test("-0.5", "-0x0.8#1", "0", false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", "0", false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", "0", false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", "0", false);
    test("-3.0e120", "-0x1.0E+100#1", "0", false);
    test("-4.0e-121", "-0x1.0E-100#1", "0", false);

    test("NaN", "NaN", "1", false);
    test("Infinity", "Infinity", "1", false);
    test("-Infinity", "-Infinity", "1", false);
    test("0.0", "0x0.0", "1", false);
    test("-0.0", "-0x0.0", "1", false);
    test("1.0", "0x1.0#1", "1", true);
    test("2.0", "0x2.0#1", "1", false);
    test("0.5", "0x0.8#1", "1", false);
    test("0.33333333333333331", "0x0.55555555555554#53", "1", false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", "1", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", "1", false);
    test("3.0e120", "0x1.0E+100#1", "1", false);
    test("4.0e-121", "0x1.0E-100#1", "1", false);
    test("-1.0", "-0x1.0#1", "1", false);
    test("-2.0", "-0x2.0#1", "1", false);
    test("-0.5", "-0x0.8#1", "1", false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", "1", false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", "1", false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", "1", false);
    test("-3.0e120", "-0x1.0E+100#1", "1", false);
    test("-4.0e-121", "-0x1.0E-100#1", "1", false);

    test("NaN", "NaN", "100", false);
    test("Infinity", "Infinity", "100", false);
    test("-Infinity", "-Infinity", "100", false);
    test("0.0", "0x0.0", "100", false);
    test("-0.0", "-0x0.0", "100", false);
    test("1.0", "0x1.0#1", "100", false);
    test("2.0", "0x2.0#1", "100", false);
    test("0.5", "0x0.8#1", "100", false);
    test("0.33333333333333331", "0x0.55555555555554#53", "100", false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", "100", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", "100", false);
    test("3.0e120", "0x1.0E+100#1", "100", false);
    test("4.0e-121", "0x1.0E-100#1", "100", false);
    test("-1.0", "-0x1.0#1", "100", false);
    test("-2.0", "-0x2.0#1", "100", false);
    test("-0.5", "-0x0.8#1", "100", false);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "100",
        false,
    );
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", "100", false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", "100", false);
    test("-3.0e120", "-0x1.0E+100#1", "100", false);
    test("-4.0e-121", "-0x1.0E-100#1", "100", false);

    let s = "2582249878086908589655919172003011874329705792829223512830659356540647622016841194629\
    645353280137831435903171972747493376";
    test("NaN", "NaN", s, false);
    test("Infinity", "Infinity", s, false);
    test("-Infinity", "-Infinity", s, false);
    test("0.0", "0x0.0", s, false);
    test("-0.0", "-0x0.0", s, false);
    test("1.0", "0x1.0#1", s, false);
    test("2.0", "0x2.0#1", s, false);
    test("0.5", "0x0.8#1", s, false);
    test("0.33333333333333331", "0x0.55555555555554#53", s, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", s, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", s, false);
    test("3.0e120", "0x1.0E+100#1", s, true);
    test("4.0e-121", "0x1.0E-100#1", s, false);
    test("-1.0", "-0x1.0#1", s, false);
    test("-2.0", "-0x2.0#1", s, false);
    test("-0.5", "-0x0.8#1", s, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", s, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", s, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", s, false);
    test("-3.0e120", "-0x1.0E+100#1", s, false);
    test("-4.0e-121", "-0x1.0E-100#1", s, false);

    // off by 1
    let s = "2582249878086908589655919172003011874329705792829223512830659356540647622016841194629\
    645353280137831435903171972747493377";
    test("NaN", "NaN", s, false);
    test("Infinity", "Infinity", s, false);
    test("-Infinity", "-Infinity", s, false);
    test("0.0", "0x0.0", s, false);
    test("-0.0", "-0x0.0", s, false);
    test("1.0", "0x1.0#1", s, false);
    test("2.0", "0x2.0#1", s, false);
    test("0.5", "0x0.8#1", s, false);
    test("0.33333333333333331", "0x0.55555555555554#53", s, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", s, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", s, false);
    test("3.0e120", "0x1.0E+100#1", s, false);
    test("4.0e-121", "0x1.0E-100#1", s, false);
    test("-1.0", "-0x1.0#1", s, false);
    test("-2.0", "-0x2.0#1", s, false);
    test("-0.5", "-0x0.8#1", s, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", s, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", s, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", s, false);
    test("-3.0e120", "-0x1.0E+100#1", s, false);
    test("-4.0e-121", "-0x1.0E-100#1", s, false);

    test("NaN", "NaN", "1/2", false);
    test("Infinity", "Infinity", "1/2", false);
    test("-Infinity", "-Infinity", "1/2", false);
    test("0.0", "0x0.0", "1/2", false);
    test("-0.0", "-0x0.0", "1/2", false);
    test("1.0", "0x1.0#1", "1/2", false);
    test("2.0", "0x2.0#1", "1/2", false);
    test("0.5", "0x0.8#1", "1/2", true);
    test("0.33333333333333331", "0x0.55555555555554#53", "1/2", false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", "1/2", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", "1/2", false);
    test("3.0e120", "0x1.0E+100#1", "1/2", false);
    test("4.0e-121", "0x1.0E-100#1", "1/2", false);
    test("-1.0", "-0x1.0#1", "1/2", false);
    test("-2.0", "-0x2.0#1", "1/2", false);
    test("-0.5", "-0x0.8#1", "1/2", false);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "1/2",
        false,
    );
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", "1/2", false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", "1/2", false);
    test("-3.0e120", "-0x1.0E+100#1", "1/2", false);
    test("-4.0e-121", "-0x1.0E-100#1", "1/2", false);

    test("NaN", "NaN", "1/3", false);
    test("Infinity", "Infinity", "1/3", false);
    test("-Infinity", "-Infinity", "1/3", false);
    test("0.0", "0x0.0", "1/3", false);
    test("-0.0", "-0x0.0", "1/3", false);
    test("1.0", "0x1.0#1", "1/3", false);
    test("2.0", "0x2.0#1", "1/3", false);
    test("0.5", "0x0.8#1", "1/3", false);
    test("0.33333333333333331", "0x0.55555555555554#53", "1/3", false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", "1/3", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", "1/3", false);
    test("3.0e120", "0x1.0E+100#1", "1/3", false);
    test("4.0e-121", "0x1.0E-100#1", "1/3", false);
    test("-1.0", "-0x1.0#1", "1/3", false);
    test("-2.0", "-0x2.0#1", "1/3", false);
    test("-0.5", "-0x0.8#1", "1/3", false);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "1/3",
        false,
    );
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", "1/3", false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", "1/3", false);
    test("-3.0e120", "-0x1.0E+100#1", "1/3", false);
    test("-4.0e-121", "-0x1.0E-100#1", "1/3", false);

    test("NaN", "NaN", "22/7", false);
    test("Infinity", "Infinity", "22/7", false);
    test("-Infinity", "-Infinity", "22/7", false);
    test("0.0", "0x0.0", "22/7", false);
    test("-0.0", "-0x0.0", "22/7", false);
    test("1.0", "0x1.0#1", "22/7", false);
    test("2.0", "0x2.0#1", "22/7", false);
    test("0.5", "0x0.8#1", "22/7", false);
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "22/7",
        false,
    );
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", "22/7", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", "22/7", false);
    test("3.0e120", "0x1.0E+100#1", "22/7", false);
    test("4.0e-121", "0x1.0E-100#1", "22/7", false);
    test("-1.0", "-0x1.0#1", "22/7", false);
    test("-2.0", "-0x2.0#1", "22/7", false);
    test("-0.5", "-0x0.8#1", "22/7", false);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "22/7",
        false,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "22/7",
        false,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "22/7",
        false,
    );
    test("-3.0e120", "-0x1.0E+100#1", "22/7", false);
    test("-4.0e-121", "-0x1.0E-100#1", "22/7", false);

    test("NaN", "NaN", "-1", false);
    test("Infinity", "Infinity", "-1", false);
    test("-Infinity", "-Infinity", "-1", false);
    test("0.0", "0x0.0", "-1", false);
    test("-0.0", "-0x0.0", "-1", false);
    test("1.0", "0x1.0#1", "-1", false);
    test("2.0", "0x2.0#1", "-1", false);
    test("0.5", "0x0.8#1", "-1", false);
    test("0.33333333333333331", "0x0.55555555555554#53", "-1", false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", "-1", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", "-1", false);
    test("3.0e120", "0x1.0E+100#1", "-1", false);
    test("4.0e-121", "0x1.0E-100#1", "-1", false);
    test("-1.0", "-0x1.0#1", "-1", true);
    test("-2.0", "-0x2.0#1", "-1", false);
    test("-0.5", "-0x0.8#1", "-1", false);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "-1",
        false,
    );
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", "-1", false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", "-1", false);
    test("-3.0e120", "-0x1.0E+100#1", "-1", false);
    test("-4.0e-121", "-0x1.0E-100#1", "-1", false);

    test("NaN", "NaN", "-100", false);
    test("Infinity", "Infinity", "-100", false);
    test("-Infinity", "-Infinity", "-100", false);
    test("0.0", "0x0.0", "-100", false);
    test("-0.0", "-0x0.0", "-100", false);
    test("1.0", "0x1.0#1", "-100", false);
    test("2.0", "0x2.0#1", "-100", false);
    test("0.5", "0x0.8#1", "-100", false);
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "-100",
        false,
    );
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", "-100", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", "-100", false);
    test("3.0e120", "0x1.0E+100#1", "-100", false);
    test("4.0e-121", "0x1.0E-100#1", "-100", false);
    test("-1.0", "-0x1.0#1", "-100", false);
    test("-2.0", "-0x2.0#1", "-100", false);
    test("-0.5", "-0x0.8#1", "-100", false);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "-100",
        false,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-100",
        false,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-100",
        false,
    );
    test("-3.0e120", "-0x1.0E+100#1", "-100", false);
    test("-4.0e-121", "-0x1.0E-100#1", "-100", false);

    let s = "-258224987808690858965591917200301187432970579282922351283065935654064762201684119462\
    9645353280137831435903171972747493376";
    test("NaN", "NaN", s, false);
    test("Infinity", "Infinity", s, false);
    test("-Infinity", "-Infinity", s, false);
    test("0.0", "0x0.0", s, false);
    test("-0.0", "-0x0.0", s, false);
    test("1.0", "0x1.0#1", s, false);
    test("2.0", "0x2.0#1", s, false);
    test("0.5", "0x0.8#1", s, false);
    test("0.33333333333333331", "0x0.55555555555554#53", s, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", s, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", s, false);
    test("3.0e120", "0x1.0E+100#1", s, false);
    test("4.0e-121", "0x1.0E-100#1", s, false);
    test("-1.0", "-0x1.0#1", s, false);
    test("-2.0", "-0x2.0#1", s, false);
    test("-0.5", "-0x0.8#1", s, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", s, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", s, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", s, false);
    test("-3.0e120", "-0x1.0E+100#1", s, true);
    test("-4.0e-121", "-0x1.0E-100#1", s, false);

    // off by 1
    let s = "-258224987808690858965591917200301187432970579282922351283065935654064762201684119462\
    9645353280137831435903171972747493377";
    test("NaN", "NaN", s, false);
    test("Infinity", "Infinity", s, false);
    test("-Infinity", "-Infinity", s, false);
    test("0.0", "0x0.0", s, false);
    test("-0.0", "-0x0.0", s, false);
    test("1.0", "0x1.0#1", s, false);
    test("2.0", "0x2.0#1", s, false);
    test("0.5", "0x0.8#1", s, false);
    test("0.33333333333333331", "0x0.55555555555554#53", s, false);
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", s, false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", s, false);
    test("3.0e120", "0x1.0E+100#1", s, false);
    test("4.0e-121", "0x1.0E-100#1", s, false);
    test("-1.0", "-0x1.0#1", s, false);
    test("-2.0", "-0x2.0#1", s, false);
    test("-0.5", "-0x0.8#1", s, false);
    test("-0.33333333333333331", "-0x0.55555555555554#53", s, false);
    test("-1.4142135623730951", "-0x1.6a09e667f3bcd#53", s, false);
    test("-3.1415926535897931", "-0x3.243f6a8885a30#53", s, false);
    test("-3.0e120", "-0x1.0E+100#1", s, false);
    test("-4.0e-121", "-0x1.0E-100#1", s, false);

    test("NaN", "NaN", "-1/2", false);
    test("Infinity", "Infinity", "-1/2", false);
    test("-Infinity", "-Infinity", "-1/2", false);
    test("0.0", "0x0.0", "-1/2", false);
    test("-0.0", "-0x0.0", "-1/2", false);
    test("1.0", "0x1.0#1", "-1/2", false);
    test("2.0", "0x2.0#1", "-1/2", false);
    test("0.5", "0x0.8#1", "-1/2", false);
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "-1/2",
        false,
    );
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", "-1/2", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", "-1/2", false);
    test("3.0e120", "0x1.0E+100#1", "-1/2", false);
    test("4.0e-121", "0x1.0E-100#1", "-1/2", false);
    test("-1.0", "-0x1.0#1", "-1/2", false);
    test("-2.0", "-0x2.0#1", "-1/2", false);
    test("-0.5", "-0x0.8#1", "-1/2", true);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "-1/2",
        false,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-1/2",
        false,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/2",
        false,
    );
    test("-3.0e120", "-0x1.0E+100#1", "-1/2", false);
    test("-4.0e-121", "-0x1.0E-100#1", "-1/2", false);

    test("NaN", "NaN", "-1/3", false);
    test("Infinity", "Infinity", "-1/3", false);
    test("-Infinity", "-Infinity", "-1/3", false);
    test("0.0", "0x0.0", "-1/3", false);
    test("-0.0", "-0x0.0", "-1/3", false);
    test("1.0", "0x1.0#1", "-1/3", false);
    test("2.0", "0x2.0#1", "-1/3", false);
    test("0.5", "0x0.8#1", "-1/3", false);
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "-1/3",
        false,
    );
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", "-1/3", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", "-1/3", false);
    test("3.0e120", "0x1.0E+100#1", "-1/3", false);
    test("4.0e-121", "0x1.0E-100#1", "-1/3", false);
    test("-1.0", "-0x1.0#1", "-1/3", false);
    test("-2.0", "-0x2.0#1", "-1/3", false);
    test("-0.5", "-0x0.8#1", "-1/3", false);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "-1/3",
        false,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-1/3",
        false,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-1/3",
        false,
    );
    test("-3.0e120", "-0x1.0E+100#1", "-1/3", false);
    test("-4.0e-121", "-0x1.0E-100#1", "-1/3", false);

    test("NaN", "NaN", "-22/7", false);
    test("Infinity", "Infinity", "-22/7", false);
    test("-Infinity", "-Infinity", "-22/7", false);
    test("0.0", "0x0.0", "-22/7", false);
    test("-0.0", "-0x0.0", "-22/7", false);
    test("1.0", "0x1.0#1", "-22/7", false);
    test("2.0", "0x2.0#1", "-22/7", false);
    test("0.5", "0x0.8#1", "-22/7", false);
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "-22/7",
        false,
    );
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", "-22/7", false);
    test("3.1415926535897931", "0x3.243f6a8885a30#53", "-22/7", false);
    test("3.0e120", "0x1.0E+100#1", "-22/7", false);
    test("4.0e-121", "0x1.0E-100#1", "-22/7", false);
    test("-1.0", "-0x1.0#1", "-22/7", false);
    test("-2.0", "-0x2.0#1", "-22/7", false);
    test("-0.5", "-0x0.8#1", "-22/7", false);
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "-22/7",
        false,
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "-22/7",
        false,
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "-22/7",
        false,
    );
    test("-3.0e120", "-0x1.0E+100#1", "-22/7", false);
    test("-4.0e-121", "-0x1.0E-100#1", "-22/7", false);
}

#[allow(clippy::needless_pass_by_value)]
fn partial_eq_rational_properties_helper(x: Float, y: Rational) {
    let eq = x == y;
    assert_eq!(y == x, eq);
    assert_eq!(rug::Float::exact_from(&x) == rug::Rational::from(&y), eq);
}

#[test]
fn partial_eq_rational_properties() {
    float_rational_pair_gen().test_properties(|(x, y)| {
        partial_eq_rational_properties_helper(x, y);
    });

    float_rational_pair_gen_var_2().test_properties(|(x, y)| {
        partial_eq_rational_properties_helper(x, y);
    });

    float_rational_pair_gen_var_1().test_properties(|(x, y)| {
        assert_eq!(Rational::exact_from(&x) == y, x == y);
    });
}
