// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::string::options::FromSciStringOptions;
use malachite_base::num::conversion::traits::{FromSciString, ToSci};
use malachite_base::rounding_modes::RoundingMode::{self, *};
use malachite_float::big_exponent_float::ComparableBigExponentFloatRef;
use malachite_float::test_util::big_exponent_float::big_exponent_float_to_float;
use malachite_float::test_util::generators::big_exponent_float_to_sci_options_pair_gen_var_1;
use malachite_float::{BigExponentFloat, Float};
use std::cmp::Ordering::{self, *};

fn same(x: &BigExponentFloat, y: &BigExponentFloat) -> bool {
    ComparableBigExponentFloatRef(x) == ComparableBigExponentFloatRef(y)
}

// The options for reading a string in `base` with `rm`.
fn from_options(base: u8, rm: RoundingMode) -> FromSciStringOptions {
    let mut o = FromSciStringOptions::default();
    o.set_base(base);
    o.set_rounding_mode(rm);
    o
}

#[test]
fn test_from_sci_string_with_options_prec() {
    // Rows whose values are outside `Float`'s exponent range were checked independently with exact
    // decimal arithmetic; the others agree with `Float`.
    let test =
        |s: &str, base: u8, prec: u64, rm: RoundingMode, out: &str, out_hex: &str, o: Ordering| {
            let r = BigExponentFloat::from_sci_string_with_options_prec(
                s,
                from_options(base, rm),
                prec,
            );
            let (x, o_alt) = r.clone().unwrap();
            assert!(x.is_valid());
            assert_eq!(x.to_string(), out, "{s} {base} {prec} {rm}");
            assert_eq!(format!("{x:?}"), out_hex, "{s} {base} {prec} {rm}");
            assert_eq!(o_alt, o, "{s} {base} {prec} {rm}");
            if base == 10 {
                assert_eq!(
                    format!("{r:?}"),
                    format!(
                        "{:?}",
                        BigExponentFloat::from_sci_string_prec_round(s, prec, rm)
                    )
                );
                if rm == Nearest {
                    assert_eq!(
                        format!("{r:?}"),
                        format!("{:?}", BigExponentFloat::from_sci_string_prec(s, prec))
                    );
                }
            }
            if let Some((x, o)) = r
                && let Some((f, fo)) =
                    Float::from_sci_string_with_options_prec(s, from_options(base, rm), prec)
                && f.is_finite()
                && f != 0u32
                && big_exponent_float_to_float(&x).is_some()
            {
                assert!(same(&x, &BigExponentFloat::from(f)));
                assert_eq!(o, fo);
            }
        };
    test("1.5", 10, 10, Nearest, "1.5", "0x1.8", Equal);
    test("1.5", 10, 4, Floor, "1.5", "0x1.8", Equal);
    test("1.5", 10, 4, Ceiling, "1.5", "0x1.8", Equal);
    test("1.5", 10, 64, Nearest, "1.5", "0x1.8", Equal);
    test("1.5", 10, 20, Down, "1.5", "0x1.8", Equal);
    test("1.5", 10, 20, Up, "1.5", "0x1.8", Equal);
    test("0.1", 10, 10, Nearest, "0.099976", "0x0.1998", Less);
    test("0.1", 10, 4, Floor, "0.094", "0x0.18", Less);
    test("0.1", 10, 4, Ceiling, "0.102", "0x0.1a", Greater);
    test(
        "0.1",
        10,
        64,
        Nearest,
        "0.100000000000000000001",
        "0x0.1999999999999999a",
        Greater,
    );
    test("0.1", 10, 20, Down, "0.09999990", "0x0.199998", Less);
    test("0.1", 10, 20, Up, "0.10000002", "0x0.19999a", Greater);
    test("-2.5e-7", 10, 10, Nearest, "-2.5006e-7", "-0x4.32E-6", Less);
    test("-2.5e-7", 10, 4, Floor, "-2.68e-7", "-0x4.8E-6", Less);
    test("-2.5e-7", 10, 4, Ceiling, "-2.4e-7", "-0x4.0E-6", Greater);
    test(
        "-2.5e-7",
        10,
        64,
        Nearest,
        "-2.5000000000000000001e-7",
        "-0x4.31bde82d7b634dbE-6",
        Less,
    );
    test(
        "-2.5e-7",
        10,
        20,
        Down,
        "-2.4999963e-7",
        "-0x4.31bd8E-6",
        Greater,
    );
    test("-2.5e-7", 10, 20, Up, "-2.500001e-7", "-0x4.31beE-6", Less);
    test(
        "123456789",
        10,
        10,
        Nearest,
        "1.235e8",
        "0x7.5cE+6",
        Greater,
    );
    test("123456789", 10, 4, Floor, "1.2e8", "0x7.0E+6", Less);
    test("123456789", 10, 4, Ceiling, "1.26e8", "0x7.8E+6", Greater);
    test(
        "123456789",
        10,
        64,
        Nearest,
        "123456789.0",
        "0x75bcd15.0",
        Equal,
    );
    test("123456789", 10, 20, Down, "1.234568e8", "0x7.5bcdE+6", Less);
    test(
        "123456789",
        10,
        20,
        Up,
        "1.2345690e8",
        "0x7.5bcd8E+6",
        Greater,
    );
    test(
        "1e100000000000000000000",
        10,
        10,
        Nearest,
        "9.9912e99999999999999999999",
        "0x8.2cE+83048202372184058696",
        Less,
    );
    test(
        "1e100000000000000000000",
        10,
        4,
        Floor,
        "9.8e99999999999999999999",
        "0x8.0E+83048202372184058696",
        Less,
    );
    test(
        "1e100000000000000000000",
        10,
        4,
        Ceiling,
        "1.10e100000000000000000000",
        "0x9.0E+83048202372184058696",
        Greater,
    );
    test(
        "1e100000000000000000000",
        10,
        64,
        Nearest,
        "9.99999999999999999972e99999999999999999999",
        "0x8.2dd9cfe01bc881dE+83048202372184058696",
        Less,
    );
    test(
        "1e100000000000000000000",
        10,
        20,
        Down,
        "9.9999849e99999999999999999999",
        "0x8.2dd9E+83048202372184058696",
        Less,
    );
    test(
        "1e100000000000000000000",
        10,
        20,
        Up,
        "1.000000e100000000000000000000",
        "0x8.2ddaE+83048202372184058696",
        Greater,
    );
    test(
        "-3.7e-100000000000000000000",
        10,
        10,
        Nearest,
        "-3.698e-100000000000000000000",
        "-0x7.3cE-83048202372184058697",
        Greater,
    );
    test(
        "-3.7e-100000000000000000000",
        10,
        4,
        Floor,
        "-3.83e-100000000000000000000",
        "-0x7.8E-83048202372184058697",
        Less,
    );
    test(
        "-3.7e-100000000000000000000",
        10,
        4,
        Ceiling,
        "-3.6e-100000000000000000000",
        "-0x7.0E-83048202372184058697",
        Greater,
    );
    test(
        "-3.7e-100000000000000000000",
        10,
        64,
        Nearest,
        "-3.7000000000000000000e-100000000000000000000",
        "-0x7.3ceaade39e2d1f2E-83048202372184058697",
        Greater,
    );
    test(
        "-3.7e-100000000000000000000",
        10,
        20,
        Down,
        "-3.6999986e-100000000000000000000",
        "-0x7.3cea8E-83048202372184058697",
        Greater,
    );
    test(
        "-3.7e-100000000000000000000",
        10,
        20,
        Up,
        "-3.700003e-100000000000000000000",
        "-0x7.3cebE-83048202372184058697",
        Less,
    );
    test(
        "1.1e1000000000",
        10,
        10,
        Nearest,
        "1.1002e1000000000",
        "0x8.24E+830482023",
        Greater,
    );
    test(
        "1.1e1000000000",
        10,
        4,
        Floor,
        "1.1e1000000000",
        "0x8.0E+830482023",
        Less,
    );
    test(
        "1.1e1000000000",
        10,
        4,
        Ceiling,
        "1.22e1000000000",
        "0x9.0E+830482023",
        Greater,
    );
    test(
        "1.1e1000000000",
        10,
        64,
        Nearest,
        "1.0999999999999999999e1000000000",
        "0x8.239accb74994da6E+830482023",
        Less,
    );
    test(
        "1.1e1000000000",
        10,
        20,
        Down,
        "1.099998e1000000000",
        "0x8.239aE+830482023",
        Less,
    );
    test(
        "1.1e1000000000",
        10,
        20,
        Up,
        "1.1000004e1000000000",
        "0x8.239bE+830482023",
        Greater,
    );
    test(
        "-9.99e-999999999",
        10,
        10,
        Nearest,
        "-9.99e-999999999",
        "-0xd.8E-830482023",
        Greater,
    );
    test(
        "-9.99e-999999999",
        10,
        4,
        Floor,
        "-1.0e-999999998",
        "-0xe.0E-830482023",
        Less,
    );
    test(
        "-9.99e-999999999",
        10,
        4,
        Ceiling,
        "-9.62e-999999999",
        "-0xd.0E-830482023",
        Greater,
    );
    test(
        "-9.99e-999999999",
        10,
        64,
        Nearest,
        "-9.9900000000000000003e-999999999",
        "-0xd.8063eb20499cc2aE-830482023",
        Less,
    );
    test(
        "-9.99e-999999999",
        10,
        20,
        Down,
        "-9.9899896e-999999999",
        "-0xd.8063E-830482023",
        Greater,
    );
    test(
        "-9.99e-999999999",
        10,
        20,
        Up,
        "-9.990001e-999999999",
        "-0xd.8064E-830482023",
        Less,
    );
    test("0.0", 10, 10, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", 10, 4, Floor, "0.0", "0x0.0", Equal);
    test("0.0", 10, 4, Ceiling, "0.0", "0x0.0", Equal);
    test("0.0", 10, 64, Nearest, "0.0", "0x0.0", Equal);
    test("0.0", 10, 20, Down, "0.0", "0x0.0", Equal);
    test("0.0", 10, 20, Up, "0.0", "0x0.0", Equal);
    test("-0.0", 10, 10, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", 10, 4, Floor, "0.0", "0x0.0", Equal);
    test("-0.0", 10, 4, Ceiling, "0.0", "0x0.0", Equal);
    test("-0.0", 10, 64, Nearest, "0.0", "0x0.0", Equal);
    test("-0.0", 10, 20, Down, "0.0", "0x0.0", Equal);
    test("-0.0", 10, 20, Up, "0.0", "0x0.0", Equal);
    test("NaN", 10, 10, Nearest, "NaN", "NaN", Equal);
    test("NaN", 10, 4, Floor, "NaN", "NaN", Equal);
    test("NaN", 10, 4, Ceiling, "NaN", "NaN", Equal);
    test("NaN", 10, 64, Nearest, "NaN", "NaN", Equal);
    test("NaN", 10, 20, Down, "NaN", "NaN", Equal);
    test("NaN", 10, 20, Up, "NaN", "NaN", Equal);
    test("Infinity", 10, 10, Nearest, "Infinity", "Infinity", Equal);
    test("Infinity", 10, 4, Floor, "Infinity", "Infinity", Equal);
    test("Infinity", 10, 4, Ceiling, "Infinity", "Infinity", Equal);
    test("Infinity", 10, 64, Nearest, "Infinity", "Infinity", Equal);
    test("Infinity", 10, 20, Down, "Infinity", "Infinity", Equal);
    test("Infinity", 10, 20, Up, "Infinity", "Infinity", Equal);
    test(
        "2.1e+100000000000000000000",
        3,
        10,
        Nearest,
        "7.426e47712125471966243729",
        "0x6.0cE+39624062518028904536",
        Less,
    );
    test(
        "2.1e+100000000000000000000",
        3,
        4,
        Floor,
        "7.4e47712125471966243729",
        "0x6.0E+39624062518028904536",
        Less,
    );
    test(
        "2.1e+100000000000000000000",
        3,
        4,
        Ceiling,
        "7.98e47712125471966243729",
        "0x6.8E+39624062518028904536",
        Greater,
    );
    test(
        "2.1e+100000000000000000000",
        3,
        64,
        Nearest,
        "7.42620802787345872974e47712125471966243729",
        "0x6.0c1aa9728b109a48E+39624062518028904536",
        Less,
    );
    test(
        "2.1e+100000000000000000000",
        3,
        20,
        Down,
        "7.4262050e47712125471966243729",
        "0x6.0c1a8E+39624062518028904536",
        Less,
    );
    test(
        "2.1e+100000000000000000000",
        3,
        20,
        Up,
        "7.426214e47712125471966243729",
        "0x6.0c1bE+39624062518028904536",
        Greater,
    );
    test("0.1", 3, 10, Nearest, "0.33350", "0x0.556", Greater);
    test("0.1", 3, 4, Floor, "0.31", "0x0.5", Less);
    test("0.1", 3, 4, Ceiling, "0.344", "0x0.58", Greater);
    test(
        "0.1",
        3,
        64,
        Nearest,
        "0.333333333333333333342",
        "0x0.55555555555555558",
        Greater,
    );
    test("0.1", 3, 20, Down, "0.3333330", "0x0.55555", Less);
    test("0.1", 3, 20, Up, "0.33333349", "0x0.555558", Greater);
    test("ff", 16, 10, Nearest, "255.0", "0xff.0", Equal);
    test("ff", 16, 4, Floor, "240.0", "0xf.0E+1", Less);
    test("ff", 16, 4, Ceiling, "2.6e2", "0x1.0E+2", Greater);
    test("ff", 16, 64, Nearest, "255.0", "0xff.0", Equal);
    test("ff", 16, 20, Down, "255.0", "0xff.0", Equal);
    test("ff", 16, 20, Up, "255.0", "0xff.0", Equal);
    test("1e5", 16, 10, Nearest, "485.0", "0x1e5.0", Equal);
    test("1e5", 16, 4, Floor, "480.0", "0x1.eE+2", Less);
    test("1e5", 16, 4, Ceiling, "5.1e2", "0x2.0E+2", Greater);
    test("1e5", 16, 64, Nearest, "485.0", "0x1e5.0", Equal);
    test("1e5", 16, 20, Down, "485.0", "0x1e5.0", Equal);
    test("1e5", 16, 20, Up, "485.0", "0x1e5.0", Equal);
    test("1e+5", 16, 10, Nearest, "1.0e6", "0x1.0E+5", Equal);
    test("1e+5", 16, 4, Floor, "1.0e6", "0x1.0E+5", Equal);
    test("1e+5", 16, 4, Ceiling, "1.0e6", "0x1.0E+5", Equal);
    test("1e+5", 16, 64, Nearest, "1.0e6", "0x1.0E+5", Equal);
    test("1e+5", 16, 20, Down, "1.0e6", "0x1.0E+5", Equal);
    test("1e+5", 16, 20, Up, "1.0e6", "0x1.0E+5", Equal);
    test(
        "a.b8e-40000000000000000000",
        16,
        10,
        Nearest,
        "6.791e-48164799306236991234",
        "0xa.b8E-40000000000000000000",
        Equal,
    );
    test(
        "a.b8e-40000000000000000000",
        16,
        4,
        Floor,
        "6.3e-48164799306236991234",
        "0xa.0E-40000000000000000000",
        Less,
    );
    test(
        "a.b8e-40000000000000000000",
        16,
        4,
        Ceiling,
        "6.97e-48164799306236991234",
        "0xb.0E-40000000000000000000",
        Greater,
    );
    test(
        "a.b8e-40000000000000000000",
        16,
        64,
        Nearest,
        "6.791e-48164799306236991234",
        "0xa.b8E-40000000000000000000",
        Equal,
    );
    test(
        "a.b8e-40000000000000000000",
        16,
        20,
        Down,
        "6.791e-48164799306236991234",
        "0xa.b8E-40000000000000000000",
        Equal,
    );
    test(
        "a.b8e-40000000000000000000",
        16,
        20,
        Up,
        "6.791e-48164799306236991234",
        "0xa.b8E-40000000000000000000",
        Equal,
    );
    test("zz.z", 36, 10, Nearest, "1296.0", "0x5.1E+2", Greater);
    test("zz.z", 36, 4, Floor, "1.3e3", "0x5.0E+2", Less);
    test("zz.z", 36, 4, Ceiling, "1.41e3", "0x5.8E+2", Greater);
    test(
        "zz.z",
        36,
        64,
        Nearest,
        "1295.97222222222222221",
        "0x50f.f8e38e38e38e38",
        Less,
    );
    test("zz.z", 36, 20, Down, "1295.9707", "0x50f.f88", Less);
    test("zz.z", 36, 20, Up, "1295.973", "0x50f.f9", Greater);
    test(
        "1.z1e+99999999999",
        36,
        10,
        Nearest,
        "2.9345e155630250075",
        "0x1.078E+129248125035",
        Less,
    );
    test(
        "1.z1e+99999999999",
        36,
        4,
        Floor,
        "2.9e155630250075",
        "0x1.0E+129248125035",
        Less,
    );
    test(
        "1.z1e+99999999999",
        36,
        4,
        Ceiling,
        "3.21e155630250075",
        "0x1.2E+129248125035",
        Greater,
    );
    test(
        "1.z1e+99999999999",
        36,
        64,
        Nearest,
        "2.93460550218253611572e155630250075",
        "0x1.0781cdcce87fa16aE+129248125035",
        Less,
    );
    test(
        "1.z1e+99999999999",
        36,
        20,
        Down,
        "2.934603e155630250075",
        "0x1.0781cE+129248125035",
        Less,
    );
    test(
        "1.z1e+99999999999",
        36,
        20,
        Up,
        "2.9346086e155630250075",
        "0x1.0781eE+129248125035",
        Greater,
    );

    // strings that are not numbers, whatever the precision and rounding mode
    let test_invalid = |s: &str, base: u8| {
        for rm in [Floor, Ceiling, Down, Up, Nearest, Exact] {
            assert!(
                BigExponentFloat::from_sci_string_with_options_prec(s, from_options(base, rm), 10)
                    .is_none()
            );
        }
    };
    test_invalid("abc", 10);
    test_invalid("1e", 10);
}

#[test]
fn test_from_sci_string() {
    let test = |s: &str, base: u8, out: &str, out_hex: &str| {
        let x =
            BigExponentFloat::from_sci_string_with_options(s, from_options(base, Nearest)).unwrap();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), out, "{s} {base}");
        assert_eq!(format!("{x:?}"), out_hex, "{s} {base}");
        if base == 10 {
            assert!(same(&BigExponentFloat::from_sci_string(s).unwrap(), &x));
        }
    };
    test("1.5", 10, "1.5", "0x1.8");
    test("0.1", 10, "0.102", "0x0.1a");
    test("0.10", 10, "0.0996", "0x0.198");
    test("-2.5e-7", 10, "-2.496e-7", "-0x4.3E-6");
    test(
        "1e100000000000000000000",
        10,
        "9.8e99999999999999999999",
        "0x8.0E+83048202372184058696",
    );
    test(
        "-3.70e-100000000000000000000",
        10,
        "-3.698e-100000000000000000000",
        "-0x7.3cE-83048202372184058697",
    );
    test("1.1e1000000000", 10, "1.098e1000000000", "0x8.2E+830482023");
    test("0.0", 10, "0.0", "0x0.0");
    test("NaN", 10, "NaN", "NaN");
    test("0.1", 3, "0.38", "0x0.6");
    test(
        "2.1e+100000000000000000000",
        3,
        "7.4e47712125471966243729",
        "0x6.0E+39624062518028904536",
    );
    test("ff", 16, "255.0", "0xff.0");
    test(
        "a.b8e-40000000000",
        16,
        "6.211e-48164799306",
        "0xa.b8E-40000000000",
    );
    test("zz.z", 36, "1295.97", "0x50f.f8");
    test(
        "1.z1e+99999999999",
        36,
        "2.93461e155630250075",
        "0x1.0782E+129248125035",
    );

    let test_invalid = |s: &str, base: u8| {
        assert!(
            BigExponentFloat::from_sci_string_with_options(s, from_options(base, Nearest))
                .is_none()
        );
    };
    test_invalid("abc", 10);
}

#[test]
#[should_panic]
fn from_sci_string_prec_round_fail_1() {
    BigExponentFloat::from_sci_string_prec_round("1.5", 0, Nearest);
}

#[test]
#[should_panic]
fn from_sci_string_prec_round_fail_2() {
    // 0.1 is not exactly representable
    BigExponentFloat::from_sci_string_prec_round("0.1", 10, Exact);
}

#[test]
#[should_panic]
fn from_sci_string_prec_round_fail_3() {
    // nor is 0.1 * 10^(10^20)
    BigExponentFloat::from_sci_string_prec_round("0.1e100000000000000000000", 10, Exact);
}

#[test]
fn from_sci_string_properties() {
    // Strings written by `to_sci`, read back with a precision and rounding mode derived from them.
    big_exponent_float_to_sci_options_pair_gen_var_1().test_properties(|(x, options)| {
        let s = x.to_sci_with_options(options).to_string();
        let base = options.get_base();
        let prec = x.get_prec().unwrap_or(1) + 3;
        let floor = BigExponentFloat::from_sci_string_with_options_prec(
            &s,
            from_options(base, Floor),
            prec,
        )
        .unwrap();
        let ceiling = BigExponentFloat::from_sci_string_with_options_prec(
            &s,
            from_options(base, Ceiling),
            prec,
        )
        .unwrap();
        let nearest = BigExponentFloat::from_sci_string_with_options_prec(
            &s,
            from_options(base, Nearest),
            prec,
        )
        .unwrap();
        for (y, o) in [&floor, &ceiling, &nearest] {
            assert!(y.is_valid());
            if y.is_finite() && *y != 0u32 {
                assert!(y.get_prec().unwrap() <= prec);
            }
            if *o == Equal {
                assert!(same(&floor.0, y) && same(&ceiling.0, y));
            }
        }
        // - the orderings bracket the string's value
        assert_ne!(floor.1, Greater);
        assert_ne!(ceiling.1, Less);
        assert!(floor.0 <= ceiling.0 || floor.0.is_nan());
        assert!(same(&nearest.0, &floor.0) || same(&nearest.0, &ceiling.0));
        // - agreement with `Float`, when both can represent the result
        if let Some((f, fo)) =
            Float::from_sci_string_with_options_prec(&s, from_options(base, Nearest), prec)
            && f.is_finite()
            && f != 0u32
            && big_exponent_float_to_float(&nearest.0).is_some()
        {
            assert!(same(&nearest.0, &BigExponentFloat::from(f)));
            assert_eq!(nearest.1, fo);
        }
        // - with an implied precision
        let implied =
            BigExponentFloat::from_sci_string_with_options(&s, from_options(base, Nearest))
                .unwrap();
        assert!(implied.is_valid());
        if let Some(f) = Float::from_sci_string_with_options(&s, from_options(base, Nearest))
            && f.is_finite()
            && f != 0u32
            && big_exponent_float_to_float(&implied).is_some()
        {
            assert!(same(&implied, &BigExponentFloat::from(f)));
        }
    });
}
