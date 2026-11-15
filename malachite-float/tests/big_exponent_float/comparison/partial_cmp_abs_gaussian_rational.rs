// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::AbsSquared;
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_exponent_is_small, float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, float_gaussian_rational_pair_gen,
};
use malachite_q::Rational;
use malachite_q::gaussian_rational::GaussianRational;
use std::cmp::Ordering::{self, *};

// Compares |x| with |y| through x^2 and |y|^2, for an `x` with a small exponent, and otherwise by
// the size of the exponent.
fn gaussian_partial_cmp_abs_oracle(x: &BigExponentFloat, y: &GaussianRational) -> Option<Ordering> {
    if x.is_nan() {
        None
    } else if !x.is_finite() {
        Some(Greater)
    } else if big_exponent_float_exponent_is_small(x) {
        Some(Rational::exact_from(x).abs_squared().cmp(&y.abs_squared()))
    } else if *x.get_exponent().unwrap() > 0u32 {
        Some(Greater)
    } else {
        Some(if *y == 0u32 { Greater } else { Less })
    }
}

#[test]
fn test_partial_cmp_abs_gaussian_rational() {
    let test = |s: &str, s_hex: &str, y: &str| {
        let x = parse_big_exponent_float_hex(s_hex);
        assert_eq!(x.to_string(), s);
        let y = GaussianRational::from_str(y).unwrap();
        let out = gaussian_partial_cmp_abs_oracle(&x, &y);
        assert_eq!(x.partial_cmp_abs(&y), out);
        assert_eq!(y.partial_cmp_abs(&x).map(Ordering::reverse), out);
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
}

#[test]
fn partial_cmp_abs_gaussian_rational_properties() {
    // - agreement with `Float`
    float_gaussian_rational_pair_gen().test_properties(|(x, y)| {
        let out = x.partial_cmp_abs(&y);
        let x = float_to_big_exponent_float(&x);
        assert_eq!(x.partial_cmp_abs(&y), out);
        assert_eq!(y.partial_cmp_abs(&x).map(Ordering::reverse), out);
        assert_eq!(
            x.partial_cmp_abs(&y),
            gaussian_partial_cmp_abs_oracle(&x, &y)
        );
    });

    big_exponent_float_gen().test_properties(|x| {
        for y in ["0", "1+i", "-3+4i", "1000000000000000000000000000000/7"] {
            let y = GaussianRational::from_str(y).unwrap();
            assert_eq!(
                x.partial_cmp_abs(&y),
                gaussian_partial_cmp_abs_oracle(&x, &y)
            );
        }
    });
}
