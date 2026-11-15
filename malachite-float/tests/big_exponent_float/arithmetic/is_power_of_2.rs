// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{IsPowerOf2, PowerOf2};
use malachite_float::BigExponentFloat;
use malachite_float::test_util::big_exponent_float::parse_big_exponent_float_hex;
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1_float,
};

#[test]
fn test_is_power_of_2() {
    let test = |x: &str, x_hex: &str, out| {
        assert_eq!(parse_big_exponent_float_hex(x_hex).to_string(), x);
        let x = &parse_big_exponent_float_hex(x_hex);
        assert_eq!(x.is_power_of_2(), out);
    };
    test("1.0", "0x1.0", true);
    test("2.0", "0x2.0", true);
    test("0.50", "0x0.8", true);
    test("1.0e3", "0x4.0E+2", true);
    test("3.0", "0x3.0", false);
    test("0.75", "0x0.c", false);
    test("-2.0", "-0x2.0", false);
    test("0.0", "0x0.0", false);
    test("NaN", "NaN", false);
    test("Infinity", "Infinity", false);
    test("-Infinity", "-Infinity", false);
    test(
        "2.4e30102999566398119521",
        "0x1.0E+25000000000000000000",
        true,
    );
    test(
        "4.2e-30102999566398119522",
        "0x1.0E-25000000000000000000",
        true,
    );
    test(
        "-2.4e30102999566398119521",
        "-0x1.0E+25000000000000000000",
        false,
    );
}

#[test]
fn is_power_of_2_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let p = x.is_power_of_2();
        assert_eq!(p, x > 0u32 && x.is_finite() && x.get_prec() == Some(1));
        if p && let Some(e) = x
            .get_exponent()
            .and_then(|e| i64::try_from(e).ok())
            .and_then(|e| e.checked_sub(1))
        {
            assert_eq!(x, BigExponentFloat::power_of_2(e));
        }
    });

    big_exponent_float_gen_var_1_float().test_properties(|(f, x)| {
        assert_eq!(x.is_power_of_2(), f.is_power_of_2());
    });
}
