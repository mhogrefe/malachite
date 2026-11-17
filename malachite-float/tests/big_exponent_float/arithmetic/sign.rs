// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Sign;
use malachite_base::num::basic::traits::NaN;
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::{
    float_to_big_exponent_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{big_exponent_float_gen, float_gen};
use std::cmp::Ordering::*;

#[test]
fn test_sign() {
    let test = |x: &str, x_hex: &str, out| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(x.sign(), out);
    };
    test("Infinity", "Infinity", Greater);
    test("-Infinity", "-Infinity", Less);
    test("0.0", "0x0.0", Equal);
    test("1.0", "0x1.0", Greater);
    test("-1.5", "-0x1.8", Less);
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        Greater,
    );
    test(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        Less,
    );
}

#[test]
#[should_panic]
fn sign_fail() {
    BigExponentFloat::NAN.sign();
}

#[test]
fn sign_properties() {
    big_exponent_float_gen().test_properties(|x| {
        if !x.is_nan() {
            let s = x.sign();
            assert_eq!(s, x.partial_cmp(&0u32).unwrap());
            assert_eq!((-&x).sign(), s.reverse());
        }
    });

    float_gen().test_properties(|x| {
        if !x.is_nan() && !x.is_zero() {
            assert_eq!(float_to_big_exponent_float(&x).sign(), x.sign());
        }
    });
}
