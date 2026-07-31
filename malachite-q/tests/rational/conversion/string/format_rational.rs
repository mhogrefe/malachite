// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::gmp_format;
use malachite_nz::natural::Natural;
use malachite_q::Rational;
use malachite_q::rational::conversion::string::format_rational::format_rational_str;
use malachite_q::test_util::generators::rational_string_pair_gen_var_1;
use std::str::FromStr;

// Formats `x` with `fmt` using GMP's own `gmp_snprintf` (linked in via rug), the reference we must
// match. Returns `None` when GMP reports an error. `fmt` must contain exactly one conversion, a
// `%Q` one, so that the variadic call is well-formed.
fn gmp_format(x: &rug::Rational, fmt: &str) -> Option<String> {
    unsafe extern "C" {
        fn __gmp_snprintf(
            buf: *mut core::ffi::c_char,
            n: usize,
            template: *const core::ffi::c_char,
            ...
        ) -> core::ffi::c_int;
    }
    let template = std::ffi::CString::new(fmt).ok()?;
    let call = |buf: &mut [u8]| unsafe {
        __gmp_snprintf(
            buf.as_mut_ptr().cast(),
            buf.len(),
            template.as_ptr(),
            x.as_raw(),
        )
    };
    let mut buf = vec![0u8; 1 << 12];
    let mut n = call(&mut buf);
    if n < 0 {
        return None;
    }
    if n as usize >= buf.len() {
        buf = vec![0u8; n as usize + 1];
        n = call(&mut buf);
    }
    Some(String::from_utf8_lossy(&buf[..n as usize]).into_owned())
}

#[test]
fn test_format_rational_str() {
    fn test(s: &str, fmt: &str, out: Option<&str>) {
        let x = Rational::from_str(s).unwrap();
        assert_eq!(format_rational_str(&x, fmt).as_deref(), out, "{s} {fmt}");
        if out.is_some() {
            // every valid case must agree with GMP itself
            assert_eq!(gmp_format(&rug::Rational::from(&x), fmt).as_deref(), out);
        }
    }
    // the conversion characters; d, i, and u are all the same
    test("255/16", "%Qd", Some("255/16"));
    test("255/16", "%Qi", Some("255/16"));
    test("255/16", "%Qu", Some("255/16"));
    test("255/16", "%Qo", Some("377/20"));
    test("255/16", "%Qx", Some("ff/10"));
    test("255/16", "%QX", Some("FF/10"));
    // a denominator of 1 is omitted, as with mpq_get_str
    test("255", "%Qd", Some("255"));
    test("0", "%Qd", Some("0"));
    test("-255/16", "%Qd", Some("-255/16"));
    test("-255/16", "%Qx", Some("-ff/10"));

    // the # flag prefixes the numerator and the denominator
    test("255/16", "%#Qx", Some("0xff/0x10"));
    test("255/16", "%#QX", Some("0XFF/0X10"));
    test("255/16", "%#Qo", Some("0377/020"));
    test("-255/16", "%#Qx", Some("-0xff/0x10"));
    test("255", "%#Qx", Some("0xff"));

    // sign flags
    test("255/16", "%+Qd", Some("+255/16"));
    test("255/16", "% Qd", Some(" 255/16"));
    test("-255/16", "%+Qd", Some("-255/16"));

    // width and justification treat the whole fraction as a unit
    test("255/16", "%10Qd", Some("    255/16"));
    test("255/16", "%-10Qd", Some("255/16    "));
    test("255/16", "%010Qd", Some("0000255/16"));
    test("-255/16", "%010Qd", Some("-000255/16"));

    // GMP documents the influence of the precision on a rational as undefined; this reproduces what
    // its code does, padding the whole string to the precision with numerator zeros
    test("255/16", "%.8Qd", Some("00255/16"));
    test("255/16", "%.2Qd", Some("255/16"));

    // a zero value with an explicit precision of 0 produces no digits
    test("0", "%.0Qd", Some(""));
    test("0", "%#.0Qx", Some("0x"));

    // literal text and %% escapes
    test("255/16", "x = %Qd!", Some("x = 255/16!"));
    test("255/16", "100%% of %Qd", Some("100% of 255/16"));

    // invalid or unsupported format strings
    test("255/16", "", None);
    test("255/16", "%d", None);
    test("255/16", "%Zd", None);
    test("255/16", "%*Qd", None);
    test("255/16", "%Qd %Qd", None);
}

#[test]
fn format_rational_str_properties() {
    rational_string_pair_gen_var_1().test_properties(|(x, fmt)| {
        let rx = rug::Rational::from(&x);
        let s = format_rational_str(&x, &fmt).unwrap();
        // The primary oracle: we must produce exactly what GMP's own `gmp_snprintf` produces.
        assert_eq!(gmp_format(&rx, &fmt).unwrap(), s, "{x} {fmt}");
        assert!(s.is_ascii());
        // Surrounding literal text and `%%` escapes pass through unchanged.
        let wrapped = format!("a %% {fmt} b");
        let sw = format_rational_str(&x, &wrapped).unwrap();
        assert_eq!(gmp_format(&rx, &wrapped).unwrap(), sw);
        assert_eq!(sw, format!("a % {s} b"));
    });
}

fn gmp_format_q_int(x: &rug::Rational, y: i32, fmt: &str) -> Option<String> {
    unsafe extern "C" {
        fn __gmp_snprintf(
            buf: *mut core::ffi::c_char,
            n: usize,
            template: *const core::ffi::c_char,
            ...
        ) -> core::ffi::c_int;
    }
    let template = std::ffi::CString::new(fmt).ok()?;
    let mut buf = vec![0u8; 1 << 12];
    let n = unsafe {
        __gmp_snprintf(
            buf.as_mut_ptr().cast(),
            buf.len(),
            template.as_ptr(),
            x.as_raw(),
            y as core::ffi::c_int,
        )
    };
    if n < 0 || n as usize >= buf.len() {
        return None;
    }
    Some(String::from_utf8_lossy(&buf[..n as usize]).into_owned())
}

#[test]
fn test_gmp_format_multi() {
    let q = Rational::from_signeds(255, 16);
    let rq = rug::Rational::from(&q);

    let fmt = "%#Qx out of %d";
    let s = gmp_format!(fmt, q, 100u32).unwrap();
    assert_eq!(s, "0xff/0x10 out of 100");
    assert_eq!(gmp_format_q_int(&rq, 100, fmt).unwrap(), s);

    // rationals and integers mix, each requiring its own type character
    assert_eq!(
        gmp_format!("%Qd | %Zd", q, Natural::from(7u32)).unwrap(),
        "255/16 | 7"
    );
    assert!(gmp_format!("%Zd", q).is_none());
    assert!(gmp_format!("%Qd", Natural::from(7u32)).is_none());
}

#[test]
fn gmp_format_multi_properties() {
    use malachite_q::test_util::generators::rational_string_pair_gen_var_1;
    rational_string_pair_gen_var_1().test_properties(|(x, fmt)| {
        let rx = rug::Rational::from(&x);
        // The single-value entry and the multi-argument walker agree on %Q templates.
        let s = format_rational_str(&x, &fmt).unwrap();
        assert_eq!(gmp_format!(&*fmt, x).unwrap(), s);
        // Appending a C conversion consumes a second argument, matching gmp_snprintf.
        let fmt2 = format!("{fmt} %-4x");
        let sw = gmp_format!(&*fmt2, x, 26u32).unwrap();
        assert_eq!(gmp_format_q_int(&rx, 26, &fmt2).unwrap(), sw, "{x} {fmt2}");
    });
}
