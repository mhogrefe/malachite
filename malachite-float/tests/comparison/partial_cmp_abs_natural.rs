// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Abs;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::test_util::common::parse_hex_string;
use malachite_float::test_util::generators::{
    float_float_natural_triple_gen, float_natural_natural_triple_gen, float_natural_pair_gen,
    float_natural_pair_gen_var_1,
};
use malachite_float::Float;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_pair_gen;
use malachite_q::Rational;
use rug;
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

#[test]
fn test_partial_cmp_abs_natural() {
    let test = |s, s_hex, t, out| {
        let u = parse_hex_string(s_hex);
        assert_eq!(u.to_string(), s);
        let v = Natural::from_str(t).unwrap();

        assert_eq!(u.partial_cmp_abs(&v), out);
        assert_eq!(v.partial_cmp_abs(&u), out.map(Ordering::reverse));
        assert_eq!(
            rug::Float::exact_from(&u)
                .abs()
                .partial_cmp(&rug::Integer::from(&v)),
            out
        );
    };
    test("NaN", "NaN", "0", None);
    test("Infinity", "Infinity", "0", Some(Greater));
    test("-Infinity", "-Infinity", "0", Some(Greater));
    test("0.0", "0x0.0", "0", Some(Equal));
    test("-0.0", "-0x0.0", "0", Some(Equal));
    test("1.0", "0x1.0#1", "0", Some(Greater));
    test("2.0", "0x2.0#1", "0", Some(Greater));
    test("0.5", "0x0.8#1", "0", Some(Greater));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "0",
        Some(Greater),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "0",
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "0",
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", "0", Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", "0", Some(Greater));
    test("-1.0", "-0x1.0#1", "0", Some(Greater));
    test("-2.0", "-0x2.0#1", "0", Some(Greater));
    test("-0.5", "-0x0.8#1", "0", Some(Greater));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "0",
        Some(Greater),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "0",
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "0",
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", "0", Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", "0", Some(Greater));

    test("NaN", "NaN", "1", None);
    test("Infinity", "Infinity", "1", Some(Greater));
    test("-Infinity", "-Infinity", "1", Some(Greater));
    test("0.0", "0x0.0", "1", Some(Less));
    test("-0.0", "-0x0.0", "1", Some(Less));
    test("1.0", "0x1.0#1", "1", Some(Equal));
    test("2.0", "0x2.0#1", "1", Some(Greater));
    test("0.5", "0x0.8#1", "1", Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "1",
        Some(Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "1",
        Some(Greater),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "1",
        Some(Greater),
    );
    test("3.0e120", "0x1.0E+100#1", "1", Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", "1", Some(Less));
    test("-1.0", "-0x1.0#1", "1", Some(Equal));
    test("-2.0", "-0x2.0#1", "1", Some(Greater));
    test("-0.5", "-0x0.8#1", "1", Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "1",
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "1",
        Some(Greater),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "1",
        Some(Greater),
    );
    test("-3.0e120", "-0x1.0E+100#1", "1", Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", "1", Some(Less));

    test("NaN", "NaN", "100", None);
    test("Infinity", "Infinity", "100", Some(Greater));
    test("-Infinity", "-Infinity", "100", Some(Greater));
    test("0.0", "0x0.0", "100", Some(Less));
    test("-0.0", "-0x0.0", "100", Some(Less));
    test("1.0", "0x1.0#1", "100", Some(Less));
    test("2.0", "0x2.0#1", "100", Some(Less));
    test("0.5", "0x0.8#1", "100", Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        "100",
        Some(Less),
    );
    test(
        "1.4142135623730951",
        "0x1.6a09e667f3bcd#53",
        "100",
        Some(Less),
    );
    test(
        "3.1415926535897931",
        "0x3.243f6a8885a30#53",
        "100",
        Some(Less),
    );
    test("3.0e120", "0x1.0E+100#1", "100", Some(Greater));
    test("4.0e-121", "0x1.0E-100#1", "100", Some(Less));
    test("-1.0", "-0x1.0#1", "100", Some(Less));
    test("-2.0", "-0x2.0#1", "100", Some(Less));
    test("-0.5", "-0x0.8#1", "100", Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        "100",
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        "100",
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        "100",
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", "100", Some(Greater));
    test("-4.0e-121", "-0x1.0E-100#1", "100", Some(Less));

    let s = "2582249878086908589655919172003011874329705792829223512830659356540647622016841194629\
    645353280137831435903171972747493376";
    test("NaN", "NaN", s, None);
    test("Infinity", "Infinity", s, Some(Greater));
    test("-Infinity", "-Infinity", s, Some(Greater));
    test("0.0", "0x0.0", s, Some(Less));
    test("-0.0", "-0x0.0", s, Some(Less));
    test("1.0", "0x1.0#1", s, Some(Less));
    test("2.0", "0x2.0#1", s, Some(Less));
    test("0.5", "0x0.8#1", s, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        s,
        Some(Less),
    );
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", s, Some(Less));
    test("3.1415926535897931", "0x3.243f6a8885a30#53", s, Some(Less));
    test("3.0e120", "0x1.0E+100#1", s, Some(Equal));
    test("4.0e-121", "0x1.0E-100#1", s, Some(Less));
    test("-1.0", "-0x1.0#1", s, Some(Less));
    test("-2.0", "-0x2.0#1", s, Some(Less));
    test("-0.5", "-0x0.8#1", s, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        s,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        s,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        s,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", s, Some(Equal));
    test("-4.0e-121", "-0x1.0E-100#1", s, Some(Less));

    // off by 1
    let s = "2582249878086908589655919172003011874329705792829223512830659356540647622016841194629\
    645353280137831435903171972747493377";
    test("NaN", "NaN", s, None);
    test("Infinity", "Infinity", s, Some(Greater));
    test("-Infinity", "-Infinity", s, Some(Greater));
    test("0.0", "0x0.0", s, Some(Less));
    test("-0.0", "-0x0.0", s, Some(Less));
    test("1.0", "0x1.0#1", s, Some(Less));
    test("2.0", "0x2.0#1", s, Some(Less));
    test("0.5", "0x0.8#1", s, Some(Less));
    test(
        "0.33333333333333331",
        "0x0.55555555555554#53",
        s,
        Some(Less),
    );
    test("1.4142135623730951", "0x1.6a09e667f3bcd#53", s, Some(Less));
    test("3.1415926535897931", "0x3.243f6a8885a30#53", s, Some(Less));
    test("3.0e120", "0x1.0E+100#1", s, Some(Less));
    test("4.0e-121", "0x1.0E-100#1", s, Some(Less));
    test("-1.0", "-0x1.0#1", s, Some(Less));
    test("-2.0", "-0x2.0#1", s, Some(Less));
    test("-0.5", "-0x0.8#1", s, Some(Less));
    test(
        "-0.33333333333333331",
        "-0x0.55555555555554#53",
        s,
        Some(Less),
    );
    test(
        "-1.4142135623730951",
        "-0x1.6a09e667f3bcd#53",
        s,
        Some(Less),
    );
    test(
        "-3.1415926535897931",
        "-0x3.243f6a8885a30#53",
        s,
        Some(Less),
    );
    test("-3.0e120", "-0x1.0E+100#1", s, Some(Less));
    test("-4.0e-121", "-0x1.0E-100#1", s, Some(Less));
}

#[test]
fn partial_cmp_abs_natural_properties() {
    float_natural_pair_gen().test_properties(|(x, y)| {
        let cmp = x.partial_cmp_abs(&y);
        assert_eq!(x.partial_cmp_abs(&Float::from(&y)), cmp);
        assert_eq!((&x).abs().partial_cmp(&y), cmp);
        assert_eq!(
            rug::Float::exact_from(&x)
                .abs()
                .partial_cmp(&rug::Integer::from(&y)),
            cmp
        );
        assert_eq!(y.partial_cmp_abs(&x), cmp.map(Ordering::reverse));
    });

    float_float_natural_triple_gen().test_properties(|(x, z, y)| {
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x.lt_abs(&z));
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x.gt_abs(&z));
        }
    });

    float_natural_natural_triple_gen().test_properties(|(y, x, z)| {
        if x.lt_abs(&y) && y.lt_abs(&z) {
            assert!(x < z);
        } else if x.gt_abs(&y) && y.gt_abs(&z) {
            assert!(x > z);
        }
    });

    natural_pair_gen().test_properties(|(x, y)| {
        assert_eq!(Float::from(&x).partial_cmp_abs(&y), x.partial_cmp(&y));
        assert_eq!(x.partial_cmp_abs(&Float::from(&y)), x.partial_cmp(&y));
    });

    float_natural_pair_gen_var_1().test_properties(|(x, y)| {
        assert_eq!(
            Rational::exact_from(&x).partial_cmp_abs(&y),
            x.partial_cmp_abs(&y)
        );
    });
}
