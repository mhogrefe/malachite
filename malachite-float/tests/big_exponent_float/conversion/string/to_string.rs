// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{DivExact, Mod, ModPowerOf2};
use malachite_base::num::basic::traits::{Infinity, NaN, NegativeInfinity, NegativeOne, One, Zero};
use malachite_base::num::conversion::traits::ToStringBase;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::strings::{ToLowerHexString, ToUpperHexString};
use malachite_float::BigExponentFloat;
use malachite_float::big_exponent_float::exhaustive::*;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, big_exponent_float_to_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::big_exponent_float_gen_var_1;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_gen;
use std::str::FromStr;

#[test]
fn test_to_hex_string() {
    let test = |x: BigExponentFloat, lower: &str, upper: &str| {
        assert_eq!(x.to_lower_hex_string(), lower);
        assert_eq!(x.to_upper_hex_string(), upper);
        let alt = if x.is_finite() {
            format!("0x{lower}").replace("0x-", "-0x")
        } else {
            lower.to_string()
        };
        assert_eq!(format!("{x:#x}"), alt);
        assert_eq!(format!("{x:?}"), format!("{x:#x}"));
    };
    test(BigExponentFloat::NAN, "NaN", "NaN");
    test(BigExponentFloat::INFINITY, "Infinity", "Infinity");
    test(
        BigExponentFloat::NEGATIVE_INFINITY,
        "-Infinity",
        "-Infinity",
    );
    test(BigExponentFloat::ZERO, "0.0", "0.0");
    test(BigExponentFloat::ONE, "1.0", "1.0");
    test(BigExponentFloat::NEGATIVE_ONE, "-1.0", "-1.0");
    test(parse_big_exponent_float_hex("0xff.0#8"), "ff.0", "FF.0");
    test(
        parse_big_exponent_float_hex("0x3.243f6a8885a30#53"),
        "3.243f6a8885a3",
        "3.243F6A8885A3",
    );
    test(
        parse_big_exponent_float_hex("0x4.0E+268435455#1"),
        "4.0E+268435455",
        "4.0E+268435455",
    );
    // - exponents outside `Float`'s range
    let test_power = |e: &str, out: &str| {
        let x = big_exponent_float_power_of_2(Integer::from_str(e).unwrap());
        assert_eq!(x.to_lower_hex_string(), out);
    };
    test_power("1073741824", "1.0E+268435456");
    test_power("1073741825", "2.0E+268435456");
    test_power("1073741827", "8.0E+268435456");
    test_power("-1073741825", "8.0E-268435457");
    test_power("100000000000000000000", "1.0E+25000000000000000000");
    test_power("-100000000000000000001", "8.0E-25000000000000000001");
}

#[test]
fn to_hex_string_properties() {
    // - agreement with `Float`
    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(format!("{x:x}"), format!("{f:x}"));
        assert_eq!(format!("{x:#x}"), format!("{f:#x}"));
        assert_eq!(format!("{x:X}"), format!("{f:X}"));
        assert_eq!(format!("{x:#X}"), format!("{f:#X}"));
    });

    integer_gen().test_properties(|s| {
        if s.unsigned_abs_ref().significant_bits() <= 31 {
            return;
        }
        // - 2^s is 2^(s mod 4) * 16^floor(s / 4)
        let x = big_exponent_float_power_of_2(s.clone());
        let digit = 1u32 << u32::try_from(&(&s).mod_power_of_2(2)).unwrap();
        let k: Integer = &s >> 2u32;
        let plus = if k >= 0u32 { "+" } else { "" };
        assert_eq!(x.to_lower_hex_string(), format!("{digit}.0E{plus}{k}"));
        // - shifting the sci-exponent by a multiple of 4 only shifts the printed exponent; the
        //   first 64 values have up to 7 significant bits
        let r = Integer::from((&s).mod_power_of_2(2));
        let near = if s >= 0u32 {
            r + Integer::from(1u32 << 29)
        } else {
            r - Integer::from(1u32 << 29)
        };
        let shift: Integer = (&s - &near) >> 2u32;
        for (y, z) in exhaustive_positive_big_exponent_floats_with_sci_exponent(s.clone())
            .zip(exhaustive_positive_big_exponent_floats_with_sci_exponent(
                near,
            ))
            .take(64)
        {
            let y = y.to_lower_hex_string();
            let z = z.to_lower_hex_string();
            let (y_digits, y_e) = y.split_once('E').unwrap();
            let (z_digits, z_e) = z.split_once('E').unwrap();
            assert_eq!(y_digits, z_digits);
            assert_eq!(
                Integer::from_str(y_e.trim_start_matches('+')).unwrap(),
                Integer::from_str(z_e.trim_start_matches('+')).unwrap() + &shift
            );
        }
    });
}

#[test]
fn power_of_2_base_properties() {
    // - agreement with `Float`
    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(format!("{x:b}"), format!("{f:b}"));
        assert_eq!(format!("{x:#b}"), format!("{f:#b}"));
        assert_eq!(format!("{x:o}"), format!("{f:o}"));
        assert_eq!(format!("{x:#o}"), format!("{f:#o}"));
        for base in [2, 4, 8, 16, 32] {
            assert_eq!(x.to_string_base(base), f.to_string_base(base));
            assert_eq!(x.to_string_base_upper(base), f.to_string_base_upper(base));
        }
    });

    // - in base 2^k, 2^s is 2^(s mod k) * (2^k)^floor(s / k), for exponents beyond `Float`'s range
    integer_gen().test_properties(|s| {
        if s.unsigned_abs_ref().significant_bits() <= 31 {
            return;
        }
        let x = big_exponent_float_power_of_2(s.clone());
        for (base, k) in [(2u8, 1u32), (4, 2), (8, 3), (32, 5)] {
            let kk = Integer::from(k);
            let r = u32::try_from(&(&s).mod_op(&kk)).unwrap();
            let q = (&s - Integer::from(r)).div_exact(&kk);
            let digit = std::char::from_digit(1 << r, u32::from(base)).unwrap();
            // - only base 32, where `E` is also a digit, writes the sign of a positive exponent
            let plus = if base == 32 && q >= 0u32 { "+" } else { "" };
            assert_eq!(x.to_string_base(base), format!("{digit}.0E{plus}{q}"));
        }
    });
}

#[test]
#[should_panic]
fn to_string_base_fail() {
    BigExponentFloat::ONE.to_string_base(10);
}
