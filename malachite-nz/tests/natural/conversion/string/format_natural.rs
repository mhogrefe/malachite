// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::gmp_format;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::conversion::string::format_natural::format_natural_str;
use malachite_nz::test_util::generators::natural_string_pair_gen_var_1;
use std::str::FromStr;

// Formats `x` with `fmt` using GMP's own `gmp_snprintf` (linked in via rug), the reference we must
// match. Returns `None` when GMP reports an error. `fmt` must contain exactly one conversion, a
// `%Z` one, so that the variadic call is well-formed.
pub fn gmp_format(x: &rug::Integer, fmt: &str) -> Option<String> {
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
fn test_format_natural_str() {
    fn test(s: &str, fmt: &str, out: Option<&str>) {
        let x = Natural::from_str(s).unwrap();
        assert_eq!(format_natural_str(&x, fmt).as_deref(), out, "{s} {fmt}");
        if out.is_some() {
            // every valid case must agree with GMP itself
            assert_eq!(gmp_format(&rug::Integer::from(&x), fmt).as_deref(), out);
        }
    }
    // the conversion characters; d, i, and u are all the same
    test("255", "%Zd", Some("255"));
    test("255", "%Zi", Some("255"));
    test("255", "%Zu", Some("255"));
    test("255", "%Zo", Some("377"));
    test("255", "%Zx", Some("ff"));
    test("255", "%ZX", Some("FF"));
    test("0", "%Zd", Some("0"));
    test("0", "%Zx", Some("0"));
    test(
        "1267650600228229401496703205376",
        "%Zd",
        Some("1267650600228229401496703205376"),
    );
    test(
        "1267650600228229401496703205376",
        "%Zx",
        Some("10000000000000000000000000"),
    );

    // the # flag adds a base prefix, unless the digits already begin with a zero
    test("255", "%#Zx", Some("0xff"));
    test("255", "%#ZX", Some("0XFF"));
    test("255", "%#Zo", Some("0377"));
    test("255", "%#Zd", Some("255"));
    test("0", "%#Zx", Some("0"));
    test("0", "%#Zo", Some("0"));

    // sign flags; a later one overwrites an earlier one
    test("255", "%+Zd", Some("+255"));
    test("255", "% Zd", Some(" 255"));
    test("255", "%+ Zd", Some(" 255"));
    test("255", "% +Zd", Some("+255"));
    test("0", "% Zd", Some(" 0"));

    // width, justification, and zero padding
    test("255", "%8Zd", Some("     255"));
    test("255", "%-8Zd", Some("255     "));
    test("255", "%08Zd", Some("00000255"));
    test("255", "%2Zd", Some("255"));
    test("255", "%+8Zd", Some("    +255"));
    test("255", "%+08Zd", Some("+0000255"));
    test("255", "%#08Zx", Some("0x0000ff"));
    test("0", "%05Zd", Some("00000"));
    // GMP quirks: the 0 flag sets the fill for left justification too, and flags may appear after
    // the width
    test("255", "%-08Zd", Some("25500000"));
    test("255", "%0-8Zd", Some("25500000"));
    test("255", "%5+Zd", Some(" +255"));
    test("255", "%-+8Zd", Some("+255    "));

    // precision is the minimum number of digits
    test("255", "%.6Zd", Some("000255"));
    test("255", "%.2Zd", Some("255"));
    test("255", "%.0Zd", Some("255"));
    test("7", "%#.3Zo", Some("0007"));
    // GMP applies both the precision zeros and the 0 flag's fill
    test("255", "%08.5Zd", Some("00000255"));
    // a zero value with an explicit precision of 0 produces no digits...
    test("0", "%.0Zd", Some(""));
    test("0", "%5.0Zd", Some("     "));
    // ...which un-suppresses the base prefix
    test("0", "%#.0Zx", Some("0x"));
    test("0", "%#.0Zo", Some("0"));
    // a `.` alone also means all necessary digits
    test("255", "%.Zd", Some("255"));

    // the ' flag is accepted but has no effect on GMP types
    test("1000000", "%'Zd", Some("1000000"));

    // parser corners, each verified against GMP: a second digit run replaces the width...
    test("255", "%1 2Zd", Some(" 255"));
    // ...the type character may be repeated, and a later type character overwrites an earlier
    // one...
    test("255", "%ZZd", Some("255"));
    test("255", "%hZd", Some("255"));
    test("255", "%lZd", Some("255"));
    // ...and flags may follow the precision
    test("255", "%.5 Zd", Some(" 00255"));

    // literal text and %% escapes
    test("255", "x = %Zd!", Some("x = 255!"));
    // literal text passes through as UTF-8
    test("255", "π = %Zd ✓", Some("π = 255 ✓"));
    test("255", "100%% of %Zd", Some("100% of 255"));
    test("255", "%%%Zd%%", Some("%255%"));

    // invalid or unsupported format strings
    test("255", "", None);
    test("255", "abc", None);
    test("255", "%", None);
    test("255", "%Z", None);
    test("255", "%d", None);
    test("255", "%s", None);
    test("255", "%Zs", None);
    test("255", "%Zn", None);
    test("255", "%Zhd", None);
    test("255", "%zd", None);
    test("255", "%Qd", None);
    test("255", "%Rf", None);
    test("255", "%*Zd", None);
    test("255", "%.*Zd", None);
    test("255", "%Zd %Zd", None);
    test("255", "%9999999999999999999999Zd", None);
    test("255", "%.9999999999999999999999Zd", None);
    // widths and precisions beyond C's int range are rejected, as GMP cannot express them
    test("255", "%2147483648Zd", None);
    test("255", "%.2147483648Zd", None);
}

#[test]
fn format_natural_str_properties() {
    natural_string_pair_gen_var_1().test_properties(|(x, fmt)| {
        let rx = rug::Integer::from(&x);
        let s = format_natural_str(&x, &fmt).unwrap();
        // The primary oracle: we must produce exactly what GMP's own `gmp_snprintf` produces.
        assert_eq!(gmp_format(&rx, &fmt).unwrap(), s, "{x} {fmt}");
        assert!(s.is_ascii());
        // Surrounding literal text and `%%` escapes pass through unchanged.
        let wrapped = format!("a %% {fmt} b");
        let sw = format_natural_str(&x, &wrapped).unwrap();
        assert_eq!(gmp_format(&rx, &wrapped).unwrap(), sw);
        assert_eq!(sw, format!("a % {s} b"));
    });
}

// The two-argument analogues of `gmp_format`, again via `gmp_snprintf`: the variadic FFI call needs
// a fixed arity, so each argument shape gets its own helper.
fn gmp_format_2z(x: &rug::Integer, y: &rug::Integer, fmt: &str) -> Option<String> {
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
            y.as_raw(),
        )
    };
    if n < 0 || n as usize >= buf.len() {
        return None;
    }
    Some(String::from_utf8_lossy(&buf[..n as usize]).into_owned())
}

fn gmp_format_z_int(x: &rug::Integer, y: i32, fmt: &str) -> Option<String> {
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
    // Mixed-type templates, each checked against gmp_snprintf itself.
    let n = Natural::from(255u32);
    let z = Integer::from(-255);
    let rn = rug::Integer::from(&n);
    let rz = rug::Integer::from(&z);

    let fmt = "%Zd and %#Zx";
    let s = gmp_format!(fmt, n, z).unwrap();
    assert_eq!(s, "255 and -0xff");
    assert_eq!(gmp_format_2z(&rn, &rz, fmt).unwrap(), s);

    let fmt = "%08Zd, % d!";
    let s = gmp_format!(fmt, z, 42u32).unwrap();
    assert_eq!(s, "-0000255,  42!");
    assert_eq!(gmp_format_z_int(&rz, 42, fmt).unwrap(), s);

    // a `*` width drawn from the argument list, with the width before the mpz
    let fmt = "%0*Zx";
    let s = gmp_format!(fmt, 8i32, n).unwrap();
    assert_eq!(s, "000000ff");
    {
        unsafe extern "C" {
            fn __gmp_snprintf(
                buf: *mut core::ffi::c_char,
                n: usize,
                template: *const core::ffi::c_char,
                ...
            ) -> core::ffi::c_int;
        }
        let template = std::ffi::CString::new(fmt).unwrap();
        let mut buf = vec![0u8; 64];
        let len = unsafe {
            __gmp_snprintf(
                buf.as_mut_ptr().cast(),
                buf.len(),
                template.as_ptr(),
                8 as core::ffi::c_int,
                rn.as_raw(),
            )
        };
        assert_eq!(
            String::from_utf8_lossy(&buf[..len as usize]).into_owned(),
            s
        );
    }

    // strings and characters mix in
    assert_eq!(gmp_format!("%s = %Zd%c", "x", n, '!').unwrap(), "x = 255!");

    // failures: a bignum refuses a plain %d, and a primitive refuses %Zd
    assert!(gmp_format!("%d", n).is_none());
    assert!(gmp_format!("%Zd", 5u32).is_none());
    assert!(gmp_format!("%Zd %Zd", n).is_none());
}

#[test]
fn gmp_format_multi_properties() {
    natural_string_pair_gen_var_1().test_properties(|(x, fmt)| {
        let rx = rug::Integer::from(&x);
        // The single-value entry and the multi-argument walker agree on %Z templates.
        let s = format_natural_str(&x, &fmt).unwrap();
        assert_eq!(gmp_format(&rx, &fmt).unwrap(), s, "{x} {fmt}");
        assert_eq!(gmp_format!(&*fmt, x).unwrap(), s);
        // Appending a C conversion consumes a second argument, matching gmp_snprintf.
        let fmt2 = format!("{fmt} % 5d");
        let sw = gmp_format!(&*fmt2, x, 42i32).unwrap();
        assert_eq!(gmp_format_z_int(&rx, 42, &fmt2).unwrap(), sw, "{x} {fmt2}");
    });
}
