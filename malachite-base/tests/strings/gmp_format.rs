// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::gmp_format;
use malachite_base::strings::gmp_format::{GmpFormatArg, gmp_format, parse_gmp_conversion_spec};

#[test]
fn test_parse_gmp_conversion_spec() {
    // the pieces land in their fields
    let (spec, rest) = parse_gmp_conversion_spec(b"#-'8.3Zd tail", &mut || None).unwrap();
    assert!(spec.alt);
    assert!(spec.left);
    assert!(spec.group);
    assert_eq!(spec.width, 8);
    assert_eq!(spec.prec, Some(3));
    assert_eq!(spec.type_chr, b'Z');
    assert_eq!(spec.conv, b'd');
    assert_eq!(rest, b" tail");

    // the later of the + and space flags wins in `sign`, but both are recorded
    let (spec, _) = parse_gmp_conversion_spec(b"+ d", &mut || None).unwrap();
    assert_eq!(spec.sign, b' ');
    assert!(spec.plus);
    assert!(spec.space);

    // a `.` with no digits is Some(-1); no `.` is None
    let (spec, _) = parse_gmp_conversion_spec(b".d", &mut || None).unwrap();
    assert_eq!(spec.prec, Some(-1));
    let (spec, _) = parse_gmp_conversion_spec(b"d", &mut || None).unwrap();
    assert_eq!(spec.prec, None);

    // doubled and later-overwritten type characters
    let (spec, _) = parse_gmp_conversion_spec(b"lld", &mut || None).unwrap();
    assert_eq!(spec.type_chr, b'l');
    assert!(spec.type_doubled);
    let (spec, _) = parse_gmp_conversion_spec(b"hZd", &mut || None).unwrap();
    assert_eq!(spec.type_chr, b'Z');
    assert!(!spec.type_doubled);

    // MPFR's rounding character comes directly after the R; a Z there is a rounding character, not
    // the mpz type
    let (spec, _) = parse_gmp_conversion_spec(b"RZf", &mut || None).unwrap();
    assert_eq!(spec.type_chr, b'R');
    assert_eq!(spec.rnd_chr, b'Z');
    assert_eq!(spec.conv, b'f');
    // an F after an R is the conversion; elsewhere it is the mpf type
    let (spec, _) = parse_gmp_conversion_spec(b"RF", &mut || None).unwrap();
    assert_eq!(spec.conv, b'F');
    let (spec, _) = parse_gmp_conversion_spec(b"Fe", &mut || None).unwrap();
    assert_eq!(spec.type_chr, b'F');
    assert_eq!(spec.conv, b'e');

    // a `*` consumes from the supplier; a negative width means left justification
    let mut it = [-8i64, 3].into_iter();
    let (spec, _) = parse_gmp_conversion_spec(b"*.*d", &mut || it.next()).unwrap();
    assert!(spec.left);
    assert_eq!(spec.width, 8);
    assert_eq!(spec.prec, Some(3));
    // a negative `*` precision is treated as 0
    let mut it = [-3i64].into_iter();
    let (spec, _) = parse_gmp_conversion_spec(b".*d", &mut || it.next()).unwrap();
    assert_eq!(spec.prec, Some(0));

    // failures: no supplier for `*`, width/precision beyond a C int, a `*` rounding character,
    // truncated or invalid input
    assert!(parse_gmp_conversion_spec(b"*d", &mut || None).is_none());
    assert!(parse_gmp_conversion_spec(b"2147483648d", &mut || None).is_none());
    assert!(parse_gmp_conversion_spec(b".2147483648d", &mut || None).is_none());
    assert!(parse_gmp_conversion_spec(b"R*f", &mut || None).is_none());
    assert!(parse_gmp_conversion_spec(b"", &mut || None).is_none());
    assert!(parse_gmp_conversion_spec(b"8", &mut || None).is_none());
    assert!(parse_gmp_conversion_spec(b"\xff", &mut || None).is_none());
}

#[test]
fn test_gmp_format_primitive_ints() {
    fn test(fmt: &str, arg: &dyn GmpFormatArg, out: Option<&str>) {
        assert_eq!(gmp_format(fmt, &[arg]).as_deref(), out, "{fmt}");
    }
    // the conversion characters; d, i, and u format the value as passed
    test("%d", &255u32, Some("255"));
    test("%i", &255u32, Some("255"));
    test("%u", &255u32, Some("255"));
    test("%o", &255u32, Some("377"));
    test("%x", &255u32, Some("ff"));
    test("%X", &255u32, Some("FF"));
    test("%d", &-255i32, Some("-255"));
    test("%x", &-255i32, Some("-ff"));
    test("%d", &0u8, Some("0"));
    test(
        "%d",
        &u128::MAX,
        Some("340282366920938463463374607431768211455"),
    );

    // C length modifiers are accepted but do not truncate the value
    test("%hd", &255u32, Some("255"));
    test("%hhd", &255u32, Some("255"));
    test("%ld", &255u64, Some("255"));
    test("%lld", &255u64, Some("255"));
    test("%jd", &255u64, Some("255"));
    test("%zd", &255u64, Some("255"));
    test("%td", &255u64, Some("255"));
    test("%qd", &255u64, Some("255"));

    // the # flag, only when the digits do not already begin with a zero
    test("%#x", &255u32, Some("0xff"));
    test("%#X", &255u32, Some("0XFF"));
    test("%#o", &255u32, Some("0377"));
    test("%#x", &0u32, Some("0"));
    test("%#o", &0u32, Some("0"));

    // C sign-flag precedence: + overrides space, in either order
    test("%+d", &255u32, Some("+255"));
    test("% d", &255u32, Some(" 255"));
    test("%+ d", &255u32, Some("+255"));
    test("% +d", &255u32, Some("+255"));
    test("%+d", &-255i32, Some("-255"));

    // width and justification
    test("%8d", &255u32, Some("     255"));
    test("%-8d", &255u32, Some("255     "));
    test("%08d", &255u32, Some("00000255"));
    test("%+08d", &255u32, Some("+0000255"));
    test("%#08x", &255u32, Some("0x0000ff"));
    test("%08d", &-255i32, Some("-0000255"));
    // in C, unlike for GMP's own types, the 0 flag is ignored with left justification or an
    // explicit precision
    test("%-08d", &255u32, Some("255     "));
    test("%08.5d", &255u32, Some("   00255"));

    // precision is the minimum number of digits
    test("%.5d", &255u32, Some("00255"));
    test("%.0d", &0u32, Some(""));
    test("%5.0d", &0u32, Some("     "));
    test("%.d", &0u32, Some(""));

    // the ' flag groups nothing in the C locale
    test("%'d", &1000000u32, Some("1000000"));

    // %c keeps the value's lowest byte, as C does
    test("%c", &65u32, Some("A"));
    test("%c", &321u32, Some("A"));
    test("%3c", &65u32, Some("  A"));

    // conversions that do not apply
    test("%s", &255u32, None);
    test("%Zd", &255u32, None);
    test("%Qd", &255u32, None);
    test("%Rf", &255u32, None);
    test("%e", &255u32, None);
    test("%p", &255u32, None);
    test("%n", &255u32, None);
}

#[test]
fn test_gmp_format_char_and_str() {
    fn test(fmt: &str, arg: &dyn GmpFormatArg, out: Option<&str>) {
        assert_eq!(gmp_format(fmt, &[arg]).as_deref(), out, "{fmt}");
    }
    test("%c", &'A', Some("A"));
    test("%c", &'é', Some("é"));
    test("%5c", &'A', Some("    A"));
    test("%-5c!", &'A', Some("A    !"));
    test("%d", &'A', None);

    test("%s", &"hello", Some("hello"));
    test("%8s", &"hello", Some("   hello"));
    test("%-8s!", &"hello", Some("hello   !"));
    // the precision is the maximum number of bytes
    test("%.3s", &"hello", Some("hel"));
    test("%.8s", &"hello", Some("hello"));
    test("%.0s", &"hello", Some(""));
    // a precision that would split a multi-byte character is rejected
    test("%.1s", &"é", None);
    test("%s", &"hello".to_string(), Some("hello"));
    test("%d", &"hello", None);
    test("%ls", &"hello", None);
}

#[test]
fn test_gmp_format_multiple() {
    // conversions consume the values in order, and literal text passes through
    assert_eq!(
        gmp_format("%d + %d = %d", &[&2u32, &2u32, &4u32]).as_deref(),
        Some("2 + 2 = 4")
    );
    assert_eq!(
        gmp_format("%c%s%c", &[&'(', &"hello", &')']).as_deref(),
        Some("(hello)")
    );
    // a `*` width or precision consumes the next value
    assert_eq!(
        gmp_format("%0*x", &[&8i32, &255u32]).as_deref(),
        Some("000000ff")
    );
    assert_eq!(
        gmp_format("%*d|", &[&-8i32, &255u32]).as_deref(),
        Some("255     |")
    );
    assert_eq!(
        gmp_format("%.*d", &[&5u32, &255u32]).as_deref(),
        Some("00255")
    );
    // no conversions at all is fine
    assert_eq!(gmp_format("100%%", &[]).as_deref(), Some("100%"));
    assert_eq!(gmp_format("", &[]).as_deref(), Some(""));
    // extra values are permitted, as in C
    assert_eq!(gmp_format("%d", &[&1u32, &2u32]).as_deref(), Some("1"));
    // too few values, a non-integer `*` supplier, or a value refusing its conversion
    assert_eq!(gmp_format("%d %d", &[&5u32]), None);
    assert_eq!(gmp_format("%*d", &[&"8", &5u32]), None);
    assert_eq!(gmp_format("%s %d", &[&5u32, &"s"]), None);

    // the macro builds the argument slice
    assert_eq!(
        gmp_format!("%s is %d", "x", 5u32).as_deref(),
        Some("x is 5")
    );
    assert_eq!(gmp_format!("no args").as_deref(), Some("no args"));
}
