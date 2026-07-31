// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::string::from_string::{
    digit_from_display_byte, digit_from_display_byte_large,
};
use malachite_base::num::conversion::string::to_string::{
    digit_to_display_byte_lower, digit_to_display_byte_upper,
};
use malachite_base::num::conversion::traits::{FromStringBase, ToStringBase};
use malachite_base::test_util::generators::{unsigned_gen, unsigned_gen_var_10};

#[test]
fn test_digit_from_display_byte() {
    let test_ok = |x, y| {
        assert_eq!(digit_from_display_byte(x).unwrap(), y);
    };
    test_ok(b'0', 0);
    test_ok(b'1', 1);
    test_ok(b'2', 2);
    test_ok(b'3', 3);
    test_ok(b'4', 4);
    test_ok(b'5', 5);
    test_ok(b'6', 6);
    test_ok(b'7', 7);
    test_ok(b'8', 8);
    test_ok(b'9', 9);
    test_ok(b'a', 10);
    test_ok(b'b', 11);
    test_ok(b'c', 12);
    test_ok(b'x', 33);
    test_ok(b'y', 34);
    test_ok(b'z', 35);
    test_ok(b'A', 10);
    test_ok(b'B', 11);
    test_ok(b'C', 12);
    test_ok(b'X', 33);
    test_ok(b'Y', 34);
    test_ok(b'Z', 35);

    let test_err = |x| {
        assert!(digit_from_display_byte(x).is_none());
    };
    test_err(b' ');
    test_err(b'!');
}

#[test]
fn digit_from_display_byte_properties() {
    unsigned_gen().test_properties(|b| {
        assert_eq!(
            digit_from_display_byte(b).is_some(),
            b.is_ascii_alphanumeric()
        );
    });

    unsigned_gen_var_10().test_properties(|b| {
        let digit = digit_from_display_byte(b).unwrap();
        assert!(b.is_ascii_alphanumeric());
        // Both of the following conditions include numeric chars.
        if !char::from(b).is_uppercase() {
            assert_eq!(digit_to_display_byte_lower(digit), Some(b));
        }
        if !char::from(b).is_lowercase() {
            assert_eq!(digit_to_display_byte_upper(digit), Some(b));
        }
    });
}

#[test]
fn test_digit_from_display_byte_large() {
    assert_eq!(digit_from_display_byte_large(b'9'), Some(9));
    assert_eq!(digit_from_display_byte_large(b'B'), Some(11));
    assert_eq!(digit_from_display_byte_large(b'b'), Some(37));
    assert_eq!(digit_from_display_byte_large(b' '), None);
}

#[test]
fn test_from_string_base_large() {
    // the two cases are distinct digits above base 36, and digits must be smaller than the base
    assert_eq!(u32::from_string_base(62, "G8"), Some(1000));
    assert_eq!(u32::from_string_base(62, "g8"), Some(2612));
    assert_eq!(u32::from_string_base(62, "+G8"), Some(1000));
    assert_eq!(u32::from_string_base(37, "a"), Some(36));
    assert_eq!(u32::from_string_base(37, "b"), None);
    assert_eq!(u32::from_string_base(62, ""), None);
    assert_eq!(u32::from_string_base(62, "+"), None);
    assert_eq!(u32::from_string_base(62, "++1"), None);
    assert_eq!(u32::from_string_base(62, "-G8"), None);
    assert_eq!(u32::from_string_base(62, "!"), None);
    // overflow is rejected
    assert_eq!(u8::from_string_base(62, "zz"), None);
    assert_eq!(u8::from_string_base(62, "45"), Some(253));
    // signed values, including the negative-accumulation path that lets MIN parse
    assert_eq!(i32::from_string_base(62, "-G8"), Some(-1000));
    assert_eq!(i32::from_string_base(62, "+G8"), Some(1000));
    assert_eq!(
        i64::from_string_base(62, &i64::MIN.to_string_base(62)),
        Some(i64::MIN)
    );
    assert_eq!(i8::from_string_base(62, "-22"), Some(-126));
    // -128 parses via the negative-accumulation path, and -134 overflows
    assert_eq!(i8::from_string_base(62, "-24"), Some(i8::MIN));
    assert_eq!(i8::from_string_base(62, "-2A"), None);
    assert_eq!(i8::from_string_base(62, "24"), None);
}
