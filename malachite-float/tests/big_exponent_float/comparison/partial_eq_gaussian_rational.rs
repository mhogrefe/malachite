// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, float_gaussian_rational_pair_gen,
};
use malachite_q::gaussian_rational::GaussianRational;

#[test]
fn test_partial_eq_gaussian_rational() {
    let test = |s: &str, s_hex: &str, y: &str| {
        let x = parse_big_exponent_float_hex(s_hex);
        assert_eq!(x.to_string(), s);
        let y = GaussianRational::from_str(y).unwrap();
        let out = y.imaginary == 0u32 && x == y.real;
        assert_eq!(x == y, out);
        assert_eq!(y == x, out);
    };
    test("NaN", "NaN", "3+4i");
    test("Infinity", "Infinity", "3+4i");
    test("-Infinity", "-Infinity", "0");
    test("0.0", "0x0.0", "0");
    test("0.0", "0x0.0", "i");
    test("5.0", "0x5.0", "3+4i");
    test("-5.0", "-0x5.0", "-3-4i");
    test("5.0", "0x5.0", "-4i");
    test("-4.0", "-0x4.0", "-4i");
    test("3.0", "0x3.0", "3");
    test("-3.0", "-0x3.0", "3");
    test("4.50", "0x4.8", "3+4i");
    test("5.50", "0x5.8", "3+4i");
    test(
        "5.000000000000000000000000052",
        "0x5.000000000000000000001",
        "3+4i",
    );
    test(
        "4.999999999999999999999999948",
        "0x4.fffffffffffffffffffff",
        "3+4i",
    );
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "3+4i",
    );
    test(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        "-3+4i",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "3+4i",
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "0",
    );
    test(
        "-4.2e-30102999566398119522",
        "-0x1.0E-25000000000000000000",
        "i",
    );
    test("2.5", "0x2.8", "3/2+2i");
    test("-2.5", "-0x2.8", "-3/2-2i");
    test("0.50", "0x0.8", "1/3+i/3");
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        "1/1000000000000000000000000000000+i/1000000000000000000000000000000",
    );
    test("3.0", "0x3.0", "3+0i");
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        "0",
    );
    #[cfg(feature = "oracle")]
    for x in [
        "NaN",
        "Infinity",
        "-Infinity",
        "0x0.0",
        "0x5.0",
        "-0x5.0",
        "-0x4.0",
        "0x3.0",
        "-0x3.0",
        "0x4.8",
        "0x5.8",
        "0x5.0000000000000000000010",
        "0x4.fffffffffffffffffffff0",
        "0x2.8",
        "-0x2.8",
        "0x0.8",
        "0x1.0E+25000000000000000000",
        "0x1.0E-25000000000000000000",
        "-0x1.0E+25000000000000000000",
        "-0x1.0E-25000000000000000000",
    ] {
        let x = parse_big_exponent_float_hex(x);
        for y in ["3+4i", "-3/2-2i", "3/2+2i", "1/3+i/3", "-3/2", "i/2", "0"] {
            let y = GaussianRational::from_str(y).unwrap();
            malachite_base::oracle_row!(
                "flint",
                "arf_cmp",
                "{x:?} {} {y}",
                malachite_base::test_util::oracle_rows::eq_symbol(x == y)
            );
        }
    }
    #[cfg(feature = "oracle")]
    for x in ["3+4i", "-3/2-2i", "3/2+2i", "1/3+i/3", "-3/2", "i/2", "0"] {
        let x = GaussianRational::from_str(x).unwrap();
        for y in [
            "Infinity",
            "-Infinity",
            "0x0.0",
            "0x5.0",
            "-0x5.0",
            "-0x4.0",
            "0x3.0",
            "-0x3.0",
            "0x4.8",
            "0x5.8",
            "0x5.0000000000000000000010",
            "0x4.fffffffffffffffffffff0",
            "0x2.8",
            "-0x2.8",
            "0x0.8",
            "0x1.0E+25000000000000000000",
            "0x1.0E-25000000000000000000",
            "-0x1.0E+25000000000000000000",
            "-0x1.0E-25000000000000000000",
        ] {
            let y = parse_big_exponent_float_hex(y);
            malachite_base::oracle_row!(
                "flint",
                "arf_cmp",
                "{x} {} {y:?}",
                malachite_base::test_util::oracle_rows::eq_symbol(x == y)
            );
        }
    }
}

#[test]
fn partial_eq_gaussian_rational_properties() {
    // - agreement with `Float`
    float_gaussian_rational_pair_gen().test_properties(|(x, y)| {
        let out = x == y;
        let x = float_to_big_exponent_float(&x);
        assert_eq!(x == y, out);
        assert_eq!(y == x, out);
    });

    big_exponent_float_gen().test_properties(|x| {
        let y = GaussianRational::from_str("1000000000000000000000000000000/7").unwrap();
        assert_eq!(x == y, x == y.real);
    });
}
