// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::conversion::string::format_natural::gmp_format;
use malachite_nz::integer::Integer;
use malachite_nz::integer::conversion::string::format_integer::format_integer_str;
use malachite_nz::test_util::generators::integer_string_pair_gen_var_1;
use std::str::FromStr;

#[test]
fn test_format_integer_str() {
    fn test(s: &str, fmt: &str, out: Option<&str>) {
        let x = Integer::from_str(s).unwrap();
        assert_eq!(format_integer_str(&x, fmt).as_deref(), out, "{s} {fmt}");
        if out.is_some() {
            // every valid case must agree with GMP itself
            assert_eq!(gmp_format(&rug::Integer::from(&x), fmt).as_deref(), out);
        }
    }
    // a negative value keeps its sign under every conversion
    test("-255", "%Zd", Some("-255"));
    test("-255", "%Zi", Some("-255"));
    test("-255", "%Zu", Some("-255"));
    test("-255", "%Zo", Some("-377"));
    test("-255", "%Zx", Some("-ff"));
    test("-255", "%ZX", Some("-FF"));
    test("255", "%Zd", Some("255"));
    test(
        "-1267650600228229401496703205376",
        "%Zd",
        Some("-1267650600228229401496703205376"),
    );

    // the sign precedes the base prefix
    test("-255", "%#Zx", Some("-0xff"));
    test("-255", "%#ZX", Some("-0XFF"));
    test("-255", "%#Zo", Some("-0377"));

    // the value's sign overrides the + and space flags
    test("-255", "%+Zd", Some("-255"));
    test("-255", "% Zd", Some("-255"));
    test("255", "%+Zd", Some("+255"));
    test("255", "% Zd", Some(" 255"));

    // width, justification, and zero padding; the padding goes after the sign and prefix
    test("-255", "%8Zd", Some("    -255"));
    test("-255", "%-8Zd", Some("-255    "));
    test("-255", "%08Zd", Some("-0000255"));
    test("-255", "%#08Zx", Some("-0x000ff"));

    // precision pads the absolute value's digits
    test("-255", "%.6Zd", Some("-000255"));
    test("-255", "%#.5Zx", Some("-0x000ff"));

    // literal text and %% escapes
    test("-255", "x = %Zd!", Some("x = -255!"));
    test("-255", "100%% of %Zd", Some("100% of -255"));

    // invalid or unsupported format strings
    test("-255", "", None);
    test("-255", "%d", None);
    test("-255", "%*Zd", None);
    test("-255", "%Zd %Zd", None);
}

#[test]
fn format_integer_str_properties() {
    integer_string_pair_gen_var_1().test_properties(|(x, fmt)| {
        let rx = rug::Integer::from(&x);
        let s = format_integer_str(&x, &fmt).unwrap();
        // The primary oracle: we must produce exactly what GMP's own `gmp_snprintf` produces.
        assert_eq!(gmp_format(&rx, &fmt).unwrap(), s, "{x} {fmt}");
        assert!(s.is_ascii());
        // Surrounding literal text and `%%` escapes pass through unchanged.
        let wrapped = format!("a %% {fmt} b");
        let sw = format_integer_str(&x, &wrapped).unwrap();
        assert_eq!(gmp_format(&rx, &wrapped).unwrap(), sw);
        assert_eq!(sw, format!("a % {s} b"));
    });
}
